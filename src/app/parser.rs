pub enum Expr {
    Number(i64),
    Var(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    }
}

pub enum Stmt {
    Assign(String, Expr),
    Print(Expr),
}