use crate::runtime::phases::PhaseOutput;

// One agent step follows the ReAct shape: perceive, reason, act.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepPhase {
    Perceive,
    Reason,
    Act,
}

// StepRecord lets the runtime report what happened in one loop cycle.
#[derive(Debug, PartialEq, Eq)]
pub struct StepRecord {
    pub step_number: u32,             // 1-based step number
    pub phases: Vec<PhaseOutput>,     // phase outputs in order
}

impl StepRecord {
    pub fn new(step_number: u32) -> Self {
        Self {
            step_number,
            phases: vec![
                PhaseOutput::new(StepPhase::Perceive, "loaded context"),
                PhaseOutput::new(StepPhase::Reason, "selected action"),
                PhaseOutput::new(StepPhase::Act, "executed action"),
            ],
        }
    }
}

#[cfg(test)]
// Step tests lock the order of the ReAct phases.
mod tests {
    use super::{StepPhase, StepRecord};

    #[test]
    fn creates_record_with_react_phase_order() {
        let record = StepRecord::new(1);

        assert_eq!(record.step_number, 1);
        assert_eq!(record.phases[0].phase, StepPhase::Perceive);
        assert_eq!(record.phases[1].phase, StepPhase::Reason);
        assert_eq!(record.phases[2].phase, StepPhase::Act);
    }
}
