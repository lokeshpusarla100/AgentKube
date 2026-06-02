use super::{AgentConfig, ConfigError, EngineConfig, validate_agent_config, validate_engine_config};

// Single entry point for loading an agent definition.
pub fn parse_agent_config(yaml: &str) -> Result<AgentConfig, ConfigError> {
    // Shape check: does the YAML match our config model?
    let config: AgentConfig =
        serde_yaml::from_str(yaml).map_err(|err| ConfigError::InvalidYaml(err.to_string()))?;

    // Safety check: is this config allowed to reach runtime?
    validate_agent_config(&config).map_err(ConfigError::InvalidConfig)?;

    Ok(config)
}

// Single entry point for loading engine infrastructure config.
pub fn parse_engine_config(yaml: &str) -> Result<EngineConfig, ConfigError> {
    // Shape check: does the YAML match our engine config model?
    let config: EngineConfig =
        serde_yaml::from_str(yaml).map_err(|err| ConfigError::InvalidYaml(err.to_string()))?;

    // Safety check: can the engine use this infrastructure config?
    validate_engine_config(&config).map_err(ConfigError::InvalidConfig)?;

    Ok(config)
}

#[cfg(test)]
// Parser tests prove bad configs fail before AgentProcess exists.
mod tests {
    use super::{parse_agent_config, parse_engine_config};

    #[test]
    fn parses_agent_yaml() {
        let yaml = r#"
api_version: agentkube/v1
kind: Agent
metadata:
  name: researcher
spec:
  model: gemini-flash
  system_prompt: "Research and cite sources."
  tools:
    - web_search
    - file_read
  resources:
    max_memory: 50MB
    max_tokens_per_task: 5000
    max_steps_per_task: 3
    timeout_per_step: 30s
    timeout_per_task: 300s
  restart_policy: on_failure
  max_restarts: 3
"#;

        let config = parse_agent_config(yaml).expect("valid yaml should parse");

        assert_eq!(config.metadata.name, "researcher");
        assert_eq!(config.spec.tools, vec!["web_search", "file_read"]);
        assert_eq!(config.spec.resources.max_tokens_per_task, 5000);
    }

    #[test]
    fn rejects_yaml_that_parses_but_fails_validation() {
        let yaml = r#"
api_version: agentkube/v1
kind: Agent
metadata:
  name: ""
spec:
  model: gemini-flash
  system_prompt: "Research and cite sources."
  tools: []
  resources:
    max_memory: 50MB
    max_tokens_per_task: 5000
    max_steps_per_task: 3
    timeout_per_step: 30s
    timeout_per_task: 300s
  restart_policy: on_failure
  max_restarts: 3
"#;

        let result = parse_agent_config(yaml);

        assert_eq!(
            result,
            Err(super::ConfigError::InvalidConfig(
                "metadata.name cannot be empty".to_string()
            ))
        );
    }

    #[test]
    fn parses_engine_yaml() {
        let yaml = r#"
api_version: agentkube/v1
kind: Engine
services:
  tool_gateway_endpoint: http://127.0.0.1:50051
"#;

        let config = parse_engine_config(yaml).expect("valid engine yaml should parse");

        assert_eq!(config.kind, "Engine");
        assert_eq!(
            config.services.tool_gateway_endpoint,
            "http://127.0.0.1:50051"
        );
    }

    #[test]
    fn rejects_engine_yaml_with_empty_tool_gateway_endpoint() {
        let yaml = r#"
api_version: agentkube/v1
kind: Engine
services:
  tool_gateway_endpoint: ""
"#;

        let result = parse_engine_config(yaml);

        assert_eq!(
            result,
            Err(super::ConfigError::InvalidConfig(
                "services.tool_gateway_endpoint cannot be empty".to_string()
            ))
        );
    }
}
