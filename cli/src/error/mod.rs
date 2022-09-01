use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ContextError {
    MissingProperty(String),
}

impl Display for ContextError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextError::MissingProperty(property) => {
                write!(f, "property {} is missing", property)
            }
        }
    }
}

impl Error for ContextError {
    fn description(&self) -> &str {
        match self {
            ContextError::MissingProperty(_) => "missing property",
        }
    }
}
