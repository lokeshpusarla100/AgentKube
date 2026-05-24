use super::{StepPhase, StepRecord};

// Turns runtime step records into readable output.
pub fn format_step_trace(records: &[StepRecord]) -> Vec<String> {
    records
        .iter()
        .map(|record| {
            let phases = record
                .phases
                .iter()
                .map(format_phase)
                .collect::<Vec<_>>()
                .join(" -> ");

            format!("step {}: {}", record.step_number, phases)
        })
        .collect()
}

fn format_phase(phase: &StepPhase) -> &'static str {
    match phase {
        StepPhase::Perceive => "Perceive",
        StepPhase::Reason => "Reason",
        StepPhase::Act => "Act",
    }
}

#[cfg(test)]
// Trace tests keep runtime output stable for demos and logs.
mod tests {
    use super::format_step_trace;
    use crate::runtime::StepRecord;

    #[test]
    fn formats_step_records() {
        let records = vec![StepRecord::new(1), StepRecord::new(2)];

        let lines = format_step_trace(&records);

        assert_eq!(
            lines,
            vec![
                "step 1: Perceive -> Reason -> Act",
                "step 2: Perceive -> Reason -> Act",
            ]
        );
    }
}
