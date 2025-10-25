#[derive(Debug, Clone, PartialEq)]
pub enum CharType {
    Number(u8),
    Letter(u8),
}

impl CharType {
    pub fn as_char(&self) -> char {
        match self {
            CharType::Number(n) => *n as char,
            CharType::Letter(l) => *l as char,
        }
    }
}
