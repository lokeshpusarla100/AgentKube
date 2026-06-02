use crate::runtime::StepPhase;

// Common shape returned by each execution phase.
#[derive(Debug, PartialEq, Eq)]
pub struct PhaseOutput {
    /// Which phase of the ReAct loop this output belongs to (Perceive, Reason, or Act).
    pub phase: StepPhase,

    /// A human-readable summary of what happened during this phase.
    pub summary: String,

    /// The name of the tool the agent decided to use (e.g., "web_search").
    /// - Some("name"): Set only during the 'Reason' phase when the agent picks a tool.
    /// - None: No tool was selected (standard for 'Perceive' and 'Act' phases).
    pub action: Option<String>,

    /// The actual data returned from the Tool Gateway after execution.
    /// - None: No tool was executed in this phase (standard for 'Perceive' and 'Reason').
    /// - Some(""): The tool ran successfully but returned an empty response.
    /// - Some("data"): The tool ran successfully and returned this specific output.
    pub tool_output: Option<String>,
}

impl PhaseOutput {
    pub fn new(phase: StepPhase, summary: impl Into<String>) -> Self {
        Self {
            phase,
            summary: summary.into(),
            action: None,
            tool_output: None,  
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
            tool_output: None,
        }
    }

    pub fn with_tool_output(
        phase: StepPhase,
        summary: impl Into<String>,
        tool_output: impl Into<String>,
    ) -> Self {
        Self {
            phase,
            summary: summary.into(),
            action: None,
            tool_output: Some(tool_output.into()),
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
        assert_eq!(output.tool_output, None);
    }

    #[test]
    fn creates_phase_output_with_action() {
        let output = PhaseOutput::with_action(StepPhase::Reason, "selected action", "web_search");

        assert_eq!(output.phase, StepPhase::Reason);
        assert_eq!(output.action, Some("web_search".to_string()));
        assert_eq!(output.tool_output, None);
    }

    #[test]
    fn creates_phase_output_with_tool_output() {
        let output = PhaseOutput::with_tool_output(StepPhase::Act, "executed tool", "result data");

        assert_eq!(output.phase, StepPhase::Act);
        assert_eq!(output.action, None);
        assert_eq!(output.tool_output, Some("result data".to_string()));
    }
}
