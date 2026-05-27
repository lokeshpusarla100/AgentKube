use crate::config::{AgentConfig, validate_agent_config};

use super::{AgentState, ProcessError, is_valid_transition};

// Runtime wrapper for one loaded agent config.
#[derive(Debug)]
pub struct AgentProcess {
    id: String,            // process id
    state: AgentState,     // guarded lifecycle
    config: AgentConfig,   // loaded YAML config
}

// All process state changes live here.
impl AgentProcess {
    pub fn from_config(config: AgentConfig) -> Result<Self, ProcessError> {
        validate_agent_config(&config).map_err(ProcessError::InvalidConfig)?;

        // The YAML name becomes the process id for now.
        let id = config.metadata.name.clone();

        Ok(Self {
            id,
            state: AgentState::Loading,
            config,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn config(&self) -> &AgentConfig {
        // Config can be inspected, not replaced.
        &self.config
    }

    pub fn state(&self) -> AgentState {
        self.state
    }

    pub fn transition_to(&mut self, next: AgentState) -> Result<(), String> {
        // Invalid transitions leave the current state untouched.
        if is_valid_transition(self.state, next) {
            self.state = next;
            Ok(())
        } else {
            Err(format!(
                "invalid state transition from {:?} to {:?}",
                self.state, next
            ))
        }
    }
}

#[cfg(test)]
// Process tests prove callers cannot skip lifecycle rules.
mod tests {
    use super::AgentProcess;
    use crate::process::AgentState;
    use crate::test_support::config_factory::test_config;

    #[test]
    fn starts_in_loading_state() {
        let agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        assert_eq!(agent.id(), "researcher");
        assert_eq!(agent.state(), AgentState::Loading);
    }

    #[test]
    fn uses_config_metadata_name_as_process_id() {
        let agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        assert_eq!(agent.id(), "researcher");
        assert_eq!(agent.state(), AgentState::Loading);
    }

    #[test]
    fn keeps_agent_config_available() {
        let agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        assert_eq!(agent.config().spec.model, "gemini-flash");
        assert_eq!(agent.config().spec.tools, vec!["web_search"]);
    }

    #[test]
    fn rejects_invalid_config_before_creating_process() {
        let mut config = test_config();
        config.metadata.name = "".to_string();

        let result = AgentProcess::from_config(config);

        assert_eq!(
            result.err(),
            Some(super::ProcessError::InvalidConfig(
                "metadata.name cannot be empty".to_string(),
            )),
        );
    }

    #[test]
    fn transitions_when_move_is_valid() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        let result = agent.transition_to(AgentState::Ready);

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Ready);
    }

    #[test]
    fn rejects_invalid_transition_without_changing_state() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        let result = agent.transition_to(AgentState::Running);

        assert!(result.is_err());
        assert_eq!(agent.state(), AgentState::Loading);
    }
}
