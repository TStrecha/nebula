use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: lexer <file>");
    }

    // Index checked before => safe to unwrap
    let input_file = args.get(1).unwrap();
    println!("FILE: {}", input_file);
}
