//! P6 (001 §6.4: "Phugpa reproduces T4 output exactly over at least
//! 1900–2100"), against the fixture `golden/reference/tdc-97d4a00-phugpa-1900-2100.csv`
//! generated from the `tibetan-date-calculator` library (spec 006 Items 5 and 6).
//!
//! The engine is reached through the public API only: `to_zdate` with
//! `System::Phugpa` named explicitly (`DEBT.md` D-1).
//!
//! **What this checks, and what it does not (METHOD rule 35).** The library
//! implements Janson's formulas from the same epoch year, 806 (its README, line
//! 5), as the engine does. P6 is therefore an agreement between two
//! implementations of one paper: it can catch a slip in either, and it cannot
//! catch a misreading of Janson that both share. It is not proof that either is
//! right. What P5 adds is the independent check: Janson's printed tables are
//! values neither implementation produced (`p5_reference_tables.rs`). The
//! library's dates are trusted only as generated under TZ=UTC (`FORKS.md` F14).
//!
//! The fixture has no doubled or skipped column (F9 is open): the engine's
//! `is_doubled` and `skipped_before` are compared with what consecutive fixture
//! rows imply. The first row has no predecessor in the fixture, so its two flags
//! are not implied and are not compared (73,413 of the 73,414 rows are).

mod common;

use common::all_days;
use common::fixtures::{JD_OF_1970_01_01, T4Row, Ymd, day_number, iso, t4_rows};
use tshes::{GregorianDate, System, to_zdate};

const FIRST: Ymd = (1900, 1, 1);
const LAST: Ymd = (2100, 12, 31);

#[test]
fn day_number_matches_the_published_anchors() {
    // Janson v1, p. 6: "1 January 2007 has JD 2454102"; the JD of 1970-01-01 is 2440588.
    assert_eq!(day_number((1970, 1, 1)), 0);
    assert_eq!(day_number((2007, 1, 1)) + JD_OF_1970_01_01, 2_454_102);
    assert_eq!(day_number((2000, 3, 1)) - day_number((2000, 2, 28)), 2);
    assert_eq!(day_number((1900, 3, 1)) - day_number((1900, 2, 28)), 1);
}

/// The key of a month in the fixture: `(year, month, is_leap_month)`.
fn month_of(row: &T4Row) -> (i64, u8, bool) {
    (row.year, row.month, row.leap)
}

/// What a row's predecessor implies about the row: `(is_doubled, skipped_before)`.
/// `is_doubled` is the second occurrence of a repeated number (001 §5.1);
/// `skipped_before` is the number omitted immediately before the row (the
/// number 30 of the previous month on a tshes 1, the number 1 on a tshes 2 that
/// follows a 30).
fn implied(prev: &T4Row, row: &T4Row) -> Result<(bool, Option<u8>), String> {
    if month_of(prev) == month_of(row) {
        return match i16::from(row.tshes) - i16::from(prev.tshes) {
            0 => Ok((true, None)),
            1 => Ok((false, None)),
            2 => Ok((false, Some(row.tshes - 1))),
            step => Err(format!(
                "{}: tshes goes from {} to {} inside a month (step {step})",
                iso(row.date),
                prev.tshes,
                row.tshes
            )),
        };
    }
    // a new month: the numbers missing between the end of one and the start of the next
    let missing = (30 - i16::from(prev.tshes)) + (i16::from(row.tshes) - 1);
    match missing {
        0 => Ok((false, None)),
        1 => Ok((false, Some(if row.tshes == 1 { 30 } else { row.tshes - 1 }))),
        n => Err(format!(
            "{}: {n} numbers missing between tshes {} and tshes {} across a month boundary",
            iso(row.date),
            prev.tshes,
            row.tshes
        )),
    }
}

/// The engine against every row of the fixture: one message per disagreement.
fn p6_mismatches(rows: &[T4Row]) -> Vec<String> {
    let mut out = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        let date = GregorianDate::new(row.date.0, row.date.1, row.date.2).expect("a valid date");
        let z = to_zdate(date, System::Phugpa).expect("in range");
        let actual = (
            z.year().number(),
            z.month().get(),
            z.is_leap_month(),
            z.tshes().get(),
        );
        let expected = (row.year, row.month, row.leap, row.tshes);
        if actual != expected {
            out.push(format!(
                "{}: fixture (year, month, leap, tshes) {expected:?}, engine {actual:?}",
                iso(row.date)
            ));
        }
        if i == 0 {
            continue;
        }
        match implied(&rows[i - 1], row) {
            Err(message) => out.push(message),
            Ok((doubled, skipped)) => {
                if z.is_doubled() != doubled {
                    out.push(format!(
                        "{}: fixture rows imply is_doubled {doubled}, engine {}",
                        iso(row.date),
                        z.is_doubled()
                    ));
                }
                let engine_skipped = z.skipped_before().map(tshes::Tshes::get);
                if engine_skipped != skipped {
                    out.push(format!(
                        "{}: fixture rows imply skipped_before {skipped:?}, engine {engine_skipped:?}",
                        iso(row.date)
                    ));
                }
            }
        }
    }
    out
}

fn first_of(mismatches: &[String]) -> &str {
    mismatches.first().map_or("none", String::as_str)
}

#[test]
fn p6_the_fixture_has_73414_contiguous_rows_covering_1900_to_2100() {
    let rows = t4_rows();
    assert_eq!(
        day_number(LAST) - day_number(FIRST) + 1,
        73_414,
        "the days of the range"
    );
    assert_eq!(rows.len(), 73_414, "rows loaded");
    assert_eq!(rows[0].date, FIRST);
    assert_eq!(rows[rows.len() - 1].date, LAST);
    for (i, row) in rows.iter().enumerate() {
        let offset = i64::try_from(i).expect("an index");
        assert_eq!(
            day_number(row.date),
            day_number(FIRST) + offset,
            "row {i} ({}) is not on day {offset} of the range",
            iso(row.date)
        );
    }
}

#[test]
fn p6_every_day_of_the_fixture_matches_the_engine_and_its_flags_follow_from_the_rows() {
    let rows = t4_rows();
    assert_eq!(rows.len(), 73_414, "rows loaded");
    let mismatches = p6_mismatches(&rows);
    assert!(
        mismatches.is_empty(),
        "P6: {} mismatches over {} days; the first: {}",
        mismatches.len(),
        rows.len(),
        first_of(&mismatches)
    );
}

/// Shape counts over a sequence of `(month key, tshes, skipped number before it
/// if any)`; the same counting for the fixture and for the engine.
#[derive(Debug, Default, PartialEq, Eq)]
struct Shapes {
    months: u64,
    leap_months: u64,
    repeated: u64,
    skipped: u64,
    rollovers: u64,
    cycle_boundaries: u64,
}

/// The rabjung cycle of Tibetan year `year`: `ceil((Y - 1026) / 60)`, Janson v1
/// eq. (4.3), p. 8.
fn cycle(year: i64) -> i64 {
    (year - 1026 + 59).div_euclid(60)
}

fn fixture_shapes(rows: &[T4Row]) -> Shapes {
    let mut s = Shapes::default();
    let mut leap_keys = std::collections::BTreeSet::new();
    for (i, row) in rows.iter().enumerate() {
        if i == 0 || month_of(&rows[i - 1]) != month_of(row) {
            s.months += 1;
        }
        if row.leap {
            leap_keys.insert(month_of(row));
        }
        if i == 0 {
            continue;
        }
        let prev = &rows[i - 1];
        let (doubled, skipped) = implied(prev, row).expect("a possible pattern");
        s.repeated += u64::from(doubled);
        s.skipped += u64::from(skipped.is_some());
        s.rollovers += u64::from(prev.month == 12 && row.month == 1);
        s.cycle_boundaries += u64::from(cycle(prev.year) != cycle(row.year));
    }
    s.leap_months = u64::try_from(leap_keys.len()).expect("a count");
    s
}

fn engine_shapes() -> Shapes {
    let days = all_days();
    let mut s = Shapes::default();
    let mut leap_keys = std::collections::BTreeSet::new();
    for (i, day) in days.iter().enumerate() {
        let key = common::month_key(&day.z);
        if i == 0 || common::month_key(&days[i - 1].z) != key {
            s.months += 1;
        }
        if day.z.is_leap_month() {
            leap_keys.insert(key);
        }
        s.repeated += u64::from(day.z.is_doubled());
        s.skipped += u64::from(day.z.skipped_before().is_some());
        if i > 0 {
            let prev = &days[i - 1].z;
            s.rollovers += u64::from(prev.month().get() == 12 && day.z.month().get() == 1);
            s.cycle_boundaries +=
                u64::from(prev.year().rabjung_cycle() != day.z.year().rabjung_cycle());
        }
    }
    s.leap_months = u64::try_from(leap_keys.len()).expect("a count");
    s
}

#[test]
fn p6_the_fixture_reaches_every_shape_and_its_counts_equal_the_engines() {
    // METHOD rule 39: a passing comparison proves only the shapes its data
    // contains. Each shape the properties depend on is asserted non-zero, and
    // the fixture's count is compared with the engine's census re-measured here.
    let fixture = fixture_shapes(&t4_rows());
    let engine = engine_shapes();
    println!(
        "P6 shapes, fixture: months {}, leap months {}, repeated tshes {}, skipped tshes {}, 12->1 rollovers {}, rabjung-cycle boundaries {}",
        fixture.months,
        fixture.leap_months,
        fixture.repeated,
        fixture.skipped,
        fixture.rollovers,
        fixture.cycle_boundaries
    );
    println!(
        "P6 shapes, engine:  months {}, leap months {}, repeated tshes {}, skipped tshes {}, 12->1 rollovers {}, rabjung-cycle boundaries {}",
        engine.months,
        engine.leap_months,
        engine.repeated,
        engine.skipped,
        engine.rollovers,
        engine.cycle_boundaries
    );
    for (name, count) in [
        ("leap months", fixture.leap_months),
        ("repeated tshes", fixture.repeated),
        ("skipped tshes", fixture.skipped),
        ("12->1 rollovers", fixture.rollovers),
        ("rabjung-cycle boundaries", fixture.cycle_boundaries),
    ] {
        assert!(count > 0, "the fixture reaches no {name}");
    }
    assert_eq!(
        fixture, engine,
        "the fixture's shape counts and the engine's census"
    );
}

// ------------------------------------------------------ seeded control (7a)

#[test]
fn seeded_control_t4_one_wrong_tshes_is_reported() {
    let mut rows = t4_rows();
    let index = rows
        .iter()
        .position(|r| r.date == (2012, 2, 22))
        .expect("2012-02-22");
    assert_eq!(
        rows[index].tshes, 1,
        "the committed row: Losar 2012 is tshes 1"
    );
    rows[index].tshes = 2;
    let mismatches = p6_mismatches(&rows);
    assert!(
        mismatches.contains(
            &"2012-02-22: fixture (year, month, leap, tshes) (2012, 1, false, 2), engine (2012, 1, false, 1)".to_owned()
        ),
        "the assertion message, among {} mismatches: {mismatches:?}",
        mismatches.len()
    );
}
