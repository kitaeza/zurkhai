//! Genden and Tsurphu are not implemented (spec 005; `DEBT.md` D-9). The engine
//! computes nothing for them and returns an error; it never falls back to
//! Phugpa. These tests pin that gap (METHOD rule 33): they are replaced by the
//! Genden and Tsurphu tests by the round that implements those systems (M2).

use tshes::{Error, GregorianDate, MonthNo, System, Tshes, ZDate, ZYear, to_gregorian, to_zdate};

fn sample_date() -> GregorianDate {
    GregorianDate::new(2024, 2, 10).unwrap()
}

fn sample_zdate(system: System) -> ZDate {
    ZDate::new(
        system,
        ZYear::from_year(2024).unwrap(),
        MonthNo::new(1).unwrap(),
        false,
        Tshes::new(1).unwrap(),
        false,
        None,
    )
}

/// `DEBT.md` D-9: Genden is not implemented yet.
#[test]
fn genden_not_yet_supported() {
    assert_eq!(
        to_zdate(sample_date(), System::Genden),
        Err(Error::SystemNotImplemented)
    );
    assert_eq!(
        to_gregorian(sample_zdate(System::Genden)),
        Err(Error::SystemNotImplemented)
    );
}

/// `DEBT.md` D-9: Tsurphu is not implemented yet.
#[test]
fn tsurphu_not_yet_supported() {
    assert_eq!(
        to_zdate(sample_date(), System::Tsurphu),
        Err(Error::SystemNotImplemented)
    );
    assert_eq!(
        to_gregorian(sample_zdate(System::Tsurphu)),
        Err(Error::SystemNotImplemented)
    );
}

#[test]
fn an_unimplemented_system_never_falls_back_to_phugpa() {
    // The same date and fields, asked of Phugpa, do compute; the unimplemented
    // systems must differ from that by being errors, not by giving the Phugpa answer.
    assert!(to_zdate(sample_date(), System::Phugpa).is_ok());
    assert!(to_gregorian(sample_zdate(System::Phugpa)).is_ok());
    assert!(to_zdate(sample_date(), System::Genden).is_err());
    assert!(to_zdate(sample_date(), System::Tsurphu).is_err());
}

#[test]
fn the_error_type_displays_and_is_a_core_error() {
    // 002 §5: `Error` implements `core::fmt::Display` and `core::error::Error`.
    fn assert_error<E: core::error::Error>(_: &E) {}
    for error in [
        Error::OutOfRange,
        Error::SkippedDay,
        Error::InvalidField,
        Error::SystemNotImplemented,
        Error::Internal,
    ] {
        assert_error(&error);
        assert!(format!("{error}").len() > 10);
    }
}
