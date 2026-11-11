//! The Phugpa version of the Tibetan calendar, Gregorian-day ↔ `ZDate` (001 §5.1),
//! and the Tsurphu and Genden versions, which Janson describes as Phugpa plus
//! differences (Appendix A.2, A.3): one shared mapping, with per-version data in
//! [`Epoch`]. The module keeps the name of the base version.
//!
//! Source throughout: S. Janson, "Tibetan calendar mathematics",
//! arXiv:1401.6285v1 (24 Jan 2014), cited as "Janson (arXiv:1401.6285v1)" with
//! section, equation and page. All arithmetic is exact: every rational is held
//! as an integer numerator over a stated denominator, and every division goes
//! through `crate::arith` (002 §4.1, §4.2).
//!
//! **What differs between the versions** (and is therefore the only data that
//! differs here): the epoch constants `m₀`, `s₀`, `a₀`; the epoch year `Y₀`; the
//! epoch value `β*` of the intercalation index and its companion `β`; and the
//! leap-month rule, held as its rounding shift (17 for Phugpa, 0 for Tsurphu,
//! 19 for Genden). The mean motions, the tables, the equations (7.1)–(7.22), the
//! lunar-day labelling (§6) and the day formula (8.1) are the same for all three
//! (Janson (arXiv:1401.6285v1) Appendix A.2, p. 38; A.3, p. 42).
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
// The leap-month rule of each version, held as the rounding shift `c` of
// n = ⌊(67M* + β* + c)/65⌋ − [ℓ]: the true month (5.3) is rounded up when the
// intercalation index is at least `65 − c`, except that a leap month always
// rounds down (the rule (5.9) and its versions).
/// Phugpa: `c = 17`, i.e. `65 − 48`.
// Janson (arXiv:1401.6285v1) §5, eqs. (5.9), (5.10), p. 15–16: n = ⌊(67M* + β* + 17)/65⌋ − [ℓ] ("Since 48 = 65 − 17").
const PHUGPA_SHIFT: i64 = 17;
/// Tsurphu: `c = 0`: a regular month rounds down and a leap month subtracts 1.
// Janson (arXiv:1401.6285v1) Appendix A.2, rule (A.9) and the sentence after it, p. 39: "a leap month is
// inserted when the intercalation index ix = 0 or 1", "for a regular month, the true month count n is
// obtained from the true month (5.3) by rounding down to the nearest integer; for a leap month we further
// subtract 1".
const TSURPHU_SHIFT: i64 = 0;
/// Genden: `c = 19`, i.e. `65 − 46`.
// Janson (arXiv:1401.6285v1) Appendix A.3, rule (A.17) and the sentence after it, p. 43: "A leap month is
// inserted when the intercalation index ix = 46 or 47. (There is also a corresponding change of (5.9), with
// 48 replaced by 46. …)"; so (5.10) holds with 17 replaced by 65 − 46 = 19.
const GENDEN_SHIFT: i64 = 19;

/// The common grain of an epoch's constants. Phugpa and Genden hold `m₀` over
/// 11312 and `s₀` over 4824; Tsurphu's `m₀` is over 7 635 600 = 11312 · 675 and
/// its `s₀` over 108 540, which needs the finer denominator 217 080 = 4824 · 45
/// (Janson (arXiv:1401.6285v1) A.2, eqs. (A.3), (A.4), (A.6), (A.7), p. 39: the
/// traditional Tsurphu radices have one more term, Remark 20, p. 39). `true_date`
/// is then a numerator over `den`, the least common multiple of the three
/// denominators of eq. (7.22) (mean date, equation of the moon over 126 · 60,
/// equation of the sun over 402 · `sun_factor` · 60).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Grain {
    /// The denominator of `true_date`.
    den: i64,
    /// `den / (11312 · mean_factor)`.
    mean_mult: i64,
    /// `m₀`'s denominator is `11312 · mean_factor`.
    mean_factor: i64,
    /// `den / (126 · 60)`.
    moon_mult: i64,
    /// `den / (402 · sun_factor · 60)`.
    sun_mult: i64,
    /// `s₀`'s denominator is `4824 · sun_factor`.
    sun_factor: i64,
}

impl Grain {
    /// Phugpa's and Genden's constants (the values of round 005).
    const COARSE: Self = Self {
        den: TRUE_DATE_DENOMINATOR,
        mean_mult: MEAN_MULTIPLIER,
        mean_factor: 1,
        moon_mult: MOON_MULTIPLIER,
        sun_mult: SUN_MULTIPLIER,
        sun_factor: 1,
    };
    /// Tsurphu's constants: `den = 15 · 102 317 040 = 1 534 755 600`.
    const FINE: Self = Self {
        den: 1_534_755_600,
        mean_mult: 201,
        mean_factor: 675,
        moon_mult: 203_010,
        sun_mult: 1_414,
        sun_factor: 45,
    };
}

/// The epoch data of one version of the calendar: the version, the year `Y₀` of
/// the epoch, the epoch value `β*` of the intercalation index with its companion
/// `β`, the leap-month rule's rounding shift, and the three epoch constants
/// `m₀`, `s₀`, `a₀`. Different epochs of one version give the same calendar
/// (Janson (arXiv:1401.6285v1) §5, p. 14, §7, p. 21; Appendix A.2, p. 39); the
/// mean motions `m₁`, `s₁`, `a₁` are the same for all versions (A.2, p. 38; A.3,
/// p. 42). `M₀ = 3` always (§5, Remark 3, p. 14).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Epoch {
    /// The version this epoch belongs to.
    system: System,
    /// `Y₀`, the epoch year.
    y0: i64,
    /// `β*`, the epoch value of the intercalation index.
    beta_star: i64,
    /// `β`, the constant of eqs. (5.19), (5.22): `201 − shift − β*` (a test
    /// derives it and compares it with the paper's printed values).
    beta: i64,
    /// The rounding shift `c` of the leap-month rule (see `PHUGPA_SHIFT`).
    shift: i64,
    /// The grain of `m₀` and `s₀` and of the `true_date` numerator.
    grain: Grain,
    /// `m₀` as a numerator over `11312 · grain.mean_factor` (the whole number of days included).
    m0: i64,
    /// `s₀` as a numerator over `4824 · grain.sun_factor` (`4824 = 804 · 6`).
    s0: i64,
    /// `a₀` as a numerator over 3528.
    a0: i64,
}

impl Epoch {
    /// The epoch E806 (month 3 of year 806, the traditional Kālacakra
    /// epoch), the constants Janson uses as the base of his formulas.
    pub const E806: Self = Self {
        system: System::Phugpa,
        // Janson (arXiv:1401.6285v1) §5, p. 14: epoch month 3 year 806.
        y0: 806,
        // Janson (arXiv:1401.6285v1) §5, eq. (5.4), p. 14.
        beta_star: 61,
        // Janson (arXiv:1401.6285v1) §5, eq. (5.16), p. 17: β = 123 (E806).
        beta: 123,
        shift: PHUGPA_SHIFT,
        grain: Grain::COARSE,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.4), p. 21: m0 = 2015501 + 4783/5656 = 2015501 + 9566/11312.
        m0: 2_015_501 * 11_312 + 4_783 * 2,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.8), p. 23: s0 = 743/804 = 4458/4824.
        s0: 743 * 6,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.14), p. 23: a0 = 475/3528.
        a0: 475,
    };

    /// The epoch E1927 (month 3 of year 1927; Henning's).
    pub const E1927: Self = Self {
        system: System::Phugpa,
        // Janson (arXiv:1401.6285v1) §5, p. 14: epoch month 3 year 1927.
        y0: 1927,
        // Janson (arXiv:1401.6285v1) §5, eq. (5.5), p. 14.
        beta_star: 55,
        // Janson (arXiv:1401.6285v1) §5, eq. (5.17), p. 17: β = 129 (E1927).
        beta: 129,
        shift: PHUGPA_SHIFT,
        grain: Grain::COARSE,
        // Janson (arXiv:1401.6285v1) §7, Remark 9, p. 22: m0 = 2424972 + 5457/5656 = 2424972 + 10914/11312.
        m0: 2_424_972 * 11_312 + 5_457 * 2,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.9), p. 23: s0 = 749/804 = 4494/4824.
        s0: 749 * 6,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.15), p. 23: a0 = 1741/3528.
        a0: 1741,
    };

    /// The epoch E1987 (month 3 of year 1987; the almanac's).
    pub const E1987: Self = Self {
        system: System::Phugpa,
        // Janson (arXiv:1401.6285v1) §5, p. 14: epoch month 3 year 1987.
        y0: 1987,
        // Janson (arXiv:1401.6285v1) §5, eq. (5.6), p. 14.
        beta_star: 0,
        // Janson (arXiv:1401.6285v1) §5, eq. (5.18), p. 17: β = 184 (E1987).
        beta: 184,
        shift: PHUGPA_SHIFT,
        grain: Grain::COARSE,
        // Janson (arXiv:1401.6285v1) §7, Remark 9, p. 22: m0 = 2446914 + 135/707 = 2446914 + 2160/11312.
        m0: 2_446_914 * 11_312 + 135 * 16,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.10), p. 23: s0 = 0 (modulo 1).
        s0: 0,
        // Janson (arXiv:1401.6285v1) §7, eq. (7.16), p. 23: a0 = 38/49 = 2736/3528.
        a0: 38 * 72,
    };

    /// The Tsurphu epoch E1732 (month 3 of year 1732; JD 2353745, Wednesday 26
    /// March 1732 Gregorian), the first of the two epochs Janson gives for the
    /// Tsurphu version; the engine's `System::Tsurphu` uses it.
    pub const E1732: Self = Self {
        system: System::Tsurphu,
        // Janson (arXiv:1401.6285v1) Appendix A.2, p. 39: "We denote these by E1732 and E1852": the
        // naming of p. 14 (E806 = month 3 of year 806), so Y0 = 1732; confirmed by γ = 55 (p. 40) through
        // eq. (5.30), p. 18, which a test checks.
        y0: 1732,
        // Janson (arXiv:1401.6285v1) Appendix A.2, p. 39: "β* = 59 (E1732)".
        beta_star: 59,
        // Janson (arXiv:1401.6285v1) Appendix A.2, p. 40: "β = 142 (E1732)"; C.3, p. 63.
        beta: 142,
        shift: TSURPHU_SHIFT,
        grain: Grain::FINE,
        // Janson (arXiv:1401.6285v1) A.2, eq. (A.3), p. 39: m0 = 2353745 + 1795153/7635600, and 7635600 = 11312 · 675.
        m0: 2_353_745 * 7_635_600 + 1_795_153,
        // Janson (arXiv:1401.6285v1) A.2, eq. (A.4), p. 39: s0 = −5983/108540 = −11966/217080 (as printed;
        // Remark 28, p. 62, adds 1, which changes nothing modulo 1: a test shows it, eq. (C.42), p. 63).
        s0: -11_966,
        // Janson (arXiv:1401.6285v1) A.2, eq. (A.5), p. 39: a0 = 207/392 = 1863/3528.
        a0: 1_863,
    };

    /// The Tsurphu epoch E1852 (month 3 of year 1852; JD 2397598, Monday 19
    /// April 1852 Gregorian): "equivalent and giving the same calendar" as E1732
    /// (Janson (arXiv:1401.6285v1) A.2, p. 39). Held so that a test can compare the two.
    pub const E1852: Self = Self {
        system: System::Tsurphu,
        // Janson (arXiv:1401.6285v1) Appendix A.2, p. 39: "We denote these by E1732 and E1852"; γ = 55 (p. 40).
        y0: 1852,
        // Janson (arXiv:1401.6285v1) Appendix A.2, p. 39: "β* = … 14 (E1852)".
        beta_star: 14,
        // Janson (arXiv:1401.6285v1) Appendix A.2, p. 40: "β = … 187 (E1852)"; C.3, p. 63.
        beta: 187,
        shift: TSURPHU_SHIFT,
        grain: Grain::FINE,
        // Janson (arXiv:1401.6285v1) A.2, eq. (A.6), p. 39: m0 = 2397598 + 1197103/7635600.
        m0: 2_397_598 * 7_635_600 + 1_197_103,
        // Janson (arXiv:1401.6285v1) A.2, eq. (A.7), p. 39: s0 = 23/27135 = 184/217080.
        s0: 184,
        // Janson (arXiv:1401.6285v1) A.2, eq. (A.8), p. 39: a0 = 1/49 = 72/3528.
        a0: 72,
    };

    /// The Genden epoch E1747 (JD 2359237, Sunday 9 April 1747 Gregorian), the
    /// epoch Yeshe Paljor used (Janson (arXiv:1401.6285v1) A.3, p. 42, footnote 55);
    /// the engine's `System::Genden` uses it.
    pub const E1747: Self = Self {
        system: System::Genden,
        // Janson (arXiv:1401.6285v1) Appendix A.3, p. 42: JD 2359237 (9 April 1747) and footnote 55: "Yeshe
        // Paljor used the epoch 1747"; the naming of p. 14 gives Y0 = 1747; confirmed by γ = 55 (p. 43)
        // through eq. (5.30), p. 18, which a test checks.
        y0: 1747,
        // Janson (arXiv:1401.6285v1) Appendix A.3, p. 42: "β* = 10".
        beta_star: 10,
        // Janson (arXiv:1401.6285v1) Appendix A.3, p. 43: "we take β = 172 ≡ 42 (mod 65)"; C.4, p. 63.
        beta: 172,
        shift: GENDEN_SHIFT,
        grain: Grain::COARSE,
        // Janson (arXiv:1401.6285v1) A.3, eq. (A.11), p. 42: m0 = 2359237 + 2603/2828 = 2359237 + 10412/11312.
        m0: 2_359_237 * 11_312 + 2_603 * 4,
        // Janson (arXiv:1401.6285v1) A.3, eq. (A.12), p. 42: s0 = 397/402 = 4764/4824.
        s0: 397 * 12,
        // Janson (arXiv:1401.6285v1) A.3, eq. (A.13), p. 42: a0 = 1523/1764 = 3046/3528.
        a0: 1_523 * 2,
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
/// The numerator is over the epoch's own `Grain::den`: `TRUE_DATE_DENOMINATOR`
/// for Phugpa and Genden (every value of round 005 unchanged), 1 534 755 600
/// for Tsurphu.
///
/// Bound (002 §4.3): for `|t| ≤ 3 · 10^9` every product below stays under
/// `9 · 10^18`. The dominant one is the mean date's, `mean_mult · mean_factor ·
/// 11135 · |t|`: 9045 · 11135 ≈ 1.0 · 10^8 per lunar day for the coarse grain
/// and 201 · 675 · 11135 ≈ 1.5 · 10^9 per lunar day for the fine grain, so at
/// the bound the fine grain reaches about 4.5 · 10^18 (a test runs both grains
/// at both ends of the bound). The engine never calls it beyond the supported
/// years (about `t < 10^5`), which is why `i64` is enough and no `i128` is used.
fn true_date_numerator(epoch: &Epoch, lunar_day: i64) -> i64 {
    let grain = &epoch.grain;
    // d = 1..=30 within the month, n the true month count.
    let n = floor_div(lunar_day - 1, 30);
    let d = fmod(lunar_day - 1, 30) + 1;

    // mean_date = n·m1 + d·m2 + m0, Janson (arXiv:1401.6285v1) §7, eq. (7.1), p. 21, with
    // m1 = 167025/5656 = 334050/11312 (7.2), m2 = 11135/11312 (7.3); the same m1 and m2 for
    // every version (A.2, p. 38; A.3, p. 42). `m0` is over 11312 · mean_factor.
    let mean = epoch.m0 + grain.mean_factor * (M1 * n + M2 * d);

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
    // The same s1, s2 and the same offset for every version; `s0` is over 4824 · sun_factor, so with
    // y = numerator over 4824 · sun_factor the argument is 12y/(4824·g) = y/(402·g).
    let sun_units = SUN_TAB_UNITS * grain.sun_factor;
    let sun_y = fmod(
        grain.sun_factor * (S1 * n + S2 * d - SUN_ANOMALY_OFFSET) + epoch.s0,
        SUN_DEN * grain.sun_factor,
    );
    let sun_k = floor_div(sun_y, sun_units);
    let sun_r = fmod(sun_y, sun_units);
    // Linear interpolation (eq. (7.21)), in units of 1/(402·sun_factor).
    let sun_equ = sun_units * sun_tab(sun_k) + (sun_tab(sun_k + 1) - sun_tab(sun_k)) * sun_r;

    // true_date = mean_date + moon_equ/60 − sun_equ/60, eq. (7.22), p. 24.
    grain.mean_mult * mean + grain.moon_mult * moon_equ - grain.sun_mult * sun_equ
}

/// `⌊true_date⌋` for lunar day `t`: the JD of the calendar day containing the
/// end of that lunar day (Janson (arXiv:1401.6285v1) §8, eq. (8.1), p. 27).
fn end_day(epoch: &Epoch, lunar_day: i64) -> i64 {
    floor_div(true_date_numerator(epoch, lunar_day), epoch.grain.den)
}

/// The month `(Y, M, leap)` of the true month count `n` (Janson
/// (arXiv:1401.6285v1) §5.3, eqs. (5.19)–(5.22), p. 17), with the epoch's `β`
/// (Phugpa: `β = 184 − β*`, eq. (5.15); Tsurphu and Genden: the printed values,
/// A.2, p. 40 and A.3, p. 43, which the derivation `201 − c − β*` reproduces) and
/// `M₀ = 3`. The derivation of (5.19)–(5.22) from (5.10) does not use the value
/// 17, only the shape `n = ⌊(67M* + β* + c)/65⌋ − [ℓ]`, so it holds for each
/// version's `c`.
fn month_of_count(epoch: &Epoch, count: i64) -> (i64, i64, bool) {
    let beta = epoch.beta;
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
/// `M* = 12(Y − Y₀) + M − M₀` and `n = ⌊(67M* + β* + c)/65⌋ − [ℓ]`, with `c = 17`
/// for Phugpa, `c = 0` for Tsurphu (A.2, p. 39) and `c = 19` for Genden (A.3, p. 43).
fn count_of_month(epoch: &Epoch, year: i64, month: i64, leap: bool) -> i64 {
    let solar_months = 12 * (year - epoch.y0) + month - EPOCH_MONTH;
    floor_div(
        LUNAR_MONTHS * solar_months + epoch.beta_star + epoch.shift,
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
/// eqs. (7.1)–(7.3)), with `m0` over `11312 · mean_factor`; the true date differs
/// from it by less than 0.6 day (the tables peak at 25 and 11, divided by 60,
/// eqs. (7.18), (7.21), (7.22)). So the label lies within a few lunar days of
/// the mean-date estimate.
fn label_of_day<F: Fn(i64) -> i64>(epoch: &Epoch, day: i64, end: F) -> Result<i64, Error> {
    let grain = epoch.grain.mean_factor;
    let estimate = floor_div(day * MEAN_DEN * grain - epoch.m0, M2 * grain);
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
        epoch.system,
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
/// As above; [`Error::InvalidField`] also for a `ZDate` of another system than
/// the epoch's.
pub fn to_jdn(epoch: &Epoch, date: &ZDate) -> Result<Jdn, Error> {
    if date.system() != epoch.system {
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

#[cfg(test)]
mod version_tests {
    //! The Tsurphu and Genden constants and rules, checked against the paper
    //! (spec 008). Core-only: no `std` in `src/` (the gate's no-std check); the
    //! Table 6 fixture is read at compile time by `include_str!`, a built-in macro.
    //!
    //! METHOD rule 35: the constants tests check the *transcription* of the
    //! paper's numbers (each constant printed twice, as radix digits and as a
    //! fraction, must agree) and the paper's own consistency (β, γ, Table 6);
    //! they do not check that the paper's calendar is Buryat practice.
    use super::*;
    use crate::civil::GregorianDate;

    /// `(numerator, denominator)` of `whole + d1/r1 + d2/(r1·r2) + …` (Janson
    /// (arXiv:1401.6285v1) §2, p. 4), in `i128`: the product of the radices is
    /// below 10^9, `whole` below 10^7 and every cross product below 10^26, far
    /// inside `i128` (002 §4.3: the bound is stated; `i64` would overflow on the
    /// cross multiplication of a JD with a 10^8 denominator).
    fn value(whole: i128, digits: &[i128], radices: &[i128]) -> (i128, i128) {
        let mut numerator = 0_i128;
        let mut denominator = 1_i128;
        for (digit, radix) in digits.iter().zip(radices) {
            numerator = numerator * radix + digit;
            denominator *= radix;
        }
        (whole * denominator + numerator, denominator)
    }

    fn equal(a: (i128, i128), b: (i128, i128)) -> bool {
        a.0 * b.1 == b.0 * a.1
    }

    /// The radices of the Tsurphu forms, Janson (arXiv:1401.6285v1) A.2,
    /// eqs. (A.1), (A.3), (A.6): (60, 60, 6, 13, 707); of the longitudes,
    /// (A.2), (A.4), (A.7): (27, 60, 60, 6, 13, 67).
    const TSURPHU_TIME: [i128; 5] = [60, 60, 6, 13, 707];
    const TSURPHU_LONG: [i128; 6] = [27, 60, 60, 6, 13, 67];
    /// Genden's `m₀`, eq. (A.11), p. 42: (60, 60, 6, 67, 707).
    const GENDEN_TIME: [i128; 5] = [60, 60, 6, 67, 707];
    /// The standard radices, eqs. (7.2), (7.6), (7.12), pp. 21–23.
    const TIME: [i128; 4] = [60, 60, 6, 707];
    const LONG: [i128; 5] = [27, 60, 60, 6, 67];
    const ANOMALY: [i128; 2] = [28, 126];

    /// One constant the paper prints twice: as radix digits and as a fraction.
    struct Printed {
        label: &'static str,
        whole: i128,
        digits: &'static [i128],
        radices: &'static [i128],
        negative: bool,
        numerator: i128,
        denominator: i128,
    }

    /// Every constant A.2 and A.3 print in radix form AND as a fraction:
    /// A.1–A.8 and A.11–A.16 (14 constants). The digits and the fractions are
    /// the paper's, typed separately; they must be equal as exact rationals.
    const PRINTED: [Printed; 14] = [
        Printed {
            label: "A.1 m1 Tsurphu",
            whole: 29,
            digits: &[31, 50, 0, 8, 584],
            radices: &TSURPHU_TIME,
            negative: false,
            numerator: 167_025,
            denominator: 5_656,
        },
        Printed {
            label: "A.2 s1 Tsurphu",
            whole: 0,
            digits: &[2, 10, 58, 1, 3, 20],
            radices: &TSURPHU_LONG,
            negative: false,
            numerator: 65,
            denominator: 804,
        },
        Printed {
            label: "A.3 m0 E1732",
            whole: 2_353_745,
            digits: &[14, 6, 2, 2, 666],
            radices: &TSURPHU_TIME,
            negative: false,
            numerator: 2_353_745 * 7_635_600 + 1_795_153,
            denominator: 7_635_600,
        },
        Printed {
            label: "A.4 s0 E1732",
            whole: 0,
            digits: &[1, 29, 17, 5, 6, 1],
            radices: &TSURPHU_LONG,
            negative: true,
            numerator: 5_983,
            denominator: 108_540,
        },
        Printed {
            label: "A.5 a0 E1732",
            whole: 0,
            digits: &[14, 99],
            radices: &ANOMALY,
            negative: false,
            numerator: 207,
            denominator: 392,
        },
        Printed {
            label: "A.6 m0 E1852",
            whole: 2_397_598,
            digits: &[9, 24, 2, 5, 417],
            radices: &TSURPHU_TIME,
            negative: false,
            numerator: 2_397_598 * 7_635_600 + 1_197_103,
            denominator: 7_635_600,
        },
        Printed {
            label: "A.7 s0 E1852",
            whole: 0,
            digits: &[0, 1, 22, 2, 4, 18],
            radices: &TSURPHU_LONG,
            negative: false,
            numerator: 23,
            denominator: 27_135,
        },
        Printed {
            label: "A.8 a0 E1852",
            whole: 0,
            digits: &[0, 72],
            radices: &ANOMALY,
            negative: false,
            numerator: 1,
            denominator: 49,
        },
        Printed {
            label: "A.11 m0 E1747",
            whole: 2_359_237,
            digits: &[55, 13, 3, 31, 394],
            radices: &GENDEN_TIME,
            negative: false,
            numerator: 2_359_237 * 2_828 + 2_603,
            denominator: 2_828,
        },
        Printed {
            label: "A.12 s0 E1747",
            whole: 0,
            digits: &[26, 39, 51, 0, 18],
            radices: &LONG,
            negative: false,
            numerator: 397,
            denominator: 402,
        },
        Printed {
            label: "A.13 a0 E1747",
            whole: 0,
            digits: &[24, 22],
            radices: &ANOMALY,
            negative: false,
            numerator: 1_523,
            denominator: 1_764,
        },
        Printed {
            label: "A.14 m0 Phugpa at JD 2359237",
            whole: 2_359_237,
            digits: &[52, 41, 2, 524],
            radices: &TIME,
            negative: false,
            numerator: 2_359_237 * 5_656 + 4_967,
            denominator: 5_656,
        },
        Printed {
            label: "A.15 s0 Phugpa at JD 2359237",
            whole: 0,
            digits: &[26, 9, 37, 3, 45],
            radices: &LONG,
            negative: false,
            numerator: 779,
            denominator: 804,
        },
        Printed {
            label: "A.16 a0 Phugpa at JD 2359237",
            whole: 0,
            digits: &[24, 19],
            radices: &ANOMALY,
            negative: false,
            numerator: 3_043,
            denominator: 3_528,
        },
    ];

    fn printed_value(p: &Printed) -> (i128, i128) {
        let (n, d) = value(p.whole, p.digits, p.radices);
        (if p.negative { -n } else { n }, d)
    }

    #[test]
    fn the_radix_form_of_every_constant_equals_its_fraction() {
        // Prediction 2 of the round; STOP RULE (4) if one disagreed.
        assert_eq!(PRINTED.len(), 14, "A.1-A.8 and A.11-A.16");
        for p in &PRINTED {
            let (n, d) = printed_value(p);
            let negative_fraction = if p.negative {
                -p.numerator
            } else {
                p.numerator
            };
            assert!(
                equal((n, d), (negative_fraction, p.denominator)),
                "{}: radix form {n}/{d} vs printed fraction {negative_fraction}/{}",
                p.label,
                p.denominator
            );
        }
        // Remark 23, p. 42: m1 as Yeshe Paljor gives it equals the standard value,
        // and the constant m0 of (A.11) equals 1;55,13,3,333 (60, 60, 6, 707) + 2359236.
        assert!(equal(
            value(1, &[31, 50, 0, 45, 345], &GENDEN_TIME),
            value(1, &[31, 50, 0, 480], &TIME)
        ));
        assert!(equal(
            value(2_359_236 + 1, &[55, 13, 3, 31, 394], &GENDEN_TIME),
            value(2_359_236 + 1, &[55, 13, 3, 333], &TIME)
        ));
        // The seeded control: one digit wrong must be reported as unequal.
        assert!(!equal(value(0, &[14, 99 + 1], &ANOMALY), (207, 392)));
    }

    #[test]
    fn the_engine_holds_each_epoch_constant_as_the_paper_prints_it() {
        // m0 over 11312 · mean_factor, s0 over 4824 · sun_factor, a0 over 3528.
        let e = Epoch::E1732;
        assert!(equal(
            printed_value(&PRINTED[2]),
            (i128::from(e.m0), 7_635_600)
        ));
        assert!(equal(
            printed_value(&PRINTED[3]),
            (i128::from(e.s0), 217_080)
        ));
        assert!(equal(printed_value(&PRINTED[4]), (i128::from(e.a0), 3_528)));
        let e = Epoch::E1852;
        assert!(equal(
            printed_value(&PRINTED[5]),
            (i128::from(e.m0), 7_635_600)
        ));
        assert!(equal(
            printed_value(&PRINTED[6]),
            (i128::from(e.s0), 217_080)
        ));
        assert!(equal(printed_value(&PRINTED[7]), (i128::from(e.a0), 3_528)));
        let e = Epoch::E1747;
        assert!(equal(
            printed_value(&PRINTED[8]),
            (i128::from(e.m0), 11_312)
        ));
        assert!(equal(printed_value(&PRINTED[9]), (i128::from(e.s0), 4_824)));
        assert!(equal(
            printed_value(&PRINTED[10]),
            (i128::from(e.a0), 3_528)
        ));
    }

    #[test]
    fn remark_28_adds_one_to_s0_and_changes_nothing_modulo_1() {
        // Janson (arXiv:1401.6285v1) C.3, eqs. (C.42), (C.43), p. 63: for the
        // definition points s0 of (A.4), (A.7) is increased by 1; "this does not
        // matter in Section 7" (p. 62) because only the fractional part of an angle
        // is used (§2, p. 5). The engine holds the printed values (A.4), (A.7); the
        // increased values (C.42), (C.43) are the same angle.
        // (C.42): 1 − 5983/108540 = 102557/108540, which is 205114/217080; (C.43):
        // 1 + 23/27135 = (217080 + 184)/217080. (A.4) is −11966/217080 and (A.7) 184/217080.
        assert_eq!(108_540 - 5_983, 102_557);
        assert_eq!(fmod(Epoch::E1732.s0, 217_080), fmod(2 * 102_557, 217_080));
        assert_eq!(fmod(Epoch::E1852.s0, 217_080), fmod(217_080 + 184, 217_080));
        // The seeded control: increasing s0 by anything but a whole circle changes it.
        assert_ne!(
            fmod(Epoch::E1732.s0, 217_080),
            fmod(2 * 102_557 + 1, 217_080)
        );
        // And no value the mapping computes changes: the end day of every lunar
        // day of 1900..2100 (lunar days 62000..=135000 from the epoch) is the same
        // with `s0` increased by one whole circle (Remark 28's "+ 1"), for both epochs.
        for epoch in [Epoch::E1732, Epoch::E1852] {
            let increased = Epoch {
                s0: epoch.s0 + 217_080,
                ..epoch
            };
            for t in 40_000..=160_000 {
                assert_eq!(end_day(&epoch, t), end_day(&increased, t), "t = {t}");
            }
        }
    }

    #[test]
    fn beta_gamma_and_the_leap_rule_constants_agree_with_the_paper() {
        // β = 201 − c − β*: the derivation of (5.19)-(5.22) from (5.10) with the
        // rounding shift c (17 Phugpa, 0 Tsurphu, 19 Genden); 201 = 3 · 67 (M0 = 3).
        // The printed values: (5.16)-(5.18), A.2 p. 40, A.3 p. 43, C.3, C.4 p. 63.
        for (epoch, printed_beta) in [
            (Epoch::E806, 123),
            (Epoch::E1927, 129),
            (Epoch::E1987, 184),
            (Epoch::E1732, 142),
            (Epoch::E1852, 187),
            (Epoch::E1747, 172),
        ] {
            assert_eq!(epoch.beta, printed_beta);
            assert_eq!(
                201 - epoch.shift - epoch.beta_star,
                printed_beta,
                "{:?}",
                epoch.system
            );
        }
        // (A.10), p. 40: β + β* ≡ 6 (mod 65) for Tsurphu; (A.18), p. 43: ≡ 52 for Genden.
        assert_eq!(fmod(Epoch::E1732.beta + Epoch::E1732.beta_star, 65), 6);
        assert_eq!(fmod(Epoch::E1852.beta + Epoch::E1852.beta_star, 65), 6);
        assert_eq!(fmod(Epoch::E1747.beta + Epoch::E1747.beta_star, 65), 52);
        // (5.30), p. 18: γ = (−Y0 − 19β) mod 65 and (5.32) γ* = 24γ mod 65, with
        // γ = 42, γ* = 33 for Phugpa ((5.38), (5.39), p. 19) and γ = 55, γ* = 20
        // for Tsurphu (A.2, p. 40) and Genden (A.3, p. 43) at every epoch.
        for (epoch, gamma, gamma_star) in [
            (Epoch::E806, 42, 33),
            (Epoch::E1927, 42, 33),
            (Epoch::E1987, 42, 33),
            (Epoch::E1732, 55, 20),
            (Epoch::E1852, 55, 20),
            (Epoch::E1747, 55, 20),
        ] {
            let g = fmod(-epoch.y0 - 19 * epoch.beta, 65);
            assert_eq!(g, gamma, "γ of {:?} {}", epoch.system, epoch.y0);
            assert_eq!(fmod(24 * g, 65), gamma_star);
        }
        // The rounding shift of each rule: 65 − 48, 0 (ix = 0 or 1), 65 − 46.
        assert_eq!(PHUGPA_SHIFT, 65 - 48);
        assert_eq!(TSURPHU_SHIFT, 0);
        assert_eq!(GENDEN_SHIFT, 65 - 46);
    }

    #[test]
    fn the_grains_multiply_back_to_their_denominators() {
        for grain in [Grain::COARSE, Grain::FINE] {
            assert_eq!(grain.mean_mult * 11_312 * grain.mean_factor, grain.den);
            assert_eq!(grain.moon_mult * 126 * 60, grain.den);
            assert_eq!(grain.sun_mult * 402 * grain.sun_factor * 60, grain.den);
        }
        assert_eq!(Grain::FINE.den, 15 * TRUE_DATE_DENOMINATOR);
        // m0 and s0 of each epoch are held over its own grain.
        assert_eq!(Epoch::E1732.grain, Grain::FINE);
        assert_eq!(Epoch::E1852.grain, Grain::FINE);
        assert_eq!(Epoch::E1747.grain, Grain::COARSE);
        assert_eq!(Epoch::E806.grain, Grain::COARSE);
        assert_eq!(7_635_600, 11_312 * 675);
        assert_eq!(217_080, 4_824 * 45);
    }

    #[test]
    fn true_date_stays_in_i64_at_the_stated_bound_in_both_grains() {
        // 002 §4.3: the bound `|t| ≤ 3·10^9` of `true_date_numerator` holds for the fine
        // grain too (about 4.5·10^18 there; overflow checks are on in every profile).
        for epoch in [Epoch::E806, Epoch::E1732, Epoch::E1747] {
            let g = epoch.grain.mean_factor;
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
                let mean_days = floor_div(epoch.m0 + M2 * g * edge, MEAN_DEN * g);
                assert!(
                    (end_day(&epoch, edge) - mean_days).abs() <= 1,
                    "end_day vs mean date at t = {edge}"
                );
            }
        }
    }

    /// The Tibetan date `(year, month, leap, tshes)` of a JD under `epoch`, with no range check.
    fn date_of(epoch: &Epoch, day: i64) -> (i64, i64, bool, i64) {
        match label_of_day(epoch, day, |t| end_day(epoch, t)) {
            Ok(lunar_day) => {
                let (year, month, leap) = month_of_count(epoch, floor_div(lunar_day - 1, 30));
                (year, month, leap, fmod(lunar_day - 1, 30) + 1)
            }
            Err(_) => (0, 0, false, 0),
        }
    }

    #[test]
    fn the_epoch_dates_and_weekdays_the_paper_prints_are_consistent() {
        // A.2, p. 39: JD 2353745 (Wednesday, 26 March 1732 (Greg.)), JD 2397598
        // (Monday, 19 April 1852); A.3, p. 42: JD 2359237 (Sunday, 9 April 1747).
        // The weekday is eq. (9.1), p. 28, with Table 5: (JD + 2) mod 7, 1 = Sunday,
        // 2 = Monday, 4 = Wednesday (`DEBT.md` D-10 is about page 1 only).
        for (year, month, day, jd, weekday) in [
            (1732, 3, 26, 2_353_745, 4),
            (1852, 4, 19, 2_397_598, 2),
            (1747, 4, 9, 2_359_237, 1),
        ] {
            let date = GregorianDate::new(year, month, day).map(GregorianDate::to_jdn);
            assert_eq!(date.map(Jdn::get), Ok(jd), "{year}-{month}-{day}");
            assert_eq!(fmod(jd + 2, 7), weekday, "JD {jd}");
        }
        // A.13, p. 50: the epoch JD 2015531 is "Monday 23 March 806 (Julian)": (JD + 2) mod 7 = 2.
        assert_eq!(fmod(2_015_531 + 2, 7), 2);
        // The seeded control: the next day is not the printed weekday.
        assert_ne!(fmod(2_353_745 + 1 + 2, 7), 4);
        // The floor of m0 is the epoch JD (Remark 16, p. 26).
        assert_eq!(floor_div(Epoch::E1732.m0, 7_635_600), 2_353_745);
        assert_eq!(floor_div(Epoch::E1852.m0, 7_635_600), 2_397_598);
        assert_eq!(floor_div(Epoch::E1747.m0, 11_312), 2_359_237);
    }

    #[test]
    fn jd_2015531_is_day_30_of_leap_month_3_in_tsurphu_and_genden() {
        // A.13, footnote 64, p. 50: the epoch JD 2015531 "happens to be the last day
        // of the preceding month in all four versions; this is day 30 in month 2 for
        // the Phugpa and Bhutanese versions, and day 30 in leap month 3 in the
        // Tsurphu and Mongolian versions."
        for epoch in [Epoch::E806, Epoch::E1927, Epoch::E1987] {
            assert_eq!(date_of(&epoch, 2_015_531), (806, 2, false, 30));
        }
        for epoch in [Epoch::E1732, Epoch::E1852, Epoch::E1747] {
            assert_eq!(
                date_of(&epoch, 2_015_531),
                (806, 3, true, 30),
                "{:?} {}",
                epoch.system,
                epoch.y0
            );
        }
        // The seeded control: the next day is not that date.
        assert_ne!(date_of(&Epoch::E1732, 2_015_532), (806, 3, true, 30));
    }

    // ---- Table 6: the epoch values for JD 2015531, carried from each epoch.

    const TABLE_6: &str = include_str!("../../../golden/reference/janson-v1-table6-epochs-806.csv");
    const TABLE_6_HEADER: &str =
        "system,m0_radix,m0_decimal,s0_radix,s0_decimal,a0_radix,a0_decimal";

    /// One row of Table 6's fixture: the radix digits and the decimals as integers
    /// (the decimal `2.376238` is 2376238, six places).
    #[derive(Clone, Copy)]
    struct Row {
        system: &'static str,
        m0: [i128; 5],
        s0: [i128; 5],
        a0: [i128; 2],
        decimals: [i128; 3],
    }

    fn digits<const N: usize>(field: &str) -> [i128; N] {
        let mut out = [0_i128; N];
        let mut count = 0;
        for (i, part) in field.split(':').enumerate() {
            if let (Some(slot), Ok(n)) = (out.get_mut(i), part.parse::<i128>()) {
                *slot = n;
                count += 1;
            }
        }
        assert_eq!(count, N, "digits of {field}");
        out
    }

    fn decimal(field: &str) -> i128 {
        let mut parts = field.split('.');
        let whole = parts.next().and_then(|p| p.parse::<i128>().ok());
        let fraction = parts.next();
        assert!(parts.next().is_none());
        match (whole, fraction) {
            (Some(w), Some(f)) if f.len() == 6 => w * 1_000_000 + f.parse::<i128>().unwrap_or(-1),
            _ => -1,
        }
    }

    fn rows() -> [Row; 3] {
        let mut lines = TABLE_6.lines();
        assert_eq!(lines.next(), Some(TABLE_6_HEADER));
        let mut out = [Row {
            system: "",
            m0: [0; 5],
            s0: [0; 5],
            a0: [0; 2],
            decimals: [0; 3],
        }; 3];
        let mut loaded = 0;
        for (slot, line) in out.iter_mut().zip(lines.by_ref()) {
            let mut f = line.split(',');
            let (system, m0, m0d, s0, s0d, a0, a0d) = (
                f.next().unwrap_or(""),
                f.next().unwrap_or(""),
                f.next().unwrap_or(""),
                f.next().unwrap_or(""),
                f.next().unwrap_or(""),
                f.next().unwrap_or(""),
                f.next().unwrap_or(""),
            );
            assert!(f.next().is_none(), "field count of {line}");
            *slot = Row {
                system: match system {
                    "phugpa" => "phugpa",
                    "tsurphu" => "tsurphu",
                    "genden" => "genden",
                    _ => "?",
                },
                m0: digits(m0),
                s0: digits(s0),
                a0: digits(a0),
                decimals: [decimal(m0d), decimal(s0d), decimal(a0d)],
            };
            loaded += 1;
        }
        assert_eq!(lines.next(), None, "more than 3 data rows");
        assert_eq!(
            loaded, 3,
            "the fixture loaded {loaded} rows, 3 measured by the transcription"
        );
        out
    }

    /// The radix digits of `numerator/denominator` (in `[0, 1)`) for `radices`, and
    /// whether the value lies exactly on the last digit (remainder 0).
    fn radix_digits<const N: usize>(
        numerator: i128,
        denominator: i128,
        radices: &[i128; N],
    ) -> ([i128; N], bool) {
        let mut out = [0_i128; N];
        let mut x = numerator;
        for (slot, radix) in out.iter_mut().zip(radices) {
            let scaled = x * radix;
            *slot = scaled.div_euclid(denominator);
            x = scaled.rem_euclid(denominator);
        }
        (out, x == 0)
    }

    /// `numerator/denominator` to six decimals, rounded half up.
    fn rounded_6(numerator: i128, denominator: i128) -> i128 {
        (numerator * 2_000_000 + denominator).div_euclid(2 * denominator)
    }

    fn truncated_6(numerator: i128, denominator: i128) -> i128 {
        (numerator * 1_000_000).div_euclid(denominator)
    }

    /// The nine values of one epoch carried to JD 2015531: `(m0 numerator over
    /// 7 635 600, s0 over 217 080, a0 over 3528)`, by `k` whole true months, the
    /// mean motions of eqs. (7.2), (7.6), (7.12) in exact arithmetic.
    fn carried(epoch: &Epoch) -> (i128, i128, i128) {
        const DM: i64 = 7_635_600;
        let m0 = epoch.m0 * floor_div(675, epoch.grain.mean_factor);
        let s0 = epoch.s0 * floor_div(45, epoch.grain.sun_factor);
        // The k with m0 + k·m1 in [2015531, 2015532): the mean new moon of that JD.
        let step = M1 * 675;
        let k = ceil_div(2_015_531 * DM - m0, step);
        let m0k = m0 + k * step;
        assert!((2_015_531 * DM..2_015_532 * DM).contains(&m0k), "k = {k}");
        let s0k = fmod(s0 + k * S1 * 45, 217_080);
        let a0k = fmod(epoch.a0 + k * A1, 3_528);
        (i128::from(m0k), i128::from(s0k), i128::from(a0k))
    }

    /// Compares one fixture row with an epoch's carried values under the one
    /// rule: the radix digits equal the exact digits, the value lying exactly on
    /// the last printed digit (so truncation and rounding there coincide), and the
    /// decimals are the exact value rounded half up at the sixth decimal.
    fn check(row: &Row, epoch: &Epoch) -> Result<(), &'static str> {
        let (m, s, a) = carried(epoch);
        let m_frac = m - 2_015_531 * 7_635_600;
        let (m_digits, m_exact) = radix_digits(m_frac, 7_635_600, &TIME);
        if !m_exact {
            return Err("m0 is not on the last printed digit");
        }
        // The first digit of the printed m0 is the day number modulo 7: JD − 2015529.
        let printed_m0_fraction = row.m0.iter().skip(1).copied();
        if row.m0.first() != Some(&(2_015_531 - 2_015_529))
            || !printed_m0_fraction.eq(m_digits.iter().copied())
        {
            return Err("m0 radix digits differ");
        }
        let (s_digits, s_exact) = radix_digits(s, 217_080, &LONG);
        if !s_exact {
            return Err("s0 is not on the last printed digit");
        }
        if row.s0 != s_digits {
            return Err("s0 radix digits differ");
        }
        let (a_digits, a_exact) = radix_digits(a, 3_528, &ANOMALY);
        if !a_exact {
            return Err("a0 is not on the last printed digit");
        }
        if row.a0 != a_digits {
            return Err("a0 radix digits differ");
        }
        if row.decimals[0] != rounded_6(m - 2_015_529 * 7_635_600, 7_635_600) {
            return Err("m0 decimal differs");
        }
        if row.decimals[1] != rounded_6(s, 217_080) {
            return Err("s0 decimal differs");
        }
        if row.decimals[2] != rounded_6(a, 3_528) {
            return Err("a0 decimal differs");
        }
        Ok(())
    }

    #[test]
    fn table_6_equals_the_epochs_carried_to_jd_2015531_by_one_rule() {
        let rows = rows();
        assert_eq!(rows.map(|r| r.system), ["phugpa", "tsurphu", "genden"]);
        let versions: [(&Row, &[Epoch]); 3] = [
            (&rows[0], &[Epoch::E806, Epoch::E1927, Epoch::E1987]),
            (&rows[1], &[Epoch::E1732, Epoch::E1852]),
            (&rows[2], &[Epoch::E1747]),
        ];
        let mut compared = 0;
        let mut truncation_differs = 0;
        for (row, epochs) in versions {
            for epoch in epochs {
                assert_eq!(
                    check(row, epoch),
                    Ok(()),
                    "{} from {:?} {}",
                    row.system,
                    epoch.system,
                    epoch.y0
                );
                compared += 1;
                let (m, s, a) = carried(epoch);
                truncation_differs += usize::from(
                    row.decimals[0] != truncated_6(m - 2_015_529 * 7_635_600, 7_635_600),
                );
                truncation_differs += usize::from(row.decimals[1] != truncated_6(s, 217_080));
                truncation_differs += usize::from(row.decimals[2] != truncated_6(a, 3_528));
            }
        }
        // Six epochs against three rows; the nine printed values (3 rows x 3 quantities) are each
        // reproduced from every epoch of their version.
        assert_eq!(compared, 6);
        // The rule matters: truncating the decimals instead of rounding them does not
        // reproduce the table (the decimals are rounded, the radix digits exact).
        assert!(
            truncation_differs > 0,
            "truncation would have fitted too; the rule is not tested"
        );
    }

    #[test]
    fn seeded_control_table_6_one_wrong_digit_or_decimal_is_reported() {
        let rows = rows();
        let good = rows[1];
        assert_eq!(check(&good, &Epoch::E1732), Ok(()), "the committed row");
        let mut wrong = good;
        wrong.m0[3] += 1;
        assert_eq!(check(&wrong, &Epoch::E1732), Err("m0 radix digits differ"));
        let mut wrong = good;
        wrong.decimals[1] += 1;
        assert_eq!(check(&wrong, &Epoch::E1732), Err("s0 decimal differs"));
        // A row compared with another version's epoch is reported too.
        assert_eq!(
            check(&rows[1], &Epoch::E1747),
            Err("m0 radix digits differ")
        );
        assert_eq!(
            check(&rows[2], &Epoch::E1732),
            Err("m0 radix digits differ")
        );
    }
}
