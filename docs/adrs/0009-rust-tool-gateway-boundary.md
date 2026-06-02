# ADR 0009: Rust Tool Gateway Boundary

## Status
Accepted

## Context
The Rust Engine owns the ReAct loop, but it should not own tool execution. Tool execution belongs in the Java Gateway because validation, permissions, rate limits, and sandboxing need one centralized place.

The Engine still needs a testable way to ask for tool results during the Act phase.

## Decision
Add a `ToolGateway` trait under `runtime/tool`.

`MockToolGateway` implements the trait for tests.

`GrpcToolClient` implements the trait for real runtime calls to Java.

The Engine will depend on `ToolGateway`, not directly on Java or tonic.

## Consequences

**Positive:**
- **Clear Boundary:** The Engine asks for a result but does not know how tools run.
- **Testability:** Tests can use `MockToolGateway` without starting Java.
- **Replaceable Transport:** The real client can change without rewriting the Act phase.

**Negative:**
- **Extra Abstraction:** There is one trait layer before the network call.
- **Wiring Needed:** `act()` and `execute_step()` still need to receive and call this trait.
