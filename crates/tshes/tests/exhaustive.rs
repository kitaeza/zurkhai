//! Properties P1–P4 and P7 (001 §6.4), run EXHAUSTIVELY over every day from
//! 1900-01-01 to 2100-12-31 (spec 005 Item 4; METHOD rule 41: the range is small
//! enough to enumerate, and enumeration reaches every month end, skipped and
//! doubled tshes, leap month, 12 → 1 rollover and range edge).
//!
//! **What these check, and what they do not (METHOD rule 35).** They check the
//! engine's *consistency*: the two directions agree, the structure is the one
//! Janson describes, the frequencies are the ones the intercalation rule
//! implies. A wrong epoch shared by both directions passes P1–P3. The
//! correctness checks are `worked_examples.rs`, `epochs.rs` (three independently
//! stated constant sets must give one calendar) and round 006's T5/T4 fixtures.

mod common;

use common::{Day, MonthKey, all_days, first_jdn, last_jdn, month_key, months_of};
use std::collections::{HashMap, HashSet};
use tshes::{Error, GregorianDate, Jdn, System, Tshes, ZDate, to_gregorian, to_zdate};

/// The days that carry a tshes in a month, as `(doubled, skipped_before, date)`.
type Entries = Vec<(bool, Option<u8>, GregorianDate)>;

fn to_i64(n: usize) -> i64 {
    i64::try_from(n).unwrap()
}

/// The tshes of a day as a plain number.
fn tshes(day: &Day) -> u8 {
    day.z.tshes().get()
}

#[test]
fn range_has_73414_days() {
    assert_eq!(last_jdn() - first_jdn() + 1, 73_414);
    assert_eq!(all_days().len(), 73_414);
}

#[test]
fn p1_every_day_maps_to_exactly_one_zdate() {
    // Totality: `all_days` panics if any day does not map. Exactly one: the
    // mapping is a function, and no two days share a ZDate.
    let days = all_days();
    let distinct: HashSet<ZDate> = days.iter().map(|day| day.z).collect();
    assert_eq!(distinct.len(), days.len(), "two days share a ZDate");
    for day in &days {
        assert_eq!(
            to_zdate(day.date, System::Phugpa),
            Ok(day.z),
            "not a function at {:?}",
            day.date
        );
    }
}

#[test]
fn p2_round_trip_from_every_day() {
    for day in all_days() {
        assert_eq!(
            to_gregorian(day.z),
            Ok(day.date),
            "to_gregorian(to_zdate(d)) != d at {:?}",
            day.date
        );
    }
}

#[test]
fn p2_round_trip_from_every_valid_zdate() {
    // to_zdate(to_gregorian(z)) == z for every valid z in the range.
    for day in all_days() {
        let date = to_gregorian(day.z).expect("a ZDate the engine produced is valid");
        assert_eq!(
            to_zdate(date, System::Phugpa),
            Ok(day.z),
            "to_zdate(to_gregorian(z)) != z at {:?}",
            day.date
        );
    }
}

#[test]
fn p2_every_combination_of_fields_is_classified_exactly() {
    // For every month wholly inside the range, every tshes 1..=30, both doubled
    // flags and four `skipped_before` values: the outcome is the one the forward
    // table (built from the days, not from `to_gregorian`) predicts.
    //  * the combination is some day's ZDate: Ok, and that day;
    //  * no day carries the tshes in that month: Err(SkippedDay);
    //  * otherwise (a wrong doubled flag or `skipped_before`): Err(InvalidField).
    let days = all_days();
    let months = months_of(&days);
    let mut table: HashMap<(MonthKey, u8), Entries> = HashMap::new();
    for day in &days {
        table
            .entry((month_key(&day.z), tshes(day)))
            .or_default()
            .push((
                day.z.is_doubled(),
                day.z.skipped_before().map(Tshes::get),
                day.date,
            ));
    }
    let mut classified = 0_usize;
    let mut skipped_cases = 0_usize;
    let mut invalid_cases = 0_usize;
    for month in &months[1..months.len() - 1] {
        let sample = days[month.start].z;
        for number in 1..=30_u8 {
            for doubled in [false, true] {
                for skipped in [None, Some(1_u8), Some(15), Some(30)] {
                    let z = ZDate::new(
                        System::Phugpa,
                        sample.year(),
                        sample.month(),
                        sample.is_leap_month(),
                        Tshes::new(number).unwrap(),
                        doubled,
                        skipped.map(|s| Tshes::new(s).unwrap()),
                    );
                    let expected = match table.get(&(month.key, number)) {
                        None => Err(Error::SkippedDay),
                        Some(entries) => {
                            match entries.iter().find(|e| e.0 == doubled && e.1 == skipped) {
                                Some(entry) => Ok(entry.2),
                                None => Err(Error::InvalidField),
                            }
                        }
                    };
                    assert_eq!(
                        to_gregorian(z),
                        expected,
                        "{month_key:?} tshes {number} doubled {doubled} skipped {skipped:?}",
                        month_key = month.key
                    );
                    classified += 1;
                    skipped_cases += usize::from(expected == Err(Error::SkippedDay));
                    invalid_cases += usize::from(expected == Err(Error::InvalidField));
                }
            }
        }
    }
    // The classification reached all three outcomes (METHOD rule 39: a check
    // that never meets a shape passes vacuously).
    assert!(classified > 100_000);
    assert!(skipped_cases > 0, "no skipped tshes was classified");
    assert!(invalid_cases > 0, "no invalid combination was classified");
}

/// `(next − previous) mod 30` for two tshes numbers.
fn tshes_step(previous: u8, next: u8) -> i32 {
    (i32::from(next) - i32::from(previous)).rem_euclid(30)
}

#[test]
fn p3_tshes_never_decrease_within_a_month_and_steps_are_zero_one_or_two() {
    let days = all_days();
    for pair in days.windows(2) {
        let (previous, next) = (&pair[0], &pair[1]);
        let step = tshes_step(tshes(previous), tshes(next));
        let same_month = month_key(&previous.z) == month_key(&next.z);
        assert!(
            step <= 2,
            "step {step} between {:?} and {:?}",
            previous.date,
            next.date
        );
        match step {
            // the same tshes twice: a repeated tshes, the first flagged false, the second true
            0 => {
                assert!(
                    same_month,
                    "a repeated tshes across a month boundary at {:?}",
                    next.date
                );
                assert!(
                    !previous.z.is_doubled() && next.z.is_doubled(),
                    "doubled flags at {:?}",
                    next.date
                );
                assert_eq!(next.z.skipped_before(), None);
            }
            1 => {
                assert!(!next.z.is_doubled());
                assert_eq!(next.z.skipped_before(), None);
            }
            _ => {
                // one tshes skipped between the two days, and named by `skipped_before`
                let skipped = u8::try_from(i32::from(tshes(previous)).rem_euclid(30) + 1).unwrap();
                assert!(!next.z.is_doubled());
                assert_eq!(
                    next.z.skipped_before().map(Tshes::get),
                    Some(skipped),
                    "skipped_before at {:?}",
                    next.date
                );
            }
        }
        // the month changes exactly when the tshes number falls (or wraps)
        assert_eq!(
            same_month,
            tshes(next) >= tshes(previous),
            "month change at {:?}",
            next.date
        );
    }
}

#[test]
fn p3_each_tshes_is_present_once_twice_or_absent_in_every_complete_month() {
    let days = all_days();
    let months = months_of(&days);
    for month in &months[1..months.len() - 1] {
        let mut counts = [0_usize; 31];
        for day in &days[month.start..month.end] {
            counts[usize::from(tshes(day))] += 1;
        }
        assert_eq!(counts[0], 0);
        for (number, count) in counts.iter().enumerate().skip(1) {
            assert!(
                *count <= 2,
                "tshes {number} occurs {count} times in {:?}",
                month.key
            );
        }
        // day 1 may be skipped (the month then starts at 2), the month ends at 29 or 30
        let first = tshes(&days[month.start]);
        let last = tshes(&days[month.end - 1]);
        assert!(first <= 2, "first tshes {first} in {:?}", month.key);
        assert!(last >= 29, "last tshes {last} in {:?}", month.key);
    }
}

#[test]
fn p4_complete_months_have_29_or_30_days() {
    let days = all_days();
    let months = months_of(&days);
    for month in &months[1..months.len() - 1] {
        let length = month.end - month.start;
        assert!(
            length == 29 || length == 30,
            "month {:?} has {length} days",
            month.key
        );
    }
}

#[test]
fn p4_month_sequence_follows_the_numbering_rules() {
    // Within a year the numbers rise 1..=12; a leap month is followed by the
    // regular month with the same number; month 12 is followed by month 1 of
    // the next year (a leap month 1 first, if there is one). Janson
    // (arXiv:1401.6285v1) §5, p. 11.
    let days = all_days();
    let months = months_of(&days);
    for pair in months.windows(2) {
        let (year, month, leap) = pair[0].key;
        let (next_year, next_month, next_leap) = pair[1].key;
        if leap {
            assert_eq!((next_year, next_month, next_leap), (year, month, false));
        } else if month == 12 {
            assert_eq!((next_year, next_month), (year + 1, 1));
        } else {
            assert_eq!((next_year, next_month), (year, month + 1));
        }
    }
}

/// The number of leap years from `y1` to `y2` inclusive, Janson
/// (arXiv:1401.6285v1) §5.5, eq. (5.36), p. 19, with γ* = 33 for Phugpa (eq. (5.39), p. 19).
fn leap_years_between(y1: i64, y2: i64) -> i64 {
    (24 * (y2 + 1) + 33).div_euclid(65) - (24 * y1 + 33).div_euclid(65)
}

#[test]
fn p4_leap_months_follow_the_intercalation_rule() {
    let days = all_days();
    let months = months_of(&days);
    let observed: Vec<(i64, u8)> = months
        .iter()
        .filter(|m| m.key.2)
        .map(|m| (m.key.0, m.key.1))
        .collect();
    // The Tibetan years with a day in the range: 1899..=2100 (a test below
    // derives them). Expected leap months by Janson's rule (5.36): the number of
    // leap years in that span.
    let expected_count = leap_years_between(1899, 2100);
    assert_eq!(
        to_i64(observed.len()),
        expected_count,
        "leap months: measured vs eq. (5.36)"
    );
    // Each leap year has its leap month where eq. (5.42), p. 19, puts it:
    // M = ⌊33 − ((24Y + 33) mod 65)/2⌋, and a year is a leap year iff
    // (24Y + 33) mod 65 ≥ 41 (eq. (5.41), p. 19).
    let mut formula: Vec<(i64, u8)> = Vec::new();
    for year in 1899..=2100_i64 {
        let residue = (24 * year + 33).rem_euclid(65);
        if residue >= 41 {
            // ⌊33 − r/2⌋ = 33 − ⌈r/2⌉ = 33 − (r + 1) div 2
            formula.push((
                year,
                u8::try_from(33 - (residue + 1).div_euclid(2)).unwrap(),
            ));
        }
    }
    assert_eq!(
        observed, formula,
        "leap months of the engine vs eqs. (5.41), (5.42)"
    );
    // Spacing (Janson §5.2, p. 12): the distance between successive leap months
    // alternates between 33 and 34 months.
    let leap_indices: Vec<usize> = months
        .iter()
        .enumerate()
        .filter(|(_, m)| m.key.2)
        .map(|(i, _)| i)
        .collect();
    let gaps: Vec<usize> = leap_indices.windows(2).map(|w| w[1] - w[0]).collect();
    assert!(gaps.iter().all(|g| *g == 33 || *g == 34), "gaps {gaps:?}");
    assert!(
        gaps.windows(2).all(|g| g[0] != g[1]),
        "gaps do not alternate: {gaps:?}"
    );
}

#[test]
fn supported_tibetan_years_are_1899_to_2100() {
    // Derives the constants `SUPPORTED_YEAR_MIN`/`MAX` of `phugpa`: the least and
    // greatest Tibetan year that has a day in the range.
    let days = all_days();
    let years: Vec<i64> = days.iter().map(|d| d.z.year().number()).collect();
    assert_eq!(*years.iter().min().unwrap(), 1899);
    assert_eq!(*years.iter().max().unwrap(), 2100);
}

#[test]
fn shape_census_over_the_range() {
    let days = all_days();
    let months = months_of(&days);
    let complete = &months[1..months.len() - 1];

    let years: HashSet<i64> = days.iter().map(|d| d.z.year().number()).collect();
    let leap_months = months.iter().filter(|m| m.key.2).count();
    let complete_leap = complete.iter().filter(|m| m.key.2).count();
    let days_29 = complete.iter().filter(|m| m.end - m.start == 29).count();
    let days_30 = complete.iter().filter(|m| m.end - m.start == 30).count();
    let skipped_tshes = days
        .iter()
        .filter(|d| d.z.skipped_before().is_some())
        .count();
    let doubled_tshes = days.iter().filter(|d| d.z.is_doubled()).count();
    let rollovers = days
        .windows(2)
        .filter(|p| {
            p[0].z.month().get() == 12
                && p[1].z.month().get() == 1
                && month_key(&p[0].z) != month_key(&p[1].z)
        })
        .count();
    let cycle_boundaries = days
        .windows(2)
        .filter(|p| p[0].z.year().rabjung_cycle() != p[1].z.year().rabjung_cycle())
        .count();

    println!("CENSUS days {}", days.len());
    println!("CENSUS tibetan years {}", years.len());
    println!(
        "CENSUS months (all, incl. the two partial at the edges) {}",
        months.len()
    );
    println!("CENSUS months (complete) {}", complete.len());
    println!("CENSUS leap months (all) {leap_months}, complete {complete_leap}");
    println!("CENSUS complete months of 29 days {days_29}, of 30 days {days_30}");
    println!("CENSUS skipped tshes (days with skipped_before) {skipped_tshes}");
    println!("CENSUS doubled tshes (days with is_doubled) {doubled_tshes}");
    println!("CENSUS 12 -> 1 rollovers {rollovers}");
    println!("CENSUS rabjung-cycle boundaries crossed {cycle_boundaries}");

    // Every shape the properties depend on was reached (METHOD rule 39).
    assert!(leap_months > 0 && days_29 > 0 && days_30 > 0);
    assert!(skipped_tshes > 0 && doubled_tshes > 0);
    // Independently derivable counts:
    assert_eq!(years.len(), 202, "Tibetan years 1899..=2100");
    assert_eq!(
        rollovers, 201,
        "one 12 -> 1 rollover per year boundary 1899/1900 .. 2099/2100"
    );
    assert_eq!(
        cycle_boundaries, 3,
        "cycles 15 -> 16 (1927), 16 -> 17 (1987), 17 -> 18 (2047) by eq. (4.1)"
    );
    assert_eq!(days_29 + days_30, complete.len());

    // The identity: over the complete months, skipped - doubled = 30 * months - days.
    // Skipped lunar days are counted from the days themselves (a tshes in 1..=30
    // that no day of the month carries), doubled ones as tshes carried twice.
    let mut missing_per_month: Vec<i64> = vec![0; months.len()];
    let mut skipped_by_absence = 0_i64;
    let mut doubled_by_presence = 0_i64;
    let mut days_in_complete = 0_i64;
    for (index, month) in months.iter().enumerate().skip(1).take(months.len() - 2) {
        let mut counts = [0_usize; 31];
        for day in &days[month.start..month.end] {
            counts[usize::from(tshes(day))] += 1;
        }
        missing_per_month[index] = to_i64(counts[1..].iter().filter(|c| **c == 0).count());
        skipped_by_absence += missing_per_month[index];
        doubled_by_presence += to_i64(counts[1..].iter().filter(|c| **c == 2).count());
        days_in_complete += to_i64(month.end - month.start);
    }
    let months_count = to_i64(complete.len());
    println!(
        "IDENTITY skipped {skipped_by_absence} - doubled {doubled_by_presence} = {} ; 30 * {months_count} - {days_in_complete} = {}",
        skipped_by_absence - doubled_by_presence,
        30 * months_count - days_in_complete
    );
    assert_eq!(
        skipped_by_absence - doubled_by_presence,
        30 * months_count - days_in_complete
    );

    // The `skipped_before` field agrees with the absence count, month by month:
    // a skipped lunar day belongs to the month of the day that records it,
    // except a skipped tshes 30, which is recorded on tshes 1 of the next month.
    let mut recorded_per_month: Vec<i64> = vec![0; months.len()];
    for (index, month) in months.iter().enumerate() {
        for day in &days[month.start..month.end] {
            if let Some(skipped) = day.z.skipped_before() {
                if skipped.get() == 30 {
                    assert!(
                        index > 0,
                        "a skipped 30 recorded in the first month of the range"
                    );
                    recorded_per_month[index - 1] += 1;
                } else {
                    recorded_per_month[index] += 1;
                }
            }
        }
    }
    for index in 1..months.len() - 1 {
        assert_eq!(
            recorded_per_month[index], missing_per_month[index],
            "skipped_before field vs absence count in month {:?}",
            months[index].key
        );
    }
}

#[test]
fn p7_outside_the_range_the_answer_is_out_of_range() {
    // The day before the range and the day after it.
    let before = GregorianDate::new(1899, 12, 31).unwrap();
    let after = GregorianDate::new(2101, 1, 1).unwrap();
    assert_eq!(to_zdate(before, System::Phugpa), Err(Error::OutOfRange));
    assert_eq!(to_zdate(after, System::Phugpa), Err(Error::OutOfRange));
    // The edges are inside.
    assert!(to_zdate(GregorianDate::new(1900, 1, 1).unwrap(), System::Phugpa).is_ok());
    assert!(to_zdate(GregorianDate::new(2100, 12, 31).unwrap(), System::Phugpa).is_ok());
    // Far outside, up to the ends of the Gregorian domain.
    for year in [
        -1_000_000_000,
        -4713,
        0,
        1,
        1027,
        1447,
        1582,
        1899,
        2101,
        5000,
        1_000_000_000,
    ] {
        let date = GregorianDate::new(year, 6, 15).unwrap();
        assert_eq!(
            to_zdate(date, System::Phugpa),
            Err(Error::OutOfRange),
            "year {year}"
        );
    }
    let last_of_domain = Jdn::new(365_244_221_425).unwrap().to_gregorian().unwrap();
    assert_eq!(
        to_zdate(last_of_domain, System::Phugpa),
        Err(Error::OutOfRange)
    );
}
