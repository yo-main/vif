use vif_scanner::ScanningError;
use vif_scanner::ScanningErrorType;

pub enum AstError {
    ScannerError(ScanningError),
    ParsingError(String),
    EOF,
}

impl From<ScanningError> for AstError {
    fn from(value: ScanningError) -> Self {
        match value.r#type {
            ScanningErrorType::EOF => AstError::EOF,
            _ => AstError::ScannerError(value),
        }
    }
}

impl std::fmt::Display for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScannerError(err) => write!(f, "{}", err),
            Self::ParsingError(s) => write!(f, "{}", s),
            Self::EOF => write!(f, "EOF"),
        }
    }
}

impl std::fmt::Debug for AstError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScannerError(err) => write!(f, "{}", err),
            Self::ParsingError(s) => write!(f, "{}", s),
            Self::EOF => write!(f, "EOF"),
        }
    }
}
