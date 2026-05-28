use tonic::transport::Server;
use crate::runtime::execution::AgentServiceImpl;
use crate::runtime::model::agent_proto::agent_service_server::AgentServiceServer;

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
    let agent_service = AgentServiceImpl::default();

    println!("AgentKube Engine listening on {}", addr);

    // Build and run the gRPC server.
    Server::builder()
        .add_service(AgentServiceServer::new(agent_service))
        .serve(addr)
        .await?;

    Ok(())
}
