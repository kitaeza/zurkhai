//! P7 and arbitrary `ZDate` field values for the Tsurphu and Genden versions
//! (spec 008 Item 4a; 001 §6.4 P7; 002 §6; METHOD rules 41, 42). The same
//! properties, strategies and model as `p7_proptest.rs` has for Phugpa, written
//! once over a `System` parameter.
//!
//! **Behaviour outside the range is defined, and asserted here** (not merely
//! "never panics"):
//!
//! * `to_zdate` on a day outside 1900-01-01..=2100-12-31 is exactly
//!   `Err(OutOfRange)`, down to the ends of the Gregorian domain (years ±10^9);
//! * `to_gregorian` on a `ZDate` whose Tibetan year is outside 1899..=2100 is
//!   exactly `Err(OutOfRange)`, whatever the other fields are;
//! * for years inside, an absent leap month is `Err(InvalidField)`, a skipped
//!   tshes is `Err(SkippedDay)`, a wrong doubled flag or `skipped_before` is
//!   `Err(InvalidField)`, and an `Ok(d)` is always inside the range with
//!   `to_zdate(d) == z` (P2, second direction);
//! * a `ZDate` is read under the system it carries: its answer is that system's.
//!
//! The strategies draw at the boundaries (range edges, month ends, skipped and
//! doubled days, leap months, 12 → 1 rollovers, cycle boundaries) as a strategy of
//! their own beside the uniform draw (METHOD rule 41), and the model they are
//! compared with is the forward table built from the days, not `to_gregorian`. A
//! seed proptest writes on a failure is committed with its test (002 §6).

mod common;

use common::{MonthKey, all_days_of, first_jdn, last_jdn, month_key, months_of};
use proptest::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;
use tshes::{Error, Jdn, MonthNo, System, Tshes, ZDate, ZYear, to_gregorian, to_zdate};

/// The days that carry a tshes in a month, as `(doubled, skipped_before, JD)`.
type Entries = Vec<(bool, Option<u8>, i64)>;

/// The forward model: what the days of the range say.
struct Model {
    table: HashMap<(MonthKey, u8), Entries>,
    /// Months wholly inside the range (not the two partial months at the edges).
    interior: HashSet<MonthKey>,
    /// Every month that has a day in the range.
    present: HashSet<MonthKey>,
    /// Days at the structure's internal boundaries, for the boundary strategy.
    boundary_days: Vec<i64>,
    shapes: BoundaryShapes,
}

#[derive(Default, Debug)]
struct BoundaryShapes {
    month_starts: usize,
    month_ends: usize,
    skipped: usize,
    doubled: usize,
    leap_month_days: usize,
    rollovers: usize,
    cycle_boundaries: usize,
    range_edges: usize,
}

fn build_model(system: System) -> Model {
    let days = all_days_of(system);
    let months = months_of(&days);
    let mut table: HashMap<(MonthKey, u8), Entries> = HashMap::new();
    for day in &days {
        table
            .entry((month_key(&day.z), day.z.tshes().get()))
            .or_default()
            .push((
                day.z.is_doubled(),
                day.z.skipped_before().map(Tshes::get),
                day.jdn,
            ));
    }
    let interior = months[1..months.len() - 1].iter().map(|m| m.key).collect();
    let present = months.iter().map(|m| m.key).collect();
    let mut shapes = BoundaryShapes::default();
    let mut boundary: HashSet<i64> = HashSet::new();
    let mut add = |jdn: i64, counter: &mut usize| {
        for neighbour in [jdn - 1, jdn, jdn + 1] {
            boundary.insert(neighbour);
        }
        *counter += 1;
    };
    for month in &months {
        add(days[month.start].jdn, &mut shapes.month_starts);
        add(days[month.end - 1].jdn, &mut shapes.month_ends);
        if month.key.2 {
            add(days[month.start].jdn, &mut shapes.leap_month_days);
            add(days[month.end - 1].jdn, &mut shapes.leap_month_days);
        }
    }
    for pair in days.windows(2) {
        if pair[0].z.month().get() == 12 && pair[1].z.month().get() == 1 {
            add(pair[1].jdn, &mut shapes.rollovers);
        }
        if pair[0].z.year().rabjung_cycle() != pair[1].z.year().rabjung_cycle() {
            add(pair[1].jdn, &mut shapes.cycle_boundaries);
        }
    }
    for day in &days {
        if day.z.skipped_before().is_some() {
            add(day.jdn, &mut shapes.skipped);
        }
        if day.z.is_doubled() {
            add(day.jdn, &mut shapes.doubled);
        }
    }
    for edge in [first_jdn(), last_jdn()] {
        for offset in -3..=3 {
            boundary.insert(edge + offset);
        }
        shapes.range_edges += 1;
    }
    let mut boundary_days: Vec<i64> = boundary.into_iter().collect();
    boundary_days.sort_unstable();
    Model {
        table,
        interior,
        present,
        boundary_days,
        shapes,
    }
}

fn model(system: System) -> &'static Model {
    static TSURPHU: OnceLock<Model> = OnceLock::new();
    static GENDEN: OnceLock<Model> = OnceLock::new();
    match system {
        System::Tsurphu => TSURPHU.get_or_init(|| build_model(System::Tsurphu)),
        _ => GENDEN.get_or_init(|| build_model(System::Genden)),
    }
}

/// Days: uniform around the range, at the boundaries, and across the whole domain.
fn jdn_strategy(system: System) -> impl Strategy<Value = i64> {
    prop_oneof![
        3 => (first_jdn() - 50_000)..=(last_jdn() + 50_000),
        3 => proptest::sample::select(model(system).boundary_days.clone()),
        1 => -365_240_778_940_i64..=365_244_221_425_i64,
    ]
}

fn cycle_strategy() -> impl Strategy<Value = i64> {
    prop_oneof![
        6 => 15_i64..=18,
        3 => 1_i64..=40,
        1 => -3_i64..=3,
        1 => 999_999_990_i64..=1_000_000_010,
        1 => any::<i64>(),
    ]
}

fn small_strategy(valid: std::ops::RangeInclusive<u8>, top: u8) -> impl Strategy<Value = u8> {
    prop_oneof![8 => valid.clone(), 1 => Just(0_u8), 1 => (*valid.end() + 1)..=top]
}

type RawFields = (i64, u8, u8, bool, u8, bool, Option<u8>);

fn raw_fields() -> impl Strategy<Value = RawFields> {
    (
        cycle_strategy(),
        small_strategy(1..=60, 64),
        small_strategy(1..=12, 14),
        any::<bool>(),
        small_strategy(1..=30, 32),
        any::<bool>(),
        proptest::option::of(small_strategy(1..=30, 32)),
    )
}

fn check_jdn(system: System, jdn: i64) -> Result<(), TestCaseError> {
    let date = Jdn::new(jdn).unwrap().to_gregorian().unwrap();
    let result = to_zdate(date, system);
    if (first_jdn()..=last_jdn()).contains(&jdn) {
        let z = result.unwrap();
        prop_assert_eq!(z.system(), system);
        prop_assert_eq!(to_gregorian(z), Ok(date));
        prop_assert_eq!(to_zdate(date, system), Ok(z));
    } else {
        prop_assert_eq!(result, Err(Error::OutOfRange));
    }
    Ok(())
}

/// Janson (arXiv:1401.6285v1) §5.4, eq. (5.24), p. 17, with the version's `Y₀`
/// and `β` (A.2, p. 39–40; A.3, p. 42–43): a leap month `M` exists in year `Y`
/// iff `24(Y − Y₀) + 2M ≡ β or β + 1 (mod 65)`.
fn leap_exists(system: System, year: i64, month: u8) -> bool {
    let (y0, beta) = if system == System::Tsurphu {
        (1732, 142_i64)
    } else {
        (1747, 172)
    };
    let residue = (24 * (year - y0) + 2 * i64::from(month)).rem_euclid(65);
    residue == beta.rem_euclid(65) || residue == (beta + 1).rem_euclid(65)
}

fn check_fields(system: System, fields: RawFields) -> Result<(), TestCaseError> {
    let (cycle, position, month, leap, tshes, doubled, skipped) = fields;
    // Only valid components reach `to_gregorian`; invalid ones are refused by the
    // constructors (`constructors_refuse_exactly_the_values_outside_their_range`).
    let (Ok(year), Ok(month_no), Ok(tshes_no)) = (
        ZYear::new(cycle, position),
        MonthNo::new(month),
        Tshes::new(tshes),
    ) else {
        return Ok(());
    };
    let skipped_no = match skipped {
        Some(s) => match Tshes::new(s) {
            Ok(t) => Some(t),
            Err(_) => return Ok(()),
        },
        None => None,
    };
    let z = ZDate::new(system, year, month_no, leap, tshes_no, doubled, skipped_no);
    let result = to_gregorian(z);
    let number = year.number();
    let key: MonthKey = (number, month, leap);
    let model = model(system);

    if !(1899..=2100).contains(&number) {
        prop_assert_eq!(result, Err(Error::OutOfRange));
        return Ok(());
    }
    let entries = model.table.get(&(key, tshes));
    let exact = entries.and_then(|e| e.iter().find(|x| x.0 == doubled && x.1 == skipped));
    if let Some(entry) = exact {
        prop_assert_eq!(
            result,
            Ok(Jdn::new(entry.2).unwrap().to_gregorian().unwrap())
        );
    } else if model.interior.contains(&key) {
        let expected = if entries.is_none() {
            Error::SkippedDay
        } else {
            Error::InvalidField
        };
        prop_assert_eq!(result, Err(expected));
    } else if !model.present.contains(&key) {
        if leap && !leap_exists(system, number, month) {
            // The month does not exist: refused first, exactly InvalidField.
            prop_assert_eq!(result, Err(Error::InvalidField));
        } else {
            // The month exists but has no day in the range, so never Ok; which error
            // depends on the documented order of checks in `phugpa::to_jdn`.
            prop_assert!(
                matches!(
                    result,
                    Err(Error::OutOfRange | Error::SkippedDay | Error::InvalidField)
                ),
                "unexpected {result:?} for a month outside the range"
            );
        }
    }
    // Whatever the outcome, an Ok is inside the range and round-trips (P2).
    if let Ok(date) = result {
        let jdn = date.to_jdn().get();
        prop_assert!((first_jdn()..=last_jdn()).contains(&jdn));
        prop_assert_eq!(to_zdate(date, system), Ok(z));
    }
    Ok(())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(3000))]

    #[test]
    fn tsurphu_to_zdate_is_exact_inside_the_range_and_out_of_range_outside_it(jdn in jdn_strategy(System::Tsurphu)) {
        check_jdn(System::Tsurphu, jdn)?;
    }

    #[test]
    fn genden_to_zdate_is_exact_inside_the_range_and_out_of_range_outside_it(jdn in jdn_strategy(System::Genden)) {
        check_jdn(System::Genden, jdn)?;
    }

    #[test]
    fn tsurphu_to_gregorian_is_classified_exactly_for_arbitrary_valid_fields(fields in raw_fields()) {
        check_fields(System::Tsurphu, fields)?;
    }

    #[test]
    fn genden_to_gregorian_is_classified_exactly_for_arbitrary_valid_fields(fields in raw_fields()) {
        check_fields(System::Genden, fields)?;
    }
}

#[test]
fn the_boundary_strategy_has_every_shape_the_properties_depend_on() {
    // METHOD rule 39: a strategy that never draws a shape passes vacuously.
    for system in [System::Tsurphu, System::Genden] {
        let model = model(system);
        let shapes = &model.shapes;
        assert!(shapes.month_starts > 2000, "{system:?} {shapes:?}");
        assert!(shapes.month_ends > 2000, "{system:?} {shapes:?}");
        assert!(shapes.skipped > 1000, "{system:?} {shapes:?}");
        assert!(shapes.doubled > 1000, "{system:?} {shapes:?}");
        assert!(shapes.leap_month_days >= 150, "{system:?} {shapes:?}");
        assert!(shapes.rollovers >= 200, "{system:?} {shapes:?}");
        assert_eq!(shapes.cycle_boundaries, 3, "{system:?} {shapes:?}");
        assert_eq!(shapes.range_edges, 2);
        assert!(model.boundary_days.contains(&first_jdn()));
        assert!(model.boundary_days.contains(&last_jdn()));
        assert!(model.boundary_days.contains(&(first_jdn() - 1)));
        assert!(model.boundary_days.contains(&(last_jdn() + 1)));
    }
}

#[test]
fn a_zdate_is_read_under_the_system_it_carries() {
    // The same fields under another system are another calendar's date: for a day on
    // which the two versions give different dates, each system's `ZDate` is accepted
    // only by its own system's inverse.
    let tsurphu = all_days_of(System::Tsurphu);
    let genden = all_days_of(System::Genden);
    let day = tsurphu
        .iter()
        .zip(&genden)
        .find(|(t, g)| t.z.tshes() != g.z.tshes())
        .expect("the versions differ on some day");
    let (t, g) = (day.0.z, day.1.z);
    assert_eq!(t.system(), System::Tsurphu);
    assert_eq!(g.system(), System::Genden);
    assert_eq!(to_gregorian(t), Ok(day.0.date));
    assert_eq!(to_gregorian(g), Ok(day.1.date));
    // Re-labelling Genden's fields as Tsurphu's does not give the same day back.
    let relabelled = ZDate::new(
        System::Tsurphu,
        g.year(),
        g.month(),
        g.is_leap_month(),
        g.tshes(),
        g.is_doubled(),
        g.skipped_before(),
    );
    assert_ne!(to_gregorian(relabelled), Ok(day.1.date));
}
