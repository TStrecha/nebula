use nebula_lexer::{
    cursor::Cursor,
    token::{LiteralKind, OperatorKind, Token},
};

#[test]
fn test_cursor_creation() {
    let mut cursor = Cursor::new("");
    assert_eq!(cursor.next_token(), Token::EOF);
}

#[test]
fn test_eof_detection() {
    let mut cursor = Cursor::new("");
    assert_eq!(cursor.next_token(), Token::EOF);

    let mut cursor = Cursor::new(";");
    assert_eq!(cursor.next_token(), Token::Semicolon);
    assert_eq!(cursor.next_token(), Token::EOF);

    let mut cursor = Cursor::new("var;");
    assert_eq!(cursor.next_token(), Token::Keyword("var"));
    assert_eq!(cursor.next_token(), Token::Semicolon);
    assert_eq!(cursor.next_token(), Token::EOF);

    let mut cursor = Cursor::new("\"test");
    assert_eq!(
        cursor.next_token(),
        Token::Literal(LiteralKind::StringLit {
            value: "test",
            terminated: false
        })
    );
    assert_eq!(cursor.next_token(), Token::EOF);

    let mut cursor = Cursor::new("123");
    assert_eq!(
        cursor.next_token(),
        Token::Literal(LiteralKind::Number(123))
    );
    assert_eq!(cursor.next_token(), Token::EOF);
}

#[test]
fn test_literal_number_detection() {
    let mut cursor = Cursor::new("123");

    assert_eq!(
        cursor.next_token(),
        Token::Literal(LiteralKind::Number(123))
    );
}

#[test]
fn test_decimal_number_detection() {
    let mut cursor = Cursor::new("12.3");

    assert_eq!(
        cursor.next_token(),
        Token::Literal(LiteralKind::Decimal(12.3))
    );
}

#[test]
fn test_string_detection() {
    let mut cursor = Cursor::new("\"12.3\"");

    assert_eq!(
        cursor.next_token(),
        Token::Literal(LiteralKind::StringLit {
            value: "12.3",
            terminated: true
        })
    );

    let mut cursor = Cursor::new("\"Hello");

    assert_eq!(
        cursor.next_token(),
        Token::Literal(LiteralKind::StringLit {
            value: "Hello",
            terminated: false
        })
    );
}

#[test]
fn test_keyword_detection() {
    let mut cursor = Cursor::new("var");
    assert_eq!(cursor.next_token(), Token::Keyword("var"));
}

#[test]
fn test_operator_detection() {
    let mut cursor = Cursor::new("=");
    assert_eq!(
        cursor.next_token(),
        Token::Operator(OperatorKind::Assignment)
    );

    let mut cursor = Cursor::new("==");
    assert_eq!(cursor.next_token(), Token::Operator(OperatorKind::Equals));
}

#[test]
#[should_panic(expected = "Unsupported operator ===")]
fn test_operator_detection_unsupported_operator() {
    let mut cursor = Cursor::new("===");
    cursor.next_token();
}

#[test]
fn test_ident_detection() {
    let mut cursor = Cursor::new("a123");
    assert_eq!(cursor.next_token(), Token::Ident("a123"));
}

#[test]
#[should_panic(expected = "Invalid decimal number")]
fn test_invalid_decimal_number_detection() {
    let mut cursor = Cursor::new("123.12.1");
    cursor.next_token();
}

#[test]
fn test_only_whitespace_data() {
    let mut cursor = Cursor::new("     \n  ");
    assert_eq!(cursor.next_token(), Token::EOF);
}
