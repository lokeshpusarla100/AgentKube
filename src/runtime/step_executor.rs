use crate::process::AgentProcess;

use super::{StepRecord, act, perceive, reason};

// Executes one fake ReAct step until real phase logic exists.
pub fn execute_step(process: &AgentProcess, step_number: u32) -> StepRecord {
    StepRecord {
        step_number,
        phases: vec![perceive(process), reason(process), act(process)],
    }
}

#[cfg(test)]
// Step executor tests keep one-step behavior separate from loop behavior.
mod tests {
    use super::execute_step;
    use crate::runtime::StepPhase;

    #[test]
    fn executes_one_react_step() {
        let agent = crate::test_support::config_factory::running_agent();

        let record = execute_step(&agent, 1);

        assert_eq!(record.step_number, 1);
        assert_eq!(record.phases[0].phase, StepPhase::Perceive);
        assert_eq!(record.phases[1].phase, StepPhase::Reason);
        assert_eq!(record.phases[2].phase, StepPhase::Act);
    }
}
