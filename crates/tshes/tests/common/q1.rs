//! The Q1 comparison (spec 014 Items 3 and 4; 001 §6.5): the rows of
//! `golden/buryat/` against what the engine computes for the same Gregorian
//! day under each of its three systems, reached through the public API only
//! (`to_zdate` with the system named, `DEBT.md` D-1).
//!
//! **The deciding field (METHOD rule 25).** The `tshes` column is read from the
//! print in all three files and the engine computes it: both sides see it, so it
//! alone counts as a match or a mismatch. `is_doubled` and `skipped` are DERIVED
//! from the tshes sequence by spec 013 (`FORKS.md` F9 (b)), so they are not
//! independent evidence: they are compared and reported, never counted as
//! further mismatches. `month` and `is_leap_month` exist only in the two
//! Aginsky files, as a label per lunation: they are compared per lunation
//! (`month_lines`), never per day.
//!
//! The function `compare` is the one path every control and every report uses.
//! **What it does not do:** it does not say which system a datsan follows; it
//! counts. A comparison of golden rows with a system is evidence of Buryat
//! practice only for T1–T3 rows (001 §6.1); the T4 fixture goes through it as a
//! control of the harness, never as evidence.
#![allow(dead_code)]

use tshes::{GregorianDate, System, to_zdate};

use super::buryat::{BuryatRow, add_days, lunation_starts};
use super::fixtures::{Ymd, day_number, t4_rows};

/// The three systems, each named explicitly (D-1), with the name the output uses.
pub const SYSTEMS: [(System, &str); 3] = [
    (System::Phugpa, "Phugpa"),
    (System::Tsurphu, "Tsurphu"),
    (System::Genden, "Genden"),
];

/// What the engine computes for one Gregorian day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineDay {
    pub year: i64,
    pub month: u8,
    pub leap: bool,
    pub tshes: u8,
    pub doubled: bool,
    pub skipped: Option<u8>,
}

impl EngineDay {
    /// The month as `(year, month, is_leap_month)`.
    pub fn month_key(&self) -> (i64, u8, bool) {
        (self.year, self.month, self.leap)
    }
}

/// The engine's `ZDate` of a day under `system`, in plain fields.
pub fn engine_day(date: Ymd, system: System) -> EngineDay {
    let day = GregorianDate::new(date.0, date.1, date.2).expect("a valid Gregorian date");
    let z = to_zdate(day, system).expect("a day inside 1900-01-01..=2100-12-31");
    EngineDay {
        year: z.year().number(),
        month: z.month().get(),
        leap: z.is_leap_month(),
        tshes: z.tshes().get(),
        doubled: z.is_doubled(),
        skipped: z.skipped_before().map(tshes::Tshes::get),
    }
}

/// One day on which the source's tshes and the engine's differ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TshesDiff {
    pub date: Ymd,
    pub golden: u8,
    pub engine: u8,
}

/// The result of comparing a list of rows with one system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comparison {
    pub rows: usize,
    /// Rows whose tshes equals the engine's: the deciding count.
    pub matches: usize,
    /// Rows whose tshes differs, in date order: the other deciding count.
    pub diffs: Vec<TshesDiff>,
    /// Rows compared for `is_doubled` and `skipped`: every row but the first,
    /// whose predecessor is not in the source (`golden/buryat/README.md`).
    pub flag_rows: usize,
    /// Rows where the engine's `is_doubled` differs from the derived column. Reported, not counted.
    pub doubled_disagree: Vec<Ymd>,
    /// Rows where the engine's `skipped_before` differs from the derived column. Reported, not counted.
    pub skipped_disagree: Vec<Ymd>,
}

/// Compares every row with the engine under `system`.
pub fn compare(rows: &[BuryatRow], system: System) -> Comparison {
    let mut out = Comparison {
        rows: rows.len(),
        matches: 0,
        diffs: Vec::new(),
        flag_rows: 0,
        doubled_disagree: Vec::new(),
        skipped_disagree: Vec::new(),
    };
    for (i, row) in rows.iter().enumerate() {
        let engine = engine_day(row.date, system);
        if engine.tshes == row.tshes {
            out.matches += 1;
        } else {
            out.diffs.push(TshesDiff {
                date: row.date,
                golden: row.tshes,
                engine: engine.tshes,
            });
        }
        if i > 0 {
            out.flag_rows += 1;
            if engine.doubled != row.doubled {
                out.doubled_disagree.push(row.date);
            }
            if engine.skipped != row.skipped {
                out.skipped_disagree.push(row.date);
            }
        }
    }
    out
}

/// Sets `doubled` and `skipped` on every row but the first from the tshes steps,
/// as spec 013 derived them (`FORKS.md` F9 (b)): step 0 is a doubled day, step 2
/// has skipped the number between.
pub fn derive_flags(rows: &mut [BuryatRow]) {
    let mut previous: Option<u8> = None;
    for row in rows.iter_mut() {
        row.doubled = false;
        row.skipped = None;
        if let Some(prev) = previous {
            match (i16::from(row.tshes) - i16::from(prev)).rem_euclid(30) {
                0 => row.doubled = true,
                2 => row.skipped = Some(prev % 30 + 1),
                _ => {}
            }
        }
        previous = Some(row.tshes);
    }
}

fn blank(date: Ymd, tshes: u8) -> BuryatRow {
    BuryatRow {
        date,
        tshes,
        month: None,
        leap: None,
        doubled: false,
        skipped: None,
        notes: String::new(),
    }
}

/// Rows holding the T4 fixture's tshes (Lotsawa House's Phugpa; tier T4) on the given
/// dates, the flags derived from the steps exactly as the golden files' are. A
/// control of the harness only (spec 014 Item 3(b), STOP RULE (5)).
pub fn t4_rows_on(dates: &[Ymd]) -> Vec<BuryatRow> {
    let fixture = t4_rows();
    let first = day_number((1900, 1, 1));
    let mut rows: Vec<BuryatRow> = dates
        .iter()
        .map(|&date| {
            let index = usize::try_from(day_number(date) - first).expect("a date from 1900-01-01");
            let row = fixture.get(index).expect("a date inside the fixture");
            assert_eq!(row.date, date, "the fixture row is for the date asked");
            blank(date, row.tshes)
        })
        .collect();
    derive_flags(&mut rows);
    rows
}

/// Rows built from the engine's own output under `system` on the given dates,
/// with the engine's own flags (the self-output control, Item 3(c)).
pub fn engine_rows_on(dates: &[Ymd], system: System) -> Vec<BuryatRow> {
    dates
        .iter()
        .enumerate()
        .map(|(i, &date)| {
            let e = engine_day(date, system);
            let mut row = blank(date, e.tshes);
            if i > 0 {
                row.doubled = e.doubled;
                row.skipped = e.skipped;
            }
            row
        })
        .collect()
}

/// A copy of the rows with every date moved by `days` (the planted shift, Item 3(d)(ii)).
pub fn shifted(rows: &[BuryatRow], days: i64) -> Vec<BuryatRow> {
    rows.iter()
        .map(|r| BuryatRow {
            date: add_days(r.date, days),
            ..r.clone()
        })
        .collect()
}

/// A copy of the rows with the tshes of one day replaced (the planted wrong cell, Item 3(d)(i)).
pub fn with_tshes(rows: &[BuryatRow], date: Ymd, tshes: u8) -> Vec<BuryatRow> {
    rows.iter()
        .map(|r| BuryatRow {
            tshes: if r.date == date { tshes } else { r.tshes },
            ..r.clone()
        })
        .collect()
}

/// The dates of the rows.
pub fn dates_of(rows: &[BuryatRow]) -> Vec<Ymd> {
    rows.iter().map(|r| r.date).collect()
}

/// The row indexes at which the engine's month changes (row 0 included).
pub fn engine_starts(rows: &[BuryatRow], system: System) -> Vec<usize> {
    let mut starts = vec![0];
    let mut previous = rows.first().map(|r| engine_day(r.date, system).month_key());
    for (i, row) in rows.iter().enumerate().skip(1) {
        let key = engine_day(row.date, system).month_key();
        if Some(key) != previous {
            starts.push(i);
        }
        previous = Some(key);
    }
    starts
}

/// One lunation of a source, beside what the engine says on its first day.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonthLine {
    /// 1-based.
    pub k: usize,
    pub start: Ymd,
    pub days: usize,
    pub source_month: Option<u8>,
    pub source_leap: bool,
    /// The label printed on the first day (the notes), without guillemets.
    pub label: String,
    pub engine: EngineDay,
    /// `(engine month − source month) mod 12`, for a source lunation that is not the extra month.
    pub offset: Option<u8>,
}

/// Each lunation of an Aginsky file (the two files whose `month` is filled) beside the engine.
pub fn month_lines(rows: &[BuryatRow], system: System) -> Vec<MonthLine> {
    let starts = lunation_starts(rows);
    starts
        .iter()
        .enumerate()
        .filter_map(|(k, &start)| {
            let end = starts.get(k + 1).copied().unwrap_or(rows.len());
            let first = rows.get(start)?;
            let engine = engine_day(first.date, system);
            let source_leap = first.leap == Some(true);
            let offset = first.month.filter(|_| !source_leap).map(|m| {
                u8::try_from((i16::from(engine.month) - i16::from(m)).rem_euclid(12)).unwrap_or(0)
            });
            Some(MonthLine {
                k: k + 1,
                start: first.date,
                days: end - start,
                source_month: first.month,
                source_leap,
                label: super::buryat::printed_label(&first.notes)
                    .unwrap_or("")
                    .to_owned(),
                engine,
                offset,
            })
        })
        .collect()
}
