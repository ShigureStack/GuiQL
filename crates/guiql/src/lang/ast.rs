#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TokenLoc {
    pub starts_at: u32,
    pub len: u32,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenContent {
    NumberLiteral(String),
    StringLiteral(String),
    Identifier(String),
    BraceLeft,
    BraceRight,
    Create,
    Component,
    Const,
    Delete,
    Else,
    Enum,
    FromKeyword,
    For,
    Func,
    If,
    Int,
    Key,
    Let,
    Match,
    Number,
    StringKeyword,
    Tag,
    Type,
    With,
    Or,
    Pub,
}

impl TokenContent {
    pub fn from_str(word: &str) -> Option<Self> {
        match word {
            "create" => Some(Self::Create),
            "component" => Some(Self::Component),
            "const" => Some(Self::Const),
            "delete" => Some(Self::Delete),
            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "from" => Some(Self::FromKeyword),
            "for" => Some(Self::For),
            "func" => Some(Self::Func),
            "if" => Some(Self::If),
            "int" => Some(Self::Int),
            "key" => Some(Self::Key),
            "let" => Some(Self::Let),
            "match" => Some(Self::Match),
            "number" => Some(Self::Number),
            "string" => Some(Self::StringKeyword),
            "tag" => Some(Self::Tag),
            "type" => Some(Self::Type),
            "with" => Some(Self::With),
            "or" => Some(Self::Or),
            "pub" => Some(Self::Pub),
            _ => None,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '{' => Some(Self::BraceLeft),
            '}' => Some(Self::BraceRight),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub loc: TokenLoc,
    pub con: TokenContent,
}
