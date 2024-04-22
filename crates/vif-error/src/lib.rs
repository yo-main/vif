use vif_objects::ast::Callable;

#[derive(Debug)]
pub enum VifError {
    CLIError(CLIError),
    WrongArgumentNumberFunction(WrongArgumentNumberFunction),
    NonMutableArgumentToMutableParameter(NonMutableArgumentToMutableParameter),
    NonMutableArgumentToMutableVariable(NonMutableArgumentToMutableVariable),
    DifferentSignatureBetweenFunction(DifferentSignatureBetweenFunction),
    DifferentSignatureBetweenReturns(DifferentSignatureBetweenReturns),
}

impl VifError {
    pub fn format(&self, content: &str) -> String {
        match self {
            Self::WrongArgumentNumberFunction(a) => a.format(content),
            Self::NonMutableArgumentToMutableParameter(a) => a.format(content),
            Self::NonMutableArgumentToMutableVariable(a) => a.format(content),
            Self::DifferentSignatureBetweenFunction(a) => a.format(content),
            Self::DifferentSignatureBetweenReturns(a) => a.format(content),
            Self::CLIError(c) => c.msg.clone(),
        }
    }
}

#[derive(Debug)]
pub struct CLIError {
    msg: String,
}

impl CLIError {
    pub fn new(msg: String) -> VifError {
        VifError::CLIError(Self { msg })
    }
}

#[derive(Debug)]
pub struct WrongArgumentNumberFunction {
    function_name: String,
    expected: usize,
    received: usize,
    line: usize,
    pos: usize,
}

impl WrongArgumentNumberFunction {
    pub fn new(
        function_name: String,
        expected: usize,
        received: usize,
        line: usize,
        pos: usize,
    ) -> VifError {
        VifError::WrongArgumentNumberFunction(Self {
            expected,
            received,
            function_name,
            line,
            pos,
        })
    }

    fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.line).unwrap();
        format!(
            "Line {} - {row}\nWrong number of argument passed. Expected {} but received {}",
            self.line, self.expected, self.received
        )
    }
}

#[derive(Debug)]
pub struct NonMutableArgumentToMutableParameter {
    function_name: String,
    argument_name: String,
    line: usize,
    pos: usize,
}

impl NonMutableArgumentToMutableParameter {
    pub fn new(function_name: String, argument_name: String, line: usize, pos: usize) -> VifError {
        VifError::NonMutableArgumentToMutableParameter(Self {
            function_name,
            argument_name,
            line,
            pos,
        })
    }

    fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.line).unwrap();
        format!(
            "Line {} - {row}\nCannot pass {} argument (non mutable) to a mutable parameter",
            self.line, self.argument_name
        )
    }
}

#[derive(Debug)]
pub struct NonMutableArgumentToMutableVariable {
    variable_name: String,
    value: String,
    line: usize,
    pos: usize,
}

impl NonMutableArgumentToMutableVariable {
    pub fn new(variable_name: String, value: String, line: usize, pos: usize) -> VifError {
        VifError::NonMutableArgumentToMutableVariable(Self {
            variable_name,
            value,
            line,
            pos,
        })
    }

    fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.line).unwrap();
        format!(
            "Line {} - {row}\nCannot assign value {} (non mutable) to mutable variable {}",
            self.line, self.value, self.variable_name
        )
    }
}

#[derive(Debug)]
pub struct DifferentSignatureBetweenFunction {
    function_a: String,
    function_b: String,
    signature_a: Option<Box<Callable>>,
    signature_b: Option<Box<Callable>>,
    line: usize,
    pos: usize,
}

impl DifferentSignatureBetweenFunction {
    pub fn new(
        function_a: String,
        function_b: String,
        signature_a: Option<Box<Callable>>,
        signature_b: Option<Box<Callable>>,
        line: usize,
        pos: usize,
    ) -> VifError {
        VifError::DifferentSignatureBetweenFunction(Self {
            function_a,
            function_b,
            signature_a,
            signature_b,
            line,
            pos,
        })
    }

    fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.line).unwrap();
        format!(
            "Line {} - {row}\n{} and {} have different signature: {} against {}",
            self.line,
            self.function_a,
            self.function_b,
            match &self.signature_a {
                None => "None".to_owned(),
                Some(c) => format!("{c}"),
            },
            match &self.signature_b {
                None => "None".to_owned(),
                Some(c) => format!("{c}"),
            }
        )
    }
}

#[derive(Debug)]
pub struct DifferentSignatureBetweenReturns {
    function: String,
    return_a: Option<Box<Callable>>,
    return_b: Option<Box<Callable>>,
    line: usize,
    pos: usize,
}

impl DifferentSignatureBetweenReturns {
    pub fn new(
        function: String,
        return_a: Option<Box<Callable>>,
        return_b: Option<Box<Callable>>,
        line: usize,
        pos: usize,
    ) -> VifError {
        VifError::DifferentSignatureBetweenReturns(Self {
            function,
            return_a,
            return_b,
            line,
            pos,
        })
    }

    fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.line).unwrap();
        format!(
            "Line {} - {row}\nThe function {} got several return signature: {} and {}",
            self.line,
            self.function,
            match &self.return_a {
                None => "None".to_owned(),
                Some(c) => format!("{c}"),
            },
            match &self.return_b {
                None => "None".to_owned(),
                Some(c) => format!("{c}"),
            }
        )
    }
}
