//! Full-system Curvy PIX test.
//!
//! This reuses the pool-independent Session scenario and compiles it with the
//! `strategy-pix-curvy` pairing. The feature-specific assertions additionally
//! prove initial ERC-20 shielding and Blokli pending/committed-note correlation;
//! the common assertions cover Entry/Exit negotiation, SSA share reconstruction,
//! withdrawal, and the exact HOPR-token gain in the Exit Safe.

#[path = "session_pix.rs"]
mod scenario;
