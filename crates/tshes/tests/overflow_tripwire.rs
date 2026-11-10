//! Spec 004 Item 3 — the 002 §4.3 tripwire (METHOD rule 31).
//!
//! Arithmetic overflow must panic in every profile. This test fails in any
//! profile where overflow checks are off, so the gate cannot silently lose
//! the overflow guarantee (002 §4.3: release wraps silently without the
//! setting; METHOD rule 32). The gate self-test (spec 004 Item 5c) plants
//! exactly that shape and asserts this test goes red.

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn overflow_tripwire() {
    // `black_box` hides the operands so the compiler cannot constant-fold
    // the addition (a literal `i32::MAX + 1` is a compile error via the
    // `arithmetic_overflow` lint) — the overflow must happen at runtime,
    // and it must panic (002 §4.3, METHOD rule 31: the mechanism, not a
    // symptom).
    let _ = core::hint::black_box(i32::MAX) + core::hint::black_box(1);
}
