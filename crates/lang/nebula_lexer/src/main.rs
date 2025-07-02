pub mod cursor;
pub mod token;
pub mod tokenizer;
use std::{env, fs::read_to_string};

use crate::{cursor::Cursor, token::Token};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: lexer <file>");
    }

    // Index checked before => safe to unwrap
    let input_file = args.get(1).unwrap();
    let mut cursor = Cursor::new(read_to_string(input_file).unwrap());
    println!("Cursor: {:?}", cursor);

    loop {
        let token = cursor.next_token();

        println!("Next Token: {:?}", token);

        if token == Token::EOF {
            break;
        }
    }
}
