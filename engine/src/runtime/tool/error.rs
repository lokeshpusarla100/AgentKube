// Errors the tool gateway can return to the engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolGatewayError {
    ToolNotFound(String),
    ValidationFailed(String),
    ExecutionFailed(String),
}
