use nebula_ast::item::{Expr, Literal};
use nebula_lexer::token::{LiteralKind, OperatorKind, Token};

use crate::parser::Parser;

impl<'t> Parser<'t> {
    pub fn parse_expr(&mut self) -> Expr {
        let token = self.consume();

        match token {
            Token::Keyword("var") => {
                let var_name = self.consume_ident().to_string();
                let operator_kind = self.consume_operator();

                if *operator_kind != OperatorKind::Assignment {
                    panic!("Expected assignment operator, found {:?}", operator_kind);
                }

                let value_expr = self.parse_expr();

                self.consume_semicolon();

                Expr::VarDecl {
                    name: String::from(var_name),
                    value: Box::new(value_expr),
                }
            }
            Token::Literal(literal_kind) => {
                let lit_val = match literal_kind {
                    LiteralKind::Number(num) => Literal::Number(*num),
                    LiteralKind::Decimal(dec) => Literal::Decimal(*dec),
                    LiteralKind::StringLit { value, terminated } => {
                        if !terminated {
                            panic!("String was not terminated")
                        }

                        Literal::StringLit(String::from(*value))
                    }
                };

                Expr::Lit(lit_val)
            }
            Token::Ident(name) => Expr::Ident(String::from(*name)),
            _ => panic!("Unsupported expr token: {:?}", token),
        }
    }
}
