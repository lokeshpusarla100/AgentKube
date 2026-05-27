use crate::config::{AgentConfig, AgentSpec, Metadata, Resources};
use crate::process::AgentProcess;

// Shared valid config for tests that do not care about config details.
pub fn test_config() -> AgentConfig {
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

// Shared running process for tests that start at runtime behavior.
pub fn running_agent() -> AgentProcess {
    let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");
    agent.load().expect("load should move to ready");
    agent.start().expect("start should move to running");
    agent
}
