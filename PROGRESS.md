# AgentKube Progress Tracker

## Phase 1: Agent Execution Engine (In Progress)
- [x] Define gRPC Protobuf Contracts (`proto/agent.proto`)
- [x] Agent Process State Machine (`engine/src/process/state.rs`)
- [x] Perceive-Reason-Act Loop (`engine/src/runtime/loop_runner.rs`)
- [x] Process Isolation (Tokio tasks & cancellation tokens)
- [ ] LLM Client streaming integration
- [ ] Policy Engine inline hooks

## Phase 2: Tool Gateway Service (Pending)
- [ ] Spring Boot Gateway scaffolding
- [ ] Dynamic JSON Schema validation
- [ ] Token-bucket rate limiting per agent class
- [ ] Unified Tool Registry Interface

## Phase 3: Scheduler & Planning Service (Pending)
- [ ] Python Planner Daemon (DAG generation)
- [ ] Java Graph Orchestrator
- [ ] Dynamic Failure Re-planner

## Phase 4: Memory Service (Pending)
- [ ] Multi-tier storage architecture (Redis, Postgres)
- [ ] Append-only episodic ledger
- [ ] Semantic Vector Index (pgvector)

## Phase 5: Kafka-Backed Agent Communication (Pending)
- [ ] Isolated Topic Topologies
- [ ] Compacted Blackboard Store
- [ ] Dead Letter Queues (DLQ)

## Phase 6: Observability Stack (Pending)
- [ ] OpenTelemetry Interceptors
- [ ] Grafana Telemetry Dashboard

## Phase 7: Policy Engine & Auth (Pending)
- [ ] Fast Inline Evaluator (Rust)
- [ ] Token Budget Guard
- [ ] Injection Defense Filters

## Phase 8: Resilience & Chaos Layer (Pending)
- [ ] Adaptive Circuit Breakers
- [ ] Loop-Detection Filter
- [ ] Automated Chaos Suite

## Phase 9: Knowledge Service (RAG) (Pending)
- [ ] Semantic Document Chunker
- [ ] Hybrid Ensemble Search (pgvector + BM25)
- [ ] Citation Tracker

## Phase 10: Deploy & Demonstrate (Pending)
- [ ] Platform Management CLI (`agentctl`)
- [ ] Helm Deployment Blueprint
- [ ] End-to-End Demo Setup
