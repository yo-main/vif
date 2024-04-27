use vif_scanner::ScannerError;

pub enum AstError {
    ScannerError(ScannerError),
    ParsingError(String),
    EOF,
}

impl From<ScannerError> for AstError {
    fn from(value: ScannerError) -> Self {
        match value {
            ScannerError::EOF(_) => AstError::EOF,
            _ => AstError::ScannerError(value),
        }
    }
}

impl std::fmt::Display for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScannerError(err) => write!(f, "{}", err.format("")),
            Self::ParsingError(s) => write!(f, "{}", s),
            Self::EOF => write!(f, "EOF"),
        }
    }
}

impl std::fmt::Debug for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScannerError(err) => write!(f, "{}", err.format("")),
            Self::ParsingError(s) => write!(f, "{}", s),
            Self::EOF => write!(f, "EOF"),
        }
    }
}
