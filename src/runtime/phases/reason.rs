use crate::runtime::{PhaseOutput, StepPhase};

// Reason will call the model or planner later.
pub fn reason() -> PhaseOutput {
    PhaseOutput::new(StepPhase::Reason, "selected next action")
}

#[cfg(test)]
mod tests {
    use super::reason;
    use crate::runtime::StepPhase;

    #[test]
    fn returns_reason_phase() {
        assert_eq!(reason().phase, StepPhase::Reason);
    }
}
