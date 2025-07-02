#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token {
    Keyword(String),
    Ident(String),
    Operator(OperatorKind),
    Literal(LiteralKind),
    Semicolon,
    EOF,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OperatorKind {
    Equals,
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LiteralKind {
    Number(u64),
    Decimal(f64),
    StringLit(String),
}
