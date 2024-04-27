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
    line: usize,
    pos: usize,
    msg: String,
}

impl SyntaxError {
    pub fn new(msg: String, line: usize, pos: usize) -> AstError {
        AstError::SyntaxError(Self { line, pos, msg })
    }

    pub fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.line).unwrap();
        format!("Line {} - {row}\n{}", self.line, self.msg)
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
