use std::env;
use std::fs::File;
use std::io::BufReader;
use nvm::machine::Machine;

#[cfg(not(tarpaulin_include))]
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: nvm <file>");
    }

    let path = &args[1];
    let file = File::open(path).expect("File not found");
    let buffer = BufReader::new(file);

    let mut machine = Machine::default();
    machine.load_program(buffer);
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
    machine.step();
}
