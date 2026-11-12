═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 012 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  012 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
ARC:          Sources
BRANCH:       spec/012-source-checks, cut from main at c4af28d before any edit; commit to it.
              No merge. No push, to any remote.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7, first branch. Every answer is discoverable:
              what a publisher's own pages say about itself, what a public register says, what a site's
              terms say, and what a statute's text says. No answer is invented: the round draws no legal
              conclusion and decides no admission (STOP RULE (4)). Two parts would be silent, and each
              runs only with an instrument that makes it loud (docs/ROUNDS.md §8):
                - a quotation. WebFetch returns a model's rendering of a page, not the page, and a
                  misquoted term or article reads like a right one. Instrument: a control on a text the
                  library already holds (Item 1), and every quotation a later ruling may rest on goes
                  into the link table for the human to save (Item 6);
                - a membership verdict from weak evidence. Instrument: each verdict lists the evidence
                  for and against, each item tagged, and the words MEMBER or NOT A MEMBER are written
                  only on a primary source (Item 2(c)); otherwise UNDETERMINED.
              No change from the last round: round 011 also ran on Sonnet 5.5, effort High.
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first. It never commits a previous session's uncommitted work without
              verifying it: it finishes the work and proves it by the item's own check, or redoes it. A
              design choice that would fall to the fallback model (Opus 5.5, effort High) is a STOP AND
              ASK. The report names the switch point.
PRECONDITION: Check each, and quote every output:
              (a) `git -C /home/kitaeza/github/zurkhai status --porcelain` prints nothing, and
                  `git -C /home/kitaeza/github/zurkhai rev-parse --abbrev-ref HEAD` prints main.
              (b) `git -C /home/kitaeza/github/zurkhai rev-parse main` prints
                  c4af28d0214bb035b2d5252d9570234544254174, spec 011's squash.
              (c) `ls /home/kitaeza/zurkhai-papers/` succeeds (the human has added the directory with
                  /add-dir).
              Anything else: STOP and report, with zero diff.
              Read first, in this order, each in full unless a part is named: CLAUDE.md;
              docs/ROUNDS.md; docs/METHOD.md rules 1, 3, 31, 35, 36, 50, 51, 56, 58, 60, 66 and 67, each
              in full, not only its **Rule.** paragraph; GATED.md; FORKS.md (its header; F2, F4, F10 and
              F22 in full; the rest by their status lines); DEBT.md (its header; D-29 in full; the rest
              by their headings); docs/sources.md (in full); docs/ROADMAP.md; docs/BACKLOG.md;
              docs/specs/001-init-spec.md (§1, §6 and §8); docs/specs/003-project-method.md (§4, the P3
              row); docs/specs/011-roadmap-backlog.md (§5.1, the flags F-a to F-d); docs/specs/INDEX.md;
              docs/specs/TEMPLATE.md. The report lists each file it read and how much of it.
ARTIFACT:     (A) The paper library, /home/kitaeza/zurkhai-papers/, read only: the seven holdings
              datsan-rinpoche-bagsha/, ivolginsky-datsan/, aginsky-datsan/, gunzechoinei-datsan/,
              khoymorsky-datsan/, kalmyk-central-khurul/ and rabten-choeling/. In particular:
                - datsan-rinpoche-bagsha/datsan-about/o-datsane.txt, line 137 (Item 1's control text);
                - rabten-choeling/calendarEN.pdf, sha256
                  13aa4256c82eec473ad0574e011313dde8d8931460b228178f4ba9d761f92898 (RELAYED, the
                  advisor, 2026-11-12).
              If a named file is absent, or its sha256 differs: STOP and report.
              (B) The web, through WebSearch and WebFetch only (GRANTS).
STOP RULE:    (1) A PRECONDITION or ARTIFACT check fails: STOP, zero diff.
              (2) Item 1's control fails: continue, but every quotation obtained through WebFetch is
              then marked "wording not verified (Item 1)", and its page goes into the link table. Not a
              stop.
              (3) A finding bears on a ruling already made (F22, or R2–R5 below): do not change the
              ruling or its row. Report the finding beside the ruling; the human rules at the landing.
              (4) No legal conclusion. Never write that a reuse is permitted, allowed, lawful,
              forbidden or unlawful, in your own voice. Quote the terms and the statute, and tag any
              reading of your own INFERRED. Admission, tier and any permission are the human's.
              (5) No contact with anyone: no email, no form, no message, no account, no purchase.
              Approaching a datsan is F4's and BACKLOG.md B-3's.
              (6) Out of scope; do not write any of it:
                - any change under crates/, golden/, rules/, clients/, tools/ or .github/;
                  docs/METHOD.md, GATED.md, CLAUDE.md, docs/ADVISOR.md, docs/ROUNDS.md,
                  docs/BACKLOG.md, an accepted spec, or another round's row in docs/specs/INDEX.md;
                - any transcription of a calendar's days; any golden row;
                - anything written into the paper library, or into git, that is a third party's text
                  beyond a quotation of at most two sentences (CLAUDE.md hard rule 17);
                - any push, fetch or pull; deleting a branch; creating a tag;
                - deciding any fork, or writing a FORKS.md row other than Item 0's.
              If something seems to need one of these, write it down for the report (by-catch, and a
              DEBT.md entry where it qualifies), and do not do it.
              (7) A read, a run or a network call outside GRANTS is needed, or a permission is denied:
              STOP and report each, with the reason. Never work around it by another tool or route
              (METHOD rule 67).
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; there is no fourth
              kind. An unobtainable source says PAYWALLED or UNREACHABLE. Read docs/sources.md before any
              fetch. Never curl/wget. (METHOD rule 50.) One attempt per host for a page that refuses: no
              second or third try at a host that refused (rule 50, item 2). A search result's snippet,
              not opened, is SNIPPET-VERIFIED. Every FETCHED citation gives its URL and the date.
GATED:        no gated path, except this round's own docs/specs/012-source-checks.md, its
              docs/specs/prompts/012-source-checks.md, and its row in docs/specs/INDEX.md, which are not
              changes to an accepted spec. Not gated, and changed only as the items say: DEBT.md,
              FORKS.md, docs/ROADMAP.md, docs/sources.md.
GRANTS:       Reads outside WORKDIR, beyond this prompt file: /home/kitaeza/zurkhai-papers/, read only,
              with the Read tool, ls, stat, sha256sum, grep, pdfinfo and pdftotext (pdftotext writes
              only into a `mktemp -d` directory).
              Network, read-only: the WebSearch and WebFetch tools, for the questions of Items 2–5. No
              other network: no curl, wget, git fetch, gh or python networking; nothing that submits a
              form, logs in or passes a captcha.
              The tools, in the repository:
                - the Read tool;
                - git, read-only (status, log, show, rev-parse, rev-list, grep, diff), and what the
                  round's own branch and commits need (switch -c, add, commit);
                - `python3 -I`, running only scripts you write into a `mktemp -d` directory;
                - tools/gate.sh, as its own header says (Item 7);
                - read-only shell tools on the repository and on your own scratch: cat, head, tail,
                  sed -n, grep, awk, cut, tr, sort, uniq, wc, diff, cmp, ls, stat, find, date, file,
                  sha256sum;
                - mkdir -p, cp and rm -r, only inside `mktemp -d` directories.
              Files in the repository are edited only with the Edit and Write tools, never by a script or
              a shell redirection. On this machine `grep` is ugrep: keep bounded repetitions below 100.
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive
              widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 012 — Source checks: who publishes each calendar in the library, and on what terms

The paper library holds seven published calendars: five of Buryat datsans, one of the Central Khurul of
Kalmykia and one of Rabten Choeling in Switzerland. None is admitted. Before any of their days become
golden rows, the human needs to know, for each, who publishes it, whether the publisher is a datsan of
the Buddhist Traditional Sangha of Russia (001 §1), what the publisher's terms say about reuse, and what
the governing law says about reusing a calendar's facts (001 §8). This round finds those things out on
the web and records them; it also files the human's rulings of 2026-11-12 on the coming Sources round.
It transcribes nothing, admits nothing, decides no tier and draws no legal conclusion.

THE HUMAN'S RULINGS this round carries (2026-11-12). Each is quoted in the report beside the record that
implements it.
  R1. This round (the human: "lets make web search round to find out what is jurisdiction"; then, on the
      advisor's three clarifying questions, "Agree with all recommendations, go"). "Jurisdiction" covers
      both (a) religious affiliation: whether each publisher is a member of the Buddhist Traditional
      Sangha of Russia, and (b) the law: which law governs each publisher's terms, and what that law
      says about reusing a calendar's facts. The round quotes, and draws no legal conclusion. It covers
      all seven holdings. Its roadmap row is agreed (Item 0). «Ринпоче Багша» stays as FORKS.md F22
      rules it, a candidate only, until the human rules on this round's findings.
  R2. The Kalmyk Central Khurul and Rabten Choeling (the human: "Lets add them to source, like for
      Rinpoche Bagsha make a web search for it"; agreed as the advisor's option (a)). Each has a
      docs/sources.md library row (Rabten Choeling's is new; it pays DEBT.md D-29) and gets the same
      checks as the datsans. No golden rows: whether they become control data is decided at the Q1
      verdict round (docs/ROADMAP.md, q1-verdict; BACKLOG.md B-2).
  R3. 001 §6.3's admission clause, "a source is admitted only if it states it is zurkhai / Tibetan
      calendar and its numbers behave like tshes" (the human: "1 agree"). Its first half is met by a
      Sangha datsan's own Buddhist lunar calendar whose numbers behave like tshes, which the Sources
      round shows by a step check. 001 is gated, so a DOCS entry in DEBT.md asks for §6.3 to be
      clarified at its next revision.
  R4. The tiers of the two Sangha datsans' calendars (the human: "2 agree; 3 agree"). The Ivolginsky
      datsan's 2021 calendar is T1: the datsan's own print file, not a scan, which its meta file will
      say. The Aginsky datsan's 2025 and 2026 editions are both T1, one golden file per edition; their
      disagreement on 9–16 January 2026 is recorded in both files, and neither is corrected. Each
      admission still waits on the terms check of 001 §8, which this round performs.
  R5. The golden `is_leap_month` column (the human: "4 agree"). It records what the source prints. The
      meta file states the source's own convention for naming the extra month. Where a source prints no
      label, the cell is empty, with a note. The Q1 round maps one convention onto another.
  R6. Round 011's latitude (2026-11-12, "Agree with all recommendations, go merge"): its Latitude 1–7
      are accepted as reported; no grant after the fact. Recorded in the report only.
  R7. Round 011's flags (same words). One DOCS entry in DEBT.md, TRIGGER "the next Process round",
      holds: (i) F-a, CLAUDE.md hard rule 12 says the gate runs "on the final committed tree", where
      docs/ROUNDS.md §3 (r4) runs it on the last commit that changes more than the round's spec file,
      its INDEX.md row and its docs/ROADMAP.md row; (ii) F-b, docs/ROUNDS.md §4.1's `gate:` line has no
      place for the commit the gate ran on, which §3 requires it to name; (iii) F-c, CLAUDE.md's "Files
      a round keeps current" says a promoted backlog entry is updated "in the same commit" without
      saying as what; (iv) docs/ROUNDS.md §9's opening still says "After the advisor's go", left over
      from r3, where r4 makes the human's word the trigger.

THE RESEARCH LEGS — copied from 003 §4's P3 row, as written there:
  Legs: ① **reference**: datsan publications, and a зурхайч where reachable. ② **field**: other
  implementations (Lotsawa House, Henning's software, existing Mongolian apps); pin where their
  conventions differ. ③ **science**: Janson, Henning, Schuh, Reingold–Dershowitz, or an explicit "none
  relevant". ④ **Does this contradict anything already shipped?** Every leg is reported, including empty
  ones, with what it found against the approach.
  This round is leg ① for the seven holdings, and its law is part of ①'s question. Legs ② and ③ are
  reported, and are expected to be "none relevant" with the search that shows it; leg ④ asks whether
  any finding contradicts docs/sources.md, FORKS.md F22 or R2–R5.

ITEM 0 — Branch, prompt, number, records.
  - Check PRECONDITION and ARTIFACT, and quote every output.
  - Create spec/012-source-checks from main at c4af28d. The head block's branch line names that commit
    and its tree, and the head block carries `arc: Sources` after `prompt:`.
  - Commit this file, byte-identical, as docs/specs/prompts/012-source-checks.md. That is the first
    commit. `cmp` it against /home/kitaeza/zurkhai-prompts/spec-012-source-checks.md before every later
    commit.
  - Verify that 012 is free in the tree. Read the Read-first list, and list it in the report.
  - Run `date` once and use that value for every date you write (METHOD rule 51). If a later `date`
    shows a new day, the records written after it carry the new date, and the report says where.
  - Choose your round's marker (METHOD rule 58). The report calls it "the round's marker" and never
    spells it.
  - Re-measure the OPEN count in FORKS.md with the grep its header defines. The advisor counted 5 on
    main at c4af28d (RELAYED). Re-measure it, do not copy it.
  - FORKS.md: four new rows, numbered from the next free F id (derive it; the advisor expects F28 to
    F31, RELAYED), each filed already RULED, in the register's shape. Fields, one per line as given:
      row: R3
        subject: The reading of 001 §6.3's admission clause for a datsan's lunar calendar
        Raised: 2026-11-12, by the advisor, before the Sources round; recorded by spec 012 Item 0
        Question: None of the held datsan calendars names itself zurkhai or the Tibetan calendar; each
          says «Лунный календарь». Does 001 §6.3's "states it is zurkhai / Tibetan calendar" admit them?
        Options: (a) yes, for a Sangha datsan's own Buddhist lunar calendar whose numbers behave like
          tshes, shown by a step check, with a DEBT.md entry to clarify §6.3; (b) no, only a source that
          names зурхай or the Tibetan calendar; (c) case by case, in each source's meta file.
        Status: RULED
        Ruled: 2026-11-12, (a). The human: "1 agree".
        Built in: not yet built; the Sources round (docs/ROADMAP.md, buryat-sources)
      row: R4-Ivolginsky
        subject: The tier of the Ivolginsky datsan's 2021 calendar
        Raised: 2026-11-12, by the advisor, before the Sources round; recorded by spec 012 Item 0
        Question: The calendar is the datsan's own print file, on its website. T1 or T2 (001 §6.1)?
        Options: (a) T1, an official calendar of a Sangha datsan, with edition and compiler; its meta
          file says it is the digital original, not a scan; (b) T2, the datsan's own web publication.
        Status: RULED
        Ruled: 2026-11-12, (a), T1. Admission still waits on the terms check of 001 §8 (spec 012). The
          human: "2 agree".
        Built in: not yet built; the Sources round
      row: R4-Aginsky
        subject: The tier, and the editions, of the Aginsky datsan's calendars
        Raised: 2026-11-12, by the advisor, before the Sources round; recorded by spec 012 Item 0
        Question: The 2025 and 2026 editions disagree on 9–16 January 2026. What tier, and how are the
          editions recorded?
        Options: (a) both T1, one golden file per edition, the disagreement recorded in both and
          neither corrected; (b) the 2025 edition only; (c) both, with the disagreeing days left out.
        Status: RULED
        Ruled: 2026-11-12, (a). Admission still waits on the terms check of 001 §8 (spec 012). The
          human: "3 agree".
        Built in: not yet built; the Sources round
      row: R5
        subject: What the golden is_leap_month column records
        Raised: 2026-11-12, by the advisor, before the Sources round; recorded by spec 012 Item 0
        Question: A source may name the extra month by a different convention from Janson v1's. Does
          the column record the source's label, or a normalised one?
        Options: (a) what the source prints, its convention stated in the meta file, the cell empty
          with a note where nothing is printed, and the Q1 round mapping conventions; (b) normalised to
          Janson v1's convention at transcription; (c) empty for every Buryat source.
        Status: RULED
        Ruled: 2026-11-12, (a). The human: "4 agree".
        Built in: not yet built; the Sources round
    The "row:" names are this prompt's labels for the rulings, not ids; each row takes its F id.
  - DEBT.md: two new entries, numbered from the next free D id (the advisor expects D-30 and D-31,
    RELAYED), Pinned by: not pinned, Status: OPEN:
      (a) DOCS — round 011's four flags: R7's text, items (i) to (iv), with file and line re-measured.
          TRIGGER: the next Process round.
      (b) DOCS — 001 §6.3's admission clause needs the reading R3 ruled. Statement: R3's ruling, and
          that 001 is gated. TRIGGER: the next spec that revises 001.
  - docs/ROADMAP.md: insert this round's row before buryat-sources, one field per line as given:
        arc: Sources
        milestone: M0
        round: source-checks — PLANNED
        achieves: for every calendar held in the library, who publishes it, whether it is a Sangha datsan, the site's terms, and what the governing law says about reusing calendar facts
        opens: buryat-sources
        TRIGGER: roadmap-backlog landed
    and replace buryat-sources' TRIGGER cell with: source-checks landed, and the human has ruled on its
    findings. A Revisions line records both changes, with the old TRIGGER text. Show the file's diff.
  - The open-forks audit: the count expected at your final commit is the count you measured at the
    base; the four new rows are RULED and add nothing. Show the arithmetic at every commit.

ITEM 1 — The control on WebFetch (METHOD rules 31, 36). Before any other fetch:
  WebFetch https://datsan-rinpoche-bagsha.ru/o-datsane, asking for the footer's sentence about the use
  and copying of the site's materials, verbatim. Compare what comes back with line 137 of
  datsan-rinpoche-bagsha/datsan-about/o-datsane.txt in the library (saved 2026-11-11), character by
  character after collapsing whitespace, by a script. Report both texts and the result. If they differ,
  or the page cannot be fetched, STOP RULE (2) applies. Say what the control cannot show: that
  WebFetch is faithful on a page it has not already been checked on.

ITEM 2 — The publishers and their Sangha membership (leg ①, R1 (a)) — the decisive item, with Item 3.
  For each of the seven holdings:
  (a) The publisher: the organisation that publishes the calendar, by the legal name it gives itself
      (an about page, a footer, the calendar's own print), quoted, FETCHED or from the library copy.
  (b) Its identifiers, if published: ИНН, ОГРН, a registration number, or the Swiss commercial
      register's UID.
  (c) Membership of the Buddhist Traditional Sangha of Russia (for the five datsans; for the Kalmyk
      Central Khurul and Rabten Choeling, the body each belongs to instead). Evidence FOR and AGAINST,
      each item tagged and cited, from, in this order of weight: a public register's entry (for Russia,
      the tax service's register of legal entities and the Ministry of Justice's register of
      non-commercial and religious organisations; for Switzerland, the commercial register); the
      Sangha's own published list of its datsans, or its own page naming the datsan; the publisher's
      own statement; and other pages. The verdict is one of: MEMBER, NOT A MEMBER (only on a register
      entry or the Sangha's own list), or UNDETERMINED, with what would settle it. A register that needs
      a form, a login or a captcha is not passed: its URL goes into the link table (Item 6), with the
      identifier to search for.
  (d) For «Ринпоче Багша», the advisor's reading of 2026-11-11, RELAYED and not a premise: its about
      page names the legal entity "Местная религиозная буддийская организация «Буддийский центр
      «Ринпоче Багша»»", ИНН 0323082410; the name lacks the words "Буддийской традиционной Сангхи
      России" that the advisor expected in a member's name; the page never names the Sangha. Check each
      part, and check the expectation itself: do the legal names of the Sangha's datsans you find carry
      such words?

ITEM 3 — The terms (leg ①, 001 §8).
  For each holding, every statement its publisher makes about reusing the site's or the calendar's
  material: a site footer, a terms-of-use page, a copyright line, a licence, and any notice printed on
  the calendar itself (for the PDFs in the library, `pdftotext` into a scratch directory, then grep; for
  images, read them). Quote each, at most two sentences, with its URL or library path, and the date.
  Where a page or a calendar says nothing, say so, with what you read. For khoymorsky-datsan, the page
  is a shop's: report the shop's terms and the datsan's own, separately.

ITEM 4 — The law (leg ①, R1 (b)).
  (a) For each holding, the governing law of its terms as far as the terms or the publisher's seat
      show it (the country, and the seat's address as published), INFERRED where it is a reading.
  (b) For each country found (expected: Russia and Switzerland), the provisions that bear on reusing
      the facts of a published calendar: whether facts and information are protected; any right in a
      compilation or a database; any exception for quotation or for information; and any other law a
      publisher could invoke against reuse (for example unfair competition). Begin with, and do not stop
      at: for Russia, the Civil Code, part 4 (articles 1259 and 1333 to 1336); for Switzerland, the
      Copyright Act (SR 231.1) and the Unfair Competition Act (SR 241). Quote each provision, at most
      two sentences, from an official publication of the statute (for Russia pravo.gov.ru or an
      equivalent official text; for Switzerland fedlex.admin.ch), FETCHED, with the article number
      and its version date if shown. Report what you found AGAINST reuse as fully as what you found for
      it (the leg's own rule).
  (c) What the project would take from each calendar: the Gregorian date, the tshes number, the month,
      the leap-month and doubled/skipped marks (001 §6.2), never the calendar's text, images or design
      (docs/sources.md). State it beside (b), and draw no conclusion (STOP RULE (4)).

ITEM 5 — docs/sources.md.
  (a) Rabten Choeling: a new library row, in the table's shape. Find the calendar's published URL on
      the publisher's site by WebSearch (FETCHED or SNIPPET-VERIFIED, as reached). Obtained: 2026-11-11,
      by the advisor, copied into the library by the human (RELAYED), and the file's modification time
      from `stat` (not a retrieval date). The sha256 you measure. Its licence cell: Item 3's finding.
      Trusted for: nothing yet; not Buryat (001 §1); its role is decided at the Q1 verdict round (R2).
      DEBT.md D-29: Status PAID, with your date and spec 012.
  (b) The six other rows: replace each "not checked (001 §8 …)" licence cell, and add to «Ринпоче
      Багша»'s, with Item 3's finding and "spec 012"; add one sentence to each Source cell with Item 2's
      verdict and its strongest citation. The Kalmyk row's Trusted-for cell: its role was put to the
      human on 2026-11-12 and is decided at the Q1 verdict round (R2). Change nothing else, the
      Bibliography included.
  (c) The library is not written to. Pages worth keeping go into the link table (Item 6).

ITEM 6 — The link table.
  One table, in the report and in the session's final message, of every page the human should open or
  save: each register that needs a form (with the identifier to search for); each page whose quotation
  a ruling may rest on (Items 3 and 4); each page that refused (UNREACHABLE). Columns: URL; what it is
  for; what to search or save; the item. The human saves pages into the library; the round never does.

ITEM 7 — The close.
  - Run tools/gate.sh once, with nothing in flight, in both profiles (CLAUDE.md hard rule 12; METHOD
    rule 65), on the last commit that changes more than the round's spec file, its INDEX.md row and its
    docs/ROADMAP.md row (docs/ROUNDS.md §3). Compare its count with spec 011's recorded count
    (RECALLED).
  - In the final commit: set docs/ROADMAP.md's source-checks row to LANDED, with a Revisions line; add
    the round's row to docs/specs/INDEX.md: 012, `012-source-checks.md`, "Source checks: who publishes
    each calendar in the library, and on what terms", draft; finish the report. Show that `git diff
    --name-only <the gated commit> <the tip>` lists only those three paths.
  - Write the squash commit message. After the final commit the branch stands still. No reviewer runs at
    this close: the round changes neither crates/ nor golden/ (docs/ADVISOR.md §6).

PREDICTIONS — written by the advisor on 2026-11-12, before this round measures anything (METHOD rule 3).
Their basis: the advisor's own reading of the library holdings on 2026-11-11 (what each calendar and
page prints about its publisher) and its general knowledge of the bodies named; it searched no register
and no statute. All of this is RELAYED or the advisor's expectation, and none of it is a premise. Score
each prediction HELD or FAILED, with the measured value. A prediction scored HELD cites your own
measurement, never this prompt.
  1. Every PRECONDITION and ARTIFACT check passes.
  2. Item 1's control holds: WebFetch returns the footer sentence equal to the library's line 137.
  3. The tax service's register of legal entities cannot be searched by WebFetch (it needs a form or a
     captcha) and goes into the link table.
  4. The Ivolginsky, Aginsky and Gunzechoinei datsans are each shown a Sangha member by a primary
     source, or by a Sangha page naming them.
  5. «Ринпоче Багша» ends UNDETERMINED or NOT A MEMBER; no source names it a Sangha member.
  6. The Central Khurul belongs to a Buddhist body of Kalmykia, not to the Sangha.
  7. ivolgdatsan.ru and aginskydatsan.ru each carry some notice about the use of the site's material;
     neither PDF calendar prints one.
  8. The Russian Civil Code's article 1259 is found and quoted, with a clause that ideas and facts are
     not objects of copyright; Swiss law has no right in a database as such.
  9. OPEN is 5 at the base and at the final commit; the new rows are F28 to F31; the new DEBT entries
     are D-30 and D-31; D-29 is PAID.
  10. The diff touches 7 files: DEBT.md, FORKS.md, docs/ROADMAP.md, docs/sources.md,
      docs/specs/012-source-checks.md, docs/specs/prompts/012-source-checks.md, docs/specs/INDEX.md.
  11. The gate's count equals spec 011's.

THE REPORT — per CLAUDE.md's report contract and docs/ROUNDS.md §4, in docs/specs/012-source-checks.md.
It holds:
  - the head block, with its `arc:` line and without a `golden:` line;
  - a report-shaped spec (docs/specs/TEMPLATE.md's header lines: Status: draft; Resolves: the four new
    rows; Milestone: M0; Governs: docs/sources.md), then the end marker;
  - then the report.
  Lead with the decisive item: one table, one row per holding, with the publisher, its identifiers, the
  membership verdict with its strongest evidence for and against, the terms (quoted), the governing law
  and what remains open. Then:
    - Item 1's control: both texts and the result;
    - Item 4's provisions, by country, quoted, with what was found against reuse;
    - the four legs, each reported, the empty ones with their searches;
    - R1–R7, each beside the record that implements it;
    - the link table (Item 6);
    - the predictions, scored;
    - the open-forks audit, with its arithmetic at every commit;
    - the marker greps, at every commit;
    - every other section of the contract, "What the prompt got wrong" included;
    - the diff scope, with its totals summing against `git diff --stat main`, stating the commit it was
      measured at;
    - every STOP AND ASK, if any;
    - last before the squash message: the searches and fetches made (each query or URL, its host, and
      what came back, one line each), the commands and the scripts, verbatim.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 012 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════
