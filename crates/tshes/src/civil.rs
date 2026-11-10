//! Gregorian dates and Julian day numbers.
//!
//! Janson (arXiv:1401.6285v1) defines the Julian day number JD (§2, p. 5) and
//! states two anchors, but gives no conversion formula (it refers to the
//! literature); the Tibetan day is numbered by JD throughout (§6, p. 20; §8,
//! eq. (8.1), p. 27). The conversion below is therefore derived here, from the
//! Gregorian leap-year rule, and anchored on the two values the paper states:
//!
//! * `RD 1` is 1 January 1 (Gregorian) and is `JD 1721426`, so `JD = RD +
//!   1721425` (Janson §2, p. 5, footnote 8);
//! * 1 January 2007 has `JD 2454102` (Janson §2, p. 6).
//!
//! **Derivation.** A Gregorian year `y` is a leap year iff it is divisible by 4
//! and not by 100, or divisible by 400. The days before 1 January of year `y`
//! are `365(y − 1) + ⌊(y − 1)/4⌋ − ⌊(y − 1)/100⌋ + ⌊(y − 1)/400⌋`; adding the
//! days of the earlier months of year `y` (a leap day counting from March) and
//! the day of the month gives `RD`, with `RD 1` = 1 January 1. The tests check
//! both anchors, and count every day from 1900-01-01 to 2100-12-31 one by one
//! against this closed form.

use crate::arith::{floor_div, fmod};
use crate::fault::Error;

/// `JD = RD + 1721425` (Janson (arXiv:1401.6285v1) §2, p. 5, footnote 8:
/// RD 1 is JD 1721426).
const JD_MINUS_RD: i64 = 1_721_425;

/// The bound on `|year|` that `GregorianDate` accepts. With it every
/// intermediate value stays far below `i64::MAX`: a year of 10^9 is about
/// 3.65 · 10^11 days (the engine's own bound, stated here so that `i64` is
/// provably enough, 002 §4.3).
const YEAR_LIMIT: i64 = 1_000_000_000;

/// The JD of the last day of the domain, 31 December of year `YEAR_LIMIT`
/// (a test derives it from the closed form).
const JDN_DOMAIN_MAX: i64 = 365_244_221_425;

/// The JD of the first day of the domain, 1 January of year `-YEAR_LIMIT`
/// (a test derives it from the closed form).
const JDN_DOMAIN_MIN: i64 = -365_240_778_940;

/// The Gregorian leap-year rule.
const fn is_leap(year: i64) -> bool {
    (fmod(year, 4) == 0 && fmod(year, 100) != 0) || fmod(year, 400) == 0
}

/// Days in a Gregorian month; 0 for a month number outside 1..=12.
const fn days_in_month(year: i64, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Days of the year that precede the first of `month` (non-leap table, plus
/// the leap day from March on).
const fn days_before_month(year: i64, month: u8) -> i64 {
    let before: i64 = match month {
        1 => 0,
        2 => 31,
        3 => 59,
        4 => 90,
        5 => 120,
        6 => 151,
        7 => 181,
        8 => 212,
        9 => 243,
        10 => 273,
        11 => 304,
        _ => 334,
    };
    if month >= 3 && is_leap(year) {
        before + 1
    } else {
        before
    }
}

/// A date of the proleptic Gregorian calendar.
///
/// The fields are private and the constructor validates (002 §4.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GregorianDate {
    year: i64,
    month: u8,
    day: u8,
}

impl GregorianDate {
    /// A date from year, month (1..=12) and day of the month.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidField`] for a month outside 1..=12 or a day that the
    /// month does not have; [`Error::OutOfRange`] for `|year|` above 10^9,
    /// the bound that keeps the day arithmetic inside `i64`.
    pub fn new(year: i64, month: u8, day: u8) -> Result<Self, Error> {
        if year.unsigned_abs() > YEAR_LIMIT.unsigned_abs() {
            return Err(Error::OutOfRange);
        }
        if day == 0 || day > days_in_month(year, month) {
            return Err(Error::InvalidField);
        }
        Ok(Self { year, month, day })
    }

    /// The year (astronomical numbering: year 1 is followed by year 0).
    #[must_use]
    pub const fn year(self) -> i64 {
        self.year
    }

    /// The month, 1..=12.
    #[must_use]
    pub const fn month(self) -> u8 {
        self.month
    }

    /// The day of the month, 1..=31.
    #[must_use]
    pub const fn day(self) -> u8 {
        self.day
    }

    /// The Julian day number of this date.
    ///
    /// `RD = 365(y − 1) + ⌊(y − 1)/4⌋ − ⌊(y − 1)/100⌋ + ⌊(y − 1)/400⌋ +
    /// (days before the month) + day`, and `JD = RD + 1721425` (module
    /// documentation: derivation and anchors).
    #[must_use]
    pub fn to_jdn(self) -> Jdn {
        let prior = self.year - 1;
        let rd = 365 * prior + floor_div(prior, 4) - floor_div(prior, 100)
            + floor_div(prior, 400)
            + days_before_month(self.year, self.month)
            + i64::from(self.day);
        Jdn(rd + JD_MINUS_RD)
    }
}

/// A Julian day number (Janson (arXiv:1401.6285v1) §2, p. 5): the integer
/// count of days from the epoch 1 January 4713 BC (Julian), 1 January 2007
/// being 2454102 (p. 6). The field is private (002 §4.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Jdn(i64);

impl Jdn {
    /// The JD of the first supported day, 1900-01-01 (checked by a test
    /// against `GregorianDate::to_jdn`).
    pub(crate) const SUPPORTED_MIN: i64 = 2_415_021;
    /// The JD of the last supported day, 2100-12-31 (checked likewise).
    pub(crate) const SUPPORTED_MAX: i64 = 2_488_434;

    /// A Julian day number.
    ///
    /// # Errors
    ///
    /// [`Error::OutOfRange`] outside the days of the Gregorian years
    /// `-10^9..=10^9` (JD -365240778940 to 365244221425): the domain on which
    /// the conversions to and from Gregorian dates are total and inside `i64`.
    pub fn new(value: i64) -> Result<Self, Error> {
        if !(JDN_DOMAIN_MIN..=JDN_DOMAIN_MAX).contains(&value) {
            return Err(Error::OutOfRange);
        }
        Ok(Self(value))
    }

    /// The number.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Whether the day lies inside the supported range (1900-01-01 to
    /// 2100-12-31, 001 §6.4 P7).
    #[must_use]
    pub const fn is_supported(self) -> bool {
        self.0 >= Self::SUPPORTED_MIN && self.0 <= Self::SUPPORTED_MAX
    }

    /// The Gregorian date of this day: the inverse of
    /// [`GregorianDate::to_jdn`], by 400-, 100-, 4- and 1-year cycles.
    ///
    /// # Errors
    ///
    /// [`Error::Internal`] only if the engine's own invariants break; the
    /// tests run every day of the supported range and the extremes.
    pub fn to_gregorian(self) -> Result<GregorianDate, Error> {
        // Days since 1 January 1 (RD 1 is day 0).
        let days = self.0 - JD_MINUS_RD - 1;
        let cycles400 = floor_div(days, 146_097);
        let rest400 = fmod(days, 146_097);
        // Within a 400-year cycle the last century has 36525 days, the others 36524.
        let centuries = floor_div(rest400, 36_524).min(3);
        let rest100 = rest400 - centuries * 36_524;
        let quads = floor_div(rest100, 1_461);
        let rest4 = rest100 - quads * 1_461;
        // Within a 4-year cycle the last year has 366 days, the others 365.
        let years = floor_div(rest4, 365).min(3);
        let mut day_of_year = rest4 - years * 365;
        let year = 400 * cycles400 + 100 * centuries + 4 * quads + years + 1;
        let mut month: u8 = 1;
        while month < 12 && day_of_year >= i64::from(days_in_month(year, month)) {
            day_of_year -= i64::from(days_in_month(year, month));
            month += 1;
        }
        let day = u8::try_from(day_of_year + 1).map_err(|_| Error::Internal)?;
        GregorianDate::new(year, month, day).map_err(|_| Error::Internal)
    }
}
