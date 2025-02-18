use vif_objects::ast::Typing;

pub struct References {
    references: Vec<Reference>,
}

pub enum Reference {
    Variable(VariableReference),
    Function(FunctionReference),
}

#[derive(Debug, Clone)]
pub struct VariableReference {
    pub name: String,
    pub typing: Typing,
}

pub struct FunctionReference {
    pub name: String,
    pub typing: Typing,
    // pub parameters: Vec<VariableReference>,
}

impl std::cmp::PartialEq for VariableReference {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.typing.mutable == other.typing.mutable
    }
}

impl VariableReference {
    pub fn new(name: String, typing: Typing) -> Self {
        Self { name, typing }
    }
}

impl Reference {
    pub fn new_variable(name: String, typing: Typing) -> Self {
        Self::Variable(VariableReference { name, typing })
    }

    pub fn new_function(name: String, typing: Typing) -> Self {
        Self::Function(FunctionReference { name, typing })
    }
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable(v) => write!(f, "var {}", v.name),
            Self::Function(v) => write!(f, "func {}", v.name),
        }
    }
}

impl std::fmt::Debug for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable(v) => write!(f, "var {}", v.name),
            Self::Function(v) => write!(f, "func {}", v.name),
        }
    }
}

impl std::fmt::Display for References {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.references
                .iter()
                .map(|r| format!("{r}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl References {
    pub fn new() -> Self {
        References {
            references: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.references.len()
    }

    pub fn truncate(&mut self, value: usize) {
        self.references.truncate(value)
    }

    pub fn push(&mut self, value: Reference) {
        self.references.push(value)
    }

    pub fn get_typing(&self, name: &str) -> Option<Typing> {
        for reference in self.references.iter() {
            match reference {
                Reference::Variable(v) if v.name == name => return Some(v.typing.clone()),
                Reference::Function(f) if f.name == name => return Some(f.typing.clone()),
                _ => (),
            };
        }
        None
    }

    pub fn get_function_typing_ref(&mut self, name: &str) -> Option<&mut Typing> {
        for reference in self.references.iter_mut() {
            match reference {
                Reference::Function(f) if f.name == name => return Some(&mut f.typing),
                _ => (),
            };
        }
        None
    }
}
