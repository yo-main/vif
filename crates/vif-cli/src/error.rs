use vif_ast::AstError;
use vif_typing::TypingError;

pub enum VifErrorType {
    CommandError,
    AstError,
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
            Self::CommandError => write!(f, "{}", "CommandError"),
            Self::AstError => write!(f, "{}", "AstError"),
        }
    }
}

impl std::fmt::Display for VifError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.r#type, self.msg)
    }
}

impl From<AstError> for VifError {
    fn from(value: AstError) -> Self {
        VifError::new(format!("{value}"), VifErrorType::AstError)
    }
}

impl From<TypingError> for VifError {
    fn from(value: TypingError) -> Self {
        VifError::new(format!("{value}"), VifErrorType::AstError)
    }
}
