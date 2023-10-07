#[derive(Debug)]
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

#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub line: u64,
}

impl Token {
    pub fn new(r#type: TokenType, line: u64) -> Self {
        Token { r#type, line }
    }
}
