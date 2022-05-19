use std::fs;
use std::ops::AddAssign;
use std::path::PathBuf;

use serde_json::*;

use crate::util;

pub fn generate() {
    let registries_data =
        fs::read_to_string("./registries.json").expect("Unable to read file 'registries.json'!");
    let registries: Map<String, Value> =
        from_str(registries_data.as_str()).expect("Unable to parse file 'registries.json'!");

    fs::create_dir_all("./registries/src/")
        .expect("Unable to write to output location './registries/src/'!");

    let mut modules: Vec<String> = Vec::new();

    for entry in registries {
        let object = entry.1.as_object().unwrap();
        let file_name = util::namespace_to_file_name(&entry.0);
        let enum_name = util::namespace_to_pascal_case(&entry.0);

        let mut contents: String = String::new();

        // Enum
        let entries = object.get("entries").unwrap().as_object().unwrap();
        contents.add_assign(format!("#![allow(non_camel_case_types, unused)]\n#[derive(Copy, Clone)] pub enum {}Registry {{ {}}}", enum_name, entries.iter().map(|kvp| format!("{} = {}, ", util::namespace_to_rust_identifier(kvp.0), kvp.1.as_object().unwrap().get("protocol_id").unwrap().as_u64().unwrap())).collect::<String>()).as_str());

        // Registry
        let protocol_id = object.get("protocol_id").unwrap().as_u64().unwrap();
        contents.add_assign(format!(" impl crate::Registry for {}Registry {{ fn get_protocol_id() -> u32 {{ return {}; }} }}", enum_name, protocol_id).as_str());

        // Default
        let default = object.get("default");
        if default.is_some() {
            contents.add_assign(format!(" impl Default for {}Registry {{fn default() -> Self {{ return {}Registry::{}; }} }}", enum_name, enum_name, util::namespace_to_rust_identifier(default.unwrap().as_str().unwrap()).as_str()).as_str());
        }

        // TODO - unused
        // fs::create_dir_all(file_path.parent().unwrap()).expect(
        //     format!(
        //         "Unable to create parent file for '{}'!",
        //         file_path.display()
        //     ).as_str(),
        // );
        fs::write(format!("./registries/src/{}.rs", file_name), contents).expect(
            format!("Unable to write to file './registries/src/{}.rs'", file_name).as_str(),
        );
        // let mut file = File::create(format!("./registries/{}.rs", snake)).expect(format!("Unable to create file '/registries/{}.rs'", snake).as_str());
        // file.write_all(contents.as_bytes()).expect(format!("Unable to write to file '/registries/{}.rs'", snake).as_str());

        modules.push(file_name.replace('/', "::"));
    }

    let mut contents: String = String::new();

    // Modules
    contents.add_assign(
        modules
            .iter()
            .map(|m| format!("pub mod {}; ", m))
            .collect::<String>()
            .as_str(),
    );

    // Trait
    contents.add_assign("pub trait Registry { fn get_protocol_id() -> u32; }");

    fs::write("./registries/src/lib.rs", contents)
        .expect("Unable to write to file './registries/src/lib.rs'");
}
