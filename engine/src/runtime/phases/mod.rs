// Keep each ReAct phase separate so real logic can replace fake logic later.
mod act;
mod output;
mod perceive;
mod reason;

pub use act::act;
pub use output::PhaseOutput;
pub use perceive::perceive;
pub use reason::reason;
