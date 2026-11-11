//! The engine's one error type (002 §5).

use core::fmt;

/// Why a conversion did not produce a date.
///
/// One `#[non_exhaustive]` enum for the whole crate (002 §5): wrappers map it
/// to their platform's error type one to one, without collapsing variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The date lies outside the supported range, 1900-01-01 to 2100-12-31
    /// inclusive (001 §6.4 P7), or a number is beyond the engine's stated
    /// arithmetic bound. The behaviour outside the range is defined: this
    /// error, never a wrong date and never a panic.
    OutOfRange,
    /// The `ZDate` names a tshes that no calendar day carries: a skipped tshes
    /// (Janson (arXiv:1401.6285v1) §6, p. 20). Typed, so that a caller has to
    /// handle it (002 §5).
    SkippedDay,
    /// A field value is invalid: out of its range, or a combination that no
    /// calendar day has (a leap flag on a month that is not a leap month, a
    /// doubled flag on a tshes that is not repeated, a `skipped_before` that
    /// is not the one the engine reports for that day).
    InvalidField,
    /// The computation system is not implemented; nothing was computed. The
    /// engine never falls back to another system. Since spec 008 all three
    /// systems of 001 §5.1 compute and no function returns this variant today;
    /// it stays for a deferred system (the karana-Sun Tsurphu, `DEBT.md` D-19,
    /// or the Bhutanese version) so that such a system fails loudly, never as
    /// Phugpa. Removing a variant from a `#[non_exhaustive]` public enum is the
    /// direction that cannot be undone once the crate is published (`DEBT.md` D-12).
    SystemNotImplemented,
    /// An internal invariant of the engine did not hold. This is a bug in
    /// the engine, never an expected result; the exhaustive properties
    /// assert that it does not occur over the supported range.
    Internal,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::OutOfRange => "the date is outside the supported range 1900-01-01 to 2100-12-31",
            Self::SkippedDay => "the tshes is skipped: no calendar day carries it",
            Self::InvalidField => "a field value is invalid or names no calendar day",
            Self::SystemNotImplemented => "the computation system is not implemented yet",
            Self::Internal => "an internal invariant of the engine did not hold",
        };
        f.write_str(text)
    }
}

impl core::error::Error for Error {}
