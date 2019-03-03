mod cartridge;

use std::env;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file specified");
    }

    let rom = cartridge::new(path::Path::new(&args[1]));
}
