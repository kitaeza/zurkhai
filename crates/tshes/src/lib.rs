//! `tshes` — the zurkhai calendar engine (001 §1; 002 §3: L1, the only crate
//! that computes).
//!
//! Spec 005 implements the Phugpa version, Gregorian ↔ `ZDate`, for the days
//! 1900-01-01 to 2100-12-31 (001 §6.4 P7; outside that range the answer is
//! [`Error::OutOfRange`], never a wrong date and never a panic), from
//! S. Janson, "Tibetan calendar mathematics", arXiv:1401.6285v1 (cited in the
//! code as "Janson (arXiv:1401.6285v1)" with section, equation and page).
//! [`System`] is an explicit parameter and has no default (`DEBT.md` D-1);
//! Genden and Tsurphu return [`Error::SystemNotImplemented`] (`DEBT.md` D-9) and
//! never fall back to Phugpa. The calendar terms are spelled as in
//! `docs/glossary.md`.
//!
//! Location dependence (001 §5.3) is a claim spec 005 checked against the
//! paper, not an assumption: the mapping takes a Gregorian date and a system
//! and nothing else, and no function in this crate reads a place, a time zone
//! or a clock.

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

pub mod arith;
mod calendar;
mod civil;
mod fault;
mod naming;
pub mod phugpa;

pub use calendar::{MonthNo, System, Tshes, ZDate};
pub use civil::{GregorianDate, Jdn};
pub use fault::Error;
pub use naming::{Animal, Element, Gender, RabjungCycle, YearInCycle, ZYear};

/// The `ZDate` of a Gregorian day under `system`.
///
/// # Errors
///
/// [`Error::OutOfRange`] outside 1900-01-01..=2100-12-31;
/// [`Error::SystemNotImplemented`] for Genden and Tsurphu.
pub fn to_zdate(date: GregorianDate, system: System) -> Result<ZDate, Error> {
    match system {
        System::Phugpa => phugpa::to_zdate(&phugpa::Epoch::E806, date.to_jdn()),
        System::Genden | System::Tsurphu => Err(Error::SystemNotImplemented),
    }
}

/// The Gregorian day of a `ZDate`, under the system the `ZDate` carries.
///
/// # Errors
///
/// [`Error::SkippedDay`] when no day carries the tshes; [`Error::InvalidField`]
/// when the fields name no calendar day; [`Error::OutOfRange`] when the day lies
/// outside 1900-01-01..=2100-12-31; [`Error::SystemNotImplemented`] for Genden
/// and Tsurphu. The order of the checks is documented on
/// [`phugpa::to_jdn`].
pub fn to_gregorian(date: ZDate) -> Result<GregorianDate, Error> {
    match date.system() {
        System::Phugpa => phugpa::to_jdn(&phugpa::Epoch::E806, &date)?.to_gregorian(),
        System::Genden | System::Tsurphu => Err(Error::SystemNotImplemented),
    }
}
