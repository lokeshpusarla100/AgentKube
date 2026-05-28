use crate::process::AgentProcess;
use crate::runtime::{AgentClient, PhaseOutput, StepPhase};

// The Reason phase asks the LLM what to do next.
pub async fn reason<C: AgentClient>(process: &AgentProcess, client: &C) -> PhaseOutput {
    let system_prompt = &process.config().spec.system_prompt;
    let model = &process.config().spec.model;

    // We ask the LLM for its reasoning. 
    // In a real scenario, we'd pass the actual conversation history here.
    let response = client
        .prompt(system_prompt, "What is the next step?")
        .await
        .unwrap_or_else(|_| "Thinking...".to_string());

    let action = process
        .config()
        .spec
        .tools
        .first()
        .cloned()
        .unwrap_or_else(|| "final_answer".to_string());

    PhaseOutput::with_action(
        StepPhase::Reason,
        format!("{}: {} (via {})", response, action, model),
        action,
    )
}

#[cfg(test)]
mod tests {
    use super::reason;
    use crate::runtime::{MockClient, StepPhase};

    #[tokio::test]
    async fn returns_reason_phase() {
        let agent = crate::test_support::config_factory::running_agent();
        let client = MockClient { response: "I should search".to_string() };

        let output = reason(&agent, &client).await;
        
        assert_eq!(output.phase, StepPhase::Reason);
        assert!(output.summary.contains("I should search"));
    }
}
