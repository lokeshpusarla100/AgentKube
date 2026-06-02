use crate::process::AgentProcess;
use crate::runtime::{AgentClient, StepRecord, ToolGateway, act, perceive, reason};

// Executes one ReAct step using the provided LLM client and tool gateway.
pub async fn execute_step<C: AgentClient, G: ToolGateway>(
    process: &AgentProcess,
    step_number: u32,
    client: &C,
    gateway: &G,
) -> StepRecord {
    // 1. Perceive: Load context and see what's happening.
    let perceive_output = perceive(process);
    
    // 2. Reason: Ask the LLM to think and pick a tool.
    let reason_output = reason(process, client).await;
    
    // 3. Act: Run the selected tool through the gateway.
    let act_output = act(process, reason_output.action.clone(), gateway).await;

    StepRecord {
        step_number,
        phases: vec![perceive_output, reason_output, act_output],
    }
}

#[cfg(test)]
mod tests {
    use super::execute_step;
    use crate::runtime::{MockClient, MockToolGateway, StepPhase, ToolExecutionResult};

    #[tokio::test]
    async fn executes_one_react_step() {
        let agent = crate::test_support::config_factory::running_agent();
        let client = MockClient { response: "think".to_string() };
        let gateway = MockToolGateway {
            result: ToolExecutionResult {
                success: true,
                output: "done".to_string(),
                errors: vec![],
            },
        };

        let record = execute_step(&agent, 1, &client, &gateway).await;

        assert_eq!(record.step_number, 1);
        assert_eq!(record.phases[0].phase, StepPhase::Perceive);
        assert_eq!(record.phases[1].phase, StepPhase::Reason);
        assert_eq!(record.phases[2].phase, StepPhase::Act);
        assert_eq!(record.phases[2].tool_output, Some("done".to_string()));
    }
}
