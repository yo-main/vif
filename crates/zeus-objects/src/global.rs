use crate::variable::Variable;

pub struct Global {
    storage: Vec<Variable>,
}

impl Global {
    pub fn new() -> Self {
        Global {
            storage: Vec::new(),
        }
    }

    pub fn push(&mut self, variable: Variable) {
        self.storage.push(variable)
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn get(&self, index: usize) -> &Variable {
        &self.storage[index]
    }

    pub fn find(&self, variable: &Variable) -> Option<usize> {
        self.storage.iter().position(|v| v == variable)
    }
}
