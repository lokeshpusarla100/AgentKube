use super::AgentConfig;

// Blocks configs that would create broken or unsafe agent processes.
pub fn validate_agent_config(config: &AgentConfig) -> Result<(), String> {
    // Names are used as process ids, so blank names are not safe.
    if config.metadata.name.trim().is_empty() {
        return Err("metadata.name cannot be empty".to_string());
    }

    // The engine cannot reason without a target model.
    if config.spec.model.trim().is_empty() {
        return Err("spec.model cannot be empty".to_string());
    }

    // Empty prompts create unpredictable agent behavior.
    if config.spec.system_prompt.trim().is_empty() {
        return Err("spec.system_prompt cannot be empty".to_string());
    }

    // Zero tokens means the agent can never complete a step.
    if config.spec.resources.max_tokens_per_task == 0 {
        return Err("spec.resources.max_tokens_per_task must be greater than 0".to_string());
    }

    Ok(())
}

#[cfg(test)]
// Validation tests define the minimum safe config rules.
mod tests {
    use super::validate_agent_config;
    use crate::config::{AgentConfig, AgentSpec, Metadata, Resources};

    #[test]
    fn accepts_valid_config() {
        let config = test_config();

        assert!(validate_agent_config(&config).is_ok());
    }

    #[test]
    fn rejects_empty_agent_name() {
        let mut config = test_config();
        config.metadata.name = " ".to_string();

        let result = validate_agent_config(&config);

        assert_eq!(result, Err("metadata.name cannot be empty".to_string()));
    }

    #[test]
    fn rejects_zero_token_budget() {
        let mut config = test_config();
        config.spec.resources.max_tokens_per_task = 0;

        let result = validate_agent_config(&config);

        assert_eq!(
            result,
            Err("spec.resources.max_tokens_per_task must be greater than 0".to_string())
        );
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
                tools: vec![],
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
