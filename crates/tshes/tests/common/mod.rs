//! Shared helpers for the integration tests: the supported range, enumerated
//! day by day through the public API only.
#![allow(dead_code)] // each test crate uses a different subset

use tshes::{GregorianDate, Jdn, System, ZDate, to_zdate};

pub mod fixtures;

/// The JD of 1900-01-01, from the Gregorian conversion (not typed twice).
pub fn first_jdn() -> i64 {
    GregorianDate::new(1900, 1, 1)
        .expect("1900-01-01")
        .to_jdn()
        .get()
}

/// The JD of 2100-12-31.
pub fn last_jdn() -> i64 {
    GregorianDate::new(2100, 12, 31)
        .expect("2100-12-31")
        .to_jdn()
        .get()
}

/// One day of the supported range with its Phugpa `ZDate`.
#[derive(Debug, Clone, Copy)]
pub struct Day {
    pub jdn: i64,
    pub date: GregorianDate,
    pub z: ZDate,
}

/// Every day from 1900-01-01 to 2100-12-31, in order, each mapped to its Phugpa `ZDate`.
pub fn all_days() -> Vec<Day> {
    all_days_of(System::Phugpa)
}

/// Every day from 1900-01-01 to 2100-12-31, in order, each mapped to its `ZDate` under `system`.
pub fn all_days_of(system: System) -> Vec<Day> {
    (first_jdn()..=last_jdn())
        .map(|jdn| {
            let date = Jdn::new(jdn).expect("jdn").to_gregorian().expect("date");
            let z = to_zdate(date, system).expect("every supported day maps");
            Day { jdn, date, z }
        })
        .collect()
}

/// A month as the triple `(Tibetan year, month number, is_leap_month)`.
pub type MonthKey = (i64, u8, bool);

pub fn month_key(z: &ZDate) -> MonthKey {
    (z.year().number(), z.month().get(), z.is_leap_month())
}

/// A month of the range: its key and the half-open index range of its days.
pub struct Month {
    pub key: MonthKey,
    pub start: usize,
    pub end: usize,
}

pub fn months_of(days: &[Day]) -> Vec<Month> {
    let mut months: Vec<Month> = Vec::new();
    for (index, day) in days.iter().enumerate() {
        let key = month_key(&day.z);
        match months.last_mut() {
            Some(last) if last.key == key => last.end = index + 1,
            _ => months.push(Month {
                key,
                start: index,
                end: index + 1,
            }),
        }
    }
    months
}
