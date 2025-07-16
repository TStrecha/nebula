#[derive(Debug, PartialEq, PartialOrd)]
pub enum Item {
    Expr(Expr),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expr {
    VarDecl { name: String, value: Box<Expr> },
    Lit(Literal),
    Ident(String),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Literal {
    Number(u64),
    Decimal(f64),
    StringLit(String),
}
