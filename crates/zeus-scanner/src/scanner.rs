use crate::error::ScanningError;
use crate::error::ScanningErrorType;
use crate::token::Token;
use crate::token::TokenType;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    next: Option<Token>,
    tokenizer: Tokenizer<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            next: None,
            tokenizer: Tokenizer::new(source),
        }
    }

    fn advance(&mut self) -> Result<(), ScanningError> {
        self.next = Some(self.tokenizer.scan()?);
        Ok(())
    }

    pub fn check(&mut self, token_type: &TokenType) -> bool {
        self.peek().is_ok_and(|t| &t.r#type == token_type)
    }

    pub fn peek(&mut self) -> Result<&Token, ScanningError> {
        if self.next.is_none() {
            self.advance()?;
        };

        return Ok(self.next.as_ref().unwrap());
    }

    pub fn scan(&mut self) -> Result<Token, ScanningError> {
        if self.next.is_some() {
            return Ok(self.next.take().unwrap());
        };

        self.tokenizer.scan()
    }

    pub fn get_line(&self) -> u64 {
        self.tokenizer.get_line()
    }

    pub fn is_at_line_start(&self) -> bool {
        self.next.is_none() && self.tokenizer.is_at_line_start()
    }
}

pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
    line: u64,
    line_start: bool,
    indent_stack: Vec<u8>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            line: 0,
            line_start: true,
            indent_stack: vec![0],
        }
    }

    fn get_line(&self) -> u64 {
        self.line
    }

    pub fn scan(&mut self) -> Result<Token, ScanningError> {
        let token = self.scan_token();
        log::debug!("Scanned token: {:?}", token);
        token
    }

    fn report_error(&mut self, error_type: ScanningErrorType) -> ScanningError {
        let err = ScanningError::from_error_type(error_type, self.line);
        log::error!("{}", err.format());
        err
    }

    fn is_at_line_start(&self) -> bool {
        self.line_start
    }

    fn scan_token(&mut self) -> Result<Token, ScanningError> {
        let token_type = if self.line_start {
            self.line += 1;
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
            }
            _ => (),
        };

        match token_type {
            TokenType::Ignore => self.scan_token(),
            TokenType::Comment(_) => self.scan_token(),
            t => Ok(Token::new(t, self.line)),
        }
    }

    fn advance(&mut self) -> Result<char, ScanningError> {
        let next = self.source.next();
        next.ok_or_else(|| self.report_error(ScanningErrorType::EOF))
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
                return Ok(TokenType::Dedent);
            } else if previous_stack > stack {
                println!("Coucou");
                self.line_start = true;
                self.line -= 1;
                return Ok(TokenType::Dedent);
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
            "not" => TokenType::Not,
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
            "assert" => TokenType::Assert,
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

#[cfg(test)]
mod tests {
    use super::Scanner;
    use super::TokenType;

    #[test]
    fn simple_string() {
        let string = "\"This is a simple string\"\n";
        let mut scanner = Scanner::new(string);

        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::String("This is a simple string".to_owned())
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
    }

    #[test]
    fn simple_number() {
        let string = "1\n";
        let mut scanner = Scanner::new(string);

        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Integer(1)
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
    }

    #[test]
    fn negative_number() {
        let string = "-1\n";
        let mut scanner = Scanner::new(string);

        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Minus
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Integer(1)
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
    }

    #[test]
    fn simple_float() {
        let string = "1.1\n";
        let mut scanner = Scanner::new(string);

        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Float(1.1)
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
    }

    #[test]
    fn float_without_decimals() {
        let string = "0.\n";
        let mut scanner = Scanner::new(string);

        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Float(0.)
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
    }

    #[test]
    fn simple_identifier() {
        let string = "cou\n";
        let mut scanner = Scanner::new(string);

        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Identifier("cou".to_owned())
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
    }

    #[test]
    fn keyword_var() {
        let string = "var\n";
        let mut scanner = Scanner::new(string);

        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Var
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
    }

    #[test]
    fn all_keywords() {
        let string = "not and or def class if else elif for while var const self return True False None @ break continue assert\n";
        let mut scanner = Scanner::new(string);

        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Not
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::And
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Or
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Def
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Class
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::If
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Else
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::ElIf
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::For
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::While
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Var
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Const
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Self_
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Return
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::True
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::False
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::None
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::At
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Break
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Continue
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Assert
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
    }

    #[test]
    fn variable_declaration() {
        let string = "var cou = \"coucou\"\n";
        let mut scanner = Scanner::new(string);

        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Var
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Identifier("cou".to_owned())
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Equal
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::String("coucou".to_owned())
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
    }

    #[test]
    fn simple_indentation() {
        let string = "True\n    True\n        True\n\t    True\n    True\nTrue\n";
        let mut scanner = Scanner::new(string);

        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::True
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Indent
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::True
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Indent
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::True
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::True
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Dedent
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::True
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Dedent
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::True
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
    }

    #[test]
    fn simple_comment() {
        let string = "True\n# this is a comment\nFalse";
        let mut scanner = Scanner::new(string);

        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::True
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::NewLine
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::False
        );
    }
}
