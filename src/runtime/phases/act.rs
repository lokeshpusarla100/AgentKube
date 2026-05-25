use crate::runtime::{PhaseOutput, StepPhase};

// Act will call tools through policy-controlled gateways later.
pub fn act() -> PhaseOutput {
    PhaseOutput::new(StepPhase::Act, "executed selected action")
}

#[cfg(test)]
mod tests {
    use super::act;
    use crate::runtime::StepPhase;

    #[test]
    fn returns_act_phase() {
        assert_eq!(act().phase, StepPhase::Act);
    }
}
