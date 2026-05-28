use crate::runtime::{AgentClient, RuntimeError, RuntimeReport};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::process::{AgentProcess, AgentState};

use super::execute_step;

// Spawns the loop into a background task with a cancellation token for isolation.
// It takes a client 'C' that implements the AgentClient trait.
pub fn spawn_agent_loop<C: AgentClient + 'static>(
    mut process: AgentProcess,
    max_steps: u32,
    token: CancellationToken,
    client: std::sync::Arc<C>, // Use Arc to share the client safely between threads
) -> JoinHandle<Result<RuntimeReport, RuntimeError>> {
    let t = token.clone();
    tokio::spawn(async move {
        tokio::select! {
            _ = token.cancelled() => Err(RuntimeError::Terminated),
            res = run_agent_loop(&mut process, max_steps, t, &*client) => res,
        }
    })
}

// Runs the main execution loop, checking the cancellation token and using the LLM client.
pub async fn run_agent_loop<C: AgentClient>(
    process: &mut AgentProcess,
    max_steps: u32,
    token: CancellationToken, // We now pass the token into the loop
    client: &C,                // The LLM client "plug"
) -> Result<RuntimeReport, RuntimeError> {
    if max_steps == 0 {
        return Err(RuntimeError::InvalidStepLimit);
    }

    if process.state() != AgentState::Running {
        return Err(RuntimeError::ProcessNotRunning);
    }

    let mut records = Vec::new();

    // Iterate through the assigned budget of steps.
    for step_number in 1..=max_steps {
        // Stop immediately if the system requested a shutdown.
        if token.is_cancelled() {
            return Err(RuntimeError::Terminated);
        }

        // Execute one step (Perceive-Reason-Act) using the LLM client.
        records.push(execute_step(process, step_number, client).await);
    }

    // Move the process to the 'Done' state once all steps finish.
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
    use crate::runtime::{MockClient, RuntimeError};
    use crate::test_support::config_factory::{running_agent, test_config};
    use std::sync::Arc;

    #[tokio::test]
    async fn cancels_loop_via_token() {
        let agent = running_agent();
        let token = CancellationToken::new();
        let client = Arc::new(MockClient { response: "think".to_string() });

        // Cancel immediately.
        token.cancel();

        let handle = spawn_agent_loop(agent, 3, token, client);
        let result = handle.await.expect("task should join");

        assert_eq!(result, Err(RuntimeError::Terminated));
    }

    #[tokio::test]
    async fn runs_three_steps_and_completes_process() {
        let mut agent = running_agent();
        let token = CancellationToken::new();
        let client = MockClient { response: "think".to_string() };

        let report = run_agent_loop(&mut agent, 3, token, &client).await.expect("runtime should run");

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
        let token = CancellationToken::new();
        let client = MockClient { response: "think".to_string() };

        let result = run_agent_loop(&mut agent, 0, token, &client).await;

        assert_eq!(result, Err(RuntimeError::InvalidStepLimit));
        assert_eq!(agent.state(), AgentState::Running);
    }

    #[tokio::test]
    async fn rejects_process_that_is_not_running() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");
        let token = CancellationToken::new();
        let client = MockClient { response: "think".to_string() };

        let result = run_agent_loop(&mut agent, 3, token, &client).await;

        assert_eq!(result, Err(RuntimeError::ProcessNotRunning));
        assert_eq!(agent.state(), AgentState::Loading);
    }
}
