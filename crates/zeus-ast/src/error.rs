use zeus_scanner::ScanningError;
use zeus_scanner::ScanningErrorType;

pub enum AstError {
    ScannerError(ScanningError),
    ParsingError(String),
    EOF,
    Generic,
}

impl From<ScanningError> for AstError {
    fn from(value: ScanningError) -> Self {
        match value.r#type {
            ScanningErrorType::EOF => AstError::EOF,
            _ => AstError::ScannerError(value),
        }
    }
}
