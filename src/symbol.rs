#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: Vec<u8>,
    pub line: usize,
    pub col: usize,
}
