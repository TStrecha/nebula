pub mod expr;
pub mod parser;

use crate::parser::Parser;

const INPUT: &'static str = "
var greet = \"Hello World\";
var a = 10.1;
var b = a;";

fn main() {
    let mut parser = Parser::new(INPUT);

    loop {
        let item = parser.next_item();
        println!("Next item: {:?}", item);

        if item == None {
            println!("End Of File");
            break;
        }
    }
}
