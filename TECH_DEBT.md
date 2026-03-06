# Qazaq IR Technical Debt

## High Priority 
1. **O(1) State Machine via BitFlags in `morpheme_registry.rs`**
   - **Current Implementation:** `is_compatible` loops over `current_suffixes` (`O(k)` complexity where `k` is the suffix chain length).
   - **Required Architecture:** The Root must mutate and accumulate a `StateFlags` bitmask (e.g., `IS_ALLOCATED | IS_MUTABLE | IS_SIGNED`) after every successfully attached Suffix.
   - **Action:** Replace `current_suffixes.contains(...)` with Bitwise AND/OR operations to guarantee absolute `O(1)` validation for high-throughput (100k+ TPS) cores.
