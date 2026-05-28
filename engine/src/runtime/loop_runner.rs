use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::process::{AgentProcess, AgentState};

use super::{RuntimeError, RuntimeReport, execute_step};

// Spawns the loop into a background task with a cancellation token for isolation.
pub fn spawn_agent_loop(
    mut process: AgentProcess,
    max_steps: u32,
    token: CancellationToken,
) -> JoinHandle<Result<RuntimeReport, RuntimeError>> {
    tokio::spawn(async move {
        tokio::select! {
            _ = token.cancelled() => Err(RuntimeError::Terminated),
            res = run_agent_loop(&mut process, max_steps) => res,
        }
    })
}

// Runs a deterministic fake loop before real LLM/tool logic exists.
pub async fn run_agent_loop(
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
    use super::{run_agent_loop, spawn_agent_loop};
    use tokio_util::sync::CancellationToken;
    use crate::process::{AgentProcess, AgentState};
    use crate::runtime::RuntimeError;
    use crate::test_support::config_factory::{running_agent, test_config};

    #[tokio::test]
    async fn cancels_loop_via_token() {
        let agent = running_agent();
        let token = CancellationToken::new();

        // Cancel immediately.
        token.cancel();

        let handle = spawn_agent_loop(agent, 3, token);
        let result = handle.await.expect("task should join");

        assert_eq!(result, Err(RuntimeError::Terminated));
    }

    #[tokio::test]
    async fn runs_three_steps_and_completes_process() {
        let mut agent = running_agent();

        let report = run_agent_loop(&mut agent, 3).await.expect("runtime should run");

        assert_eq!(report.step_count(), 3);
        assert_eq!(report.agent_id, "researcher");
        assert_eq!(report.steps[0].step_number, 1);
        assert_eq!(report.steps[2].step_number, 3);
        assert_eq!(report.final_state, AgentState::Done);
        assert_eq!(agent.state(), AgentState::Done);
    }

    #[tokio::test]
    async fn rejects_zero_steps() {
        let mut agent = running_agent();

        let result = run_agent_loop(&mut agent, 0).await;

        assert_eq!(result, Err(RuntimeError::InvalidStepLimit));
        assert_eq!(agent.state(), AgentState::Running);
    }

    #[tokio::test]
    async fn rejects_process_that_is_not_running() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        let result = run_agent_loop(&mut agent, 3).await;

        assert_eq!(result, Err(RuntimeError::ProcessNotRunning));
        assert_eq!(agent.state(), AgentState::Loading);
    }
}
