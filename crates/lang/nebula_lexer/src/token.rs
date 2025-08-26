#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Token<'s> {
    Keyword(&'s str),
    Ident(&'s str),
    Operator(OperatorKind),
    Literal(LiteralKind<'s>),
    Semicolon,
    EOF,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum OperatorKind {
    Assignment,
    Equals,
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LiteralKind<'s> {
    Number(u64),
    Decimal(f64),
    StringLit { value: &'s str, terminated: bool },
}
