// Runtime sub-modules organized by responsibility.
pub mod client;
pub mod execution;
pub mod model;
pub mod phases;
pub mod tool;

mod error;

// Re-export core types for a clean public API.
pub use client::{AgentClient, ClientError, MockClient};
pub use error::RuntimeError;
pub use execution::{run_agent_loop, spawn_agent_loop, execute_step};
pub use model::{RuntimeReport, StepPhase, StepRecord, format_step_trace};
pub use phases::{act, perceive, reason, PhaseOutput};
pub use tool::{
    MockToolGateway, ToolExecutionResult, ToolGateway, ToolGatewayError, ToolRequest,
};
