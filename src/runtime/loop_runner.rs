use crate::process::{AgentProcess, AgentState};

use super::{RuntimeError, StepRecord};

// Runs a deterministic fake loop before real LLM/tool logic exists.
pub fn run_fixed_steps(
    process: &mut AgentProcess,
    max_steps: u32,
) -> Result<Vec<StepRecord>, RuntimeError> {
    if max_steps == 0 {
        return Err(RuntimeError::InvalidStepLimit);
    }

    if process.state() != AgentState::Running {
        return Err(RuntimeError::ProcessNotRunning);
    }

    let mut records = Vec::new();

    for step_number in 1..=max_steps {
        records.push(StepRecord::new(step_number));
    }

    process.complete().map_err(RuntimeError::Lifecycle)?;

    Ok(records)
}

#[cfg(test)]
// Runtime tests prove execution only happens in Running state.
mod tests {
    use super::run_fixed_steps;
    use crate::config::{AgentConfig, AgentSpec, Metadata, Resources};
    use crate::process::{AgentProcess, AgentState};
    use crate::runtime::RuntimeError;

    #[test]
    fn runs_three_steps_and_completes_process() {
        let mut agent = running_agent();

        let records = run_fixed_steps(&mut agent, 3).expect("runtime should run");

        assert_eq!(records.len(), 3);
        assert_eq!(records[0].step_number, 1);
        assert_eq!(records[2].step_number, 3);
        assert_eq!(agent.state(), AgentState::Done);
    }

    #[test]
    fn rejects_zero_steps() {
        let mut agent = running_agent();

        let result = run_fixed_steps(&mut agent, 0);

        assert_eq!(result, Err(RuntimeError::InvalidStepLimit));
        assert_eq!(agent.state(), AgentState::Running);
    }

    #[test]
    fn rejects_process_that_is_not_running() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        let result = run_fixed_steps(&mut agent, 3);

        assert_eq!(result, Err(RuntimeError::ProcessNotRunning));
        assert_eq!(agent.state(), AgentState::Loading);
    }

    fn running_agent() -> AgentProcess {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");
        agent.load().expect("load should move to ready");
        agent.start().expect("start should move to running");
        agent
    }

    fn test_config() -> AgentConfig {
        AgentConfig {
            api_version: "agentkube/v1".to_string(),
            kind: "Agent".to_string(),
            metadata: Metadata {
                name: "researcher".to_string(),
            },
            spec: AgentSpec {
                model: "gemini-flash".to_string(),
                system_prompt: "Research and cite sources.".to_string(),
                tools: vec!["web_search".to_string()],
                resources: Resources {
                    max_memory: "50MB".to_string(),
                    max_tokens_per_task: 5000,
                    timeout_per_step: "30s".to_string(),
                    timeout_per_task: "300s".to_string(),
                },
                restart_policy: "on_failure".to_string(),
                max_restarts: 3,
            },
        }
    }
}
