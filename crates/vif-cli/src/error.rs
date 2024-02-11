pub enum VifErrorType {
    FileNotFound,
}

pub struct VifError {
    msg: String,
    r#type: VifErrorType,
}

impl VifError {
    pub fn new(msg: String, error_type: VifErrorType) -> Self {
        VifError {
            msg,
            r#type: error_type,
        }
    }
}

impl std::fmt::Display for VifErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound => write!(f, "{}", "FileNotFound"),
        }
    }
}

impl std::fmt::Display for VifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.r#type, self.msg)
    }
}
