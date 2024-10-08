#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ASTItemLoc {
    pub starts_at: u32,
    pub len: u32,
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct CreateQuery {
    pub elm_name: String,
    pub view: ViewRoot,
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct ReplaceQuery {}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
pub struct ViewRoot {}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Module {
    pub name: String,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TokenLoc {
    pub starts_at: u32,
    pub len: u32,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum TokenContent {
    Anchor(String),
    Identifier(String),
    NumberLiteral(String),
    StringLiteral(String),
    BraceLeft,
    BraceRight,
    Create,
    Delete,
    Else,
    Enum,
    FromKeyword,
    For,
    If,
    Int,
    Insert,
    Key,
    Let,
    New,
    Number,
    StringKeyword,
    Tag,
    Type,
    With,
    Or,
    Pub,
    Replace,
}

impl TokenContent {
    pub fn from_str(word: &str) -> Option<Self> {
        match word.to_lowercase().as_str() {
            "create" => Some(Self::Create),
            "delete" => Some(Self::Delete),
            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "from" => Some(Self::FromKeyword),
            "for" => Some(Self::For),
            "if" => Some(Self::If),
            "int" => Some(Self::Int),
            "insert" => Some(Self::Insert),
            "key" => Some(Self::Key),
            "let" => Some(Self::Let),
            "new" => Some(Self::New),
            "number" => Some(Self::Number),
            "string" => Some(Self::StringKeyword),
            "tag" => Some(Self::Tag),
            "type" => Some(Self::Type),
            "with" => Some(Self::With),
            "or" => Some(Self::Or),
            "pub" => Some(Self::Pub),
            "replace" => Some(Self::Replace),
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

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Token {
    pub loc: TokenLoc,
    pub con: TokenContent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeLoc {
    start: u32,
    end: u32,
}

pub trait Node {
    fn loc(&mut self) -> NodeLoc;
    fn set_loc(&mut self, loc: NodeLoc);
}

pub trait HasChildren {
    fn append_child<T: Node>(&mut self, node: T);
    fn children(&mut self) -> Vec<Box<dyn Node>>;
}

pub trait Query: Node {}

pub trait Scope: Node + HasChildren {}
