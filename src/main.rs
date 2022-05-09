use std::fs;

mod util;
mod registries;

fn main() {
    println!("Starting...");

    // Module: registries
    registries::generate();

    // lib
    fs::write("./lib.rs", "pub mod registries;").expect("Unable to write to file '/lib.rs'");

    println!("Done!")
}
