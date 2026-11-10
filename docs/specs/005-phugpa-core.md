spec: 005
prompt: docs/specs/prompts/005-phugpa-core.md
branch: spec/005-phugpa-core (cut from main at 59bad84)
merge SHA: not yet merged
pushed: no
model: Sonnet 5.5 (claude-sonnet-5-5), effort high — one session; the switch point to the fallback model (Opus 5.5) was never reached: no design choice fell to it
gate: 65 passed, 0 failed (debug) / 65 passed, 0 failed (release) — no failed tests; skipped legs: wasm-pack (crates/tshes-wasm does not exist)

Status: draft — the round is complete; status is set to accepted at landing
Depends on: 001, 002, 003, 004
Resolves: none
Milestone: M1
Governs: crates/tshes, tools/, docs/glossary.md

— spec text ends here —

# Report — spec 005 (the Phugpa core)

Dates in this report come from `date`, run once at the start of the round: 2026-11-10 (METHOD rule 51). Research legs: not applicable. 003 §4 sets their precondition as Q1, every L2 category and any new attribute; this round implements 001 §5.1 from the source 001 names (spec 003 §10: a round that settles a known pattern from its source needs no legs).

## The decisive item: the citation table, and Item 5's examples

**What was cited, and how.** Every row below is a formula or a constant in `crates/tshes/src/`, with the value *as written in the code*, Janson v1's location, and the paper's own expression. Source: S. Janson, "Tibetan calendar mathematics", arXiv:1401.6285v1, 24 Jan 2014, 86 pages, the library copy `janson-1401.6285v1.pdf` (sha256 `7cafc7df563a3020849c86f4e397daa18b7f7f5ff46244403180aca90b9d0f77`). Label for every citation: **FETCHED** (read page by page this round from the library copy, no network). Quotations are two sentences at most.

**The second pass.** After the code was written (commit 801b79e; the named constants of 4935c36 changed no value), each cited page was re-opened and compared against the code: pp. 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 38, 41, 44, 50, 56, 60, 62 and 68. Verdict MATCH or MISMATCH. **One MISMATCH was found in the first pass and fixed:** a comment cited "§7, p. 5" for the convention that angular numbers are "considered modulo 1"; the page is right but the section is §2 ("Angular units"). Fixed in 4935c36 and re-checked: MATCH (row 51). No value, equation number or page number was found wrong.

| # | Name in code | Value or expression as written in code | Janson v1: section, eq., page | The paper's expression | 2nd pass |
|---|---|---|---|---|---|
| 1 | `JD_MINUS_RD` | `1_721_425` | §2, fn 8, p. 5 | "RD 1 is 1 January 1 (Gregorian) which is JD 1721426 … JD = RD + 1721425" | MATCH |
| 2 | `GregorianDate::to_jdn` | `365·(y−1) + ⌊(y−1)/4⌋ − ⌊(y−1)/100⌋ + ⌊(y−1)/400⌋ + days_before_month + day + 1721425` | DERIVED: Janson gives no Gregorian↔JD formula (Item 3's one exception); anchors at §2, p. 5 fn 8 and p. 6 | "1 January 2007 has JD 2454102" | anchors MATCH; the closed form is tested against a day-by-day count of all 73,414 days of the range |
| 3 | anchor test `janson_anchor_1_january_2007_is_jd_2454102` | `GregorianDate(2007,1,1).to_jdn() == 2_454_102` | §2, p. 6 | JD 2454102 | MATCH |
| 4 | `floor_div` | `a.div_euclid(b)`, `b > 0` | §2, p. 6 | `⌊x⌋`: x − 1 < ⌊x⌋ ≤ x | MATCH |
| 5 | `fmod` | `a.rem_euclid(b)`, `b > 0` | §2, p. 6 | `m mod n = x` if x ≡ m (mod n) and 0 ≤ x < n | MATCH |
| 6 | `amod` | `1 + fmod(a − 1, b)` | §2, p. 6 | `m amod n = 1 + (m − 1) mod n` | MATCH |
| 7 | `ceil_div` | `−floor_div(−a, b)` | §2, p. 6 | `⌈x⌉`: x ≤ ⌈x⌉ < x + 1 | MATCH |
| 8 | `radix_fraction` (unit tests) | `a0;a1,…,an (b1,…,bn) = a0 + (a1 + (… + an/bn)/…)/b1`, innermost first | §2, p. 4 | `a0; a1,…,an (b1,…,bn) = a0 + (a1; a2,…,an (b2,…,bn))/b1` | MATCH |
| 9 | `year_number` | `60·cycle + year_in_cycle + 966` | §4, eq. (4.1), p. 8 | Y = 1027 + (m−1)60 + (n−1) = 60m + n + 966 | MATCH |
| 10 | `ZYear::from_year`, position | `amod(Y − 1026, 60)` | §4, eq. (4.2), p. 8 | n = (Y − 1026) amod 60 | MATCH |
| 11 | `ZYear::from_year`, cycle | `ceil_div(Y − 1026, 60)` | §4, eq. (4.3), p. 8 | m = ⌈(Y − 1026)/60⌉ | MATCH |
| 12 | `RabjungCycle::new` | `1..=1_000_000_000` | §4, p. 8 (lower bound only; the upper bound is the engine's own, stated in the type's doc) | "(with 1 ≤ n ≤ 60 and, presumably, m ≥ 1)" | MATCH (lower bound) |
| 13 | `YearInCycle::new` | `1..=60` | §4, p. 8 | 1 ≤ n ≤ 60 | MATCH |
| 14 | `ZYear::derive`, ten-cycle position | `amod(Y − 3, 10)` | §4, p. 9 | z = (Y − 3) amod 10 | MATCH |
| 15 | element arms | `1\|2 Wood, 3\|4 Fire, 5\|6 Earth, 7\|8 Iron, _ Water` | §4, p. 9 (`⌈z/2⌉`); Table 2, p. 10; numbering Table 14, p. 74 | years 1,2 wood; 3,4 fire; 5,6 earth; 7,8 iron; 9,10 water | MATCH |
| 16 | gender | `fmod(z, 2) == 1 → Male` | Tables 2 and 3, p. 10; §4, p. 9 | odd male, even female ("the year mod 2") | MATCH |
| 17 | twelve-cycle position | `amod(Y − 3, 12)` | §4, p. 9 | z = (Y − 3) amod 12 | MATCH |
| 18 | animal arms | 1 Mouse, 2 Ox, 3 Tiger, 4 Rabbit, 5 Dragon, 6 Snake, 7 Horse, 8 Sheep, 9 Monkey, 10 Bird, 11 Dog, 12 Pig | Table 3, p. 10 | the same twelve, in that order | MATCH |
| 19 | `Tshes::new` | `1..=30` | §3, p. 7 | "The lunar days are numbered by 1–30" | MATCH |
| 20 | `MonthNo::new` | `1..=12` | §5, p. 11 | "The regular months in each year are numbered 1–12" | MATCH |
| 21 | `EPOCH_MONTH` | `3` | §5, Remark 3, p. 14 | M₀ = 3 | MATCH |
| 22 | `LUNAR_MONTHS`, `SOLAR_MONTHS` | `67`, `65` | §5.2, eq. (5.1), p. 12 | 67 lunar months = 65 solar months | MATCH |
| 23 | `BETA_BASE` | `184` | §5.3, eq. (5.15), p. 17 | β = 184 − β* | MATCH |
| 24 | `ROUNDING_SHIFT` | `17` | §5, eq. (5.10), p. 16 | n = ⌊(67M* + β* + 17)/65⌋ − [ℓ] ("Since 48 = 65 − 17") | MATCH |
| 25 | `Epoch::E806.y0`, `E1927.y0`, `E1987.y0` | `806`, `1927`, `1987` | §5, p. 14 | "month 3 year 806 … month 3 year 1927 … month 3 year 1987" | MATCH |
| 26 | `beta_star` of the three epochs | `61`, `55`, `0` | §5, eqs. (5.4)–(5.6), p. 14 | β* = 61 (E806), 55 (E1927), 0 (E1987) | MATCH |
| 27 | `M1` | `167_025 * 2` (over 11312) | §7, eq. (7.2), p. 21 | m₁ = 167025/5656 | MATCH |
| 28 | `M2` | `11_135` (over 11312) | §7, eq. (7.3), p. 21 | m₂ = 11135/11312 | MATCH |
| 29 | `E806.m0` | `2_015_501 * 11_312 + 4_783 * 2` | §7, eq. (7.4), p. 21 | m₀ = 2015501 + 4783/5656 | MATCH |
| 30 | `E1927.m0` | `2_424_972 * 11_312 + 5_457 * 2` | §7, Remark 9, p. 22 | m₀ + 13866·m₁ = 2424972 + 5457/5656 | MATCH |
| 31 | `E1987.m0` | `2_446_914 * 11_312 + 135 * 16` | §7, Remark 9, p. 22 | m₀ + 14609·m₁ = 2446914 + 135/707 | MATCH |
| 32 | `S1` | `65 * 6` (over 4824) | §7, eq. (7.6), p. 23 | s₁ = 65/804 | MATCH |
| 33 | `S2` | `13` (over 4824) | §7, eq. (7.7), p. 23 | s₂ = 13/4824 | MATCH |
| 34 | `E806.s0` | `743 * 6` | §7, eq. (7.8), p. 23 | s₀ = 743/804 | MATCH |
| 35 | `E1927.s0` | `749 * 6` | §7, eq. (7.9), p. 23 | s₀ + 13866·s₁ ≡ 749/804 | MATCH |
| 36 | `E1987.s0` | `0` | §7, eq. (7.10), p. 23 | s₀ + 14609·s₁ ≡ 0 | MATCH |
| 37 | `A1` | `253` (over 3528) | §7, eq. (7.12), p. 23 | a₁ = 253/3528 | MATCH |
| 38 | `A2` | `126` (over 3528) | §7, eq. (7.13), p. 23 | a₂ = 1/28 (= 126/3528) | MATCH |
| 39 | `E806.a0` | `475` | §7, eq. (7.14), p. 23 | a₀ = 475/3528 | MATCH |
| 40 | `E1927.a0` | `1741` | §7, eq. (7.15), p. 23 | a₀ + 13866·a₁ ≡ 1741/3528 | MATCH |
| 41 | `E1987.a0` | `38 * 72` | §7, eq. (7.16), p. 23 | a₀ + 14609·a₁ ≡ 38/49 | MATCH |
| 42 | `SUN_ANOMALY_OFFSET` | `1_206` (over 4824) | §7, eq. (7.19), p. 24 | anomaly_sun = mean_sun − 1/4 | MATCH |
| 43 | `moon_tab_base` | 0 5 10 15 19 22 24 25 | §7, eq. (7.18), p. 24 | the same eight values | MATCH |
| 44 | `moon_tab` symmetry | `T(14−i)=T(i)`, `T(14+i)=−T(i)`, `T(28+i)=T(i)` | §7, eq. (7.18), p. 24 | the same three rules | MATCH |
| 45 | `sun_tab_base` | 0 6 10 11 | §7, eq. (7.21), p. 24 | the same four values | MATCH |
| 46 | `sun_tab` symmetry | `U(6−i)=U(i)`, `U(6+i)=−U(i)`, `U(12+i)=U(i)` | §7, eq. (7.21), p. 24 | the same three rules | MATCH |
| 47 | `MOON_TAB_UNITS` | `126` | §7, eq. (7.17), p. 24 | moon_equ = moon_tab(28·anomaly_moon); with anomaly = x/3528 the argument is x/126 | MATCH (derived) |
| 48 | `SUN_TAB_UNITS` | `402` | §7, eq. (7.20), p. 24 | sun_equ = sun_tab(12·anomaly_sun); with anomaly = y/4824 the argument is y/402 | MATCH (derived) |
| 49 | mean date | `epoch.m0 + M1·n + M2·d` | §7, eq. (7.1), p. 21 | mean_date(d, n) = n·m₁ + d·m₂ + m₀ | MATCH |
| 50 | mean sun, sun anomaly | `fmod(S1·n + S2·d + s0 − 1206, 4824)` | §7, eqs. (7.5), p. 22; (7.19), p. 24 | mean_sun(d, n) = n·s₁ + d·s₂ + s₀; anomaly_sun = mean_sun − 1/4 | MATCH |
| 51 | moon anomaly | `fmod(A1·n + A2·d + a0, 3528)` (angles mod 1: §2, "Angular units", p. 5) | §7, eq. (7.11), p. 23; §2, p. 5 | anomaly_moon(d, n) = n·a₁ + d·a₂ + a₀; "the numbers are considered modulo 1" | MATCH (the section label of the modulo-1 convention was the first-pass MISMATCH, fixed) |
| 52 | moon equation | `126·T(k) + (T(k+1) − T(k))·r`, `x = 126k + r` | §7, eqs. (7.17), (7.18), p. 24 | "linear interpolation is used between integer arguments" | MATCH |
| 53 | sun equation | `402·U(k) + (U(k+1) − U(k))·r`, `y = 402k + r` | §7, eqs. (7.20), (7.21), p. 24 | the same interpolation | MATCH |
| 54 | `true_date_numerator` | `9045·mean + 13534·moon_equ − 4242·sun_equ`, over 102_317_040 | §7, eq. (7.22), p. 24 | true_date = mean_date + moon_equ/60 − sun_equ/60 | MATCH |
| 55 | `TRUE_DATE_DENOMINATOR`, `MEAN_`/`MOON_`/`SUN_MULTIPLIER` | `102_317_040`; `9_045`; `13_534`; `4_242` | DERIVED: lcm(11312, 126·60, 402·60); no Janson page | — | derivation tested by `multipliers_are_the_stated_quotients` |
| 56 | `end_day` | `⌊true_date⌋` (floor of the numerator over the denominator) | §8, eq. (8.1), p. 27 | JD = ⌊true_date⌋ | MATCH |
| 57 | repeated day | day `== end_this` is the second, `end_this − 1` the first | §8, eq. (8.1), p. 27 | "if the Tibetan date is repeated, this gives the JD of the second day; for the first we thus have to subtract 1" | MATCH |
| 58 | `label_of_day` | the smallest lunar day `t` with `end_day(t) ≥ J` | §6, p. 20 | "labelled by the lunar day that is current at the beginning of the calendar day"; two lunar days ending the same day: the first; none: the following day's name | MATCH |
| 59 | `label_of_day` window | `(J·11312 − m0)/11135 − 3 .. + 4` | §7, eqs. (7.1)–(7.3), p. 21; Remark 15, p. 26 | the mean date; "the length of the (true) lunar day varies between, roughly, 21.5 and 25.7 hours" | MATCH (the window is derived; no `Error::Internal` over the range) |
| 60 | `month_of_count`, x | `ceil_div(65n + β, 67)` | §5.3, eq. (5.19), p. 17 | x = ⌈(65n + β)/67⌉ | MATCH |
| 61 | `month_of_count`, M | `amod(x, 12)` | §5.3, eq. (5.20), p. 17 | M = x amod 12 | MATCH |
| 62 | `month_of_count`, Y | `(x − M)/12 + Y0` | §5.3, eq. (5.21), p. 17 | Y = (x − M)/12 + Y₀ | MATCH |
| 63 | `month_of_count`, leap | `(65n + β) mod 67 ∈ {1, 2}` | §5.3, eq. (5.22), p. 17 | ℓ = {(65n + β) mod 67 = 1 or 2} | MATCH |
| 64 | `count_of_month` | `⌊(67·M* + β* + 17)/65⌋ − [ℓ]`, `M* = 12(Y−Y0) + M − 3` | §5, eqs. (5.2), p. 14; (5.10), p. 16 | M* = 12(Y − Y₀) + M − M₀; n = ⌊(67M* + β* + 17)/65⌋ − [ℓ] | MATCH |
| 65 | `is_doubled` (the second occurrence) | `count == 2 && day == end_this` | 001 §5.1 fixes the flag; Janson's *leap day* is the *first* of the two days, §6, p. 20, and p. 1 | "the first of the two days with the same number is regarded as a leap day" | MATCH (the two are different days; recorded in `docs/glossary.md`) |
| 66 | `SUPPORTED_YEAR_MIN`, `SUPPORTED_YEAR_MAX` | `1899`, `2100` | DERIVED by the test `supported_tibetan_years_are_1899_to_2100`; no Janson page | — | derivation tested |

**Formulas and constants used as oracles in the tests** (second pass over the same pages):

| # | Where | Expression as written in the test | Janson v1 | The paper's expression | 2nd pass |
|---|---|---|---|---|---|
| T1 | `exhaustive.rs`, leap years | `(24Y + 33) mod 65 ≥ 41` | §5.5, eqs. (5.40), (5.41), p. 19 | Y is a leap year iff (24Y + 33) mod 65 ≥ 41 | MATCH |
| T2 | `exhaustive.rs`, leap month number | `33 − (r + 1) div 2`, `r = (24Y + 33) mod 65` | §5.5, eq. (5.42), p. 19 | M = ⌊33 − ((24Y + 33) mod 65)/2⌋ | MATCH (the test's first version floored `r/2` before subtracting; the engine passed, the test was wrong, and was fixed) |
| T3 | `exhaustive.rs`, leap count | `⌊(24(Y2+1) + 33)/65⌋ − ⌊(24·Y1 + 33)/65⌋` | §5.5, eqs. (5.36), (5.39), p. 19 | ⌊(24(Y₂+1) + γ*)/65⌋ − ⌊(24Y₁ + γ*)/65⌋, γ* = 33 | MATCH |
| T4 | `epochs.rs`, leap rule | `24(Y − Y0) + 2M ≡ β or β + 1 (mod 65)`, β = 184 − β* | §5.4, eq. (5.24), p. 17 | the same | MATCH |
| T5 | `epochs.rs`, Appendix C | `⌊((12(Y−Y0)+M)·den − num)·67 / (65·den)⌋ − [ℓ]`, α = 1832/1005, 1922/1005, 41/15 | App. C, eq. (C.22), p. 60; (C.30)–(C.32), p. 62 | n = ⌊(12(Y−Y₀)+M−α)/(12s₁)⌋ − [ℓ], 12s₁ = 65/67 | MATCH |
| T6 | `worked_examples.rs`, weekday | `(JD + 2) mod 7`; 0 Saturday … 6 Friday | §9, eq. (9.1) and Table 5, p. 28 | day_of_week = (JD + 2) mod 7 | MATCH |
| T7 | `phugpa.rs` tests, Table 6 | `2;22,34,2,518`, `0,8,3,3,33`, `5,98` | A.13, Table 6, p. 50 | the Phugpa column | MATCH |
| T8 | `phugpa.rs` tests, epoch dates | JD 2015501 = 29/1 806; JD 2015531 = 30/2 806; JD 2424972 = 29/1 1927; JD 2446914 = 1/3 1987 | §7, Remark 16 and p. 27; A.13, fn 64, p. 50; D.1, (D.2), p. 68 | "the Tibetan dates are 29/1 806, 29/1 1927 and 1/3 1987"; "day 30 in month 2 for the Phugpa … versions"; "JD 2424972 (Friday, 1 April 1927)" | MATCH |
| T9 | `phugpa.rs` tests, exact `a₂` | `a2 = 3781/105840` over 105840; JDs 2451951, 2453866, 2460999; 9 days in 1900–1999, 8 in 2000–2099 | §7, eq. (7.24), p. 25; Remark 14, p. 26 | `a₂ = (1 + a₁)/30 = 3781/105840`; "9 examples in 1900–1999 and 8 examples in 2000–2099"; the three JDs | MATCH |

Rows: 66 formulas and constants in the engine (1–66) and 9 oracle rows (T1–T9) = 75 rows. Second-pass verdicts: 73 rows cite a Janson page and each is MATCH; 2 rows (55, 66) are derived constants with no Janson page, their derivations tested; no MISMATCH stands. One MISMATCH was found in the first pass and fixed (row 51), and one test-side slip was fixed (T2: not a citation error).

### Item 5 — Janson's worked examples

**Sections searched.** Every page, 1–86: §1 (pp. 1–3), §2 Notation (pp. 4–6), §3 Concepts (pp. 6–8), §4 Numbering and naming of years (pp. 8–10), §5 Months and leap months (pp. 11–19), §6 Days (pp. 19–21), §7 Astronomical functions (pp. 21–27), §8 Calendrical functions (pp. 27–28), §9 Day of week (p. 28), §10 Further calculations (pp. 29–33), §11 Holidays (p. 34), §12 Mean lengths and astronomical accuracy (pp. 34–36), §13 Period (p. 37), Appendix A (pp. 38–54), B (pp. 55–56), C (pp. 57–67), D (pp. 68–72), E (pp. 73–83), the references (pp. 84–86).

**Finding.** Janson v1 has **no worked example that carries a date through the lunar-day formulas (7.1)–(7.22) with its intermediate values** (no mean date, no anomaly, no equation, no true date for any named day). It has these, each made a test with a seeded control (a deliberately wrong value, asserted to fail) and checked in a second pass against its page:

*Example A — the year naming of AD 2007 (§4, pp. 8–9), with intermediate values.* Test `ad_2007_is_the_21st_year_of_the_17th_cycle_and_fire_female_pig`; control `seeded_control_ad_2007_with_one_value_wrong_fails` (six wrong values, each asserted to fail).

```
year: 2007
rabjung cycle m: 17                  (p. 8: "the 17th Prabhava cycle")
year in cycle n: 21                  (p. 8: "the 21st year")
Chinese-cycle number: 24             (p. 9: "the 24th year in the Chinese cycle")
(Y - 3) amod 10: 4                   (derived, eq. (4.4); Table 2, p. 10)
(Y - 3) amod 12: 12                  (derived, eq. (4.4); Table 3, p. 10)
element: Fire
gender: female
animal: Pig                          (p. 9: "Fire-female-Pig or simply Fire-Pig")
```

*Example B — the first year of the 17th cycle (pp. 8–9) and the start of the Chinese cycle (p. 9).* Test `the_17th_cycle_began_in_1987_and_its_first_year_is_the_fourth_of_the_chinese_cycle`.

```
year: 1987      rabjung cycle: 17   year in cycle: 1    Chinese-cycle number: 4
                element: Fire   gender: female   animal: Rabbit        (p. 9)
year: 1984      rabjung cycle: 16   year in cycle: 58   Chinese-cycle number: 1
                element: Wood   gender: male     animal: Mouse   (p. 9 "The last cycle thus started 1984"; Table 10, p. 56, line 58)
control: year 1988 is not the first year of the 17th cycle
```

*Example C — page 1, the second date line.* Test `page_1_water_snake_date_8_january_2014_is_wednesday_8_month_11`; its controls: tshes 9 and month 12 are wrong.

```
gregorian date: 2014-01-08       JD: 2456666
tibetan year: 2013               (p. 8: numbered by the Gregorian year in which it starts)
element, animal: Water, Snake    (p. 1: "Water-Snake year")
month: 11    leap: false    tshes: 8
weekday by eq. (9.1): Wednesday  (p. 1: "Wednesday 8")
```

*Example D — page 1, the first date line.* Test `page_1_fire_pig_date_31_december_2007_is_the_23rd_of_month_11`; control: tshes 24. The date triple agrees; the weekday word does not (below, and `DEBT.md` D-10).

```
gregorian date: 2007-12-31       JD: 2454466
tibetan year: 2007               element, animal: Fire, Pig
month: 11    leap: false    tshes: 23
weekday by eq. (9.1): Monday     (p. 1 says "Sunday 23")
```

*Example E — the leap month 1 of 2000 (§4, p. 9; Remark 17 and fn 28, p. 28).* Test `losar_2000_was_the_first_day_of_leap_month_1_a_sunday`; controls: the regular month 1; the next day.

```
gregorian date: 2000-02-06       JD: 2451581
tibetan year: 2000    month: 1    leap: true    tshes: 1
weekday by eq. (9.1): Sunday     (fn 28: "Losar was celebrated on Sunday, February 6th (2000), which was the first day of leap month 1")
```

*Example F — the epoch days (§7, Remark 16 and p. 27; fn 64, p. 50; D.1, p. 68).* Tests `the_epoch_days_of_1927_and_1987_have_the_dates_of_p_27` (public API) and `epoch_days_of_e806_have_the_dates_janson_states`, `epoch_days_of_e1927_and_e1987_…` (crate unit tests; 806 is outside the supported range); controls: the next day.

```
E806:   JD 2015501 = floor(m0)       tibetan date: 29/1 806     (p. 27)
E806:   JD 2015531 (Table 6 epoch)   tibetan date: 30/2 806     (fn 64, p. 50)
E1927:  JD 2424972 = 1927-04-01 (Friday, p. 68)   tibetan date: 29/1 1927   (p. 27)
E1987:  JD 2446914 = 1987-04-28      tibetan date: 1/3 1987     (p. 27)
```

*Example G — moving the epoch (§7, Remarks 9–11, pp. 22–23).* Tests `remark_9_to_11_e1927_is_e806_shifted_by_13866_months`, `remark_9_to_11_e1987_…`; control `seeded_control_a_wrong_epoch_shift_fails` (13867, 14610, and the wrong target).

```
shift: 13866 months (E1927)    m0 + 13866*m1 = 2424972 + 5457/5656
                               s0 + 13866*s1 = 749/804 (mod 1)      a0 + 13866*a1 = 1741/3528 (mod 1)
shift: 14609 months (E1987)    m0 + 14609*m1 = 2446914 + 135/707
                               s0 + 14609*s1 = 0 (mod 1)            a0 + 14609*a1 = 38/49 (mod 1)
409465 = 7 * 58495             (a whole number of weeks)
```

*Example H — Table 6 (A.13, p. 50).* Test `table_6_phugpa_epoch_data_is_e806_one_month_later`; control: one digit wrong.

```
m0: 2;22,34,2,518 (60,60,6,707) + 2015529 = E806 m0 + m1
s0: 0,8,3,3,33 (27,60,60,6,67)            = E806 s0 + s1 (mod 1)
a0: 5,98 (28,126)                         = E806 a0 + a1 (mod 1)
```

*Example I — the two versions of `a₂` (§7, Remark 14, p. 26).* Tests `remark_14_the_two_versions_of_a2_differ_on_the_days_janson_lists` and `remark_14_footnote_27_the_two_versions_differ_on_9_and_8_days`; control: the neighbouring days of each listed JD do not differ. The count is Janson's, from his own computer search; the engine, with `a₂ = 1/28`, and a separate test-side implementation of eq. (7.24), reproduce it exactly.

```
JD 2451951 (2001-02-10): the two versions give different Tibetan dates
JD 2453866 (2006-05-10): the two versions give different Tibetan dates
JD 2460999 (2025-11-19): the two versions give different Tibetan dates
days in 1900-1999 on which they differ: 9      days in 2000-2099 on which they differ: 8
```

**Rule 35.** P1–P3 check consistency, not correctness; a shared wrong epoch passes them all. The planted control C below demonstrates it: an epoch one day late in both directions passes all 13 exhaustive tests, three runs in three, and is caught only by the cross-epoch check and by these examples. The correctness checks are this item, the three-epoch comparison (`epochs.rs`), and round 006's T5 tables and T4 output. The round does not claim more.

## Every property, DONE / DEFERRED / N-A

| Property | Verdict | Evidence |
|---|---|---|
| P1 every day maps to exactly one `ZDate` | DONE | `p1_every_day_maps_to_exactly_one_zdate`: 73,414 days, 73,414 distinct `ZDate`s, `to_zdate(d)` identical on a second call; expected 73,414, actual 73,414 (the planted control A makes it 71,209 — red) |
| P2 round trip, both directions | DONE | `p2_round_trip_from_every_day` (`to_gregorian(to_zdate(d)) == d`, 73,414 days); `p2_round_trip_from_every_valid_zdate` (`to_zdate(to_gregorian(z)) == z`); `p2_every_combination_of_fields_is_classified_exactly`: 2,486 complete months × 30 tshes × 2 doubled flags × 4 `skipped_before` values = 596,640 combinations, each classified as the forward table (built from the days, not from `to_gregorian`) predicts (Ok with that day / `SkippedDay` / `InvalidField`); all three outcomes reached |
| P3 within a month tshes never decrease; once, twice or absent | DONE | `p3_tshes_never_decrease_within_a_month_and_steps_are_zero_one_or_two` (every pair of consecutive days: step 0 with the doubled flags, 1, or 2 with `skipped_before` naming the skipped tshes; the month changes exactly when the number falls); `p3_each_tshes_is_present_once_twice_or_absent_in_every_complete_month` |
| P4 months of 29 or 30 days | DONE | `p4_complete_months_have_29_or_30_days`: 1,168 months of 29 days and 1,318 of 30 days, 0 others; `p4_month_sequence_follows_the_numbering_rules` (a leap month is followed by the regular month with its number; 12 → 1 of the next year) |
| P4 leap months at the frequency the rule implies | DONE | `p4_leap_months_follow_the_intercalation_rule`: expected by eq. (5.36), p. 19 (Y₁ = 1899, Y₂ = 2100): ⌊(24·2101 + 33)/65⌋ − ⌊(24·1899 + 33)/65⌋ = 776 − 701 = **75**; measured **75** (74 in complete months + the leap month 11 of 1899, which holds 1900-01-01); every leap month where eqs. (5.41), (5.42) put it; successive gaps are 33 or 34 months and alternate |
| P5 reproduces T5 tables | N-A here | round 006 (T5 tables); nothing under `golden/` is touched |
| P6 reproduces T4 output | N-A here | round 006 (T4 fixtures) |
| P7 range, behaviour outside it defined | DONE | stated and asserted, not "never panics": `to_zdate` outside 1900-01-01..=2100-12-31 is exactly `Err(OutOfRange)`, down to the ends of the Gregorian domain (`p7_outside_the_range_the_answer_is_out_of_range`, `to_zdate_is_exact_inside_the_range_and_out_of_range_outside_it`); `to_gregorian` of a Tibetan year outside 1899..=2100 is exactly `Err(OutOfRange)`; a skipped tshes is `Err(SkippedDay)`; an absent leap month, a wrong doubled flag and a wrong `skipped_before` are `Err(InvalidField)` (`to_gregorian_is_classified_exactly_for_arbitrary_valid_fields`, proptest, 3000 cases, boundary strategy beside the uniform one); each constructor refuses exactly the values outside its range with `InvalidField` (`constructors_refuse_exactly_the_values_outside_their_range`) |

**Run settings, quoted from the commands** (METHOD rule 23): the exhaustive and proptest tests ran in `cargo test --workspace` and `cargo test --workspace --release` through `tools/gate.sh`, on `rustc 1.99.0 (b940084d7 2026-09-28)`; the proptest property uses `ProptestConfig::with_cases(3000)` with proptest's own seeding (a new seed is new coverage, rule 42) and the committed regression seed; the planted-control runs used `cargo test --manifest-path <scratch copy>/Cargo.toml -p tshes --test exhaustive` with `CARGO_TARGET_DIR` set to the copy's own directory.

### Shape coverage (003 §3), measured and counted

From `shape_census_over_the_range` (the test prints these lines; counts, not "green"):

```
days                                   73,414    (1900-01-01 .. 2100-12-31)
Tibetan years                             202    (1899 .. 2100)
months, all                             2,488    (2,486 complete + 2 partial at the edges)
months, complete                        2,486
  leap months, all                         75    (complete: 74; the 75th is the leap month 11 of 1899)
  complete months of 29 days            1,168
  complete months of 30 days            1,318    (1,168 + 1,318 = 2,486)
skipped tshes (days with skipped_before) 2,800
doubled tshes (days with is_doubled)     1,632
12 -> 1 rollovers                         201
rabjung-cycle boundaries crossed            3    (cycles 15->16 in 1927, 16->17 in 1987, 17->18 in 2047)
```

The identity over the complete months: skipped − doubled = 30 × months − days. Skipped lunar days are counted from the days themselves (a tshes in 1..=30 that no day of its month carries), doubled ones as tshes carried twice. **2,800 − 1,632 = 1,168 = 30 × 2,486 − 73,412 = 74,580 − 73,412.** The `skipped_before` field is also checked against the absence count month by month (a skipped 30 is recorded on tshes 1 of the next month), and agrees in every complete month. Every shape is asserted non-zero (METHOD rule 39); the years, the rollovers and the cycle boundaries are asserted at their independently derived values.

### The planted controls (METHOD rules 41, 42; Item 4)

All on scratch copies of the committed tree under the scratchpad (never the working tree), each copy with its own `CARGO_TARGET_DIR`; the plant script asserts that its anchor occurs exactly once. Run at the tip 4935c36's source (and, before the named-constants refactor, at 801b79e: identical results).

| Plant | Mutation | Exhaustive run, 3 runs | The failing assertions, by name |
|---|---|---|---|
| A, month end | `tshes_of` reports tshes 30 as 29 (`.min(29)`) | red, red, red (7 of 13 tests fail) | `p1_every_day_maps_to_exactly_one_zdate`: "two days share a ZDate" (left 71209, right 73414); `p2_round_trip_from_every_day`: "to_gregorian(to_zdate(d)) != d" (left `Err(OutOfRange)`, 1900-01-01); `p3_each_tshes_is_present_once_twice_or_absent…`: "tshes 29 occurs 3 times in (1900, 6, false)"; `p3_tshes_never_decrease…`: "skipped_before at 1900-01-02" (left `None`, right `Some(30)`); `p2_every_combination_of_fields_is_classified_exactly`; `p2_round_trip_from_every_valid_zdate`; `shape_census_over_the_range` (identity: left 1368, right 1168) |
| B, leap-month boundary | the leap rule (5.22) tests `{1}` instead of `{1, 2}` | red, red, red (9 of 13 fail) | `p4_leap_months_follow_the_intercalation_rule`: "leap months: measured vs eq. (5.36)" (left 38, right 75); `p4_complete_months_have_29_or_30_days`: "month (1902, 7, false) has 59 days"; `p3_each_tshes_…`: "tshes 13 occurs 3 times in (1902, 7, false)"; `p2_round_trip_from_every_day`; `p1_…`; `p3_tshes_never_decrease…`; `p2_every_combination…`; `p2_round_trip_from_every_valid_zdate`; `shape_census…` |
| C, shared wrong epoch (rule 35) | `E806.m0` one day late (`2_015_502` for `2_015_501`) | **green, green, green: 13 of 13 pass** | not caught by P1–P4, P7. Caught by the three-epoch comparison (`e806_e1927_and_e1987_give_the_same_zdate_on_every_day_of_the_range`, `the_other_epochs_invert_their_own_mapping_too`), by Janson's examples (4 tests in `worked_examples.rs`; 6 in the crate's unit tests, among them `e806_epoch_constants_match_the_traditional_forms`) |

The proptest P7 properties (`to_zdate_is_exact_inside_the_range_and_out_of_range_outside_it`, `to_gregorian_is_classified_exactly_for_arbitrary_valid_fields`) also went red on plants A and B in 3 of 3 runs each, so the boundary strategy reaches the planted edges; on plant B the shape-count test `the_boundary_strategy_has_every_shape_the_properties_depend_on` went red too. On plant A and B plants a regression seed was written in the scratch copy only (deleted with it).

## Item 6 — 001 §5.3, location independence (Appendix A, Task 7)

For each quantity this round computes, with the page. Every quotation is two sentences at most.

| Quantity | Verdict | Page | Quotation |
|---|---|---|---|
| the date mapping (Gregorian civil day ↔ `ZDate`) | **VERIFIED** | p. 6; p. 20 (Remark 6) | p. 6: "Hence, to convert a Julian date to a Julian day number, we need in practice to know both the local time the day begins and the time zone, while these are irrelevant for calculations with the Julian day number. For example, 1 January 2007 has JD 2454102, everywhere." p. 20: "We do not have to worry about when the day starts; the calendar day is from dawn to dawn, but the formulas take this into account (at least theoretically), and no further modification is done. In particular, no calculation of sunrise is required (as it is in Indian calendar calculations [2])." |
| tshes, `is_doubled`, `skipped_before` | **VERIFIED** (by the same two statements: the labels are assigned by one rule from `true_date` and JD) | p. 20 | "A calendar day is labelled by the lunar day that is current at the beginning of the calendar day. … (Thus the JD of the calendar day is the integer part of true_date at the end of the corresponding lunar day.)" |
| month number, `is_leap_month` | **NOT DECIDED BY THE PAPER** | pp. 14–17 | no sentence says the month numbering is independent of location; eqs. (5.2)–(5.22) take a year, a month and a count and nothing else, but that is the formulas' shape, not the paper's claim |
| year: rabjung cycle, year in cycle, element, gender, animal | **NOT DECIDED BY THE PAPER** | p. 8 | "One common method, especially among Westeners, is to simply number a Tibetan year by the Gregorian (or Julian) year in which it starts (and where most of it falls)." Nothing about a place or a time zone. |

**How Janson v1 relates a calendar day to a Gregorian civil day** (pp. 6, 20–21, 27). A Tibetan calendar day runs dawn to dawn and is numbered by the Julian day number of the day it begins on, "regardless of when they begin and end" (p. 6: "Moreover, the Julian day number numbers the days regardless of when they begin and end, while the Julian date depends on the time of day, at Greenwich."); the Gregorian date of a JD is fixed by the anchors of pp. 5–6. `true_date` "should be regarded as a kind of local Julian date … so that it instead assumes integer values at local (mean) dawn" (p. 20), and `JD = ⌊true_date⌋` is "the JD of the calendar day containing the end of the corresponding lunar day" (§8, eq. (8.1), p. 27). The paper's day start is Henning's "mean daybreak = 5 am local mean solar time" and, for Lhasa, "about −1 UT, i.e. 11 p.m. UT the preceding day" (pp. 20–21); that offset matters for comparing the calendar with astronomy (§12.1, p. 35), not for computing it.

**What this means for the engine, and what it does not.** The engine's functions take a Gregorian date and a system and nothing else; no function reads a place, a time zone or a clock (002 §4.5), and the API has no such parameter. That is a property of the signatures ("by construction", METHOD rule 31). The paper settles location independence of the date mapping and the day labels; it does not settle it for the month and year fields in so many words. 001 §5.3 stays a claim verified in part; no location parameter is written, and STOP RULE (4) did not fire.

## Item 1 — the gate, hardened (R2)

### 1a. The zero-dependencies leg

Prediction 1 asked what spec 004's text-based leg (`grep -nE '^\[(dev-)?dependencies\]|^\[build-dependencies\]'` over crates/tshes/Cargo.toml) says about two planted manifests. MEASURED, on scratch copies, running the leg exactly as 004 committed it (the 59bad84 script):

- a path dependency written as a `[dependencies.planted-dep]` table: `PASS zero_deps_check`, exit 0 — unseen;
- a path dependency under `[target.'cfg(unix)'.dependencies]`: `PASS zero_deps_check`, exit 0 — unseen.

The leg now reads the manifest as Cargo reads it: `cargo metadata --manifest-path <Cargo.toml> --no-deps --format-version 1`, then `jq` over `.packages[] | select(.name == "tshes") | .dependencies[]`. A dependency whose `kind` is not `"dev"` (Cargo reports `null` for normal, `"build"` for build) fails the leg with a message naming its kind, its name and its target; dev-dependencies pass (002 §4.9). The leg also fails, by name, if `jq` is absent, if `cargo metadata` fails, or if Cargo does not list exactly one package named `tshes` (a vacuous pass cannot happen). `cargo metadata --no-deps` resolves nothing and needs no network, so a planted registry dependency is seen as well as a path one. (That `jq` is preinstalled on the hosted runner is RECALLED / UNVERIFIED; if it is not, the leg fails loudly with its own message, which is the safe direction.)

| Plant | Shape | Expected message (asserted by the self-test) | Observed (the new leg) | 004's leg on the same shape | Verdict |
|---|---|---|---|---|---|
| zd_tbl | `[dependencies.planted-dep]` table, `path = ...` | `ERROR zero-deps-check: crates/tshes has a normal dependency on planted-dep (target: any)` | same | PASSED unseen | REJECTED |
| zd_cfg | `[target.'cfg(unix)'.dependencies]` | `... normal dependency on planted-dep (target: cfg(unix))` | same | PASSED unseen | REJECTED |
| zd_build | `[build-dependencies]`, path dependency | `... has a build dependency on planted-dep` | same | rejected (text match, line 16) | REJECTED |
| zd_dev_control | `[dev-dependencies]`, path dependency | the gate passes and the leg prints `zero-deps ok: no normal or build dependency; dev-dependencies: planted-dep,proptest` (the planted line goes into the manifest's existing `[dev-dependencies]` table, which holds proptest from Item 4) | same | REJECTED (stricter than 002 §4.9) | PASS (control) |
| d4_path_dep (004's plant, kept) | `[dependencies]` path dependency | `zero-deps-check` | same | rejected | REJECTED |
| e_outside_cwd | the copy's gate run from a directory outside every workspace, `RUSTUP_TOOLCHAIN` unset | the gate passes and 6 of 6 toolchain lines are `toolchain: rustc 1.99.0` | same | n/a (unfixed gate: exit 1; fmt passed on 1.97.1) | PASS; mutation (the `cd` line deleted): FAIL, 1 pinned line of 6 |

### 1b. The caller's directory

The advisor RECALLED (UNVERIFIED) that rustup chooses the toolchain from the current directory. MEASURED: `rustup show active-toolchain` run from the scratchpad directory (outside the repository) prints `stable-x86_64-unknown-linux-gnu (default)` and `rustc -V` prints `rustc 1.97.1 (8bab26f4f 2026-07-14)`; run from the repository it prints `1.99.0-x86_64-unknown-linux-gnu (overridden by '/home/kitaeza/github/zurkhai/rust-toolchain.toml')` and `rustc 1.99.0 (b940084d7 2026-09-28)`. The claim holds. (Runs from a chosen directory use `env --chdir=<dir>`, so the session's own Bash calls carry no shell `cd`.)

Prediction 2's run: the unmodified 004 gate (every leg but the self-test) from outside the repository, MEASURED:

| Leg | Toolchain it printed | Result |
|---|---|---|
| fmt | rustc 1.97.1 (8bab26f4f 2026-07-14) | **PASS — on the wrong toolchain** |
| clippy | 1.97.1 | FAIL: `error: rustc 1.97.1 is not supported by the following packages: tshes@0.1.0 requires rustc 1.99.0` |
| test | 1.97.1 | FAIL, the same error |
| test_release | 1.97.1 | FAIL, the same error |
| build_wasm | 1.97.1 | FAIL, the same error |
| deny | cargo-deny 0.20.2 | PASS (the leg's own subshell `cd`) |
| msrv | rustc 1.99.0 (b940084d7 2026-09-28) | PASS (explicit `cargo +1.99.0`) |
| the three source greps | — | PASS |

`GATE: 10 legs run — 6 passed, 4 failed, 1 skipped`. Four cargo legs ran on 1.97.1 and failed visibly (the `rust-version` check refuses them), but `fmt` ran on 1.97.1 and passed: a leg passed on a toolchain other than the pin.

The fix: `tools/gate.sh` enters its own tree first (`cd "$GATE_ROOT"`, reason in a comment), so rustup reads the repository's `rust-toolchain.toml` whatever the caller's directory is; `GATE_ROOT` is made absolute; the deny leg's own subshell `cd` is removed as redundant. MEASURED after the fix, from outside: `GATE: 11 legs run — 11 passed, 0 failed, 1 skipped`, each cargo leg printing `rustc 1.99.0 (b940084d7 2026-09-28)`. The plant `e_outside_cwd` can fail (METHOD rule 31): a scratch copy of the fixed tree with the `cd` line deleted, self-test run from outside, gave `FAIL plant e_outside_cwd: gate exit 1; 1 pinned toolchain lines and 6 toolchain lines in total, 6 expected of each` (the 1 is `msrv`, which names its toolchain itself).

### 1c. DEBT.md

D-8 (Kind: TOOLING) filed: `actions/checkout@v4` is pinned by tag, not SHA. TRIGGER: before the human's first push of `main` to a public remote, the hosted workflow's first run.

## Every clause of 002 §4–§6 that this round's code touches

- **§4.1 no floating point [CI]:** DONE. Every rational is an integer numerator over a stated denominator (rows 27–54); `#![deny(clippy::float_arithmetic)]` is in lib.rs; the gate's clippy leg is clean; plant b1 of spec 004's self-test still rejects a float.
- **§4.2 floor division and modulo are explicit [CI]:** DONE. Every division goes through `floor_div`, `fmod`, `amod`, `ceil_div` (`arith.rs`), each with doc tests on negative inputs (`floor_div(-7, 2) == -4`, `fmod(-7, 5) == 3`, `amod(-1, 12) == 11`, `ceil_div(-7, 2) == -3`: 4 doc tests, both profiles); `integer_division` and `modulo_arithmetic` are denied.
- **§4.3 overflow checked in every profile [CI]:** DONE. `i64` throughout, no `i128`. Bounds stated: `|year| ≤ 10^9` and the `Jdn` domain (the JD of 1 January −10^9 to 31 December 10^9, endpoints asserted by `round_trip_at_the_extremes_of_the_domain`); `true_date_numerator` is bounded for `|t| ≤ 3·10^9` and called only for `t < 10^5`; ZDates outside the Tibetan years 1899..=2100 are refused before any arithmetic. `overflow-checks = true` in release; the suite ran in both profiles (65 and 65 tests), and `true_date_stays_in_i64_and_near_the_mean_date_at_its_stated_bound` runs the stated bound at both its ends.
- **§4.4 no panics on valid input [CI]:** DONE. The deny list (`unwrap_used`, `expect_used`, `panic`, `indexing_slicing`) is in lib.rs and clippy is clean; out-of-range and invalid input return `Err`; `debug_assert!` in `arith`/`moon_tab_base`/`sun_tab_base` is backed by the exhaustive and proptest runs.
- **§4.5 no unsafe, no I/O, no clock, no locale [CI]:** DONE. `#![forbid(unsafe_code)]`; the gate's `no_std_check` passes on the new sources (no `std::` in `src/`: the `cfg(test)` modules are core-only).
- **§4.6 `no_std` (F6):** DONE. No alloc; the gate's `no_alloc_check` passes. The error type and every container-free API make no need for alloc.
- **§4.7 newtypes with validating constructors and private fields:** DONE. `Jdn`, `GregorianDate`, `Tshes`, `MonthNo`, `RabjungCycle`, `YearInCycle`, `ZYear`; no `From<u8>`; the constructors return `Result`; `ZYear` derives element, gender and animal so a self-contradicting year cannot be built. `Mansion`: N-A (no lunar-mansion attribute this round).
- **§4.8 every formula cites its source:** DONE. The citation table above; rows 1–66 each carry a `// Janson (arXiv:1401.6285v1) §…, eq. (…), p. …` comment or doc comment; derived constants are marked DERIVED with their derivation in a doc comment or a test (rows 2, 55, 66).
- **§4.9 dependencies:** DONE. Zero dependencies; one dev-dependency, `proptest 1.12.0` with `default-features = false, features = ["std"]`; `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok` (cargo-deny 0.20.2) with `deny.toml` unchanged.
- **§5 error handling:** DONE. One `#[non_exhaustive] enum Error` (`fault.rs`): `OutOfRange`, `SkippedDay`, `InvalidField`, plus `SystemNotImplemented` and `Internal`; `core::fmt::Display` and `core::error::Error` (test `the_error_type_displays_and_is_a_core_error`); no `thiserror`; `to_gregorian` on a skipped tshes is a typed `Err(SkippedDay)`. "Wrappers map `Error` one to one": N-A (no wrapper exists).
- **§6 testing standard:** property tests for P1–P4, P7 over the full range: DONE (exhaustive, and proptest for P7); a committed seed for a failure found: DONE (`tests/p7_proptest.proptest-regressions`); fixture tests for P5/P6 reading `golden/reference/`: N-A (round 006); seeded control: DONE (Item 5: nine examples, each with a wrong-value control); both profiles: DONE; WASM smoke test: N-A (`crates/tshes-wasm` does not exist; the gate names the skipped leg).

## The predictions, scored

1. **HELD.** 004's leg passed both planted shapes (`PASS zero_deps_check`, exit 0 each).
2. **FAILED as stated.** From outside the repository four cargo legs failed visibly on 1.97.1 (clippy, test, test_release, build_wasm), as predicted, but `fmt` ran on 1.97.1 and passed, and `msrv` ran on 1.99.0 through its explicit `+1.99.0`. A leg did pass on another toolchain.
3. **HELD, with the named exception exercised.** Every constant and formula the Phugpa mapping needs is in Janson v1 (rows 9–64). STOP RULE (1) was not hit for Phugpa. The one thing Janson does not give is a Gregorian↔JD formula (Item 3's exception): it was derived and anchored on two values Janson states. Janson describes the inverse mapping (JD → Tibetan date) in words (§8, p. 27: "compute approximate true month count and day … and then search the neighbouring lunar days for an exact match") and refers to Dershowitz–Reingold for code; the search implemented is that description plus the §6 rule.
4. **HELD.** P1–P3 pass on all 73,414 days in both profiles with zero failures; P2 holds in both directions.
5. **HELD.** Predicted 74 ± 1; measured 75 leap months in the range (74 in complete months), and 75 is what eq. (5.36) gives exactly.
6. **HELD.** Predicted 1167 ± 10; measured 1,168, and the identity sums exactly (2,800 − 1,632 = 1,168 = 30 × 2,486 − 73,412).
7. **FAILED in part.** VERIFIED for the date mapping and the day labels (pp. 6, 20); NOT DECIDED BY THE PAPER for the month and leap fields and for the year fields. Nothing was REFUTED.
8. **HELD.** `cargo deny check` passes with proptest's tree under the unchanged allow list.
9. **HELD as worded, FAILED in its strong reading.** Janson v1 contains worked numerical examples of parts of the Phugpa computation (the year naming of AD 2007 with its intermediate values; the epoch-shift arithmetic of Remarks 9–11; Table 6) but **no example that carries a date through the lunar-day formulas** (Item 5).

## D-1 and D-2; the open-forks audit; the marker greps

**D-1 (no client defaults `System`): OPEN, respected.** `System` is an explicit parameter of `to_zdate` and carried by `ZDate`; `Default` is not implemented for it; no wrapper or client exists.

**D-2 (the glossary): PAID 2026-11-10, 4935c36.** `docs/glossary.md` created from Janson v1 and 001/002; the entry was marked PAID in e764831 (a commit cannot name its own hash).

**The open-forks audit (METHOD rule 60):** `expected = previous OPEN count + forks raised + gated findings filed with options named and none chosen − rulings recorded`, then `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`. Re-measured at the start (not copied): 8 at 59bad84 (F2, F4, F5, F7, F8, F9, F10, F11 OPEN; F1, F3, F6, F12, F13 RULED). No fork was raised this round, none ruled:

| Commit | Expected | Measured |
|---|---|---|
| c3067aa (the prompt) | 8 + 0 + 0 − 0 = 8 | 8 |
| 46ccfa7 (Item 1) | 8 | 8 |
| 801b79e (Items 2–5) | 8 | 8 |
| 4935c36 (named constants, glossary) | 8 | 8 |
| e764831 (D-2 PAID) | 8 | 8 |
| the final commits | 8 | 8 |

**The grep of 001 and 002 for forks with no ruling (METHOD rule 59):** `grep -nE '\bF[0-9]+\b' docs/specs/001-init-spec.md docs/specs/002-rust-code-standard.md`, compared with `FORKS.md`. Hits: F1 (ruled), F2 (code half ruled; the data licence is OPEN and no data is committed), F3 (ruled), F4 (OPEN; no datsan contact in this round), F5 (OPEN; no localised name is written), F6 (ruled (a): this round builds `no_std` without alloc), F7 (OPEN; no FFI crate), F8 (OPEN; no rules crate), F9 (OPEN; the golden `skipped` column: nothing under `golden/` is written, and `ZDate::skipped_before` follows 001 §5.1 as written), F10 (OPEN; no outputs of the unknown-system app recorded), F9/F10 govern data this round does not create. No fork is built through.

**The marker (METHOD rule 58).** The round's marker is a single token chosen before the first edit and absent from the prompt file; this report never spells it. The grep: `grep -rn --exclude-dir=.git --exclude-dir=target --exclude-dir=.codewhale --exclude-dir=prompts -e <the round's marker> <WORKDIR>`, expected count in brackets. Before c3067aa: 0 hits (the tree and the prompt) [0]. Before 46ccfa7, 801b79e, 4935c36, e764831: exactly 1 hit each, the placeholder on this report's `gate:` line [1]. Before the final commit: 0 hits, the placeholder replaced by the measured gate line [0].

## The prompt's questions, answered

- *Item 0: verify 005 is free; re-measure the OPEN count; the marker.* `ls docs/specs/*.md` showed 001–004 and INDEX/TEMPLATE: free. OPEN count 8 (above). Marker: chosen, not spelled.
- *Item 1a: what does 004's leg say about the two planted manifests?* It passes both (Item 1a).
- *Item 1b: what does each cargo leg do outside the repository, and which toolchain does it print?* Table in Item 1b.
- *Item 2: the row.* In `docs/sources.md`: file, citation with v1, `stat` modification date labelled as such and "placed in the library by the human", sha256, the arXiv licence, "Trusted for" copied from the Bibliography row; the Bibliography row's "In the library" is `yes (v1)`. Page 1 carries `arXiv:1401.6285v1 [math.HO] 24 Jan 2014`; 86 pages.
- *Item 3: how is an unimplemented system spelled?* `Error::SystemNotImplemented`: `to_zdate(.., Genden|Tsurphu)` and `to_gregorian` of a `ZDate` that carries one return it; nothing is computed; the tests `genden_not_yet_supported`, `tsurphu_not_yet_supported` pin the gap and name `DEBT.md` D-9.
- *Item 3: Gregorian ↔ JDN, cited or derived?* Derived (row 2), anchored on two JDs Janson states (§2, p. 5 fn 8; p. 6), tested against a day-by-day count of the whole range and against the extremes of the domain.
- *Item 4: P4's expected leap-month count, derived from Janson's rule.* 75, eq. (5.36); measured 75.
- *Item 4: behaviour outside the range, stated and asserted.* P7 row above.
- *Item 5: sections searched; examples; control.* Item 5.
- *Item 6: verdicts, and how Janson relates a calendar day to a Gregorian civil day.* Item 6.
- *Item 7: the glossary and D-2.* `docs/glossary.md`; D-2 PAID (4935c36).
- *Item 8: the grep of 001 and 002; the gate; the toolchain each leg ran on.* Above and below.

## The stop rule that governed the round

STOP RULE (1): a constant or formula that Phugpa's mapping needs but Janson v1 does not state. Not hit: every constant and formula is stated (rows 9–64); the one exception it names, Gregorian ↔ JDN, was handled as Item 3 prescribes. (2): a property fails and the cause is not a coding slip traceable to a line against the paper. Not hit: two failures occurred and both were slips traceable to a line of *test* code, none in the engine and none a constant (below, "Forks"). (3): deny rejects the dev-dependency tree. Not hit (prediction 8). (4): Janson v1 says a computed quantity depends on location or time zone. Not hit: no sentence says so. (5): a network fetch, read or install outside GRANTS, or a denied permission. One permission was denied (an `rm -rf` of my own old scratch directories, "Forks" item 12) and was reported, not worked around; no read or fetch outside GRANTS was needed. Otherwise not hit: reads were the PDF through the Read tool, `sha256sum` and `stat`; `pdfinfo` was used once for the page count (see "What the prompt got wrong"); the network was used only by cargo to fetch proptest's tree. (6): out of scope, not written: Genden or Tsurphu arithmetic, any 001 §5.2 attribute beyond `ZDate`, anything under `golden/`, localised names. The weekday appears only in test code, as Janson's eq. (9.1), to check the paper's own dates; the engine has no weekday.

## Diff scope

(Figures against `main` at the final tip are in "Numbers that must sum".) Files touched, by category:

- **Gated, `crates/tshes/**`, under accepted specs 001 §5.1, §6.4 and 002: YES.** New: `src/arith.rs`, `calendar.rs`, `civil.rs`, `fault.rs`, `naming.rs`, `phugpa.rs`; `tests/common/mod.rs`, `civil.rs`, `epochs.rs`, `exhaustive.rs`, `p7_proptest.rs`, `system_not_implemented.rs`, `worked_examples.rs`. Changed: `src/lib.rs`, `Cargo.toml` (the dev-dependency).
- This round's own `docs/specs/005-phugpa-core.md`, `docs/specs/prompts/005-phugpa-core.md` and its row in `docs/specs/INDEX.md`: not changes to an accepted spec.
- Not gated: `tools/gate.sh`, `tools/gate-self-test.sh`, `Cargo.lock`, `DEBT.md`, `docs/sources.md`, `docs/glossary.md`.
- `golden/**`, `docs/METHOD.md`, `GATED.md`: untouched.
- **Files the test framework wrote:** `crates/tshes/tests/p7_proptest.proptest-regressions`, one seed (`7af0e37f…`, a failure of the test's own model: see "Forks"), kept per 002 §6 and named in the commit message of 801b79e. The scratch plants wrote seeds only inside their own scratch copies, which are not in the tree.
- **Scratch outside the tree:** the plant copies, the unfixed-gate copies and the gate logs under the session scratchpad; the self-test's own directories under the system temp directory, deleted by the script on exit.

## Forks encountered

No fork was raised and none is OPEN from this round. Resolved without escalating, as "considered X, chose Y because Z" (ROUNDS §6: implementation shape):

1. **Which epoch does the engine use?** Considered E806, E1927, E1987. Chose E806: Janson gives its constants as the base of his formulas (§7, p. 21), and the other two, all independently stated, are carried as `Epoch` constants so the tests can compare. The three give one calendar over all 73,414 days.
2. **`is_doubled`: the first or the second of the two days?** 001 §5.1 says the second occurrence; Janson's "leap day" is the first (§6, p. 20). Chose 001 (an accepted spec: a forced answer is not a fork) and recorded the difference in the glossary.
3. **Does `ZDate` carry `system`?** The prompt's field list omits it; 001 §5.1 lists it first, and `to_gregorian(zdate)` takes no system parameter. Chose to carry it.
4. **How strict is `to_gregorian`?** Considered ignoring `skipped_before` on input (lenient) or requiring it to equal the engine's. Chose strict (`InvalidField` unless the ZDate is exactly what the engine reports for that day), so that an `Ok` implies P2 for every accepted ZDate. Cost: a hand-made ZDate with a wrong `skipped_before` is refused; F9 (OPEN) concerns the golden column, not this.
5. **The supported range.** Considered a wider range (the formulas are total) or exactly the 001 minimum. Chose exactly 1900-01-01..=2100-12-31 with `OutOfRange` outside: a range claim needs a test at its edges (METHOD rule 13), and the edge tests exist for these two edges, the Gregorian domain's ends, and nothing in between.
6. **The Gregorian and JD domains.** The first cut had a `Jdn` limit (4·10^11) wider than the image of the Gregorian year limit (±10^9), so `to_gregorian` could fail for a valid `Jdn`; found by the extreme-domain test, fixed by making the `Jdn` domain exactly the days of the years −10^9..=10^9 (endpoints asserted).
7. **proptest's features.** `default-features = false, features = ["std"]`: the smallest tree that gives the strategies and the persistence; deny passed, so deny.toml was not touched.
8. **An error variant for "not implemented", and one for a broken invariant.** Added `SystemNotImplemented` and `Internal` to the enum (002 §5 says "variants such as"): a wrong error name would mislead, and `Internal` makes an invariant break loud instead of a panic or a wrong date.
9. **Unit tests inside `src/`.** The gate's `no_std_check` greps `src/` for `std::`; the `cfg(test)` modules there use only `core` (`assert!`, `assert_eq!`, slices, iterators) so private constants and `end_day` are reachable without weakening the check.
10. **Two test failures, neither in the engine.** (a) `p4_leap_months_follow_the_intercalation_rule` first failed because the test floored `r/2` before subtracting from 33, where eq. (5.42) floors `33 − r/2` (`p. 19`): the engine's leap months were the right ones, and the test's formula gave a month 13 for the year 1907, which no month number can be; fixed in the test (row T2). (b) The proptest `to_gregorian_is_classified_exactly_for_arbitrary_valid_fields` failed at `p7_proptest.rs:248` on `(cycle 15, position 33, month 1, tshes 1, doubled true)` (Tibetan year 1899, outside the range): the test model had pre-decided `OutOfRange`/`SkippedDay` for a month that exists but lies outside the range, while the documented order of checks (`phugpa::to_jdn`) refuses a doubled flag on a non-repeated tshes first, with `InvalidField`. The engine behaved as documented; the model was widened to the three defined outcomes, never `Ok`. Both are coding slips traceable to a line, not STOP RULE (2) cases. The seed is committed.

11. **The self-test's disk use.** Considered sharing one target directory across the plant copies (rule 64 says each tree has its own), shrinking the plants' legs, or deleting each copy's target directory after its run. Chose the deletion: it keeps rule 64 and every plant's full set of legs, and costs only rebuild time.
12. **A denied `rm`.** After the first final gate run I tried to `rm -rf` my own old scratch plant directories under the scratchpad; the harness denied it. Reported here, not worked around (METHOD rule 67): the directories are outside the repository and the disk was not full; they remain.

## Latitude taken beyond or differently from the prompt

1. **Public `phugpa::Epoch`, `phugpa::to_zdate`, `phugpa::to_jdn`:** API surface beyond the prompt, so the three-epoch comparison (the strongest outside check this round has) can run through the public API. They default to nothing; `System::Phugpa` uses E806.
2. **`Error::SystemNotImplemented` and `Error::Internal`** (above).
3. **The Gregorian domain bound** (±10^9 years) and the `Jdn` domain (above): stated bounds so that `i64` is provably enough (002 §4.3).
4. **`DEBT.md` D-9 and D-10 filed:** D-9 because METHOD rule 33 needs an id for the test that pins the gap; D-10 because a source discrepancy that later rounds could trip over is a negative entry (rule 53).
5. **Named constants and exact-shape citation comments:** the prompt's comment shape `// Janson (arXiv:1401.6285v1) §<section>, eq. (<n>), p. <page>` is applied to every formula and named constant; the table has one row per formula and per constant.
6. **Oracle tests beyond the prompt's list:** the Appendix C route (C.22), eqs. (5.24), (5.36), (5.41), (5.42), the independent decode of Janson's mixed-radix forms, and the exact-`a₂` search that reproduces Janson's 9 and 8.
7. **Weekday in test code** (eq. (9.1), Table 5) to check the paper's own dates; no weekday in the engine.
8. **`pdfinfo` once**, for the page count Item 2 asks for (read-only metadata; no content was read through it).
9. **Bash calls that began with a `cd` to the repository root** (CLAUDE.md hard rule 16): six of this session's Bash calls (counted from the session's own commands) carried a leading `cd /home/kitaeza/github/zurkhai` that the rule forbids; the shell already started there, so no result depended on it. Every later call used absolute paths and `--manifest-path`/`git -C`; runs from another directory used `env --chdir=`.
10. **The self-test's per-plant cleanup** (the by-catch above): a change to `tools/gate-self-test.sh` beyond the prompt's Item 1, made and the plant fix for the `zd_dev_control` control, because the final gate showed the self-test could not complete on this machine.

## Numbers that must sum

- **Days:** 73,414 = 73,412 (days in the 2,486 complete months) + 2 (one day in each of the two partial months at the edges: 1900-01-01, the last day of the leap month 11 of 1899; and 2100-12-31, the first of month 11 of 2100).
- **Months:** 2,488 = 2,486 + 2; 2,486 = 1,168 (29 days) + 1,318 (30 days); leap months 75 = 74 (complete) + 1 (partial).
- **The identity:** skipped 2,800 − doubled 1,632 = 1,168 = 30 × 2,486 − 73,412 (= 74,580 − 73,412).
- **Tests per profile:** 65 = 17 (crate unit tests) + 8 (`civil`) + 5 (`epochs`) + 13 (`exhaustive`) + 1 (`overflow_tripwire`) + 5 (`p7_proptest`) + 4 (`system_not_implemented`) + 8 (`worked_examples`) + 4 (doc tests); debug 65 passed, 0 failed; release 65 passed, 0 failed.
- **Self-test plants:** 26 = 21 (spec 004) + 5 (`zd_tbl`, `zd_cfg`, `zd_build`, `zd_dev_control`, `e_outside_cwd`).
- **Citation rows:** 75 = 66 (the engine) + 9 (the oracles); 73 MATCH + 2 derived and tested = 75.
- **Gate legs:** 11 run + 1 skipped = 12 named; 11 passed, 0 failed.
- **The diff against `main`, at the final tip, summing:**

Per file, insertions and deletions, `git diff --numstat main` at the final tip (working tree = the final commit):

```
insertions  deletions  file
       133          0  Cargo.lock
        25          1  DEBT.md
         5          2  crates/tshes/Cargo.toml
        70          0  crates/tshes/src/arith.rs
       155          0  crates/tshes/src/calendar.rs
       218          0  crates/tshes/src/civil.rs
        48          0  crates/tshes/src/fault.rs
        56          6  crates/tshes/src/lib.rs
       246          0  crates/tshes/src/naming.rs
       854          0  crates/tshes/src/phugpa.rs
       165          0  crates/tshes/tests/civil.rs
        70          0  crates/tshes/tests/common/mod.rs
       155          0  crates/tshes/tests/epochs.rs
       484          0  crates/tshes/tests/exhaustive.rs
         7          0  crates/tshes/tests/p7_proptest.proptest-regressions
       313          0  crates/tshes/tests/p7_proptest.rs
        74          0  crates/tshes/tests/system_not_implemented.rs
       307          0  crates/tshes/tests/worked_examples.rs
        64          0  docs/glossary.md
         2          2  docs/sources.md
       541          0  docs/specs/005-phugpa-core.md
         1          0  docs/specs/INDEX.md
       311          0  docs/specs/prompts/005-phugpa-core.md
        98          2  tools/gate-self-test.sh
        34         11  tools/gate.sh
      4436         24  TOTAL (25 files)
```

- engine sources `crates/tshes/src/`: +1647 −6
- engine tests and the seed `crates/tshes/tests/`: +1575 −0
- manifests `Cargo.lock`, `crates/tshes/Cargo.toml`: +138 −2
- gate `tools/`: +132 −13
- records `DEBT.md`, `docs/sources.md`, `docs/glossary.md`, `docs/specs/INDEX.md`: +92 −3
- this report and the committed prompt `docs/specs/`: +852 −0
- Subtotals summing: insertions 1647 + 1575 + 138 + 132 + 92 + 852 = 4436 (total 4436); deletions 6 + 0 + 2 + 13 + 3 + 0 = 24 (total 24).
- Gated paths touched: yes (`crates/tshes/**`, under accepted specs 001 and 002, named in the prompt); `golden/**`, `docs/METHOD.md`, `GATED.md`: no.
Sums: see above.

## What the prompt got wrong

- **Prediction 2** (the cargo legs "do not run on 1.99.0, and they fail visibly rather than pass on another toolchain"): `fmt` ran on 1.97.1 and passed. The fix and a plant for it are in Item 1b.
- **Prediction 9 / Item 5's premise** that Janson v1 holds "a date carried through the formulas with its intermediate values": it does not. The paper has year-naming and constant-shift examples and dated data points (Item 5); the lunar-day formulas have no worked date.
- **The ZDate field list in Item 3** omits `system`, which 001 §5.1 includes and which `to_gregorian(zdate)` needs.
- **GRANTS** names the Read tool and `sha256sum` for the artifact, but Item 2 asks for the page count, which neither gives; `pdfinfo` was used once (read-only metadata).
- **Item 7** asks to mark D-2 PAID "with the date and the commit" in the same round: a commit cannot hold its own hash; the entry was marked in the next commit, citing the glossary's commit (4935c36).
- **Item 6's prediction 7** read "VERIFIED for every quantity": the paper decides the date mapping and the labels but not, in so many words, the month and year fields.
- Nothing else found: STOP RULE (1) is correctly scoped with the Gregorian↔JDN exception, the OPEN count was 8, the preconditions held (main at 59bad84, 005 free, the artifact present and identified), and `deny` held.

## By-catch

- **D-10** (filed): Janson v1, page 1, gives "Sunday" for 31 December 2007, which eq. (9.1) makes a Monday; the date triple on that line agrees with the engine, and the other date line (8 January 2014, Wednesday) agrees in every part.
- **D-9** (filed): Genden and Tsurphu are not implemented; the tests pin the gap.
- **The final gate found two defects in the self-test, both fixed.** The first final gate run, on 33d1091, passed every leg but the self-test (24 of 26 plants rejected as expected). (1) `zd_dev_control` failed: the plant appended a second `[dev-dependencies]` header to a manifest that has had one since Item 4 added proptest, a duplicate key, so the copy could not load (reproduced by hand; the self-test's earlier full run predates proptest, which is why it had passed). (2) `e_outside_cwd`, and the plants after it, failed with `error: failed to build archive … Disk quota exceeded (os error 122)`: each plant copy has its own `CARGO_TARGET_DIR` (METHOD rule 63's sibling, rule 64), and with proptest in the tree each compiles its own dev-dependencies, so 26 copies together exceeded the quota. Neither is a code failure nor a flake (rule 63). Fixes: the plant now adds its line inside the existing table and expects `dev-dependencies: planted-dep,proptest`; `run_gate` deletes a copy's target directory as soon as that gate has run (the log stays), so peak use is one, and the `e_outside_cwd` block does the same. A second run, on 2ca193d, had the disk fixed and still failed on (1) alone (25 of 26 rejected as expected, 0 "Disk quota" lines), which is how (1) was told apart from (2). The gate was run again on the final tip, and that run is the one this report quotes. The duration of the gate is in the session summary, not in this file. Not filed as debt: both are fixed.
- The proptest regression file is written next to the test (`tests/p7_proptest.proptest-regressions`), not in a `proptest-regressions/` directory: the persistence mode falls back to that because the test is in `tests/` (proptest printed: "SourceParallel set, but failed to find lib.rs or main.rs"). Nothing to fix: 002 §6 asks only that seeds be committed.

## Debt and forks

Filed: **D-8** (TOOLING), **D-9** (COVERAGE), **D-10** (PROVENANCE). Paid: **D-2** (4935c36). Unchanged: D-1 (OPEN, respected), D-3, D-4, D-5, D-6, D-7. FORKS.md: no row added, none ruled; OPEN count 8 at every commit (audit above). The marker greps: above.

## Held

Identified items only: D-1, D-3, D-4, D-5, D-6, D-7, D-8, D-9, D-10 (OPEN); F2, F4, F5, F7, F8, F9, F10, F11 (OPEN); P5 and P6 wait for round 006; the wasm-pack leg stays SKIPPED until `crates/tshes-wasm` exists (named in the gate line).

## The gate (Item 8, the final run)

The gate ran on the final committed tree (the tip; the tip changes the self-test script (the cleanup and the control plant) and this report, and nothing under `crates/`; its parent is 2ca193d, not 33d1091), in both profiles, with no edits in flight, from outside the repository (the directory Item 1b proved: the session's scratchpad), as one command: `env --chdir=<scratchpad> bash /home/kitaeza/github/zurkhai/tools/gate.sh`, every leg including the self-test. Two earlier runs (on 33d1091 and 2ca193d) failed in the self-test for the reasons given in the by-catch section; they are not the gate line below.

Summary line, quoted from the gate's own output:

```
GATE: 11 legs run — 11 passed, 0 failed, 1 skipped
```

The skipped leg, named with its reason: wasm-pack — `crates/tshes-wasm` does not exist (002 §7: this leg runs once the crate exists; METHOD rule 65).

The toolchain each leg ran on, quoted from each tool's own output (METHOD rule 23): fmt, clippy, test, test_release, build_wasm and msrv each printed `toolchain: rustc 1.99.0 (b940084d7 2026-09-28)`; the deny leg printed `tool: cargo-deny 0.20.2` and `advisories ok, bans ok, licenses ok, sources ok`; the zero-deps leg printed `zero-deps ok: no normal or build dependency; dev-dependencies: proptest`; the self-test leg printed `tool: bash GNU bash, version 5.3.9(1)-release (x86_64-pc-linux-gnu)` and `SELF-TEST: 26 plants — 26 rejected with their named diagnostic, 0 failures`.

Test counts, both profiles, summed from each `test result:` line of the run (17 crate unit tests, 8 `civil`, 5 `epochs`, 13 `exhaustive`, 1 `overflow_tripwire`, 5 `p7_proptest`, 4 `system_not_implemented`, 8 `worked_examples`, 4 doc tests): debug 65 passed, 0 failed; release 65 passed, 0 failed. The msrv leg runs the same toolchain as the pin and therefore the same tests; its 65 are not counted again.

## The squash commit message

```
spec 005: the Phugpa core, Gregorian <-> ZDate from Janson v1, and the gate hardened

The gate: tools/gate.sh reads crates/tshes's dependencies through cargo metadata
(a [dependencies.<name>] table and a target-cfg section both passed spec 004's text
leg unseen; dev-dependencies pass, 002 s4.9) and enters its own tree, so rustup reads
the pin whatever the caller's directory (measured: fmt passed on 1.97.1 from
outside). The self-test has 26 plants.

crates/tshes: the Phugpa mapping from Janson (arXiv:1401.6285v1): year naming (4.1)-
(4.4), months (5.2)-(5.22), lunar days (7.1)-(7.22) and (8.1), E806 constants; the
Gregorian <-> JDN conversion is derived from the leap-year rule and anchored on
Janson's JD 2454102 and RD 1 = JD 1721426; one Error enum; Genden and Tsurphu return
Err(SystemNotImplemented). Tests: P1-P4 exhaustive over 1900-2100, P7 by proptest (a
dev-dependency; cargo deny clean), the E806/E1927/E1987 cross-check, Janson's worked
examples with seeded controls; planted off-by-ones red three runs in three, and a
shared wrong epoch passes P1-P4 (rule 35). docs/glossary.md created; docs/sources.md
holds the Janson v1 row. The test framework wrote
crates/tshes/tests/p7_proptest.proptest-regressions (seed 7af0e37f...).

DEBT: D-2 PAID; D-8, D-9, D-10 filed. FORKS: none raised, none ruled.

Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>
```
