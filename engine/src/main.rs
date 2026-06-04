use tonic::transport::Server;
use std::sync::Arc;
use crate::runtime::execution::AgentServiceImpl;
use crate::runtime::model::agent_proto::agent_service_server::AgentServiceServer;
use crate::runtime::{MockClient, GrpcToolClient};

// App startup glue lives outside main so it can be tested.
pub mod app;
// Expose config loading as part of the engine surface.
pub mod config;
// Expose runtime process handling as part of the engine surface.
pub mod process;
// Expose the execution loop runtime.
pub mod runtime;
// Test-only helpers keep repeated setup out of production modules.
#[cfg(test)]
pub mod test_support;

// The main entry point starts the gRPC server to handle agent requests over the network.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    
    // 1. Initialize the shared LLM client (currently Mock).
    let client = Arc::new(MockClient {
        response: "I will use the calculator tool to solve this math problem.".to_string(),
    });

    // 2. Connect to the Java Tool Gateway.
    // Default port for Spring Boot gRPC is 9090.
    let gateway_endpoint = "http://127.0.0.1:9090";
    println!("Connecting to Tool Gateway at {}...", gateway_endpoint);
    
    let gateway = Arc::new(GrpcToolClient::connect(gateway_endpoint).await?);

    // 3. Initialize the Agent Service with real shared components.
    let agent_service = AgentServiceImpl::new(client, gateway);

    println!("AgentKube Engine listening on {}", addr);

    // Build and run the gRPC server.
    Server::builder()
        .add_service(AgentServiceServer::new(agent_service))
        .serve(addr)
        .await?;

    Ok(())
}
