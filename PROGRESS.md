# AgentKube Progress Tracker
> Kubernetes for AI Agents: A 16-Week Build Plan

## 🛠 Current Focus: Architectural Plumbing
We are currently working across Phase 1 and Phase 2 because the **Execution Engine (Rust)** cannot complete a full ReAct loop without the **Tool Gateway (Java)**. The Engine handles the "brain," but the Gateway handles the "hands" (tool execution).

The current milestone is to make the Act phase real. The Rust Engine already has the loop shape, but tool execution is still not wired into that loop. We are building the boundary that lets Rust ask Java to run a tool without Rust knowing how that tool works.

## Current Architecture Checkpoint
The project now has a shared tool execution contract in `proto/agent.proto`. That contract defines `ToolGatewayService.ExecuteTool`, `ToolExecutionRequest`, and `ToolExecutionResult`.

The request shape is intentionally small: `agent_id`, `tool_name`, and `input_json`. The `agent_id` gives us room for rate limits, permissions, and audit logs. The `tool_name` tells the gateway which registered tool to use. The `input_json` carries tool-specific arguments without changing the proto for every new tool.

The result shape is `success`, `output`, and `errors`. This matches the Java-side `ToolExecutionResult` model and gives the engine a simple way to record either a successful tool response or validation/execution failures.

On the Rust side, `runtime/tool` now owns the tool boundary. `ToolGateway` is the trait the engine will call. `MockToolGateway` lets tests run without Java. `GrpcToolClient` is the real network client skeleton that uses tonic-generated protobuf types.

The old generic `runtime/client` folder was renamed to `runtime/llm_client` so the codebase is clearer. LLM calls and tool calls are different boundaries, so they should not share one vague client name.

Engine infrastructure config now lives in `examples/engine.yaml`. This is where the Java Gateway endpoint belongs because it describes infrastructure, not a specific agent.

## Phase 1: Agent Execution Engine (In Progress)
- [x] Define gRPC Protobuf Contracts (`proto/agent.proto`)
  The proto now includes both agent-control RPCs and the tool-execution RPC. Rust generation through `tonic_build` compiles successfully.
- [x] Agent Process State Machine (`engine/src/process/state.rs`)
  Agent lifecycle states and allowed transitions are covered by tests.
- [x] Perceive-Reason-Act (ReAct) Loop implementation
  The loop runs through perceive, reason, and act phases, but Act still uses stub behavior.
- [x] Process Isolation via Tokio tasks & cancellation tokens
  Agent loops can be spawned and cancelled through Tokio task handles and cancellation tokens.
- [x] Traited LLM Client (`AgentClient`) for provider abstraction
  The LLM boundary now lives under `runtime/llm_client`.
- [x] Engine-level infrastructure config (`examples/engine.yaml`)
  Engine config now stores `services.tool_gateway_endpoint`.
- [x] Rust Tool Gateway boundary (`ToolGateway`, `MockToolGateway`)
  The engine can depend on a trait instead of hardcoding Java or gRPC details.
- [x] Rust gRPC Tool Gateway client skeleton (`GrpcToolClient`)
  The client can connect to an endpoint and map Rust tool types to protobuf tool types.
- [ ] LLM Streaming integration (reqwest/eventsource)
- [ ] gRPC Service implementation (`AgentService`)
- [x] Wire Act phase to `ToolGateway`
  The `act()` phase is now async and calls the gateway trait. `StepExecutor` and `LoopRunner` are updated to support this dependency.
- [ ] LLM Streaming integration (reqwest/eventsource)
- [x] gRPC Service implementation (`AgentService`)
  Rust gRPC server for agent lifecycle management is implemented with `DashMap` and `CancellationToken`.
- [ ] Real-time log streaming interface

## Phase 2: Tool Gateway Service (In Progress)
- [x] Spring Boot Gateway Scaffolding
  The Java service exists and tests run through Maven.
- [x] Unified Tool Registry Interface
  Tools can be registered and looked up by name.
- [x] JSON Schema contract verification
  Tool inputs are validated before execution using Jackson 3 and NetworkNT.
- [x] Tool execution gRPC contract (`ToolGatewayService`)
  Shared proto contract is implemented in both Rust and Java.
- [x] Java gRPC server implementation (`ExecuteTool`)
  Implemented `GrpcToolGatewayService` in Java to route requests to the `ToolExecutionService`.
- [ ] Token-bucket rate limiting per agent class
- [x] Live Rust Engine -> Java Gateway gRPC call
  Verified: Rust successfully called `ExecuteTool` on port 9090 and received a response.

## Current Integration Checkpoint
- [x] Rust tests passing: 54 tests
- [x] Java tests passing: 3 tests + Compilation Success
- [x] System Atlas Documentation: Detailed 26-step chronological roadmap completed.
- [x] ADRs logged: 0012, 0013, 0014, 0015, 0016.
- [x] Live End-to-End Handshake: COMPLETED.
1. Wire `act()` to accept a `ToolGateway`.
2. Update `execute_step()` to pass the gateway into the Act phase.
3. Add tests using `MockToolGateway`.
4. Implement Java `ExecuteTool` gRPC server.
5. Run a live Rust -> Java tool call.

## Phase 3: Scheduler & Planning Service (Pending)
- [ ] Python Planner Daemon (LLM-based DAG generation)
- [ ] Java Graph Orchestrator (Topological task dispatch)
- [ ] Dynamic Failure Re-planner (Self-healing graphs)

## Phase 4: Memory Service (Pending)
- [ ] 3-Tier Memory Architecture (Redis/Postgres/pgvector)
- [ ] Ultra-low latency Working Memory (Redis)
- [ ] Append-only Episodic Ledger (Postgres)
- [ ] Semantic Vector Index (pgvector + context budgeting)

## Phase 5: Kafka-Backed Agent Communication (Pending)
- [ ] Isolated Topic Topologies (`agent.*.inbox`)
- [ ] Compacted Blackboard Store (Global state)
- [ ] Dead Letter Queues (DLQ) & Fault Recovery

## Phase 6: Observability Stack (Pending)
- [ ] OpenTelemetry Interceptors (distributed tracing)
- [ ] Grafana Dashboards (Cost, Latency, Error rates)
- [ ] LLM-as-a-judge Evaluation Pipeline

## Phase 7: Policy Engine & Auth (Pending)
- [ ] Rust Inline Policy Evaluator (High-speed)
- [ ] Token Budget Guard (Spend limits)
- [ ] Prompt Injection Defense Filters

## Phase 8: Resilience & Chaos Layer (Pending)
- [ ] Adaptive Circuit Breakers (Resilience4j)
- [ ] Loop-Detection Filter
- [ ] Automated Chaos Suite (Simulated infrastructure failures)

## Phase 9: Knowledge Service (RAG) (Pending)
- [ ] Semantic Document Chunker
- [ ] Hybrid Ensemble Search (pgvector + BM25)
- [ ] Citation Tracker (Source verification)

## Phase 10: Deploy & Demonstrate (Pending)
- [ ] Platform Management CLI (`agentctl`)
- [ ] Helm Deployment Blueprint (K8s)
- [ ] Multi-agent research team demo
