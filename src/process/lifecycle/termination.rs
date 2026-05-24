use crate::process::{AgentProcess, AgentState};

// Termination actions close an execution path.
impl AgentProcess {
    pub fn complete(&mut self) -> Result<(), String> {
        // Done means successful completion and later resource cleanup.
        self.transition_to(AgentState::Done)
    }

    pub fn fail(&mut self) -> Result<(), String> {
        // Failed preserves error context for logs and restart decisions.
        self.transition_to(AgentState::Failed)
    }
}

#[cfg(test)]
// Termination tests prove terminal states cannot be revived.
mod tests {
    use crate::config::{AgentConfig, AgentSpec, Metadata, Resources};
    use crate::process::{AgentProcess, AgentState};

    #[test]
    fn complete_moves_running_process_to_done() {
        let mut agent = running_agent();

        let result = agent.complete();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Done);
    }

    #[test]
    fn complete_rejects_paused_process() {
        let mut agent = running_agent();
        agent.pause().expect("pause should work from running");

        let result = agent.complete();

        assert!(result.is_err());
        assert_eq!(agent.state(), AgentState::Paused);
    }

    #[test]
    fn fail_moves_running_process_to_failed() {
        let mut agent = running_agent();

        let result = agent.fail();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Failed);
    }

    #[test]
    fn fail_moves_paused_process_to_failed() {
        let mut agent = running_agent();
        agent.pause().expect("pause should work from running");

        let result = agent.fail();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Failed);
    }

    #[test]
    fn terminal_process_cannot_resume() {
        let mut agent = running_agent();
        agent.complete().expect("complete should work from running");

        let result = agent.resume();

        assert!(result.is_err());
        assert_eq!(agent.state(), AgentState::Done);
    }

    fn running_agent() -> AgentProcess {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");
        agent.load().expect("load should move to ready");
        agent.start().expect("start should move to running");
        agent
    }

    fn test_config() -> AgentConfig {
        AgentConfig {
            api_version: "agentkube/v1".to_string(),
            kind: "Agent".to_string(),
            metadata: Metadata {
                name: "researcher".to_string(),
            },
            spec: AgentSpec {
                model: "gemini-flash".to_string(),
                system_prompt: "Research and cite sources.".to_string(),
                tools: vec!["web_search".to_string()],
                resources: Resources {
                    max_memory: "50MB".to_string(),
                    max_tokens_per_task: 5000,
                    timeout_per_step: "30s".to_string(),
                    timeout_per_task: "300s".to_string(),
                },
                restart_policy: "on_failure".to_string(),
                max_restarts: 3,
            },
        }
    }
}
