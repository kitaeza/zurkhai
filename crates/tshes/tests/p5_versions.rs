//! P5 (001 §6.4: "Reproduces the system's own published tables (T5) exactly")
//! for the Tsurphu and Genden versions, from the columns headed "Tsurphu" and
//! "Mongolia" of Tables 7, 8 and 9 of S. Janson, "Tibetan calendar
//! mathematics", arXiv:1401.6285v1 (spec 008 Items 5 and 6). Table 6 (the epoch
//! values) needs the private epoch constants and is tested inside the crate
//! (`phugpa::version_tests`: `include_str!` of the fixture).
//!
//! The engine is reached through the public API only: `to_zdate` with
//! `System::Tsurphu` and `System::Genden` named explicitly (`DEBT.md` D-1). The
//! fixtures are parsed with std; no dependency is added.
//!
//! **What these check, and what they do not (METHOD rules 25, 35).** Table 9
//! decides only the first day of a Tibetan year: its tests claim nothing about
//! any other day. Table 7 decides only which years have a leap month and its
//! number; Table 8 only the repeated and skipped numbers of Tibetan year 2012.
//! Tables 7 and 9 print the same values for the two versions on every row, so
//! they cannot tell Tsurphu from Genden; only Table 8 (months 9 and 11) and
//! Table 6 can (see `the_tables_separate_tsurphu_from_genden_by_8_printed_cells`).
//!
//! The "paper against itself" tests (`t5_*`) never call the engine: they compare
//! the fixtures with each other and with Janson's own formulas, so a fixture
//! that contradicts the paper stops the round (spec 008 STOP RULE (4)) before it
//! is used as an oracle.

mod common;

use common::all_days_of;
use common::fixtures::{Ymd, day_number, iso, load, parse_iso};
use std::collections::{BTreeMap, BTreeSet};
use tshes::{GregorianDate, System, to_zdate};

// ------------------------------------------------------------------ fixtures

/// The files of one version.
#[derive(Clone, Copy)]
struct Version {
    system: System,
    name: &'static str,
    table7: &'static str,
    table8: &'static str,
    table9: &'static str,
    /// Janson's `Y₀` and `β` (A.2, p. 39–40; A.3, p. 42–43) and `γ*` (p. 40, p. 43).
    y0: i64,
    beta: i64,
    gamma_star: i64,
}

const TSURPHU: Version = Version {
    system: System::Tsurphu,
    name: "Tsurphu",
    table7: "janson-v1-table7-leap-months-tsurphu.csv",
    table8: "janson-v1-table8-2012-tsurphu.csv",
    table9: "janson-v1-table9-losar-tsurphu.csv",
    y0: 1732,
    beta: 142,
    gamma_star: 20,
};

const GENDEN: Version = Version {
    system: System::Genden,
    name: "Genden (the column headed Mongolia)",
    table7: "janson-v1-table7-leap-months-genden.csv",
    table8: "janson-v1-table8-2012-genden.csv",
    table9: "janson-v1-table9-losar-genden.csv",
    y0: 1747,
    beta: 172,
    gamma_star: 20,
};

/// The Phugpa files of round 006, for the cross-version checks.
const PHUGPA_TABLE8: &str = "janson-v1-table8-2012-phugpa.csv";
const PHUGPA_TABLE9: &str = "janson-v1-table9-losar-phugpa.csv";
const TABLE6: &str = "janson-v1-table6-epochs-806.csv";

/// A row of Table 9: `(year, first day of that year)`.
fn table9(file: &str) -> Vec<(i64, Ymd)> {
    load(file, &["tibetan_year", "gregorian_date"])
        .iter()
        .map(|f| (f[0].parse().expect("tibetan_year"), parse_iso(&f[1])))
        .collect()
}

/// A row of Table 7: `(year, leap month number or none)`.
fn table7(file: &str) -> Vec<(i64, Option<u8>)> {
    load(file, &["year", "leap_month"])
        .iter()
        .map(|f| {
            let leap = if f[1].is_empty() {
                None
            } else {
                Some(f[1].parse().expect("leap_month"))
            };
            (f[0].parse().expect("year"), leap)
        })
        .collect()
}

/// An event of Table 8: `(month, tshes, kind)`, kind `repeated` or `skipped`.
type Event = (u8, u8, String);

fn table8(file: &str) -> Vec<Event> {
    load(file, &["tibetan_year", "month", "tshes", "kind"])
        .iter()
        .map(|f| {
            assert_eq!(f[0], "2012", "Table 8 holds Tibetan year 2012 only");
            let kind = f[3].clone();
            assert!(kind == "repeated" || kind == "skipped", "kind: {kind}");
            (
                f[1].parse().expect("month"),
                f[2].parse().expect("tshes"),
                kind,
            )
        })
        .collect()
}

// ------------------------------------------------------- the engine's answers

fn ymd(date: GregorianDate) -> Ymd {
    (date.year(), date.month(), date.day())
}

/// The engine's first day of Tibetan year `year` under `system`: the first
/// Gregorian day from 1 January of that year whose `ZDate` lies in the Tibetan
/// year `year` (the year begins with month 1, or with the leap month 1, Janson
/// v1 p. 9; it is the day after the last day of the previous year, p. 27).
fn engine_first_day(system: System, year: i64) -> Option<GregorianDate> {
    for month in 1..=4 {
        for day in 1..=31 {
            if let Ok(date) = GregorianDate::new(year, month, day) {
                let z = to_zdate(date, system).expect("in range");
                if z.year().number() == year {
                    return Some(date);
                }
            }
        }
    }
    None
}

/// Table 9 against the engine's first day of each year: one message per disagreement.
fn table9_mismatches(system: System, rows: &[(i64, Ymd)]) -> Vec<String> {
    let mut out = Vec::new();
    for &(year, expected) in rows {
        match engine_first_day(system, year) {
            Some(date) if ymd(date) == expected => {}
            Some(date) => out.push(format!(
                "Table 9, Tibetan year {year}: first day expected {} (table), engine {}",
                iso(expected),
                iso(ymd(date))
            )),
            None => out.push(format!(
                "Table 9, Tibetan year {year}: the engine has no first day"
            )),
        }
    }
    out
}

/// The leap month numbers of every Tibetan year the range holds, from every day
/// of 1900-01-01..=2100-12-31.
fn engine_leap_months(system: System) -> BTreeMap<i64, BTreeSet<u8>> {
    let mut leaps: BTreeMap<i64, BTreeSet<u8>> = BTreeMap::new();
    for day in all_days_of(system) {
        let year = day.z.year().number();
        leaps.entry(year).or_default();
        if day.z.is_leap_month() {
            leaps.entry(year).or_default().insert(day.z.month().get());
        }
    }
    leaps
}

/// Table 7 against the engine's leap months: a printed leap month must be the
/// engine's, and an empty cell must be a year without one.
fn table7_mismatches(
    rows: &[(i64, Option<u8>)],
    engine: &BTreeMap<i64, BTreeSet<u8>>,
) -> Vec<String> {
    let mut out = Vec::new();
    for &(year, cell) in rows {
        let expected: BTreeSet<u8> = cell.into_iter().collect();
        match engine.get(&year) {
            Some(actual) if *actual == expected => {}
            Some(actual) => out.push(format!(
                "Table 7, year {year}: leap months expected {expected:?} (table), engine {actual:?}"
            )),
            None => out.push(format!(
                "Table 7, year {year}: the engine's range holds no day of it"
            )),
        }
    }
    out
}

/// The engine's repeated and skipped numbers in each month 1..=12 of Tibetan
/// year 2012, counted from the days of each month (a number on two days is
/// repeated, a number on no day is skipped).
fn engine_events_2012(system: System) -> BTreeSet<Event> {
    let mut months: BTreeMap<u8, BTreeMap<u8, u32>> = BTreeMap::new();
    for day in all_days_of(system) {
        if day.z.year().number() != 2012 {
            continue;
        }
        assert!(
            !day.z.is_leap_month(),
            "2012 has no leap month (Table 7, Table 8's note)"
        );
        *months
            .entry(day.z.month().get())
            .or_default()
            .entry(day.z.tshes().get())
            .or_default() += 1;
    }
    assert_eq!(
        months.len(),
        12,
        "the engine holds all twelve months of 2012"
    );
    let mut events = BTreeSet::new();
    for (month, counts) in &months {
        for tshes in 1..=30 {
            match counts.get(&tshes).copied().unwrap_or(0) {
                0 => events.insert((*month, tshes, "skipped".to_owned())),
                1 => false,
                2 => events.insert((*month, tshes, "repeated".to_owned())),
                n => panic!("tshes {tshes} of month {month} is on {n} days"),
            };
        }
    }
    events
}

/// Table 8 against the engine's events, in both directions: an event the table
/// prints and the engine lacks, and an event the engine has where the table
/// prints none (so a month with no entry is checked too).
fn table8_mismatches(table: &[Event], engine: &BTreeSet<Event>) -> Vec<String> {
    let printed: BTreeSet<Event> = table.iter().cloned().collect();
    let mut out = Vec::new();
    for (month, tshes, kind) in printed.difference(engine) {
        out.push(format!(
            "Table 8, month {month}: {kind} {tshes} is in the table and not in the engine"
        ));
    }
    for (month, tshes, kind) in engine.difference(&printed) {
        out.push(format!(
            "Table 8, month {month}: {kind} {tshes} is in the engine and not in the table"
        ));
    }
    out
}

fn first_of(mismatches: &[String]) -> &str {
    mismatches.first().map_or("none", String::as_str)
}

// ------------------------------------------------------------------ P5 tests

fn p5_table9(version: &Version) {
    let rows = table9(version.table9);
    assert_eq!(rows.len(), 31, "Table 9 rows loaded");
    assert_eq!((rows[0].0, rows[30].0), (2000, 2030));
    let mismatches = table9_mismatches(version.system, &rows);
    assert!(
        mismatches.is_empty(),
        "P5 Table 9, {}: {} mismatches in {} rows; the first: {}",
        version.name,
        mismatches.len(),
        rows.len(),
        first_of(&mismatches)
    );
}

fn p5_table7(version: &Version) {
    let rows = table7(version.table7);
    assert_eq!(rows.len(), 21, "Table 7 rows loaded");
    assert_eq!((rows[0].0, rows[20].0), (2000, 2020));
    assert_eq!(
        rows.iter().filter(|r| r.1.is_some()).count(),
        8,
        "years with a leap month"
    );
    let mismatches = table7_mismatches(&rows, &engine_leap_months(version.system));
    assert!(
        mismatches.is_empty(),
        "P5 Table 7, {}: {} mismatches in {} rows; the first: {}",
        version.name,
        mismatches.len(),
        rows.len(),
        first_of(&mismatches)
    );
}

fn p5_table8(version: &Version) {
    let rows = table8(version.table8);
    assert_eq!(rows.len(), 21, "Table 8 events loaded");
    assert_eq!(rows.iter().filter(|e| e.2 == "repeated").count(), 8);
    assert_eq!(rows.iter().filter(|e| e.2 == "skipped").count(), 13);
    // every month but 10 has an entry; month 10's cell is empty
    let months: BTreeSet<u8> = rows.iter().map(|e| e.0).collect();
    assert_eq!(
        months,
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12].into_iter().collect()
    );
    let mismatches = table8_mismatches(&rows, &engine_events_2012(version.system));
    assert!(
        mismatches.is_empty(),
        "P5 Table 8, {}: {} mismatches in {} events; the first: {}",
        version.name,
        mismatches.len(),
        rows.len(),
        first_of(&mismatches)
    );
}

#[test]
fn p5_table9_first_day_of_every_year_matches_the_engine_tsurphu() {
    p5_table9(&TSURPHU);
}

#[test]
fn p5_table9_first_day_of_every_year_matches_the_engine_genden() {
    p5_table9(&GENDEN);
}

#[test]
fn p5_table7_leap_month_of_every_printed_year_matches_the_engine_tsurphu() {
    p5_table7(&TSURPHU);
}

#[test]
fn p5_table7_leap_month_of_every_printed_year_matches_the_engine_genden() {
    p5_table7(&GENDEN);
}

#[test]
fn p5_table8_repeated_and_skipped_numbers_of_2012_match_the_engine_tsurphu() {
    p5_table8(&TSURPHU);
}

#[test]
fn p5_table8_repeated_and_skipped_numbers_of_2012_match_the_engine_genden() {
    p5_table8(&GENDEN);
}

// ----------------------------------------------------------- seeded controls

#[test]
fn seeded_control_table9_one_wrong_date_is_reported() {
    let mut rows = table9(TSURPHU.table9);
    let row = rows.iter_mut().find(|r| r.0 == 2025).unwrap();
    assert_eq!(row.1, (2025, 3, 1), "the committed row");
    row.1 = (2025, 3, 2);
    let mismatches = table9_mismatches(System::Tsurphu, &rows);
    assert_eq!(
        mismatches,
        ["Table 9, Tibetan year 2025: first day expected 2025-03-02 (table), engine 2025-03-01"]
    );
    // The Genden file, fed Tsurphu's engine with the Tsurphu file's own date, passes: they are equal.
    let genden_against_tsurphu_file = table9_mismatches(System::Genden, &table9(TSURPHU.table9));
    assert_eq!(
        genden_against_tsurphu_file.len(),
        0,
        "{genden_against_tsurphu_file:?}"
    );
    let mut genden = table9(GENDEN.table9);
    genden.iter_mut().find(|r| r.0 == 2006).unwrap().1 = (2006, 2, 28); // Phugpa's date
    assert_eq!(
        table9_mismatches(System::Genden, &genden),
        ["Table 9, Tibetan year 2006: first day expected 2006-02-28 (table), engine 2006-01-30"]
    );
}

#[test]
fn seeded_control_table7_one_wrong_leap_month_is_reported() {
    let mut rows = table7(TSURPHU.table7);
    let row = rows.iter_mut().find(|r| r.0 == 2008).unwrap();
    assert_eq!(row.1, Some(9), "the committed row");
    row.1 = Some(10);
    let mismatches = table7_mismatches(&rows, &engine_leap_months(System::Tsurphu));
    assert_eq!(
        mismatches,
        ["Table 7, year 2008: leap months expected {10} (table), engine {9}"]
    );
    // an absent leap month is checked too: a printed cell for a year with none
    let mut genden = table7(GENDEN.table7);
    genden.iter_mut().find(|r| r.0 == 2001).unwrap().1 = Some(5);
    assert_eq!(
        table7_mismatches(&genden, &engine_leap_months(System::Genden)),
        ["Table 7, year 2001: leap months expected {5} (table), engine {}"]
    );
}

#[test]
fn seeded_control_table8_one_wrong_tshes_is_reported() {
    let mut rows = table8(TSURPHU.table8);
    let row = rows
        .iter_mut()
        .find(|e| e.0 == 9 && e.1 == 19 && e.2 == "repeated")
        .expect("the committed row: month 9 repeats 19");
    row.1 = 21;
    let mismatches = table8_mismatches(&rows, &engine_events_2012(System::Tsurphu));
    assert_eq!(
        mismatches,
        [
            "Table 8, month 9: repeated 21 is in the table and not in the engine",
            "Table 8, month 9: repeated 19 is in the engine and not in the table"
        ]
    );
}

#[test]
fn seeded_control_the_other_versions_table_8_is_reported_in_months_9_and_11_only() {
    // Genden's printed events fed against Tsurphu's engine, and Tsurphu's against
    // Genden's: exactly the two cells where the printed columns differ.
    let against_tsurphu =
        table8_mismatches(&table8(GENDEN.table8), &engine_events_2012(System::Tsurphu));
    assert_eq!(
        against_tsurphu,
        [
            "Table 8, month 9: repeated 20 is in the table and not in the engine",
            "Table 8, month 11: skipped 4 is in the table and not in the engine",
            "Table 8, month 9: repeated 19 is in the engine and not in the table",
            "Table 8, month 11: skipped 3 is in the engine and not in the table"
        ]
    );
    let against_genden =
        table8_mismatches(&table8(TSURPHU.table8), &engine_events_2012(System::Genden));
    assert_eq!(
        against_genden,
        [
            "Table 8, month 9: repeated 19 is in the table and not in the engine",
            "Table 8, month 11: skipped 3 is in the table and not in the engine",
            "Table 8, month 9: repeated 20 is in the engine and not in the table",
            "Table 8, month 11: skipped 4 is in the engine and not in the table"
        ]
    );
}

// ------------------------------------------------- the paper against itself

fn days_between(from: Ymd, to: Ymd) -> i64 {
    day_number(to) - day_number(from)
}

/// `(skipped, repeated)` per month from a Table 8 fixture.
fn table8_counts(events: &[Event]) -> BTreeMap<u8, (i64, i64)> {
    let mut counts: BTreeMap<u8, (i64, i64)> = (1..=12).map(|m| (m, (0, 0))).collect();
    for (month, _, kind) in events {
        let entry = counts.get_mut(month).expect("month 1..=12");
        if kind == "skipped" {
            entry.0 += 1;
        } else {
            entry.1 += 1;
        }
    }
    counts
}

#[test]
fn t5_table9_tsurphu_equals_genden_on_every_row() {
    // Janson v1 p. 52: "we do not see any differences between the Tsurphu and
    // Mongolian New Year in Table 9".
    let (tsurphu, genden) = (table9(TSURPHU.table9), table9(GENDEN.table9));
    assert_eq!(tsurphu.len(), 31);
    assert_eq!(tsurphu, genden);
}

#[test]
fn t5_table7_tsurphu_equals_genden_on_every_row() {
    // p. 51: "the Tsurphu and Mongolian versions have the same leap months".
    let (tsurphu, genden) = (table7(TSURPHU.table7), table7(GENDEN.table7));
    assert_eq!(tsurphu.len(), 21);
    assert_eq!(tsurphu, genden);
}

/// The leap months of year `y` by the general rule, eq. (5.24), p. 17.
fn leap_months_by_5_24(version: &Version, y: i64) -> Vec<u8> {
    (1..=12_u8)
        .filter(|m| {
            let residue = (24 * (y - version.y0) + 2 * i64::from(*m)).rem_euclid(65);
            residue == version.beta.rem_euclid(65) || residue == (version.beta + 1).rem_euclid(65)
        })
        .collect()
}

#[test]
fn t5_table7_follows_the_general_leap_month_form_with_each_versions_constants() {
    // Janson v1 §5.4, eq. (5.24), p. 17 ("the rule (5.24) holds also for other
    // versions of the Tibetan calendar, with appropriate β", p. 18), with Y0 and
    // β of A.2 (p. 39–40) and A.3 (p. 42–43); and §5.5, eqs. (5.33), (5.35),
    // p. 18–19, with γ* = 20 (p. 40, p. 43).
    for version in [&TSURPHU, &GENDEN] {
        let rows = table7(version.table7);
        for (year, cell) in &rows {
            let by_5_24 = leap_months_by_5_24(version, *year);
            let residue = (24 * year + version.gamma_star).rem_euclid(65);
            let by_gamma: Vec<u8> = if residue >= 41 {
                vec![u8::try_from(33 - (residue + 1).div_euclid(2)).unwrap()]
            } else {
                Vec::new()
            };
            let printed: Vec<u8> = cell.iter().copied().collect();
            assert_eq!(
                by_5_24, printed,
                "{}: eq. (5.24) vs the table, year {year}",
                version.name
            );
            assert_eq!(
                by_gamma, printed,
                "{}: eqs. (5.33), (5.35) vs the table, year {year}",
                version.name
            );
        }
        // The seeded control: a wrong beta fails the same comparison.
        let wrong = Version {
            beta: version.beta + 2,
            ..*version
        };
        assert!(
            rows.iter().any(|(y, cell)| leap_months_by_5_24(&wrong, *y)
                != cell.iter().copied().collect::<Vec<u8>>()),
            "a wrong beta must disagree with Table 7 somewhere"
        );
    }
}

#[test]
fn t5_table7_year_column_read_as_the_tibetan_year_or_the_start_year_gives_the_same_rows() {
    // The text (p. 51) says Table 7 shows leap months for "(Gregorian) years". Read as
    // Janson's Y (the Tibetan year, p. 8) or as the Gregorian year in which the leap month
    // begins, the 21 rows are the same: computed here from the paper's own Table 9 (the
    // first day of the Tibetan year Y, in Gregorian year Y) and the month number, never from
    // the engine. A leap month M of year Y comes after the M-1 regular months before it, each
    // at most 30 days, so it begins between the first day of Y and 30·(M-1) days later; with
    // M ≤ 11 that is inside the Gregorian year Y, so the two readings agree on every row.
    for version in [&TSURPHU, &GENDEN] {
        let new_year: BTreeMap<i64, Ymd> = table9(version.table9).into_iter().collect();
        let mut leap_rows = 0;
        for (year, cell) in table7(version.table7) {
            let Some(month) = cell else { continue };
            leap_rows += 1;
            assert!(
                month <= 11,
                "{}: a leap month 12 would begin near the year's end",
                version.name
            );
            let start = day_number(new_year[&year]) + 30 * (i64::from(month) - 1);
            let december_31 = day_number((year, 12, 31));
            assert!(
                start <= december_31,
                "{}: leap month {month} of {year} could begin after 31 December",
                version.name
            );
            assert_eq!(
                new_year[&year].0, year,
                "the year's first day lies in Gregorian year {year}"
            );
        }
        assert_eq!(leap_rows, 8);
    }
}

#[test]
fn t5_table8_year_length_equals_losar_to_losar_for_all_three_versions() {
    // 30 × 12 − skipped + repeated = the days from Losar 2012 to Losar 2013 (Table 9),
    // for each of the three versions this project has (the Phugpa files are round 006's).
    for (name, t8, t9) in [
        ("Phugpa", PHUGPA_TABLE8, PHUGPA_TABLE9),
        ("Tsurphu", TSURPHU.table8, TSURPHU.table9),
        ("Genden", GENDEN.table8, GENDEN.table9),
    ] {
        let counts = table8_counts(&table8(t8));
        let skipped: i64 = counts.values().map(|c| c.0).sum();
        let repeated: i64 = counts.values().map(|c| c.1).sum();
        let days_in_table_8 = 30 * 12 - skipped + repeated;
        let rows: BTreeMap<i64, Ymd> = table9(t9).into_iter().collect();
        let losar_to_losar = days_between(rows[&2012], rows[&2013]);
        println!(
            "{name}: 360 - {skipped} + {repeated} = {days_in_table_8}; Losar 2012 to Losar 2013 = {losar_to_losar}"
        );
        assert_eq!(days_in_table_8, losar_to_losar, "{name}");
        assert_eq!(days_in_table_8, 355, "{name}");
    }
}

#[test]
fn t5_table8_each_month_has_the_same_length_in_the_three_versions() {
    // p. 51: "each month has the same length in all four versions"; checked for the three
    // versions this project has: per month, 30 − skipped + repeated is equal.
    let phugpa = table8_counts(&table8(PHUGPA_TABLE8));
    let tsurphu = table8_counts(&table8(TSURPHU.table8));
    let genden = table8_counts(&table8(GENDEN.table8));
    let mut lengths = Vec::new();
    for month in 1..=12_u8 {
        let length = |c: &BTreeMap<u8, (i64, i64)>| 30 - c[&month].0 + c[&month].1;
        let (p, t, g) = (length(&phugpa), length(&tsurphu), length(&genden));
        assert!(
            p == t && t == g,
            "month {month}: Phugpa {p}, Tsurphu {t}, Genden {g}"
        );
        assert!(p == 29 || p == 30, "month {month} has {p} days");
        lengths.push(p);
    }
    assert_eq!(lengths.iter().sum::<i64>(), 355);
    // The seeded control: dropping one event from one version breaks the equality.
    let mut shortened = table8(GENDEN.table8);
    shortened.retain(|e| !(e.0 == 11 && e.2 == "repeated"));
    let broken = table8_counts(&shortened);
    assert_ne!(
        30 - broken[&11].0 + broken[&11].1,
        30 - tsurphu[&11].0 + tsurphu[&11].1
    );
}

#[test]
fn the_tables_separate_tsurphu_from_genden_by_8_printed_cells() {
    // How far P5 can tell the two versions apart: the printed cells that differ in
    // Tables 6–9. Table 7 and Table 9: none. Table 8: the cells of months 9 and 11.
    // Table 6: all six cells of the two rows (the three epoch values, radix and decimal).
    let t7 = (table7(TSURPHU.table7), table7(GENDEN.table7));
    let t9 = (table9(TSURPHU.table9), table9(GENDEN.table9));
    let differing_7 = t7.0.iter().zip(&t7.1).filter(|(a, b)| a != b).count();
    let differing_9 = t9.0.iter().zip(&t9.1).filter(|(a, b)| a != b).count();
    let (t8_t, t8_g) = (table8(TSURPHU.table8), table8(GENDEN.table8));
    let months_differing: BTreeSet<u8> = (1..=12_u8)
        .filter(|m| {
            let cell = |events: &[Event]| -> Vec<(u8, String)> {
                events
                    .iter()
                    .filter(|e| e.0 == *m)
                    .map(|e| (e.1, e.2.clone()))
                    .collect()
            };
            cell(&t8_t) != cell(&t8_g)
        })
        .collect();
    let table6 = load(
        TABLE6,
        &[
            "system",
            "m0_radix",
            "m0_decimal",
            "s0_radix",
            "s0_decimal",
            "a0_radix",
            "a0_decimal",
        ],
    );
    assert_eq!(table6.len(), 3, "Table 6 rows loaded");
    let tsurphu_row = table6.iter().find(|r| r[0] == "tsurphu").unwrap();
    let genden_row = table6.iter().find(|r| r[0] == "genden").unwrap();
    let differing_6 = (1..=6)
        .filter(|i| tsurphu_row[*i] != genden_row[*i])
        .count();
    println!(
        "cells separating Tsurphu from Genden: Table 6 {differing_6}, Table 7 {differing_7}, Table 8 {} (months {months_differing:?}), Table 9 {differing_9}",
        months_differing.len()
    );
    assert_eq!(differing_7, 0);
    assert_eq!(differing_9, 0);
    assert_eq!(months_differing, [9, 11].into_iter().collect());
    assert_eq!(differing_6, 6);
    assert_eq!(
        differing_6 + months_differing.len() + differing_7 + differing_9,
        8
    );
    // In Table 8 the difference is one entry in each of the two cells: in month 9 the
    // repeated day (19 vs 20), in month 11 the first skipped day (3 vs 4).
    let entry = |events: &[Event], month: u8, kind: &str| -> Vec<u8> {
        events
            .iter()
            .filter(|e| e.0 == month && e.2 == kind)
            .map(|e| e.1)
            .collect()
    };
    assert_eq!(
        (entry(&t8_t, 9, "repeated"), entry(&t8_g, 9, "repeated")),
        (vec![19], vec![20])
    );
    assert_eq!(
        (entry(&t8_t, 11, "skipped"), entry(&t8_g, 11, "skipped")),
        (vec![3, 28], vec![4, 28])
    );
}
