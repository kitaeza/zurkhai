# golden/buryat — the formats

Buryat golden data (001 §6.2): the tshes of every printed day of calendars published by datsans of the Buddhist Traditional Sangha of Russia, as transcribed by spec 013 (M0, 001 §10). Each file is one calendar edition, tier T1 (001 §6.1; `FORKS.md` F29, F30). They are evidence of Buryat practice and are never compared here with any engine system: that is the Q1 verdict round's (`docs/ROADMAP.md`, q1-verdict). The files hold facts only (dates, numbers, the printed names of lunations), never a calendar's design or prose (001 §2). Licence: `../LICENSE.md` (CC BY 4.0), which grants no right that a source's own terms withhold; each file's rights are in its meta file. Admission: `FORKS.md` F32 (membership), F33 (Ivolginsky), F34 (Aginsky), F35 (Gunzechoinei and Khoymorsky: no rows; `DEBT.md` D-34).

## Rules for every file

These are the rules of `../reference/README.md`, adopted for this directory.

- CSV, UTF-8, LF line endings, one header line, no quoting (no field holds a comma: a comma in a note is written `;` or left out).
- Dates are ISO 8601 (`YYYY-MM-DD`), one row per calendar day, in date order, with no gap. Booleans are `true` or `false`. An empty field means "none".
- Every CSV has a sibling `<name>.meta.yaml`, one `key: value` per line, the keys below in this order. The values are plain text: a value may hold `: ` (for example "Discrepancies between the passes: 0"), so a meta file is read line by line, not as strict YAML; no value holds a comma (a comma inside a quotation is written `;`).
- A source's printed label is quoted in the notes with its guillemets; the day-mark letters and icons of a calendar (Д, Б, Т, МХ, the scissors and the aeroplane) are L2 matter and are not transcribed.

## The columns (001 §6.2)

`gregorian_date,tshes,month,is_leap_month,is_doubled,skipped,notes`

- **`tshes`**: the lunar day number 1 to 30 as printed (Janson's "tithi"; never a Western "lunar day", 001 §2).
- **`is_doubled`**: true on the second of two consecutive days that print the same tshes (001 §5.1); false otherwise, and on the first row.
- **`skipped`**: the tshes number omitted immediately before this day (`FORKS.md` F9 (b), mirroring `ZDate.skipped_before`), empty where none was omitted. None of the three calendars prints a skip marker, so every value is **derived** from the neighbouring days (the previous day's tshes plus 2, modulo 30); it is empty on the first row of each file, whose predecessor is not in the source, and a note says so.
- **`month`** and **`is_leap_month`** (R8 / `FORKS.md` F36, F31): per **lunation**. A lunation begins on the first day whose tshes is smaller than the previous day's tshes (so a lunation whose tshes 1 is skipped begins on its 2) and runs to the day before the next such day; the first lunation of a file begins on its first row. `month` is the number the source's printed label implies in the seasonal order: «Первый весенний» 1, «Второй весенний» 2, «Третий весенний» 3, «Первый летний» 4 to «Третий летний» 6, «Первый осенний» 7 to «Третий осенний» 9, «Первый зимний» 10, «Второй зимний» 11, «Третий зимний» 12. The seasonal order is checked inside each source: its Сагаалган line falls on the first day of the lunation labelled «Первый весенний». A lunation labelled «Дополнительный …» has `is_leap_month` true and an empty `month`, and a note says the source prints no number for it; the Q1 round maps conventions. Where a source prints no label for a lunation, both are empty and a note says so (the Ivolginsky calendar throughout: it prints no lunar month label). The label as printed is in the `notes` of the lunation's first day.
- **`notes`**: empty unless a row needs one: the first-row note, a lunation's printed label, an extra month's note and, on the days where the two Aginsky editions disagree, the other edition's value (`FORKS.md` F30: neither is corrected).

## The meta keys

```
tier:                         T1
source:                       the full citation: title, year as printed, compilers as printed, publisher, URL, library path
publisher:                    the publisher as the source or its site gives it
datsan:                       the datsan
lama:                         the compiler(s) as printed
location:                     the library path and "page 1"
sha256:                       of the PDF
retrieval:                    the date and route (docs/sources.md)
edition_year:                 the calendar's year
membership:                   R4 - the publisher's own line naming the Buddhist Traditional Sangha of Russia, with file and line
method:                       the passes, their routes, the comparison, the number settled, the third witness
transcribed_or_generated_by:  spec 013, the model as the session reports it
date:                         the day the file was made (from `date`)
covers:                       first and last date, the row count, "every printed day"
rights:                       the source's own terms as found, quoted or "none found" with what was read
licence:                      CC BY 4.0
notes:                        month and is_leap_month; the label rule and the seasonal order; the extra month's convention (F31); the Сагаалган check; derived skipped; discrepancies
```

## The files

| File | Tier | Publisher | First date | Last date | Rows |
|---|---|---|---|---|---|
| `ivolginsky-datsan-2021.csv` | T1 | Иволгинский дацан «Хамбын Хурээ» (calendar «Лунный календарь 2021») | 2021-01-01 | 2022-02-02 | 398 |
| `aginsky-datsan-2025.csv` | T1 | Агинский дацан «Дэчен Лхундублинг» (the 2025 edition) | 2025-01-01 | 2026-02-17 | 413 |
| `aginsky-datsan-2026.csv` | T1 | Агинский дацан «Дэчен Лхундублинг» (the 2026 edition) | 2026-01-01 | 2027-02-06 | 402 |

## Coverage (MEASURED by spec 013's checks over the committed files)

| File | First | Last | Days | Gaps | Lunations (R8's first rule) | Doubled days | Skipped days | Lunations labelled |
|---|---|---|---|---|---|---|---|---|
| `ivolginsky-datsan-2021.csv` | 2021-01-01 | 2022-02-02 | 398 | none | 15 (two edge fragments of 13 days and 1 day) | 10 | 17 | none (the source prints none) |
| `aginsky-datsan-2025.csv` | 2025-01-01 | 2026-02-17 | 413 | none | 14 | 8 | 14 | 14 (one «Дополнительный зимний месяц») |
| `aginsky-datsan-2026.csv` | 2026-01-01 | 2027-02-06 | 402 | none | 14 | 10 | 16 | 14 |

Every lunation of every file that is not an edge fragment is 29 or 30 days long, and in every file every step between consecutive days is +1, +2 (one skipped) or +0 (a doubled day, never twice in a row), modulo 30.

## M0's done-when against these files (a measured comparison, not a verdict)

001 §10: "≥ 12 months of Buryat tshes data from ≥ 2 independent T1–T3 sources". Whether M0 is met is the human's to say at the landing.

- **Sources and months.** Two datsans: the Ivolginsky datsan (one file: 2021-01-01 to 2022-02-02, 398 days, 13 whole calendar months, January 2021 to January 2022, and 2 days of February 2022) and the Aginsky datsan (two files, two editions: 2025-01-01 to 2026-02-17, 413 days, 13 whole months and 17 days; 2026-01-01 to 2027-02-06, 402 days, 13 whole months and 6 days; together 2025-01-01 to 2027-02-06 with the 48 days 2026-01-01 to 2026-02-17 in both). Each datsan alone has at least 12 months. All three files are tier T1.
- **Independent as far as the files show.** Different datsans (the Ivolginsky «Хамбын Хурээ» and the Aginsky «Дэчен Лхундублинг»); different compilers as printed (the Ivolginsky sheet: «Ширээтэ лама Ацайского дацана Содбо Айсуев»; both Aginsky editions: «Жапов А. Т.» and «Дондоков Б. Д.»); no common dates between the Ivolginsky file (2021-01-01 to 2022-02-02) and either Aginsky file (from 2025-01-01). The two Aginsky editions share a datsan and its compilers, and agree on 40 of their 48 common days and differ on 8 (2026-01-09 to 2026-01-16): they are not independent of each other. The files cannot show whether the datsans compute from a common source (for example one treatise); that is not measured here (`docs/BACKLOG.md` B-5).
- **What this does not show.** Which computation system any of them follows (the Q1 round); who is a member of the Sangha beyond each publisher's own statement (`DEBT.md` D-33); the Ivolginsky site's terms (`DEBT.md` D-35).
