use crate::process::AgentProcess;
use crate::runtime::{PhaseOutput, StepPhase};

// Reason will call the model or planner later.
pub fn reason(process: &AgentProcess) -> PhaseOutput {
    PhaseOutput::new(
        StepPhase::Reason,
        format!("selected action using {}", process.config().spec.model),
    )
}

#[cfg(test)]
mod tests {
    use super::reason;
    use crate::runtime::StepPhase;

    #[test]
    fn returns_reason_phase() {
        let agent = crate::test_support::config_factory::running_agent();

        assert_eq!(reason(&agent).phase, StepPhase::Reason);
    }
}
