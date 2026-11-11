═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 010 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  010 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
BRANCH:       spec/010-publication, cut before any edit from main's tip, once every PRECONDITION check
              passes; commit to it. No merge. No push, to any remote, the private mirror included.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7, first branch. Every answer here is
              discoverable: a tag looked up in the action's own repository, two histories paired commit
              by commit, a scan of what main's history holds, and a docs edit that carries the human's
              rulings as they were given. Three parts would be silent, and each runs only with an
              instrument that makes it loud (docs/ROUNDS.md §8):
                - the pin. A wrong commit id that exists passes CI without a sign: GitHub also resolves
                  a commit id that lives only in a fork of the action's repository. Instrument: the id
                  comes only from a tag ref of actions/checkout itself, by two independent routes that
                  must agree (Item 2);
                - the map. A wrong pairing reads like a right one. Instrument: every row compared field
                  by field, and an off-by-one control that must fail (Item 3);
                - the audit. An address or a secret the scan misses is published for good. Instrument:
                  the same commands, run first on a scratch repository with planted items, must find
                  every one (Item 1).
              No change from the last round: round 009 also ran on Sonnet 5.5, effort High.
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first. It never commits a previous session's uncommitted work without
              verifying it: it finishes the work and proves it by the item's own check, or redoes it. A
              design choice that would fall to the fallback model (Opus 5.5, effort High) is a STOP AND
              ASK. The report names the switch point.
PRECONDITION: Spec 009 has been merged, and the human has then rewritten main's history (R3, below).
              This round checks the result; it never makes it. There is no fixed commit id here, because
              the rewrite gave every commit a new one. Check each of these, and quote every output:
              (a) `git -C /home/kitaeza/github/zurkhai status --porcelain` prints nothing, and
                  `git -C /home/kitaeza/github/zurkhai rev-parse --abbrev-ref HEAD` prints main.
              (b) `git -C /home/kitaeza/github/zurkhai log --format='%h %T %s' main` lists 11 commits.
                  The oldest has the subject "Initial commit", the newest is spec 009's squash, and
                  `git -C /home/kitaeza/github/zurkhai rev-list --max-parents=0 main` prints one commit.
              (c) `git -C /home/kitaeza/github/zurkhai log --format='%ae%n%ce' main | sort -u` prints
                  only addresses from THE ALLOWLIST (below).
              (d) The branch archive/main-pre-rewrite exists and has 11 commits; its tree equals main's
                  (`git -C /home/kitaeza/github/zurkhai rev-parse 'main^{tree}'
                  'archive/main-pre-rewrite^{tree}'` prints two equal hashes); and
                  `git -C /home/kitaeza/github/zurkhai ls-remote rounds refs/heads/archive/main-pre-rewrite`
                  prints the commit that `git -C /home/kitaeza/github/zurkhai rev-parse
                  archive/main-pre-rewrite` prints.
              (e) `git -C /home/kitaeza/github/zurkhai rev-list --count main ^archive/main-pre-rewrite`
                  prints 11: the two histories share no commit, the initial one included.
              (f) `git -C /home/kitaeza/github/zurkhai var GIT_AUTHOR_IDENT` names the first address of
                  THE ALLOWLIST.
              Anything else: STOP and report, with zero diff. The merge and the rewrite are the human's.
              Read first, in this order, each in full: CLAUDE.md; docs/ROUNDS.md; docs/specs/003-project-
              method.md (§2, §6, §7, §11, §12); docs/METHOD.md rules 1, 3, 8, 15, 31, 35, 36, 50, 51, 56,
              58, 60, 61, 65 and 67, each in full, not only its **Rule.** paragraph; GATED.md; FORKS.md
              (F11 in full, its header, the rest by their status lines); DEBT.md (its header, and D-8 in
              full); .github/workflows/ci.yml; docs/specs/INDEX.md; docs/specs/TEMPLATE.md;
              docs/specs/009-henning-differential.md (its head block only); docs/sources.md (its first
              section). The report lists each file it read and how much of it.
ARTIFACT:     None outside the repository. Item 2 reads two network sources, named there; if either is
              UNREACHABLE, STOP and report.
STOP RULE:    (1) A PRECONDITION check fails: STOP, zero diff.
              (2) Item 1 finds anything it calls a FINDING: STOP AND ASK. Never spell what was found:
              cite the commit, the path and the line, and for an email address only the part after the
              @. Never edit, remove or rewrite anything to make the audit pass: the history is the
              human's.
              (3) Item 2's two routes disagree, or the tag v4 does not point to the commit of a
              published v4.x.y release: STOP AND ASK, with both outputs.
              (4) Item 3 finds a field that differs on a true pair (a signature excepted, Item 3c): STOP
              AND ASK, with the row.
              (5) Out of scope; do not write any of it:
                - any change under crates/, golden/, rules/, clients/ or tools/; docs/METHOD.md,
                  GATED.md, CLAUDE.md, docs/ADVISOR.md, docs/sources.md, or an accepted spec;
                - a README, a licence change, a newer major version of actions/checkout, or any other
                  change to the workflow (its triggers, permissions, jobs or caching);
                - any push, fetch or pull; deleting a branch; creating a tag; rewriting any history;
                - deciding any fork.
              If something seems to need one of these, write it down for the report (by-catch, and a
              DEBT.md entry where it qualifies), and do not do it.
              (6) A read, a run or a network call outside GRANTS is needed, or a permission is denied:
              STOP and report each, with the reason. Never work around it by another tool or route
              (METHOD rule 67).
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; there is no fourth
              kind. An unobtainable source says PAYWALLED or UNREACHABLE. Read docs/sources.md before any
              fetch. Never curl/wget. (METHOD rule 50.) This round's only fetches are Item 2's lookups:
              each is cited as FETCHED, with its command, its output and the date.
GATED:        no gated path, except this round's own docs/specs/010-publication.md, its
              docs/specs/prompts/010-publication.md, and its row in docs/specs/INDEX.md, which are not
              changes to an accepted spec. Not gated, and changed only as the items say:
              .github/workflows/ci.yml, DEBT.md, FORKS.md, docs/ROUNDS.md.
GRANTS:       Reads outside WORKDIR, beyond this prompt file: none.
              Network, read-only, and only:
                - `git ls-remote https://github.com/actions/checkout` with a ref pattern (Item 2);
                - `gh api` GET requests under repos/actions/checkout/ (Item 2);
                - `git -C /home/kitaeza/github/zurkhai ls-remote rounds` (PRECONDITION (d)).
              No other network: no fetch, pull or push, and no gh command that writes.
              The tools:
                - the Read tool, on the repository;
                - git on the repository, read-only (status, log, show, cat-file, rev-parse, rev-list,
                  ls-tree, grep, diff, var), and what the round's own branch and commits need
                  (switch -c, add, commit);
                - `git init`, `git add` and `git commit`, only inside a `mktemp -d` scratch repository
                  (Item 1's control);
                - `python3 -I`, running only scripts you write into a `mktemp -d` directory;
                - tools/gate.sh, as its own header says, on the final committed tree (Item 5);
                - read-only shell tools on the repository and on your own scratch: cat, head, tail,
                  sed -n, grep, awk, cut, tr, sort, uniq, wc, diff, cmp, od, ls, stat, find, date, file,
                  sha256sum;
                - mkdir -p, cp, mv, rm -r and dd, only inside `mktemp -d` directories.
              Files in the repository are edited only with the Edit and Write tools, never by a script or
              a shell redirection (R4). On this machine `grep` is ugrep: a bounded repetition of 100 or
              more, such as `.\{0,120\}`, fails with "exceeds complexity limits"; keep bounds below 100.
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive
              widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 010 — Before the first public push: the CI pin, the history audit and map, and the private mirror

The repository's public origin still holds only its initial commit. Before the human pushes main there,
three things are owed. DEBT.md D-8 has a TRIGGER that fires at that push: the workflow pins
actions/checkout by a movable tag. The human has rewritten main's history, so that its commits carry
their GitHub no-reply address. This round checks that rewrite and commits the map from old commit ids to
new. And F11 is now ruled, which changes how a merged round's branch is kept. The round also audits main's
history once, before it becomes public, for anything that must not be published. It does NOT push,
rewrite, delete or tag anything; it does not touch the engine; and it decides no fork.

THE ALLOWLIST — the only email addresses main's history may hold, in its commits' author and committer
fields, in its messages, and in its files:
  8291740+kitaeza@users.noreply.github.com   the human's GitHub no-reply address
  noreply@github.com                         GitHub's own (the initial commit's committer: it was made in
                                             GitHub's web interface)
  noreply@anthropic.com                      the Co-Authored-By trailers of rounds run on Claude models
  noreply@deepseek.com                       the Co-Authored-By trailer of round 004

THE HUMAN'S RULINGS this round carries (2026-11-11):
  R1. F11 = (c). Merged round branches go to a private mirror, the git remote `rounds`, and never to the
      public origin. After a merge's tree check, the human pushes the branch there, checks that the
      mirror holds its tip, and then deletes the local branch. The human: "agree with recommendations,
      go". The mirror is named in the tree by its remote only, never by its URL.
  R2. main is pushed to the public origin once D-8 is paid, and from then on after every merge
      (docs/ROUNDS.md §9, step 4). Same words.
  R3. Before the first public push, every commit on main, the initial commit included, is rewritten so
      that a personal email address of the human's is replaced by their GitHub no-reply address. The
      old history is kept in the private mirror as the branch archive/main-pre-rewrite. This round
      commits the map from old commit ids to new. The public origin's one existing commit is replaced
      once, by a force-push after this round merges. The human: "keep mail address private", and "yes
      force-push". The human made the rewrite before this round. This round checks it and never repeats
      it. It does not need to know which address was replaced, and must not write one anywhere: no
      format string it runs on archive/main-pre-rewrite holds %ae, %aE, %ce or %cE.
  R4. Standing since round 009 (its R1): repository files are edited only with the Edit and Write tools,
      and METHOD rules 31 and 36 are explicit checks on this round's own controls (Items 1, 2 and 3):
      each control shows its instrument can fail, and reports exactly the seeded difference and no other.

ITEM 0 — Branch, prompt, number, records.
  - Check PRECONDITION, and quote every output.
  - Create spec/010-publication from main's tip. The head block's branch line names that commit and
    its tree.
  - Commit this file, byte-identical, as docs/specs/prompts/010-publication.md. That is the first
    commit. `cmp` it against /home/kitaeza/zurkhai-prompts/spec-010-publication.md before every later
    commit.
  - Verify that 010 is free in the tree.
  - Read the Read-first list, in full, and list it in the report.
  - Run `date` once and use that value for every date you write (METHOD rule 51). If a later `date`
    shows a new day, the records written after it carry the new date, and the report says where.
  - Choose your round's marker (METHOD rule 58). The report calls it "the round's marker" and never
    spells it.
  - Re-measure the OPEN count in FORKS.md with the grep its header defines. The advisor counted 6 on
    main before round 009 and before the rewrite (RELAYED); round 009 may have changed it. Re-measure it,
    do not copy it.
  - FORKS.md, F11: write R1 onto its row, in the register's shape: `Status:` RULED; `Ruled:` 2026-11-11,
    (c), a private mirror, the git remote `rounds`, and the human's words; `Built in:` spec 010. The
    row's interim line ("Until ruled, …") stays, and gets "(superseded: RULED 2026-11-11)" at its end.
  - FORKS.md, one new row for R3, numbered from the next free F id (F24 was next before round 009, RELAYED:
    derive it from the file). It is filed already RULED. Its fields, one per line as given:
      subject: The commit email address on main, before the first public push
      Raised: 2026-11-11, by the advisor, before the first public push; recorded by spec 010 Item 0
      Question: main's commits carried a personal email address of the human's, as author and committer.
        Publish them as they were, or rewrite them first?
      Options: (a) publish them as they were; (b) rewrite every commit on main, the initial commit
        included, to the human's GitHub no-reply address, keep the old history in the private mirror,
        and commit a map from old commit ids to new.
      Status: RULED
      Ruled: 2026-11-11, (b), the initial commit included. The human: "keep mail address private";
        "yes force-push".
      Built in: the rewrite, by the human, before spec 010; the map, spec 010 (Item 3).
  - The open-forks audit: the count expected at your final commit is the count you measured at the base,
    minus 1 (F11). The new row is RULED and adds nothing. Show the arithmetic at every commit.

ITEM 1 — The audit of main's history (the decisive item, with Item 2).
  The question: does main's history, as it will be published, hold anything that must not be? The scope
  is all 11 commits of main: every text file at every commit (`git grep`, per commit), every blob
  reachable from main, every commit message, and every author and committer field.
  (a) Email addresses. Use this extended regular expression:
        [A-Za-z0-9._%+-]+@[A-Za-z0-9-]+(\.[A-Za-z0-9-]+)*\.[A-Za-z]{2,}
      Count every distinct match, in files and in messages. A match outside THE ALLOWLIST is listed by
      commit, path and line, with only the part after the @. If you judge a listed match not to be an
      email address, give your reason; a match you judge to be one, or cannot judge, is a FINDING.
  (b) Secrets. Patterns: `ghp_`, `gho_`, `ghu_`, `ghs_` or `ghr_` followed by 36 letters or digits;
      `github_pat_` followed by 22 or more letters, digits or underscores; `sk-` followed by 20 or more
      letters, digits, `-` or `_`; `AKIA` followed by 16 capitals or digits; `BEGIN`, then any capitals
      and spaces, then `PRIVATE KEY`; `xox` followed by one of a, b, p, r, s, then `-`. Any match is a
      FINDING, listed by commit, path, line and the pattern's name, never its text.
  (c) Blobs. List every blob reachable from main that contains a NUL byte, or is larger than 1,000,000
      bytes, or whose path ends in one of: .pdf .djvu .png .jpg .jpeg .gif .webp .tif .tiff .zip .gz .tgz
      .xz .7z .rar .tar .doc .docx .xls .xlsx .odt .exe .dll .so .dylib. Give its path, its size, the
      first commit that added it, whether it is in main's tip, and the spec or record that accounts for
      it. A listed blob is a FINDING if it is third-party material (CLAUDE.md hard rule 17), or if
      nothing in the tree accounts for it.
  (d) Paths deleted along main: every path that some commit of main holds and main's tip does not, with
      the commit that removed it and what it was. A FINDING if it held anything (a), (b) or (c) would
      catch, or third-party material.
  The control comes first (METHOD rules 31, 36). In a `mktemp -d` scratch repository, make two commits:
    - the first adds a text file holding three things: one address outside THE ALLOWLIST; one string
      of the `ghp_` shape; and noreply@anthropic.com, which is on THE ALLOWLIST. It also adds planted.bin,
      1,100,000 bytes with a NUL byte in it. Your script builds the first two strings from parts at run
      time, so neither whole string is ever written into a file outside the scratch directory;
    - the second commit deletes the text file.
  Run (a)–(d) on the scratch repository. Expected, and nothing else: (a) one address and not the
  allowlisted one; (b) one match; (c) planted.bin; (d) the text file. Report what was seeded, what was
  expected and what was observed. Then run (a)–(d) on main.

ITEM 2 — D-8: pin actions/checkout by commit id.
  (a) Quote every `uses:` line of .github/workflows/ci.yml with its line number. D-8 names two, at lines
      29 and 42 before the rewrite (RELAYED: re-measure).
  (b) Route A: `git ls-remote https://github.com/actions/checkout 'refs/tags/v4*'`. For an annotated
      tag, the line ending `^{}` is its commit. Quote the output.
  (c) Route B: `gh api repos/actions/checkout/git/matching-refs/tags/v4`. For each annotated tag you rely
      on, `gh api repos/actions/checkout/git/tags/<the tag object's id>` gives the commit it points to.
      Then `gh api 'repos/actions/checkout/releases?per_page=100' --paginate --jq '.[] | [.tag_name,
      .draft, .prerelease, .published_at] | @tsv'`. Quote the outputs. The releases list may be trimmed to
      its v4.* lines and the five newest others.
  (d) Choose the highest v4.x.y, in numeric order, that is a published release (not a draft, not a
      prerelease) and whose commit equals the commit that v4 points to. Both routes must give the same
      commit for v4 and for the chosen release. If they disagree, or if v4 points to no such release's
      commit: STOP AND ASK (STOP RULE (3)).
  (e) Edit both `uses:` lines to `uses: actions/checkout@<the commit's 40 hex digits> # v4.x.y`. Change
      the file's header comment where it states the tag pin and its reason, so that it states the
      commit pin, the release, how the id was found (both routes, the date) and D-8, in as few words as
      that needs. Change nothing else in the file.
  (f) The check: every `uses:` line under .github/ ends in `@` and 40 hex digits, then optionally a
      comment. The control comes first: the same check, run on a scratch copy of the unedited file
      (`git show main:.github/workflows/ci.yml`), must report exactly the two tag-pinned lines. Then
      `git diff main -- .github/` must show only those two lines and the comment. If `python3 -I -c
      'import yaml'` succeeds, parse the edited file and say so. If it fails, say so, and install nothing.
  (g) DEBT.md D-8: `Status:` PAID, with your date and "spec 010". As by-catch, report any newer major
      version of actions/checkout, and anything its release notes say about the support of v4 (FETCHED).
      Change nothing for either. File a DEBT.md entry only if a release note gives a date on which v4
      stops working.
  (h) docs/sources.md is the bibliography of calendar sources. This lookup is not one, and takes no row
      there. Its citation lives in ci.yml's comment and in the report.

ITEM 3 — The history map.
  (a) Check that main and archive/main-pre-rewrite are both linear: `git rev-list --min-parents=2 --count`
      prints 0 for each. Pair their commits by position, oldest first (`git rev-list --reverse`).
  (b) For every pair, compare: the tree (%T); the author name (%an); the author date (%ad, with
      --date=raw); the committer name (%cn); the committer date (%cd, with --date=raw); and the raw
      message (%B), byte for byte. Also check that the new commit's parent is the new commit paired with
      the old commit's parent (the initial commits have none). Email addresses are not compared (R3).
  (c) Signatures: for each commit on each side, say whether it carries one (`git cat-file commit <id>`,
      a line starting `gpgsig`). A rewrite cannot keep a signature. A signed old commit whose new commit
      is unsigned is reported, and is not a FINDING.
  (d) The control comes first (METHOD rules 31, 36): pair old commit i with new commit i+1. The tree
      comparison must fail on all 10 shifted pairs. Report the count.
  (e) A field of (b) that differs on any true pair: STOP AND ASK (STOP RULE (4)).
  (f) Write the map into your own spec file, docs/specs/010-publication.md, as a section
      "## The history map", with the Write or Edit tool (R4), never by a redirection. It opens with one
      paragraph: what was rewritten and why, citing the new row's F id; that the old history is kept in
      the private mirror as archive/main-pre-rewrite, which a public reader cannot reach; which fields
      are equal, by (b); and how to translate a commit id that the tree cites from before the rewrite.
      Then a table with one row per commit: position, old commit (40 hex digits), new commit (40 hex
      digits), tree (40 hex digits), subject.
  (g) The transcription check: a script derives the 11 rows from git again and compares them with the
      table as committed; every row must agree. The control comes first: a scratch copy of the file with
      one hex digit changed in one row must make the check report exactly that row.
  (h) Where the tree cites old ids: in main's tip, count the mentions of each old commit's 7-character
      id, per file. The advisor counted, before round 009 and before the rewrite, 45 mentions of the nine
      commits after the initial one, in 10 files, and 1 mention of the initial commit, in 1 file
      (RELAYED). Round 009's files may add more: re-measure. Report the table. Edit none of these
      mentions: the committed prompts and the reports are records.

ITEM 4 — docs/ROUNDS.md §9, revision r3.
  The round words it. The advisor reads the wording at the landing.
  (a) Step 4 becomes R2, the push of main after every merge, with a check first. The check:
      `git -C /home/kitaeza/github/zurkhai log --format='%ae%n%ce' origin/main..main | sort -u` must
      print only THE ALLOWLIST's first address. If it prints anything else, the human does not push and
      asks the advisor. Then `git -C /home/kitaeza/github/zurkhai push origin main`.
  (b) Step 5 becomes R1. After step 3's tree check, run
      `git -C /home/kitaeza/github/zurkhai push rounds main spec/NNN-name`. Then check that
      `git -C /home/kitaeza/github/zurkhai ls-remote rounds refs/heads/spec/NNN-name` prints the commit
      that `git -C /home/kitaeza/github/zurkhai rev-parse spec/NNN-name` prints. Then run
      `git -C /home/kitaeza/github/zurkhai branch -D spec/NNN-name`. Keep the sentence on why `-d`
      refuses after a squash. Name the mirror by its remote only.
  (c) Add one sentence to §9: main's history was rewritten once, before the first public push (the new
      row's F id), and a commit id that the tree cites from before then is translated by
      docs/specs/010-publication.md, "The history map".
  (d) Set the Status line to r3, with your date. Add a revision entry: r3 (<your date>): §9 steps 4 and 5,
      and the history note; on the human's rulings of 2026-11-11 (F11 (c), the new row, and R2).
  (e) grep docs/, CLAUDE.md and the root files for "kept locally", "F11" and "when the human decides to
      publish". A sentence beside your edit that now contradicts it may be fixed (CLAUDE.md hard rule
      19), and the report says so. Every other hit is listed as a flag, and is not fixed.

ITEM 5 — The close.
  - Run tools/gate.sh on the final committed tree, with nothing in flight, in both profiles (CLAUDE.md
    hard rule 12; METHOD rule 65). Compare its count with the last gate count recorded on main, naming
    the spec it comes from (RECALLED).
  - In the final commit, add the round's row to docs/specs/INDEX.md: 010, `010-publication.md`, "Before
    the first public push: the CI pin, the history audit and map, the private mirror", draft.
  - Write the report and the squash commit message. After the final commit the branch stands still. No
    reviewer runs at this close: the round changes neither crates/ nor golden/ (docs/ADVISOR.md §6).

PREDICTIONS — written by the advisor on 2026-11-11, before this round measures anything (METHOD rule 3).
Their basis: the advisor grepped, at the commit that was main's tip before round 009 and the rewrite,
every path it may read (all but crates/ and clients/), and every commit message on main and on the round
branches, for email addresses; it found none outside THE ALLOWLIST. It found one file over 1,000,000
bytes, golden/reference/tdc-97d4a00-phugpa-1900-2100.csv (1,976,432 bytes). It found that the old
initial commit carries a signature and no other commit does, that main has no merge commits, and that
the repository has no tags. It did not read crates/ or clients/, and did not look up actions/checkout.
All of this is RELAYED, and none of it is a premise. Score each prediction HELD or FAILED, with the
measured value. A prediction scored HELD cites your own measurement, never this prompt.
  1. Every PRECONDITION check passes.
  2. Every control (Items 1, 2 and 3) reports exactly what was seeded.
  3. Item 1 finds no FINDING: no address outside THE ALLOWLIST, no secret, no blob that is binary, a
     document, an image or an archive, and no path deleted along main.
  4. Every blob over 1,000,000 bytes is under golden/reference/, and is accounted for by spec 006.
  5. The tag v4 points to the commit of exactly one published v4.x.y release, the highest v4 release,
     and the two routes agree.
  6. actions/checkout has published a release with a major version above 4.
  7. All 11 pairs agree on every field of Item 3(b); the old initial commit is signed and its new
     commit is not; no other commit on either side is signed.
  8. The tree cites old 7-character ids at least 46 times.
  9. The gate's count equals the last count recorded on main.
  10. OPEN at the final commit equals OPEN at the base minus 1.
  11. The diff touches 7 files: .github/workflows/ci.yml, DEBT.md, FORKS.md, docs/ROUNDS.md,
      docs/specs/010-publication.md, docs/specs/prompts/010-publication.md, docs/specs/INDEX.md.

THE REPORT — per CLAUDE.md's report contract and docs/ROUNDS.md §4, in docs/specs/010-publication.md.
It holds:
  - the head block, without a `golden:` line;
  - a report-shaped spec (docs/specs/TEMPLATE.md's header lines: Status: draft; Resolves: F11 and the
    new row; Milestone: none, the publication of the repository; Governs: .github/workflows/ci.yml and
    docs/ROUNDS.md §9), then the end marker;
  - then the report, with "## The history map" as its own section.
  Lead with the decisive item: Item 1's four checks, with their counts, and Item 2's pin, with both
  routes' outputs. Then:
    - every control of Items 1, 2 and 3: what was seeded, what was expected, what was observed;
    - Item 3's comparison, its signatures and its citation table;
    - the predictions, scored;
    - R1 to R4, and the records Item 0 wrote, by id;
    - the open-forks audit, with its arithmetic at every commit;
    - the marker greps, at every commit;
    - every other section of the contract, "What the prompt got wrong" included;
    - the diff scope, with its totals summing against `git diff --stat main`, stating the commit it was
      measured at;
    - every STOP AND ASK, if any;
    - last: the commands and the scripts, verbatim. A script's planted strings are shown as the parts
      it builds them from, never whole.
  The report never holds an email address outside THE ALLOWLIST.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 010 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════
