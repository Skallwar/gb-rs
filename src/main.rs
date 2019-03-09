use std::env;
use std::path;

mod cartridge;
mod cpu;
mod mmu;
mod regs;

use cpu::Cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file specified");
    }

    let path = path::Path::new(&args[1]);
    if !path.is_file() {
        panic!("Path does not exist or is not a file !");
    }

    let mut cpu = Cpu::new(path);
    cpu.run();
}
