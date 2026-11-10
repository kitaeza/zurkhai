//! Janson's worked examples, as an outside check (spec 005 Item 5).
//!
//! Janson (arXiv:1401.6285v1) has no example that carries a date through the
//! lunar-day formulas (7.1)–(7.22) with its intermediate values; the sections
//! and pages searched are listed in `docs/specs/005-phugpa-core.md`. What it
//! does give, and what is checked here through the public API:
//!
//! * the year-naming example of AD 2007 (§4, pp. 8–9), with its intermediate
//!   values (cycle, position, Chinese-cycle numbers, element, gender, animal);
//! * Tibetan dates of named Gregorian days: the two dates on page 1, the epoch
//!   days of §7 Remark 16 and p. 27, and the leap month 1 of 2000 (§4 p. 9,
//!   Remark 17 and footnote 28, p. 28).
//!
//! Each test cites its page. Every example has a seeded control: the same
//! example with one value deliberately wrong, asserted to fail (002 §6,
//! METHOD rule 31). The weekday check uses Janson's eq. (9.1) and Table 5
//! (p. 28) in test code only: the weekday is a 001 §5.2 attribute that this
//! round does not implement in the engine.

use tshes::{Animal, Element, Gender, GregorianDate, System, ZYear, to_gregorian, to_zdate};

// ---------------------------------------------------------------- year naming

struct YearExample {
    year: i64,
    cycle: i64,
    position: u8,
    chinese_position: i64,
    element: Element,
    gender: Gender,
    animal: Animal,
}

/// Whether the engine reproduces every value of the example, and the example
/// is consistent with the Chinese-cycle number of §4 eq. (4.4), p. 9.
fn year_example_holds(e: &YearExample) -> bool {
    // (4.4), p. 9: the number of year Y in the Chinese 60-year cycle is (Y − 3) amod 60.
    let chinese = (e.year - 3 - 1).rem_euclid(60) + 1;
    let Ok(year) = ZYear::from_year(e.year) else {
        return false;
    };
    chinese == e.chinese_position
        && year.rabjung_cycle().get() == e.cycle
        && year.year_in_cycle().get() == e.position
        && year.element() == e.element
        && year.gender() == e.gender
        && year.animal() == e.animal
        // the inverse: Y = 60m + n + 966 (4.1)
        && ZYear::new(e.cycle, e.position).is_ok_and(|built| built == year)
        && year.number() == e.year
}

#[test]
fn ad_2007_is_the_21st_year_of_the_17th_cycle_and_fire_female_pig() {
    // §4, pp. 8–9: "AD 2007 is the 21st year in the 17th Prabhava cycle, which
    // began in 1987" (eqs. (4.1)–(4.3)); "AD 2007 is the 24th year in the Chinese
    // cycle (21st in the Indian) and is thus Fire–female–Pig or simply Fire–Pig".
    let example = YearExample {
        year: 2007,
        cycle: 17,
        position: 21,
        chinese_position: 24,
        element: Element::Fire,
        gender: Gender::Female,
        animal: Animal::Pig,
    };
    assert!(year_example_holds(&example));
}

#[test]
fn seeded_control_ad_2007_with_one_value_wrong_fails() {
    let good = || YearExample {
        year: 2007,
        cycle: 17,
        position: 21,
        chinese_position: 24,
        element: Element::Fire,
        gender: Gender::Female,
        animal: Animal::Pig,
    };
    assert!(!year_example_holds(&YearExample {
        element: Element::Water,
        ..good()
    }));
    assert!(!year_example_holds(&YearExample {
        animal: Animal::Dog,
        ..good()
    }));
    assert!(!year_example_holds(&YearExample {
        position: 22,
        ..good()
    }));
    assert!(!year_example_holds(&YearExample {
        cycle: 16,
        ..good()
    }));
    assert!(!year_example_holds(&YearExample {
        gender: Gender::Male,
        ..good()
    }));
    assert!(!year_example_holds(&YearExample {
        chinese_position: 25,
        ..good()
    }));
}

#[test]
fn the_17th_cycle_began_in_1987_and_its_first_year_is_the_fourth_of_the_chinese_cycle() {
    // §4, p. 8: "the 17th Prabhava cycle, which began in 1987"; p. 9: "the first
    // year (Prabhava, rab byung) in the Indian cycle is the fourth year
    // (Fire–female–Rabbit) in the Chinese cycle"; "The last cycle thus started 1984".
    let example = YearExample {
        year: 1987,
        cycle: 17,
        position: 1,
        chinese_position: 4,
        element: Element::Fire,
        gender: Gender::Female,
        animal: Animal::Rabbit,
    };
    assert!(year_example_holds(&example));
    // the Chinese cycle began in 1984 (position 1: Wood–male–Mouse, Table 10 p. 56 line 58 / Tables 2, 3 p. 10)
    let chinese_start = YearExample {
        year: 1984,
        cycle: 16,
        position: 58,
        chinese_position: 1,
        element: Element::Wood,
        gender: Gender::Male,
        animal: Animal::Mouse,
    };
    assert!(year_example_holds(&chinese_start));
    // the seeded control: the same example one year later is not the first year
    assert!(!year_example_holds(&YearExample {
        year: 1988,
        ..example
    }));
}

// ----------------------------------------------------------- dates and weekdays

/// Janson §9, eq. (9.1), p. 28: `day_of_week = (JD + 2) mod 7`, numbered as in
/// Table 5, p. 28: 0 Saturday, 1 Sunday, 2 Monday, 3 Tuesday, 4 Wednesday,
/// 5 Thursday, 6 Friday.
fn weekday(jdn: i64) -> &'static str {
    match (jdn + 2).rem_euclid(7) {
        0 => "Saturday",
        1 => "Sunday",
        2 => "Monday",
        3 => "Tuesday",
        4 => "Wednesday",
        5 => "Thursday",
        _ => "Friday",
    }
}

struct DateExample {
    gregorian: (i64, u8, u8),
    tibetan_year: i64,
    month: u8,
    leap: bool,
    tshes: u8,
}

/// Whether the engine maps the Gregorian day to the stated Tibetan date, and
/// back.
fn date_example_holds(e: &DateExample) -> bool {
    let Ok(date) = GregorianDate::new(e.gregorian.0, e.gregorian.1, e.gregorian.2) else {
        return false;
    };
    let Ok(z) = to_zdate(date, System::Phugpa) else {
        return false;
    };
    z.year().number() == e.tibetan_year
        && z.month().get() == e.month
        && z.is_leap_month() == e.leap
        && z.tshes().get() == e.tshes
        && !z.is_doubled()
        && to_gregorian(z) == Ok(date)
}

fn jdn_of(year: i64, month: u8, day: u8) -> i64 {
    GregorianDate::new(year, month, day).unwrap().to_jdn().get()
}

#[test]
fn page_1_water_snake_date_8_january_2014_is_wednesday_8_month_11() {
    // Page 1 (the date line under the abstract): "revised 8 January, 2014
    // (Wednesday 8, month 11, Water–Snake year)". The Tibetan year starts in
    // 2013 (§4, p. 8: numbered by the Gregorian year in which it starts).
    let example = DateExample {
        gregorian: (2014, 1, 8),
        tibetan_year: 2013,
        month: 11,
        leap: false,
        tshes: 8,
    };
    assert!(date_example_holds(&example));
    assert_eq!(weekday(jdn_of(2014, 1, 8)), "Wednesday");
    let year = ZYear::from_year(2013).unwrap();
    assert_eq!(
        (year.element(), year.animal()),
        (Element::Water, Animal::Snake)
    );
    // the seeded control: tshes 9 is wrong
    assert!(!date_example_holds(&DateExample {
        tshes: 9,
        ..example
    }));
    assert!(!date_example_holds(&DateExample {
        month: 12,
        ..example
    }));
}

#[test]
fn page_1_fire_pig_date_31_december_2007_is_the_23rd_of_month_11() {
    // Page 1: "Date: 31 December, 2007 (Sunday 23, month 11, Fire–Pig year)".
    let example = DateExample {
        gregorian: (2007, 12, 31),
        tibetan_year: 2007,
        month: 11,
        leap: false,
        tshes: 23,
    };
    assert!(date_example_holds(&example));
    let year = ZYear::from_year(2007).unwrap();
    assert_eq!(
        (year.element(), year.animal()),
        (Element::Fire, Animal::Pig)
    );
    // the seeded control: the 24th is wrong
    assert!(!date_example_holds(&DateExample {
        tshes: 24,
        ..example
    }));
}

#[test]
fn header_date_2007_weekday_word_disagrees_with_eq_9_1_in_the_paper() {
    // A known gap in the SOURCE, not in the engine (reported in the spec 005
    // report, not absorbed): page 1 gives the weekday of "31 December, 2007" as
    // "Sunday", but eq. (9.1), p. 28, with Table 5, p. 28, gives Monday for JD
    // 2454466, and 31 December 2007 was a Monday. The same page's 8 January 2014
    // is a Wednesday by (9.1), as stated. The date triple (23, month 11,
    // Fire–Pig) agrees with the engine; only the weekday word does not.
    assert_eq!(jdn_of(2007, 12, 31), 2_454_466);
    assert_eq!(weekday(2_454_466), "Monday");
    assert_ne!(weekday(2_454_466), "Sunday");
}

#[test]
fn losar_2000_was_the_first_day_of_leap_month_1_a_sunday() {
    // §4, p. 9: "in case there is a leap month 1, the year begins with the leap
    // month, which precedes the regular month 1. (This happened in 2000.)"
    // Footnote 28, p. 28: "Losar was celebrated on Sunday, February 6th
    // (2000), which was the first day of leap month 1."
    let example = DateExample {
        gregorian: (2000, 2, 6),
        tibetan_year: 2000,
        month: 1,
        leap: true,
        tshes: 1,
    };
    assert!(date_example_holds(&example));
    assert_eq!(weekday(jdn_of(2000, 2, 6)), "Sunday");
    // the seeded controls: the regular month 1, and the next day
    assert!(!date_example_holds(&DateExample {
        leap: false,
        ..example
    }));
    assert!(!date_example_holds(&DateExample {
        gregorian: (2000, 2, 7),
        ..example
    }));
}

#[test]
fn the_epoch_days_of_1927_and_1987_have_the_dates_of_p_27() {
    // §7, Remark 16 and p. 27: the epoch is the day floor(m0); "the Tibetan
    // dates are 29/1 806, 29/1 1927 and 1/3 1987". p. 68, (D.2): the E1927
    // epoch is JD 2424972 (Friday, 1 April 1927). The E1987 epoch is JD 2446914
    // (Remark 9, p. 22: m0 + 14609·m1 = 2446914 + 135/707). (29/1 806 lies
    // outside the supported range; the crate's own unit tests check it.)
    assert_eq!(jdn_of(1927, 4, 1), 2_424_972);
    assert_eq!(weekday(2_424_972), "Friday");
    assert_eq!(jdn_of(1987, 4, 28), 2_446_914);
    let e1927 = DateExample {
        gregorian: (1927, 4, 1),
        tibetan_year: 1927,
        month: 1,
        leap: false,
        tshes: 29,
    };
    let e1987 = DateExample {
        gregorian: (1987, 4, 28),
        tibetan_year: 1987,
        month: 3,
        leap: false,
        tshes: 1,
    };
    assert!(date_example_holds(&e1927));
    assert!(date_example_holds(&e1987));
    // the seeded controls
    assert!(!date_example_holds(&DateExample { tshes: 30, ..e1927 }));
    assert!(!date_example_holds(&DateExample { month: 2, ..e1987 }));
}
