# Data Cleaning & Knowledge Extraction Pipeline V2

## Part 2 — Ingestion, Deduplication, Extraction, Structure Recovery, and Media Processing

### Stage 1: Ingestion

Objectives:
- Accept heterogeneous document formats.
- Validate integrity.
- Generate immutable document identifiers.
- Record provenance.

Supported Inputs:
- PDF
- DOCX
- PPTX
- XLSX
- TXT
- HTML
- Markdown
- Images
- Audio
- Video

Processing:
1. File discovery.
2. MIME verification.
3. Hash generation.
4. Metadata capture.
5. Queue registration.

Outputs:
- Raw document registry.
- Source metadata.
- Initial audit trail.

---

### Stage 2: Deduplication

Goals:
- Remove exact duplicates.
- Detect near duplicates.
- Preserve document lineage.

Layers:

#### Layer 1: Exact Hash Matching
- SHA256
- BLAKE3

#### Layer 2: Fuzzy Matching
- SimHash
- MinHash

#### Layer 3: Semantic Similarity
- Embedding-assisted clustering.

Outputs:
- Canonical document.
- Duplicate clusters.
- Duplicate confidence score.

---

### Stage 3: Extraction

Objectives:
- Recover maximum information.
- Preserve formatting where possible.

Preferred Tools:
- Docling
- PyMuPDF
- Apache Tika

Extraction Targets:
- Text
- Tables
- Headers
- Footers
- References
- Captions

Failure Strategy:
- Retry with secondary extractor.
- Escalate to OCR.

---

### Stage 3.5: Document Structure Recovery

Purpose:
Convert raw text into structured knowledge.

Recover:
- Sections
- Subsections
- Headings
- Lists
- Tables
- References

Outputs:
- Hierarchical document tree.
- Layout metadata.

---

### Stage 4: Media Processing

Image Pipeline:
- OCR
- Caption generation
- Metadata extraction

Audio Pipeline:
- Whisper transcription
- Speaker segmentation

Video Pipeline:
- Scene detection
- Frame extraction
- OCR on frames

GPU Offload:
Runs on RX 9070 workstation.

---

### Operational Notes

Laptop Responsibilities:
- Ingestion
- Hashing
- Metadata capture
- Queue management

RX 9070 Responsibilities:
- OCR acceleration
- Vision-language processing
- Media understanding

---

### Success Metrics

- Extraction success >95%
- Duplicate detection precision >90%
- OCR confidence >85%
- Media extraction coverage >80%

End of Part 2
