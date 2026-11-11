═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 009 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  009 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
BRANCH:       spec/009-henning-differential, cut before any edit from main at the commit PRECONDITION
              names; commit to it, the halt included. No merge. No push.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7, first branch. This half of the round is
              discoverable: it builds one published program from its source, runs it, measures it
              against the engine day by day, and traces each disagreement to its cause. It changes no
              engine code and decides nothing. A differential's disagreements are loud: each one is
              listed. Four parts would be silent, and each runs here only with an instrument that makes
              it loud (docs/ROUNDS.md §8):
                - the port. A Linux build of a program written for Open Watcom on Win32 could compute
                  something its author's build does not. Instrument: the build must reproduce the
                  author's own published output files (Item 2);
                - the reading of the program's output (its month numbering, leap months, years and
                  dates). Instrument: the read output against the paper's printed tables for each
                  version (Item 4);
                - the engine's side. Instrument: the engine's Phugpa dump must equal the T4 fixture on
                  every day (Item 5);
                - the cause named for a disagreement. Instrument: Janson v1's formulas, recomputed by a
                  script of the round's own that is shown able to fail, and a one-change probe that
                  removes exactly the disagreements the cause claims (Item 7; METHOD rules 27–29, 31).
              What Henning's output becomes in the tree, and at which tier, goes to the human at the
              halt (Item 8), so no invented answer is left to this model. No change from the last
              round: round 008 also ran on Sonnet 5.5, effort High.
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first. It never commits a previous session's uncommitted work without
              verifying it: it finishes the work and proves it by the item's own check, or redoes it. A
              design choice that would fall to the fallback model (Opus 5.5, effort High) is a STOP AND
              ASK. The report names the switch point.
PRECONDITION: main at 07b5bbf (spec 008 merged; tree cdc937a95e657ba99e8d2e3dd7f34311d02caab7). Check
              with `git -C /home/kitaeza/github/zurkhai log -1 --format='%h %T %s' main`. Anything else:
              STOP and report, with zero diff. Merging is the human's.
              Read first, in this order, each in full: CLAUDE.md; docs/ROUNDS.md; docs/specs/001-init-spec.md
              (§1, §4, §5.1, §6, §9, §10, §11, §12); docs/specs/003-project-method.md (§4, §7);
              docs/specs/006-reference-fixtures.md (the T4 fixture's section, P6, "What the prompt got
              wrong"); docs/specs/008-genden-tsurphu.md (§1, §2, §4, §12, §15); docs/METHOD.md rules 1, 3,
              11, 18, 25, 27, 28, 29, 31, 32, 35, 36, 39, 50, 51, 58, 59, 60, 64, 65 and 67, each in full,
              not only its **Rule.** paragraph; GATED.md; FORKS.md (F10 and F23 in full, the rest by their
              status lines); DEBT.md (D-1, D-12, D-19, and the entries Item 0 files); docs/sources.md;
              golden/reference/README.md. The report lists each file it read and how much of it (round
              008 read parts of its list by their headings only, and said so; this round reads these in
              full).
ARTIFACT:     (A) /home/kitaeza/zurkhai-papers/janson-1401.6285v1.pdf. Its sha256 must equal
              7cafc7df563a3020849c86f4e397daa18b7f7f5ff46244403180aca90b9d0f77 (docs/sources.md), and
              `pdfinfo` must say 86 pages.
              (B) /home/kitaeza/zurkhai-papers/henning-tcg/: E. Henning, "TCG", Tibetan calendar software
              for grub rtsis, version 1.06 (3 September 2013), source code with a Win32 executable, from
              kalacakra.org/calendar/os_tib.htm. The directory must hold exactly one file, tcg1309.zip:
              93,207 bytes, sha256 8389de8efcd18f7a66a7cef9b6277c222e5193b2acec440f721096a5ad1e83d2.
              `unzip -l` must list 15 files: makefile, bcd.h, bcd.ext, tc.h, tc.ext, tests.c,
              tcgnotes.txt, tc.def, bcd.c, utils.c, t1.c, t2.c, t3.c, t4.c, tcg.exe.
              (C) /home/kitaeza/zurkhai-papers/henning-kalacakra/: 17 files, with the manifest
                `find /home/kitaeza/zurkhai-papers/henning-kalacakra -type f | LC_ALL=C sort | xargs -d '\n' sha256sum | sha256sum`
              equal to 659b9347ae860b6d68ca74ad3482bfe39a439dcb4614232b6c49b654351bacf1 (docs/sources.md's
              row).
              If any check fails: STOP and report, with zero diff. Never reconstruct a program, a formula
              or a table from memory or from another version.
STOP RULE:    (1) An ARTIFACT or PRECONDITION check fails: STOP, zero diff.
              (2) Item 8 is a STOP AND ASK by design. The round halts there, whatever Items 1–7 found.
              What Henning's output becomes in the tree is the human's ruling (R4, below).
              (3) The program cannot be built from ARTIFACT (B)'s source with gcc, unless you edit one of
              Henning's own files (the .c, .h, .def, .ext files and the makefile). Adding files of your own
              (header stand-ins for <conio.h>, <dos.h>, <bios.h>) and choosing compiler flags is allowed.
              If it still fails: STOP AND ASK, with the compiler's errors. Never run tcg.exe, under any
              emulator or compatibility layer, and never use any other build of the program.
              (4) The build check (Item 2) fails on a calendar field: STOP AND ASK, with the differing
              lines. The calendar fields are the year, the month heading, the tshes number and the
              western date. A difference only in another field is listed and carried to Item 8; the round
              continues.
              (5) For any of the three versions, Henning and the engine disagree on more than 300 days of
              1900-01-01..2100-12-31: STOP AND ASK before Item 7. Report the counts per field and the
              first 20 days.
              (6) A disagreement whose cause lies on the engine's side, where the engine departs from
              Janson v1's formulas (category E, Item 7): record it, carry it to Item 8 as its own STOP AND
              ASK, and change nothing under crates/. Items 6 and 7 continue for the other disagreements.
              Never change a constant, on either side, to make the two agree (CLAUDE.md hard rule 7).
              (7) Out of scope; do not write any of it:
                - any change under crates/, golden/, rules/, clients/, tools/, docs/glossary.md,
                  docs/METHOD.md, GATED.md or an accepted spec. This half writes records and a report;
                - Henning's other systems (Error Correction, mkhas pa'i snying nor), his karana-Sun
                  Tsurphu program (os_tsurphu.htm, tdata/ts_2012.txt, tiblist2.htm; DEBT.md D-19), and
                  his program's astrological output (planets, mansions, elements, earth-lords) beyond
                  noting that it exists;
                - golden/buryat/, every datsan holding in the library, the Q1 diff, and any statement
                  about which system a Buryat calendar follows;
                - deciding any fork, the tier of Henning's output included.
              If something seems to need one of these, write it down for Item 8, and do not do it.
              (8) A read, a run, a fetch or an install outside GRANTS is needed, or a permission is
              denied: STOP and report each, with the reason. Never work around it by another tool or
              route (METHOD rule 67).
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; there is no fourth
              kind. An unobtainable source says PAYWALLED or UNREACHABLE. Read docs/sources.md before any
              fetch; this round fetches nothing. Never curl/wget. (METHOD rule 50.) A quotation from a
              source in the repository is two sentences at most. Henning's source code is cited by
              file:line and described; it is never copied into the repository, even in part (CLAUDE.md
              hard rule 17), although its licence would allow it.
GATED:        no gated path, except this round's own docs/specs/009-henning-differential.md and
              docs/specs/prompts/009-henning-differential.md, which are not changes to an accepted spec.
              The round's row in docs/specs/INDEX.md lands in its final commit, after the resume, not at
              the halt. Nothing under crates/ or golden/ is touched. Not gated, and changed only as the
              items say: DEBT.md, FORKS.md, docs/sources.md.
GRANTS:       Reads outside WORKDIR, beyond this prompt file: /home/kitaeza/zurkhai-papers/ (the human adds
              it with /add-dir). In it, read only: ARTIFACT (A); ARTIFACT (B), only through an extraction
              into a `mktemp -d` directory; and, in henning-kalacakra/, epochs.htm, os_tib.htm and the six
              files tdata/pl_1900.txt, pl_1950.txt, pl_2000.txt, pl_2012.txt, pl_2025.txt, pl_2049.txt.
              Open nothing else there, and write nothing there. The tools:
                - the Read tool, on the PDF's pages, on the files above, and on ARTIFACT (B)'s
                  extraction;
                - `sha256sum`, `pdfinfo`, `unzip -l`, and the manifest command of ARTIFACT (C);
                - `unzip`, only into a `mktemp -d` directory;
                - `pdftotext -layout`, on the PDF, writing only into a `mktemp -d` directory;
                - `gcc` (or `cc`), compiling Henning's source and your own header stand-ins, only inside
                  `mktemp -d` directories; and running the program built there, only there, with its
                  input from a file or a pipe;
                - `python3 -I`, running only scripts you write into a `mktemp -d` directory. They may read
                  the files above, the extraction, the program's output, golden/reference/, the PDF's
                  pdftotext output and the engine dumps of Item 5;
                - the engine: `git -C /home/kitaeza/github/zurkhai archive HEAD | tar -x -C <dir>` into a
                  `mktemp -d` directory; your own probe test files added to that copy only; then
                  `cargo +1.99.0 test --offline --manifest-path <dir>/Cargo.toml …` on that copy, in debug
                  and in release, with CARGO_TARGET_DIR inside a `mktemp -d` directory. Never in the
                  working tree;
                - read-only shell tools on the repository and on your own scratch: cat, head, tail,
                  sed -n, grep, awk, cut, tr, sort, uniq, wc, diff, cmp, od, ls, stat, find, date, file;
                - mkdir -p, cp, mv and rm -r, only inside `mktemp -d` directories.
              Files in the repository are edited only with the Edit and Write tools, never by a script or
              a shell redirection (the human's ruling R1). Network: none. Cargo works offline from the lock
              file and its cache. On this machine `grep` is ugrep: a bounded repetition of 100 or more,
              such as `.\{0,120\}`, fails with "exceeds complexity limits"; keep bounds below 100.
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive
              widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 009 — The Henning differential

Round 009 runs a second, independent implementation of the three systems the engine computes. That
implementation is E. Henning's TCG program, version 1.06, built from its published source. The round
compares TCG with the engine on every day from 1900-01-01 to 2100-12-31: Generalised Phugpa, Generalised
Tsurphu and New Genden, against the engine's Phugpa, Tsurphu and Genden. Every disagreement is listed and
traced to its cause. It runs now because M3, the Q1 verdict, will compare Buryat calendars with all three
of the engine's systems. So far Janson v1's tables check Tsurphu and Genden only on a few years, and on
8 printed cells between the two (spec 008). This half halts by design at Item 8 and builds nothing: what
Henning's output becomes in the tree, and its tier, is the human's. It does NOT change the engine, read
any Buryat source, or say anything about Buryat practice.

THE HUMAN'S RULINGS this round carries (2026-11-11):
  R1. Round 008's "Latitude taken", items 5, 6, 7 and 11, are accepted as reported, with no grant after
      the fact. They were: (5) python3 scripts used to edit repository files, beyond its GRANTS; (6) five
      Bash calls that began with `cd`; (7) the first transcription pass typed after the pdftotext text had
      been seen; (11) parts of the Read-first list read by their headings only. The human: "Agree with all
      recommendations". From this round on, repository files are edited only with the Edit and Write
      tools (GRANTS), and METHOD rules 31 and 36 are explicit checks on this round's own controls (Items
      2, 4, 5, 6 and 7): each control shows its instrument can fail, and reports exactly the seeded
      difference and no other. Record R1 in Item 0's account. Round 008 is closed, and its file is not edited.
  R2. Round 008 was read at its close by a read-only reviewer (Opus 5.5), at commit 39318f5 (docs/ROUNDS.md
      §10). Its findings on round 008's tests and report are filed as two DEBT.md entries in Item 0, with
      the TRIGGER "the next round that edits crates/tshes/". The 008 file is not edited. The human: "Agree
      with all recommendations". The findings are quoted in Item 0 as the reviewer reported them, RELAYED.
      This round does not read the reviewer's report, which stays outside the tree, and does not re-check
      the findings: its TRIGGER does not fire here, since nothing under crates/ changes.
  R3. A defect older than round 008, which the same reviewer found: some public functions panic on inputs
      their documentation admits. It is filed in Item 0 as a CORRECTNESS entry, with the TRIGGER "the next
      round that edits crates/tshes/src/, or before the crate's first publish, whichever first", to be paid
      with a property test over the whole i64 domain of each function. The human: "Agree with all
      recommendations".
  R4. This round itself. Earlier on 2026-11-11 the human ruled ("Agree with all recommendations", the
      advisor's Decision 2, reshaped): a round after 008 and before M3 runs Henning's siddhanta software
      for New Genden, Generalised Tsurphu and Phugpa over 1900–2100. It is a DIFFERENTIAL, listing and
      settling every disagreement, not an exact fixture. Its tier is asked in this round: 001 §6.1 limits
      T4 to "Engine vs Phugpa only" and gives T5 to "Tables printed in Janson / Henning" (Item 8).
  R5. No reviewer runs at this halt: this half changes neither crates/ nor golden/ (docs/ADVISOR.md §6).
      After your halt commit the branch stands still (docs/ROUNDS.md §3).

ITEM 0 — Branch, prompt, number, records.
  - Check PRECONDITION and ARTIFACT, and quote every output.
  - Create spec/009-henning-differential from main's tip.
  - Commit this file, byte-identical, as docs/specs/prompts/009-henning-differential.md. That is the
    first commit. `cmp` it against /home/kitaeza/zurkhai-prompts/spec-009-henning-differential.md before
    every later commit.
  - Verify that 009 is free in the tree.
  - Read the Read-first list, in full, and list it in the report.
  - Run `date` once and use that value for every date you write (METHOD rule 51). If a later `date`
    shows a new day, the records written after it carry the new date, and the report says where.
  - Choose your round's marker (METHOD rule 58). The report calls it "the round's marker" and never
    spells it.
  - Re-measure the OPEN count in FORKS.md with the grep its header defines. The advisor counted 6 on
    main (RELAYED: re-measure it, do not copy it).
  - The gate does not run in this half: it changes no tracked source (docs/ROUNDS.md §3; METHOD rule
    65). The head block's `gate:` line says "not run: halt by design, no tracked source changed".
  - File three DEBT.md entries, in the register's shape, numbered from the next free id (D-20 at 07b5bbf,
    RELAYED: verify it). Write each field below as given. A statement may be reworded for the register's
    style, but no item may be dropped, and no file:line or number may change. All three are RELAYED from
    the reviewer's report on 39318f5 (2026-11-11). Do not re-check them: R2 and R3.

    Entry one:
      title: Round 008's tests are weaker than its report says
      Filed: <your date>, spec 009 Item 0 (the human's ruling R2; the reviewer of 39318f5, RELAYED)
      Kind: COVERAGE
      Statement, item 1: in crates/tshes/tests/p5_versions.rs, the Genden halves of
        `seeded_control_table9_one_wrong_date_is_reported` (lines 366–367) and
        `seeded_control_table7_one_wrong_leap_month_is_reported` (lines 386–387) overwrite a row without
        first asserting its committed value. `seeded_control_the_other_versions_table_8_is_reported_in_months_9_and_11_only`
        (lines 413–437) asserts no committed value. Spec 008's report (§1.2) says each seeded control
        asserts the committed value first.
      Statement, item 2: `the_tables_separate_tsurphu_from_genden_by_8_printed_cells` asserts a loaded row
        count for Table 6 only (p5_versions.rs:650). Tables 7 and 9 are compared pairwise, which stops at
        the shorter file (623–624), and Table 8 is not counted.
      Statement, item 3: the c3 test's built-in "swapped" control (crates/tshes/tests/versions_compared.rs,
        lines 151–159) cannot fail once the per-day assertion (line 134) has passed. The c3 test itself
        can fail: two planted mutations of Tsurphu's epoch turned it red.
      Statement, item 4: `tsurphu::a_zdate_of_another_system_is_refused` and
        `genden::a_zdate_of_another_system_is_refused` (crates/tshes/tests/exhaustive_versions.rs,
        lines 522–550) assert no refusal: they accept a mapped-back Ok or any of three errors.
      Statement, item 5: the PROPTEST_CASES environment variable replaces the case count configured in
        crates/tshes/tests/p7_proptest_versions.rs (3000, line 266) and in p7_proptest.rs. The reviewer
        measured 10 cases under PROPTEST_CASES=10, with every test passing. Neither tools/gate.sh nor
        .github/ sets the variable.
      Pinned by: not pinned
      TRIGGER: the next round that edits crates/tshes/.
      Status: OPEN

    Entry two:
      title: Round 008's citations and descriptions disagree with its code in six places
      Filed: <your date>, spec 009 Item 0 (the human's ruling R2; the reviewer of 39318f5, RELAYED)
      Kind: DOCS
      Statement, item 1: spec 008's report (§1.1, "The branch-to-row match") says `month_of_count`
        consumes β, Y₀ and β*. It reads β and Y₀ (crates/tshes/src/phugpa.rs:463, 469). β* is read only by
        `count_of_month` (phugpa.rs:482).
      Statement, item 2: the changed formula of spec 008's citation row 16, `label_of_day`'s estimate
        (phugpa.rs:505), has no citation in spec 002 §4.8's form. Its comment (phugpa.rs:498–499) names
        eqs. (7.1)–(7.3), with no section, no page and no "Janson (arXiv:1401.6285v1)". The same comment
        stood at the base 34430fb.
      Statement, item 3: `end_day` (phugpa.rs:452) now divides by the epoch's grain denominator instead
        of round 005's fixed one. No row of spec 008's citation table names this change. The line's own
        citation, eq. (8.1), p. 27 (phugpa.rs:450), is unchanged.
      Statement, item 4: spec 008's report (§12, item 6) calls crates/tshes/tests/p7_proptest_versions.rs a
        macro over a system. It is four explicitly named proptests that take a system argument, and two
        tests that use both systems. Each still runs on the system its name says.
      Statement, item 5: `true_date_numerator`'s doc comment (phugpa.rs:403–404) says the engine never
        calls it beyond about t < 10^5. The reviewer measured the largest t reached through the public
        API: 480,510 for E806 (the same at the base), 136,890 for E1732, 131,310 for E1747. The tested
        bound, 3·10^9, still holds.
      Statement, item 6: the squash commit message of spec 008 (07b5bbf) says "the only branch on the
        system is lib.rs epoch_of". `to_jdn` also compares a ZDate's system with the epoch's
        (phugpa.rs:581), as spec 008's report §1.1 says in its item (c).
      Pinned by: not pinned
      TRIGGER: the next round that edits crates/tshes/. The fix is in the code's comments and docs; the
        spec 008 report and its commit message are not edited.
      Status: OPEN

    Entry three:
      title: Public functions panic on inputs their documentation admits
      Filed: <your date>, spec 009 Item 0 (the human's ruling R3; the reviewer of 39318f5, RELAYED)
      Kind: CORRECTNESS
      Statement: the reviewer called each function under catch_unwind, with identical results at the base
        34430fb and at 39318f5, in debug and in release. naming.rs and arith.rs did not change between
        them. `ZYear::from_year(i64::MIN)` and `ZYear::from_year(i64::MIN + 1)` panic (naming.rs:155),
        where the documentation promises InvalidField for a year before the first cycle.
        `arith::amod(i64::MIN, 12)`, `arith::ceil_div(i64::MIN, 1)` and `arith::ceil_div(i64::MIN, 2)`
        panic, where each is documented for b > 0. The reports of specs 005 and 008 score spec 002
        §4.4 ("no panics on valid input") DONE; that does not hold at these inputs. No date-mapping path
        reaches them: the reviewer counted 0 arithmetic calls from out-of-range input.
      Pinned by: not pinned
      TRIGGER: the next round that edits crates/tshes/src/, or before the crate's first publish, whichever
        first. Paid by a property test over the whole i64 domain of each public function named, beside
        the fix.
      Status: OPEN

  - The records commit holds the three entries and R1's account. OPEN does not change (no fork is
    filed here).

ITEM 1 — The holding, and what the program computes.
  - docs/sources.md, in the commit that first cites ARTIFACT (B): one library row for henning-tcg/, in
    the table's column shape:
      - the source: E. Henning, TCG, version 1.06, 3 September 2013 (the date tcgnotes.txt gives; check
        it), with its contents as `unzip -l` lists them;
      - "Obtained": fetched by the advisor on 2026-11-11 from http://www.kalacakra.org/calendar/tcg1309.zip,
        over plain HTTP, because the site's HTTPS certificate does not match its host name, and placed
        in the library by the human (RELAYED);
      - the zip's sha256 and size;
      - "Licence": as the source files state it. Quote at most two sentences, and name the files that
        carry it;
      - "Trusted for": a second implementation of Generalised Phugpa, Generalised Tsurphu and New Genden,
        for the spec 009 differential only. No tier and no golden row until the human rules at the halt.
        tcg.exe is never run.
  - In the same commit, the henning-kalacakra/ row's "Trusted for" cell gains one sentence: the six
    tdata/pl_*.txt files are Henning's own output of his program, used to check a build of it (spec 009,
    Item 2), not as an oracle of the engine. Change nothing else in that row.
  - Read tcgnotes.txt and os_tib.htm. State in the report, each with file:line or a quotation:
      a. the menu entries for the three versions this round uses;
      b. how "Print calendar" is asked for a range of years, and the file it writes;
      c. which Tsurphu the program computes. Janson v1's A.2 takes the true date from the siddhanta Sun.
         Remark 22 (p. 40) describes almanacs that take it from the karana calculation (D-19). Henning's
         os_tib.htm says recent Tsurphu calendars' Sun is implemented in a different program. Establish
         from TCG's source (file:line) which Sun its Tsurphu true date uses. If it is the karana one: STOP
         AND ASK, because then TCG's Tsurphu is not A.2's;
      d. where the source holds each version's epoch data, and the arithmetic it computes in (its BCD
         and radix routines; any floating point, and what it is used for).

ITEM 2 — The build, and its check (STOP RULES (3), (4)).
  - Extract ARTIFACT (B) into a `mktemp -d` directory. Write your own header stand-ins there, as
    separate files, for whatever <conio.h>, <dos.h> and <bios.h> provide that the source uses. Change
    none of Henning's files: check with `cmp` against a fresh extraction after the build.
  - The original is a 32-bit Win32 program built by Open Watcom with the flags in its makefile. State the
    data model and the compiler flags you build with, and every compiler warning, by count and kind.
    Build it twice, with -O0 and with -O2. Every output file of every run in this round is produced by
    both builds and compared; a difference is a finding (METHOD rule 32's question, asked of this
    program).
  - THE BUILD CHECK (METHOD rule 64: a built artifact's provenance is verified before its result is
    trusted). Run "Print calendar" with the Generalised Phugpa epoch for the six years 1900, 1950, 2000,
    2012, 2025 and 2049. Compare each file with Henning's published tdata/pl_<year>.txt, after
    normalising line endings and trailing spaces only. Report per year: lines, equal lines, and every
    differing line, classed as a calendar field or another field (STOP RULE (4)).
  - METHOD rules 31 and 36 (prove the instrument can fail, and prove the assertion): the comparison
    script reports exactly the two seeded differences on a copy of one published file with one tshes and
    one western date changed, and exactly the missing line on a copy with one line removed.
  - Record the build commands, the stand-in headers (your own code) and the input you feed the program,
    verbatim, in the report's last section, so that the build can be re-run from the tree.

ITEM 3 — The epochs, before any day is compared.
  For each version, take the epoch data TCG uses (Item 1d): Generalised Phugpa and Generalised Tsurphu at
  their −1000 epochs, and New Genden at 1747. Carry them by exact rational arithmetic, with the mean
  motions of Janson v1 (7.2), (7.3), (7.6), (7.7), (7.12), (7.13), to Janson v1's own epochs: E806 (§7,
  (7.4), (7.8), (7.14)), E1732 (A.2, (A.3)–(A.5)) and E1747 (A.3, (A.11)–(A.13)), and to Table 6's JD
  2015531. Compare digit for digit: m₀, s₀, a₀, and the true-month constant β* (or the form TCG holds it
  in). Also compare the mean motions TCG uses with Janson's. Report a table: version; quantity; TCG
  (file:line, as written); Janson v1 (equation, page); carried value; verdict. This item says whether a
  later disagreement can come from the constants. It does not stop the round: a constant that differs is
  carried to Items 7 and 8.

ITEM 4 — Run, read, and check the reading.
  - Run "Print calendar" for each version, over enough Tibetan years to cover every day from 1900-01-01
    to 2100-12-31, with both builds.
  - Read the output into one row per Gregorian day, in the T4 fixture's columns: gregorian_date,
    tibetan_year, month, is_leap_month, tshes (golden/reference/README.md). Doubled and skipped tshes
    follow from consecutive rows, as in the T4 file. State the mapping from TCG's fields to these
    columns, each with its evidence (tcgnotes.txt, os_tib.htm or file:line): the year, the month heading,
    how a leap month is marked, the tshes, and the western date.
  - Checks on TCG's own output, per version: 73,414 rows; one per day; no gap; tshes steps of 0, 1 or 2
    between consecutive days; months of 29 or 30 days; and skipped − doubled = 30 × (complete months) −
    (days in them), shown summing.
  - THE READING CHECK, against the paper (the T5 files of golden/reference/, by the files' own
    columns): for each version, TCG's leap months against Table 7 (2000–2020), its repeated and skipped
    tshes of Tibetan year 2012 against Table 8, and its New Year days against Table 9 (2000–2030); and,
    for Phugpa, against Table 1. A mismatch is first a reading slip: fix the reading and say so. A
    mismatch that survives a correct reading is a finding (TCG against Janson's table), listed with both
    values and carried to Item 8.
  - METHOD rules 31 and 36: the reading check reports exactly two mismatches when one leap month and
    one New Year are changed in a copy of TCG's read output.

ITEM 5 — The engine's side.
  - Check that the engine you dump is main's: `git -C /home/kitaeza/github/zurkhai diff --stat main HEAD
    -- crates Cargo.toml Cargo.lock rust-toolchain.toml` prints nothing.
  - On a `git archive` copy of HEAD (GRANTS), add a probe test of your own. Through the public API only
    (`to_zdate` with System::Phugpa, System::Tsurphu and System::Genden named explicitly; DEBT.md D-1),
    it writes one row per day of 1900-01-01..=2100-12-31 for each system, in the same five columns.
    Run it in debug and in release; the two outputs must be byte-identical.
  - THE DUMP CHECK: the Phugpa dump equals golden/reference/tdc-97d4a00-phugpa-1900-2100.csv on all
    73,414 rows (P6, spec 006). It is the control that the dump reads the engine correctly. If it fails,
    the dump is wrong, not the engine: fix it before Item 6. METHOD rules 31 and 36: the dump check
    reports exactly one mismatch when one row of a copy of the dump has its tshes changed.
  - The probe stays in the copy. Record it verbatim in the report's last section.

ITEM 6 — THE DIFFERENTIAL (the decisive item, with Item 7).
  For each version, compare TCG's rows with the engine's, day by day, over 1900-01-01..2100-12-31.
  - Per field, the count of days that differ, out of 73,414. Then the number of disagreeing days, the
    runs of consecutive disagreeing days, and the length of the longest run.
  - Per Tibetan year 1899..2100: does the set of months (year, month, leap) agree; does the leap month
    agree; does the first day of the year agree? Report the counts, and every year that differs.
  - THE LIST: every disagreeing day, one line each: date; engine (year, month, leap, tshes); TCG (the
    same); whether either calendar doubles or skips a tshes on that day or the day next to it.
  - For Phugpa, also TCG against the T4 fixture directly. It must give the same list as against the
    engine, because the dump check passed; say so.
  - METHOD rule 25 (a differential is a check only where both sides can see the evidence): say which
    fields a disagreement can show and which it cannot. Neither side prints the true date in these rows.
  - METHOD rules 31 and 36: the comparison reports a seeded disagreement (one day's tshes changed in a
    copy of one side), and only that one.
  - RELAYED, not a premise: the advisor compared Henning's published Phugpa lists (the six years of Item
    2) with the T4 fixture on 2026-11-11. They differed on 2 of 2,159 days, 2001-02-10 and 2025-11-19,
    each at a doubled tshes. State whether those two days are in your Phugpa list. Draw no conclusion
    from this paragraph.

ITEM 7 — Settling each disagreement (METHOD rules 27, 28, 29, 31, 35).
  A disagreement is SETTLED only by a measured answer to two questions.
  (i) Which side follows Janson v1? Write your own script, from the paper only and never from the engine's
      code or TCG's, that computes in exact rational arithmetic, for any version and any lunar day, the
      quantities the day's label depends on: the true month count and the leap rule (§5, (5.1)–(5.10),
      with the version's changes, A.2 and A.3), the mean date, mean sun, anomaly and true date (§7,
      (7.1)–(7.22)), and the day's label (§8, (8.1)).
      Before you trust it (METHOD rule 31):
        - it reproduces the T5 files' Table 8 for all three versions;
        - it reproduces the engine's dump on 30 days you list before running it, including doubled and
          skipped days of each version;
        - with one constant seeded wrong, it fails both of these.
      Then, for each disagreeing day, compute the true dates of the lunar days that decide its label.
      Report each side's label and the script's. Report the true date's distance from the day boundary
      that decides the label, in the true date's own units.
  (ii) Where do the two computations part? Find the place: file:line in TCG's source, or, on the engine's
      side, file:line in crates/tshes/src/ (you may read the engine's code; you change none of it).
      Confirm it with a probe that changes exactly one thing, on a copy of the side concerned (METHOD
      rule 27). Report which disagreements the probe removes, and everything else it changes that you
      can measure (rule 28). A cause settles exactly the disagreements its probe removes. A probe that
      varies a suspected cause and moves nothing eliminates that cause; write the cause beside the run
      (rule 29).
  Class each disagreement by what (i) and (ii) measured:
    H  TCG departs from Janson v1's formulas, at a place in its source that the probe confirms;
    E  the engine departs from Janson v1's formulas (STOP RULE (6));
    C  both sides compute the same quantities as the script, and differ by a rule Janson v1 does not
       state (name the rule, and where each side applies it);
    U  not settled: name the instrument that would settle it.
  Report a table: cause; class; disagreements it settles (count, and the list's dates); probe; what
  else the probe changed. Every disagreement of Item 6 is in exactly one row; the counts sum to Item 6's.
  No cause is named by the run that reproduces a disagreement, only by the run that removes it (rule 27).
  The advisor offers no mechanism for any disagreement.

ITEM 8 — The halt, by design (STOP RULE (2)).
  - One STOP AND ASK per fork, in docs/ROUNDS.md §5's shape, with its headings verbatim, in your response
    and in the spec file. File each as an OPEN row in FORKS.md, in the same commit, numbered from the
    next free F-number (F24 at 07b5bbf, RELAYED: verify it). One kind per question: a fork that spans two
    kinds of decision is two forks.
  - At least these questions are put. Each becomes a fork, or, where a rule, a spec or a ruling already
    decides it, a forced answer with that rule named (CLAUDE.md hard rule 4):
      a. What TCG's output becomes in the tree: nothing beyond this report; a fixture under
         golden/reference/, and for which versions; or a re-runnable tool under tools/ that rebuilds TCG
         from the library and runs the differential. Give each option's real cost. For a fixture, quote
         what 001 §6.1 says as written: T4 is "Engine vs Phugpa only"; T5 is "Tables printed in Janson /
         Henning". Whether TCG's output built in this round, or Henning's own published lists, can be
         either tier is the question, not yours to answer. Say what each reading would need (an amendment
         of 001 §6.1 is a change to an accepted spec).
      b. Each class E disagreement: its own fork, because it is an engine defect. The fix is a later
         round's; the options include "a fix round before M3".
      c. Each class U disagreement: settle it in a further round with the instrument named, or record it
         and go on.
      d. Each class C rule: which rule the engine keeps, and whether a record is needed (a DEBT entry or a
         glossary line in the resume).
      e. Each finding of Items 2–4 that the round carried forward: TCG against the paper's tables; a
         constant that differs (Item 3); a non-calendar field of the build check.
    Class H disagreements need no fork: the engine follows the paper, and CLAUDE.md hard rule 7 forbids
    changing it to agree with TCG. Say so, and list them.
  - IF THE ANSWER IS DELAYED: nothing. The round has ended by design.
  - Before the halt commit: grep 001 and 002 for forks with no ruling, and compare them with FORKS.md
    (METHOD rule 59); name the grep. The open-forks arithmetic: expected OPEN = the count re-measured in
    Item 0 + the forks raised (METHOD rule 60); then the grep.

RESEARCH LEGS: not applicable. 003 §4 sets their precondition as Q1, every L2 rule category and any new
attribute. This round measures an existing computation against a second implementation. Leg ② of 003
§4 names "Henning's software" for the rounds that need a field leg; this is not one of them. The report
says so.

PREDICTIONS — written by the advisor on 2026-11-11, before this round measures anything (METHOD rule 3).
Their basis: the advisor listed the zip's contents, read tcgnotes.txt's first page and the licence lines
of the source files, counted the DOS-only calls (`getch` only, in four files), and did not build or run
the program. It compared Henning's published Phugpa lists with the T4 fixture (Item 6's paragraph). All
of this is RELAYED, and none of it is a premise. Score each prediction HELD or FAILED, with the measured
value. A prediction scored HELD cites your own measurement, never this prompt.
  1. Every ARTIFACT check passes.
  2. TCG builds with gcc from Henning's files unchanged, with header stand-ins only.
  3. The build check passes on every line of all six years, not only on the calendar fields, in both
     builds; the -O0 and -O2 builds give identical output everywhere.
  4. TCG's Tsurphu true date uses the siddhanta Sun, as Janson v1's A.2 does (Item 1c).
  5. TCG's epoch data and mean motions, carried to E806, E1732 and E1747, equal Janson v1's on every
     digit (Item 3).
  6. TCG's read output passes the reading check for all three versions: Tables 7, 8 and 9, and Table 1.
  7. Phugpa: TCG and the engine disagree on between 10 and 200 of 73,414 days, and 2001-02-10 and
     2025-11-19 are among them.
  8. Tsurphu and Genden: each disagrees with the engine on between 10 and 200 days.
  9. In every version, the set of months and the leap months agree in all 202 Tibetan years. The first
     day of the year differs in at most 3 years per version.
  10. Every disagreeing day is in a run of at most 2 days, beside a doubled or skipped tshes in at least
      one of the two calendars.
  11. No disagreement is class E or U.
  12. One cause settles every Phugpa disagreement, and the same cause settles every Tsurphu and Genden
      disagreement.
  13. Item 8 raises between 1 and 4 forks, and OPEN at the halt commit is 6 + the forks raised.

THE REPORT — per CLAUDE.md's report contract and docs/ROUNDS.md §4, in
docs/specs/009-henning-differential.md. It holds:
  - the head block, without a `golden:` line, and with the `gate:` line of Item 0;
  - a report-shaped spec (docs/specs/TEMPLATE.md's header lines: Status: draft — halted by design at
    Item 8; Milestone: M2, its cross-check before M3; Governs: nothing built in this half), then the end
    marker;
  - then the report.
  Lead with the decisive item: Item 6's counts and Item 7's table, with the counts summing. Then:
    - Item 1's statements (a–d); Item 2's build, warnings and build check; Item 3's table; Item 4's
      mapping, checks and reading check; Item 5's dump check;
    - every METHOD rule 31 control of Items 2, 4, 6 and 7: what was seeded, what was expected, what was
      observed;
    - the predictions, scored;
    - R1's record and the three DEBT entries of Item 0, by id;
    - the open-forks audit, with its arithmetic at every commit;
    - the marker greps, at every commit;
    - every other section of the contract, "What the prompt got wrong" included;
    - the diff scope, with its totals summing against `git diff --stat main`, stating the commit it was
      measured at;
    - every STOP AND ASK;
    - last: the commands, your header stand-ins, the program's input, the probe and the scripts,
      verbatim. Henning's source is cited by file:line, never copied.
  The squash commit message is written by the resume, not at the halt.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 009 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════

═══════════════════════════════════════════════════════════════════════════
▼▼▼ ADDENDUM 1 — SPEC 009 — 2026-11-11 — BEGINS HERE. PART OF THE PROMPT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

Appended after the halt by design at 6248886 (Item 8: STOP AND ASKs spec 009 Forks 1 and 2, FORKS.md
F24 and F25). Where it differs from the text above, it governs. This is a resume: a fresh session on
spec/009-henning-differential. The header is unchanged, except MODEL's reason, ARTIFACT, GRANTS, STOP
RULE and PATHS, which change as below. The round builds nothing: the human ruled that TCG's output
stays in the report and that the engine keeps its a2. This session writes the rulings onto the
registers, removes from the report what a report must not carry, completes the report, runs the gate
once, and closes the round.

MODEL — unchanged: Sonnet 5.5, effort High. Reason: 003 §7, first branch. The rulings below decide both
forks. What is left is discoverable: register entries whose fields are given here, a check of the
report against Henning's source, the report's completion, and one gate run. Two parts would be silent,
and each has an instrument here:
  - a register entry that misstates a ruling: its fields are given below, to be written as given
    (Item 10);
  - a report that still copies Henning's code after the fix: a mechanical check against the source,
    whose control is that it finds the copies in the halt's version of the report (Item 11).
Nothing is built, so no design choice falls to this model. No change from the halt's session.

THE HUMAN'S RULINGS (2026-11-11). At the halt the advisor sent its recommendations; the human answered
"Agree with all recommendations".
  R6. F24 (spec 009 Fork 1): (a) — nothing beyond this round's report. TCG's output gets no tier and no
      golden row, and no tool under tools/ is written. Whether Henning's published lists, or output of
      software built from his source, could be T5 under 001 §6.1 stays undecided: no fixture needs the
      answer. One DEBT.md entry (COVERAGE) records that Tsurphu and Genden have no full-range oracle in
      the tree, and when the differential is run again (Item 10).
  R7. F25 (spec 009 Fork 2): (a) — the engine computes a2 = 1/28, Janson v1 eq. (7.13), for Phugpa,
      Tsurphu and Genden; (7.24) is not implemented. One DEBT.md entry (COVERAGE) names the 53 days by
      reference to this report's §1.2, with a TRIGGER at the Q1 differential (Item 10). The advisor's
      reasons, RELAYED: (1) 001 §6.4's P6 requires Phugpa to reproduce the T4 output exactly over
      1900–2100, and the T4 fixture equals the engine's (7.13) dump on every day (this report, §8);
      (2) Janson v1 A.2 (p. 38) and A.3 (p. 42) give Tsurphu and Genden the Phugpa values of the mean
      motions, a2 included, each with "see Remark 14"; (3) Remark 14 (pp. 25–26) calls (7.13) the
      value usually used when calculating calendars. Verify the A.2 and A.3 sentences on their pages and
      cite them FETCHED.
  R8. Session 1's "Latitude taken" items 1 (env --chdir), 5 (the seeded control chosen after eight
      trials), 6 (rm -r outside a mktemp -d directory), 7 (four Bash calls that began with cd) and 9
      (the sources.md row one commit late) are accepted as reported. No grant is made after the fact.
      Items 2, 3, 4, 8 and 10 are choices inside GRANTS and the items, and need no ruling.

THE ADVISOR'S CORRECTIONS TO THIS PROMPT (2026-11-11; not rulings):
  C1. Item 7's classes, and Item 8's line "Class H disagreements need no fork", assumed that Janson v1
      prints one formula for each quantity. For a2 it prints two. Session 1 was right to raise F25; its
      §18 says so, and stands.
  C2. Prediction 5 was ambiguous as written: Janson v1 prints two values of a2, and TCG's equals (7.24)
      on every digit. The advisor meant the values the engine carries; under that reading it FAILED, as
      session 1 scored it. The ambiguity is the advisor's defect.
  C3. Items 2 and 5 asked for the commands and the probe "verbatim" in the report's last section, and
      Item 7 for probes. For a probe of Henning's code, "verbatim" collides with CITATIONS ("never
      copied into the repository, even in part"). For a probe of the engine, it puts the engine's
      implementation into a report, which a round's report never carries (docs/ADVISOR.md §6: verdicts,
      test names, assertion text, file:line, commands and their output; never quoted implementation).
      Item 11 removes both. This round's own dump probe (Item 5's test file, through the public API)
      stays verbatim: D-24's re-run needs it.

ARTIFACT — this session re-verifies (A) and (B) as the header states them. (C) is not used.

GRANTS — REPLACED for this session (METHOD rule 67). Reads outside WORKDIR, beyond this prompt file:
/home/kitaeza/zurkhai-papers/ (the human adds it with /add-dir). In it, read only ARTIFACT (A), and
ARTIFACT (B) only through an extraction into a `mktemp -d` directory. Write nothing there. The tools:
  - the Read tool, on the PDF's pages, on the extraction, and on the repository;
  - `sha256sum`, `pdfinfo` and `unzip -l`, for the ARTIFACT checks;
  - `unzip`, only into a `mktemp -d` directory;
  - `pdftotext -layout`, on the PDF, writing only into a `mktemp -d` directory (R7's two sentences);
  - `awk`, writing only into a `mktemp -d` directory (Item 11's check);
  - read-only shell tools on the repository and on that directory: cat, head, tail, sed -n, grep, cut,
    tr, sort, uniq, wc, diff, cmp, ls, stat, find, date, file;
  - `git -C /home/kitaeza/github/zurkhai`: log, show, diff, status, rev-parse, grep, add, commit;
  - /home/kitaeza/github/zurkhai/tools/gate.sh, run once with no arguments (Item 13), and the cargo
    commands it runs;
  - mkdir -p, cp, mv and rm -r, only inside `mktemp -d` directories.
This session builds and runs nothing else: no gcc, no TCG, no python3, no cargo outside gate.sh.
Repository files are edited only with the Edit and Write tools (R1). Network: none. On this machine
`grep` is ugrep: keep any bounded repetition below 100.

STOP RULE — ADDITION:
  (9) Item 9's status or HEAD check fails, or an ARTIFACT check fails: STOP and report, with zero diff.
  (10) Item 11's check, run on the halt's report, does not list the line of §21.3's P-TCG block that
       copies t2.c:637: the check is wrong. Fix it, and run it again before trusting any count it
       gives. Not a STOP.
  (11) Any gate leg fails (Item 13): STOP AND ASK. No tracked source has changed since 07b5bbf, so a
       failure is pre-existing or environmental, and which one is not this round's to decide.
  (12) Out of scope, beyond STOP RULE (7): any change to the engine's a2; any fixture or tool made from
       TCG's output; any change to 001 §6.1; the paper library's datsan holdings. A next free id that
       differs from the one given here is not a STOP: use the free one, and report it under "What the
       prompt got wrong".

PATHS — ADDITION: a local hook may reject a Bash call that runs cd. Rewrite the call with absolute
paths; never route around the hook (METHOD rule 67).

ITEM 9 — RESUME.
  - Read this prompt through ADDENDUM 1, then `git log`, then the report so far. Quote
    `git status --porcelain` and `git rev-parse HEAD`. The status is empty and HEAD is
    62488863abc2ca86482a6bd32ac2b0551e9107a8. If either differs: STOP RULE (9).
  - Re-verify ARTIFACT (A) and (B) (CLAUDE.md hard rule 10).
  - Run `date` once, at the top of this session, and use that value for every date this session
    writes. Dates the halt's session wrote stay as written. The rulings' date is the human's,
    2026-11-11. The report states both sessions' dates and which records each covers.
  - Choose this session's marker (METHOD rule 58). The report calls it "session 2's marker" and never
    spells it. The marker greps are reported per commit, for both sessions.
  - Re-measure the OPEN count with the grep FORKS.md defines. The halt measured 8 (RELAYED:
    re-measure it).
  - docs/ROUNDS.md §3 asks a resume to run the workspace's tests before it builds anything. This
    session builds nothing, and no tracked source has changed since 07b5bbf: the gate runs once, at
    Item 13.
  - Commit at every item boundary: Items 10, 11 and 12 one commit each; Item 13 ends with the final
    commit.

ITEM 10 — THE RULINGS ONTO THE REGISTERS (R6, R7). One commit: FORKS.md, DEBT.md, docs/sources.md and
the report's account.
  - FORKS.md, F24: its Status line becomes RULED, and the row gains, in the register's row shape:
      Ruled: 2026-11-11, (a) — nothing beyond spec 009's report: no tier, no golden row, no tool; DEBT.md D-24 names when the differential is run again; the human: "Agree with all recommendations" (spec 009 ADDENDUM 1, R6).
  - FORKS.md, F25: its Status line becomes RULED, and the row gains:
      Ruled: 2026-11-11, (a) — a2 = 1/28, Janson v1 (7.13), for Phugpa, Tsurphu and Genden; the 53 days on which (7.24) gives other dates are DEBT.md D-23; the human: "Agree with all recommendations" (spec 009 ADDENDUM 1, R7). The advisor's reasons, RELAYED: <R7's three reasons, the A.2 and A.3 sentences cited FETCHED>.
    Neither row gains a `Built in:` line: nothing is built.
  - DEBT.md: two entries in the register's shape, numbered from the next free id (D-23 at 6248886,
    RELAYED: verify it). Write each field below as given. A statement may be reworded for the
    register's style, but no number, date, equation, page or test name may change.

    Entry one:
      title: The engine computes a2 = 1/28, Janson v1 (7.13); Henning's (7.24) gives other dates on 53 days of 1900–2100
      Filed: <your date>, spec 009 ADDENDUM 1 (the human's ruling R7, on F25)
      Kind: COVERAGE
      Statement: Janson v1 prints two values of the Moon's anomaly increment per lunar day: a2 = 1/28,
        eq. (7.13), p. 23, and a2 = 3781/105840, eq. (7.24), p. 25, which Henning uses (Remark 14,
        pp. 25–26). The engine computes (7.13) for Phugpa, Tsurphu and Genden (F25, RULED (a)); (7.24)
        is not implemented. Under (7.24) the three systems give other Tibetan dates on 17 (Phugpa), 13
        (Tsurphu) and 23 (Genden) days of 1900-01-01..2100-12-31, each listed in spec 009's report,
        §1.2 (MEASURED by spec 009, with TCG 1.06 and with a script from the paper under each value).
        Within 2021–2026 the lists hold three days: Tsurphu 2022-06-28, Genden 2024-09-19, Phugpa
        2025-11-19.
      Pinned by: `remark_14_the_two_versions_of_a2_differ_on_the_days_janson_lists` and
        `remark_14_footnote_27_the_two_versions_differ_on_9_and_8_days` (spec 005's Example I; Phugpa
        only), if a grep under crates/tshes/ at HEAD finds both names; otherwise "not pinned". The
        report says which, with the grep.
      TRIGGER: a round that compares any source with the engine on a day of spec 009's lists (the Q1
        differential, M3, first). It reports the source's value on that day against the engine's and
        against the (7.24) value in spec 009's lists. If an admitted source matches the (7.24) value of
        its best-matching system on such a day, F25 is put to the human again, with a named variant of
        each system among its options.
      Status: OPEN

    Entry two:
      title: Tsurphu and Genden have no full-range oracle in the tree
      Filed: <your date>, spec 009 ADDENDUM 1 (the human's ruling R6, on F24)
      Kind: COVERAGE
      Statement: In the gate, the engine's Tsurphu and Genden are checked only against Janson v1's
        printed tables (P5: Tables 6–9) and against their own consistency (P1–P4, P7); Phugpa also has
        the T4 fixture over 1900–2100 (P6). Spec 009 compared all three systems with Henning's TCG 1.06,
        built from its published source (the library's henning-tcg/tcg1309.zip), on every day of
        1900-01-01..2100-12-31, on the engine of 07b5bbf: equal on every day except the 53 days of
        D-23. TCG's output is not in the tree (F24, RULED (a)). The build, the program's input, the
        readers and the comparison are in spec 009's report, §21.
      Pinned by: not pinned
      TRIGGER: the first round after spec 009 that edits crates/tshes/src/, or the Q1 verdict spec,
        whichever comes first. That round runs spec 009's differential again for the three systems over
        1900–2100, from §21's record, and reports the counts per system. A disagreeing day outside
        D-23's lists is a finding.
      Status: OPEN

  - docs/sources.md, the henning-tcg/ row's "Trusted for" cell. Replace the words
      for the spec 009 differential only. No tier and no golden row until the human rules at the halt (spec 009 Item 8).
    with
      for the spec 009 differential and its re-runs (`DEBT.md` D-24) only. No tier and no golden row (`FORKS.md` F24, RULED (a), 2026-11-11).
    Keep the cell's last sentence, on tcg.exe. Change nothing else in the file.
  - OPEN after this commit: 8 − 2 = 6. Grep it before the commit (METHOD rule 60).

ITEM 11 — THE REPORT: WHAT IT MUST NOT CARRY (C3; CITATIONS). One commit: the report only.
  - THE CHECK. Its steps, with $T a `mktemp -d` directory and $X ARTIFACT (B)'s extraction in another:
      1. Patterns, from the 13 text files of the program (t1.c, t2.c, t3.c, t4.c, utils.c, tests.c,
         bcd.c, tc.h, bcd.h, tc.ext, bcd.ext, tc.def, makefile): every line with its carriage return,
         spaces and tabs deleted and one trailing semicolon removed, kept if 12 characters or longer,
         sorted and made unique:
           awk '{sub(/\r$/,""); gsub(/[ \t]/,""); sub(/;$/,""); if (length($0)>=12) print}' <the 13 files> | LC_ALL=C sort -u > $T/patterns.txt
      2. The report with spaces and tabs deleted, line for line, so line numbers are kept:
           awk '{gsub(/[ \t]/,""); print}' <the report> > $T/report-nows.md
      3. grep -n -F -f $T/patterns.txt $T/report-nows.md
    Run it on the halt's report (`git -C /home/kitaeza/github/zurkhai show 6248886:docs/specs/009-henning-differential.md`,
    written into $T), and later on the working file. Its blind spots: a copy shorter than a whole
    source line, and a copied line that was then changed. The list of places below names the two such
    places the advisor found.
  - The control (METHOD rule 31; STOP RULE (10)): on the halt's report, the check lists line 694, the
    line of §21.3's P-TCG block that copies t2.c:637.
  - Classify every line the check lists, on the halt's report and again after the fix:
      (a) a statement or expression of Henning's code: an assignment, a call with its arguments, a
          format string, a line of a diff → replaced by a description with file:line;
      (b) an identifier or a numeric value cited with file:line, as Item 3 asked ("TCG (file:line, as
          written)") → stays;
      (c) the licence notice, quoted under Item 1 (two sentences at most), and the program's menu text
          as tcgnotes.txt documents it → stays;
      (d) a generic C line that is not particular to Henning's program, such as an #include of a
          standard header in this round's own stand-ins → stays.
  - The class (a) places the advisor found at 6248886 (RELAYED: confirm each, and add any others you
    find): §1.2 (ii), line 133 (t2.c:637, and t2.c:623); §4 c, line 190 (a call in t4.c's day loop);
    §5, line 198 (the statement at t4.c:700); §19, line 574 (the format string of t4.c:700); §21.3,
    lines 692–697 (the P-TCG diff, both of its lines); §21.5's epochs.py, lines 1608–1609 (a print
    string that copies t2.c:623 and a term of t2.c:637). Replace each with a description and
    file:line. For the epochs.py string, the report says that one string of the script as recorded
    differs from the script that ran, and how.
  - §21.3's P-ENG block: replace the diff of crates/tshes/src/phugpa.rs with a description and
    file:line (§1.2's probe table already gives one). Item 5's dump probe stays verbatim (C3).
  - The report's two statements that Henning's source is "never copied" (the heading of §4, and the
    opening of §21) were untrue at the halt. Each gets a `[CORRECTED yyyy-mm-dd: …]` record that says
    what was removed and where, without quoting it.
  - After the fix, run the check on the working file: every line it lists is in class (b), (c) or
    (d), and no place of the list above remains (read each one).
  - Report the check's commands, its counts by class before and after the fix, and the control's
    result.

ITEM 12 — THE REPORT: SESSION 2, in the same file. One commit.
  1. The head block: `model:` gives one entry per session; `gate:` waits for Item 13. The header
     lines: `Status: draft — the round is complete; status is set to accepted at landing`;
     `Resolves: F24 and F25 (raised by this round; RULED (a) each)`; `Governs: nothing built (F24 (a),
     F25 (a))`.
  2. The STOP AND ASK blocks stay as written, as the round's history. Fork 1's DONE AND GREEN says "0
     mismatches in 283 cells", where §7 and §17 count 341 table rows. Below that block, a
     `[CORRECTED …]` record gives the measured count and what 283 counted. If 283 cannot be derived
     from the round's measurements, the record says so.
  3. A "Session 2" section records R6–R8 and C1–C3, the point where one session handed over to the
     next, and Items 9–13.
  4. Prediction 5: C2 is recorded beside its score, which stays FAILED.
  5. "Latitude taken": R8, item by item.
  6. "What the prompt got wrong": anything this addendum gets wrong, and anything this session finds.
  7. The predictions below, scored.
  8. The open-forks arithmetic at every commit of both sessions (METHOD rule 60): 6 at the prompt
     commit … 8 at the halt, then each of this session's commits, ending at 6.
  9. The marker greps at every commit of both sessions. Where a grep was not run before a commit,
     say so; do not reconstruct it.
  10. The diff scope at the final commit (`git diff --stat main`), its totals summing, the commit it
      was measured at named.
  11. A section headed `## The squash commit message`, holding one fenced block (docs/ROUNDS.md §9
      extracts it). The message names: TCG 1.06 built from its source and checked against Henning's
      six published years; the differential's counts (17, 13, 23; 53 in all) and its one cause (a2,
      (7.13) against (7.24)); that nothing under crates/ or golden/ changed; F24 and F25, raised and
      RULED (a); D-20, D-21, D-22, D-23 and D-24 filed; docs/sources.md's henning-tcg/ row and the
      henning-kalacakra/ sentence. It quotes no code. It ends with the Co-Authored-By trailer of each
      model that committed (two sessions on one model give one line).

ITEM 13 — THE GATE AND THE CLOSE.
  - After Item 12's commit, with nothing in flight, run /home/kitaeza/github/zurkhai/tools/gate.sh
    once, with no arguments (docs/ROUNDS.md §3; METHOD rule 65). No tracked source has changed since
    07b5bbf; the run shows that the final tree is green in both profiles (CLAUDE.md hard rule 12).
    Record the commit it ran on, each leg's result, and the counts. A failing leg is STOP RULE (11).
  - The final commit holds the head block's `gate:` line, the docs/specs/INDEX.md row, the diff
    scope, the audits and the squash message. The INDEX.md row, as one line:
      row: | 009 | `009-henning-differential.md` | The Henning differential: TCG 1.06 against the engine's three systems, 1900–2100 | draft |
  - Before the final commit, show that it changes nothing the gate reads. Both commands print
    nothing:
      git -C /home/kitaeza/github/zurkhai diff --stat <the gated commit> HEAD -- crates golden tools Cargo.toml Cargo.lock rust-toolchain.toml deny.toml
      git -C /home/kitaeza/github/zurkhai diff --cached --stat -- crates golden tools Cargo.toml Cargo.lock rust-toolchain.toml deny.toml
    The report records both commands and their empty output, so the gate's result holds for the
    tip.
  - After the final commit the branch stands still (docs/ROUNDS.md §3).

PREDICTIONS — ADDITIONS, written by the advisor on 2026-11-11, before this session measures anything
(METHOD rule 3). Their basis is the advisor's own measurements in its session: Item 11's check, run
on the halt's report with the library's extraction; Janson v1's pp. 23–26, 38 and 42 through
pdftotext. They are RELAYED, and none is a premise. Score each HELD or FAILED, with the measured value.
  14. ARTIFACT (A) and (B) pass, and HEAD is 6248886.
  15. After Item 10, FORKS.md has 6 OPEN rows, and D-23 and D-24 are the ids filed.
  16. Item 11's check, on the halt's report, lists 12 lines: 133, 184, 186, 190, 198, 652, 653, 694,
      1570, 1572, 1574 and 1608. Of these, 133, 190, 198, 694 and 1608 are class (a).
  17. After Item 11, the check lists 7 lines, none in class (a): one licence line, one line of menu
      text, two #include lines and three lines of epochs.py's values.
  18. Every gate leg that runs passes with round 008's counts: 147 passed, 0 failed (debug) and 147
      passed, 0 failed (release), with wasm-pack skipped (RELAYED from round 008's head block:
      re-measure it).

THE REPORT — ADDITION: this session's report is complete (CLAUDE.md's report contract: "Every
session's report is complete, a resumed session's included"), in the same file, as Item 12 orders it.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ADDENDUM 1 — SPEC 009 — ENDS HERE. NOTHING BELOW IS PART OF IT.      ▲▲▲
═══════════════════════════════════════════════════════════════════════════
═══════════════════════════════════════════════════════════════════════════
▼▼▼ ADDENDUM 2 — SPEC 009 — 2026-11-11 — BEGINS HERE. PART OF THE PROMPT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

Appended after the round's close at cf2167e (session 2's final commit, Item 13). Where it differs from
the text above, it governs. This is a resume: a fresh session, session 3, on
spec/009-henning-differential. The header is unchanged, except MODEL's reason, ARTIFACT, GRANTS, STOP
RULE and the Read-first list, which change as below. At the close the advisor found that the committed
prompt, docs/specs/prompts/009-henning-differential.md, is the prompt as the first commit stored it
(41,868 bytes, ending at the original prompt's end banner): it does not hold ADDENDUM 1, which governed
session 2. GATED.md says a round's committed prompt changes only by an appended ADDENDUM, and CLAUDE.md
hard rule 2 says the copy is compared with `cmp` against the prompt file before every later commit;
together they decide that the copy holds every addendum. This session appends ADDENDUM 1 and this
ADDENDUM 2 to the copy, brings the report up to date with that one change, and closes the round again.
It builds nothing, runs no gate, and changes nothing else.

MODEL — unchanged: Sonnet 5.5, effort High. Reason: 003 §7, first branch. Everything here is
discoverable: one file made byte-identical to another, whose check is a whole-file `cmp`; records whose
content is given below; and diff-scope and audit lines measured with git. One part would be silent: a
copy that differs from the prompt file in a byte the eye does not see (a dash, a space, a box-drawing
character, a line-number prefix of the Read tool). Its instrument is the whole-file `cmp`, which must be
silent before the copy is committed (Item 15). Nothing is built, and no design choice falls to this
model. No change from sessions 1 and 2.

THE HUMAN'S RULINGS (2026-11-11). At the close the advisor sent its adjudication, with two questions and
a recommendation on each; the human answered "Agree with your recommendations".
  R9.  The committed prompt: (a) — a records-only resume (this session) appends ADDENDUM 1 and ADDENDUM
       2 to docs/specs/prompts/009-henning-differential.md, so that a whole-file `cmp` against
       /home/kitaeza/zurkhai-prompts/spec-009-henning-differential.md is silent; it brings the report's
       diff scope, records and squash message up to date; the merge follows. The options not taken:
       the human appending the file on the branch; a later round appending it; leaving it out of the
       tree.
  R10. Session 2's "Latitude taken" item 11 (the prompt copy compared with `cmp -n 41868`, its length,
       instead of a whole-file `cmp`) is accepted as reported. No grant is made after the fact. Items
       12, 13 and 14 are choices inside ADDENDUM 1's items and GRANTS, and need no ruling.

THE ADVISOR'S CORRECTIONS TO ADDENDUM 1 (2026-11-11; not rulings):
  C4. ADDENDUM 1 did not order session 2 to append it to the committed prompt. The omission is the
      advisor's; session 2 was right to flag it. The tree already decided the question (GATED.md's line
      on docs/specs/**; CLAUDE.md hard rule 2), so it is not a fork.
  C5. ADDENDUM 1's Item 11 named two statements that Henning's source was "never copied" (the heading
      of §4 and the opening of §21). There were three: session 2 found the third, in the report's
      opening paragraph, and corrected it.
  C6. ADDENDUM 1's R7, and D-23 after it, say "Remark 14, pp. 25–26". Remark 14 begins on p. 24; its
      passage on a2, (7.24) and the examples is on pp. 25–26, as session 2 noted. D-23's wording is true
      of that passage, and DEBT.md is not changed.

The advisor's check of what this addendum puts into the tree (RELAYED; this session does not re-run
it, having no ARTIFACT): ADDENDUM 1's Item 11 check, run on this prompt file through ADDENDUM 2 with
the patterns of tcg1309.zip's 13 text files, lists no line.

ARTIFACT — none. This session reads no source: ARTIFACT (A), (B) and (C) are not used.

READ FIRST — REPLACED for this session: this prompt file through ADDENDUM 2, in full; CLAUDE.md;
docs/ROUNDS.md §2, §3, §4 and §9; GATED.md; docs/METHOD.md rules 51, 58, 60 and 65, each in full; then
the report as Item 14 says. The report lists what it read.

GRANTS — REPLACED for this session (METHOD rule 67). Reads outside WORKDIR: this prompt file only. The
paper library is not read. The tools:
  - the Read tool, on the repository and on this prompt file;
  - the Edit and Write tools, on the two files Items 15 and 16 name, and on nothing else (repository
    files are edited only with these two tools: R1);
  - read-only shell tools on the repository and on this prompt file: cat, head, tail, sed -n, grep,
    cut, tr, sort, uniq, wc, diff, cmp, od, ls, stat, find, date, file;
  - `git -C /home/kitaeza/github/zurkhai`: log, show, diff, status, rev-parse, grep, add, commit.
This session runs no cargo, no gate.sh, no python3, no awk and no other program. Network: none. On this
machine `grep` is ugrep: keep any bounded repetition below 100.

STOP RULE — ADDITION:
  (13) Item 14's status or HEAD check fails: STOP and report, with zero diff.
  (14) The whole-file `cmp` of Item 15 cannot be made silent with the Edit and Write tools: STOP and
       report, with `cmp`'s first differing byte and `cmp -l | head -20`, and nothing committed. Never
       copy the file by a shell command, and never commit a copy that the whole-file `cmp` does not pass.
  (15) Out of scope: a change to any file other than docs/specs/prompts/009-henning-differential.md and
       docs/specs/009-henning-differential.md; any change to the first 41,868 bytes of the copy; any
       change to a record written by session 1 or 2, except an in-place `[CORRECTED …]` record, a
       bracketed note, or the squash message, as Item 16 orders; any gate run. A rule this addendum
       seems to get wrong is reported under "What the prompt got wrong", not fixed.

PATHS — unchanged from ADDENDUM 1: a local hook may reject a Bash call that runs cd. Rewrite the call
with absolute paths; never route around the hook (METHOD rule 67).

ITEM 14 — RESUME.
  - Read as READ FIRST says; then `git log`; then the report's head block, its Session 2 section, §16,
    §18, §19 and its section "The squash commit message". Quote `git status --porcelain` and
    `git rev-parse HEAD`. The status is empty and HEAD is cf2167e2c42a8ccea96a5b4547be15c48d31cb68. If
    either differs: STOP RULE (13).
  - Run `date` once, at the top, and use that value for every date this session writes.
  - Choose this session's marker (METHOD rule 58). The report calls it "session 3's marker" and never
    spells it.
  - Re-measure the OPEN count with the grep FORKS.md defines. The advisor measured 6 at cf2167e
    (RELAYED). Nothing in this session changes it.
  - Measure the copy and the prompt file: `wc -c` and `wc -l` of each, and
    `head -c 41868 /home/kitaeza/zurkhai-prompts/spec-009-henning-differential.md | cmp - <the copy>`,
    which the advisor found silent.

ITEM 15 — THE COMMITTED PROMPT (R9). One commit: the copy only.
  - Append to docs/specs/prompts/009-henning-differential.md everything in the prompt file after its
    first 41,868 bytes: ADDENDUM 1 and ADDENDUM 2, each from its opening rule line to its closing rule
    line, with no line between them and none added. Use the Edit tool (old_string: the original
    prompt's end banner, the copy's last three lines) or the Write tool (the whole file). The first
    41,868 bytes do not change. The Read tool shows each line after a line number and a tab: these are
    not part of the file.
  - The check: `cmp /home/kitaeza/zurkhai-prompts/spec-009-henning-differential.md <the copy>` is
    silent, and `wc -c` of the two are equal. If not, find the first difference with `cmp -l`, read both
    files at that byte with `od -c`, fix it with the Edit tool, and check again. The prompt file holds
    no tab, no carriage return and no trailing space, and ends with one newline (the advisor's
    measurement, RELAYED: re-measure it with grep and tail -c 1 | od -c). STOP RULE (14) if the `cmp`
    cannot be made silent.
  - From this commit on, every `cmp` of the copy is the whole-file `cmp` (CLAUDE.md hard rule 2).
  - Before the commit, in the commit's own chain: the whole-file `cmp`; `git status --porcelain` shows
    the copy only; `git diff --numstat` of the copy shows insertions and no deletion; the OPEN count;
    the marker grep.

ITEM 16 — THE REPORT AND THE CLOSE. One commit, the final one: the report only.
  1. The head block: `model:` gains session 3's entry. `gate:` keeps its text and gains, at its end:
     "; session 3 ran none: every leg skipped, because no file the gate reads changed after f63ad2f
     (Session 3)" (METHOD rule 65: the skipped legs are named).
  2. A "Session 3" section, after the Session 2 section and before "The squash commit message", holding
     this session's part of the report contract (CLAUDE.md), each item present, "none" where empty:
     R9, R10 and C4–C6; the hand-over from cf2167e; Items 14–16; the copy's sizes before and after;
     the whole-file `cmp` and its result; the predictions below, scored; the stop rule that governed
     the session; latitude taken; what the prompt got wrong; by-catch.
  3. In-place `[CORRECTED …]` records, each saying what changed and pointing to the Session 3 section,
     where a statement of session 2's is not true of the tree that merges:
       - S2.5's diff scope ("Diff scope at the final commit"): it was measured at cf2167e; the round's
         diff scope at its final commit is the Session 3 section's;
       - S2.5's "Commits of the branch": session 3's commits are listed in the Session 3 section.
     Session 2's statements that the copy does not hold the addendum, or was left unchanged (S2.1, §16
     item 11, §18, §19), are true of session 2 and stay as written. Each gets one bracketed note at its
     end: "[Resolved 2026-11-11 by session 3 (ADDENDUM 2, R9): ADDENDUM 1 and 2 are appended; see
     Session 3.]", with this session's date.
  4. §16, item 11: R10 beside it ("accepted as reported (ADDENDUM 2, R10); no grant").
  5. The open-forks arithmetic for each of session 3's commits: 6 + 0 + 0 − 0 = 6.
  6. Session 3's marker greps, per commit.
  7. The diff scope at the final commit: `git diff --numstat main` and `git diff --shortstat main`, the
     totals summing, the commit it was measured at named, and gated paths touched (no).
  8. That the final commit changes nothing the gate reads. Both commands print nothing, and the report
     records both and their empty output:
       git -C /home/kitaeza/github/zurkhai diff --stat f63ad2f HEAD -- crates golden tools Cargo.toml Cargo.lock rust-toolchain.toml deny.toml
       git -C /home/kitaeza/github/zurkhai diff --cached --stat -- crates golden tools Cargo.toml Cargo.lock rust-toolchain.toml deny.toml
  9. The squash commit message: edit the one fenced block under `## The squash commit message` in
     place. docs/ROUNDS.md §9 extracts the first fenced block after that heading: never add a second
     heading of that name, or a second block under it. Change only these: the sentence on the gate
     gains "(run on f63ad2f; no file it reads changed after it)"; and the paragraph that names the
     registers ends with the sentence "The committed prompt holds the round prompt and its two
     addenda." No code; one Co-Authored-By line.
  - docs/specs/INDEX.md is not changed: its row stands.
  - After the final commit the branch stands still (docs/ROUNDS.md §3).

PREDICTIONS — ADDITIONS, written by the advisor on 2026-11-11, before this session measures anything
(METHOD rule 3). Their basis is the advisor's own measurements at cf2167e and of this file. They are
RELAYED, and none is a premise. Score each HELD or FAILED, with the measured value.
  19. HEAD is cf2167e2c42a8ccea96a5b4547be15c48d31cb68, the status is empty, and the OPEN count is 6.
  20. Before Item 15, the copy is 41,868 bytes and 508 lines, and a byte prefix of the prompt file.
  21. After Item 15, the copy is 78,122 bytes and 955 lines, equal to the prompt file, and Item 15's
      commit shows 447 insertions and 0 deletions for it.
  22. At the final commit, `git diff --numstat main` lists the six files it lists at cf2167e. Four have
      the counts they have there: DEBT.md 40 0, FORKS.md 14 0, docs/sources.md 2 1,
      docs/specs/INDEX.md 1 0. The copy has 955 0, and the report more than 2,343 insertions and 0
      deletions.
  23. Both of Item 16's gate-input commands print nothing.

THE REPORT — ADDITION: this session's report is complete (CLAUDE.md's report contract: "Every
session's report is complete, a resumed session's included"), in the same file, as Item 16 orders it.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ADDENDUM 2 — SPEC 009 — ENDS HERE. NOTHING BELOW IS PART OF IT.      ▲▲▲
═══════════════════════════════════════════════════════════════════════════
