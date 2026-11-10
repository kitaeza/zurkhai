//! The Phugpa version of the Tibetan calendar, Gregorian-day ↔ `ZDate` (001 §5.1).
//!
//! Source throughout: S. Janson, "Tibetan calendar mathematics",
//! arXiv:1401.6285v1 (24 Jan 2014), cited as "Janson (arXiv:1401.6285v1)" with
//! section, equation and page. All arithmetic is exact: every rational is held
//! as an integer numerator over a stated denominator, and every division goes
//! through `crate::arith` (002 §4.1, §4.2).
//!
//! **The mapping, in one place.** Lunar days are counted by `t = 30n + d`:
//! `n` is the true month count (Janson §5, p. 14ff) and `d` the lunar day
//! 1..=30 within the month. `end_day(t)` is `⌊true_date(d, n)⌋`, the Julian day
//! number of the calendar day containing the end of lunar day `t` (§8, eq.
//! (8.1), p. 27). The calendar day with number `J` is labelled by the lunar day
//! that is current at its beginning (§6, p. 20), which is the *first* lunar day
//! `t` with `end_day(t) ≥ J`. Hence the days labelled `t` are those `J` with
//! `end_day(t − 1) < J ≤ end_day(t)`: none (a skipped tshes), one, or two (a
//! repeated tshes). `end_day` never repeats a value three times and never skips
//! two calendar days (`true_date` advances by between 0.89 and 1.07 days per
//! lunar day, §7, Remark 15, p. 26), which the exhaustive tests confirm over
//! the supported range.

use crate::arith::{amod, ceil_div, floor_div, fmod};
use crate::calendar::{MonthNo, System, Tshes, ZDate};
use crate::civil::Jdn;
use crate::fault::Error;
use crate::naming::ZYear;

/// The least Tibetan year (Gregorian-numbered, Janson §4, p. 8) that has a day
/// in the supported range 1900-01-01..=2100-12-31. A test derives it from the
/// mapping itself; `ZDate`s with an earlier year are `OutOfRange` without any
/// arithmetic (which keeps the arithmetic bounded, 002 §4.3).
pub(crate) const SUPPORTED_YEAR_MIN: i64 = 1899;
/// The greatest Tibetan year with a day in the supported range (derived by the
/// same test).
pub(crate) const SUPPORTED_YEAR_MAX: i64 = 2100;

/// The common denominator of `true_date`: `lcm(11312, 126·60, 402·60)`.
/// `mean_date` has denominator 11312 (`2·5656`, eq. (7.2)–(7.4)); the equation
/// of the moon is in units of 1/126 and divided by 60 (eqs. (7.17), (7.22));
/// the equation of the sun is in units of 1/402 and divided by 60 (eqs.
/// (7.20), (7.22)). A test checks the three multipliers below.
const TRUE_DATE_DENOMINATOR: i64 = 102_317_040;
/// `TRUE_DATE_DENOMINATOR / 11312`.
const MEAN_MULTIPLIER: i64 = 9_045;
/// The denominator of the mean date (eqs. (7.2)–(7.4): 2 · 5656).
const MEAN_DEN: i64 = 11_312;
/// `TRUE_DATE_DENOMINATOR / (126 · 60)`.
const MOON_MULTIPLIER: i64 = 13_534;
/// `TRUE_DATE_DENOMINATOR / (402 · 60)`.
const SUN_MULTIPLIER: i64 = 4_242;

// The mean motions, as integer numerators over the denominators named in the
// comments (all exact; 002 §4.1).

// Janson (arXiv:1401.6285v1) §7, eq. (7.2), p. 21: m1 = 167025/5656 = 334050/11312.
/// `m₁`, the mean length of the month, over 11312.
const M1: i64 = 167_025 * 2;
// Janson (arXiv:1401.6285v1) §7, eq. (7.3), p. 21: m2 = 11135/11312.
/// `m₂`, the mean length of the lunar day, over 11312.
const M2: i64 = 11_135;
// Janson (arXiv:1401.6285v1) §7, eq. (7.6), p. 23: s1 = 65/804 = 390/4824.
/// `s₁`, the mean motion of the sun per month, over 4824.
const S1: i64 = 65 * 6;
// Janson (arXiv:1401.6285v1) §7, eq. (7.7), p. 23: s2 = 13/4824.
/// `s₂`, the mean motion of the sun per lunar day, over 4824.
const S2: i64 = 13;
// Janson (arXiv:1401.6285v1) §7, eq. (7.12), p. 23: a1 = 253/3528.
/// `a₁`, the mean motion of the moon's anomaly per month, over 3528.
const A1: i64 = 253;
// Janson (arXiv:1401.6285v1) §7, eq. (7.13), p. 23: a2 = 1/28 = 126/3528.
/// `a₂`, the motion of the moon's anomaly per lunar day, over 3528.
const A2: i64 = 126;
// Janson (arXiv:1401.6285v1) §7, eq. (7.19), p. 24: anomaly_sun = mean_sun − 1/4, and 1/4 = 1206/4824.
/// The `1/4` subtracted from the mean sun, over 4824.
const SUN_ANOMALY_OFFSET: i64 = 1_206;
// Janson (arXiv:1401.6285v1) §7, eq. (7.17), p. 24: moon_equ = moon_tab(28 · anomaly_moon); with
// anomaly_moon = x/3528 the argument is 28x/3528 = x/126.
/// The units of `moon_tab`'s interpolation: 1/126.
const MOON_TAB_UNITS: i64 = 126;
// Janson (arXiv:1401.6285v1) §7, eq. (7.20), p. 24: sun_equ = sun_tab(12 · anomaly_sun); with
// anomaly_sun = y/4824 the argument is 12y/4824 = y/402.
/// The units of `sun_tab`'s interpolation: 1/402.
const SUN_TAB_UNITS: i64 = 402;
/// The denominator of the moon's anomaly (eqs. (7.12)–(7.14)).
const MOON_DEN: i64 = 3_528;
/// The denominator of the sun's mean longitude (eqs. (7.6)–(7.8): 804 · 6).
const SUN_DEN: i64 = 4_824;

// Janson (arXiv:1401.6285v1) §5, eq. (5.1), p. 12: 67 lunar months = 65 solar months.
/// Lunar months per leap-year cycle (the `67` of eqs. (5.10), (5.19), (5.22)).
const LUNAR_MONTHS: i64 = 67;
/// Solar months per leap-year cycle (the `65` of eqs. (5.10), (5.19), (5.22)).
const SOLAR_MONTHS: i64 = 65;
// Janson (arXiv:1401.6285v1) §5, Remark 3, p. 14: the epoch is always at the beginning of month 3, M0 = 3.
/// `M₀`, the epoch month.
const EPOCH_MONTH: i64 = 3;
// Janson (arXiv:1401.6285v1) §5, eq. (5.15), p. 17: β = 184 − β*.
/// The 184 of `β = 184 − β*`.
const BETA_BASE: i64 = 184;
// Janson (arXiv:1401.6285v1) §5, eq. (5.10), p. 16: n = ⌊(67M* + β* + 17)/65⌋ − [ℓ].
/// The 17 of eq. (5.10) (`65 − 48`: the leap-month rule (5.8), p. 15).
const ROUNDING_SHIFT: i64 = 17;

/// The epoch data of one version of the Phugpa constants: the year `Y₀` of the
/// epoch, the epoch value `β*` of the intercalation index and the three epoch
/// constants `m₀`, `s₀`, `a₀`. Different epochs give the same calendar
/// (Janson (arXiv:1401.6285v1) §5, p. 14, and §7, p. 21); the mean motions
/// `m₁`, `s₁`, `a₁` are the same for all of them. `M₀ = 3` always (§5, Remark 3,
/// p. 14).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Epoch {
    /// `Y₀`, the epoch year.
    y0: i64,
    /// `β*`, the epoch value of the intercalation index.
    beta_star: i64,
    /// `m₀` as a numerator over 11312 (the whole number of days included).
    m0: i64,
    /// `s₀` as a numerator over 4824 (`= 804 · 6`).
    s0: i64,
    /// `a₀` as a numerator over 3528.
    a0: i64,
}

impl Epoch {
    /// The epoch E806 (month 3 of year 806, the traditional Kālacakra
    /// epoch), the constants Janson uses as the base of his formulas.
    pub const E806: Self = Self {
        // Janson (arXiv:1401.6285v1) §5, p. 14: epoch month 3 year 806.
        y0: 806,
        // Janson (arXiv:1401.6285v1) §5, eq. (5.4), p. 14.
        beta_star: 61,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.4), p. 21: m0 = 2015501 + 4783/5656 = 2015501 + 9566/11312.
        m0: 2_015_501 * 11_312 + 4_783 * 2,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.8), p. 23: s0 = 743/804 = 4458/4824.
        s0: 743 * 6,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.14), p. 23: a0 = 475/3528.
        a0: 475,
    };

    /// The epoch E1927 (month 3 of year 1927; Henning's).
    pub const E1927: Self = Self {
        // Janson (arXiv:1401.6285v1) §5, p. 14: epoch month 3 year 1927.
        y0: 1927,
        // Janson (arXiv:1401.6285v1) §5, eq. (5.5), p. 14.
        beta_star: 55,
        // Janson (arXiv:1401.6285v1) §7, Remark 9, p. 22: m0 = 2424972 + 5457/5656 = 2424972 + 10914/11312.
        m0: 2_424_972 * 11_312 + 5_457 * 2,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.9), p. 23: s0 = 749/804 = 4494/4824.
        s0: 749 * 6,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.15), p. 23: a0 = 1741/3528.
        a0: 1741,
    };

    /// The epoch E1987 (month 3 of year 1987; the almanac's).
    pub const E1987: Self = Self {
        // Janson (arXiv:1401.6285v1) §5, p. 14: epoch month 3 year 1987.
        y0: 1987,
        // Janson (arXiv:1401.6285v1) §5, eq. (5.6), p. 14.
        beta_star: 0,
        // Janson (arXiv:1401.6285v1) §7, Remark 9, p. 22: m0 = 2446914 + 135/707 = 2446914 + 2160/11312.
        m0: 2_446_914 * 11_312 + 135 * 16,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.10), p. 23: s0 = 0 (modulo 1).
        s0: 0,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.16), p. 23: a0 = 38/49 = 2736/3528.
        a0: 38 * 72,
    };
}

/// The base values of `moon_tab` at the integer arguments 0..=7 (Janson
/// (arXiv:1401.6285v1) §7, eq. (7.18), p. 24): 0 5 10 15 19 22 24 25.
fn moon_tab_base(index: i64) -> i64 {
    debug_assert!((0..=7).contains(&index));
    match index {
        0 => 0,
        1 => 5,
        2 => 10,
        3 => 15,
        4 => 19,
        5 => 22,
        6 => 24,
        _ => 25,
    }
}

/// `moon_tab(k)` at any integer `k`, by the symmetry rules of Janson
/// (arXiv:1401.6285v1) §7, eq. (7.18), p. 24: `moon_tab(14 − i) = moon_tab(i)`,
/// `moon_tab(14 + i) = −moon_tab(i)`, `moon_tab(28 + i) = moon_tab(i)`.
fn moon_tab(k: i64) -> i64 {
    let k = fmod(k, 28);
    if k <= 7 {
        moon_tab_base(k)
    } else if k <= 14 {
        moon_tab_base(14 - k)
    } else if k <= 21 {
        -moon_tab_base(k - 14)
    } else {
        -moon_tab_base(28 - k)
    }
}

/// The base values of `sun_tab` at the integer arguments 0..=3 (Janson
/// (arXiv:1401.6285v1) §7, eq. (7.21), p. 24): 0 6 10 11.
fn sun_tab_base(index: i64) -> i64 {
    debug_assert!((0..=3).contains(&index));
    match index {
        0 => 0,
        1 => 6,
        2 => 10,
        _ => 11,
    }
}

/// `sun_tab(k)` at any integer `k`, by the symmetry rules of Janson
/// (arXiv:1401.6285v1) §7, eq. (7.21), p. 24: `sun_tab(6 − i) = sun_tab(i)`,
/// `sun_tab(6 + i) = −sun_tab(i)`, `sun_tab(12 + i) = sun_tab(i)`.
fn sun_tab(k: i64) -> i64 {
    let k = fmod(k, 12);
    if k <= 3 {
        sun_tab_base(k)
    } else if k <= 6 {
        sun_tab_base(6 - k)
    } else if k <= 9 {
        -sun_tab_base(k - 6)
    } else {
        -sun_tab_base(12 - k)
    }
}

/// `true_date(d, n)` for the lunar day `t = 30n + d`, as a numerator over
/// `TRUE_DATE_DENOMINATOR` (Janson (arXiv:1401.6285v1) §7, eq. (7.22), p. 24:
/// `true_date = mean_date + moon_equ/60 − sun_equ/60`).
///
/// Bound (002 §4.3): for `|t| ≤ 3 · 10^9` every product below stays under
/// `9 · 10^18`, the dominant one being `MEAN_MULTIPLIER · 11135 · |t|`
/// (about `1.0 · 10^8 · |t|`); the engine never calls it beyond the supported
/// years (about `t < 10^5`), which is why `i64` is enough and no `i128` is used.
fn true_date_numerator(epoch: &Epoch, lunar_day: i64) -> i64 {
    // d = 1..=30 within the month, n the true month count.
    let n = floor_div(lunar_day - 1, 30);
    let d = fmod(lunar_day - 1, 30) + 1;

    // mean_date = n·m1 + d·m2 + m0, Janson (arXiv:1401.6285v1) §7, eq. (7.1), p. 21, with
    // m1 = 167025/5656 = 334050/11312 (7.2), m2 = 11135/11312 (7.3).
    let mean = epoch.m0 + M1 * n + M2 * d;

    // anomaly_moon = n·a1 + d·a2 + a0, Janson (arXiv:1401.6285v1) §7, eq. (7.11), p. 23, with
    // a1 = 253/3528 (7.12), a2 = 1/28 = 126/3528 (7.13); only the fractional
    // part matters (Janson (arXiv:1401.6285v1) §2, "Angular units", p. 5: numbers "considered modulo 1").
    // moon_equ = moon_tab(28 · anomaly_moon), eq. (7.17), p. 24: with
    // anomaly_moon = x/3528 the argument is 28x/3528 = x/126.
    let moon_x = fmod(A1 * n + A2 * d + epoch.a0, MOON_DEN);
    let moon_k = floor_div(moon_x, MOON_TAB_UNITS);
    let moon_r = fmod(moon_x, MOON_TAB_UNITS);
    // Linear interpolation between integer arguments (eq. (7.18)), in units of 1/126.
    let moon_equ =
        MOON_TAB_UNITS * moon_tab(moon_k) + (moon_tab(moon_k + 1) - moon_tab(moon_k)) * moon_r;

    // mean_sun = n·s1 + d·s2 + s0, Janson (arXiv:1401.6285v1) §7, eq. (7.5), p. 22, with
    // s1 = 65/804 = 390/4824 (7.6), s2 = 13/4824 (7.7).
    // anomaly_sun = mean_sun − 1/4, eq. (7.19), p. 24 (1/4 = 1206/4824);
    // sun_equ = sun_tab(12 · anomaly_sun), eq. (7.20), p. 24: with
    // anomaly_sun = y/4824 the argument is 12y/4824 = y/402.
    let sun_y = fmod(S1 * n + S2 * d + epoch.s0 - SUN_ANOMALY_OFFSET, SUN_DEN);
    let sun_k = floor_div(sun_y, SUN_TAB_UNITS);
    let sun_r = fmod(sun_y, SUN_TAB_UNITS);
    // Linear interpolation (eq. (7.21)), in units of 1/402.
    let sun_equ = SUN_TAB_UNITS * sun_tab(sun_k) + (sun_tab(sun_k + 1) - sun_tab(sun_k)) * sun_r;

    // true_date = mean_date + moon_equ/60 − sun_equ/60, eq. (7.22), p. 24.
    MEAN_MULTIPLIER * mean + MOON_MULTIPLIER * moon_equ - SUN_MULTIPLIER * sun_equ
}

/// `⌊true_date⌋` for lunar day `t`: the JD of the calendar day containing the
/// end of that lunar day (Janson (arXiv:1401.6285v1) §8, eq. (8.1), p. 27).
fn end_day(epoch: &Epoch, lunar_day: i64) -> i64 {
    floor_div(true_date_numerator(epoch, lunar_day), TRUE_DATE_DENOMINATOR)
}

/// The month `(Y, M, leap)` of the true month count `n` (Janson
/// (arXiv:1401.6285v1) §5.3, eqs. (5.19)–(5.22), p. 17), with
/// `β = 184 − β*` (eq. (5.15)) and `M₀ = 3`.
fn month_of_count(epoch: &Epoch, count: i64) -> (i64, i64, bool) {
    let beta = BETA_BASE - epoch.beta_star;
    // x = ⌈(65n + β)/67⌉, eq. (5.19).
    let x = ceil_div(SOLAR_MONTHS * count + beta, LUNAR_MONTHS);
    // M = x amod 12, eq. (5.20).
    let month = amod(x, 12);
    // Y = (x − M)/12 + Y0, eq. (5.21).
    let year = floor_div(x - month, 12) + epoch.y0;
    // ℓ = {(65n + β) mod 67 = 1 or 2}, eq. (5.22).
    let leap = matches!(fmod(SOLAR_MONTHS * count + beta, LUNAR_MONTHS), 1 | 2);
    (year, month, leap)
}

/// The true month count `n` of month `M` of year `Y` (leap or regular) (Janson
/// (arXiv:1401.6285v1) §5, eqs. (5.2) and (5.10), pp. 14, 16):
/// `M* = 12(Y − Y₀) + M − M₀` and `n = ⌊(67M* + β* + 17)/65⌋ − [ℓ]`.
fn count_of_month(epoch: &Epoch, year: i64, month: i64, leap: bool) -> i64 {
    let solar_months = 12 * (year - epoch.y0) + month - EPOCH_MONTH;
    floor_div(
        LUNAR_MONTHS * solar_months + epoch.beta_star + ROUNDING_SHIFT,
        SOLAR_MONTHS,
    ) - i64::from(leap)
}

/// The tshes number of lunar day `t`, 1..=30.
fn tshes_of(lunar_day: i64) -> Result<Tshes, Error> {
    Tshes::new(u8::try_from(fmod(lunar_day - 1, 30) + 1).map_err(|_| Error::Internal)?)
}

/// The label of calendar day `day`: the first lunar day `t` with
/// `end(t) ≥ day` (the lunar day current at the beginning of the calendar day,
/// Janson (arXiv:1401.6285v1) §6, p. 20), where `end(t)` is `⌊true_date⌋`, the
/// calendar day on which lunar day `t` ends. `end` is a parameter so that the
/// tests can run the search with a variant of the true date.
///
/// The mean date of lunar day `t` is `(m0 + 11135·t)/11312` (`m1 = 30·m2`,
/// eqs. (7.1)–(7.3)); the true date differs from it by less than 0.6 day (the
/// tables peak at 25 and 11, divided by 60, eqs. (7.18), (7.21), (7.22)). So the
/// label lies within a few lunar days of the mean-date estimate.
fn label_of_day<F: Fn(i64) -> i64>(epoch: &Epoch, day: i64, end: F) -> Result<i64, Error> {
    let estimate = floor_div(day * MEAN_DEN - epoch.m0, M2);
    let mut label = None;
    for candidate in (estimate - 3)..=(estimate + 4) {
        if end(candidate) >= day {
            label = Some(candidate);
            break;
        }
    }
    let lunar_day = label.ok_or(Error::Internal)?;
    if end(lunar_day - 1) >= day {
        // `lunar_day` is not the first with end(t) ≥ day: the window missed it.
        return Err(Error::Internal);
    }
    Ok(lunar_day)
}

/// The `ZDate` of calendar day `day` (a JD) under `epoch`, with no range check.
fn zdate_of_day(epoch: &Epoch, day: i64) -> Result<ZDate, Error> {
    let lunar_day = label_of_day(epoch, day, |t| end_day(epoch, t))?;
    let end_this = end_day(epoch, lunar_day);
    let end_before = end_day(epoch, lunar_day - 1);
    let end_before_that = end_day(epoch, lunar_day - 2);
    let count = end_this - end_before;
    if !(1..=2).contains(&count) {
        return Err(Error::Internal);
    }
    // Janson (arXiv:1401.6285v1) §6, p. 20: of two days with the same number, the first is the leap
    // day; 001 §5.1 flags the second occurrence.
    let is_doubled = count == 2 && day == end_this;
    // A tshes is skipped when no calendar day carries it: end_day(t−1) = end_day(t−2).
    // It is "immediately before" this day when this is the first day of its label.
    let skipped_before = if day == end_before + 1 && end_before == end_before_that {
        Some(tshes_of(lunar_day - 1)?)
    } else {
        None
    };
    let (year, month, is_leap_month) = month_of_count(epoch, floor_div(lunar_day - 1, 30));
    Ok(ZDate::new(
        System::Phugpa,
        ZYear::from_year(year)?,
        MonthNo::new(u8::try_from(month).map_err(|_| Error::Internal)?)?,
        is_leap_month,
        tshes_of(lunar_day)?,
        is_doubled,
        skipped_before,
    ))
}

/// The `ZDate` of calendar day `jdn` under `epoch`.
///
/// # Errors
///
/// [`Error::OutOfRange`] if the day is outside 1900-01-01..=2100-12-31;
/// [`Error::Internal`] only if an invariant of the engine breaks.
pub fn to_zdate(epoch: &Epoch, jdn: Jdn) -> Result<ZDate, Error> {
    if !jdn.is_supported() {
        return Err(Error::OutOfRange);
    }
    zdate_of_day(epoch, jdn.get())
}

/// The calendar day of a `ZDate` under `epoch`.
///
/// The checks run in this order, and each answer is final: the year is inside
/// the supported years (`OutOfRange`); the month exists, so that a leap flag on
/// a regular month is refused (`InvalidField`); the tshes is carried by some day
/// (`SkippedDay`); a doubled flag is set only on a repeated tshes
/// (`InvalidField`); the day is inside the supported range (`OutOfRange`); and
/// the `ZDate` equals the one the engine reports for that day, so that a wrong
/// `skipped_before` is refused (`InvalidField`).
///
/// # Errors
///
/// As above; [`Error::InvalidField`] also for a `ZDate` of another system.
pub fn to_jdn(epoch: &Epoch, date: &ZDate) -> Result<Jdn, Error> {
    if date.system() != System::Phugpa {
        return Err(Error::InvalidField);
    }
    let year = date.year().number();
    if !(SUPPORTED_YEAR_MIN..=SUPPORTED_YEAR_MAX).contains(&year) {
        return Err(Error::OutOfRange);
    }
    let month = i64::from(date.month().get());
    let leap = date.is_leap_month();
    let count = count_of_month(epoch, year, month, leap);
    if month_of_count(epoch, count) != (year, month, leap) {
        return Err(Error::InvalidField);
    }
    let lunar_day = 30 * count + i64::from(date.tshes().get());
    let end_this = end_day(epoch, lunar_day);
    let days_with_label = end_this - end_day(epoch, lunar_day - 1);
    let day = match days_with_label {
        0 => return Err(Error::SkippedDay),
        1 if date.is_doubled() => return Err(Error::InvalidField),
        2 if !date.is_doubled() => end_this - 1,
        1 | 2 => end_this,
        _ => return Err(Error::Internal),
    };
    let jdn = Jdn::new(day)?;
    if !jdn.is_supported() {
        return Err(Error::OutOfRange);
    }
    if to_zdate(epoch, jdn)? != *date {
        return Err(Error::InvalidField);
    }
    Ok(jdn)
}

#[cfg(test)]
mod constant_tests {
    //! The constants, decoded from Janson's own mixed-radix forms (§2, p. 4)
    //! and compared with the numerators written above. Two forms of each
    //! constant that Janson prints, transcribed separately, must agree
    //! exactly (METHOD rule 35: this checks the transcription, not the
    //! calendar). Core-only: no `std` in `src/` (the gate's no-std check).
    use super::*;

    /// The value of `0; d1, d2, …, dn (b1, b2, …, bn)` as `(numerator, denominator)`,
    /// by Janson's inductive definition (§2, p. 4): the innermost digit first.
    fn radix_fraction(digits: &[i64], radices: &[i64]) -> (i64, i64) {
        let mut numerator = 0;
        let mut denominator = 1;
        for (digit, radix) in digits.iter().rev().zip(radices.iter().rev()) {
            numerator += digit * denominator;
            denominator *= radix;
        }
        (numerator, denominator)
    }

    /// Whether `whole + digits/radices == numerator / denominator`.
    fn decodes_to(
        whole: i64,
        digits: &[i64],
        radices: &[i64],
        numerator: i64,
        denominator: i64,
    ) -> bool {
        let (num, den) = radix_fraction(digits, radices);
        (whole * den + num) * denominator == numerator * den
    }

    const TIME: [i64; 4] = [60, 60, 6, 707];
    const LONGITUDE: [i64; 5] = [27, 60, 60, 6, 67];
    const ANOMALY: [i64; 2] = [28, 126];

    #[test]
    fn multipliers_are_the_stated_quotients() {
        assert_eq!(MEAN_MULTIPLIER * 11_312, TRUE_DATE_DENOMINATOR);
        assert_eq!(MOON_MULTIPLIER * 126 * 60, TRUE_DATE_DENOMINATOR);
        assert_eq!(SUN_MULTIPLIER * 402 * 60, TRUE_DATE_DENOMINATOR);
    }

    #[test]
    fn mean_motions_match_the_traditional_forms() {
        // (7.2): m1 = 29;31,50,0,480 (60,60,6,707) = 167025/5656 = 334050/11312.
        assert!(decodes_to(29, &[31, 50, 0, 480], &TIME, 334_050, 11_312));
        // (7.3): m2 = 0;59,3,4,16 (60,60,6,707) = 11135/11312.
        assert!(decodes_to(0, &[59, 3, 4, 16], &TIME, 11_135, 11_312));
        // (7.6): s1 = 2,10,58,1,17 (27,60,60,6,67) = 65/804 = 390/4824.
        assert!(decodes_to(0, &[2, 10, 58, 1, 17], &LONGITUDE, 390, 4_824));
        // (7.7): s2 = 0,4,21,5,43 (27,60,60,6,67) = 13/4824.
        assert!(decodes_to(0, &[0, 4, 21, 5, 43], &LONGITUDE, 13, 4_824));
        // (7.12), (7.13): a1 = 2,1 (28,126) = 253/3528, a2 = 1,0 (28,126) = 1/28 = 126/3528.
        assert!(decodes_to(0, &[2, 1], &ANOMALY, 253, 3_528));
        assert!(decodes_to(0, &[1, 0], &ANOMALY, 126, 3_528));
    }

    #[test]
    fn e806_epoch_constants_match_the_traditional_forms() {
        let e = Epoch::E806;
        // (7.4): m0 = 0;50,44,2,38 (60,60,6,707) + 2015501.
        assert!(decodes_to(2_015_501, &[50, 44, 2, 38], &TIME, e.m0, 11_312));
        // (7.8): s0 = 24,57,5,2,16 (27,60,60,6,67).
        assert!(decodes_to(0, &[24, 57, 5, 2, 16], &LONGITUDE, e.s0, 4_824));
        // (7.14): a0 = 3,97 (28,126).
        assert!(decodes_to(0, &[3, 97], &ANOMALY, e.a0, 3_528));
        assert_eq!((e.y0, e.beta_star), (806, 61));
    }

    #[test]
    fn e1927_epoch_constants_match_the_traditional_forms() {
        let e = Epoch::E1927;
        // Remark 9, p. 22: m0 = 2015501 + 409465 + 6;57,53,2,20 (60,60,6,707).
        assert!(decodes_to(
            2_015_501 + 409_465 + 6,
            &[57, 53, 2, 20],
            &TIME,
            e.m0,
            11_312
        ));
        // (7.9): s0 = 25,9,10,4,32 (27,60,60,6,67).
        assert!(decodes_to(0, &[25, 9, 10, 4, 32], &LONGITUDE, e.s0, 4_824));
        // (7.15): a0 = 13,103 (28,126).
        assert!(decodes_to(0, &[13, 103], &ANOMALY, e.a0, 3_528));
        assert_eq!((e.y0, e.beta_star), (1927, 55));
    }

    #[test]
    fn e1987_epoch_constants_match_the_traditional_forms() {
        let e = Epoch::E1987;
        // Remark 9, p. 22: m0 = 2015501 + 431410 + 3;11,27,2,332 (60,60,6,707).
        assert!(decodes_to(
            2_015_501 + 431_410 + 3,
            &[11, 27, 2, 332],
            &TIME,
            e.m0,
            11_312
        ));
        // (7.10): s0 = 0.
        assert_eq!(e.s0, 0);
        // (7.16): a0 = 21,90 (28,126).
        assert!(decodes_to(0, &[21, 90], &ANOMALY, e.a0, 3_528));
        assert_eq!((e.y0, e.beta_star), (1987, 0));
    }

    #[test]
    fn beta_matches_the_stated_values() {
        // (5.15): β = 184 − β*; (5.16)–(5.18): 123, 129, 184.
        assert_eq!(184 - Epoch::E806.beta_star, 123);
        assert_eq!(184 - Epoch::E1927.beta_star, 129);
        assert_eq!(184 - Epoch::E1987.beta_star, 184);
    }

    #[test]
    fn tables_match_the_printed_rows() {
        // (7.18): moon_tab(0..=7) = 0 5 10 15 19 22 24 25, extended by its symmetries.
        let moon: [i64; 8] = [0, 5, 10, 15, 19, 22, 24, 25];
        for (i, value) in (0_i64..8).zip(moon) {
            assert_eq!(moon_tab(i), value);
            assert_eq!(moon_tab(14 - i), value);
            assert_eq!(moon_tab(14 + i), -value);
            assert_eq!(moon_tab(28 + i), value);
        }
        // (7.21): sun_tab(0..=3) = 0 6 10 11, extended by its symmetries.
        let sun: [i64; 4] = [0, 6, 10, 11];
        for (i, value) in (0_i64..4).zip(sun) {
            assert_eq!(sun_tab(i), value);
            assert_eq!(sun_tab(6 - i), value);
            assert_eq!(sun_tab(6 + i), -value);
            assert_eq!(sun_tab(12 + i), value);
        }
    }
}

#[cfg(test)]
mod worked_examples {
    //! Janson's worked numerical examples, as an outside check (spec 005 Item 5).
    //! Each test cites the section and page; each has a seeded control with one
    //! value deliberately wrong, asserted to fail (002 §6, METHOD rule 31).
    //! Core-only: no `std` in `src/`.
    use super::*;
    use crate::civil::GregorianDate;

    const TIME: [i64; 4] = [60, 60, 6, 707];
    const LONGITUDE: [i64; 5] = [27, 60, 60, 6, 67];
    const ANOMALY: [i64; 2] = [28, 126];

    fn radix_fraction(digits: &[i64], radices: &[i64]) -> (i64, i64) {
        let mut numerator = 0;
        let mut denominator = 1;
        for (digit, radix) in digits.iter().rev().zip(radices.iter().rev()) {
            numerator += digit * denominator;
            denominator *= radix;
        }
        (numerator, denominator)
    }

    fn decodes_to(
        whole: i64,
        digits: &[i64],
        radices: &[i64],
        numerator: i64,
        denominator: i64,
    ) -> bool {
        let (num, den) = radix_fraction(digits, radices);
        (whole * den + num) * denominator == numerator * den
    }

    /// Janson (arXiv:1401.6285v1) §7, Remarks 9–11, pp. 22–23: moving the epoch
    /// by `shift` true months. "`n` differs by 13866" for E1927 and by 14609 for
    /// E1987; the constants then are `m0 + shift·m1` (exact), and `s0 + shift·s1`,
    /// `a0 + shift·a1` modulo 1.
    fn shifted_constants_match(shift: i64, target: &Epoch) -> bool {
        let base = Epoch::E806;
        let m0_ok = base.m0 + shift * 334_050 == target.m0;
        let s0_ok = fmod(base.s0 + shift * 390, 4824) == fmod(target.s0, 4824);
        let a0_ok = fmod(base.a0 + shift * 253, 3528) == fmod(target.a0, 3528);
        m0_ok && s0_ok && a0_ok
    }

    #[test]
    fn remark_9_to_11_e1927_is_e806_shifted_by_13866_months() {
        // Remark 9, p. 22: m0 + 13866·m1 = 2424972 + 5457/5656, and
        // 409465 = 7·58495 (a whole number of weeks). (7.9): s0 + 13866·s1 ≡ 749/804.
        // (7.15): a0 + 13866·a1 ≡ 1741/3528.
        assert!(shifted_constants_match(13_866, &Epoch::E1927));
        assert_eq!(
            Epoch::E806.m0 + 13_866 * 334_050,
            2_424_972 * 11_312 + 5_457 * 2
        );
        assert_eq!(fmod(409_465, 7), 0);
        assert_eq!(409_465, 7 * 58_495);
        assert_eq!(fmod(Epoch::E806.s0 + 13_866 * 390, 4824), 749 * 6);
        assert_eq!(fmod(Epoch::E806.a0 + 13_866 * 253, 3528), 1741);
    }

    #[test]
    fn remark_9_to_11_e1987_is_e806_shifted_by_14609_months() {
        // Remark 9, p. 22: m0 + 14609·m1 = 2446914 + 135/707. (7.10): s0 + 14609·s1 ≡ 0.
        // (7.16): a0 + 14609·a1 ≡ 38/49.
        assert!(shifted_constants_match(14_609, &Epoch::E1987));
        assert_eq!(
            Epoch::E806.m0 + 14_609 * 334_050,
            2_446_914 * 11_312 + 135 * 16
        );
        assert_eq!(fmod(Epoch::E806.s0 + 14_609 * 390, 4824), 0);
        assert_eq!(fmod(Epoch::E806.a0 + 14_609 * 253, 3528), 38 * 72);
    }

    #[test]
    fn seeded_control_a_wrong_epoch_shift_fails() {
        // The same example with one value deliberately wrong (13867 for 13866,
        // 14610 for 14609): the check must fail.
        assert!(!shifted_constants_match(13_867, &Epoch::E1927));
        assert!(!shifted_constants_match(14_610, &Epoch::E1987));
        assert!(!shifted_constants_match(13_866, &Epoch::E1987));
    }

    #[test]
    fn table_6_phugpa_epoch_data_is_e806_one_month_later() {
        // Table 6, p. 50: the Phugpa epoch values for JD 2015531 (23 March 806),
        // in Tibetan form, given modulo 7 for m0 ("add 2015529" for JD). They are
        // the E806 constants after one true month (n = 1, d = 0).
        let base = Epoch::E806;
        // m0 = 2;22,34,2,518 (60,60,6,707) + 2015529
        assert!(decodes_to(
            2_015_529 + 2,
            &[22, 34, 2, 518],
            &TIME,
            base.m0 + 334_050,
            11_312
        ));
        // s0 = 0,8,3,3,33 (27,60,60,6,67)
        assert!(decodes_to(
            0,
            &[0, 8, 3, 3, 33],
            &LONGITUDE,
            fmod(base.s0 + 390, 4824),
            4824
        ));
        // a0 = 5,98 (28,126)
        assert!(decodes_to(
            0,
            &[5, 98],
            &ANOMALY,
            fmod(base.a0 + 253, 3528),
            3528
        ));
        // The seeded control: one digit wrong (519 for 518) must not decode to it.
        assert!(!decodes_to(
            2_015_529 + 2,
            &[22, 34, 2, 519],
            &TIME,
            base.m0 + 334_050,
            11_312
        ));
    }

    /// The Tibetan date `(year, month, leap, tshes)` of a JD under E806, with no
    /// range check (these epochs lie outside the supported range).
    fn e806_date(day: i64) -> (i64, i64, bool, i64) {
        let epoch = Epoch::E806;
        match label_of_day(&epoch, day, |t| end_day(&epoch, t)) {
            Ok(lunar_day) => {
                let (year, month, leap) = month_of_count(&epoch, floor_div(lunar_day - 1, 30));
                (year, month, leap, fmod(lunar_day - 1, 30) + 1)
            }
            Err(_) => (0, 0, false, 0),
        }
    }

    #[test]
    fn epoch_days_of_e806_have_the_dates_janson_states() {
        // §7, Remark 16 and p. 27: the epoch is the day floor(m0) = JD 2015501,
        // and its Tibetan date is 29/1 806 ("29/1 806, 29/1 1927 and 1/3 1987").
        assert_eq!(e806_date(2_015_501), (806, 1, false, 29));
        // §A.13, p. 50, footnote 64: the Table 6 epoch JD 2015531 "is day 30 in
        // month 2 for the Phugpa version".
        assert_eq!(e806_date(2_015_531), (806, 2, false, 30));
        // The seeded control: the next day is not 29/1 806.
        assert_ne!(e806_date(2_015_502), (806, 1, false, 29));
    }

    #[test]
    fn epoch_days_of_e1927_and_e1987_have_the_dates_janson_states() {
        // p. 27: the Tibetan dates of the epoch days are 29/1 1927 and 1/3 1987;
        // the JDs are floor(m0) = 2424972 (p. 68, (D.2): Friday 1 April 1927)
        // and 2446914.
        let one = |epoch: Epoch, day: i64| match label_of_day(&epoch, day, |t| end_day(&epoch, t)) {
            Ok(lunar_day) => {
                let (year, month, leap) = month_of_count(&epoch, floor_div(lunar_day - 1, 30));
                (year, month, leap, fmod(lunar_day - 1, 30) + 1)
            }
            Err(_) => (0, 0, false, 0),
        };
        assert_eq!(one(Epoch::E1927, 2_424_972), (1927, 1, false, 29));
        assert_eq!(one(Epoch::E1987, 2_446_914), (1987, 3, false, 1));
        // The seeded controls: one day later is not the stated date.
        assert_ne!(one(Epoch::E1927, 2_424_973), (1927, 1, false, 29));
        assert_ne!(one(Epoch::E1987, 2_446_915), (1987, 3, false, 1));
    }

    /// The end day of lunar day `t` with the exact anomaly increment of Janson
    /// §7, eq. (7.24), p. 25: `a2 = (1 + a1)/30 = 3781/105840` in place of the
    /// customary `a2 = 1/28` (7.13). The denominators are 11312 (mean date),
    /// 3780·60 (moon) and 402·60 (sun); their lcm is 1534755600.
    fn end_day_exact_a2(epoch: &Epoch, lunar_day: i64) -> i64 {
        let n = floor_div(lunar_day - 1, 30);
        let d = fmod(lunar_day - 1, 30) + 1;
        let mean = epoch.m0 + M1 * n + M2 * d;
        // anomaly_moon = (7590 n + 3781 d + 30·a0)/105840 (a1 = 253/3528 = 7590/105840)
        let moon_x = fmod(7_590 * n + 3_781 * d + 30 * epoch.a0, 105_840);
        let moon_k = floor_div(moon_x, 3_780);
        let moon_r = fmod(moon_x, 3_780);
        let moon = 3_780 * moon_tab(moon_k) + (moon_tab(moon_k + 1) - moon_tab(moon_k)) * moon_r;
        let sun_y = fmod(390 * n + 13 * d + epoch.s0 - 1206, 4824);
        let sun_k = floor_div(sun_y, 402);
        let sun_r = fmod(sun_y, 402);
        let sun = 402 * sun_tab(sun_k) + (sun_tab(sun_k + 1) - sun_tab(sun_k)) * sun_r;
        floor_div(135_675 * mean + 6_767 * moon - 63_630 * sun, 1_534_755_600)
    }

    fn day_of(year: i64, month: u8, day: u8) -> i64 {
        match GregorianDate::new(year, month, day) {
            Ok(date) => date.to_jdn().get(),
            Err(_) => 0,
        }
    }

    /// Whether the two versions of `a2` give different Tibetan dates to `day`.
    fn versions_differ(epoch: &Epoch, day: i64) -> bool {
        let standard = label_of_day(epoch, day, |t| end_day(epoch, t));
        let exact = label_of_day(epoch, day, |t| end_day_exact_a2(epoch, t));
        match (standard, exact) {
            (Ok(a), Ok(b)) => a != b,
            _ => false,
        }
    }

    #[test]
    fn true_date_stays_in_i64_and_near_the_mean_date_at_its_stated_bound() {
        // 002 §4.3, METHOD rules 34 and 37: the doc comment of `true_date_numerator`
        // says no product overflows `i64` for |t| <= 3·10^9; tested at both ends
        // (overflow checks are on in every profile, so an overflow would panic). At
        // the bound `true_date` must still be within 0.6 day of the mean date
        // (m0 + 11135·t)/11312, and `end_day` must still advance by 0, 1 or 2 per
        // lunar day.
        let epoch = Epoch::E806;
        for edge in [-3_000_000_000_i64, 3_000_000_000] {
            let mut previous = end_day(&epoch, edge);
            for step in 1..=60 {
                let day = end_day(&epoch, edge + step);
                assert!(
                    (0..=2).contains(&(day - previous)),
                    "end_day step at t = {}",
                    edge + step
                );
                previous = day;
            }
            let mean_days = floor_div(epoch.m0 + M2 * edge, MEAN_DEN);
            assert!(
                (end_day(&epoch, edge) - mean_days).abs() <= 1,
                "end_day vs mean date at t = {edge}"
            );
        }
    }

    #[test]
    fn exact_a2_multipliers_are_the_stated_quotients() {
        assert_eq!(11_312 * 135_675, 1_534_755_600);
        assert_eq!(3_780 * 60 * 6_767, 1_534_755_600);
        assert_eq!(402 * 60 * 63_630, 1_534_755_600);
    }

    #[test]
    fn remark_14_the_two_versions_of_a2_differ_on_the_days_janson_lists() {
        // §7, Remark 14, p. 26: "Some recent examples are 10 February 2001
        // (JD 2451951) and 10 May 2006 (JD 2453866); the next example is 19 November
        // 2025 (JD 2460999)." These JDs are the paper's own; the Gregorian dates
        // are checked against the closed form.
        let epoch = Epoch::E806;
        assert_eq!(day_of(2001, 2, 10), 2_451_951);
        assert_eq!(day_of(2006, 5, 10), 2_453_866);
        assert_eq!(day_of(2025, 11, 19), 2_460_999);
        for listed in [2_451_951, 2_453_866, 2_460_999] {
            assert!(versions_differ(&epoch, listed), "JD {listed}");
        }
        // The seeded control: the neighbouring days are not in the list.
        for neighbour in [
            2_451_950, 2_451_952, 2_453_865, 2_453_867, 2_460_998, 2_461_000,
        ] {
            assert!(!versions_differ(&epoch, neighbour), "JD {neighbour}");
        }
    }

    #[test]
    fn remark_14_footnote_27_the_two_versions_differ_on_9_and_8_days() {
        // §7, Remark 14, p. 26 and footnote 27: "a computer search finding 9
        // examples in 1900–1999 and 8 examples in 2000–2099".
        let epoch = Epoch::E806;
        let count = |from: i64, to: i64| {
            let mut found = 0;
            let mut day = from;
            while day <= to {
                if versions_differ(&epoch, day) {
                    found += 1;
                }
                day += 1;
            }
            found
        };
        assert_eq!(
            count(day_of(1900, 1, 1), day_of(1999, 12, 31)),
            9,
            "examples in 1900-1999"
        );
        assert_eq!(
            count(day_of(2000, 1, 1), day_of(2099, 12, 31)),
            8,
            "examples in 2000-2099"
        );
    }
}
