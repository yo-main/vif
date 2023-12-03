pub enum ValueError {
    Generic(String),
    DivideByZero(String),
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic(s) => write!(f, "{s}"),
            Self::DivideByZero(s) => write!(f, "{s}"),
        }
    }
}

#[macro_export]
macro_rules! value_error {
    ($($arg:tt)*) => {{
        Err($crate::errors::ValueError::Generic(format!($($arg)*)))
    }}
}

#[macro_export]
macro_rules! divide_by_zero_error {
    ($($arg:tt)*) => {{
        Err($crate::errors::ValueError::DivideByZero(format!($($arg)*)))
    }}
}

pub enum ExecutionError {
    Generic(String),
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic(s) => write!(f, "{s}"),
        }
    }
}
