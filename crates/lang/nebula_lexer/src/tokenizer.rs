use crate::{cursor::Cursor, token::Token};

pub fn tokenize(data: String) -> Vec<Token> {
    let mut cursor = Cursor::new(data);

    let mut tokens = vec![];

    loop {
        let next_token = cursor.next_token();

        if next_token == Token::EOF {
            tokens.push(next_token);
            break;
        }
        tokens.push(next_token);
    }

    return tokens;
}

pub fn is_terminator(ch: char) -> bool {
    matches!(ch, ';' | '{' | '}')
}

pub fn is_whitespace(ch: char) -> bool {
    matches!(ch, ' ' | '\n')
}

pub fn is_keyword(token_str: &str) -> bool {
    matches!(token_str, "var")
}

pub fn is_operator(ch: char) -> bool {
    matches!(ch, '=')
}

pub fn is_numeric(ch: char) -> bool {
    ch.is_digit(10)
}
