#[derive(Debug)]
pub enum ScanningErrorType {
    Generic,
    EOF,
    Indentation,
    Unidentified(char),
    UnclosedString,
}

#[derive(Debug)]
pub struct ScanningError {
    pub msg: String,
    pub line: u64,
    pub r#type: ScanningErrorType,
}

impl ScanningError {
    pub fn format(&self) -> String {
        format!("[{}] Scanning error: {}", self.line, self.msg)
    }

    pub fn from_error_type(value: ScanningErrorType, line: u64) -> Self {
        match value {
            ScanningErrorType::EOF => Self {
                msg: "EOF".to_owned(),
                r#type: ScanningErrorType::EOF,
                line,
            },
            ScanningErrorType::Generic => Self {
                msg: "Generic Error".to_owned(),
                r#type: ScanningErrorType::Generic,
                line,
            },
            ScanningErrorType::Indentation => Self {
                msg: "Indentation error".to_owned(),
                r#type: ScanningErrorType::Indentation,
                line,
            },
            ScanningErrorType::Unidentified(char) => Self {
                msg: format!("Unidentified character: {char}"),
                r#type: ScanningErrorType::Indentation,
                line,
            },
            ScanningErrorType::UnclosedString => Self {
                msg: "Unclosed string".to_owned(),
                r#type: ScanningErrorType::UnclosedString,
                line,
            },
        }
    }
}

impl std::fmt::Display for ScanningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scanning error [{}]: {}", self.line, self.msg)
    }
}
