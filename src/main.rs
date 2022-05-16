mod blocks;
mod registries;
mod util;

fn main() {
    println!("Starting...");

    // Crate: blocks
    println!("Generating Blocks...");
    blocks::generate();

    // Crate: registries
    println!("Generating Registries...");
    registries::generate();

    println!("Done!")
}
