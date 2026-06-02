use async_trait::async_trait;

// Errors the LLM client can return to the engine.
#[derive(Debug, PartialEq)]
pub enum ClientError {
    ApiError(String),
    Timeout,
}

// Boundary between the engine and any LLM provider.
#[async_trait]
pub trait AgentClient: Send + Sync {
    async fn prompt(&self, system_prompt: &str, user_message: &str) -> Result<String, ClientError>;
}

// Test client that returns a fixed model response.
pub struct MockClient {
    pub response: String,
}

#[async_trait]
impl AgentClient for MockClient {
    async fn prompt(&self, _system_prompt: &str, _user_message: &str) -> Result<String, ClientError> {
        Ok(self.response.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Proves the mock client returns the response we configured.
    #[tokio::test]
    async fn mock_client_returns_configured_response() {
        let client = MockClient {
            response: "Hello from mock".to_string(),
        };

        let result = client.prompt("system", "user").await.unwrap();

        assert_eq!(result, "Hello from mock");
    }
}
