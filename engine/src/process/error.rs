// Errors that can happen while creating or managing an agent process.
#[derive(Debug, PartialEq)]
pub enum ProcessError {
    InvalidConfig(String),
}
