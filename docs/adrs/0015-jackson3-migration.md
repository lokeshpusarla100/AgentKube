# ADR 0015: Jackson 3 & NetworkNT Validator Migration

## Status
Accepted

## Context
The project required high-performance JSON processing and schema validation in the Java Gateway. Standard Jackson 2.x and existing validators often conflict with modern Spring Boot 3+ classpaths or lack support for the latest JSON Schema drafts. Furthermore, Jackson 3.x introduced a total namespace shift from `com.fasterxml` to `tools.jackson`.

## Decision
We migrated the Java Gateway to the Jackson 3 (Alpha) ecosystem and matched it with the `networknt/json-schema-validator` (v3.0.x).
1. **Namespace isolation**: Using `tools.jackson` allows our project to avoid "Jar Hell" if other dependencies pull in legacy Jackson 2.x versions.
2. **Strict Validation**: NetworkNT was selected for its native support for the Jackson AST (`JsonNode`), allowing us to validate without intermediate string conversions.

## Consequences
### Positive
- **Future-Proofing**: We are aligned with the next generation of Java JSON processing.
- **Zero-Copy Validation**: We validate the `JsonNode` directly as it comes off the gRPC stream.

### Negative
- **Bleeding Edge**: Jackson 3 is still in Alpha/Beta, meaning we may encounter breaking API changes in future updates.
- **Dependency Precision**: We must manually manage `javax.annotation-api` as the generated gRPC code relies on legacy annotations removed in modern JDKs.
