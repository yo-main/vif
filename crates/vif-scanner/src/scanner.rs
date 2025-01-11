use crate::error::EOFError;
use crate::error::IndentationError;
use crate::error::ScannerError;
use crate::error::UnclosedString;
use crate::error::UnidentifiedError;
use crate::token::Token;
use crate::token::TokenType;
use std::iter::Peekable;
use std::str::Chars;
use vif_loader::log;
use vif_objects::span::Span;

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

    fn advance(&mut self) -> Result<(), ScannerError> {
        self.next = Some(self.tokenizer.scan()?);
        Ok(())
    }

    pub fn check(&mut self, token_type: &TokenType) -> bool {
        self.peek().is_ok_and(|t| &t.r#type == token_type)
    }

    pub fn peek(&mut self) -> Result<&Token, ScannerError> {
        if self.next.is_none() {
            self.advance()?;
        };

        return Ok(self.next.as_ref().unwrap());
    }

    pub fn scan(&mut self) -> Result<Token, ScannerError> {
        if self.next.is_some() {
            return Ok(self.next.take().unwrap());
        };

        self.tokenizer.scan()
    }

    pub fn get_span(&self) -> &Span {
        self.tokenizer.get_position()
    }

    pub fn is_at_line_start(&self) -> bool {
        self.next.is_none() && self.tokenizer.is_at_line_start()
    }
}

pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
    span: Span,
    line_start: bool,
    indent_stack: Vec<u8>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            span: Span::new(0, 0),
            line_start: true,
            indent_stack: Vec::new(),
        }
    }

    fn get_position(&self) -> &Span {
        &self.span
    }

    pub fn scan(&mut self) -> Result<Token, ScannerError> {
        let token = self.scan_token();
        log::debug!("Scanned token: {:?}", token);
        token
    }

    fn is_at_line_start(&self) -> bool {
        self.line_start
    }

    fn scan_token(&mut self) -> Result<Token, ScannerError> {
        let token_type = if self.line_start {
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
                    false => return Err(UnidentifiedError::new(self.span.clone(), "!".to_owned())),
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
                c => return Err(UnidentifiedError::new(self.span.clone(), String::from(c))),
            }
        };

        match token_type {
            TokenType::NewLine => {
                self.line_start = true;
            }
            TokenType::IgnoreNewLine => {
                self.line_start = true;
            }
            _ => (),
        };

        match token_type {
            TokenType::Ignore => self.scan_token(),
            TokenType::IgnoreNewLine => self.scan_token(),
            TokenType::Comment(_) => self.scan_token(),
            t => Ok(Token::new(t, self.get_position().get_line())),
        }
    }

    fn advance(&mut self) -> Result<char, ScannerError> {
        self.source
            .next()
            .and_then(|r| {
                self.span.incr_index();
                Some(r)
            })
            .ok_or_else(|| EOFError::new(self.span.clone()))
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

    fn parse_indentation(&mut self) -> Result<TokenType, ScannerError> {
        let mut stack = 0;
        self.span.new_line();
        self.line_start = false;

        loop {
            match self.peek() {
                ' ' => {
                    self.advance().unwrap();
                    stack += 1;
                }
                '\t' => {
                    self.advance().unwrap();
                    stack += 4;
                }
                '\n' => {
                    self.advance().unwrap();
                    return Ok(TokenType::IgnoreNewLine);
                }
                '\0' => {
                    return Ok(TokenType::EOF);
                }
                _ => break,
            }
        }

        if self.indent_stack.is_empty() {
            self.indent_stack.push(stack);
        }
        let current_stack = *self.indent_stack.last().unwrap();

        log::debug!("Scanning indentation: {} {}", stack, current_stack);

        let token = if stack == current_stack {
            TokenType::Ignore
        } else if stack > current_stack {
            self.indent_stack.push(stack);
            TokenType::Indent
        } else {
            self.indent_stack.pop().unwrap();
            let previous_stack = *self.indent_stack.last().unwrap();
            if stack == previous_stack {
                TokenType::Dedent
            } else if previous_stack > stack {
                // we need to return every dedent singely, even when consecutive
                // so we return here but we decrease the line by 1 as it'll be incr back next iteration
                self.span.decr_line();
                self.line_start = true;
                TokenType::Dedent
            } else {
                return Err(IndentationError::new(self.span.clone()));
            }
        };

        Ok(token)
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
            TokenType::ValueFloat(str.parse::<f64>().unwrap())
        } else {
            TokenType::ValueInteger(str.parse::<i64>().unwrap())
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
            "mut" => TokenType::Mut,
            "self" => TokenType::Self_,
            "return" => TokenType::Return,
            "True" => TokenType::True,
            "False" => TokenType::False,
            "None" => TokenType::None,
            "int" => TokenType::Int,
            "str" => TokenType::Str,
            "float" => TokenType::Float,
            "bool" => TokenType::Bool,
            "@" => TokenType::At,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "assert" => TokenType::Assert,
            _ => TokenType::ValueIdentifier(str),
        }
    }

    fn parse_string(&mut self) -> Result<TokenType, ScannerError> {
        let mut str = String::new();
        loop {
            match self.peek() {
                &'"' => {
                    self.advance().unwrap();
                    break;
                }
                &'\0' => {
                    return Err(UnclosedString::new(self.span.clone()));
                }
                &'\n' => {
                    return Err(UnclosedString::new(self.span.clone()));
                }
                _ => str.push(self.advance().unwrap()),
            };
        }
        Ok(TokenType::ValueString(str))
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
            TokenType::ValueString("This is a simple string".to_owned())
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
            TokenType::ValueInteger(1)
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
            TokenType::ValueInteger(1)
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
            TokenType::ValueFloat(1.1)
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
            TokenType::ValueFloat(0.)
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
            TokenType::ValueIdentifier("cou".to_owned())
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
        let string = "not and or def class if else elif for while var mut self return True False None @ break continue assert\n";
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
            TokenType::Mut
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
            TokenType::ValueIdentifier("cou".to_owned())
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::Equal
        );
        assert_eq!(
            scanner.tokenizer.scan_token().unwrap().r#type,
            TokenType::ValueString("coucou".to_owned())
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
