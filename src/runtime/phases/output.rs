use crate::runtime::StepPhase;

// Common shape returned by each fake phase for now.
#[derive(Debug, PartialEq, Eq)]
pub struct PhaseOutput {
    pub phase: StepPhase,   // which phase ran
    pub summary: String,    // short trace message
}

impl PhaseOutput {
    pub fn new(phase: StepPhase, summary: impl Into<String>) -> Self {
        Self {
            phase,
            summary: summary.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PhaseOutput;
    use crate::runtime::StepPhase;

    #[test]
    fn creates_phase_output() {
        let output = PhaseOutput::new(StepPhase::Perceive, "loaded context");

        assert_eq!(output.phase, StepPhase::Perceive);
        assert_eq!(output.summary, "loaded context");
    }
}
