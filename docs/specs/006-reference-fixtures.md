spec: 006
prompt: docs/specs/prompts/006-reference-fixtures.md
branch: spec/006-reference-fixtures (cut from main at 0ebaaf8)
merge SHA: not yet merged
pushed: no
model: Sonnet 5.5 (claude-sonnet-5-5), effort high — session 1 (Items 0–2, the halt at 31457a1); Sonnet 5.5 (claude-sonnet-5-5), effort high — session 2 (the resume from 31457a1, ADDENDUM 1). The switch point to the fallback model (Opus 5.5) was never reached
gate: 84 passed, 0 failed (debug) / 84 passed, 0 failed (release) — no failed tests; skipped legs: wasm-pack (crates/tshes-wasm does not exist)
golden: 195 rows added in four T5 files (janson-v1-table1: 120, table9: 31, table7: 21, table8: 23) and 73,414 rows in one T4 file (tdc-97d4a00-phugpa-1900-2100); 0 corrected

Status: draft — the round is complete; status is set to accepted at landing
Depends on: 001, 002, 003, 005
Resolves: F2 (the data half)
Milestone: M1
Governs: golden/reference/, golden/LICENSE.md, crates/tshes/tests/, tools/t4-tdc.mjs

— spec text ends here —

# Report — spec 006 (the reference fixtures)

Two sessions wrote this report. **Session 1** (2026-11-10) did Items 0, 1 and 2, wrote the first generator and halted at `31457a1` with STOP AND ASK Fork 1 (F14). **Session 2** (2026-11-11, a fresh session on the same branch, ADDENDUM 1) wrote the ruling (R4), R2's table, the pinned generator, the T4 fixture, the T5 fixtures, the tests, the controls and the gate. Dates in session 1's records are 2026-11-10 and stay as written; every date session 2 writes is 2026-11-11 (`date` was run once at the top of each session, METHOD rule 51). The handover point is the halt, quoted under "History". Research legs: not applicable. 003 §4 sets their precondition as Q1, every L2 category and any new attribute; this round transcribes and generates the oracles 001 §6.1 already names.

## The decisive item: P5 and P6

| | Result | First mismatch | Evidence |
|---|---|---|---|
| **P5** (T5, Janson v1 Tables 1, 7, 8, 9, Phugpa columns) | **PASS**, zero mismatches on all four files | none | `p5_table1_first_day_and_name_of_every_year_match_the_engine` (120 rows), `p5_table9_first_day_of_every_year_matches_the_engine` (31), `p5_table7_leap_month_of_every_printed_year_matches_the_engine` (21 rows, 8 with a leap month), `p5_table8_repeated_and_skipped_numbers_of_2012_match_the_engine` (23 events: 9 repeated, 14 skipped; month 10 has none, both sides). Each asserts its row count and `mismatches.is_empty()`. |
| **P6** (T4, the library's output, every day 1900-01-01..2100-12-31) | **PASS**, zero mismatches | none | `p6_every_day_of_the_fixture_matches_the_engine_and_its_flags_follow_from_the_rows`: 73,414 rows (year, month, leap flag, tshes), and `is_doubled` and `skipped_before` against the consecutive rows on 73,413 rows (row 1 has no predecessor in the fixture); `p6_the_fixture_has_73414_contiguous_rows_covering_1900_to_2100`; `p6_the_fixture_reaches_every_shape_and_its_counts_equal_the_engines` |

**This is the round's purpose, and the clean pass is not read as a result until the instrument is shown able to fail.** Round 005's planted control C (E806.m0 one day late) passed all 13 exhaustive tests. Re-planted on a scratch copy of the committed tree, it turns P5 and P6 red three runs in three (Item 7 below): P5 Table 1 (first mismatch: Tibetan year 1927, expected 1927-03-04 (table), engine 1927-03-05; 120 of 120 rows) and Table 9 (first: year 2000, expected 2000-02-06, engine 2000-02-07; 31 of 31), P6 (80,646 mismatches over 73,414 days; first: `1900-01-01: fixture (year, month, leap, tshes) (1899, 11, true, 30), engine (1899, 11, true, 29)`); the 13 exhaustive tests stay green again. So the tests can fail on the defect P1–P4 could not see, and the engine, unplanted, passes them.

**What a pass is, and is not (METHOD rule 35).** P5 compares the engine with values printed in the paper that neither the engine nor the library produced; it is the independent check. P6 compares two implementations of one paper (the library's README says it uses year 806 as the epoch, as the engine does): it can catch a slip in either and cannot catch a misreading of Janson that both share, so it is **not proof that either is right**. Tables 1 and 9 decide only the first day of a Tibetan year: their tests claim nothing about any other day (METHOD rule 25). Table 7 decides only which years have a leap month and its number, Table 8 only the repeated and skipped numbers of Tibetan year 2012. The T4 oracle's dates are trusted only as generated under `TZ=UTC` (F14).

## Every property, DONE / DEFERRED / N-A

P1–P4 and P7 are not rebuilt; the gate's run (below) re-runs the 65 existing tests of round 005 in both profiles.

| Property | Verdict | Evidence |
|---|---|---|
| P1, P2, P3, P4, P7 | **DONE in round 005; not rebuilt** | the gate re-confirms them: the 13 `exhaustive` tests, the 5 `p7_proptest` tests and the rest of round 005's 65 tests pass in both profiles on the final tip (counts under "The gate") |
| **P5** reproduces T5 exactly | **DONE** | the four P5 tests above; fixtures `janson-v1-table1-losar-phugpa.csv` (120 rows), `…table9-…` (31), `…table7-leap-months-phugpa.csv` (21), `…table8-2012-phugpa.csv` (23); two transcription passes by two routes identical on every row; seeded control per file; plant C red 3/3 |
| **P6** reproduces T4 exactly over 1900–2100 | **DONE** | the three P6 tests above; fixture `tdc-97d4a00-phugpa-1900-2100.csv` (73,414 rows, `sha256` `8bfd1407…4ce4`); seeded control; plant C red 3/3; planted deleted row and duplicated date red 3/3 |
| 002 §6: a fixture test that loads zero rows fails | **DONE** | `common/fixtures.rs::load` asserts a non-empty file, the exact header, LF endings and the field count; every P5/P6 test also asserts the row count the transcription or generator measured (120, 31, 21, 23, 73,414) |
| 002 §6: a seeded control per fixture | **DONE** | five, one per file (Item 7a) |
| 001 §6.4 "shapes" (METHOD rule 39) | **DONE** | `p6_the_fixture_reaches_every_shape_and_its_counts_equal_the_engines`: leap months 75, repeated tshes 1,632, skipped tshes 2,800, 12 → 1 rollovers 201, rabjung-cycle boundaries 3, each asserted non-zero and equal to the engine's census (months 2,488 for both) |

Task-level (Appendix A, Tasks 4 and 5): Task 4 (T5 transcription and P5): DONE. Task 5 (T4 generation and P6): DONE, with the zone finding (F14, RULED).

## Items 0, 1 and 2 (session 1, 2026-11-10; kept as written, with session 2's re-checks)

**Item 0.** Dates from `date`, run once at the start of session 1: 2026-11-10 (the machine's clock passed midnight during session 1, and 2026-11-10 stayed its date, METHOD rule 51). Branch cut from `0ebaaf8`; the prompt committed byte-identical (`23cb9fb`); spec number 006 free in the tree. OPEN forks, re-measured: 8 at `0ebaaf8` (not copied from the prompt). R1 recorded on F2's row: Status RULED; expected OPEN count 8 + 0 + 0 − 1 = 7; measured 7. R3a, MEASURED: `env --chdir=/home/kitaeza/github/zurkhai RUSTUP_TOOLCHAIN=stable rustc -V` printed `rustc 1.97.1 (8bab26f4f 2026-07-14)`; without the variable, `rustc 1.99.0 (b940084d7 2026-09-28)`. The variable overrides the pin: filed as D-11. R3b filed as D-12. Session 2's resume checks (MEASURED, 2026-11-11): `git status --porcelain` empty and `git rev-parse HEAD` = `31457a148f35f511bd44cbaeeba6a28a603698be` at the start; both artifacts re-verified (below); `cargo test --workspace` (debug) before anything was built: 65 passed, 0 failed (17 crate unit tests, 8 `civil`, 5 `epochs`, 13 `exhaustive`, 1 `overflow_tripwire`, 5 `p7_proptest`, 4 `system_not_implemented`, 8 `worked_examples`, 4 doc tests).

**Item 1, the artifact checks (MEASURED, session 1 2026-11-10; re-measured by session 2 2026-11-11 with the same values).** (A) `sha256sum` of `/home/kitaeza/zurkhai-papers/janson-1401.6285v1.pdf` = `7cafc7df563a3020849c86f4e397daa18b7f7f5ff46244403180aca90b9d0f77`, equal to the `docs/sources.md` library row. (B) the clone's `git rev-parse HEAD` = `97d4a004e74551983dfe0f88cd9fbbd2d646b0f1`, `git status --porcelain` empty, `dist/index.js` present. `sha256sum dist/index.js` = `87e5dc87f8b7154f252d035360aedad5ca973dd33d1b5ad77463063fb9b5cfba`; `node --version` = `v24.14.0`.

**The library's package.json** (read): `name` `tibetan-date-calculator`, `version` `1.2.3`, `license` `"MIT"`; the repository has no LICENSE file (no tracked file named licence, copying or notice).

**`dist/index.js`, read through in full (820 lines) before it was run. Every use of the clock, the time zone, the locale, the network and the file system:**

| Kind | Location (`dist/index.js`) | What it does |
|---|---|---|
| clock: a `Date` built without arguments | line 344, `unixFromJulian`: `new Date().getTimezoneOffset()` | reads **now's** offset of the machine's zone and adds it into the Date it returns. Reached on every Tibetan → Gregorian conversion |
| clock | line 548, `TibetanYear` constructor with no argument | today's year. Not reached by the generator |
| clock | line 647, `TibetanMonth` constructor with no argument | today's month. Not reached |
| clock | line 737, `TibetanDate` constructor with no argument | today's date. Not reached |
| `Date.now` | none | |
| time zone | line 270, `julianFromUnix`: `unixTime.getTimezoneOffset()` | Gregorian → JD route (a Date input). Not reached by the generator |
| time zone / local-time getters | lines 72–77, `getDateStr`: `getFullYear`, `getMonth`, `getDate` (local, not UTC) | forms every Gregorian date string the library returns (`westernDateStr`, `startDateStr`, `endDateStr`) |
| time zone, date string parsed as local | line 268: `new Date(dateOnly + "T18:00:00")`; lines 651, 741: `new Date(arg)` for a string `arg` | the Gregorian → Tibetan route takes only a string (see "What the prompt got wrong") |
| locale | none (no `toLocale*`, no `Intl`) | |
| network | none | |
| file system | none (no `require`, no `import`, no `process`) | |

**The README's own statements about time zones** (README.md line 32, quoted): "__Note__: using date string without time stamp (e.g. '2019-06-21') can create bugs when run in different timezones due to the way native Date object handles it." It says nothing about the offset read at line 344.

**`docs/sources.md`.** A library row added for the directory (citation with commit and GitLab URL; Obtained 2026-11-10, cloned and detached by the human, RELAYED from the advisor; the sha256 of `dist/index.js`; the licence as found; "Trusted for" copied from the Bibliography row). The Bibliography row's "In the library" is now `yes`; recorded there as RELAYED (the advisor, 2026-11-10, FETCHED from lotsawahouse.org/Static/tools/phugpa.html) that the calculator page names this repository as its "calendar library", and as UNVERIFIED whether the live calculator runs this commit. **The Janson row needs no change** (its sha256 was re-measured equal; the file and version are as recorded). Session 2 added, to the library row's "Trusted for", that the library's dates are trusted only as generated under `TZ=UTC` (F14).

**Item 2 — the licence and the formats (R1).** `golden/LICENSE.md` states CC BY 4.0 with the link (no legal text copied), that each file's provenance and third-party source are in its sibling `.meta.yaml`, and that the licence covers the compilation and grants no right a source's terms withhold. `golden/reference/README.md` records the formats (CSV conventions, the meta fields one per line, the five files and their columns, what the columns mean), as the tree's record of them (003 §2). F2's row carries `Built in: spec 006`.

## Item 3 — the T5 transcription (Janson v1, Phugpa columns of Tables 1, 7, 8, 9)

**3a–b, the two passes** (MEASURED, 2026-11-11). Pass 1: typed from the rendered pages (the Read tool's page images of pp. 2, 52, 53, 54) into raw lines in the page's own layout, then converted to CSV by a mechanical awk step (day/month and year to an ISO date; the printed name split into element and animal; sorted by year). Pass 2: `pdftotext -layout` of the same PDF into a `mktemp -d` directory (never in the repository), parsed by a second awk script, then `diff`ed against pass 1 cell by cell.

| File | Rows | Pass 1 vs pass 2 |
|---|---|---|
| `janson-v1-table1-losar-phugpa.csv` | 120 (Tibetan years 1927–2046) | identical, 0 disagreements |
| `janson-v1-table9-losar-phugpa.csv` | 31 (2000–2030) | identical, 0 disagreements |
| `janson-v1-table7-leap-months-phugpa.csv` | 21 (2000–2020), 8 with a leap month | identical, 0 disagreements |
| `janson-v1-table8-2012-phugpa.csv` | 23 events (9 repeated, 14 skipped), months 1–9, 11, 12; month 10 has none | identical, 0 disagreements |

No disagreement exists to list (001 §6.3). One slip of the second-pass *script*, not a disagreement: its first Table 8 parse also matched lines of a later table of the paper (the heading "month Phugpa" recurs) and printed 14 extra lines; the 12 real rows were already identical to pass 1, and the script was fixed to stop at Table 8's own caption (re-run: identical, 23 of 23). The second-pass commands are the awk lines recorded in each meta's `method:`; the scripts themselves were scratch and are not in the tree (the pdftotext output never enters the repository, CLAUDE.md hard rule 17).

**3c, the readings, each with its page.**
- *Table 7's year column.* The text on p. 51 says the table "shows leap months for a range of (Gregorian) years". Reading it as the Tibetan year Y and as the Gregorian year in which the leap month begins give the same 21 rows (`t5_table7_year_column_read_as_the_tibetan_year_or_the_start_year_gives_the_same_rows`, computed from the T4 fixture, which is independent of the engine). The eight leap months begin 2000-02-06, 2002-11-05, 2005-07-07, 2008-04-07, 2010-12-06, 2013-09-06, 2016-05-07 and 2019-02-05, each in the Gregorian year of its Tibetan year. STOP RULE (4) did not fire.
- *Table 8's completeness.* Every Phugpa cell is transcribed: 11 of the 12 months have entries; month 10's Phugpa cell is empty, which the file's `covers:` states as a claim of none. The P5 test asserts that neither the table nor the engine has an event in month 10 and compares both directions over all twelve months.
- *Which day Tables 1 and 9 give.* The first day of the Tibetan year as Janson defines it: the year begins with month 1, and with the leap month if month 1 has one (p. 9: "in case there is a leap month 1, the year begins with the leap month"); it is "1 + JD of the last day of (regular) month 12 the preceding year" (p. 27, the paragraph "Tibetan New Year"), so it is day 2 when day 1 is skipped. The 2000 row, 2000-02-06, is the first day of the leap month 1 (footnote 28, p. 28).

**3d, the paper against itself** (test code only; the engine is not called; all pass, so STOP RULE (3) did not fire):
- Table 9's Phugpa column equals Table 1 on every year both print: `t5_table9_phugpa_column_equals_table1_on_every_year_both_print`, 31 of 31 years.
- Table 8's year length: 30 × 12 − 14 + 9 = 355, and Losar 2012 (2012-02-22) to Losar 2013 (2013-02-11) by Table 1 is 355 days (`t5_table8_2012_length_equals_losar_2012_to_losar_2013_by_table1`); Table 7's 2012 Phugpa cell is empty, so both tables give 2012 no leap month. The per-month lengths from Table 8's counts are 30, 30, 29, 30, 30, 29, 30, 29, 29, 30, 29, 30 (each 29 or 30, Janson v1 p. 7), summing to 355, and no number is both repeated and skipped in a month (`t5_table8_each_month_is_29_or_30_days_and_no_number_is_both_repeated_and_skipped`: an extra check this round added because the year-length check reads counts only, see "What the prompt got wrong").
- Table 7 against eqs. (5.40)–(5.42), p. 19: for each of the 21 years, a leap year iff (24Y + 33) mod 65 ≥ 41 and then M = ⌊33 − ((24Y + 33) mod 65)/2⌋: 8 leap years and 13 without, equal to the table's cells (`t5_table7_leap_months_follow_eqs_5_40_to_5_42`).

## Item 5 — the generator, the field mapping, the flags, the hashes

**The mapping from the library's fields to the fixture's columns** (read from `dist/index.js` and `README.md` before any run; locations are lines of `dist/index.js` at `97d4a004`):

| Fixture column | The library's field | Location | Reading |
|---|---|---|---|
| `gregorian_date` | `TibetanDate.westernDateStr` | 777–783, through `getDateStr` 72–77 (local-time getters) | the `Date` that `unixFromJulian` (343–348) makes from the JD that `julianFromTibetan` (318) gives; the JD is Janson's `⌊true_date⌋`, eq. (8.1), p. 27 |
| `tibetan_year` | `TibetanDate.year` (754) minus 127 | `YEAR_DIFF = 127`, line 12, used at 284 and 375; README lines 40 and 53 ("Tibetan year number (ex. 2135)") | **the library numbers a Tibetan year as the Gregorian year in which it starts plus 127** (Janson v1 p. 8: a year starting in Gregorian year Y is numbered Y + 127 in the epoch-127-BC method), so `tibetan_year` = the library's year − 127 = Janson's Y, eq. (4.1), p. 8 |
| `month` | `TibetanDate.month` (755) | `getMonths` 574–592, `getMonthFromMonthCount` 371–378 (Janson (5.19), (5.20)) | 1..12 |
| `is_leap_month` | `TibetanDate.monthObj.isLeapMonth` (756–760, set at 419 as `isLeapMonth && isDoubledMonth`) | README lines 42, 102 ("first of repeated months"); line 376 (Janson (5.22)) | true on the first of two months with one number |
| `tshes` | `TibetanDate.date` (753) | `getDays` 683–711 | 1..30 |

**The library's own flags** (read at 404–426, README lines 59–62; checked by the generator against what consecutive rows imply, never written as columns):

| Flag | Meaning in the library | Not the same as |
|---|---|---|
| `isDoubledDay` | `julianDate === julianDatePrevious + 2` (406): the number is repeated; **true on both days** of the pair | 001 §5.1 `is_doubled` (the second occurrence only) |
| `isLeapDay` | the instance built with `isLeapDay: true` on a doubled number: the **first** of the two days (JD − 1, line 412) | Janson's leap day, p. 20: the same day; the project's `is_doubled` is the other one |
| `isSkippedDay` | `julianDate === julianDatePrevious` (407): the number occupies no day; absent from `getDays` | 001's `skipped_before` (the number omitted before a day) |
| `isPreviousSkipped` | `julianDatePrevious === julianDate2DaysBefore` (408): the lunar day before is skipped; true on the day after a skip, **and on the second day of a repeated number that follows a skip** | a boolean, not a number |

Counts over the enumeration of Tibetan years 1898–2101 (74,504 checked rows; MEASURED, stderr of the pinned generator): `isDoubledDay` true on 3,312 rows (the rows imply 3,312), `isLeapDay` on 1,656 (1,656), `isPreviousSkipped` on 2,852 rows (2,840 on a first or single day + 12 on the second day of a repeated number), 2,840 skipped numbers flagged by the library and 2,840 skip transitions implied by the rows, 0 absent but unflagged: **0 disagreements**. STOP RULE (6) did not fire; prediction 7: HELD.

**The generator and its zone regime (R4, F14 (a), RULED 2026-11-11).** `tools/t4-tdc.mjs` sets `process.env.TZ = 'UTC'` before it loads the library and before it creates any Date; then checks (1) `Intl.DateTimeFormat().resolvedOptions().timeZone` is `UTC`, (2) `new Date().getTimezoneOffset()` is 0, (3) `getTimezoneOffset()` is 0 at 12:00 UTC on every day of 1898-01-01..2102-12-31 (74,874 days) and on each of the 74,506 enumerated days; a failed check writes nothing to stdout, names itself on stderr and exits 4. It passes the library explicit Tibetan components (year, month, leap flag, day), because the library's Gregorian → Tibetan route takes only a date string or the clock (`dist/index.js` 736–743; the README's own warning about date strings is line 32): the generator walks each Tibetan month through the library's `getMonths` and `getDays` and takes the Gregorian date it returns. The library itself reads the clock's zone offset on every conversion (344); under the pin that offset is 0 at every instant, so the clock's value never reaches the output; the check reads the clock's zone only in order to refuse.

**The three runs** (MEASURED on the committed generator at `c333c84`, 2026-11-11; `node v24.14.0`; the command `env TZ=<zone> node /home/kitaeza/github/zurkhai/tools/t4-tdc.mjs /home/kitaeza/zurkhai-papers/TibetanDateCalcualtor`):

| TZ | exit | stdout | sha256 of stdout |
|---|---|---|---|
| `UTC` | 0 | 1,976,432 bytes, 73,415 lines (header + 73,414 rows) | `8bfd140735bf33b724abe56b0bbb6bb9b63f20164c07cf570622f70f66444ce4` |
| `Pacific/Kiritimati` | 0 | the same | `8bfd140735bf33b724abe56b0bbb6bb9b63f20164c07cf570622f70f66444ce4` |
| `Pacific/Pago_Pago` | 0 | the same | `8bfd140735bf33b724abe56b0bbb6bb9b63f20164c07cf570622f70f66444ce4` |

The UTC output equals the committed fixture byte for byte (`cmp`). Prediction 12: **HELD** (all three equal the hash the halt's report gives, re-measured; the same hashes were also measured on the working-tree generator before it was committed). **What these runs show:** because of the pin, the zone the library sees is `UTC` whatever the caller's zone is, so they show that the *generator* does not depend on the caller's zone; they show nothing about the *library*, whose zone dependence the pin hides by construction (the controls below measure the claim).

**The finding, and its cause.** The library's Gregorian dates depend on the zone it runs in: `dist/index.js:344` adds `new Date().getTimezoneOffset()`, the zone's offset now, and `getDateStr` (72–77) forms the date with the offset at that date. Without the pin, under `TZ=Pacific/Kiritimati`, every date 1900–1994 comes out one day early and 1994-12-31 is missing (control T-b). Control T-c names the cause (METHOD rule 27): `TZ=Etc/GMT-14`, a fixed +14 h with no history, gives byte-identical output, so the cause is **NAMED**: the change in the zone's offset over time, read against today's offset at line 344, and not the size of the offset. The meta's `notes:` and `docs/sources.md` carry the regime and the finding.

**Shape counts in the fixture** (METHOD rule 39; re-measured this round, both from the fixture and from the engine by the P6 test; round 005's figures were RECALLED and are not a baseline; they happen to equal them): months 2,488; leap months 75; repeated tshes (second occurrences) 1,632; skipped tshes 2,800; 12 → 1 rollovers 201; rabjung-cycle boundaries 3 (cycles 15 → 16 in 1927, 16 → 17 in 1987, 17 → 18 in 2047). Each asserted non-zero.

## R2 — 001 §5.3 for the month and year fields

Re-read this session, from the library copy of the PDF (FETCHED, no network): p. 17 (eqs. (5.12)–(5.24)), p. 20 ("The basic rule is"), p. 6 (the Julian day number), p. 8 (eqs. (4.1)–(4.3)) and p. 9 (eq. (4.4), the ten- and twelve-year cycles). The shape is round 005's Item 6 table. Every quotation is two sentences at most.

| Quantity | Verdict | Page | Quotation, and the chain in one line |
|---|---|---|---|
| month number `M` | **VERIFIED** | p. 20; p. 17, eqs. (5.19), (5.20) | p. 20: "In other words, a lunar day gives its name (number and month) to the calendar day where the lunar day ends. (Thus the JD of the calendar day is the integer part of true_date at the end of the corresponding lunar day.)" Chain: the day's JD is location-free (p. 6) → the day takes the name, month included, of the lunar day that ends on it (p. 20) → that lunar day's true month count `n` gives `x = ⌈(65n + β)/67⌉` and `M = x amod 12`, functions of `n` and the constant `β` alone (p. 17) |
| leap flag `ℓ` | **VERIFIED** | p. 17, eq. (5.22) | the same chain; `ℓ = {(65n + β) mod 67 = 1 or 2}`, the third of the triple `(Y, M, ℓ)` that p. 17 computes "from `n`" |
| year number `Y` (Janson's Y) | **VERIFIED** | p. 17, eq. (5.21); p. 8 | `Y = (x − M)/12 + Y₀`, a function of the same `x` and the epoch year `Y₀`. p. 8: "One common method, especially among Westeners, is to simply number a Tibetan year by the Gregorian (or Julian) year in which it starts (and where most of it falls). For convenience, we will use this numbering below." The sentence fixes what the number means (a label); the number itself comes from `n` by (5.21) |
| rabjung cycle `m`, year in cycle `n` | **VERIFIED** | p. 8, eqs. (4.2), (4.3) | `n = (Y − 1026) amod 60`, `m = ⌈(Y − 1026)/60⌉`: functions of `Y` alone |
| element, gender, animal | **VERIFIED** | p. 9, eq. (4.4) and the two cycles | "using the numberings in Tables 2 and 14, year Y is z = (Y − 3) amod 10 in the Chinese 10 year cycle, and has element ⌈z/2⌉"; the animal is `z = (Y − 3) amod 12` in the 12-year cycle (Table 3, p. 10), and the gender is "given by the year mod 2": functions of `Y` alone |

**What VERIFIED means here, and what it does not.** It is a chain of two statements the paper makes: (1) a calendar day's name, month included, is the name of the lunar day that ends on it, and its JD "numbers the days regardless of when they begin and end" and needs no time zone (pp. 20, 6); (2) `(Y, M, ℓ)` is computed from `n` by equations that take no place, time zone or clock (p. 17), and every year-naming field is a function of `Y` (pp. 8–9). No sentence of the paper says "the month number is independent of the observer's location" in those words; the verdict rests on the chain, and it holds within the paper's model of the Phugpa calendar. It says nothing about any calendar in use (that is Q1, 001 §4), and it does not change the engine, whose functions already take no location (spec 005, "What this means for the engine"). Round 005's report left these fields "NOT DECIDED BY THE PAPER" in prose only; that report is frozen (METHOD rule 26), and this table supersedes its Item 6 rows for the month and year fields. **No `DEBT.md` entry is filed by R2**: nothing remains undecided. Prediction 9: **HELD** (VERIFIED for the month fields and for the year fields). R2 landed late: Item 0 placed it before Item 1, and session 1 deferred it; session 2 wrote it in its first commit (`5400223`), before any fixture file landed.

## R3 — the two `DEBT.md` entries (session 1)

- **R3a → D-11, Kind TOOLING.** MEASURED (session 1, 2026-11-10): `env --chdir=/home/kitaeza/github/zurkhai RUSTUP_TOOLCHAIN=stable rustc -V` printed `rustc 1.97.1 (8bab26f4f 2026-07-14)`; without the variable the same call printed `rustc 1.99.0 (b940084d7 2026-09-28)`. The variable overrides the pin, so the entry is filed; TRIGGER: the next round that changes `tools/gate.sh`. Prediction 10: **HELD**.
- **R3b → D-12, Kind NEGATIVE.** `phugpa::Epoch`, `phugpa::to_zdate` and `phugpa::to_jdn` are public API beyond round 005's prompt (005 report, "Latitude taken", item 1); the crate is not published with this surface undecided. TRIGGER: before the crate's first publish.

## Item 7 — the controls (002 §6; METHOD rules 31, 36, 41)

**7a. Seeded wrong rows, in test code, one per fixture file** (each fed through the same comparison as its P5/P6 test and asserted to be reported):

| File | Test | The one wrong field | The assertion message asserted |
|---|---|---|---|
| Table 1 | `seeded_control_table1_one_wrong_date_is_reported` | the 1995 row's date 1995-03-02 → 1995-03-03 | `Table 1, Tibetan year 1995: first day expected 1995-03-03 (table), engine 1995-03-02` (exactly one mismatch) |
| Table 9 | `seeded_control_table9_one_wrong_date_is_reported` | the 2011 row 2011-03-05 → 2011-03-06 | `Table 9, Tibetan year 2011: first day expected 2011-03-06 (table), engine 2011-03-05` (exactly one) |
| Table 7 | `seeded_control_table7_one_wrong_leap_month_is_reported` | the 2010 row's leap month 11 → 10 | `Table 7, year 2010: leap months expected {10} (table), engine {11}` (exactly one) |
| Table 8 | `seeded_control_table8_one_wrong_tshes_is_reported` | month 1's repeated 5 → 6 | `Table 8, month 1: repeated 6 is in the table and not in the engine` and `… repeated 5 is in the engine and not in the table` (exactly two) |
| T4 | `seeded_control_t4_one_wrong_tshes_is_reported` | the 2012-02-22 row's tshes 1 → 2 | `2012-02-22: fixture (year, month, leap, tshes) (2012, 1, false, 2), engine (2012, 1, false, 1)` among the mismatches |

Each control asserts the committed value first ("the committed row"), so it cannot pass on a changed fixture by accident; as a consequence a control also turns red when its fixture or the engine changes under it (visible in plant C below), which is a property of the controls, not a fault of the instrument.

**7b and 7c. Planted controls on scratch copies of the committed tree** (`git archive HEAD` of `c333c84` into a scratch directory, its own `CARGO_TARGET_DIR` per copy, the anchor text asserted to occur exactly once before each edit; never the working tree; each run three times with `cargo test --offline -p tshes … --no-fail-fast`), and the zone controls of ADDENDUM 1 (R4), joined in one table:

| Control | Mutation | Expected red | Observed (runs) | Verdict |
|---|---|---|---|---|
| **C** (005's plant) | `E806.m0` one day late (`2_015_501` → `2_015_502` in `phugpa.rs`) | P5 and P6 | red 3 of 3, **19 tests red each run**: P5 `p5_table1_…` (first: year 1927, expected 1927-03-04, engine 1927-03-05; 120/120) and `p5_table9_…` (first: year 2000, expected 2000-02-06, engine 2000-02-07; 31/31); P6 `p6_every_day_…` (80,646 mismatches; first `1900-01-01`: fixture `(1899, 11, true, 30)`, engine `(1899, 11, true, 29)`) and `p6_the_fixture_reaches_every_shape_…`; also 6 crate unit tests, the 2 three-epoch tests, 4 of 8 worked-example tests, and the three seeded controls whose own assertion is "exactly the seeded row". Green: all 13 `exhaustive` tests (P1–P4 again blind to it), P5 Tables 7 and 8 (INFERRED: a uniform one-day shift of every date moves no leap-month or repeated/skipped structure; not isolated by a further run), `p6_the_fixture_has_73414_…` (it reads the fixture only) | as expected: prediction 11 **HELD** |
| **R** | the T4 row `2012-02-22,…` deleted (73,414 → 73,413 rows) | the count and contiguity assertions | red 3 of 3 (4 tests): the count assertion `rows loaded` (left 73413, right 73414) in both `p6_the_fixture_has_…` and `p6_every_day_…`, the shape-count equality (skipped 2801 against the engine's 2800), and the seeded control (its own lookup of the deleted row panics) | count: as expected. **Contiguity not shown by R**: the count assertion comes first in the same test, so the contiguity loop is never reached |
| **R2** (added) | the same row's date changed to `2012-02-23` (row count unchanged, a duplicate date) | contiguity | red 3 of 3 (3 tests): `row 40959 (2012-02-23) is not on day 40959 of the range` (left 15393, right 15392), `p6_every_day_…`, the seeded control | as expected: the contiguity assertion can fail by itself |
| **T6** | Table 8's `2012,1,5,repeated` → `2012,1,6,repeated` | the Table 8 test and Item 3d's length check | red 3 of 3 (2 tests): `p5_table8_…` (2 mismatches; first `Table 8, month 1: repeated 6 is in the table and not in the engine`) and the seeded control (its anchor); **the length check stays green** | Table 8 test as expected; **the length check did not turn red**: it reads counts, which a changed tshes does not move (see "What the prompt got wrong") |
| **T19** (added) | the same event's tshes changed to 19, which month 1 already skips | a check that reads the table's values | red 3 of 3 (3 tests): `t5_table8_each_month_is_29_or_30_days_and_no_number_is_both_repeated_and_skipped` (`month 1: the number 19 appears twice among the entries (skipped and another)`), `p5_table8_…`, the seeded control; the year-length check stays green | as expected |
| **T-a** | the generator copy with the pin's assignment deleted | non-zero exit, empty stdout, stderr names the check | `TZ=Pacific/Kiritimati`: exit 4, stdout 0 bytes (`e3b0c442…`), stderr `REFUSED: zone check 1 failed: Intl.DateTimeFormat().resolvedOptions().timeZone is "Pacific/Kiritimati", not "UTC"`; 3 of 3 | as expected (prediction 13). **Reaches check 1 only**: `DEBT.md` D-13 |
| **T-b** | the copy with the pin and the checks deleted (41 lines) | the generator's own checks fail, exit 3, empty stdout | `TZ=Pacific/Kiritimati`: exit 3, stdout 0 bytes; 3 of 3. First-rows line: `ends: first rows 1900-01-01,1899,11,false,1 \| 1900-01-02,1899,11,false,2; last rows 2100-12-30,2100,10,false,30 \| 2100-12-31,2100,11,false,1`. First problem line: `not contiguous: 1994-12-30 then 1995-01-01 (gap 2)` (then `range holds 73413 rows, expected 73414`) | as expected; the halt's finding re-measured (CLAUDE.md hard rule 10) |
| **T-c** | the T-b copy, once | stdout byte-identical to the fixture → cause NAMED | `TZ=Etc/GMT-14`: exit 0, 73,415 lines, sha256 `8bfd1407…4ce4`, `cmp` against the fixture silent | **the cause is NAMED** (prediction 14 HELD): the change of a zone's offset over time against today's offset at `dist/index.js:344`, not the size of the offset. Mechanism: the library converts a JD to a JS `Date` with today's offset and reads the date back with the date's own offset, so it is right only where the two agree |

The T-a, T-b and T-c copies were made by `cp` into directories made by `mktemp -d`, edited there by line-number deletion with the anchor text asserted first, and loaded the same `dist/index.js` by absolute path. The plant copies for C, R, R2, T6 and T19 were made under the session's scratchpad directory (see "Latitude taken").

## The predictions, scored

Written by the advisor before any measurement; scored with the measured value.

| # | Prediction | Score | Measured |
|---|---|---|---|
| 1 | both ARTIFACT checks pass | HELD | both sessions: PDF sha256 `7cafc7df…0f77`; library HEAD `97d4a004…`, status empty, `dist/index.js` present |
| 2 | row counts: Table 1 120; Table 9 31; Table 7 21 years with 8 leap; Table 8 23 entries, 9 repeated, 14 skipped | HELD | re-measured from the transcription: 120; 31; 21 with 8; 23 with 9 and 14 |
| 3 | every Item 3d check agrees; Table 8 gives 2012 360 − 14 + 9 = 355 days, equal to Losar 2012 → 2013 by Table 1 | HELD | 355 = 355 (2012-02-22 → 2013-02-11); Table 9 = Table 1 on 31 years; Table 7 = eqs. (5.41), (5.42) on 21 years |
| 4 | Table 7's two year readings give the same rows | HELD | 21 of 21 rows equal under both readings |
| 5 | P5: zero mismatches on every T5 file | HELD | 0 on 120, 31, 21 and 23 |
| 6 | the T4 fixture has 73,414 rows; the three TZ runs are byte-identical (scored on the pinned generator's output) | HELD | 73,414 rows; one sha256 in three zones |
| 7 | the library's own flags agree with consecutive rows on every day | HELD | 0 disagreements over 74,504 checked rows |
| 8 | P6: zero mismatches over the 73,414 days | HELD | 0 |
| 9 | R2: VERIFIED for the month and year fields | HELD | five rows VERIFIED |
| 10 | `RUSTUP_TOOLCHAIN=stable` inside the repository gives `rustc 1.97.1` and overrides the pin | HELD | `rustc 1.97.1 (8bab26f4f 2026-07-14)` (session 1) |
| 11 | plant C turns at least one P5 test and the P6 test red, three runs in three | HELD | P5 Tables 1 and 9, P6 two tests; 19 red per run, 3 of 3 |
| 12 | the pinned generator's stdout has the halt's sha256 in all three zones | HELD | `8bfd1407…4ce4` ×3 |
| 13 | T-a and T-b each behave as expected, three runs in three | HELD | T-a exit 4 ×3 empty stdout; T-b exit 3 ×3 empty stdout |
| 14 | T-c's stdout is byte-identical to the fixture, so the cause is NAMED | HELD | `cmp` silent |

## D-1, D-10 and the entries this round files

- **D-1** (no client defaults `System`): **OPEN, respected.** Every P5/P6 call names `System::Phugpa` explicitly; no `Default` is used or added.
- **D-10** (Janson v1 p. 1 gives "Sunday" for 31 December 2007, a Monday by eq. (9.1)): **OPEN.** Item 8: no file this round writes takes a weekday from Janson v1 (a case-insensitive grep for the weekday names, "weekday" and "day of week" over `golden/`, the three test files and the generator finds nothing; the T4 fixture has no weekday column and the generator does not read the library's `day` getter). The discrepancy is unexplained, and a dated line saying so is in the entry.
- **Filed this round:** D-11 (TOOLING), D-12 (NEGATIVE) by session 1 (R3); **D-13** (COVERAGE: the generator's zone checks 2 and 3 have no plant of their own; T-a reaches check 1 only, and GRANTS allowed three copies, not more); **D-14** (DOCS: 001 §11 still calls the data licence open, and 001 Appendix A's Task 5 names a route the library does not have). Paid: none.

## The open-forks audit and the marker greps

**The audit** (METHOD rule 60): `expected = previous OPEN count + forks raised + gated findings filed with options named and none chosen − rulings recorded`, then `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`. MEASURED at the start of session 1 from `0ebaaf8`: 8 (F4, F5, F7, F8, F9, F10, F11 and F2 OPEN). The advisor's sequence for both sessions was `8 → 7 → 7 → 7 → 8 at the halt → 7 at R4`. Session 1's report did not record the arithmetic or the greps per commit, and this report does not reconstruct them as if they had been run then; the OPEN counts below are **measured now, on each commit's tree**, with `git show <sha>:FORKS.md`; session 2's rows are the arithmetic run before each of its commits.

| Commit | Session | What the commit does to forks | Expected | Measured OPEN |
|---|---|---|---|---|
| `23cb9fb` | 1 | the prompt | 8 + 0 + 0 − 0 = 8 | 8 |
| `396a5f1` | 1 | R1 recorded (F2 RULED) | 8 + 0 + 0 − 1 = 7 | 7 |
| `35304c4` | 1 | none | 7 | 7 |
| `3f4c353` | 1 | `Built in` on F2 | 7 | 7 |
| `31457a1` | 1 | F14 raised (the halt) | 7 + 1 = 8 | 8 |
| `5400223` | 2 | R4 recorded (F14 RULED) | 8 + 0 − 1 = 7 | 7 |
| `5ef02bd` | 2 | `Built in` on F14; D-13 filed (no options named: not a fork) | 7 | 7 |
| `c333c84` | 2 | none | 7 | 7 |
| the final commits | 2 | none | 7 | 7 |

**The grep of 001 and 002 for forks with no ruling** (METHOD rule 59): `grep -nE '\bF[0-9]+\b' docs/specs/001-init-spec.md docs/specs/002-rust-code-standard.md`, compared with `FORKS.md`. Hits: F1 (ruled), F2 (code half ruled in 002 §2; the data half RULED 2026-11-10: 001 §11's text still says "open", D-14), F3 (ruled), F4 (OPEN; no datsan contact here), F5 (OPEN; no localised name is written), F6 (ruled (a); this round adds no `std` or `alloc` to the engine), F7 (OPEN; no FFI crate), F8 (OPEN; no rules crate), F9 (OPEN), F10 (OPEN; no outputs of the unknown-system app recorded). **Why this round's files do not decide F9:** F9 concerns what the `skipped` column of `golden/buryat/<source>.csv` holds (001 §6.2: one row per Gregorian day of a Buryat source). This round writes `golden/reference/`, a different directory with different formats (`golden/reference/README.md`): Table 8's `kind` column names the events of a month of the paper's table, one row per event (`repeated` or `skipped`, with the number), not a per-day column, and the T4 fixture has no doubled or skipped column at all (they follow from consecutive rows). No file here fixes a `skipped` column for `golden/buryat/`, no `golden/buryat/` file exists, and nothing was built through F9. No fork was found that the round built straight through.

**The marker greps** (METHOD rule 58). The round's marker is a single token chosen by session 1 and absent from the prompt file; this report never spells it in prose, and it stands in this file's head block only, on the `gate:` line until the gate has run. The grep: `grep -rn --exclude-dir=.git --exclude-dir=target --exclude-dir=.codewhale --exclude-dir=prompts -e <the round's marker> /home/kitaeza/github/zurkhai`. Session 1's report records none of its greps; **where a grep was not run before a commit, this report says so and does not reconstruct it**: session 1's commits `396a5f1`, `35304c4`, `3f4c353`, `31457a1` have no recorded pre-commit grep. Measured now on each commit's tree (`git grep`, prompts excluded), not as a claim of what was run then: `23cb9fb` 0 hits; `396a5f1`, `35304c4`, `3f4c353`, `31457a1`, `5400223`, `5ef02bd`, `c333c84`: 2 hits each (the two head-block placeholders). Session 2 ran the grep before each of its commits `5400223`, `5ef02bd`, `c333c84` (expected 2, measured 2 each: the two head-block placeholders). The commit that carries this report in its final form has the marker on the `gate:` line and on the four gate-dependent places (the tests per profile, the gate legs, the diff numbers, the gate section): the chain asserts that count before the commit; the last commit replaces them all with the measured lines and its chain asserts 0.

## The prompt's questions, answered

- *Item 0: branch, prompt byte-identical, number free, Read-first list, `date`, the marker, the OPEN count, R1, R2, R3.* Branch cut from `0ebaaf8`; prompt committed (`23cb9fb`), and after ADDENDUM 1 appended to the prompt file the committed copy was brought up to date in `5400223` (before: a byte-for-byte prefix, `cmp` EOF after byte 28096, line 382; after: `cmp` silent; `cmp` run before every commit of session 2); number 006 free; Read-first list read in both sessions; dates from `date`; the marker chosen; OPEN 8 re-measured; R1 on F2's row; R2 above (VERIFIED, five rows); R3 above (D-11 with its measured line, D-12), landed in the commit after the prompt, before any fixture.
- *Item 1: the artifacts, `sha256sum` and `node --version`; the clock, zone, locale, network and file-system uses of `dist/index.js` with locations; the README's statements; the sources row; the Janson row.* Above, in "Items 0, 1 and 2" (session 1's account, kept): the clock is read at 344, 548, 647 and 737 and the zone at 72–77, 270, 268, 651 and 741; no locale, network or file system. **The Janson row needs no change** (its sha256 was re-measured equal).
- *Item 2: the licence and the formats.* Above, in "Items 0, 1 and 2".
- *Item 3: the four files; both passes; the readings; the paper against itself.* Above.
- *Item 4: P5.* Above; Tables 1 and 9 claim only the first day of a year.
- *Item 5: the generator, the mapping, the flags and their counts, the three runs, the shape counts.* Above.
- *Item 6: P6, and what it is.* Above.
- *Item 7: the controls.* Above.
- *Item 8: D-10.* Above.
- *Item 9: the grep, the gate, the head's `golden:` line, the INDEX row.* Above and below. The INDEX row is in the final commit.
- *ADDENDUM 1: R4 on F14's row, R5, the pin, the check, the three runs, the record, the three controls.* Above; `docs/sources.md`'s "Trusted for" carries the regime.
- *"Report (3) of the prompt: which tests, their first failing row, for plant C."* Above.

## The stop rule that governed the round

STOP RULE (5), reached in session 1: once the generator passed the library explicit date components, its output still differed between time zones (Kiritimati). It halted the round (STOP AND ASK Fork 1, F14); the human ruled option (a) with three additions (R4), and session 2 resumed. Not fired: (1) the artifacts were present and identified in both sessions; (2) the engine agreed with every T5 row and every T4 row, so no engine mismatch exists to list; (3) the paper agreed with itself in every Item 3d check; (4) Table 7's two readings gave the same rows and no cell needed a reading the page does not fix; (6) the library's flags agree with the rows (0 disagreements); (7) nothing under `crates/tshes/src/` or `crates/tshes/Cargo.toml` changed (`git diff --stat main` over those paths is empty); (8) see "Latitude taken" for the reads and runs outside GRANTS, none of which was a denial; (9) out of scope and not written: Tsurphu, Mongolia and Bhutan columns, Genden or Tsurphu arithmetic, any 001 §5.2 attribute, `golden/buryat/`, `rules/`, a `skipped` column, localised names, INDEX statuses of earlier rounds; (10) the pinned generator's checks passed in all three runs, and T-a and T-b behaved as expected in every run.

## Diff scope

(`git diff --numstat main` at the final tip; the figures are in "Numbers that must sum".) Files, by category:

- **Gated, `golden/**`, under accepted specs 001 (§6.1, §6.4, §8, F2 as ruled) and 002 (§6): YES.** New: `golden/LICENSE.md`; `golden/reference/README.md`; four T5 CSVs and their four `.meta.yaml`; the T4 CSV and its `.meta.yaml`.
- **Gated, `crates/tshes/tests/**` only, under 001 §6.4 and 002 §6: YES.** New: `p5_reference_tables.rs`, `p6_t4_fixture.rs`, `common/fixtures.rs`. Changed: `common/mod.rs` (+2 lines: `pub mod fixtures;`). **`crates/tshes/src/` and `crates/tshes/Cargo.toml`: untouched.**
- This round's own `docs/specs/006-reference-fixtures.md`, `docs/specs/prompts/006-reference-fixtures.md` (with ADDENDUM 1 appended) and its row in `docs/specs/INDEX.md`: not changes to an accepted spec.
- Not gated: `tools/t4-tdc.mjs` (new), `DEBT.md` (D-10 line, D-11, D-12, D-13, D-14), `FORKS.md` (F2, F14), `docs/sources.md` (the library row, the Bibliography row, the regime in "Trusted for").
- `docs/METHOD.md`, `GATED.md`, `Cargo.toml`, `Cargo.lock`, `tools/gate.sh`, `tools/gate-self-test.sh`: untouched.
- **Files the test framework wrote:** none. No `proptest-regressions` file appeared in `git status --porcelain` before any commit of session 2, and the new tests use no proptest; the scratch plants ran on scratch copies, whose files are not in the tree.
- **The fixture's size:** `golden/reference/tdc-97d4a00-phugpa-1900-2100.csv`: 1,976,432 bytes, 73,415 lines.

## Forks encountered

- **The one STOP AND ASK: F14 (spec 006 Fork 1)**, the zone regime of the T4 fixture. RULED 2026-11-11, (a); built in `tools/t4-tdc.mjs`. History below.
- **Resolved without escalating** ("considered X, chose Y because Z"):
  1. *The committed prompt after ADDENDUM 1.* Considered leaving the committed copy a prefix of the prompt file (and `cmp` then fails every time), or bringing it up to date. Chose to append the addendum to the committed copy in session 2's first commit: GATED.md allows exactly that ("an ADDENDUM is appended, nothing else changes") and ROUNDS §2 says the same.
  2. *Where the second pass for Table 7 and Table 8 reads columns.* Considered a character-position rule or a regular expression over the text. Chose the right-edge rule for Table 7 (the cell whose right edge is nearest the "Phugpa" heading's) and the comma-continued run for Table 8 (a cell is separated from the next column by a single space in some rows, so spacing cannot split them).
  3. *How the engine's "first day of a Tibetan year" is found.* Considered `to_gregorian` of month 1, tshes 1 (which is skipped in some years) or the day after the last day of month 12. Chose the first Gregorian day from 1 January whose `to_zdate` year is Y, which is the first day of the year under Janson's definition (p. 27) and needs the public API only.
  4. *Table 8's events in the engine.* Considered the `is_doubled` and `skipped_before` flags or counting days per tshes. Chose the day counts per number in each month (a number on two days is repeated, on none skipped): independent of the flags, which P6 and P3 check separately.
  5. *P6 and the first row.* The fixture has no predecessor for 1900-01-01, so its two flags cannot be implied; considered extending the fixture to 1899-12-31 (outside the range 001 §6.4 names) or leaving the row out of the flag comparison. Chose the latter and said so.
  6. *The year-length check cannot see a changed tshes.* Added a value-sensitive per-month check beside it (Item 3d) rather than changing the count-based check, and planted T19 to show it can fail.
  7. *Zone checks.* The check's third part runs twice (a fixed span before the library loads, the enumerated days after): one is the prompt's reading, the other guards against a day the span missed.

## Latitude taken, beyond or differently from the prompt

Session 1 (carried from its draft, with R5):
1. `pdfinfo` was run once on the PDF for the page count (not in GRANTS; read-only metadata; not needed).
2. Library files were read with `cat`, `ls -la`, `git ls-files` and `git remote -v` (GRANTS names the Read tool and `git rev-parse`, `status`, `log -1` only); read-only, none denied; the facts are in the sources row. **R5 (2026-11-11, "agree"): accepted as reported; no grant is made after the fact; session 2 did not repeat these reads.**
3. Two Bash calls began with a no-op `cd` (CLAUDE.md hard rule 16); neither changed anything.

R2's late landing (see "R2"): the prompt placed R2 before Item 1; it landed in session 2's first commit, before any fixture.

Session 2:
4. `node --check` was run on the T-a and T-b copies (a syntax check that executes none of their code) and once `node -e` with a one-line inline script (it printed the zone and three offsets under `TZ=Etc/GMT-14`). GRANTS lets `node` run only the generator and the copies T-a, T-b and T-c; neither call is named. Both were read-only and changed nothing; the inline script was not needed (T-c's verdict rests on its own `cmp`). Not repeated.
5. Session 2 read `pdftotext -layout`'s listing of the whole PDF to find the tables (pp. 2, 51–54) **before** it typed the first pass from the rendered pages. The first pass was typed from the page images and the comparison is a script, but the reader of the page was not blind to the second route's text: the independence of the first pass was weaker than the prompt means. A disagreement would still have shown (the two parses are different mechanisms), and there was none.
6. The plant copies for C, R, R2, T6 and T19 were made under the session's scratchpad directory (the harness's designated scratch), not by `mktemp -d` as the prompt's scratch line says; the pdftotext output and the T-a/T-b copies were in `mktemp -d` directories. Their target directories were deleted after each run; the copies (a few hundred KB) are outside the repository.
7. Two plants beyond the prompt's three (R2: a duplicated date, T19: a colliding tshes), because R and T6 each left one assertion unshown (see Item 7). A per-month check was added to Item 3d for the same reason (Forks, item 6).
8. `D-14` was filed for two stale statements in accepted spec 001 found by the Item 9 grep and by Item 5; 001 is not edited.
9. The plant script ran its `cargo test` from a subshell `cd` into the scratch copy (rustup picks the toolchain by directory), where `env --chdir=` was the repository's convention for runs from another directory; the baseline run used `env --chdir=`. One commit command began with a variable assignment and a `;` (`cd_free=1;`, not a `cd`); the commit itself ran in an `&&` chain.
10. A baseline of the 65 tests at `0ebaaf8` was run in both profiles on a `git archive` copy under `mktemp -d` (METHOD rule 4).

## Numbers that must sum

- **Rows:** 195 = 120 (Table 1) + 31 (Table 9) + 21 (Table 7) + 23 (Table 8); 21 = 8 leap + 13 without; 23 = 9 repeated + 14 skipped; Table 8's months 1–9, 11, 12 carry the 23, month 10 none. T4: 73,414 = days from 1900-01-01 to 2100-12-31 inclusive; 73,415 lines = 73,414 + 1 header.
- **Table 8:** the per-month lengths 30 + 30 + 29 + 30 + 30 + 29 + 30 + 29 + 29 + 30 + 29 + 30 = 355 = 360 − 14 + 9 = the days from 2012-02-22 to 2013-02-11.
- **The T4 shape identity:** 30 × 2,486 − 2,800 + 1,632 = 73,412 = 73,414 − 2, where 2,486 = 2,488 months − 2 partial months at the edges (1900-01-01 is the 30th of the leap month 11 of 1899; 2100-12-31 is the 1st of month 11 of 2100), each contributing one day to the range.
- **The library's flags:** 3,312 = 2 × 1,656 (`isDoubledDay` is true on both days of a repeated number, `isLeapDay` on the first); 2,852 = 2,840 + 12 (`isPreviousSkipped`).
- **Plant C:** 19 red = 6 (crate unit) + 2 (`epochs`) + 4 (`p5`) + 3 (`p6`) + 4 (`worked_examples`); 13 `exhaustive` green.
- **Tests per profile:** 84 = 17 (crate unit) + 8 (`civil`) + 5 (`epochs`) + 13 (`exhaustive`) + 1 (`overflow_tripwire`) + 5 (`p7_proptest`) + 4 (`system_not_implemented`) + 8 (`worked_examples`) + 14 (`p5_reference_tables`) + 5 (`p6_t4_fixture`) + 4 (doc tests); at `0ebaaf8`, re-measured on a `git archive` copy: 65 passed, 0 failed (debug) and 65 passed, 0 failed (release), so the change is +19 (14 + 5) in each profile; debug 84/0, release 84/0, msrv 84/0 (the msrv leg runs the pinned toolchain, and its 84 are not counted again).
- **Gate legs:** 12 named = 11 run + 1 skipped; 11 passed, 0 failed (fmt, clippy, test, test_release, build_wasm, deny, msrv, no_std_check, no_alloc_check, zero_deps_check, self_test); skipped: wasm-pack.
- **The diff against `main`, summing** (`git diff --numstat main` at the final tip; insertions, deletions, file):

```
   131    0  crates/tshes/tests/common/fixtures.rs
     2    0  crates/tshes/tests/common/mod.rs
   572    0  crates/tshes/tests/p5_reference_tables.rs
   292    0  crates/tshes/tests/p6_t4_fixture.rs
    33    0  DEBT.md
    11    2  FORKS.md
     2    1  docs/sources.md
     1    0  docs/specs/INDEX.md
  474    0  docs/specs/006-reference-fixtures.md
   509    0  docs/specs/prompts/006-reference-fixtures.md
     9    0  golden/LICENSE.md
    42    0  golden/reference/README.md
   121    0  golden/reference/janson-v1-table1-losar-phugpa.csv
    11    0  golden/reference/janson-v1-table1-losar-phugpa.meta.yaml
    22    0  golden/reference/janson-v1-table7-leap-months-phugpa.csv
    11    0  golden/reference/janson-v1-table7-leap-months-phugpa.meta.yaml
    24    0  golden/reference/janson-v1-table8-2012-phugpa.csv
    11    0  golden/reference/janson-v1-table8-2012-phugpa.meta.yaml
    32    0  golden/reference/janson-v1-table9-losar-phugpa.csv
    11    0  golden/reference/janson-v1-table9-losar-phugpa.meta.yaml
 73415    0  golden/reference/tdc-97d4a00-phugpa-1900-2100.csv
    11    0  golden/reference/tdc-97d4a00-phugpa-1900-2100.meta.yaml
   275    0  tools/t4-tdc.mjs
76022    3  TOTAL (23 files)
```

- `golden/`: +73,720 −0 (9 + 42 + 199 + 44 + 73,415 + 11: the licence, the formats, the four T5 CSVs, their four metas, the T4 CSV and its meta)
- `crates/tshes/tests/`: +997 −0 (131 + 2 + 572 + 292); `crates/tshes/src/` and `Cargo.toml`: +0 −0
- `tools/t4-tdc.mjs`: +275 −0
- records `DEBT.md`, `FORKS.md`, `docs/sources.md`, `docs/specs/INDEX.md`: +47 −3 (33 + 11 + 2 + 1; 0 + 2 + 1 + 0)
- this report and the committed prompt: +474 +509 (the prompt with ADDENDUM 1: 509 lines)
- Subtotals summing: insertions 73,720 + 997 + 275 + 47 + 474 + 509 = 76,022 (total 76,022); deletions 0 + 0 + 0 + 3 + 0 = 3 (total 3).
- The fixture's size: `golden/reference/tdc-97d4a00-phugpa-1900-2100.csv` is 1,976,432 bytes (73,415 lines); all of `golden/` is 1,999,714 bytes.
- Gated paths touched: yes (`golden/**` and `crates/tshes/tests/**`, under accepted specs 001 and 002, named in the prompt); `crates/tshes/src/`, `crates/tshes/Cargo.toml`, `docs/METHOD.md`, `GATED.md`: no.

## What the prompt got wrong

- **(a) Item 5 assumed the library accepts Gregorian date components.** Confirmed: the Gregorian → Tibetan route accepts only a date string or the clock (`dist/index.js` lines 736–743: the branches at 736 and 740, `new Date(arg)` at 741; ADDENDUM 1 cites 739–741 and session 1's draft 740–743, so the span is 736–743). The generator therefore passes explicit Tibetan components, the library's only explicit-component route.
- **(b) Item 5's three-zone test assumed a library that ignores the zone, and "never the clock" cannot hold inside a library that reads it at `dist/index.js:344`.** Confirmed and measured (T-b, T-c): the library's dates depend on the zone's offset history; the test the prompt wanted is only meaningful under the pin of R4, and then it shows the generator's independence, not the library's.
- **(c) Item 1 asked whether the library has a LICENSE file, but GRANTS gave no route to list its files.** Confirmed: session 1 used `ls -la`, `git ls-files` and `git remote -v` (Latitude 2; R5). The finding is the same either way: package.json `license` is `"MIT"`, and no tracked file is named licence or copying.
- **Item 7b expected "one Table 8 tshes changed" to turn "Item 3d's length check" red.** Wrong for that check as Item 3d defines it: 30 × 12 − skipped + repeated reads counts, which a changed tshes does not move (plant T6: the Table 8 P5 test red, the length check green, 3 runs in 3). A check that sees the values (no number both repeated and skipped) goes red only when the change collides (plant T19). METHOD rule 25: the deciding field must be visible to the check.
- **Item 7b expected a deleted T4 row to turn "the count and contiguity assertions" red.** The count assertion fires first in the same test and the contiguity loop is never reached; a plant that keeps the count and breaks the order (R2) shows contiguity.
- **Item 3's ordering of the two passes** ("first pass from the rendered page, second by `pdftotext`") is hard to keep blind: finding Tables 1, 7, 8 and 9 in an 86-page PDF without the text listing needs a page-by-page read of the rendered pages (allowed by GRANTS, but slow). Not a prompt error, and the weakening was this session's (Latitude 5).
- **ADDENDUM 1's sequence "8 → 7 → 7 → 7 → 8 at the halt → 7 at R4"** matches the six commits measured here (`23cb9fb` … `5400223`). Nothing else found wrong: STOP RULE (7) was correctly scoped, the OPEN count was 8, the preconditions held (main at `0ebaaf8`, 006 free, both artifacts present and identified), the T5 counts the advisor predicted were exact, and the T4 hash the advisor relayed was exact.

## By-catch

- **D-13** (filed): the zone checks 2 and 3 are untested instruments behind check 1.
- **D-14** (filed): 001 §11 says the data licence is open; 001 Appendix A Task 5 names a route the library lacks.
- **The library has no LICENSE file; its package.json says MIT.** The fixture holds facts only; the library stays outside git. Recorded in `docs/sources.md` and the T4 meta (`rights:`). Not a debt: the repository does not redistribute the library.
- **The library's `isDoubledDay` is true on both days of a repeated number**, which differs from 001 §5.1's `is_doubled` (the second occurrence): recorded in the T4 meta and above; any later round that reads the library's flags directly must not equate them.
- **A seeded control that asserts "exactly the seeded row" also turns red when the engine itself is wrong** (plant C turned three of them red). That is a property of the controls (each asserts the committed value first), not a defect; stated so a reader of a red run is not misled.
- Nothing else found.

## Held

Identified items only: D-1, D-3, D-4, D-5, D-6, D-7, D-8, D-9, D-10, D-11, D-12, D-13, D-14 (OPEN); F4, F5, F7, F8, F9, F10, F11 (OPEN); the wasm-pack leg stays SKIPPED until `crates/tshes-wasm` exists (named in the gate line).

## The gate

**Run 1**, on commit `08b4121` (the report in its final form but for the gate-dependent lines; the tree was clean, `git status --porcelain` empty, no edit in flight), in both profiles, from outside the repository (the session's scratchpad directory, the directory spec 005 Item 1b proved), as one command: `env --chdir=<scratchpad> bash /home/kitaeza/github/zurkhai/tools/gate.sh`, every leg including the self-test (METHOD rule 65; CLAUDE.md hard rule 12). Summary line, quoted from the gate's own output:

```
GATE: 11 legs run — 11 passed, 0 failed, 1 skipped
```

The skipped leg, named with its reason: wasm-pack — `crates/tshes-wasm` does not exist (002 §7: this leg runs once the crate exists; METHOD rule 65).

The toolchain each leg ran on, quoted from each tool's own output (METHOD rule 23): fmt, clippy, test, test_release, build_wasm and msrv each printed `toolchain: rustc 1.99.0 (b940084d7 2026-09-28)`; the deny leg printed `tool: cargo-deny 0.20.2` and `advisories ok, bans ok, licenses ok, sources ok`; the zero-deps leg printed `zero-deps ok: no normal or build dependency; dev-dependencies: proptest`; the self-test leg printed `tool: bash GNU bash, version 5.3.9(1)-release (x86_64-pc-linux-gnu)` and `SELF-TEST: 26 plants — 26 rejected with their named diagnostic, 0 failures`.

Test counts, both profiles, summed from each `test result:` line of the run (11 result lines per leg): debug 84 passed, 0 failed; release 84 passed, 0 failed; msrv 84 passed, 0 failed. Round 005's 65 tests (P1–P4, P7, the three-epoch comparison, Janson's worked examples) are among them and pass in both profiles: **the gate re-confirms P1–P4 and P7.** The count is comparable to the previous gate's (65) as 65 + 19; no test was removed or ignored (0 `ignored` in any result line).

**Run 2.** This report and `docs/specs/INDEX.md` were edited after run 1 (the gate-dependent lines, the INDEX row), so the gate is run again on the tip of the branch, as the prompt requires. The change from `08b4121` is those two files and nothing under `crates/`, `golden/` or `tools/`. A report cannot hold the result of a run made after its last edit without changing the tip that run used, so run 2's summary line and counts are given in the session's closing message, not in this file; they are expected to equal run 1's, and a difference would be reported there as a failure.

## History — session 1's halt, and the ruling (as written)

Session 1 halted at `31457a1`. The block below is the STOP AND ASK as session 1 wrote it, kept as the round's history. The ruling follows it.

```
STOP AND ASK — spec 006 Fork 1, the zone regime of the T4 fixture

DONE AND GREEN
  Items 0, 1 and 2 are committed (the prompt; F2 RULED, D-11, D-12; the library's row in docs/sources.md and
  the dist/index.js read-through; golden/LICENSE.md and the formats). tools/t4-tdc.mjs exists and, under
  TZ=UTC (twice) and TZ=Pacific/Pago_Pago, writes the same 73,414 rows (one sha256 for all three), covering
  1900-01-01..2100-12-31 contiguously. The library's own flags agree with the rows on every checked day (0
  disagreements). Nothing under crates/ is touched.

THE FINDING
  Under TZ=Pacific/Kiritimati the generator's output differs. dist/index.js:344 (`unixFromJulian`) builds the
  Date it returns from `new Date().getTimezoneOffset()`, the offset of the machine's zone NOW; the date string
  then comes from the local-time getters (`getDateStr`, lines 72-77), which use the zone's offset AT THAT
  DATE. In UTC and in Pago_Pago the two coincide. Kiritimati's offset is +14 now and was about -10 (plus
  historical oddities) before the zone crossed the date line at the end of 1994, so each Gregorian date
  before 1995 comes out a day early.
  Reproduced: the first row of the range under TZ=UTC is `1900-01-01,1899,11,true,30` and the second
  `1900-01-02,1899,11,false,1`; under Kiritimati the first is `1900-01-01,1899,11,false,1`: the day UTC
  labels 1900-01-02. The rows then run contiguously to 1994-12-30 and the next is 1995-01-01: the local date
  1994-12-31 does not exist in that zone, and the generator's contiguity check fails ("not contiguous:
  1994-12-30 then 1995-01-01 (gap 2)"; 73,413 rows instead of 73,414). From 1995 on the dates agree with UTC.

EVIDENCE
  MEASURED: the table above; the first-rows line of each run (stderr of the generator); the generator's
  checks under TZ=UTC: isDoubledDay true on 3,312 rows (the rows imply 3,312), isLeapDay on 1,656 (1,656),
  isPreviousSkipped on 2,852 (2,840 on a first or single day + 12 on the second day of a repeated number),
  2,840 skipped numbers flagged by the library and 2,840 skip transitions implied by the rows, 0 absent but
  unflagged. Shapes in the range: 2,488 months, 75 leap months, 1,632 repeated tshes, 2,800 skipped tshes,
  201 12->1 rollovers, 3 rabjung-cycle boundaries.
  INFERRED: that the Kiritimati shift is the offset mismatch described (read from lines 72-77, 343-348 and
  reproduced in its effect, not isolated by a run that removes one variable: METHOD rule 27).
  RELAYED: none.

THE QUESTION
  Which regime defines the fixture? The prompt asks for three runs, byte-identical, with explicit date
  components and no clock. The library cannot give that in every zone: its Tibetan -> Gregorian route reads
  the zone's offset. Choosing what the oracle IS (the output under UTC, or something the generator
  corrects) is not local and not cheap to undo: it is what P6 later compares the engine against.

OPTIONS — a starting set, not a menu
  1. The generator sets process.env.TZ = 'UTC' for itself before loading the library (F14 (a)). The three
     runs are then byte-identical by construction; the report states that they prove the generator is
     independent of the caller's zone, not that the library is. The Kiritimati defect is a recorded finding.
     Cost: the three-zone test cannot exercise the library in Kiritimati any more; "by construction" gets
     its own check (a run of the generator with a deliberately zone-dependent stand-in: METHOD rule 31).
  2. The generator refuses to run unless its zone has offset 0 at every day of the range (F14 (b)); the
     fixture is the output under TZ=UTC; Pago_Pago is shown equal as evidence. Cost: the Kiritimati run
     "fails" by design and the prompt's "three outputs must be byte-identical" becomes "two identical, one
     refused": a change of the prompt's acceptance test.
  3. The generator cancels the library's offset by reading new Date().getTimezoneOffset() (F14 (c)). Output
     identical in every zone, DST included. Cost: it reads the clock's zone (the prompt forbids "the clock")
     and relies on the library's internals (lines 344-347) staying as they are.
  4. Something not listed.

IF THE ANSWER IS DELAYED
  Items 3 and 4 (the T5 tables from Janson v1 and P5), the R2 table, Item 8 (D-10) and the T5 half of Item 7a
  are independent of the T4 regime and can continue. The T4 fixture, P6, plant C's P6 half, the T4 half of
  Item 7 and the gate wait for the ruling.
```

**The ruling, and the handover (session 2).** On 2026-11-11 the human answered "agree" to option (a) with the advisor's three additions (ADDENDUM 1, R4): the generator sets its own zone to UTC before it loads the library, checks that the zone holds on every day, and refuses to write anything if it does not; two planted controls show that the check and the three-zone runs can fail, and one run names the cause. `FORKS.md` F14 carries it (`Status: RULED`, `Built in: spec 006 (tools/t4-tdc.mjs)`). **R5** (the same day, "agree"): the halt's reads outside GRANTS (`pdfinfo`; `cat`, `ls -la`, `git ls-files`, `git remote -v`) are accepted as reported and stay in "Latitude taken"; no grant is made after the fact; session 2 did not repeat them. Session 1's halt table of runs (UTC twice and Pago_Pago equal at `8bfd1407…4ce4`, Kiritimati exit 3) was MEASURED by session 1; session 2 re-measured the finding (T-b) and the hash (above).

## The squash commit message

```
spec 006: the reference fixtures: T5 tables from Janson v1, the T4 oracle, P5 and P6

golden/reference, the project's first data commit (CC BY 4.0, golden/LICENSE.md):
the Phugpa columns of Janson v1's Tables 1, 7, 8 and 9 (195 rows; two transcription
passes by two routes, identical) and every day 1900-2100 (73,414 rows) from
tibetan-date-calculator 1.2.3 at 97d4a004 (sha256 8bfd1407...), each with a
meta.yaml. tools/t4-tdc.mjs pins its zone to UTC, checks it on every day and
refuses otherwise: the library's dates depend on the zone's offset history
(dist/index.js:344; the cause named by a run under TZ=Etc/GMT-14), so the fixture
is trusted only as generated under UTC. crates/tshes/tests: P5 and P6 (zero
mismatches), the paper against itself, Table 7's year reading, five seeded
controls; round 005's planted control C (E806.m0 one day late), which passed all 13
exhaustive tests, turns P5 and P6 red three runs in three. 001 s5.3 is VERIFIED
for the month and year fields (a chain of two statements, Janson v1 pp. 6, 17, 20).
crates/tshes/src and Cargo.toml untouched; no dependency added; the test framework
wrote no file.

FORKS: F2 RULED (CC BY 4.0; Built in); F14 raised (spec 006 Fork 1) and RULED (a)
(Built in). OPEN 8 -> 7. DEBT: D-11 (TOOLING) and D-12 (NEGATIVE) filed; D-13
(COVERAGE) and D-14 (DOCS) filed; D-10 gains its dated line (stays OPEN).

Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>
```
