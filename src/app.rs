use std::path::Path;

use crate::config::{ConfigError, load_agent_config_from_file};
use crate::process::{AgentProcess, ProcessError};
use crate::runtime::{RuntimeError, RuntimeReport, format_step_trace, run_agent_loop};

// App-level errors keep startup failures readable.
#[derive(Debug, PartialEq)]
pub enum AppError {
    Config(ConfigError),
    Process(ProcessError),
    Lifecycle(String),
    Runtime(RuntimeError),
}

// Loads an agent file and creates a safe process from it.
pub fn load_process_from_file(path: &Path) -> Result<AgentProcess, AppError> {
    let config = load_agent_config_from_file(path).map_err(AppError::Config)?;
    AgentProcess::from_config(config).map_err(AppError::Process)
}

// Runs one agent file through the current engine path.
pub fn run_agent_file(path: &Path) -> Result<RuntimeReport, AppError> {
    let mut process = load_process_from_file(path)?;

    process.load().map_err(AppError::Lifecycle)?;
    process.start().map_err(AppError::Lifecycle)?;

    let max_steps = process.config().spec.resources.max_steps_per_task;

    run_agent_loop(&mut process, max_steps).map_err(AppError::Runtime)
}

// Default local run used by cargo run.
pub fn run_default_agent() -> Result<(), AppError> {
    let path = Path::new("../examples/agents/researcher.yaml");
    let report = run_agent_file(path)?;

    for line in format_step_trace(&report.steps) {
        println!("{}", line);
    }

    println!(
        "loaded agent process: state={:?}, steps={}",
        report.final_state,
        report.step_count()
    );

    Ok(())
}

#[cfg(test)]
// App tests cover the full config-to-process path.
mod tests {
    use std::fs;

    use super::{load_process_from_file, run_agent_file};
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
    max_steps_per_task: 3
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

    #[test]
    fn runs_agent_file_to_completion() {
        let path = std::env::temp_dir().join("agentkube-app-run-test.yaml");
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
    max_steps_per_task: 3
    timeout_per_step: 30s
    timeout_per_task: 300s
  restart_policy: on_failure
  max_restarts: 3
"#,
        )
        .expect("test file should be written");

        let report = run_agent_file(&path).expect("agent file should run");

        fs::remove_file(&path).expect("test file should be removed");

        assert_eq!(report.final_state, AgentState::Done);
        assert_eq!(report.step_count(), 3);
    }
}
