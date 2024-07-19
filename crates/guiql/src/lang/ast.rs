#[derive(PartialEq, Clone, Debug)]
pub struct TokenLoc {
    pub starts_at: u32,
    pub len: u32,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenContent {
    NumberLiteral(String),
    StringLiteral(String),
    Reserved,
}

impl TokenContent {
    pub fn from_str(word: &str) -> Option<Self> {
        match word {
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub loc: TokenLoc,
    pub con: TokenContent,
}
