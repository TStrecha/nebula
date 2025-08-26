use nebula_ast::item::{Expr, Item};
use nebula_lexer::token::{LiteralKind, OperatorKind};
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

#[test]
fn test_consume_ident() {
    let mut parser = Parser::new("a");
    let ident = parser.consume_ident();

    assert_eq!(ident, "a");
}

#[test]
#[should_panic(expected = "Expected identifier, found Keyword(\"var\"")]
fn test_consume_ident_fail() {
    let mut parser = Parser::new("var");
    parser.consume_ident();
}

#[test]
fn test_consume_semicolon() {
    let mut parser = Parser::new(";");
    parser.consume_semicolon();
}

#[test]
#[should_panic(expected = "Expected semicolon, found Ident(\"a\")")]
fn test_consume_semicolon_fail() {
    let mut parser = Parser::new("a");
    parser.consume_semicolon();
}

#[test]
fn test_consume_lit() {
    let mut parser = Parser::new("123");
    let lit = parser.consume_lit();

    assert_eq!(lit, &LiteralKind::Number(123));
}

#[test]
#[should_panic(expected = "Expected literal, found Ident(\"a\")")]
fn test_consume_lit_fail() {
    let mut parser = Parser::new("a");
    parser.consume_lit();
}

#[test]
fn test_consume_operator() {
    let mut parser = Parser::new("==");
    let lit = parser.consume_operator();

    assert_eq!(lit, &OperatorKind::Equals);
}

#[test]
#[should_panic(expected = "Expected operator, found Ident(\"a\")")]
fn test_consume_operator_fail() {
    let mut parser = Parser::new("a");
    parser.consume_operator();
}