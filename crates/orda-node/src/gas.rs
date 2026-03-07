use qazaq_ir::SuffixMorpheme;

/// Calculates the required physical generic gas to execute a sequence of `SuffixMorphemes`.
/// Evaluation time is strictly O(n) relative to the number of morphological operations.
pub fn calculate_required_gas(morphs: &[SuffixMorpheme]) -> u64 {
    let mut total_gas: u64 = 0;

    for morph in morphs {
        total_gas += match morph {
            SuffixMorpheme::AllocHeap => 5,
            SuffixMorpheme::WriteToTarget => 10,
            SuffixMorpheme::IterateUntilEmpty => 20,
            SuffixMorpheme::BranchIfValid => 15,
            SuffixMorpheme::MakeMutable => 15,
            SuffixMorpheme::StreamData => 5,
            SuffixMorpheme::VerifyConsensus => 100,
            SuffixMorpheme::SignWithMLDSA(_) => 50, // Cryptography is expensive
        };
    }

    total_gas
}
