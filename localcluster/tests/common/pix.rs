//! PIX accounting shared by `session_pix` and `session_pix_soak`.
//!
//! Kept out of [`super`], which is about bringing a cluster up; a balance helper there would
//! make it a junk drawer.

use hopr_lib::api::types::primitive::prelude::HoprBalance;

/// Ceiling when interpreting a Safe delta as a whole number of cycles; a bound on the
/// division, not an expectation.
pub const MAX_PLAUSIBLE_CYCLES: u64 = 100_000;

/// Whole SSA deposits represented by `delta`, or `None` when it is not an exact multiple —
/// which would mean something other than PIX moved the balance.
///
/// Division rather than a search over `0..=MAX_PLAUSIBLE_CYCLES`: the two PIX suites used to
/// carry one implementation each, and they had diverged into different algorithms, different
/// zero-handling and different ceilings — so the two tests no longer agreed on what a whole
/// number of cycles meant. This is the soak's, which is the one that stays correct at soak
/// magnitudes.
pub fn completed_cycles(delta: HoprBalance, per_cycle: HoprBalance) -> Option<u64> {
    if per_cycle.is_zero() {
        return None;
    }
    let n = delta.amount() / per_cycle.amount();
    if n > MAX_PLAUSIBLE_CYCLES.into() {
        return None;
    }
    let n = n.as_u64();
    (per_cycle * n == delta).then_some(n)
}
