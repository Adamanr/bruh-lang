#[derive(Debug)]
pub struct BruhError {
    pub line: usize,
    pub code: &'static str,
    pub msg: String,
}

#[derive(Debug)]
pub struct LexError {
    pub line: usize,
    pub col: usize,
    pub raw: String,
}
