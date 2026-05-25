use crate::runtime::{PhaseOutput, StepPhase};

// Perceive will gather context and observations later.
pub fn perceive() -> PhaseOutput {
    PhaseOutput::new(StepPhase::Perceive, "gathered runtime context")
}

#[cfg(test)]
mod tests {
    use super::perceive;
    use crate::runtime::StepPhase;

    #[test]
    fn returns_perceive_phase() {
        assert_eq!(perceive().phase, StepPhase::Perceive);
    }
}
