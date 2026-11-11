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
- **Status:** OPEN

### D-10 — Janson v1, page 1, gives "Sunday" for 31 December 2007, which eq. (9.1) makes a Monday
- **Filed:** 2026-11-10, spec 005 Item 5 (the worked examples).
- **Kind:** PROVENANCE
- **Statement:** The date line on page 1 of Janson v1 reads "31 December, 2007 (Sunday 23, month 11, Fire–Pig year)". The date triple agrees with the engine (23rd of month 11, Fire–Pig), but JD 2454466 is a Monday by eq. (9.1), p. 28 with Table 5, and 31 December 2007 was a Monday. The page's other date, 8 January 2014 "Wednesday 8, month 11, Water–Snake year", agrees in every part. So the weekday word on page 1 is not an oracle: no round uses it as a T5 fact until the discrepancy is explained.
- **Pinned by:** `crates/tshes/tests/worked_examples.rs::header_date_2007_weekday_word_disagrees_with_eq_9_1_in_the_paper`
- **TRIGGER:** round 006 (the T5 tables), or any round that takes a weekday from Janson v1, or the day a later version of the paper enters the library.
- **Status:** OPEN
- **2026-11-11, spec 006 Item 8 (TRIGGER named this round):** no file the round writes takes a weekday from Janson v1. MEASURED: a case-insensitive grep for the seven weekday names, "weekday" and "day of week" over `golden/`, `crates/tshes/tests/p5_reference_tables.rs`, `p6_t4_fixture.rs`, `common/fixtures.rs` and `tools/t4-tdc.mjs` finds nothing; the T5 files hold years, Gregorian dates, names, leap-month numbers and tshes, and the T4 fixture has no weekday column (the generator does not read the library's `day` getter). The discrepancy is not explained: D-10 stays OPEN.

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
- **Status:** OPEN

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
- **Status:** OPEN

### D-18 — Rows 6, 7 and 8 of 001 §5.2 are explicitly deferred
- **Filed:** 2026-11-11, spec 007 ADDENDUM 1 (R3; Items 10 and 11).
- **Kind:** COVERAGE
- **Statement:** Rows 6, 7 and 8 of 001 §5.2 are explicitly deferred (001 §10), by the human's ruling on F15–F21. Spec 007's Item 3 table holds the ten lines: FOUND in Janson v1 T1, T2, T3, T5, T6, T9, T10 (Appendix E); NOT IN JANSON V1 T4, T7, T8. The reasons, each with the value measured: no check of any FOUND line exists beyond the paper itself (the T4 library computes none; Item 4 is the paper against itself, METHOD rule 35); the identification of "mewa" and "parkha" with Janson's "number" and "trigram" is INFERRED (the words occur in no library source); Janson v1 calls its trigram numbering "perhaps not traditional" (p. 74); the lunar-day lines depend on the version: T6 and T10 through the month's animal A (Table 24's two columns give a different trigram and a different number on 360 of 360 pairs, Item 10a), T2 through the month's element ((E.6) or (E.7)); a candidate Buryat source labels its months by Janson's Tsurphu rules (27 of 27 animals match Table 24's Tsurphu column, 0 of 27 its Phugpa column; Item 10b; not admitted, not golden data, not a Q1 finding). The round that reopens the rows starts from that table and puts F15–F18's questions again. At 001's next revision, rows 6–8 say "deferred" (with D-16).
- **Pinned by:** not pinned
- **TRIGGER:** the first of: (i) an admitted T1–T3 source prints one of these attributes, or names мэнгэ or парка; (ii) an L2 rule (M4 or later) needs one in its condition; (iii) the Q1 verdict spec is accepted (M3).
- **Status:** OPEN
