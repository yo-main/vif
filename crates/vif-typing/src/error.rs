use vif_objects::{ast::Callable, span::Span};

#[derive(Debug)]
pub enum TypingError {
    WrongArgumentNumberFunction(WrongArgumentNumberFunction),
    NonMutableArgumentToMutableParameter(NonMutableArgumentToMutableParameter),
    NonMutableArgumentToMutableVariable(NonMutableArgumentToMutableVariable),
    DifferentSignatureBetweenFunction(DifferentSignatureBetweenFunction),
    DifferentSignatureBetweenReturns(DifferentSignatureBetweenReturns),
}

#[derive(Debug)]
pub struct WrongArgumentNumberFunction {
    function_name: String,
    expected: usize,
    received: usize,
    span: Span,
}

impl TypingError {
    pub fn format(&self, content: &str) -> String {
        match self {
            Self::WrongArgumentNumberFunction(a) => a.format(content),
            Self::NonMutableArgumentToMutableParameter(a) => a.format(content),
            Self::NonMutableArgumentToMutableVariable(a) => a.format(content),
            Self::DifferentSignatureBetweenFunction(a) => a.format(content),
            Self::DifferentSignatureBetweenReturns(a) => a.format(content),
        }
    }
}

impl WrongArgumentNumberFunction {
    pub fn new(function_name: String, expected: usize, received: usize, span: Span) -> TypingError {
        TypingError::WrongArgumentNumberFunction(Self {
            expected,
            received,
            function_name,
            span,
        })
    }

    fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.span.get_line() - 1).unwrap();
        format!(
            "Line {} - {row}\nWrong number of argument passed. Expected {} but received {}",
            self.span.get_line(),
            self.expected,
            self.received
        )
    }
}

#[derive(Debug)]
pub struct NonMutableArgumentToMutableParameter {
    function_name: String,
    argument_name: String,
    span: Span,
}

impl NonMutableArgumentToMutableParameter {
    pub fn new(function_name: String, argument_name: String, span: Span) -> TypingError {
        TypingError::NonMutableArgumentToMutableParameter(Self {
            function_name,
            argument_name,
            span,
        })
    }

    fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.span.get_line() - 1).unwrap();
        format!(
            "Line {} - {row}\nCannot pass {} argument (non mutable) to a mutable parameter",
            self.span.get_line(),
            self.argument_name
        )
    }
}

#[derive(Debug)]
pub struct NonMutableArgumentToMutableVariable {
    variable_name: String,
    value: String,
    span: Span,
}

impl NonMutableArgumentToMutableVariable {
    pub fn new(variable_name: String, value: String, span: Span) -> TypingError {
        TypingError::NonMutableArgumentToMutableVariable(Self {
            variable_name,
            value,
            span,
        })
    }

    fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.span.get_line() - 1).unwrap();
        format!(
            "Line {} - {row}\nCannot assign value {} (non mutable) to mutable variable {}",
            self.span.get_line(),
            self.value,
            self.variable_name
        )
    }
}

#[derive(Debug)]
pub struct DifferentSignatureBetweenFunction {
    function_a: String,
    function_b: String,
    signature_a: Option<Box<Callable>>,
    signature_b: Option<Box<Callable>>,
    span: Span,
}

impl DifferentSignatureBetweenFunction {
    pub fn new(
        function_a: String,
        function_b: String,
        signature_a: Option<Box<Callable>>,
        signature_b: Option<Box<Callable>>,
        span: Span,
    ) -> TypingError {
        TypingError::DifferentSignatureBetweenFunction(Self {
            function_a,
            function_b,
            signature_a,
            signature_b,
            span,
        })
    }

    fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.span.get_line() - 1).unwrap();
        format!(
            "Line {} -{row}\n{} and {} have different signature: {} against {}",
            self.span.get_line(),
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
    span: Span,
}

impl DifferentSignatureBetweenReturns {
    pub fn new(
        function: String,
        return_a: Option<Box<Callable>>,
        return_b: Option<Box<Callable>>,
        span: Span,
    ) -> TypingError {
        TypingError::DifferentSignatureBetweenReturns(Self {
            function,
            return_a,
            return_b,
            span,
        })
    }

    fn format(&self, content: &str) -> String {
        let row = content.split('\n').nth(self.span.get_line() - 1).unwrap();
        format!(
            "Line {} - {row}\nThe function {} got several return signature: {} and {}",
            self.span.get_line(),
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
