use crate::error::ScanningError;
use crate::error::ScanningErrorType;
use crate::token::Token;
use crate::token::TokenType;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    line: u64,
    line_start: bool,
    indent_stack: Vec<u8>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            line: 1,
            line_start: true,
            indent_stack: vec![0],
        }
    }

    pub fn get_line(&self) -> u64 {
        self.line
    }

    pub fn scan(&mut self) -> Result<Token, ScanningError> {
        match self.scan_token() {
            Err(e) => match e.r#type {
                ScanningErrorType::EOF => return Ok(Token::new(TokenType::EOF, self.line)),
                _ => return Err(e),
            },
            Ok(t) => Ok(t),
        }
    }

    fn report_error(&mut self, error_type: ScanningErrorType) -> ScanningError {
        let err = ScanningError::from_error_type(error_type, self.line);
        log::error!("{}", err.format());
        err
    }

    fn scan_token(&mut self) -> Result<Token, ScanningError> {
        let token_type = if self.line_start {
            self.line_start = false;
            self.parse_indentation()?
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
                '-' => match *self.peek() {
                    '=' => {
                        self.advance().unwrap();
                        TokenType::MinusEqual
                    }
                    _ => TokenType::Minus,
                },
                '/' => match self.r#match('=') {
                    true => TokenType::SlashEqual,
                    false => TokenType::Slash,
                },
                '*' => match self.r#match('=') {
                    true => TokenType::StarEqual,
                    false => TokenType::Star,
                },
                d if d.is_digit(10) => self.parse_number(d),
                c if c.is_ascii_alphabetic() => self.parse_identifier(c),
                '"' => self.parse_string()?,
                ' ' => TokenType::Ignore,
                '\t' => TokenType::Ignore,
                '\r' => TokenType::Ignore,
                '\n' => TokenType::NewLine,
                c => return Err(self.report_error(ScanningErrorType::Unidentified(c))),
            }
        };

        match token_type {
            TokenType::NewLine => {
                self.line_start = true;
                self.line += 1;
            }
            _ => self.line_start = false,
        };
        log::debug!("Scanned token: {:?}, {}", token_type, self.line_start);

        match token_type {
            TokenType::Ignore => self.scan_token(),
            TokenType::Comment(_) => self.scan_token(),
            t => Ok(Token::new(t, self.line)),
        }
    }

    fn advance(&mut self) -> Result<char, ScanningError> {
        self.source
            .next()
            .ok_or(self.report_error(ScanningErrorType::EOF))
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

    fn parse_indentation(&mut self) -> Result<TokenType, ScanningError> {
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
                &'\n' => return Ok(TokenType::Ignore),
                _ => break,
            }
        }

        log::debug!("Scanning indentation: {} {}", stack, current_stack);
        if stack == current_stack {
            Ok(TokenType::Ignore)
        } else if stack > current_stack {
            self.indent_stack.push(stack);
            Ok(TokenType::Indent)
        } else {
            self.indent_stack.pop().unwrap();
            let previous_stack = *self.indent_stack.last().unwrap();
            if stack == previous_stack {
                Ok(TokenType::Dedent)
            } else {
                return Err(self.report_error(ScanningErrorType::Indentation));
            }
        }
    }

    fn parse_number(&mut self, initial: char) -> TokenType {
        let mut str = String::from(initial);

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

    fn parse_identifier(&mut self, initial: char) -> TokenType {
        let mut str = String::from(initial);
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
            "var" => TokenType::Var,
            "const" => TokenType::Const,
            "self" => TokenType::Self_,
            "return" => TokenType::Return,
            "True" => TokenType::True,
            "False" => TokenType::False,
            "None" => TokenType::None,
            "@" => TokenType::At,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            _ => TokenType::Identifier(str),
        }
    }

    fn parse_string(&mut self) -> Result<TokenType, ScanningError> {
        let mut str = String::new();
        loop {
            match self.peek() {
                &'"' => {
                    self.advance().unwrap();
                    break;
                }
                &'\0' => {
                    return Err(self.report_error(ScanningErrorType::UnclosedString));
                }
                &'\n' => {
                    return Err(self.report_error(ScanningErrorType::UnclosedString));
                }
                _ => str.push(self.advance().unwrap()),
            };
        }
        Ok(TokenType::String(str))
    }
}
