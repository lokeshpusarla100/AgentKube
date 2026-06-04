# ADR 0016: Graceful Termination via Cancellation Tokens

## Status
Accepted

## Context
In the Rust engine, agents run as background `tokio::spawn` tasks. If a user wants to stop an agent, or if the system needs to reclaim resources, we need a way to terminate these tasks. Brute-force "killing" or "aborting" a task in Rust can lead to resource leaks (open sockets, half-written files) and prevents clean state persistence.

## Decision
We implemented the `CancellationToken` pattern (provided by `tokio-util`).
1. Each `AgentProcess` is associated with a token.
2. The `LoopRunner` checks `token.is_cancelled()` at the start of every ReAct step.
3. The `AgentService` uses `tokio::select!` to race the entire execution loop against the token's cancellation signal.

## Consequences
### Positive
- **Safety**: Tasks exit at deterministic points, allowing the stack to unwind and `Drop` implementations to clean up memory/sockets.
- **Responsiveness**: Termination is nearly instantaneous (O(1)) because we don't have to wait for a full step to finish if the `select!` macro catches the signal.

### Negative
- **Logic Complexity**: The execution loop must be designed to be "cancellation-aware."
- **Passing Requirements**: The token must be cloned and passed deep into the call stack, increasing function signature noise.
