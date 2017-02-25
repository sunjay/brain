use super::Rule;

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub line: usize,
    pub col: usize,
    pub expected: Vec<Rule>,
}
