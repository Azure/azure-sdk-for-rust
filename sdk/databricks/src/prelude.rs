use std::error::Error;
use std::fmt;

pub type DatabricksResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug)]
pub struct DatabricksError {
    details: String,
}

impl DatabricksError {
    pub fn new(msg: &str) -> DatabricksError {
        DatabricksError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for DatabricksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for DatabricksError {
    fn description(&self) -> &str {
        &self.details
    }
}
