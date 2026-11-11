═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 007 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  007 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
BRANCH:       spec/007-to-verify-attributes, cut from main at 45efbbc before any edit; commit to it, the
              halt included. No merge. No push.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7, first branch. This half of the round is
              discoverable: it reads one paper and one library, searches them, and computes the paper's
              formulas against the paper's own printed data. It builds nothing and decides nothing. Its
              wrong answers would be silent (docs/ROUNDS.md §8: a citation to Janson is silent), and each
              runs on the smaller model only with an instrument that makes it loud:
                - a definition missed ("not in Janson" where it is there). Instruments: the paper's
                  section list as the denominator, a term search whose controls show it can find a term
                  and can miss one (Item 2), and every absence stated with its search (Item 3);
                - a citation wrong. Instrument: every cited formula read by two routes, the rendered page
                  and the pdftotext text (Item 3);
                - a formula misread. Instrument: the paper against itself, each check with a seeded wrong
                  value (Item 4);
                - a project term identified with a quantity of the paper by its name alone. Instrument:
                  the identification is stated by structure, labelled, and left to the human (Items 3
                  and 7; METHOD rule 48).
              Every choice this half would otherwise make — what 001's words mean, which source to trust,
              what to implement — goes to the human at the halt (Item 7), so no invented answer is left
              to this model. No change from the last round: round 006 also ran on Sonnet 5.5, effort High.
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first. It never commits a previous session's uncommitted work without
              verifying it: it finishes the work and proves it by the item's own check, or redoes it. A
              design choice that would fall to the fallback model (Opus 5.5, effort High) is a STOP AND
              ASK. The report names the switch point.
PRECONDITION: main at 45efbbc (spec 006 merged: the reference fixtures, tree 73e135b). Anything else: STOP
              and report, with zero diff. Merging is the human's.
              Read first, in this order: CLAUDE.md; docs/ROUNDS.md; docs/specs/001-init-spec.md (§1–§5,
              §6.1, §10, §11, §12 and Appendix A); docs/specs/002-rust-code-standard.md (§4.7, §8);
              docs/specs/003-project-method.md (§2, §3, §4, §7); docs/specs/006-reference-fixtures.md
              (R2's table and the paragraph after it, Item 3's two passes, "What the prompt got wrong");
              docs/METHOD.md, all of Part 1 (these rules bind this round directly: 1, 3, 11, 12, 13, 15,
              25, 31, 35, 36, 46, 47, 48, 50, 51, 56, 58, 59, 60, 66, 67) and Part 2's P3; GATED.md;
              FORKS.md; DEBT.md; docs/sources.md; docs/glossary.md.
ARTIFACT:     (A) /home/kitaeza/zurkhai-papers/janson-1401.6285v1.pdf. Its sha256 must equal the value in
              docs/sources.md's library row (7cafc7df563a3020849c86f4e397daa18b7f7f5ff46244403180aca90b9d0f77).
              (B) /home/kitaeza/zurkhai-papers/TibetanDateCalcualtor/ (the spelling is the project's own):
              HEAD must be 97d4a004e74551983dfe0f88cd9fbbd2d646b0f1, `git status --porcelain` empty, and
              the sha256 of dist/index.js must equal the value in docs/sources.md's library row
              (87e5dc87f8b7154f252d035360aedad5ca973dd33d1b5ad77463063fb9b5cfba).
              If either check fails: STOP and report, with zero diff. Never reconstruct a definition, a
              formula or a table from memory, from secondary pages or from another version of the paper.
STOP RULE:    (1) An ARTIFACT check fails: STOP, zero diff (above).
              (2) Item 7 is a STOP AND ASK by design. The round halts there, whatever Items 1–6 found. It
              implements nothing: 001 §5.2 says "Rows marked TO VERIFY are not implemented until a source
              is cited in the PR", and what to implement from the source found is the human's ruling.
              (3) The paper disagrees with itself in an Item 4 check: file a DEBT.md entry (Kind:
              PROVENANCE) in the next commit, and carry it into Item 7 as its own STOP AND ASK. The other
              items continue.
              (4) A change under crates/, golden/, rules/, clients/, docs/glossary.md, docs/METHOD.md,
              GATED.md or an accepted spec seems needed: write it down for Item 7, and do not make it.
              This half writes records and a report, nothing else.
              (5) A read, run, fetch or install outside GRANTS is needed, or a permission is denied: STOP
              and report each, with its reason. Never work around it by another tool or route (METHOD
              rule 67).
              (6) Out of scope, and not written: implementing any attribute; any glossary entry;
              localised names (F5 is OPEN); any interpretation of an attribute (what it is favourable or
              unfavourable for); the Tsurphu, Genden, Mongolian and Bhutanese variants of any attribute,
              beyond noting where Janson gives one; golden/buryat/; rules/; deciding any fork. If
              something seems to need one of these, report it under "What the prompt got wrong".
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; there is no fourth
              kind. An unobtainable source says PAYWALLED or UNREACHABLE. Read docs/sources.md before any
              fetch; this round fetches nothing. Never curl/wget. (METHOD rule 50.) A quotation from a
              source in the repository is two sentences at most.
GATED:        no gated path, except this round's own docs/specs/007-to-verify-attributes.md and
              docs/specs/prompts/007-to-verify-attributes.md, which are not changes to an accepted spec.
              The round's row in docs/specs/INDEX.md lands in its final commit, after the resume, not at
              the halt. Nothing under crates/ or golden/ is touched.
GRANTS:       Reads outside WORKDIR, beyond this prompt file: /home/kitaeza/zurkhai-papers/ (the human
              adds it with /add-dir). Nothing is written there. The tools:
                - the Read tool, for the PDF's pages and for the library's files;
                - `sha256sum`, on the PDF and on the library's dist/index.js;
                - `pdfinfo`, on the PDF, for its page count;
                - `pdftotext -layout`, on the PDF, writing only into a directory made by `mktemp -d`.
                  Its output never enters the repository;
                - `ls`, `grep` and `wc`, on /home/kitaeza/zurkhai-papers/ and on the mktemp directory;
                - read-only `git -C /home/kitaeza/zurkhai-papers/TibetanDateCalcualtor`: rev-parse and
                  status;
                - `awk` and `python3 -I`, running only this round's own scratch commands and scripts, on
                  files in the mktemp directory. No script enters the repository. Each check's command
                  is recorded verbatim in the report, with its output, so that it can be re-run.
              No `node`: this round does not run the library. `cargo` is not needed in this half.
              Network: none.
              Scratch: directories made by `mktemp -d`.
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive
              widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 007 — the TO-VERIFY attributes of 001 §5.2: what Janson v1 defines (Appendix A, Task 6)

Round 007 is M1's fourth and last round. It covers 001 Appendix A, Task 6, as the human split the tasks
on 2026-10-02: "For each row in spec §5.2 marked TO VERIFY: find whether Janson defines it. Report
section or "not in Janson". Implement only those found." M1 is done when P1–P7 pass, the gate passes,
and the TO-VERIFY rows are "resolved or explicitly deferred" (001 §10); rounds 004–006 did the rest.
The round runs in two halves on one branch. This first half reads, searches and measures, and halts
by design at Item 7 with the choices the paper leaves open; the human rules on them, an ADDENDUM
carries the rulings, and a resume implements what the rulings admit, or nothing. This half does NOT
change the engine, golden/ or the glossary, and decides no fork.

THE HUMAN'S RULINGS this round carries (2026-11-11, "agree", at round 006's landing):
  R1. A DEBT.md entry (Kind: DOCS) for the `method:` lines of three of round 006's meta files
      (Item 0).
  R2. Round 006 session 2's "Latitude taken" items 4, 6 and 9 (`node --check` and `node -e` outside
      GRANTS; plant copies under the scratchpad instead of `mktemp -d`; a subshell `cd` and a `;`
      before a commit) are accepted as reported. No grant is made after the fact.

ITEM 0 — Branch, prompt, number, rulings.
  - Create spec/007-to-verify-attributes from 45efbbc.
  - Commit this file, byte-identical, as docs/specs/prompts/007-to-verify-attributes.md. That is the
    first commit. `cmp` it against /home/kitaeza/zurkhai-prompts/spec-007-to-verify-attributes.md before
    every later commit.
  - Verify that 007 is free in the tree.
  - Read the Read-first list.
  - Run `date` once, at the top, and use that value for every date this session writes (METHOD rule
    51). If the clock passes midnight during the session, the records keep the value read at the top,
    and the report says so.
  - Choose your round's marker (METHOD rule 58). The report calls it "the round's marker" and never
    spells it.
  - Re-measure the OPEN count in FORKS.md with the grep its header defines. The advisor measured 7 at
    45efbbc (RELAYED: re-measure it, do not copy it).
  - R1. Read the `method:` line of each of the four golden/reference/janson-v1-*.meta.yaml files, and
    the sentence of docs/specs/006-reference-fixtures.md that begins "The second-pass commands". The
    advisor's reading (RELAYED, 2026-11-11):
      - the Table 7, Table 8 and Table 9 lines carry Table 1's conversion step ("the printed day/month
        and the year to an ISO date; the name split into element and animal; sorted by year"). Neither
        half fits Tables 7 and 8, which hold no dates and no names; the names half does not fit
        Table 9;
      - the 006 report says the second-pass commands "are the awk lines recorded in each meta's
        `method:`", but the metas describe the parse in prose, and the scripts were not kept, so the
        second pass cannot be re-run from the tree.
    Confirm or correct each part against the files. Then file one DEBT.md entry in the file's entry
    shape, with these fields:
      Kind: DOCS
      Statement: what you found, file by file; that the data rows are not in question (RELAYED: at
        the 006 landing the advisor re-measured all 195 T5 rows against its own pdftotext parse and
        against the T4 fixture); that the second pass cannot be re-run from the tree
      Pinned by: not pinned
      TRIGGER: the next round that writes golden/reference/
    The fix it names: the three `method:` lines corrected in place, each with a `[CORRECTED
    yyyy-mm-dd: …]` record (GATED.md, golden/**). This round does not make it.
  - R2. Record it in the report's account of Item 0. Nothing else changes.
  - These records land in the commit after the prompt.

ITEM 1 — The sources.
  - Verify ARTIFACT (A) and (B) as stated in the header. Record `pdfinfo`'s page count.
  - Read docs/sources.md. For each source that 003 §4's legs name (datsan publications; a зурхайч;
    Lotsawa House; Henning's software; existing Mongolian apps; Janson; Henning; Schuh;
    Reingold–Dershowitz), state whether the paper library holds it, and its label. This round fetches
    nothing.
  - The Janson row and the library row need no change unless the checks find otherwise. Say which.

ITEM 2 — The search instrument, and its controls (METHOD rules 15, 31).
  - Run `pdftotext -layout` on the whole PDF into the mktemp directory. Make a second copy with the
    line breaks joined and end-of-line hyphens removed, so that a word split across two lines is
    found. Search both copies and report both counts.
  - List the paper's sections and appendices with their pages, from its table of contents or, if it
    has none, from the headings in the text. Report the list. It is the denominator for every absence
    in Item 3.
  - The term list. Start from the terms below, one per line. Add every term your reading shows the
    paper uses for these quantities. Report the final list with the hits and the pages per term.
      element
      animal
      trigram
      nine
      colour
      magic square
      mewa
      sme ba
      sme-ba
      parkha
      spar kha
      spar-kha
      nag rtsis
      Chinese
      Henning
      Tseng
    "mewa" and "parkha" are 001's words (§5.2); "sme ba" and "spar kha" are their Tibetan forms. That
    mewa and parkha are the 9-cycle and the 8-cycle of Chinese-derived Tibetan astrology is RECALLED /
    UNVERIFIED by the advisor. Neither is a premise (Item 3).
  - The controls, each run on both copies:
      - a term the paper uses on a page you have read, found on that page;
      - a term printed with a diacritic or a ligature that the plain search misses, and the form of
        the search that finds it. Report both results;
      - a word split across a line break, found in the joined copy and missed in the raw one. If the
        paper has none, say so.
    The controls show that the search can find what is there and can miss what is there. They are
    the reason an absence in Item 3 counts.

ITEM 3 — The decisive item: 001 §5.2's table, row by row.
  - For every row of 001 §5.2 (ten rows), state its source status in Janson v1.
  - The three TO-VERIFY rows get one line per unit that 001 names: day element / animal: the day;
    mewa: the year, the month, the day; parkha: the year, the month, the day. Where Janson defines a
    quantity for more than one kind of day (the calendar day, the lunar day), give one line per kind.
    Each line is one of:
      - FOUND: Janson's own term; the section, equation and page; a quotation of two sentences at
        most, read on the rendered page (the Read tool) and matched in the pdftotext text; the version
        it is stated for (Phugpa, or another version only); the label (FETCHED). If Janson gives the
        quantity in more than one numbering, or says its own numbering is not the traditional one, say
        so, with the page;
      - NOT IN JANSON V1: the search that found nothing: the terms (Item 2), and the sections read in
        full, with their pages. "Not in Janson v1" is the claim, never "undefined" (METHOD rule 46;
        003 §3).
  - The rows not marked TO VERIFY (weekday + planet; tshes number; lunar mansion; yoga; karana; month
    leap flag and length; year element, gender, animal, mewa, parkha): the section, equation and page,
    with a quotation, or NOT IN JANSON V1 with its search. These rows are read and cited only. They get
    no research legs, no fork and no implementation: they are not part of Task 6. If 001 §5.2's
    "Source status" cell is wrong for any row, file one DEBT.md entry for all such cells together
    (Kind: DOCS; TRIGGER: the next revision of 001).
  - The identification. 001 names "mewa (мэнгэ, 9)" and "parkha (парка, 8)". For each, state which
    quantity in Janson v1, if any, it is, by structure (METHOD rule 48): its cycle length, and the
    names, colours or directions Janson prints for it, with pages. State whether any source in the
    library names that quantity "mewa", "sme ba", "parkha" or "spar kha", in any spelling. If none
    does, the identification is INFERRED: it goes to Item 7 as part of a fork, and it is never written
    as settled.
  - The table's shape: one row per line above. Columns: 001 row; unit and kind of day; Janson's term;
    section, equation, page; quotation; version; label; status. Count the lines, and show FOUND plus
    NOT IN JANSON V1 summing to the total, for the TO-VERIFY lines and for the other rows separately.

ITEM 4 — The paper against itself (METHOD rules 31, 35).
  For every formula that Item 3 finds for a TO-VERIFY line, look for what the paper prints that the
  formula must reproduce or satisfy: a table of values, a worked example, a statement that two
  formulas give the same quantity, or a stated relation between two quantities. For each one found:
    - compute it with an `awk` or `python3 -I` command on scratch files, from the paper's formula and
      the paper's printed values, never from the engine. Record the command verbatim, and its output;
    - check every row the paper prints, or, where the statement is general, a full period of the
      formula (state the period);
    - seed one wrong value (a printed value, or one term of the formula, changed in the scratch copy)
      and show that the same command reports it. Record that command and its output too.
  Where the paper prints nothing to check a formula against, say so, with the pages read. A
  disagreement is STOP RULE (3).

ITEM 5 — The research legs (copied from 003 §4's canonical list).
  003 §4: Legs: ① **reference**: datsan publications, and a зурхайч where reachable. ② **field**: other implementations (Lotsawa House, Henning's software, existing Mongolian apps); pin where their conventions differ. ③ **science**: Janson, Henning, Schuh, Reingold–Dershowitz, or an explicit "none relevant". ④ **Does this contradict anything already shipped?** Every leg is reported, including empty ones, with what it found against the approach. A research round that lowers a spec's advertised value has succeeded. Citations resolve to a tree path (`docs/sources.md`, `golden/…`), never to someone's notes.
  The approach the legs test: "implement each TO-VERIFY attribute that Janson v1 defines for the
  Phugpa version, from Janson v1's formula, under System::Phugpa". For each leg, report what it found,
  what it found AGAINST that approach, and, where it found nothing, the search.
    - ②: the library in ARTIFACT (B) is the only implementation in the paper library. Read its
      dist/index.js and README.md (no `node`), and state which 001 §5.2 attributes it computes, with
      line numbers.
    - ③: beyond Janson v1, state whether Janson v1 describes its own coverage of these attributes as
      complete, and which sources it names for them, with pages.
    - ④: the engine's shipped fields (ZDate's year fields, round 005; 006's R2 table) and 001
      §5.1–§5.3. Read the engine's source as you need to.

ITEM 6 — 001 §5.3, for each attribute found.
  For each FOUND line of a TO-VERIFY row, in the shape of spec 006's R2 table: does Janson v1 make the
  quantity a function of quantities that take no place, time zone or clock (the Julian day number,
  the Tibetan date, the year number Y)?
    - VERIFIED: a quotation, and the chain in one line;
    - NOT DECIDED BY THE PAPER: file a DEBT.md entry (Kind: PROVENANCE) with a TRIGGER;
    - REFUTED: it goes to Item 7, because 001 §5.3 says such an attribute "gets an explicit
      parameter".
  Where Janson states a variant of an attribute for another version that does depend on time or
  place, say so with the page, as by-catch, and do not transcribe it.

ITEM 7 — The halt, by design (STOP RULE (2)).
  - One STOP AND ASK per fork, in docs/ROUNDS.md §5's shape, with its headings verbatim, in your
    response and in the spec file. File each as an OPEN row in FORKS.md, in the same commit, numbered
    from the next free F-number (F15 at 45efbbc, RELAYED: verify it). One kind per question: a fork
    that spans two kinds of decision is two forks.
  - At least these questions are put. Each becomes a fork, or, where a rule, a spec or a ruling
    already decides it, a forced answer with that rule named (CLAUDE.md hard rule 4):
      a. for each TO-VERIFY line FOUND: implement it in the resume from Janson v1; defer it; or first
         obtain a further source (named, with its label);
      b. for each TO-VERIFY line NOT IN JANSON V1: defer it (001 §10: "resolved or explicitly
         deferred"), or first obtain a source;
      c. what 001 §5.2's words mean where Janson defines more than one quantity that a row could
         name: the kind of day, or one of several quantities for the same unit;
      d. the identification of "mewa" and "parkha" (Item 3), where it is INFERRED;
      e. the numbering or naming the engine would expose, where Janson says its own numbering is not
         the traditional one, or names a source that numbers differently.
    Questions a and b may be put as one fork per 001 row, with one line per unit. The options are a
    starting set, never a menu, and each fork's options include "defer".
  - THE LINK TABLE (METHOD rule 50, step 4): one batched table, at the end of the halt, of the
    sources the legs named that are not in the library. Per source: the citation as the work that
    names it gives it, with its date; what it would decide (which fork, which line); its label
    (PAYWALLED, UNREACHABLE, or RECALLED / UNVERIFIED for whether it is available now).
  - IF THE ANSWER IS DELAYED: nothing. The round has ended by design.
  - The gate does not run at the halt, because this half changes no tracked source (docs/ROUNDS.md
    §3). The head block's `gate:` line says "not run: halt by design, no tracked source changed", and
    the report names it as skipped, with that reason. The resume runs it.
  - Before the halt commit: grep 001 and 002 for forks with no ruling, and compare them with
    FORKS.md (METHOD rule 59); name the grep. The open-forks arithmetic: expected OPEN = the count
    re-measured in Item 0 + the forks raised (METHOD rule 60); then the grep.

RESEARCH LEGS: applicable (Item 5). 003 §4 sets their precondition as Q1, every L2 rule category and
any new attribute, and the TO-VERIFY rows are new attributes.

PREDICTIONS — written by the advisor on 2026-11-11, before this round measures anything (METHOD rule
3). Their basis: the advisor listed Janson v1's section headings from a pdftotext rendering, searched
it for a few terms, and read the paragraph headings of one appendix. It did not read the pages
through. These are RELAYED, and none is a premise. Score each HELD or FAILED, with the measured
value. A prediction scored HELD cites the round's own quotation and page, never this prompt.
  1. Both ARTIFACT checks pass.
  2. Every Item 2 control behaves as stated.
  3. Of the seven TO-VERIFY unit lines (day element / animal: the day; mewa: the year, the month, the
     day; parkha: the year, the month, the day), four are FOUND for the Phugpa version and three are
     NOT IN JANSON V1 for it, counting a unit as FOUND if any kind of day has it.
  4. Every FOUND TO-VERIFY line is in one appendix of the paper; none is in its numbered sections.
  5. No page of Janson v1 uses the words "mewa", "sme ba", "parkha" or "spar kha", in any spelling,
     so both identifications are INFERRED.
  6. For at least one TO-VERIFY row, Janson defines the quantity for both the calendar day and the
     lunar day.
  7. At least two Item 4 checks exist, and the paper agrees with itself in every one.
  8. The library in ARTIFACT (B) computes none of the three TO-VERIFY rows.
  9. Leg ① is empty, and leg ③ finds no source beyond Janson v1 in the paper library.
  10. Every FOUND TO-VERIFY line for the Phugpa version is VERIFIED under 001 §5.3.
  11. Item 7 raises between five and nine forks.

THE REPORT — per CLAUDE.md's report contract and docs/ROUNDS.md §4, in
docs/specs/007-to-verify-attributes.md. It holds:
  - the head block, without a `golden:` line, and with the `gate:` line Item 7 gives;
  - a report-shaped spec (docs/specs/TEMPLATE.md's header lines: Status: draft — halted by design at
    Item 7; Milestone M1; Governs: 001 §5.2's TO-VERIFY rows, a report in this half, nothing built),
    then the end marker;
  - then the report.
  Lead with the decisive item: Item 3's table, with its counts summing. Then:
    - Appendix A Task 6, part by part: DONE / DEFERRED / REFUSED. The implementation is DEFERRED to
      the resume by design;
    - Item 2's term list, counts and controls; Item 4's checks, with their commands, outputs and
      seeded controls; the four legs; Item 6's table;
    - the predictions, scored;
    - R1's DEBT.md entry and R2's record;
    - the open-forks audit, with its arithmetic at every commit;
    - the marker greps, at every commit;
    - every other section of the contract, "What the prompt got wrong" included;
    - the diff scope, with its totals summing against `git diff --stat main`, stating the commit it
      was measured at;
    - every STOP AND ASK, and the link table.
  The squash commit message is written by the resume, not at the halt.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 007 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════

═══════════════════════════════════════════════════════════════════════════
▼▼▼ ADDENDUM 1 — SPEC 007 — 2026-11-11 — BEGINS HERE. PART OF THE PROMPT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

Appended after the halt by design at cfb7e0e (Item 7: STOP AND ASKs spec 007 Forks 1–7, FORKS.md
F15–F21). Where it differs from the text above, it governs. This is a resume: a fresh session on
spec/007-to-verify-attributes. The header is unchanged, except MODEL's reason, ARTIFACT, GRANTS and
STOP RULE, which gain the lines below. The round implements nothing: the human deferred all three
rows. This session writes records, re-measures two readings, runs the gate, and closes the round.

MODEL — unchanged: Sonnet 5.5, effort High. Reason: 003 §7, first branch. The rulings below decide
every question the halt put. What is left is discoverable: records written from the rulings, the
paper library and the paper, and two re-measurements. Its wrong answers would be silent, and each
has an instrument that makes it loud:
  - a source row that names the wrong file, URL or publisher: ARTIFACT (C)'s hashes, the URLs given
    below, and every attribution checked on a library file or labelled RELAYED (Item 9);
  - a re-measurement that cannot fail: one seeded wrong value per check (Item 10).
Nothing is built, so no design choice falls to this model. No change from the halt's session.

THE HUMAN'S RULINGS (2026-11-11). At the halt the human answered the advisor's recommendations with
"Agree with all recommendations". Two earlier recommendations, made between rounds 006 and 007,
were answered "Ok agree".
  R3. Forks 1–7 (F15–F21): defer. Nothing in rows 6, 7 and 8 of 001 §5.2 is implemented.
        F15 (the kind of day): (d) defer.
        F16 (which quantity): (c) defer.
        F17 (the identification of "mewa" and "parkha"): (d) defer.
        F18 (the numbering): (d) defer.
        F19 (row 6, day element / animal): (c) defer, T1 and T2.
        F20 (row 7, mewa): (c) defer all four lines, T3–T6.
        F21 (row 8, parkha): (c) defer all four lines, T7–T10.
      One DEBT.md entry, D-18 (Kind COVERAGE), records the deferral and its TRIGGER (Item 11).
      F15–F18 are deferred with the rows: the round that reopens rows 6–8 puts their questions
      again.
  R4. F9 (the golden `skipped` column): (b), "Ok agree". The column holds the tshes number omitted
      immediately before the day, mirroring `ZDate.skipped_before`. Where a source prints no skip
      marker, the value is derived from the neighbouring days and the meta says "derived". A day
      whose predecessor is not in the source leaves the column empty, with a note.
  R5. A new fork, filed already RULED as F22: may the lunar calendar of the datsan «Ринпоче Багша»
      (Ulan-Ude) supply Buryat golden rows? The options put were:
        (a) admit it now as T2, the datsan's own website;
        (b) a candidate only: its docs/sources.md row and the library, and no golden rows until the
            M0 round's leg ① checks Sangha membership (001 §1), a statement that it is a zurkhai /
            Tibetan calendar (001 §6.3), and the publisher's terms (001 §8);
        (c) do not use it.
      Ruled: (b), "Ok agree".
  R6. docs/sources.md gets one library row for each of six holdings now in the paper library: the
      datsan «Ринпоче Багша» (under R5) and five more. No row gives a tier or an admission; those
      stay with the M0 round (Item 9).
  R7. Session 1's "Latitude taken" items 1–3 are accepted as reported. No grant is made after the
      fact, and this session uses only what GRANTS below allows.
  Also agreed: prediction 4 is re-scored HELD, with a dated correction (Item 12).

ARTIFACT — ADDITION:
  (C) Six holdings in /home/kitaeza/zurkhai-papers/, placed there by the human on 2026-11-11. For
      each, the file count and a manifest hash, by these exact commands with <dir> replaced:
        find /home/kitaeza/zurkhai-papers/<dir> -type f | wc -l
        find /home/kitaeza/zurkhai-papers/<dir> -type f | LC_ALL=C sort | xargs -d '\n' sha256sum | sha256sum
      Expected (the advisor's measurement, 2026-11-11):
        dir: datsan-rinpoche-bagsha
        files: 103
        manifest: 7d59a423eba9e184b7c727d7541c6b609d09b8e61f1a27696d7676a29596bead

        dir: ivolginsky-datsan
        files: 21
        manifest: b1c2402462ca3c674c9ccfdc88fe3291f759b23b4387cd0027145282046a91e2

        dir: aginsky-datsan
        files: 10
        manifest: 5beb61e8809841a3fe013103f645e9514ab68404582562fb3784fedc502733ca

        dir: gunzechoinei-datsan
        files: 18
        manifest: 4a0b7fa9a7422da03f8c064d7b8380dbdd2dce47f0a325bc4461589be013e024

        dir: khoymorsky-datsan
        files: 8
        manifest: 4ec3e9b81520db62a06447ec89d937431a14885f6759987ec12188e562ad0af5

        dir: kalmyk-central-khurul
        files: 31
        manifest: 9b9a88e82c175399b4b4b5b7a562b392c3fa2c453f6aef74e577f66a81f648e5
      And these single files, by `sha256sum`:
        file: ivolginsky-datsan/lunnii_kalendar-2021.pdf
        sha256: 2e140c65b7cac71cbb6c98b837e1f7488a91d47ccd7ea299f3b062d7e320fe25

        file: aginsky-datsan/aga/kalendar2025.pdf
        sha256: 12137318075f3a7ae946f946b5bbf1b4f628e4186adbdad033b510f2868bd7bb

        file: aginsky-datsan/aga2026/kalendar-a2-format.pdf
        sha256: 13e7d720b3ba9dd8d47abe090584970afe91f43d6892e8f5cc811351832aa156

        file: gunzechoinei-datsan/dazanspb/kalendar_plakat.jpg
        sha256: f96f0a14a4901c7bc44e8ee23947b79cd0e4064fceb7b9034c617bb8a0a0ce61

        file: khoymorsky-datsan/cal2025.jpg
        sha256: c39eb89e89854ebed00fcb2d5b2c8cb4532b9892af77bfa2f02d16eff21f0a7a

        file: kalmyk-central-khurul/2023_07_c1d72caa-bd5a-4489-a0ed-eaea35a682fe.webp
        sha256: 049afcd1a50ff9d4c115bd79b900bd4a50346b55686ddfe96907b847206231e6

        file: kalmyk-central-khurul/2023_12_b9f80f39-9ecd-4f96-8c80-73111d872d6b.webp
        sha256: 681a91ccf81bc3faf5fcd581ad5d40eba7e4a59390ca7269d9fd1952b2358b5e

        file: kalmyk-central-khurul/2025_01_6b760479-1ffc-403b-8d7c-67a3bceb1155.jpg
        sha256: e0156e8294b2c6faa1a192353007d35d5287a90fa98f4d1f1c98db54e39459ca

        file: kalmyk-central-khurul/2026_01_9128c8d7-b39e-4fa4-aa6c-8af4f7cde532.jpg
        sha256: b720824e80d6fabc8b9e78c26e2a1ee0c057c5e935ec5d8bdf9dedb4e9bd066e
      If any value differs: STOP and report, with zero diff. Each holding's `readings/` directory
      holds transcriptions made by models in the advisor's sessions. They are not sources and not
      golden data, and this round does not use them.

GRANTS — ADDITION (METHOD rule 67). Network: still none. Nothing is written into the library.
  - `find`, `sort`, `xargs`, `sha256sum`, `wc`, `ls` and `grep`, read-only, on
    /home/kitaeza/zurkhai-papers/ (ARTIFACT (C), Item 9);
  - `pdfinfo` on the library's PDFs (Item 9);
  - the Read tool on the library's image, HTML and text files (Item 9);
  - `pdftotext -layout` on the two Aginsky PDFs of ARTIFACT (C), writing only into a directory made
    by `mktemp -d` (Items 9 and 10b);
  - `cp`, `sed`, `diff` and `cat` on files in that directory, to make and inspect seeded copies, and
    `python3 -I` running this round's own scratch scripts on files there (Item 10). No script
    enters the repository; each command is recorded verbatim in the report, with its output;
  - /home/kitaeza/github/zurkhai/tools/gate.sh, run once with no arguments (Item 13), and the cargo
    commands it runs.

STOP RULE — ADDITION:
  (7) Item 8's status or HEAD check fails, or an ARTIFACT check fails: STOP and report, with zero
      diff.
  (8) A re-measurement in Item 10 disagrees with the advisor's reading: not a STOP. The records
      state the measured value only, and the report names the disagreement under "What the prompt
      got wrong". If Item 10b does not show the months labelled by Janson's Tsurphu rules, D-18
      leaves that reason out.
  (9) Any gate leg fails (Item 13): STOP AND ASK. No tracked source has changed since 45efbbc, so a
      failure is pre-existing or environmental, and which one it is is not this round's to decide.
  (10) A fact a row needs is in no library file and is not given here: the cell says "not
      recorded", and the report lists it. Never fill it from memory or from the web.

ITEM 8 — RESUME.
  - Read this prompt through ADDENDUM 1, then `git log`, then the report so far. Quote
    `git status --porcelain` and `git rev-parse HEAD`. The status is empty and HEAD is
    cfb7e0e30eda908afac621e9addb1fc19e56df52. If either differs: STOP and report, with zero diff.
  - Re-verify ARTIFACT (A) and (B) as the header states them, and verify (C) (CLAUDE.md hard rule
    10).
  - Run `date` once, at the top of this session, and use that value for every date this session
    writes. Dates the halt's session wrote stay as written. The rulings' date is the human's,
    2026-11-11, as given here. The report states both sessions' dates and which records each covers.
  - Choose this session's marker (METHOD rule 58). The halt's marker is not spelled anywhere, so this
    is a new one, and the report says so. The marker greps are reported per commit, for both
    sessions.
  - Re-measure the OPEN count with the grep FORKS.md defines. The halt measured 14 (RELAYED:
    re-measure it).
  - Commit at every item boundary: Items 9, 10, 11 and 12 one commit each; Item 13 ends with the
    final commit.

ITEM 9 — THE LIBRARY ROWS (R5, R6). One commit: docs/sources.md and the report's account.
  - One row per holding in docs/sources.md's "The library" table, in its column shape (File; Source;
    Obtained; sha256; Licence; Trusted for), from the lines below. A line marked RELAYED is the
    advisor's, and its cell says RELAYED. A line marked "verify" is checked on the library file it
    names (a `grep`, or the Read tool on an image): the cell says FETCHED if it holds; if it does
    not, the cell says what the file says, and the report names the difference.
  - The sha256 cell gives ARTIFACT (C)'s file count and manifest hash, the command that made them,
    and the single-file hashes ARTIFACT (C) lists for that holding.
  - The Bibliography's last row ("Buryat golden sources …") has "—" under "In the library". It now
    names the five Buryat holdings, as candidates not admitted (CLAUDE.md hard rule 19: "—" would
    contradict the new rows). The Kalmyk holding is not Buryat and is not named there.
  - "Reference facts" stays "None yet": these rows admit nothing.
  - The report's Item 1 table stays as session 1 measured it. Item 9's account says the library
    now holds eight entries, and when the six arrived: `ls -la --time-style=full-iso` on the
    library shows their modification times.

  Holding 1
    file: datsan-rinpoche-bagsha/ (a directory)
    contents: datsan-rb/ (the 2026 edition: m01.png … m14.png, legend.png, SHA256SUMS);
      datsan-rb-archive/2022, 2023, 2024, 2025 (14 month images each); wayback/ (11 Wayback
      snapshots, raw and decoded; plan.txt, which lists the image ids; sitemap.xml); datsan-about/
      (the About page); readings/ (not a source). Verify with `ls`.
    source: the "Лунный календарь" page of the datsan «Ринпоче Багша», Ulan-Ude. RELAYED
    publisher: Местная религиозная буддийская организация «Буддийский центр «Ринпоче Багша»».
      Verify on datsan-about/o-datsane.txt
    url: https://datsan-rinpoche-bagsha.ru/lunar-calendar (the 2026 edition, January 2026 to
      7 February 2027). RELAYED
    url: https://datsan-rinpoche-bagsha.ru/o-datsane (the About page). Verify: the canonical link in
      datsan-about/o-datsane.html
    earlier editions: 2022, 2023, 2024 and 2025, each January Y to February Y+1, from web.archive.org
      captures of the lunar-calendar URL. RELAYED
    capture timestamps: 20220913115415, 20230327001710, 20230605101555, 20231002171124,
      20240229010809, 20240616205853, 20240813205502, 20241204235258, 20250206214602,
      20250621133205, 20251018130332. Verify: the wayback/ file names
    images: served by static.tildacdn.com under the ids in wayback/plan.txt. RELAYED
    obtained: 2026-11-11, by the advisor with curl (the live pages, the images, and the Wayback
      snapshots, from a capture list the human supplied); copied into the library by the human,
      2026-11-11. RELAYED
    licence: the site's footer allows use or copying only with the rights holder's written
      permission and a link to the source. Verify, and quote at most two sentences, with
      `grep -o '.\{0,120\}письменного разрешения.\{0,80\}'` on datsan-about/o-datsane.txt.
      001 §8 is not yet decided for it
    trusted for: nothing yet. A candidate only (F22, RULED (b)): no golden rows until the M0
      round's leg ① checks Sangha membership (001 §1), a zurkhai / Tibetan-calendar statement
      (001 §6.3) and these terms (001 §8)

  Holding 2
    file: ivolginsky-datsan/
    contents: lunnii_kalendar-2021.pdf (one A2 page); text.txt (its text layer); overview-1.png and
      crops/ (renderings made by the advisor); readings/ (not a source). Verify with `ls`.
    source: "Лунный календарь 2021", the year named "Сагаагшин түмэр үхэр жэл"; the footer names
      the Buddhist Traditional Sangha of Russia and the local religious organisation of Buddhists
      of the Ivolginsky datsan. Verify with the Read tool on crops/footer.png and overview-1.png;
      if illegible, RELAYED
    compiler: "Ширээтэ лама Ацайского дацана Содбо Айсуев". Verify with the Read tool on
      crops/compiler.png; if illegible, RELAYED
    pdf: one page; producer CorelDRAW; created 2021-01-15. RELAYED; `pdfinfo` may verify it
    url: https://ivolgdatsan.ru/upload/file/lunnii_kalendar-2021.pdf. RELAYED
    obtained: 2026-11-11, by the advisor with curl; copied into the library by the human. RELAYED
    licence: not checked (001 §8; the M0 round)
    trusted for: nothing yet. A candidate; its admission and tier are not ruled (the M0 round)

  Holding 3
    file: aginsky-datsan/
    contents: aga/kalendar2025.pdf and aga2026/kalendar-a2-format.pdf (two A2 calendars with a text
      layer); text.txt, aga-2025-textlayer.txt, aga-2026-textlayer.txt, bbox.html and the PNGs
      (derivatives made by the advisor). Verify with `ls`.
    source: two calendars headed "Буддийская Традиционная Сангха России", "Агинский дацан",
      "«Дэчен Лхундублинг»"; compilers "Жапов А. Т., Дондоков Б. Д.". Verify on `pdftotext -layout` copies of
      the two PDFs, made into the mktemp directory here and reused by Item 10b
    coverage: the 2025 edition, January 2025 to 17 February 2026; the 2026 edition, January 2026 to
      6 February 2027. Verify on the same copies
    url: https://aginskydatsan.ru/wp-content/uploads/2025/05/kalendar2025.pdf. RELAYED
    url: https://aginskydatsan.ru/wp-content/uploads/2026/03/kalendar-a2-format.pdf. RELAYED
    obtained: 2026-11-11, by the advisor with curl; copied into the library by the human. RELAYED
    licence: not checked (001 §8; the M0 round)
    trusted for: nothing yet. A candidate; its admission and tier are not ruled (the M0 round)

  Holding 4
    file: gunzechoinei-datsan/
    contents: three news pages of the datsan Гунзэчойнэй (St Petersburg), each with page.html,
      page.txt and the calendar images the page carries: dazanspb2024/, dazanspb/ (the 2025
      calendars), dazanspb2026/; readings/ (not a source). Only a few calendar pages are on the
      web; the printed calendars are not in the library. Verify with `ls`.
    url: https://dazanspb.ru/news/buddijskie-kalendari-na-2024-god-ot-dacana/ (dated 12 December
      2023). RELAYED; verify the date on dazanspb2024/page.txt
    url: https://dazanspb.ru/news/vypuweny-tradicionnye-buddijskie-kalendari-na-buduwij-god/ (dated
      6 December 2024). RELAYED; verify the date on dazanspb/page.txt
    url: https://dazanspb.ru/news/buddijskie-kalendari-dacana-na-2026-god-yarkij-podarok-so-smyslom/
      (dated 16 December 2025). RELAYED; verify the date on dazanspb2026/page.txt
    source: the 2025 page says the lunar data follow the Buryat-Mongolian astrological tradition
      used by the Buddhist Sangha of Russia. Verify on dazanspb/page.txt, and quote at most two
      sentences
    obtained: 2026-11-11, by the advisor with curl; copied into the library by the human. RELAYED
    licence: not checked (001 §8; the M0 round)
    trusted for: nothing yet. A candidate (001 §9 names this datsan's annual calendar); its
      admission and tier are not ruled (the M0 round)

  Holding 5
    file: khoymorsky-datsan/
    contents: cal2025.jpg (the poster image the shop page serves); page.html and page.txt (the shop
      page); jan.png, feb.png, dec.png and overview.png (crops made by the advisor); readings/ (not
      a source). Verify with `ls`.
    source: "Буддийский лунный календарь на 2025 год", a 42 × 59 cm poster of the Хойморский datsan
      «Дэчен Равжалин» (Republic of Buryatia); "лама астролог Лобсан Чойнзин (Сотников И.В.)".
      Verify on page.txt or with the Read tool on cal2025.jpg; else RELAYED
    url: https://dharma.ru/product/30910-buddiyskiy-lunnyy-kalendar-na-2025-god-42-h-59-sm (a shop's
      page, not the datsan's). RELAYED
    obtained: 2026-11-11, by the advisor with curl; copied into the library by the human. RELAYED
    licence: not checked (001 §8; the M0 round)
    trusted for: nothing yet. A candidate; its admission and tier are not ruled (the M0 round). The
      image is a seller's copy, not the datsan's own publication

  Holding 6
    file: kalmyk-central-khurul/
    contents: four calendar images, one each for 2023, 2024, 2025 and 2026 (RELAYED), the files
      named 2023_07_…, 2023_12_…, 2025_01_… and 2026_01_… (WebP even where named .jpg: verify with
      the Read tool or `grep -c WEBP`); m2023.html … m2026.html, page.html, page.txt; png/,
      crops2025/ and crops2026/ (renderings made by the advisor); readings/ (not a source). Verify
      with `ls`.
    source: "Буддийский календарь" of the Central Khurul «Золотая обитель Будды Шакьямуни», Elista.
      Verify on page.txt
    compiler: "Анджа-гэлюнг". RELAYED
    url: https://khurul.ru/materials/buddijskij-kalendar. Verify: the canonical link in page.html
    obtained: 2026-11-11, by the advisor with curl; copied into the library by the human. RELAYED
    licence: not checked (001 §8)
    trusted for: nothing. Not Buryat (001 §1), so not Buryat evidence. Its role (for instance a
      Phugpa control) is undecided and has not been put to the human

ITEM 10 — TWO RE-MEASUREMENTS (CLAUDE.md hard rules 10 and 11). One commit: the report's account.
No other file changes in it. Each is the advisor's reading, given as an instruction to check, never
as a premise. Record each command verbatim with its output, and each seeded control's command and
output (METHOD rule 31).
  10a. The version of lines T6 and T10. The advisor's reading: (E.10) and (E.11), p. 81, are stated
       for "a month with animal number A (in Table 3)". The month's animal comes from Table 24,
       p. 80, which has a Phugpa column and a Tsurphu column. So T6 and T10 depend on the version
       through A, as T2 does through (E.6) and (E.7). Item 4's C4c and C5c ran the Phugpa column
       only. Read pp. 80–81 on the rendered pages, and confirm or correct this reading. Then
       compute, for M = 1 … 12 and D = 1 … 30, with A taken from each column of Table 24 (numbered
       by Table 3), on how many of the 360 pairs (M, D) the two columns give a different trigram
       (E.10), a different number (E.11) and a different animal (E.9). Seed one wrong Table 24
       entry in a scratch copy, and show that the counts move.
  10b. The month labels of a candidate Buryat source (Holding 3). The Aginsky calendars head each
       lunar month with its ordinal and season, then a colour and an animal (for instance "Первый
       весенний месяц – желтый тигр").
         - Run `pdftotext -layout` on both PDFs into the mktemp directory. Extract every lunar-month
           label: its edition, its line, its Tibetan year Y and month M, its colour, its animal.
           In the text layer a heading can end in "месяц –" with its label on a later line: such a
           heading takes the first colour-and-animal pair that follows it.
         - The month number: "Первый весенний" is M = 1, and so on to "Третий зимний", M = 12. This
           mapping is INFERRED. Check it against the edition's own hural list: the 2025 edition
           dates "Сагаалган" to 1 March, which must fall in the month it heads "Первый весенний".
         - The element of a colour, by Table 14 (p. 74): white is iron, yellow earth, red fire.
           Table 14 gives wood "green (blue)" and water "dark blue (black)". Fix "синий" and
           "черный" from the editions' own year names and eq. (4.4), p. 9: the 2025 edition's year
           heading names a "синеватая" snake, and the 2026 edition's a "красная" horse. A
           "-оватый" or "-еватый" form has the same element as its base colour.
         - Compare each label with Table 24's two columns and with (E.6) (Phugpa) and (E.7)
           (Tsurphu), p. 80. Report the counts, and where the two editions overlap, whether they
           agree.
         - Seed one wrong label in a scratch copy, and show that the counts move.
       This source is a candidate, not admitted. The result is a reason recorded with a ruling: not
       golden data and not a Q1 finding. The report says so.

ITEM 11 — THE RULINGS ONTO THE REGISTERS (R3, R4, R5). One commit: FORKS.md, DEBT.md and the
report's account.
  - F15 … F21. Each row's `Status:` becomes RULED, followed by its `Ruled:` line:
      F15: `Ruled: 2026-11-11, (d) defer — deferred with rows 6–8 (D-18); the round that reopens them puts this question again; the human: "Agree with all recommendations" (spec 007 ADDENDUM 1, R3).`
      F16: the same line, with (c).
      F17: the same line, with (d).
      F18: the same line, with (d).
      F19: `Ruled: 2026-11-11, (c) defer — T1 and T2 are not implemented (D-18); the human: "Agree with all recommendations" (spec 007 ADDENDUM 1, R3).`
      F20: `Ruled: 2026-11-11, (c) defer all four lines, T3–T6 (D-18); the human: "Agree with all recommendations" (spec 007 ADDENDUM 1, R3).`
      F21: `Ruled: 2026-11-11, (c) defer all four lines, T7–T10 (D-18); the human: "Agree with all recommendations" (spec 007 ADDENDUM 1, R3).`
    No `Built in:` line: nothing is built.
  - F9. `Status:` becomes RULED, followed by:
      `Ruled: 2026-11-11, (b) — the tshes number omitted immediately before the day, mirroring ZDate.skipped_before; where a source prints no skip marker, derived from the neighbouring days, and the meta says "derived"; empty, with a note, where the day's predecessor is not in the source; the human: "Ok agree" (spec 007 ADDENDUM 1, R4).`
    The row's last line ("Decided before the first golden row is transcribed (M0).") stays.
  - F22, a new row in the register's shape. Subject: the admission of the datsan «Ринпоче Багша»'s
    lunar calendar as a Buryat source. Raised: 2026-11-11, by the advisor between rounds 006 and
    007; filed by spec 007 ADDENDUM 1. Question and options: as R5 gives them. Status: RULED.
    Ruled: `2026-11-11, (b) — a candidate only; the human: "Ok agree" (spec 007 ADDENDUM 1, R5).`
  - D-18, in DEBT.md's entry shape, short (DEBT.md's "Length"):
      Kind: COVERAGE
      Statement: rows 6, 7 and 8 of 001 §5.2 are explicitly deferred (001 §10), by the human's
        ruling on F15–F21. Spec 007's Item 3 table holds the ten lines: FOUND in Janson v1 T1, T2,
        T3, T5, T6, T9, T10 (Appendix E); NOT IN JANSON V1 T4, T7, T8. The reasons, each with the
        value this round measured (STOP RULE (8)): no check of any FOUND line exists beyond the
        paper itself (the T4 library computes none; Item 4 is the paper against itself, METHOD
        rule 35); the identification of "mewa" and "parkha" is INFERRED; Janson v1 calls its trigram
        numbering "perhaps not traditional" (p. 74); the lunar-day lines depend on the version
        through the month's animal and element (Item 10a); a candidate Buryat source labels its
        months by Janson's Tsurphu rules (Item 10b; not admitted). The round that reopens the rows
        starts from that table and puts F15–F18's questions again. At 001's next revision, rows
        6–8 say "deferred" (with D-16).
      Pinned by: not pinned
      TRIGGER: the first of: (i) an admitted T1–T3 source prints one of these attributes, or names
        мэнгэ or парка; (ii) an L2 rule (M4 or later) needs one in its condition; (iii) the Q1
        verdict spec is accepted (M3).
  - The OPEN count: expected 14 − 7 (F15–F21) − 1 (F9) + 0 (F22 is filed RULED) = 6. Then the grep.

ITEM 12 — THE REPORT: FIXES TO THE HALT'S DRAFT, in the same file. One commit.
  1. The head block: `model:` gives one entry per session; `gate:` waits for Item 13. The header
     lines: `Status: draft`; `Resolves:` names F9 and F15–F21 as RULED, and F22 as filed RULED.
  2. The STOP AND ASK blocks stay as written, as the round's history. A "Session 2" section records
     R3–R7, the point where one session handed over to the next, and Items 8–13.
  3. Appendix A Task 6, the part "Implement only those found": DEFERRED by the human's ruling (R3,
     D-18), 001 §10's "explicitly deferred".
  4. Prediction 4: re-scored HELD, by a `[CORRECTED yyyy-mm-dd: …]` record that keeps the original
     text. As worded it held: all seven FOUND lines are in Appendix E. Table 5's weekday element
     (§9, p. 28) is not one of them; that finding stays, under Fork 2. The predictions' sum
     follows.
  5. Item 10a's result. If it confirms the advisor's reading, the "Version" cells of T6 and T10,
     leg ③'s point (i), and Item 4's description of C4c and C5c each get a `[CORRECTED …]` record
     that keeps the original text. If it does not, nothing in them changes, and the report says so.
  6. "Latitude taken": session 1 committed Items 1–2, 3–4 and 5–6 in one commit each, where the
     FALLBACK asks for a commit at every item boundary. Confirm it from `git log`, and record it
     with R7.
  7. "What the prompt got wrong": anything this addendum gets wrong, Item 10's disagreements
     included, and anything this session finds.
  8. The predictions below, scored.
  9. The open-forks arithmetic at every commit of both sessions (METHOD rule 60): 7 at the prompt
     commit … 14 at the halt, then each of this session's commits, ending at 6.
  10. The marker greps at every commit of both sessions. Where a grep was not run before a commit,
      say so; do not reconstruct it.
  11. The diff scope at the final commit (`git diff --stat main`), its totals summing, the commit it
      was measured at named.
  12. A section headed `## The squash commit message`, holding one fenced block (docs/ROUNDS.md §9
      extracts it). The message names: F15–F21, raised and RULED (defer); F9 RULED; F22 filed
      RULED; D-15, D-16, D-17 and D-18 filed; docs/sources.md's six library rows; rows 6–8 of
      001 §5.2 explicitly deferred. It ends with the Co-Authored-By trailers of both sessions.

ITEM 13 — THE GATE AND THE CLOSE.
  - After Item 12's commit, with nothing in flight, run /home/kitaeza/github/zurkhai/tools/gate.sh
    once, with no arguments (docs/ROUNDS.md §3; METHOD rule 65). No tracked source has changed since
    45efbbc; the run shows that the final tree is green in both profiles (CLAUDE.md hard rule 12).
    Record the commit it ran on, each leg's result, and the counts. A failing leg is STOP RULE (9).
  - The final commit holds the head block's `gate:` line, the docs/specs/INDEX.md row, the diff
    scope, the audits and the squash message. The INDEX.md row, as one line:
      row: | 007 | `007-to-verify-attributes.md` | The TO-VERIFY attributes of 001 §5.2: what Janson v1 defines | draft |
  - Before the final commit, show that it changes nothing the gate reads. Both commands print
    nothing:
      git -C /home/kitaeza/github/zurkhai diff --stat <the gated commit> HEAD -- crates golden tools Cargo.toml Cargo.lock rust-toolchain.toml deny.toml
      git -C /home/kitaeza/github/zurkhai diff --cached --stat -- crates golden tools Cargo.toml Cargo.lock rust-toolchain.toml deny.toml
    The report records both commands and their empty output, so the gate's result holds for the
    tip.

PREDICTIONS — ADDITIONS, written by the advisor on 2026-11-11, before this session measures anything
(METHOD rule 3). Their basis is the advisor's own measurements, made in its sessions; they are
RELAYED, and none is a premise. Score each HELD or FAILED, with the measured value.
  12. ARTIFACT (A), (B) and (C) all pass.
  13. After Item 11, FORKS.md has 6 OPEN rows.
  14. Item 10a: the two columns of Table 24 give a different trigram on 360 of 360 pairs (M, D), a
      different number on 360 of 360, and the same animal on 360 of 360.
  15. Item 10b: the two editions carry 27 labels for 25 distinct months (Y, M), from month 12 of 2024
      to month 12 of 2026. The editions agree on the two months both carry (months 11 and 12 of
      2025). The animal matches Table 24's Tsurphu column on 27 of 27 labels and its Phugpa column
      on 0 of 27. The element matches (E.7) on 27 of 27 and (E.6) on 25 of 27; the two differences
      are months 11 and 12 of 2026. The 2025 edition's leap month ("Дополнительный зимний месяц")
      carries no label.
  16. Every gate leg that runs passes, with round 006's counts: 84 passed, 0 failed (debug) and
      84 passed, 0 failed (release), and wasm-pack skipped (RELAYED from 006's head block:
      re-measure it).
═══════════════════════════════════════════════════════════════════════════
▲▲▲ ADDENDUM 1 — SPEC 007 — ENDS HERE. NOTHING BELOW IS PART OF IT.      ▲▲▲
═══════════════════════════════════════════════════════════════════════════
