#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    line: usize,
    index: usize,
}

impl Span {
    pub fn new(line: usize, index: usize) -> Self {
        Span { line, index }
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.index = 0;
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn incr_index(&mut self) {
        self.index += 1
    }

    pub fn decr_line(&mut self) {
        self.line -= 1
    }

    pub fn format(&self, content: &str, msg: &str) -> String {
        let row = content.split('\n').nth(self.line - 1).unwrap();
        format!("Line {} - {row}\n{msg}", self.line)
    }
}
