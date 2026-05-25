use crate::process::AgentProcess;
use crate::runtime::{PhaseOutput, StepPhase};

// Act will call tools through policy-controlled gateways later.
pub fn act(process: &AgentProcess) -> PhaseOutput {
    PhaseOutput::new(
        StepPhase::Act,
        format!("available tools: {}", process.config().spec.tools.join(", ")),
    )
}

#[cfg(test)]
mod tests {
    use super::act;
    use crate::runtime::StepPhase;

    #[test]
    fn returns_act_phase() {
        let agent = crate::test_support::config_factory::running_agent();

        assert_eq!(act(&agent).phase, StepPhase::Act);
    }
}
