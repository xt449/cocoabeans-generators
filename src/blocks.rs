use std::fs;

use serde_json::*;

use crate::util;

pub fn generate() {
    let blocks_data = fs::read_to_string("./blocks.json").expect("Unable to read file 'blocks.json'!");
    let blocks: Map<String, Value> = from_str(blocks_data.as_str()).expect("Unable to parse file 'blocks.json'!");

    fs::create_dir_all("./blocks/src/").expect("Unable to write to output location './blocks/src/'!");

    let mut blockstate_registry: Vec<(String, usize)> = vec!();

    let modules = "pub mod block_state;";

    let mut blockstates_contents: String = "#![allow(non_camel_case_types)]\nuse std::fmt::{Display, Formatter};use crate::blocks::BlockState;".to_owned();

    // Block entries
    for entry in blocks {
        let object = entry.1.as_object().unwrap();
        let file_name = util::namespace_to_file_name(&entry.0);
        let pascal_name = util::namespace_to_pascal_case(&entry.0);

        // Default BlockState
        let mut block_default_state: String = String::new();

        // States
        let states = object.get("states").unwrap().as_array().unwrap();

        for state in states {
            let state_obj = state.as_object().unwrap();
            let state_properties_raw = state_obj.get("properties");
            // Single state block
            if state_properties_raw.is_none() {
                block_default_state = format!("{} {{}}", pascal_name);
                blockstate_registry.push((format!("crate::blocks::block_state::{} {{}}", pascal_name), state_obj.get("id").unwrap().as_u64().unwrap() as usize));
                continue;
            }
            // Everything else
            let mut blockstate_entry = format!("crate::blocks::block_state::{} {{ ", pascal_name);
            let properties = state_properties_raw.unwrap().as_object().unwrap();
            for property in properties {
                blockstate_entry += format!("{}: crate::blocks::block_state::{}{}::{}, ", util::property_instance_to_rust_identifier(property.0), pascal_name, util::namespace_to_pascal_case(property.0), util::property_instance_to_rust_identifier(property.1.as_str().unwrap())).as_str();
            }
            blockstate_entry += "}";

            if state_obj.get("default").is_some() {
                block_default_state = blockstate_entry.to_owned();
            }

            blockstate_registry.push((blockstate_entry, state_obj.get("id").unwrap().as_u64().unwrap() as usize));
        }

        let blockstate_struct: String;

        let block_property_names: Vec<String>;
        let mut block_property_variants: Vec<Vec<&str>> = vec!();

        let blockstate_stringer: String;

        // Properties
        //let properties_raw = object.get("properties");
        match object.get("properties") {
            None => {
                // No block properties
                block_property_names = vec!();
                // Simple block state
                blockstate_struct = format!("#[derive(Eq, PartialEq)]pub struct {} {{}}", pascal_name);
                // set block state stringer
                blockstate_stringer = format!("fn to_state_string(&self) -> String {{return \"{}[]\".to_owned();}}", file_name);
            }
            Some(properties) => {
                let properties_map = properties.as_object().unwrap();

                // Some block properties
                block_property_names = properties_map.keys().map(|s| s.to_owned()).collect();
                // Simple block state
                blockstate_struct = format!("#[derive(Eq, PartialEq)]pub struct {} {{ {} }}", pascal_name, block_property_names.iter().map(|p| format!("pub {}: {}{},", util::property_instance_to_rust_identifier(p), pascal_name, util::namespace_to_pascal_case(p))).collect::<String>());
                // set block state stringer
                blockstate_stringer = format!("fn to_state_string(&self) -> String {{return format!(\"{}[{}]\",{});}}", file_name, properties_map.keys().map(|p| p.to_owned() + ":{}").collect::<Vec<String>>().join(","), properties_map.keys().map(|p| format!("self.{}", util::property_instance_to_rust_identifier(p))).collect::<Vec<String>>().join(","));

                for property in properties_map {
                    let variants = property.1.as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect::<Vec<&str>>();
                    block_property_variants.push(variants);

                    // TODO
                }
            }
        }

        let default_impl: String = format!("impl Default for {} {{fn default() -> Self {{return {};}} }}", pascal_name, block_default_state);
        let blockstate_impl: String = format!("impl BlockState for {} {{ {} }}", pascal_name, blockstate_stringer);
        let block_properties: String = (0..block_property_names.len()).map(|i| format!("#[derive(Eq, PartialEq)]pub enum {}{} {{ {} }} impl Display for {}{} {{ fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{ match self {{ {} }} }} }}", pascal_name, util::namespace_to_pascal_case(&block_property_names[i]), block_property_variants[i].iter().map(|v| util::property_instance_to_rust_identifier(v) + ",").collect::<String>(), pascal_name, util::namespace_to_pascal_case(&block_property_names[i]), block_property_variants[i].iter().map(|v| format!("Self::{} => f.write_str(\"{}\"),", util::property_instance_to_rust_identifier(v), v)).collect::<String>())).collect::<String>();

        blockstates_contents += format!("{} {} {} {} ", blockstate_struct, default_impl, blockstate_impl, block_properties).as_str();
    }

    // \/ blocks/block_state.rs

    fs::write("./blocks/src/block_state.rs", blockstates_contents).expect("idk");

    // \/ blocks/mod.rs

    let blockstate_trait = "pub trait BlockState { fn to_state_string(&self) -> String; }";
    let blockstate_id_getter = "pub fn get_id_from_blockstate(blockstate: &dyn BlockState) -> usize { return BLOCKSTATE_TO_ID[&blockstate.to_state_string()]; }";
    let blockstate_state_getter = "pub fn get_blockstate_by_id(id: usize) -> &'static dyn BlockState { return ID_TO_BLOCKSTATE[id]; }";

    // Using crate lazy_static
    let blockstate_id_registry = format!("static BLOCKSTATE_TO_ID: once_cell::sync::Lazy<HashMap<String, usize>> = Lazy::new(|| HashMap::from([{}]));", blockstate_registry.iter().map(|kvp| format!("({}.to_state_string(),{}),", kvp.0, kvp.1)).collect::<String>());
    let blockstate_state_registry = format!("pub const ID_TO_BLOCKSTATE: [&'static dyn BlockState; {}] = [{}];", blockstate_registry.len(), blockstate_registry.iter().map(|kvp| format!("&{},", kvp.0)).collect::<String>());

    let block_simple_trait_struct = "pub trait Block { fn get_blockstate<'a>(&self) -> &'a dyn BlockState; } pub struct SimpleBlock<'t> { blockstate: &'t dyn BlockState, } impl Block for SimpleBlock<'static> { fn get_blockstate<'a>(&self) -> &'a dyn BlockState { return self.blockstate; } }";

    fs::write("./blocks/src/lib.rs",
              format!("{} use std::collections::HashMap; {} {} {} {} {} {}",
                      modules,
                      blockstate_trait,
                      blockstate_id_getter,
                      blockstate_state_getter,
                      blockstate_id_registry,
                      blockstate_state_registry,
                      block_simple_trait_struct
              ),
    ).expect("Unable to write to file './blocks/src/lib.rs'");
}
