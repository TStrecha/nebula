use nebula_ast::Item;
use nebula_ir::{IRInstruction, IRTemp};

pub struct IRBuilder {
    ast: Vec<Item>,
    temp_index: usize,
}

impl IRBuilder {

    pub fn new(ast: Vec<Item>) -> Self {
        Self { ast, temp_index: 0 }
    }

    pub fn next_ir_instruction_batch(&mut self) -> Vec<IRInstruction> {
        let mut batch = vec![];

        let next_item = self.consume_item();
        if let Some(item) = next_item {
            match item {
                Item::Expr(expr) => {
                    for instr in self.handle_expr(expr, None) {
                        batch.push(instr);
                    }
                }
            }
        }

        batch
    }

    fn consume_item(&mut self) -> Option<Item> {
        if self.ast.is_empty() {
            return None;
        }

        Some(self.ast.remove(0))
    }

    pub fn create_temp(&mut self) -> IRTemp {
        let temp = IRTemp(self.temp_index.clone());
        self.temp_index += 1;

        temp
    }
}