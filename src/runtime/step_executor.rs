use super::StepRecord;

// Executes one fake ReAct step until real phase logic exists.
pub fn execute_step(step_number: u32) -> StepRecord {
    StepRecord::new(step_number)
}

#[cfg(test)]
// Step executor tests keep one-step behavior separate from loop behavior.
mod tests {
    use super::execute_step;
    use crate::runtime::StepPhase;

    #[test]
    fn executes_one_react_step() {
        let record = execute_step(1);

        assert_eq!(record.step_number, 1);
        assert_eq!(
            record.phases,
            vec![StepPhase::Perceive, StepPhase::Reason, StepPhase::Act]
        );
    }
}
