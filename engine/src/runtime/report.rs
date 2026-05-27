use crate::process::AgentState;

use super::StepRecord;

// Structured result of one runtime execution.
#[derive(Debug, PartialEq, Eq)]
pub struct RuntimeReport {
    pub agent_id: String,           // process id that produced this report
    pub final_state: AgentState,      // state after runtime exits
    pub steps: Vec<StepRecord>,       // ordered execution steps
}

impl RuntimeReport {
    pub fn new(agent_id: String, final_state: AgentState, steps: Vec<StepRecord>) -> Self {
        Self {
            agent_id,
            final_state,
            steps,
        }
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

#[cfg(test)]
// Report tests keep summary behavior stable.
mod tests {
    use super::RuntimeReport;
    use crate::process::AgentState;
    use crate::runtime::StepRecord;

    #[test]
    fn counts_steps() {
        let report = RuntimeReport::new(
            "researcher".to_string(),
            AgentState::Done,
            vec![StepRecord::new(1), StepRecord::new(2)],
        );

        assert_eq!(report.step_count(), 2);
    }
}
