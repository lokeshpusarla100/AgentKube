use crate::process::{AgentProcess, AgentState};

// Startup moves a validated process toward active execution.
impl AgentProcess {
    pub fn load(&mut self) -> Result<(), String> {
        // Loading finishes by making the process ready to run.
        self.transition_to(AgentState::Ready)
    }

    pub fn start(&mut self) -> Result<(), String> {
        // Running is where tools, memory, and token budgets become active.
        self.transition_to(AgentState::Running)
    }
}

#[cfg(test)]
// Startup tests prove callers cannot skip required stages.
mod tests {
    use crate::config::{AgentConfig, AgentSpec, Metadata, Resources};
    use crate::process::{AgentProcess, AgentState};

    #[test]
    fn load_moves_process_to_ready() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        let result = agent.load();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Ready);
    }

    #[test]
    fn start_moves_ready_process_to_running() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");
        agent.load().expect("load should move to ready");

        let result = agent.start();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Running);
    }

    #[test]
    fn start_rejects_process_that_is_not_ready() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        let result = agent.start();

        assert!(result.is_err());
        assert_eq!(agent.state(), AgentState::Loading);
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
                    max_steps_per_task: 3,
                    timeout_per_step: "30s".to_string(),
                    timeout_per_task: "300s".to_string(),
                },
                restart_policy: "on_failure".to_string(),
                max_restarts: 3,
            },
        }
    }
}
