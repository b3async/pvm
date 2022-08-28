use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnsupportedSystem(String),
    UnsupportedDistro(String),
    Failed(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsupportedSystem(system) => write!(f, "{system} is not supported yet"),
            Error::UnsupportedDistro(distro) => write!(f, "{distro} is not supported yet"),
            Error::Failed(reason) => write!(f, "{reason}"),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            Error::UnsupportedSystem(_) => None,
            Error::UnsupportedDistro(_) => None,
            Error::Failed(_) => None,
        }
    }

    fn description(&self) -> &str {
        match self {
            Error::UnsupportedSystem(_) => "unsupported system",
            Error::UnsupportedDistro(_) => "unsupported distro",
            Error::Failed(_) => "execution failed",
        }
    }
}
