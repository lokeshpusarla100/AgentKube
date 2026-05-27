# ADR 0001: Use Rust for the Agent Execution Engine

## Status
Accepted

## Context
AgentKube acts as the runtime infrastructure for AI agents. The Execution Engine is the absolute core of the data plane. It runs the Perceive-Reason-Act loop for hundreds or thousands of agents simultaneously. It sits in the critical path for every single action an agent takes. We need a language that can handle massive concurrency, strict state management, and have an incredibly small footprint so we can pack as many agents onto a node as possible.

## Decision
We chose Rust for the Execution Engine instead of Python, Java, or Go. 

## Consequences

**Positive:**
- **Predictable Performance:** No garbage collection pauses means predictable sub-millisecond latency for state transitions and policy checks.
- **Resource Density:** Idle memory footprint is <20MB per agent runner, allowing massive scaling compared to JVM or Python runtimes.
- **Compile-Time Safety:** Rust's type system and borrow checker allow us to encode valid state transitions (e.g., `Loading` -> `Ready`) at compile time, eliminating an entire class of runtime orchestration bugs.
- **Concurrency Model:** `tokio` allows us to isolate thousands of agent processes onto a small number of OS threads with native asynchronous cancellation.

**Negative:**
- **Steeper Learning Curve:** Requires engineers to understand ownership, lifetimes, and async/await intricacies compared to Python scripts.
- **Slower Compilation:** Compile times are longer, which can slow down the inner development loop slightly.
