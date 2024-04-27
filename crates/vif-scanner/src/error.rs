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
    line: usize,
    pos: usize,
}

impl IndentationError {
    pub fn new(line: usize, pos: usize) -> ScannerError {
        ScannerError::Indentation(Self { line, pos })
    }

    pub fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.line).unwrap();
        format!("Line {} - {row}\nIndentation error", self.line)
    }
}

#[derive(Debug)]
pub struct EOFError {
    line: usize,
    pos: usize,
}

impl EOFError {
    pub fn new(line: usize, pos: usize) -> ScannerError {
        ScannerError::EOF(Self { line, pos })
    }

    pub fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.line).unwrap();
        format!("Line {} - {row}\nEOF", self.line)
    }
}

#[derive(Debug)]
pub struct UnidentifiedError {
    line: usize,
    pos: usize,
    value: String,
}

impl UnidentifiedError {
    pub fn new(line: usize, pos: usize, value: String) -> ScannerError {
        ScannerError::Unidentified(Self { line, pos, value })
    }

    pub fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.line).unwrap();
        format!(
            "Line {} - {row}\nUndidentified characters: {}",
            self.line, self.value
        )
    }
}

#[derive(Debug)]
pub struct UnclosedString {
    line: usize,
    pos: usize,
}

impl UnclosedString {
    pub fn new(line: usize, pos: usize) -> ScannerError {
        ScannerError::UnclosedString(Self { line, pos })
    }

    pub fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.line).unwrap();
        format!("Line {} - {row}\nString is not closed", self.line)
    }
}
