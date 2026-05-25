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
    use crate::process::{AgentProcess, AgentState};
    use crate::runtime::RuntimeError;
    use crate::test_support::config_factory::{running_agent, test_config};

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
}
