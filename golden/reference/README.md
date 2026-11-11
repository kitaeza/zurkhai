# golden/reference — the formats

The reference fixtures: the tables printed in S. Janson, "Tibetan calendar mathematics", arXiv:1401.6285v1 (tier T5, 001 §6.1), and the output of the `tibetan-date-calculator` library (tier T4). The T5 tables check the engine's arithmetic for the version each column of the paper names: Phugpa (spec 006), and Tsurphu and Genden (the paper's column headed "Mongolia"; spec 008); the T4 file checks Phugpa only. Tables 6–9 never count as Buryat evidence and never count towards Q1 (001 §6.1): they are the paper's own figures for three versions, and say nothing about which version a Buryat calendar follows. Spec 006 created this directory; spec 008 added the Tsurphu and Genden files. Licence: `../LICENSE.md` (CC BY 4.0).

## Rules for every file

- CSV, UTF-8, LF line endings, one header line, no quoting needed (no field holds a comma).
- Dates are ISO 8601 (`YYYY-MM-DD`). Booleans are `true` or `false`. An empty field means "none".
- The `year` column of every file is Janson's Y: the Gregorian year in which the Tibetan year begins (eq. (4.1), p. 8).
- Element and animal names use the spellings of `docs/glossary.md`.
- Every CSV has a sibling `<name>.meta.yaml`, one `key: value` per line:

```
tier:                         T5 | T4
source:                       the full citation, with version, or package and version
location:                     table and page | file and commit
sha256:                       of the PDF | of dist/index.js
method:                       the transcription passes and their routes | the generator command, node version, TZ runs
transcribed_or_generated_by:  spec 006, the model as the session reports it
date:                         the day the file was made (from `date`)
covers:                       what the file holds completely
rights:                       the source's own terms, as found
licence:                      CC BY 4.0
notes:                        discrepancies and readings, or "none"
```

## The files

| File | Tier | Columns | One row per |
|---|---|---|---|
| `janson-v1-table1-losar-phugpa.csv` | T5 | `tibetan_year`, `gregorian_date`, `element`, `animal` | Tibetan year 1927–2046: the day of its first day (Losar) and the year's name |
| `janson-v1-table9-losar-phugpa.csv` | T5 | `tibetan_year`, `gregorian_date` | Tibetan year in Table 9's range: the Phugpa column's day of the first day of the year |
| `janson-v1-table7-leap-months-phugpa.csv` | T5 | `year`, `leap_month` | year the table prints; `leap_month` is the number of the Phugpa leap month, empty where the Phugpa cell is empty |
| `janson-v1-table8-2012-phugpa.csv` | T5 | `tibetan_year`, `month`, `tshes`, `kind` | event in Tibetan year 2012: `kind` is `repeated` or `skipped` (the table marks skipped days with "-"). A month with no row has no event: the file's `covers:` states the months |
| `janson-v1-table6-epochs-806.csv` | T5 | `system`, `m0_radix`, `m0_decimal`, `s0_radix`, `s0_decimal`, `a0_radix`, `a0_decimal` | version (`phugpa`, `tsurphu`, `genden`: Table 6's columns Phugpa, Tsurphu and Mongolia; the Bhutan column is not transcribed): the epoch values `m0`, `s0`, `a0` for JD 2015531 (p. 50), in the printed radix form (digits joined by `:`, below) and in the printed six decimals |
| `janson-v1-table7-leap-months-tsurphu.csv` | T5 | `year`, `leap_month` | year the table prints; the number of the Tsurphu leap month, empty where the cell is empty |
| `janson-v1-table7-leap-months-genden.csv` | T5 | `year`, `leap_month` | as above, for the column headed "Mongolia" |
| `janson-v1-table8-2012-tsurphu.csv` | T5 | `tibetan_year`, `month`, `tshes`, `kind` | event in Tibetan year 2012, as the Phugpa file, for the Tsurphu column |
| `janson-v1-table8-2012-genden.csv` | T5 | `tibetan_year`, `month`, `tshes`, `kind` | as above, for the column headed "Mongolia" |
| `janson-v1-table9-losar-tsurphu.csv` | T5 | `tibetan_year`, `gregorian_date` | Tibetan year in Table 9's range: the Tsurphu column's day of the first day of the year |
| `janson-v1-table9-losar-genden.csv` | T5 | `tibetan_year`, `gregorian_date` | as above, for the column headed "Mongolia" |
| `tdc-97d4a00-phugpa-1900-2100.csv` | T4 | `gregorian_date`, `tibetan_year`, `month`, `is_leap_month`, `tshes` | Gregorian day, 1900-01-01 to 2100-12-31 inclusive |

## What the columns mean

- **The first day of a year** (Tables 1 and 9) is the first day of month 1; where month 1 has a leap month, the year begins with the leap month (Janson v1, p. 9). Tables 1 and 9 decide only that day, nothing else about the year.
- **`is_leap_month`** is true on the days of the leap (first) of two months with one number (the leap month takes the number of the month that follows it, Janson v1, p. 11; Tsurphu, p. 39; Genden, p. 43).
- **Table 6's encoding.** No field holds a comma, so the radix digits of a value are joined by `:` in the printed order. `m0` is printed `D; d1, d2, d3, d4` with radices (60, 60, 6, 707): the first digit `D`, before the semicolon, is written first (`2:22:34:2:518`); `s0` has radices (27, 60, 60, 6, 67) and `a0` (28, 126). `m0` is "in the traditional form modulo 7": add 2015529 to the decimal to get the JD (p. 50). The `genden` row is the column the paper headed "Mongolia".
- **The T4 file has no doubled or skipped column.** Both follow from consecutive rows: a tshes that appears on two consecutive rows is repeated; a number missing between two consecutive rows is skipped. F9 (`FORKS.md`) was RULED (b) on 2026-11-11: the `skipped` column of `golden/buryat/<source>.csv` holds the tshes number omitted immediately before the day, mirroring `ZDate.skipped_before`. The ruling governs `golden/buryat/`, not the files in this directory; nothing here decides or depends on it.
- **Table 8's `repeated` and `skipped`** are the table's events: a repeated date is one number on two calendar days; a skipped date is a number on no calendar day.
