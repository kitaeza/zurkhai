# DEBT — what the project owes, and what it must not do yet

Deliberate debt, negative debt (a constraint that must NOT be relaxed until a condition holds), and accepted risks. This is not a bug list (fix those) and not a list of open questions (those are `FORKS.md`). **Filing means an entry here**, with an id and a TRIGGER. A code comment or a spec's prose is not filed (METHOD rule 56).

**Entry shape:**

```
### D-n — <one-line statement>
- **Filed:** <date>, <where it was found>
- **Kind:** CORRECTNESS | PROVENANCE | COVERAGE | NEGATIVE | TOOLING | DOCS | METHOD
- **Statement:** <what is owed or forbidden, and why>
- **Pinned by:** <test name> | not pinned
- **TRIGGER:** <a checkable condition under which the entry is acted on>
- **Status:** OPEN | PAID <date, commit> | RETIRED <date, reason>
```

**How entries are kept:**

- **Numbers.** `D-n` numbers are taken from this file when the entry is filed and are never reused.
- **No deletions.** An entry is never deleted. When it is paid or retired, its `Status:` changes and is dated.
- **Corrections.** A correction is made in place, with `[CORRECTED yyyy-mm-dd: …]`.
- **Length.** An entry stays short. A long account belongs in the spec that found it, cited here.
- **Forks.** A gated finding whose fix shapes are named and none chosen also gets an OPEN row in `FORKS.md`.

---

### D-1 — No client defaults `System` before the Q1 verdict
- **Filed:** 2026-10-02, 001 §4, 003 §3.
- **Kind:** NEGATIVE
- **Statement:** No client, wrapper or rule may default the computation system (Phugpa, Genden, Tsurphu) until the Q1 verdict spec exists. Until then the engine takes `System` as an explicit parameter everywhere. A default chosen earlier would assert an answer to Q1 that only the data can give.
- **Pinned by:** not pinned
- **TRIGGER:** the Q1 verdict spec is accepted (M3).
- **Status:** OPEN

### D-2 — The glossary does not exist
- **Filed:** 2026-10-02, 002 §8.
- **Kind:** DOCS
- **Statement:** 002 §8 requires calendar terms to use the transliteration given in `docs/glossary.md`, one spelling per term. The file does not exist yet, and its spellings must come from sources, not from memory.
- **Pinned by:** not pinned
- **TRIGGER:** the first round that names a calendar term in code (M1) creates it.
- **Status:** PAID 2026-11-10, 4935c36 (spec 005 created `docs/glossary.md`, one spelling per term, each from Janson v1 with its page)

### D-3 — Two naming checks before the first client
- **Filed:** 2026-10-02, 001 §11 (F3).
- **Kind:** PROVENANCE
- **Statement:** (a) A Buryat speaker confirms the spelling of "Хүрдэн". (b) The name is run past a lama when Гунзэчойнэй datsan is approached (F4). If either objects, only the app brand changes.
- **Pinned by:** not pinned
- **TRIGGER:** before the first client is released (M5).
- **Status:** OPEN

### D-4 — 003 §7 names no non-Claude model
- **Filed:** 2026-11-10, spec 004 ADDENDUM 1 (R1, FORKS.md F12).
- **Kind:** METHOD
- **Statement:** 003 §7's model table names only Claude models. Round 004 runs outside that table under a one-round trial (DeepSeek V4 Pro, in CodeWhale). 003's next revision decides the general policy for non-Claude models, with round 004's landing as its evidence.
- **Pinned by:** not pinned
- **TRIGGER:** the next revision of 003.
- **Status:** OPEN

### D-5 — The pin 1.99.0 predates the FFI `bool` miscompilation fix in 1.99.1
- **Filed:** 2026-11-10, spec 004 ADDENDUM 2 (R3, FORKS.md F13).
- **Kind:** TOOLING
- **Statement:** The pinned toolchain, Rust 1.99.0, predates 1.99.1, which fixes "rustc: fix miscompilation of FFI `bool` return values on x86_64" (RELEASES.md, "Version 1.99.1 (2026-10-15)"; FETCHED by the advisor, 2026-11-10). The core has no FFI, so the risk is confined to the future FFI wrapper.
- **Pinned by:** not pinned
- **TRIGGER:** the round that creates crates/tshes-ffi. That round raises the pin to at least 1.99.1, deliberately, in its own PR (002 §2), or it states why not. It also states whether the MSRV moves with the pin.
- **Status:** OPEN

### D-6 — CodeWhale writes session state inside the repository
- **Filed:** 2026-11-10, spec 004 ADDENDUM 1 H4 (observed this round: .codewhale/state/).
- **Kind:** TOOLING
- **Statement:** The CodeWhale harness writes its own session state into the working directory (`.codewhale/state/`: subagents.v1.json, subagents.v1.lock, .gitignore). These files are never committed and never added to the repository's .gitignore (H4); every pre-commit `git status --porcelain` accounts for them.
- **Pinned by:** not pinned
- **TRIGGER:** the next round run in CodeWhale.
- **Status:** OPEN

### D-7 — The gate self-test leg is not in 002 §7's list
- **Filed:** 2026-11-10, spec 004 Item 4 (the self-test leg, Item 5, extends 002 §7).
- **Kind:** DOCS
- **Statement:** tools/gate.sh runs one leg beyond 002 §7's list: the gate self-test (spec 004 Item 5). 002's next revision should list it, so the spec's gate list matches the gate as built.
- **Pinned by:** not pinned
- **TRIGGER:** the next revision of 002.
- **Status:** OPEN

### D-8 — The workflow pins actions/checkout by major-version tag, not by commit SHA
- **Filed:** 2026-11-10, spec 005 Item 1c (an accepted risk that spec 004 stated only in `.github/workflows/ci.yml`'s header comment, METHOD rule 56).
- **Kind:** TOOLING
- **Statement:** `.github/workflows/ci.yml` uses `actions/checkout@v4` in both jobs. A major-version tag is movable: a retagged or compromised `v4` would run in this repository's CI with the job's token. Pinning by full commit SHA (the tag kept in a comment) removes the risk; resolving a tag to a SHA needs a network lookup that round 004's GRANTS did not include. Accepted until the trigger.
- **Pinned by:** not pinned
- **TRIGGER:** before the human's first push of `main` to a public remote, which is the hosted workflow's first run.
- **Status:** OPEN

### D-9 — Genden and Tsurphu are not implemented: `to_zdate` and `to_gregorian` return `Err(SystemNotImplemented)` for them
- **Filed:** 2026-11-10, spec 005 Item 3 (METHOD rule 33: a test that pins a known gap names the entry).
- **Kind:** COVERAGE
- **Statement:** Spec 005 implements Phugpa only. For `System::Genden` and `System::Tsurphu` the engine computes nothing and returns `Err(Error::SystemNotImplemented)`; it never falls back to Phugpa (a fallback would answer 001's Q1 by accident). Out of scope until M2 (001 §10).
- **Pinned by:** `tests/system_not_implemented.rs::genden_not_yet_supported`, `tsurphu_not_yet_supported` (they pin the gap: they go red, deliberately, the day M2 lands, and are replaced by the Genden and Tsurphu tests).
- **TRIGGER:** the round that implements Genden or Tsurphu (M2) replaces the matching test and closes this entry for that system.
- **Status:** PAID 2026-11-11, 8eba6c8 (spec 008), for both systems: Genden and Tsurphu compute through the same API as Phugpa; the two pinning tests are deleted, with a third, `an_unimplemented_system_never_falls_back_to_phugpa`, which asserted the same gap and went red for the same reason; the Genden and Tsurphu tests replace them (`exhaustive_versions.rs`, `p5_versions.rs`, `p7_proptest_versions.rs`, `versions_compared.rs`, and the unit tests in `phugpa.rs`).

### D-10 — Janson v1, page 1, gives "Sunday" for 31 December 2007, which eq. (9.1) makes a Monday
- **Filed:** 2026-11-10, spec 005 Item 5 (the worked examples).
- **Kind:** PROVENANCE
- **Statement:** The date line on page 1 of Janson v1 reads "31 December, 2007 (Sunday 23, month 11, Fire–Pig year)". The date triple agrees with the engine (23rd of month 11, Fire–Pig), but JD 2454466 is a Monday by eq. (9.1), p. 28 with Table 5, and 31 December 2007 was a Monday. The page's other date, 8 January 2014 "Wednesday 8, month 11, Water–Snake year", agrees in every part. So the weekday word on page 1 is not an oracle: no round uses it as a T5 fact until the discrepancy is explained.
- **Pinned by:** `crates/tshes/tests/worked_examples.rs::header_date_2007_weekday_word_disagrees_with_eq_9_1_in_the_paper`
- **TRIGGER:** round 006 (the T5 tables), or any round that takes a weekday from Janson v1, or the day a later version of the paper enters the library.
- **Status:** OPEN
- **2026-11-11, spec 006 Item 8 (TRIGGER named this round):** no file the round writes takes a weekday from Janson v1. MEASURED: a case-insensitive grep for the seven weekday names, "weekday" and "day of week" over `golden/`, `crates/tshes/tests/p5_reference_tables.rs`, `p6_t4_fixture.rs`, `common/fixtures.rs` and `tools/t4-tdc.mjs` finds nothing; the T5 files hold years, Gregorian dates, names, leap-month numbers and tshes, and the T4 fixture has no weekday column (the generator does not read the library's `day` getter). The discrepancy is not explained: D-10 stays OPEN.
- **2026-11-11, spec 008 Item 9 (TRIGGER: "any round that takes a weekday from Janson v1"):** no data file this round writes under `golden/` takes a weekday (the seven new CSVs hold years, months, numbers, kinds, dates and the epoch values of Table 6; MEASURED: a case-insensitive grep for the seven weekday names and "weekday" over them finds nothing). One meta does, in prose: `janson-v1-table6-epochs-806.meta.yaml`'s notes repeat the paper's "Monday 23 March 806 (Julian)" for JD 2015531 (p. 50); by eq. (9.1) (2015531 + 2) mod 7 = 2 = Monday (Table 5), so it agrees, checked in the test named next. One TEST takes the weekdays too: `crates/tshes/src/phugpa.rs::version_tests::the_epoch_dates_and_weekdays_the_paper_prints_are_consistent` takes the weekdays printed beside the epochs in A.2 (Wednesday for JD 2353745, Monday for JD 2397598, p. 39) and A.3 (Sunday for JD 2359237, p. 42) and checks them against eq. (9.1), p. 28 and Table 5: all three agree ((JD + 2) mod 7 = 4, 2, 1). So those three weekdays are not in doubt, and the page-1 discrepancy is neither explained nor widened: D-10 stays OPEN.

### D-11 — `RUSTUP_TOOLCHAIN` in the caller's environment overrides `rust-toolchain.toml`, and the gate does not fail on it
- **Filed:** 2026-11-10, spec 006 Item 0 (R3a), measured this round.
- **Kind:** TOOLING
- **Statement:** MEASURED: `env --chdir=<repository> RUSTUP_TOOLCHAIN=stable rustc -V` prints `rustc 1.97.1 (8bab26f4f 2026-07-14)`, while the same call without the variable prints `rustc 1.99.0 (b940084d7 2026-09-28)`: the environment variable beats the pin. `tools/gate.sh` prints each leg's toolchain but does not fail when it differs from the pin, so a gate run with the variable set passes or fails on the wrong compiler, visibly only to a reader of the toolchain lines.
- **Pinned by:** not pinned
- **TRIGGER:** the next round that changes `tools/gate.sh`.
- **Status:** OPEN

### D-12 — `phugpa::Epoch`, `phugpa::to_zdate` and `phugpa::to_jdn` are public API beyond round 005's prompt
- **Filed:** 2026-11-10, spec 006 Item 0 (R3b; spec 005 report, "Latitude taken", item 1).
- **Kind:** NEGATIVE
- **Statement:** Spec 005 made the three items public so the three-epoch comparison could run through the public API; its prompt did not ask for that surface. The crate must not be published with this surface undecided: a published item is a promise to every later version.
- **Pinned by:** not pinned
- **TRIGGER:** before the crate's first publish.
- **Status:** OPEN
- **2026-11-11, spec 008 Item 9 (Item 3: every public item beyond the two functions and the `System` variants is Latitude):** the round made three more items public: `phugpa::Epoch::E1732` and `phugpa::Epoch::E1852` (the two Tsurphu epochs; the second so that the paper's "the other epoch gives the same calendar" can be tested through the public API) and `phugpa::Epoch::E1747` (Genden's epoch). `phugpa::Epoch`, `phugpa::to_zdate` and `phugpa::to_jdn` now serve all three versions through the epoch's own data, while the module is still named `phugpa` (the base version; Janson presents the other two as differences from it), so the name no longer says what the module does: a decision for the same trigger. The `Epoch` fields stay private; the grain and the rounding shift are private. `Error::SystemNotImplemented` is kept though no function returns it (`fault.rs`).

### D-13 — `tools/t4-tdc.mjs`'s zone checks 2 and 3 have no planted control of their own
- **Filed:** 2026-11-11, spec 006 Item 7 (controls T-a, T-b, T-c; METHOD rule 31).
- **Kind:** COVERAGE
- **Statement:** The generator pins its zone to UTC and then runs three checks (check 1: the resolved zone is `UTC`; check 2: `getTimezoneOffset()` is 0 now; check 3: it is 0 at 12:00 UTC on every enumerated day). Control T-a (the pin deleted, `TZ=Pacific/Kiritimati`) is refused by check 1, which comes first, so it shows that check 1 can fail and says nothing about checks 2 and 3: after check 1 has passed, only a zone named `UTC` with a non-zero offset could reach them. Round 006's GRANTS allowed `node` to run the copies T-a, T-b and T-c only, so no copy that removes check 1 and reaches check 2 or 3 was run. Checks 2 and 3 are untested instruments: they are defence in depth behind check 1, and nothing here proves they can fire.
- **Pinned by:** not pinned
- **TRIGGER:** the next round that runs or changes `tools/t4-tdc.mjs`, or the first round whose GRANTS allow `node` to run a further control copy: it plants one copy per unproven check, with the earlier checks removed, and shows each refuses.
- **Status:** OPEN

### D-14 — 001 §8 and §11 still call the data licence open, and 001 Appendix A's Task 5 names a route the library does not have
- **Filed:** 2026-11-11, spec 006 Item 9 (the grep of 001 and 002 for forks with no ruling, METHOD rule 59) and Item 5.
- **Kind:** DOCS
- **Statement:** (1) 001 §11's F2 line says the data licence "(CC BY 4.0 proposed) is open — decide before the first data commit"; the human ruled it on 2026-11-10 (`FORKS.md` F2: CC BY 4.0) and spec 006 made the first data commit under it, so the line is stale. (2) 001 Appendix A, Task 5, says to generate Lotsawa House output "for a sample of years (or use their free library directly)"; the library's Gregorian → Tibetan route takes only a date string or the clock (`dist/index.js` 739–741), and its dates depend on the zone (`FORKS.md` F14), which Task 5 does not say. 001 is an accepted spec and changes only by revision, so the round does not edit it.
- **Pinned by:** not pinned
- **TRIGGER:** the next revision of 001.
- **Status:** OPEN

### D-15 — Three of round 006's `method:` lines describe a conversion their files do not have, and the second pass cannot be re-run from the tree
- **Filed:** 2026-11-11, spec 007 Item 0 (R1; the advisor's reading RELAYED, 2026-11-11, confirmed against the files).
- **Kind:** DOCS
- **Statement:** MEASURED by reading `golden/reference/janson-v1-table{1,7,8,9}-*.meta.yaml` and the CSVs. (1) The `method:` lines of Tables 7, 8 and 9 repeat Table 1's first-pass sentence, "converted to this CSV by a mechanical awk step (the printed day/month and the year to an ISO date; the name split into element and animal; sorted by year)". Table 1's CSV has dates and names, and its line fits. Table 7's CSV (`year,leap_month`) and Table 8's (`tibetan_year,month,tshes,kind`) hold no date and no name, so neither half of the sentence fits them. Table 9's CSV (`tibetan_year,gregorian_date`) holds dates and no name, so the dates half fits and the names half does not. (2) Spec 006's report, "Item 3 — 3a–b", says "The second-pass commands are the awk lines recorded in each meta's `method:`". Each `method:` describes the second-pass parse in prose ("a second awk script", "the column whose heading's right edge is nearest…") and holds no command; the scripts were scratch and no script is in the tree. So the second pass cannot be re-run from the tree. The data rows are not in question: RELAYED (the advisor, at the 006 landing, 2026-11-11), all 195 T5 rows (120 + 31 + 21 + 23) were re-measured against the advisor's own pdftotext parse and against the T4 fixture.
- **Pinned by:** not pinned
- **TRIGGER:** the next round that writes `golden/reference/`. The fix: the three `method:` lines are corrected in place, each with a `[CORRECTED yyyy-mm-dd: …]` record (GATED.md, `golden/**`); and either the second-pass commands are written into the metas, or the metas say they were not kept. Spec 007 does not make it.
- **Status:** PAID 2026-11-11, spec 008 Item 5g: the three `method:` lines are corrected in place, each with a `[CORRECTED 2026-11-11: …]` record that keeps the original line and says the second-pass commands were not kept (no script is in the tree: MEASURED, `git grep pdftotext` finds only prose). The commit: 705eb37.

### D-16 — 001 §5.2's "Source status" cells disagree with each other and with Janson v1
- **Filed:** 2026-11-11, spec 007 Item 3 (the table, and "001 §5.2's Source status cells").
- **Kind:** DOCS
- **Statement:** MEASURED against Janson v1 (spec 007's Item 3). (1) Year mewa and year parkha are named in two rows each: rows 7 and 8 say TO VERIFY, row 10 says "Janson / Henning". (2) Row 10 names Janson for year parkha; Janson v1 gives no rule for a year's trigram (Appendix E.1, pp. 73–78). (3) Rows 3, 4 and 5 say "verify section"; the sections are §10 (iv), (vi)–(vii) and (viii), pp. 30–31; row 1's cell is "—" though §9, p. 28, defines the weekday and the planet. (4) Row 4's "yoga (27)" is §10's yoga; Janson v1 also calls a different quantity "yoga" (the elemental yoga, p. 83), which a reader of "yoga" must not take for it. 001 is accepted, so it changes only by revision; spec 007 does not edit it.
- **Pinned by:** not pinned
- **TRIGGER:** the next revision of 001.
- **Status:** OPEN

### D-17 — Schuh is named by 003 §4's leg ③ but is in neither 001 §9 nor `docs/sources.md`'s bibliography
- **Filed:** 2026-11-11, spec 007 Item 1.
- **Kind:** DOCS
- **Statement:** 003 §4's leg ③ lists "Janson, Henning, Schuh, Reingold–Dershowitz". 001 §9 and the bibliography of `docs/sources.md` list Janson, Henning, Reingold–Dershowitz, Salmi and Lotsawa House, and no Schuh. Janson v1 says its description "is based mainly on the books by Schuh [12] and Henning [7]" (p. 3) and cites Schuh as refs. [1], [12], [13] and [38] (pp. 84, 86). No work of Schuh is in the paper library. Which of his works the bibliography lists, and what each is trusted for, is for the round that first needs one; Janson's ref. [12] is Dieter Schuh, *Untersuchungen zur Geschichte der tibetischen Kalenderrechnung*, Franz Steiner Verlag, Wiesbaden, 1973.
- **Pinned by:** not pinned
- **TRIGGER:** the next round that edits the bibliography of `docs/sources.md`, or the first round whose leg ③ would read Schuh.
- **Status:** PAID 2026-11-11, 7e3e7c4 (spec 008 Item 1 edited the Henning bibliography row, which fired the TRIGGER, and paid it with one Bibliography row for Janson's ref. [12], "Trusted for: nothing yet", "In the library: no"; no other work of Schuh's is listed). The ref. [12] is on p. 84 of Janson v1 (not p. 86, as the round's prompt said; spec 008 "What the prompt got wrong").

### D-18 — Rows 6, 7 and 8 of 001 §5.2 are explicitly deferred
- **Filed:** 2026-11-11, spec 007 ADDENDUM 1 (R3; Items 10 and 11).
- **Kind:** COVERAGE
- **Statement:** Rows 6, 7 and 8 of 001 §5.2 are explicitly deferred (001 §10), by the human's ruling on F15–F21. Spec 007's Item 3 table holds the ten lines: FOUND in Janson v1 T1, T2, T3, T5, T6, T9, T10 (Appendix E); NOT IN JANSON V1 T4, T7, T8. The reasons, each with the value measured: no check of any FOUND line exists beyond the paper itself (the T4 library computes none; Item 4 is the paper against itself, METHOD rule 35); the identification of "mewa" and "parkha" with Janson's "number" and "trigram" is INFERRED (the words occur in no library source); Janson v1 calls its trigram numbering "perhaps not traditional" (p. 74); the lunar-day lines depend on the version: T6 and T10 through the month's animal A (Table 24's two columns give a different trigram and a different number on 360 of 360 pairs, Item 10a), T2 through the month's element ((E.6) or (E.7)); a candidate Buryat source labels its months by Janson's Tsurphu rules (27 of 27 animals match Table 24's Tsurphu column, 0 of 27 its Phugpa column; Item 10b; not admitted, not golden data, not a Q1 finding). The round that reopens the rows starts from that table and puts F15–F18's questions again. At 001's next revision, rows 6–8 say "deferred" (with D-16).
- **Pinned by:** not pinned
- **TRIGGER:** the first of: (i) an admitted T1–T3 source prints one of these attributes, or names мэнгэ or парка; (ii) an L2 rule (M4 or later) needs one in its condition; (iii) the Q1 verdict spec is accepted (M3).
- **Status:** OPEN

### D-19 — Tsurphu's karana-Sun variant (Janson v1 Remark 22, p. 40) is not implemented
- **Filed:** 2026-11-11, spec 008 Item 9 (R3; `FORKS.md` F23, RULED (a)).
- **Kind:** COVERAGE
- **Statement:** The engine's `System::Tsurphu` is Janson v1's A.2 (pp. 38–40). Remark 22 says some Tsurphu almanacs give the true solar longitude from the karana calculation (Appendix A.5), and in some the true date is calculated with the karana solar equation, so a different Tsurphu. Not implemented, and not a fourth `System`. RELAYED (the advisor, measured 2026-11-11): Henning's published Tsurphu calendars (kalacakra.org) are made by software that takes the Sun's longitude from the karana calculation; his 2012 file differs from Janson v1's Table 8 Tsurphu column in months 4, 5 and 11, with every month length equal. Ruling out A.2 alone does not rule out the variant.
- **Pinned by:** not pinned
- **TRIGGER:** the first of: (i) a Q1 verdict spec would rule Tsurphu out; (ii) the Q1 differential report (M3) finds Tsurphu the best-matching system for any source; (iii) an admitted source names a Karma Kagyu almanac as its basis.
- **Status:** OPEN

### D-20 — Round 008's tests are weaker than its report says
- **Filed:** 2026-11-11, spec 009 Item 0 (the human's ruling R2; the reviewer of 39318f5, RELAYED).
- **Kind:** COVERAGE
- **Statement:** RELAYED from the reviewer's report on 39318f5 (2026-11-11), not re-checked here (R2). (1) In `crates/tshes/tests/p5_versions.rs`, the Genden halves of `seeded_control_table9_one_wrong_date_is_reported` (lines 366–367) and `seeded_control_table7_one_wrong_leap_month_is_reported` (lines 386–387) overwrite a row without first asserting its committed value. `seeded_control_the_other_versions_table_8_is_reported_in_months_9_and_11_only` (lines 413–437) asserts no committed value. Spec 008's report (§1.2) says each seeded control asserts the committed value first. (2) `the_tables_separate_tsurphu_from_genden_by_8_printed_cells` asserts a loaded row count for Table 6 only (`p5_versions.rs:650`). Tables 7 and 9 are compared pairwise, which stops at the shorter file (623–624), and Table 8 is not counted. (3) The c3 test's built-in "swapped" control (`crates/tshes/tests/versions_compared.rs`, lines 151–159) cannot fail once the per-day assertion (line 134) has passed. The c3 test itself can fail: two planted mutations of Tsurphu's epoch turned it red. (4) `tsurphu::a_zdate_of_another_system_is_refused` and `genden::a_zdate_of_another_system_is_refused` (`crates/tshes/tests/exhaustive_versions.rs`, lines 522–550) assert no refusal: they accept a mapped-back Ok or any of three errors. (5) The `PROPTEST_CASES` environment variable replaces the case count configured in `crates/tshes/tests/p7_proptest_versions.rs` (3000, line 266) and in `p7_proptest.rs`. The reviewer measured 10 cases under `PROPTEST_CASES=10`, with every test passing. Neither `tools/gate.sh` nor `.github/` sets the variable.
- **Pinned by:** not pinned
- **TRIGGER:** the next round that edits `crates/tshes/`.
- **Status:** OPEN

### D-21 — Round 008's citations and descriptions disagree with its code in six places
- **Filed:** 2026-11-11, spec 009 Item 0 (the human's ruling R2; the reviewer of 39318f5, RELAYED).
- **Kind:** DOCS
- **Statement:** RELAYED from the reviewer's report on 39318f5 (2026-11-11), not re-checked here (R2). (1) Spec 008's report (§1.1, "The branch-to-row match") says `month_of_count` consumes β, Y₀ and β*. It reads β and Y₀ (`crates/tshes/src/phugpa.rs:463, 469`). β* is read only by `count_of_month` (`phugpa.rs:482`). (2) The changed formula of spec 008's citation row 16, `label_of_day`'s estimate (`phugpa.rs:505`), has no citation in spec 002 §4.8's form. Its comment (`phugpa.rs:498–499`) names eqs. (7.1)–(7.3), with no section, no page and no "Janson (arXiv:1401.6285v1)". The same comment stood at the base 34430fb. (3) `end_day` (`phugpa.rs:452`) now divides by the epoch's grain denominator instead of round 005's fixed one. No row of spec 008's citation table names this change. The line's own citation, eq. (8.1), p. 27 (`phugpa.rs:450`), is unchanged. (4) Spec 008's report (§12, item 6) calls `crates/tshes/tests/p7_proptest_versions.rs` a macro over a system. It is four explicitly named proptests that take a system argument, and two tests that use both systems. Each still runs on the system its name says. (5) `true_date_numerator`'s doc comment (`phugpa.rs:403–404`) says the engine never calls it beyond about t < 10^5. The reviewer measured the largest t reached through the public API: 480,510 for E806 (the same at the base), 136,890 for E1732, 131,310 for E1747. The tested bound, 3·10^9, still holds. (6) The squash commit message of spec 008 (07b5bbf) says "the only branch on the system is lib.rs epoch_of". `to_jdn` also compares a ZDate's system with the epoch's (`phugpa.rs:581`), as spec 008's report §1.1 says in its item (c).
- **Pinned by:** not pinned
- **TRIGGER:** the next round that edits `crates/tshes/`. The fix is in the code's comments and docs; the spec 008 report and its commit message are not edited.
- **Status:** OPEN

### D-22 — Public functions panic on inputs their documentation admits
- **Filed:** 2026-11-11, spec 009 Item 0 (the human's ruling R3; the reviewer of 39318f5, RELAYED).
- **Kind:** CORRECTNESS
- **Statement:** RELAYED from the reviewer's report on 39318f5 (2026-11-11), not re-checked here (R3). The reviewer called each function under `catch_unwind`, with identical results at the base 34430fb and at 39318f5, in debug and in release. `naming.rs` and `arith.rs` did not change between them. `ZYear::from_year(i64::MIN)` and `ZYear::from_year(i64::MIN + 1)` panic (`naming.rs:155`), where the documentation promises `InvalidField` for a year before the first cycle. `arith::amod(i64::MIN, 12)`, `arith::ceil_div(i64::MIN, 1)` and `arith::ceil_div(i64::MIN, 2)` panic, where each is documented for b > 0. The reports of specs 005 and 008 score spec 002 §4.4 ("no panics on valid input") DONE; that does not hold at these inputs. No date-mapping path reaches them: the reviewer counted 0 arithmetic calls from out-of-range input.
- **Pinned by:** not pinned
- **TRIGGER:** the next round that edits `crates/tshes/src/`, or before the crate's first publish, whichever first. Paid by a property test over the whole i64 domain of each public function named, beside the fix.
- **Status:** OPEN

### D-23 — The engine computes a2 = 1/28, Janson v1 (7.13); Henning's (7.24) gives other dates on 53 days of 1900–2100
- **Filed:** 2026-11-11, spec 009 ADDENDUM 1 (the human's ruling R7, on F25).
- **Kind:** COVERAGE
- **Statement:** Janson v1 prints two values of the Moon's anomaly increment per lunar day: a2 = 1/28, eq. (7.13), p. 23, and a2 = 3781/105840, eq. (7.24), p. 25, which Henning uses (Remark 14, pp. 25–26). The engine computes (7.13) for Phugpa, Tsurphu and Genden (`FORKS.md` F25, RULED (a)); (7.24) is not implemented. Under (7.24) the three systems give other Tibetan dates on 17 (Phugpa), 13 (Tsurphu) and 23 (Genden) days of 1900-01-01..2100-12-31, each listed in spec 009's report, §1.2 (MEASURED by spec 009, with TCG 1.06 and with a script from the paper under each value). Within 2021–2026 the lists hold three days: Tsurphu 2022-06-28, Genden 2024-09-19, Phugpa 2025-11-19.
- **Pinned by:** `remark_14_the_two_versions_of_a2_differ_on_the_days_janson_lists` and `remark_14_footnote_27_the_two_versions_differ_on_9_and_8_days` (spec 005's Example I; Phugpa only). MEASURED 2026-11-11 (spec 009 session 2): `git grep -n -E` for the two names, at HEAD under `crates/tshes`, found both, at `crates/tshes/src/phugpa.rs:991` and `:1012`, each on `Epoch::E806`.
- **TRIGGER:** a round that compares any source with the engine on a day of spec 009's lists (the Q1 differential, M3, first). It reports the source's value on that day against the engine's and against the (7.24) value in spec 009's lists. If an admitted source matches the (7.24) value of its best-matching system on such a day, F25 is put to the human again, with a named variant of each system among its options.
- **Status:** OPEN

### D-24 — Tsurphu and Genden have no full-range oracle in the tree
- **Filed:** 2026-11-11, spec 009 ADDENDUM 1 (the human's ruling R6, on F24).
- **Kind:** COVERAGE
- **Statement:** In the gate, the engine's Tsurphu and Genden are checked only against Janson v1's printed tables (P5: Tables 6–9) and against their own consistency (P1–P4, P7); Phugpa also has the T4 fixture over 1900–2100 (P6). Spec 009 compared all three systems with Henning's TCG 1.06, built from its published source (the library's `henning-tcg/tcg1309.zip`), on every day of 1900-01-01..2100-12-31, on the engine of 07b5bbf: equal on every day except the 53 days of D-23. TCG's output is not in the tree (`FORKS.md` F24, RULED (a)). The build, the program's input, the readers and the comparison are in spec 009's report, §21.
- **Pinned by:** not pinned
- **TRIGGER:** the first round after spec 009 that edits `crates/tshes/src/`, or the Q1 verdict spec, whichever comes first. That round runs spec 009's differential again for the three systems over 1900–2100, from §21's record, and reports the counts per system. A disagreeing day outside D-23's lists is a finding.
- **Status:** OPEN
