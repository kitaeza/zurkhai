═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 015 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  015 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
ARC:          Interpretation (the arc this round's Item 0 renames from Rules, on the human's ruling I1),
              assigned by the advisor with the spec number; the human may overrule it.
BRANCH:       spec/015-interpretation-sources, cut from main at 99c56e3 before any edit; commit to it.
              No merge. No push, to any remote.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7's first branch, "discoverable": what a calendar
              prints, where, and with what words is read, not invented; the records of Item 0 are
              dictated word for word. A wrong reading would be SILENT (a missed legend, a category
              misnamed, a key the source never states), so the round runs on the smaller model only with
              the instruments that make it loud: every finding carries its location and a short quote;
              a blind second reading by a delegated reader (Item 4); and a count of every per-day mark
              against the golden tshes and the weekday, which a misread mark moves (Item 3). A CHANGE from
              the last round: spec 014's session 2 ran on Opus 5.5 (spec drafting); this round drafts no
              spec text beyond its own report, and decides nothing.
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first, and never commits a previous session's uncommitted work without
              proving it by the item's own check. A design choice that would fall to the fallback model
              (Opus 5.5, effort High) is a STOP AND ASK. The report names the switch point.
PRECONDITION: Check each, and quote every output:
              (a) `git -C /home/kitaeza/github/zurkhai status --porcelain` prints nothing, and
                  `git -C /home/kitaeza/github/zurkhai rev-parse --abbrev-ref HEAD` prints main.
              (b) `git -C /home/kitaeza/github/zurkhai rev-parse main` prints
                  99c56e3a31b675458626f14fce6b3574e3234f68, spec 014's squash.
              (c) `ls /home/kitaeza/zurkhai-papers/` succeeds (the human has added it with /add-dir).
              Anything else: STOP and report, with zero diff.
              Read first, in this order, each in full unless a part is named: CLAUDE.md;
              docs/ROUNDS.md; docs/METHOD.md rules 1, 3, 4, 27 to 29, 31, 35, 50, 51, 58, 60, 66 and 67,
              and Part 2's P2 and P3, each in full; GATED.md; FORKS.md (its header; F4, F8, F32 to F38 in
              full; the rest by their status lines); DEBT.md (its header; D-18, D-33, D-34, D-35 and D-40
              in full; the rest by their headings); docs/ROADMAP.md; docs/BACKLOG.md;
              docs/specs/001-init-spec.md (§1, §2, §3, §5.2, §6.3, §7, §10, §12);
              docs/specs/003-project-method.md (§4; §7); docs/specs/007-to-verify-attributes.md (the head
              block and its Item 3 table); docs/specs/012-source-checks.md (the head block and its
              findings per source); docs/specs/013-buryat-sources.md (the head block and the rulings it
              records); docs/specs/014-q1-verdict.md (the spec text above "— spec text ends here —");
              golden/buryat/README.md; docs/sources.md (in full); docs/specs/INDEX.md;
              docs/specs/TEMPLATE.md. The report lists each file it read and how much of it.
ARTIFACT:     In the tree at 99c56e3: golden/buryat/ivolginsky-datsan-2021.csv,
              golden/buryat/aginsky-datsan-2025.csv, golden/buryat/aginsky-datsan-2026.csv (re-count
              their rows). In the paper library, /home/kitaeza/zurkhai-papers/, read only, each with the
              sha256 the advisor MEASURED on 2026-11-13 (RELAYED); re-measure each:
                - ivolginsky-datsan/lunnii_kalendar-2021.pdf
                  2e140c65b7cac71cbb6c98b837e1f7488a91d47ccd7ea299f3b062d7e320fe25
                - aginsky-datsan/aga/kalendar2025.pdf
                  12137318075f3a7ae946f946b5bbf1b4f628e4186adbdad033b510f2868bd7bb
                - aginsky-datsan/aga2026/kalendar-a2-format.pdf
                  13e7d720b3ba9dd8d47abe090584970afe91f43d6892e8f5cc811351832aa156
                - gunzechoinei-datsan/dazanspb2024/kalendar_2024-2025_gg.jpg
                  e4b68f82049112d1f89816b663a1f44de0522f958b0750ea6746c23634f4c290
                - gunzechoinei-datsan/dazanspb2024/perekidnoj_kalendar_2024.jpg
                  d321a83c1a9a679ba572df8283885fb6946c001fd7ffeae9f35514b3d127f6cf
                - gunzechoinei-datsan/dazanspb2024/perekidnoj_2024_str.jpg
                  68a2fecf018e6b893173027e19eb4cfb31a6bd9c9997aa6a3491f8fd67051c77
                - gunzechoinei-datsan/dazanspb/perekidnoj_2025.jpg
                  f076c327ff07520da6a7946f9f7d06995bf7595bbf50753c15ad5e37335ac285
                - gunzechoinei-datsan/dazanspb/perekidnoj_2025_1.jpg
                  012865226594832b6d96b2e159966839f5082ed1a5fa13c4979633631fd119e0
                - gunzechoinei-datsan/dazanspb/kalendar_plakat.jpg
                  f96f0a14a4901c7bc44e8ee23947b79cd0e4064fceb7b9034c617bb8a0a0ce61
                - gunzechoinei-datsan/dazanspb2026/al_bom_kalendar_2026_1.jpg
                  406fb146db317d605ba39795d87b00d0c2da89282a478a3722d253982b008213
                - gunzechoinei-datsan/dazanspb2026/al_bom_kalendar_2026_2.jpg
                  2008f5abd0b7206286e888488edfa4a49c572dee79ea73750e348d7c401224f6
                - erlewine-tibetan-astrology/tibetan-astrology-9780979497063_compress.pdf
                  479c7517f547e8370893da69a4c39d41972fa84ffd391f4088ca33e0ac16420f
              Also readable, as the text conversions beside them: the page.txt of each of the three
              gunzechoinei-datsan/ directories; aginsky-datsan/aga/aga-2025-textlayer.txt and
              aginsky-datsan/aga2026/aga-2026-textlayer.txt; ivolginsky-datsan/text.txt (each is a
              derivative made by the advisor, not a source: anything cited from one is re-read on the
              PDF or image it came from). If a named file is absent, or a sha256 differs: STOP and report.
STOP RULE:    (1) A PRECONDITION or ARTIFACT check fails: STOP, zero diff.
              (2) Nothing is interpreted in your own voice. A category is named in the publication's own
              words (quoted, at most one line, with its location), and a key is recorded only as the
              publication states it ("keyed on: as printed, «…»"); where it states none, write "keyed on:
              not stated". Never write that a mark means, follows or is keyed on an attribute because the
              counts of Item 3 suggest it: the counts are reported as counts.
              (3) If a held publication turns out to mix Western astrological "лунные дни" with tshes (001
              §2, §6.3, §12; CLAUDE.md hard rule 9), STOP AND ASK before anything else is committed: it
              would bear on golden rows and on spec 014's scope.
              (4) A finding bears on a ruling already made (F32 to F41, or the rulings below): do not
              change the ruling or its row. Report the finding beside it.
              (5) Out of scope; do not write any of it: any change under crates/, rules/, clients/,
              tools/, .github/ or golden/; Cargo.toml, Cargo.lock, rust-toolchain.toml, deny.toml;
              docs/METHOD.md, GATED.md, CLAUDE.md, docs/ADVISOR.md, docs/ROUNDS.md; any accepted spec
              except spec 014's revision of Item 0 (f); another round's row in docs/specs/INDEX.md; any
              rule, rule schema or rule text; any attribute's implementation; any per-day mark entered
              as golden data; any contact with anyone; any network access; deciding any fork.
              (6) A read, a run or a network call outside GRANTS is needed, or a permission is denied:
              STOP and report each, with the reason. Never work around it by another tool or route
              (METHOD rule 67).
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; there is no fourth
              kind. A citation of a library file is FETCHED with its path and page, crop or line. A
              figure from an earlier round's report is RECALLED with its spec and section, and
              re-measured wherever this round can measure it (CLAUDE.md hard rule 10). Read
              docs/sources.md before anything is cited. No network (GRANTS). (METHOD rule 50.)
GATED:        yes: docs/specs/014-q1-verdict.md, for Item 0 (f)'s revision only (two in-place
              corrections and a Revisions line), on the human's ruling C1–C2 of 2026-11-13; this round's
              own docs/specs/015-interpretation-sources.md, its committed prompt and its INDEX row,
              which are not changes to an accepted spec. Not gated, and changed only as the items say:
              DEBT.md, FORKS.md, docs/ROADMAP.md, docs/sources.md.
GRANTS:       Reads outside WORKDIR, beyond this prompt file: /home/kitaeza/zurkhai-papers/, read only,
              and only the files of ARTIFACT, with the Read tool (images included), ls, stat, sha256sum,
              grep, pdfinfo, pdftotext and pdftoppm. pdftotext and pdftoppm write only into a `mktemp -d`
              directory; crops you make go there too.
              Network: none. No WebSearch, no WebFetch, no curl, wget, git fetch, gh, cargo commands
              that reach the network (use `--offline`), or python networking.
              The Agent tool: exactly one delegated reader, for Item 4 only. Its brief carries this
              GRANTS line verbatim (CLAUDE.md hard rule 11; METHOD rules 66, 67), names the exact images
              and crops it reads, forbids it every other file of the library and of the repository and
              any sight of your own inventory, and names the model it runs on; the report records that
              model as the delegate reports it. Its output is a file in your own `mktemp -d` directory.
              The tools, in the repository:
                - the Read tool; the Edit and Write tools, for the files the items name;
                - git, read-only (status, log, show, rev-parse, diff), and what the round's own branch
                  and commits need (switch -c, add, commit);
                - tools/gate.sh, once, at Item 8, as its own header says; cargo only through it;
                - `python3 -I`, running only scripts you write into a `mktemp -d` directory, reading
                  only the repository, the ARTIFACT files and your scratch; never an inline `-c` run;
                - read-only shell tools on the repository and your scratch: cat, head, tail, sed -n,
                  grep, awk, cut, tr, sort, uniq, wc, diff, cmp, ls, stat, find, date, file, sha256sum;
                - mkdir -p, cp and rm -r, only inside `mktemp -d` directories.
              On this machine `grep` is ugrep: keep bounded repetitions below 100.
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive
              widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 015 — Interpretation sources: what the held Buryat calendars print about a day, keyed on what

The Q1 verdict is merged (spec 014): the engine's Genden is the computation system the admitted Buryat
sources support. The next layer is L2, the sourced interpretations of a day (001 §3, §7). Before a rule
schema is designed, this round takes an inventory of what the Buryat Sangha calendars in the paper
library actually print about a day: which categories (haircut, travel, water procedures, …), keyed on
what, how often, and on what terms. It also records the human's rulings of 2026-11-13 on the
Interpretation arc and the fixes carried from the close review of spec 014. It writes no rule, no
schema, no attribute and no golden row, and contacts no one.

THE HUMAN'S RULINGS this round carries (2026-11-13; the human, to the advisor's recommendations: "Agree
with all recommendations", for I1 to I4, A and B; "Agree with all recommendations, go." for C1 to C7).
Each is quoted in the report beside the record that implements it.
  I1. The arc that holds the L2 rules is named Interpretation (it was Rules). Item 0 (a).
  I2. The advice for a day is the list of every sourced rule whose condition holds that day, each with
      its source and tradition, contradicting rules side by side; a combined verdict only where a
      source itself states how its rules combine; no generated prose (001 §2 stands). FORKS.md F39,
      Item 0 (b).
  I3. Forced, not a choice: interpretations come from Buryat Sangha sources only (001 §1, §2;
      CLAUDE.md hard rule 8). Recorded in the report.
  I4. The order of the Interpretation arc: this inventory first, then l2-schema, then day-attributes
      (the attributes the inventory shows the sources use; it reopens DEBT.md D-18), then further rule
      tables, the website, and the review by a зурхайч. Item 0 (a).
  A.  Whether a Tibetan interpretive rule set (for example the Tsurphu electional rules of M. Erlewine's
      "Tibetan Astrology", in the library) may serve as an oracle against the recommendations Buryat
      datsans print is decided AFTER this round lands, on its numbers. FORKS.md F40 (OPEN), Item 0 (b);
      Item 6 (b) gives the numbers.
  B.  No second interpretive tradition (such as recommendations labelled as Erlewine's) now; 001 §1–2
      stand; revisit no earlier than M4/M5. FORKS.md F41, Item 0 (b).
  C1–C7. The fixes from the close review of spec 014 (a reviewer session on Fable 5.2, effort High, at
      7bb0bed; its findings RELAYED here, the engine never reads its report): C1 and C2, Item 0 (f);
      C3, C4, C6 and C7, Item 0 (c); C5, Item 0 (a).

THE ADVISOR'S CORRECTION (2026-11-13; not a ruling): DEBT.md D-18's TRIGGER (iii), "the Q1 verdict spec
is accepted (M3)", fired with spec 014, and spec 014's ADDENDUM 1 named no action for it. Item 0 (c)
records it.

ITEM 0 — The branch, the prompt, the records. One commit after the prompt's, before Item 1.
  - Verify 015 is free; create the branch; commit this prompt byte-identical to
    docs/specs/prompts/015-interpretation-sources.md as the first commit, and `cmp` it against
    /home/kitaeza/zurkhai-prompts/spec-015-interpretation-sources.md before every later commit. Run
    `date` once (METHOD rule 51); every <date> below is its date. Choose the round's marker (METHOD rule
    58). Re-measure the OPEN count (4 expected: F5, F7, F8, F10; RELAYED).
  (a) docs/ROADMAP.md, r8:
      - The Status line's r7 becomes r8, and its date the round's date.
      - In the list of arcs, the line "- **Rules**: the L2 rules (M4)." becomes:
        - **Interpretation**: the L2 rules: what Buryat sources say a day means, each rule sourced and tradition-tagged (M4).
      - Table 1, before the l2-schema row, a new row, one field per line here:
          arc: Interpretation
          milestone: M4
          round: interpretation-sources — PLANNED
          achieves: the inventory: which held Buryat Sangha publications print interpretations of a day, in which categories, keyed on which attributes, and on what terms
          opens: l2-schema; day-attributes; the numbers FORKS.md F40 waits for
          TRIGGER: the human's ruling I4 (2026-11-13)
      - The l2-schema row: its arc cell "Rules" becomes "Interpretation"; its TRIGGER cell becomes
        "interpretation-sources landed". Nothing else in the row changes.
      - After the l2-schema row, two new rows:
          arc: Engine
          milestone: none
          round: day-attributes — PLANNED
          achieves: the L1 day attributes the inventory shows Buryat sources condition on, each from a cited algorithm (001 §5.2); it reopens DEBT.md D-18 for the rows it needs
          opens: rule tables keyed on those attributes
          TRIGGER: interpretation-sources landed
        and
          arc: Interpretation
          milestone: none
          round: rule-tables — PLANNED
          achieves: further sourced rule tables, one Buryat source per round, in the categories the inventory finds
          opens: the website's day view (FORKS.md F39)
          TRIGGER: l2-schema landed, and the attributes each table needs are exposed
      - Table 2, Gate 1's "what it checks" cell (C5): "which computation system the Buryat calendars
        follow" becomes "which computation system the admitted Buryat sources support (001 §4; scoped
        by spec 014's verdict)". Its state cell is not changed.
      - A Revisions line: "- r8 (<date>): the arc Rules renamed Interpretation; Table 1: new rows
        interpretation-sources, day-attributes and rule-tables, and l2-schema's arc and TRIGGER
        changed; Table 2: Gate 1's 'what it checks' cell scoped to spec 014's verdict; on the human's
        rulings I1, I4 and C5 of 2026-11-13 (spec 015 Item 0)."
  (b) FORKS.md, three new rows, numbered from the next free id (F39 expected, RELAYED; if not, use the
      next free ids in order and say so), one field per line here:
      F39:
        subject: What a client shows as the advice for a day
        Raised: 2026-11-13, the advisor's question I2 to the human (recorded by spec 015 Item 0)
        Question: When a client shows a day, how are the interpretations that hold on it presented?
        Options: (a) every sourced rule whose condition holds that day, each with its source and tradition, contradicting rules side by side; a combined verdict only where a source itself states how its rules combine (001 §7, §10 M5); (b) a combined verdict per category; (c) prose generated from the rules (against 001 §2, "No authored interpretations")
        Status: RULED
        Ruled: 2026-11-13, (a). The human: "Agree with all recommendations" (spec 015, I2)
      F40:
        subject: Whether a Tibetan interpretive rule set may serve as an oracle against Buryat datsans' printed advice
        Raised: 2026-11-13, the advisor's question A to the human (recorded by spec 015 Item 0); the human ruled that it is decided after spec 015 lands, on its numbers
        Question: May the rules of a Tibetan (non-Buryat) interpretive source, such as M. Erlewine's "Tibetan Astrology" (Tsurphu), be applied to dates and compared with what Buryat datsan calendars print, as a measurement kept outside rules/?
        Options: (a) yes, as an interpretation oracle outside rules/, which needs CLAUDE.md hard rule 8 and 001 §2 amended first; a match would not show that a datsan uses that source; (b) no
        Status: OPEN
      F41:
        subject: A second interpretive tradition beside the Buryat one
        Raised: 2026-11-13, the advisor's question B to the human (recorded by spec 015 Item 0)
        Question: Should clients offer, beside the Buryat rules, a separately labelled Tibetan tradition (for example recommendations labelled as Erlewine's "Tibetan Astrology"), computed on its own system's dates?
        Options: (a) not now: 001 §1–2 stand; revisit no earlier than M4/M5; (b) yes, which needs a revision of 001 §1–2, CLAUDE.md hard rule 8 changed, and the publisher's written permission; (c) revisit at M5
        Status: RULED
        Ruled: 2026-11-13, (a). The human: "Agree with all recommendations" (spec 015, B)
      The open-forks audit after (b): 4 + 1 (F40) = 5.
  (c) DEBT.md:
      - D-18 (C6): after its TRIGGER line's text, in place: "[CORRECTED <date>: TRIGGER (iii) fired
        when spec 014 was accepted and merged (2026-11-13, 99c56e3); spec 014 named no action for it
        (the advisor's correction, spec 015). On the human's ruling I4 of 2026-11-13 the rows are
        reopened by the day-attributes round of docs/ROADMAP.md, for the rows the
        interpretation-sources inventory (spec 015) shows a Buryat source needs. The TRIGGER is now:
        the day-attributes round's prompt is written.]". Status OPEN. Nothing else changes.
      - Three new entries from the next free id (D-41 expected, RELAYED), each Status OPEN:
        D-41 (C4):
          subject: The 6 open days of spec 014's verdict are unexplained
          Filed: <date>, spec 015 Item 0 (the close review of spec 014, its NOT ASKED 2, RELAYED; the human's C4)
          Kind: CORRECTNESS
          Statement: On 2025-12-22 to 2025-12-27 the Aginsky 2025 edition prints Genden's tshes plus one (golden/buryat/aginsky-datsan-2025.csv). Spec 014's verdict records these 6 days as UNEXPLAINED and OPEN and keeps them as its weakness; no variant is computed or adopted (CLAUDE.md hard rule 7).
          Pinned by: q1_pin_a_measurement_not_a_correctness_claim_tshes_matches_and_mismatching_dates_of_spec_014
          TRIGGER: the first of: a зурхайч, or an admitted source, names a published rule that accounts for them (spec 014, Reopening 3); the approach to Гунзэчойнэй under FORKS.md F4 (b) records an answer about them; an admitted calendar prints that lunation again.
        D-42 (C3):
          subject: Two figures spec 014's verdict cites are asserted by no test, and a test comment calls F38 OPEN
          Filed: <date>, spec 015 Item 0 (the close review of spec 014, its FINDINGs 4.1 and 4.2 and NOT ASKED 1, RELAYED; the human's C3)
          Kind: COVERAGE
          Statement: (1) The Non-properties' "the only day of spec 009's lists inside the files, 2025-11-19" is printed by q1_report_d23_the_days_of_spec_009_inside_the_golden_files_and_whether_it_fires and asserted by no test. (2) "The extra month": that the second lunation, 2025-01-30 to 2025-02-28, carries month 12 in Genden and Tsurphu is printed only. (3) The doc comment of the spec 014 pin in crates/tshes/tests/q1_differential.rs still describes FORKS.md F38 as OPEN. The reviewer re-measured (1) and (2) as true.
          Pinned by: not pinned
          TRIGGER: the next round that edits crates/tshes/tests/.
        D-43 (C7):
          subject: The gate's deny leg may reach the network unseen
          Filed: <date>, spec 015 Item 0 (the close review of spec 014, its NOT ASKED 7, RELAYED; the human's C7)
          Kind: TOOLING
          Statement: tools/gate.sh runs `cargo deny check` with no offline switch of its own; a round whose GRANTS say "Network: none" cannot see from the gate's output whether cargo-deny fetched its advisory database.
          Pinned by: not pinned
          TRIGGER: the next round of the Process arc, or the next round that edits tools/gate.sh.
  (d) docs/sources.md, the library table: one row for the directory erlewine-tibetan-astrology/ (the
      human asked the advisor on 2026-11-13 to add the book to the library; the advisor copied it there
      that day, 14:51 +0300, from a copy the human had downloaded; where the human obtained the copy is
      not recorded: RELAYED). Measure what the row states: `pdfinfo` (pages, creator, creation date),
      the sha256, and the title page and the copyright page by `pdftotext` (FETCHED, with page). Its
      "Licence" cell quotes the copyright page's rights line (one sentence at most). Its "Trusted
      for" cell: "Nothing yet: a secondary, Tsurphu (Karma Kagyu) account, read only as research leg
      ② (field) by spec 015; never evidence of Buryat practice and never an interpretation in rules/
      (CLAUDE.md hard rule 8; FORKS.md F40, F41)."
  (e) The open-forks audit, and the marker grep, before the commit.
  (f) docs/specs/014-q1-verdict.md, revision r2 (C1, C2), in place, nothing else changed:
      - In the spec text's "The open lunation, described", after the words
        "14 holds only for δ in [−1.2, +2.6) minutes", insert: " [CORRECTED <date>: the measured
        interval is [−1.17, +2.55) minutes, as S2.1 and S2.4 give it; the rounding above is wider
        than it at both ends (the close review's FINDING 6.1, RELAYED; the human's C1)]".
      - In the report's opening paragraph, after the words "the report states counts, and which
        branch of 001 §6.5 they satisfy as its words read", insert: " [CORRECTED <date>: the counts
        satisfy no rule of 001 §6.5 by themselves; which rule applies is the human's ruling R5 (b),
        as §1's correction says (the close review's FINDING 7.1, RELAYED; the human's C2)]".
      - A Revisions line under r1: "- r2 (<date>): two in-place corrections, in "The open lunation,
        described" and in the report's opening paragraph, on the human's ruling C1–C2 of 2026-11-13
        (spec 015 Item 0)." The Status line and the INDEX row are not changed.
      If either quoted phrase is not found exactly once: STOP and report, zero diff for (f).

ITEM 1 — What the round stands on.
  - PRECONDITION and ARTIFACT, every output quoted; the golden row counts re-measured.
  - The research legs, copied from 003 §4 (P3) and reported each, including empty ones, with what it
    found against the approach: ① reference: datsan publications, and a зурхайч where reachable (this
    round: the held publications of ARTIFACT; no зурхайч is reachable, say so). ② field: other
    implementations (Lotsawa House, Henning's software, existing Mongolian apps); pin where their
    conventions differ (this round: Item 5's reading of Erlewine's book only; nothing else is read).
    ③ science: Janson, Henning, Schuh, Reingold–Dershowitz, or an explicit "none relevant" (this
    round: which 001 §5.2 attributes Janson v1 defines, RECALLED from spec 007's Item 3 table, for
    Item 6 (a)). ④ Does this contradict anything already shipped?
  - Which held publications are in scope (I3): the publications whose Sangha membership is shown under
    FORKS.md F32 (a), as spec 013 applied it (each library row of docs/sources.md records the result,
    "Sangha membership under R4 (FORKS.md F32)"): the Ivolginsky datsan (2021), the Aginsky datsan
    (2025, 2026) and the datsan Гунзэчойнэй (the images of ARTIFACT). Out of scope, each with its
    reason, read for nothing: the Хойморский datsan and «Ринпоче Багша» (membership not shown under
    F32 (a)), the Kalmyk Central Khurul and Rabten Choeling (not Buryat). Re-check each against its
    docs/sources.md row and say so; a publication whose row says otherwise than this list is a STOP AND
    ASK.

ITEM 2 — The inventory, per publication. For every element of each in-scope publication that says
  something about a day or a period beyond its date, tshes, month and year (a legend, a sign or icon in
  a day's cell, a line of text, a named period, a table of recommendations, a note in a margin), record,
  one field per line:
    publication: <file>
    location: <page, region or crop; the crop file you made, by its sha256>
    category: <in the publication's own words, quoted, at most one line>
    form: legend | per-day sign | per-day text | named period | table | other (say which)
    keyed on: as printed, «…» | not stated
    days or periods carrying it: <count; for a per-day sign, the count of days>
    terms: <what docs/sources.md and DEBT.md D-35 record for the publication's reuse terms>
  Also record what is absent: for each publication, the categories of 001 §7's list (haircut, travel,
  construction, medical, wedding) it does NOT print, each with the search that found nothing (on the
  text layer or conversion, the words searched; on the image, the regions read). A legend whose sign
  is printed but never read on a day, and a sign read on a day with no legend entry, are both findings.

ITEM 3 — Per-day signs against the golden tshes and the weekday (a measured description; STOP RULE (2)).
  Only for per-day signs (Item 2's form "per-day sign" or "per-day text") in the Ivolginsky 2021 and the
  two Aginsky editions, whose days have golden rows:
  (a) Transcribe, for every day, which signs its cell carries, into a file in your scratch (one line
      per day: the date, then the categories, by Item 2's names). Report its sha256; put in the
      report's appendix only the days that carry a sign, one line each.
  (b) With a script of your own (`python3 -I`, in your scratch), count for each category: the days
      carrying it per golden tshes value 1 to 30 (the golden file's own `tshes` column) and per weekday
      (from the Gregorian date). Report the tables, and for each category whether every day with a
      given tshes carries it, none does, or some do (with the counts). Do the same per weekday. These
      are counts, never a statement of what the sign is keyed on.
  (c) The control, first (METHOD rule 31): run the script on a copy of your transcription with one
      sign planted on a day that does not carry it, and show the tables move by exactly that day.
  If no in-scope publication prints a per-day sign, Item 3 is N-A, with the evidence of Item 2.

ITEM 4 — The blind second reading (the instrument for Items 2 and 3).
  One delegated reader (GRANTS), on a model it names, reads blind: the whole of the Aginsky 2025
  edition and the Ivolginsky 2021 calendar, as page images or crops you make and name in its brief
  (never your inventory, never the text conversions). It reports, in the form of Item 2, every element
  it finds, and, for two months you choose and name in its brief (one per publication; say why those
  two), the per-day signs of every day. Compare with your Items 2 and 3: every element one reading has
  and the other lacks, and every day the two transcriptions disagree on, listed. Re-read each
  disagreement yourself on the image and record which reading the image supports; a disagreement you
  cannot settle stays open in the report. Report the delegate's model as it reports it.

ITEM 5 — Leg ② (field): Erlewine's "Tibetan Astrology", as a field reading only.
  From its table of contents and its chapter on electional astrology (find the pages), list the
  categories of day advice it gives and what each is keyed on as the book states it (lunar day,
  weekday, day animal, karana, yoga, mewa, parkha, lunar mansion, a person's birth year, …), with
  pages. Counts and keys only: quote nothing beyond a heading or a phrase of a few words, and transcribe
  no table. Say for each category whether some in-scope publication prints the same category (Item 2).
  This is never evidence of Buryat practice and enters no record but this report (FORKS.md F40, F41).

ITEM 6 — What the inventory feeds.
  (a) The attribute demand table: for each key that Item 2 records as printed (and, separately, each
      key that Item 5 records for Erlewine's categories), the matching row of 001 §5.2's table, whether
      Janson v1 defines it (RECALLED from spec 007's Item 3 table, cited), and whether the engine
      exposes it today (read the engine's public API at 99c56e3 and cite the item). This is the input
      of the day-attributes round; it decides nothing.
  (b) For FORKS.md F40, the numbers only: per in-scope publication, the categories it prints per day
      that Erlewine's book also covers, and the number of days that carry each. No option is chosen and
      none is argued.
  (c) The terms: for each in-scope publication, what its recorded terms (docs/sources.md, DEBT.md D-35,
      FORKS.md F33 to F35) say about reusing an interpretation, as opposed to a date or a tshes; where
      they say nothing on it, say so. No legal conclusion.

ITEM 7 — docs/sources.md, "Reference facts": one entry per in-scope publication, tagged DOC, stating in
  one or two sentences what it prints about a day (its categories and forms from Item 2, with their
  locations) and, where Item 2 found it, the key it states. Facts only, in your words, no quotation
  beyond a category's name. Replace the section's "None yet." line.

ITEM 8 — The close.
  - docs/specs/015-interpretation-sources.md: the head block (docs/ROUNDS.md §4.1); Status: draft;
    Depends on: 001, 003, 012, 013, 014; Resolves: F39, F41 (filed already RULED by Item 0); Milestone:
    M4; Governs: docs/sources.md (Reference facts). Then the report (CLAUDE.md's report contract, in its
    order).
  - docs/ROADMAP.md: the interpretation-sources row becomes LANDED; a Revisions line (r9).
  - docs/specs/INDEX.md: this round's row, status `draft`.
  - tools/gate.sh once, with nothing in flight, both profiles, on the last commit that changes a path
    beyond the report; its count against spec 014's 180 (no test is expected to change).
  - The open-forks audit (5 expected). The marker grep. Account for every file `git status
    --porcelain` shows.
  - "The squash commit message" at the end of the report.

PREDICTIONS — written by the advisor on 2026-11-13, before this round measures anything (METHOD rule 3).
Score each HELD or FAILED with your own measured value.
  18. The text layers of both Aginsky editions hold none of the words стриж, волос, благоприят,
      неблагоприят, поездк, путешеств, дорог (basis: the advisor's grep of aga-2025-textlayer.txt and
      aga-2026-textlayer.txt, 0 lines each, 2026-11-13; MEASURED by the advisor, RELAYED). So any day
      advice they print is in signs, not words, or absent.
  19. The Ivolginsky 2021 calendar prints a legend of named periods, among them «Пагшаг» and «Риха»,
      described in terms of water procedures (basis: the advisor's grep of ivolginsky-datsan/text.txt,
      2 lines; MEASURED, RELAYED).
  20. No in-scope publication states the attribute a per-day sign is keyed on (the advisor's guess,
      no measurement behind it).
  21. OPEN is 4 at the base and 5 after Item 0 (F40).

THE REPORT leads with the inventory (Item 2's table, every publication, every element, with what is
absent), then Item 3's counts and Item 4's disagreements, then Items 5 and 6. Then the report contract:
every item DONE / DEFERRED / N-A with evidence; the rulings I1 to I4, A, B and C1 to C7 beside their
records; the research legs; the predictions scored; what the prompt got wrong ("none found" is an
entry); by-catch with its DEBT.md id; the diff scope summing against `git diff --stat main`.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 015 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════
