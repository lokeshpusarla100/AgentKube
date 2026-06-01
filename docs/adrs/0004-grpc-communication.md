# ADR 0004: gRPC for Inter-service Communication

## Status
Accepted

## Context
The platform consists of services in Rust (Engine), Java (Gateway, Memory), and Python (Planner). These services need to communicate with high performance and strong typing.

## Decision
Use gRPC (Protocol Buffers) as the primary communication protocol between all internal services.

## Consequences

**Positive:**
- **Strong Typing:** Protobuf files (`.proto`) act as a single source of truth across three different languages.
- **Performance:** HTTP/2 multiplexing and binary serialization are faster and more efficient than JSON over HTTP/1.1.
- **Streaming Support:** Native support for bidirectional streaming, which is critical for real-time agent log streaming and LLM output streaming.
- **Code Generation:** `tonic` (Rust), `grpc-java` (Java), and `grpcio` (Python) provide robust generated clients.

**Negative:**
- **Debugging Complexity:** Binary payloads are harder to inspect with `curl` compared to JSON. (Mitigated by using `grpcurl` or `Postman`).
- **Infrastructure Requirements:** Load balancers must support HTTP/2 and gRPC.
