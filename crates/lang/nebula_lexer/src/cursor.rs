use crate::{
    token::{LiteralKind, OperatorKind, Token},
    tokenizer,
};

#[derive(Debug)]
pub struct Cursor<'d> {
    pos: usize,
    len: usize,
    data: &'d str,
}

impl<'d> Cursor<'d> {
    pub fn new(data: &'d str) -> Self {
        Self {
            pos: 0,
            len: data.len(),
            data,
        }
    }

    pub fn next_token(&mut self) -> Token<'d> {
        if self.pos == self.len {
            return Token::EOF;
        }

        loop {
            if let Some(ch) = self.peek() {
                if tokenizer::is_whitespace(ch) {
                    self.consume();
                } else {
                    break;
                }
            } else {
                return Token::EOF;
            }
        }

        let token_type = if let Some(first_char) = self.peek() {
            Cursor::identify_token_type(first_char)
        } else {
            return Token::EOF;
        };

        let start_pos = self.pos;
        let token = match token_type {
            TokenType::StringLiteral => {
                let mut terminated = false;
                self.consume();

                loop {
                    let ch = if let Some(ch) = self.peek() {
                        ch
                    } else {
                        break;
                    };

                    if ch == '"' {
                        terminated = true;
                        self.consume();
                        break;
                    }

                    self.consume();
                }

                let lit_value =
                    &self.data[start_pos + 1..self.pos - if terminated { 1 } else { 0 }];

                Token::Literal(LiteralKind::StringLit {
                    value: lit_value,
                    terminated,
                })
            }
            TokenType::NumericLiteral => {
                let mut literal_value = String::new();
                let mut decimal = false;

                loop {
                    let ch = if let Some(ch) = self.peek() {
                        ch
                    } else {
                        break;
                    };

                    if tokenizer::is_numeric(ch) {
                        literal_value.push(ch);
                        self.consume();
                    } else {
                        if ch == '_' {
                            self.consume();
                            continue;
                        }
                        if ch == '.' {
                            if decimal == false {
                                decimal = true;
                                literal_value.push(ch);
                                self.consume();
                            } else {
                                panic!("Invalid decimal number");
                            }
                        } else {
                            break;
                        }
                    }
                }

                if decimal {
                    Token::Literal(LiteralKind::Decimal(literal_value.parse().unwrap()))
                } else {
                    Token::Literal(LiteralKind::Number(literal_value.parse().unwrap()))
                }
            }
            TokenType::Ident => {
                loop {
                    let ch = if let Some(ch) = self.peek() {
                        ch
                    } else {
                        break;
                    };

                    if tokenizer::is_whitespace(ch) || tokenizer::is_terminator(ch) {
                        break;
                    }

                    self.consume();
                }

                let value = &self.data[start_pos..self.pos];

                let token = if tokenizer::is_keyword(&value) {
                    Token::Keyword(value)
                } else {
                    Token::Ident(value)
                };

                return token;
            }
            TokenType::Operator => {
                let mut value = String::new();

                loop {
                    let ch = if let Some(ch) = self.peek() {
                        ch
                    } else {
                        break;
                    };

                    if !tokenizer::is_operator(ch) {
                        break;
                    }

                    value.push(ch);
                    self.consume();
                }

                match value.as_str() {
                    "=" => Token::Operator(OperatorKind::Assignment),
                    "==" => Token::Operator(OperatorKind::Equals),
                    _ => panic!("Unsupported operator {}", value),
                }
            }
            TokenType::Semicolon => {
                self.consume();
                Token::Semicolon
            }
        };

        return token;
    }

    pub fn identify_token_type(ch: char) -> TokenType {
        return match ch {
            '"' => TokenType::StringLiteral,
            ';' => TokenType::Semicolon,
            x if tokenizer::is_numeric(x) => TokenType::NumericLiteral,
            x if tokenizer::is_operator(x) => TokenType::Operator,
            _ => TokenType::Ident,
        };
    }

    pub fn peek(&self) -> Option<char> {
        self.data.chars().nth(self.pos)
    }

    pub fn consume(&mut self) -> Option<char> {
        let ch = self.peek();
        self.step();

        ch
    }

    pub fn step(&mut self) {
        self.pos = self.pos + 1;
    }
}

#[derive(Debug)]
pub enum TokenType {
    StringLiteral,
    NumericLiteral,
    Ident,
    Operator,
    Semicolon,
}
