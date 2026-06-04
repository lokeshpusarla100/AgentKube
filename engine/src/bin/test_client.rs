use tonic::Request;
use engine::runtime::model::agent_proto::agent_service_client::AgentServiceClient;
use engine::runtime::model::agent_proto::StartAgentRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AgentServiceClient::connect("http://127.0.0.1:50051").await?;

    // The engine currently loads agent config from ../examples/agents/{agent_id}.yaml
    // We'll use the 'researcher' which exists in the folder.
    let request = Request::new(StartAgentRequest {
        agent_id: "researcher".to_string(),
        user_message: "Calculate 2+2".to_string(),
        max_steps: 1,
    });

    println!("Sending StartAgent request for 'researcher' to Engine...");
    let response = client.start_agent(request).await?;

    println!("Response from Engine: {:?}", response.into_inner());
    
    // Wait a few seconds to let the agent run and talk to Java.
    println!("Waiting for agent to execute...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    Ok(())
}