# ADR 0013: Shared Tool Gateway Connection via Arc

## Status
Accepted

## Context
The `AgentService` is responsible for orchestrating multiple concurrent agent loops. When an agent enters the `Act` phase, it must communicate with the Java Tool Gateway. 

If `AgentService` instantiated a new `GrpcToolClient` for every agent loop spawned, the system would rapidly exhaust network ports and waste memory managing redundant connection pools. The engine needs a way to provide tool execution capabilities to individual tasks without duplicating the underlying infrastructure.

## Decision
We implemented Atomically Reference Counted (`Arc`) shared ownership for the `ToolGateway` trait.
1. `AgentService` instantiates a single `ToolGateway` (e.g., `GrpcToolClient`) at startup.
2. The gateway is wrapped in an `Arc<G>`.
3. When `spawn_agent_loop` creates a new background Tokio task for an agent, it passes an `Arc::clone()` of the gateway.

## Consequences
### Positive
- **Resource Efficiency**: 1,000 active agents can safely share a single gRPC multiplexed channel and connection pool.
- **Dependency Injection**: By depending on `Arc<G: ToolGateway>`, the background loops remain completely decoupled from how the connection is managed or mocked.

### Negative
- **Thread Constraints**: Passing the trait across `tokio::spawn` bounds requires the trait implementation to satisfy `Send + Sync + 'static`, limiting how the client can hold state internally.
