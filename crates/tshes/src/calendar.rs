//! The zurkhai date (001 §5.1) and the small types it is made of (002 §4.7).

use crate::fault::Error;
use crate::naming::ZYear;

/// The computation system (001 §4, §5.1). It is an explicit parameter
/// everywhere and has no default (`DEBT.md` D-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum System {
    /// The Phugpa version, the standard Tibetan calendar (Janson
    /// (arXiv:1401.6285v1) §1, p. 1; Appendix A.1, p. 38).
    Phugpa,
    /// The Mongolian New Genden version (Janson (arXiv:1401.6285v1) Appendix
    /// A.3, p. 41; the column headed "Mongolia" in Tables 6–9, pp. 50–54).
    Genden,
    /// The Tsurphu version (Janson (arXiv:1401.6285v1) Appendix A.2, p. 38); the
    /// version of A.2, not the karana-Sun variant of Remark 22, p. 40 (`DEBT.md` D-19).
    Tsurphu,
}

/// A tshes (lunar day) number, 1..=30 (Janson (arXiv:1401.6285v1) §3, p. 7:
/// "The lunar days are numbered by 1–30"). The field is private (002 §4.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tshes(u8);

impl Tshes {
    /// A tshes number.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidField`] outside 1..=30.
    pub fn new(value: u8) -> Result<Self, Error> {
        if (1..=30).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Error::InvalidField)
        }
    }

    /// The number.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// A month number, 1..=12 (Janson (arXiv:1401.6285v1) §5, p. 11: "The regular
/// months in each year are numbered 1–12"). The field is private (002 §4.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonthNo(u8);

impl MonthNo {
    /// A month number.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidField`] outside 1..=12.
    pub fn new(value: u8) -> Result<Self, Error> {
        if (1..=12).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Error::InvalidField)
        }
    }

    /// The number.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// A zurkhai date: 001 §5.1's `ZDate`.
///
/// `is_doubled` is true on the *second* occurrence of a repeated tshes (001
/// §5.1); Janson calls the *first* of the two days the leap day (§6, p. 20), so
/// the two flags are not the same day. `skipped_before` is the tshes that no
/// day carries and that fell immediately before this day, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZDate {
    system: System,
    year: ZYear,
    month: MonthNo,
    is_leap_month: bool,
    tshes: Tshes,
    is_doubled: bool,
    skipped_before: Option<Tshes>,
}

impl ZDate {
    /// A date from its fields. Every component is validated by its own
    /// constructor, so this cannot fail; whether the combination names a
    /// calendar day is for [`crate::to_gregorian`] to say.
    #[must_use]
    pub const fn new(
        system: System,
        year: ZYear,
        month: MonthNo,
        is_leap_month: bool,
        tshes: Tshes,
        is_doubled: bool,
        skipped_before: Option<Tshes>,
    ) -> Self {
        Self {
            system,
            year,
            month,
            is_leap_month,
            tshes,
            is_doubled,
            skipped_before,
        }
    }

    /// The computation system.
    #[must_use]
    pub const fn system(&self) -> System {
        self.system
    }

    /// The year with its names.
    #[must_use]
    pub const fn year(&self) -> ZYear {
        self.year
    }

    /// The month number, 1..=12.
    #[must_use]
    pub const fn month(&self) -> MonthNo {
        self.month
    }

    /// Whether the month is a leap month (the first of two months with the
    /// same number, Janson (arXiv:1401.6285v1) §5, p. 11).
    #[must_use]
    pub const fn is_leap_month(&self) -> bool {
        self.is_leap_month
    }

    /// The tshes, 1..=30.
    #[must_use]
    pub const fn tshes(&self) -> Tshes {
        self.tshes
    }

    /// Whether this is the second occurrence of a repeated tshes.
    #[must_use]
    pub const fn is_doubled(&self) -> bool {
        self.is_doubled
    }

    /// The tshes omitted immediately before this day, if any.
    #[must_use]
    pub const fn skipped_before(&self) -> Option<Tshes> {
        self.skipped_before
    }
}
