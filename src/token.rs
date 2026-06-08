#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    VarA,
    VarB,
    VarC,
    Momentum,
    Moment,
    Sound,
    Effect,
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct Spanned {
    pub tok: Token,
    pub pos: Pos,
}
