use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
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
    Let,
    Const,
    For,
    Or,
    None,
    True,
    False,
    Return,
    Self_,
    While,

    // indents
    Indent,
    Dedent,

    // other
    Ignore,
    EOF,
}

pub struct Token {
    pub r#type: TokenType,
    pub line: u64,
}

impl Token {
    pub fn new(r#type: TokenType, line: u64) -> Self {
        Token { r#type, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r#type)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let int_to_string = match self {
            TokenType::Integer(v) => v.to_string(),
            TokenType::Float(v) => v.to_string(),
            _ => String::new(),
        };
        write!(
            f,
            "{}",
            match self {
                TokenType::LeftParen => "(",
                TokenType::RightParen => ")",
                TokenType::LeftBrace => "[",
                TokenType::RightBrace => "]",
                TokenType::LeftAccolade => "{",
                TokenType::RightAccolade => "}",
                TokenType::Comma => ",",
                TokenType::DoubleDot => ":",
                TokenType::Minus => "-",
                TokenType::Plus => "+",
                TokenType::Modulo => "%",
                TokenType::Semicolon => ";",
                TokenType::Hash => "#",
                TokenType::Slash => "/",
                TokenType::Star => "*",
                TokenType::Equal => "=",
                TokenType::Greater => ">",
                TokenType::Less => "<",
                TokenType::Bang => "!",
                TokenType::NewLine => "\\n",
                TokenType::At => "@",
                TokenType::EqualEqual => "==",
                TokenType::BangEqual => "!=",
                TokenType::GreaterEqual => ">=",
                TokenType::LessEqual => "<=",
                TokenType::PlusEqual => "+=",
                TokenType::MinusEqual => "-=",
                TokenType::StarEqual => "*=",
                TokenType::SlashEqual => "/=",
                TokenType::Identifier(v) => v,
                TokenType::String(v) => v,
                TokenType::Integer(v) => &int_to_string,
                TokenType::Float(v) => &int_to_string,
                TokenType::Comment(v) => v,
                TokenType::And => "and",
                TokenType::Def => "def",
                TokenType::Class => "class",
                TokenType::ElIf => "elif",
                TokenType::Else => "else",
                TokenType::If => "if",
                TokenType::Let => "let",
                TokenType::Const => "const",
                TokenType::For => "for",
                TokenType::Or => "or",
                TokenType::None => "None",
                TokenType::True => "True",
                TokenType::False => "False",
                TokenType::Return => "return",
                TokenType::Self_ => "self",
                TokenType::While => "while",
                TokenType::Indent => "indent",
                TokenType::Dedent => "dedant",
                TokenType::Ignore => "ignore",
                TokenType::EOF => "EOF",
            }
        )
    }
}
