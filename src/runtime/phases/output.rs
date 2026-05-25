use crate::runtime::StepPhase;

// Common shape returned by each fake phase for now.
#[derive(Debug, PartialEq, Eq)]
pub struct PhaseOutput {
    pub phase: StepPhase,   // which phase ran
    pub summary: String,    // short trace message
    pub action: Option<String>, // action selected by Reason
}

impl PhaseOutput {
    pub fn new(phase: StepPhase, summary: impl Into<String>) -> Self {
        Self {
            phase,
            summary: summary.into(),
            action: None,
        }
    }

    pub fn with_action(
        phase: StepPhase,
        summary: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            phase,
            summary: summary.into(),
            action: Some(action.into()),
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
        assert_eq!(output.action, None);
    }

    #[test]
    fn creates_phase_output_with_action() {
        let output = PhaseOutput::with_action(StepPhase::Reason, "selected action", "web_search");

        assert_eq!(output.phase, StepPhase::Reason);
        assert_eq!(output.action, Some("web_search".to_string()));
    }
}
