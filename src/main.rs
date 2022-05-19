use std::fs;

mod blocks;
mod registries;
mod util;

fn main() {
    println!("Starting...");

    // Module: blocks
    println!("Generating Blocks...");
    blocks::generate();

    // Module: registries
    println!("Generating Registries...");
    registries::generate();

    // Lib
    println!("Finishing...");
    //fs::write("./lib.rs", "pub mod blocks; pub mod registries;").expect("Unable to write to file '/lib.rs'");

    println!("Done!")
}
