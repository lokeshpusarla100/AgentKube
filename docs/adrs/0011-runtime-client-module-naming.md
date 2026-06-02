# ADR 0011: Runtime Client Module Naming

## Status
Accepted

## Context
The old `runtime/client` name was too vague. The project now has two different external boundaries: the LLM provider and the Java Tool Gateway.

Using one generic client module makes it harder to tell what each client talks to.

## Decision
Rename `runtime/client` to `runtime/llm_client`.

Keep tool execution code under `runtime/tool`.

The real tool network caller lives as `runtime/tool/grpc_client.rs`.

## Consequences

**Positive:**
- **Clear Names:** `llm_client` talks to model providers, and `tool` handles tool execution.
- **Better Navigation:** New files have an obvious place to live.
- **Less Confusion:** The Java Gateway client is not mixed with the LLM client.

**Negative:**
- **Small Refactor:** Existing references to the old module path must be updated.
