// Data the engine sends when an agent wants to use a tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRequest {
    pub agent_id: String,
    pub tool_name: String,
    pub input: String,
}
