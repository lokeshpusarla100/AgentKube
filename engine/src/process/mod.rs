// Keep runtime state rules separate from the process wrapper.
mod agent;
mod error;
mod lifecycle;
mod state;

// Other modules should import process types from one place.
pub use agent::AgentProcess;
pub use error::ProcessError;
pub use state::{AgentState, is_valid_transition};
