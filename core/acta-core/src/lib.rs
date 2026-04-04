//! ACTA protocol core (Phase 0+).
//!
//! This crate will hold ONLY deterministic, verifiable logic:
//! - canonicalization rules
//! - hashing
//! - receipts (multi-signature-ready)
//! - Chronos (prev hash chain)
//! - Merkle epochs + proofs
//! - bundle verification
//!
//! No networking, no DB, no Cardano-specific logic in here.

pub mod types;
pub mod canonical;
pub mod hash;
pub mod receipt;
pub mod chronos;
pub mod merkle;
pub mod bundle;
pub mod process;
