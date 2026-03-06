# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-03-06

### Added
- **Turing Completeness**: Linear O(1) Control Flow through `IterateUntilEmpty` and `BranchIfValid` suffixes.
- **LLM Bridge**: Automatic OpenAPI JSON Schema generation via `schemars` and the `--emit-schema` flag to strictly guide AI Structured Outputs.
- **Cargo Workspace Architecture**: Separated the ecosystem into `crates/qazaq-ir` and our future node backend `crates/orda-node`.
- **Orda Node Sandbox**: Initial implementation of the `TransactionPool` (Mempool) that validates Post-Quantum logic from client intents gracefully.

## [0.2.0] - 2026-03-06

### Added
- **Qazaq CLI Compiler (`qazaqc`)**: A production-ready command-line tool taking JSON intents and emitting executable bounds.
- **LLVM IR Backend**: Zero-dependency textual generation of Static Single Assignment (`.ll`) Instructions.
- **Post-Quantum Cryptography Engine (`orda_pqc`)**: Mock structures and explicit integration of NIST FIPS 204 (ML-DSA-44).
- **Zero Hidden Context Enforcement**: `SuffixMorpheme::SignWithMLDSA` refactored to explicitly mandate `KeyAlias` strings inside Intent payloads.
- **Vitepress Documentation**: Added `CLI Compiler` and `Post-Quantum Cryptography` pages to formal architectural docs.

## [0.1.0] - 2026-03-06

### Added
- Core MVP of the **Qazaq IR Compiler Engine**.
- `MorphemeRegistry` with O(1) Bitwise Validation capabilities.
- `QazaqLexer` utilizing linear token structures instead of heavily nested ASTs.
- `SemanticRouter` for extracting deterministic JSON payloads from LLM generations.
- Academic documentation powered by Vitepress, showcasing the architecture and origin stories of the project.
- Dual-Licensing strategy including CC BY-ND 4.0 for the Whitepaper and BSL 1.1 for the Core Compiler codebase.
