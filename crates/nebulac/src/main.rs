use nebula_ir_gen::IRBuilder;

fn main() {
    let input = "var number = 123; var greet = \"Hello World!\"; var a = number;";

    let mut parser = nebula_parser::Parser::new(input);

    let mut ast = vec![];
    loop {
        let next_item = parser.next_item();

        if let Some(next_item) = next_item {
            ast.push(next_item);
        } else {
            break;
        }
    }

    let mut ir_builder = IRBuilder::new(ast);
    loop {
        let ir_batch = ir_builder.next_ir_instruction_batch();

        if ir_batch.is_empty() {
            break;
        }

        println!("Instruction batch: {:?}", ir_batch);
    }
}
