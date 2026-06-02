// Data the gateway sends back after running the tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolExecutionResult {
    pub output: String,
}
