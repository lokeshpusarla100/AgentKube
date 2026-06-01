# ADR 0003: Tool Gateway Service (Java/Spring Boot)

## Status
Accepted

## Context
Agents need to interact with the world via tools (search, database, API calls). Letting the Rust Engine execute these directly would create tight coupling and security risks. We need a centralized layer to manage tool registration, validate inputs against schemas, and enforce rate limits.

## Decision
Build a Tool Gateway using Java (Spring Boot) and Spring Cloud Gateway.

## Consequences

**Positive:**
- **Schema Enforcement:** Spring Boot's validation and JSON Schema libraries make it easy to enforce strict contracts for tool calls.
- **Resilience:** Leverage `Resilience4j` for circuit breaking and rate limiting per agent class.
- **Enterprise Ready:** Java has mature clients for almost any database or external service the tools might need to hit.
- **Separation of Concerns:** The Rust Engine focuses on the execution loop, while the Java Gateway focuses on the "outside world" interaction.

**Negative:**
- **Serialization Overhead:** Every tool call requires a gRPC/HTTP hop between the Engine and the Gateway.
- **JVM Memory Footprint:** Higher than Rust, but acceptable for a centralized control-plane component.
