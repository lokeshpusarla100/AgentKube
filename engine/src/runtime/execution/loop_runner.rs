use crate::runtime::{AgentClient, RuntimeError, RuntimeReport, ToolGateway};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use std::sync::Arc;

use crate::process::{AgentProcess, AgentState};

use super::execute_step;

// Spawns the loop into a background task with a cancellation token for isolation.
// It takes a client 'C' and a gateway 'G' that implement their respective traits.
pub fn spawn_agent_loop<C: AgentClient + 'static, G: ToolGateway + 'static>(
    mut process: AgentProcess,
    max_steps: u32,
    token: CancellationToken,
    client: Arc<C>, 
    gateway: Arc<G>,
) -> JoinHandle<Result<RuntimeReport, RuntimeError>> {
    let t = token.clone();
    tokio::spawn(async move {
        tokio::select! {
            _ = token.cancelled() => Err(RuntimeError::Terminated),
            res = run_agent_loop(&mut process, max_steps, t, &*client, &*gateway) => res,
        }
    })
}

// Runs the main execution loop, checking the cancellation token and using the LLM client and tool gateway.
pub async fn run_agent_loop<C: AgentClient, G: ToolGateway>(
    process: &mut AgentProcess,
    max_steps: u32,
    token: CancellationToken,
    client: &C,
    gateway: &G,
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

        // Execute one step (Perceive-Reason-Act) using the LLM client and tool gateway.
        records.push(execute_step(process, step_number, client, gateway).await);
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
mod tests {
    use super::{run_agent_loop, spawn_agent_loop};
    use tokio_util::sync::CancellationToken;
    use crate::process::{AgentProcess, AgentState};
    use crate::runtime::{MockClient, MockToolGateway, RuntimeError, ToolExecutionResult};
    use crate::test_support::config_factory::{running_agent, test_config};
    use std::sync::Arc;

    #[tokio::test]
    async fn cancels_loop_via_token() {
        let agent = running_agent();
        let token = CancellationToken::new();
        let client = Arc::new(MockClient { response: "think".to_string() });
        let gateway = Arc::new(MockToolGateway {
            result: ToolExecutionResult {
                success: true,
                output: "".to_string(),
                errors: vec![],
            },
        });

        // Cancel immediately.
        token.cancel();

        let handle = spawn_agent_loop(agent, 3, token, client, gateway);
        let result = handle.await.expect("task should join");

        assert_eq!(result, Err(RuntimeError::Terminated));
    }

    #[tokio::test]
    async fn runs_three_steps_and_completes_process() {
        let mut agent = running_agent();
        let token = CancellationToken::new();
        let client = MockClient { response: "think".to_string() };
        let gateway = MockToolGateway {
            result: ToolExecutionResult {
                success: true,
                output: "done".to_string(),
                errors: vec![],
            },
        };

        let report = run_agent_loop(&mut agent, 3, token, &client, &gateway).await.expect("runtime should run");

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
        let gateway = MockToolGateway {
            result: ToolExecutionResult {
                success: true,
                output: "".to_string(),
                errors: vec![],
            },
        };

        let result = run_agent_loop(&mut agent, 0, token, &client, &gateway).await;

        assert_eq!(result, Err(RuntimeError::InvalidStepLimit));
        assert_eq!(agent.state(), AgentState::Running);
    }

    #[tokio::test]
    async fn rejects_process_that_is_not_running() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");
        let token = CancellationToken::new();
        let client = MockClient { response: "think".to_string() };
        let gateway = MockToolGateway {
            result: ToolExecutionResult {
                success: true,
                output: "".to_string(),
                errors: vec![],
            },
        };

        let result = run_agent_loop(&mut agent, 3, token, &client, &gateway).await;

        assert_eq!(result, Err(RuntimeError::ProcessNotRunning));
        assert_eq!(agent.state(), AgentState::Loading);
    }
}
