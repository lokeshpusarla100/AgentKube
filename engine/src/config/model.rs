use serde::Deserialize;

// Full agent definition loaded from YAML.
#[derive(Debug, Deserialize, PartialEq)]
pub struct AgentConfig {
    pub api_version: String, // config contract version
    pub kind: String,        // expected to be Agent
    pub metadata: Metadata,  // identity block
    pub spec: AgentSpec,     // runtime behavior block
}

// Identity fields for naming and tracking an agent.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Metadata {
    pub name: String, // stable agent name
}

// Settings the engine needs before it can run the agent.
#[derive(Debug, Deserialize, PartialEq)]
pub struct AgentSpec {
    pub model: String,          // model provider/name
    pub system_prompt: String,  // base instruction
    pub tools: Vec<String>,     // requested tool names
    pub resources: Resources,   // runtime limits
    pub restart_policy: String, // restart behavior
    pub max_restarts: u32,      // retry cap
}

// Limits that stop one agent from consuming the whole engine.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Resources {
    pub max_memory: String,           // memory budget label
    pub max_tokens_per_task: u32,     // token budget
    pub max_steps_per_task: u32,      // loop step budget
    pub timeout_per_step: String,     // single loop timeout
    pub timeout_per_task: String,     // whole task timeout
}

// Full engine definition loaded from YAML.
#[derive(Debug, Deserialize, PartialEq)]
pub struct EngineConfig {
    pub api_version: String,      // config contract version
    pub kind: String,             // expected to be Engine
    pub services: EngineServices, // external services the engine calls
}

// Network addresses for services outside the Rust engine.
#[derive(Debug, Deserialize, PartialEq)]
pub struct EngineServices {
    pub tool_gateway_endpoint: String, // Java gateway gRPC address
}
