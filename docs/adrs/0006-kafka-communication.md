# ADR 0006: Kafka-Backed Agent Communication

## Status
Accepted

## Context
Multi-agent systems require coordination. Synchronous HTTP/gRPC calls between agents are fragile and don't scale well for large-scale "swarms" or long-running tasks.

## Decision
Use Apache Kafka (or Redpanda) as the event-driven backbone for inter-agent communication and system-wide event broadcasting.

## Consequences

**Positive:**
- **Decoupling:** Agents don't need to know where other agents are; they just publish to topics.
- **Backpressure:** Kafka handles spikes in message volume, allowing consumers (agents) to catch up at their own pace.
- **Durability:** Messages are persisted, allowing for recovery if an agent crashes.
- **Auditability:** The Kafka log acts as a global trace of all system-wide communications.

**Negative:**
- **Infrastructure Overhead:** Running a Kafka cluster (even a small one like Redpanda) adds significant operational complexity.
- **Asynchronous Debugging:** Tracking a request across multiple topics and consumers is harder than tracing a synchronous call.
