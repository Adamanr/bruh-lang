#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Var {
    A,
    B,
    C,
}

#[derive(Debug, Clone)]
pub enum LoopCond {
    NotEqual(Var, Var),
    Greater(Var, Var),
    Less(Var, Var),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    IncBy1(Var),
    DecBy1(Var),
    IncByVar(Var, Var),
    DecByVar(Var, Var),
    Assign { dst: Var, src: Var },
    PrintInt(Var),
    PrintChar(Var),
    Loop {
        cond: LoopCond,
        body: Vec<Stmt>,
        line: usize,
    },
}
