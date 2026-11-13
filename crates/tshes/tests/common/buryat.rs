//! Reading and checking `golden/buryat/` (spec 014 Item 2; `DEBT.md` D-37).
//!
//! The files are tier T1 facts transcribed by spec 013, in the formats of
//! `golden/buryat/README.md`. std only, no dependency (002 §4.9). Nothing here
//! calls the engine: the day arithmetic is the TEST side's own (`fixtures::day_number`
//! and `from_day_number`), so a check built on it is independent of the engine
//! (METHOD rule 35).
//!
//! Every refusal is a `String` that starts with a CODE and a colon, for example
//! `STEP:`, so a test (and a planted control, METHOD rule 31) can assert which
//! check fired and not merely that something failed.
//!
//! **What these checks do not do:** they do not compare a file with any
//! computation system (`q1_differential.rs`), they do not re-read a printed
//! calendar, and a file that passes is well-formed, not correct.
#![allow(dead_code)]

use std::fs;

use super::fixtures::{Ymd, day_number, iso};

/// The columns of 001 §6.2, in order.
pub const HEADER: [&str; 7] = [
    "gregorian_date",
    "tshes",
    "month",
    "is_leap_month",
    "is_doubled",
    "skipped",
    "notes",
];

/// The meta keys of `golden/buryat/README.md`, section "The meta keys", in order.
pub const META_KEYS: [&str; 17] = [
    "tier",
    "source",
    "publisher",
    "datsan",
    "lama",
    "location",
    "sha256",
    "retrieval",
    "edition_year",
    "membership",
    "method",
    "transcribed_or_generated_by",
    "date",
    "covers",
    "rights",
    "licence",
    "notes",
];

/// One golden file the tests name.
#[derive(Debug, Clone, Copy)]
pub struct SourceFile {
    /// The file name without `.csv`.
    pub stem: &'static str,
    /// Whether the source prints a lunar month label, so that `month` and
    /// `is_leap_month` are filled (the Ivolginsky calendar prints none: R8 of spec 013).
    pub labelled: bool,
    /// The rows `golden/buryat/README.md` counts for the file.
    pub rows: usize,
    /// The tier of 001 §6.1 (`FORKS.md` F29, F30): all three files are T1.
    pub tier: &'static str,
    /// The first and last date `golden/buryat/README.md` gives.
    pub first: &'static str,
    pub last: &'static str,
}

/// The three files of `golden/buryat/` (spec 013). A fourth CSV in the
/// directory is refused by `buryat_the_csv_files_are_the_three_the_tests_name`.
pub const FILES: [SourceFile; 3] = [
    SourceFile {
        stem: "ivolginsky-datsan-2021",
        labelled: false,
        rows: 398,
        tier: "T1",
        first: "2021-01-01",
        last: "2022-02-02",
    },
    SourceFile {
        stem: "aginsky-datsan-2025",
        labelled: true,
        rows: 413,
        tier: "T1",
        first: "2025-01-01",
        last: "2026-02-17",
    },
    SourceFile {
        stem: "aginsky-datsan-2026",
        labelled: true,
        rows: 402,
        tier: "T1",
        first: "2026-01-01",
        last: "2027-02-06",
    },
];

/// One data row of a golden file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuryatRow {
    pub date: Ymd,
    pub tshes: u8,
    /// Empty in the file: `None`.
    pub month: Option<u8>,
    /// Empty in the file: `None` (the Ivolginsky file throughout).
    pub leap: Option<bool>,
    pub doubled: bool,
    /// The tshes number omitted immediately before the day (`FORKS.md` F9 (b)); empty: `None`.
    pub skipped: Option<u8>,
    pub notes: String,
}

/// The path of a file under `golden/buryat/`.
pub fn path(name: &str) -> String {
    format!("{}/../../golden/buryat/{name}", env!("CARGO_MANIFEST_DIR"))
}

/// The directory `golden/buryat/`.
pub fn dir() -> String {
    format!("{}/../../golden/buryat", env!("CARGO_MANIFEST_DIR"))
}

/// The text of a file under `golden/buryat/`; a missing file is a panic with the path.
pub fn read(name: &str) -> String {
    fs::read_to_string(path(name))
        .unwrap_or_else(|e| panic!("cannot read golden/buryat/{name}: {e}"))
}

fn days_in_month(year: i64, month: u8) -> u8 {
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// `YYYY-MM-DD` to a date, refusing any other shape and any day the month does not have.
pub fn parse_date(text: &str) -> Result<Ymd, String> {
    let mut parts = text.split('-');
    let (Some(y), Some(m), Some(d), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(format!("not YYYY-MM-DD: {text:?}"));
    };
    let digits = |p: &str| p.bytes().all(|b| b.is_ascii_digit());
    if y.len() != 4 || m.len() != 2 || d.len() != 2 || !digits(y) || !digits(m) || !digits(d) {
        return Err(format!("not YYYY-MM-DD: {text:?}"));
    }
    let year: i64 = y.parse().map_err(|_| format!("year of {text:?}"))?;
    let month: u8 = m.parse().map_err(|_| format!("month of {text:?}"))?;
    let day: u8 = d.parse().map_err(|_| format!("day of {text:?}"))?;
    if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
        return Err(format!("no such day: {text:?}"));
    }
    Ok((year, month, day))
}

/// The date `n` days after `date`, by the civil-from-days algorithm of H. Hinnant
/// (public domain), the inverse of `fixtures::day_number`.
pub fn add_days(date: Ymd, n: i64) -> Ymd {
    let z = day_number(date) + n + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe.div_euclid(1_460) + doe.div_euclid(36_524) - doe.div_euclid(146_096))
        .div_euclid(365);
    let doy = doe - (365 * yoe + yoe.div_euclid(4) - yoe.div_euclid(100));
    let mp = (5 * doy + 2).div_euclid(153);
    let day = doy - (153 * mp + 2).div_euclid(5) + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    (
        year,
        u8::try_from(month).unwrap_or(0),
        u8::try_from(day).unwrap_or(0),
    )
}

fn parse_flag(what: &str, row_no: usize, text: &str) -> Result<bool, String> {
    match text {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!(
            "BOOL: row {row_no}: {what} is {other:?}, not true or false"
        )),
    }
}

fn parse_row(name: &str, row_no: usize, line: &str) -> Result<BuryatRow, String> {
    let fields: Vec<&str> = line.split(',').collect();
    let &[date, tshes, month, leap, doubled, skipped, notes] = fields.as_slice() else {
        return Err(format!(
            "FIELD-COUNT: {name}: row {row_no} has {} fields, not 7",
            fields.len()
        ));
    };
    let date = parse_date(date).map_err(|e| format!("DATE: {name}: row {row_no}: {e}"))?;
    let tshes: u8 = tshes
        .parse()
        .ok()
        .filter(|t| (1..=30).contains(t))
        .ok_or_else(|| {
            format!("TSHES-RANGE: {name}: row {row_no}: tshes {tshes:?} is not 1 to 30")
        })?;
    let month = if month.is_empty() {
        None
    } else {
        Some(
            month
                .parse::<u8>()
                .ok()
                .filter(|m| (1..=12).contains(m))
                .ok_or_else(|| {
                    format!(
                        "MONTH-RANGE: {name}: row {row_no}: month {month:?} is not empty or 1 to 12"
                    )
                })?,
        )
    };
    let leap = if leap.is_empty() {
        None
    } else {
        Some(parse_flag("is_leap_month", row_no, leap)?)
    };
    let doubled = parse_flag("is_doubled", row_no, doubled)?;
    let skipped = if skipped.is_empty() {
        None
    } else {
        Some(
            skipped
                .parse::<u8>()
                .ok()
                .filter(|s| (1..=30).contains(s))
                .ok_or_else(|| {
                    format!("SKIPPED-RANGE: {name}: row {row_no}: skipped {skipped:?} is not empty or 1 to 30")
                })?,
        )
    };
    Ok(BuryatRow {
        date,
        tshes,
        month,
        leap,
        doubled,
        skipped,
        notes: notes.to_owned(),
    })
}

/// The rows of a golden CSV text, or the first refusal.
///
/// Refused, each with its code: a carriage return (`CR`); a quote character
/// (`QUOTE`); no final newline (`FINAL-NEWLINE`); a header other than 001 §6.2's
/// seven columns (`HEADER`); a row of another field count (`FIELD-COUNT`); a date
/// that is not ISO or not a real day (`DATE`) or not one day after the previous
/// row (`DATE-STEP`); a tshes outside 1 to 30 (`TSHES-RANGE`); a month outside
/// empty or 1 to 12 (`MONTH-RANGE`); a boolean other than `true` / `false`
/// (`BOOL`; `is_leap_month` may also be empty, as `golden/buryat/README.md` says
/// for the unlabelled Ivolginsky file); a skipped outside empty or 1 to 30
/// (`SKIPPED-RANGE`); no data row (`ZERO-ROWS`, 002 §6).
pub fn parse(name: &str, text: &str) -> Result<Vec<BuryatRow>, String> {
    if text.contains('\r') {
        return Err(format!("CR: {name}: carriage return (LF endings only)"));
    }
    if text.contains('"') {
        return Err(format!("QUOTE: {name}: a quote character (no quoting)"));
    }
    if !text.ends_with('\n') {
        return Err(format!("FINAL-NEWLINE: {name}: no final newline"));
    }
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("");
    if header != HEADER.join(",") {
        return Err(format!("HEADER: {name}: header is {header:?}"));
    }
    let mut rows: Vec<BuryatRow> = Vec::new();
    for (i, line) in lines.enumerate() {
        let row = parse_row(name, i + 1, line)?;
        if let Some(previous) = rows.last()
            && day_number(row.date) != day_number(previous.date) + 1
        {
            return Err(format!(
                "DATE-STEP: {name}: row {}: {} is not one day after {}",
                i + 1,
                iso(row.date),
                iso(previous.date)
            ));
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Err(format!("ZERO-ROWS: {name}: the file holds no data row"));
    }
    Ok(rows)
}

/// The rows of `golden/buryat/<stem>.csv`; any refusal is a panic with its message.
pub fn load(stem: &str) -> Vec<BuryatRow> {
    let name = format!("{stem}.csv");
    parse(&name, &read(&name)).unwrap_or_else(|e| panic!("{e}"))
}

/// The step check (`FORKS.md` F28; 001 §6.3): between consecutive days the tshes
/// goes up by 1, or by 2 (one number skipped), or by 0 (a doubled day, never on
/// two days running), modulo 30; and `is_doubled` and `skipped` agree with the
/// step. The first row has neither flag (its predecessor is not in the source).
/// Codes: `FIRST-ROW`, `STEP`, `DOUBLED-TWICE`, `DOUBLED-FLAG`, `SKIPPED-VALUE`.
pub fn step_check(name: &str, rows: &[BuryatRow]) -> Result<(), String> {
    let Some(first) = rows.first() else {
        return Err(format!("ZERO-ROWS: {name}: no rows"));
    };
    if first.doubled || first.skipped.is_some() {
        return Err(format!(
            "FIRST-ROW: {name}: {}: the first row has a flag its predecessor cannot give",
            iso(first.date)
        ));
    }
    let mut previous_step = 1;
    for pair in rows.windows(2) {
        let [prev, row] = pair else { continue };
        let step = (i16::from(row.tshes) - i16::from(prev.tshes)).rem_euclid(30);
        let at = iso(row.date);
        let (want_doubled, want_skipped) = match step {
            0 => {
                if previous_step == 0 {
                    return Err(format!(
                        "DOUBLED-TWICE: {name}: {at}: tshes {} on three days running",
                        row.tshes
                    ));
                }
                (true, None)
            }
            1 => (false, None),
            2 => (false, Some(prev.tshes % 30 + 1)),
            other => {
                return Err(format!(
                    "STEP: {name}: {at}: tshes goes from {} to {} (step +{other}, modulo 30)",
                    prev.tshes, row.tshes
                ));
            }
        };
        if row.doubled != want_doubled {
            return Err(format!(
                "DOUBLED-FLAG: {name}: {at}: is_doubled is {}, the step +{step} gives {want_doubled}",
                row.doubled
            ));
        }
        if row.skipped != want_skipped {
            return Err(format!(
                "SKIPPED-VALUE: {name}: {at}: skipped is {:?}, the step +{step} gives {want_skipped:?}",
                row.skipped
            ));
        }
        previous_step = step;
    }
    Ok(())
}

/// The row indexes at which a lunation begins (`golden/buryat/README.md`, the
/// `month` and `is_leap_month` column): the first row, and every row whose tshes
/// is smaller than the previous row's.
pub fn lunation_starts(rows: &[BuryatRow]) -> Vec<usize> {
    let mut starts = vec![0];
    for (i, pair) in rows.windows(2).enumerate() {
        let [prev, row] = pair else { continue };
        if row.tshes < prev.tshes {
            starts.push(i + 1);
        }
    }
    starts
}

/// The month number a printed label implies in the seasonal order of
/// `golden/buryat/README.md` («Первый весенний» 1 to «Третий зимний» 12), or
/// `None` for «Дополнительный …» (the extra month: no number) and for text that
/// is not a label.
pub fn label_month(label: &str) -> Option<u8> {
    let mut words = label.split_whitespace();
    let ordinal = match words.next()? {
        "Первый" => 1,
        "Второй" => 2,
        "Третий" => 3,
        _ => return None,
    };
    let season = match words.next()? {
        "весенний" => 0,
        "летний" => 3,
        "осенний" => 6,
        "зимний" => 9,
        _ => return None,
    };
    Some(season + ordinal)
}

/// The printed label in a row's notes (`printed label «…»`), without the guillemets.
pub fn printed_label(notes: &str) -> Option<&str> {
    let (_, rest) = notes.split_once("printed label «")?;
    rest.split_once('»').map(|(label, _)| label)
}

/// The month rule of `golden/buryat/README.md` (`FORKS.md` F36) for one file.
///
/// An unlabelled file (`labelled` false) has `month` and `is_leap_month` empty on
/// every row (`UNLABELLED`). A labelled file: every row of a lunation carries the
/// same `(month, is_leap_month)` (`LUNATION-LABEL`); a regular lunation has
/// `is_leap_month` false and a month, an extra lunation `is_leap_month` true and an
/// empty month (`LEAP-MONTH-FORM`); the first row of every lunation prints the
/// label, and it implies that month (`LABEL-NOTE`); the regular months follow the
/// seasonal order, 12 then 1, the extra month skipped and never two in a row
/// (`LABEL-ORDER`).
pub fn month_rule(name: &str, rows: &[BuryatRow], labelled: bool) -> Result<(), String> {
    if !labelled {
        return match rows.iter().find(|r| r.month.is_some() || r.leap.is_some()) {
            Some(r) => Err(format!(
                "UNLABELLED: {name}: {}: month or is_leap_month is filled in a source that prints no label",
                iso(r.date)
            )),
            None => Ok(()),
        };
    }
    let starts = lunation_starts(rows);
    let mut last_regular: Option<u8> = None;
    let mut last_was_extra = false;
    for (k, &start) in starts.iter().enumerate() {
        let end = starts.get(k + 1).copied().unwrap_or(rows.len());
        let Some(first) = rows.get(start) else {
            continue;
        };
        let at = iso(first.date);
        let label_of = (first.month, first.leap);
        if rows
            .get(start..end)
            .is_some_and(|l| l.iter().any(|r| (r.month, r.leap) != label_of))
        {
            return Err(format!(
                "LUNATION-LABEL: {name}: lunation {} from {at}: its rows do not carry one label",
                k + 1
            ));
        }
        let extra = match label_of {
            (None, Some(true)) => true,
            (Some(_), Some(false)) => false,
            _ => {
                return Err(format!(
                    "LEAP-MONTH-FORM: {name}: lunation {} from {at}: (month, is_leap_month) is {label_of:?}",
                    k + 1
                ));
            }
        };
        let label = printed_label(&first.notes).ok_or_else(|| {
            format!(
                "LABEL-NOTE: {name}: lunation {} from {at}: the first row prints no label",
                k + 1
            )
        })?;
        let implied = label_month(label);
        let words_extra = label.starts_with("Дополнительный");
        if words_extra != extra || (!extra && implied != first.month) {
            return Err(format!(
                "LABEL-NOTE: {name}: lunation {} from {at}: label «{label}» implies {implied:?} (extra {words_extra}), the row says {label_of:?}",
                k + 1
            ));
        }
        if extra {
            if last_was_extra {
                return Err(format!(
                    "LABEL-ORDER: {name}: lunation {} from {at}: two extra months in a row",
                    k + 1
                ));
            }
        } else if let (Some(prev), Some(month)) = (last_regular, first.month)
            && month != prev % 12 + 1
        {
            return Err(format!(
                "LABEL-ORDER: {name}: lunation {} from {at}: month {month} follows month {prev}",
                k + 1
            ));
        }
        if !extra {
            last_regular = first.month;
        }
        last_was_extra = extra;
    }
    Ok(())
}

/// The meta check: `key: value` per line, exactly [`META_KEYS`] in that order,
/// each value non-empty, read line by line (a value may hold `: `).
/// Codes: `META-KEYS`, `META-EMPTY`.
pub fn meta_check(name: &str, text: &str) -> Result<(), String> {
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() != META_KEYS.len() {
        return Err(format!(
            "META-KEYS: {name}: {} lines, not {}",
            lines.len(),
            META_KEYS.len()
        ));
    }
    for (line, key) in lines.iter().zip(META_KEYS) {
        let Some((found, value)) = line.split_once(':') else {
            return Err(format!(
                "META-KEYS: {name}: a line without a key: {line:.40}"
            ));
        };
        if found != key {
            return Err(format!(
                "META-KEYS: {name}: expected key {key}, found {found}"
            ));
        }
        if value.trim().is_empty() {
            return Err(format!("META-EMPTY: {name}: the value of {key} is empty"));
        }
    }
    Ok(())
}

/// The names in `golden/buryat/` that end in `.csv`, sorted.
pub fn csv_names() -> Vec<String> {
    names_ending(".csv")
}

/// The names in `golden/buryat/` that end in `.meta.yaml`, sorted.
pub fn meta_names() -> Vec<String> {
    names_ending(".meta.yaml")
}

fn names_ending(suffix: &str) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir())
        .unwrap_or_else(|e| panic!("cannot list golden/buryat/: {e}"))
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.ends_with(suffix))
        .collect();
    names.sort();
    names
}
