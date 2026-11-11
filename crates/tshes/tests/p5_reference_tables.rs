//! P5 (001 §6.4: "Reproduces the system's own published tables (T5)
//! exactly"), from the Phugpa columns of S. Janson, "Tibetan calendar
//! mathematics", arXiv:1401.6285v1: Tables 1, 7, 8 and 9 (spec 006 Items 3 and 4).
//!
//! The engine is reached through the public API only: `to_zdate` with
//! `System::Phugpa` named explicitly (`DEBT.md` D-1), and the `ZDate` and
//! `ZYear` accessors. The fixtures are parsed with std; no dependency is added.
//!
//! **What these check, and what they do not (METHOD rule 25).** Tables 1 and 9
//! decide only the first day of a Tibetan year: their tests claim nothing about
//! any other day. Table 7 decides only which years have a leap month and its
//! number; Table 8 decides only the repeated and skipped numbers of the months
//! of Tibetan year 2012. This is the independent check P1–P4 could not give
//! (round 005's planted control C passed all of them): the tables are printed
//! values, not derived from the engine and not produced by software.
//!
//! The "paper against itself" tests (`t5_*`) never call the engine: they compare
//! the fixtures with each other and with Janson's own formulas, so a fixture
//! that contradicts the paper stops the round (spec 006 STOP RULE (3)) before
//! it is used as an oracle.

mod common;

use common::all_days;
use common::fixtures::{JD_OF_1970_01_01, T4Row, Ymd, day_number, iso, load, parse_iso, t4_rows};
use std::collections::{BTreeMap, BTreeSet};
use tshes::{GregorianDate, System, ZDate, to_zdate};

// ------------------------------------------------------------------ fixtures

const TABLE1: &str = "janson-v1-table1-losar-phugpa.csv";
const TABLE9: &str = "janson-v1-table9-losar-phugpa.csv";
const TABLE7: &str = "janson-v1-table7-leap-months-phugpa.csv";
const TABLE8: &str = "janson-v1-table8-2012-phugpa.csv";

/// A row of Table 1: the first day of Tibetan year `year` and the year's name.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Table1Row {
    year: i64,
    date: Ymd,
    element: String,
    animal: String,
}

fn table1() -> Vec<Table1Row> {
    load(
        TABLE1,
        &["tibetan_year", "gregorian_date", "element", "animal"],
    )
    .iter()
    .map(|f| Table1Row {
        year: f[0].parse().expect("tibetan_year"),
        date: parse_iso(&f[1]),
        element: f[2].clone(),
        animal: f[3].clone(),
    })
    .collect()
}

/// A row of Table 9's Phugpa column: `(year, first day of that year)`.
fn table9() -> Vec<(i64, Ymd)> {
    load(TABLE9, &["tibetan_year", "gregorian_date"])
        .iter()
        .map(|f| (f[0].parse().expect("tibetan_year"), parse_iso(&f[1])))
        .collect()
}

/// A row of Table 7's Phugpa column: `(year, leap month number or none)`.
fn table7() -> Vec<(i64, Option<u8>)> {
    load(TABLE7, &["year", "leap_month"])
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

fn table8() -> Vec<Event> {
    load(TABLE8, &["tibetan_year", "month", "tshes", "kind"])
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

#[test]
fn day_number_matches_the_published_anchors() {
    // Janson v1, p. 6: "1 January 2007 has JD 2454102"; the JD of 1970-01-01 is 2440588.
    assert_eq!(day_number((1970, 1, 1)), 0);
    assert_eq!(day_number((2007, 1, 1)) + JD_OF_1970_01_01, 2_454_102);
    // a leap day, and the Gregorian century rule
    assert_eq!(day_number((2000, 3, 1)) - day_number((2000, 2, 28)), 2);
    assert_eq!(day_number((1900, 3, 1)) - day_number((1900, 2, 28)), 1);
}

// ------------------------------------------------------- the engine's answers

fn ymd(date: GregorianDate) -> Ymd {
    (date.year(), date.month(), date.day())
}

/// The engine's first day of Tibetan year `year`: the first Gregorian day from
/// 1 January of that year whose Phugpa `ZDate` lies in the Tibetan year `year`
/// (the year begins with month 1, or with the leap month 1, Janson v1 p. 9).
fn engine_first_day(year: i64) -> Option<(GregorianDate, ZDate)> {
    for month in 1..=4 {
        for day in 1..=31 {
            if let Ok(date) = GregorianDate::new(year, month, day) {
                let z = to_zdate(date, System::Phugpa).expect("in range");
                if z.year().number() == year {
                    return Some((date, z));
                }
            }
        }
    }
    None
}

/// Table 1 against the engine: one message per disagreement.
fn table1_mismatches(rows: &[Table1Row]) -> Vec<String> {
    let mut out = Vec::new();
    for row in rows {
        let Some((date, z)) = engine_first_day(row.year) else {
            out.push(format!(
                "Table 1, Tibetan year {}: the engine has no first day before 1 May",
                row.year
            ));
            continue;
        };
        if ymd(date) != row.date {
            out.push(format!(
                "Table 1, Tibetan year {}: first day expected {} (table), engine {}",
                row.year,
                iso(row.date),
                iso(ymd(date))
            ));
        }
        let element = format!("{:?}", z.year().element());
        let animal = format!("{:?}", z.year().animal());
        if element != row.element || animal != row.animal {
            out.push(format!(
                "Table 1, Tibetan year {}: name expected {}-{} (table), engine {element}-{animal}",
                row.year, row.element, row.animal
            ));
        }
    }
    out
}

/// Table 9 (Phugpa column) against the engine's first day of each year.
fn table9_mismatches(rows: &[(i64, Ymd)]) -> Vec<String> {
    let mut out = Vec::new();
    for &(year, expected) in rows {
        match engine_first_day(year) {
            Some((date, _)) if ymd(date) == expected => {}
            Some((date, _)) => out.push(format!(
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

/// The leap month numbers of every Tibetan year the range holds completely,
/// from every day of 1900-01-01..=2100-12-31.
fn engine_leap_months() -> BTreeMap<i64, BTreeSet<u8>> {
    let mut leaps: BTreeMap<i64, BTreeSet<u8>> = BTreeMap::new();
    for day in all_days() {
        let year = day.z.year().number();
        leaps.entry(year).or_default();
        if day.z.is_leap_month() {
            leaps.entry(year).or_default().insert(day.z.month().get());
        }
    }
    leaps
}

/// Table 7 (Phugpa column) against the engine's leap months: a printed leap
/// month must be the engine's, and an empty cell must be a year without one.
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
fn engine_events_2012() -> BTreeSet<Event> {
    let mut months: BTreeMap<u8, BTreeMap<u8, u32>> = BTreeMap::new();
    for day in all_days() {
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

/// Table 8 (Phugpa column) against the engine's events, in both directions:
/// an event the table prints and the engine lacks, and an event the engine has
/// where the table prints none (so a month with no entry is checked too).
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

#[test]
fn p5_table1_first_day_and_name_of_every_year_match_the_engine() {
    let rows = table1();
    // the transcription measured 120 rows: 60 lines of two years in each panel
    assert_eq!(rows.len(), 120, "Table 1 rows loaded");
    assert_eq!((rows[0].year, rows[119].year), (1927, 2046));
    let mismatches = table1_mismatches(&rows);
    assert!(
        mismatches.is_empty(),
        "P5 Table 1: {} mismatches in {} rows; the first: {}",
        mismatches.len(),
        rows.len(),
        first_of(&mismatches)
    );
}

#[test]
fn p5_table9_first_day_of_every_year_matches_the_engine() {
    let rows = table9();
    assert_eq!(rows.len(), 31, "Table 9 rows loaded");
    assert_eq!((rows[0].0, rows[30].0), (2000, 2030));
    let mismatches = table9_mismatches(&rows);
    assert!(
        mismatches.is_empty(),
        "P5 Table 9: {} mismatches in {} rows; the first: {}",
        mismatches.len(),
        rows.len(),
        first_of(&mismatches)
    );
}

#[test]
fn p5_table7_leap_month_of_every_printed_year_matches_the_engine() {
    let rows = table7();
    assert_eq!(rows.len(), 21, "Table 7 rows loaded");
    assert_eq!(
        rows.iter().filter(|r| r.1.is_some()).count(),
        8,
        "years with a Phugpa leap month"
    );
    let mismatches = table7_mismatches(&rows, &engine_leap_months());
    assert!(
        mismatches.is_empty(),
        "P5 Table 7: {} mismatches in {} rows; the first: {}",
        mismatches.len(),
        rows.len(),
        first_of(&mismatches)
    );
}

#[test]
fn p5_table8_repeated_and_skipped_numbers_of_2012_match_the_engine() {
    let table = table8();
    assert_eq!(table.len(), 23, "Table 8 events loaded");
    assert_eq!(table.iter().filter(|e| e.2 == "repeated").count(), 9);
    assert_eq!(table.iter().filter(|e| e.2 == "skipped").count(), 14);
    let engine = engine_events_2012();
    // month 10 has no entry in the table: the engine must have none either
    assert!(table.iter().all(|e| e.0 != 10) && engine.iter().all(|e| e.0 != 10));
    let mismatches = table8_mismatches(&table, &engine);
    assert!(
        mismatches.is_empty(),
        "P5 Table 8: {} mismatches in {} events; the first: {}",
        mismatches.len(),
        table.len(),
        first_of(&mismatches)
    );
}

// ----------------------------------------------------- seeded controls (7a)
//
// One field deliberately wrong per fixture file (five with the T4 control in
// `p6_t4_fixture.rs`), fed through the same comparison the P5 test uses and
// asserted to be reported (002 §6; METHOD rule 31).

#[test]
fn seeded_control_table1_one_wrong_date_is_reported() {
    let mut rows = table1();
    let row = rows.iter_mut().find(|r| r.year == 1995).expect("1995");
    assert_eq!(row.date, (1995, 3, 2), "the committed row");
    row.date = (1995, 3, 3);
    let mismatches = table1_mismatches(&rows);
    assert_eq!(mismatches.len(), 1, "exactly the seeded row");
    assert!(
        mismatches[0].contains(
            "Table 1, Tibetan year 1995: first day expected 1995-03-03 (table), engine 1995-03-02"
        ),
        "the assertion message: {}",
        mismatches[0]
    );
}

#[test]
fn seeded_control_table9_one_wrong_date_is_reported() {
    let mut rows = table9();
    let row = rows.iter_mut().find(|r| r.0 == 2011).expect("2011");
    assert_eq!(row.1, (2011, 3, 5), "the committed row");
    row.1 = (2011, 3, 6);
    let mismatches = table9_mismatches(&rows);
    assert_eq!(mismatches.len(), 1, "exactly the seeded row");
    assert!(
        mismatches[0].contains(
            "Table 9, Tibetan year 2011: first day expected 2011-03-06 (table), engine 2011-03-05"
        ),
        "the assertion message: {}",
        mismatches[0]
    );
}

#[test]
fn seeded_control_table7_one_wrong_leap_month_is_reported() {
    let mut rows = table7();
    let row = rows.iter_mut().find(|r| r.0 == 2010).expect("2010");
    assert_eq!(row.1, Some(11), "the committed row");
    row.1 = Some(10);
    let mismatches = table7_mismatches(&rows, &engine_leap_months());
    assert_eq!(mismatches.len(), 1, "exactly the seeded row");
    assert!(
        mismatches[0]
            .contains("Table 7, year 2010: leap months expected {10} (table), engine {11}"),
        "the assertion message: {}",
        mismatches[0]
    );
}

#[test]
fn seeded_control_table8_one_wrong_tshes_is_reported() {
    let mut table = table8();
    let event = table
        .iter_mut()
        .find(|e| e.0 == 1 && e.2 == "repeated")
        .expect("month 1");
    assert_eq!(event.1, 5, "the committed row");
    event.1 = 6;
    let mismatches = table8_mismatches(&table, &engine_events_2012());
    assert_eq!(
        mismatches.len(),
        2,
        "the wrong event, and the one now missing"
    );
    assert!(
        mismatches.contains(
            &"Table 8, month 1: repeated 6 is in the table and not in the engine".to_owned()
        ) && mismatches.contains(
            &"Table 8, month 1: repeated 5 is in the engine and not in the table".to_owned()
        ),
        "the assertion messages: {mismatches:?}"
    );
}

// ------------------------------------- the paper against itself (Item 3d, 3c)
//
// Computed from the fixtures and Janson's own formulas; the engine is not
// called. A disagreement is spec 006 STOP RULE (3).

#[test]
fn t5_table9_phugpa_column_equals_table1_on_every_year_both_print() {
    let table1: BTreeMap<i64, Ymd> = table1().into_iter().map(|r| (r.year, r.date)).collect();
    let mut compared = 0;
    for (year, date) in table9() {
        let in_table1 = table1.get(&year).copied();
        assert_eq!(
            in_table1,
            Some(date),
            "Table 9 gives {} for {year}, Table 1 gives {in_table1:?}",
            iso(date)
        );
        compared += 1;
    }
    assert_eq!(
        compared, 31,
        "every year of Table 9 is also in Table 1 (1927-2046)"
    );
}

/// The events of Table 8 by month, as `(repeated count, skipped count)` per month 1..=12.
fn table8_counts() -> BTreeMap<u8, (i64, i64)> {
    let mut counts: BTreeMap<u8, (i64, i64)> = (1..=12).map(|m| (m, (0, 0))).collect();
    for (month, _, kind) in table8() {
        let entry = counts.get_mut(&month).expect("month 1..=12");
        if kind == "repeated" {
            entry.0 += 1;
        } else {
            entry.1 += 1;
        }
    }
    counts
}

#[test]
fn t5_table8_2012_length_equals_losar_2012_to_losar_2013_by_table1() {
    // Each of the 12 months has 30 lunar days; a skipped number is no calendar
    // day and a repeated number is two (Janson v1, p. 20), so the year has
    // 30 x 12 - skipped + repeated calendar days.
    let counts = table8_counts();
    let repeated: i64 = counts.values().map(|c| c.0).sum();
    let skipped: i64 = counts.values().map(|c| c.1).sum();
    assert_eq!((repeated, skipped), (9, 14));
    let length = 30 * 12 - skipped + repeated;
    assert_eq!(length, 355, "30 x 12 - 14 + 9");
    let losar: BTreeMap<i64, Ymd> = table1().into_iter().map(|r| (r.year, r.date)).collect();
    let days = day_number(losar[&2013]) - day_number(losar[&2012]);
    assert_eq!(
        length,
        days,
        "Table 8 gives 2012 {length} days, Table 1 gives {days} from {} to {}",
        iso(losar[&2012]),
        iso(losar[&2013])
    );
    // The two tables say 2012 has no Phugpa leap month (Janson v1, p. 51): so
    // the year has 12 months, and Table 7 must agree.
    let leap_2012 = table7()
        .into_iter()
        .find(|r| r.0 == 2012)
        .expect("2012 is in Table 7");
    assert_eq!(leap_2012.1, None, "Table 7 gives 2012 a leap month");
}

#[test]
fn t5_table8_each_month_is_29_or_30_days_and_no_number_is_both_repeated_and_skipped() {
    // Janson v1, p. 7: a calendar month has 29 or 30 days; p. 20: a number is
    // skipped, repeated or neither. This check sees the VALUES of the table's
    // entries, which the year-length check above (counts only) cannot.
    let counts = table8_counts();
    let lengths: Vec<i64> = counts.values().map(|c| 30 - c.1 + c.0).collect();
    assert!(
        lengths.iter().all(|&n| n == 29 || n == 30),
        "month lengths {lengths:?}"
    );
    assert_eq!(lengths.iter().sum::<i64>(), 355);
    let mut seen = BTreeSet::new();
    for (month, tshes, kind) in table8() {
        assert!((1..=30).contains(&tshes), "tshes {tshes} of month {month}");
        assert!(
            seen.insert((month, tshes)),
            "month {month}: the number {tshes} appears twice among the entries ({kind} and another)"
        );
    }
}

#[test]
fn t5_table7_leap_months_follow_eqs_5_40_to_5_42() {
    // Janson v1 §5.5, p. 19: Y is a leap year iff (24Y + 33) mod 65 >= 41
    // (5.41); then the leap month is M = floor(33 - ((24Y + 33) mod 65)/2) (5.42).
    let mut leap_years = 0;
    for (year, cell) in table7() {
        let r = (24 * year + 33).rem_euclid(65);
        let formula =
            (r >= 41).then(|| u8::try_from(33 - (r + 1).div_euclid(2)).expect("a month number"));
        assert_eq!(
            cell, formula,
            "Table 7 gives {cell:?} for {year}, eqs. (5.41), (5.42) give {formula:?}"
        );
        leap_years += i32::from(cell.is_some());
    }
    assert_eq!(leap_years, 8);
}

/// The leap months of the T4 fixture as `(tibetan_year, month, first Gregorian
/// year of the leap month)`.
fn t4_leap_months(rows: &[T4Row]) -> Vec<(i64, u8, i64)> {
    let mut out: Vec<(i64, u8, i64)> = Vec::new();
    for row in rows.iter().filter(|r| r.leap) {
        if !out.iter().any(|m| (m.0, m.1) == (row.year, row.month)) {
            out.push((row.year, row.month, row.date.0));
        }
    }
    out
}

#[test]
fn t5_table7_year_column_read_as_the_tibetan_year_or_the_start_year_gives_the_same_rows() {
    // Spec 006 Item 3c, STOP RULE (4): the text on p. 51 says "for a range of
    // (Gregorian) years". Reading A: the row's year is the Tibetan year Y.
    // Reading B: it is the Gregorian year in which the leap month begins. Both
    // are computed here from the T4 fixture (independent of the engine) and
    // compared with the table.
    let leaps = t4_leap_months(&t4_rows());
    for (year, cell) in table7() {
        let reading_a: Vec<u8> = leaps.iter().filter(|m| m.0 == year).map(|m| m.1).collect();
        let reading_b: Vec<u8> = leaps.iter().filter(|m| m.2 == year).map(|m| m.1).collect();
        let expected: Vec<u8> = cell.into_iter().collect();
        assert_eq!(reading_a, expected, "year {year}, read as the Tibetan year");
        assert_eq!(
            reading_b, expected,
            "year {year}, read as the Gregorian year the leap month begins"
        );
    }
}
