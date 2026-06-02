use crate::process::AgentProcess;
use crate::runtime::{PhaseOutput, StepPhase, ToolGateway, ToolRequest};

/// The Act phase takes the tool choice from the Reason phase and actually runs it.
/// It uses the ToolGateway to talk to the external Java service.
pub async fn act<G: ToolGateway>(
    process: &AgentProcess,
    action: Option<String>,
    gateway: &G,
) -> PhaseOutput {
    // If the Reason phase didn't pick a tool, we assume it's the agent's final answer.
    // We return a simple output with no tool results.
    let tool_name = match action {
        Some(name) => name,
        None => return PhaseOutput::new(StepPhase::Act, "Agent provided a final answer"),
    };

    // We build a request to send to the Java Gateway.
    // For now, we use a placeholder "{}" for tool arguments.
    let request = ToolRequest {
        agent_id: process.id().to_string(),
        tool_name: tool_name.clone(),
        input: "{}".to_string(),
    };

    // We call the gateway and wait for the response (this is a network call).
    match gateway.execute(request).await {
        Ok(result) => {
            let summary = if result.success {
                format!("Successfully executed tool: {}", tool_name)
            } else {
                format!("Tool execution failed: {}", result.errors.join(", "))
            };

            // We store the tool's response in the 'tool_output' field.
            PhaseOutput::with_tool_output(StepPhase::Act, summary, result.output)
        }
        Err(err) => {
            // If the gateway itself fails (e.g. network down), we report the error.
            PhaseOutput::with_tool_output(
                StepPhase::Act,
                format!("Gateway communication error: {}", err),
                "ERROR_GATEWAY_UNREACHABLE",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::act;
    use crate::runtime::{MockToolGateway, StepPhase, ToolExecutionResult};

    #[tokio::test]
    async fn executes_tool_via_gateway() {
        let agent = crate::test_support::config_factory::running_agent();
        let gateway = MockToolGateway {
            result: ToolExecutionResult {
                success: true,
                output: "search results".to_string(),
                errors: vec![],
            },
        };

        // We simulate the agent picking the 'web_search' tool.
        let output = act(&agent, Some("web_search".to_string()), &gateway).await;

        assert_eq!(output.phase, StepPhase::Act);
        assert_eq!(output.tool_output, Some("search results".to_string()));
    }

    #[tokio::test]
    async fn returns_none_when_no_action_provided() {
        let agent = crate::test_support::config_factory::running_agent();
        let gateway = MockToolGateway {
            result: ToolExecutionResult {
                success: true,
                output: "".to_string(),
                errors: vec![],
            },
        };

        // No action means final answer.
        let output = act(&agent, None, &gateway).await;

        assert_eq!(output.tool_output, None);
        assert!(output.summary.contains("final answer"));
    }
}
