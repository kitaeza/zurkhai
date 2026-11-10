//! Three independently stated constant sets must give one calendar (spec 005
//! Item 4; METHOD rule 35). Janson (arXiv:1401.6285v1) gives complete Phugpa
//! constants for the epochs E806 (§7, eqs. (7.2)–(7.14), pp. 21–23), E1927 and
//! E1987 (§5, eqs. (5.4)–(5.6), p. 14; §7, Remarks 9–11, eqs. (7.9), (7.10),
//! (7.15), (7.16), pp. 22–23), and states that "any epoch will give the same
//! calendar" (§5, p. 14). The engine's default is E806; if any of the many
//! transcribed constants of E1927 or E1987 were wrong, or the E806 ones, the
//! three would disagree on some day. This is a check that does not share a
//! blind spot with the round trip: a wrong constant shared by both directions
//! of one epoch still shows against the other two.

mod common;

use common::{all_days, month_key, months_of};
use std::collections::HashSet;
use tshes::phugpa::{self, Epoch};
use tshes::{Jdn, ZDate};

fn jdn(day: i64) -> Jdn {
    Jdn::new(day).unwrap()
}

#[test]
fn e806_e1927_and_e1987_give_the_same_zdate_on_every_day_of_the_range() {
    let days = all_days();
    for day in &days {
        for (name, epoch) in [
            ("E806", Epoch::E806),
            ("E1927", Epoch::E1927),
            ("E1987", Epoch::E1987),
        ] {
            assert_eq!(
                phugpa::to_zdate(&epoch, jdn(day.jdn)),
                Ok(day.z),
                "{name} at {:?}",
                day.date
            );
        }
    }
}

#[test]
fn the_other_epochs_invert_their_own_mapping_too() {
    // Each epoch's `to_jdn` returns the day, for every day of the range.
    for day in all_days() {
        for (name, epoch) in [("E1927", Epoch::E1927), ("E1987", Epoch::E1987)] {
            assert_eq!(
                phugpa::to_jdn(&epoch, &day.z).map(Jdn::get),
                Ok(day.jdn),
                "{name} at {:?}",
                day.date
            );
        }
    }
}

/// `(Y0, β*, α as numerator over denominator)` per epoch: Y0 and β* from
/// (5.4)–(5.6), p. 14; α from (C.30)–(C.32), p. 62.
const EPOCHS: [(&str, i64, i64, i64, i64); 3] = [
    ("E806", 806, 61, 1832, 1005),
    ("E1927", 1927, 55, 1922, 1005),
    ("E1987", 1987, 0, 41, 15),
];

#[test]
fn leap_rule_5_24_holds_for_every_year_and_month_for_every_epoch() {
    // Janson §5.4, eq. (5.24), p. 17: there is a leap month M in year Y iff
    // 24(Y − Y0) + 2M ≡ β or β + 1 (mod 65), with β = 184 − β* (eq. (5.15),
    // p. 17). Compared with the leap months the engine's day table contains,
    // for every year 1899..=2100 and every month number.
    let days = all_days();
    let observed: HashSet<(i64, u8)> = days
        .iter()
        .filter(|d| d.z.is_leap_month())
        .map(|d| (d.z.year().number(), d.z.month().get()))
        .collect();
    for (name, y0, beta_star, _, _) in EPOCHS {
        let beta = 184 - beta_star;
        for year in 1899..=2100_i64 {
            for month in 1..=12_u8 {
                let residue = (24 * (year - y0) + 2 * i64::from(month)).rem_euclid(65);
                let leap_by_rule =
                    residue == beta.rem_euclid(65) || residue == (beta + 1).rem_euclid(65);
                assert_eq!(
                    observed.contains(&(year, month)),
                    leap_by_rule,
                    "{name}: leap month {month} of {year}"
                );
            }
        }
    }
}

#[test]
fn the_true_month_count_is_continuous_by_5_10_and_by_the_appendix_c_formula() {
    // Along the engine's month sequence the true month count rises by exactly 1
    // per month ("a continuous count of months", §5, p. 15), whether computed by
    // eq. (5.10), p. 16 (n = ⌊(67M* + β* + 17)/65⌋ − [ℓ], M* = 12(Y − Y0) + M − 3)
    // or by Appendix C, eq. (C.22), p. 60 (n = ⌊(12(Y − Y0) + M − α)/(12 s1)⌋ −
    // [ℓ], 12 s1 = 65/67), and the two agree (§C.1, p. 60: "it is easily verified
    // directly that this is equivalent to (5.10)").
    let days = all_days();
    let months = months_of(&days);
    for (name, y0, beta_star, alpha_num, alpha_den) in EPOCHS {
        let mut previous: Option<(i64, i64)> = None;
        for month in &months {
            let (year, number, leap) = month.key;
            let leap_flag = i64::from(leap);
            let solar = 12 * (year - y0) + i64::from(number) - 3;
            let by_5_10 = (67 * solar + beta_star + 17).div_euclid(65) - leap_flag;
            // (C.22): ⌊(12(Y − Y0) + M − α) · 67/65⌋ with α = alpha_num/alpha_den
            let scaled = (12 * (year - y0) + i64::from(number)) * alpha_den - alpha_num;
            let by_c22 = (scaled * 67).div_euclid(65 * alpha_den) - leap_flag;
            assert_eq!(
                by_5_10, by_c22,
                "{name}: (5.10) vs (C.22) at {:?}",
                month.key
            );
            if let Some((count_5_10, count_c22)) = previous {
                assert_eq!(
                    by_5_10,
                    count_5_10 + 1,
                    "{name}: (5.10) not continuous at {:?}",
                    month.key
                );
                assert_eq!(
                    by_c22,
                    count_c22 + 1,
                    "{name}: (C.22) not continuous at {:?}",
                    month.key
                );
            }
            previous = Some((by_5_10, by_c22));
        }
    }
}

#[test]
fn the_three_epochs_were_compared_on_every_shape() {
    // METHOD rule 39: the comparison above reached leap months, skipped and
    // doubled tshes and both range edges.
    let days = all_days();
    let leap: HashSet<_> = days
        .iter()
        .filter(|d| d.z.is_leap_month())
        .map(|d| month_key(&d.z))
        .collect();
    let doubled = days.iter().filter(|d| d.z.is_doubled()).count();
    let skipped = days
        .iter()
        .filter(|d| d.z.skipped_before().is_some())
        .count();
    assert!(leap.len() > 70 && doubled > 1000 && skipped > 1000);
    let _: Option<ZDate> = days.first().map(|d| d.z);
}
