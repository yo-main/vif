#[derive(Debug)]
pub struct ZeusError {
    pub msg: String,
    pub line: Option<i64>,
}

impl ZeusError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            line: None,
        }
    }

    pub fn new_from_line(msg: &str, line: i64) -> Self {
        Self {
            msg: msg.to_owned(),
            line: Some(line),
        }
    }

    pub fn format(&self) -> String {
        match self.line.as_ref() {
            Some(line) => format!("[{}] Error: {}", line, self.msg),
            _ => format!("Error: {}", self.msg),
        }
    }
}
