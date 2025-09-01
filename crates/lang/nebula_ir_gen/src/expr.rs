use nebula_ast::{Expr, Item, Literal};
use nebula_ir::{IRInstruction, IRTemp, IRValue};
use crate::IRBuilder;

impl IRBuilder {
    pub fn handle_expr(&mut self, expr: Expr, temp: Option<IRTemp>) -> Vec<IRInstruction> {
        let mut batch = vec![];

        match expr {
            Expr::VarDecl { name, value } => {
                let temp = self.create_temp();

                let value_instructions = self.handle_expr(*value, Some(temp));
                for instr in value_instructions {
                    batch.push(instr)
                }

                batch.push(IRInstruction::StoreVar {
                    name,
                    source: temp
                })
            }
            Expr::Lit(lit) => {
                if let Some(temp) = temp {
                    match lit {
                        Literal::Number(val) => batch.push(IRInstruction::LoadConst {
                            target: temp,
                            value: IRValue::Number(val),
                        }),
                        Literal::Decimal(val) => batch.push(IRInstruction::LoadConst {
                            target: temp,
                            value: IRValue::Decimal(val),
                        }),
                        Literal::StringLit(val) => batch.push(IRInstruction::LoadConst {
                            target: temp,
                            value: IRValue::String(val),
                        }),
                    }
                }
            }
            Expr::Ident(ident) => {
                if let Some(temp) = temp {
                    batch.push(IRInstruction::LoadVar {
                        target: temp,
                        name: ident,
                    });
                }
            }
        }

        batch
    }
}