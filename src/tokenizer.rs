use crate::errors::ZeusError;
use crate::errors::ZeusErrorType;
use crate::tokens::Token;
use crate::tokens::TokenType;
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>,
    line: u64,
    line_start: bool,
    indent_stack: Vec<u8>,
    pub has_error: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Tokenizer {
            source: source.chars().peekable(),
            tokens: Vec::new(),
            line: 1,
            line_start: true,
            indent_stack: vec![0],
            has_error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        loop {
            if self.scan_token().is_err() {
                break;
            }
        }

        self.tokens.push(Token {
            r#type: TokenType::EOF,
            line: self.line,
        });

        &self.tokens
    }

    fn report_error(&mut self, msg: &str) {
        tracing::error!(msg);
        self.has_error = true;
    }

    fn scan_token(&mut self) -> Result<(), ZeusError> {
        let token_type = if self.line_start {
            self.line_start = false;
            self.parse_indentation()
        } else {
            match self.advance()? {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '[' => TokenType::LeftBrace,
                ']' => TokenType::RightBrace,
                '{' => TokenType::LeftAccolade,
                '}' => TokenType::RightAccolade,
                '%' => TokenType::Modulo,
                ',' => TokenType::Comma,
                ':' => TokenType::DoubleDot,
                ';' => TokenType::Semicolon,
                '@' => TokenType::At,
                '#' => {
                    let mut str = String::new();
                    while !vec!['\n', '\0'].contains(self.peek()) {
                        str.push(self.advance().unwrap());
                    }
                    TokenType::Comment(str)
                }
                '!' => match self.r#match('=') {
                    true => TokenType::BangEqual,
                    false => TokenType::Bang,
                },
                '=' => match self.r#match('=') {
                    true => TokenType::EqualEqual,
                    false => TokenType::Equal,
                },
                '<' => match self.r#match('=') {
                    true => TokenType::LessEqual,
                    false => TokenType::Less,
                },
                '>' => match self.r#match('=') {
                    true => TokenType::GreaterEqual,
                    false => TokenType::Greater,
                },
                '+' => match self.r#match('=') {
                    true => TokenType::PlusEqual,
                    false => TokenType::Plus,
                },
                '-' => match self.r#match('=') {
                    true => TokenType::MinusEqual,
                    false => TokenType::Minus,
                },
                '/' => match self.r#match('=') {
                    true => TokenType::SlashEqual,
                    false => TokenType::Slash,
                },
                '*' => match self.r#match('=') {
                    true => TokenType::StarEqual,
                    false => TokenType::Star,
                },
                d if d.is_digit(10) => {
                    let mut str = String::from(d);

                    loop {
                        let c = self.peek();
                        if !c.is_digit(10) {
                            break;
                        }
                        str.push(self.advance().unwrap());
                    }
                    if self.peek() == &'.' {
                        str.push(self.advance().unwrap());
                        loop {
                            let c = self.peek();
                            if !c.is_digit(10) {
                                break;
                            }
                            str.push(self.advance().unwrap());
                        }
                    }

                    if str.contains('.') {
                        TokenType::Float(str.parse::<f64>().unwrap())
                    } else {
                        TokenType::Integer(str.parse::<i64>().unwrap())
                    }
                }
                c if c.is_ascii_alphabetic() => {
                    let mut str = String::from(c);
                    loop {
                        let c = self.peek();
                        if c.is_ascii_alphanumeric() || c == &'_' {
                            str.push(self.advance().unwrap());
                        } else {
                            break;
                        }
                    }

                    match str.as_str() {
                        "and" => TokenType::And,
                        "or" => TokenType::Or,
                        "def" => TokenType::Def,
                        "class" => TokenType::Class,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "elif" => TokenType::ElIf,
                        "for" => TokenType::For,
                        "while" => TokenType::While,
                        "let" => TokenType::Let,
                        "const" => TokenType::Const,
                        "self" => TokenType::Self_,
                        "return" => TokenType::Return,
                        _ => TokenType::Identifier(str),
                    }
                }
                '"' => {
                    let mut str = String::new();
                    loop {
                        match self.peek() {
                            &'"' => {
                                self.advance().unwrap();
                                break;
                            }
                            &'\0' => {
                                self.report_error("Unclosed string: EOF");
                                return Ok(());
                            }
                            &'\n' => {
                                self.report_error("Unclosed string: new line");
                                return Ok(());
                            }
                            _ => str.push(self.advance()?),
                        };
                    }
                    TokenType::String(str)
                }
                ' ' => TokenType::Ignore,
                '\t' => TokenType::Ignore,
                '\r' => TokenType::Ignore,
                '\n' => TokenType::NewLine,
                _ => {
                    self.report_error("Unidentified character");
                    TokenType::Ignore
                }
            }
        };

        match token_type {
            TokenType::NewLine => {
                self.line_start = true;
                self.line += 1;
            }
            _ => self.line_start = false,
        };
        // println!("{:?}, {}", token_type, self.line_start);

        match token_type {
            TokenType::Comment(_) => (),
            TokenType::Ignore => (),
            _ => self.add_token(token_type),
        };

        Ok(())
    }

    fn advance(&mut self) -> Result<char, ZeusErrorType> {
        self.source.next().ok_or(ZeusErrorType::EOF)
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.source.peek() == Some(&expected) {
            self.advance().unwrap();
            return true;
        }
        return false;
    }

    fn peek(&mut self) -> &char {
        self.source.peek().unwrap_or(&'\0')
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, self.line));
    }

    fn parse_indentation(&mut self) -> TokenType {
        let mut stack = 0;
        let current_stack = *self.indent_stack.last().unwrap();

        loop {
            match self.peek() {
                &' ' => {
                    self.advance().unwrap();
                    stack += 1;
                }
                &'\t' => {
                    self.advance().unwrap();
                    stack += 4;
                }
                _ => break,
            }
        }

        // println!("{} {}", stack, current_stack);
        if stack == current_stack {
            TokenType::Ignore
        } else if stack > current_stack {
            self.indent_stack.push(stack);
            TokenType::Indent
        } else {
            self.indent_stack.pop().unwrap();
            let previous_stack = *self.indent_stack.last().unwrap();
            if stack == previous_stack {
                TokenType::Dedent
            } else {
                self.report_error("Indentation error");
                TokenType::Ignore
            }
        }
    }
}
