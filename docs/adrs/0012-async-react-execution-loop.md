# ADR 0012: Asynchronous ReAct Execution Loop

## Status
Accepted

## Context
The core execution engine follows the ReAct pattern (Perceive -> Reason -> Act). Initially, the `Act` phase was implemented as a synchronous stub. However, executing real tools requires network communication with the external Java Tool Gateway via gRPC. 

Synchronous network calls block the thread they run on. If an agent waits 5 seconds for a tool to execute, that Tokio worker thread is frozen, severely limiting the engine's concurrency and overall throughput.

## Decision
We transitioned the `Act` phase, and consequently the entire execution call stack upwards (`execute_step`, `run_agent_loop`), to be fully asynchronous (`async fn`). 
- `Act` now uses `.await` when calling the `ToolGateway`.
- The engine can yield the thread back to Tokio while waiting for the Java Gateway to respond.

## Consequences
### Positive
- **Non-blocking Execution**: Network I/O during tool execution will not freeze the Tokio runtime threads.
- **High Concurrency**: The engine can pause execution for one agent waiting on a tool and seamlessly process the "Reason" phase for another agent on the same thread.

### Negative
- **Async Viral Effect**: Changing one core phase to `async` required rewriting the signatures of all parent orchestration functions.
- **Testing Complexity**: Unit tests testing the core loop now require `#[tokio::test]` to provide an async runtime environment.
