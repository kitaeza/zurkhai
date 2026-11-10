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
| Genden | 001 §4, §5.1 | "New Genden (Mongolian *Tögs buyant*)", Appendix A.3, p. 41 | `System::Genden` (not implemented: `DEBT.md` D-9) |
| Tsurphu | 001 §4, §5.1 | "Tsurphu (*Tsurluk*)", Appendix A.2, p. 38 | `System::Tsurphu` (not implemented: `DEBT.md` D-9) |
| ZDate | 001 §5.1 | not Janson's term | `ZDate` |

## Arithmetic

| Project spelling | Fixed by | Janson v1's spelling, and page | Used in code as |
|---|---|---|---|
| Julian day number | Janson v1 | "Julian day number", abbreviated JD, §2, p. 5; RD (Dershowitz–Reingold) is related by JD = RD + 1721425, footnote 8, p. 5 | `Jdn` |
| epoch | Janson v1 | "epoch" (month `M₀` = 3 of year `Y₀`), §5, p. 14; E806, E1927, E1987 | `Epoch` |
| true month count | Janson v1 | "true month count" `n`, §5, p. 15 (the "true month", *zla-dag*, p. 14) | the count `n` in `phugpa` |
| intercalation index | Janson v1 | "intercalation index" `ix`, p. 14–15; eq. (5.7) | `β*` in the `Epoch` data |
| mean date | Janson v1 | "mean date (*gza' bar pa*)", eq. (7.1), p. 21 | in `true_date_numerator` |
| true date | Janson v1 | "true date (*gza' dag*)", eq. (7.22), p. 24 | `true_date_numerator`, `end_day` |
| mean sun | Janson v1 | "mean longitude of the sun (*nyi ma bar pa*)", eq. (7.5), p. 22 | in `true_date_numerator` |
| anomaly of the moon | Janson v1 | "anomaly of the moon (*ril-po dang cha-shas*)", eq. (7.11), p. 23 | in `true_date_numerator` |
| equation of the moon, of the sun | Janson v1 | "the equation of the moon (*zla rkang*)", eq. (7.17); "of the sun (*nyi rkang*)", eq. (7.20), p. 24 | `moon_tab`, `sun_tab` |
| tshes-day label | Janson v1 | "A calendar day is labelled by the lunar day that is current at the beginning of the calendar day", p. 20 | `label_of_day` |

## Hazards the glossary records

- **"Lunar day".** Janson's "lunar day" is the tshes interval. Western astrological "lunar days" are a different quantity and are never mixed with tshes (001 §2, §6.3).
- **Leap day versus doubled.** Janson's leap day is the first of two days with one number; 001's `is_doubled` is true on the second. Both are recorded; the code follows 001.
- **Month numbering.** A leap month takes the number of the *following* month in Phugpa (Janson v1, p. 11); the Bhutanese version does the opposite (p. 11, footnote 14; Appendix A.4, p. 44).
