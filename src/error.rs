use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AutoCopyError {
    ConfigFolderError(String),
    StdIOError(String),
}

impl Error for AutoCopyError {
    fn description(&self) -> &str {
        match self {
            AutoCopyError::ConfigFolderError(msg) => msg,
            AutoCopyError::StdIOError(msg) => msg,
        }
    }
}

impl Display for AutoCopyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

impl From<std::io::Error> for AutoCopyError {
    fn from(err: std::io::Error) -> Self {
        AutoCopyError::StdIOError(err.to_string())
    }
}
