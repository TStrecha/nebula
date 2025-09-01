#[derive(Debug)]
pub enum IRValue {
    Number(u64),
    Decimal(f64),
    String(String),
    Var(String),
    IRTemp(IRTemp),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IRTemp(pub usize);

#[derive(Debug)]
pub enum IRInstruction {
    LoadConst { target: IRTemp, value: IRValue },
    LoadVar { target: IRTemp, name: String },
    StoreVar { name: String, source: IRTemp },
    Add { target: IRTemp, left: IRValue, right: IRValue },
    Sub { target: IRTemp, left: IRValue, right: IRValue },
    Multiply { target: IRTemp, left: IRValue, right: IRValue },
    Divide { target: IRTemp, left: IRValue, right: IRValue },
}