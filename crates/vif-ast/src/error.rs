use vif_objects::span::Span;
use vif_scanner::ScannerError;

#[derive(Debug)]
pub enum AstError {
    ScannerError(ScannerError),
    SyntaxError(SyntaxError),
    EOF,
}

impl AstError {
    pub fn format(&self, content: &str) -> String {
        match self {
            Self::ScannerError(e) => e.format(content),
            Self::SyntaxError(e) => e.format(content),
            Self::EOF => "EOF".to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct SyntaxError {
    span: Span,
    msg: String,
}

impl SyntaxError {
    pub fn new(msg: String, span: Span) -> AstError {
        AstError::SyntaxError(Self { msg, span })
    }

    pub fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.span.get_line() - 1).unwrap();
        format!("Line {} - {row}\n{}", self.span.get_line(), self.msg)
    }
}

impl From<ScannerError> for AstError {
    fn from(value: ScannerError) -> Self {
        match value {
            ScannerError::EOF(_) => AstError::EOF,
            _ => AstError::ScannerError(value),
        }
    }
}
