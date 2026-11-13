//! The Q1 differential (spec 014 Items 3 and 4; 001 §6.5): every row of the three
//! `golden/buryat/` files against what the engine's Phugpa, Tsurphu and Genden
//! compute for that Gregorian day, through the public API only.
//!
//! **Run with `--nocapture`** to print the report (`Q1 ...` lines). Every test is in
//! the ordinary gate (no `#[ignore]`).
//!
//! **What this measures, and what it does not (METHOD rules 25, 35).** The deciding
//! field is `tshes`: both sides see it. `is_doubled` and `skipped` are derived from
//! the tshes sequence (spec 013), so they are reported and never counted as further
//! mismatches; `month` and `is_leap_month` exist in the two Aginsky files only and are
//! compared per lunation. A count here is a measurement of three files; it is not a
//! verdict, and no test in this file says which system any datsan follows (001 §6.5,
//! METHOD P4: the gate measures, a human decides). The T4 fixture goes through the same
//! path only as a control of the harness (it is never evidence for Q1, 001 §6.1).
//!
//! Controls (METHOD rule 31): an external positive control (the T4 fixture), a
//! self-output control, a planted wrong cell and a planted one-day shift, each shown to
//! fail; the pin (`q1_pin_...`) is shown to fail on the planted cell.

mod common;

use common::buryat::{BuryatRow, FILES, SourceFile, add_days, load, lunation_starts};
use common::fixtures::{Ymd, iso};
use common::q1::{
    Comparison, SYSTEMS, compare, dates_of, engine_day, engine_rows_on, engine_starts, month_lines,
    shifted, t4_rows_on, with_tshes,
};
use tshes::System;

struct Loaded {
    file: SourceFile,
    rows: Vec<BuryatRow>,
}

fn loaded() -> Vec<Loaded> {
    FILES
        .iter()
        .map(|&file| Loaded {
            file,
            rows: load(file.stem),
        })
        .collect()
}

fn parse(date: &str) -> Ymd {
    common::buryat::parse_date(date).expect("a date in this file")
}

fn row_on(rows: &[BuryatRow], date: Ymd) -> Option<&BuryatRow> {
    rows.iter().find(|r| r.date == date)
}

/// Spec 009 §1.2's three tables (RECALLED: spec 009 section 1.2, the Phugpa, Tsurphu and
/// Genden tables, one row per disagreeing day of 1900-01-01..2100-12-31): `(system, date,
/// the engine's tshes in the table, the TCG tshes in the table)`. TCG uses Janson v1's
/// (7.24) for the Moon's anomaly increment, the engine (7.13) (`FORKS.md` F25, `DEBT.md`
/// D-23). Produced by a script that reads the tables of the spec file, not typed by hand.
const LISTS_009: [(System, &str, u8, u8); 53] = [
    (System::Phugpa, "1914-02-17", 22, 23),
    (System::Phugpa, "1922-08-14", 22, 21),
    (System::Phugpa, "1929-02-01", 23, 22),
    (System::Phugpa, "1946-05-30", 29, 30),
    (System::Phugpa, "1956-04-09", 29, 28),
    (System::Phugpa, "1967-09-25", 22, 21),
    (System::Phugpa, "1968-03-22", 23, 24),
    (System::Phugpa, "1968-09-12", 21, 20),
    (System::Phugpa, "1977-07-12", 27, 26),
    (System::Phugpa, "2001-02-10", 17, 18),
    (System::Phugpa, "2006-05-10", 13, 12),
    (System::Phugpa, "2025-11-19", 30, 29),
    (System::Phugpa, "2045-11-06", 27, 28),
    (System::Phugpa, "2046-07-18", 16, 15),
    (System::Phugpa, "2046-10-18", 19, 18),
    (System::Phugpa, "2064-07-19", 5, 6),
    (System::Phugpa, "2099-10-14", 30, 1),
    (System::Tsurphu, "1936-01-27", 3, 4),
    (System::Tsurphu, "1953-12-01", 26, 25),
    (System::Tsurphu, "1967-07-31", 25, 24),
    (System::Tsurphu, "1975-12-26", 23, 24),
    (System::Tsurphu, "1994-12-26", 23, 24),
    (System::Tsurphu, "2014-10-03", 9, 10),
    (System::Tsurphu, "2022-06-28", 30, 29),
    (System::Tsurphu, "2038-09-25", 27, 26),
    (System::Tsurphu, "2049-11-19", 25, 24),
    (System::Tsurphu, "2052-06-25", 28, 29),
    (System::Tsurphu, "2055-06-23", 29, 28),
    (System::Tsurphu, "2079-10-18", 23, 24),
    (System::Tsurphu, "2089-02-02", 23, 22),
    (System::Genden, "1903-11-01", 12, 11),
    (System::Genden, "1920-06-26", 11, 10),
    (System::Genden, "1926-04-08", 25, 26),
    (System::Genden, "1934-06-10", 29, 28),
    (System::Genden, "1934-12-25", 20, 19),
    (System::Genden, "1943-02-19", 15, 14),
    (System::Genden, "1958-01-14", 24, 25),
    (System::Genden, "1962-08-10", 10, 9),
    (System::Genden, "1971-03-25", 28, 29),
    (System::Genden, "1988-11-19", 10, 11),
    (System::Genden, "1990-04-04", 9, 10),
    (System::Genden, "2006-04-26", 28, 29),
    (System::Genden, "2013-07-21", 13, 14),
    (System::Genden, "2024-09-19", 16, 17),
    (System::Genden, "2033-03-17", 17, 16),
    (System::Genden, "2033-11-05", 13, 14),
    (System::Genden, "2043-12-28", 28, 27),
    (System::Genden, "2045-01-07", 19, 20),
    (System::Genden, "2049-09-09", 12, 13),
    (System::Genden, "2061-01-06", 15, 16),
    (System::Genden, "2073-02-25", 18, 19),
    (System::Genden, "2077-06-15", 24, 25),
    (System::Genden, "2087-04-20", 18, 17),
];

fn name_of(system: System) -> &'static str {
    SYSTEMS.iter().find(|s| s.0 == system).map_or("?", |s| s.1)
}

fn first_and_last(rows: &[BuryatRow]) -> (Ymd, Ymd) {
    (
        rows.first().expect("rows").date,
        rows.last().expect("rows").date,
    )
}

fn comparisons(rows: &[BuryatRow]) -> Vec<(System, Comparison)> {
    SYSTEMS
        .iter()
        .map(|&(s, _)| (s, compare(rows, s)))
        .collect()
}

/// The systems with the most tshes matches in a file (a tie is several).
fn best_of(cmps: &[(System, Comparison)]) -> Vec<System> {
    let most = cmps.iter().map(|c| c.1.matches).max().unwrap_or(0);
    cmps.iter()
        .filter(|c| c.1.matches == most)
        .map(|c| c.0)
        .collect()
}

fn diff_dates(c: &Comparison) -> Vec<String> {
    c.diffs.iter().map(|d| iso(d.date)).collect()
}

// ---------------------------------------------------------------- the controls

#[test]
fn q1_control_t4_phugpa_has_zero_tshes_differences_on_the_golden_dates_through_the_same_path() {
    // Item 3(b): the T4 fixture's tshes column on the dates of each golden file, through
    // the same `compare` as the Buryat rows. It tests the harness only (001 §6.1).
    let mut total_phugpa = 0;
    for l in loaded() {
        let rows = t4_rows_on(&dates_of(&l.rows));
        for (system, c) in comparisons(&rows) {
            println!(
                "Q1 CONTROL-T4 file={} system={} rows={} match={} mismatch={} doubled_disagree={} skipped_disagree={}",
                l.file.stem,
                name_of(system),
                c.rows,
                c.matches,
                c.diffs.len(),
                c.doubled_disagree.len(),
                c.skipped_disagree.len()
            );
            if system == System::Phugpa {
                total_phugpa += c.diffs.len();
                assert_eq!(c.rows, l.file.rows, "{}: rows", l.file.stem);
                assert!(
                    c.diffs.is_empty()
                        && c.doubled_disagree.is_empty()
                        && c.skipped_disagree.is_empty(),
                    "{}: the T4 fixture differs from the engine's Phugpa at {:?}",
                    l.file.stem,
                    diff_dates(&c)
                );
            }
        }
    }
    assert_eq!(total_phugpa, 0, "T4 against Phugpa, all three files");
}

#[test]
fn q1_control_the_engines_own_output_has_zero_differences_through_the_harness() {
    // Item 3(c): rows built from the engine's own output on the dates of every file, for
    // each system, through `compare`: 0 differences in tshes and in both flags.
    for l in loaded() {
        for &(system, name) in &SYSTEMS {
            let rows = engine_rows_on(&dates_of(&l.rows), system);
            let c = compare(&rows, system);
            println!(
                "Q1 CONTROL-SELF file={} system={name} rows={} match={} mismatch={} doubled_disagree={} skipped_disagree={}",
                l.file.stem,
                c.rows,
                c.matches,
                c.diffs.len(),
                c.doubled_disagree.len(),
                c.skipped_disagree.len()
            );
            assert!(
                c.diffs.is_empty()
                    && c.doubled_disagree.is_empty()
                    && c.skipped_disagree.is_empty(),
                "{} {name}: the engine's own output differs from itself",
                l.file.stem
            );
        }
    }
}

/// A day of `rows` on which every system matches, and a tshes that is not the printed one.
fn a_day_all_systems_match(rows: &[BuryatRow]) -> (Ymd, u8) {
    let row = rows
        .iter()
        .skip(100)
        .find(|r| {
            SYSTEMS
                .iter()
                .all(|&(s, _)| engine_day(r.date, s).tshes == r.tshes)
        })
        .expect("a day on which all three systems match");
    (row.date, (row.tshes + 6) % 30 + 1)
}

#[test]
fn q1_control_a_planted_wrong_cell_is_reported_on_exactly_that_day_for_every_system() {
    // Item 3(d)(i): one tshes changed on one day of a copy of the rows (in memory; the
    // committed file is not touched): exactly that day, for every system that matched it
    // before, is added to the list.
    for l in loaded() {
        let (date, wrong) = a_day_all_systems_match(&l.rows);
        let planted = with_tshes(&l.rows, date, wrong);
        let real_tshes = row_on(&l.rows, date).expect("row").tshes;
        assert_ne!(wrong, real_tshes, "the plant changes the cell");
        for ((system, before), (_, after)) in
            comparisons(&l.rows).into_iter().zip(comparisons(&planted))
        {
            let new: Vec<String> = diff_dates(&after)
                .into_iter()
                .filter(|d| !diff_dates(&before).contains(d))
                .collect();
            println!(
                "Q1 CONTROL-PLANT-CELL file={} system={} day={} tshes {real_tshes} -> {wrong}: mismatches {} -> {}, newly reported {new:?}",
                l.file.stem,
                name_of(system),
                iso(date),
                before.diffs.len(),
                after.diffs.len()
            );
            assert_eq!(
                new,
                vec![iso(date)],
                "{}: exactly the planted day",
                l.file.stem
            );
            assert_eq!(after.diffs.len(), before.diffs.len() + 1);
            assert_eq!(after.matches + 1, before.matches);
        }
    }
}

#[test]
fn q1_control_a_planted_one_day_shift_is_reported_on_most_days_for_every_system() {
    // Item 3(d)(ii): every golden date moved by one day (in memory): most days disagree.
    for l in loaded() {
        let planted = shifted(&l.rows, 1);
        for (system, c) in comparisons(&planted) {
            println!(
                "Q1 CONTROL-PLANT-SHIFT file={} system={} shift=+1 rows={} match={} mismatch={}",
                l.file.stem,
                name_of(system),
                c.rows,
                c.matches,
                c.diffs.len()
            );
            assert!(
                c.diffs.len() * 10 > c.rows * 8,
                "{} {}: the shifted rows disagree on {} of {} days",
                l.file.stem,
                name_of(system),
                c.diffs.len(),
                c.rows
            );
        }
    }
}

#[test]
fn q1_the_engine_still_computes_the_engine_values_spec_009_recorded_on_its_53_days() {
    // A value-level check that the engine under test is the engine spec 009 compared with
    // TCG (the precondition's `git diff` is the source-level one): the 53 days, the
    // engine's tshes as the spec table records it, and TCG's differing from it.
    assert_eq!(LISTS_009.len(), 53);
    let counts: Vec<usize> = SYSTEMS
        .iter()
        .map(|&(s, _)| LISTS_009.iter().filter(|r| r.0 == s).count())
        .collect();
    assert_eq!(counts, [17, 13, 23], "Phugpa, Tsurphu, Genden");
    for &(system, date, engine, tcg) in &LISTS_009 {
        assert_eq!(
            engine_day(parse(date), system).tshes,
            engine,
            "{} {date}",
            name_of(system)
        );
        assert_ne!(
            engine,
            tcg,
            "{} {date}: the (7.24) value differs from the engine's",
            name_of(system)
        );
    }
}

// ---------------------------------------------------------------------- the pin

/// MEASURED by spec 014 (debug and release, `--nocapture`; produced from the report's own
/// lines by a script, not typed by hand): `(file stem, system, rows, tshes matches, the
/// mismatching dates in date order)`.
const PIN: [(&str, &str, usize, usize, &str); 9] = [
    (
        "ivolginsky-datsan-2021",
        "Phugpa",
        398,
        374,
        "2021-01-01 2021-01-02 2021-01-03 2021-01-14 2021-01-15 2021-01-16 2021-01-17 2021-02-19 2021-03-03 2021-04-05 2021-04-12 2021-05-17 2021-05-21 2021-06-01 2021-06-04 2021-07-11 2021-07-15 2021-07-30 2021-09-05 2021-09-06 2021-09-07 2021-09-18 2021-10-29 2021-11-03",
    ),
    (
        "ivolginsky-datsan-2021",
        "Tsurphu",
        398,
        391,
        "2021-01-03 2021-03-03 2021-06-02 2021-06-03 2021-09-23 2021-10-28 2021-12-19",
    ),
    ("ivolginsky-datsan-2021", "Genden", 398, 398, ""),
    (
        "aginsky-datsan-2025",
        "Phugpa",
        413,
        398,
        "2025-01-20 2025-02-04 2025-02-22 2025-02-28 2025-04-15 2025-04-26 2025-09-15 2025-11-10 2025-11-19 2025-12-04 2025-12-24 2025-12-25 2025-12-26 2026-01-31 2026-02-11",
    ),
    (
        "aginsky-datsan-2025",
        "Tsurphu",
        413,
        403,
        "2025-08-02 2025-09-26 2025-12-04 2025-12-22 2025-12-23 2025-12-24 2025-12-25 2025-12-26 2025-12-27 2026-01-31",
    ),
    (
        "aginsky-datsan-2025",
        "Genden",
        413,
        407,
        "2025-12-22 2025-12-23 2025-12-24 2025-12-25 2025-12-26 2025-12-27",
    ),
    (
        "aginsky-datsan-2026",
        "Phugpa",
        402,
        373,
        "2026-01-09 2026-01-10 2026-01-11 2026-01-12 2026-01-13 2026-01-14 2026-01-15 2026-01-16 2026-01-31 2026-02-11 2026-02-24 2026-03-17 2026-03-18 2026-03-19 2026-03-30 2026-03-31 2026-04-02 2026-04-03 2026-04-22 2026-05-08 2026-08-26 2026-12-11 2026-12-24 2027-01-14 2027-01-15 2027-01-16 2027-01-28 2027-01-29 2027-01-30",
    ),
    (
        "aginsky-datsan-2026",
        "Tsurphu",
        402,
        390,
        "2026-01-09 2026-01-10 2026-01-11 2026-01-12 2026-01-13 2026-01-14 2026-01-15 2026-01-16 2026-01-31 2026-07-01 2026-09-02 2026-12-24",
    ),
    (
        "aginsky-datsan-2026",
        "Genden",
        402,
        394,
        "2026-01-09 2026-01-10 2026-01-11 2026-01-12 2026-01-13 2026-01-14 2026-01-15 2026-01-16",
    ),
];

/// Where the rows differ from the pin: one message per file and system.
fn pin_violations(files: &[(&str, Vec<BuryatRow>)]) -> Vec<String> {
    let mut out = Vec::new();
    for &(stem, system_name, rows, matches, dates) in &PIN {
        let Some((_, file_rows)) = files.iter().find(|f| f.0 == stem) else {
            out.push(format!("{stem}: no rows"));
            continue;
        };
        let system = SYSTEMS
            .iter()
            .find(|s| s.1 == system_name)
            .expect("a system")
            .0;
        let c = compare(file_rows, system);
        let found = diff_dates(&c).join(" ");
        if c.rows != rows || c.matches != matches || found != dates {
            out.push(format!(
                "{stem} {system_name}: rows {} matches {} mismatching [{found}]; pinned rows {rows} matches {matches} [{dates}]",
                c.rows, c.matches
            ));
        }
    }
    out
}

/// PINS A MEASUREMENT, NOT A CORRECTNESS CLAIM. It records, per golden file and per system, how many
/// rows' tshes equal the engine's and on which dates they differ, as spec 014 measured them
/// (`DEBT.md` D-1: no system is a default; no verdict is made here). It does not say the engine or
/// any source is right, nor that a mismatch is acceptable: a changed engine, a changed golden
/// file or a changed harness turns this red so that the change is seen. What resolves the
/// mismatches it lists is the verdict round's, put to the human as `FORKS.md` F38 (spec 014 Item
/// 6, OPEN); until then this test is the record (METHOD rules 33, 44; `DEBT.md` D-23, which
/// names the days on which (7.24) would give other dates).
#[test]
fn q1_pin_a_measurement_not_a_correctness_claim_tshes_matches_and_mismatching_dates_of_spec_014() {
    let files: Vec<(&str, Vec<BuryatRow>)> = loaded()
        .into_iter()
        .map(|l| (l.file.stem, l.rows))
        .collect();
    let violations = pin_violations(&files);
    assert!(
        violations.is_empty(),
        "the pinned measurement changed:\n{}",
        violations.join("\n")
    );
    // The totals that must sum: the pinned dates are the mismatches, per row count.
    for &(stem, system, rows, matches, dates) in &PIN {
        let n = if dates.is_empty() {
            0
        } else {
            dates.split(' ').count()
        };
        assert_eq!(
            matches + n,
            rows,
            "{stem} {system}: matches + mismatches = rows"
        );
    }
}

#[test]
fn q1_pin_fails_on_the_planted_wrong_cell_and_names_the_day_for_every_system() {
    // Item 3(e): the pin is shown to fail on plant (d)(i) of Item 3.
    let files = loaded();
    let l = &files[1];
    let (date, wrong) = a_day_all_systems_match(&l.rows);
    let planted: Vec<(&str, Vec<BuryatRow>)> = files
        .iter()
        .map(|f| {
            (
                f.file.stem,
                if f.file.stem == l.file.stem {
                    with_tshes(&f.rows, date, wrong)
                } else {
                    f.rows.clone()
                },
            )
        })
        .collect();
    let violations = pin_violations(&planted);
    println!(
        "Q1 PIN-PLANT day={} violations={}",
        iso(date),
        violations.len()
    );
    assert_eq!(
        violations.len(),
        3,
        "one violation per system for the planted file"
    );
    assert!(
        violations
            .iter()
            .all(|v| v.starts_with(l.file.stem) && v.contains(&iso(date)))
    );
}

// ------------------------------------------------------------------ the report

fn print_table(l: &Loaded, cmps: &[(System, Comparison)]) {
    let (first, last) = first_and_last(&l.rows);
    for (system, c) in cmps {
        println!(
            "Q1 TABLE file={} tier={} system={} rows={} match={} mismatch={} days={} first={} last={} flag_rows={} doubled_disagree={} skipped_disagree={}",
            l.file.stem,
            l.file.tier,
            name_of(*system),
            c.rows,
            c.matches,
            c.diffs.len(),
            c.rows,
            iso(first),
            iso(last),
            c.flag_rows,
            c.doubled_disagree.len(),
            c.skipped_disagree.len()
        );
        assert_eq!(c.matches + c.diffs.len(), c.rows, "match + mismatch = rows");
    }
}

fn lunation_of(rows: &[BuryatRow], date: Ymd) -> (usize, Ymd, Ymd) {
    let starts = lunation_starts(rows);
    let index = rows
        .iter()
        .position(|r| r.date == date)
        .expect("a row of the file");
    let k = starts
        .iter()
        .rposition(|&s| s <= index)
        .expect("a lunation");
    let end = starts.get(k + 1).copied().unwrap_or(rows.len());
    (k + 1, rows[starts[k]].date, rows[end - 1].date)
}

fn print_mismatches(l: &Loaded, cmps: &[(System, Comparison)]) {
    for (system, c) in cmps {
        if c.diffs.is_empty() {
            println!(
                "Q1 MISMATCHES file={} system={}: none (0 of {} rows)",
                l.file.stem,
                name_of(*system),
                c.rows
            );
        }
        for d in &c.diffs {
            let (k, from, to) = lunation_of(&l.rows, d.date);
            let label = row_on(&l.rows, from).map_or("", |r| {
                common::buryat::printed_label(&r.notes).unwrap_or("")
            });
            println!(
                "Q1 MISMATCH file={} tier={} system={} date={} golden={} engine={} lunation={k} ({}..{}) label=\"{label}\"",
                l.file.stem,
                l.file.tier,
                name_of(*system),
                iso(d.date),
                d.golden,
                d.engine,
                iso(from),
                iso(to)
            );
        }
    }
}

fn distinct_days(files: &[Loaded]) -> Vec<Ymd> {
    let mut dates: Vec<Ymd> = files.iter().flat_map(|l| dates_of(&l.rows)).collect();
    dates.sort_by_key(|&d| common::fixtures::day_number(d));
    dates.dedup();
    dates
}

/// Per system: `(name, rows, tshes matches, distinct days on which every source value equals
/// the engine's, distinct days on which none does, distinct days on which the editions split)`.
type Totals = (&'static str, usize, usize, usize, usize, usize);

/// The totals over every row of `files` and over their distinct days, per system (the one path
/// `print_totals` and the verdict's assertion both read).
fn totals_of(files: &[Loaded]) -> Vec<Totals> {
    let days = distinct_days(files);
    SYSTEMS
        .iter()
        .map(|&(system, name)| {
            let (mut rows, mut matched) = (0, 0);
            for l in files {
                let c = compare(&l.rows, system);
                rows += c.rows;
                matched += c.matches;
            }
            let (mut all_match, mut none_match, mut split) = (0, 0, 0);
            for &d in &days {
                let engine = engine_day(d, system).tshes;
                let values: Vec<u8> = files
                    .iter()
                    .filter_map(|l| row_on(&l.rows, d))
                    .map(|r| r.tshes)
                    .collect();
                let hits: usize = values.iter().map(|&v| usize::from(v == engine)).sum();
                if hits == values.len() {
                    all_match += 1;
                } else if hits == 0 {
                    none_match += 1;
                } else {
                    split += 1;
                }
            }
            (name, rows, matched, all_match, none_match, split)
        })
        .collect()
}

fn print_totals(files: &[Loaded]) {
    let days = distinct_days(files).len();
    for (name, rows, matched, all_match, none_match, split) in totals_of(files) {
        println!(
            "Q1 TOTAL system={name} rows={rows} match={matched} mismatch={}",
            rows - matched
        );
        println!(
            "Q1 DISTINCT system={name} days={days} all_sources_match={all_match} no_source_matches={none_match} editions_split={split}"
        );
        assert_eq!(all_match + none_match + split, days);
    }
}

fn print_best(files: &[Loaded]) -> Vec<(SourceFile, Vec<System>)> {
    let mut out = Vec::new();
    for l in files {
        let cmps = comparisons(&l.rows);
        let best = best_of(&cmps);
        let names: Vec<&str> = best.iter().map(|&s| name_of(s)).collect();
        let counts: Vec<String> = cmps
            .iter()
            .map(|(s, c)| format!("{}={}", name_of(*s), c.matches))
            .collect();
        println!(
            "Q1 BEST file={} best={} tie={} matches: {}",
            l.file.stem,
            names.join(","),
            best.len() > 1,
            counts.join(" ")
        );
        out.push((l.file, best));
    }
    let tsurphu = out.iter().any(|(_, b)| b.contains(&System::Tsurphu));
    println!("Q1 D19-TRIGGER-II tsurphu_among_the_best_for_any_file={tsurphu}");
    out
}

#[test]
fn q1_report_the_counts_per_file_and_system_the_mismatches_and_the_totals() {
    let files = loaded();
    println!(
        "Q1 SETTINGS systems=Phugpa,Tsurphu,Genden files={} golden=golden/buryat/ deciding_field=tshes api=tshes::to_zdate(date, system)",
        files.len()
    );
    for l in &files {
        let cmps = comparisons(&l.rows);
        print_table(l, &cmps);
        print_mismatches(l, &cmps);
    }
    print_totals(&files);
    let best = print_best(&files);
    assert_eq!(best.len(), 3);
}

#[test]
fn q1_dump_the_engines_values_on_every_distinct_golden_date_for_an_outside_recount() {
    // With `--nocapture`: one `Q1DUMP` line per distinct golden date and system, for a script
    // that recounts the matches from the CSV files and these lines alone (a check of the
    // harness's arithmetic, not of the engine: both use the same engine values).
    let files = loaded();
    let days = distinct_days(&files);
    assert_eq!(days.len(), 1165, "398 + 413 + 402 - 48 distinct days");
    for &d in &days {
        for &(system, name) in &SYSTEMS {
            let e = engine_day(d, system);
            println!(
                "Q1DUMP {} {name} {} {} {:?} {} {} {}",
                iso(d),
                e.tshes,
                e.doubled,
                e.skipped,
                e.year,
                e.month,
                e.leap
            );
        }
    }
}

// ------------------------------------------- months and the extra month (Item 4(c))

/// Consecutive runs of `true` in `flags` as `(first index, last index)`.
fn runs_of(flags: &[bool]) -> Vec<(usize, usize)> {
    let mut runs: Vec<(usize, usize)> = Vec::new();
    for (i, &on) in flags.iter().enumerate() {
        match runs.last_mut() {
            Some(last) if on && last.1 + 1 == i => last.1 = i,
            _ if on => runs.push((i, i)),
            _ => {}
        }
    }
    runs
}

fn print_months(l: &Loaded, system: System) {
    let name = name_of(system);
    let stem = l.file.stem;
    let source_starts = lunation_starts(&l.rows);
    let engine_starts = engine_starts(&l.rows, system);
    let start_dates =
        |starts: &[usize]| -> Vec<Ymd> { starts.iter().map(|&i| l.rows[i].date).collect() };
    let (src, eng) = (start_dates(&source_starts), start_dates(&engine_starts));
    let only_source: Vec<String> = src
        .iter()
        .filter(|d| !eng.contains(d))
        .map(|&d| iso(d))
        .collect();
    let only_engine: Vec<String> = eng
        .iter()
        .filter(|d| !src.contains(d))
        .map(|&d| iso(d))
        .collect();
    println!(
        "Q1 BOUNDARIES file={stem} system={name} source_lunations={} engine_months={} equal={} only_source={only_source:?} only_engine={only_engine:?}",
        src.len(),
        eng.len(),
        only_source.is_empty() && only_engine.is_empty()
    );
    let lines = month_lines(&l.rows, system);
    for m in &lines {
        println!(
            "Q1 LUNATION file={stem} system={name} k={} start={} days={} source_month={} source_leap={} label=\"{}\" engine_year={} engine_month={} engine_leap={} offset={}",
            m.k,
            iso(m.start),
            m.days,
            m.source_month
                .map_or_else(|| "-".to_owned(), |v| v.to_string()),
            m.source_leap,
            m.label,
            m.engine.year,
            m.engine.month,
            m.engine.leap,
            m.offset.map_or_else(|| "-".to_owned(), |v| v.to_string())
        );
    }
    let mut counts = [0usize; 12];
    for m in &lines {
        if let Some(o) = m.offset {
            counts[usize::from(o)] += 1;
        }
    }
    let regular = lines.iter().filter(|m| m.offset.is_some()).count();
    let tally: Vec<String> = counts
        .iter()
        .enumerate()
        .filter(|(_, n)| **n > 0)
        .map(|(o, n)| format!("{o}:{n}"))
        .collect();
    assert_eq!(
        counts.iter().sum::<usize>(),
        regular,
        "the offsets sum to the regular lunations"
    );
    // The engine's leap-month days against the source's extra-month days.
    let engine_leap: Vec<bool> = l
        .rows
        .iter()
        .map(|r| engine_day(r.date, system).leap)
        .collect();
    let source_extra: Vec<bool> = l.rows.iter().map(|r| r.leap == Some(true)).collect();
    let span = |runs: Vec<(usize, usize)>| -> Vec<String> {
        runs.iter()
            .map(|&(a, b)| {
                format!(
                    "{}..{} ({} days)",
                    iso(l.rows[a].date),
                    iso(l.rows[b].date),
                    b - a + 1
                )
            })
            .collect()
    };
    println!(
        "Q1 OFFSETS file={stem} system={name} regular_lunations={regular} offsets(engine-source mod 12)={{{}}} engine_leap_days={:?} source_extra_days={:?}",
        tally.join(" "),
        span(runs_of(&engine_leap)),
        span(runs_of(&source_extra))
    );
}

#[test]
fn q1_report_the_lunation_boundaries_the_months_and_the_extra_month_of_the_aginsky_files() {
    for l in loaded() {
        // The Ivolginsky file prints no label (month empty); its lunation boundaries are
        // reported too, since they follow from its tshes alone.
        for &(system, _) in &SYSTEMS {
            if l.file.labelled {
                print_months(&l, system);
            } else {
                let (s, e) = (lunation_starts(&l.rows), engine_starts(&l.rows, system));
                println!(
                    "Q1 BOUNDARIES file={} system={} source_lunations={} engine_months={} (no label printed: month and leap are not compared)",
                    l.file.stem,
                    name_of(system),
                    s.len(),
                    e.len()
                );
            }
        }
    }
}

// ----------------------------------------------- the Sagaalgan days (Item 4(d))

/// The Sagaalgan days the sources print (spec 013, RECALLED; each is named in a meta file's
/// notes, which `q1_the_sagaalgan_days_are_named_in_the_metas` checks).
const SAGAALGAN: [(&str, &str); 5] = [
    ("2021-02-12", "Ivolginsky hural schedule"),
    ("2022-02-02", "Ivolginsky hural schedule"),
    ("2025-03-01", "Aginsky 2025 edition"),
    ("2026-02-18", "Aginsky 2026 edition"),
    (
        "2027-02-07",
        "Aginsky 2026 edition (the day after its last row)",
    ),
];

#[test]
fn q1_the_sagaalgan_days_are_named_in_the_metas() {
    let ivolginsky = common::buryat::read("ivolginsky-datsan-2021.meta.yaml");
    assert!(ivolginsky.contains("12 February 2021") && ivolginsky.contains("2 February 2022"));
    assert!(
        common::buryat::read("aginsky-datsan-2025.meta.yaml").contains("«1 марта – Сагаалган»")
    );
    let a2026 = common::buryat::read("aginsky-datsan-2026.meta.yaml");
    assert!(
        a2026.contains("«18 февраля – Сагаалган»")
            && a2026.contains("«7 февраля 2027 г. – Сагаалган»")
    );
}

/// `(Сагаалган day, system, the engine's year, month, is_leap_month and tshes on that day,
/// whether the day is the first day of the engine's month 1 and that month is not a leap
/// month)`, for the five days of `SAGAALGAN` and the three systems. MEASURED by spec 014
/// (session 1's §2(d); session 2 asserted it, ADDENDUM 1 V2, and re-measured it outside this
/// harness from spec 009's dump probe). PINS A MEASUREMENT, NOT A CORRECTNESS CLAIM.
type SagaalganCell = (&'static str, &'static str, i64, u8, bool, u8, bool);

const SAGAALGAN_CELLS: [SagaalganCell; 15] = [
    ("2021-02-12", "Phugpa", 2021, 1, false, 1, true),
    ("2021-02-12", "Tsurphu", 2021, 1, false, 1, true),
    ("2021-02-12", "Genden", 2021, 1, false, 1, true),
    ("2022-02-02", "Phugpa", 2021, 12, false, 1, false),
    ("2022-02-02", "Tsurphu", 2022, 1, false, 1, true),
    ("2022-02-02", "Genden", 2022, 1, false, 1, true),
    ("2025-03-01", "Phugpa", 2025, 1, false, 2, false),
    ("2025-03-01", "Tsurphu", 2025, 1, false, 2, true),
    ("2025-03-01", "Genden", 2025, 1, false, 2, true),
    ("2026-02-18", "Phugpa", 2026, 1, false, 1, true),
    ("2026-02-18", "Tsurphu", 2026, 1, false, 1, true),
    ("2026-02-18", "Genden", 2026, 1, false, 1, true),
    ("2027-02-07", "Phugpa", 2027, 1, false, 1, true),
    ("2027-02-07", "Tsurphu", 2027, 1, false, 1, true),
    ("2027-02-07", "Genden", 2027, 1, false, 1, true),
];

/// The engine's cell of `SAGAALGAN_CELLS` on `date` under `system`.
fn sagaalgan_cell(date: Ymd, system: System) -> (i64, u8, bool, u8, bool) {
    let z = engine_day(date, system);
    let first_of_month = engine_day(add_days(date, -1), system).month_key() != z.month_key();
    (
        z.year,
        z.month,
        z.leap,
        z.tshes,
        first_of_month && z.month == 1 && !z.leap,
    )
}

/// Every cell of `SAGAALGAN_CELLS` that the engine does not give on the day checked: the
/// `i`-th day of `SAGAALGAN` is checked on `days[i]` (a planted control moves one).
fn sagaalgan_violations(days: &[Ymd; 5]) -> Vec<String> {
    let mut out = Vec::new();
    for (i, (date, _)) in SAGAALGAN.iter().enumerate() {
        for &(system, name) in &SYSTEMS {
            let pinned = SAGAALGAN_CELLS
                .iter()
                .find(|c| c.0 == *date && c.1 == name)
                .map(|c| (c.2, c.3, c.4, c.5, c.6))
                .expect("a pinned cell");
            let measured = sagaalgan_cell(days[i], system);
            if measured != pinned {
                out.push(format!(
                    "SAGAALGAN: {date} {name}, checked on {}: measured {measured:?}, pinned {pinned:?}",
                    iso(days[i])
                ));
            }
        }
    }
    out
}

#[test]
fn q1_report_the_sagaalgan_days_in_each_system() {
    // Asserted since spec 014 session 2 (ADDENDUM 1, V2): all fifteen cells and the "first
    // day of month 1" verdicts of session 1's §2(d), then a plant that moves one day.
    let days = SAGAALGAN.map(|(d, _)| parse(d));
    let violations = sagaalgan_violations(&days);
    assert!(
        violations.is_empty(),
        "the Сагаалган table changed:\n{}",
        violations.join("\n")
    );
    let mut planted = days;
    planted[2] = add_days(days[2], 1);
    let caught = sagaalgan_violations(&planted);
    println!("Q1 SAGAALGAN-PLANT violations={caught:?}");
    assert_eq!(
        caught.len(),
        3,
        "the moved day is reported for every system"
    );
    assert!(
        caught
            .iter()
            .all(|v| v.starts_with("SAGAALGAN: 2025-03-01"))
    );
    for (date, source) in SAGAALGAN {
        let when = parse(date);
        for &(system, name) in &SYSTEMS {
            let z = engine_day(when, system);
            let prev = engine_day(add_days(when, -1), system);
            let first_of_month = prev.month_key() != z.month_key();
            println!(
                "Q1 SAGAALGAN date={date} source=\"{source}\" system={name} year={} month={} leap={} tshes={} doubled={} skipped={:?} first_day_of_its_month={first_of_month} first_day_of_month_1={}",
                z.year,
                z.month,
                z.leap,
                z.tshes,
                z.doubled,
                z.skipped,
                first_of_month && z.month == 1 && !z.leap
            );
        }
    }
}

// ----------------------------------- D-23, D-19 and the two editions (Items 4(e) to (g))

/// The source rows of `date` in every file that holds it: `(file stem, tshes)`.
fn source_values(files: &[Loaded], date: Ymd) -> Vec<(&'static str, u8)> {
    files
        .iter()
        .filter_map(|l| row_on(&l.rows, date).map(|r| (l.file.stem, r.tshes)))
        .collect()
}

#[test]
fn q1_report_d23_the_days_of_spec_009_inside_the_golden_files_and_whether_it_fires() {
    let files = loaded();
    let best = print_best(&files);
    let mut d23_fires = false;
    for &(system, date, engine, tcg) in &LISTS_009 {
        let when = parse(date);
        let inside = source_values(&files, when);
        if inside.is_empty() {
            continue;
        }
        for (stem, value) in inside {
            let now = engine_day(when, system).tshes;
            let best_for_file = best
                .iter()
                .find(|b| b.0.stem == stem)
                .is_some_and(|b| b.1.contains(&system));
            let matches_724 = value == tcg;
            let fire = best_for_file && matches_724;
            d23_fires |= fire;
            println!(
                "Q1 D23-DAY file={stem} system={} date={date} source_tshes={value} engine_now={now} spec_009_engine={engine} tcg_7_24={tcg} source_matches_engine={} source_matches_7_24={matches_724} system_is_best_for_file={best_for_file} fires={fire}",
                name_of(system),
                value == now
            );
        }
    }
    println!("Q1 D23 fires={d23_fires}");
    let inside_count = LISTS_009
        .iter()
        .filter(|r| !source_values(&files, parse(r.1)).is_empty())
        .count();
    println!(
        "Q1 D23 list_days_inside_the_golden_dates={inside_count} of {}",
        LISTS_009.len()
    );
}

#[test]
fn q1_report_the_two_aginsky_editions_on_the_days_they_disagree() {
    let files = loaded();
    let (e25, e26) = (&files[1], &files[2]);
    let mut common_days = 0;
    let mut differing = Vec::new();
    for r in &e26.rows {
        if let Some(o) = row_on(&e25.rows, r.date) {
            common_days += 1;
            if o.tshes != r.tshes {
                differing.push((r.date, o.tshes, r.tshes));
            }
        }
    }
    println!(
        "Q1 EDITIONS common_days={common_days} equal={} different={}",
        common_days - differing.len(),
        differing.len()
    );
    for (date, v25, v26) in &differing {
        let vals: Vec<String> = SYSTEMS
            .iter()
            .map(|&(s, n)| format!("{n}={}", engine_day(*date, s).tshes))
            .collect();
        println!(
            "Q1 EDITIONS-DAY date={} edition_2025={v25} edition_2026={v26} {}",
            iso(*date),
            vals.join(" ")
        );
    }
    assert_eq!(common_days, 48, "2026-01-01..2026-02-17");
    assert_eq!(differing.len(), 8, "spec 013: 2026-01-09..2026-01-16");
}

// -------------------------------- the investigation of the best system (Item 5(b) to (e))

/// Mismatching days a blind re-read disagreed with the golden row on (Item 5(a)); filled in
/// by the reading. Empty: the reader agreed with every golden row it read.
const TRANSCRIPTION_DISAGREED: [&str; 0] = [];

#[derive(Debug, Clone, PartialEq, Eq)]
enum Class {
    Transcription,
    SourceDisagreement,
    VariantCandidate,
    Unexplained,
}

struct Mismatch {
    file: &'static str,
    system: System,
    diff: common::q1::TshesDiff,
    other_edition: Option<u8>,
    in_list_of_system: Option<(u8, u8)>,
    equals_other_systems: Vec<&'static str>,
    class: Class,
}

fn investigate(files: &[Loaded], best: &[(SourceFile, Vec<System>)]) -> Vec<Mismatch> {
    let mut out = Vec::new();
    for l in files {
        let systems = &best
            .iter()
            .find(|b| b.0.stem == l.file.stem)
            .expect("a best entry")
            .1;
        for &system in systems {
            for d in compare(&l.rows, system).diffs {
                let other_edition = if l.file.labelled {
                    files
                        .iter()
                        .filter(|o| o.file.labelled && o.file.stem != l.file.stem)
                        .find_map(|o| row_on(&o.rows, d.date))
                        .map(|r| r.tshes)
                        .filter(|&v| v != d.golden)
                } else {
                    None
                };
                let date_text = iso(d.date);
                let in_list_of_system = LISTS_009
                    .iter()
                    .find(|r| r.0 == system && r.1 == date_text)
                    .map(|r| (r.2, r.3));
                let equals_other_systems: Vec<&'static str> = SYSTEMS
                    .iter()
                    .filter(|&&(s, _)| s != system && engine_day(d.date, s).tshes == d.golden)
                    .map(|s| s.1)
                    .collect();
                let class = if TRANSCRIPTION_DISAGREED.contains(&date_text.as_str()) {
                    Class::Transcription
                } else if other_edition.is_some() {
                    Class::SourceDisagreement
                } else if in_list_of_system.is_some() || !equals_other_systems.is_empty() {
                    Class::VariantCandidate
                } else {
                    Class::Unexplained
                };
                out.push(Mismatch {
                    file: l.file.stem,
                    system,
                    diff: d,
                    other_edition,
                    in_list_of_system,
                    equals_other_systems,
                    class,
                });
            }
        }
    }
    out
}

#[test]
fn q1_report_the_mismatches_of_the_best_system_by_class_run_and_one_day_shift() {
    let files = loaded();
    let best = print_best(&files);
    let all = investigate(&files, &best);
    for m in &all {
        let shift = |s: i64| engine_day(add_days(m.diff.date, s), m.system).tshes == m.diff.golden;
        println!(
            "Q1 INVEST file={} system={} date={} golden={} engine={} other_edition={:?} in_spec_009_list_of_system={:?} equals_other_systems={:?} equals_engine_of_previous_day={} equals_engine_of_next_day={} class={:?}",
            m.file,
            name_of(m.system),
            iso(m.diff.date),
            m.diff.golden,
            m.diff.engine,
            m.other_edition,
            m.in_list_of_system,
            m.equals_other_systems,
            shift(-1),
            shift(1),
            m.class
        );
    }
    for l in &files {
        let mine: Vec<&Mismatch> = all.iter().filter(|m| m.file == l.file.stem).collect();
        let days: Vec<i64> = mine
            .iter()
            .map(|m| common::fixtures::day_number(m.diff.date))
            .collect();
        let mut run_start = 0;
        for i in 0..days.len() {
            if i + 1 == days.len() || days[i + 1] != days[i] + 1 {
                let run = &mine[run_start..=i];
                let shifts: Vec<i64> = [-1, 1]
                    .into_iter()
                    .filter(|&s| {
                        run.iter().all(|m| {
                            engine_day(add_days(m.diff.date, s), m.system).tshes == m.diff.golden
                        })
                    })
                    .collect();
                println!(
                    "Q1 RUN file={} system={} from={} to={} len={} lunation_first={} lunation_last={} common_one_day_shift={shifts:?}",
                    l.file.stem,
                    name_of(run[0].system),
                    iso(run[0].diff.date),
                    iso(run[run.len() - 1].diff.date),
                    run.len(),
                    lunation_of(&l.rows, run[0].diff.date).0,
                    lunation_of(&l.rows, run[run.len() - 1].diff.date).0,
                );
                run_start = i + 1;
            }
        }
        let count = |c: &Class| mine.iter().filter(|m| &m.class == c).count();
        println!(
            "Q1 CLASSES file={} mismatches={} TRANSCRIPTION={} SOURCE_DISAGREEMENT={} VARIANT_CANDIDATE={} UNEXPLAINED={}",
            l.file.stem,
            mine.len(),
            count(&Class::Transcription),
            count(&Class::SourceDisagreement),
            count(&Class::VariantCandidate),
            count(&Class::Unexplained)
        );
        assert_eq!(
            count(&Class::Transcription)
                + count(&Class::SourceDisagreement)
                + count(&Class::VariantCandidate)
                + count(&Class::Unexplained),
            mine.len(),
            "the classes sum to the mismatch count"
        );
    }
}

// --------------------------------- the verdict's figures (spec 014 session 2, Item 10)
//
// Every figure the verdict spec (the spec text of docs/specs/014-q1-verdict.md) cites that
// session 1 left as printed output only (ADDENDUM 1: C2; the reviewer's V2 and N1) is asserted
// below, each group with an in-memory plant shown to turn it red (METHOD rule 31; nothing on
// disk changes). The values were MEASURED by spec 014 session 2 twice: by a script outside this
// harness, from the CSV files and spec 009's dump probe, and by these tests. Like the pin, these
// PIN A MEASUREMENT, NOT A CORRECTNESS CLAIM (METHOD rule 33): they record what the verdict
// (`FORKS.md` F38, RULED (1)) was ruled on, so that a change in the engine, a golden file or the
// harness is loud; they say nothing about which system is right.

/// A copy of `files` with the rows of `stem` replaced (an in-memory plant).
fn with_rows(files: &[Loaded], stem: &str, rows: &[BuryatRow]) -> Vec<Loaded> {
    files
        .iter()
        .map(|l| Loaded {
            file: l.file,
            rows: if l.file.stem == stem {
                rows.to_vec()
            } else {
                l.rows.clone()
            },
        })
        .collect()
}

/// The verdict's totals (session 1's §1): per system over the 1,213 rows, and the three-way split
/// over the 1,165 distinct days.
const TOTALS: [Totals; 3] = [
    ("Phugpa", 1213, 1145, 1099, 58, 8),
    ("Tsurphu", 1213, 1184, 1137, 20, 8),
    ("Genden", 1213, 1199, 1151, 6, 8),
];

fn totals_violations(files: &[Loaded]) -> Vec<String> {
    totals_of(files)
        .into_iter()
        .zip(TOTALS)
        .filter(|(measured, pinned)| measured != pinned)
        .map(|(measured, pinned)| format!("TOTALS: measured {measured:?}, pinned {pinned:?}"))
        .collect()
}

#[test]
fn q1_verdict_the_totals_over_the_rows_and_the_distinct_days_and_a_planted_cell_moves_them() {
    let files = loaded();
    assert_eq!(distinct_days(&files).len(), 1165, "398 + 413 + 402 - 48");
    let violations = totals_violations(&files);
    assert!(
        violations.is_empty(),
        "the verdict's totals changed:\n{}",
        violations.join("\n")
    );
    // The plant: one cell of the Ivolginsky file (no overlap) on a day every system matches.
    let (date, wrong) = a_day_all_systems_match(&files[0].rows);
    let planted = with_rows(
        &files,
        files[0].file.stem,
        &with_tshes(&files[0].rows, date, wrong),
    );
    let caught = totals_violations(&planted);
    println!("Q1 TOTALS-PLANT day={} violations={caught:?}", iso(date));
    assert_eq!(caught.len(), 3, "one violation per system");
    for ((name, rows, m, all, none, split), pinned) in totals_of(&planted).into_iter().zip(TOTALS) {
        assert_eq!(
            (name, rows, m + 1, all + 1, none, split),
            (
                pinned.0,
                pinned.1,
                pinned.2,
                pinned.3,
                pinned.4 + 1,
                pinned.5
            ),
            "{name}: exactly one match and one all-sources day lost, one no-source day gained"
        );
    }
}

/// Per file, the source's lunation starts (`golden/buryat/README.md`'s rule), space-separated.
const SOURCE_STARTS: [(&str, &str); 3] = [
    (
        "ivolginsky-datsan-2021",
        "2021-01-01 2021-01-14 2021-02-12 2021-03-14 2021-04-13 2021-05-12 2021-06-11 2021-07-10 2021-08-09 2021-09-08 2021-10-07 2021-11-05 2021-12-05 2022-01-03 2022-02-02",
    ),
    (
        "aginsky-datsan-2025",
        "2025-01-01 2025-01-30 2025-03-01 2025-03-30 2025-04-28 2025-05-28 2025-06-26 2025-07-25 2025-08-24 2025-09-22 2025-10-22 2025-11-21 2025-12-20 2026-01-19",
    ),
    (
        "aginsky-datsan-2026",
        "2026-01-01 2026-01-19 2026-02-18 2026-03-20 2026-04-18 2026-05-17 2026-06-16 2026-07-15 2026-08-13 2026-09-12 2026-10-11 2026-11-10 2026-12-09 2027-01-08",
    ),
];

/// Per file and system: `(file, system, the source's lunation starts that are not starts of the
/// engine's months, the engine's month starts that are not the source's)` among the file's days.
/// Empty: the boundary dates are equal (ADDENDUM 1, N1: the Ivolginsky file included).
const BOUNDARIES_DIFFER: [(&str, &str, &str, &str); 9] = [
    (
        "ivolginsky-datsan-2021",
        "Phugpa",
        "2021-04-13 2021-09-08",
        "2021-04-12 2021-09-07",
    ),
    ("ivolginsky-datsan-2021", "Tsurphu", "", ""),
    ("ivolginsky-datsan-2021", "Genden", "", ""),
    ("aginsky-datsan-2025", "Phugpa", "2025-03-01", "2025-02-28"),
    ("aginsky-datsan-2025", "Tsurphu", "", ""),
    ("aginsky-datsan-2025", "Genden", "", ""),
    ("aginsky-datsan-2026", "Phugpa", "2026-03-20", "2026-03-19"),
    ("aginsky-datsan-2026", "Tsurphu", "", ""),
    ("aginsky-datsan-2026", "Genden", "", ""),
];

fn joined(dates: &[Ymd]) -> String {
    dates.iter().map(|&d| iso(d)).collect::<Vec<_>>().join(" ")
}

fn boundary_violations(files: &[Loaded]) -> Vec<String> {
    let mut out = Vec::new();
    for l in files {
        let stem = l.file.stem;
        let at =
            |starts: Vec<usize>| -> Vec<Ymd> { starts.iter().map(|&i| l.rows[i].date).collect() };
        let source = at(lunation_starts(&l.rows));
        let pinned = SOURCE_STARTS
            .iter()
            .find(|s| s.0 == stem)
            .expect("a file")
            .1;
        if joined(&source) != pinned {
            out.push(format!(
                "BOUNDARIES: {stem}: source starts [{}], pinned [{pinned}]",
                joined(&source)
            ));
        }
        for &(system, name) in &SYSTEMS {
            let engine = at(engine_starts(&l.rows, system));
            let only_source: Vec<Ymd> = source
                .iter()
                .filter(|d| !engine.contains(d))
                .copied()
                .collect();
            let only_engine: Vec<Ymd> = engine
                .iter()
                .filter(|d| !source.contains(d))
                .copied()
                .collect();
            let measured = (joined(&only_source), joined(&only_engine));
            let p = BOUNDARIES_DIFFER
                .iter()
                .find(|b| b.0 == stem && b.1 == name)
                .expect("a pinned pair");
            if (measured.0.as_str(), measured.1.as_str()) != (p.2, p.3) {
                out.push(format!(
                    "BOUNDARIES: {stem} {name}: only source [{}] only engine [{}], pinned [{}] [{}]",
                    measured.0, measured.1, p.2, p.3
                ));
            }
        }
    }
    out
}

#[test]
fn q1_verdict_the_lunation_boundary_dates_of_every_file_and_system_and_a_planted_boundary_moves_them()
 {
    let files = loaded();
    let violations = boundary_violations(&files);
    assert!(
        violations.is_empty(),
        "the boundary dates changed:\n{}",
        violations.join("\n")
    );
    // The plant: the first day of the Ivolginsky file's lunation from 2021-04-13 printed as 30,
    // so the source's lunation begins a day later.
    let day = parse("2021-04-13");
    let planted = with_rows(
        &files,
        files[0].file.stem,
        &with_tshes(&files[0].rows, day, 30),
    );
    let caught = boundary_violations(&planted);
    println!("Q1 BOUNDARIES-PLANT violations={caught:?}");
    assert_eq!(
        caught.len(),
        4,
        "the source's starts and each system's differences, for the planted file only"
    );
    assert!(
        caught
            .iter()
            .all(|v| v.starts_with("BOUNDARIES: ivolginsky-datsan-2021"))
    );
}

/// `(the distinct days on which all three systems give one tshes, on which at least two differ)`.
const ALL_THREE_AGREE: (usize, usize) = (1098, 67);

/// Per pair of systems `(a, b, distinct days on which a and b differ, the source rows on those
/// days, rows whose tshes equals a's, rows whose tshes equals b's)` (session 1's §2(h); R6).
type Separation = (&'static str, &'static str, usize, usize, usize, usize);

const SEPARATE: [Separation; 3] = [
    ("Phugpa", "Tsurphu", 62, 63, 12, 51),
    ("Phugpa", "Genden", 58, 60, 3, 57),
    ("Tsurphu", "Genden", 14, 15, 0, 15),
];

fn separation_of(files: &[Loaded]) -> ((usize, usize), Vec<Separation>) {
    let days = distinct_days(files);
    let t = |d: Ymd, s: System| engine_day(d, s).tshes;
    let same = days
        .iter()
        .filter(|&&d| {
            t(d, System::Phugpa) == t(d, System::Tsurphu)
                && t(d, System::Tsurphu) == t(d, System::Genden)
        })
        .count();
    let pairs = [
        (System::Phugpa, System::Tsurphu),
        (System::Phugpa, System::Genden),
        (System::Tsurphu, System::Genden),
    ];
    let per_pair = pairs
        .iter()
        .map(|&(a, b)| {
            let apart: Vec<Ymd> = days
                .iter()
                .copied()
                .filter(|&d| t(d, a) != t(d, b))
                .collect();
            let rows: Vec<(Ymd, u8)> = apart
                .iter()
                .flat_map(|&d| {
                    source_values(files, d)
                        .into_iter()
                        .map(move |(_, v)| (d, v))
                })
                .collect();
            let equal_a = rows.iter().filter(|&&(d, v)| v == t(d, a)).count();
            let equal_b = rows.iter().filter(|&&(d, v)| v == t(d, b)).count();
            (
                name_of(a),
                name_of(b),
                apart.len(),
                rows.len(),
                equal_a,
                equal_b,
            )
        })
        .collect();
    ((same, days.len() - same), per_pair)
}

#[test]
fn q1_verdict_the_days_that_separate_the_systems_and_a_planted_tsurphu_value_is_counted() {
    let files = loaded();
    let (all_three, pairs) = separation_of(&files);
    assert_eq!(
        all_three, ALL_THREE_AGREE,
        "all three agree / at least two differ"
    );
    assert_eq!(pairs, SEPARATE.to_vec(), "the days that separate each pair");
    // The plant: on the first Tsurphu-Genden day of the first file that has one, the source
    // prints Tsurphu's value.
    let (stem, day) = files
        .iter()
        .find_map(|l| {
            l.rows
                .iter()
                .find(|r| {
                    engine_day(r.date, System::Tsurphu).tshes
                        != engine_day(r.date, System::Genden).tshes
                })
                .map(|r| (l.file.stem, r.date))
        })
        .expect("a Tsurphu-Genden day");
    let rows = &files
        .iter()
        .find(|l| l.file.stem == stem)
        .expect("file")
        .rows;
    let planted = with_rows(
        &files,
        stem,
        &with_tshes(rows, day, engine_day(day, System::Tsurphu).tshes),
    );
    let (_, caught) = separation_of(&planted);
    println!(
        "Q1 SEPARATE-PLANT file={stem} day={} pairs={caught:?}",
        iso(day)
    );
    assert_ne!(caught, SEPARATE.to_vec(), "the plant is seen");
    assert_eq!(
        caught[2],
        ("Tsurphu", "Genden", 14, 15, 1, 14),
        "exactly one row moves from Genden's value to Tsurphu's"
    );
}

/// Every Genden mismatch: `(file, date, golden tshes, Genden's tshes)` (R5 (d)).
const GENDEN_MISMATCHES: [(&str, &str, u8, u8); 14] = [
    ("aginsky-datsan-2025", "2025-12-22", 3, 2),
    ("aginsky-datsan-2025", "2025-12-23", 4, 3),
    ("aginsky-datsan-2025", "2025-12-24", 5, 4),
    ("aginsky-datsan-2025", "2025-12-25", 6, 5),
    ("aginsky-datsan-2025", "2025-12-26", 7, 6),
    ("aginsky-datsan-2025", "2025-12-27", 8, 7),
    ("aginsky-datsan-2026", "2026-01-09", 22, 21),
    ("aginsky-datsan-2026", "2026-01-10", 23, 22),
    ("aginsky-datsan-2026", "2026-01-11", 24, 23),
    ("aginsky-datsan-2026", "2026-01-12", 25, 24),
    ("aginsky-datsan-2026", "2026-01-13", 26, 25),
    ("aginsky-datsan-2026", "2026-01-14", 27, 26),
    ("aginsky-datsan-2026", "2026-01-15", 28, 27),
    ("aginsky-datsan-2026", "2026-01-16", 29, 28),
];

/// The one lunation that holds every Genden mismatch: the 2025 edition's lunation 13, of which
/// the 2026 edition's first lunation (from 2026-01-01) is the end.
const OPEN_LUNATION: (&str, &str) = ("2025-12-20", "2026-01-18");

/// `(lunations of the Ivolginsky file, of the 2025 edition, of the 2026 edition, lunations the two
/// editions share, distinct lunations of the three files)`.
const LUNATIONS: (usize, usize, usize, usize, usize) = (15, 14, 14, 2, 41);

/// The lunations of every file as date ranges, merged where two files' ranges overlap:
/// `(per-file counts, ranges shared by two files, the merged ranges)`.
fn merged_lunations(files: &[Loaded]) -> (Vec<usize>, usize, Vec<(Ymd, Ymd)>) {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut per_file = Vec::new();
    for l in files {
        let starts = lunation_starts(&l.rows);
        per_file.push(starts.len());
        for (k, &s) in starts.iter().enumerate() {
            let e = starts.get(k + 1).copied().unwrap_or(l.rows.len()) - 1;
            ranges.push((
                common::fixtures::day_number(l.rows[s].date),
                common::fixtures::day_number(l.rows[e].date),
            ));
        }
    }
    ranges.sort_unstable();
    let mut merged: Vec<(i64, i64)> = Vec::new();
    let mut shared = 0;
    for (a, b) in ranges {
        match merged.last_mut() {
            Some(last) if a <= last.1 => {
                last.1 = last.1.max(b);
                shared += 1;
            }
            _ => merged.push((a, b)),
        }
    }
    let first = common::fixtures::day_number((1900, 1, 1));
    let back = |n: i64| add_days((1900, 1, 1), n - first);
    let dates = merged.iter().map(|&(a, b)| (back(a), back(b))).collect();
    (per_file, shared, dates)
}

fn lunation_violations(files: &[Loaded]) -> Vec<String> {
    let mut out = Vec::new();
    let mut found = Vec::new();
    for l in files {
        for d in compare(&l.rows, System::Genden).diffs {
            found.push((l.file.stem, iso(d.date), d.golden, d.engine));
        }
    }
    let pinned: Vec<(&str, String, u8, u8)> = GENDEN_MISMATCHES
        .iter()
        .map(|&(f, d, g, e)| (f, d.to_owned(), g, e))
        .collect();
    if found != pinned {
        out.push(format!(
            "LUNATION: Genden mismatches {found:?}, pinned {pinned:?}"
        ));
    }
    let (per_file, shared, merged) = merged_lunations(files);
    let counts = (per_file[0], per_file[1], per_file[2], shared, merged.len());
    if counts != LUNATIONS {
        out.push(format!(
            "LUNATION: lunations {counts:?}, pinned {LUNATIONS:?}"
        ));
    }
    let open = (parse(OPEN_LUNATION.0), parse(OPEN_LUNATION.1));
    if !merged.contains(&open) {
        out.push(format!(
            "LUNATION: {OPEN_LUNATION:?} is not one of the merged lunations"
        ));
    }
    let (lo, hi) = (
        common::fixtures::day_number(open.0),
        common::fixtures::day_number(open.1),
    );
    for (stem, date, _, engine) in &found {
        let n = common::fixtures::day_number(parse(date));
        if n < lo || n > hi {
            out.push(format!(
                "LUNATION: {stem} {date} lies outside {OPEN_LUNATION:?}"
            ));
        }
        // On a day the other edition also prints, it prints Genden's value (R5 (d)).
        for (other, value) in source_values(files, parse(date)) {
            if other != *stem && value != *engine {
                out.push(format!(
                    "LUNATION: {date}: {other} prints {value}, Genden {engine}"
                ));
            }
        }
    }
    out
}

#[test]
fn q1_verdict_every_genden_mismatch_lies_in_one_lunation_of_the_41_and_a_planted_split_is_seen() {
    let files = loaded();
    let violations = lunation_violations(&files);
    assert!(
        violations.is_empty(),
        "the verdict's lunation figures changed:\n{}",
        violations.join("\n")
    );
    // The plant: a day of the Ivolginsky file inside a lunation, printed as 1 (a new lunation
    // begins there, and Genden gains a mismatch outside the open lunation).
    let row = files[0]
        .rows
        .iter()
        .skip(100)
        .find(|r| {
            r.tshes >= 5
                && SYSTEMS
                    .iter()
                    .all(|&(s, _)| engine_day(r.date, s).tshes == r.tshes)
        })
        .expect("a day");
    let planted = with_rows(
        &files,
        files[0].file.stem,
        &with_tshes(&files[0].rows, row.date, 1),
    );
    let caught = lunation_violations(&planted);
    println!(
        "Q1 LUNATION-PLANT day={} violations={caught:?}",
        iso(row.date)
    );
    assert!(
        caught
            .iter()
            .any(|v| v.contains("lunations (16, 14, 14, 2, 42)"))
    );
    assert!(
        caught
            .iter()
            .any(|v| v.contains(&format!("{} lies outside", iso(row.date))))
    );
}

/// Per Aginsky file and system: `(file, system, the engine's leap-month days among the file's,
/// the source's «Дополнительный …» days, the offsets (engine month - source month) mod 12 over
/// the source's regular lunations as offset:count)`; a run of days is written `first..last (n)`
/// (C1, `DEBT.md` D-40: one extra month, n = 1).
const EXTRA_MONTH: [(&str, &str, &str, &str, &str); 6] = [
    (
        "aginsky-datsan-2025",
        "Phugpa",
        "",
        "2025-01-30..2025-02-28 (30)",
        "0:12 11:1",
    ),
    (
        "aginsky-datsan-2025",
        "Tsurphu",
        "2025-01-01..2025-01-29 (29)",
        "2025-01-30..2025-02-28 (30)",
        "0:13",
    ),
    (
        "aginsky-datsan-2025",
        "Genden",
        "2025-01-01..2025-01-29 (29)",
        "2025-01-30..2025-02-28 (30)",
        "0:13",
    ),
    ("aginsky-datsan-2026", "Phugpa", "", "", "0:14"),
    ("aginsky-datsan-2026", "Tsurphu", "", "", "0:14"),
    ("aginsky-datsan-2026", "Genden", "", "", "0:14"),
];

fn spans(rows: &[BuryatRow], flags: &[bool]) -> String {
    runs_of(flags)
        .iter()
        .map(|&(a, b)| {
            format!(
                "{}..{} ({})",
                iso(rows[a].date),
                iso(rows[b].date),
                b - a + 1
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn extra_month_violations(files: &[Loaded]) -> Vec<String> {
    let mut out = Vec::new();
    for l in files.iter().filter(|l| l.file.labelled) {
        let extra: Vec<bool> = l.rows.iter().map(|r| r.leap == Some(true)).collect();
        for &(system, name) in &SYSTEMS {
            let leap: Vec<bool> = l
                .rows
                .iter()
                .map(|r| engine_day(r.date, system).leap)
                .collect();
            let mut counts = [0usize; 12];
            for m in month_lines(&l.rows, system) {
                if let Some(o) = m.offset {
                    counts[usize::from(o)] += 1;
                }
            }
            let offsets: Vec<String> = counts
                .iter()
                .enumerate()
                .filter(|(_, n)| **n > 0)
                .map(|(o, n)| format!("{o}:{n}"))
                .collect();
            let measured = (
                spans(&l.rows, &leap),
                spans(&l.rows, &extra),
                offsets.join(" "),
            );
            let p = EXTRA_MONTH
                .iter()
                .find(|e| e.0 == l.file.stem && e.1 == name)
                .expect("a pinned row");
            if (
                measured.0.as_str(),
                measured.1.as_str(),
                measured.2.as_str(),
            ) != (p.2, p.3, p.4)
            {
                out.push(format!(
                    "EXTRA-MONTH: {} {name}: measured {measured:?}, pinned {:?}",
                    l.file.stem,
                    (p.2, p.3, p.4)
                ));
            }
        }
    }
    out
}

#[test]
fn q1_verdict_the_extra_month_relation_n_1_and_a_planted_month_label_moves_the_offsets() {
    let files = loaded();
    let violations = extra_month_violations(&files);
    assert!(
        violations.is_empty(),
        "the extra-month relation changed:\n{}",
        violations.join("\n")
    );
    // The plant: the 2025 edition's lunation from 2025-03-30 labelled one month later.
    let l = &files[1];
    let (from, to) = (parse("2025-03-30"), parse("2025-04-27"));
    let range = common::fixtures::day_number(from)..=common::fixtures::day_number(to);
    let rows: Vec<BuryatRow> = l
        .rows
        .iter()
        .map(|r| BuryatRow {
            month: if range.contains(&common::fixtures::day_number(r.date)) {
                r.month.map(|m| m % 12 + 1)
            } else {
                r.month
            },
            ..r.clone()
        })
        .collect();
    let caught = extra_month_violations(&with_rows(&files, l.file.stem, &rows));
    println!("Q1 EXTRA-MONTH-PLANT violations={caught:?}");
    assert_eq!(
        caught.len(),
        3,
        "every system's offsets, for the planted file only"
    );
    assert!(
        caught
            .iter()
            .all(|v| v.starts_with("EXTRA-MONTH: aginsky-datsan-2025") && v.contains("11:"))
    );
}

/// The open lunation in each Aginsky edition: `(file, the printed tshes, space-separated)`
/// (ADDENDUM 1, N3; the verdict spec's account of the open days).
const OPEN_PRINTED: [(&str, &str); 2] = [
    (
        "aginsky-datsan-2025",
        "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30",
    ),
    (
        "aginsky-datsan-2026",
        "13 14 15 16 17 18 19 20 22 23 24 25 26 27 28 29 29 30",
    ),
];

/// On the same days, per file and for Tsurphu and Genden: `(file, system, the engine's tshes,
/// the days it doubles, the days after a skipped number as day:number)`.
const OPEN_ENGINE: [(&str, &str, &str, &str, &str); 4] = [
    (
        "aginsky-datsan-2025",
        "Tsurphu",
        "1 2 2 3 4 5 6 7 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30",
        "2025-12-22",
        "2025-12-28:8",
    ),
    (
        "aginsky-datsan-2025",
        "Genden",
        "1 2 2 3 4 5 6 7 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30",
        "2025-12-22",
        "2025-12-28:8",
    ),
    (
        "aginsky-datsan-2026",
        "Tsurphu",
        "13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30",
        "",
        "",
    ),
    (
        "aginsky-datsan-2026",
        "Genden",
        "13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30",
        "",
        "",
    ),
];

fn open_days_violations(files: &[Loaded]) -> Vec<String> {
    let (lo, hi) = (
        common::fixtures::day_number(parse(OPEN_LUNATION.0)),
        common::fixtures::day_number(parse(OPEN_LUNATION.1)),
    );
    let mut out = Vec::new();
    for l in files.iter().filter(|l| l.file.labelled) {
        let window: Vec<&BuryatRow> = l
            .rows
            .iter()
            .filter(|r| (lo..=hi).contains(&common::fixtures::day_number(r.date)))
            .collect();
        let list = |v: Vec<u8>| v.iter().map(u8::to_string).collect::<Vec<_>>().join(" ");
        let printed = list(window.iter().map(|r| r.tshes).collect());
        let p = OPEN_PRINTED
            .iter()
            .find(|p| p.0 == l.file.stem)
            .expect("a file");
        if printed != p.1 {
            out.push(format!(
                "OPEN-DAYS: {} printed [{printed}], pinned [{}]",
                l.file.stem, p.1
            ));
        }
        for &(system, name) in SYSTEMS.iter().filter(|s| s.0 != System::Phugpa) {
            let e: Vec<_> = window
                .iter()
                .map(|r| (r.date, engine_day(r.date, system)))
                .collect();
            let tshes = list(e.iter().map(|(_, z)| z.tshes).collect());
            let doubled = joined(
                &e.iter()
                    .filter(|(_, z)| z.doubled)
                    .map(|(d, _)| *d)
                    .collect::<Vec<_>>(),
            );
            let skipped = e
                .iter()
                .filter_map(|(d, z)| z.skipped.map(|s| format!("{}:{s}", iso(*d))))
                .collect::<Vec<_>>()
                .join(" ");
            let q = OPEN_ENGINE
                .iter()
                .find(|q| q.0 == l.file.stem && q.1 == name)
                .expect("a pinned row");
            if (tshes.as_str(), doubled.as_str(), skipped.as_str()) != (q.2, q.3, q.4) {
                out.push(format!(
                    "OPEN-DAYS: {} {name}: [{tshes}] doubled [{doubled}] skipped [{skipped}], pinned {:?}",
                    l.file.stem,
                    (q.2, q.3, q.4)
                ));
            }
        }
    }
    out
}

#[test]
fn q1_verdict_the_open_lunation_as_printed_and_as_computed_and_a_planted_doubled_day_is_seen() {
    let files = loaded();
    let violations = open_days_violations(&files);
    assert!(
        violations.is_empty(),
        "the open lunation changed:\n{}",
        violations.join("\n")
    );
    // The plant: the 2025 edition prints 2025-12-23 as 3, a doubled day.
    let l = &files[1];
    let planted = with_rows(
        &files,
        l.file.stem,
        &with_tshes(&l.rows, parse("2025-12-23"), 3),
    );
    let caught = open_days_violations(&planted);
    println!("Q1 OPEN-DAYS-PLANT violations={caught:?}");
    assert_eq!(
        caught.len(),
        1,
        "the printed sequence of the planted file only"
    );
    assert!(caught[0].starts_with("OPEN-DAYS: aginsky-datsan-2025 printed [1 2 3 3 5"));
}
