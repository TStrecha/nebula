use nebula_ast::item::{Expr, Item};
use nebula_parser::parser::Parser;

#[test]
fn test_parse_lit_expr() {
    let mut parser = Parser::new("var a = abcd;");

    assert_eq!(
        parser.next_item(),
        Some(Item::Expr(Expr::VarDecl {
            name: String::from("a"),
            value: Box::new(Expr::Ident(String::from("abcd")))
        }))
    );

    assert_eq!(parser.next_item(), None)
}
