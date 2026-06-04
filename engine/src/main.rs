use tonic::transport::Server;
use std::sync::Arc;
use engine::runtime::execution::AgentServiceImpl;
use engine::runtime::model::agent_proto::agent_service_server::AgentServiceServer;
use engine::runtime::{MockClient, GrpcToolClient};

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
