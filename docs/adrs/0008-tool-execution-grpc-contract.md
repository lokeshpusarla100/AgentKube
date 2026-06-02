# ADR 0008: Tool Execution gRPC Contract

## Status
Accepted

## Context
The Rust Engine needs to request tool execution from the Java Gateway. Both services need one shared contract so request and response shapes stay consistent.

REST would be easier to inspect, but this platform already uses gRPC for internal service calls. gRPC also gives generated Rust and Java types from one proto file.

## Decision
Add `ToolGatewayService` to `proto/agent.proto`.

The service exposes one RPC:

```text
ExecuteTool(ToolExecutionRequest) -> ToolExecutionResult
```

`ToolExecutionRequest` carries `agent_id`, `tool_name`, and `input_json`.

`ToolExecutionResult` carries `success`, `output`, and `errors`.

## Consequences

**Positive:**
- **Shared Shape:** Rust and Java use the same protobuf contract.
- **Flexible Input:** `input_json` supports different tools without changing the proto.
- **Future Control:** `agent_id` supports rate limits, permissions, and audit logs.

**Negative:**
- **Mapping Needed:** Rust and Java still need conversion code between domain models and proto models.
- **Harder Debugging:** gRPC payloads are less direct to inspect than plain REST JSON.
