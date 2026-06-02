use async_trait::async_trait;

use super::{ToolExecutionResult, ToolGatewayError, ToolRequest};

// Boundary between the Rust engine and the external tool gateway.
#[async_trait]
pub trait ToolGateway: Send + Sync {
    async fn execute(&self, request: ToolRequest) -> Result<ToolExecutionResult, ToolGatewayError>;
}

// Test gateway that lets us run the engine without Java.
pub struct MockToolGateway {
    pub result: ToolExecutionResult,
}

#[async_trait]
impl ToolGateway for MockToolGateway {
    async fn execute(&self, _request: ToolRequest) -> Result<ToolExecutionResult, ToolGatewayError> {
        Ok(self.result.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_gateway_returns_configured_result() {
        let gateway = MockToolGateway {
            result: ToolExecutionResult {
                output: "tool output".to_string(),
            },
        };
        let request = ToolRequest {
            agent_id: "researcher".to_string(),
            tool_name: "web_search".to_string(),
            input: "{\"query\":\"rust tonic\"}".to_string(),
        };

        let result = gateway.execute(request).await.unwrap();

        assert_eq!(result.output, "tool output");
    }
}
