# ADR 0007: Tool Gateway Internal Architecture

## Status
Accepted

## Context
The Tool Gateway must handle hundreds of concurrent requests from multiple agents. We need an internal architecture that is thread-safe, modular, and easy to extend as we add new tool types.

## Decision
Separate the gateway into three distinct layers:
1. **Tool Registry**: A thread-safe, in-memory catalog of `ToolDefinition` objects using `ConcurrentHashMap`.
2. **Schema Validator**: A stateless component using `networknt/json-schema-validator` (3.0.x/Jackson 3) to enforce tool contracts.
3. **Execution Orchestrator**: A service that coordinates lookup and validation before delegating to actual tool logic.

## Consequences

**Positive:**
- **Concurrency**: High-throughput reads are possible via the `ConcurrentHashMap`.
- **Testability**: Each layer (Registry, Validator, Orchestrator) can be tested in isolation.
- **Resilience**: Validation happens *before* any tool execution, preventing bad data from hitting external APIs or sandboxes.

**Negative:**
- **In-Memory Volatility**: Since the registry is in-memory for now, tool definitions are lost on restart (Mitigated by Phase 4/Postgres integration later).
- **API Complexity**: Developers must update both the Registry and the Validator logic if the contract format changes.
