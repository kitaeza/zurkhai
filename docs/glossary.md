# Glossary — one spelling per calendar term

Created 2026-11-10 by spec 005, the first round that names calendar terms in code (`DEBT.md` D-2; 002 §8).

**The rule** (002 §8): the code, its doc comments and the docs use the **project spelling** in the first column and no other (`tshes`, never `tithi` or `tsé`). Where 001 or 002 already spell a term, the project spelling is theirs. Every other term takes the spelling of S. Janson, "Tibetan calendar mathematics", arXiv:1401.6285v1 ("Janson v1"), with the page it is given on. Janson's own spelling is recorded beside the project's where the two differ. Localised names (Russian, Buryat, Tibetan renderings) are out of scope: `FORKS.md` F5 is OPEN.

Pages are the printed page numbers of Janson v1 (the PDF's page index equals them). Every spelling below was read on the page named, this round (FETCHED from the library copy, `docs/sources.md`).

## Calendar structure

| Project spelling | Fixed by | Janson v1's spelling, and page | Used in code as |
|---|---|---|---|
| tshes | 001 §5.1 (`tshes`), 002 §8 | "lunar day (*tshes-zhag*, Sanskrit *tithi*)": the days "are numbered by 1–30", p. 7 | `Tshes`, `ZDate::tshes` |
| lunar day | Janson v1 | "lunar day", p. 7: 1/30 of a lunar month, the time the elongation of the moon increases by 12°. Its number is the tshes. | `lunar_day` (the count `30n + d`) |
| calendar day | Janson v1 | "calendar day (*gza'* or *nyin-zhag*)", p. 7: dawn to dawn, 24 hours | the Gregorian day / `Jdn` |
| month | 001 §5.1 (`month`) | "calendar month (*zla-ba*)": 29 or 30 calendar days, p. 7; the regular months "are numbered 1–12", p. 11 | `MonthNo`, `ZDate::month` |
| lunar month | Janson v1 | "lunar month (*tshes-zla*)": new moon to new moon, p. 7 | n/a |
| leap month | 001 §5.1 (`is_leap_month`) | "leap (or intercalary) month", p. 11: takes the number of the following month; the first of the two months with that number | `ZDate::is_leap_month` |
| doubled | 001 §5.1 (`is_doubled`) | a "repeated" date, p. 20: two calendar days carry the same number | `ZDate::is_doubled`: true on the **second** of the two days (001) |
| leap day | Janson v1 | "leap day", denoted "Extra" in almanacs, p. 20: the **first** of the two days with the same number | not a field; `ZDate::is_doubled` is *not* this day |
| skipped | 001 §5.1 (`skipped_before`) | a "skipped" date, p. 20: no calendar day carries the number | `ZDate::skipped_before`, `Error::SkippedDay` |
| calendar year | Janson v1 | "calendar year": 12 or 13 calendar months, p. 7 | n/a |
| Tibetan year number | Janson v1 | a Tibetan year numbered by "the Gregorian (or Julian) year in which it starts", p. 8 | `ZYear::number` |
| Losar | Janson v1 | "*Losar*; the Tibetan New Year", p. 9 | docs only |

## The year

| Project spelling | Fixed by | Janson v1's spelling, and page | Used in code as |
|---|---|---|---|
| rabjung cycle | 001 §5.1 (`rabjung_cycle`) | "rab byung" (the "Prabhava cycle"), numbered from the cycle that began in AD 1027, p. 8, eq. (4.1) | `RabjungCycle`, `ZYear::rabjung_cycle` |
| year in cycle | 001 §5.1 (`year_in_cycle`) | "year *n* in cycle *m* (with 1 ≤ *n* ≤ 60)", p. 8 | `YearInCycle`, `ZYear::year_in_cycle` |
| element | 001 §5.1 | "element": wood, fire, earth, iron, water, p. 9 and Table 2, p. 10 | `Element` |
| gender | 001 §5.1 | "gender": male, female, Tables 2 and 3, p. 10 | `Gender` |
| animal | 001 §5.1 | "animal": Mouse … Pig, Table 3, p. 10 | `Animal` |

## Systems

| Project spelling | Fixed by | Janson v1's spelling, and page | Used in code as |
|---|---|---|---|
| Phugpa | 001 §4, §5.1 | "Phugpa (*phug-lugs*)", Appendix A.1, p. 38 (the "Phugpa version" is described in the main text, p. 1) | `System::Phugpa` |
| Genden | 001 §4, §5.1 | "New Genden (Mongolian *Tögs buyant*)", Appendix A.3, p. 41. The column headed "Mongolia" in Tables 6–9 (pp. 50–54) is this version: A.3 begins "The Mongolian calendar is a version of the Tibetan" and "follows … a version of the Tibetan calendar known as New Genden" (pp. 40–41), and A.13 compares "the four different versions … in Appendices A.1–A.4" under the headings Phugpa, Tsurphu, Mongolia, Bhutan (p. 50). The paper's "Mongolian" version is the project's Genden | `System::Genden` (spec 008) |
| Tsurphu | 001 §4, §5.1 | "Tsurphu (*Tsurluk*)", Appendix A.2, p. 38; the version of A.2, not the karana-Sun variant of Remark 22, p. 40 (`DEBT.md` D-19) | `System::Tsurphu` (spec 008) |
| ZDate | 001 §5.1 | not Janson's term | `ZDate` |

## Arithmetic

| Project spelling | Fixed by | Janson v1's spelling, and page | Used in code as |
|---|---|---|---|
| Julian day number | Janson v1 | "Julian day number", abbreviated JD, §2, p. 5; RD (Dershowitz–Reingold) is related by JD = RD + 1721425, footnote 8, p. 5 | `Jdn` |
| epoch | Janson v1 | "epoch" (month `M₀` = 3 of year `Y₀`), §5, p. 14; E806, E1927, E1987 | `Epoch` |
| true month count | Janson v1 | "true month count" `n`, §5, p. 15 (the "true month", *zla-dag*, p. 14) | the count `n` in `phugpa` |
| intercalation index | Janson v1 | "intercalation index" `ix`, p. 14–15; eq. (5.7) | `β*` in the `Epoch` data |
| epoch value of the intercalation index | Janson v1 | "the epoch value of the intercalation index" `β*`, A.2, p. 39 (59 for E1732, 14 for E1852; 10 for E1747, A.3, p. 42); "β* is the initial value of ix at the epoch", p. 15 | `Epoch::beta_star` |
| leap-month rule | Janson v1 | "The traditional Phugpa leap month rule": a leap month is inserted when the intercalation index is 48 or 49, (5.8), p. 15; Tsurphu: 0 or 1, (A.9), p. 39; Genden: 46 or 47, (A.17), p. 43 | `Epoch`'s rounding shift (the 17, 0, 19 of eq. (5.10) and its versions) |
| the constants β and γ, γ* | Janson v1 | β = 184 − β*, eq. (5.15), p. 17 (Phugpa) and the values 142, 187 (A.2, p. 40) and 172 (A.3, p. 43); γ and γ*, eqs. (5.30), (5.32), p. 18: 42 and 33 for Phugpa, 55 and 20 for Tsurphu and Genden | `Epoch::beta` (γ and γ* only in tests) |
| rounding shift, grain | not Janson's terms | the engine's own: the shift is the `c` of n = ⌊(67M* + β* + c)/65⌋ − [ℓ] (17, 0, 19); the grain is the denominator an epoch's `m₀`, `s₀` are held over (Tsurphu's are finer, A.2, p. 39, Remark 20) | `Epoch` fields; private |
| mean date | Janson v1 | "mean date (*gza' bar pa*)", eq. (7.1), p. 21 | in `true_date_numerator` |
| true date | Janson v1 | "true date (*gza' dag*)", eq. (7.22), p. 24 | `true_date_numerator`, `end_day` |
| mean sun | Janson v1 | "mean longitude of the sun (*nyi ma bar pa*)", eq. (7.5), p. 22 | in `true_date_numerator` |
| anomaly of the moon | Janson v1 | "anomaly of the moon (*ril-po dang cha-shas*)", eq. (7.11), p. 23 | in `true_date_numerator` |
| equation of the moon, of the sun | Janson v1 | "the equation of the moon (*zla rkang*)", eq. (7.17); "of the sun (*nyi rkang*)", eq. (7.20), p. 24 | `moon_tab`, `sun_tab` |
| tshes-day label | Janson v1 | "A calendar day is labelled by the lunar day that is current at the beginning of the calendar day", p. 20 | `label_of_day` |

## Hazards the glossary records

- **"Lunar day".** Janson's "lunar day" is the tshes interval. Western astrological "lunar days" are a different quantity and are never mixed with tshes (001 §2, §6.3).
- **Leap day versus doubled.** Janson's leap day is the first of two days with one number; 001's `is_doubled` is true on the second. Both are recorded; the code follows 001.
- **Month numbering.** A leap month takes the number of the *following* month in Phugpa (Janson v1, p. 11, a statement about the Tibetan calendar generally), and Janson v1 gives no exception for the other two versions it describes as differences from Phugpa: Tsurphu (A.2, p. 39: the regular month's true month count is the rounded-down true month and "for a leap month we further subtract 1", so the leap month has the number of, and precedes, the regular month with that number) and Genden (A.3, p. 43: the same change to (5.9) and Remark 4, "with 48 replaced by 46", so the same shape). The general rule (5.24), p. 17, with the version's β, which Janson says holds for the other versions (p. 18), puts the leap month at the number of the following month. The Bhutanese version does the opposite (p. 11, footnote 14; Appendix A.4, p. 44) and is not implemented. Tables 7–9 (pp. 52–54) agree on every year: the engine's leap and regular months are checked against them (spec 008). One caveat for Genden: "The months [of the Mongolian calendar] are not numbered, but are named" (A.3, p. 43); the number 1–12 is Janson's computational numbering, which his Tables 7–8 use for the Mongolia column, and the month names (A.3, Appendix E.2) are not implemented: out of scope for spec 008, and localised names are `FORKS.md` F5 (OPEN).
- **"Mongolia" and Genden.** The column headed "Mongolia" in Janson v1's Tables 6–9 is the Genden version (see the Systems table). The fixtures in `golden/reference/` name it `genden` and record the heading as printed in each meta's `location:`.
