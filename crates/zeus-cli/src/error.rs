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

impl std::fmt::Display for ZeusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
