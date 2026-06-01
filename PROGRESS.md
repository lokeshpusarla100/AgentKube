# AgentKube Progress Tracker
> Kubernetes for AI Agents: A 16-Week Build Plan

## 🛠 Current Focus: Architectural Plumbing
We are currently working across Phase 1 and Phase 2 because the **Execution Engine (Rust)** cannot complete a full ReAct loop without the **Tool Gateway (Java)**. The Engine handles the "brain," but the Gateway handles the "hands" (tool execution).

## Phase 1: Agent Execution Engine (In Progress)
- [x] Define gRPC Protobuf Contracts (`proto/agent.proto`)
- [x] Agent Process State Machine (`engine/src/process/state.rs`)
- [x] Perceive-Reason-Act (ReAct) Loop implementation
- [x] Process Isolation via Tokio tasks & cancellation tokens
- [x] Traited LLM Client (`AgentClient`) for provider abstraction
- [ ] LLM Streaming integration (reqwest/eventsource)
- [ ] gRPC Service implementation (`AgentService`)
- [ ] Real-time log streaming interface

## Phase 2: Tool Gateway Service (In Progress)
- [x] Spring Boot Gateway Scaffolding
- [x] Unified Tool Registry Interface
- [x] JSON Schema contract verification
- [ ] Token-bucket rate limiting per agent class
- [ ] Tool execution proxy (sandbox execution)

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
