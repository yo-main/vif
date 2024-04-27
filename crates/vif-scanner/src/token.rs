use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // singles char
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftAccolade,
    RightAccolade,
    Comma,
    // Dot, no need ? Only for numbers ?
    DoubleDot,
    Minus,
    Plus,
    Modulo,
    Semicolon,
    Hash,
    Slash,
    Star,
    Equal,
    Greater,
    Less,
    NewLine,
    At,

    // multi chars
    EqualEqual,
    BangEqual,
    GreaterEqual,
    LessEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    // literals
    Identifier(String),
    String(String),
    Integer(i64),
    Float(f64),
    Comment(String),

    // keywords
    Not,
    And,
    Def,
    Class,
    ElIf,
    Else,
    If,
    Var,
    Mut,
    For,
    Or,
    None,
    True,
    False,
    Return,
    Self_,
    While,
    Break,
    Continue,
    Assert,

    // indents
    Indent,
    Dedent,

    // other
    IgnoreNewLine,
    Ignore,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, line: usize) -> Self {
        Token { r#type, line }
    }

    pub fn create(r#type: TokenType) -> Self {
        Token { r#type, line: 0 }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r#type)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "["),
            TokenType::RightBrace => write!(f, "]"),
            TokenType::LeftAccolade => write!(f, "{{"),
            TokenType::RightAccolade => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::DoubleDot => write!(f, ":"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Modulo => write!(f, "%"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Hash => write!(f, "#"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            TokenType::Equal => write!(f, "="),
            TokenType::Greater => write!(f, ">"),
            TokenType::Less => write!(f, "<"),
            TokenType::NewLine => write!(f, "\\n"),
            TokenType::IgnoreNewLine => write!(f, "\\n(-)"),
            TokenType::At => write!(f, "@"),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::PlusEqual => write!(f, "+="),
            TokenType::MinusEqual => write!(f, "-="),
            TokenType::StarEqual => write!(f, "*="),
            TokenType::SlashEqual => write!(f, "/="),
            TokenType::Identifier(v) => write!(f, "Identifier {}", v),
            TokenType::String(v) => write!(f, "String {}", v),
            TokenType::Integer(v) => write!(f, "Integer {}", v),
            TokenType::Float(v) => write!(f, "Float {}", v),
            TokenType::Comment(v) => write!(f, "Comment {}", v),
            TokenType::And => write!(f, "and"),
            TokenType::Def => write!(f, "def"),
            TokenType::Class => write!(f, "class"),
            TokenType::ElIf => write!(f, "elif"),
            TokenType::Else => write!(f, "else"),
            TokenType::If => write!(f, "if"),
            TokenType::Var => write!(f, "var"),
            TokenType::Mut => write!(f, "mut"),
            TokenType::For => write!(f, "for"),
            TokenType::Or => write!(f, "or"),
            TokenType::None => write!(f, "None"),
            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),
            TokenType::Return => write!(f, "return"),
            TokenType::Self_ => write!(f, "self"),
            TokenType::While => write!(f, "while"),
            TokenType::Indent => write!(f, "indent"),
            TokenType::Dedent => write!(f, "dedent"),
            TokenType::Ignore => write!(f, "ignore"),
            TokenType::Break => write!(f, "break"),
            TokenType::Continue => write!(f, "continue"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::Not => write!(f, "not"),
            TokenType::Assert => write!(f, "assert"),
        }
    }
}
