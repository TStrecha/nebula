use nebula_ast::item::{Expr, Literal};
use nebula_parser::parser::Parser;

#[test]
fn test_parse_lit_expr() {
    let mut parser = Parser::new("1234");

    assert_eq!(parser.parse_expr(), Expr::Lit(Literal::Number(1234)));
}

#[test]
fn test_parse_ident_expr() {
    let mut parser = Parser::new("abcd");

    assert_eq!(parser.parse_expr(), Expr::Ident(String::from("abcd")));
}

#[test]
fn test_parse_var_decl_expr() {
    let mut parser = Parser::new("var a = abcd;");

    assert_eq!(
        parser.parse_expr(),
        Expr::VarDecl {
            name: String::from("a"),
            value: Box::new(Expr::Ident(String::from("abcd")))
        }
    );
}
