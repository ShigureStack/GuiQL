#[derive(PartialEq, Clone, Debug)]
pub enum ItemContent {
    ComponentDeclaration,
    Query,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct ItemLoc {
    pub starts_at: u32,
    pub len: u32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Item {
    pub con: ItemContent,
    pub loc: ItemLoc,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TokenLoc {
    pub starts_at: u32,
    pub len: u32,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenContent {
    Element(String),
    Identifier(String),
    NumberLiteral(String),
    StringLiteral(String),
    BraceLeft,
    BraceRight,
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
    Insert,
    Key,
    Let,
    Match,
    New,
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
            "insert" => Some(Self::Insert),
            "key" => Some(Self::Key),
            "let" => Some(Self::Let),
            "match" => Some(Self::Match),
            "new" => Some(Self::New),
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
