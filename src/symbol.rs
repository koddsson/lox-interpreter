#[derive(Debug)]
pub struct Symbol<'a> {
    pub name: &'a str,
    pub line: usize,
    pub col: usize,
}
