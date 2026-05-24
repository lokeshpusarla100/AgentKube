use std::path::Path;

use crate::config::{ConfigError, load_agent_config_from_file};
use crate::process::{AgentProcess, ProcessError};

// App-level errors keep startup failures readable.
#[derive(Debug, PartialEq)]
pub enum AppError {
    Config(ConfigError),
    Process(ProcessError),
}

// Loads an agent file and creates a safe process from it.
pub fn load_process_from_file(path: &Path) -> Result<AgentProcess, AppError> {
    let config = load_agent_config_from_file(path).map_err(AppError::Config)?;
    AgentProcess::from_config(config).map_err(AppError::Process)
}

#[cfg(test)]
// App tests cover the full config-to-process path.
mod tests {
    use std::fs;

    use super::load_process_from_file;
    use crate::process::AgentState;

    #[test]
    fn loads_process_from_agent_file() {
        let path = std::env::temp_dir().join("agentkube-app-test.yaml");
        fs::write(
            &path,
            r#"
api_version: agentkube/v1
kind: Agent
metadata:
  name: researcher
spec:
  model: gemini-flash
  system_prompt: "Research and cite sources."
  tools: []
  resources:
    max_memory: 50MB
    max_tokens_per_task: 5000
    timeout_per_step: 30s
    timeout_per_task: 300s
  restart_policy: on_failure
  max_restarts: 3
"#,
        )
        .expect("test file should be written");

        let process = load_process_from_file(&path).expect("process should load");

        fs::remove_file(&path).expect("test file should be removed");

        assert_eq!(process.id(), "researcher");
        assert_eq!(process.state(), AgentState::Loading);
    }
}
