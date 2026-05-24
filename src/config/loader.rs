use std::fs;
use std::path::Path;

use super::{ConfigError, parse_agent_config};

// Loads an agent YAML file before the process is created.
pub fn load_agent_config_from_file(path: &Path) -> Result<super::AgentConfig, ConfigError> {
    let yaml = fs::read_to_string(path).map_err(|err| ConfigError::ReadFailed(err.to_string()))?;

    parse_agent_config(&yaml)
}

#[cfg(test)]
// Loader tests prove real files can become validated configs.
mod tests {
    use std::fs;

    use super::load_agent_config_from_file;

    #[test]
    fn loads_agent_config_from_yaml_file() {
        let path = std::env::temp_dir().join("agentkube-loader-test.yaml");
        fs::write(
            &path,
            r#"
api_version: agentkube/v1
kind: Agent
metadata:
  name: researcher
spec:
  model: gemini-flash
  system_prompt: "Research and cite sources."
  tools: []
  resources:
    max_memory: 50MB
    max_tokens_per_task: 5000
    timeout_per_step: 30s
    timeout_per_task: 300s
  restart_policy: on_failure
  max_restarts: 3
"#,
        )
        .expect("test file should be written");

        let config = load_agent_config_from_file(&path).expect("file config should load");

        fs::remove_file(&path).expect("test file should be removed");

        assert_eq!(config.metadata.name, "researcher");
        assert_eq!(config.spec.model, "gemini-flash");
    }
}
