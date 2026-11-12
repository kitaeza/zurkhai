═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 011 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  011 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
ARC:          Process — the arc's label in docs/ROADMAP.md, which this round creates (Item 1).
BRANCH:       spec/011-roadmap-backlog, cut from main at 0d1d2db before any edit; commit to it.
              No merge. No push, to any remote.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7, first branch. Every decision this round
              writes down is the human's, carried below as R1–R9; the wording is the round's, and the
              advisor reads it at the landing. Two parts would be silent, and each runs only with an
              instrument that makes it loud (docs/ROUNDS.md §8):
                - the dictated rows. A cell copied wrong into docs/ROADMAP.md or docs/BACKLOG.md reads
                  like a right one. Instrument: a script compares every committed cell with this
                  prompt's dictated fields, and a seeded wrong cell must make it fail (Items 1, 2);
                - a ruling written into ADVISOR.md or ROUNDS.md so that it says something the human did
                  not rule. Instrument: the report quotes each ruling's sentence from this prompt beside
                  the sentence that implements it (METHOD rule 61), one pair per ruling (Items 3, 4).
              No change from the last round: round 010 also ran on Sonnet 5.5, effort High.
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first. It never commits a previous session's uncommitted work without
              verifying it: it finishes the work and proves it by the item's own check, or redoes it. A
              design choice that would fall to the fallback model (Opus 5.5, effort High) is a STOP AND
              ASK. The report names the switch point.
PRECONDITION: Check each, and quote every output:
              (a) `git -C /home/kitaeza/github/zurkhai status --porcelain` prints nothing, and
                  `git -C /home/kitaeza/github/zurkhai rev-parse --abbrev-ref HEAD` prints main.
              (b) `git -C /home/kitaeza/github/zurkhai rev-parse main` prints
                  0d1d2dbad3ca25854991865ac26d4b75f50910b3, spec 010's squash, and
                  `git -C /home/kitaeza/github/zurkhai rev-list --count main` prints 12.
              Anything else: STOP and report, with zero diff. The merge is docs/ROUNDS.md §9's.
              Read first, in this order, each in full unless a part is named: CLAUDE.md;
              docs/ROUNDS.md; docs/ADVISOR.md; docs/specs/003-project-method.md (§2, §4's P4 row, §7,
              §11, §12); docs/specs/001-init-spec.md (§10 and §11); docs/METHOD.md rules 1, 3, 8, 9,
              31, 36, 51, 56, 58, 60, 61, 65 and 67, each in full, not only its **Rule.** paragraph;
              GATED.md; FORKS.md (its header; F4, F5, F7, F10, F11, F22 and F26 in full; the rest by
              their status lines); DEBT.md (its header; D-1, D-8, D-19, D-23 and D-24 in full; the rest
              by their headings); docs/specs/INDEX.md; docs/specs/TEMPLATE.md;
              docs/specs/010-publication.md (its head block, and its by-catch on actions/checkout);
              docs/sources.md (its first section, and its rows for the library holdings
              kalmyk-central-khurul, rabten-choeling and henning-kalacakra, if present);
              .github/workflows/ci.yml. The report lists each file it read and how much of it.
ARTIFACT:     None outside the repository. Item 5 reads one network source, named in GRANTS; if it is
              UNREACHABLE, write the entry from the RELAYED text below, say so, and continue.
STOP RULE:    (1) A PRECONDITION check fails: STOP, zero diff.
              (2) A ruling below cannot be written without deciding something it leaves open, or two
              rulings contradict each other, or a ruling contradicts an accepted spec in a way that is
              not one of the lags Item 5(d) lists: STOP AND ASK, quoting both texts.
              (3) A seed of Item 2 is already covered by a DEBT.md entry or a FORKS.md row: do not file
              it; the report names the id that covers it. Not a stop.
              (4) Out of scope; do not write any of it:
                - any change under crates/, golden/, rules/, clients/, tools/ or .github/;
                  docs/METHOD.md, GATED.md, docs/sources.md, docs/glossary.md, an accepted spec,
                  docs/specs/TEMPLATE.md, or another round's row in docs/specs/INDEX.md;
                - CLAUDE.md beyond the two edits of Item 4(g);
                - any push, fetch or pull; deleting a branch; creating a tag;
                - deciding any fork, or writing a FORKS.md row other than Item 0's.
              If something seems to need one of these, write it down for the report (by-catch, and a
              DEBT.md entry where it qualifies), and do not do it.
              (5) A read, a run or a network call outside GRANTS is needed, or a permission is denied:
              STOP and report each, with the reason. Never work around it by another tool or route
              (METHOD rule 67).
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; there is no fourth
              kind. An unobtainable source says PAYWALLED or UNREACHABLE. Read docs/sources.md before any
              fetch. Never curl/wget. (METHOD rule 50.) This round's only fetch is Item 5(a)'s: cited as
              FETCHED, with its command, the lines used and the date.
GATED:        no gated path, except this round's own docs/specs/011-roadmap-backlog.md, its
              docs/specs/prompts/011-roadmap-backlog.md, and its row in docs/specs/INDEX.md, which are
              not changes to an accepted spec. Not gated, and changed only as the items say:
              docs/ROADMAP.md (new), docs/BACKLOG.md (new), docs/ADVISOR.md, docs/ROUNDS.md, CLAUDE.md,
              DEBT.md, FORKS.md.
GRANTS:       Reads outside WORKDIR, beyond this prompt file: none.
              Network, read-only, and only: `gh run view 43107437324 -R kitaeza/zurkhai` (Item 5(a)).
              No other network, and no gh command that writes.
              The tools:
                - the Read tool, on the repository;
                - git on the repository, read-only (status, log, show, rev-parse, rev-list, ls-tree,
                  grep, diff), and what the round's own branch and commits need (switch -c, add, commit);
                - `python3 -I`, running only scripts you write into a `mktemp -d` directory;
                - tools/gate.sh, as its own header says (Item 7);
                - read-only shell tools on the repository and on your own scratch: cat, head, tail,
                  sed -n, grep, awk, cut, tr, sort, uniq, wc, diff, cmp, ls, stat, find, date, file,
                  sha256sum;
                - mkdir -p, cp and rm -r, only inside `mktemp -d` directories.
              Files in the repository are edited only with the Edit and Write tools, never by a script or
              a shell redirection (standing since round 009). On this machine `grep` is ugrep: a bounded
              repetition of 100 or more, such as `.\{0,120\}`, fails with "exceeds complexity limits";
              keep bounds below 100.
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive
              widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 011 — The roadmap, the backlog and arcs; ADVISOR r4 and ROUNDS r4

The human has ruled how the project keeps its plan: a roadmap whose rows each carry a TRIGGER and an
arc, a backlog of possibilities kept apart from debt and forks, and a check at every landing that a
round's arc agrees in three places. The human has also ruled that the advisor runs the merge and push
commands on the human's word, and that the advisor's starter is a file rewritten at every handoff. None
of this is in the tree yet. This round writes it all down, records the first client as a website, files
four DEBT entries the advisor found, and records two latitude rulings. It is a records round: it touches
no code, no golden data and no gated path, and it decides no fork.

THE HUMAN'S RULINGS this round carries. Each is quoted in the report beside the text that implements it.
  R1. The roadmap (2026-10-02, "option a"; 2026-11-12, "Yes, add ROADMAP.md to the next prompt", and
      "Agree with all recommendations" for its shape). docs/ROADMAP.md holds two tables. Table 1, the
      plan: one row per round, landed or planned, with the columns arc | milestone | round | achieves |
      opens | TRIGGER. The milestone cell names a milestone of 001 §10, or says none. The plan names no
      spec numbers, no dates and no model. Table 2, the gates: 003 §4 P4's two gates, each with its
      state. A status cell is replaced when the state changes, never appended to; the old state goes
      into a one-line entry under the file's Revisions. 001 §10 stays authoritative for what each
      milestone delivers: the roadmap tracks order and state only.
  R2. The backlog (2026-11-12, "Agree with all recommendations"). docs/BACKLOG.md is a list of
      possibilities, not a queue of commitments. DEBT.md is what is owed, FORKS.md is a decision open,
      BACKLOG.md is a possibility; one item is filed in only one of them. Each entry has four fields and
      no more: a short name; where it came from (the date, and whether from an advisor conversation or
      a spec's aside); what it would buy, in one or two sentences; its state. States: idea → sized →
      promoted (naming the spec) | stale (its evidence aged or was superseded, with the reason) |
      dropped (decided against, with the reason). Stale and dropped entries are kept, never deleted.
      Each entry carries the date its evidence was gathered, and promoting a stale entry requires
      re-verifying its facts first.
  R3. Following the roadmap (2026-11-12, "Agree with all recommendations"). At each landing three
      things agree: the prompt's arc, the arc in the spec's head block, and the arc of the round's row
      in docs/ROADMAP.md. A mismatch is a finding. The advisor checks the roadmap, the fork register and
      the backlog for drift at each landing. A round that is not on the roadmap gets its row, agreed
      with the human, before its prompt is written.
  R4. Arcs (2026-11-12, "Agree with R4 (a)"). The prompt carries an `ARC:` line and the head block an
      `arc:` line. The advisor assigns the arc with the spec number; the human may overrule it. A round
      that landed before this one gets its arc only in its roadmap row; no landed spec file is edited.
      The arcs, each with what it covers:
        Engine: crates/tshes, the computation systems (milestones M1, M2)
        Validation: fixtures, oracles and differentials
        Sources: Buryat calendars: the library, admission, golden data (M0)
        Verdict: the Q1 verdict (M3)
        Rules: the L2 rules (M4)
        Clients: the website and Android (M5, M6)
        Outreach: datsans, a зурхайч, the Buryat strings (M7, F4, F5)
        Process: method, records and publication (no milestone)
      The advisor's own caveat, RELAYED: the weakest boundary is Engine against Validation.
  R5. The advisor runs the human's git commands (2026-11-12, "Agree with option 2"). The advisor runs
      the docs/ROUNDS.md §9 merge (the squash, and the commit with the round's extracted message), the
      pushes (main to the public origin, branches to the private mirror), the branch clean-up and, once,
      the first public push. It runs each set only after the human's explicit word in chat for THAT
      set, and shows the exact commands first. A force-push to the public origin is asked a second time,
      separately, right before it runs. Never while an engine session is running in the engine
      repository, because the merge checks out main there. Not covered: writing file content;
      committing anything a round did not write; reading code (docs/ADVISOR.md §3 is unchanged). The
      backstop: a permissions `ask` list in the advisor clone's `.claude/settings.local.json`, which git
      ignores, covering the git commands that write (add, commit, merge, push, reset, branch, checkout,
      switch, restore, tag, update-ref, rebase, cherry-pick, filter-branch, stash, rm) on the engine
      repository, and reset and checkout on the clone. The human placed it on 2026-11-12, and a push
      --dry-run showed its prompt (RELAYED).
  R6. The advisor's starter is a file (2026-11-11, "agree"). At EVERY handoff, whether the advisor's
      session should be cleared or not, the advisor overwrites /home/kitaeza/zurkhai-prompts/advisor-
      starter.txt with the docs/ADVISOR.md §8 starter, composed from files, and shows it with `cat`.
      The human's shell has an alias, zadv-start, that prints that file; a fresh advisor session starts
      with it. The starter ends with a stale-check paragraph: main's full commit id, what the public
      origin's main should print, and the sha256 of every running or prompted prompt file. The fresh
      session checks those first; any mismatch means the starter is stale, and the session says so and
      rebuilds the state from the files and the advisor's notes. METHOD rule 9 is unchanged: the clear
      line and the starter are still one message.
  R7. The first client is a website (2026-10-02; the human's words were not recorded). 001 §10's M5
      reads "web or TG Mini App". The human chose the website, and on 2026-10-02 registered the domain
      zurkhai.org for it (RELAYED). Nothing was served there on 2026-10-02 (RELAYED; not re-checked
      since).
  R8. Round 010's latitude (2026-11-12, "Agree with all recommendations, go merge"): its Latitude 1–5,
      and the read-only or process commands it ran outside its GRANTS (kill, pkill, pgrep, timeout,
      sleep, git remote -v, git merge-base), are accepted as reported; no grant after the fact.
  R9. Round 009 session 3's latitude (2026-11-12, "approve"): its Latitude 1–6, and its unlisted
      rewording of the head block's model line ("either:" became "sessions 1 and 2:"), are accepted as
      reported; no grant after the fact.

ITEM 0 — Branch, prompt, number, records.
  - Check PRECONDITION, and quote every output.
  - Create spec/011-roadmap-backlog from main at 0d1d2db. The head block's branch line names that commit
    and its tree. The head block carries `arc: Process` on the line after `prompt:`, the first head
    block to carry one (Item 4(d) adds the line to docs/ROUNDS.md §4.1).
  - Commit this file, byte-identical, as docs/specs/prompts/011-roadmap-backlog.md. That is the first
    commit. `cmp` it against /home/kitaeza/zurkhai-prompts/spec-011-roadmap-backlog.md before every
    later commit.
  - Verify that 011 is free in the tree.
  - Read the Read-first list, and list it in the report.
  - Run `date` once and use that value for every date you write (METHOD rule 51). If a later `date`
    shows a new day, the records written after it carry the new date, and the report says where.
  - Choose your round's marker (METHOD rule 58). The report calls it "the round's marker" and never
    spells it.
  - Re-measure the OPEN count in FORKS.md with the grep its header defines. The advisor counted 5 on
    main at 0d1d2db (F4, F5, F7, F8, F10; RELAYED). Re-measure it, do not copy it.
  - FORKS.md, one new row for R7, numbered from the next free F id (derive it from the file; the
    advisor expects F27, RELAYED). It is filed already RULED. Its fields, one per line as given:
      subject: The first client
      Raised: 2026-10-02, by the advisor: 001 §10's M5 reads "web or TG Mini App"; recorded by spec 011
        Item 0
      Question: Which client does M5 build first?
      Options: (a) a website; (b) a Telegram Mini App.
      Status: RULED
      Ruled: 2026-10-02, (a), a website. The human registered the domain zurkhai.org for it the same
        day. The human's words were not recorded.
      Built in: not yet built; docs/ROADMAP.md's website row (spec 011)
  - R8 and R9 are recorded in the report's rulings section, and nowhere else: they are not forks.
  - The open-forks audit: the count expected at your final commit is the count you measured at the base;
    the new row is RULED and adds nothing. Show the arithmetic at every commit.

ITEM 1 — docs/ROADMAP.md (the decisive item, with Item 4's arc check).
  (a) Write it with the Write tool. Its opening, in your words and short: what the file is; that 001 §10
      is authoritative for what a milestone delivers and the roadmap tracks only order and state (R1);
      why it names no spec numbers, dates or model; that every row carries a TRIGGER, a checkable
      condition and not an intention; that a status is replaced, never appended to, and the old state
      goes to Revisions; the arcs of R4, each with what it covers, and the boundary caveat; how a new
      row enters (R3: agreed with the human before the round's prompt).
  (b) Table 1 — the plan. The round cell is `<short name> — <STATUS>`, where the short name is the
      round's branch name after its spec number, and STATUS is LANDED or PLANNED. A round's own final
      commit sets its row to LANDED (Item 4(g) writes that duty into CLAUDE.md); a halt does not. The
      rows, in this order, one field per line as given. Copy every field exactly:
      row: 1
        arc: Engine
        milestone: M1
        round: workspace-gate — LANDED
        achieves: The workspace and the gate
        opens: every later round's gate (002 §7)
        TRIGGER: —
      row: 2
        arc: Engine
        milestone: M1
        round: phugpa-core — LANDED
        achieves: The Phugpa core
        opens: the reference fixtures
        TRIGGER: —
      row: 3
        arc: Validation
        milestone: M1
        round: reference-fixtures — LANDED
        achieves: The reference fixtures: T5 tables from Janson v1, the T4 oracle, P5 and P6
        opens: Phugpa checked on every day of 1900–2100; the other systems checkable against Janson's tables
        TRIGGER: —
      row: 4
        arc: Engine
        milestone: M1
        round: to-verify-attributes — LANDED
        achieves: The TO-VERIFY attributes of 001 §5.2: what Janson v1 defines; rows 6–8 deferred (D-18)
        opens: M1's last condition: the §5.2 rows resolved or explicitly deferred
        TRIGGER: —
      row: 5
        arc: Engine
        milestone: M2
        round: genden-tsurphu — LANDED
        achieves: Genden and Tsurphu: the two other candidate systems, Gregorian ↔ ZDate from Janson v1; P5 for both
        opens: all three candidate systems, for the Q1 differential
        TRIGGER: —
      row: 6
        arc: Validation
        milestone: none
        round: henning-differential — LANDED
        achieves: The Henning differential: TCG 1.06 against the engine's three systems, 1900–2100
        opens: D-23 and D-24
        TRIGGER: —
      row: 7
        arc: Process
        milestone: none
        round: publication — LANDED
        achieves: Before the first public push: the CI pin, the history audit and map, the private mirror
        opens: the public repository, and CI on hosted runners
        TRIGGER: —
      row: 8
        arc: Process
        milestone: none
        round: roadmap-backlog — PLANNED
        achieves: The roadmap, the backlog and arcs; ADVISOR r4 and ROUNDS r4; the first client recorded as a website
        opens: an arc on every later prompt; the Sources round
        TRIGGER: publication landed
      row: 9
        arc: Sources
        milestone: M0
        round: buryat-sources — PLANNED
        achieves: the admission of the Buryat datsan calendars in the paper library, and their tshes as golden rows (001 §10 M0)
        opens: the Q1 verdict
        TRIGGER: roadmap-backlog landed, and the human has answered the questions on admission and tier that the round's prompt needs
      row: 10
        arc: Verdict
        milestone: M3
        round: q1-verdict — PLANNED
        achieves: the Q1 differential report, and the verdict spec or a recorded stop-and-consult (001 §10 M3)
        opens: D-1 is lifted only by the verdict spec; the TRIGGERs of D-19, D-23 and D-24 name this round
        TRIGGER: M0's done-when is met (001 §10), and genden-tsurphu landed
      row: 11
        arc: Rules
        milestone: M4
        round: l2-schema — PLANNED
        achieves: the L2 schema, its validator and the first sourced table (001 §10 M4)
        opens: sourced rules for the first client
        TRIGGER: the human orders it; 001 §10 sets it no dependency, and it is not yet ordered
      row: 12
        arc: Clients
        milestone: M5
        round: website — PLANNED
        achieves: the first client, a website at zurkhai.org (F27) (001 §10 M5)
        opens: Android; the second gate of 003 §4 P4
        TRIGGER: the Q1 verdict spec exists (D-1), and l2-schema landed
      row: 13
        arc: Clients
        milestone: M6
        round: android — PLANNED
        achieves: Android, on the same engine through bindings (001 §10 M6)
        opens: —
        TRIGGER: website landed, and F7 is ruled
      row: 14
        arc: Outreach
        milestone: M7
        round: zurkhaich-review — PLANNED
        achieves: a review by a зурхайч: written feedback recorded, an endorsement if given (001 §10 M7)
        opens: —
        TRIGGER: F4 is ruled
      If Item 0 filed the row under an id other than F27, row 12 carries that id instead, and the
      report says so. Row 8 is this round's own: Item 7 sets it to LANDED in the final commit, with a
      Revisions line.
  (c) Table 2 — the gates. Columns: gate | what it checks | state | TRIGGER. Two rows, one field per line:
      row: 1
        gate: Gate 1, the Q1 verdict (003 §4 P4; M3)
        what it checks: which computation system the Buryat calendars follow
        state: NOT REACHED: M0 has no admitted source yet; D-1 holds
        TRIGGER: the Q1 verdict spec is accepted
      row: 2
        gate: Gate 2, the first public release of a client (003 §4 P4)
        what it checks: correctness, provenance display, tradition labelling, and a reviewer's sign-off
        state: NOT REACHED: no client exists
        TRIGGER: the website's first release candidate
      Before you write them, check both rows' "what it checks" against 003 §4's P4 row and 001 §4, and
      report any difference. Do not change a row to fit: a difference is a STOP AND ASK (STOP RULE (2)).
  (d) Revisions: r1, with your date: initial, on the human's rulings R1, R3, R4 and R7 (spec 011).
  (e) The check. A `python3 -I` script reads docs/ROADMAP.md and checks: every Table 1 row has six
      cells and every Table 2 row four; every arc cell is one of R4's eight; every milestone cell is
      M0–M7 or none; every round cell ends in — LANDED or — PLANNED; no TRIGGER cell is empty; Table 1
      holds no date (yyyy-mm-dd) and no round's spec number (a standalone three-digit number from 004
      up; 001, 002 and 003 are the charter's specs and may be cited). Then it compares every cell with
      the fields dictated in (b) and (c), which the script reads from this prompt file in the repository
      (docs/specs/prompts/011-roadmap-backlog.md), not from a copy you typed. The control comes first (METHOD rules 31, 36): on a scratch copy with one cell changed in
      one row, and a second scratch copy with one arc not in R4's list, the script must report exactly
      that cell and that arc, and nothing else. Report what was seeded, what was expected and what was
      observed. Run it again at the final commit, after row 8 becomes LANDED (the only expected
      difference from the dictated fields; the script is told so, and says so).

ITEM 2 — docs/BACKLOG.md.
  (a) Write it with the Write tool. Its opening, in your words and short: R2's distinction (a
      possibility, not a plan; DEBT owed, FORKS open, BACKLOG possible; one item in only one of them),
      the four fields, the states, that stale and dropped entries are kept, the evidence date, and
      re-verifying before promoting. Entries are numbered B-n, never reused.
  (b) The seeds. Before filing each, search DEBT.md and FORKS.md for it (name the greps). A seed that an
      entry or row already covers is not filed (STOP RULE (3)); the report names the id. Each seed's
      origin is "2026-11-11, advisor conversation" unless given otherwise; its evidence date is the
      date given; its state is idea. One field per line:
      seed: 1
        short name: a Telegram Mini App client
        origin: 2026-10-02, advisor conversation; 001 §10's M5 reads "web or TG Mini App"
        would buy: a second client, inside Telegram, on the same engine through wasm-bindgen (F1's option (a) names the Telegram Mini App)
        evidence date: 2026-10-02
      seed: 2
        short name: real-world Phugpa controls
        would buy: published calendars that follow Phugpa — the Kalmyk Central Khurul's printed calendars (2023–2026) and Rabten Choeling's 2026 calendar, both in the paper library — as a positive control: the Q1 comparison should find them Phugpa, which shows it can tell Phugpa from the system the Buryat datsans follow. Their role and tier are undecided.
        evidence date: 2026-11-11 (the advisor's reading of three months and a delegated reader's of nine: the Khurul's 2025 calendar equals Phugpa on all 365 days; RELAYED)
      seed: 3
        short name: approaching the other datsans
        would buy: beyond F4's one datsan: asking the datsans whose calendars the library holds for older editions, for written permission to publish their calendars' facts (001 §8), and for how their calendar is computed
        evidence date: 2026-11-11
      seed: 4
        short name: Henning's published Tsurphu lists as the variant's oracle
        would buy: an oracle, already in the paper library (henning-kalacakra), for the karana-Sun Tsurphu variant of D-19, if that variant is ever built
        evidence date: 2026-11-11 (his 2012 list differs from Janson v1's Table 8 in months 4, 5 and 11; RELAYED, as D-19 says)
      Seed 4 is the one most likely to be covered: D-19 is the variant's debt. File it only if D-19
      does not already name those lists as its oracle; the report gives your reading of D-19 either way.
      Copy every given field exactly; "would buy" may be shortened only to fit two sentences, and the
      report then shows both texts.
  (c) The check, as Item 1(e): every entry has exactly the four fields and an evidence date, and every
      state is one of R2's five; and every filed entry's fields equal the dictated ones. The control
      comes first: a scratch copy with one state outside R2's list, and one with a field missing, must
      each be reported, exactly.

ITEM 3 — docs/ADVISOR.md, revision r4.
  The round words it, in the file's own style. The advisor reads the wording at the landing. Each change
  below names the ruling it implements; the report pairs each ruling's sentence with yours (METHOD rule
  61).
  (a) §1, "It never": the advisor writes no file content to either clone and commits nothing of its
      own; it runs the docs/ROUNDS.md §9 commands on the human's word, as R5 says, in full: the sets, the
      word per set, the commands shown first, the second ask before a force-push to the public origin,
      never while an engine session runs, what is not covered, and the backstop.
  (b) §2: "It commits nothing" agrees with (a): the only commit the advisor makes is a round's squash,
      in the engine repository. After it, the clone's `pull --ff-only`, as now.
  (c) §5: a round's prompt carries an `ARC:` line; the advisor assigns the arc with the spec number, and
      the human may overrule it (R4). A round not on the roadmap gets its row, agreed with the human,
      before its prompt is written (R3).
  (d) §6: a new check in step 3, the arc agrees three ways (R3): the prompt's `ARC:` line, the head
      block's `arc:` line, and the round's row in docs/ROADMAP.md, which its final commit set to LANDED.
      A mismatch is a finding. A new last step after the merge, the drift check (R3): the advisor reads
      docs/ROADMAP.md, FORKS.md and docs/BACKLOG.md and names, in the adjudication, every planned row
      whose TRIGGER has now fired, every gate whose state has changed, and every backlog entry the round
      promoted. Step 5's "the human's merge commands" becomes the commands the advisor runs on the
      human's word (R5).
  (e) §8 (R6): the starter file, its path, its overwrite at every handoff, the `cat`, the alias, and the
      stale-check paragraph with what it holds and what a mismatch means. Update the starter's template
      and the line that follows it: the human opens a fresh session in the clone, on Opus, effort xhigh,
      and runs `! zadv-start`.
  (f) §8, the mechanical check: as written it forbids the second last-line form ("No need to clear the
      advisor session: …"), which holds the same words without a starter block. Reword the check so
      that the words appear together only in a message with the starter block, or in exactly that
      second form. This restores §8's consistency with itself (CLAUDE.md hard rule 19, within §8).
  (g) The Status line: r4, with your date. A revision entry: r4 (<your date>): §1, §2, §5, §6 and §8;
      on the human's rulings R3–R6 (spec 011).

ITEM 4 — docs/ROUNDS.md, revision r4, and two lines of CLAUDE.md.
  The round words it. Each change names its ruling.
  (a) §1: the loop's last line, and the bullets on the human and the advisor: the human gives the word
      for each merge and push; the advisor runs the commands (R5). The engine session still never
      pushes or merges.
  (b) §2.2: an `ARC:` line in the header, after `SPEC NUMBER:` (R4). The PRECONDITION template's
      "the human merges" becomes "the merge is §9's". After the template, one sentence on ugrep, as
      this prompt's GRANTS says it.
  (c) §3, the gate: the gate runs on the last commit that changes any path other than the round's own
      spec file, its INDEX.md row and its docs/ROADMAP.md row; the final commit changes only those
      three, which no gate leg reads (check this against tools/gate.sh and say what you found); the
      report's gate line names the commit the gate ran on; and `git diff --name-only <that
      commit> <the tip>` shows it. This is how round 010 closed (RECALLED, its head block's gate line).
      The advisor's own reason, RELAYED: a report cannot hold a gate run on the commit that holds the
      report.
  (d) §4.1: an `arc:` line in the head block, after `prompt:` (R4).
  (e) §9: its title and opening say that the human gives the word for each set and the advisor runs
      the commands (R5), with the second ask before a force-push to the public origin. In steps 4 and
      5, "the human" becomes whoever runs the step, as R5 says. Change nothing else in §9.
  (f) The Status line: r4, with your date. A revision entry: r4 (<your date>): §1, §2.2, §3, §4.1 and
      §9; on the human's rulings R4 and R5 (spec 011), and the gate-on-the-tip practice.
  (g) CLAUDE.md, two edits and no others: hard rule 2's "The human merges." becomes "The merge is the
      human's decision (`docs/ROUNDS.md` §9)."; and "Files a round keeps current" gains
      `docs/ROADMAP.md` (your own row set to LANDED, in your final commit) and `docs/BACKLOG.md` (an
      entry the prompt tells you to promote, in the same commit).
  (h) The grep: search docs/, CLAUDE.md and the root files for "the human merges", "human merges",
      "squash-merges", "pushes main", "pushes or merges", "commits nothing" and "MILESTONE". Hits in
      committed prompts (docs/specs/prompts/) and in round reports (docs/specs/004-… and later) are
      records: count them per file, and list none. A hit beside your own edit that now contradicts it may be fixed (CLAUDE.md hard rule 19), and the report
      says so. Every other hit is listed as a flag and not fixed; a hit in a gated file goes into Item
      5(d)'s entry. FORKS.md F11's Ruled line ("the human pushes the branch there") is a record of a
      ruling: list it, do not edit it.

ITEM 5 — Four DEBT.md entries, numbered from the next free D id (derive it; the advisor expects D-25,
RELAYED). Fields one per line, in the register's shape; Status OPEN; Pinned by: not pinned.
  (a) TOOLING — actions/checkout v4 runs on a deprecated Node.js 20.
      Statement: the first hosted CI run on main (run 43107437324, 2026-11-12) carries an annotation
      that Node.js 20 is deprecated and that actions/checkout at the pinned commit was forced to run on
      Node.js 24. v4 targets Node.js 20; spec 010 pinned v4.4.0 (D-8, PAID) and found v5, v6 and v7
      published (RECALLED, its by-catch). The run also carries an annotation that the ubuntu-latest
      label will migrate to a newer Ubuntu; the gate pins its Rust toolchain (rust-toolchain.toml).
      Fetch the run with GRANTS' one command and quote the two annotations' lines (FETCHED). If the
      fetch fails, write the statement from this RELAYED text, and say so.
      TRIGGER: the first of: GitHub announces a date on which Node.js 20 is removed from its hosted
      runners; a CI run fails, or warns that it will fail, because of it; the next round that edits
      .github/workflows/.
  (b) METHOD — rule 51 does not say what a round crossing midnight, or resumed on a later day, writes.
      Statement: rule 51 says to run `date` once, at the top. Round 006's first session dated its
      records 2026-11-10 and committed them on 2026-11-11 (RELAYED, the advisor). Prompts since spec 010
      carry an interim clause in Item 0; quote it from this prompt. METHOD is gated: it changes only by
      an amendment spec.
      TRIGGER: the next amendment spec to docs/METHOD.md.
  (c) DOCS — the INDEX status of a merged round is never changed.
      Statement: every round row in docs/specs/INDEX.md (re-measure which) says draft after its merge;
      no step of docs/ROUNDS.md §9 changes it, and nothing defines what status a merged, report-shaped
      round's file has (TEMPLATE.md offers draft | accepted | superseded by NNN). docs/ROADMAP.md's
      LANDED now records a round's state. INDEX.md rows other than a round's own are gated.
      TRIGGER: the next spec that revises 003 or docs/specs/TEMPLATE.md, or the first record that reads
      an INDEX status as a round's state.
  (d) DOCS — gated texts that lag the rulings of 2026-11-12.
      Statement: (i) 003 §11's list of repository files names neither docs/ROADMAP.md nor
      docs/BACKLOG.md; (ii) docs/specs/TEMPLATE.md's head block has no `arc:` line; (iii) 003 says the
      human merges and pushes (give its lines, re-measured: the advisor found lines 23, 30 and 174,
      RELAYED), where under R5 the decision is the human's and the commands the advisor's; plus any
      gated hit of Item 4(h). Each is gated, so this round does not change it.
      TRIGGER: the next accepted spec that revises 003 or docs/specs/TEMPLATE.md.
  If one of these is already covered by an existing entry, do not file it; name the id.

ITEM 6 — The consistency read.
  Read docs/ROADMAP.md, docs/BACKLOG.md, docs/ADVISOR.md, docs/ROUNDS.md and CLAUDE.md once more as
  committed, start to end. List every sentence that contradicts another sentence in these five files,
  or a ruling of this prompt. Fix the ones beside your own edits (CLAUDE.md hard rule 19) and say so;
  list the rest as flags.

ITEM 7 — The close.
  - Run tools/gate.sh once, with nothing in flight, in both profiles (CLAUDE.md hard rule 12; METHOD
    rule 65), on the commit that Item 4(c) describes. Compare its count with the last count recorded on
    main, naming the spec it comes from (RECALLED).
  - In the final commit: set docs/ROADMAP.md row 8 to LANDED, with its Revisions line; add the round's
    row to docs/specs/INDEX.md: 011, `011-roadmap-backlog.md`, "The roadmap, the backlog and arcs;
    ADVISOR r4 and ROUNDS r4", draft; finish the report. Run Item 1(e)'s check again on that commit.
    Show that `git diff --name-only <the gated commit> <the tip>` lists only
    docs/specs/011-roadmap-backlog.md, docs/specs/INDEX.md and docs/ROADMAP.md.
  - Write the squash commit message. After the final commit the branch stands still. No reviewer runs
    at this close: the round changes neither crates/ nor golden/ (docs/ADVISOR.md §6).

PREDICTIONS — written by the advisor on 2026-11-12, before this round measures anything (METHOD rule 3).
Their basis: the advisor read, at 0d1d2db, FORKS.md, DEBT.md's headings, INDEX.md, docs/ROUNDS.md,
docs/ADVISOR.md, CLAUDE.md, GATED.md, 001 §10, 003 §7 and §11, TEMPLATE.md and the 010 report's head
block, and grepped those files for the merge and push wording. It did not read crates/ or clients/, and
did not fetch the CI run. All of this is RELAYED, and none of it is a premise. Score each prediction
HELD or FAILED, with the measured value. A prediction scored HELD cites your own measurement, never this
prompt.
  1. Both PRECONDITION checks pass.
  2. OPEN is 5 at the base and 5 at the final commit; the new row is F27.
  3. The new DEBT entries are D-25, D-26, D-27 and D-28; none is covered by an existing entry.
  4. Seeds 1, 2 and 3 are filed, as B-1, B-2 and B-3; seed 4 is filed as B-4, because D-19 names
     Henning's calendars as evidence but not as the variant's oracle.
  5. Every control of Items 1 and 2 reports exactly what was seeded.
  6. Table 2's rows agree with 003 §4's P4 row.
  7. Outside the records, Item 4(h)'s grep hits CLAUDE.md line 12, docs/ADVISOR.md lines 11 and 31 and
     docs/ROUNDS.md lines 18 and 63, all of which this round edits, and 003 lines 23, 30, 163 and 174;
     of these, line 163 (the engine session never pushes or merges) does not contradict R5. No hit for
     "MILESTONE".
  8. tools/gate.sh reads no file under docs/.
  9. The gate's count equals the last count recorded on main.
  10. The diff touches 10 files: CLAUDE.md, DEBT.md, FORKS.md, docs/ADVISOR.md, docs/BACKLOG.md,
      docs/ROADMAP.md, docs/ROUNDS.md, docs/specs/011-roadmap-backlog.md,
      docs/specs/prompts/011-roadmap-backlog.md, docs/specs/INDEX.md.

THE REPORT — per CLAUDE.md's report contract and docs/ROUNDS.md §4, in docs/specs/011-roadmap-backlog.md.
It holds:
  - the head block, with its `arc:` line and without a `golden:` line;
  - a report-shaped spec (docs/specs/TEMPLATE.md's header lines: Status: draft; Resolves: the new row;
    Milestone: none; Governs: docs/ROADMAP.md, docs/BACKLOG.md, docs/ADVISOR.md, docs/ROUNDS.md), then
    the end marker;
  - then the report.
  Lead with the decisive item: docs/ROADMAP.md, its check and its control, and the arc agreeing three
  ways for this round itself (this prompt's ARC:, the head block's arc:, row 8). Then:
    - docs/BACKLOG.md: each seed, filed or covered, with the greps that decided it;
    - each of R1–R7 beside the sentences that implement it, and R8 and R9 recorded;
    - the four DEBT entries by id, and the fetch of Item 5(a);
    - the consistency read's list, and Item 4(h)'s hits, each fixed or flagged;
    - the predictions, scored;
    - the open-forks audit, with its arithmetic at every commit;
    - the marker greps, at every commit;
    - every other section of the contract, "What the prompt got wrong" included;
    - the diff scope, with its totals summing against `git diff --stat main`, stating the commit it was
      measured at;
    - every STOP AND ASK, if any;
    - last: the commands and the scripts, verbatim.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 011 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════
