# ADR 0010: Engine Infrastructure Config

## Status
Accepted

## Context
The Rust Engine needs to know where the Java Gateway is running. This value is infrastructure config, not agent behavior.

Putting the gateway endpoint into each agent YAML would duplicate the same platform setting across many agents.

## Decision
Add engine-level config in `examples/engine.yaml`.

The config currently stores:

```text
services.tool_gateway_endpoint
```

Agent YAML remains focused on agent identity, model, prompts, tools, and resource limits.

## Consequences

**Positive:**
- **Clean Separation:** Agent config describes agents, engine config describes infrastructure.
- **Less Duplication:** Gateway endpoint is configured once for the engine.
- **Runtime Ready:** `GrpcToolClient` can be created from engine config.

**Negative:**
- **Second Config File:** Running the engine now needs both agent config and engine config.
- **More Loading Flow:** The app must load infrastructure config before creating network clients.
