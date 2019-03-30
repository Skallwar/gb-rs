use std::env;
use std::path;
use std::{thread, time};

mod cartridge;
mod cpu;
mod lcd;
mod mmu;
mod ppu;
mod regs;

use cpu::Cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file specified !");
    }

    let path = path::Path::new(&args[1]);
    if !path.is_file() {
        panic!("Path does not exist or is not a file !");
    }

    let mut cpu = Cpu::new(path);
    cpu.run();

    println!("------------Thanks for trying this prototype------------");

    let wait = time::Duration::from_millis(10000);
    thread::sleep(wait);
}
