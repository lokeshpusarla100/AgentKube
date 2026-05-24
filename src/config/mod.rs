// Keep config concerns split: shape, parsing, and validation.
mod error;
mod loader;
mod model;
mod parser;
mod validation;

// Callers should not care how the config folder is split internally.
pub use error::ConfigError;
pub use loader::load_agent_config_from_file;
pub use model::{AgentConfig, AgentSpec, Metadata, Resources};
pub use parser::parse_agent_config;
pub use validation::validate_agent_config;
