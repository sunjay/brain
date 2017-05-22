use std::fmt;
use std::error::Error;

use super::Rule;

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError {
    pub line: usize,
    pub col: usize,
    pub expected: Vec<Rule>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.expected.is_empty() {
            write!(f, "no token expected at line {} col {}", self.line, self.col)
        } else {
            write!(f, "expected token(s): {} at line {} col {}",
                self.expected.iter().map(|r| format!("{}", r)).collect::<Vec<String>>().join(", "),
                self.line, self.col)
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        if self.expected.is_empty() {
            "no tokens expected"
        } else {
            "expected tokens which were not found"
        }
    }
}
