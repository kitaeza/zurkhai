═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 006 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  006 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
BRANCH:       spec/006-reference-fixtures, cut from main at 0ebaaf8 before any edit; commit to it, the
              halt included. No merge. No push.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7, first branch. The answer is discoverable: every
              fact this round writes is printed in Janson v1 or produced by the T4 library, and every
              comparison is a measurement. Three kinds of error would be silent, and each runs on the
              smaller model only with an instrument that makes it loud (003 §7; docs/ROUNDS.md §8):
                - the transcription of Janson's tables. Instruments: two passes by two different routes
                  (Item 3); the paper checked against itself (Item 3d); a seeded wrong row per file
                  (Item 7a);
                - reading the T4 library's fields. Instruments: the library's own doubled/skipped flags
                  are checked against what consecutive days imply (Item 5); P6 compares every day of the
                  range (Item 6);
                - the §5.3 reading. Instrument: the verdict is quoted with its page (Item 0, R2).
              No change from the last round: round 005 also ran on Sonnet 5.5, effort High.
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first. It never commits a previous session's uncommitted work without
              verifying it: it finishes the work and proves it by the item's own check, or redoes it. A
              design choice that would fall to the fallback model (Opus 5.5, effort High) is a STOP AND
              ASK. The report names the switch point.
PRECONDITION: main at 0ebaaf8 (spec 005 merged: the Phugpa core, tree 4406c74). Anything else: STOP and
              report, with zero diff. Merging is the human's.
              Read first, in this order: CLAUDE.md; docs/ROUNDS.md; docs/specs/001-init-spec.md (§2,
              §5.3, §6, §8, §11, §12 and Appendix A); docs/specs/002-rust-code-standard.md (§4.9, §6,
              §7); docs/specs/003-project-method.md; docs/specs/005-phugpa-core.md (the citation table,
              Item 5's examples, the planted controls, Item 6, the shape census, "Latitude taken");
              docs/METHOD.md, all of Part 1 (these rules bind this round directly: 3, 4, 10, 12, 13, 17,
              23, 25, 27, 31, 33, 35, 36, 39, 41, 47, 48, 50, 51, 56, 58–60, 64–67); GATED.md;
              FORKS.md; DEBT.md; docs/sources.md; docs/glossary.md; tools/gate.sh.
ARTIFACT:     (A) /home/kitaeza/zurkhai-papers/janson-1401.6285v1.pdf. Its sha256 must equal the value in
              docs/sources.md's library row (7cafc7df563a3020849c86f4e397daa18b7f7f5ff46244403180aca90b9d0f77).
              (B) /home/kitaeza/zurkhai-papers/TibetanDateCalcualtor/ (the spelling is the project's own):
              a git clone of gitlab.com/TibetanCalendar/TibetanDateCalcualtor, placed by the human and
              detached at 97d4a004e74551983dfe0f88cd9fbbd2d646b0f1. It must have exactly that HEAD, an
              empty `git status --porcelain`, and a file dist/index.js.
              If either check fails: STOP and report, with zero diff. Never reconstruct a table from
              memory, from secondary pages or from another version, and never fetch the library again.
STOP RULE:    (1) An ARTIFACT check fails: STOP, zero diff (above).
              (2) The engine disagrees with a T5 row whose printed value the second pass confirms, or
              with any T4 row: STOP AND ASK, with every mismatching row listed (date, both values, file,
              source location). Never change a constant, a fixture value, the generator's output or the
              library to make them agree (CLAUDE.md hard rule 7). A mismatch traced to a slip on a line
              of this round's own test or generator code is fixed and reported, as round 005 did; it
              is not a STOP.
              (3) The paper disagrees with itself in one of Item 3d's checks: file a DEBT.md entry
              (Kind: PROVENANCE) and STOP AND ASK which printing the fixture follows.
              (4) A table cell needs a reading the page does not fix, and the plausible readings give
              different rows: STOP AND ASK. Item 3 names one such reading to check.
              (5) Once the generator passes the library explicit date components, its output still
              differs between time zones or between runs: STOP AND ASK.
              (6) The library's own doubled or skipped flags disagree with what consecutive days of its
              output imply: STOP AND ASK. The oracle disagrees with itself.
              (7) Any change under crates/tshes/src/ or to crates/tshes/Cargo.toml seems needed: STOP
              AND ASK. This round writes tests, data, a generator and records, and nothing in the
              engine.
              (8) A read, run, fetch or install outside GRANTS is needed, or a permission is denied:
              STOP and report each denial with its reason. Never work around it by another tool or route
              (METHOD rule 67).
              (9) Out of scope, and not written: the Tsurphu, Mongolia and Bhutan columns of any table;
              Genden or Tsurphu arithmetic; any 001 §5.2 attribute; golden/buryat/; rules/; a `skipped`
              column in any golden file (F9 is OPEN and governs 001 §6.2's golden/buryat format);
              localised names (F5 is OPEN); the INDEX.md status of earlier rounds. If something seems to
              need one of these, report it under "What the prompt got wrong".
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; there is no fourth
              kind. An unobtainable source says PAYWALLED or UNREACHABLE. Read docs/sources.md before any
              fetch; this round needs none. Never curl/wget. (METHOD rule 50.) A quotation from the paper
              in the repository is two sentences at most. Dates, numbers, names and equation numbers are
              facts, and the golden files hold facts only, never a table's layout or prose (001 §2;
              CLAUDE.md hard rule 17).
GATED:        yes:
                - golden/** (new: golden/LICENSE.md, golden/reference/**), under accepted specs 001
                  (§6.1, §6.4, §8, with F2 as ruled below) and 002 (§6);
                - crates/tshes/tests/** only, under 001 §6.4 and 002 §6;
                - this round's own docs/specs/006-reference-fixtures.md and
                  docs/specs/prompts/006-reference-fixtures.md, and its row in docs/specs/INDEX.md,
                  which are not changes to an accepted spec.
              No other gated path. crates/tshes/src/ and crates/tshes/Cargo.toml are not touched.
GRANTS:       Reads outside WORKDIR, beyond this prompt file: /home/kitaeza/zurkhai-papers/ (the human
              adds it with /add-dir). Nothing is written there. The tools:
                - the Read tool, for the PDF's pages and for the library's files;
                - `sha256sum`, on the PDF and on the library's dist/index.js;
                - `pdftotext -layout`, on the PDF, writing only into a directory made by `mktemp -d`.
                  Its output never enters the repository;
                - read-only `git -C /home/kitaeza/zurkhai-papers/TibetanDateCalcualtor`: rev-parse,
                  status, log -1;
                - `node` (the machine's v24.14.0, through nvm), running only this round's own generator
                  (Item 5), which loads the library's dist/index.js by absolute path. Run it with `TZ=`
                  set per run. No `npm`, no `npx`, no install, no build of the library. The human placed
                  the library and consents to this round running its dist/index.js.
              Network: none. Cargo works offline from the lock file and its cache. This round adds no
              dependency, so a fetch is never needed.
              Scratch: planted controls and pdftotext output go in directories made by `mktemp -d`, or
              under target/.
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive
              widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 006 — the reference fixtures: T5 tables from Janson v1, T4 output from the Lotsawa House library, P5 and P6

Round 006 is M1's third round. It covers 001 Appendix A, Tasks 4 and 5, as the human split them on
2026-10-02. It runs now because round 005's engine passes P1–P4 and P7, and those check only
consistency: 005's planted control C (E806.m0 one day late) passed all thirteen exhaustive tests. This
round adds the independent checks:
  - P5: the Phugpa columns of Janson v1's printed tables (T5);
  - P6: every day 1900–2100 from the library that Lotsawa House's calculator links (T4).
Both become fixtures under golden/reference/. It also makes the project's first data commit, under the
data licence the human has just ruled. It does NOT change the engine, transcribe any other system's
column, touch golden/buryat/ or rules/, or decide F9.

THE HUMAN'S RULINGS this round carries (2026-11-10):
  R1. F2, the data half: "1, CC BY 4.0". Golden data and rule tables are licensed CC BY 4.0. The code
      half (MIT OR Apache-2.0, 002 §2) was ruled before.
  R2. At round 005's landing ("agree"): round 005 left 001 §5.3 for the month and year fields "NOT
      DECIDED BY THE PAPER" in its report's prose only. This round settles it (Item 0).
  R3. At round 005's landing ("agree"): two DEBT.md entries are filed (Item 0).

ITEM 0 — Branch, prompt, number, rulings.
  - Create spec/006-reference-fixtures from 0ebaaf8.
  - Commit this file, byte-identical, as docs/specs/prompts/006-reference-fixtures.md. That is the first
    commit. `cmp` it against /home/kitaeza/zurkhai-prompts/spec-006-reference-fixtures.md before every
    later commit.
  - Verify that 006 is free in the tree.
  - Read the Read-first list.
  - Run `date` once, and use that value for every date you write (METHOD rule 51).
  - Choose your round's marker (METHOD rule 58). The report calls it "the round's marker" and never
    spells it.
  - Re-measure the OPEN count in FORKS.md with the grep its header defines. The advisor counted 8 at
    0ebaaf8 (RELAYED: re-measure it, do not copy it).
  - R1, on F2's row. Keep the code-half text. Set `Status:` to RULED. Add the date, the option and the
    human's words "1, CC BY 4.0" to `Ruled:`. Once golden/LICENSE.md exists, add `Built in: spec 006`.
    The expected OPEN count after this edit is the re-measured count minus 1.
  - R2, 001 §5.3 for the month fields (number, leap flag) and the year fields (rabjung cycle, year in
    cycle, element, gender, animal). Re-read Janson v1 p. 20, the passage that begins "The basic rule
    is", including its sentence "In other words, a lunar day gives its name (number and month) to the
    calendar day where the lunar day ends". Re-read eqs. (5.19)–(5.21), p. 17, and the statements
    round 005 quoted from pp. 6 and 20. Then decide, field by field:
      - VERIFIED: give a quotation of two sentences at most, the page, and the chain of reasoning in
        one line (for example: the month comes from the label by p. 20, and the year from the month
        count by (5.21));
      - or still not decided by the paper: file a DEBT.md entry (Kind: PROVENANCE) with a TRIGGER.
    Report it as a table in the same shape as round 005's Item 6.
  - R3, two DEBT.md entries, each in the file's entry shape:
      a. Kind TOOLING. `RUSTUP_TOOLCHAIN` in the caller's environment overrides rust-toolchain.toml.
         tools/gate.sh prints each leg's toolchain but does not fail when it differs from the pin.
         Measure it before filing: run `env RUSTUP_TOOLCHAIN=stable rustc -V` from inside the
         repository (`env --chdir=`), and quote the line it prints. If the pin wins, file nothing and
         report the measured line. TRIGGER: the next round that changes tools/gate.sh.
      b. Kind NEGATIVE. `phugpa::Epoch`, `phugpa::to_zdate` and `phugpa::to_jdn` are public API beyond
         round 005's prompt (005 report, "Latitude taken", item 1). The crate is not published with
         this surface undecided. TRIGGER: before the crate's first publish.
  - These records land in the commit after the prompt, before any fixture.

ITEM 1 — The sources.
  - Verify ARTIFACT (A) and (B) as stated in the header. Record `sha256sum` of dist/index.js and the
    output of `node --version`.
  - Read dist/index.js before you run it. Report every use it makes of the clock (a `Date` built
    without arguments, `Date.now`), of the time zone or locale, of the network and of the file system,
    each with its location in the file. Report also the README's own statements about time zones,
    quoted, two sentences at most.
  - Add a docs/sources.md library row for the library. The row holds:
      - the directory;
      - the citation: package name and version from its package.json, the commit, and the GitLab URL;
      - "Obtained": 2026-11-10, cloned and detached by the human (RELAYED from the advisor);
      - the sha256 of dist/index.js;
      - the licence, as found: the `license` field of package.json, and whether the repository has a
        LICENSE file;
      - "Trusted for": copied from the Bibliography row for Lotsawa House.
    Set that Bibliography row's "In the library" to yes. Record there, as RELAYED (the advisor,
    2026-11-10, FETCHED from lotsawahouse.org/Static/tools/phugpa.html), that the calculator page
    names this repository as its "calendar library". Record as UNVERIFIED whether the live calculator
    runs this commit.
  - The Janson row needs no change. Say so.

ITEM 2 — The licence and the formats (R1; 001 §6.1, §8).
  - golden/LICENSE.md, short. It states:
      - the licence: CC BY 4.0, https://creativecommons.org/licenses/by/4.0/;
      - that each file's provenance and its third-party source are in its sibling .meta.yaml;
      - that the licence covers the project's compilation and grants no right that a source's own
        terms withhold.
    It does not copy the licence's legal text.
  - golden/reference/README.md: the formats below, as the tree's record of them (003 §2: positions live
    in the tree). Every CSV has a header line, UTF-8, LF line endings, ISO 8601 dates, and booleans
    written `true` or `false`. Every CSV has a sibling `<name>.meta.yaml`.
  - The meta fields, one per line:
      tier: T5 | T4
      source: the full citation, with version, or package and version
      location: table and page | file and commit
      sha256: of the PDF | of dist/index.js
      method: the transcription passes and their routes | the generator command, node version, TZ runs
      transcribed_or_generated_by: spec 006, the model as the session reports it
      date: from `date`, this round
      covers: what the file holds completely (for example: "every Phugpa cell of Table 8, months 1–12
              of Tibetan year 2012")
      rights: the source's own terms, as found
      licence: CC BY 4.0
      notes: discrepancies and readings, or "none"
  - The year column of every file is Janson's Y, the year number of eq. (4.1), p. 8. That is the
    Gregorian year in which the Tibetan year begins.
  - Element and animal names use docs/glossary.md's spellings.

ITEM 3 — T5: the Phugpa columns of Janson v1, Tables 1, 7, 8 and 9 (Appendix A, Task 4).
  The four files and their columns:
    golden/reference/janson-v1-table1-losar-phugpa.csv
      tibetan_year
      gregorian_date
      element
      animal
    golden/reference/janson-v1-table9-losar-phugpa.csv
      tibetan_year
      gregorian_date
    golden/reference/janson-v1-table7-leap-months-phugpa.csv
      year
      leap_month          (empty where the Phugpa cell is empty; one row per year the table prints)
    golden/reference/janson-v1-table8-2012-phugpa.csv
      tibetan_year
      month
      tshes
      kind                (repeated | skipped; the table marks skipped days with "-")
  a. First pass: transcribe from the page as the Read tool renders it.
  b. Second pass, by the other route: `pdftotext -layout` into the mktemp directory, compared cell by
     cell against the first pass by a script or a test, not by eye. Every disagreement is settled
     against the rendered page and listed in the report with both readings. No cell is silently fixed
     (001 §6.3).
  c. Readings to state, each with its page:
       - Table 7's year column. The text on p. 51 says "for a range of (Gregorian) years". Check that
         reading it as the Tibetan year Y and reading it as the Gregorian year in which the leap
         month begins give the same rows. If they differ on any row: STOP RULE (4);
       - Table 8's completeness. Every Phugpa cell is transcribed. The meta's `covers:` states the
         months the table covers, so a month with no entry is a claim of none (the test checks it,
         Item 4);
       - which day Tables 1 and 9 give: the first day of the year, as Janson v1 defines it (p. 9 on a
         leap month 1; p. 27, the paragraph "Tibetan New Year"). Cite both.
  d. The paper against itself, computed by the test code from the fixtures and the paper's own
     formulas, never from the engine. Each must agree; if one does not, STOP RULE (3):
       - Table 9's Phugpa column equals Table 1 on every year both print;
       - Table 8's 2012 events against the length of Tibetan year 2012: 30 × 12 − skipped + repeated
         equals the days from Losar 2012 to Losar 2013 by Table 1. The two tables say 2012 has no
         Phugpa leap month (p. 51); check that against Table 7 too;
       - Table 7's leap months against eqs. (5.40)–(5.42), p. 19, computed in the test.

ITEM 4 — P5 (001 §6.4: "Reproduces the system's own published tables (T5) exactly").
  Tests in crates/tshes/tests/, which read the four files through the public API only (`to_zdate`,
  `to_gregorian`, System::Phugpa named explicitly, DEBT.md D-1). Parse with std in the test code. Add no
  dependency.
    - Tables 1 and 9: for each row, the engine's first day of Tibetan year Y equals gregorian_date. For
      Table 1, the engine's element and animal for Y equal the row's.
    - Table 7: for each year printed, the engine's leap month equals the cell, and so does its absence.
    - Table 8: for each month of Tibetan year 2012, the engine's set of repeated tshes and its set of
      skipped tshes equal the file's, including the months the file gives no entry for.
  - A fixture test that loads zero rows fails (002 §6). Each test also asserts the row count it loaded,
    against the count the transcription measured.
  - METHOD rule 25: Tables 1 and 9 decide only the first day of a year. Their tests claim nothing about
    other days. Say so in the report.

ITEM 5 — T4: the generator and the fixture (Appendix A, Task 5; 001 §6.1).
  - tools/t4-tdc.mjs: this round's own script, under the repository's code licence. It takes the
    library's path as its argument, loads dist/index.js, and writes the fixture to stdout. It covers
    every Gregorian day from 1900-01-01 to 2100-12-31, the range 001 §6.4 P6 names. Give the library
    explicit date components, never a date string or the clock.
  - The fixture, golden/reference/tdc-97d4a00-phugpa-1900-2100.csv, with these columns:
      gregorian_date
      tibetan_year
      month
      is_leap_month
      tshes
    It has no doubled or skipped column. Those follow from consecutive rows, and F9 is not this
    round's to decide.
  - The mapping from the library's fields to these columns is read from dist/index.js and its README,
    with locations. That includes the library's year numbering, mapped to Janson's Y. State it in the
    report and in the meta's `notes:`.
  - The library's own flags: in the generator, for every day, check its doubled and skipped flags
    against what the consecutive rows imply. State the library's meaning of each flag, which may
    differ from 001 §5.1's `is_doubled` (the second occurrence). Report the counts. A disagreement is
    STOP RULE (6).
  - Reproducibility: run the generator three times, under TZ=UTC, TZ=Pacific/Kiritimati and
    TZ=Pacific/Pago_Pago. The three outputs must be byte-identical (compare their sha256). Record the
    command and each hash. A difference that survives explicit components is STOP RULE (5).
  - Shape counts in the fixture (METHOD rule 39), each asserted non-zero by the P6 test:
      - leap months;
      - repeated tshes;
      - skipped tshes;
      - 12 → 1 rollovers;
      - rabjung-cycle boundaries.
    Compare them with the engine's census, re-measured this round. Round 005's figures are RECALLED,
    not a baseline.

ITEM 6 — P6 (001 §6.4: "Phugpa reproduces T4 output exactly over at least 1900–2100").
  - A test reads the T4 fixture and asserts, for every row:
      - to_zdate(date, Phugpa) has that year, month, leap flag and tshes;
      - the engine's is_doubled and skipped_before agree with what the consecutive fixture rows imply.
    It asserts the row count, and that the rows are contiguous and cover the range.
  - METHOD rule 35: the library implements Janson's formulas from the same epoch year (its README says
    806). P6 is therefore an agreement between two implementations of one paper. It is not proof that
    either one is right. Say so, and say what P5 adds.

ITEM 7 — The controls (002 §6; METHOD rules 31, 36, 41).
  a. Seeded rows, in test code, one per fixture file (five): one field deliberately wrong, fed through
     the same comparison, asserted to be reported as a mismatch, with the assertion's message named.
  b. Planted controls, on a scratch copy of the committed tree only, never the working tree. Run each
     three times; each must turn the named tests red three runs in three:
       - round 005's plant C (E806.m0 one day late). P5 and P6 must go red; report which tests and
         their first failing row. This is the round's purpose: P1–P4 passed this plant;
       - one row deleted from the T4 fixture: the count and contiguity assertions go red;
       - one Table 8 tshes changed: the Table 8 test and Item 3d's length check go red.
  c. Report a table: control; mutation; expected red tests; observed; verdict.

ITEM 8 — DEBT.md D-10.
  D-10's TRIGGER names this round. Check whether any file this round writes takes a weekday from
  Janson v1. Add a dated line to D-10 saying what you found. It stays OPEN unless the discrepancy is
  explained.

ITEM 9 — The gate, last.
  - Before the final commit, grep 001 and 002 for forks with no ruling and compare them with FORKS.md
    (METHOD rule 59); name the grep. F2 now has its ruling; F9's text names "the first golden row".
    State why this round's files do not decide F9, or STOP AND ASK if you find they do.
  - Then run the gate on the final committed tree, in both profiles, with no edits in flight, as
    round 005 ran it (METHOD rule 65; CLAUDE.md hard rule 12). If the report changes after the gate
    has run, run the gate again on the new tip.
  - The gate line follows docs/ROUNDS.md §4.1: counts, never "green", every skipped leg named. Quote
    the toolchain each leg ran on, from the tool's own output (METHOD rule 23). Re-measure the test
    count at 0ebaaf8 before stating the change (METHOD rule 4).
  - The head block's `golden:` line counts rows added per file.
  - Your row in docs/specs/INDEX.md goes in the final commit, not before (METHOD rule 8). An entry that
    records a commit cites an earlier commit: a commit cannot hold its own hash.

RESEARCH LEGS: not applicable. 003 §4 sets their precondition as Q1, every L2 category and any new
attribute. This round transcribes and generates the oracles 001 §6.1 already names. The report says
so.

PREDICTIONS — written by the advisor before any measurement (METHOD rule 3). Score each HELD or FAILED,
with the measured value.
  1. Both ARTIFACT checks pass.
  2. Row counts. Table 1: 120 (Tibetan years 1927–2046). Table 9: 31 (2000–2030). Table 7: 21 years,
     8 of them with a Phugpa leap month. Table 8: 23 entries, 9 repeated and 14 skipped. The advisor
     counted these off a pdftotext rendering. Re-measure them; do not copy them.
  3. Every check in Item 3d agrees. In particular, Table 8 gives Tibetan year 2012 a length of
     360 − 14 + 9 = 355 days, which equals the days from Losar 2012 to Losar 2013 in Table 1.
  4. Table 7's two year readings give the same rows.
  5. P5: zero mismatches on every T5 file.
  6. The T4 fixture has 73,414 rows, and the three TZ runs are byte-identical.
  7. The library's own doubled and skipped flags agree with consecutive rows on every day.
  8. P6: zero mismatches over the 73,414 days.
  9. R2: VERIFIED for the month fields and for the year fields.
  10. R3a: with RUSTUP_TOOLCHAIN=stable, rustc reports 1.97.1 from inside the repository; the variable
      overrides the pin.
  11. Plant C turns at least one P5 test and the P6 test red, three runs in three.

THE REPORT — per CLAUDE.md's report contract and docs/ROUNDS.md §4, in
docs/specs/006-reference-fixtures.md. It holds:
  - the head block, with its `golden:` line;
  - a report-shaped spec (docs/specs/TEMPLATE.md's header lines: Milestone M1; Governs:
    golden/reference/, golden/LICENSE.md, crates/tshes/tests/, tools/t4-tdc.mjs), then the end marker;
  - then the report.
  Lead with the decisive item: P5 and P6, each pass or fail, with the first mismatch (expected, actual,
  source row) if any. Then:
    - every property P1–P7: DONE / DEFERRED / N-A, with evidence. P1–P4 and P7 are not rebuilt; say
      whether the gate's run re-confirms them;
    - Item 3's transcription: both passes, every disagreement and how it was settled, the readings,
      and Item 3d's checks with their numbers summing;
    - the field mapping and flag meanings of Item 5, with locations in dist/index.js; the three hashes;
    - R2's table; R3's entries, with the measured line for R3a;
    - Item 7's control table;
    - the predictions, scored;
    - D-1's and D-10's states;
    - the open-forks audit, with its arithmetic at every commit;
    - the marker greps, at every commit;
    - every other section of the contract, "What the prompt got wrong" included;
    - the diff scope, with its totals summing against `git diff --stat main` at the final tip,
      including the fixture's size in bytes;
    - the squash commit message (docs/ROUNDS.md §4.2, item 13), naming every FORKS.md and DEBT.md entry
      the round adds or changes.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 006 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════

═══════════════════════════════════════════════════════════════════════════
▼▼▼ ADDENDUM 1 — SPEC 006 — 2026-11-11 — BEGINS HERE. PART OF THE PROMPT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

Appended after the halt at 31457a1 (STOP AND ASK spec 006 Fork 1, FORKS.md F14). Where it differs
from the text above, it governs. MODEL, FALLBACK and the header are unchanged, except GRANTS and
STOP RULE, which gain the lines below. This is a resume: a fresh session on
spec/006-reference-fixtures, Sonnet 5.5, effort High. The ruling removes the design choice the halt
named, so the fallback model is not needed.

THE HUMAN'S RULINGS (2026-11-11, "agree", given at the halt):
  R4. F14 (spec 006 Fork 1): option (a), with the advisor's three additions. The generator sets
      its own zone to UTC before it loads the library. It then checks that the zone holds on every
      day, and refuses to write anything if it does not. Two planted controls show that the check
      and the three-zone runs can fail, and one run names the cause (Items 5 and 7 below).
  R5. The halt's reads outside GRANTS (`pdfinfo` on the PDF; `cat`, `ls -la`, `git ls-files` and
      `git remote -v` on the library): accepted as reported. They stay in "Latitude taken". No
      grant is made for them after the fact, and this session does not repeat them.

ITEM 0 — RESUME.
  - Read this prompt through ADDENDUM 1, then `git log`, then the report so far. Quote
    `git status --porcelain` and `git rev-parse HEAD`. The status is empty and HEAD is 31457a1.
    If either differs: STOP and report, with zero diff.
  - Re-verify ARTIFACT (A) and (B) as the header states them (CLAUDE.md hard rule 10).
  - Run `date` once, at the top of this session, and use that value for every date this session
    writes. Dates the halt's session wrote stay as written. The report states both sessions'
    dates and which records each one covers.
  - Write R4 onto F14's row:
      `Status: RULED`
      `Ruled: 2026-11-11, (a) — the generator pins its own zone to UTC, checks it on every day, and refuses otherwise; the human: "agree" (spec 006 ADDENDUM 1, R4).`
    Once the pinned generator is committed, add `Built in: spec 006 (tools/t4-tdc.mjs)`.
    The expected OPEN count after this edit is 8 − 1 = 7. Re-measure it with the grep FORKS.md
    defines.
  - R2 is still owed. Item 0 placed it before Item 1, and the halt's session deferred it. This
    session's first commit holds R4's row and R2's table, with any DEBT.md entry R2 files. No
    fixture file lands before that commit.

ITEM 5 — CHANGES (R4). Where these differ from Item 5, they govern.
  - The pin. tools/t4-tdc.mjs assigns `process.env.TZ = 'UTC'` before it loads the library and
    before it creates any Date.
  - The check. After the pin and before any output, the generator checks three things:
      - `Intl.DateTimeFormat().resolvedOptions().timeZone` is `UTC`;
      - `new Date().getTimezoneOffset()` is 0;
      - `getTimezoneOffset()` is 0 for a Date at 12:00 UTC on every Gregorian day the generator
        enumerates.
    If any check fails, the generator writes nothing to stdout, names the failed check on stderr,
    and exits non-zero. The check reads the clock's zone only in order to refuse, and nothing
    from it reaches stdout.
  - Explicit components. The generator gives the library explicit Tibetan components (year,
    month, leap flag, day), as the halt's generator does: the library's Gregorian -> Tibetan route
    accepts only a date string or the clock (dist/index.js lines 739–741). The library itself reads
    the clock's zone offset on every conversion (dist/index.js:344). Under the pin that offset is
    0 at every instant, so the clock's value never reaches the output. The report says so.
  - Reproducibility. The three runs, under TZ=UTC, TZ=Pacific/Kiritimati and TZ=Pacific/Pago_Pago,
    each exit 0 and give byte-identical stdout. A difference, or a non-zero exit, is STOP RULE (5).
    These runs now show that the generator does not depend on the caller's zone. They show nothing
    about the library, because the pin hides its zone dependence by construction. The report
    says so.
  - The record. The fixture's meta `notes:` and the report state:
      - the regime: generated under the generator's own TZ=UTC pin;
      - the finding: the library's Gregorian dates depend on the zone it runs in (dist/index.js:344,
        and the local-time getters at lines 72–77). Without the pin, under TZ=Pacific/Kiritimati,
        every date 1900–1994 comes out one day early and 1994-12-31 is missing, as control T-b
        re-measures it;
      - the cause's state after run T-c: NAMED or INFERRED.
  - docs/sources.md. The library row's "Trusted for" adds that the library's dates are trusted
    only as generated under TZ=UTC (F14). In a zone whose offset has changed by about 12 hours
    within 1900–2100, the library gives other dates (spec 006).

ITEM 7 — ADDITIONS (R4). Each control runs on a copy of tools/t4-tdc.mjs, made by `cp` into a
directory made by `mktemp -d` and edited there, never in the working tree. Each copy loads the same
dist/index.js by absolute path. The results join Item 7c's table.
  T-a. The pin's assignment deleted. Run under TZ=Pacific/Kiritimati. Expected: a non-zero exit,
       empty stdout, and stderr naming the failed check. Three runs in three.
  T-b. The pin's assignment and the check both deleted. Run under TZ=Pacific/Kiritimati.
       Expected: the generator's own checks fail, with exit 3 and empty stdout, as at the halt.
       Three runs in three. Quote its first-rows line and its first problem line. This re-measures
       the halt's finding (CLAUDE.md hard rule 10).
  T-c. The cause (METHOD rule 27). The T-b copy, run once under TZ=Etc/GMT-14, a fixed offset of
       +14 hours with no history. If its stdout is byte-identical to the fixture, the cause is
       NAMED: the change in the zone's offset over time, read against today's offset at
       dist/index.js:344, and not the size of the offset. If its stdout differs, the cause stays
       INFERRED, and the meta's note says so. Report either result; neither is a STOP.

GRANTS — ADDITION:
  - `node` may also run the copies T-a, T-b and T-c above, with TZ set per run, TZ=Etc/GMT-14
    included. Nothing else is added.

STOP RULE — ADDITION:
  (10) The check fails in any of the three runs of the pinned generator, or T-a or T-b does not
       behave as expected in any of its runs: STOP AND ASK. Either the instrument cannot fail, or
       the halt's finding did not reproduce.

PREDICTIONS — ADDITIONS, written by the advisor before this session measures anything (METHOD rule
3). Score each HELD or FAILED, with the measured value. Prediction 6 is scored on the pinned
generator's output.
  12. In all three zones, the pinned generator's stdout has the sha256 the halt's report gives for
      its TZ=UTC runs, 8bfd140735bf33b724abe56b0bbb6bb9b63f20164c07cf570622f70f66444ce4
      (RELAYED from that report: re-measure it).
  13. T-a and T-b each behave as expected, three runs in three.
  14. T-c's stdout is byte-identical to the fixture, so the cause is NAMED.

THE REPORT — FIXES TO THE HALT'S DRAFT, made in the same file:
  1. The STOP AND ASK block stays as written, as the round's history. The report records R4 and R5
     below it, and names the point where one session handed over to the next. Both sessions run
     Sonnet 5.5, effort High; the `model:` line gives one entry per session.
  2. "What the prompt got wrong" carries the three errors the advisor names, which the round
     confirms or corrects, together with anything the round finds itself:
       a. Item 5 assumed the library accepts Gregorian date components. Its Gregorian -> Tibetan
          route accepts only a date string or the clock (dist/index.js lines 739–741);
       b. Item 5's three-zone test assumed a library that ignores the zone. And "never the clock"
          cannot hold inside a library that reads it at dist/index.js:344;
       c. Item 1 asked whether the library has a LICENSE file, but GRANTS gave no route to list
          its files.
     Item 1's pointer to that section then resolves.
  3. "Latitude taken" carries the halt's reads outside GRANTS, with R5. It also carries R2's late
     landing (see Item 0 above).
  4. The open-forks arithmetic at every commit of both sessions (METHOD rule 60):
     8 → 7 → 7 → 7 → 8 at the halt → 7 at R4.
  5. The marker greps at every commit of both sessions. Where a grep was not run before a commit,
     say so; do not reconstruct it.
  6. The squash commit message names F14 as raised and RULED, with F2, D-11, D-12 and every entry
     this session adds.
═══════════════════════════════════════════════════════════════════════════
▲▲▲ ADDENDUM 1 — SPEC 006 — ENDS HERE. NOTHING BELOW IS PART OF IT.      ▲▲▲
═══════════════════════════════════════════════════════════════════════════
