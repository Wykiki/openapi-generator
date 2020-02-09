use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CodeGenError {
    description: String,
    source: Option<&'static (dyn Error + 'static)>,
}

impl CodeGenError {
    pub fn new<S: Into<String>>(
        description: S,
        source: Option<&'static (dyn Error + 'static)>,
    ) -> Self {
        CodeGenError {
            description: description.into(),
            source: source,
        }
    }
}

impl fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGen error : {}", self.description)
    }
}

impl Error for CodeGenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
    }
}
