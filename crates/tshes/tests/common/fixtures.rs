//! Reading the reference fixtures in `golden/reference/` (spec 006).
//!
//! std only, no dependency (002 §4.9). Every loader fails, rather than
//! returns nothing, when a file is missing, its header is not the expected
//! one, a line has a carriage return or the wrong number of fields, or the file
//! holds no row (002 §6: a fixture test that loads zero rows fails).
//!
//! The day arithmetic here is the TEST side's own (`day_number`), anchored on
//! two published values; it never calls the engine, so a check built on it is
//! independent of the engine (METHOD rule 35).
#![allow(dead_code)]

use std::fs;

/// The path of a file under `golden/reference/`.
pub fn path(name: &str) -> String {
    format!(
        "{}/../../golden/reference/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

/// The data rows of a CSV fixture, each split on commas. The header must equal
/// `header` exactly; at least one data row must follow.
pub fn load(name: &str, header: &[&str]) -> Vec<Vec<String>> {
    let text = fs::read_to_string(path(name)).unwrap_or_else(|e| panic!("cannot read {name}: {e}"));
    assert!(
        !text.contains('\r'),
        "{name}: carriage return (LF endings only)"
    );
    assert!(text.ends_with('\n'), "{name}: no final newline");
    let mut lines = text.lines();
    let first = lines.next().unwrap_or("");
    assert_eq!(first, header.join(","), "{name}: header");
    let rows: Vec<Vec<String>> = lines
        .map(|line| line.split(',').map(str::to_owned).collect())
        .collect();
    assert!(!rows.is_empty(), "{name}: the fixture loaded zero rows");
    for (i, row) in rows.iter().enumerate() {
        assert_eq!(
            row.len(),
            header.len(),
            "{name}: data row {} has the wrong field count",
            i + 1
        );
    }
    rows
}

/// A civil date as `(year, month, day)`.
pub type Ymd = (i64, u8, u8);

/// `YYYY-MM-DD` to `(year, month, day)`.
pub fn parse_iso(text: &str) -> Ymd {
    let mut parts = text.split('-');
    let mut next = |what: &str| parts.next().unwrap_or_else(|| panic!("{text}: no {what}"));
    let year = next("year")
        .parse()
        .unwrap_or_else(|_| panic!("{text}: year"));
    let month = next("month")
        .parse()
        .unwrap_or_else(|_| panic!("{text}: month"));
    let day = next("day")
        .parse()
        .unwrap_or_else(|_| panic!("{text}: day"));
    assert!(parts.next().is_none(), "{text}: more than three parts");
    (year, month, day)
}

/// `(year, month, day)` as `YYYY-MM-DD`.
pub fn iso(date: Ymd) -> String {
    format!("{:04}-{:02}-{:02}", date.0, date.1, date.2)
}

/// The number of days from 1970-01-01 to a proleptic Gregorian date, by the
/// civil-from-days algorithm of H. Hinnant (public domain), written here and
/// not taken from the engine. Anchors, asserted by `day_number_matches_the_published_anchors`
/// in every test crate that uses it: 1970-01-01 is day 0 and, with the Julian day
/// number 2440588 of 1970-01-01, 1 January 2007 has JD 2454102 (Janson v1, p. 6).
pub fn day_number(date: Ymd) -> i64 {
    let (year, month, day) = (date.0, i64::from(date.1), i64::from(date.2));
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let month_from_march = (month + 9).rem_euclid(12);
    let day_of_year = (153 * month_from_march + 2).div_euclid(5) + day - 1;
    let day_of_era =
        year_of_era * 365 + year_of_era.div_euclid(4) - year_of_era.div_euclid(100) + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// The JD of 1970-01-01.
pub const JD_OF_1970_01_01: i64 = 2_440_588;

/// One row of the T4 fixture `tdc-97d4a00-phugpa-1900-2100.csv`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct T4Row {
    pub date: Ymd,
    pub year: i64,
    pub month: u8,
    pub leap: bool,
    pub tshes: u8,
}

/// The file name of the T4 fixture.
pub const T4_FILE: &str = "tdc-97d4a00-phugpa-1900-2100.csv";

/// Every row of the T4 fixture, in file order.
pub fn t4_rows() -> Vec<T4Row> {
    let header = [
        "gregorian_date",
        "tibetan_year",
        "month",
        "is_leap_month",
        "tshes",
    ];
    load(T4_FILE, &header)
        .iter()
        .map(|f| T4Row {
            date: parse_iso(&f[0]),
            year: f[1].parse().expect("tibetan_year"),
            month: f[2].parse().expect("month"),
            leap: match f[3].as_str() {
                "true" => true,
                "false" => false,
                other => panic!("is_leap_month is neither true nor false: {other}"),
            },
            tshes: f[4].parse().expect("tshes"),
        })
        .collect()
}
