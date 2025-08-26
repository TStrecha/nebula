use nebula_ast::item::{Expr, Literal};
use nebula_parser::parser::Parser;

#[test]
fn test_parse_lit_expr() {
    let mut parser = Parser::new("1234");
    assert_eq!(parser.parse_expr(), Expr::Lit(Literal::Number(1234)));

    let mut parser = Parser::new("123.4");
    assert_eq!(parser.parse_expr(), Expr::Lit(Literal::Decimal(123.4)));

    let mut parser = Parser::new("\"1234\"");
    assert_eq!(
        parser.parse_expr(),
        Expr::Lit(Literal::StringLit(String::from("1234")))
    );
}

#[test]
#[should_panic(expected = "String was not terminated")]
fn test_parse_string_lit_unterminated() {
    let mut parser = Parser::new("\"1234");
    parser.parse_expr();
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

#[test]
#[should_panic(expected = "Expected assignment operator, found Equals")]
fn test_parse_var_decl_expr_wrong_operator() {
    let mut parser = Parser::new("var a == abcd;");
    parser.parse_expr();
}

#[test]
#[should_panic(expected = "Expected semicolon: ;")]
fn test_parse_var_decl_expr_missing_expression() {
    let mut parser = Parser::new("var a = abcd");
    parser.parse_expr();
}

#[test]
#[should_panic(expected = "Expected operator, found Ident(\"abcd\")")]
fn test_parse_var_decl_expr_missing_operator() {
    let mut parser = Parser::new("var a abcd;");
    parser.parse_expr();
}
