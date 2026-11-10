//! `tshes` — the zurkhai calendar engine (001 §1; 002 §3: L1, the only crate
//! that computes). Spec 004 sets up the workspace and the gate on an empty
//! core: no calendar code, no error type, no arithmetic helpers land here
//! until the first calendar round.
//!
//! The crate name `tshes` is the only calendar term written in code this
//! round (001 F3, ruled 2026-10-02). The glossary is created by the first
//! round that writes calendar code (`DEBT.md` D-2, still OPEN).

// F6, ruled (a): no standard library, and no alloc either (002 §4.6).
#![no_std]
// 002 §4.5 [CI]: no unsafe.
#![forbid(unsafe_code)]
// The deny list of 002 §4.1, §4.2 and §4.4, applied to the core ([CI]):
#![deny(
    clippy::float_arithmetic, // 002 §4.1: exact rational arithmetic only
    clippy::integer_division, // 002 §4.2: `/` truncates toward zero
    clippy::modulo_arithmetic, // 002 §4.2: `%` truncates toward zero
    clippy::unwrap_used,      // 002 §4.4: no panics on valid input
    clippy::expect_used,      // 002 §4.4
    clippy::panic,            // 002 §4.4
    clippy::indexing_slicing  // 002 §4.4
)]
