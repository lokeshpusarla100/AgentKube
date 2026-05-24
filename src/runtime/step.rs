// One fake agent step follows the ReAct shape: perceive, reason, act.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepPhase {
    Perceive,
    Reason,
    Act,
}

// StepRecord lets the runtime report what happened in one loop cycle.
#[derive(Debug, PartialEq, Eq)]
pub struct StepRecord {
    pub step_number: u32,        // 1-based step number
    pub phases: Vec<StepPhase>,  // phases executed in order
}

impl StepRecord {
    pub fn new(step_number: u32) -> Self {
        Self {
            step_number,
            phases: vec![StepPhase::Perceive, StepPhase::Reason, StepPhase::Act],
        }
    }
}

#[cfg(test)]
// Step tests lock the order of the fake ReAct phases.
mod tests {
    use super::{StepPhase, StepRecord};

    #[test]
    fn creates_record_with_react_phase_order() {
        let record = StepRecord::new(1);

        assert_eq!(record.step_number, 1);
        assert_eq!(
            record.phases,
            vec![StepPhase::Perceive, StepPhase::Reason, StepPhase::Act]
        );
    }
}
