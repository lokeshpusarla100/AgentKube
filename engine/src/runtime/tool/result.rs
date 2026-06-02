// Data the gateway sends back after running the tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolExecutionResult {
    pub success: bool,
    pub output: String,
    pub errors: Vec<String>,
}
