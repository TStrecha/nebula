use crate::{
    token::{LiteralKind, OperatorKind, Token},
    tokenizer,
};

#[derive(Debug)]
pub struct Cursor {
    pos: usize,
    len: usize,
    data: String,
}

impl Cursor {
    pub fn new(data: String) -> Self {
        Self {
            pos: 0,
            len: data.len(),
            data,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.pos == self.len - 1 {
            return Token::EOF;
        }

        while tokenizer::is_whitespace(self.data.chars().nth(self.pos).unwrap()) {
            self.pos = self.pos + 1;
        }

        let mut token_str = String::new();
        let mut lit_kind = LitKind::None;

        loop {
            let mut include_char = true;
            let curr_char = self.data.chars().nth(self.pos).unwrap();

            if tokenizer::is_whitespace(curr_char) && lit_kind != LitKind::String {
                break;
            }

            if tokenizer::is_terminator(curr_char) && !token_str.is_empty() {
                break;
            }

            if curr_char == '"' {
                if lit_kind == LitKind::None {
                    include_char = false;
                    lit_kind = LitKind::String;
                } else if lit_kind == LitKind::String {
                    self.pos = self.pos + 1;
                    break;
                }
            }

            if tokenizer::is_numeric(curr_char) && lit_kind == LitKind::None {
                lit_kind = LitKind::Number;
            } else {
                if curr_char == '.' {
                    if lit_kind == LitKind::Number {
                        lit_kind = LitKind::Decimal;
                    } else {
                        break;
                    }
                } else if lit_kind == LitKind::Number {
                    break;
                }
            }

            if include_char {
                token_str.push(self.data.chars().nth(self.pos).unwrap());
            }
            self.pos = self.pos + 1;
        }

        if lit_kind != LitKind::None {
            return match lit_kind {
                LitKind::Number => Token::Literal(LiteralKind::Number(token_str.parse().unwrap())),
                LitKind::Decimal => {
                    Token::Literal(LiteralKind::Decimal(token_str.parse().unwrap()))
                }
                LitKind::String => Token::Literal(LiteralKind::StringLit(token_str)),
                LitKind::None => unreachable!(),
            };
        }

        if tokenizer::is_keyword(&token_str) {
            return Token::Keyword(token_str);
        }

        if tokenizer::is_operator(&token_str) {
            return Token::Operator(OperatorKind::Equals);
        }

        if token_str == ";" {
            return Token::Semicolon;
        }

        Token::Ident(token_str)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LitKind {
    None,
    Number,
    Decimal,
    String,
}
