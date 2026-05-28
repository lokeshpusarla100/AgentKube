// Runtime errors happen after a process has been created.
#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    InvalidStepLimit,
    ProcessNotRunning,
    Lifecycle(String),
    Terminated,
}
