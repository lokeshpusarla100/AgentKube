# Data Cleaning & Knowledge Extraction Pipeline V2

# Part 4 — Synthetic Query Generation, Indexing, Storage, Evaluation and Retrieval

## Stage 8.5 Synthetic Query Generation
Purpose: Generate likely user questions from documents to improve retrieval recall.

Outputs:
- Question sets
- Retrieval prompts
- Query variations
- Hard negative candidates

## Stage 9 Indexing Architecture
Indexes:
- BM25 Index
- Dense Vector Index
- Entity Index
- Metadata Index
- Media Index

Hybrid Search:
BM25 + Dense Retrieval + Entity Signals.

## Stage 10 Storage Architecture
Layers:
- Raw Storage
- Cleaned Storage
- Metadata Store
- Vector Store
- Graph Store

Primary Database:
SQLite

Future:
PostgreSQL

## Stage 10.5 Evaluation Framework
Metrics:
- Recall@K
- Precision@K
- MRR
- NDCG
- Latency

Evaluation Sets:
- Human Queries
- Synthetic Queries
- Domain Queries

## Stage 11 Retrieval Architecture
Pipeline:
Query
-> BM25
-> Dense Retrieval
-> Reranking
-> Final Results

Future:
Agentic Retrieval
Knowledge Graph Traversal

End of Part 4