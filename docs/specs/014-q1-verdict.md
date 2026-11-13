# 014 — The Q1 verdict: Genden, within the scope of the three golden/buryat/ files of two datsans

```
spec: 014
prompt: docs/specs/prompts/014-q1-verdict.md
arc: Verdict
branch: spec/014-q1-verdict (cut from main at 756681ea6a39039470e140052f0f6eac0fa3090a, tree 1a55389bacf041c3bb3eed1513d93a181cf83155)
merge SHA: not yet merged
pushed: no
model: Sonnet 5.5 (claude-sonnet-5-5), effort High (session 1, Items 0 to 7, the halt; the switch point to the fallback model was never reached; one delegated reader, Item 5(a), which reports itself as claude-sonnet-5-5); Opus 5.5 (claude-opus-5-5), effort High (session 2, ADDENDUM 1, Items 8 to 14; no delegate)
gate: 180 tests passed, 0 failed (debug) / 180 passed, 0 failed (release) — no failure; session 2's run, once, with nothing in flight, on b9ed79887debfef7c6c50243645aab0610817143, the last commit that changes a path beyond this file, its docs/specs/INDEX.md row and its docs/ROADMAP.md row (11 legs run, 11 passed, 0 failed; the msrv leg also 180 / 0; the self-test 26 plants, 26 rejected); skipped legs: wasm-pack (crates/tshes-wasm does not exist, 002 §7). Session 1's run: 174 / 174 on e96187d
```

Status: accepted (the human's ruling on `FORKS.md` F38, 2026-11-13; in force on main from the merge, `docs/ROUNDS.md` §9)
Depends on: 001, 008, 009, 013
Resolves: F38
Milestone: M3
Governs: the answer to Q1 (001 §4) and D-1's default; crates/tshes/tests/ (golden/buryat/ under the gate; the Q1 differential)

## Purpose

This spec records the answer to Q1 (001 §4) that the human ruled on 2026-11-13 (`FORKS.md` F38, RULED (1); spec 014 ADDENDUM 1, R5), on the evidence of the Q1 differential of 001 §6.5 that this round measured: the three tier-T1 files of `golden/buryat/` against the engine's Phugpa, Tsurphu and Genden. It states the verdict, its basis, its scope, every mismatch with both values and how each is recorded, what the verdict rules out and what it leaves open, the one extra month observed, the conditions that reopen it, and the weakness it keeps. It changes nothing in the engine (no constant, epoch, rule, offset or variant), adds or changes no golden row, and says nothing about any datsan, date, tier or source outside its scope. The measurement, the controls and the investigation are in the report below the end marker (session 1, §1 to §5; session 2, S2.1 to S2.6); the figures cited here are asserted by the tests named under Properties.

## The verdict

**Genden, as the engine computes it (Janson v1 A.3; epoch E1747; a2 = 1/28, (7.13), `FORKS.md` F25), is the computation system the admitted Buryat sources support. Q1 (001 §4) is answered "Genden", within the scope below.**

**Its basis** is 001 §6.5's second rule, "Best system matches but with a small number of mismatches → each mismatch is investigated (transcription? source error? genuine variant?) before any verdict", after the investigation that rule requires (the report's §3). It is not the first rule: "One system matches 100% of T1–T3 rows" is read over all Buryat golden rows, as §6.5's opening words say ("For each candidate system, over all Buryat golden rows:"), and over them no system matches 100% (Genden 1,199 of 1,213). The per-source reading is not taken. 001 §6.5 does not define "a small number"; this spec defines no threshold, and the ruling applies the second rule to the counts below.

| System | Rows | tshes matches | Mismatches | Distinct days (1,165): every source value matches / none / the editions split |
|---|---|---|---|---|
| Phugpa (E806) | 1,213 | 1,145 | 68 | 1,099 / 58 / 8 |
| Tsurphu (E1732) | 1,213 | 1,184 | 29 | 1,137 / 20 / 8 |
| **Genden (E1747)** | 1,213 | **1,199** | **14** | 1,151 / 6 / 8 |

Per file, Genden is the best-matching system with no tie: Ivolginsky 2021 398 of 398, Aginsky 2025 407 of 413, Aginsky 2026 394 of 402 (Tsurphu 391 / 403 / 390, Phugpa 374 / 398 / 373).

## Scope

Three T1 files of two datsans: the Ivolginsky datsan's 2021 calendar (`golden/buryat/ivolginsky-datsan-2021.csv`, 2021-01-01 to 2022-02-02, 398 rows) and the Aginsky datsan's 2025 and 2026 editions (`aginsky-datsan-2025.csv`, 2025-01-01 to 2026-02-17, 413 rows; `aginsky-datsan-2026.csv`, 2026-01-01 to 2027-02-06, 402 rows): 1,213 rows, 1,165 distinct days (the editions share 2026-01-01 to 2026-02-17), 2021-01-01 to 2022-02-02 and 2025-01-01 to 2027-02-06. Sangha membership rests on the publishers' own statements (`DEBT.md` D-33). The Ivolginsky rows stand on the human's acceptance pending the datsan's answer (D-35).

## The 14 mismatches

All 14 lie in one lunation, 2025-12-20 to 2026-01-18: the 2025 edition's lunation 13, labelled «Второй зимний месяц – желтая мышь», of which the 2026 edition's first lunation (2026-01-01 to 2026-01-18, the same label) is the end. Every one is listed with both values.

| File | Date | Printed tshes | Genden's tshes | The other edition | Recorded as |
|---|---|---|---|---|---|
| aginsky-datsan-2025 | 2025-12-22 | 3 | 2 | (not in the 2026 edition) | UNEXPLAINED, OPEN |
| aginsky-datsan-2025 | 2025-12-23 | 4 | 3 | (not in the 2026 edition) | UNEXPLAINED, OPEN |
| aginsky-datsan-2025 | 2025-12-24 | 5 | 4 | (not in the 2026 edition) | UNEXPLAINED, OPEN |
| aginsky-datsan-2025 | 2025-12-25 | 6 | 5 | (not in the 2026 edition) | UNEXPLAINED, OPEN |
| aginsky-datsan-2025 | 2025-12-26 | 7 | 6 | (not in the 2026 edition) | UNEXPLAINED, OPEN |
| aginsky-datsan-2025 | 2025-12-27 | 8 | 7 | (not in the 2026 edition) | UNEXPLAINED, OPEN |
| aginsky-datsan-2026 | 2026-01-09 | 22 | 21 | 2025 edition: 21 | a disagreement between the datsan's two editions |
| aginsky-datsan-2026 | 2026-01-10 | 23 | 22 | 2025 edition: 22 | a disagreement between the datsan's two editions |
| aginsky-datsan-2026 | 2026-01-11 | 24 | 23 | 2025 edition: 23 | a disagreement between the datsan's two editions |
| aginsky-datsan-2026 | 2026-01-12 | 25 | 24 | 2025 edition: 24 | a disagreement between the datsan's two editions |
| aginsky-datsan-2026 | 2026-01-13 | 26 | 25 | 2025 edition: 25 | a disagreement between the datsan's two editions |
| aginsky-datsan-2026 | 2026-01-14 | 27 | 26 | 2025 edition: 26 | a disagreement between the datsan's two editions |
| aginsky-datsan-2026 | 2026-01-15 | 28 | 27 | 2025 edition: 27 | a disagreement between the datsan's two editions |
| aginsky-datsan-2026 | 2026-01-16 | 29 | 28 | 2025 edition: 28 | a disagreement between the datsan's two editions |

The 8 days of the 2026 edition are recorded as a disagreement between the datsan's two editions; the 2025 edition prints Genden's value on all 8. The 6 days of the 2025 edition are recorded as UNEXPLAINED and OPEN. (Session 1's class rule put 3 of the 6, 2025-12-22, 2025-12-23 and 2025-12-27, in its class VARIANT CANDIDATE because the printed value equals the engine's Phugpa there; that class names a rule of the round's prompt, not a finding that a variant exists, and the verdict records all 6 as UNEXPLAINED and OPEN.) Every printed value on these 14 days was re-read blind from the library PDFs and agrees with the golden row (session 1's §3(a): 84 of 84 cells). "A local variant" (001 §4) is not excluded by the verdict; no variant is computed or adopted, because none has a published basis (001 §6.5, §12; `CLAUDE.md` hard rule 7).

### The open lunation, described (METHOD rules 27 to 29: a description, no cause named)

- **The 2025 edition** prints lunation 13 as 1 to 30 on its 30 days (2025-12-20 to 2026-01-18), with no doubled and no skipped day. Genden (and Tsurphu) compute for the same days 1, 2, 2, 3, …, 7, 9, 10, …, 30: tshes 2 doubled (2025-12-22) and 8 skipped (before 2025-12-28). The 6 mismatches are the days 2025-12-22 to 2025-12-27, where the print is Genden's value plus one; from 2025-12-28 the two agree again.
- **The 2026 edition** prints the same lunation's last 18 days (2026-01-01 to 2026-01-18) as 13 to 20, 22 to 29, 29, 30: 21 skipped (before 2026-01-09) and 29 doubled (2026-01-17). Genden (and Tsurphu) compute 13 to 30 with no doubled and no skipped day, and so does the 2025 edition. The 8 mismatches are 2026-01-09 to 2026-01-16, where the 2026 edition's value on day d is Genden's on day d + 1.
- **The day-start offset probe** (the human's question of 2026-11-13, ADDENDUM 1 C3; Janson v1 Remark 6, p. 20: "the true date should be regarded as a kind of local Julian date that is offset from the standard astronomical one … so that it instead assume integer values at local (mean) dawn", FETCHED). A lunar day's end was moved by a constant offset δ of the day's start, against the true dates of Janson v1's formulas (spec 009's `janson_ref.py`, extracted verbatim; at δ = 0 it reproduces the engine's labels on every golden date, all three systems). MEASURED: Genden's mismatches over the 1,213 rows are 14 at δ = 0; 69 at +0.0447 d and 86 at +0.0650 d (meridians about 16° and 23° east of Lhasa's 91°, roughly those of Ivolginsk and Aginskoye: RECALLED / UNVERIFIED longitudes), 77 and 99 at the same offsets westward; on the scan from −0.15 to +0.15 d by 0.01 the minimum is 14, at 0 only; evaluated exactly between every breakpoint of the range, no δ gives fewer than 14, and 14 holds only for δ in [−1.2, +2.6) minutes [CORRECTED 2026-11-13: the measured interval is [−1.17, +2.55) minutes, as S2.1 and S2.4 give it; the rounding above is wider than it at both ends (the close review's FINDING 6.1, RELAYED; the human's C1)]. In the open lunation, Genden's lunar day 2 ends 14.3 minutes after the start of 2025-12-22 and lunar day 8 ends 14.3 minutes before the end of 2025-12-27; the 2025 edition's printed sequence, read by the same labelling rule, puts the ends of lunar days 2 to 7 each one calendar day earlier than Genden's true dates do (by at least 14.3, 79.7, 121.0, 138.4, 119.0 and 64.4 minutes), and the 2026 edition's those of lunar days 21 to 28 (by at least 153 to 762 minutes); in neither edition does one constant δ put every lunar day of the lunation on the day the print implies (the intersection of the ranges is empty). On the golden dates, 28 lunar days of Genden end within 15 minutes of a day boundary with both neighbouring days in the files; on 26 of them every file prints Genden's value on both days, and the other 2 are the ends of lunar days 2 and 8 above. **The instrument's blind spot (METHOD rule 35):** it tests one constant offset of the day's start only, not a different equation, rounding, table, epoch or rule; it says nothing about the cause of the open days.

## What the verdict rules out, and what it leaves

- **Tsurphu as Janson v1's A.2 computes it is ruled out.** On the 14 distinct days (15 rows) on which the engine's Tsurphu and Genden give different tshes, the source equals Genden on 15 rows and Tsurphu on 0. **The karana-Sun variant of Remark 22 (p. 40) was not measured** (`DEBT.md` D-19, OPEN).
- **Phugpa**: 1,145 of 1,213 rows (68 mismatches). On the 58 distinct days (60 rows) on which Phugpa and Genden differ, the source equals Genden on 57 rows and Phugpa on 3; on the 62 days (63 rows) on which Phugpa and Tsurphu differ, Tsurphu on 51 and Phugpa on 12. Two Сагаалган days the sources print, 2022-02-02 and 2025-03-01, are not the first day of the engine's Phugpa month 1; all five are the first day of Genden's (and Tsurphu's) month 1.
- Of the 1,165 distinct days, all three systems give one tshes on 1,098; at least two differ on 67.
- **Left open**: "None of the above / a local variant" (001 §4) for the open days; the karana-Sun Tsurphu; anything outside the scope.

## The extra month (n = 1)

One extra month occurs in the scope. In the 2025 edition the lunations 2025-01-01 to 2025-01-29 and 2025-01-30 to 2025-02-28 both carry month 12 in the engine's Genden (and Tsurphu): the engine flags `is_leap_month` on the **first** of the two (Janson v1, p. 11: the first of two months with one number is the leap month), and the source prints «Дополнительный зимний месяц» on the **second**. On every regular lunation of both Aginsky files the engine's Genden month equals the month the source's label implies (offset 0, 13 and 14 lunations). This spec records that relation for that one instance and maps nothing beyond it; how a client names the pair is `DEBT.md` D-40.

## Reopening

The verdict reopens, as a fork put to the human, on the first of:

1. a refusal under `DEBT.md` D-35 that withdraws the Ivolginsky rows;
2. an admitted T1–T3 Buryat source whose best-matching system is not Genden;
3. a зурхайч, or an admitted source, that names a published rule which accounts for the 6 open days;
4. D-33's evidence showing that a datsan of the scope is not a Sangha member.

## The weakness it keeps

The 6 open days of the 2025 edition (2025-12-22 to 2025-12-27) are unexplained. 001 §6.5's "each mismatch is investigated" is met by investigation, not by explanation: every mismatch was re-read, classed and described, and no cause is named for any of the 6.

## Properties

Each is a measurement the gate checks (the Q1 differential and its seeded controls, `crates/tshes/tests/`). Like the session-1 pin they PIN A MEASUREMENT, NOT A CORRECTNESS CLAIM (METHOD rule 33): a change in the engine, a golden file or the harness turns them red so that it is seen; none says which system is right. Every group is shown to fail on an in-memory plant inside its own test.

```
P1  The pinned measurement: per file and system, rows, tshes matches and the mismatching dates.
    Test: q1_pin_a_measurement_not_a_correctness_claim_tshes_matches_and_mismatching_dates_of_spec_014
          (its control: q1_pin_fails_on_the_planted_wrong_cell_and_names_the_day_for_every_system)
    Shapes: the nine file-system pairs; 0 to 29 mismatches each.
P2  The totals over the 1,213 rows and the three-way split over the 1,165 distinct days, per system.
    Test: q1_verdict_the_totals_over_the_rows_and_the_distinct_days_and_a_planted_cell_moves_them
    Shapes: the 48 days both editions print, 8 of them split.
P3  The Сагаалган table: the engine's year, month, leap flag and tshes on the five printed days, and
    whether each is the first day of month 1 (not a leap month), for the three systems.
    Test: q1_report_the_sagaalgan_days_in_each_system
    Shapes: a first day of month 1 whose tshes 1 is skipped (2025-03-01); 2027-02-07, the day after
            the 2026 edition's last row.
P4  The lunation boundary dates of every file and system, the Ivolginsky file included.
    Test: q1_verdict_the_lunation_boundary_dates_of_every_file_and_system_and_a_planted_boundary_moves_them
    Shapes: the edge fragments of each file; the four Phugpa boundaries that fall a day earlier.
P5  The days that separate each pair of systems, the source rows on them, and which system each equals.
    Test: q1_verdict_the_days_that_separate_the_systems_and_a_planted_tsurphu_value_is_counted
    Shapes: the 14 Tsurphu-Genden days (15 rows); the 58 Phugpa-Genden days (60 rows).
P6  The 14 Genden mismatches with both values; all in the lunation 2025-12-20..2026-01-18, one of the
    41 distinct lunations of the files (15 + 14 + 14, less the 2 the editions share); on the days
    the other edition prints, it prints Genden's value.
    Test: q1_verdict_every_genden_mismatch_lies_in_one_lunation_of_the_41_and_a_planted_split_is_seen
    Shapes: a lunation split across two files.
P7  The extra-month relation (n = 1) and the month offsets of every regular lunation.
    Test: q1_verdict_the_extra_month_relation_n_1_and_a_planted_month_label_moves_the_offsets
    Shapes: the one «Дополнительный» lunation; a file with none.
P8  The open lunation as each edition prints it and as Tsurphu and Genden compute it.
    Test: q1_verdict_the_open_lunation_as_printed_and_as_computed_and_a_planted_doubled_day_is_seen
    Shapes: a doubled and a skipped day in the engine where the 2025 edition has neither; the reverse
            in the 2026 edition.
P9  golden/buryat/ is read by the gate: the file list, the metas, the formats, the tshes steps and
    the month rule (DEBT.md D-37, PAID).
    Test: the 12 tests of crates/tshes/tests/buryat_golden.rs, among them
          buryat_the_csv_files_are_the_three_the_tests_name and
          buryat_the_tshes_steps_are_plus_1_plus_2_or_a_doubled_day_and_the_flags_agree
P10 The harness can fail and can pass: the T4 fixture through the same path (Phugpa 0 differences),
    the engine's own output (0), a planted wrong cell (exactly that day), a planted one-day shift
    (most days).
    Test: q1_control_t4_phugpa_has_zero_tshes_differences_on_the_golden_dates_through_the_same_path,
          q1_control_the_engines_own_output_has_zero_differences_through_the_harness,
          q1_control_a_planted_wrong_cell_is_reported_on_exactly_that_day_for_every_system,
          q1_control_a_planted_one_day_shift_is_reported_on_most_days_for_every_system
```

## Non-properties

- **No claim beyond the scope**: no datsan, edition, date range, tier or source outside the three files; nothing about Buryat practice in general.
- **No claim about the 6 open days**: no cause is named (see "The open lunation, described").
- **No claim on any datsan outside the scope**, held in the library or not (`DEBT.md` D-34 holds the Gunzechoinei and Khoymorsky calendars, without rows).
- **Nothing on a2 beyond F25** (`FORKS.md` F25, RULED (a)): D-23 did not fire (ADDENDUM 1, R7); the only day of spec 009's lists inside the files, 2025-11-19, is on Phugpa's list and not on Genden's.
- **No threshold** for 001 §6.5's "a small number of mismatches".
- **The T4 control does not exercise the golden file's own tshes column** on its way into the compared row; that step is covered by the other tests, by session 1's plant (i) on disk and by its outside recount (the reviewer's V1, RELAYED).
- **Measured once, outside the gate**: the day-start offset probe (session 2's S2.4) and the re-run of spec 009's TCG differential (S2.3; TCG's output does not enter the tree, `FORKS.md` F24). They are recorded in the report with their scripts; no test checks them.
- **No engine change and no default in the engine**: `DEBT.md` D-1 is PAID by this spec; a client that defaults System shows the system and this verdict's scope (001 §10 M5). The engine still takes `System` as a parameter.

## Forks

### F38 — The outcome of the Q1 differential (spec 014 Fork 1)

RULED (1) on 2026-11-13, in the shape of ADDENDUM 1's R5, recorded above. The options put were: (1) a verdict naming one system; (2) the stop-and-consult outcome of 001 §6.5; (3) more data first; (4) a verdict scoped to the Ivolginsky datsan. `FORKS.md` F4 (when to approach Гунзэчойнэй) was RULED (b) the same day: after M3, the approach carrying this verdict, its 6 open days and the two editions' disagreement as its questions.

## Stop rules

- A reopening condition above is met: put the verdict to the human again as a fork; do not change this spec's verdict in place.
- A new admitted source, or a change of the engine, turns a property above red: report it; never tune a constant, epoch or rule to restore it (001 §6.5, §12; `CLAUDE.md` hard rule 7).

## Reference

- S. Janson, "Tibetan calendar mathematics", arXiv:1401.6285v1 (`docs/sources.md`, `janson-1401.6285v1.pdf`, sha256 `7cafc7df…0f77`; FETCHED, `pdftotext -layout` of the library copy, session 2): A.3, "Mongolia", pp. 40 to 44 (the epoch E1747, (A.11) to (A.13), p. 42; the leap rule (A.17), p. 43); Remark 6, pp. 20 to 21; Remark 14, pp. 24 to 26 ((7.13) and (7.24)); Remark 22, p. 40 (the karana Sun of some Tsurphu almanacs); §5, p. 11 (the leap month is the first of two months with one number).
- Spec 009 (`docs/specs/009-henning-differential.md`): §1.1, §1.2 (the 53 days, `DEBT.md` D-23) and §21 (the record re-run by session 2); `docs/sources.md` rows `henning-tcg/` and `henning-kalacakra/`.
- Spec 013 (`docs/specs/013-buryat-sources.md`): the three golden files, their metas and `golden/buryat/README.md`; `docs/sources.md` rows `ivolginsky-datsan/` and `aginsky-datsan/`.

## Revisions

- r1 (2026-11-13): initial — the verdict as the human ruled it (F38, R5), with the assertions of session 2's Item 10.
- r2 (2026-11-13): two in-place corrections, in "The open lunation, described" and in the report's opening paragraph, on the human's ruling C1–C2 of 2026-11-13 (spec 015 Item 0).

— spec text ends here —

# Report — spec 014 (the Q1 differential)

The date is 2026-11-13 throughout: `date` printed `пт 13 нояб 2026 00:28:53 MSK` at the start and `01:36:48 MSK` after the gate; no day change. "The round's marker" is a token written nowhere in the tree before this round, never spelled in this file. Evidence tags: MEASURED (this round), RECALLED (an earlier round's record, cited by spec and section), INFERRED, RELAYED (`docs/ROUNDS.md` §4.3). Janson v1 means the library copy (sha256 `7cafc7df…0f77`, 86 pages); citations are FETCHED by `pdftotext -layout` of it, with the PDF page. The T1 files are the evidence; the T4 fixture appears only as a control of the harness. **Nothing here says which system any datsan or calendar follows, uses or is** (STOP RULE (4)); the report states counts, and which branch of 001 §6.5 they satisfy as its words read [CORRECTED 2026-11-13: the counts satisfy no rule of 001 §6.5 by themselves; which rule applies is the human's ruling R5 (b), as §1's correction says (the close review's FINDING 7.1, RELAYED; the human's C2)].

## 1. The decisive item: the Q1 differential, one table

`tshes` is the deciding field (METHOD rule 25: both sides see it). MEASURED, one run of the committed test `q1_differential` with `--nocapture`, debug and release outputs equal (`diff` silent over 3,836 lines each). **Best-matching** system per file marked ★ (the most tshes matches; no tie in any file).

| File (tier T1) | Rows | System | Matches | Mismatches |
|---|---|---|---|---|
| `ivolginsky-datsan-2021` (2021-01-01 to 2022-02-02) | 398 | Phugpa | 374 | 24 |
| | | Tsurphu | 391 | 7 |
| | | Genden ★ | 398 | 0 |
| `aginsky-datsan-2025` (2025-01-01 to 2026-02-17) | 413 | Phugpa | 398 | 15 |
| | | Tsurphu | 403 | 10 |
| | | Genden ★ | 407 | 6 |
| `aginsky-datsan-2026` (2026-01-01 to 2027-02-06) | 402 | Phugpa | 373 | 29 |
| | | Tsurphu | 390 | 12 |
| | | Genden ★ | 394 | 8 |

**Totals per system over the T1 rows** (398 + 413 + 402 = 1,213): Phugpa 1,145 matches and 68 mismatches; Tsurphu 1,184 and 29; Genden 1,199 and 14 (1,145 + 68 = 1,184 + 29 = 1,199 + 14 = 1,213). **Over the 1,165 distinct days** (the two Aginsky editions overlap on 2026-01-01 to 2026-02-17, 48 days: 1,165 = 398 + 413 + 402 − 48): every source value on the day equals the engine's / no source value does / the editions split (they differ on 8 days, one of them matching): Phugpa 1,099 / 58 / 8; Tsurphu 1,137 / 20 / 8; Genden 1,151 / 6 / 8 (each row sums to 1,165). An outside recount from the CSV files and the harness's dump of the engine's values (`recount.py`, Appendix) gives the same counts and the same dates for all nine file-system pairs and the totals.

**The controls** (METHOD rule 31; each shown to fail):

| Control | Result |
|---|---|
| Item 2(c), five plants in a `git archive` copy of `24821a1` (below, §4) | each plant turned at least one test red: 5 of 5 |
| Item 3(b), the T4 fixture's tshes on the dates of each file through the same path | Phugpa 0 differences on all three files (398, 413, 402 matches); Tsurphu 27 / 15 / 21 and Genden 24 / 15 / 21 differences (the T4 fixture is Phugpa's; these two counts compare systems with each other and are not evidence of practice; STOP RULE (5)) |
| Item 3(c), rows built from the engine's own output | 0 differences in tshes, `is_doubled` and `skipped`, all 3 systems × 3 files |
| Item 3(d)(i), one tshes changed on one day | exactly that day newly reported, for all 3 systems, in all 3 files (for example 2025-04-11, tshes 14 → 21: mismatches 15 → 16, 10 → 11, 6 → 7) |
| Item 3(d)(ii), every date shifted +1 day | disagreement on 396 / 384 / 388 of 398 rows (Phugpa / Tsurphu / Genden, Ivolginsky), 407 / 397 / 399 of 413, 392 / 384 / 385 of 402 |
| Item 3(e), the pin, on plant (d)(i) | red for all 3 systems of the planted file (`q1_pin_fails_on_the_planted_wrong_cell_and_names_the_day_for_every_system`: 3 violations, each naming 2025-04-11) |
| Item 5(a), the comparison script, on one planted wrong reading | exactly that cell flagged (c14, 2025-12-25, tshes 6 → 7) |

**Which branch of 001 §6.5 the counts satisfy, as its words read.** The words are: (i) "One system matches 100% of T1–T3 rows with ≥ 12 months of coverage → Q1 answered, record it in a spec"; (ii) "Best system matches but with a small number of mismatches → each mismatch is investigated (transcription? source error? genuine variant?) before any verdict"; (iii) "No system matches well → stop, report, and take the data to a зурхайч". MEASURED: Genden matches 100% of one file's rows (Ivolginsky, 398 of 398, 13 whole calendar months and 2 days) and 1,199 of 1,213 (98.85%) of all rows, 14 mismatches in two runs of consecutive days (Aginsky 2025: 2025-12-22 to 2025-12-27; Aginsky 2026: 2026-01-09 to 2026-01-16). If (i)'s "100% of T1–T3 rows" is read over all rows pooled, (i) is not satisfied; if it is read per source, (i) is satisfied by one source (one datsan, one print) and not by the other two. 001 §6.5 does not say which. (ii) is satisfied as written (Genden is the best system on every file; each of its 14 mismatches is investigated, §3); **001 §6.5 does not define "a small number of mismatches"**, so the counts are given and no threshold is applied. (iii): "matches well" is not defined either; the matching shares are Phugpa 94.4%, Tsurphu 97.6%, Genden 98.8% of rows. The words do not decide between (i) per source, (ii) and (iii). The decision is the human's (§12, F38). [CORRECTED 2026-11-13 (session 2, Item 13 (e); the reviewer's V4 and V5): "001 §6.5 does not say which" leaves out §6.5's opening words, "For each candidate system, over all Buryat golden rows:", under which rule (i) is read over all rows pooled, and over them no system matches 100%; no word of 001 §6.5 or §10 supports a per-source reading. And "(ii) is satisfied as written" applied the undefined "small number" while the same paragraph says no threshold is applied: the counts satisfy no rule by themselves. The human ruled the basis (ADDENDUM 1, R5 (b)): 001 §6.5's second rule, after its investigation, the per-source reading not taken; see the spec text, "The verdict".]

## 2. The Q1 differential (Item 4)

**Settings** (METHOD rule 23, from the test's own `Q1 SETTINGS` line): systems Phugpa, Tsurphu, Genden, each named explicitly (D-1); the three files of `golden/buryat/`, all T1; the engine through `tshes::to_zdate(date, system)` only; deciding field `tshes`; profile debug and release (outputs equal); commit `e96187d` (the harness is at `d161393`). The epochs: Phugpa E806, Tsurphu E1732, Genden E1747 (`crates/tshes/src/lib.rs:50-56`, re-checked; spec 008 says Tsurphu computes E1732, RECALLED, holds).

### 2(a) Per file and system, and the flags (reported, never counted)

`is_doubled` and `skipped` are derived from the tshes steps (spec 013, F9 (b)), so they are not independent evidence (§6, Item 1(c)). Rows compared for them: every row but the first of a file (397, 412, 401). Disagreements with the engine's flags, doubled / skipped: Ivolginsky Phugpa 16 / 17, Tsurphu 7 / 5, Genden 0 / 0; Aginsky 2025 Phugpa 11 / 15, Tsurphu 5 / 5, Genden 1 / 1; Aginsky 2026 Phugpa 14 / 14, Tsurphu 3 / 7, Genden 1 / 1. Coverage (days with data / days in the file's range): 398/398, 413/413, 402/402 (no gaps; 001 §6.3's coverage is complete inside each file). Rows sum: per file, matches + mismatches = rows for each of the nine pairs (asserted by the test).

### 2(b) Every mismatch (null results reported)

Format: date golden→engine, and the file's lunation number (`golden/buryat/README.md`'s rule, L1 first). Tier T1 for all. Ivolginsky has no printed month label, so no label is given. Genden on the Ivolginsky file: **none (0 of 398)**.

- `ivolginsky-datsan-2021`, Phugpa, 24: 2021-01-01 17→18 L1; 2021-01-02 18→19 L1; 2021-01-03 19→20 L1; 2021-01-14 1→2 L2; 2021-01-15 2→3 L2; 2021-01-16 3→4 L2; 2021-01-17 4→5 L2; 2021-02-19 7→8 L3; 2021-03-03 19→20 L3; 2021-04-05 23→24 L4; 2021-04-12 30→1 L4; 2021-05-17 5→6 L6; 2021-05-21 9→10 L6; 2021-06-01 21→22 L6; 2021-06-04 24→25 L6; 2021-07-11 1→2 L8; 2021-07-15 5→6 L8; 2021-07-30 21→22 L8; 2021-09-05 28→29 L9; 2021-09-06 29→30 L9; 2021-09-07 30→1 L9; 2021-09-18 12→13 L10; 2021-10-29 23→24 L11; 2021-11-03 28→29 L11
- `ivolginsky-datsan-2021`, Tsurphu, 7: 2021-01-03 19→20 L1; 2021-03-03 19→20 L3; 2021-06-02 23→22 L6; 2021-06-03 24→23 L6; 2021-09-23 18→17 L10; 2021-10-28 23→22 L11; 2021-12-19 16→15 L13
- `aginsky-datsan-2025`, Phugpa, 15: 2025-01-20 21→22 L1; 2025-02-04 6→7 L2; 2025-02-22 24→25 L2; 2025-02-28 30→1 L2; 2025-04-15 17→18 L4; 2025-04-26 28→29 L4; 2025-09-15 23→24 L9; 2025-11-10 20→21 L11; 2025-11-19 29→30 L11; 2025-12-04 14→15 L12; 2025-12-24 5→4 L13; 2025-12-25 6→5 L13; 2025-12-26 7→6 L13; 2026-01-31 13→14 L14; 2026-02-11 24→25 L14
- `aginsky-datsan-2025`, Tsurphu, 10: 2025-08-02 9→8 L8; 2025-09-26 5→4 L10; 2025-12-04 14→15 L12; 2025-12-22 3→2 L13; 2025-12-23 4→3 L13; 2025-12-24 5→4 L13; 2025-12-25 6→5 L13; 2025-12-26 7→6 L13; 2025-12-27 8→7 L13; 2026-01-31 13→14 L14
- `aginsky-datsan-2025`, Genden, 6: 2025-12-22 3→2 L13; 2025-12-23 4→3 L13; 2025-12-24 5→4 L13; 2025-12-25 6→5 L13; 2025-12-26 7→6 L13; 2025-12-27 8→7 L13
- `aginsky-datsan-2026`, Phugpa, 29: 2026-01-09 22→21 L1; 2026-01-10 23→22 L1; 2026-01-11 24→23 L1; 2026-01-12 25→24 L1; 2026-01-13 26→25 L1; 2026-01-14 27→26 L1; 2026-01-15 28→27 L1; 2026-01-16 29→28 L1; 2026-01-31 13→14 L2; 2026-02-11 24→25 L2; 2026-02-24 7→8 L3; 2026-03-17 28→29 L3; 2026-03-18 29→30 L3; 2026-03-19 30→1 L3; 2026-03-30 12→13 L4; 2026-03-31 13→14 L4; 2026-04-02 15→16 L4; 2026-04-03 16→17 L4; 2026-04-22 5→6 L5; 2026-05-08 21→22 L5; 2026-08-26 13→14 L9; 2026-12-11 2→3 L13; 2026-12-24 15→16 L13; 2027-01-14 6→7 L14; 2027-01-15 7→8 L14; 2027-01-16 8→9 L14; 2027-01-28 21→22 L14; 2027-01-29 22→23 L14; 2027-01-30 23→24 L14
- `aginsky-datsan-2026`, Tsurphu, 12: 2026-01-09 22→21 L1; 2026-01-10 23→22 L1; 2026-01-11 24→23 L1; 2026-01-12 25→24 L1; 2026-01-13 26→25 L1; 2026-01-14 27→26 L1; 2026-01-15 28→27 L1; 2026-01-16 29→28 L1; 2026-01-31 13→14 L2; 2026-07-01 17→16 L7; 2026-09-02 21→20 L9; 2026-12-24 15→16 L13
- `aginsky-datsan-2026`, Genden, 8: 2026-01-09 22→21 L1; 2026-01-10 23→22 L1; 2026-01-11 24→23 L1; 2026-01-12 25→24 L1; 2026-01-13 26→25 L1; 2026-01-14 27→26 L1; 2026-01-15 28→27 L1; 2026-01-16 29→28 L1

The lunations the Genden mismatches fall in: Aginsky 2025 L13 (2025-12-20 to 2026-01-18, «Второй зимний месяц – желтая мышь»), Aginsky 2026 L1 (2026-01-01 to 2026-01-18, the same printed label). Both runs lie inside one lunation each. Mismatch counts: 24 + 7 + 0 = 31 (Ivolginsky), 15 + 10 + 6 = 31 (2025), 29 + 12 + 8 = 49 (2026); 31 + 31 + 49 = 111 = 68 + 29 + 14.

### 2(c) Months, and the extra month (`FORKS.md` F31 (a): the Q1 round maps conventions; this measures the relation and chooses none)

MEASURED, by lunation, for the two Aginsky files (`Q1 BOUNDARIES`, `Q1 LUNATION`, `Q1 OFFSETS`):

| File | System | Engine's month boundaries = the source's lunation boundaries? | Offset (engine month − source month) mod 12 over the non-extra lunations | Engine's `is_leap_month` days in the file | The source's «Дополнительный …» days |
|---|---|---|---|---|---|
| 2025 | Phugpa | no: the source begins 2025-03-01, the engine 2025-02-28 (14 and 14 months) | {0: 12, 11: 1} (13 lunations) | none | 2025-01-30 to 2025-02-28 (30 days) |
| 2025 | Tsurphu | yes (14 and 14) | {0: 13} | 2025-01-01 to 2025-01-29 (29 days) | same |
| 2025 | Genden | yes (14 and 14) | {0: 13} | 2025-01-01 to 2025-01-29 (29 days) | same |
| 2026 | Phugpa | no: the source begins 2026-03-20, the engine 2026-03-19 | {0: 14} | none | none (the edition prints no extra month) |
| 2026 | Tsurphu | yes | {0: 14} | none | none |
| 2026 | Genden | yes | {0: 14} | none | none |

The relation, as measured: for Tsurphu and Genden, one constant offset (0) over every regular lunation of both files, and the extra month at **one** position in the one file that has one. The engine flags `is_leap_month` on the lunation 2025-01-01 to 2025-01-29, which the source labels «Третий зимний месяц – красноватая корова» (month 12 in `golden/buryat/README.md`'s order) and the engine's next lunation, 2025-01-30 to 2025-02-28, is a regular month 12 (year 2024, `leap=false`) that the source labels «Дополнительный зимний месяц»; the following lunation is month 1 in both. So the same two lunations carry the pair of month-12 labels, with the leap flag on the **first** in the engine (Janson v1, p. 11: the first of two months with one number is the leap month) and the source's «Дополнительный» on the **second**. For Phugpa the engine has no leap month in either file's range, and its first Aginsky 2025 lunation is month 11 where the source's is 12 (offset 11 once). No exception to "one constant offset with the extra month at one position" for Tsurphu and Genden; one instance of the extra month exists (n = 1), so nothing is generalised. Nothing is mapped (STOP RULE (4)). The Ivolginsky file prints no label: only its lunation boundaries are compared (15 source lunations; 15 engine months for each system; month and leap are not compared there).

### 2(d) The Сагаалган days (re-read from the metas' notes by `q1_the_sagaalgan_days_are_named_in_the_metas`)

"First of month 1" = the first day of the engine's month 1, not a leap month. Rows: year / month / tshes (flags) per system.

| Day (source) | Phugpa | Tsurphu | Genden |
|---|---|---|---|
| 2021-02-12 (Ivolginsky hural schedule) | 2021/1/1 yes | 2021/1/1 yes | 2021/1/1 yes |
| 2022-02-02 (Ivolginsky) | 2021/12/1 **no** | 2022/1/1 yes | 2022/1/1 yes |
| 2025-03-01 (Aginsky 2025) | 2025/1/2 **no** (its month began 2025-02-28) | 2025/1/2 (skipped 1) yes | 2025/1/2 (skipped 1) yes |
| 2026-02-18 (Aginsky 2026) | 2026/1/1 yes | 2026/1/1 yes | 2026/1/1 yes |
| 2027-02-07 (Aginsky 2026, the day after its last row) | 2027/1/1 yes | 2027/1/1 yes | 2027/1/1 yes |

### 2(e) D-23's TRIGGER

Spec 009 §1.2's three lists (RECALLED: 17 + 13 + 23 = 53 days; embedded in the test as `LISTS_009`, produced by a script that reads the tables; the test `q1_the_engine_still_computes_the_engine_values_spec_009_recorded_on_its_53_days` checks the engine's tshes on all 53 against the spec's "Engine" column: 53 of 53). **One of the 53 days lies inside the golden files' dates: Phugpa 2025-11-19** (the Tsurphu and Genden lists' days in 2021–2026, 2022-06-28 and 2024-09-19, fall outside every file). On it: the 2025 edition prints tshes 29, the second of two (line 324 of the CSV: 2025-11-18 and 2025-11-19 both 29, `is_doubled` true on the second); the engine's Phugpa gives 30; the (7.24) Phugpa value of spec 009 is 29 (TCG 2025/9/29). So the source equals Phugpa's (7.24) value and differs from Phugpa's engine value. Genden's value on the day equals the source's (Genden has no mismatch there). **D-23 as written** ("matches the (7.24) value of its **best-matching** system on such a day"): the best-matching system of that file is Genden, not Phugpa, and no day of Genden's list lies inside the files; **it does not fire.** The report notes, beside it and without changing F25, that the one list day inside the data is a day where the source equals the (7.24) value of a system that is not best. [CORRECTED 2026-11-13 (session 2, Item 13 (e); the reviewer's V3): "it does not fire" folded the TRIGGER into its F25 clause. The TRIGGER's first part was met: this round compared a source with the engine on a day of spec 009's lists and reported the source's value against the engine's and the (7.24) value (above). The F25 clause's non-firing rested on a reading, "a day of the best-matching system's own list", that D-23's words did not state. The human ruled (ADDENDUM 1, R7): not fired, the clause read as that reading, and D-23's TRIGGER corrected in place to its words; on 2025-11-19 Genden's two values of a2 give one label (the day is not on Genden's list), so a match there carries nothing on a2.]

### 2(f) The best-matching system, and D-19's TRIGGER (ii)

Genden for each file, no tie (398, 407, 394 matches against 391 / 403 / 390 for Tsurphu). **D-19 (ii) does not fire** (Tsurphu is not the best-matching system for any source). D-19 (i) is a condition on a verdict (§12).

### 2(g) The two Aginsky editions on their 8 disagreeing days (F30)

48 common days: 40 equal, 8 different (40 + 8 = 48), all in 2026-01-09 to 2026-01-16. Every system gives the 2025 edition's value on all 8 (Phugpa, Tsurphu, Genden):

| Day | 2025 edition | 2026 edition | Phugpa | Tsurphu | Genden |
|---|---|---|---|---|---|
| 2026-01-09 | 21 | 22 | 21 | 21 | 21 |
| 2026-01-10 | 22 | 23 | 22 | 22 | 22 |
| 2026-01-11 | 23 | 24 | 23 | 23 | 23 |
| 2026-01-12 | 24 | 25 | 24 | 24 | 24 |
| 2026-01-13 | 25 | 26 | 25 | 25 | 25 |
| 2026-01-14 | 26 | 27 | 26 | 26 | 26 |
| 2026-01-15 | 27 | 28 | 27 | 27 | 27 |
| 2026-01-16 | 28 | 29 | 28 | 28 | 28 |

### 2(h) How much the files can tell the systems apart (MEASURED, `discr.py` over the files and the dump; used in the options of §12)

Of the 1,165 distinct days, all three systems give the same tshes on 1,098 and at least two differ on 67. Phugpa and Tsurphu differ on 62 days, Phugpa and Genden on 58, **Tsurphu and Genden on 14**. On those 14 days (15 rows: Ivolginsky 7, Aginsky 2025 4, Aginsky 2026 4) the source equals Genden on all 15 and Tsurphu on none; on the 58 Phugpa–Genden days the source equals Genden on 24 + 12 + 21 = 57 rows of 60 rows and Phugpa on 0 + 3 + 0.

## 3. Item 5: every mismatch of the best-matching system investigated

The best-matching system is Genden for each file: 14 mismatches (Ivolginsky 0, Aginsky 2025 6, Aginsky 2026 8).

**(a) Transcription.** The plan was fixed before any reading (`plan.py`, which holds the decoy rule; Appendix): one crop per mismatching day (14; the cap of 40 per file was not reached; 0 days not re-read) and 14 decoys, the j-th decoy being the first date at or after the j-th mismatching day + 14 days on which the file's golden tshes equals all three systems' on the day before, the day and the day after, outside the edition's last (partial) month and more than 2 days from earlier decoys. Ivolginsky has no mismatch, so no crop and no decoy. Rendering: `pdftoppm -r 200` of the library PDFs (the Aginsky editions' day cells read clearly at 200 dpi: the reader read all 84 cells); each crop is the row-label strip and three day columns (the day before, the day, the day after), Gregorian day, lunar number and weekday rows (`crops.py` places a crop from `pdftotext -bbox` column positions and reads no value from it). The crop order is the ascending sha256 of the file name and date. One delegated reader, blind: told no golden value, no engine value, and not which crops are decoys; it read 28 crops (42 cells of mismatch crops and 42 of decoy crops) and wrote 84 lines. **Compared by `compare_reading.py` with the golden rows (Gregorian day, tshes and the weekday of the date): 84 of 84 cells agree, 0 disagreements, 0 UNSURE notes, on every mismatch crop and every decoy.** STOP RULE (3) did not fire. The reader is `claude-sonnet-5-5` (its own report); the brief is in the Appendix, its GRANTS block equal to the prompt's lines 127 to 152 (`diff` silent). Re-verification of its claims (CLAUDE.md hard rule 11): the file it wrote has 84 lines (3 per crop) and the script found each cell equal; its "no command run, only the brief and the 28 crops read" is the reader's statement; the harness recorded 31 tool uses where 30 were expected (the brief, 28 crops, one write), so one use is unexplained here and was not checked further (the reader said it read nothing else).

**(b) Source disagreement.** All 8 mismatches of Aginsky 2026 (2026-01-09 to 2026-01-16) are days where the two editions disagree (F30); the other edition prints the value Genden gives (2025 edition 21 to 28 against 2026 edition 22 to 29). None of the 6 of Aginsky 2025 is in the overlap (2025-12-22 to 2025-12-27 precede the 2026 file).

**(c) Variant candidates** (by the rule: a mismatch on a day of spec 009's lists, or one whose value equals another system's on that day). Genden's list (`LISTS_009`) has no day in either file. The value equals Phugpa's on 2025-12-22 (3), 2025-12-23 (4) and 2025-12-27 (8) of the 2025 file; on 2025-12-24, 25 and 26 it equals no other system's (Phugpa and Tsurphu give 4, 5, 6 and 4, 5, 6 where the source prints 5, 6, 7).

**(d) Pattern** (described, not explained; METHOD rules 27 to 29). Two runs of consecutive days, each inside one lunation: 2025-12-22 to 2025-12-27 (6 days, L13 of the 2025 edition) and 2026-01-09 to 2026-01-16 (8 days, L1 of the 2026 edition). In the 2026 run the source's tshes on day d equals the engine's on d + 1 on all 8 days (a one-day shift); in the 2025 run it does on 5 of 6 (2025-12-22 to 2025-12-26) and not on 2025-12-27, so the run is not a pure one-day shift. Isolated mismatches: none (no Genden mismatch is outside a run). Tsurphu and Phugpa on the same two stretches: Tsurphu mismatches the same 6 days of the 2025 run and the same 8 of the 2026 run; Phugpa mismatches 3 of the 6 (2025-12-24 to 26) and the 8 of the 2026 run.

**(e) The classes** (rule fixed in advance, in order: TRANSCRIPTION if (a) disagreed; else SOURCE DISAGREEMENT; else VARIANT CANDIDATE; else UNEXPLAINED):

| File | Mismatches | TRANSCRIPTION | SOURCE DISAGREEMENT | VARIANT CANDIDATE | UNEXPLAINED |
|---|---|---|---|---|---|
| Ivolginsky 2021 | 0 | 0 | 0 | 0 | 0 |
| Aginsky 2025 | 6 | 0 | 0 | 3 | 3 |
| Aginsky 2026 | 8 | 0 | 8 | 0 | 0 |
| Sum | 14 | 0 | 8 | 3 | 3 |

0 + 0 + 0 = 0; 0 + 0 + 3 + 3 = 6; 0 + 8 + 0 + 0 = 8; 0 + 6 + 8 = 14. "UNEXPLAINED" means only that none of (a) to (c) accounts for the day: it names no cause. A "variant candidate" is a class of the prompt's rule, not a finding that a variant exists.

## 4. Item 2, golden/buryat/ under the gate (pays D-37)

**Shape: considered X, chose Y because Z.** *Considered* (A) one file of tests with the loader inline; (B) a shared helper in `crates/tshes/tests/common/` (`buryat.rs`, with a named refusal code per defect) and one test file per concern; (C) a build script or a CLI in `tools/`. *Chose* (B): the existing tests share `common/` helpers, the loader must be reused by the Q1 harness, and a refusal code lets a planted control assert which check fired and not only that something did; (C) is out of scope (`tools/` is not touched). `buryat_golden.rs` has 12 tests, all ordinary: the file list; metas present, keys in order, none empty; loader formats and row counts and dates; the step check; the month rule; the add-days helper; and refusal tests on in-memory plants for the loader (19 plants, each with its code), the step check (6), the month rule (6) and the meta check (4).

**METHOD rule 31, five plants in a `git archive` copy of `24821a1e56502e47c49ce87e8bde820322a73879`** inside a `mktemp -d` directory (`plant.py`; `CARGO_TARGET_DIR` in the same directory; control: the unplanted copy ran 12 passed, 0 failed):

| # | Plant | Tests that went red | First failing assertion |
|---|---|---|---|
| (i) | a +3 step: `aginsky-datsan-2025.csv` line 12 (2025-01-11) tshes 13 → 14 | `buryat_the_tshes_steps_are_plus_1_plus_2_or_a_doubled_day_and_the_flags_agree`; `buryat_the_step_check_refuses_a_plus_3_step_and_each_flag_defect` (its first line asserts the real file passes) | `STEP: aginsky-datsan-2025.csv: 2025-01-11: tshes goes from 11 to 14 (step +3, modulo 30)` (10 passed, 2 failed) |
| (ii) | an eighth field on `ivolginsky-datsan-2021.csv` line 100 (2021-04-09) | `buryat_every_csv_loads_with_the_formats_of_its_readme`; `…_months_follow_the_lunation_rule…`; `…_tshes_steps_are_plus_1…`; `…_month_rule_refuses…`; `…_loader_refuses_each_defect_it_names` (they load the real file) | `FIELD-COUNT: ivolginsky-datsan-2021.csv: row 99 has 8 fields, not 7` (7 passed, 5 failed) |
| (iii) | `aginsky-datsan-2026.meta.yaml` deleted | `buryat_every_csv_has_its_meta_file_and_every_meta_file_its_csv`; `buryat_every_meta_file_has_the_readme_keys_in_order_and_none_empty` | `the meta files of golden/buryat/` (left: 2 names, right: 3); then `cannot read golden/buryat/aginsky-datsan-2026.meta.yaml: No such file or directory` (10 passed, 2 failed) |
| (iv) | `extra-datsan-2099.csv` added (a valid copy of one file, which no test names) | `buryat_the_csv_files_are_the_three_the_tests_name` | `the CSV files of golden/buryat/` (left has `extra-datsan-2099.csv`) (11 passed, 1 failed) |
| (v) | a label moved: lunation 4 of the 2025 edition (from 2025-03-30) takes lunation 3's month 1 and its printed label | `buryat_the_months_follow_the_lunation_rule_of_the_readme`; `buryat_the_month_rule_refuses_a_label_on_the_wrong_lunation_and_each_form_defect` | `LABEL-ORDER: aginsky-datsan-2025.csv: lunation 4 from 2025-03-30: month 1 follows month 1` (10 passed, 2 failed) |

Each plant turned at least one test red: 5 of 5. D-37 is PAID (`e96187d`).

## 5. Item 1, what the comparison stands on

**(a) The engine** (public items the harness calls, MEASURED by reading the source): `tshes::to_zdate(date: GregorianDate, system: System) -> Result<ZDate, Error>` (`crates/tshes/src/lib.rs:63`); `GregorianDate::new` (`civil.rs:105`); `System` (`calendar.rs:9`: `Phugpa`, `Genden`, `Tsurphu`); `ZDate::year` (`calendar.rs:123`), `ZYear::number` (`naming.rs:237`), `ZDate::month` (`calendar.rs:129`), `MonthNo::get` (`:68`), `ZDate::is_leap_month` (`:136`), `ZDate::tshes` (`:142`), `Tshes::get` (`:42`), `ZDate::is_doubled` (`:148`), `ZDate::skipped_before` (`:154`). The fields: `tshes` is 1 to 30, Janson §3, p. 7 (`calendar.rs:21-23`); `month` 1 to 12, §5, p. 11 (`:47`); `is_leap_month` is the first of two months with one number, §5, p. 11 (`:133-135`); `is_doubled` is true on the **second** occurrence of a repeated tshes (`calendar.rs:75-77`; `phugpa.rs:531-533`: "of two days with the same number, the first is the leap day; 001 §5.1 flags the second occurrence"); `skipped_before` is the tshes omitted immediately before the day, set on the first day of its label when the label before it has no day (`calendar.rs:78`, `:152-153`; `phugpa.rs:534-540`). **That is the convention `golden/buryat/README.md` states** (`is_doubled` true on the second of two consecutive days with one tshes; `skipped` the number omitted immediately before the day, previous tshes + 2 modulo 30, F9 (b)), and the control of Item 3(c) (the engine's own flags through the harness: 0 disagreements) and 3(b) (the T4 fixture with flags derived from steps against Phugpa: 0) show the two agree on 1,213 rows. Epochs: §2 above.

**(b) D-37 re-measured with a grep for `buryat` alone** (R3 (c)'s method), MEASURED at `756681e` over `crates tools .github`: one hit, `crates/tshes/src/phugpa.rs:1049`, a doc comment about Buryat practice, no reader. A grep for `golden`: `phugpa.rs:1479` (`include_str!` of `golden/reference/janson-v1-table6-epochs-806.csv`, `#[cfg(test)]`), `crates/tshes/tests/common/fixtures.rs:1, 15, 18`, `p6_t4_fixture.rs:2`. `git grep` for `read_dir`, `WalkDir`, `glob(` over `crates tools`: none (exit 1) at the base. By any mechanism: `tools/gate-self-test.sh:39`, the whole-tree `tar -C "$REPO_ROOT" --exclude=.git --exclude=target --exclude=.codewhale -cf - . | tar -C "$SCRATCH/$1" -xf -`, copies `golden/buryat/` and checks nothing. **R3 (a) HOLDS; R3 (b) HOLDS; R3 (c) HOLDS as a structural fact** (`fixtures.rs:16-21` fixes the prefix `golden/reference/`, so a relative name such as `../buryat/x` would reach `golden/buryat/` and not match a grep for `golden/buryat`; no such reader exists); **R3 (e) HOLDS** (a case-insensitive grep for `meta` in `crates tools .github`, minus `metadata` and `import.meta`, finds only `tools/gate.sh`'s variable holding `cargo metadata`; no code reads a `.meta.yaml`; D-39 is filed). R3 (d) and (f) were not re-measured (the prompt asks for (a) to (c) and (e)). This round's `buryat.rs` now uses `std::fs::read_dir` in the tests (two helpers); the grep above was the base.

**(c) The fields counted.** `tshes` decides: it is printed in all three files and computed by the engine, both sides see it. `is_doubled` and `skipped` follow from the tshes sequence and are reported, never counted (§2(a)). `month` and `is_leap_month` exist only in the Aginsky files and are compared per lunation (§2(c)); the Ivolginsky file prints no label, so no month or leap field is compared there (METHOD rule 25). The decisive table counts `tshes` for that reason.

## 6. The rulings and findings carried (R1 to R4)

- **R1** (the human, 2026-11-13: "Agree with all recommendations, go merge"): `FORKS.md` F37 RULED (c); `DEBT.md` D-35's added sentence; `docs/ROADMAP.md` Gate 1's state cell and the Revisions line r6. M0 is recorded as met on that condition.
- **R2** (spec 013's Latitude 1 to 11 accepted as reported): recorded here only; no grant after the fact.
- **R3** (the review of spec 013's tip, RELAYED): re-verified in §5(b).
- **R4**: D-37 re-measured by R3 (c)'s method (§5(b)); D-39 filed (R3 (e) held); the reader's model recorded (§3(a)).

## 7. The four research legs (003 §4, P3)

1. **① Reference.** The three golden files are the evidence (datsan publications). A зурхайч is not reachable from this round (STOP RULE (7); `FORKS.md` F4 is OPEN). Found against the approach: the Aginsky 2025 and 2026 editions disagree on 8 days and the 2026 edition's 8 are Genden's 8 mismatches there; no single system reproduces every printed day (§2).
2. **② Field.** The T4 fixture (Lotsawa House's Phugpa) is the harness's control only. Henning's TCG is spec 009's (RECALLED) and D-24's, not run here. The RuStore app of 001 §11 is not consulted (`FORKS.md` F10 is OPEN). The library holds, besides these, `henning-kalacakra/` and `henning-tcg/` (spec 009), the Gunzechoinei, Khoymorsky, Kalmyk and Rabten calendars (D-34), `datsan-rinpoche-bagsha/` (F22): none was read this round. Found against the approach: nothing new.
3. **③ Science.** Janson v1 (`pdftotext -layout` of the library copy; hits of a grep for `mongol|buryat|buriat|kalmyk|russia`, with lines of the text copy): A.3, "Mongolia", opens at line 1877 (PDF page 40) and continues on p. 41; the sentence on p. 41 (line 1901): "According to [21], the Buryats and Tuvinians of Siberia (in Russia) follow the New Genden version (i.e., the version described here), while the Kalmyk Mongols in Russia follow the Phugpa version." (FETCHED; [21] is Berzin; the paper says on the same page "I have no reliable Mongolian sources, and not even a printed calendar as an example"). p. 43 (line 2027): "the Mongolian version has the same leap months as the Tsurphu version (although one may start a day before the other), see Table 7." p. 1 (line 35): Bhutan and Mongolia "also use versions of the Tibetan calendar"; the other hits are Tables 6 to 9's column headed "Mongolia" (pp. 50 to 54), A.13's comparisons, the references, and C.4 (line 2909, Mongolian definition points). No hit names Buryatia or the Buryat tradition beyond that one sentence; no page of the paper uses the word Buryat elsewhere (grep). This is a secondary claim reported by Janson (METHOD rule 46: documentation is not evidence); it is not used here to prefer any system. A.2 and A.3 give where each version is used only in these sentences.
4. **④ Contradicts anything already shipped?** 001 §4's description of the candidates ("its leap months coincide with Tsurphu's, while individual days can fall one day apart"): consistent with the measurement (Tsurphu and Genden flag the same leap lunation in 2025; they differ on 14 of 1,165 days, each by one day). F25's ruling (a2 = 1/28 for all three): not contradicted, and D-23 does not fire. `golden/buryat/README.md`: its statement that the files "are never compared here with any engine system" was true at spec 013 and is this round's change (the comparison is the Q1 round's); the README's convention for `is_doubled` and `skipped` equals the engine's (§5(a)). One tension reported, not resolved: the README says the Q1 round maps the convention of the extra month; §2(c) measures it and maps nothing.

## 8. The predictions, scored (METHOD rule 3)

| # | Prediction | Score | Measured value |
|---|---|---|---|
| 1 | Every PRECONDITION and ARTIFACT check passes; the engine source is unchanged since `d174965` | HELD | §10: all outputs quoted; `git diff --stat d174965 HEAD -- crates/tshes/src` printed nothing at the start and at the end |
| 2 | Nothing but the self-test's tar copy reaches `golden/buryat/`; R3 (a), (b), (e) hold | HELD | §5(b) |
| 3 | T4 against Phugpa on the golden dates: 0 differences | HELD | 0 of 398, 0 of 413, 0 of 402 |
| 4 | Genden is the best-matching system for each of the three files | HELD | 398, 407, 394 matches; no tie |
| 5 | Each Сагаалган day is the first of Genden's month 1; 2022-02-02 and 2025-03-01 are not the first of Phugpa's month 1 | HELD | §2(d) |
| 6 | Genden flags the lunation 2025-01-30 to 2025-02-28, which the 2025 edition labels «Дополнительный зимний месяц», as a leap month | **FAILED** | Genden flags 2025-01-01 to 2025-01-29, the lunation before it; 2025-01-30 to 2025-02-28 is its regular month 12 (§2(c)) |
| 7 | No system matches 100% of the T1 rows (the editions differ on 2026-01-09 to 16) | HELD over all T1 rows (Genden 1,199 of 1,213); **FAILED per file**: Genden matches 398 of 398 Ivolginsky rows | §1 |
| 8 | On 2025-11-19 the 2025 edition prints 29, the second of two (line 324); that differs from Phugpa's engine value and equals its (7.24) value; D-23 fires only if Phugpa is that file's best | HELD | line 324 is `2025-11-19,29,9,false,true`; Phugpa engine 30, (7.24) 29; Phugpa is not best; D-23 does not fire (§2(e)) |
| 9 | The blind re-read agrees with the golden rows on every crop | HELD | 84 of 84 cells |
| 10 | Every planted control turns its test red; the T4 control is green | HELD | §1, §4 |
| 11 | OPEN is 5 at the base and 6 at the halt | HELD | §9 |

## 9. The open-forks audit and the marker greps, at every commit

`expected = previous OPEN + forks raised + gated findings filed with options and none chosen − rulings recorded` (METHOD rule 60), measured with `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`. The OPEN rows at the base: F4, F5, F7, F8, F10 (5; the advisor's count, RELAYED, re-measured: 5).

| Commit | Previous | Raised | Filed, none chosen | Rulings recorded | Expected | Measured | Marker grep |
|---|---|---|---|---|---|---|---|
| `d35dd7b` (the prompt) | 5 (base) | 0 | 0 | 0 | 5 | 5 | no hit |
| `afc69ca` (Item 0; F37 is born RULED) | 5 | 0 | 0 | 0 | 5 | 5 | no hit |
| `24821a1` (Item 2) | 5 | 0 | 0 | 0 | 5 | 5 | no hit |
| `d161393` (Item 3) | 5 | 0 | 0 | 0 | 5 | 5 | no hit |
| `e96187d` (D-37 PAID; F38 filed) | 5 | 1 (F38) | 0 | 0 | 6 | 6 | no hit |
| the halt (this file) | 6 | 0 | 0 | 0 | 6 | 6 | no hit |

The marker grep is `git grep -n` for the marker over everything but the frozen prompt copy. Control (METHOD rule 31): the same grep over a scratch file that holds it once is not run in the tree; the round's marker never occurs in a tracked file, so each grep printed nothing and exit 1.

## 10. Preconditions, quoted (CLAUDE.md rule 14: every output)

(a) `git status --porcelain` printed nothing; `rev-parse --abbrev-ref HEAD` printed `main`. (b) `rev-parse main` printed `756681ea6a39039470e140052f0f6eac0fa3090a`. (c) `git diff --stat d174965 HEAD -- crates/tshes/src` printed nothing (re-run at the end: nothing); `docs/specs/010-publication.md`, "The history map", row 10, reads `07b5bbf0632debab15a32175970fbd6c83a70cce` → `d174965afc95cee0e7e0091261922c88aba56974` (checked). (d) `ls /home/kitaeza/zurkhai-papers/` succeeded (13 entries). ARTIFACT: `sha256sum` of the four PDFs printed `2e140c65…0fe25` (Ivolginsky), `12137318…d7bb` (Aginsky 2025), `13e7d720…a156` (Aginsky 2026), `7cafc7df…0f77` (Janson v1), each equal to the prompt's; the files `wc -l` to 399, 414, 403 lines (398, 413 and 402 data rows, equal to each meta's `covers`); the T4 fixture 73,415 lines. Spec 014 was free (`ls docs/specs/*.md` listed 001 to 013). The Read-first list: `CLAUDE.md`, `docs/ROUNDS.md`, `docs/METHOD.md` and `GATED.md` in full; `FORKS.md`, `DEBT.md`, `docs/ROADMAP.md` in full (the prompt asked for some entries by their headings only); `docs/specs/001-init-spec.md`, `002-rust-code-standard.md`, `003-project-method.md`, `TEMPLATE.md`, `INDEX.md` in full; `008-genden-tsurphu.md` lines 1 to 120 (the head and §1); `009-henning-differential.md` lines 1 to 155 and 380 to 420 (head, §1.1, §1.2, the start of §13) and the opening of §21 (the rest is spec 009's scripts, not read); `013-buryat-sources.md` lines 1 to 422 (through Appendix A) and the heading list (Appendix B's scripts, lines 423 to 1744, were not read); `golden/buryat/README.md` and the three metas, `golden/reference/README.md`, `golden/LICENSE.md` in full; `docs/sources.md` the Janson, Ivolginsky, Aginsky and bibliography rows by grep.

## 11. Every spec property, and every prompt question

| Item | State | Evidence |
|---|---|---|
| 0: branch, prompt, number, records | DONE | branch from `756681e`; the prompt `cmp`-ed before every commit; F37 (RULED), D-35 sentence, D-38 PAID, D-39 filed, ROADMAP r6 (`afc69ca`) |
| 1: what the comparison stands on | DONE | §5 |
| 2: `golden/buryat/` under the gate | DONE | §4; 12 tests; D-37 PAID |
| 3: the harness and its controls | DONE | §1 controls; 15 tests in `q1_differential.rs` (the pin `q1_pin_a_measurement_not_a_correctness_claim_tshes_matches_and_mismatching_dates_of_spec_014`, its failure `q1_pin_fails_on_the_planted_wrong_cell_and_names_the_day_for_every_system`); the debug and release outputs equal |
| 4: the differential report | DONE | §2 (a) to (g), (h) beyond the prompt |
| 5: every mismatch of the best system investigated | DONE | §3; classes sum 14 |
| 6: the verdict as a STOP AND ASK | DONE (put, not chosen) | §12; `FORKS.md` F38 OPEN |
| 7: the halt's gate and commit | DONE | head block; §13 |

**The stop rule that governed the round:** STOP RULE (9), the halt at Item 6 by design; STOP RULES (2), (3), (4), (5), (6) were read and none fired: the engine and the golden rows are unchanged, the blind re-read agreed on every crop, no verdict is written, the T4 data is a control only, D-23's TRIGGER is not met. [CORRECTED 2026-11-13 (session 2, Item 13 (e); V3): D-23's TRIGGER was met in its first part (a source compared on a list day, and reported); its F25 clause did not fire only under a reading its words did not state, which the human has since ruled (ADDENDUM 1, R7; §2(e)'s correction).] Every prompt question is answered in the section it names: Item 1(a) to (c) in §5; Item 2(c) in §4; Item 3(b) to (e) in §1; Item 4(a) to (g) in §2; Item 5(a) to (e) in §3; the four legs in §7.

**Gate** (Item 7). `tools/gate.sh` ran once, in the background with nothing in flight, on `e96187ddf9142e78a51ecdd6cfb64e5a140077fb`, the last commit that changes a path beyond this file: `GATE: 11 legs run — 11 passed, 0 failed, 1 skipped`. Sums of the `test result:` lines per leg: `test` 174 passed 0 failed, `test_release` 174 and 0, `msrv` 174 and 0; `SELF-TEST: 26 plants — 26 rejected with their named diagnostic, 0 failures`; `git status --porcelain` printed nothing after the run. Spec 013's recorded count is 147 in each profile (RECALLED); the difference is 27 = 12 (`buryat_golden.rs`) + 15 (`q1_differential.rs`), all new, all passing. The new tests: the 12 of `buryat_golden.rs` (`buryat_the_csv_files_are_the_three_the_tests_name`, `buryat_every_csv_has_its_meta_file_and_every_meta_file_its_csv`, `buryat_every_meta_file_has_the_readme_keys_in_order_and_none_empty`, `buryat_every_csv_loads_with_the_formats_of_its_readme`, `buryat_the_tshes_steps_are_plus_1_plus_2_or_a_doubled_day_and_the_flags_agree`, `buryat_the_months_follow_the_lunation_rule_of_the_readme`, `buryat_the_add_days_helper_inverts_the_day_number`, `buryat_the_loader_refuses_each_defect_it_names`, `buryat_the_step_check_refuses_a_plus_3_step_and_each_flag_defect`, `buryat_the_month_rule_refuses_a_label_on_the_wrong_lunation_and_each_form_defect`, `buryat_the_label_words_give_the_months_of_the_readme_seasonal_order`, `buryat_the_meta_check_refuses_a_missing_key_a_wrong_order_and_an_empty_value`) and the 15 of `q1_differential.rs` (`q1_control_t4_phugpa_has_zero_tshes_differences_on_the_golden_dates_through_the_same_path`, `q1_control_the_engines_own_output_has_zero_differences_through_the_harness`, `q1_control_a_planted_wrong_cell_is_reported_on_exactly_that_day_for_every_system`, `q1_control_a_planted_one_day_shift_is_reported_on_most_days_for_every_system`, `q1_the_engine_still_computes_the_engine_values_spec_009_recorded_on_its_53_days`, `q1_pin_a_measurement_not_a_correctness_claim_tshes_matches_and_mismatching_dates_of_spec_014`, `q1_pin_fails_on_the_planted_wrong_cell_and_names_the_day_for_every_system`, `q1_report_the_counts_per_file_and_system_the_mismatches_and_the_totals`, `q1_dump_the_engines_values_on_every_distinct_golden_date_for_an_outside_recount`, `q1_report_the_lunation_boundaries_the_months_and_the_extra_month_of_the_aginsky_files`, `q1_the_sagaalgan_days_are_named_in_the_metas`, `q1_report_the_sagaalgan_days_in_each_system`, `q1_report_d23_the_days_of_spec_009_inside_the_golden_files_and_whether_it_fires`, `q1_report_the_two_aginsky_editions_on_the_days_they_disagree`, `q1_report_the_mismatches_of_the_best_system_by_class_run_and_one_day_shift`). The WASM smoke test (`wasm-pack`) was not run: `crates/tshes-wasm` does not exist (002 §7). The Q1 report leg and the rules validator do not exist as gate legs; the Q1 differential runs inside the ordinary `test` legs. Every test the report names is a `fn` in the tree (METHOD rule 7: `grep -c "fn buryat_\|fn q1_"` printed 12 and 15). `D-37`'s "under the gate from spec 014" phrase in F28's `Built in:` line is true at the halt (Item 2 is built), so no correction is made.

## 12. STOP AND ASK — spec 014 Fork 1, the outcome of the Q1 differential (F38)

```
STOP AND ASK — spec 014 Fork 1, the outcome of the Q1 differential

DONE AND GREEN
  Items 0 to 5 and the gate (Item 7): golden/buryat/ is read by 12 tests in the ordinary gate (D-37 PAID);
  the Q1 harness and its controls are built and shown to fail (T4 through the same path: Phugpa 0
  differences); the 1,213 T1 rows were compared with Phugpa, Tsurphu and Genden; every Genden
  mismatch was investigated (a blind re-read of 28 crops: 84 of 84 cells agree). Gate: 174 passed, 0
  failed in debug and in release. The engine, the golden rows and every ruling (F25, F28 to F37) are
  unchanged. M0 is met under F37 (c).

THE FINDING
  Rows 1,213 (distinct days 1,165): Genden 1,199 matches / 14 mismatches; Tsurphu 1,184 / 29; Phugpa
  1,145 / 68. Per file (matches of rows), Genden: Ivolginsky 2021 398 of 398, Aginsky 2025 407 of
  413, Aginsky 2026 394 of 402; Tsurphu 391, 403, 390; Phugpa 374, 398, 373. Genden is the best-
  matching system on each file, no tie. D-23 does not fire; D-19 (ii) does not fire. Genden's 14
  mismatches are two runs of consecutive days: 2025-12-22 to 27 (the 2025 edition, 6 days; classes:
  3 variant candidates, 3 unexplained) and 2026-01-09 to 16 (the 2026 edition, 8 days; all 8 are days
  where the two Aginsky editions disagree and the 2025 edition equals Genden: source disagreement);
  transcription 0. Tsurphu and Genden give the same lunation boundaries as the source and offset 0 on
  every regular lunation; both flag the leap on the lunation before the one the source labels
  «Дополнительный» (one instance). The Сагаалган days are the first of month 1 in Tsurphu and Genden
  (5 of 5), in Phugpa 3 of 5. Which branch of 001 §6.5 the counts satisfy: (ii), best system with
  mismatches, each investigated; (i) is satisfied by one file only and not by all rows pooled (001 does
  not say which reading); "a small number of mismatches" and "matches well" are not defined by 001
  §6.5, so no threshold is applied here.
  [CORRECTED 2026-11-13 (session 2, Item 13 (e); V3, V4, V5): "D-23 does not fire" rested on a
  reading of D-23 its words did not state (ruled since: R7); "(ii)" applied the undefined "small
  number", and "001 does not say which reading" left out §6.5's opening words, "over all Buryat
  golden rows", which give the pooled reading. The basis ruled: R5 (b), 001 §6.5's second rule.]

EVIDENCE
  MEASURED: §1, §2, §3 of this file (the committed tests `q1_differential` and `buryat_golden`, debug
  and release outputs equal; an outside recount equal; 5 planted defects red; the pin red on a planted
  cell). INFERRED: none. RECALLED: spec 009 §1.2's 53 days; the Сагаалган days (spec 013, re-read from
  the metas). RELAYED: Sangha membership rests on the publishers' own statements (D-33); the Ivolginsky
  rows stand on the human's acceptance and await the datsan's answer (D-35). What the files can tell
  apart: Tsurphu and Genden differ on 14 of the 1,165 distinct days (the source equals Genden on all
  15 rows); the 2026 edition's 8 mismatches are the only days on which the two editions differ.

THE QUESTION
  Which outcome of 001 §10 M3 does the verdict spec record: a verdict (and for which system, on what
  scope), or the stop-and-consult outcome of 001 §6.5? It cannot be decided inside this round: 001
  §6.5 and METHOD P4 make the call the human's, and 001 §6.5 leaves "small number" undefined.

OPTIONS — a starting set, not a menu
  1. A verdict naming one system (the counts name Genden), with its scope and how its 14 mismatches are
     recorded. Follows 001 §6.5's second bullet (and the first, per source, for one file). Scope: three
     T1 files of two datsans, 1,165 distinct days, 2021-01-01 to 2022-02-02 and 2025-01-01 to
     2027-02-06; membership rests on the publishers' statements (D-33); only 14 of the days separate
     Genden from Tsurphu. Real costs: (a) it lifts D-1 and lets clients default a system on this
     evidence; (b) it would rule out only Tsurphu's A.2, not the karana-Sun variant (D-19), and D-19's
     TRIGGER (i) fires; (c) the two runs must be recorded (the 2026 edition's 8 days as a disagreement
     between editions, the 2025 edition's 6 as unexplained or variant), and a pooled 100% is not
     reached; (d) the extra-month convention must be recorded: the source's «Дополнительный» is the
     second of the pair and the engine's leap flag the first (one instance); (e) if the Ivolginsky
     datsan refuses (D-35), M0 reopens (F37) and only the Aginsky datsan, whose two editions are not
     independent, remains.
  2. The stop-and-consult outcome of 001 §6.5: the data to a зурхайч (F4 is OPEN: approach
     Гунзэчойнэй before or after M3). Follows 001 §6.5's third bullet, whose words ("matches well")
     sit uneasily with Genden's 98.8% of rows. Real costs: contact outside the engine's scope (an
     outreach round and F4 ruled first); D-1 holds, so the website's TRIGGER (a verdict spec) stays
     unmet; the two runs would be what the зурхайч is asked about.
  3. More data before any verdict (D-33's register evidence; D-34's full printed calendars of
     Gunzechoinei or Khoymorsky; further years of the same datsans). Real costs: nothing is
     decided now; at about 1.2% of days (14 of 1,165) separating Genden from Tsurphu, each further
     year adds about 4 to 5 discriminating days (INFERRED from the 14 days over about 3.2 years of
     distinct coverage); the two runs stay unexplained until a source covers them again.
  4. A verdict scoped to the Ivolginsky datsan only (Genden 398 of 398, 13 whole months; 7 days
     separate it from Tsurphu and 24 from Phugpa within the file), with Aginsky's two runs left to
     option 2 or 3. Follows the first bullet per source. Real costs: the narrowest evidence rests on
     rows whose reuse the datsan has not yet permitted (D-35: a refusal withdraws them); one print,
     one compiler, one year.
  The adjudicator may answer with something not listed (for example option 1 with a stated reopening
  condition, as F37 has).

IF THE ANSWER IS DELAYED
  Nothing: the branch stands still. The verdict spec, D-1 and D-24 belong to the resume.
```

## 13. Diff scope, forks, latitude, sums, what the prompt got wrong, by-catch, held

**Diff scope** (MEASURED at `e96187d`, `git diff --numstat main e96187d`): `DEBT.md` +10 −2; `FORKS.md` +18 −4; `crates/tshes/tests/buryat_golden.rs` +428 −0; `crates/tshes/tests/common/buryat.rs` +539 −0; `crates/tshes/tests/common/mod.rs` +2 −0; `crates/tshes/tests/common/q1.rs` +276 −0; `crates/tshes/tests/q1_differential.rs` +1062 −0; `docs/ROADMAP.md` +3 −2; `docs/specs/prompts/014-q1-verdict.md` +488 −0. Sum: 10 + 18 + 428 + 539 + 2 + 276 + 1062 + 3 + 488 = **2,826 insertions**, 2 + 4 + 2 = **8 deletions**, equal to `git diff --shortstat main e96187d` ("9 files changed, 2826 insertions(+), 8 deletions(-)"). This file is the tenth. Gated paths touched: **yes**, `crates/tshes/tests/` (test code only, under accepted specs 001 and 002), this round's own spec file and prompt copy; nothing under `crates/tshes/src/`, `golden/`, `rules/`, `clients/`, `tools/`, `.github/`, `docs/METHOD.md`, `GATED.md`, `CLAUDE.md`, `docs/ROUNDS.md`, `docs/ADVISOR.md`, an accepted spec or another round's INDEX row. Files the test framework wrote: **none** (`git status --porcelain` printed nothing after the gate and after the debug and release runs). `docs/specs/INDEX.md` and the ROADMAP's LANDED are not touched (STOP RULE (9)).

**Forks encountered, resolved ("considered X, chose Y because Z").** (1) Where the planted controls of Item 3(d) live: considered writing scratch copies of the CSV files, chose in-memory copies of the loaded rows because they never touch the committed files and need no file writer (METHOD rule 49). (2) How to count the two Aginsky editions' 48 overlapping days in "the distinct days": considered counting the day once with one edition's value, chose a three-way split (every source value matches / none / editions split) because the editions differ on 8 of them and either single value would decide those 8 silently. (3) How to order the classes of Item 5(e) when a day fits two: fixed before the reading as TRANSCRIPTION, SOURCE DISAGREEMENT, VARIANT CANDIDATE, UNEXPLAINED, the order the prompt lists. (4) The reader's crop shape: a three-column window with the row-label strip instead of a whole month band, so that no crop shows which days are mismatches. (5) The reader's model: set to sonnet through the Agent tool's parameter, and recorded as the reader reports it.

**Latitude taken beyond the prompt.** (1) `common/buryat.rs` and `common/q1.rs` are new shared helpers (the prompt left the shape to me). (2) §2(h), the days on which the systems differ, and the 14-of-1,165 figure used in the options. (3) The lunation boundaries of the Ivolginsky file, compared for each system although the file prints no label (Item 4(c) asks for the Aginsky files). (4) `q1_the_engine_still_computes_the_engine_values_spec_009_recorded_on_its_53_days`, a value-level check of the engine's tshes on spec 009's 53 days. (5) The outside recount script and the dump test. (6) `D-37`'s Status names the tests and the grep it was re-measured with. (7) The ROADMAP's "Status: r5" line became "r6" beside the new Revisions line (hard rule 19: it would contradict the line below). (8) **One slip against the prompt's GRANTS:** during the harness work I ran one inline `python3 -I -` with a heredoc (it printed one word, read nothing and wrote nothing); the prompt says every script is a file written first. No other inline run was made. (9) The planted `.csv` copies and the extracted copies were written by `python3 -I` scripts and `tar` inside `mktemp -d` directories (allowed by GRANTS as scratch).

**Numbers that must sum.** Rows 398 + 413 + 402 = 1,213; distinct days 1,213 − 48 = 1,165 = 398 + 413 + 402 − 48; per system per file matches + mismatches = rows (nine pairs, asserted by the test); mismatches 68 + 29 + 14 = 111 = 31 + 31 + 49; editions 40 + 8 = 48; Genden's classes 0 + 8 + 3 + 3 = 14; crops 14 + 14 = 28, cells 42 + 42 = 84; gate 147 + 27 = 174 in each profile; diff §13 (2,826 and 8); OPEN 5 + 1 = 6.

**What the prompt got wrong.** (1) The `DEBT.md` D-38 instruction says "If the words 'under the gate from spec 014' turn out false at the halt (Item 2 not built)…": they are true; nothing to correct. (2) Item 4(d) lists the 2027-02-07 Сагаалган day among those "printed by the sources", which is right but outside the golden files' dates (the day after the 2026 edition's last row); reported as such. (3) Prediction 6 (the engine's Genden flags the lunation labelled «Дополнительный») is false: it flags the lunation before. (4) Prediction 7's "no system matches 100% of the T1 rows" is true pooled and false for the Ivolginsky file. (5) Item 5(a) says "one crop per mismatching day … the reader writes, per crop, the Gregorian day and the tshes printed": taken literally one cell per crop would leave no context; the crops carry three columns. (6) Item 3(d) calls the planted files "scratch copies of the golden files read by the harness"; the harness reads text, so the copies are in memory. (7) Item 2's "refuses … a boolean other than true / false where one is required, as `golden/buryat/README.md` defines each column": the README does not say which columns require one; `is_leap_month` is empty in the whole Ivolginsky file (it prints no label), so only `is_doubled` is required to be a boolean and `is_leap_month` may be empty. Nothing else found against the prompt's dictated rows; F37 and D-39 were the next free ids as the advisor expected, and F38 for the verdict fork.

**By-catch.** (1) The harness shows that the two Aginsky runs are bracketed by a doubled and a skipped day in one source and not in the other (2026-01-09 to 16: the 2026 edition skips 21 at 2026-01-09 and the 2025 edition does not); this is a description of the data, not an explanation, and it is not filed because it is part of F38's evidence. (2) The refusal tests of `buryat_golden.rs` start from the real files, so a planted defect in a real file also turns the unplanted-baseline assertion of a refusal test red (§4's plants (i), (ii) and (v)): intended, noted. (3) `tools/gate-self-test.sh:39` copies `golden/buryat/` and checks nothing (R3 (a)); the gate now reads the directory through the tests, so no entry is filed. (4) No `DEBT.md` entry beyond D-39 (opened by the prompt).

**Held** (identified items only): the register evidence (D-33); the Gunzechoinei and Khoymorsky rows (D-34); the Ivolginsky rows' standing and M0's conditional reopening (D-35, F37); the clear-line process disagreement (D-36); the missing `golden/reference/` meta reader (D-39); the verdict fork (F38), D-1, D-19 and D-24 for the resume; F4 (outreach).

**The marker greps and `cmp`.** Before every commit: `cmp` of the prompt copy against `/home/kitaeza/zurkhai-prompts/spec-014-q1-verdict.md` printed `CMP-OK`; `git grep` of the round's marker over everything but the frozen prompt copy printed nothing (exit 1); `git status --porcelain` listed only the files the commit meant to change.

## Appendix A. The delegated reader's brief, and its model

Its model, as the reader reports it: `claude-sonnet-5-5`; its launch: the Agent tool, `subagent_type` general-purpose, model parameter sonnet; the prompt it was given was one sentence naming the brief file. The brief (`reader/brief.md`, sha256 `a2b1916840f09c9a04700c8be4051fabe6f75a8aa60c49b0caa888596d426580`), verbatim:

```
BRIEF FOR A DELEGATED READER — spec 014, Item 5(a): a blind re-read of 28 small crops of printed calendars

You are a blind reader. Your job is to read printed numbers off 28 small images and write them down. You do not
interpret, correct, infer or "fix" anything: you write what is printed. If something is hard to read you write your
best reading and mark it UNSURE (see the format). You never fill a value from the pattern of the neighbouring cells
or from what you expect a calendar to contain: your reading must be independent of any expectation. You are told
nothing else about the images, and you do not try to find out.

GRANTS (verbatim from the round's prompt; it binds you too):
GRANTS:       Reads outside WORKDIR, beyond this prompt file: /home/kitaeza/zurkhai-papers/, read only,
              and only the four files of ARTIFACT, with the Read tool (images included), ls, stat,
              sha256sum, grep, pdfinfo, pdftotext and pdftoppm. pdftotext and pdftoppm write only into
              a `mktemp -d` directory.
              Network: none. No WebSearch, no WebFetch, no curl, wget, git fetch, gh, cargo commands
              that reach the network (use `--offline`), or python networking.
              The Agent tool: at most one delegated reader, for Item 5(a) only. Its brief carries this
              GRANTS line verbatim (CLAUDE.md hard rule 11; METHOD rules 66, 67), names the exact crops
              it reads, forbids it every other file of the library and of the repository, and names the
              model it runs on; the report records that model as the delegate reports it. Its output
              is a file in your own `mktemp -d` directory.
              The tools, in the repository:
                - the Read tool; the Edit and Write tools, for the files the items name;
                - git, read-only (status, log, show, rev-parse, rev-list, grep, diff), and what the
                  round's own branch and commits need (switch -c, add, commit);
                - cargo, offline, on the repository (build, test, fmt, clippy) and tools/gate.sh, as
                  its own header says (Item 7);
                - `python3 -I`, running only scripts you write into a `mktemp -d` directory, reading
                  only the repository, the four ARTIFACT files and your scratch;
                - read-only shell tools on the repository and on your own scratch: cat, head, tail,
                  sed -n, grep, awk, cut, tr, sort, uniq, wc, diff, cmp, ls, stat, find, date, file,
                  sha256sum;
                - mkdir -p, cp and rm -r, only inside `mktemp -d` directories; a copy of the
                  repository for a planted control is made there (`git archive` of a commit), never
                  by editing the working tree.
              On this machine `grep` is ugrep: keep bounded repetitions below 100.

YOUR MODEL. Run as the model you are; at the end of your report state your own model ID as you know it.

WHAT YOU MAY READ — exactly these 28 image files, with the Read tool, and nothing else (plus this brief):
  <scratch>/reader/crops/c01.png
  ... and the same directory's c02.png, c03.png, c04.png, c05.png, c06.png, c07.png, c08.png, c09.png, c10.png,
  c11.png, c12.png, c13.png, c14.png, c15.png, c16.png, c17.png, c18.png, c19.png, c20.png, c21.png, c22.png,
  c23.png, c24.png, c25.png, c26.png, c27.png, c28.png.
You must NOT read any other file of the library (/home/kitaeza/zurkhai-papers/), any file of the repository
(/home/kitaeza/github/zurkhai/), any other file in the scratch directory above or its sub-directories (in
particular no file or directory whose name contains "plan", "map", "private", "page", "work", "script", "run",
"out-", "notes" or "pin"), and you must not list any directory or run any command that reads them. If you find
yourself needing one, stop and say so in your report instead.

WHAT YOU MAY WRITE — exactly one file, with the Write tool (not a shell redirection):
  <scratch>/reader/out/reading.txt
Nothing else may be written.

WHAT THE IMAGES ARE. Each image is a small crop of one part of a printed Buddhist lunar wall calendar in Russian.
At the left is a strip with three row labels, from top to bottom: «Григорианский» (the day of the month, 1 to 31, the
"Gregorian day"), «Лунный» (the lunar day number, 1 to 30, called tshes) and «Дни недели» (the weekday, one of
Пн Вт Ср Чт Пт Сб Вс). To the right of the strip are exactly three day columns, side by side; each column has a
Gregorian day number on the top row, a lunar number on the middle row and a weekday on the bottom row. The top of
an image may show the cut-off edge of a coloured bar or of a month name; ignore it. Numbers may be red or blue or
black; read the digits.

OUTPUT FORMAT. Fields separated by a single TAB, one line per day column, no header line, no comments, no blank
lines. Write the crops in numeric order (c01 first), and the three columns of a crop left to right:
  <crop name without .png> <TAB> <column 1, 2 or 3> <TAB> <Gregorian day> <TAB> <lunar number> <TAB> <weekday as printed> <TAB> <note>
The note is empty if you are sure of all three values. If you are not sure of a value, write your best reading and
put UNSURE and what you doubt in the note (for example "UNSURE lunar 13 or 18"). Do not leave a column out. Do not
add columns that are not there. The file has 84 lines (28 crops, 3 columns each).

When the file is written, reply with: the number of lines in the file per crop; every cell you marked UNSURE; the
list of every file you read (it must be only the 28 images and this brief) and every command you ran; any read or
write outside this brief (there must be none); and your model ID.
```

(In the file the brief carried the absolute scratch paths; `<scratch>` stands for the session's `mktemp -d` directory. The reader's report: 84 lines, 3 per crop; no UNSURE; read only the brief and the 28 images; ran no command; no read or write outside the brief; model `claude-sonnet-5-5`.)

## Appendix B. The commands and the scripts

**Commands** (absolute paths; `SC` is the session's `mktemp -d` scratch directory): the preconditions and the greps of §5(b) as quoted there; `cargo test --offline --manifest-path <repo>/Cargo.toml -p tshes --test buryat_golden` and `… --test q1_differential -- --nocapture --test-threads=1` (the reports; `--release` for the second profile; stdout filtered with `grep -v "^test result"` and compared with `diff`); `cargo clippy --offline … --workspace --all-targets -- -D warnings`; `cargo fmt … --all --check`; for each plant, `git -C <repo> archive 24821a1e56502e47c49ce87e8bde820322a73879 | tar -x -C $SC/copy`, `python3 -I $SC/scripts/plant.py $SC/copy N`, `CARGO_TARGET_DIR=$SC/target cargo test --offline --manifest-path $SC/copy/Cargo.toml -p tshes --test buryat_golden`; `python3 -I $SC/scripts/lists.py <repo>/docs/specs/009-henning-differential.md`; `python3 -I $SC/scripts/mkpin.py <captured report output>`; `python3 -I $SC/scripts/recount.py <repo> <captured dump>`; `python3 -I $SC/scripts/plan.py <repo> <dump> $SC/plan.json`; `python3 -I $SC/scripts/crops.py $SC/plan.json $SC/crops-work 200` (calls `pdftotext -bbox` and `pdftoppm -r 200` on the two Aginsky PDFs); `python3 -I $SC/scripts/compare_reading.py <repo> <crop map> <reading.txt>`; `python3 -I $SC/scripts/plant_reading.py`; `python3 -I $SC/scripts/discr.py <repo> <dump>`; `python3 -I $SC/scripts/gen_mism.py <captured output>`; `pdftotext -layout /home/kitaeza/zurkhai-papers/janson-1401.6285v1.pdf $SC/janson/janson.txt` and `grep -n -i` over it; `bash <repo>/tools/gate.sh` once at the end. The scripts, verbatim (each is a file written into `$SC/scripts/` before it was run; the Rust tests are in the tree).

`plant.py` (Item 2(c)):

```python
#!/usr/bin/env python3
"""Spec 014 Item 2(c): plant one defect in a COPY of golden/buryat/ (never the repository).

usage: python3 -I plant.py <copy root> <plant number 1..5>
The copy root is a `git archive` extraction inside a mktemp -d directory.
"""
import os
import sys

root, plant = sys.argv[1], int(sys.argv[2])
gb = os.path.join(root, "golden", "buryat")


def read_lines(name):
    with open(os.path.join(gb, name), encoding="utf-8", newline="") as f:
        return f.read().split("\n")  # keeps the final empty item


def write_lines(name, lines):
    with open(os.path.join(gb, name), "w", encoding="utf-8", newline="") as f:
        f.write("\n".join(lines))


if plant == 1:
    # (i) a +3 step: the first row that follows a skipped number gets tshes + 1.
    name = "aginsky-datsan-2025.csv"
    lines = read_lines(name)
    for i in range(2, len(lines) - 1):
        f = lines[i].split(",")
        if f[5] != "" and int(f[1]) < 30:
            print("plant 1: %s line %d (%s): tshes %s -> %d" % (name, i + 1, f[0], f[1], int(f[1]) + 1))
            f[1] = str(int(f[1]) + 1)
            lines[i] = ",".join(f)
            break
    write_lines(name, lines)
elif plant == 2:
    # (ii) an eighth field on one data row.
    name = "ivolginsky-datsan-2021.csv"
    lines = read_lines(name)
    print("plant 2: %s line 100 (%s): an eighth field" % (name, lines[99].split(",")[0]))
    lines[99] = lines[99] + ",extra"
    write_lines(name, lines)
elif plant == 3:
    # (iii) a deleted meta file.
    name = "aginsky-datsan-2026.meta.yaml"
    os.remove(os.path.join(gb, name))
    print("plant 3: deleted %s" % name)
elif plant == 4:
    # (iv) a CSV the tests do not name (a valid copy of one file under a new name).
    src = read_lines("aginsky-datsan-2026.csv")
    write_lines("extra-datsan-2099.csv", src)
    print("plant 4: added golden/buryat/extra-datsan-2099.csv (a copy of aginsky-datsan-2026.csv)")
elif plant == 5:
    # (v) a label moved to the neighbouring lunation: the 4th lunation of the 2025 edition
    # takes the 3rd lunation's month and its printed label.
    name = "aginsky-datsan-2025.csv"
    lines = read_lines(name)
    rows = [l.split(",") for l in lines[1:-1]]
    starts = [0] + [i for i in range(1, len(rows)) if int(rows[i][1]) < int(rows[i - 1][1])]
    ends = starts[1:] + [len(rows)]

    def label(note):
        a = note.index("printed label «")
        b = note.index("»", a)
        return note[a:b + 1]

    k = 3
    month_prev = rows[starts[k - 1]][2]
    label_prev = label(rows[starts[k - 1]][6])
    label_own = label(rows[starts[k]][6])
    for i in range(starts[k], ends[k]):
        rows[i][2] = month_prev
    rows[starts[k]][6] = rows[starts[k]][6].replace(label_own, label_prev)
    print("plant 5: lunation %d from %s: month %s and %s copied from lunation %d" % (k + 1, rows[starts[k]][0], month_prev, label_prev, k))
    write_lines(name, [lines[0]] + [",".join(r) for r in rows] + [lines[-1]])
else:
    raise SystemExit("plant must be 1..5")
```

`lists.py` (Item 4(e); the three tables of spec 009 §1.2 as Rust rows):

```python
#!/usr/bin/env python3
"""Spec 014 Item 4(e): read spec 009 section 1.2's three tables of disagreeing days
(Phugpa 17, Tsurphu 13, Genden 23) and print them as Rust array rows:
(System, "date", engine tshes, TCG tshes).  Reads only docs/specs/009-henning-differential.md.

usage: python3 -I lists.py <path of 009-henning-differential.md>
"""
import re
import sys

text = open(sys.argv[1], encoding="utf-8").read().split("\n")
system = None
out = []
for line in text[39:131]:  # section 1.2's tables only (lines 40-131 of the file)
    m = re.match(r"\*\*(Phugpa|Tsurphu|Genden) \((\d+)\):\*\*", line)
    if m:
        system = m.group(1)
        continue
    m = re.match(r"\| (\d{4}-\d\d-\d\d) \| (\S+) \| (\S+) \|", line)
    if m and system:
        date, eng, tcg = m.groups()
        e = eng.split("/")
        t = tcg.split("/")
        out.append((system, date, int(e[2]), int(t[2]), e[1], t[1]))
counts = {}
for r in out:
    counts[r[0]] = counts.get(r[0], 0) + 1
print("// counts:", counts, "total", len(out), file=sys.stderr)
for system, date, e, t, em, tm in out:
    print('    (System::%s, "%s", %d, %d),' % (system, date, e, t))
```

`mkpin.py` (Item 3(e)):

```python
#!/usr/bin/env python3
"""Spec 014 Item 3(e): turn the `Q1 TABLE` and `Q1 MISMATCH` lines of one --nocapture run of
q1_report_the_counts_per_file_and_system_the_mismatches_and_the_totals into the rows of the pin.

usage: python3 -I mkpin.py <captured output file>
Prints Rust array rows: (file stem, system name, rows, matches, "mismatching dates").
"""
import re
import sys

table = {}
mism = {}
order = []
for line in open(sys.argv[1], encoding="utf-8"):
    line = re.sub(r"^test \S+ \.\.\. ", "", line.rstrip("\n"))
    m = re.match(r"Q1 TABLE file=(\S+) tier=\S+ system=(\S+) rows=(\d+) match=(\d+) mismatch=(\d+)", line)
    if m:
        key = (m.group(1), m.group(2))
        table[key] = (int(m.group(3)), int(m.group(4)), int(m.group(5)))
        order.append(key)
        mism[key] = []
        continue
    m = re.match(r"Q1 MISMATCH file=(\S+) tier=\S+ system=(\S+) date=(\S+) ", line)
    if m:
        mism[(m.group(1), m.group(2))].append(m.group(3))
for key in order:
    rows, match, miss = table[key]
    assert match + miss == rows and len(mism[key]) == miss, key
    dates = " ".join(mism[key])
    print('    ("%s", "%s", %d, %d, "%s"),' % (key[0], key[1], rows, match, dates))
```

`recount.py` (the outside recount):

```python
#!/usr/bin/env python3
"""Spec 014: recount, from the three golden CSV files and the harness's `Q1DUMP` lines alone,
the tshes matches and mismatching dates per file and system, and the totals. An outside check of
the harness's arithmetic (it shares the engine values with the harness).

usage: python3 -I recount.py <repo root> <captured --nocapture output of q1_dump_...>
Reads only <repo root>/golden/buryat/*.csv and the captured file.
"""
import re
import sys

root, dump = sys.argv[1], sys.argv[2]
files = ["ivolginsky-datsan-2021", "aginsky-datsan-2025", "aginsky-datsan-2026"]
engine = {}
for line in open(dump, encoding="utf-8"):
    line = re.sub(r"^test \S+ \.\.\. ", "", line.strip())
    m = re.match(r"Q1DUMP (\S+) (\S+) (\d+) ", line)
    if m:
        engine[(m.group(2), m.group(1))] = int(m.group(3))
rows = {}
for f in files:
    lines = open("%s/golden/buryat/%s.csv" % (root, f), encoding="utf-8").read().split("\n")
    assert lines[-1] == ""
    rows[f] = [(r.split(",")[0], int(r.split(",")[1])) for r in lines[1:-1]]
tot = {}
for f in files:
    for s in ["Phugpa", "Tsurphu", "Genden"]:
        miss = [d for d, t in rows[f] if engine[(s, d)] != t]
        print("RECOUNT %s %s rows=%d match=%d mismatch=%d dates=%s" % (f, s, len(rows[f]), len(rows[f]) - len(miss), len(miss), " ".join(miss)))
        a = tot.setdefault(s, [0, 0])
        a[0] += len(rows[f])
        a[1] += len(miss)
days = sorted({d for f in files for d, _ in rows[f]})
for s in ["Phugpa", "Tsurphu", "Genden"]:
    allm = nom = split = 0
    for d in days:
        vals = [t for f in files for dd, t in rows[f] if dd == d]
        hits = sum(1 for v in vals if v == engine[(s, d)])
        if hits == len(vals):
            allm += 1
        elif hits == 0:
            nom += 1
        else:
            split += 1
    print("RECOUNT-TOTAL %s rows=%d match=%d mismatch=%d | distinct=%d all=%d none=%d split=%d" % (s, tot[s][0], tot[s][0] - tot[s][1], tot[s][1], len(days), allm, nom, split))
```

`plan.py` (Item 5(a); it holds the decoy rule, fixed before the reading):

```python
#!/usr/bin/env python3
"""Spec 014 Item 5(a): the plan of the blind re-read, FIXED BEFORE ANY READING.

The crops: one per mismatching day of each file's best-matching system (Genden, Item 4(f)), up to
40 per file, and as many decoys. THE DECOY RULE (recorded before the reading): for the j-th
mismatching day m_j of a file (date order), the decoy is the first date d >= m_j + 14 days in the
same file such that (i) the file's golden tshes equals the tshes of ALL THREE systems on d-1, d and
d+1, (ii) d lies in a month band of the edition that is not its last printed month (a partial
month), (iii) d is more than 2 days from every decoy already chosen for the file.
The crop order is fixed by sha256("spec014-" + file + date): ascending hexdigest.

usage: python3 -I plan.py <repo root> <captured Q1DUMP output> <plan json out>
Reads only <repo root>/golden/buryat/*.csv and the captured file.
"""
import datetime
import hashlib
import json
import re
import sys

root, dump, out = sys.argv[1], sys.argv[2], sys.argv[3]
engine = {}
for line in open(dump, encoding="utf-8"):
    line = re.sub(r"^test \S+ \.\.\. ", "", line.strip())
    m = re.match(r"Q1DUMP (\S+) (\S+) (\d+) ", line)
    if m:
        engine[(m.group(2), m.group(1))] = int(m.group(3))

# file -> (pdf label, best system's mismatching dates computed here from the files, not copied)
FILES = ["aginsky-datsan-2025", "aginsky-datsan-2026"]
BEST = "Genden"
plan = []
for f in FILES:
    rows = [r.split(",") for r in open("%s/golden/buryat/%s.csv" % (root, f), encoding="utf-8").read().split("\n")[1:-1]]
    golden = {r[0]: int(r[1]) for r in rows}
    dates = sorted(golden)
    last_band = dates[-1][:7]
    mism = [d for d in dates if engine[(BEST, d)] != golden[d]]
    assert len(mism) <= 40, (f, len(mism))
    chosen = []

    def day(s, n):
        return (datetime.date.fromisoformat(s) + datetime.timedelta(days=n)).isoformat()

    for m in mism:
        d = day(m, 14)
        while True:
            ok = d in golden and d[:7] != last_band
            ok = ok and all(day(d, k) in golden and all(engine[(s, day(d, k))] == golden[day(d, k)] for s in ("Phugpa", "Tsurphu", "Genden")) for k in (-1, 0, 1))
            ok = ok and all(abs((datetime.date.fromisoformat(d) - datetime.date.fromisoformat(c)).days) > 2 for c in chosen)
            if ok:
                break
            d = day(d, 1)
            assert d in golden, "no decoy found for " + m
        chosen.append(d)
    for kind, ds in (("mismatch", mism), ("decoy", chosen)):
        for d in ds:
            plan.append({"file": f, "date": d, "kind": kind, "key": hashlib.sha256(("spec014-" + f + d).encode()).hexdigest()})
plan.sort(key=lambda p: p["key"])
for i, p in enumerate(plan, 1):
    p["crop"] = "c%02d" % i
json.dump(plan, open(out, "w", encoding="utf-8"), indent=1)
for p in sorted(plan, key=lambda p: (p["file"], p["date"])):
    print(p["crop"], p["file"], p["date"], p["kind"])
print("crops:", len(plan), "mismatch:", sum(p["kind"] == "mismatch" for p in plan), "decoy:", sum(p["kind"] == "decoy" for p in plan))
```

`crops.py` (Item 5(a); the first render used `0.5 * row_h` for the top edge and cut into the label bar; it was changed to `0.3 * row_h` and re-run before the reader saw any crop):

```python
#!/usr/bin/env python3
"""Spec 014 Item 5(a): render the crops of the blind re-read from the Aginsky PDFs.

usage: python3 -I crops.py <plan json> <outdir> <dpi>
Reads (GRANTS) only the two Aginsky PDFs of the paper library; pdftotext -bbox is used ONLY to place a
crop (the column positions of the Gregorian row), never for any value; the pixels are what is read.
Each crop: the row-label strip at the left, then three day columns (the day before, the day, the day
after), the three rows: Gregorian day, lunar number, weekday. The label bar is not in the crop.
"""
import json
import os
import re
import subprocess
import sys

from PIL import Image

Image.MAX_IMAGE_PIXELS = None
plan_file, outdir, dpi = sys.argv[1], sys.argv[2], int(sys.argv[3])
PDF = {
    "aginsky-datsan-2025": "/home/kitaeza/zurkhai-papers/aginsky-datsan/aga/kalendar2025.pdf",
    "aginsky-datsan-2026": "/home/kitaeza/zurkhai-papers/aginsky-datsan/aga2026/kalendar-a2-format.pdf",
}
MONTHS = ["ЯНВАРЬ", "ФЕВРАЛЬ", "МАРТ", "АПРЕЛЬ", "МАЙ", "ИЮНЬ", "ИЮЛЬ", "АВГУСТ", "СЕНТЯБРЬ", "ОКТЯБРЬ", "НОЯБРЬ", "ДЕКАБРЬ"]
START_YEAR = {"aginsky-datsan-2025": 2025, "aginsky-datsan-2026": 2026}
os.makedirs(outdir, exist_ok=True)
plan = json.load(open(plan_file, encoding="utf-8"))
scale = dpi / 72.0
pages = {}
words = {}
for f, pdf in PDF.items():
    bb = subprocess.run(["pdftotext", "-bbox", pdf, "-"], capture_output=True, text=True, check=True).stdout
    words[f] = [(float(a), float(b), float(c), float(d), t) for a, b, c, d, t in re.findall(r'<word xMin="([\d.]+)" yMin="([\d.]+)" xMax="([\d.]+)" yMax="([\d.]+)">([^<]*)</word>', bb)]
    prefix = os.path.join(outdir, "page-" + f)
    subprocess.run(["pdftoppm", "-r", str(dpi), "-png", pdf, prefix], check=True)
    png = [x for x in os.listdir(outdir) if x.startswith("page-" + f) and x.endswith(".png")][0]
    pages[f] = Image.open(os.path.join(outdir, png)).convert("RGB")

mapping = {}
for p in plan:
    f, d = p["file"], p["date"]
    y, mo, dd = int(d[:4]), int(d[5:7]), int(d[8:])
    index = (y - START_YEAR[f]) * 12 + (mo - 1)
    ws = words[f]
    page_w = pages[f].width / scale
    heads = sorted([w for w in ws if w[4] in MONTHS and w[0] > 0.2 * page_w], key=lambda w: w[1])
    h = heads[index]
    assert h[4] == MONTHS[mo - 1], (d, h[4])
    band_top = h[1] - 12
    band_bot = heads[index + 1][1] - 12 if index + 1 < len(heads) else band_top + 175
    nums = [w for w in ws if re.fullmatch(r"\d{1,2}", w[4]) and band_top <= w[1] < band_bot and w[0] > h[0] - 5]
    rows = {}
    for w in nums:
        rows.setdefault(round(w[1] / 3), []).append(w)
    greg = None
    for k in sorted(rows):
        r = sorted(rows[k], key=lambda w: w[0])
        vals = [int(w[4]) for w in r]
        if vals == list(range(1, len(vals) + 1)) and len(vals) >= 5:
            greg = r
            break
    assert greg is not None, d
    ys = sorted(rows)
    lunar = [rows[k] for k in ys if k > round(greg[0][1] / 3)][0]
    row_h = lunar[0][1] - greg[0][1]
    col = {int(w[4]): w for w in greg}
    lo, hi = max(dd - 1, 1), min(dd + 1, max(col))
    x0, x1 = col[lo][0] - 6, col[hi][2] + 6
    y0, y1 = greg[0][1] - 0.3 * row_h, greg[0][1] + 2.7 * row_h
    label_x0 = h[0] - 8
    label_x1 = col[1][0] - 3
    im = pages[f]
    strip = im.crop((int(label_x0 * scale), int(y0 * scale), int(label_x1 * scale), int(y1 * scale)))
    cells = im.crop((int(x0 * scale), int(y0 * scale), int(x1 * scale), int(y1 * scale)))
    canvas = Image.new("RGB", (strip.width + cells.width + 8, strip.height), (255, 255, 255))
    canvas.paste(strip, (0, 0))
    canvas.paste(cells, (strip.width + 8, 0))
    name = p["crop"] + ".png"
    canvas.save(os.path.join(outdir, name))
    mapping[p["crop"]] = {"file": f, "date": d, "kind": p["kind"], "columns": [lo, hi], "size": canvas.size}
    print(name, f, d, p["kind"], "columns", lo, hi, canvas.size)
json.dump(mapping, open(os.path.join(os.path.dirname(os.path.abspath(plan_file)), "crop-map.json"), "w", encoding="utf-8"), indent=1)
```

`compare_reading.py` (Item 5(a)) and `plant_reading.py` (its control):

```python
#!/usr/bin/env python3
"""Spec 014 Item 5(a): compare the blind reader's readings with the golden rows.

usage: python3 -I compare_reading.py <repo root> <crop-map.json> <reading.txt>
Reads only <repo root>/golden/buryat/*.csv and the two scratch files.
For each line (crop, column, Gregorian day, lunar number, weekday, note) the date is the crop's month
with that day of the month; the golden tshes of the crop's file on that date and the weekday of the
date are compared. Prints every disagreement and the totals per kind (mismatch crop / decoy crop).
"""
import datetime
import json
import sys

root, mapfile, reading = sys.argv[1], sys.argv[2], sys.argv[3]
cmap = json.load(open(mapfile, encoding="utf-8"))
golden = {}
for f in ("aginsky-datsan-2025", "aginsky-datsan-2026"):
    for r in open("%s/golden/buryat/%s.csv" % (root, f), encoding="utf-8").read().split("\n")[1:-1]:
        c = r.split(",")
        golden[(f, c[0])] = int(c[1])
WD = ["Пн", "Вт", "Ср", "Чт", "Пт", "Сб", "Вс"]
lines = [l.split("\t") for l in open(reading, encoding="utf-8").read().split("\n") if l.strip()]
print("lines:", len(lines))
per_crop = {}
bad = 0
cells = {"mismatch": 0, "decoy": 0}
for ln in lines:
    crop, col, greg, lunar, wd = ln[0], int(ln[1]), int(ln[2]), int(ln[3]), ln[4].strip()
    note = ln[5] if len(ln) > 5 else ""
    m = cmap[crop]
    per_crop[crop] = per_crop.get(crop, 0) + 1
    expect_day = m["columns"][0] + col - 1
    date = m["date"][:8] + "%02d" % greg
    cells[m["kind"]] += 1
    problems = []
    if greg != expect_day:
        problems.append("Gregorian day read %d, column %d should be day %d" % (greg, col, expect_day))
    g = golden.get((m["file"], date))
    if g is None:
        problems.append("no golden row for %s in %s" % (date, m["file"]))
    elif g != lunar:
        problems.append("tshes read %d, golden %d" % (lunar, g))
    wd_true = WD[datetime.date.fromisoformat(date).weekday()]
    if wd != wd_true:
        problems.append("weekday read %s, calendar %s" % (wd, wd_true))
    if note:
        problems.append("note: " + note)
    if problems:
        bad += 1
        print("DISAGREE", crop, m["kind"], m["file"], date, "; ".join(problems))
print("crops:", len(per_crop), "lines per crop:", sorted(set(per_crop.values())))
print("cells read: mismatch-crop cells", cells["mismatch"], "decoy-crop cells", cells["decoy"])
print("cells with a disagreement or a note:", bad)
```

```python
#!/usr/bin/env python3
"""Spec 014 Item 5(a) control: copy the reader's file with ONE tshes changed (line 40), into scratch.
usage: python3 -I plant_reading.py <reading.txt> <planted out>"""
import sys

lines = open(sys.argv[1], encoding="utf-8").read().split("\n")
f = lines[39].split("\t")
print("planted line 40:", f[0], f[1], "tshes", f[3], "->", int(f[3]) % 30 + 1)
f[3] = str(int(f[3]) % 30 + 1)
lines[39] = "\t".join(f)
open(sys.argv[2], "w", encoding="utf-8", newline="").write("\n".join(lines))
```

`discr.py` (§2(h)) and `gen_mism.py` (§2(b)):

```python
#!/usr/bin/env python3
"""Spec 014 Item 6: how many of the golden days can tell the systems apart (MEASURED from the files
and the harness's Q1DUMP lines). usage: python3 -I discr.py <repo root> <captured Q1DUMP output>"""
import re
import sys

root, dump = sys.argv[1], sys.argv[2]
eng = {}
for line in open(dump, encoding="utf-8"):
    line = re.sub(r"^test \S+ \.\.\. ", "", line.strip())
    m = re.match(r"Q1DUMP (\S+) (\S+) (\d+) ", line)
    if m:
        eng[(m.group(2), m.group(1))] = int(m.group(3))
files = ["ivolginsky-datsan-2021", "aginsky-datsan-2025", "aginsky-datsan-2026"]
src = {}
for f in files:
    for r in open("%s/golden/buryat/%s.csv" % (root, f), encoding="utf-8").read().split("\n")[1:-1]:
        c = r.split(",")
        src.setdefault(c[0], {})[f] = int(c[1])
days = sorted(src)
S = ["Phugpa", "Tsurphu", "Genden"]
print("distinct days", len(days))
for a, b in (("Phugpa", "Tsurphu"), ("Phugpa", "Genden"), ("Tsurphu", "Genden")):
    diff = [d for d in days if eng[(a, d)] != eng[(b, d)]]
    print("days where %s and %s differ: %d" % (a, b, len(diff)))
    for f in files:
        rows = [d for d in diff if f in src[d]]
        ha = sum(1 for d in rows if src[d][f] == eng[(a, d)])
        hb = sum(1 for d in rows if src[d][f] == eng[(b, d)])
        print("   %s: %d of those days are in the file; source = %s on %d, = %s on %d, neither on %d" % (f, len(rows), a, ha, b, hb, len(rows) - ha - hb + sum(1 for d in rows if src[d][f] == eng[(a, d)] == eng[(b, d)])))
same_all = sum(1 for d in days if eng[("Phugpa", d)] == eng[("Tsurphu", d)] == eng[("Genden", d)])
print("days on which all three systems give the same tshes:", same_all, "of", len(days))
print("days on which at least two differ:", len(days) - same_all)
```

```python
#!/usr/bin/env python3
"""Spec 014 report: the Q1 MISMATCH lines of one --nocapture run as compact markdown lines.
usage: python3 -I gen_mism.py <captured output of q1_differential>"""
import re
import sys

groups = {}
order = []
for line in open(sys.argv[1], encoding="utf-8"):
    line = re.sub(r"^test \S+ \.\.\. ", "", line.rstrip("\n"))
    m = re.match(r"Q1 MISMATCH file=(\S+) tier=(\S+) system=(\S+) date=(\S+) golden=(\d+) engine=(\d+) lunation=(\d+) ", line)
    if m:
        key = (m.group(1), m.group(2), m.group(3))
        if key not in groups:
            groups[key] = []
            order.append(key)
        groups[key].append("%s %s→%s L%s" % (m.group(4), m.group(5), m.group(6), m.group(7)))
for key in order:
    print("- `%s` (tier %s), %s, %d mismatches: %s" % (key[0], key[1], key[2], len(groups[key]), "; ".join(groups[key])))
```

One printed line of `discr.py`'s per-file rows says "neither on 0" in every case, by a formula that also adds back the days where the two systems agree; the figures quoted in §2(h) come from the other columns. The one inline command that was outside GRANTS (Latitude 8) was `python3 -I -` with `print("ok")`.

## Session 2 — the resume (ADDENDUM 1; Items 8 to 14)

The date is 2026-11-13 throughout session 2: `date` printed `пт 13 нояб 2026 11:35:28 MSK` at the start, `11:57:02` and `12:31:04` around the gate; no day change. "Session 2's marker" is a token written nowhere in the tree before this session, never spelled here. Evidence tags as session 1's (`docs/ROUNDS.md` §4.3). Session 2 ran on Opus 5.5 (claude-opus-5-5), effort High, as the ADDENDUM's MODEL line names; no delegate (GRANTS: the Agent tool, none). The verdict is the human's ruling R5; the spec text above records it, and this section reports how.

### S2.1 The decisive item: the verdict spec, and the three measurements that could have stopped it

The spec text above the end marker is now the verdict spec (Status: accepted; Resolves: F38), written as R5 rules it: Genden, as the engine computes it, within the scope of the three T1 files of two datsans, under 001 §6.5's second rule; its 14 mismatches listed with both values; four reopening conditions; the weakness it keeps. None of the three stop rules that guard it fired:

| Item | Stop rule | What would have fired it | MEASURED |
|---|---|---|---|
| 10, the assertions behind the verdict's figures | (7) | an assertion written to a figure the report states that fails on the tree | every assertion passes (21 of 21 in `q1_differential.rs`, debug and release outputs equal over 3,849 lines); each of the seven groups red on its own plant |
| 11, D-24's re-run of spec 009's differential | (6) | a day outside D-23's 53, or an engine dump that differs from spec 009's | 17 / 13 / 23 days, equal to D-23's lists date by date and cell by cell, none outside; the three dumps' sha256 equal to spec 009 §8's |
| 12, the day-start offset probe | (8) | a non-zero offset with fewer Genden mismatches than offset 0 | none: 14 at 0 is the minimum over [−0.15, +0.15] d, evaluated exactly on all 681 breakpoints; it holds only for δ in [−1.17, +2.55) minutes |

### S2.2 Item 8 — the resume (MEASURED 2026-11-13)

- **PRECONDITION**, every output quoted. (a) `git status --porcelain` printed nothing; `rev-parse --abbrev-ref HEAD` printed `spec/014-q1-verdict`. (b) `rev-parse HEAD` printed `5cec12a33f0a6145cfca3ccc0ea6e4206ea16899`; `rev-parse main` printed `756681ea6a39039470e140052f0f6eac0fa3090a`. (c) `git diff --stat d174965 HEAD -- crates/tshes/src` printed nothing. (d) `ls /home/kitaeza/zurkhai-papers/` listed 13 entries. (e) `git show HEAD:docs/specs/prompts/014-q1-verdict.md | cmp - <prompt file>` printed `cmp: EOF on ‘-’ after byte 40771, line 488`; `head -n 488` of the prompt file is 40,771 bytes, the committed copy is 40,771 bytes and ends with the original end banner: only "EOF on -".
- **ARTIFACT.** The golden CSVs `wc -l` to 399, 414 and 403 lines (398, 413, 402 rows). `sha256sum`: `janson-1401.6285v1.pdf` `7cafc7df…0f77`, equal to `docs/sources.md`; `henning-tcg/tcg1309.zip` `8389de8efcd18f7a66a7cef9b6277c222e5193b2acec440f721096a5ad1e83d2`, 93,207 bytes, equal to `docs/sources.md`. `henning-kalacakra/tdata/` holds the 7 files `docs/sources.md` names (`pl_1900`, `pl_1950`, `pl_2000`, `pl_2012`, `pl_2025`, `pl_2049`, `ts_2012`, `.txt`); their sha256, MEASURED here and recorded nowhere before: `a4915140…017a`, `e8f495d7…ccf9`, `988a2116…1969`, `5cd13c09…7e41`, `197f6f36…050e`, `0cdf521e…87b2`, `9692b684…9366`. **`docs/sources.md` has no per-file hash for them** to compare with: its row holds one manifest over all 17 files of `henning-kalacakra/`, and recomputing it would hash 10 files outside this session's grant (GRANTS: `tdata/` only). The hash was not computed (METHOD rule 67: no route around a grant). In its place, before anything was committed, spec 009's build check was run (S2.3): Henning's six `pl_` files equal this session's TCG build on 8,940 of 8,940 lines, as spec 009 §5 recorded, so their content is what spec 009 compared. `ts_2012.txt` is read by no step of §21 and so is not verified (What the prompt got wrong, (1), (2)).
- **The baseline run** (`docs/ROUNDS.md` §3, a resume): `cargo test --offline --workspace` before anything was built: 174 passed, 0 failed.
- **The prompt copy**: ADDENDUM 1 appended with the Edit tool after the original end banner; `cmp` of the whole files silent; commit `e4ac576` (392 lines added, nothing above the banner changed). `cmp` silent before every later commit.
- **Session 2's marker** chosen; the grep of it over the tree outside the frozen prompt copy printed nothing (exit 1) before every commit.
- **OPEN forks** (`grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`): **6** (F4, F5, F7, F8, F10, F38), equal to the advisor's relayed count.
- **Read first** (the ADDENDUM's list), and how much: the prompt file through ADDENDUM 1 in full; `CLAUDE.md` (loaded); `docs/ROUNDS.md` in full; `docs/METHOD.md` lines 1 to 130 and 255 to 708 (so rules 1 to 11, 25 to 36 and 50 to 67 and Part 2 in full, which hold every rule the list names); `GATED.md` in full; `FORKS.md` the header, F1 to F10, F24 to F38 in full, the rest by status lines; `DEBT.md` the header, D-1, D-18 to D-35 in full, the rest by headings; `docs/ROADMAP.md` in full; `docs/specs/001-init-spec.md` lines 40 to 301 (§4 to §12); `002-rust-code-standard.md` in full; `003-project-method.md` §7 and §8; `TEMPLATE.md` and `INDEX.md` in full; `009-henning-differential.md` lines 1 to 220 (head, §1.1, §1.2, §2 to §5), 248 to 295 (§7, §8) and 879 to 2450 (§21 in full); this file in full; `golden/buryat/README.md` in full. Then `git log --format='%h %s' main..HEAD` (six commits, `d35dd7b` to `5cec12a`). Also read: `crates/tshes/tests/common/q1.rs` in full, `common/buryat.rs` lines 1 to 180 and 360 to 414, `q1_differential.rs` in full (Item 10 writes into it).

### S2.3 Item 11 — D-24's re-run of spec 009's differential (MEASURED 2026-11-13)

Spec 009 §21's record, extracted from `docs/specs/009-henning-differential.md` by `extract009.py` (Appendix C) and a second time by `sed -n` over the same line ranges; the two extractions `cmp` silent, 14 of 14. Each extraction's sha256 (the block's lines in the spec file):

| File | Lines | sha256 |
|---|---|---|
| `conio.h` | 942–962 | `328f6863f912cb869cf7658033a347550e17b1b79221bb26715b629ec6c04017` |
| `dos.h` | 968–972 | `76a2b7063a22b688b69330276e2d05172dbcbcd210823d9e292ffcc23bc7ca68` |
| `bios.h` | 978–982 | `d99e62449c3d112ed412071ccf0b59331093ca440ba99a23a68357de3126cea9` |
| `zz_spec009_dump.rs` | 994–1040 | `3c736e86ad50e3fd8e69ad727984152e16811c6dd7ce4466762a4f5ac68225a9` |
| `cmpbuild.py` | 1085–1229 | `cdc0dc392fa064d01b043e490a3563233de7e040ddb84aea868a280386e48366` |
| `readtcg.py` | 1235–1314 | `e09dde1d7b5aaceaca6318c558816d219802381f7a280317271a111b5ae93f6e` |
| `checkrows.py` | 1320–1413 | `b6821be5f7b48f978a97632b8af5e18e0f59ec036d9409e76296ba4d5657069b` |
| `omitcheck.py` | 1419–1463 | `df84522896f10659fe2816effaf5779484736ae54a7307c15316e6857c6c0af1` |
| `readcheck.py` | 1469–1651 | `2d6e19ff953c039c1ba09a68084a48de96ce94d2bbc9e5c1ed1b5b044ece4d8b` |
| `diffrows.py` | 1657–1793 | `1743463e9621ddbb5c6c606934aeb1bb897eeb6f475389c087be24acc876f9b7` |
| `epochs.py` | 1801–1945 | `bf82b62db7f2177515c276279c8e6443a225178cfdcc2fcd8abed119d7a5b8df` (not run: Item 11 does not name it) |
| `probecmp.py` | 1951–1999 | `d16203e1968c8550ec24dfc45a184912e489085a090aa9239efb60196f25caa0` (not run) |
| `mdtable.py` | 2005–2047 | `8f4e1dcfef3a63465b018211e4b08b786f19b08d0048694baeebf51a9bd31857` (not run) |
| `janson_ref.py` | 2053–2449 | `ca2d2ede05e66044ae9d0950a6be67e58d3790a9e70fbbd6f273c9d043a53f1e` (Item 12; equal to the advisor's relayed hash) |

- **The build.** `tcg1309.zip` extracted with `unzip -q` (15 files) into a `mktemp -d` directory; `gcc (Ubuntu 15.2.0-16ubuntu1) 15.2.0`, `-std=gnu89 -O0` and `-O2`, `-Wall -Wextra -I <stand-ins>`, as §21.1: warnings 26 (-O0) and 41 (-O2), as spec 009 §5 recorded; the two binaries' sha256 differ (`f69cff8b…1ea2`, `6ef914cc…4141`; METHOD rule 64).
- **The build check** (`cmpbuild.py real`, spec 009's six years): 1,470 + 1,470 + 1,590 + 1,470 + 1,470 + 1,470 = **8,940 of 8,940 lines equal**, 0 differing, in each build; the six files of the two builds `cmp` equal. Its controls (`cmpbuild.py controls`): 4 of 4 AS EXPECTED, exit 0.
- **The full ranges.** Phugpa and Tsurphu one process each, Genden one process per year 1899 to 2100 (a Phugpa print first), every process exit 25 (§5's value); 202 files per version per build; the 606 files of `-O0` and `-O2` byte-identical, and the standard output of the Phugpa and Tsurphu runs equal.
- **The readers and checks**, per version, as §7 recorded: `readtcg.py` 73,796 day lines, 73,414 kept; `checkrows.py` ALL PASS (steps 0 / 1 / 2: Phugpa 1,632 / 68,981 / 2,800, Tsurphu 1,631 / 68,983 / 2,799, Genden 1,634 / 68,977 / 2,802; 2,488 months, 2,486 complete, 1,168 of 29 days and 1,318 of 30; leap 75 / 74, 75 / 75, 75 / 75); `omitcheck.py` 0 months differ (2,800, 2,799, 2,802 omitted numbers); `readcheck.py` 0 mismatches (Phugpa Tables 7, 8, 9 and 1: 21, 23, 31, 120 rows; Tsurphu and Genden 21, 21, 31), and its control AS EXPECTED for each version (exit 0).
- **The engine dump** (§21.3's probe added to a `git archive` copy of `f8b2af8` only; `cargo +1.99.0 test --offline`, `CARGO_TARGET_DIR` in the scratch directory): debug and release `cmp` equal for all three systems; the sha256 of the dumps, Phugpa `8bfd1407…44ce4`, Tsurphu `b1457047…bfa23`, Genden `f602429f…551cf`, **equal to spec 009 §8's**; the Phugpa dump `cmp` equal to `golden/reference/tdc-97d4a00-phugpa-1900-2100.csv`; `checkrows.py` ALL PASS on each. The tree object of `crates/tshes/src` at `f8b2af8` is `6423704…`, the same as at `d174965`.
- **The differential** (`diffrows.py diff`, the engine against TCG, 73,414 days per version): **Phugpa 17** days (the month field 1, tshes 17), **Tsurphu 13**, **Genden 23**, every one a run of length 1 beside a doubled or skipped tshes, 0 years whose months, leap month or first day differ. Its controls (`diffrows.py control`): exactly the planted day (1982-02-20) and exactly the planted month (2009-07-08), each version, exit 0. The T4 fixture against TCG: the same 17 Phugpa days (`diff` of the two lists silent).
- **Against D-23's 53 days** (`cmp53.py`, Appendix C, reading spec 009 §1.2's three tables): 17 = 17, 13 = 13, 23 = 23, **equal date by date and in every engine and TCG cell; no disagreeing day outside the lists.** Its control (one Genden date dropped from the re-run's list in memory): reported exactly, 1903-11-01, exit 1. **STOP RULE (6) did not fire.** D-24 corrected in place (`389c67e`): the counts, the commit, its new TRIGGER; Status OPEN (no oracle enters the tree, F24).

### S2.4 Item 12 — the day-start offset probe (MEASURED 2026-11-13; a description, nothing adopted)

The instrument: `offset_probe.py` (Appendix C), which loads `janson_ref.py` (the verbatim extraction above) by its path and labels each day J with the first lunar day N whose ⌊true date(N) + δ⌋ ≥ J, `janson_ref.labels`' rule with δ added and nothing else changed; exact rationals. Janson v1's Remark 6 was re-fetched from the library copy (`pdftotext -layout`, line 928, PDF p. 20; the sentence naming Lhasa's longitude 91° is on p. 21).

- **The control, first** (δ = 0): the copy of the rule equals `janson_ref.labels` itself on every golden date, the labels equal the engine's tshes (Item 11's dumps) on every golden date, and the mismatching dates are the engine's; matches Phugpa 374 / 398 / 373, Tsurphu 391 / 403 / 390, Genden 398 / 407 / 394. CONTROL PASS: the instrument is trusted.
- **(a) Genden in the open lunation.** Every lunar day ending in 2025-12-20..2026-01-18 with its fraction of the day: lunar day 1 ends 2025-12-20 at 0.9483; **2 ends 2025-12-22 at 0.009929 (14.3 minutes after the day's start)**; 3 to 7 end 2025-12-23 to 2025-12-27 at 0.0553, 0.0840, 0.0961, 0.0826, 0.0447; **8 ends 2025-12-27 at 0.990079 (14.3 minutes before the next day's start)**; 9 to 30 one a day from 2025-12-28 to 2026-01-18 (fractions 0.9188 falling to 0.0932 on 2026-01-08 and rising to 0.6624). Per lunation of the files (the 43 file lunations merged where two files overlap: 41), the smallest margin of a Genden lunar day's end to a day boundary runs from 1.2 minutes (2021-01-01..2021-01-13, lunar day 19) to 205.8 minutes (2025-04-28..2025-05-27); the open lunation's is 14.3 minutes. On the golden dates **28 lunar days of Genden end within 15 minutes of a day boundary with both neighbouring days in the files; on 26 every file prints Genden's tshes on both days; the 2 others are lunar day 2 (2025-12-21 | 2025-12-22) and lunar day 8 (2025-12-27 | 2025-12-28).** What the printed sequences imply under the same rule (`(a2)`): in the 2025 edition each of lunar days 2 to 7 ends one calendar day earlier than Genden's true date puts it, by at least 14.3, 79.7, 121.0, 138.4, 119.0 and 64.4 minutes; lunar days 1 and 8 to 30 end on Genden's day. In the 2026 edition lunar days 21 to 28 end one day earlier, by at least 153.1 to 762.3 minutes; the others on Genden's day. For each edition the δ ranges that would put every lunar day of the lunation on its implied day have an **empty intersection**.
- **(b) The named offsets**, mismatches per file (Ivolginsky / Aginsky 2025 / Aginsky 2026) and in all: δ = 0: Phugpa 24 / 15 / 29 = 68, Tsurphu 7 / 10 / 12 = 29, Genden 0 / 6 / 8 = 14; δ = +0.0447 d: 8 / 10 / 16 = 34, 27 / 19 / 24 = 70, 26 / 19 / 24 = 69; δ = −0.0447 d: 50 / 31 / 47 = 128, 22 / 17 / 29 = 68, 26 / 19 / 32 = 77; δ = +0.0650 d: 18 / 11 / 16 = 45, 37 / 24 / 30 = 91, 33 / 24 / 29 = 86; δ = −0.0650 d: 59 / 39 / 58 = 156, 35 / 21 / 38 = 94, 36 / 22 / 41 = 99. The offsets are the prompt's; 0.0447 d and 0.0650 d are 16.1° and 23.4° of longitude, for Ivolginsk at about 107.2° E and Aginskoye at about 114.5° E against Lhasa's 91° (the longitudes RECALLED / UNVERIFIED, from the prompt; 107.2 − 91 = 16.2° would be 0.0450 d, 114.5 − 91 = 23.5° 0.0653 d).
- **(c) The scan**, Genden over the 1,213 rows, δ from −0.15 to +0.15 by 0.01: 208, 197, 187, 177, 164, 143, 126, 118, 104, 96, 82, 66, 53, 39, 28, **14** (δ = 0), 26, 38, 50, 66, 72, 83, 95, 105, 120, 129, 142, 149, 158, 168, 178; the minimum 14 at 0 only. **Exactly**: the count is constant between breakpoints (δ where some true date plus δ crosses an integer); all 681 breakpoints of the range and every interval's midpoint (1,365 values) were evaluated: no δ gives fewer than 14, and 14 holds only for δ in [−0.000813, +0.001770) d = **[−1.17, +2.55) minutes**. **STOP RULE (8) did not fire.**
- **The blind spot** (METHOD rule 35): one constant offset of the day's start only; not a different equation, rounding, table, epoch or rule. The spec text's "The open lunation, described" carries this as a description.

### S2.5 Item 10 — the assertions behind the verdict's figures (MEASURED 2026-11-13; commit `b9ed798`)

Test code only, `crates/tshes/tests/q1_differential.rs`, reusing session 1's helpers (`compare`, `engine_day`, `lunation_starts`, `engine_starts`, `month_lines`, `with_tshes`, `runs_of`, `source_values`). Every expected value was first computed outside the harness by `figures.py` (Appendix C), from the three CSV files and Item 11's engine dumps (spec 009's probe, not this harness), and the tests were then written to those values; they passed on their first run, so **STOP RULE (7) did not fire**. Every group's test asserts the tree, then shows an in-memory plant red (METHOD rule 31; nothing on disk changes):

| Group (Item 10) | Test | Asserted (from the tree) | The plant, and what it turned red |
|---|---|---|---|
| (a) totals | `q1_verdict_the_totals_over_the_rows_and_the_distinct_days_and_a_planted_cell_moves_them` | 1,165 distinct days; per system rows / matches / all / none / split: Phugpa 1,213 / 1,145 / 1,099 / 58 / 8, Tsurphu 1,213 / 1,184 / 1,137 / 20 / 8, Genden 1,213 / 1,199 / 1,151 / 6 / 8 | Ivolginsky 2021-04-11, tshes 30 → 7: 3 violations, each system one match and one all-sources day fewer, one no-source day more |
| (b) Сагаалган | `q1_report_the_sagaalgan_days_in_each_system` (session 1's test, now asserting; V2) | the 15 cells and the 5 × 3 "first day of month 1" verdicts of session 1's §2(d) | 2025-03-01 checked on 2025-03-02: 3 violations, all naming 2025-03-01 |
| (c) boundaries | `q1_verdict_the_lunation_boundary_dates_of_every_file_and_system_and_a_planted_boundary_moves_them` | the source's lunation starts of every file (15, 14, 14 dates) and, per system, the dates where the engine's month starts differ: Phugpa Ivolginsky 2021-04-13 and 2021-09-08 against 2021-04-12 and 2021-09-07, Aginsky 2025 2025-03-01 against 2025-02-28, Aginsky 2026 2026-03-20 against 2026-03-19; Tsurphu and Genden none in any file (N1) | Ivolginsky 2021-04-13 printed 30: 4 violations, all the Ivolginsky file's (its starts, and each system's differences) |
| (d) separation | `q1_verdict_the_days_that_separate_the_systems_and_a_planted_tsurphu_value_is_counted` | all three agree on 1,098 days, two differ on 67; per pair days / rows / = first / = second: Phugpa–Tsurphu 62 / 63 / 12 / 51, Phugpa–Genden 58 / 60 / 3 / 57, Tsurphu–Genden 14 / 15 / 0 / 15 | Ivolginsky 2021-01-03 printed with Tsurphu's value: Tsurphu–Genden becomes 14 / 15 / 1 / 14 (and Phugpa–Genden 58 / 60 / 4 / 56) |
| (e) the mismatches and the lunations | `q1_verdict_every_genden_mismatch_lies_in_one_lunation_of_the_41_and_a_planted_split_is_seen` | the 14 Genden mismatches with both values (the spec text's table); all in 2025-12-20..2026-01-18, which is one merged lunation; lunations 15, 14, 14, shared 2, distinct 41; on the 8 shared days the 2025 edition prints Genden's value | Ivolginsky 2021-04-11 printed 1: lunations 16 / 14 / 14 / 2 / 42, and 2021-04-11 "lies outside" the open lunation |
| (beyond (e): C1) the extra month | `q1_verdict_the_extra_month_relation_n_1_and_a_planted_month_label_moves_the_offsets` | 2025 edition: engine leap days Tsurphu and Genden 2025-01-01..2025-01-29 (29), Phugpa none; the source's «Дополнительный» 2025-01-30..2025-02-28 (30); offsets Tsurphu and Genden 0:13, Phugpa 0:12 11:1; 2026 edition: no leap, no extra, 0:14 for all | the 2025 edition's lunation from 2025-03-30 labelled one month later: 3 violations, each an offset 11 |
| (beyond (e): N3) the open lunation | `q1_verdict_the_open_lunation_as_printed_and_as_computed_and_a_planted_doubled_day_is_seen` | as printed: 2025 edition 1 to 30; 2026 edition 13 to 20, 22 to 29, 29, 30; as computed (Tsurphu and Genden): 1, 2, 2, 3 … 7, 9 … 30 doubled 2025-12-22, skipped 8 before 2025-12-28; 13 to 30 with neither | the 2025 edition prints 2025-12-23 as 3: 1 violation, its printed sequence |

`print_totals` now reads the new `totals_of`, so the printed totals and the asserted ones come from one path; a `git archive` copy of the halt's tree run beside the new one prints the same session-1 lines (`diff`: only the new tests' lines and the Сагаалган plant's line differ). 21 of 21 pass in debug and in release, the two `--nocapture` outputs equal; `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check` clean. The three groups beyond (a) to (e) are there because C2 asks for an assertion behind every figure the verdict spec cites (Latitude 2).

### S2.6 The rulings, the calls and the correction, beside their records (Item 9, commit `f8b2af8`, and Items 11 and 13)

- **R5** (F38, option (1), in its shape; the human: "Agree with all recommendations, go."): `FORKS.md` F38 RULED, its `Ruled:` line as Item 9 dictates and `Built in: spec 014 (the verdict spec, Item 13)`; the verdict spec's sections The verdict ((a), (b)), Scope ((c)), The 14 mismatches ((d)), Reopening ((e)), The weakness it keeps ((f)).
- **R6** (D-19, (a)): `DEBT.md` D-19 corrected in place (TRIGGER (i) fired with spec 014; the new TRIGGER (ii), (iii), (iv) spec 014's verdict reopens; OPEN); the spec's "What the verdict rules out": Tsurphu as A.2 computes it, the 14 days and 15 rows re-measured and asserted (Genden 15, Tsurphu 0), the karana-Sun variant not measured.
- **R7** (D-23, (a), not fired): `DEBT.md` D-23's TRIGGER corrected in place to the best system's own list, 2025-11-19 recorded; session 1's §2(e), §11 and §12 corrected in place (Item 13 (e)); the spec's Non-properties ("nothing on a2 beyond F25").
- **R8** (D-33, (a)): `DEBT.md` D-33 corrected in place (its prompt TRIGGER fired with spec 014's prompt and was not acted on, K1; the TRIGGER now "the human saves any of them into the library"; OPEN); the verdict's Scope cites D-33.
- **R9** (F4, (b), after M3): `FORKS.md` F4 RULED, its `Ruled:` line as Item 9 dictates; the spec's Forks section.
- **Forced by the records**: D-1 PAID (`DEBT.md`, with the added sentence on a client's default); D-24's re-run (S2.3) and its correction (`389c67e`).
- **C1** (the extra month, n = 1): the spec's "The extra month"; `DEBT.md` D-40 filed; asserted (S2.5). **C2**: S2.5. **C3**: S2.4 and the spec's "The open lunation, described". **C4**: the head block's model line.
- **K1**: D-33's correction names it.
- D-35's added sentence (the verdict's Reopening (i)).

### S2.7 Every property and every task (Items 8 to 14)

| Item | State | Evidence |
|---|---|---|
| 8 the resume | DONE | S2.2; `e4ac576` |
| 9 the records | DONE | `f8b2af8`: F38, F4, D-1, D-19, D-23, D-33, D-35, D-40; D-24 in `389c67e` after Item 11, as the item says |
| 10 the assertions | DONE | S2.5; `b9ed798` |
| 11 D-24's re-run | DONE | S2.3; `389c67e` |
| 12 the offset probe | DONE | S2.4 (no tree change) |
| 13 the verdict spec | DONE | the spec text; (e) the four corrections at §1, §2(e), §11 and §12; this section; commit `f7142cc` |
| 14 the close | DONE | S2.10; the final commit |

The verdict spec's properties, each DONE, each test looked up in the tree before the gate (METHOD rule 7: `grep -c "fn <name>()"` printed 1 for each of the 19 names cited, and the files hold 21 and 12 tests): **P1** the pin and its control (session 1; green in the gate); **P2** to **P8** the Item 10 tests of S2.5 (each green on the tree and red on its plant); **P9** the 12 tests of `buryat_golden.rs` (session 1, D-37); **P10** the four controls of session 1. Gate (Item 14): 180 passed, 0 failed, in debug, release and the msrv leg.

### S2.8 The reviewer's findings, each answered

- **V1** (the T4 control cannot exercise the golden tshes column): no action; cited in the spec's Non-properties.
- **V2** (`q1_report_the_sagaalgan_days_in_each_system` asserted nothing): it now asserts all fifteen cells and is shown red on a moved day (P3). The test still reads no golden file (its days are the constant `SAGAALGAN`, which `q1_the_sagaalgan_days_are_named_in_the_metas` ties to the metas); which other three tests pass with `golden/buryat/` deleted was not re-measured here.
- **V3** (D-23's non-firing rested on an unstated reading): R7; D-23 corrected; session 1's §2(e), §11 and §12 corrected in place.
- **V4** (§6.5's "over all Buryat golden rows" left out): R5 (b) in the verdict; session 1's §1 and §12 corrected in place.
- **V5** ("(ii) is satisfied" applied the undefined "small number"): session 1's §1 and §12 corrected in place; the verdict defines no threshold (Non-properties).
- **V6** (D-24's re-run not among the costs): Item 11, S2.3.
- **V7**: closed by R5; no action.
- **V8** ("a local variant" not among the options): the verdict says it is not excluded (The 14 mismatches; What the verdict rules out).
- **N1** (the Ivolginsky boundaries printed as counts): P4 asserts the dates; MEASURED equal for Tsurphu and Genden in all three files, and the two Phugpa boundaries the reviewer named (2021-04-12 against 2021-04-13, 2021-09-07 against 2021-09-08).
- **N2** (D-33's TRIGGER fired, the entry OPEN): R8.
- **N3** (the 2025 edition's 2025-12-20 to 2025-12-29 against Tsurphu and Genden): MEASURED for the whole lunation (1 to 30, against Genden's doubled 2 and skipped 8) and asserted (P8); the spec's "The open lunation, described".

### S2.9 The predictions, scored (METHOD rule 3)

| # | Prediction | Score | Measured value |
|---|---|---|---|
| 12 | D-24's re-run: exactly D-23's 53 days (17 / 13 / 23), no other | HELD | S2.3: 17 / 13 / 23, equal date by date and cell by cell |
| 13 | `janson_ref.py` at a2 = 1/28 labels the golden dates with the engine's matches, Phugpa 374 / 398 / 373, Tsurphu 391 / 403 / 390, Genden 398 / 407 / 394 | HELD | S2.4's control: those counts, and the labels equal the engine's tshes on every golden date; the extraction's sha256 equals the advisor's |
| 14 | lunar day 2 ends about 0.010 after the daybreak of 2025-12-22 and lunar day 8 about 0.990 of 2025-12-27; the 2025 edition's values "need the first earlier and the second later"; 28 lunar days end within 15 minutes of a daybreak, on 26 the sources print Genden's value on both days, the 2 others those two | **FAILED in one clause** | 0.009929 and 0.990079: HELD; 28, 26 and the 2 named: HELD. "The second later": FAILED. Lunar day 8 already ends on the day the 2025 edition implies (2025-12-27); what the print needs, read by the same rule, is lunar days 2 to 7 each ending a day earlier (S2.4 (a)) |
| 15 | Genden mismatches 14 at 0; 69 at +0.0447, 86 at +0.0650; 77 and 99 westward; 0 the unique minimum of the scan | HELD | S2.4 (b), (c); exactly, the minimum region is [−1.17, +2.55) minutes |
| 16 | every Item 10 assertion passes; the gate's count rises by the tests Item 10 adds | HELD | 21 of 21; the gate 174 → 180 = 174 + 6 |
| 17 | OPEN 6 at the base, 4 at the close | HELD | S2.11 |

### S2.10 Item 14 — the close

- `docs/ROADMAP.md` (r7): the q1-verdict row LANDED; Gate 1's state cell "REACHED: spec 014's verdict (Genden, scoped; FORKS.md F38 RULED (1)), in force on main from its merge; D-1 PAID"; a Revisions line; the Status line r6 → r7 beside it (hard rule 19). **TRIGGERs this fires:** zurkhaich-review's ("F4 is ruled") **fires in whole**: F4 is RULED (b); the website's ("the Q1 verdict spec exists (D-1), and l2-schema landed") **in part**: its first half holds once this spec is on main, l2-schema has not landed; android's and l2-schema's are untouched.
- `docs/specs/INDEX.md`: this round's row, status `accepted`. **D-27's TRIGGER does not fire**: nothing revises 003 or `TEMPLATE.md`, and the row's `accepted` is the status of a spec, which `TEMPLATE.md` defines, not read by any record as a round's state.
- **The gate** ran once, with nothing in flight, on `b9ed79887debfef7c6c50243645aab0610817143`, the last commit that changes a path beyond this file, its INDEX row and its ROADMAP row (`docs/ROUNDS.md` §3): `GATE: 11 legs run — 11 passed, 0 failed, 1 skipped` (wasm-pack: `crates/tshes-wasm` does not exist); the `test result:` sums: test 180 / 0, test_release 180 / 0, msrv 180 / 0; `SELF-TEST: 26 plants — 26 rejected`; `git status --porcelain` printed nothing after it (11:57:02 to 12:31:04). Session 1's count was 174 (its own record); the difference, 6, is the six new tests of S2.5, all passing: `q1_verdict_the_totals_over_the_rows_and_the_distinct_days_and_a_planted_cell_moves_them`, `q1_verdict_the_lunation_boundary_dates_of_every_file_and_system_and_a_planted_boundary_moves_them`, `q1_verdict_the_days_that_separate_the_systems_and_a_planted_tsurphu_value_is_counted`, `q1_verdict_every_genden_mismatch_lies_in_one_lunation_of_the_41_and_a_planted_split_is_seen`, `q1_verdict_the_extra_month_relation_n_1_and_a_planted_month_label_moves_the_offsets`, `q1_verdict_the_open_lunation_as_printed_and_as_computed_and_a_planted_doubled_day_is_seen`. The commits after it change only this file, `docs/specs/INDEX.md` and `docs/ROADMAP.md`.
- Every file `git status --porcelain` showed before each commit was one the commit meant to change; the test framework wrote no file (no `proptest-regressions` seed).

### S2.11 The stop rule, forks resolved, latitude, sums, what the prompt got wrong, by-catch, held

**The stop rule that governed session 2**: none fired. STOP RULES (1) (S2.2, with the ARTIFACT gap reported, not passed over), (6) (S2.3), (7) (S2.5) and (8) (S2.4) were each checked against their condition; (2), (3), (5) and (10) were kept: no file under `crates/tshes/src/`, `golden/`, `rules/`, `clients/`, `tools/` or `.github/` changed, no variant was added to the tree, and the T4 fixture appears only as a control; (4): the verdict sentence is R5's, written once, and no other sentence says that a datsan, a calendar or a source follows, uses or is a system; (9): no read outside the grant (the `henning-kalacakra/` manifest was not computed for that reason).

**Forks met and resolved inside the scope ("considered X, chose Y because Z").** (1) The `tdata/` check: considered hashing all 17 files of `henning-kalacakra/` to recompute the manifest of `docs/sources.md`, chose the build check plus per-file hashes recorded now, because the 10 `.htm` files are outside the grant and the build check compares the six files' content line by line with what spec 009 compared. (2) Where Item 10's tests live: considered a new test file, chose `q1_differential.rs`, because its private helpers (`loaded`, `distinct_days`, `row_on`, `source_values`, `runs_of`) would otherwise be copied (the ADDENDUM: "no new harness"). (3) Each group's plant inside its own test, after the assertion on the tree, as `buryat_golden.rs`'s refusal tests do, rather than a separate plant test per group. (4) The `print_totals` refactor onto `totals_of`, so that the printed and the asserted totals cannot drift; its printed lines unchanged (S2.5). (5) Item 12's "any non-zero offset" (STOP RULE (8)) checked exactly over the range, on every breakpoint, as well as on the prompt's 0.01 grid, because a grid can step over a piece. (6) The lunations of Item 12 (a) and of P6 merged across files by overlapping date ranges, which gives the advisor's 41. (7) The verdict spec records session 1's three VARIANT CANDIDATE days as UNEXPLAINED and OPEN, as R5 (d) rules, with a sentence saying what session 1's class meant. (8) The human's words in F38's and F4's `Ruled:` lines in double quotes, the register's style, where the ADDENDUM's text nests single quotes inside its own double quotes. (9) The spec's title changed from the differential's to the verdict's (the INDEX row carries it). (10) Session 1's squash message replaced by one covering both sessions (`docs/ROUNDS.md` §9 extracts the first block under that heading).

**Latitude taken beyond the prompt.** (1) The ARTIFACT's `tdata/` sha256 comparison with `docs/sources.md` was not made (it cannot be made within the grant); the build check stands in for it, run before the first commit. (2) Item 10 asserts three groups beyond (a) to (e): the 14 mismatches with both values (inside (e)), the extra-month relation and the open lunation (N3), for C2's "every figure the verdict spec cites". (3) Item 12's exact evaluation and its per-lunar-day `(a2)` description. (4) The Phugpa–Tsurphu separation (62 days, 63 rows, 12 and 51), measured and asserted, which session 1 did not report. (5) The build check of Item 11 run before Item 8's commit (it settled an ARTIFACT question). (6) The ROADMAP's Status line r6 → r7 (hard rule 19). (7) The spec's title, and the squash message replaced.

**Numbers that must sum.** Rows 398 + 413 + 402 = 1,213; per system 1,145 + 68 = 1,184 + 29 = 1,199 + 14 = 1,213; distinct days 1,213 − 48 = 1,165 = 1,099 + 58 + 8 = 1,137 + 20 + 8 = 1,151 + 6 + 8 = 1,098 + 67; Genden's mismatches 6 + 8 = 14; lunations 15 + 14 + 14 − 2 = 41; separation rows 12 + 51 = 63, 3 + 57 = 60, 0 + 15 = 15; D-24 17 + 13 + 23 = 53; TCG files 3 × 202 = 606; build check 1,470 × 5 + 1,590 = 8,940; near-boundary lunar days 26 + 2 = 28; gate 174 + 6 = 180 in each profile; OPEN 6 − 2 = 4.

**What the prompt got wrong.** (1) ARTIFACT asks to compare each `tdata/` sha256 with `docs/sources.md`'s row; that row has no per-file hash, only a manifest over all 17 files of `henning-kalacakra/`, which the session's grant (`tdata/` only) does not let it recompute. (2) "the pl_<year>.txt and ts_<year>.txt files spec 009's build check reads": the build check reads the six `pl_` files only; `ts_2012.txt` is read by no step of spec 009's §21. (3) Item 12 places Remark 6's Lhasa sentence on p. 20; the remark opens on p. 20 and that sentence is on p. 21. (4) Prediction 14's "the second later" (S2.9). (5) **`DEBT.md` D-18's TRIGGER (iii), "the Q1 verdict spec is accepted (M3)", fires with this spec**, and the ADDENDUM names no action for it; D-18 is not edited here (DEBT.md changes only as the items say), and the round it calls for ("the round that reopens the rows starts from that table and puts F15–F18's questions again") is out of scope (By-catch (1)). (6) Item 13 (e) names four places for the corrections; statements of the same kind stand farther away and are flagged, not corrected (`CLAUDE.md` hard rule 19): §7's leg ④ ("D-23 does not fire"), §8's prediction 8 row ("D-23 does not fire (§2(e))"), and §12's options 1 ("and the first, per source, for one file") and 4 ("Follows the first bullet per source"). (7) Item 12 (b)'s offsets are 16.1° and 23.4° of longitude, not quite the 16.2° and 23.5° its longitudes give (S2.4 (b)); the prompt's values were used. (8) Item 9 dictates D-1's Status as "PAID <date>, spec 014 (…)", where `DEBT.md`'s shape is "PAID <date, commit>"; written as dictated, since the commit that puts it in force is the merge. (9) Item 14 asks for "the marker greps for both sessions' markers"; session 1's marker was never spelled in the tree or the prompt, so its grep cannot be run by a later session (S2.12). Nothing else found against the ADDENDUM's dictated text: D-40 was the next free id, and the figures it RELAYED (41 lunations, the predictions' counts, the extraction hash) re-measured equal.

**By-catch.** (1) D-18's TRIGGER (iii) (above, (5)): filed already as D-18; its firing is for the advisor's next prompt. (2) `henning-kalacakra/tdata/ts_2012.txt` is held and read by nothing; no entry (it is a library file, and spec 009's `docs/sources.md` row already says it is not an oracle). (3) For neither Aginsky edition does a constant day-start offset reproduce the open lunation (S2.4 (a)); part of the verdict's description, not filed. (4) zurkhaich-review's ROADMAP TRIGGER fires with F4's ruling (S2.10). Nothing else.

**Held** (identified items only): D-18 (fires with this spec); D-19, D-23, D-24, D-33, D-35, D-39, D-40 (OPEN); F5, F7, F8, F10 (OPEN).

### S2.12 The open-forks audit and the marker greps, at every commit of session 2

`expected = previous OPEN + forks raised + gated findings filed with options and none chosen − rulings recorded` (METHOD rule 60), measured with `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md` inside each commit's `&&`-chain.

| Commit | Previous | Raised | Filed, none chosen | Rulings recorded | Expected | Measured | Session 2's marker |
|---|---|---|---|---|---|---|---|
| `e4ac576` (Item 8) | 6 (base) | 0 | 0 | 0 | 6 | 6 | no hit |
| `f8b2af8` (Item 9) | 6 | 0 | 0 | 2 (F38, F4) | 4 | 4 | no hit |
| `389c67e` (Item 11, D-24) | 4 | 0 | 0 | 0 | 4 | 4 | no hit |
| `b9ed798` (Item 10) | 4 | 0 | 0 | 0 | 4 | 4 | no hit |
| `f7142cc` (Item 13) | 4 | 0 | 0 | 0 | 4 | 4 | its three placeholder lines, in this file only, as the chain asserted |
| the final commit (Item 14) | 4 | 0 | 0 | 0 | 4 | 4 | no hit |

Session 2's marker stood only in this file's own placeholders between the Item 13 commit and the final one, replaced there; the final grep prints nothing. **Session 1's marker was not grepped**: session 1 never spelled it (its report calls it "the round's marker" only), so this session does not know the token, and no grep for it could be run (What the prompt got wrong, (9)). What can be measured instead: session 1's report says the token never occurred in a tracked file, and every line session 2 changed in a file session 1 wrote was read in this session.

### S2.13 Diff scope

MEASURED at `f7142cc` (Item 13) with `git diff --numstat main f7142cc`, both sessions: `DEBT.md` +24 −8; `FORKS.md` +22 −5; `crates/tshes/tests/buryat_golden.rs` +428 −0; `crates/tshes/tests/common/buryat.rs` +539 −0; `crates/tshes/tests/common/mod.rs` +2 −0; `crates/tshes/tests/common/q1.rs` +276 −0; `crates/tshes/tests/q1_differential.rs` +1,882 −0; `docs/ROADMAP.md` +3 −2; `docs/specs/014-q1-verdict.md` +1,821 −0; `docs/specs/prompts/014-q1-verdict.md` +880 −0. Sum: 24 + 22 + 428 + 539 + 2 + 276 + 1,882 + 3 + 1,821 + 880 = **5,877 insertions**, 8 + 5 + 2 = **15 deletions**, equal to `git diff --shortstat main f7142cc` ("10 files changed, 5877 insertions(+), 15 deletions(-)"). Session 2's share (`git diff --numstat 5cec12a b9ed798` before the spec file): `DEBT.md` +15 −7, `FORKS.md` +5 −2, `q1_differential.rs` +848 −28, the prompt copy +392 −0. The final commit adds `docs/ROADMAP.md` +4 −3 (on top of the +3 −2 above; MEASURED on the final edits with `git diff --numstat`), `docs/specs/INDEX.md` +1 −0 (an eleventh file), and this file's own last edits (the placeholders filled), which a report cannot count for the commit that holds it. Gated paths touched: **yes**, `crates/tshes/tests/` (test code only, under accepted specs 001 and 002), this round's own spec file and prompt copy, and its INDEX row (final commit); nothing under `crates/tshes/src/`, `golden/`, `rules/`, `clients/`, `tools/`, `.github/`, `docs/METHOD.md`, `GATED.md`, `CLAUDE.md`, `docs/ROUNDS.md`, `docs/ADVISOR.md`, an accepted spec, or another round's INDEX row. Files the test framework wrote: none.

### Appendix C. Session 2's commands and scripts

**Commands** (absolute paths; `S` the session's `mktemp -d` directory, `W` its work directory, `X` the zip's extraction, `E` the engine copy): the preconditions as quoted in S2.2; `sha256sum` of the three library holdings and the seven `tdata/` files; `cargo test --offline --manifest-path <repo>/Cargo.toml --workspace` (the baseline); `python3 -I $S/tools/extract009.py <repo>/docs/specs/009-henning-differential.md $S/x009`, then `sed -n '<a>,<b>p'` of the same ranges and `cmp`; `unzip -q <library>/henning-tcg/tcg1309.zip -d $X` and §21.1's `gcc` lines for `-O0` and `-O2`; §21.1's inputs run with `env --chdir=<run dir> $W/o<opt>/tcg < in.txt` (the build check; Phugpa; Tsurphu; Genden one process per year 1899..2100); `cmpbuild.py real` and `controls`, `readtcg.py`, `checkrows.py`, `omitcheck.py`, `readcheck.py` and its `control`, as §21.1 runs them; `git -C <repo> archive HEAD | tar -x -C $E`, the probe copied into `$E/crates/tshes/tests/`, and §21.1's two `cargo +1.99.0 test --offline` lines; `cmp` of the dumps (debug, release, the T4 fixture); `diffrows.py diff` and `control` per version, and the T4 fixture against TCG; `python3 -I $S/tools/cmp53.py <009 file> $S/lists [--plant]`; `pdftotext -layout <library>/janson-1401.6285v1.pdf $S/janson/janson.txt` and `python3 -I $S/tools/pages.py $S/janson/janson.txt <patterns>`; `python3 -I $S/tools/offset_probe.py $S/x009/janson_ref.py <repo> $S/dump-debug`; `python3 -I $S/tools/figures.py <repo> $S/dump-debug`; `cargo test --offline … --test q1_differential -- --nocapture --test-threads=1`, with `--release`, the outputs `diff`ed, and the same on a `git archive` copy of the halt's tree; `cargo clippy --offline … --workspace --all-targets -- -D warnings`; `cargo fmt … --all --check`; `bash <repo>/tools/gate.sh` once. After the scripts were written into this appendix, a script of the session's (`extractC.py`, in the scratch directory) extracted the five blocks back out of this file, and each `cmp` with the file that ran was silent. The scripts (sha256 of each file as run: `extract009.py` `91d90486…a8240`, `cmp53.py` `d4d6b88a…a963`, `offset_probe.py` `202a688f…eabb5`, `figures.py` `8ab20718…1486b`, `pages.py` `007f67bd…a01d`), verbatim:

`extract009.py`:

```python
#!/usr/bin/env python3
"""Spec 014 session 2, Item 11: extract spec 009 section 21's recorded files verbatim.

usage: python3 -I extract009.py <path of 009-henning-differential.md> <out dir>
Writes each fenced block that follows a heading `#### <name>` (21.5, the scripts) or a line
`` `<name>`: `` (21.2, the stand-in headers) or the dump probe of 21.3, byte for byte, with the
block's own final newline, and prints: name, first and last line number of the block's body in
the spec file (1-based), its sha256. Reads only the spec file.
"""
import hashlib
import os
import re
import sys

src, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
lines = open(src, encoding="utf-8").read().split("\n")
start = next(i for i, ln in enumerate(lines) if ln.startswith("## 21. "))
name = None
i = start
found = []
while i < len(lines):
    ln = lines[i]
    m = re.match(r"^#### ([A-Za-z0-9_]+\.py)$", ln) or re.match(r"^`([a-z]+\.h)`:$", ln)
    if m:
        name = m.group(1)
    elif ln.startswith("**The engine dump probe** `crates/tshes/tests/zz_spec009_dump.rs`"):
        name = "zz_spec009_dump.rs"
    elif ln.startswith("```") and name is not None:
        j = i + 1
        while not lines[j].startswith("```"):
            j += 1
        body = "\n".join(lines[i + 1:j]) + "\n"
        path = os.path.join(out, name)
        with open(path, "w", encoding="utf-8", newline="") as f:
            f.write(body)
        found.append((name, i + 2, j, hashlib.sha256(body.encode("utf-8")).hexdigest()))
        name = None
        i = j
    i += 1
for n, a, b, h in found:
    print(f"{n} lines {a}-{b} sha256 {h}")
print("files:", len(found))
```

`cmp53.py`:

```python
#!/usr/bin/env python3
"""Spec 014 session 2, Item 11: compare this re-run's disagreement lists with spec 009 section 1.2's
three tables (D-23's 53 days), date by date, and the engine and TCG cells too.

usage: python3 -I cmp53.py <009-henning-differential.md> <lists dir> [--plant]
The lists dir holds diffrows.py's phugpa.txt, tsurphu.txt, genden.txt. --plant drops the first
Genden date from the re-run's list in memory (a control: exactly that date must be reported).
Reads only the two inputs.
"""
import re
import sys

spec, lists = sys.argv[1], sys.argv[2]
plant = "--plant" in sys.argv
text = open(spec, encoding="utf-8").read().split("\n")
sec = [i for i, ln in enumerate(text) if ln.startswith("### 1.2 ")][0]
end = [i for i, ln in enumerate(text) if ln.startswith("**(ii) Where the two computations part.**")][0]
rec = {}
system = None
for ln in text[sec:end]:
    m = re.match(r"\*\*(Phugpa|Tsurphu|Genden) \((\d+)\):\*\*", ln)
    if m:
        system = m.group(1).lower()
        rec[system] = {}
        continue
    m = re.match(r"\| (\d{4}-\d\d-\d\d) \| (\S+) \| (\S+) \|", ln)
    if m and system:
        rec[system][m.group(1)] = (m.group(2), m.group(3))


def cell(t):
    y, mo, lp, ts = (x.strip() for x in t.strip("() ").split(","))
    return f"{y}/{mo}{'L' if lp == chr(39) + 'true' + chr(39) else ''}/{ts}"


ok = True
total_rec = total_new = 0
for v in ("phugpa", "tsurphu", "genden"):
    new = {}
    for ln in open(f"{lists}/{v}.txt", encoding="utf-8"):
        if ln[0].isdigit():
            p = [x.strip() for x in ln.split(" | ")]
            new[p[0]] = (cell(p[1]), cell(p[2]))
    if plant and v == "genden":
        del new[sorted(new)[0]]
    r = rec[v]
    total_rec += len(r)
    total_new += len(new)
    only_new = sorted(set(new) - set(r))
    only_rec = sorted(set(r) - set(new))
    cells_differ = sorted(d for d in set(new) & set(r) if new[d] != r[d])
    good = not only_new and not only_rec and not cells_differ
    ok &= good
    print(f"{v}: spec 009 lists {len(r)}, this re-run {len(new)}; only in the re-run {only_new}; "
          f"only in spec 009 {only_rec}; dates whose engine or TCG cell differs {cells_differ}: "
          f"{'EQUAL' if good else 'NOT EQUAL'}")
print(f"total: spec 009 {total_rec}, re-run {total_new}: {'EQUAL date by date and cell by cell' if ok else 'NOT EQUAL'}")
sys.exit(0 if ok else 1)
```

`pages.py`:

```python
#!/usr/bin/env python3
"""Spec 014 session 2: the PDF page (form feeds + 1) of each line of a pdftotext -layout copy that
matches one of the given regular expressions. usage: python3 -I pages.py <text file> <regex>..."""
import re
import sys

text = open(sys.argv[1], encoding="utf-8").read()
page = 1
for n, line in enumerate(text.split("\n"), 1):
    page += line.count("\f")
    for pat in sys.argv[2:]:
        if re.search(pat, line):
            print(f"p. {page:3d} line {n:5d}: {line.strip()[:150]}")
```

`offset_probe.py` (Item 12; the sections `(a2)` and the exact interval were added before its last run, which is the output S2.4 quotes):

```python
#!/usr/bin/env python3
"""Spec 014 session 2, Item 12: the day-start offset probe. A description; it adopts nothing.

usage: python3 -I offset_probe.py <janson_ref.py> <repo root> <engine dump dir>
  <janson_ref.py>  spec 009's script, extracted verbatim (loaded by path; nothing in it is changed)
  <repo root>      reads only golden/buryat/*.csv
  <engine dump dir> engine-<v>.csv of Item 11 (the engine of HEAD), for the control only

The rule under test: calendar day J is labelled by the first lunar day N with
floor(true_date(N) + delta) >= J. At delta = 0 this is janson_ref.labels (Section 6, (8.1)).
delta > 0 moves the start of every day earlier against the true date (a meridian east of the
one the formulas assume); delta < 0 later. Exact rational arithmetic throughout.
"""
import importlib.util
import sys
from fractions import Fraction as F

jr_path, root, dumpdir = sys.argv[1], sys.argv[2], sys.argv[3]
spec = importlib.util.spec_from_file_location("janson_ref", jr_path)
jr = importlib.util.module_from_spec(spec)
spec.loader.exec_module(jr)

FILES = ["ivolginsky-datsan-2021", "aginsky-datsan-2025", "aginsky-datsan-2026"]
SYSTEMS = ["phugpa", "tsurphu", "genden"]
golden = {}
for f in FILES:
    lines = open(f"{root}/golden/buryat/{f}.csv", encoding="utf-8").read().split("\n")
    assert lines[-1] == ""
    golden[f] = [(r.split(",")[0], int(r.split(",")[1])) for r in lines[1:-1]]
ROWS = sum(len(v) for v in golden.values())


def jd(s):
    return jr.jd_of(int(s[:4]), int(s[5:7]), int(s[8:]))


RANGES = [(jd(golden[f][0][0]), jd(golden[f][-1][0])) for f in FILES]
JMIN, JMAX = min(a for a, _ in RANGES), max(b for _, b in RANGES)

# the true dates of every lunar day near the golden dates, per system (independent of delta)
TD = {}
for v in SYSTEMS:
    p, a2 = jr.make(v)
    n0 = int((JMIN - p["m0"]) / jr.M2) - 12
    n1 = int((JMAX - p["m0"]) / jr.M2) + 12
    TD[v] = {N: jr.true_date(p, a2, N) for N in range(n0, n1 + 1)}


def labels_delta(v, j_first, j_last, delta):
    """janson_ref.labels' rule with floor(td + delta) in place of floor(td); tshes only."""
    tds = TD[v]
    ns = sorted(tds)
    fl = {N: jr.floor(tds[N] + delta) for N in ns}
    assert fl[ns[0]] < j_first and fl[ns[-1]] > j_last
    out = {}
    i = 0
    for J in range(j_first, j_last + 1):
        while fl[ns[i]] < J:
            i += 1
        n, d = divmod(ns[i] - 1, 30)
        out[J] = (d + 1, ns[i])
    return out


def mismatches(v, delta):
    per = {}
    for f, (a, b) in zip(FILES, RANGES):
        lab = labels_delta(v, a, b, delta)
        per[f] = [d for d, t in golden[f] if lab[jd(d)][0] != t]
    return per


# ---------------------------------------------------------------- the control, first
print("CONTROL (delta = 0)")
ok = True
for v in SYSTEMS:
    eng = {}
    for ln in open(f"{dumpdir}/engine-{v}.csv", encoding="utf-8").read().split("\n")[1:]:
        if ln:
            c = ln.split(",")
            eng[c[0]] = int(c[4])
    # (1) the copy of the rule equals janson_ref.labels itself on every golden date
    same_rule = True
    for (a, b) in RANGES:
        ref = jr.labels(v, a, b)
        mine = labels_delta(v, a, b, F(0))
        same_rule &= all(ref[J][3] == mine[J][0] and ref[J][4] == mine[J][1] for J in range(a, b + 1))
    # (2) the labels equal the engine's tshes on every golden date
    lab_eq_eng = True
    for f, (a, b) in zip(FILES, RANGES):
        lab = labels_delta(v, a, b, F(0))
        lab_eq_eng &= all(lab[jd(d)][0] == eng[d] for d, _ in golden[f])
    per = mismatches(v, F(0))
    eng_miss = {f: [d for d, t in golden[f] if eng[d] != t] for f in FILES}
    match = " / ".join(str(len(golden[f]) - len(per[f])) for f in FILES)
    same_miss = all(per[f] == eng_miss[f] for f in FILES)
    print(f"  {v}: matches {match}; rule copy = janson_ref.labels on every golden date: {same_rule}; "
          f"labels = engine tshes on every golden date: {lab_eq_eng}; mismatching dates = the engine's: {same_miss}")
    ok &= same_rule and lab_eq_eng and same_miss
print("CONTROL", "PASS" if ok else "FAIL")
if not ok:
    sys.exit(1)

# ---------------------------------------------------------------- (a) Genden in the lunation 2025-12-20..2026-01-18
print()
print("(a) Genden: every lunar day ending in 2025-12-20..2026-01-18 (true date, its day and its fraction of the day)")
lo, hi = jd("2025-12-20"), jd("2026-01-18")
for N in sorted(TD["genden"]):
    t = TD["genden"][N]
    k = jr.floor(t)
    if lo <= k <= hi:
        fr = t - k
        n, d = divmod(N - 1, 30)
        mins = float(min(fr, 1 - fr)) * 1440
        side = "after the day's start" if fr < F(1, 2) else "before the next day's start"
        print(f"  lunar day {d + 1:2d} (N={N}) ends on {jr.date_of(k)}: fraction {float(fr):.6f} "
              f"({mins:6.1f} min {side})")

# lunations of the files (golden/buryat/README.md: a lunation begins where tshes falls), merged across files
lun = []
for f in FILES:
    rows = golden[f]
    starts = [0] + [i for i in range(1, len(rows)) if rows[i][1] < rows[i - 1][1]]
    ends = starts[1:] + [len(rows)]
    for s, e in zip(starts, ends):
        lun.append((jd(rows[s][0]), jd(rows[e - 1][0]), f))
lun.sort()
merged = []
for a, b, f in lun:
    if merged and a <= merged[-1][1]:
        merged[-1] = (merged[-1][0], max(b, merged[-1][1]), merged[-1][2] + [f])
    else:
        merged.append((a, b, [f]))
print()
print(f"lunations of the files: {len(lun)} per file ({', '.join(str(sum(1 for x in lun if x[2] == f)) for f in FILES)}); "
      f"distinct after merging overlaps: {len(merged)}")
print("smallest margin of a Genden lunar day's end to a day boundary, per lunation (minutes; the lunar day; its day)")
for a, b, fs in merged:
    best = None
    for N, t in TD["genden"].items():
        k = jr.floor(t)
        if a <= k <= b:
            fr = t - k
            m = min(fr, 1 - fr)
            if best is None or m < best[0]:
                best = (m, N, k, fr)
    m, N, k, fr = best
    print(f"  {jr.date_of(a)}..{jr.date_of(b)} ({'+'.join(x.split('-')[0][:3] + x[-4:] for x in fs)}): "
          f"{float(m) * 1440:6.1f} min, lunar day {(N - 1) % 30 + 1} ending {jr.date_of(k)} at fraction {float(fr):.4f}")

# lunar days ending within 15 minutes of a boundary, with both neighbouring days in the files
Q = F(15, 1440)
days_in = {}
for f in FILES:
    for d, t in golden[f]:
        days_in.setdefault(jd(d), []).append((f, t))
eng_g = {}
for f, (a, b) in zip(FILES, RANGES):
    for J, (ts, N) in labels_delta("genden", a, b, F(0)).items():
        eng_g[J] = ts
near = []
for N, t in sorted(TD["genden"].items()):
    k = jr.floor(t)
    fr = t - k
    if fr < Q:
        b = k            # just after the start of day k: the neighbours are k-1 and k
    elif 1 - fr < Q:
        b = k + 1        # just before the start of day k+1: the neighbours are k and k+1
    else:
        continue
    if b - 1 in days_in and b in days_in:
        both = all(t2 == eng_g[J] for J in (b - 1, b) for _, t2 in days_in[J])
        near.append((N, b, fr, both))
print()
print(f"Genden lunar days ending within 15 minutes of a day boundary, both neighbouring days in the files: {len(near)}; "
      f"on both neighbouring days every file prints Genden's tshes: {sum(1 for x in near if x[3])}; not: "
      f"{[(str(jr.date_of(b - 1)) + '|' + str(jr.date_of(b)), 'lunar day %d' % ((N - 1) % 30 + 1), round(float(fr if fr < Q else fr - 1) * 1440, 1)) for N, b, fr, both in near if not both]}")

# ---------------------------------------------------------------- (b) the named offsets
print()
print("(b) mismatches per file (Ivolginsky / Aginsky 2025 / Aginsky 2026) and the total of 1,213 rows")
NAMED = [F(0), F(447, 10000), F(-447, 10000), F(65, 1000), F(-65, 1000)]
for delta in NAMED:
    cells = []
    for v in SYSTEMS:
        per = mismatches(v, delta)
        cells.append(f"{v} {' / '.join(str(len(per[f])) for f in FILES)} = {sum(len(per[f]) for f in FILES)}")
    print(f"  delta {float(delta):+.4f} d: " + "; ".join(cells))

# ---------------------------------------------------------------- (c) the scan, and the exact minimum
print()
print("(c) Genden mismatches over the 1,213 rows, delta -0.15..+0.15 by 0.01")
scan = []
for k in range(-15, 16):
    delta = F(k, 100)
    per = mismatches("genden", delta)
    scan.append((delta, sum(len(x) for x in per.values())))
print("  " + "; ".join(f"{float(d):+.2f}: {n}" for d, n in scan))
mn = min(n for _, n in scan)
print(f"  the scan's minimum {mn} at {[float(d) for d, n in scan if n == mn]}")
# every piece: the count changes only where some td(N) + delta crosses an integer
brk = set()
for t in TD["genden"].values():
    for j in range(jr.floor(t) - 1, jr.floor(t) + 3):
        x = j - t
        if F(-15, 100) <= x <= F(15, 100):
            brk.add(x)
pts = sorted(brk | {F(-15, 100), F(15, 100)})
probe = sorted(set(pts) | {(a + b) / 2 for a, b in zip(pts, pts[1:])})
best = {}
for delta in probe:
    n = sum(len(x) for x in mismatches("genden", delta).values())
    best.setdefault(n, []).append(delta)
lo_n = min(best)
zero_piece = [d for d in probe if sum(len(x) for x in mismatches("genden", d).values()) == 14]
print(f"  EXACT over [-0.15, +0.15]: {len(brk)} breakpoints, {len(probe)} values probed (every breakpoint and every "
      f"piece's midpoint); the minimum count is {lo_n}, at delta from {float(min(best[lo_n])):+.6f} to "
      f"{float(max(best[lo_n])):+.6f} ({len(best[lo_n])} probed values)")
neg = [d for d in brk if d <= 0]
pos = [d for d in brk if d > 0]
print(f"  the piece that holds delta = 0 runs from the breakpoint {float(max(neg)):+.6f} d "
      f"({float(max(neg)) * 1440:+.1f} min) to {float(min(pos)):+.6f} d ({float(min(pos)) * 1440:+.1f} min)")
print(f"  any probed delta with fewer than 14 Genden mismatches: {[float(d) for n, ds in best.items() if n < 14 for d in ds]}")
# the exact set of delta with the minimum count, as intervals [left, right) between breakpoints
cnt = {d: sum(len(x) for x in mismatches("genden", d).values()) for d in probe}
runs = []
for i, d in enumerate(probe):
    if cnt[d] == lo_n:
        lo_b = max([b for b in pts if b <= d], default=F(-15, 100))
        hi_b = min([b for b in pts if b > d], default=F(15, 100))
        if runs and runs[-1][1] >= lo_b:
            runs[-1] = (runs[-1][0], max(runs[-1][1], hi_b))
        else:
            runs.append((lo_b, hi_b))
print("  the delta with the minimum count, exactly (intervals [left, right) between breakpoints): " +
      "; ".join(f"[{float(a):+.6f}, {float(b):+.6f}) d = [{float(a) * 1440:+.2f}, {float(b) * 1440:+.2f}) min" for a, b in runs))

# ---------------------------------------------------------------- (a2) what the printed sequence implies, per lunar day
print()
print("(a2) per lunar day of the lunation 2025-12-20..2026-01-18: the day the source's printed tshes imply it ends")
print("     on (under the same labelling rule: the last day of the lunation whose printed tshes is <= the lunar day),")
print("     against Genden's, and the range of a constant delta that would put it on the implied day")
for f in ("aginsky-datsan-2025", "aginsky-datsan-2026"):
    rows = [(jd(d), t) for d, t in golden[f] if lo <= jd(d) <= hi]
    base = None
    for N, t in TD["genden"].items():
        if jr.floor(t) >= rows[0][0] and (N - 1) % 30 + 1 >= rows[0][1]:
            base = N - ((N - 1) % 30 + 1)
            break
    ok_lo, ok_hi = F(-10), F(10)
    print(f"  {f} ({jr.date_of(rows[0][0])}..{jr.date_of(rows[-1][0])}):")
    for dnum in range(rows[0][1], 31):
        N = base + dnum
        t = TD["genden"][N]
        imp = [J for J, ts in rows if ts <= dnum]
        if not imp:
            continue
        E = imp[-1]
        if E == rows[-1][0] and dnum > rows[-1][1]:
            continue
        a, b = E - t, E + 1 - t          # delta must lie in [a, b)
        ok_lo, ok_hi = max(ok_lo, a), min(ok_hi, b)
        k = jr.floor(t)
        need = "the same day" if E == k else ("%+.1f min or more earlier" % (float(t - (E + 1)) * 1440) if E < k else "%.1f min or more later" % (float(E - t) * 1440))
        print(f"    lunar day {dnum:2d}: implied {jr.date_of(E)}, Genden {jr.date_of(k)} (fraction {float(t - k):.4f}): {need}; "
              f"delta in [{float(a):+.4f}, {float(b):+.4f})")
    print(f"    intersection of the ranges: {'EMPTY' if ok_lo >= ok_hi else '[%+.4f, %+.4f)' % (float(ok_lo), float(ok_hi))}")
```

`figures.py` (Item 10; the outside computation of every asserted value):

```python
#!/usr/bin/env python3
"""Spec 014 session 2, Item 10: the figures the verdict spec cites, computed OUTSIDE the Rust harness,
from the three golden CSV files and the engine dumps of Item 11 (spec 009's dump probe on a copy of
HEAD; one row per day 1900..2100: gregorian_date,tibetan_year,month,is_leap_month,tshes). The Rust
assertions are written to these values and must agree with them.

usage: python3 -I figures.py <repo root> <dump dir>
"""
import sys
from collections import Counter

root, dumpdir = sys.argv[1], sys.argv[2]
FILES = ["ivolginsky-datsan-2021", "aginsky-datsan-2025", "aginsky-datsan-2026"]
S = ["Phugpa", "Tsurphu", "Genden"]
eng = {}
for s in S:
    rows = open(f"{dumpdir}/engine-{s.lower()}.csv", encoding="utf-8").read().split("\n")[1:]
    rows = [r.split(",") for r in rows if r]
    prev = None
    for d, y, m, lp, t in rows:
        t = int(t)
        step = None if prev is None else (t - prev) % 30
        eng[(s, d)] = dict(year=int(y), month=int(m), leap=lp == "true", tshes=t,
                           doubled=step == 0, skipped=(prev % 30 + 1) if step == 2 else None)
        prev = t
dates_all = sorted({k[1] for k in eng})
idx = {d: i for i, d in enumerate(dates_all)}
gold = {}
for f in FILES:
    lines = open(f"{root}/golden/buryat/{f}.csv", encoding="utf-8").read().split("\n")[1:-1]
    gold[f] = [dict(date=c[0], tshes=int(c[1]), month=c[2], leap=c[3], notes=c[6]) for c in (l.split(",", 6) for l in lines)]

print("(a) totals over the rows; three-way split over the distinct days")
days = sorted({r["date"] for f in FILES for r in gold[f]})
src = {}
for f in FILES:
    for r in gold[f]:
        src.setdefault(r["date"], {})[f] = r["tshes"]
print(f"  rows {sum(len(gold[f]) for f in FILES)} distinct {len(days)}")
for s in S:
    rows = sum(len(gold[f]) for f in FILES)
    match = sum(1 for f in FILES for r in gold[f] if r["tshes"] == eng[(s, r["date"])]["tshes"])
    a = n = sp = 0
    for d in days:
        hits = sum(1 for v in src[d].values() if v == eng[(s, d)]["tshes"])
        if hits == len(src[d]):
            a += 1
        elif hits == 0:
            n += 1
        else:
            sp += 1
    print(f"  {s}: rows {rows} match {match} mismatch {rows - match}; distinct all {a} none {n} split {sp}")

print("(b) Sagaalgan: year/month/leap/tshes and first day of month 1 (not leap)")
for d in ["2021-02-12", "2022-02-02", "2025-03-01", "2026-02-18", "2027-02-07"]:
    cells = []
    for s in S:
        z = eng[(s, d)]
        p = eng[(s, dates_all[idx[d] - 1])]
        first = (p["year"], p["month"], p["leap"]) != (z["year"], z["month"], z["leap"])
        cells.append(f"{s} {z['year']}/{z['month']}{'L' if z['leap'] else ''}/{z['tshes']} skipped={z['skipped']} first_of_1={first and z['month'] == 1 and not z['leap']}")
    print(f"  {d}: " + "; ".join(cells))

print("(c) lunation starts: the source's, and per system where the engine's month starts differ")
for f in FILES:
    g = gold[f]
    st = [0] + [i for i in range(1, len(g)) if g[i]["tshes"] < g[i - 1]["tshes"]]
    sd = [g[i]["date"] for i in st]
    print(f"  {f}: source starts ({len(sd)}): {' '.join(sd)}")
    for s in S:
        ed = [g[0]["date"]] + [g[i]["date"] for i in range(1, len(g))
                               if (eng[(s, g[i]['date'])]['year'], eng[(s, g[i]['date'])]['month'], eng[(s, g[i]['date'])]['leap'])
                               != (eng[(s, g[i - 1]['date'])]['year'], eng[(s, g[i - 1]['date'])]['month'], eng[(s, g[i - 1]['date'])]['leap'])]
        print(f"    {s}: engine months {len(ed)}; only source {[d for d in sd if d not in ed]}; only engine {[d for d in ed if d not in sd]}")

print("(d) the days that separate the systems")
same = sum(1 for d in days if eng[("Phugpa", d)]["tshes"] == eng[("Tsurphu", d)]["tshes"] == eng[("Genden", d)]["tshes"])
print(f"  all three equal on {same} of {len(days)}; at least two differ on {len(days) - same}")
for a, b in (("Phugpa", "Tsurphu"), ("Phugpa", "Genden"), ("Tsurphu", "Genden")):
    dd = [d for d in days if eng[(a, d)]["tshes"] != eng[(b, d)]["tshes"]]
    rows = [(d, f) for d in dd for f in src[d]]
    ea = sum(1 for d, f in rows if src[d][f] == eng[(a, d)]["tshes"])
    eb = sum(1 for d, f in rows if src[d][f] == eng[(b, d)]["tshes"])
    print(f"  {a}-{b}: days {len(dd)} rows {len(rows)} source={a} {ea} source={b} {eb} neither {len(rows) - ea - eb}")

print("(e) lunations: per file, merged; the Genden mismatches with both values and their lunation")
lun = []
for f in FILES:
    g = gold[f]
    st = [0] + [i for i in range(1, len(g)) if g[i]["tshes"] < g[i - 1]["tshes"]]
    en = st[1:] + [len(g)]
    for a, b in zip(st, en):
        lun.append((g[a]["date"], g[b - 1]["date"], f))
lun.sort()
merged = []
for a, b, f in lun:
    if merged and a <= merged[-1][1]:
        merged[-1] = [merged[-1][0], max(b, merged[-1][1]), merged[-1][2] + [f]]
    else:
        merged.append([a, b, [f]])
print(f"  per file {Counter(x[2] for x in lun)}; distinct after merging overlapping date ranges {len(merged)}; "
      f"shared {[m[:2] for m in merged if len(m[2]) > 1]}")
for f in FILES:
    for r in gold[f]:
        e = eng[("Genden", r["date"])]["tshes"]
        if e != r["tshes"]:
            lu = [m for m in lun if m[2] == f and m[0] <= r["date"] <= m[1]][0]
            other = [src[r["date"]][o] for o in FILES if o != f and o in src[r["date"]]]
            print(f"  {f} {r['date']} golden {r['tshes']} Genden {e} lunation {lu[0]}..{lu[1]} other edition {other}")

print("(f) the extra month: engine leap days and source extra days per Aginsky file; offsets per system")
for f in FILES[1:]:
    g = gold[f]
    print(f"  {f}: source extra days {[r['date'] for r in g if r['leap'] == 'true'][:1]}..{[r['date'] for r in g if r['leap'] == 'true'][-1:]} "
          f"({sum(1 for r in g if r['leap'] == 'true')})")
    st = [0] + [i for i in range(1, len(g)) if g[i]["tshes"] < g[i - 1]["tshes"]]
    for s in S:
        lp = [r["date"] for r in g if eng[(s, r["date"])]["leap"]]
        offs = Counter((eng[(s, g[i]["date"])]["month"] - int(g[i]["month"])) % 12 for i in st if g[i]["leap"] != "true")
        print(f"    {s}: engine leap days {lp[:1]}..{lp[-1:]} ({len(lp)}); offsets {dict(offs)}")

print("(g) N3: 2025-12-20..2026-01-18 in the 2025 edition and in Tsurphu and Genden; the 2026 edition's 2026-01-01..18")
for f, lo, hi in (("aginsky-datsan-2025", "2025-12-20", "2026-01-18"), ("aginsky-datsan-2026", "2026-01-01", "2026-01-18")):
    w = [r for r in gold[f] if lo <= r["date"] <= hi]
    ts = [r["tshes"] for r in w]
    steps = Counter((b - a) % 30 for a, b in zip(ts, ts[1:]))
    print(f"  {f} {lo}..{hi}: {len(w)} days, tshes {ts}, steps {dict(steps)}")
    for s in ("Tsurphu", "Genden"):
        e = [eng[(s, r['date'])] for r in w]
        print(f"    {s}: tshes {[x['tshes'] for x in e]}; doubled on {[r['date'] for r, x in zip(w, e) if x['doubled']]}; "
              f"skipped before {[(r['date'], x['skipped']) for r, x in zip(w, e) if x['skipped']]}")
```

## The squash commit message

```
spec 014: the Q1 verdict: Genden, scoped to the three golden/buryat/ files of two datsans (F38 RULED (1))

The Q1 differential of 001 §6.5 and the verdict the human ruled on it. Session 1 put
golden/buryat/ under the gate (D-37 PAID; 12 tests) and built the Q1 harness with its
controls (the T4 fixture: Phugpa 0 differences; the engine's own output; a planted cell
and shift; the pin): over 1,213 T1 rows Genden matches 1,199 (14 mismatches), Tsurphu
1,184, Phugpa 1,145; a blind re-read agreed on 84 of 84 cells; it halted at the verdict
(F38). Session 2 records the ruling as the verdict spec (Status accepted, Resolves F38):
Genden, as the engine computes it (Janson v1 A.3, E1747, a2 = 1/28), within the scope of
the Ivolginsky 2021 calendar and the Aginsky 2025 and 2026 editions, under 001 §6.5's
second rule; the 2026 edition's 8 days recorded as a disagreement between editions, the
2025 edition's 6 as UNEXPLAINED and OPEN, all 14 in the lunation 2025-12-20..2026-01-18;
Tsurphu as A.2 computes it ruled out (14 separating days, source = Genden on 15 of 15
rows), the karana-Sun variant not measured; one extra month recorded (n = 1, D-40);
four reopening conditions. Six tests assert the figures the verdict cites, each shown
red on a plant. D-24's re-run of spec 009's TCG differential: 17 / 13 / 23 days, equal
to D-23's 53. A day-start offset probe finds no offset with fewer than 14 Genden
mismatches. Records: F38 and F4 RULED; F37 RULED (c); D-1, D-37, D-38 PAID; D-19, D-23,
D-24, D-33 corrected in place; D-35 two sentences; D-39, D-40 filed; ROADMAP r6, r7
(q1-verdict LANDED, Gate 1 REACHED); INDEX row 014. The engine and the golden rows are
unchanged. Gate: 180 passed in debug and release (147 + 27 in session 1, + 6 in session 2).

Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
```
