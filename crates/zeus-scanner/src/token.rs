use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

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
    Bang,
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
    And,
    Def,
    Class,
    ElIf,
    Else,
    If,
    Var,
    Const,
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

    // indents
    Indent,
    Dedent,

    // other
    Ignore,
    EOF,
}

impl Hash for TokenType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::LeftParen => state.write_u8(1),
            Self::RightParen => state.write_u8(2),
            Self::LeftBrace => state.write_u8(3),
            Self::RightBrace => state.write_u8(4),
            Self::LeftAccolade => state.write_u8(5),
            Self::RightAccolade => state.write_u8(6),
            Self::Comma => state.write_u8(7),
            Self::DoubleDot => state.write_u8(8),
            Self::Minus => state.write_u8(9),
            Self::Plus => state.write_u8(10),
            Self::Modulo => state.write_u8(12),
            Self::Semicolon => state.write_u8(13),
            Self::Hash => state.write_u8(14),
            Self::Slash => state.write_u8(15),
            Self::Star => state.write_u8(16),
            Self::Equal => state.write_u8(17),
            Self::Greater => state.write_u8(18),
            Self::Less => state.write_u8(19),
            Self::Bang => state.write_u8(20),
            Self::NewLine => state.write_u8(21),
            Self::At => state.write_u8(22),
            Self::EqualEqual => state.write_u8(23),
            Self::BangEqual => state.write_u8(24),
            Self::GreaterEqual => state.write_u8(25),
            Self::LessEqual => state.write_u8(26),
            Self::PlusEqual => state.write_u8(27),
            Self::MinusEqual => state.write_u8(28),
            Self::StarEqual => state.write_u8(29),
            Self::SlashEqual => state.write_u8(30),
            Self::Identifier(v) => state.write_u8(31),
            Self::String(v) => state.write_u8(32),
            Self::Integer(v) => state.write_u8(33),
            Self::Float(v) => state.write_u8(34),
            Self::Comment(v) => state.write_u8(35),
            Self::And => state.write_u8(36),
            Self::Def => state.write_u8(37),
            Self::Class => state.write_u8(38),
            Self::ElIf => state.write_u8(39),
            Self::Else => state.write_u8(40),
            Self::If => state.write_u8(41),
            Self::Var => state.write_u8(42),
            Self::Const => state.write_u8(43),
            Self::For => state.write_u8(44),
            Self::Or => state.write_u8(45),
            Self::None => state.write_u8(46),
            Self::True => state.write_u8(47),
            Self::False => state.write_u8(48),
            Self::Return => state.write_u8(49),
            Self::Self_ => state.write_u8(50),
            Self::While => state.write_u8(51),
            Self::Indent => state.write_u8(52),
            Self::Dedent => state.write_u8(53),
            Self::Ignore => state.write_u8(54),
            Self::Break => state.write_u8(55),
            Self::Continue => state.write_u8(56),
            Self::EOF => state.write_u8(57),
        }
    }
}

impl Eq for TokenType {}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub line: u64,
}

impl Token {
    pub fn new(r#type: TokenType, line: u64) -> Self {
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
            TokenType::Bang => write!(f, "!"),
            TokenType::NewLine => write!(f, "\\n"),
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
            TokenType::Const => write!(f, "const"),
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
        }
    }
}
