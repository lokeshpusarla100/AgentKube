use crate::process::AgentProcess;
use crate::runtime::{PhaseOutput, StepPhase};

// Perceive will gather context and observations later.
pub fn perceive(process: &AgentProcess) -> PhaseOutput {
    PhaseOutput::new(
        StepPhase::Perceive,
        format!("loaded context for {}", process.id()),
    )
}

#[cfg(test)]
mod tests {
    use super::perceive;
    use crate::runtime::StepPhase;

    #[test]
    fn returns_perceive_phase() {
        let agent = crate::test_support::config_factory::running_agent();

        assert_eq!(perceive(&agent).phase, StepPhase::Perceive);
    }
}
