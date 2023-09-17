//! The zkEVM keccak circuit implementation, with some minor modifications
//! Credit goes to https://github.com/privacy-scaling-explorations/zkevm-circuits/tree/main/zkevm-circuits/src/keccak_circuit

use halo2_base::halo2_proofs;

/// Keccak packed multi
pub mod keccak;
/// SHA-256 bit
pub mod sha256_bit;
/// Util
pub mod util;
