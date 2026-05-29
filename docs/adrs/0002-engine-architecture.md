# ADR 0002: Agent Execution Engine Architecture

## Context
We need a highly concurrent, state-machine driven engine in Rust to execute agents. The engine must support plugging in different LLMs, isolating tasks, and communicating over gRPC.

## Architecture Diagram

```mermaid
graph TD
    %% External Triggers
    Client([Client / Gateway]) -->|gRPC Start/Stop| Server[gRPC Server]

    %% Service Layer
    subgraph Service [AgentService]
        Server --> Registry[(DashMap: Active Agents)]
        Server --> Spawner[Task Spawner]
    end

    %% State & Config
    subgraph Memory [Process Context]
        Config[YAML Config] --> Process[AgentProcess State Machine]
        Process --> Loading --> Ready --> Running --> Done/Failed
    end

    %% Execution Loop
    subgraph Execution [Tokio Background Task]
        Spawner -->|Isolates with CancellationToken| Loop[run_agent_loop]
        Loop --> Step[execute_step]
        
        Step --> P[Perceive Phase]
        Step --> R[Reason Phase]
        Step --> A[Act Phase]
    end

    %% External Interfaces
    subgraph Interfaces [Traits / Plugs]
        R -->|AgentClient Trait| LLM((LLM API))
        A -->|Tool Call| Gateway((Java Gateway))
    end

    %% Connections
    Registry -.->|Token Cancel| Loop
    Process -.->|Borrow| Loop
```

## Decisions
1. **Tokio Isolation**: We use `tokio::spawn` and `CancellationToken` to run agents concurrently. If an agent goes rogue or the user calls `StopAgent`, we cancel the token and the loop exits immediately.
2. **State Machine**: The `AgentProcess` enforces valid transitions (e.g., cannot `start` if not `Ready`).
3. **Traited Client**: The `AgentClient` trait abstracts the LLM, allowing us to swap providers without touching core logic.