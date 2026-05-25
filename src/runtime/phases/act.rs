use crate::process::AgentProcess;
use crate::runtime::{PhaseOutput, StepPhase};

// Act will call tools through policy-controlled gateways later.
pub fn act(process: &AgentProcess, action: Option<&str>) -> PhaseOutput {
    let action = action.unwrap_or("final_answer");

    PhaseOutput::new(
        StepPhase::Act,
        format!(
            "executed {} with allowed tools [{}]",
            action,
            process.config().spec.tools.join(", ")
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::act;
    use crate::runtime::StepPhase;

    #[test]
    fn returns_act_phase() {
        let agent = crate::test_support::config_factory::running_agent();

        assert_eq!(act(&agent, Some("web_search")).phase, StepPhase::Act);
    }
}
