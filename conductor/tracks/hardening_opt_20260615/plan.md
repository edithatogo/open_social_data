# Plan: Ingestion Optimization and Pipeline Hardening

All tasks require Git commits upon completion. Pushes and reviews must occur at the end of each phase.

## Phase 1: Panic Safety & Connection Robustness
- [x] Task: Set up `reqwest` connection pooling and configure exponential backoff using local retry helper
- [ ] Task: Establish catch-panic boundaries (`std::panic::catch_unwind`) and circuit breakers around provider calls
- [ ] Task: Conductor - Push changes, perform peer review of Connection Hardening Phase (Protocol in workflow.md)

## Phase 2: Memory Limits & Deserialization Defense
- [ ] Task: Integrate SIMD-enabled deserialization with maximum depth and payload length limits to prevent resource exhaustion
- [ ] Task: Build memory-leak and CPU usage benchmark suites under malformed payloads
- [ ] Task: Conductor - Push changes, perform peer review of Parser Optimization Phase (Protocol in workflow.md)

## Swarm Notes - 2026-06-15
- Added `hardening` module with retry policy, async retry helper, simple circuit-breaker state, and hardened reqwest client construction.
- Wired ABS and Stats NZ providers to the hardened client constructor.
- Panic catch boundaries remain open; async panic capture needs careful implementation and source-level review.
