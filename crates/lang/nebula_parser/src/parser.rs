use nebula_ast::item::Item;
use nebula_lexer::{
    token::{LiteralKind, OperatorKind, Token},
    tokenizer,
};

pub struct Parser<'t> {
    tokens: Vec<Token<'t>>,
    index: usize,
}

impl<'t> Parser<'t> {
    pub fn new(input: &'t str) -> Self {
        Self {
            tokens: tokenizer::tokenize(input),
            index: 0,
        }
    }

    pub fn next_item(&mut self) -> Option<Item> {
        let token = self.peek();

        match token {
            Token::Keyword(_) => Some(Item::Expr(self.parse_expr())),
            _ => None,
        }
    }

    pub fn consume_ident(&mut self) -> &str {
        let token = self.consume();

        if let Token::Ident(name) = token {
            return name;
        }

        panic!("");
    }

    pub fn consume_operator(&mut self) -> &OperatorKind {
        let token = self.consume();

        if let Token::Operator(kind) = token {
            return kind;
        }

        panic!("");
    }

    pub fn consume_lit(&mut self) -> &LiteralKind {
        let token = self.consume();

        if let Token::Literal(kind) = token {
            return kind;
        }

        panic!("Expected literal, found {:?}", token);
    }

    pub fn consume_semicolon(&mut self) {
        let token = self.consume();

        if *token != Token::Semicolon {
            panic!("Expected semicolon: ;");
        }
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.index.clone()]
    }

    pub fn consume(&mut self) -> &Token {
        let token = &self.tokens[self.index.clone()];

        self.index = self.index + 1;

        token
    }
}
