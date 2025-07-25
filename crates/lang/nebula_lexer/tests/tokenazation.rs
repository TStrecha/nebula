use nebula_lexer::{
    token::{LiteralKind, OperatorKind, Token},
    tokenizer,
};

#[test]
fn test_tokenize_number_var_declaration() {
    let tokens = tokenizer::tokenize("var a = 123_12___3;".to_string());

    assert_eq!(tokens.len(), 6);

    assert_eq!(tokens[0], Token::Keyword("var".to_string()));
    assert_eq!(tokens[1], Token::Ident("a".to_string()));
    assert_eq!(tokens[2], Token::Operator(OperatorKind::Assignment));
    assert_eq!(tokens[3], Token::Literal(LiteralKind::Number(123123)));
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::EOF);
}

#[test]
fn test_tokenize_string_var_declaration() {
    let tokens = tokenizer::tokenize("var a = \"Hello World\";".to_string());

    assert_eq!(tokens.len(), 6);

    assert_eq!(tokens[0], Token::Keyword("var".to_string()));
    assert_eq!(tokens[1], Token::Ident("a".to_string()));
    assert_eq!(tokens[2], Token::Operator(OperatorKind::Assignment));
    assert_eq!(
        tokens[3],
        Token::Literal(LiteralKind::StringLit {
            value: "Hello World".to_string(),
            terminated: true
        })
    );
    assert_eq!(tokens[4], Token::Semicolon);
    assert_eq!(tokens[5], Token::EOF);

    let tokens = tokenizer::tokenize("var a = \"Hello World;".to_string());

    assert_eq!(tokens.len(), 5);

    assert_eq!(tokens[0], Token::Keyword("var".to_string()));
    assert_eq!(tokens[1], Token::Ident("a".to_string()));
    assert_eq!(tokens[2], Token::Operator(OperatorKind::Assignment));
    assert_eq!(
        tokens[3],
        Token::Literal(LiteralKind::StringLit {
            value: "Hello World;".to_string(),
            terminated: false
        })
    );
    assert_eq!(tokens[4], Token::EOF);
}

#[test]
fn test_tokenize_equals_statement() {
    let tokens = tokenizer::tokenize("a  ==   b".to_string());

    assert_eq!(tokens.len(), 4);

    assert_eq!(tokens[0], Token::Ident("a".to_string()));
    assert_eq!(tokens[1], Token::Operator(OperatorKind::Equals));
    assert_eq!(tokens[2], Token::Ident("b".to_string()));
    assert_eq!(tokens[3], Token::EOF);
}
