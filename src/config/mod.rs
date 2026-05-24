// Keep config concerns split: shape, parsing, and validation.
mod model;
mod parser;
mod validation;

// Callers should not care how the config folder is split internally.
pub use model::{AgentConfig, AgentSpec, Metadata, Resources};
pub use parser::{ConfigError, parse_agent_config};
pub use validation::validate_agent_config;
