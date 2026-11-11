═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 008 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  008 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
BRANCH:       spec/008-genden-tsurphu, cut before any edit from main at the commit PRECONDITION names;
              commit to it, the halt included. No merge. No push.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7, first branch. The answer is discoverable:
              Janson v1 states each version as its differences from Phugpa (Appendices A.2, A.3), and
              001 §12 forbids filling a gap from anywhere else. Most wrong answers are loud: the
              exhaustive properties, the paper's printed Tsurphu and Mongolia columns (P5), and the
              paper's own cross-version statements (Item 4c) fail visibly. Three parts would be silent,
              and each runs here only with an instrument that makes it loud (docs/ROUNDS.md §8):
                - the constants of A.2 and A.3: each is printed twice, in radix form and as a fraction,
                  and the two are computed equal (Item 2), with a second pass against the page (Item 3),
                  and each is compared digit for digit with the source Janson cites for it, Henning's
                  epoch page (Item 2, R4);
                - the transcription of the new T5 columns: two routes, a seeded row per file, and the
                  paper checked against itself (Item 5);
                - the version-specific readings, each quoted with its page (Item 2).
              Unchanged from rounds 005, 006 and 007. New this round: a read-only reviewer reads the
              final tip, on Opus 5.5, before the advisor adjudicates (docs/ADVISOR.md §6, r3).
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first. It never commits a previous session's uncommitted work without
              verifying it: it finishes the work and proves it by the item's own check, or redoes it. A
              design choice that would fall to the fallback model (Opus 5.5, effort High) is a STOP AND
              ASK. The report names the switch point.
PRECONDITION: main's tip is the process commit: its subject is exactly
                ADVISOR.md r3, ROUNDS.md r2: the reviewer session at a round's close
              its only parent is 62413d193bd2df623da5ba3e1062999e4a7af501 (spec 007 merged, tree
              b5f8e943c194745308638232715dcdf1458428d3), and `git -C /home/kitaeza/github/zurkhai diff
              --name-only main~1 main` lists exactly docs/ADVISOR.md and docs/ROUNDS.md. Check with
              `git -C /home/kitaeza/github/zurkhai log -1 --format='%P%n%s' main`. Anything else: STOP
              and report, with zero diff. Committing and merging are the human's.
              Read first, in this order: CLAUDE.md; docs/ROUNDS.md (§3 and §10 are new: the branch stands
              still after the final commit, and a reviewer reads it); docs/specs/001-init-spec.md (§4,
              §5.1, §6, §10, §12); docs/specs/002-rust-code-standard.md; docs/specs/003-project-method.md;
              docs/specs/005-phugpa-core.md (the citation table, the properties, the planted controls, the
              API choices); docs/specs/006-reference-fixtures.md (the T5 transcription, P5, the controls);
              docs/METHOD.md, all of Part 1 (these rules bind this round directly: 3, 4, 7, 8, 23, 25, 31,
              34, 35, 36, 41, 42, 45, 50, 51, 56, 57, 58–60, 63–65 and 67); GATED.md; FORKS.md; DEBT.md
              (D-1, D-9, D-10, D-12, D-15 and D-17 bind this round); docs/sources.md; docs/glossary.md;
              golden/reference/README.md and its metas; tools/gate.sh.
ARTIFACT:     (A) /home/kitaeza/zurkhai-papers/janson-1401.6285v1.pdf: S. Janson, "Tibetan calendar
              mathematics", arXiv:1401.6285v1. Its sha256 must equal
              7cafc7df563a3020849c86f4e397daa18b7f7f5ff46244403180aca90b9d0f77 (docs/sources.md), and
              `pdfinfo` must say 86 pages.
              (B) /home/kitaeza/zurkhai-papers/henning-kalacakra/epochs.htm: E. Henning, "Epoch data for
              Kālacakra and Tibetan calendars", kalacakra.org/calendar/epochs.htm, fetched by the advisor
              on 2026-11-11 and placed in the library by the human. This is Janson v1's ref. [24, Epoch
              data], which A.2 and A.3 cite. Its sha256 must equal
              039d48ecec5a8c33a7ef5202d2ecf93749699cad99c5925b8798d0ff2652d0e6. The directory must hold 17
              files, with the manifest
                `find /home/kitaeza/zurkhai-papers/henning-kalacakra -type f | LC_ALL=C sort | xargs -d '\n' sha256sum | sha256sum`
              equal to 659b9347ae860b6d68ca74ad3482bfe39a439dcb4614232b6c49b654351bacf1.
              If either is absent or any check fails: STOP and report, with zero diff. Never implement from
              memory, from secondary pages, or from another version.
STOP RULE:    (1) An ARTIFACT or PRECONDITION check fails: STOP, zero diff.
              (2) A constant, formula or rule that Tsurphu's or Genden's Gregorian ↔ ZDate mapping needs
              is not stated in Janson v1: STOP AND ASK at that point. Report the section and the gap;
              substitute nothing (001 §12). Never from Henning, Salmi, Berzin, Schuh, a web page or
              memory, even where Janson cites them for it.
              (3) A property, a T5 table or a cross-version check fails, and the cause is not a coding
              slip traceable to a line against the paper: STOP AND ASK. Never change a constant to make a
              check pass (CLAUDE.md hard rule 7).
              (4) The paper disagrees with itself on a Tsurphu or Mongolia fact: a constant's radix form
              against its fraction, Table 6 against the epoch values of A.2, A.3 and §7, one table
              against another, or a table against the text. Or Janson v1 disagrees with Henning's epoch
              page (ARTIFACT (B)) on a constant. STOP AND ASK, with both readings and their pages. A
              disagreement you can explain by a coding slip is not one: fix it and say so.
              (5) Any Phugpa result changes: a Phugpa test goes red, or its row counts move. STOP AND
              ASK. This round does not change Phugpa; shared code may be reshaped only if every Phugpa
              test stays green with its counts.
              (6) A read, a network fetch or an install outside GRANTS is needed, or a permission is
              denied: STOP and report each denial with the reason given. Never work around it by another
              tool or route (METHOD rule 67). Dropping a step you added yourself, which no item asks for,
              and re-running the rest with granted tools is not a workaround: report it under "Latitude
              taken" (the human's ruling R1, below).
              (7) Out of scope; do not write any of it:
                - the Bhutanese version and every other version of Appendix A;
                - Tsurphu's karana-sun variant (Remark 22, p. 40; Item 9 files it);
                - any 001 §5.2 attribute, including the month names and animals of Appendix E.2 and of
                  A.3 (rows 6–8 are deferred, D-18);
                - golden/buryat/, the Q1 diff, and every library holding except ARTIFACT (A) and (B);
                - Henning's calendar lists and software (the rest of henning-kalacakra/). They are not
                  this round's oracle: his Tsurphu lists follow the karana variant, and a later round
                  takes his software (R3);
                - any statement about which system Buryat calendars follow. That includes Janson's
                  citation of Berzin on p. 41, which 001 §4 rules out as evidence;
                - localised names (F5 is OPEN).
              If something seems to need one of these, report it under "What the prompt got wrong".
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; there is no fourth
              kind. An unobtainable source says PAYWALLED or UNREACHABLE. Read docs/sources.md before any
              fetch. Never curl/wget. (METHOD rule 50.) A quotation from the paper in the repository is two
              sentences at most; formulas, constants, table cells and equation numbers are facts.
GATED:        yes:
                - crates/tshes/**, under accepted specs 001 (§4, §5.1, §6.4, §10 M2, §12) and 002. No new
                  dependency of any kind;
                - golden/reference/** (the new T5 files and their metas, README.md, and D-15's in-place
                  corrections), under accepted specs 001 (§6.1, §6.4) and 002 (§6);
                - this round's own docs/specs/008-genden-tsurphu.md and
                  docs/specs/prompts/008-genden-tsurphu.md, and its row in docs/specs/INDEX.md, which are
                  not changes to an accepted spec.
              No other gated path. Not gated, and changed only as the items say: docs/glossary.md,
              docs/sources.md, DEBT.md, FORKS.md.
GRANTS:       Reads outside WORKDIR, beyond this prompt file: /home/kitaeza/zurkhai-papers/ (the human adds
              it with /add-dir). In it, read only ARTIFACT (A) and (B). For Item 1's library row you may
              also list henning-kalacakra/ with `ls` and run ARTIFACT (B)'s manifest command over it; open
              no other file there. Nothing else in the library is this round's, and nothing is written
              there. The tools:
                - the Read tool, on the PDF's pages and on epochs.htm;
                - `sha256sum` and `pdfinfo`, on the PDF; `sha256sum` on epochs.htm; the manifest command;
                - `pdftotext -layout`, on the PDF, writing only into a directory made by `mktemp -d`. Its
                  output never enters the repository;
                - read-only shell tools on the repository and on your own scratch: cat, head, tail,
                  sed -n, grep, awk, cut, tr, sort, uniq, wc, diff, cmp, od, ls, stat, find, date;
                - `python3 -I`, running only scripts you write into a `mktemp -d` directory. They may read
                  the PDF's pdftotext output and epochs.htm (for instance to strip its HTML), and nothing
                  else in the library;
                - planted controls: `git -C /home/kitaeza/github/zurkhai archive HEAD | tar -x -C <dir>`
                  into a `mktemp -d` directory, then edits and cargo there with `--manifest-path`;
                - mkdir -p, cp, mv and rm -r, only inside `mktemp -d` directories and target/;
                - cargo (build, test, fmt, clippy) and tools/gate.sh.
              Network: none. Cargo works offline from the lock file and its cache; this round adds no
              dependency. On this machine `grep` is ugrep: a bounded repetition of 100 or more, such as
              `.\{0,120\}`, fails with "exceeds complexity limits"; keep bounds below 100.
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive
              widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 008 — Genden and Tsurphu

Round 008 is milestone M2 (001 §10): the engine gains the two other candidate systems of 001 §4, Tsurphu
and Mongolian New Genden ("Genden" in code, docs/glossary.md), each as Janson v1 describes it, through
the same `to_zdate` / `to_gregorian` API as Phugpa. It runs now because M3, the Q1 verdict, needs all
three systems, and M1 is complete (spec 007). M2 is done when P1–P5 pass for each against Janson's tables;
001 §6.4 says P7 must hold for every system, so it is checked too. It does NOT change Phugpa, transcribe
or read any Buryat source, run the Q1 diff, implement Bhutan or any 001 §5.2 attribute, or say anything
about Buryat practice.

THE HUMAN'S RULINGS this round carries (2026-11-11):
  R1. Round 007's second session, "Latitude taken", items 4 and 5, are accepted as reported, with no grant
      after the fact: (4) a denied command chain re-run without the session's own needless `rm -rf`, and
      (5) read-only tools beyond the addendum's GRANTS, none refused. The human: "Agree, go". Record it in
      Item 0's account. Round 007 is closed, and its file is not edited.
  R2. The reviewer. A read-only reviewer session runs by default at the close of every round that changes
      crates/ or golden/ (docs/ADVISOR.md §6, r3; docs/ROUNDS.md §10, r2; the human: "Ok agree with all
      recommends"). For this round it means one thing: after your final commit the branch stands still
      (docs/ROUNDS.md §3). Do nothing for the reviewer, and never read or answer its report.
  R3. Which Tsurphu the engine implements: Janson v1's A.2. The karana-Sun variant of Remark 22 (p. 40)
      is deferred. The human: "Agree with all recommendations". Item 9 files it as F23, RULED (a), and
      files the variant's DEBT entry. The advisor's reasons, RELAYED (measured 2026-11-11, not premises
      for this round): Henning's published Tsurphu calendars (kalacakra.org) are made by software that
      takes the Sun's longitude from the karana calculation. His 2012 file differs from Janson v1's
      Table 8 Tsurphu column in months 4, 5 and 11, with every month length equal. So those files are a
      different Tsurphu from Janson's, and a later round, not this one, takes his software.
  R4. The epoch cross-check: every constant this round takes from A.2 and A.3 is compared digit for digit
      with Henning's epoch page (ARTIFACT (B)), the source Janson v1 cites for them. The page gets no
      tier and gives no golden row. The human: "Agree with all recommendations".

ITEM 0 — Branch, prompt, number, baseline.
  - Check PRECONDITION and ARTIFACT, and quote both outputs.
  - Create spec/008-genden-tsurphu from main's tip.
  - Commit this file, byte-identical, as docs/specs/prompts/008-genden-tsurphu.md. That is the first
    commit. `cmp` it against /home/kitaeza/zurkhai-prompts/spec-008-genden-tsurphu.md before every later
    commit.
  - Verify that 008 is free in the tree.
  - Read the Read-first list.
  - Run `date` once and use that value for every date you write (METHOD rule 51). If a later `date`
    shows a new day, the records written after it carry the new date, and the report says where the date
    changed.
  - Choose your round's marker (METHOD rule 58). The report calls it "the round's marker" and never
    spells it.
  - Re-measure the OPEN count in FORKS.md with the grep its header defines. The advisor counted 6 on
    main (RELAYED: re-measure it, do not copy it).
  - The baseline: run tools/gate.sh once, before any edit, on the prompt commit, and record its legs and
    counts. Round 007's gate gave 84 passed, 0 failed in each profile (RELAYED; the process commit changes
    no gate input). Every count you report as a change is measured against this run (METHOD rule 4).

ITEM 1 — The reading.
  - Read Appendix A.2 (pp. 38–40), A.3 (pp. 40–43), A.13 with Tables 6–9 (pp. 50–54), Appendix C.3 and
    C.4 (pp. 62–63), and every section those pages send you to for the mapping: §5 (the true month, the
    leap rules and their general form, pp. 11–19), §7 and §8 (pp. 21–28).
  - List, in the report, each page you read and what it gave this round.
  - The PDF's docs/sources.md row exists. If you cite Janson v1 for a fact its "Trusted for" cell does
    not cover, say so under "What the prompt got wrong".
  - docs/sources.md, in the commit that first cites ARTIFACT (B):
    - one library row for henning-kalacakra/ (a directory), in the table's column shape, with:
      - its contents as `ls` shows them;
      - "Obtained": fetched by the advisor from kalacakra.org on 2026-11-11 and placed by the human
        (RELAYED);
      - the file count and manifest of ARTIFACT (B), and epochs.htm's own sha256;
      - "Licence": not checked;
      - "Trusted for": epochs.htm, as a witness of the constants Janson v1 cites it for (R4); nothing
        else. Add the advisor's two findings, as RELAYED, so that no later round trusts the lists as
        oracles: the Tsurphu lists follow the karana variant (R3), and the Phugpa lists differ from the
        T4 fixture on 2 of 2,159 sampled days (2001-02-10 and 2025-11-19);
    - the Bibliography's Henning row: its "In the library" cell becomes "kalacakra.org pages:
      henning-kalacakra/; the 2007 book: no". That edits the bibliography, so DEBT.md D-17's TRIGGER
      fires: Item 9.

ITEM 2 — THE DIFFERENCES TABLE (the decisive item, with P5 in Item 6).
  One row for every quantity or rule where Tsurphu's or Genden's mapping differs from Phugpa's, as Janson v1
  states it. Columns:
    - the quantity;
    - Phugpa's value, as round 005's citation table gives it (RECALLED: re-read it on its page);
    - Tsurphu's value;
    - Genden's value;
    - Janson v1 section, equation and page for each;
    - the second-pass verdict (Item 3).
  The rows include at least:
    - the epoch: JD, m0, s0 and a0. For Tsurphu, both epochs the paper gives (E1732 and E1852);
    - the epoch value of the intercalation index, β*;
    - the leap rule, with every modification the paper names. For Genden, that includes the change to
      (5.9) and to Remark 4 (p. 43);
    - the constants of the general leap-month form, with their values for each version (pp. 40, 43);
    - every rule of §5, §7 or §8 that Phugpa uses and the paper changes for either version.
  Add one row per Phugpa formula the paper says is unchanged ("the same values … for mean motions",
  pp. 38, 42), citing that sentence. A formula that differs and is not in this table is the round's
  silent failure. So, after the code is written, list every place the code branches on the system, and
  match each one to a row.
  The radix check: every constant A.2 and A.3 print in radix form AND as a fraction (A.1–A.8, A.11–A.16)
  is computed both ways, in exact rational arithmetic, in a test, and the two must be equal. A
  disagreement is STOP RULE (4).
  The epoch cross-check (R4): for each epoch this round uses or tests (Tsurphu 1732 and 1852, New Genden
  1747), compare with Henning's entry for the same epoch, digit for digit: the JD and the date, the true
  month's constant (β*), the mean weekday (m0), the anomaly (a0) and the mean sun (s0). Henning writes the
  quantities in his own form (for instance "M × 1;31,50,0,480 + …" and an "Equivalent to" line), so
  state how each of his lines maps to Janson's symbols. Report a table: quantity; Janson v1 (equation,
  page); Henning (the line as written); verdict. A disagreement is STOP RULE (4).
  Readings to state, each with its page and a quotation of two sentences at most:
    a. The identification. 001's Genden, the glossary's "New Genden", and the column headed "Mongolia" in
       Tables 6–9 are one version: cite where Janson v1 says so.
    b. Tsurphu's two epochs. Which one the engine uses (your choice: say why), and that the paper says
       the other "gives the same calendar" (p. 39). Item 4c checks it.
    c. The leap month's number in each version: the number of the following month, or of the preceding
       one. Cite where Janson v1 decides it for Tsurphu and for Genden, or that its general rule covers
       them and where. If the paper does not decide it for either version: STOP RULE (2).
    d. The first day of the year in each version: cite it, as round 006 did for Phugpa.
    e. Remark 28 (pp. 62–63) increases Tsurphu's s0 by 1 for the definition points of Appendix C. State
       whether that changes any value the mapping computes, and show it.
    f. Remark 22 (p. 40): some Tsurphu almanacs compute the true date with the karana solar equation.
       By the human's ruling (R3), this round implements A.2. The variant is out of scope (STOP RULE (7);
       Item 9 files it). If you find that the paper's Tables 6–9 use the variant: STOP RULE (4).

ITEM 3 — Tsurphu and Genden, Gregorian ↔ ZDate (001 §5.1; 002 §4, §5).
  - The API is round 005's: `to_zdate(gregorian, system)` and `to_gregorian(zdate) -> Result<_, Error>`,
    via JDN, with `System` explicit everywhere and no default (DEBT.md D-1). A ZDate carries its system,
    and `to_gregorian` uses it.
  - The shape is your choice: report it as "considered X, chose Y because Z". The paper describes each
    version as Phugpa plus its differences, so one shared mapping with per-version data is the
    expected shape.
    - Every public item you add beyond the two functions and the `System` variants is Latitude, and goes
      onto D-12 (Item 9).
    - `Error::SystemNotImplemented` has no use once all three systems compute. Keeping it, or removing
      it from this `#[non_exhaustive]` enum, is your choice: report it.
  - Code standard, per 002 §4–§6, as round 005 applied it (no_std, no alloc, zero dependencies, the named
    division helpers, i64 with i128 only where a bound is stated, no panics on valid input). Every formula
    and constant carries `// Janson (arXiv:1401.6285v1) §<section>, eq. (<n>), p. <page>` (002 §4.8).
  - THE CITATION TABLE, in the report, in round 005's columns, for every constant and formula this round
    adds or changes. The second pass happens after the code is written: re-open each cited page and
    compare it against the code. MATCH or MISMATCH; a MISMATCH is fixed and re-checked before the round
    lands. The advisor re-checks the table against the library copy at landing.
  - DEBT.md D-9: delete the two tests that pin the gap (`genden_not_yet_supported`,
    `tsurphu_not_yet_supported`). This round's tests replace them, as D-9's TRIGGER says (Item 9).

ITEM 4 — Properties, and the paper's cross-version statements (001 §6.4; 002 §6; METHOD rules 31, 34, 35,
41, 42).
  a. For EACH of Tsurphu and Genden, as round 005 ran them for Phugpa:
     - P1–P3 and P4's month lengths, EXHAUSTIVELY over every day from 1900-01-01 to 2100-12-31, in both
       profiles. P2 runs in both directions;
     - P4's frequency: the expected number of leap months in the range, derived from that version's own
       rule with its page, against the measured count;
     - P7, and arbitrary ZDate fields, by proptest with boundary-weighted strategies: the behaviour outside
       the range is defined, stated and asserted; SkippedDay and InvalidField as for Phugpa;
     - the shape-coverage counts (years; months split into leap, 29-day and 30-day; skipped tshes; doubled
       tshes; 12 → 1 rollovers; rabjung-cycle boundaries), and the identity skipped − doubled =
       30 × (complete months) − (days in them), shown summing.
  b. METHOD rule 35: P1–P4 check consistency, not correctness. A wrong epoch passes them all. The report
     says so. The correctness checks are P5 (Item 6) and c below.
  c. The paper's own statements about the versions, computed through the engine over the whole range, in
     both profiles. These are this round's only correctness checks beyond the printed tables, so state
     each one's exact scope: the days or years it covers, the ones it excludes, and why, with counts.
       c1. Tsurphu: the two epochs give the same calendar (p. 39). Every day of the range is equal under
           both.
       c2. Tsurphu and Genden have the same leap months (pp. 43, 51): for every Tibetan year of the range,
           the same leap month, or none.
       c3. "When the Phugpa and Tsurphu dates of a calendar day differ, the Phugpa date is always larger
           (by 1)" (p. 51). Define "date" from the paper's own terms, so that months whose numbering the
           two versions' different leap months shift are compared correctly. Report the count of differing
           days. The paper's estimate is "about one day in 20" (p. 50): report the ratio, and make no
           assertion on it.
       c4. Tsurphu and Genden New Year: "the last time these versions had different New Year was 1900
           (31/1 vs 1/2) and the next will be 2161" (p. 52). Within the range, the first days of the
           Tibetan years are equal in every year except 1900, and 1900's two dates are the ones printed.
       c5. Report, with no assertion: the number of days on which Tsurphu and Genden differ (the paper:
           "only a few days each year", p. 51), and on which Genden and Phugpa differ.
  d. Planted controls (rule 31), on a `git archive` copy of the committed tree only, never on the working
     tree. Each runs three times and must turn the named tests red three runs in three. Report the first
     failing assertion by name:
       - Genden computed with Tsurphu's epoch values;
       - Tsurphu computed with Genden's epoch values;
       - for each version, its leap rule's intercalation-index pair shifted by one;
       - one control of your own on the code c1–c4 run through.
     For each, say in advance which tests you expect to go red, and score it.

ITEM 5 — T5: the Tsurphu and Mongolia columns of Janson v1's Tables 6–9 (001 §6.1; spec 006's method).
  The files:
    golden/reference/janson-v1-table6-epochs-806.csv
      system              (phugpa | tsurphu | genden; Table 6's Phugpa, Tsurphu and Mongolia columns;
                           the Bhutan column is not transcribed, and the meta's covers: says so)
      the values of m0, s0 and a0, in radix form and in decimals, as printed (p. 50). No field may hold a
      comma (README.md's rule), so choose an encoding of the radix digits, keep their printed order, and
      state it in README.md.
    golden/reference/janson-v1-table7-leap-months-tsurphu.csv and …-genden.csv
      year, leap_month    (as the Phugpa file: one row per year printed; empty where the cell is empty)
    golden/reference/janson-v1-table8-2012-tsurphu.csv and …-genden.csv
      tibetan_year, month, tshes, kind    (as the Phugpa file)
    golden/reference/janson-v1-table9-losar-tsurphu.csv and …-genden.csv
      tibetan_year, gregorian_date        (as the Phugpa file)
  The `-genden` files transcribe the column headed "Mongolia"; each meta's location names the column
  heading as printed.
  a. First pass: from the page as the Read tool renders it.
  b. Second pass, by the other route: `pdftotext -layout` into the mktemp directory, compared cell by cell
     against the first pass by a script, not by eye. Every disagreement is settled against the rendered
     page and listed in the report with both readings. No cell is silently fixed (001 §6.3).
  c. The metas, in README.md's shape. Each new meta's `method:` gives the second-pass command, or names
     where the report records the script verbatim. That is the D-15 fix, applied to the new files.
  d. The readings round 006 stated for Phugpa, restated here with their pages for the new columns:
     Table 7's year column read as the Tibetan year (check that it gives the same rows as reading it as
     the Gregorian year in which the leap month begins; if not, STOP RULE (4)); Table 8's completeness
     and the months it covers; which day Tables 6 and 9 give. (For Table 6: the mean new moon of JD
     2015531, and the "modulo 7" form of m0, p. 50.)
  e. The paper against itself. The test code computes each check from the fixtures and the paper's own
     formulas, never from the engine. Each must agree; if one does not, STOP RULE (4):
       - Table 9: Tsurphu equals Genden on every row (p. 52);
       - Table 7: Tsurphu equals Genden on every row (p. 51);
       - Table 7 against the general leap-month form with each version's constants (pp. 19, 40, 43);
       - Table 8 against Table 9, for each of the three versions: 30 × 12 − skipped + repeated equals the
         number of days from Losar 2012 to Losar 2013;
       - Table 8: "each month has the same length in all four versions" (p. 51), for the three versions
         this project has: per month, 30 − skipped + repeated is equal across them (the Phugpa file is
         round 006's);
       - Table 6 against the epochs: each version's epoch values of A.2, A.3 and §7, carried to JD
         2015531 by the mean motions in exact rational arithmetic, against Table 6's printed values.
         State the one rule (truncation or rounding at the last printed radix, with your reason) that you
         apply to all nine values. If no single rule fits all nine: STOP RULE (4).
  f. golden/reference/README.md: rows for the seven new files, and Table 6's encoding. Its first
     paragraph says the fixtures "check the engine's Phugpa arithmetic only"; it now covers three
     versions, and Tables 6–9 still never count as Buryat evidence (001 §6.1). Its F9 sentence calls F9
     "still open", which is false since 2026-11-11 (F9 RULED (b), FORKS.md). Correct it to say what the
     ruling decides and that it governs golden/buryat/, not these files. Name both edits in the report
     (CLAUDE.md hard rule 19).
  g. DEBT.md D-15: correct the three `method:` lines of round 006's Table 7, 8 and 9 metas in place, each
     with a `[CORRECTED yyyy-mm-dd: …]` record that keeps the original line (GATED.md, golden/**). The
     second-pass commands are not in the tree, so the corrected lines say they were not kept, unless you
     find them in the tree. D-15 is PAID in the same commit.

ITEM 6 — P5 for Tsurphu and Genden (001 §6.4: "Reproduces the system's own published tables (T5)
exactly"), in crates/tshes/tests/, through the public API (System::Tsurphu and System::Genden named
explicitly; D-1), parsed with std in the test code. No dependency is added.
  - Table 9: for each row, the engine's first day of Tibetan year Y in that version equals the row's date.
  - Table 7: for each year printed, the engine's leap month in that version equals the cell, and so does
    its absence.
  - Table 8: for each month of Tibetan year 2012, the engine's sets of repeated and of skipped tshes in
    that version equal the file's, including the months the file gives no entry for. Months 9 and 11 are
    where the printed Tsurphu and Mongolia columns differ (the advisor's reading, RELAYED: check it on the
    page). Say in the report how many printed cells separate Tsurphu from Genden in all of Tables 6–9:
    that number is how far P5 can tell the two apart.
  - Table 6: each version's epoch values at JD 2015531, as the engine holds them, equal the file's, under
    the rule of Item 5e. This test also runs the Phugpa row. If it fails for Phugpa: STOP RULE (5). If
    this test needs more than the public API, put it inside the crate (`#[cfg(test)]`) and say so;
    do not widen the public API for it.
  - A fixture test that loads zero rows fails (002 §6), and each test asserts the row count it loaded
    against the count the transcription measured.
  - Seeded rows (002 §6; METHOD rule 36): one per new file, one field deliberately wrong, fed through the
    same comparison, asserted to be reported as a mismatch, with the assertion's message named.
  - METHOD rule 25: Table 9 decides only the first day of a year, and its tests claim nothing about other
    days. Say so in the report.

ITEM 7 — 001 §5.3 for the two versions.
  For each quantity this round adds, a verdict with its page: VERIFIED (the paper says it does not depend
  on location or time zone), REFUTED (it says it does: STOP AND ASK before writing any parameter), or NOT
  DECIDED BY THE PAPER. Round 005's verdicts for Phugpa are RECALLED: say whether anything in A.2 or A.3
  changes them.

ITEM 8 — The glossary.
  Update docs/glossary.md: the Genden and Tsurphu rows no longer say "not implemented"; the month-numbering
  note says what Item 2c found for each version, with its page. Every new term in code (for instance the
  intercalation index's epoch value) gets one spelling, from the source, with its page (002 §8).

ITEM 9 — The registers.
  - D-9: PAID for both systems, with the date and the commit, once the two pinning tests are replaced.
  - D-10: say whether any file this round writes takes a weekday from Janson v1 (A.2 and A.3 print
    weekdays beside their epochs). If one does, check that weekday against eq. (9.1) and Table 5, p. 28,
    and add a dated line to D-10. It stays OPEN unless the page-1 discrepancy is explained.
  - D-12: if Item 3 made anything public, add a dated line naming each item.
  - A new entry, Kind COVERAGE: Tsurphu's karana-Sun variant (Remark 22, p. 40) is not implemented. The
    engine's Tsurphu is A.2's (R3). The statement carries R3's reasons, as RELAYED. TRIGGER, the first of:
    (i) a Q1 verdict spec would rule Tsurphu out, because ruling out A.2 alone does not rule out the
    variant; (ii) the Q1 differential report (M3) finds Tsurphu the best-matching system for any source;
    (iii) an admitted source names a Karma Kagyu almanac as its basis.
  - D-17: its TRIGGER fires on Item 1's bibliography edit. Pay it with one Bibliography row for the work
    Janson v1 names as one of its two main sources: Janson's ref. [12] (p. 86; D-17 gives the
    citation). "Trusted for": nothing yet. "In the library": no. Mark D-17 PAID, with the date and the
    commit. List no other work of Schuh's: D-17 leaves the rest to the round that needs one.
  - FORKS.md: file F23, "Which Tsurphu the engine implements (spec 008 R3)", already RULED, in the
    register's shape, as round 007 filed F22. Options: (a) A.2, the variant deferred; (b) both, the
    variant as a fourth system; (c) the variant only. Status: RULED. Ruled: 2026-11-11, (a); the
    human: "Agree with all recommendations" (spec 008 R3). Check first that F23 is the next free number;
    if it is not, take the next free one and say so. Also file a row for any STOP AND ASK this round
    raises, in the same commit.

ITEM 10 — The gate, last.
  - Before the final commit, grep 001 and 002 for forks with no ruling and compare them with FORKS.md
    (METHOD rule 59); name the grep.
  - Then run tools/gate.sh on the final committed tree, in both profiles, with no edits in flight (METHOD
    rule 65; CLAUDE.md hard rule 12). If any tracked file the gate reads changes after the gate has run,
    run it again on the new tip. A report-only change does not need a re-run: show with `git diff --stat`
    that no gate input changed between the gated commit and the tip, and write that command and its
    output into the report.
  - The gate line follows docs/ROUNDS.md §4.1: counts, never "green", every skipped leg named. Quote the
    toolchain each leg ran on, from the tool's own output (METHOD rule 23). State the count change against
    Item 0's baseline.
  - The head block's `golden:` line counts rows added per file and the metas corrected.
  - Your row in docs/specs/INDEX.md goes in the final commit, not before (METHOD rule 8).
  - After the final commit the branch stands still (R2).

RESEARCH LEGS: not applicable. 003 §4 sets their precondition as Q1, every L2 category and any new
attribute. This round implements 001 §5.1 for two more systems, from the source 001 §9 names. The report
says so.

PREDICTIONS — written by the advisor before any measurement (METHOD rule 3). The counts come from the
advisor's own reading of pp. 50–54 (pdftotext, 2026-11-11), RELAYED. They are predictions, not premises,
and your transcription never uses them. Score each HELD or FAILED, with the measured value.
  1. Janson v1 states every constant and rule both mappings need. Item 2c is decided by the paper's
     general rule on p. 11 (a leap month takes the number of the following month; footnote 14 and A.4 give
     Bhutan as the exception), and STOP RULE (2) is not hit.
  2. The radix check holds for every constant printed both ways in A.2 and A.3.
  3. Table 7 prints 21 years, 2000–2020. Tsurphu has a leap month in 8 of them, and the Genden column
     equals Tsurphu's on all 21.
  4. Table 8 has 21 events in each of the Tsurphu and Mongolia columns. The two columns differ in months 9
     and 11 only, by one cell each: in month 9 the repeated day, in month 11 the first skipped day.
  5. Table 9 prints 31 years, 2000–2030. Tsurphu equals Genden on all 31, and differs from Phugpa in 8.
  6. Item 5e's 2012 check: 360 − 13 + 8 = 355 for Tsurphu and for Genden, and 360 − 14 + 9 = 355 for
     Phugpa.
  7. Table 6's nine values are reproduced from the epochs by one rule, for all three versions.
  8. P1–P3 hold on every day of the range for both versions, in both profiles, with zero failures; P2
     holds in both directions.
  9. Leap months in the range: Tsurphu's count equals Genden's, and each is within 1 of Phugpa's (75,
     round 005's measurement, RECALLED).
  10. c1 holds on every day; c2 holds in every year; c4 holds, with 1900 the only differing year in the
      range.
  11. c3 holds on every day it covers. Phugpa and Tsurphu differ on about one day in 20: between 3% and 7%
      of the days in the range.
  12. P5 passes for both versions on every row of the seven new files, and the Table 6 test passes for
      Phugpa.
  13. Both epoch-swap controls turn the Table 8 test of the version they alter red. The Genden one goes red
      on months 9 and 11 only.
  14. Every Phugpa test stays green with its counts, and STOP RULE (5) is not hit.
  15. No fork is raised. F23 is filed already RULED, so OPEN is 6 at the final commit.
  16. Every gate leg that runs passes, wasm-pack is skipped as before, and both profiles' test counts rise
      above Item 0's baseline.
  17. Henning's epoch page agrees with Janson v1 on every quantity of the epoch cross-check, for Tsurphu
      1732, Tsurphu 1852 and New Genden 1747. This is the advisor's reading of both, RELAYED.

THE REPORT — per CLAUDE.md's report contract and docs/ROUNDS.md §4, in docs/specs/008-genden-tsurphu.md:
the head block; a report-shaped spec (docs/specs/TEMPLATE.md's header lines — Milestone M2; Governs:
crates/tshes, golden/reference/, docs/glossary.md — then the end marker); then the report.
  - Lead with the decisive item: THE DIFFERENCES TABLE with its second-pass verdicts and the branch-to-row
    match, then P5 for both versions, Table 8's months 9 and 11 first. Then say how many printed cells
    separate Tsurphu from Genden.
  - Then:
    - P1–P4 and P7 for each version, each DONE / DEFERRED / N-A with evidence, and the shape-coverage
      counts;
    - P6: N-A (T4 is Phugpa only, 001 §6.1);
    - Item 4c's checks, each with its scope and counts;
    - the planted controls' table (control; mutation; expected red tests; observed; verdict);
    - the transcription's two-pass disagreements;
    - Item 5e's results;
    - Item 7's verdicts.
  - Every clause of 002 §4–§6 that this round's code touches, scored.
  - The epoch cross-check table (R4), beside THE DIFFERENCES TABLE.
  - The predictions, scored. D-1, D-9, D-10, D-12, D-15 and D-17's states; F23. The open-forks audit,
    with its arithmetic at every commit. The marker greps, at every commit.
  - Every other section of the contract, "What the prompt got wrong" included.
  - The diff scope with its totals shown summing against `git diff --stat main`, at the final tip.
  - The squash commit message (docs/ROUNDS.md §4.2, item 13), naming every FORKS.md and DEBT.md entry the
    round adds or changes.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 008 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════
