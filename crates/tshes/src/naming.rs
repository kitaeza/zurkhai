//! The year: rabjung cycle, year in the cycle, element, gender, animal
//! (001 §5.1; Janson (arXiv:1401.6285v1) §4, pp. 8–10).
//!
//! The Tibetan year is numbered by the Gregorian year in which it starts
//! (Janson §4, p. 8); the names are derived from that number.

use crate::arith::{amod, ceil_div, fmod};
use crate::fault::Error;

/// Janson (arXiv:1401.6285v1) §4, p. 9: the element of a year, Table 2, p. 10.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    /// Number 1 in the standard order (Table 14, p. 74).
    Wood,
    /// Number 2.
    Fire,
    /// Number 3.
    Earth,
    /// Number 4.
    Iron,
    /// Number 5.
    Water,
}

/// Janson (arXiv:1401.6285v1) §4, p. 9: the gender of a year, Tables 2 and 3,
/// p. 10 (odd position in the Chinese cycle: male; even: female).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gender {
    /// Odd position in the cycle.
    Male,
    /// Even position in the cycle.
    Female,
}

/// Janson (arXiv:1401.6285v1) §4, p. 9: the animal of a year, Table 3, p. 10.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Animal {
    /// Position 1 in the Chinese 12-year cycle.
    Mouse,
    /// Position 2.
    Ox,
    /// Position 3.
    Tiger,
    /// Position 4.
    Rabbit,
    /// Position 5.
    Dragon,
    /// Position 6.
    Snake,
    /// Position 7.
    Horse,
    /// Position 8.
    Sheep,
    /// Position 9.
    Monkey,
    /// Position 10.
    Bird,
    /// Position 11.
    Dog,
    /// Position 12.
    Pig,
}

/// The number of a rabjung (rab byung, Janson (arXiv:1401.6285v1) §4, p. 8)
/// cycle: 1 is the cycle that began in AD 1027 (eq. (4.1)). The upper bound,
/// 10^9, is the engine's own arithmetic bound, not Janson's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RabjungCycle(i64);

impl RabjungCycle {
    /// A cycle number.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidField`] outside `1..=1_000_000_000` (Janson numbers
    /// cycles from 1, "presumably `m ≥ 1`", §4, p. 8).
    pub fn new(value: i64) -> Result<Self, Error> {
        if (1..=1_000_000_000).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Error::InvalidField)
        }
    }

    /// The number.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

/// The position of a year in its rabjung cycle, 1..=60 (Janson
/// (arXiv:1401.6285v1) §4, p. 8: `1 ≤ n ≤ 60`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YearInCycle(u8);

impl YearInCycle {
    /// A position in the cycle.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidField`] outside 1..=60.
    pub fn new(value: u8) -> Result<Self, Error> {
        if (1..=60).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Error::InvalidField)
        }
    }

    /// The position.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// A Tibetan year with its names: the `year` field of 001 §5.1.
///
/// The element, gender and animal follow from the cycle and the position, so
/// the constructors derive them: a `ZYear` that contradicts itself cannot be
/// built (002 §4.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZYear {
    cycle: RabjungCycle,
    year_in_cycle: YearInCycle,
    element: Element,
    gender: Gender,
    animal: Animal,
}

impl ZYear {
    /// The year at position `year_in_cycle` of rabjung cycle `cycle`.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidField`] for a cycle outside `1..=10^9` or a position
    /// outside 1..=60.
    pub fn new(cycle: i64, year_in_cycle: u8) -> Result<Self, Error> {
        let cycle = RabjungCycle::new(cycle)?;
        let year_in_cycle = YearInCycle::new(year_in_cycle)?;
        let number = year_number(cycle, year_in_cycle);
        Ok(Self::derive(cycle, year_in_cycle, number))
    }

    /// The year whose Gregorian-numbered start year is `number` (Janson
    /// (arXiv:1401.6285v1) §4, p. 8).
    ///
    /// # Errors
    ///
    /// [`Error::InvalidField`] if the year precedes the first cycle or lies
    /// beyond the engine's bound on the cycle number.
    pub fn from_year(number: i64) -> Result<Self, Error> {
        // Janson (arXiv:1401.6285v1) §4, eq. (4.3), p. 8: m = ⌈(Y − 1026)/60⌉.
        let cycle = RabjungCycle::new(ceil_div(number - 1026, 60))?;
        // Janson (arXiv:1401.6285v1) §4, eq. (4.2), p. 8: n = (Y − 1026) amod 60.
        let position = amod(number - 1026, 60);
        let year_in_cycle = YearInCycle::new(u8::try_from(position).map_err(|_| Error::Internal)?)?;
        Ok(Self::derive(cycle, year_in_cycle, number))
    }

    fn derive(cycle: RabjungCycle, year_in_cycle: YearInCycle, number: i64) -> Self {
        // Janson (arXiv:1401.6285v1) §4, p. 9: year Y is z = (Y − 3) amod 10 in the Chinese 10-year
        // cycle and has element ⌈z/2⌉ (Table 2, p. 10; numbering Table 14, p. 74).
        let in_ten = amod(number - 3, 10);
        let element = match in_ten {
            1 | 2 => Element::Wood,
            3 | 4 => Element::Fire,
            5 | 6 => Element::Earth,
            7 | 8 => Element::Iron,
            _ => Element::Water,
        };
        // Janson (arXiv:1401.6285v1) §4, p. 9 and Table 2, p. 10: odd position, male; even, female.
        let gender = if fmod(in_ten, 2) == 1 {
            Gender::Male
        } else {
            Gender::Female
        };
        // Janson (arXiv:1401.6285v1) §4, p. 9: year Y is z = (Y − 3) amod 12 in the 12-year animal
        // cycle (Table 3, p. 10).
        let animal = match amod(number - 3, 12) {
            1 => Animal::Mouse,
            2 => Animal::Ox,
            3 => Animal::Tiger,
            4 => Animal::Rabbit,
            5 => Animal::Dragon,
            6 => Animal::Snake,
            7 => Animal::Horse,
            8 => Animal::Sheep,
            9 => Animal::Monkey,
            10 => Animal::Bird,
            11 => Animal::Dog,
            _ => Animal::Pig,
        };
        Self {
            cycle,
            year_in_cycle,
            element,
            gender,
            animal,
        }
    }

    /// The rabjung cycle.
    #[must_use]
    pub const fn rabjung_cycle(&self) -> RabjungCycle {
        self.cycle
    }

    /// The position in the cycle, 1..=60.
    #[must_use]
    pub const fn year_in_cycle(&self) -> YearInCycle {
        self.year_in_cycle
    }

    /// The element.
    #[must_use]
    pub const fn element(&self) -> Element {
        self.element
    }

    /// The gender.
    #[must_use]
    pub const fn gender(&self) -> Gender {
        self.gender
    }

    /// The animal.
    #[must_use]
    pub const fn animal(&self) -> Animal {
        self.animal
    }

    /// The Gregorian-numbered start year `Y` (Janson (arXiv:1401.6285v1) §4,
    /// eq. (4.1), p. 8).
    #[must_use]
    pub fn number(&self) -> i64 {
        year_number(self.cycle, self.year_in_cycle)
    }
}

/// Janson (arXiv:1401.6285v1) §4, eq. (4.1), p. 8:
/// `Y = 1027 + (m − 1)·60 + (n − 1) = 60m + n + 966`.
fn year_number(cycle: RabjungCycle, year_in_cycle: YearInCycle) -> i64 {
    60 * cycle.0 + i64::from(year_in_cycle.0) + 966
}
