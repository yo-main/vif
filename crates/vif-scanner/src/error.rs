use crate::scanner::Span;

#[derive(Debug)]
pub enum ScannerError {
    UnclosedString(UnclosedString),
    Indentation(IndentationError),
    EOF(EOFError),
    Unidentified(UnidentifiedError),
}

impl ScannerError {
    pub fn format(&self, content: &str) -> String {
        match self {
            Self::EOF(e) => e.format(content),
            Self::UnclosedString(e) => e.format(content),
            Self::Indentation(e) => e.format(content),
            Self::Unidentified(e) => e.format(content),
        }
    }
}

#[derive(Debug)]
pub struct IndentationError {
    span: Span,
}

impl IndentationError {
    pub fn new(span: Span) -> ScannerError {
        ScannerError::Indentation(Self { span })
    }

    pub fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.span.line).unwrap();
        format!("Line {} - {row}\nIndentation error", self.span.line)
    }
}

#[derive(Debug)]
pub struct EOFError {
    span: Span,
}

impl EOFError {
    pub fn new(span: Span) -> ScannerError {
        ScannerError::EOF(Self { span })
    }

    pub fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.span.line).unwrap();
        format!("Line {} - {row}\nEOF", self.span.line)
    }
}

#[derive(Debug)]
pub struct UnidentifiedError {
    span: Span,
    value: String,
}

impl UnidentifiedError {
    pub fn new(span: Span, value: String) -> ScannerError {
        ScannerError::Unidentified(Self { span, value })
    }

    pub fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.span.line).unwrap();
        format!(
            "Line {} - {row}\nUndidentified characters: {}",
            self.span.line, self.value
        )
    }
}

#[derive(Debug)]
pub struct UnclosedString {
    span: Span,
}

impl UnclosedString {
    pub fn new(span: Span) -> ScannerError {
        ScannerError::UnclosedString(Self { span })
    }

    pub fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.span.line).unwrap();
        format!("Line {} - {row}\nString is not closed", self.span.line)
    }
}
