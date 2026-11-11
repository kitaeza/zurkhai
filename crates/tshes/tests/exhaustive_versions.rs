//! Properties P1–P4 and P7 (001 §6.4) for the Tsurphu and Genden versions, run
//! EXHAUSTIVELY over every day from 1900-01-01 to 2100-12-31, in both profiles
//! (spec 008 Item 4a; METHOD rule 41). The same properties, the same shapes and
//! the same assertions as `exhaustive.rs` has for Phugpa, written once as a
//! macro and instantiated for each system, so that no property is checked for
//! one version and forgotten for the other.
//!
//! **What these check, and what they do not (METHOD rule 35).** They check
//! *consistency*: the two directions agree, the structure is the one Janson
//! describes, the leap months are where the version's own leap-year rule puts
//! them. **A wrong epoch passes P1–P4**: a Tsurphu computed with Genden's epoch
//! values passes every test in this file (planted control, spec 008 report). The
//! correctness checks are the paper's printed tables (`p5_versions.rs`, Table 6
//! in the crate's unit tests) and the paper's own statements about the versions
//! (`versions_compared.rs`).
//!
//! Each version's leap rule is stated by the paper in general form: Y is a leap
//! year iff `(24Y + γ*) mod 65 ≥ 41` (eq. (5.33), p. 18) and then the leap month
//! is `M = ⌊33 − ((24Y + γ*) mod 65)/2⌋` (eq. (5.35), p. 19), with `γ* = 20` for
//! Tsurphu (A.2, p. 40) and for Genden (A.3, p. 43); the same rule in its first
//! form is eq. (5.24), p. 17: a leap month `M` exists in year `Y` iff
//! `24(Y − Y₀) + 2M ≡ β or β + 1 (mod 65)`, with `Y₀ = 1732, β = 142` (E1732) and
//! `Y₀ = 1747, β = 172` (E1747).

mod common;

use common::{Day, MonthKey, all_days_of, first_jdn, last_jdn, month_key, months_of};
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

/// `(next − previous) mod 30` for two tshes numbers.
fn tshes_step(previous: u8, next: u8) -> i32 {
    (i32::from(next) - i32::from(previous)).rem_euclid(30)
}

/// Janson (arXiv:1401.6285v1) §5.5, eq. (5.36), p. 19: the number of leap years
/// from `y1` to `y2` inclusive, for a version with the constant `γ*`.
fn leap_years_between(gamma_star: i64, y1: i64, y2: i64) -> i64 {
    (24 * (y2 + 1) + gamma_star).div_euclid(65) - (24 * y1 + gamma_star).div_euclid(65)
}

/// The leap month of year `y` by eqs. (5.33) and (5.35), p. 18–19, if there is one.
fn leap_month_by_gamma_star(gamma_star: i64, y: i64) -> Option<u8> {
    let residue = (24 * y + gamma_star).rem_euclid(65);
    if residue >= 41 {
        // ⌊33 − r/2⌋ = 33 − ⌈r/2⌉ = 33 − (r + 1) div 2
        Some(u8::try_from(33 - (residue + 1).div_euclid(2)).unwrap())
    } else {
        None
    }
}

/// The leap months of year `y` by the general rule, eq. (5.24), p. 17, with the
/// version's `Y₀` and `β`: every `M` in 1..=12 with `24(Y − Y₀) + 2M ≡ β or β + 1 (mod 65)`.
fn leap_months_by_5_24(y0: i64, beta: i64, y: i64) -> Vec<u8> {
    (1..=12_u8)
        .filter(|m| {
            let residue = (24 * (y - y0) + 2 * i64::from(*m)).rem_euclid(65);
            residue == beta.rem_euclid(65) || residue == (beta + 1).rem_euclid(65)
        })
        .collect()
}

macro_rules! version_suite {
    ($module:ident, $system:expr, $gamma_star:expr, $y0:expr, $beta:expr) => {
        mod $module {
            use super::*;

            const SYSTEM: System = $system;
            const GAMMA_STAR: i64 = $gamma_star;
            const Y0: i64 = $y0;
            const BETA: i64 = $beta;

            fn days() -> Vec<Day> {
                all_days_of(SYSTEM)
            }

            #[test]
            fn range_has_73414_days_and_every_zdate_carries_the_system() {
                let days = days();
                assert_eq!(days.len(), 73_414);
                assert!(days.iter().all(|d| d.z.system() == SYSTEM));
            }

            #[test]
            fn p1_every_day_maps_to_exactly_one_zdate() {
                let days = days();
                let distinct: HashSet<ZDate> = days.iter().map(|day| day.z).collect();
                assert_eq!(distinct.len(), days.len(), "two days share a ZDate");
                for day in &days {
                    assert_eq!(
                        to_zdate(day.date, SYSTEM),
                        Ok(day.z),
                        "not a function at {:?}",
                        day.date
                    );
                }
            }

            #[test]
            fn p2_round_trip_from_every_day() {
                for day in days() {
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
                for day in days() {
                    let date = to_gregorian(day.z).expect("a ZDate the engine produced is valid");
                    assert_eq!(
                        to_zdate(date, SYSTEM),
                        Ok(day.z),
                        "to_zdate(to_gregorian(z)) != z at {:?}",
                        day.date
                    );
                }
            }

            #[test]
            fn p2_every_combination_of_fields_is_classified_exactly() {
                let days = days();
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
                                    SYSTEM,
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
                                        match entries
                                            .iter()
                                            .find(|e| e.0 == doubled && e.1 == skipped)
                                        {
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
                println!(
                    "CLASSIFIED {classified}: skipped {skipped_cases}, invalid {invalid_cases}"
                );
                assert!(classified > 100_000);
                assert!(skipped_cases > 0, "no skipped tshes was classified");
                assert!(invalid_cases > 0, "no invalid combination was classified");
            }

            #[test]
            fn p3_tshes_never_decrease_within_a_month_and_steps_are_zero_one_or_two() {
                let days = days();
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
                            let skipped =
                                u8::try_from(i32::from(tshes(previous)).rem_euclid(30) + 1)
                                    .unwrap();
                            assert!(!next.z.is_doubled());
                            assert_eq!(
                                next.z.skipped_before().map(Tshes::get),
                                Some(skipped),
                                "skipped_before at {:?}",
                                next.date
                            );
                        }
                    }
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
                let days = days();
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
                    let first = tshes(&days[month.start]);
                    let last = tshes(&days[month.end - 1]);
                    assert!(first <= 2, "first tshes {first} in {:?}", month.key);
                    assert!(last >= 29, "last tshes {last} in {:?}", month.key);
                }
            }

            #[test]
            fn p4_complete_months_have_29_or_30_days() {
                let days = days();
                let months = months_of(&days);
                let (mut n29, mut n30) = (0_usize, 0_usize);
                for month in &months[1..months.len() - 1] {
                    let length = month.end - month.start;
                    assert!(
                        length == 29 || length == 30,
                        "month {:?} has {length} days",
                        month.key
                    );
                    n29 += usize::from(length == 29);
                    n30 += usize::from(length == 30);
                }
                println!("MONTHS of 29 days {n29}, of 30 days {n30}");
                assert!(n29 > 0 && n30 > 0);
            }

            #[test]
            fn p4_month_sequence_follows_the_numbering_rules() {
                // A leap month takes the number of the FOLLOWING month (Janson
                // (arXiv:1401.6285v1) §5, p. 11; Tsurphu A.2, p. 39; Genden A.3,
                // p. 43): the leap month is followed by the regular month with
                // its number, and month 12 by month 1 of the next year.
                let days = days();
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

            #[test]
            fn p4_leap_months_follow_the_versions_own_intercalation_rule() {
                let days = days();
                let months = months_of(&days);
                let observed: Vec<(i64, u8)> = months
                    .iter()
                    .filter(|m| m.key.2)
                    .map(|m| (m.key.0, m.key.1))
                    .collect();
                // The expected count, eq. (5.36), p. 19, over the Tibetan years
                // with a day in the range, 1899..=2100.
                let expected_count = leap_years_between(GAMMA_STAR, 1899, 2100);
                println!(
                    "LEAP months measured {}, eq. (5.36) with gamma* = {GAMMA_STAR} gives {expected_count}",
                    observed.len()
                );
                assert_eq!(
                    to_i64(observed.len()),
                    expected_count,
                    "leap months: measured vs eq. (5.36)"
                );
                // Each leap year has its leap month where eqs. (5.33) and (5.35)
                // put it, AND where the first form, eq. (5.24), with the
                // version's Y0 and beta puts it: two statements of one rule.
                let mut by_gamma: Vec<(i64, u8)> = Vec::new();
                for year in 1899..=2100_i64 {
                    let by_5_24 = leap_months_by_5_24(Y0, BETA, year);
                    let by_5_35 = leap_month_by_gamma_star(GAMMA_STAR, year);
                    assert_eq!(
                        by_5_24,
                        by_5_35.map_or_else(Vec::new, |m| vec![m]),
                        "eq. (5.24) vs eqs. (5.33), (5.35) in year {year}"
                    );
                    if let Some(m) = by_5_35 {
                        by_gamma.push((year, m));
                    }
                }
                assert_eq!(
                    observed, by_gamma,
                    "leap months of the engine vs eqs. (5.33), (5.35)"
                );
                // Spacing (Janson §5.2, p. 12): the distance between successive
                // leap months alternates between 33 and 34 months.
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
                // Derives the constants `SUPPORTED_YEAR_MIN`/`MAX` of `phugpa` for this version.
                let days = days();
                let years: Vec<i64> = days.iter().map(|d| d.z.year().number()).collect();
                assert_eq!(*years.iter().min().unwrap(), 1899);
                assert_eq!(*years.iter().max().unwrap(), 2100);
            }

            #[test]
            fn shape_census_over_the_range() {
                let days = days();
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
                    "CENSUS months (all, incl. the partial at the edges) {}",
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
                // Independently derivable counts (the same for every version):
                assert_eq!(years.len(), 202, "Tibetan years 1899..=2100");
                assert_eq!(rollovers, 201, "one 12 -> 1 rollover per year boundary");
                assert_eq!(cycle_boundaries, 3, "cycles 15 -> 16, 16 -> 17, 17 -> 18 (4.1)");
                assert_eq!(days_29 + days_30, complete.len());

                // The identity over the complete months:
                // skipped - doubled = 30 * months - days.
                let mut missing_per_month: Vec<i64> = vec![0; months.len()];
                let mut skipped_by_absence = 0_i64;
                let mut doubled_by_presence = 0_i64;
                let mut days_in_complete = 0_i64;
                for (index, month) in months.iter().enumerate().skip(1).take(months.len() - 2) {
                    let mut counts = [0_usize; 31];
                    for day in &days[month.start..month.end] {
                        counts[usize::from(tshes(day))] += 1;
                    }
                    missing_per_month[index] =
                        to_i64(counts[1..].iter().filter(|c| **c == 0).count());
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

                // `skipped_before` agrees with the absence count, month by month.
                let mut recorded_per_month: Vec<i64> = vec![0; months.len()];
                for (index, month) in months.iter().enumerate() {
                    for day in &days[month.start..month.end] {
                        if let Some(skipped) = day.z.skipped_before() {
                            if skipped.get() == 30 {
                                assert!(index > 0, "a skipped 30 recorded in the first month");
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
                let before = GregorianDate::new(1899, 12, 31).unwrap();
                let after = GregorianDate::new(2101, 1, 1).unwrap();
                assert_eq!(to_zdate(before, SYSTEM), Err(Error::OutOfRange));
                assert_eq!(to_zdate(after, SYSTEM), Err(Error::OutOfRange));
                assert!(to_zdate(GregorianDate::new(1900, 1, 1).unwrap(), SYSTEM).is_ok());
                assert!(to_zdate(GregorianDate::new(2100, 12, 31).unwrap(), SYSTEM).is_ok());
                for year in [
                    -1_000_000_000,
                    -4713,
                    0,
                    1,
                    1027,
                    1447,
                    1582,
                    1732,
                    1747,
                    1899,
                    2101,
                    5000,
                    1_000_000_000,
                ] {
                    let date = GregorianDate::new(year, 6, 15).unwrap();
                    assert_eq!(to_zdate(date, SYSTEM), Err(Error::OutOfRange), "year {year}");
                }
                let last_of_domain = Jdn::new(365_244_221_425).unwrap().to_gregorian().unwrap();
                assert_eq!(to_zdate(last_of_domain, SYSTEM), Err(Error::OutOfRange));
                // The range edges in JD terms.
                assert!(to_zdate(Jdn::new(first_jdn()).unwrap().to_gregorian().unwrap(), SYSTEM).is_ok());
                assert!(to_zdate(Jdn::new(last_jdn()).unwrap().to_gregorian().unwrap(), SYSTEM).is_ok());
            }

            #[test]
            fn a_zdate_of_another_system_is_refused() {
                // A ZDate carries its system and `to_gregorian` uses it: the same
                // fields under another system are another calendar's date, and a
                // ZDate this system produced is never accepted as another's.
                let day = &days()[1000];
                let other = if SYSTEM == System::Genden {
                    System::Tsurphu
                } else {
                    System::Genden
                };
                let moved = ZDate::new(
                    other,
                    day.z.year(),
                    day.z.month(),
                    day.z.is_leap_month(),
                    day.z.tshes(),
                    day.z.is_doubled(),
                    day.z.skipped_before(),
                );
                // Whatever the other system answers for these fields, it is that
                // system's answer: never this system's day by accident of the type.
                match to_gregorian(moved) {
                    Ok(date) => assert_eq!(to_zdate(date, other), Ok(moved)),
                    Err(error) => assert!(matches!(
                        error,
                        Error::SkippedDay | Error::InvalidField | Error::OutOfRange
                    )),
                }
            }
        }
    };
}

version_suite!(tsurphu, System::Tsurphu, 20, 1732, 142);
version_suite!(genden, System::Genden, 20, 1747, 172);
