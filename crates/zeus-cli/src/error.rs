pub enum ZeusErrorType {
    FileNotFound,
}

pub struct ZeusError {
    msg: String,
    r#type: ZeusErrorType,
}

impl ZeusError {
    pub fn new(msg: String, error_type: ZeusErrorType) -> Self {
        ZeusError {
            msg,
            r#type: error_type,
        }
    }
}

impl std::fmt::Display for ZeusErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound => write!(f, "{}", "FileNotFound"),
        }
    }
}

impl std::fmt::Display for ZeusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.r#type, self.msg)
    }
}
