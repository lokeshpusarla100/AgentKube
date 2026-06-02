use async_trait::async_trait;
use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::runtime::model::agent_proto::{
    ToolExecutionRequest as ProtoToolExecutionRequest,
    ToolExecutionResult as ProtoToolExecutionResult,
    tool_gateway_service_client::ToolGatewayServiceClient,
};

use super::{ToolExecutionResult, ToolGateway, ToolGatewayError, ToolRequest};

// Real tool client that talks to the Java gateway over gRPC.
pub struct GrpcToolClient {
    client: Mutex<ToolGatewayServiceClient<Channel>>,
}


impl GrpcToolClient {
    // Opens a gRPC connection to the Java tool gateway.
    pub async fn connect(endpoint: &str) -> Result<Self, ToolGatewayError> {
        let client = ToolGatewayServiceClient::connect(endpoint.to_string())
            .await
            .map_err(|err| ToolGatewayError::ExecutionFailed(err.to_string()))?;

        Ok(Self {
            client: Mutex::new(client),
        })
    }
}


#[async_trait]
impl ToolGateway for GrpcToolClient {
    async fn execute(&self, request: ToolRequest) -> Result<ToolExecutionResult, ToolGatewayError> {
        let proto_request = to_proto_request(request);
        let mut client = self.client.lock().await;

        let response = client
            .execute_tool(proto_request)
            .await
            .map_err(|err| ToolGatewayError::ExecutionFailed(err.to_string()))?;

        Ok(from_proto_result(response.into_inner()))
    }
}

// Converts engine tool requests into protobuf requests.
fn to_proto_request(request: ToolRequest) -> ProtoToolExecutionRequest {
    ProtoToolExecutionRequest {
        agent_id: request.agent_id,
        tool_name: request.tool_name,
        input_json: request.input,
    }
}

// Converts protobuf tool results into engine tool results.
fn from_proto_result(result: ProtoToolExecutionResult) -> ToolExecutionResult {
    ToolExecutionResult {
        success: result.success,
        output: result.output,
        errors: result.errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_tool_request_to_proto_request() {
        let request = ToolRequest {
            agent_id: "researcher".to_string(),
            tool_name: "web_search".to_string(),
            input: "{\"query\":\"rust tonic\"}".to_string(),
        };

        let proto = to_proto_request(request);

        assert_eq!(proto.agent_id, "researcher");
        assert_eq!(proto.tool_name, "web_search");
        assert_eq!(proto.input_json, "{\"query\":\"rust tonic\"}");
    }

    #[test]
    fn converts_proto_result_to_tool_result() {
        let proto = ProtoToolExecutionResult {
            success: true,
            output: "{\"results\":[]}".to_string(),
            errors: vec![],
        };

        let result: ToolExecutionResult = from_proto_result(proto);

        assert_eq!(result.success, true);
        assert_eq!(result.output, "{\"results\":[]}".to_string(),);
        assert!(result.errors.is_empty());
    }
}
