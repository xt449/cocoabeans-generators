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
    fs::write("./src/lib.rs", "pub mod blocks; pub mod registries;")
        .expect("Unable to write to file './src/lib.rs'");

    println!("Done!")
}
