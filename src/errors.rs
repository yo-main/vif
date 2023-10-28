#[derive(Debug)]
pub enum ZeusErrorType {
    Generic,
    EOF,
    NoMoreTokens,
    Break,
    Continue,
    ParsingError(String),
    InterpreterError(String),
    UnassignedVariable(String),
}

#[derive(Debug)]
pub struct ZeusError {
    pub msg: String,
    pub line: Option<i64>,
    pub r#type: ZeusErrorType,
}

impl ZeusError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            line: None,
            r#type: ZeusErrorType::Generic,
        }
    }

    pub fn new_from_line(msg: &str, line: i64) -> Self {
        Self {
            msg: msg.to_owned(),
            line: Some(line),
            r#type: ZeusErrorType::Generic,
        }
    }

    pub fn format(&self) -> String {
        match self.line.as_ref() {
            Some(line) => format!("[{}] Error: {}", line, self.msg),
            _ => format!("Error: {}", self.msg),
        }
    }
}

impl From<ZeusErrorType> for ZeusError {
    fn from(value: ZeusErrorType) -> Self {
        match value {
            ZeusErrorType::EOF => ZeusError {
                msg: "EOF".to_owned(),
                line: None,
                r#type: ZeusErrorType::EOF,
            },
            ZeusErrorType::Generic => ZeusError {
                msg: "Generic Error".to_owned(),
                line: None,
                r#type: ZeusErrorType::Generic,
            },
            ZeusErrorType::NoMoreTokens => ZeusError {
                msg: "No More Tokens".to_owned(),
                line: None,
                r#type: ZeusErrorType::NoMoreTokens,
            },
            ZeusErrorType::ParsingError(_) => ZeusError {
                msg: "Parsing error".to_owned(),
                line: None,
                r#type: value,
            },
            ZeusErrorType::InterpreterError(_) => ZeusError {
                msg: "Interpreter error".to_owned(),
                line: None,
                r#type: value,
            },
            ZeusErrorType::UnassignedVariable(_) => ZeusError {
                msg: "Unassigned varible".to_owned(),
                line: None,
                r#type: value,
            },
            ZeusErrorType::Break => ZeusError {
                msg: "Break".to_owned(),
                line: None,
                r#type: value,
            },
            ZeusErrorType::Continue => ZeusError {
                msg: "Continue".to_owned(),
                line: None,
                r#type: value,
            },
        }
    }
}
