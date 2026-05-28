use crate::process::AgentProcess;
use crate::runtime::{AgentClient, StepRecord, act, perceive, reason};

// Executes one ReAct step using the provided LLM client.
pub async fn execute_step<C: AgentClient>(
    process: &AgentProcess,
    step_number: u32,
    client: &C,
) -> StepRecord {
    let perceive_output = perceive(process);
    
    // The Reasoning phase now uses the real LLM client.
    let reason_output = reason(process, client).await;
    
    let act_output = act(process, reason_output.action.as_deref());

    StepRecord {
        step_number,
        phases: vec![perceive_output, reason_output, act_output],
    }
}

#[cfg(test)]
mod tests {
    use super::execute_step;
    use crate::runtime::{MockClient, StepPhase};

    #[tokio::test]
    async fn executes_one_react_step() {
        let agent = crate::test_support::config_factory::running_agent();
        let client = MockClient { response: "think".to_string() };

        let record = execute_step(&agent, 1, &client).await;

        assert_eq!(record.step_number, 1);
        assert_eq!(record.phases[0].phase, StepPhase::Perceive);
        assert_eq!(record.phases[1].phase, StepPhase::Reason);
        assert_eq!(record.phases[2].phase, StepPhase::Act);
    }
}
