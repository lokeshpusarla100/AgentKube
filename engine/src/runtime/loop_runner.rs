use crate::process::{AgentProcess, AgentState};

use super::{RuntimeError, RuntimeReport, execute_step};

// Runs a deterministic fake loop before real LLM/tool logic exists.
pub fn run_agent_loop(
    process: &mut AgentProcess,
    max_steps: u32,
) -> Result<RuntimeReport, RuntimeError> {
    if max_steps == 0 {
        return Err(RuntimeError::InvalidStepLimit);
    }

    if process.state() != AgentState::Running {
        return Err(RuntimeError::ProcessNotRunning);
    }

    let mut records = Vec::new();

    for step_number in 1..=max_steps {
        records.push(execute_step(process, step_number));
    }

    process.complete().map_err(RuntimeError::Lifecycle)?;

    Ok(RuntimeReport::new(
        process.id().to_string(),
        process.state(),
        records,
    ))
}

#[cfg(test)]
// Runtime tests prove execution only happens in Running state.
mod tests {
    use super::run_agent_loop;
    use crate::process::{AgentProcess, AgentState};
    use crate::runtime::RuntimeError;
    use crate::test_support::config_factory::{running_agent, test_config};

    #[test]
    fn runs_three_steps_and_completes_process() {
        let mut agent = running_agent();

        let report = run_agent_loop(&mut agent, 3).expect("runtime should run");

        assert_eq!(report.step_count(), 3);
        assert_eq!(report.agent_id, "researcher");
        assert_eq!(report.steps[0].step_number, 1);
        assert_eq!(report.steps[2].step_number, 3);
        assert_eq!(report.final_state, AgentState::Done);
        assert_eq!(agent.state(), AgentState::Done);
    }

    #[test]
    fn rejects_zero_steps() {
        let mut agent = running_agent();

        let result = run_agent_loop(&mut agent, 0);

        assert_eq!(result, Err(RuntimeError::InvalidStepLimit));
        assert_eq!(agent.state(), AgentState::Running);
    }

    #[test]
    fn rejects_process_that_is_not_running() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        let result = run_agent_loop(&mut agent, 3);

        assert_eq!(result, Err(RuntimeError::ProcessNotRunning));
        assert_eq!(agent.state(), AgentState::Loading);
    }
}
