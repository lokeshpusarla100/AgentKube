use super::{StepPhase, StepRecord};

// Turns runtime step records into readable output.
pub fn format_step_trace(records: &[StepRecord]) -> Vec<String> {
    let mut lines = Vec::new();

    for record in records {
            let phases = record
                .phases
                .iter()
                .map(|output| format_phase(&output.phase))
                .collect::<Vec<_>>()
                .join(" -> ");

        lines.push(format!("step {}: {}", record.step_number, phases));

        for output in &record.phases {
            lines.push(format!(
                "  {}: {}",
                format_phase(&output.phase),
                output.summary
            ));
        }
    }

    lines
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
                "  Perceive: gathered runtime context",
                "  Reason: selected next action",
                "  Act: executed selected action",
                "step 2: Perceive -> Reason -> Act",
                "  Perceive: gathered runtime context",
                "  Reason: selected next action",
                "  Act: executed selected action",
            ]
        );
    }
}
