use thiserror::Error;

// Errors the tool gateway can return to the engine.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ToolGatewayError {
    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
}
