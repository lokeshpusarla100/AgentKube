use crate::process::AgentProcess;
use crate::runtime::{PhaseOutput, StepPhase};

// Reason will call the model or planner later.
pub fn reason(process: &AgentProcess) -> PhaseOutput {
    let action = process
        .config()
        .spec
        .tools
        .first()
        .cloned()
        .unwrap_or_else(|| "final_answer".to_string());

    PhaseOutput::with_action(
        StepPhase::Reason,
        format!("selected {} using {}", action, process.config().spec.model),
        action,
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
