# ADR 0014: JSON Schema Validation for Tool Integrity

## Status
Accepted

## Context
AI agents generate non-deterministic outputs. When an agent decides to use a tool, there is a high risk that the LLM will hallucinate arguments, omit required fields, or provide the wrong data types in the JSON payload. Allowing these malformed requests to reach physical tool logic (e.g., database queries or API calls) leads to brittle error handling and security vulnerabilities.

## Decision
We implemented a strict JSON Schema validation layer in the Java Tool Gateway.
1. Every tool in the `ToolRegistry` must define a JSON Schema.
2. The `SchemaValidator` compiles these schemas into an execution graph.
3. Before any tool logic is executed, the `ToolExecutionService` validates the agent's input against the schema.
4. If validation fails, the request is rejected immediately with a list of specific structural errors, preventing the tool from ever running.

## Consequences
### Positive
- **Fault Isolation**: LLM hallucinations are caught at the gateway boundary, not inside tool logic.
- **Contract Enforcement**: Tools can assume they will only ever receive valid, typed data.
- **Security**: Prevents injection attacks that rely on bypassing loosely typed inputs.

### Negative
- **Latency**: Adding a validation step adds sub-millisecond overhead to every tool call.
- **Registry Overhead**: Tool developers must maintain accurate schema definitions.
