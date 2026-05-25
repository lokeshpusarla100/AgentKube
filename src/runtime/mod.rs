// Runtime owns what happens while an agent is Running.
mod error;
mod loop_runner;
mod phases;
mod report;
mod step;
mod step_executor;
mod trace;

pub use error::RuntimeError;
pub use loop_runner::run_agent_loop;
pub use phases::{PhaseOutput, act, perceive, reason};
pub use report::RuntimeReport;
pub use step::{StepPhase, StepRecord};
pub use step_executor::execute_step;
pub use trace::format_step_trace;
