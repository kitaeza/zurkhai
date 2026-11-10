═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 005 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  005 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
BRANCH:       spec/005-phugpa-core, cut from main at 59bad84 before any edit; commit to it, the halt
              included. No merge. No push.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7, first branch. The answer is discoverable:
              every formula and constant comes from Janson v1, and Item 4's exhaustive runs make
              arithmetic slips loud. Some of the round's errors would be silent: the citations, the
              transcription of worked examples, and the §5.3 reading. These run on the smaller model
              only with instruments that make them loud (003 §7; docs/ROUNDS.md §8):
                - the citation table, with a second pass that re-reads each page (Item 3), which the
                  advisor re-checks against the library copy at landing;
                - Janson's worked examples, made into tests (Item 5);
                - the §5.3 verdict, quoted with its page (Item 6).
              CHANGE from the last round: round 004 ran on DeepSeek V4 Pro, as a one-round trial
              (FORKS.md F12). This round is back on a Claude model, by the human's ruling of
              2026-11-10.
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first. It never commits a previous session's uncommitted work without
              verifying it: it finishes the work and proves it by the item's own check, or redoes
              it. A design choice that would fall to the fallback model (Opus 5.5, effort High) is a
              STOP AND ASK. The report names the switch point.
PRECONDITION: main at 59bad84 (spec 004 merged: the workspace and the gate). Anything else: STOP and
              report, with zero diff. Merging is the human's.
              Read first, in this order: CLAUDE.md; docs/ROUNDS.md; docs/specs/001-init-spec.md
              (§4, §5, §6, §10, §12 and Appendix A); docs/specs/002-rust-code-standard.md;
              docs/specs/003-project-method.md; docs/specs/004-workspace-gate.md (the report: its gate,
              its self-test table and its harness notes); docs/METHOD.md, all of Part 1 (these rules
              bind this round directly: 3, 8, 23, 31, 33, 34, 35, 37, 41, 42, 45, 49, 50, 51, 56,
              58–60, 63–65 and 67); GATED.md; FORKS.md; DEBT.md; docs/sources.md; tools/gate.sh;
              tools/gate-self-test.sh.
ARTIFACT:     /home/kitaeza/zurkhai-papers/janson-1401.6285v1.pdf. This is S. Janson, "Tibetan
              calendar mathematics", arXiv:1401.6285v1 (24 Jan 2014; v1 is the only version arXiv
              lists). The human places it in the paper library before this round starts. If it is
              absent, or if its first page does not carry the identifier arXiv:1401.6285v1: STOP and
              report, with zero diff. Never implement from memory, from secondary pages, or from
              another version.
STOP RULE:    (1) A constant or formula that Phugpa's Gregorian ↔ ZDate mapping needs is not stated in
              Janson v1: STOP at that point; report the section and the gap; substitute nothing
              (001 Appendix A). Item 3 names the one exception, Gregorian ↔ JDN.
              (2) A property (Item 4) fails, and the cause is not a coding slip traceable to a line
              against the paper: STOP AND ASK. Never change a constant to make a property pass
              (CLAUDE.md hard rule 7).
              (3) `cargo deny check` rejects a licence, an advisory or a source in the new
              dev-dependency tree: STOP AND ASK. Do not widen deny.toml.
              (4) Janson v1 says that a quantity this round computes depends on location or time
              zone (Item 6): STOP AND ASK before writing the parameter. 001 §5.3 fixes that it is an
              explicit parameter; the values it takes are not fixed anywhere.
              (5) A network fetch, read or install outside GRANTS is needed, or a permission is
              denied: STOP and report each denial with its reason. Never work around it by another
              tool or route (METHOD rule 67).
              (6) Out of scope; do not write any of it: Genden or Tsurphu arithmetic; any 001 §5.2
              attribute beyond what ZDate itself carries; anything under golden/ (the T5 tables and
              P5/P6 belong to round 006); localised names (F5 is OPEN). If something seems to need
              one of these, report it under "What the prompt got wrong".
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; there is no
              fourth kind. An unobtainable source says PAYWALLED or UNREACHABLE. Read docs/sources.md
              before any fetch. Never curl/wget. (METHOD rule 50.) A quotation from the paper in the
              repository is two sentences at most; formulas, constants and equation numbers are
              facts.
GATED:        yes: crates/tshes/**, under accepted specs 001 (§5.1, §6.4) and 002. Also your own
              docs/specs/005-phugpa-core.md, docs/specs/prompts/005-phugpa-core.md and your row in
              docs/specs/INDEX.md, which are not changes to an accepted spec. No other gated path.
              golden/** is not touched.
GRANTS:       Reads outside WORKDIR, beyond this prompt file: /home/kitaeza/zurkhai-papers/ (the
              human adds it with /add-dir). Read the PDF with the Read tool, and `sha256sum` it.
              Nothing is written there. Network: crates.io for the dev-dependency Item 4 adds
              (proptest, 002 §4.9 and §6, through cargo's own fetch), and cargo-deny's advisory
              database. No installs: the toolchain and cargo-deny are already in place from round 004.
              Scratch: the gate self-test and Item 4's planted controls may create and delete their
              own directories under the system temp directory (mktemp -d) or under target/.
              Nothing else.
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive
              widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 005 — the Phugpa core

Round 005 is M1's second round. It covers 001 Appendix A, Tasks 2, 3 and 7, as the human split them
on 2026-10-02 (Task 1 was round 004; Tasks 4 and 5 are round 006; Task 6 is round 007). It runs now
because round 006's T5/T4 fixtures need an engine to check. It first hardens the gate, as the human
ruled at round 004's landing. It then implements Phugpa Gregorian ↔ ZDate from Janson v1, with
properties P1–P4 and P7 over 1900–2100, and checks 001 §5.3 (location independence) against the
paper. It does NOT transcribe Janson's tables, compare against Lotsawa House, implement Genden or
Tsurphu, or compute any 001 §5.2 attribute.

THE HUMAN'S RULINGS this round carries (2026-11-10):
  R1. This round runs on a Claude model, per 003 §7. The DeepSeek trial (F12) covered round 004 only.
  R2. At round 004's landing the human ruled "go (a)": merge 004 as it stands, and harden the gate in
      this round's first item, before any calendar code lands (Item 1).

ITEM 0 — Branch, prompt, number.
  - Create spec/005-phugpa-core from 59bad84.
  - Commit this file, byte-identical, as docs/specs/prompts/005-phugpa-core.md. That is the first
    commit. `cmp` it against /home/kitaeza/zurkhai-prompts/spec-005-phugpa-core.md before every later
    commit.
  - Verify that 005 is free in the tree.
  - Read the Read-first list.
  - Run `date` once and use that value for every date you write (METHOD rule 51).
  - Choose your round's marker (METHOD rule 58). The report calls it "the round's marker" and never
    spells it: round 004's report spelled its marker, and that broke the rule.
  - Re-measure the OPEN count in FORKS.md with the grep its header defines. The advisor counted 8 at
    59bad84 (RELAYED: re-measure it, do not copy it).

ITEM 1 — The gate, hardened (R2). This item lands, committed, before any calendar code.
  a. The zero-dependencies leg. 002 §4.9: the core has zero dependencies; dev-dependencies are
     allowed. The leg 004 built reads the manifest as text, for the section headers
     `[dependencies]`, `[dev-dependencies]` and `[build-dependencies]`.
     First, run 004's leg as it stands against two planted manifests (prediction 1):
       - a path dependency written as a `[dependencies.<name>]` table;
       - a path dependency under `[target.'cfg(unix)'.dependencies]`.
     Record what it says about each.
     Then rewrite the leg to use Cargo's own reading of the manifest, not the text: no dependency of
     crates/tshes may have the normal or build kind. Dev-dependencies pass, because 002 §4.9 allows
     them; 004's leg also rejected them, which was stricter than 002.
     Add self-test plants, each asserted by the leg's own named message:
       - the two shapes above;
       - a `[build-dependencies]` path dependency;
       - a control: a dev-dependency, which the leg must pass.
     Keep 004's plant d4.
  b. The caller's directory. The legs pass `--manifest-path` but never change directory. The advisor
     RECALLS (UNVERIFIED) that rustup chooses the toolchain from the current directory, not from
     the manifest; this is a claim for you to check, not a premise to build on. Run the gate from a
     directory outside the repository, and record what each cargo leg does and which toolchain it
     prints (prediction 2).
     If any leg runs on anything other than the pin, or fails for that reason: make the gate
     independent of the caller's directory, and add a self-test plant that runs the gate from
     outside the repository and asserts the pinned toolchain line.
     If every leg already runs on the pin, report the evidence and change nothing.
  c. File a DEBT.md entry (Kind: TOOLING). The workflow pins actions/checkout by its major-version
     tag, not by commit SHA. That is an accepted risk which round 004 stated only in the workflow's
     comment (METHOD rule 56). TRIGGER: before the human's first push of main to a public remote,
     which is the hosted workflow's first run.
  Report a table: plant; shape; expected message; observed; verdict. The self-test leg stays green
  with its new plant count.

ITEM 2 — The source.
  - Verify the ARTIFACT: the identifier on page 1, and the page count.
  - Add its row to docs/sources.md's library table in the same commit as the first code that cites
    it. The row holds:
      - the file;
      - the full citation with v1;
      - "Obtained": the file's modification date from `stat`, labelled as that, and "placed in the
        library by the human";
      - the sha256;
      - the licence, as arXiv states it: "arXiv non-exclusive distribution licence 1.0
        (arxiv.org/licenses/nonexclusive-distrib/1.0/)". The PDF stays outside git (CLAUDE.md hard
        rule 17);
      - "Trusted for", copied from the Bibliography row.
  - Set the Bibliography row's "In the library" to yes.

ITEM 3 — Phugpa, Gregorian ↔ ZDate (001 §5.1; 002 §4, §5).
  - The API: `to_zdate(gregorian, system)` and `to_gregorian(zdate) -> Result<_, Error>`, via JDN.
    - `System` is an explicit parameter everywhere, with no default (DEBT.md D-1).
    - A system this round does not implement computes nothing and returns an error. It never falls
      back to Phugpa.
    - How that is spelled is your choice; report it.
  - ZDate carries 001 §5.1's fields:
    - year: rabjung cycle, year in cycle (1..=60), element, gender, animal;
    - month: 1..=12, plus the leap flag;
    - tshes: 1..=30;
    - is_doubled;
    - skipped_before.
    The Mongolian cycle numbering and every rendering of names are out of scope.
  - Code standard, per 002:
    - newtypes with validating constructors and private fields (§4.7);
    - floor division and modulo only through named helpers, with doc tests on negative inputs
      (§4.2);
    - i64, and i128 only where a bound is stated (§4.3);
    - no panics on valid input (§4.4);
    - one `#[non_exhaustive] enum Error` with at least OutOfRange, SkippedDay and InvalidField,
      implementing `core::fmt::Display` and `core::error::Error` (§5);
    - no_std with no alloc, and zero dependencies (F6; §4.9).
  - Every formula and constant carries the comment `// Janson (arXiv:1401.6285v1) §<section>,
    eq. (<n>), p. <page>` (002 §4.8). A derived constant shows its derivation in a doc comment or
    a test.
  - Gregorian ↔ JDN:
    - Cite Janson v1 where it gives the conversion.
    - If it does not, derive the conversion from the Gregorian leap-year rule, with the derivation
      in a doc comment. Anchor it on one JDN value that Janson v1 states, citing where, and test it
      against a day-by-day count over the whole range.
    - If Janson v1 states no JDN at all: STOP RULE (1).
  - THE CITATION TABLE, in the report. One row per formula and per constant. Columns:
    - the name in code;
    - the value or expression as written in code;
    - Janson v1 section, equation number and page;
    - the paper's expression (numbers and symbols only);
    - the second-pass verdict.
    The second pass happens after the code is written: re-open each cited page and compare it
    against the code. Its verdict is MATCH or MISMATCH, and a MISMATCH is fixed and re-checked
    before the round lands. The advisor re-checks this table against the library copy at landing.

ITEM 4 — Properties P1–P4 and P7 (001 §6.4; 002 §6; METHOD rules 35, 41, 42).
  - P1–P3, and P4's month lengths: run EXHAUSTIVELY over every day from 1900-01-01 to 2100-12-31,
    in both profiles. The range is small enough to enumerate, and enumeration reaches every month
    end, skipped and doubled tshes, leap month, 12 → 1 rollover and range edge (rule 41). P2 runs
    in both directions:
      - `to_gregorian(to_zdate(d)) == d` for every day d in the range;
      - `to_zdate(to_gregorian(z)) == z` for every valid z in the range.
  - P4's frequency: derive the expected number of leap months in the range from Janson v1's
    intercalation rule, citing it, and compare it with the measured count.
  - P7, and arbitrary ZDate field values: proptest, added as a dev-dependency (002 §4.9, §6), with
    strategies drawn at the boundaries beside the uniform draw (rule 41).
    - Outside the range, the behaviour is defined: state it and assert it. "Never panics" alone is
      not the property.
    - `to_gregorian` on a skipped tshes returns Err(SkippedDay), and on invalid fields it returns
      Err(InvalidField).
    - A seed proptest writes is named in the commit message, or removed (CLAUDE.md hard rule 14).
  - Planted controls (rule 41), on a scratch copy only, never on the working tree. Plant an
    off-by-one at a month end and another at a leap-month boundary. Each must turn the exhaustive
    run red, three runs in three. Report the failing assertion by name.
  - Rule 35: P1–P4 check consistency, not correctness; a shared wrong epoch passes them all. The
    report says so. The correctness checks are round 006's T5 tables and T4 output, and this
    round's Item 5.
  - Shape coverage, measured and reported (003 §3), each as a count:
    - years;
    - months, split into leap months, 29-day months and 30-day months;
    - skipped tshes;
    - doubled tshes;
    - 12 → 1 rollovers;
    - rabjung-cycle boundaries crossed.
    Also check the identity that, over the complete months in the range,
    skipped − doubled = 30 × (months) − (days), and show it summing.

ITEM 5 — Janson's worked examples, as an outside check.
  - Search Janson v1 for worked numerical examples of the Phugpa computation: a date carried through
    the formulas with its intermediate values. Name the sections you searched.
  - Each example becomes a test whose comment cites the section and page. Its values are written
    in the report one field per line, and checked in a second pass against the page.
  - Add one seeded control: the same example with one value deliberately wrong, asserted to fail
    (002 §6; METHOD rule 31).
  - If v1 has no such example, say so, and name the sections you searched. Then the seeded-control
    requirement of 002 §6 waits for round 006's fixtures: say so, and file nothing.

ITEM 6 — 001 §5.3, location independence (Appendix A, Task 7).
  For each quantity this round computes (the date mapping, each ZDate field), give a verdict with
  its page:
  - VERIFIED: Janson v1 says it does not depend on location or time zone;
  - REFUTED: it says it does, and STOP RULE (4) applies;
  - NOT DECIDED BY THE PAPER.
  Each verdict carries a quotation of two sentences at most. Also record how Janson v1 relates a
  calendar day to a Gregorian civil day, with its page.

ITEM 7 — The glossary (DEBT.md D-2: its TRIGGER fires here).
  - This round is the first to name calendar terms in code, so it creates docs/glossary.md, one
    spelling per term (002 §8), from the source.
  - Where 001 or 002 already spell a term (tshes, ZDate's fields), the glossary uses that spelling
    and records Janson v1's beside it, with the page.
  - Every other term takes Janson v1's spelling, with its page.
  - Mark D-2 PAID, with the date and the commit.

ITEM 8 — The gate, last.
  - Before the final commit, grep 001 and 002 for forks with no ruling and compare them with
    FORKS.md (METHOD rule 59); name the grep.
  - Then run the gate on the final committed tree, in both profiles, with no edits in flight, from
    the same directory Item 1b proved the gate works from (METHOD rule 65; CLAUDE.md hard rule 12).
    If the report changes after the gate has run, run the gate again on the new tip.
  - The gate line follows docs/ROUNDS.md §4.1: counts, never "green", every skipped leg named.
  - Quote, from each tool's own output, the toolchain each leg ran on (METHOD rule 23).
  - Your row in docs/specs/INDEX.md goes in the final commit, not before (METHOD rule 8).

RESEARCH LEGS: not applicable. 003 §4 sets their precondition as Q1, every L2 category and any new
attribute. This round implements 001 §5.1 from the source 001 names. The report says so.

PREDICTIONS — written by the advisor before any measurement (METHOD rule 3). Score each HELD or
FAILED, with the measured value.
  1. 004's zero-dependencies leg passes both planted shapes of Item 1a: the `[dependencies.<name>]`
     table and the `[target.'cfg(unix)'.dependencies]` section. The advisor read this off the leg's
     pattern; it did not run it.
  2. Run from outside the repository, the cargo legs do not run on 1.99.0, and they fail visibly
     rather than pass on another toolchain. The machine's default stable, 1.97.1 (round 004's
     report), is older than rust-version 1.99.0.
  3. Janson v1 states every constant and formula the Phugpa mapping needs; STOP RULE (1) is not hit.
  4. P1–P3 hold on every day of the range, in both profiles, with zero failures; P2 holds in both
     directions.
  5. Leap months in 1900–2100: 74 ± 1. The advisor derived this from Phugpa intercalating 2 months
     in every 65 solar months (RECALLED / UNVERIFIED); Janson v1's rule decides.
  6. The identity of Item 4 sums exactly. Over the range, skipped − doubled = 1167 ± 10. The advisor
     derived this from a mean lunar month of about 29.5306 days over roughly 2486 months
     (RECALLED / UNVERIFIED).
  7. §5.3: VERIFIED for every quantity this round computes.
  8. `cargo deny check` passes with proptest's dependency tree under the current allow list.
  9. Janson v1 contains at least one worked numerical example of the Phugpa computation.

THE REPORT — per CLAUDE.md's report contract and docs/ROUNDS.md §4, in
docs/specs/005-phugpa-core.md: the head block; a report-shaped spec (docs/specs/TEMPLATE.md's header
lines — Milestone M1; Governs: crates/tshes, tools/, docs/glossary.md — then the end marker); then the
report.
  - Lead with the decisive item: THE CITATION TABLE with its second-pass verdicts, and Item 5's
    examples.
  - Then:
    - P1–P4 and P7, each DONE / DEFERRED / N-A with evidence, and the shape-coverage counts;
    - P5 and P6: N-A here, waiting for round 006;
    - Item 6's verdicts;
    - Item 1's plant table.
  - Every clause of 002 §4–§6 that this round's code touches, scored.
  - The predictions, scored. D-1's and D-2's states. The open-forks audit, with its arithmetic at
    every commit. The marker greps, at every commit.
  - Every other section of the contract, "What the prompt got wrong" included.
  - The diff scope with its totals shown summing against `git diff --stat main`, at the final tip.
  - The squash commit message (docs/ROUNDS.md §4.2, item 13), naming every FORKS.md and DEBT.md
    entry the round adds or changes.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 005 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════
