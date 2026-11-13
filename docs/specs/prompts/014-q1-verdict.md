═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 014 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  014 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
ARC:          Verdict, assigned by the advisor with the spec number; the human may overrule it.
BRANCH:       spec/014-q1-verdict, cut from main at 756681e before any edit; commit to it, the halt
              included. No merge. No push, to any remote.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7 names "run the Q1 diff" in its first branch
              (discoverable: what the engine computes on a day, what a golden row holds, and what a
              rendering prints are all measured, not invented). The one invented and expensive answer,
              the verdict itself, is NOT made in this round: it halts by design at Item 6 with the
              verdict as a STOP AND ASK, because 001 §6.5 and METHOD P4 make the call the human's, and
              003 §7 reserves Fable for "the Q1 verdict if the data is ambiguous", which only this
              round's numbers can show. A wrong comparison would be SILENT (a harness that compares the
              wrong day, the wrong system or the wrong field still prints counts), so the round runs on
              the smaller model only with the instruments that make it loud, each shown to fail
              (METHOD rule 31): an external positive control (the T4 Phugpa fixture through the same
              harness, Item 3(b)); a self-output control (Item 3(c)); a planted wrong cell and a
              planted one-day shift (Item 3(d)); and a blind re-read of every mismatching cell with
              decoys (Item 5(a)). No change from the last round: round 013 also ran on Sonnet 5.5,
              effort High.
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first. It never commits a previous session's uncommitted work without
              verifying it: it finishes the work and proves it by the item's own check, or redoes it. A
              design choice that would fall to the fallback model (Opus 5.5, effort High) is a STOP AND
              ASK. The report names the switch point.
PRECONDITION: Check each, and quote every output:
              (a) `git -C /home/kitaeza/github/zurkhai status --porcelain` prints nothing, and
                  `git -C /home/kitaeza/github/zurkhai rev-parse --abbrev-ref HEAD` prints main.
              (b) `git -C /home/kitaeza/github/zurkhai rev-parse main` prints
                  756681ea6a39039470e140052f0f6eac0fa3090a, spec 013's squash.
              (c) `git -C /home/kitaeza/github/zurkhai diff --stat d174965 HEAD -- crates/tshes/src`
                  prints nothing: the engine's source is the one spec 009 compared with TCG 1.06.
                  Spec 009's head block names 07b5bbf, a commit from before the history rewrite;
                  docs/specs/010-publication.md, "The history map", row 10, translates it to
                  d174965afc95cee0e7e0091261922c88aba56974 on today's main. Check that row yourself.
                  If the diff prints anything, STOP.
              (d) `ls /home/kitaeza/zurkhai-papers/` succeeds (the human has added the directory with
                  /add-dir).
              Anything else: STOP and report, with zero diff.
              Read first, in this order, each in full unless a part is named: CLAUDE.md;
              docs/ROUNDS.md; docs/METHOD.md rules 1, 3, 25, 27, 31, 33, 35, 36, 38, 44, 50, 51, 58, 60,
              61, 65, 66 and 67, and Part 2's P1, P3 and P4, each in full, not only its **Rule.**
              paragraph; GATED.md; FORKS.md (its header; F4, F10, F25 and F28 to F36 in full; the rest by
              their status lines); DEBT.md (its header; D-1, D-18, D-19, D-23, D-24 and D-33 to D-38 in
              full; the rest by their headings); docs/ROADMAP.md; docs/specs/001-init-spec.md (§4, §5.1,
              §6 and §10 to §12); docs/specs/002-rust-code-standard.md (in full: this round writes
              tests); docs/specs/003-project-method.md (§4, the P3 and P4 rows; §7);
              docs/specs/008-genden-tsurphu.md (the head block and §1); docs/specs/009-henning-differential.md
              (the head block, §1.1, §1.2 with its three lists of days, §13 and §21);
              docs/specs/013-buryat-sources.md (in full); golden/buryat/README.md and the three
              .meta.yaml files beside it; golden/reference/README.md; golden/LICENSE.md;
              docs/sources.md (the Janson row, and the ivolginsky-datsan/ and aginsky-datsan/ rows);
              docs/specs/INDEX.md; docs/specs/TEMPLATE.md. The report lists each file it read and how
              much of it.
ARTIFACT:     In the tree at 756681e: golden/buryat/ivolginsky-datsan-2021.csv (398 rows),
              golden/buryat/aginsky-datsan-2025.csv (413 rows), golden/buryat/aginsky-datsan-2026.csv
              (402 rows), each with its .meta.yaml; golden/reference/tdc-97d4a00-phugpa-1900-2100.csv
              (the T4 fixture) and its .meta.yaml. Re-count the rows; do not copy these counts.
              In the paper library, /home/kitaeza/zurkhai-papers/, read only, each with the sha256 given
              (MEASURED by the advisor on 2026-11-12 and 2026-11-13, RELAYED); re-measure each:
                - ivolginsky-datsan/lunnii_kalendar-2021.pdf
                  2e140c65b7cac71cbb6c98b837e1f7488a91d47ccd7ea299f3b062d7e320fe25
                - aginsky-datsan/aga/kalendar2025.pdf
                  12137318075f3a7ae946f946b5bbf1b4f628e4186adbdad033b510f2868bd7bb
                - aginsky-datsan/aga2026/kalendar-a2-format.pdf
                  13e7d720b3ba9dd8d47abe090584970afe91f43d6892e8f5cc811351832aa156
                - janson-1401.6285v1.pdf
                  7cafc7df563a3020849c86f4e397daa18b7f7f5ff46244403180aca90b9d0f77
              If a named file is absent, or a sha256 or a row count differs from the golden file's own
              meta: STOP and report.
STOP RULE:    (1) A PRECONDITION or ARTIFACT check fails: STOP, zero diff.
              (2) The engine is not changed. Nothing under crates/tshes/src/ is edited, no constant,
              epoch, rule or variant is added, and no engine variant is computed in the tree: a value
              under Janson v1's (7.24) comes only from spec 009's recorded lists (RECALLED, cited by
              section and row). If anything in this round seems to need an engine change: STOP AND ASK.
              Never tune a constant to fit a source (CLAUDE.md hard rule 7; 001 §6.5, §12).
              (3) Golden rows are not changed. If Item 5(a)'s blind re-read disagrees with a golden row,
              do not edit the row: record it, finish every other item, and write a STOP AND ASK for it
              (it is its own fork, separate from the verdict).
              (4) No verdict in your own voice (METHOD P4: the gate measures; a human decides). Never
              write that the Buryat calendars, a datsan or a source follow, use or are a system. State
              the counts, which branch of 001 §6.5 the counts satisfy as that section's words read, and
              where its words do not decide (for example, 001 §6.5 does not define "a small number of
              mismatches": say so, and give the counts). The verdict is Item 6's STOP AND ASK.
              (5) T4 and T5 data are never evidence for Q1 (001 §6.1). The T4 fixture is a control of
              the harness only (Item 3(b)), and the report never sets it beside a Buryat source as a
              comparison of practice.
              (6) A finding bears on a ruling already made (F25, F28 to F37, or R1 to R9 below): do not
              change the ruling or its row. Report the finding beside the ruling. If D-23's TRIGGER
              condition is met (Item 4(e)), F25 is put to the human again by its own STOP AND ASK, not
              by changing F25.
              (7) Out of scope; do not write any of it:
                - any change under crates/tshes/src/, rules/, clients/, tools/ or .github/; Cargo.toml,
                  Cargo.lock, rust-toolchain.toml, deny.toml; docs/METHOD.md, GATED.md, CLAUDE.md,
                  docs/ADVISOR.md, docs/ROUNDS.md, an accepted spec, or another round's row in
                  docs/specs/INDEX.md;
                - any change under golden/;
                - D-24's re-run of spec 009's TCG differential (its TRIGGER names the Q1 verdict spec,
                  which this round does not write), and D-1 (lifted only by the verdict spec);
                - the verdict spec itself, or any text that drafts it;
                - any contact with anyone, any network access;
                - deciding any fork, or writing a FORKS.md row other than Item 0's and those of
                  STOP RULE (2), (3) and (6) and Item 6.
              If something seems to need one of these, write it down for the report (by-catch, and a
              DEBT.md entry where it qualifies), and do not do it.
              (8) A read, a run or a network call outside GRANTS is needed, or a permission is denied:
              STOP and report each, with the reason. Never work around it by another tool or route
              (METHOD rule 67). Spec 013's Latitude 10 (inline `python3 -I -c` runs) was outside its
              GRANTS: here every script is a file you write first.
              (9) The halt: the round stops at Item 6 by design. It does not write the verdict spec, set
              its docs/ROADMAP.md row to LANDED, or add its docs/specs/INDEX.md row.
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; there is no fourth
              kind. A citation of a library file is FETCHED with its path and page or line. A figure
              from an earlier round's report is RECALLED with its spec and section, and re-measured
              wherever this round can measure it (CLAUDE.md hard rule 10). Read docs/sources.md before
              anything is cited. No network (GRANTS). (METHOD rule 50.)
GATED:        yes: crates/tshes/** for test code only (crates/tshes/tests/, its common/ helpers, and
              files the test framework writes, each accounted for), under accepted specs 001 (§6.4,
              §6.5, §10 M3) and 002 (the code standard). Nothing under crates/tshes/src/ (STOP RULE (2)).
              Also this round's own docs/specs/014-q1-verdict.md and its
              docs/specs/prompts/014-q1-verdict.md, which are not changes to an accepted spec. Not
              gated, and changed only as the items say: DEBT.md, FORKS.md, docs/ROADMAP.md.
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
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive
              widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 014 — The Q1 differential: the three golden/buryat/ files against the engine's three systems

Spec 013 committed three T1 Buryat calendars to golden/buryat/, and the human has ruled M0 met (R1
below). This round is the Q1 differential of 001 §6.5: every golden row against what the engine's
Phugpa, Tsurphu and Genden compute for that day, with every mismatch listed and investigated, under
instruments shown to fail. It also puts golden/buryat/ under the gate (DEBT.md D-37). It halts by
design at Item 6, where the verdict is put to the human as a STOP AND ASK: it does not write the
verdict spec, change the engine or any golden row, or contact anyone.

THE HUMAN'S RULINGS AND THE REVIEWER'S FINDINGS this round carries. Each is quoted in the report beside
the record that implements it.
  R1. Whether M0 is met (the human, 2026-11-13: "Agree with all recommendations, go merge"): option
      (c), met, on the condition that a refusal under DEBT.md D-35 which withdraws the Ivolginsky rows
      reopens M0, because only one datsan would remain. The weakness the human accepted with it: Sangha
      membership rests on the publishers' own statements until D-33's register evidence is in the
      library. Recorded by Item 0 (FORKS.md, DEBT.md D-35, docs/ROADMAP.md).
  R2. Spec 013's Latitude 1 to 11 (same words): accepted as reported; no grant after the fact.
      Recorded in the report only.
  R3. The review of spec 013's tip (b6f3a4a), RELAYED from the reviewer's report (Opus 5.5, effort
      High, 2026-11-12; the advisor re-checked item (a) in tools/): (a) "tools/gate-self-test.sh:39,
      the self_test leg's whole-tree tar copy, reads every file under golden/buryat/ (and would read
      any file added there) by a mechanism neither of the report's greps covers"; it copies bytes and
      checks nothing. (b) A second golden reader exists that D-37's Statement does not name: an
      `include_str!` in crates/tshes/src/phugpa.rs (about line 1479, under #[cfg(test)]), which reads
      golden/reference/janson-v1-table6-epochs-806.csv only. (c) A future reader that reaches
      golden/buryat/ through the existing path helper (crates/tshes/tests/common/fixtures.rs, about
      lines 16 to 21) with a relative name would not match a grep for `golden/buryat`; a re-measure
      needs a grep for `buryat` alone, or a run that breaks the files. (d) The reviewer broke
      golden/buryat/ (a +3 step, an extra comma, a deleted meta file), and in another copy deleted
      the directory: the gate stayed green in every leg, both times. (e) No reader in the tree reads
      any .meta.yaml, under golden/reference/ as well as golden/buryat/. (f) The generic loader in
      fixtures.rs keeps an empty field as an empty string and refuses a row whose field count differs
      from its header; every typed loader refuses a golden/buryat/ header. Re-verify (a) to (c) and
      (e) from the code before relying on them (CLAUDE.md hard rule 11).
  R4. The advisor's calls, told to the human with the rulings and not objected to (overrulable): D-37
      is re-measured by R3 (c)'s method before it is paid (Item 1); a DEBT.md entry records R3 (e)
      (Item 0); the delegated reader's model is recorded (GRANTS).

THE RESEARCH LEGS — copied from 003 §4's P3 row, as written there:
  Legs: ① **reference**: datsan publications, and a зурхайч where reachable. ② **field**: other
  implementations (Lotsawa House, Henning's software, existing Mongolian apps); pin where their
  conventions differ. ③ **science**: Janson, Henning, Schuh, Reingold–Dershowitz, or an explicit "none
  relevant". ④ **Does this contradict anything already shipped?** Every leg is reported, including empty
  ones, with what it found against the approach.
  Leg ① is the three golden files (a зурхайч is not reachable from this round: STOP RULE (7); FORKS.md
  F4 is OPEN). Leg ②: the T4 fixture (Lotsawa House's Phugpa) is the harness's control, never Q1
  evidence; Henning's TCG is spec 009's (RECALLED) and D-24's; the RuStore app of 001 §11 is not
  consulted, because FORKS.md F10 (its tier) is OPEN and must be decided before its outputs are
  recorded; say what else, if anything, the library holds. Leg ③: what Janson v1 says of the Mongolian
  / New Genden version and of where each version is used (A.2, A.3 and any passage the text layer
  shows naming Mongolia, Buryatia or the Buryats: grep the `pdftotext` copy, report every hit with
  its page, and quote at most two sentences). Leg ④: whether anything this round finds contradicts
  001 §4 (the candidates' description: "its leap months coincide with Tsurphu's, while individual days
  can fall one day apart"), F25's ruling, or golden/buryat/README.md.

ITEM 0 — Branch, prompt, number, records.
  - Check PRECONDITION and ARTIFACT, and quote every output.
  - Create spec/014-q1-verdict from main at 756681e. The head block's branch line names that commit and
    its tree, and the head block carries `arc: Verdict` after `prompt:`.
  - Commit this file, byte-identical, as docs/specs/prompts/014-q1-verdict.md. That is the first
    commit. `cmp` it against /home/kitaeza/zurkhai-prompts/spec-014-q1-verdict.md before every later
    commit.
  - Verify that 014 is free in the tree. Read the Read-first list, and list it in the report.
  - Run `date` once and use that value for every date you write (METHOD rule 51). If a later `date`
    shows a new day, the records written after it carry the new date, and the report says where.
  - Choose your round's marker (METHOD rule 58). The report calls it "the round's marker" and never
    spells it.
  - Re-measure the OPEN count in FORKS.md with the grep its header defines. The advisor counted 5 on
    main at 756681e (F4, F5, F7, F8, F10; RELAYED). Re-measure it; do not copy it.
  - FORKS.md: one new row, numbered from the next free F id (derive it; the advisor expects F37,
    RELAYED), filed already RULED, in the register's shape. Fields, one per line as given:
      row: R1
        subject: Whether M0's done-when is met by the three golden/buryat/ files
        Raised: 2026-11-12, by the advisor at spec 013's landing; recorded by spec 014 Item 0
        Question: 001 §10's M0 asks for "≥ 12 months of Buryat tshes data from ≥ 2 independent T1–T3
          sources". The tree holds the Ivolginsky datsan's 2021 calendar and the Aginsky datsan's 2025
          and 2026 editions, all T1 (spec 013). Is M0 met?
        Options: (a) met, unconditionally; (b) not met until the Ivolginsky datsan answers the request
          for written permission (DEBT.md D-35); (c) met, on the condition that a refusal under D-35
          which withdraws the Ivolginsky rows reopens M0, because only one datsan would remain.
        Status: RULED
        Ruled: 2026-11-13, (c). The human: "Agree with all recommendations, go merge".
        Built in: spec 014 (DEBT.md D-35; docs/ROADMAP.md, Gate 1's state cell)
    The "row:" name is this prompt's label for the ruling, not an id; the row takes its F id.
  - DEBT.md:
      (a) D-35: append one sentence to its Statement: "A refusal that withdraws these rows also
          reopens M0 (FORKS.md <R1's id>, RULED (c)): only the Aginsky datsan would remain." Change
          nothing else in it.
      (b) D-38: its TRIGGER ("the next round that edits FORKS.md") fires in this round. Pay it: set
          the `Built in:` line of F28 to "spec 013 (the step check of golden/buryat/, run by its
          scripts; under the gate from spec 014, D-37)", of F29 to "spec 013
          (golden/buryat/ivolginsky-datsan-2021.csv, T1)", of F30 to "spec 013
          (golden/buryat/aginsky-datsan-2025.csv and aginsky-datsan-2026.csv, T1, the
          disagreements noted in both)", and of F31 to "spec 013 (is_leap_month per the source's
          label; golden/buryat/README.md)"; then D-38's Status: "PAID <date>, spec 014 Item 0".
          If the words "under the gate from spec 014" turn out false at the halt (Item 2 not built),
          the halt's commit corrects that one phrase, and the report says so.
      (c) One new entry, numbered from the next free D id (the advisor expects D-39, RELAYED),
          Pinned by: not pinned, Status: OPEN:
            subject: No test reads a golden .meta.yaml under golden/reference/
            Kind: COVERAGE
            Statement: R3 (e) of spec 014's prompt (RELAYED from the review of spec 013's tip, and
              re-measured by spec 014): no code in the tree reads any .meta.yaml, so a missing or
              malformed meta file under golden/reference/ fails nothing. Spec 014 checks the
              golden/buryat/ metas only (D-37).
            TRIGGER: the next round that writes golden/reference/.
          If Item 1 measures that some code does read a golden/reference/ meta, file nothing, and
          report what reads it.
  - docs/ROADMAP.md: Table 2, Gate 1's state cell becomes "NOT REACHED: the Q1 verdict spec does not
    exist; M0 is met under FORKS.md <R1's id> (c); D-1 holds", with a Revisions line. The q1-verdict
    row of Table 1 is not changed by this round (STOP RULE (9)).
  - The open-forks audit: the count expected at the halt is the count you measured at the base, plus
    one for Item 6's STOP AND ASK, plus one for each STOP AND ASK that STOP RULE (2), (3) or (6)
    files; R1's row is RULED and adds nothing. Show the arithmetic at every commit.

ITEM 1 — What the comparison stands on. Before any test is written.
  (a) The engine: name every public item of crates/tshes that the harness will call (the conversion
      from a Gregorian date to a ZDate, the System values, and the ZDate fields read), with file:line,
      and how each field the harness reads is defined (its doc comment, or the spec clause it
      implements): tshes, month, is_leap_month, is_doubled, skipped_before. State how the engine
      marks the second of two equal tshes and the number omitted before a day, and whether that is
      the convention golden/buryat/README.md states for is_doubled and skipped (001 §5.1, F9 (b)).
      State which epoch each System uses (spec 008's report says Tsurphu computes E1732; RECALLED,
      re-check it).
  (b) D-37 re-measured (R3 (c)): grep for `buryat` alone over crates/, tools/ and .github/, and list
      every reader of a golden path in the tree by any mechanism, with file:line. Compare with R3
      (a), (b) and (e), and say for each whether it holds.
  (c) The 001 §6.5 rules, quoted, and the fields this round compares under each, with METHOD rule 25
      applied: a field decides a disagreement only where both sides can see it. The tshes column is
      read from the print in all three files; is_doubled and skipped are derived from the tshes
      sequence (spec 013, F9 (b)), so they are not independent evidence and are reported, never
      counted as separate mismatches; month and is_leap_month exist only in the two Aginsky files
      (the Ivolginsky file prints no label, R8 of spec 013). Say which field the decisive table
      counts, and why.

ITEM 2 — golden/buryat/ under the gate (pays D-37).
  Test code only, in crates/tshes/tests/ and its common/ helpers; spec 002 governs it. The shape (file
  names, helper names, how the loader reaches golden/buryat/) is yours: report it as "considered X,
  chose Y because Z".
  (a) A loader for a golden/buryat/ CSV that refuses, with a named assertion: a header other than
      001 §6.2's seven columns; a row with another field count; a quote character; a CR; a date not
      ISO or not one day after the previous row; a tshes outside 1 to 30; a month outside empty or 1
      to 12; a boolean other than true / false where one is required, as golden/buryat/README.md
      defines each column.
  (b) Tests, every one in the ordinary gate (no #[ignore]):
        - the set of CSV files in golden/buryat/ equals the list the tests name, so a new file
          without a test is loud;
        - each CSV has its .meta.yaml, holding every key of golden/buryat/README.md's section "The
          meta keys", in that order, each non-empty, read line by line as that README says;
        - the tshes step check (F28; 001 §6.3): +1, +2 (one skipped) or 0 (a doubled day, never
          twice in a row), modulo 30; and is_doubled and skipped agree with the steps;
        - for the Aginsky files, the month rule of golden/buryat/README.md (FORKS.md F36): every
          lunation carries one label, the labels in seasonal order, «Дополнительный …» with
          is_leap_month true and an empty month.
  (c) METHOD rule 31, in a `git archive` copy of your commit inside a `mktemp -d` directory, never in
      the working tree: plant (i) a +3 step, (ii) an eighth field, (iii) a deleted meta file, (iv) a
      CSV added to golden/buryat/ that the tests do not name, (v) a label moved to the neighbouring
      lunation; run the tests on the copy, and report for each plant the test that went red and its
      first failing assertion. Each plant must turn at least one test red.
  (d) D-37: set its Status to "PAID <date>, spec 014 Item 2 (<the test names>)".

ITEM 3 — The harness, and its controls. Before any Buryat result is read.
  (a) A test (in the ordinary gate) that, for each golden/buryat/ file and for each of Phugpa,
      Tsurphu and Genden, compares every row with what the engine computes for that Gregorian date:
      tshes (the decisive field, Item 1(c)), is_doubled, skipped, and for the Aginsky files month and
      is_leap_month per lunation (Item 4(c)). It prints, with `--nocapture`, one line per
      disagreeing day and system (date, file, tier, the golden value, the engine's value), and the
      counts of Item 4(a).
  (b) External positive control: the same comparison path run on the T4 fixture's rows for the
      dates of the three golden files (the T4 file's tshes column against System::Phugpa). Expected:
      0 tshes differences, because the T4 fixture equals the engine's Phugpa on 1900–2100 (spec 009
      §8 and 001 §6.4 P6; RECALLED). Report the count for Phugpa, Tsurphu and Genden. This control
      tests the harness only (STOP RULE (5)).
  (c) Self-output control: for one system, rows built from the engine's own output for the dates of
      one golden file, through the harness: 0 differences for that system.
  (d) Planted controls (METHOD rule 31), on scratch copies of the golden files read by the harness
      (never the committed files): (i) one tshes changed on one day: exactly that day, for every
      system that matched it before, is reported; (ii) every golden date shifted by one day: the
      harness reports disagreement on most days for every system. Report both outputs.
  (e) A pin: one test asserts, per file and system, the tshes match and mismatch counts measured in
      Item 4, and the list of mismatching dates, so that a change in the engine or in a golden file
      is loud. Its name says it pins a measurement, not a correctness claim, and its doc comment
      names Item 6's FORKS.md row as what resolves the mismatches it lists (METHOD rules 33, 44).
      Show it fails on plant (d)(i).
  Run the tests in debug and in release (002 §4.3); the report's tables are copied from one run of the
  committed test with `--nocapture`, and the two profiles' outputs are compared by `diff` (expected:
  equal).

ITEM 4 — The differential report (001 §6.5), in the report's own section "The Q1 differential".
  (a) Per file and system: rows, tshes matches, tshes mismatches, coverage (the file's days, its first
      and last date), and the doubled and skipped agreement (reported, not counted, Item 1(c)). Then
      the distinct days of all three files together (the two Aginsky editions overlap on 2026-01-01
      to 2026-02-17) and the totals per system over T1–T3 rows. Every row must sum.
  (b) Every mismatch, per system: date, file, tier, the golden tshes, the engine's tshes, and the
      lunation it falls in. Null results are reported (001 §6.5): a system with 0 mismatches on a
      file says so.
  (c) Months and the extra month (FORKS.md F31 (a): "the Q1 round mapping conventions"), for the two
      Aginsky files and each system, by lunation (golden/buryat/README.md's lunation rule): whether the
      engine's lunation boundaries equal the source's; the engine's month and is_leap_month on each
      lunation's first day beside the source's month and label; for each non-leap lunation the
      difference engine month minus source month, modulo 12, with its count over the file; and which
      lunation, if any, the engine flags is_leap_month against the lunation the source labels
      «Дополнительный …». This measures the relation between the two conventions; it chooses none.
      If the relation is not one constant offset with the extra month at one position, report every
      exception; do not pick a mapping (STOP RULE (4)).
  (d) The Сагаалган days printed by the sources (spec 013 found 2021-02-12 and 2022-02-02 in the
      Ivolginsky hural schedule, 2025-03-01, 2026-02-18 and 2027-02-07 in the Aginsky editions;
      RECALLED, re-read each from the golden files' notes or metas): for each, and for each system,
      the engine's ZDate on that day and whether it is the first day of the engine's month 1 (not a
      leap month). Report the table.
  (e) D-23's TRIGGER (spec 009's lists, §1.2): for each day of the three lists inside the golden
      files' dates, the source's tshes beside the engine's value and the (7.24) value spec 009 lists
      for that day and system (RECALLED, cited by section and row). D-23 says: "If an admitted source
      matches the (7.24) value of its best-matching system on such a day, F25 is put to the human
      again". Apply it as written, per file, with "best-matching" as Item 4(f) defines it, and say
      whether it fires. If it does: STOP RULE (6).
  (f) The best-matching system per file: the system with the most tshes matches; a tie is reported
      as a tie. D-19's TRIGGER (ii) ("the Q1 differential report (M3) finds Tsurphu the
      best-matching system for any source"): say whether it fires.
  (g) The two Aginsky editions on their 8 disagreeing days (F30): each edition's value beside each
      system's.

ITEM 5 — Every mismatch of the best-matching system, investigated (001 §6.5: "each mismatch is
  investigated (transcription? source error? genuine variant?) before any verdict").
  (a) Transcription: one delegated reader (GRANTS), blind, reads crops that you render from the
      library PDFs (`pdftoppm`, a resolution where every number reads clearly; state it): one crop per
      mismatching day of each file's best-matching system, up to 40 per file (beyond 40, the first 40
      by date, and the report says how many were not re-read), and at least as many decoy days,
      chosen by a rule you fix and record before the reading. The reader is told neither the golden
      value nor the engine's, nor which crops are decoys; it writes, per crop, the Gregorian day and
      the tshes printed. Compare its readings with the golden rows by a script. A disagreement on any
      crop (mismatch or decoy) is STOP RULE (3). Record the brief verbatim, and the model the reader
      ran on.
  (b) Source disagreement: a mismatch on a day where the two Aginsky editions disagree (F30) is
      classed as such, with both editions' values.
  (c) Variant candidates: a mismatch on a day of spec 009's lists (D-23), or one that equals the
      value of another of the three systems on that day, is classed as such, with the values.
  (d) Pattern: whether the mismatches of each file cluster (consecutive days, one lunation, one
      season) or are isolated; report the runs, and for each run whether the source and the engine
      differ by a shift of one day (the source's tshes on day d equals the engine's on day d ± 1).
      Describe; do not explain beyond what (a) to (c) measure (METHOD rules 27 to 29).
  (e) The classes, counted: every mismatch of the best-matching system falls in exactly one of
      TRANSCRIPTION (only if (a) disagreed), SOURCE DISAGREEMENT, VARIANT CANDIDATE, or UNEXPLAINED,
      with the counts summing to the mismatch count per file.

ITEM 6 — The verdict, as a STOP AND ASK. The halt.
  Write it in docs/ROUNDS.md §5's shape, in the report and as an OPEN row in FORKS.md, numbered from
  the next free F id (the advisor expects F38 if no other fork came first, RELAYED):
    - DONE AND GREEN: Items 0 to 5, the gate (Item 7).
    - THE FINDING: Item 4's totals per system, Item 4(c), (d) and (f), and Item 5(e)'s classes, and
      which branch of 001 §6.5 the counts satisfy as its words read, with STOP RULE (4)'s caveat
      where its words do not decide.
    - THE QUESTION: which outcome of 001 §10 M3 the verdict spec records: a verdict, or the
      stop-and-consult outcome.
    - OPTIONS — a starting set, not a menu: at least (1) a verdict naming one system, with its scope
      (which datsans and dates the evidence covers) and how its listed mismatches are recorded; (2)
      the stop-and-consult outcome of 001 §6.5 (the data to a зурхайч; FORKS.md F4 is OPEN); (3) more
      data before any verdict (DEBT.md D-33, D-34); each with its real cost, measured where it can
      be. Add any option the numbers suggest, and say which branch of 001 §6.5 each one follows.
    - IF THE ANSWER IS DELAYED: nothing; the branch stands still.
  The round does not choose (STOP RULE (4)). The verdict spec, D-1 and D-24 belong to the resume.

ITEM 7 — The halt's gate and commit.
  - Run tools/gate.sh once, with nothing in flight, in both profiles (CLAUDE.md hard rule 12; METHOD
    rule 65), on the halt's commit (docs/ROUNDS.md §3: a halt runs the gate if a tracked source
    changed). Compare its count with spec 013's recorded count (147 in each profile; RECALLED), and
    name every new test.
  - Account for every file `git status --porcelain` shows, including any the test framework wrote.
  - The halt commit holds the report so far, the STOP AND ASK(s) and their FORKS.md rows. No INDEX.md
    row and no ROADMAP.md LANDED (STOP RULE (9)). After the commit the branch stands still: a reviewer
    reads its tip before the human rules (docs/ADVISOR.md §6, before a heavy adjudication).

PREDICTIONS — written by the advisor on 2026-11-13, before this round measures anything (METHOD rule 3).
Their basis is stated per line; none is a premise. Score each HELD or FAILED, with the measured value; a
prediction scored HELD cites your own measurement, never this prompt.
  1. Every PRECONDITION and ARTIFACT check passes; crates/tshes/src/ is unchanged since d174965
     (basis: the advisor's `git diff --stat d174965 main -- crates/tshes/src`, empty, 2026-11-13;
     RELAYED).
  2. Item 1(b): nothing but the self-test's tar copy reaches golden/buryat/; R3 (a), (b) and (e) hold
     (basis: R3).
  3. Item 3(b): 0 tshes differences between the T4 fixture and the engine's Phugpa on the golden
     files' dates (basis: spec 009 §8, RECALLED).
  4. Genden is the best-matching system for each of the three files (basis: the advisor's
     recollection that the published Mongolian Tsagaan Sar fell on 2022-02-02 and 2025-03-01, where
     the Tibetan Losar fell on 2022-03-03 and 2025-02-28; RECALLED / UNVERIFIED, not from the tree).
  5. Each Сагаалган day of Item 4(d) is the first day of Genden's month 1, and 2022-02-02 and
     2025-03-01 are not the first day of Phugpa's month 1 (basis: as 4).
  6. The engine's Genden flags the lunation 2025-01-30 to 2025-02-28, which the 2025 edition labels
     «Дополнительный зимний месяц», as a leap month (basis: as 4; INFERRED).
  7. No system matches 100% of the T1 rows: on 2026-01-09 to 2026-01-16 the two Aginsky editions
     print different tshes (spec 013), so at most one of them can match a system there (basis: the
     golden files, MEASURED by the advisor 2026-11-12).
  8. On 2025-11-19 the 2025 edition prints tshes 29, the second of two (line 324 of
     golden/buryat/aginsky-datsan-2025.csv), which
     differs from the engine's Phugpa (spec 009 §1.2's Phugpa list: engine 2025/9/30, TCG 2025/9/29;
     RECALLED) and equals the (7.24) Phugpa value; D-23 fires only if Phugpa is that file's
     best-matching system, which prediction 4 says it is not (basis: the advisor's reading of the
     golden file and of spec 009's table, 2026-11-13).
  9. Item 5(a)'s blind re-read agrees with the golden rows on every crop (basis: spec 013's two passes
     and the advisor's spot reads at 300 dpi of 2021-06-01..04 and 2026-03-31..04-02, 2026-11-12).
  10. Every planted control of Items 2(c) and 3(d) turns its test red, and the T4 control of 3(b) is
      green.
  11. OPEN is 5 at the base and 6 at the halt (the verdict fork only).

THE REPORT — per CLAUDE.md's report contract and docs/ROUNDS.md §4, in docs/specs/014-q1-verdict.md.
It holds:
  - the head block, with its `arc:` line (no `golden:` line: golden/ is not touched);
  - a report-shaped spec (docs/specs/TEMPLATE.md's header lines: Status: draft; Resolves: none yet,
    the verdict fork being OPEN; Milestone: M3; Governs: crates/tshes/tests/ (golden/buryat/ under
    the gate, the Q1 differential)), then the end marker;
  - then the report.
  Lead with the decisive item: one table, one row per file and system: rows, tshes matches, tshes
  mismatches, and the best-matching system marked; under it, the totals per system over T1–T3 rows,
  and the result of every control of Items 2(c) and 3(b) to 3(e). Then:
    - "The Q1 differential" (Item 4), with every mismatch listed;
    - Item 5's investigation, its classes summing;
    - Item 1's findings, and R3 (a) to (c) and (e), each held or not;
    - R1 to R4, each beside the record that implements it;
    - the four legs, each reported, the empty ones with what was searched;
    - the predictions, scored;
    - the open-forks audit, with its arithmetic at every commit;
    - the marker greps, at every commit;
    - every other section of the contract, "What the prompt got wrong" included;
    - the diff scope, with its totals summing against `git diff --stat main`, stating the commit it was
      measured at;
    - every STOP AND ASK, Item 6's last;
    - last: the delegated reader's brief and model, the commands and the scripts, verbatim.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 014 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════

═══════════════════════════════════════════════════════════════════════════
▼▼▼ ADDENDUM 1 — SPEC 014 — 2026-11-13 — BEGINS HERE. PART OF THE PROMPT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine (a FRESH session; the round resumes on its own branch)
WORKDIR:      /home/kitaeza/github/zurkhai
BRANCH:       spec/014-q1-verdict, at its tip 5cec12a33f0a6145cfca3ccc0ea6e4206ea16899 (the halt). Commit
              to it. No merge. No push, to any remote. No new branch.
ARC:          Verdict (unchanged).
MODEL:        Opus 5.5. EFFORT: High. Reason: 003 §7's third branch, "Invented and expensive to redo:
              spec drafting". The verdict itself is the human's (R5 below); this session drafts the
              verdict spec that records it, the records it moves, and their wording, which is invented
              text whose errors are silent. Its measurement items (Items 10 to 12) are discoverable and
              each has an instrument shown to fail. A CHANGE from session 1, which ran on Sonnet 5.5,
              effort High. Fable was not taken: the data does not leave the choice of system ambiguous
              (on the 14 days that separate Tsurphu from Genden the source equals Genden on 15 of 15
              rows), and the question it would have served is the one the human has ruled.
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first, and never commits a previous session's uncommitted work without
              proving it by the item's own check.
PRECONDITION: Check each, and quote every output:
              (a) `git -C /home/kitaeza/github/zurkhai status --porcelain` prints nothing, and
                  `git -C /home/kitaeza/github/zurkhai rev-parse --abbrev-ref HEAD` prints
                  spec/014-q1-verdict (switch to it with `git -C … switch spec/014-q1-verdict` if HEAD
                  is main and the tree is clean; anything else: STOP).
              (b) `git -C /home/kitaeza/github/zurkhai rev-parse HEAD` prints
                  5cec12a33f0a6145cfca3ccc0ea6e4206ea16899, and `rev-parse main` prints
                  756681ea6a39039470e140052f0f6eac0fa3090a.
              (c) `git -C /home/kitaeza/github/zurkhai diff --stat d174965 HEAD -- crates/tshes/src`
                  prints nothing (session 1's PRECONDITION (c), unchanged).
              (d) `ls /home/kitaeza/zurkhai-papers/` succeeds (the human has added it with /add-dir).
              (e) `git -C /home/kitaeza/github/zurkhai show HEAD:docs/specs/prompts/014-q1-verdict.md |
                  cmp - /home/kitaeza/zurkhai-prompts/spec-014-q1-verdict.md` reports only "EOF on -"
                  after the committed copy's last byte: the committed copy is this file up to the
                  original prompt's end banner, and this ADDENDUM follows it. Any reported difference
                  before that EOF: STOP.
              Anything else: STOP and report, with zero diff.
ARTIFACT:     In the tree at 5cec12a: the three golden/buryat/ CSVs and metas (398, 413, 402 rows);
              docs/specs/009-henning-differential.md §21 (the build, the stand-in headers, the dump
              probe, the scripts, among them janson_ref.py, §21.5); the round's report so far,
              docs/specs/014-q1-verdict.md. In the paper library, read only, re-measure each sha256
              and compare it with docs/sources.md's row (the advisor has not measured these two today):
                - henning-tcg/tcg1309.zip
                - henning-kalacakra/tdata/ (the pl_<year>.txt and ts_<year>.txt files spec 009's
                  build check reads)
              and Janson v1 (janson-1401.6285v1.pdf, 7cafc7df563a3020849c86f4e397daa18b7f7f5ff46244403180aca90b9d0f77).
              If a named file is absent, or a sha256 differs from docs/sources.md: STOP and report.
READ FIRST:   REPLACED for this session: this prompt file through ADDENDUM 1, in full; CLAUDE.md;
              docs/ROUNDS.md; docs/METHOD.md rules 3, 10, 25, 27 to 29, 31, 35, 50, 51, 56, 58, 60, 65,
              66 and 67 and Part 2's P4, each in full; GATED.md; FORKS.md (its header; F4, F25, F37 and
              F38 in full; the rest by their status lines); DEBT.md (its header; D-1, D-19, D-23, D-24,
              D-27, D-33, D-35 in full; the rest by their headings); docs/ROADMAP.md;
              docs/specs/001-init-spec.md (§4, §6, §10 to §12); docs/specs/002-rust-code-standard.md (in
              full: Item 10 writes tests); docs/specs/003-project-method.md (§7); docs/specs/TEMPLATE.md;
              docs/specs/INDEX.md; docs/specs/009-henning-differential.md (head block, §1.1, §1.2, §21 in
              full); the round's own report, docs/specs/014-q1-verdict.md, in full; golden/buryat/README.md.
              Then `git -C /home/kitaeza/github/zurkhai log --format='%h %s' main..HEAD`.
STOP RULE:    Session 1's STOP RULES are REPLACED for this session by these:
              (1) A PRECONDITION or ARTIFACT check fails: STOP, zero diff.
              (2) The engine is not changed: nothing under crates/tshes/src/ is edited, and no constant,
              epoch, rule, offset or variant is added. Never tune a constant to fit a source (CLAUDE.md
              hard rule 7). Item 12's day-start offsets are a test of a named hypothesis, computed
              outside the engine; none is adopted anywhere.
              (3) Golden rows are not changed.
              (4) The verdict is the human's ruling R5, as written below. Write it as ruled: do not
              widen it (no datsan, date range, tier or system beyond its scope) and do not narrow it.
              Where its words do not decide a sentence the verdict spec needs, and the choice changes
              what the verdict claims, STOP AND ASK. Never write that the Buryat calendars, a datsan or
              a source "follow", "use" or "are" a system except in the verdict sentence R5 gives.
              (5) T4 and T5 data are never evidence for Q1 (001 §6.1).
              (6) D-24's re-run (Item 11): a disagreeing day outside D-23's 53 days, or an engine dump
              that differs from the one spec 009 compared (its counts), is a finding: STOP AND ASK
              before the verdict spec is committed.
              (7) Item 10: an assertion written to a figure the report states, which fails on the
              tree, means the report's figure is wrong: do not change the figure to make it pass; STOP
              AND ASK.
              (8) Item 12: if any non-zero day-start offset gives fewer Genden mismatches against the
              golden rows than the offset 0, STOP AND ASK (it would bear on the verdict).
              (9) A read, a run or a network call outside GRANTS is needed, or a permission is denied:
              STOP and report each, with the reason (METHOD rule 67).
              (10) Out of scope; do not write any of it: any change under crates/tshes/src/, rules/,
              clients/, tools/, .github/, golden/; Cargo.toml, Cargo.lock, rust-toolchain.toml,
              deny.toml; docs/METHOD.md, GATED.md, CLAUDE.md, docs/ADVISOR.md, docs/ROUNDS.md, an
              accepted spec, another round's row in docs/specs/INDEX.md; a computed variant of any
              system in the tree (D-19's karana Sun, (7.24), a day-start offset); any contact with
              anyone; deciding any fork the rulings below do not decide.
CITATIONS:    As session 1. Every figure copied from session 1's report or from this ADDENDUM is
              RECALLED and re-measured wherever this session can measure it (CLAUDE.md hard rule 10).
GATED:        yes: crates/tshes/tests/** for test code only (Item 10), under accepted specs 001 (§6.5,
              §10 M3) and 002. This round's own docs/specs/014-q1-verdict.md, which becomes the verdict
              spec (Item 13), its committed prompt copy (an ADDENDUM is appended, nothing else changes),
              and its own row in docs/specs/INDEX.md: not changes to an accepted spec (GATED.md). Not
              gated, changed only as the items say: DEBT.md, FORKS.md, docs/ROADMAP.md.
GRANTS:       Session 1's GRANTS, with these changes:
              Reads outside WORKDIR: /home/kitaeza/zurkhai-papers/, read only, only the files of
              ARTIFACT (Janson v1, henning-tcg/tcg1309.zip, henning-kalacakra/tdata/), with ls, stat,
              sha256sum, grep, pdftotext (into a `mktemp -d` directory) and `unzip -q` (into a
              `mktemp -d` directory).
              Network: none (unchanged).
              The Agent tool: none.
              Added tools, for Item 11 only, as spec 009 §21.1 runs them: gcc (compile and link the
              extraction and the stand-in headers of §21.2, typed from the record, inside a `mktemp -d`
              directory); `env --chdir=<scratch dir>` to run the built program there, with its input
              from a file; the dump probe of §21.3 added to a `git archive` copy of HEAD inside a
              `mktemp -d` directory, never to the working tree, run by `cargo +1.99.0 test --offline`
              with CARGO_TARGET_DIR in the same directory.
              Scripts: `python3 -I`, running only scripts you write into a `mktemp -d` directory,
              including spec 009 §21.5's scripts extracted verbatim from docs/specs/009-henning-
              differential.md (report each extraction's sha256 and that it equals the block in the
              file, `cmp` silent). Never an inline `python3 -I -` or `-c` run.
              The repository: as session 1, and `git switch spec/014-q1-verdict` (PRECONDITION (a)).
ASKING:       As session 1 (docs/ROUNDS.md §5). Never an interactive widget.
PATHS:        As session 1.

THE HUMAN'S RULINGS (2026-11-13; the human, to the advisor's recommendations after the review of the
halt: "Agree with all recommendations, go."). Each is written onto its record by Item 9 and quoted in
the report beside it.
  R5. F38, the outcome of the Q1 differential: option (1), in this shape.
      (a) The verdict: Genden, as the engine computes it (Janson v1 A.3; epoch E1747; a2 = 1/28,
          (7.13), FORKS.md F25), is the computation system the admitted Buryat sources support.
          Q1 (001 §4) is answered "Genden", within the scope of (c).
      (b) The basis: 001 §6.5's second rule, after the investigation that rule requires (session 1's
          §3). Not the first rule: "One system matches 100% of T1–T3 rows" is read over all Buryat
          golden rows, as §6.5's opening words say ("For each candidate system, over all Buryat golden
          rows:"), and over them no system matches 100% (Genden 1,199 of 1,213). The per-source
          reading is not taken.
      (c) The scope: three T1 files of two datsans (the Ivolginsky datsan's 2021 calendar, the Aginsky
          datsan's 2025 and 2026 editions), 1,213 rows, 1,165 distinct days, 2021-01-01 to 2022-02-02
          and 2025-01-01 to 2027-02-06. Sangha membership rests on the publishers' own statements
          (DEBT.md D-33). The Ivolginsky rows stand on the human's acceptance pending the datsan's
          answer (D-35).
      (d) The 14 mismatches, every one listed with both values: the 8 days of the 2026 edition
          (2026-01-09 to 2026-01-16) recorded as a disagreement between the datsan's two editions,
          the 2025 edition printing Genden's value on all 8; the 6 days of the 2025 edition
          (2025-12-22 to 2025-12-27) recorded as UNEXPLAINED and OPEN. All 14 lie in one lunation,
          2025-12-20 to 2026-01-18 (re-measure it, Item 10). "A local variant" (001 §4) is not
          excluded by the verdict; no variant is computed or adopted, because none has a published
          basis (001 §6.5, §12; CLAUDE.md hard rule 7).
      (e) Reopening: the verdict reopens, as a fork put to the human, on the first of: (i) a refusal
          under D-35 that withdraws the Ivolginsky rows; (ii) an admitted T1–T3 Buryat source whose
          best-matching system is not Genden; (iii) a зурхайч, or an admitted source, that names a
          published rule which accounts for the 6 open days; (iv) D-33's evidence showing that a
          datsan of the scope is not a Sangha member.
      (f) The weakness it keeps, stated in the verdict spec: the 6 open days; and that 001 §6.5's
          "each mismatch is investigated" is met by investigation, not by explanation.
  R6. D-19, whose TRIGGER (i) ("a Q1 verdict spec would rule Tsurphu out") fires with R5: option (a).
      The verdict rules out Tsurphu as Janson v1's A.2 computes it (on the days that separate the
      engine's Tsurphu from its Genden the source equals Genden on every row: re-measure the count),
      and states that the karana-Sun variant of Remark 22 was not measured. D-19 stays OPEN,
      corrected in place: TRIGGER (i) recorded as fired by spec 014; its TRIGGER becomes (ii), (iii),
      or the verdict of spec 014 reopening.
  R7. D-23: option (a), not fired. Its first sentence's duty (report the source's value against the
      engine's and the (7.24) value on a day of spec 009's lists) was met by session 1's §2(e). Its
      F25 clause is read as "a day of its best-matching system's own list"; on 2025-11-19, the only
      list day inside the files, Genden's two values of a2 agree, so a match there carries nothing on
      a2. D-23's TRIGGER is corrected in place to that wording. The fact beside it, recorded: on that
      day the 2025 edition equals Phugpa's (7.24) value; Phugpa is not that file's best system.
  R8. D-33, whose TRIGGER ("the q1-verdict prompt is written") fired with session 1's prompt and was
      not acted on: option (a). Record the firing in place; the verdict's scope carries the caveat
      (R5 (c)); D-33 stays OPEN with the TRIGGER "the human saves any of them into the library".
  R9. F4 (when to approach Гунзэчойнэй): option (b), after M3. The approach carries the verdict, its 6
      open days and the two Aginsky editions' disagreement as its questions; the contact is the
      human's, outside the tree.

FORCED BY THE RECORDS (not rulings; the records decide them):
  - D-1's TRIGGER ("the Q1 verdict spec is accepted (M3)") fires with R5: D-1 is PAID by Item 9. A
    client that defaults System shows the system and the verdict's scope; 001 §10 M5's done-when
    already requires "tradition and source visible".
  - D-24's TRIGGER names the Q1 verdict spec: its re-run is Item 11.

THE ADVISOR'S CALLS, told to the human with the rulings (overrulable; not objected to):
  C1. The extra month (session 1's §2(c)): the verdict spec records the relation as measured, n = 1
      (the engine's is_leap_month on the first of the two month-12 lunations, the source's
      «Дополнительный» on the second), and maps nothing beyond it. A DEBT.md entry (Item 9) carries
      how a client names the pair.
  C2. Item 10: every figure the verdict spec cites that today rests only on printed output gets an
      assertion.
  C3. Item 12: the day-start offset probe, on the human's question to the advisor of 2026-11-13
      ("could [the] lunation difference [be] connected with lunar day begins on exact longitude?").
  C4. The model change (MODEL line).

THE ADVISOR'S CORRECTION (2026-11-13; not a ruling):
  K1. Session 1's prompt did not act on D-33's TRIGGER ("the q1-verdict prompt is written"), which
      that prompt fired. R8 records it now.

THE REVIEWER'S FINDINGS on the halt's tip 5cec12a (RELAYED from the reviewer's report: Fable 5.2,
effort High, 2026-11-13; the engine never reads that report; these are its words where quoted).
Every count and value of session 1's §1 to §3 was reproduced by the reviewer's own probe through the
public API (1900-01-01 to 2100-12-31; "FIGURES THAT DIFFER FROM THE REPORT: 0"); the gate on the tip:
11 legs, 11 passed, 1 skipped; 174 / 174 / 174. Its findings, and what this session does with each:
  V1. "the T4 control shares the date and comparison path with the T1 comparison but cannot exercise
      the golden file's own tshes column on its way into the compared row; that step is covered by
      other tests, by plant (i) on disk and by the outside recount." No action; cite it in the
      verdict spec's account of its controls.
  V2. "four tests pass with golden/buryat/ deleted or empty; none reads a golden file; one of them,
      q1_report_the_sagaalgan_days_in_each_system, asserts nothing, so §2(d)'s table has no
      assertion behind it." Item 10.
  V3. "'D-23 does not fire' folds the TRIGGER (met: the round compared a source on a list day and
      reported it) into the F25 clause, and the clause's non-firing rests on a reading ('a day of the
      best-matching system's own list') that the record's words do not state." R7.
  V4. "'001 §6.5 does not say which' (pooled or per source) leaves out §6.5's opening words 'over all
      Buryat golden rows'; no word of 001 §6.5 or §10 supports a per-source reading of rule (i)." R5
      (b); Item 13 (e).
  V5. "'(ii) is satisfied as written' and §12's 'the counts satisfy: (ii)' apply the undefined 'small
      number' while the same paragraph says no threshold is applied." Item 13 (e).
  V6. "D-24's re-run of spec 009's differential, which D-24 attaches to the Q1 verdict spec, is not
      among the costs of options 1 and 4." Item 11.
  V7. "option 4 states no effect on D-1, D-19 (i) or D-24; option 3 is neither of 001 §10 M3's two
      outcomes." Closed by R5 (option 1); no action.
  V8. "001 §4's outcome 'None of the above / a local variant' is not among the listed options." R5 (d).
  And from its NOT ASKED notes: (N1) for the Ivolginsky file the months test prints lunation and month
  counts only, while session 1's §2(c) says "only its lunation boundaries are compared"; the reviewer
  computed the boundary dates equal for Tsurphu and Genden in all three files, and two Phugpa
  boundaries differing (2021-04-12 / 2021-04-13, 2021-09-07 / 2021-09-08). Item 10. (N2) D-33's
  TRIGGER fired and the entry is OPEN. R8. (N3) on 2025-12-20 to 2025-12-29 the 2025 edition prints
  1 to 10 with no doubled and no skipped day, where Tsurphu and Genden double 2 (2025-12-22) and skip
  8 (before 2025-12-28): the report described this for the 2026 run only. Item 13 (d).

PREDICTIONS — written by the advisor on 2026-11-13, before this session measures anything (METHOD rule
3). Score each HELD or FAILED with your own measured value.
  12. D-24's re-run: the engine and TCG 1.06 differ on exactly D-23's 53 days (17 Phugpa, 13 Tsurphu,
      23 Genden) of 1900-01-01..2100-12-31, and on no other (basis: spec 009 §1.1; the engine source is
      unchanged since d174965, PRECONDITION (c); RECALLED).
  13. spec 009's janson_ref.py, extracted verbatim, with a2 = 1/28, labels the golden dates with the
      same tshes matches as the engine, per file and system: Phugpa 374 / 398 / 373, Tsurphu 391 / 403
      / 390, Genden 398 / 407 / 394 (basis: the advisor's run of that script, 2026-11-13, extraction
      sha256 ca2d2ede05e66044ae9d0950a6be67e58d3790a9e70fbbd6f273c9d043a53f1e, MEASURED by the advisor,
      RELAYED).
  14. In the lunation 2025-12-20 to 2026-01-18, the script's Genden true dates put the end of lunar
      day 2 at about 0.010 of a day after the mean daybreak of 2025-12-22 and the end of lunar day 8
      at about 0.990 of 2025-12-27 (about 14 minutes inside each side of a boundary); the 2025
      edition's values need the first earlier and the second later. On the golden dates, 28 lunar
      days end within 15 minutes of a daybreak (both neighbouring days in the files); on 26 of them the
      golden tshes equals Genden's on both neighbouring days, and the 2 others are those two ends of
      lunar days 2 and 8 (basis: as 13).
  15. Day-start offsets delta (Item 12), Genden mismatches over the 1,213 rows: 14 at delta 0; 69 at
      +0.0447 d and 86 at +0.0650 d (the datsans' longitudes east of Lhasa's); 77 and 99 at the same
      offsets westward; and delta 0 the unique minimum of the scan −0.15 to +0.15 by 0.01 (basis: as 13).
  16. Every assertion of Item 10 passes on the tree; the gate's count rises by the tests Item 10 adds.
  17. OPEN is 6 at the resume's base and 4 at the close (F38 and F4 ruled; F5, F7, F8, F10 remain).

ITEM 8 — The resume.
  - Check PRECONDITION and ARTIFACT, and quote every output. Run `date` once (METHOD rule 51).
  - Append this ADDENDUM to docs/specs/prompts/014-q1-verdict.md so that the committed copy is
    byte-identical to /home/kitaeza/zurkhai-prompts/spec-014-q1-verdict.md through ADDENDUM 1 (`cmp`
    silent over the whole files). That is this session's first commit. `cmp` the whole files before
    every later commit.
  - Choose this session's marker (METHOD rule 58); the report calls it "session 2's marker" and never
    spells it.
  - Re-measure the OPEN count with FORKS.md's grep (6 expected, F4, F5, F7, F8, F10, F38; RELAYED).

ITEM 9 — The records. One commit, after Item 8.
  - FORKS.md F38: Status RULED; "Ruled: 2026-11-13, (1), in the shape of spec 014 ADDENDUM 1's R5:
    Genden, scoped to the three golden/buryat/ files of two datsans, under 001 §6.5's second rule;
    the 2026 edition's 8 days recorded as a disagreement between editions, the 2025 edition's 6 as
    open; four reopening conditions (spec 014's verdict, its Reopening section). The human: 'Agree
    with all recommendations, go.'"; "Built in: spec 014 (the verdict spec, Item 13)". Change nothing
    else in the row.
  - FORKS.md F4: Status RULED; "Ruled: 2026-11-13, (b), after M3: the approach carries spec 014's
    verdict, its 6 open days and the two Aginsky editions' disagreement as its questions; the contact
    is the human's. The human: 'Agree with all recommendations, go.' (spec 014 ADDENDUM 1, R9)".
  - DEBT.md D-1: Status "PAID <date>, spec 014 (the Q1 verdict spec, FORKS.md F38 RULED (1); in force
    on main from its merge)". Append to its Statement one sentence: "A client that defaults System
    shows the system and the scope of spec 014's verdict (001 §10 M5)."
  - DEBT.md D-19 (R6): an in-place `[CORRECTED <date>: …]` that records TRIGGER (i) as fired by spec
    014, whose verdict rules out Tsurphu as A.2 computes it and did not measure the karana-Sun
    variant; the TRIGGER becomes "the first of: (ii) …; (iii) …; (iv) spec 014's verdict reopens".
    Status OPEN.
  - DEBT.md D-23 (R7): an in-place `[CORRECTED <date>: …]` of its TRIGGER's last sentence to "If an
    admitted source matches the (7.24) value of its best-matching system on a day of that system's
    own list, F25 is put to the human again, with a named variant of each system among its options."
    and one sentence recording spec 014 §2(e) (2025-11-19). Status OPEN.
  - DEBT.md D-33 (R8): an in-place `[CORRECTED <date>: …]` recording that its TRIGGER fired with spec
    014's prompt (2026-11-13) and was not acted on there (the advisor's correction K1); the TRIGGER
    becomes "the human saves any of them into the library"; the verdict's scope cites it. Status
    OPEN.
  - DEBT.md D-35: append one sentence to its Statement: "A refusal that withdraws these rows also
    reopens spec 014's Q1 verdict (its Reopening (i))."
  - DEBT.md D-24: after Item 11, an in-place `[CORRECTED <date>: …]` recording the re-run's counts and
    the commit it ran on; its TRIGGER becomes "the first round after spec 014 that edits
    crates/tshes/src/". Status OPEN (the re-run adds no oracle to the tree, F24). If STOP RULE (6)
    fires, do not touch D-24 beyond what the STOP AND ASK files.
  - DEBT.md, one new entry from the next free D id (the advisor expects D-40, RELAYED), Pinned by: not
    pinned, Status: OPEN:
      subject: How a client names the extra month of a Buryat year
      Kind: DOCS
      Statement: Spec 014 measured one extra month (n = 1): the engine flags is_leap_month on the
        first of two lunations with month 12 (Janson v1, p. 11), and the Aginsky 2025 edition prints
        «Дополнительный зимний месяц» on the second (golden/buryat/aginsky-datsan-2025.csv,
        2025-01-30 to 2025-02-28). The verdict spec records the relation and maps nothing beyond it.
      TRIGGER: the first client that shows a month's name or its leap flag.
  - The open-forks audit, with its arithmetic: 6 − 2 rulings recorded = 4.

ITEM 10 — The assertions behind the verdict's figures (C2; V2; N1). Test code only, crates/tshes/tests/;
spec 002 governs it; no new harness, the session-1 helpers are reused.
  Turn into assertions, each with a named message, every figure the verdict spec will cite that today
  rests only on printed output (session 1's report lists them; the reviewer's list is V2 and N1):
  (a) the per-system totals over the 1,213 rows (1,145 / 68, 1,184 / 29, 1,199 / 14) and over the
      1,165 distinct days (the three-way split per system);
  (b) the Сагаалган table of §2(d), all fifteen cells and the "first of month 1" verdicts; the test
      `q1_report_the_sagaalgan_days_in_each_system` asserts them or is replaced by one that does;
  (c) the lunation boundary dates (not only the counts) for every file and system, including the
      Ivolginsky file (N1);
  (d) the days that separate the systems (session 1's §2(h): 1,098 / 67; Tsurphu–Genden 14 days, 15
      rows, the source equal to Genden on 15 and to Tsurphu on 0; Phugpa–Genden 58 days, 60 rows,
      57 and 3);
  (e) that every Genden mismatch lies in the lunation 2025-12-20 to 2026-01-18, and that the files
      cover 41 distinct lunations (the advisor's count, RELAYED: 15 + 14 + 14, less the 2 the
      Aginsky editions share; re-measure it).
  Each new assertion is shown to fail (METHOD rule 31): one in-memory plant per assertion group, in
  the session-1 style. The numbers come from the tree, never from this ADDENDUM. STOP RULE (7).

ITEM 11 — D-24's re-run (V6). Spec 009 §21's record, re-run on HEAD's engine, nothing in the tree:
  - The TCG build from tcg1309.zip with §21.2's stand-in headers, at -O0 and -O2; the build check of
    §21.1 against henning-kalacakra/tdata (spec 009's years); the full ranges for the three epochs;
    readtcg.py, checkrows.py, omitcheck.py and readcheck.py as §21.1 runs them.
  - The engine dump by §21.3's probe on a `git archive` copy of HEAD, debug and release, `cmp` equal;
    the Phugpa dump `cmp`-equal to the T4 fixture.
  - diffrows.py: the days per system on which the engine and TCG differ, over 1900-01-01..2100-12-31,
    with the controls §21.1 runs. Compare the lists with D-23's 53 days, date by date.
  - Report the counts per system, every disagreeing day outside D-23's lists (none expected), and
    each script's extraction sha256. STOP RULE (6).

ITEM 12 — The day-start offset probe (C3). A description; it explains nothing and adopts nothing.
  The question: could the 6 open days come from a lunar day's end being reckoned at the datsans'
  own longitude rather than at the mean daybreak the paper's formulas assume (Janson v1, Remark 6,
  p. 20: "the true date should be regarded as a kind of local Julian date … that it instead assume
  integer values at local (mean) dawn"; Henning's daybreak at 5 a.m. local mean solar time, Lhasa at
  longitude 91°, quoted there; FETCHED by the advisor from the library copy, re-fetch it)?
  - The instrument: spec 009's janson_ref.py, extracted verbatim, called by a script of your own that
    labels every day J with the first lunar day N whose floor(true date + delta) ≥ J, the script's own
    rule (janson_ref.labels) with delta added and nothing else changed.
  - Its control, first: at delta 0 it reproduces the engine's tshes matches per file and system
    (prediction 13). If it does not, STOP: the instrument is not trusted.
  - Then: (a) for Genden, the true date's fraction of the day for every lunar day ending in
    2025-12-20..2026-01-18, and the smallest margin to a day boundary in each lunation the files
    cover, with how many lunar days of the files end within 15 minutes of a boundary and how many of
    those the sources print as Genden does; (b) Genden, Tsurphu and Phugpa mismatches per file at
    delta = 0, ±0.0447 and ±0.0650 days (Ivolginsk, about 107.2° E, and Aginskoye, about 114.5° E,
    against Lhasa's 91°; the longitudes are RECALLED / UNVERIFIED approximations, say so); (c) Genden's
    mismatches over the 1,213 rows for delta from −0.15 to +0.15 by 0.01. STOP RULE (8).
  - Report it in the verdict spec's account of the 6 open days as a measured description, with the
    instrument's blind spot (METHOD rule 35): it tests a constant offset of the day's start only, not a
    different equation, rounding or table.

ITEM 13 — The verdict spec. docs/specs/014-q1-verdict.md's spec text (above "— spec text ends here —")
becomes the verdict spec, in docs/specs/TEMPLATE.md's shape; the report follows the end marker.
  (a) The header lines: Status: accepted (the human's ruling on FORKS.md F38, 2026-11-13; in force on
      main from the merge, docs/ROUNDS.md §9). Depends on: 001, 008, 009, 013. Resolves: F38.
      Milestone: M3. Governs: the answer to Q1 (001 §4) and D-1's default; crates/tshes/tests/
      (golden/buryat/ under the gate; the Q1 differential).
  (b) Purpose; the verdict (R5 (a), (b)); its scope (R5 (c)); the 14 mismatches as R5 (d) records
      them, all listed with both values and their lunation; what the verdict rules out and what it
      leaves (R6: Tsurphu A.2 ruled out, the karana-Sun variant not measured; Phugpa's counts);
      the extra month (C1, n = 1); the Reopening section (R5 (e)); the weakness (R5 (f)).
  (c) Properties, each with its test name: the pinned measurement (the session-1 pin), and the
      Item 10 assertions, each a property the gate checks. Non-properties: what the verdict does not
      claim (no claim beyond the scope; no claim about the 6 days; no claim on any datsan outside
      the scope; nothing on a2 beyond F25).
  (d) The 6 open days described, from the tree: the 2025 edition's L13 printed 1 to 30 with no doubled
      and no skipped day where Genden doubles 2 and skips 8 (N3); the 2026 edition's 8 days; Item
      12's measurement. Describe; do not explain (METHOD rules 27 to 29).
  (e) In session 1's report: an in-place `[CORRECTED <date>: …]` note, the original kept, at each of
      the three statements V3, V4 and V5 name (§1's last paragraph, §2(e), §11 and §12's THE
      FINDING), pointing to R5 (b) and R7.
  (f) Reference: Janson v1 (A.3, Remark 6, Remark 14, Remark 22), spec 009, spec 013, with
      docs/sources.md rows.
  The report: a "Session 2 — the resume (ADDENDUM 1; Items 8 to 14)" section, in the report contract's
  order, with every property DONE / DEFERRED / N-A with evidence, every prediction (12 to 17) scored,
  R5 to R9 and C1 to C4 beside their records, V1 to V8 and N1 to N3 each answered, "What the prompt got
  wrong" (this ADDENDUM included), by-catch, and the diff scope summing against `git diff --stat main`.
  The head block: model per session ("Sonnet 5.5, effort High (session 1); Opus 5.5, effort High
  (session 2)"), and the gate line of Item 14's run.

ITEM 14 — The close.
  - docs/ROADMAP.md: the q1-verdict row of Table 1 becomes LANDED; Table 2, Gate 1's state cell
    becomes "REACHED: spec 014's verdict (Genden, scoped; FORKS.md F38 RULED (1)), in force on main
    from its merge; D-1 PAID"; a Revisions line. Report which planned rows' TRIGGERs this fires, in
    whole or in part (the website's names "the Q1 verdict spec exists (D-1), and l2-schema landed").
  - docs/specs/INDEX.md: this round's row, status `accepted`; say whether D-27's TRIGGER fires.
  - tools/gate.sh once, with nothing in flight, both profiles, on the last commit that changes a path
    beyond the report; compare its count with session 1's 174 and name every new test.
  - Account for every file `git status --porcelain` shows. The marker greps for both sessions'
    markers. The open-forks audit (4 expected).
  - "The squash commit message" at the end of the report, covering both sessions.
  - After the final commit the branch stands still: a reviewer reads its tip before the merge
    (docs/ADVISOR.md §6: the diff touches crates/).

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ADDENDUM 1 — SPEC 014 — ENDS HERE. NOTHING BELOW IS PART OF IT.      ▲▲▲
═══════════════════════════════════════════════════════════════════════════
