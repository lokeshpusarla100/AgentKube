use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use dashmap::DashMap;
use tokio_stream::Stream;
use tokio_util::sync::CancellationToken;
use tonic::{Request, Response, Status};

use crate::app::load_process_from_file;
use crate::runtime::{AgentClient, ToolGateway, spawn_agent_loop};
use crate::runtime::model::agent_proto::{
    agent_service_server::AgentService,
    StartAgentRequest, StartAgentResponse,
    StopAgentRequest, StopAgentResponse,
    StreamAgentRequest, AgentStatusResponse
};

pub struct AgentServiceImpl<C, G> 
where 
    C: AgentClient + 'static,
    G: ToolGateway + 'static
{
    // Thread-safe map of active agent IDs to their cancellation tokens.
    active_agents: DashMap<String, CancellationToken>,
    // Shared LLM client used by all agents spawned by this service.
    client: Arc<C>,
    // Shared Tool Gateway used by all agents to execute tools.
    gateway: Arc<G>,
}

impl<C, G> AgentServiceImpl<C, G> 
where 
    C: AgentClient + 'static,
    G: ToolGateway + 'static
{
    pub fn new(client: Arc<C>, gateway: Arc<G>) -> Self {
        Self {
            active_agents: DashMap::new(),
            client,
            gateway,
        }
    }
}

#[tonic::async_trait]
impl<C, G> AgentService for AgentServiceImpl<C, G> 
where 
    C: AgentClient + 'static,
    G: ToolGateway + 'static
{
    async fn start_agent(
        &self,
        request: Request<StartAgentRequest>,
    ) -> Result<Response<StartAgentResponse>, Status> {
        let req = request.into_inner();
        let agent_id = req.agent_id.clone();

        // 1. Prevent starting the same agent twice.
        if self.active_agents.contains_key(&agent_id) {
            return Err(Status::already_exists(format!("Agent {} is already running", agent_id)));
        }

        // 2. Load the agent configuration from the examples folder.
        let path_str = format!("../examples/agents/{}.yaml", agent_id);
        let path = Path::new(&path_str);
        
        let mut process = load_process_from_file(path)
            .map_err(|e| Status::internal(format!("Failed to load agent: {:?}", e)))?;

        // 3. Prepare the process for execution.
        process.load().map_err(|e| Status::internal(e))?;
        process.start().map_err(|e| Status::internal(e))?;

        // 4. Create the isolation token.
        let token = CancellationToken::new();
        self.active_agents.insert(agent_id.clone(), token.clone());

        // 5. Spawn the background execution task.
        let max_steps = req.max_steps.max(process.config().spec.resources.max_steps_per_task);
        let client_ref = self.client.clone();
        let gateway_ref = self.gateway.clone();
        let agents_map_ref = self.active_agents.clone();
        let id_for_cleanup = agent_id.clone();

        spawn_agent_loop(process, max_steps, token, client_ref, gateway_ref);

        // 6. Spawn a "garbage collector" task to clean up the map when done.
        tokio::spawn(async move {
            // In a real version, we'd await the join handle, but for now, we'll
            // just keep the token until StopAgent is called or the task finishes.
            // (Simplification for fast-tracking).
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
            agents_map_ref.remove(&id_for_cleanup);
        });

        Ok(Response::new(StartAgentResponse {
            success: true,
            message: format!("Agent {} started successfully", agent_id),
        }))
    }

    async fn stop_agent(
        &self,
        request: Request<StopAgentRequest>,
    ) -> Result<Response<StopAgentResponse>, Status> {
        let req = request.into_inner();
        
        // Find and trigger the cancellation token for the agent.
        if let Some((_, token)) = self.active_agents.remove(&req.agent_id) {
            token.cancel();
            Ok(Response::new(StopAgentResponse { success: true }))
        } else {
            Err(Status::not_found(format!("Agent {} not found or not running", req.agent_id)))
        }
    }

    type StreamAgentStream = Pin<Box<dyn Stream<Item = Result<AgentStatusResponse, Status>> + Send>>;

    async fn stream_agent(
        &self,
        _request: Request<StreamAgentRequest>,
    ) -> Result<Response<Self::StreamAgentStream>, Status> {
        Err(Status::unimplemented("StreamAgent is part of the next iteration"))
    }
}
