//! Gregorian ↔ JDN (spec 005 Item 3). Janson (arXiv:1401.6285v1) states no
//! conversion formula, so the engine derives it from the Gregorian leap-year
//! rule (module `civil`) and these tests check it against what the paper does
//! state, and against a day-by-day count.

mod common;

use tshes::{Error, GregorianDate, Jdn};

#[test]
fn janson_anchor_1_january_2007_is_jd_2454102() {
    // Janson (arXiv:1401.6285v1) §2, p. 6: "1 January 2007 has JD 2454102".
    let date = GregorianDate::new(2007, 1, 1).unwrap();
    assert_eq!(date.to_jdn().get(), 2_454_102);
}

#[test]
fn janson_anchor_rd_1_is_jd_1721426() {
    // Janson §2, p. 5, footnote 8: RD 1 is 1 January 1 (Gregorian), JD 1721426.
    let date = GregorianDate::new(1, 1, 1).unwrap();
    assert_eq!(date.to_jdn().get(), 1_721_426);
}

#[test]
fn every_day_of_the_range_matches_a_day_by_day_count() {
    // An independent counter: start at 1900-01-01 (JD from the closed form is
    // checked against the anchor above) and advance one day at a time with the
    // month lengths written out, never with the closed form.
    let mut year = 1900_i64;
    let mut month = 1_u8;
    let mut day = 1_u8;
    let mut count = 0_i64;
    let first = common::first_jdn();
    let mut jdn = first;
    while !(year == 2101 && month == 1 && day == 1) {
        let date = GregorianDate::new(year, month, day).unwrap();
        assert_eq!(date.to_jdn().get(), jdn, "JD of {year}-{month:02}-{day:02}");
        let back = Jdn::new(jdn).unwrap().to_gregorian().unwrap();
        assert_eq!(back, date, "date of JD {jdn}");
        // advance
        let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
        let length = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            _ => {
                if leap {
                    29
                } else {
                    28
                }
            }
        };
        if day < length {
            day += 1;
        } else {
            day = 1;
            if month == 12 {
                month = 1;
                year += 1;
            } else {
                month += 1;
            }
        }
        jdn += 1;
        count += 1;
    }
    assert_eq!(
        count, 73_414,
        "days from 1900-01-01 to 2100-12-31 inclusive"
    );
    assert_eq!(jdn - 1, common::last_jdn());
}

#[test]
fn supported_range_bounds_are_exactly_1900_01_01_and_2100_12_31() {
    let before = Jdn::new(common::first_jdn() - 1).unwrap();
    let first = Jdn::new(common::first_jdn()).unwrap();
    let last = Jdn::new(common::last_jdn()).unwrap();
    let after = Jdn::new(common::last_jdn() + 1).unwrap();
    assert!(!before.is_supported());
    assert!(first.is_supported());
    assert!(last.is_supported());
    assert!(!after.is_supported());
}

#[test]
fn leap_year_rule_at_the_century_years() {
    // 1900 and 2100 are not leap years, 2000 is (Gregorian rule).
    assert_eq!(GregorianDate::new(1900, 2, 29), Err(Error::InvalidField));
    assert_eq!(GregorianDate::new(2100, 2, 29), Err(Error::InvalidField));
    assert!(GregorianDate::new(2000, 2, 29).is_ok());
    assert!(GregorianDate::new(2024, 2, 29).is_ok());
    assert_eq!(GregorianDate::new(2023, 2, 29), Err(Error::InvalidField));
}

#[test]
fn invalid_fields_are_refused() {
    assert_eq!(GregorianDate::new(2000, 0, 1), Err(Error::InvalidField));
    assert_eq!(GregorianDate::new(2000, 13, 1), Err(Error::InvalidField));
    assert_eq!(GregorianDate::new(2000, 1, 0), Err(Error::InvalidField));
    assert_eq!(GregorianDate::new(2000, 4, 31), Err(Error::InvalidField));
}

#[test]
fn round_trip_at_the_extremes_of_the_domain() {
    // 002 §4.3 and METHOD rule 34: the i64 headroom is tested at the extreme
    // of the domain, not only inside the supported range.
    for year in [
        -1_000_000_000_i64,
        -400,
        -1,
        0,
        1,
        1582,
        999_999_999,
        1_000_000_000,
    ] {
        for (month, day) in [(1_u8, 1_u8), (2, 28), (3, 1), (12, 31)] {
            let date = GregorianDate::new(year, month, day).unwrap();
            let jdn = date.to_jdn();
            assert_eq!(jdn.to_gregorian().unwrap(), date, "year {year}");
            assert_eq!(Jdn::new(jdn.get()).unwrap(), jdn);
        }
    }
    assert_eq!(
        GregorianDate::new(1_000_000_001, 1, 1),
        Err(Error::OutOfRange)
    );
    assert_eq!(
        GregorianDate::new(-1_000_000_001, 1, 1),
        Err(Error::OutOfRange)
    );
    // The Jdn domain is exactly the days of the years -10^9..=10^9: its
    // endpoints are the JDs of the first and last of those days, and the
    // conversions are total on it.
    let last = GregorianDate::new(1_000_000_000, 12, 31).unwrap();
    let first = GregorianDate::new(-1_000_000_000, 1, 1).unwrap();
    assert_eq!(last.to_jdn().get(), 365_244_221_425);
    assert_eq!(first.to_jdn().get(), -365_240_778_940);
    assert_eq!(
        Jdn::new(365_244_221_425).unwrap().to_gregorian().unwrap(),
        last
    );
    assert_eq!(
        Jdn::new(-365_240_778_940).unwrap().to_gregorian().unwrap(),
        first
    );
    assert_eq!(Jdn::new(365_244_221_426), Err(Error::OutOfRange));
    assert_eq!(Jdn::new(-365_240_778_941), Err(Error::OutOfRange));
}

#[test]
fn consecutive_days_differ_by_one_across_leap_and_century_boundaries() {
    // Day-to-day continuity of the closed form well beyond the range (the
    // 400-year cycle, the century and the 4-year exceptions).
    let start = GregorianDate::new(1500, 1, 1).unwrap().to_jdn().get();
    let end = GregorianDate::new(2600, 12, 31).unwrap().to_jdn().get();
    let mut previous = Jdn::new(start).unwrap().to_gregorian().unwrap();
    for jdn in (start + 1)..=end {
        let date = Jdn::new(jdn).unwrap().to_gregorian().unwrap();
        assert_eq!(date.to_jdn().get(), jdn);
        assert!(date > previous);
        previous = date;
    }
}
