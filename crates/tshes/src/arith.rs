//! Floor division and modulo, named (002 §4.2).
//!
//! Rust's `/` and `%` truncate toward zero; Janson's formulas use the mathematical
//! floor, `mod`, `amod`, `⌊x⌋` and `⌈x⌉`, defined on p. 6 (Janson,
//! arXiv:1401.6285v1, §2 "Other notations"). Every division in the crate goes
//! through these helpers. Each divisor is a positive constant at every call
//! site, so none of them can divide by zero or overflow on the engine's
//! bounded values.

/// `⌊a / b⌋` for `b > 0` (Janson (arXiv:1401.6285v1) §2, p. 6).
///
/// ```
/// use tshes::arith::floor_div;
/// assert_eq!(floor_div(7, 2), 3);
/// assert_eq!(floor_div(-7, 2), -4); // truncation would give -3
/// assert_eq!(floor_div(-6, 2), -3);
/// assert_eq!(floor_div(-1, 30), -1);
/// ```
#[must_use]
pub const fn floor_div(a: i64, b: i64) -> i64 {
    debug_assert!(b > 0);
    a.div_euclid(b)
}

/// Janson's `a mod b`: the `x` with `x ≡ a (mod b)` and `0 ≤ x < b`, for
/// `b > 0` and `a` of either sign (Janson (arXiv:1401.6285v1) §2, p. 6).
///
/// ```
/// use tshes::arith::fmod;
/// assert_eq!(fmod(7, 5), 2);
/// assert_eq!(fmod(-7, 5), 3); // `%` would give -2
/// assert_eq!(fmod(-5, 5), 0);
/// ```
#[must_use]
pub const fn fmod(a: i64, b: i64) -> i64 {
    debug_assert!(b > 0);
    a.rem_euclid(b)
}

/// Janson's `a amod b`: the `x` with `x ≡ a (mod b)` and `0 < x ≤ b`, for
/// `b > 0` (Janson (arXiv:1401.6285v1) §2, p. 6: `m amod n = 1 + (m − 1) mod n`).
///
/// ```
/// use tshes::arith::amod;
/// assert_eq!(amod(12, 12), 12); // a multiple of 12 gives 12, not 0
/// assert_eq!(amod(13, 12), 1);
/// assert_eq!(amod(0, 12), 12);
/// assert_eq!(amod(-1, 12), 11);
/// ```
#[must_use]
pub const fn amod(a: i64, b: i64) -> i64 {
    1 + fmod(a - 1, b)
}

/// `⌈a / b⌉` for `b > 0`: the ceiling, named as such (002 §4.2; Janson
/// (arXiv:1401.6285v1) §2, p. 6 defines `⌈x⌉`).
///
/// ```
/// use tshes::arith::ceil_div;
/// assert_eq!(ceil_div(7, 2), 4);
/// assert_eq!(ceil_div(-7, 2), -3); // the ceiling of -3.5
/// assert_eq!(ceil_div(-6, 2), -3);
/// assert_eq!(ceil_div(0, 5), 0);
/// assert_eq!(ceil_div(1, 5), 1);
/// ```
#[must_use]
pub const fn ceil_div(a: i64, b: i64) -> i64 {
    debug_assert!(b > 0);
    -floor_div(-a, b)
}
