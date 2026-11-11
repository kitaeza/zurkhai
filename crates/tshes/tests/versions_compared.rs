//! The paper's own statements about how the versions relate, computed through
//! the engine over every day of 1900-01-01..=2100-12-31 (spec 008 Item 4c).
//! These, with Tables 6–9 (`p5_versions.rs` and the crate's unit tests), are
//! the round's only correctness checks: P1–P4 check consistency, and **a wrong
//! epoch passes P1–P4** (METHOD rule 35).
//!
//! Each test states its exact scope: the days or years it covers, the ones it
//! excludes, and why. The counts of c3 and c5 are printed (`--nocapture`), not
//! asserted: the paper's "about one day in 20" (p. 50) and "only a few days each
//! year" (p. 51) are estimates.

mod common;

use common::{Day, all_days_of, first_jdn, last_jdn};
use std::collections::BTreeMap;
use tshes::phugpa::{Epoch, to_jdn, to_zdate};
use tshes::{GregorianDate, Jdn, System, ZDate};

fn jdn(n: i64) -> Jdn {
    Jdn::new(n).expect("a JD in the domain")
}

/// The Gregorian date of a day, as `(year, month, day)`.
fn ymd(day: &Day) -> (i64, u8, u8) {
    (day.date.year(), day.date.month(), day.date.day())
}

#[test]
fn c1_the_two_tsurphu_epochs_give_the_same_calendar_on_every_day() {
    // Janson v1 A.2, p. 39: the epoch "1485 months later (equivalent and giving the
    // same calendar)". Scope: all 73,414 days of the range, forward (E1732 and E1852
    // give the same `ZDate`) and backward (E1852's inverse returns the day). Nothing excluded.
    let engine = all_days_of(System::Tsurphu);
    let mut compared = 0_usize;
    for day in &engine {
        let j = jdn(day.jdn);
        let from_1732 = to_zdate(&Epoch::E1732, j).unwrap();
        let from_1852 = to_zdate(&Epoch::E1852, j).unwrap();
        assert_eq!(from_1732, from_1852, "JD {} ({:?})", day.jdn, ymd(day));
        assert_eq!(from_1732, day.z, "the engine's Tsurphu is E1732's");
        assert_eq!(
            to_jdn(&Epoch::E1852, &from_1732),
            Ok(j),
            "E1852's inverse at JD {}",
            day.jdn
        );
        compared += 1;
    }
    assert_eq!(compared, 73_414);
    // The seeded control: the comparison can fail. Genden's epoch is another calendar:
    // it differs from Tsurphu's on some day of the range.
    let differs = engine
        .iter()
        .any(|d| to_zdate(&Epoch::E1747, jdn(d.jdn)).unwrap().tshes() != d.z.tshes());
    assert!(
        differs,
        "Genden never differs from Tsurphu: the comparison cannot fail"
    );
}

/// The leap month numbers of each Tibetan year, from the days of the range.
fn leap_months(days: &[Day]) -> BTreeMap<i64, Vec<u8>> {
    let mut out: BTreeMap<i64, Vec<u8>> = BTreeMap::new();
    for day in days {
        let year = day.z.year().number();
        let entry = out.entry(year).or_default();
        if day.z.is_leap_month() && !entry.contains(&day.z.month().get()) {
            entry.push(day.z.month().get());
        }
    }
    out
}

#[test]
fn c2_tsurphu_and_genden_have_the_same_leap_months_in_every_year() {
    // Janson v1 pp. 43 (footnote 56), 51: "the Mongolian version has the same leap months
    // as the Tsurphu version (although one may start a day before the other)". Scope:
    // the 202 Tibetan years with a day in the range, 1899..=2100, from the days of the
    // range; the leap month of 1899 or 2100 would be missed if it lay wholly outside
    // the range, but both versions hold 75 leap months in the range and 75 is also what
    // eq. (5.36) gives for 1899..=2100 (`exhaustive_versions.rs`), so none is missed.
    let tsurphu = leap_months(&all_days_of(System::Tsurphu));
    let genden = leap_months(&all_days_of(System::Genden));
    assert_eq!(tsurphu.len(), 202);
    assert_eq!(tsurphu, genden);
    let with_leap = tsurphu.values().filter(|m| !m.is_empty()).count();
    assert_eq!(with_leap, 75);
    assert!(tsurphu.values().all(|m| m.len() <= 1));
    // The seeded control: Phugpa's leap months are not Tsurphu's.
    let phugpa = leap_months(&all_days_of(System::Phugpa));
    assert_ne!(phugpa, tsurphu);
}

/// The lunar-day count of each day of the range under one version: `30 · (the number of
/// months since the first month of the range) + tshes`. Janson's `t = 30n + d` (§5–§7,
/// p. 14ff) with `n` the true month count, which counts every month, regular or leap,
/// consecutively ("a continuous count of months", p. 15) and so differs between two
/// versions only by a constant (their epochs differ); counting from the first month of
/// the range removes it without any use of the months' numbers, which the two versions'
/// different leap months shift.
fn lunar_day_counts(days: &[Day]) -> Vec<i64> {
    let mut counts = Vec::with_capacity(days.len());
    let mut month_ordinal = 0_i64;
    let mut previous: Option<&ZDate> = None;
    for day in days {
        if let Some(p) = previous {
            let same_month = (p.year(), p.month(), p.is_leap_month())
                == (day.z.year(), day.z.month(), day.z.is_leap_month());
            if !same_month {
                month_ordinal += 1;
            }
        }
        counts.push(30 * month_ordinal + i64::from(day.z.tshes().get()));
        previous = Some(&day.z);
    }
    counts
}

#[test]
fn c3_where_phugpa_and_tsurphu_differ_the_phugpa_date_is_larger_by_one() {
    // Janson v1 p. 51: "when the Phugpa and Tsurphu dates of a calendar day differ,
    // the Phugpa date is always larger (by 1)". "Date" is the lunar-day count above.
    // Scope: all 73,414 days. Alignment: both versions start the range in the same lunar
    // month (the first partial month); the test asserts it (the first day's difference is
    // 0 or 1). A day on which the two versions are in different months but the same
    // count of lunar days elapsed is not a difference.
    let phugpa = lunar_day_counts(&all_days_of(System::Phugpa));
    let tsurphu = lunar_day_counts(&all_days_of(System::Tsurphu));
    assert_eq!(phugpa.len(), 73_414);
    let mut differing = 0_usize;
    let mut first_difference = None;
    for (index, (p, t)) in phugpa.iter().zip(&tsurphu).enumerate() {
        let difference = p - t;
        assert!(
            difference == 0 || difference == 1,
            "day index {index}: Phugpa count {p}, Tsurphu count {t}"
        );
        if difference == 1 {
            differing += 1;
            first_difference.get_or_insert(index);
        }
    }
    let ratio = ratio_text(differing, phugpa.len());
    println!(
        "C3 days on which Phugpa and Tsurphu dates differ: {differing} of {} ({ratio}); the paper estimates about 1 in 20 (p. 50)",
        phugpa.len()
    );
    assert!(differing > 0, "the comparison never met a difference");
    // The seeded control: the direction of the claim is tested. With the versions swapped
    // (Tsurphu minus Phugpa) the same comparison fails on every differing day.
    let swapped = tsurphu
        .iter()
        .zip(&phugpa)
        .filter(|(t, p)| *t - *p < 0)
        .count();
    assert_eq!(
        swapped, differing,
        "swapped, every differing day violates the claim"
    );
}

/// `numerator / denominator` as a decimal text with three decimals, in integers.
fn ratio_text(numerator: usize, denominator: usize) -> String {
    let thousandths = (numerator * 1000 * 2 + denominator) / (2 * denominator);
    format!("{}.{:03}", thousandths / 1000, thousandths % 1000)
}

/// The first day of Tibetan year `year`: the first day of the range whose `ZDate` is in it.
fn first_days(days: &[Day]) -> BTreeMap<i64, (i64, u8, u8)> {
    let mut out = BTreeMap::new();
    for day in days {
        out.entry(day.z.year().number()).or_insert_with(|| ymd(day));
    }
    out
}

#[test]
fn c4_tsurphu_and_genden_new_year_differ_only_in_1900_within_the_range() {
    // Janson v1 p. 52: "the last time these versions had different New Year was 1900
    // (31/1 vs 1/2) and the next will be 2161 (26/2 vs 25/2)". Scope: the Tibetan years
    // 1900..=2100, whose first day lies in the range (year 1899 began before 1900-01-01,
    // so its first day is not in the range and is excluded: 201 years compared). 2161 is
    // outside the range and is not checked.
    let tsurphu = first_days(&all_days_of(System::Tsurphu));
    let genden = first_days(&all_days_of(System::Genden));
    assert_eq!(tsurphu.len(), 202);
    let mut differing = Vec::new();
    let mut compared = 0;
    for year in 1900..=2100_i64 {
        compared += 1;
        if tsurphu[&year] != genden[&year] {
            differing.push((year, tsurphu[&year], genden[&year]));
        }
    }
    assert_eq!(compared, 201);
    println!("C4 years with different New Year: {differing:?}");
    assert_eq!(
        differing.iter().map(|d| d.0).collect::<Vec<_>>(),
        [1900],
        "the only year with different New Year"
    );
    // The two dates of 1900 are the ones the paper prints, 31 January and 1 February.
    let (_, a, b) = differing[0];
    let mut dates = [a, b];
    dates.sort_unstable();
    assert_eq!(dates, [(1900, 1, 31), (1900, 2, 1)]);
    // The seeded control: Phugpa's New Years differ from Tsurphu's in many years.
    let phugpa = first_days(&all_days_of(System::Phugpa));
    assert!(
        (1900..=2100_i64)
            .filter(|y| phugpa[y] != tsurphu[y])
            .count()
            > 10
    );
}

#[test]
fn c5_how_many_days_the_versions_differ_report_only() {
    // Janson v1 p. 51: Tsurphu and Mongolia "differ ... only a few days each year".
    // Printed, not asserted: the days on which two versions' `ZDate`s differ in any
    // field, over all 73,414 days.
    let phugpa = all_days_of(System::Phugpa);
    let tsurphu = all_days_of(System::Tsurphu);
    let genden = all_days_of(System::Genden);
    let differ = |a: &[Day], b: &[Day]| {
        a.iter()
            .zip(b)
            .filter(|(x, y)| {
                (
                    x.z.year(),
                    x.z.month(),
                    x.z.is_leap_month(),
                    x.z.tshes(),
                    x.z.is_doubled(),
                ) != (
                    y.z.year(),
                    y.z.month(),
                    y.z.is_leap_month(),
                    y.z.tshes(),
                    y.z.is_doubled(),
                )
            })
            .count()
    };
    let t_vs_g = differ(&tsurphu, &genden);
    let g_vs_p = differ(&genden, &phugpa);
    let t_vs_p = differ(&tsurphu, &phugpa);
    println!(
        "C5 days on which Tsurphu and Genden differ: {t_vs_g} of 73414 ({} per year over 201 years); Genden and Phugpa: {g_vs_p}; Tsurphu and Phugpa: {t_vs_p}",
        ratio_text(t_vs_g, 201)
    );
    // The same two comparisons by the lunar-day count of c3, which does not move when a
    // leap month shifts the months' numbers: the other measure of "differ".
    let (lp, lt, lg) = (
        lunar_day_counts(&phugpa),
        lunar_day_counts(&tsurphu),
        lunar_day_counts(&genden),
    );
    let count_differ = |a: &[i64], b: &[i64]| a.iter().zip(b).filter(|(x, y)| x != y).count();
    println!(
        "C5 by lunar-day count: Tsurphu and Genden differ on {}, Genden and Phugpa on {}, Tsurphu and Phugpa on {}",
        count_differ(&lt, &lg),
        count_differ(&lg, &lp),
        count_differ(&lt, &lp)
    );
    // The first day on which Tsurphu and Genden differ, as a report of what the difference looks like.
    let first = tsurphu
        .iter()
        .zip(&genden)
        .find(|(x, y)| x.z.tshes() != y.z.tshes() || x.z.month() != y.z.month());
    if let Some((x, y)) = first {
        println!(
            "C5 the first day on which they differ: {:?}, Tsurphu {}/{} tshes {}, Genden {}/{} tshes {}",
            ymd(x),
            x.z.year().number(),
            x.z.month().get(),
            x.z.tshes().get(),
            y.z.year().number(),
            y.z.month().get(),
            y.z.tshes().get()
        );
    }
    assert_eq!(
        first_jdn(),
        GregorianDate::new(1900, 1, 1).unwrap().to_jdn().get()
    );
    assert_eq!(last_jdn() - first_jdn() + 1, 73_414);
}
