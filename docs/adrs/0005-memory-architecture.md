# ADR 0005: 3-Tier Memory Architecture

## Status
Accepted

## Context
Agents require different types of memory: immediate context (what just happened), episodic memory (history of past tasks), and semantic memory (knowledge base).

## Decision
Implement a 3-tier memory system:
1. **Working Memory (Redis):** Short-term, ultra-low latency context.
2. **Episodic Memory (Postgres):** Long-term, append-only history of every transaction.
3. **Semantic Memory (pgvector):** Embeddings-based retrieval for RAG and relevant fact-finding.

## Consequences

**Positive:**
- **Optimized Retrieval:** Matches the right storage engine to the access pattern.
- **Cost Control:** Allows offloading old episodic data to cheaper storage while keeping active context in Redis.
- **Unified Retrieval:** Using `pgvector` inside the same Postgres instance as the episodic ledger simplifies architecture and ensures data consistency.

**Negative:**
- **Complexity:** Managing three different storage tiers requires careful orchestration in the Memory Service.
- **Consistency:** Ensuring Redis and Postgres stay in sync during a task requires robust transactional logic.
