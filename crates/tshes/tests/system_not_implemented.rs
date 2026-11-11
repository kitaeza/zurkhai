//! The engine's error type (002 §5). This file held the tests that pinned the
//! gap "Genden and Tsurphu are not implemented" (spec 005; `DEBT.md` D-9). Spec 008
//! implements both, so the tests that asserted `Err(SystemNotImplemented)` for them
//! (`genden_not_yet_supported`, `tsurphu_not_yet_supported`, and
//! `an_unimplemented_system_never_falls_back_to_phugpa`, which asserted the same
//! gap) are removed, and the Genden and Tsurphu tests replace them
//! (`exhaustive_versions.rs`, `p5_versions.rs`, `p7_proptest_versions.rs`,
//! `versions_compared.rs`; the question the third test asked, that no system
//! answers as Phugpa by accident, is `versions_compared.rs`'s c3 and c5).
//! `Error::SystemNotImplemented` stays in the enum for a deferred system.

use tshes::Error;

#[test]
fn the_error_type_displays_and_is_a_core_error() {
    // 002 §5: `Error` implements `core::fmt::Display` and `core::error::Error`.
    fn assert_error<E: core::error::Error>(_: &E) {}
    for error in [
        Error::OutOfRange,
        Error::SkippedDay,
        Error::InvalidField,
        Error::SystemNotImplemented,
        Error::Internal,
    ] {
        assert_error(&error);
        assert!(format!("{error}").len() > 10);
    }
}
