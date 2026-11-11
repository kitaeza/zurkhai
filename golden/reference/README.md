# golden/reference — the formats

The reference fixtures: the tables printed in S. Janson, "Tibetan calendar mathematics", arXiv:1401.6285v1 (tier T5, 001 §6.1), and the output of the `tibetan-date-calculator` library (tier T4). They check the engine's Phugpa arithmetic only; they are never evidence of Buryat practice and never count towards Q1 (001 §6.1). Spec 006 created them. Licence: `../LICENSE.md` (CC BY 4.0).

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
| `tdc-97d4a00-phugpa-1900-2100.csv` | T4 | `gregorian_date`, `tibetan_year`, `month`, `is_leap_month`, `tshes` | Gregorian day, 1900-01-01 to 2100-12-31 inclusive |

## What the columns mean

- **The first day of a year** (Tables 1 and 9) is the first day of month 1; where month 1 has a leap month, the year begins with the leap month (Janson v1, p. 9). Tables 1 and 9 decide only that day, nothing else about the year.
- **`is_leap_month`** is true on the days of the leap (first) of two months with one number (Phugpa: the leap month takes the number of the month that follows it, Janson v1, p. 11).
- **The T4 file has no doubled or skipped column.** Both follow from consecutive rows: a tshes that appears on two consecutive rows is repeated; a number missing between two consecutive rows is skipped. What the `skipped` column of `golden/buryat/` holds is `FORKS.md` F9, still open; nothing here decides it.
- **Table 8's `repeated` and `skipped`** are the table's events: a repeated date is one number on two calendar days; a skipped date is a number on no calendar day.
