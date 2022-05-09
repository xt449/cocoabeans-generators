mod util;
mod registries;

fn main() {
    println!("Starting...");

    registries::generate();

    println!("Done!")
}
