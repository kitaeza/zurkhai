═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 013 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  013 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
ARC:          Sources
BRANCH:       spec/013-buryat-sources, cut from main at 01f4d09 before any edit; commit to it.
              No merge. No push, to any remote.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7, first branch, with docs/ROUNDS.md §8's
              condition. Every answer is discoverable: what a calendar prints on each day, and what a
              saved page says. Nothing is invented: every interpretive question is ruled (R4–R8 below).
              But transcribing a datsan's calendar into golden/ is SILENT (docs/ROUNDS.md §8): a misread
              cell fails no test and compounds into the Q1 verdict. So the round runs on the smaller
              model only with the instruments that make a wrong cell loud, each with a planted failure
              that shows it can fail (METHOD rule 31):
                - two passes by two routes per source, compared by a script, cell by cell (Items 3, 4);
                - a seeded wrong cell that the comparison must report (Item 5(a));
                - the tshes step check, with a planted bad step (Item 5(b));
                - the weekday check against the weekdays the calendar prints (Item 5(c));
                - for the Aginsky editions, the label-to-lunation check, with a planted shifted label
                  (Item 5(d)); and the two editions' overlap (Item 5(e)).
              No change from the last round: round 012 also ran on Sonnet 5.5, effort High.
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first. It never commits a previous session's uncommitted work without
              verifying it: it finishes the work and proves it by the item's own check, or redoes it. A
              design choice that would fall to the fallback model (Opus 5.5, effort High) is a STOP AND
              ASK. The report names the switch point.
PRECONDITION: Check each, and quote every output:
              (a) `git -C /home/kitaeza/github/zurkhai status --porcelain` prints nothing, and
                  `git -C /home/kitaeza/github/zurkhai rev-parse --abbrev-ref HEAD` prints main.
              (b) `git -C /home/kitaeza/github/zurkhai rev-parse main` prints
                  01f4d095d4b6ad02960c828ea1b8567b26594511, spec 012's squash.
              (c) `ls /home/kitaeza/zurkhai-papers/` succeeds (the human has added the directory with
                  /add-dir).
              Anything else: STOP and report, with zero diff.
              Read first, in this order, each in full unless a part is named: CLAUDE.md;
              docs/ROUNDS.md; docs/METHOD.md rules 1, 3, 8, 31, 35, 36, 47, 50, 51, 56, 58, 60, 65, 66
              and 67, each in full, not only its **Rule.** paragraph; GATED.md; FORKS.md (its header;
              F9, F22 and F28 to F31 in full; the rest by their status lines); DEBT.md (its header; D-29
              to D-32 in full; the rest by their headings); docs/sources.md (in full); docs/ROADMAP.md;
              docs/BACKLOG.md; docs/specs/001-init-spec.md (§1, §5.1, §6, §8 and §10);
              docs/specs/003-project-method.md (§4, the P3 row); docs/specs/012-source-checks.md (the
              head block, §0, §1, §3 and §6); golden/LICENSE.md; golden/reference/README.md;
              docs/specs/INDEX.md; docs/specs/TEMPLATE.md. The report lists each file it read and how
              much of it.
ARTIFACT:     The paper library, /home/kitaeza/zurkhai-papers/, read only. Each file below must exist
              with the sha256 given (MEASURED by the advisor on 2026-11-12, RELAYED); re-measure each.
              (A) The calendars this round transcribes:
                - ivolginsky-datsan/lunnii_kalendar-2021.pdf
                  2e140c65b7cac71cbb6c98b837e1f7488a91d47ccd7ea299f3b062d7e320fe25
                - aginsky-datsan/aga/kalendar2025.pdf
                  12137318075f3a7ae946f946b5bbf1b4f628e4186adbdad033b510f2868bd7bb
                - aginsky-datsan/aga2026/kalendar-a2-format.pdf
                  13e7d720b3ba9dd8d47abe090584970afe91f43d6892e8f5cc811351832aa156
              (B) The pages the human saved on 2026-11-12, each beside a `.headers` file of the same
                  name plus `.headers` (HTTP response headers), except the EGRUL extract:
                - datsan-rinpoche-bagsha/saved-2026-11-12/rb-o-datsane.html
                  8a269b448c298e2fbf4a7554da26852bbdb63190cc32ba0caee233d9a7664ba8
                - datsan-rinpoche-bagsha/saved-2026-11-12/egrul-0323082410-2026-11-12.pdf
                  fa3eadd53481389d93b07279129a6a1104627ebbee10c92f983bea757f5a51f7
                - ivolginsky-datsan/saved-2026-11-12/ivolg-home.html
                  c1e5ff15ef0b1ead45b3707f9342adaa093f33da6edde611b32f5240643fb59c
                - aginsky-datsan/saved-2026-11-12/agin-home.html
                  042906adae1d0a973641f9303542893790cb306393477caff7e179de0b219205
                - aginsky-datsan/saved-2026-11-12/agin-o-daczane.html
                  c57565382c81ec3c9e55dfdf917bd96c8a42867e23d4d90fc515726dbeb7e7f6
                - aginsky-datsan/saved-2026-11-12/agin-kontakty.html
                  7aae479b03b54897afd00784c2f40433944307bab2d5c18602cfb60a75c5e156
                - aginsky-datsan/saved-2026-11-12/agin-privacy-policy.html
                  2c045d9e0872a141d2d621c0387a81bcea6929bfe9ee61d67a60c971a8205349
                - gunzechoinei-datsan/saved-2026-11-12/dazanspb-home.html
                  f774e6b1e0a93bdfbd181ad0d1ebfbd114286379b415b137889b913b07d874a8
                - gunzechoinei-datsan/saved-2026-11-12/dazanspb-news-2024.html
                  38e149bed50cb9b8676a7d269a89352f5676341eb529006574ac3613768171dd
                - gunzechoinei-datsan/saved-2026-11-12/dazanspb-news-2025.html
                  3ed4c3ce8fc256a9de1179233d1e83c03fdc48555bba078c61f99c33a88e7575
                - gunzechoinei-datsan/saved-2026-11-12/dazanspb-news-2026.html
                  d056e5dced2342ba5354a87c8b86a67ca34101ca2e0442425359d8a7292e1e9c
                - khoymorsky-datsan/saved-2026-11-12/dharma-oferta.html
                  5390a156e23d8fb3a792291000e25aa9c3416f9854dd81ec2e2ea687ccf55e48
                - kalmyk-central-khurul/saved-2026-11-12/khurul-calendar.html
                  7ce26a3bc1a34ec5049f679c428b762c6b66b4e7f69cf36947cd561a51e531e7
                - kalmyk-central-khurul/saved-2026-11-12/khurul-contacts.html
                  1b68135567d95997a2c001cef142a7666f8a9f1534c611281f0060f4c1642beb
                - rabten-choeling/saved-2026-11-12/rabten-editorial.html
                  76b874fc7c91ae617da1262ab49008ef0ce9289324ce020ce3d92f4e0bf9ba3f
                - rabten-choeling/saved-2026-11-12/rabten-iDevCalendar.html
                  58e35eca9a70b305e21daff829764c738eabe45f35758acb25113b1b26661575
                - law-switzerland/SR-231.1-de.pdf
                  ae9b7743ce1827480bff7e2dc3ebaa1cc49515574fbe5bdbb9abe571b1a08947
                - law-switzerland/SR-241-de-20250101.pdf
                  4ba11ff97726b3fbf8c6255b7d89294c18e7c2a993a011738bc7d27abee37f9e
                - sangha-secondary/ruwiki-BTSR.html
                  2b56c2876fd069181894ba0df3979ab1085b3b3aac64c067e51b465da0a59511
              (C) The holdings' manifests, made by `find <dir> -type f | LC_ALL=C sort | xargs -d '\n'
                  sha256sum | sha256sum`, with the file count by the same `find … | wc -l`:
                - datsan-rinpoche-bagsha/: 106 files, 45a92edcadb758e1e6b69b3c09f9de95b0f97e9b114312bffdb3844d6c332b4f
                - ivolginsky-datsan/: 23 files, bc4f1454a16cc3e39f3063bd4eb1a80c8d55ce1222a859a6bd2f635fd54d0b9a
                - aginsky-datsan/: 18 files, 6b22e6b82f4ec4a5d92fd7fb3f1f929d248239a6b6a6653ee971ef1a7ae55dc0
                - gunzechoinei-datsan/: 26 files, 55e9f475833e36101bee46072de4c02d722d4fe8868ab71fab523de6e6890ae7
                - khoymorsky-datsan/: 10 files, b607faedb2d0d6c808aee211395845aa1a75e53811542277b6c92e9ca98f664d
                - kalmyk-central-khurul/: 35 files, 8757377eece1214f8f6b0373bbc7c668141cb371e8c66fde4f0bace82689ee1c
                - rabten-choeling/: 5 files, 4652d0ad921a03e4131e9a00dc700911a2182c9c7aae4ba04a1c94abb421e674
                - law-switzerland/: 4 files, d58fe0ddd692fff9b7ff9b1655395d046e44d5d46a1fc5bc07a57d6242454bdc
                - sangha-secondary/: 2 files, 6ff7f23babd2eef03f09fffee5d1ce7df1c17ff4443e85c63d1a3c0f70a068ab
                The same command with `-not -path '*/saved-2026-11-12/*'` after `-type f` gives, for each
                of the first seven, the manifest docs/sources.md already records (made 2026-11-11) for
                datsan-rinpoche-bagsha/, ivolginsky-datsan/, aginsky-datsan/, gunzechoinei-datsan/,
                khoymorsky-datsan/ and kalmyk-central-khurul/, and for rabten-choeling/ the value
                73e4fb9d808d82d41b6444f36e6119e2f1cb9197f4bfd83b2f1df945286fee4d (RELAYED; the
                advisor's, 2026-11-11). That shows the files of 2026-11-11 are unchanged.
              If a named file is absent, or a sha256 or a manifest differs: STOP and report.
STOP RULE:    (1) A PRECONDITION or ARTIFACT check fails: STOP, zero diff.
              (2) Item 1 finds that the evidence a ruling (R4 to R7) rests on is not what this prompt
              says it is (a quoted line absent or different, a terms statement found where the prompt
              says none was found, a Sangha statement absent): do not transcribe that source. Write a
              STOP AND ASK for it, file its FORKS.md row, and continue with the sources whose evidence
              held. Not a stop for the round.
              (3) Item 5 fails on a source and the failure is not a transcription slip settled against
              the rendered page (Item 3(d), 4(d)): the source's file is not committed. STOP AND ASK.
              (4) A finding bears on a ruling already made (F22, F28 to F31, or R4 to R8 below): do not
              change the ruling or its row. Report the finding beside the ruling.
              (5) No legal conclusion. Never write, in your own voice, that a reuse is permitted,
              allowed, lawful, forbidden or unlawful. Quote the terms; admission is the human's, and
              ruled (R4 to R7).
              (6) No contact with anyone: no email, no form, no message, no account, no purchase. The
              request to the Ivolginsky datsan (R5) is the human's, not the round's.
              (7) Out of scope; do not write any of it:
                - any change under crates/, rules/, clients/, tools/ or .github/; docs/METHOD.md,
                  GATED.md, CLAUDE.md, docs/ADVISOR.md, docs/ROUNDS.md, an accepted spec, or another
                  round's row in docs/specs/INDEX.md;
                - any file under golden/ other than golden/buryat/ (the three CSVs, their three meta
                  files, and golden/buryat/README.md);
                - golden rows for any source other than the three of ARTIFACT (A);
                - any comparison of a Buryat source with an engine system or with the T4/T5 fixtures:
                  that is the Q1 verdict round's (docs/ROADMAP.md, q1-verdict);
                - opening any `readings/` directory of the library before Item 3(e) and Item 4(e)
                  allow it;
                - anything written into the paper library; anything written into git that is a third
                  party's text beyond a quotation of at most two sentences (CLAUDE.md hard rule 17);
                - any line of the EGRUL extract that names a natural person, or a personal ИНН, in any
                  file of the repository or in the report;
                - any push, fetch or pull; deleting a branch; creating a tag;
                - deciding any fork, or writing a FORKS.md row other than Item 0's and STOP RULE (2)'s.
              If something seems to need one of these, write it down for the report (by-catch, and a
              DEBT.md entry where it qualifies), and do not do it.
              (8) A read, a run or a network call outside GRANTS is needed, or a permission is denied:
              STOP and report each, with the reason. Never work around it by another tool or route
              (METHOD rule 67).
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; there is no fourth
              kind. A citation of a library file is FETCHED with its path and, for a page, the line of
              your own text conversion (Item 1). Read docs/sources.md before anything else is cited. No
              network (GRANTS). (METHOD rule 50.)
GATED:        yes: golden/buryat/ (new files only: three CSVs, their three .meta.yaml files and
              golden/buryat/README.md), under accepted spec 001 (§6.2, the golden format; §10, M0). Also
              this round's own docs/specs/013-buryat-sources.md, its docs/specs/prompts/013-buryat-sources.md
              and its row in docs/specs/INDEX.md, which are not changes to an accepted spec. Not gated,
              and changed only as the items say: DEBT.md, FORKS.md, docs/ROADMAP.md, docs/BACKLOG.md,
              docs/sources.md.
GRANTS:       Reads outside WORKDIR, beyond this prompt file: /home/kitaeza/zurkhai-papers/, read only,
              with the Read tool (images included), ls, stat, sha256sum, grep, find, pdfinfo, pdftotext
              and pdftoppm. pdftotext and pdftoppm write only into a `mktemp -d` directory.
              Network: none. No WebSearch, no WebFetch, no curl, wget, git fetch, gh or python
              networking.
              The Agent tool: at most one delegated reader per source, for Item 3(c) and Item 4(c) only.
              Its brief carries this GRANTS line verbatim (CLAUDE.md hard rule 11; METHOD rules 66, 67),
              names the exact renderings it reads, and forbids it every other file of the library and
              of the repository. Its output is a file in your own `mktemp -d` directory.
              The tools, in the repository:
                - the Read tool;
                - git, read-only (status, log, show, rev-parse, rev-list, grep, diff), and what the
                  round's own branch and commits need (switch -c, add, commit);
                - `python3 -I`, running only scripts you write into a `mktemp -d` directory;
                - tools/gate.sh, as its own header says (Item 7);
                - read-only shell tools on the repository and on your own scratch: cat, head, tail,
                  sed -n, grep, awk, cut, tr, sort, uniq, wc, diff, cmp, ls, stat, find, date, file,
                  sha256sum, iconv;
                - mkdir -p, cp and rm -r, only inside `mktemp -d` directories.
              Files in the repository are edited only with the Edit and Write tools, never by a script or
              a shell redirection; a CSV a script produced in scratch enters the repository by the Write
              tool, and is then `cmp`-ed against the scratch file. On this machine `grep` is ugrep: keep
              bounded repetitions below 100.
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive
              widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 013 — Buryat sources: the admission of three datsan calendars, and their tshes as golden rows

Spec 012 checked who publishes each calendar in the paper library and on what terms; the human then
saved the pages it could not reach, and ruled on admission (R4 to R8). This round records those rulings,
re-verifies the evidence they rest on from the saved copies, brings docs/sources.md up to date, and
transcribes the three admitted calendars, the Ivolginsky datsan's of 2021 and the Aginsky datsan's 2025
and 2026 editions, into golden/buryat/ as 001 §6.2 defines, each by two passes and under the checks the
MODEL line names. It is the M0 round (001 §10). It compares no Buryat source with any engine system or
fixture (that is the Q1 verdict round's), writes no engine code, and contacts no one.

THE HUMAN'S RULINGS this round carries (2026-11-12). Each is quoted in the report beside the record that
implements it.
  R1. Spec 012's Q1 (the human: "Agree with all recommendations, go merge"): the human saves the pages of
      spec 012's link table and the register extracts into the library. How it was done (RELAYED, the
      advisor, 2026-11-12): once the hosts were reachable, the advisor fetched the pages of ARTIFACT (B)
      with curl, each with its response headers, and the human copied them into the library; the EGRUL
      extract the human obtained from the tax service. Not done for the items of R9.
  R2. Spec 012's Q2 (same words): the advisor reads the saved copies and drafts the admission forks; the
      human rules; this round carries the rulings and the new docs/sources.md rows, and re-verifies the
      evidence (Item 1).
  R3. Spec 012's Q3 (same words): spec 012's Latitude 1 to 4 are accepted as reported; no grant after
      the fact. Recorded in the report only.
  R4. What shows Sangha membership (001 §1) (the human: "Agree with all recommendations, go"): the
      publisher names the Buddhist Traditional Sangha of Russia as its own body, on the calendar or on its
      site, and the source's meta file quotes that line. Register evidence is added later (R9). On the
      advisor's reading (RELAYED, Item 1 re-verifies): the Ivolginsky, Aginsky and Gunzechoinei datsans
      meet it; the Khoymorsky datsan and «Ринпоче Багша» do not.
  R5. The Ivolginsky datsan's 2021 calendar (same words). Its site's footer allows the use of the site's
      material for non-commercial activity and requires the written permission of the datsan or the
      named author in all other cases (Item 1 quotes it). Ruled: admitted now, facts only, its meta's
      `rights` quoting that footer verbatim; the human writes to the datsan to ask for written
      permission; a refusal is the TRIGGER to withdraw its rows with an in-place `[CORRECTED …]` record.
      The weakness, carried in its DEBT.md entry: rows already pushed stay in the public history. Tier T1
      (F29).
  R6. The Aginsky datsan's 2025 and 2026 editions (same words): admitted, tier T1, one file per edition
      (F30). No terms of reuse were found; each meta's `rights` says so and names exactly what was read:
      the four saved site pages of ARTIFACT (B) and the text layers of both PDFs. The site's «Лунный
      календарь» page was not saved and is not part of that denominator.
  R7. The Gunzechoinei and Khoymorsky calendars (same words): no golden rows this round. The library holds
      one to three legible months of each, and the Khoymorsky poster's publisher is unsettled (DEBT.md
      D-32 (5)). A DEBT.md entry, TRIGGER: a full printed calendar of either is in the library.
  R8. The golden `month` column for a Buryat source (the human: "Agree"):
      - A lunation begins on the first day whose tshes is smaller than the previous day's tshes, and runs
        to the day before the next such day. (So a lunation whose tshes 1 is skipped begins on its 2.)
      - `month` is the number the source's printed label implies, in the seasonal order: «Первый
        весенний» 1, «Второй весенний» 2, «Третий весенний» 3, then summer 4 to 6, autumn 7 to 9, winter
        10 to 12. The meta file states that order, and it is checked inside the source: the source's
        Сагаалган line falls on the first day of the lunation labelled «Первый весенний».
      - The printed label goes into `notes` on the lunation's first day.
      - A lunation labelled «Дополнительный …» has `is_leap_month` true (F31) and an empty `month`, with
        a note that the source prints no number for it; the Q1 round maps conventions.
      - Where the source prints no label for a lunation (the Ivolginsky calendar throughout, and any
        edge lunation of an Aginsky edition), `month` and `is_leap_month` are empty, with a note.
      - Every label is assigned to exactly one lunation; the check of Item 5(d) must fail on a planted
        shifted label.
  R9. The human's "could we add this to debt" (2026-11-12): what is still missing becomes a DEBT.md
      entry (Item 0), and D-32 is RETIRED, superseded by it.
  R10. Under "Agree with all recommendations": one BACKLOG.md entry for the treatise the Aginsky datsan's
      about page names (Item 0).
  R11. Under the same words: the human's instruction of 2026-11-12 that the advisor's clear line names the
      starter file and its sha256 instead of reprinting the starter. It contradicts METHOD rule 9's "ONE
      message" and docs/ADVISOR.md §8 r4's mechanical check. This round files it as a METHOD entry in
      DEBT.md only, and edits neither file: process work stays off a round that writes golden/
      (docs/ADVISOR.md §5).

THE RESEARCH LEGS — copied from 003 §4's P3 row, as written there:
  Legs: ① **reference**: datsan publications, and a зурхайч where reachable. ② **field**: other
  implementations (Lotsawa House, Henning's software, existing Mongolian apps); pin where their
  conventions differ. ③ **science**: Janson, Henning, Schuh, Reingold–Dershowitz, or an explicit "none
  relevant". ④ **Does this contradict anything already shipped?** Every leg is reported, including empty
  ones, with what it found against the approach.
  This round is leg ① for the three calendars. Leg ② is expected to be "none relevant" (no other
  implementation transcribes these calendars). Leg ③: Janson v1's convention for which of two lunations
  bearing one number is the leap month (find it in the paper; RELAYED, the advisor's reading: p. 11),
  set beside the Aginsky label «Дополнительный зимний месяц»; report the two conventions side by side and
  map nothing (R8). Leg ④: whether golden/buryat/ as built contradicts 001 §6.2, F9, F31,
  golden/LICENSE.md or golden/reference/README.md's rules for every file.

ITEM 0 — Branch, prompt, number, records.
  - Check PRECONDITION and ARTIFACT, and quote every output.
  - Create spec/013-buryat-sources from main at 01f4d09. The head block's branch line names that commit
    and its tree, and the head block carries `arc: Sources` after `prompt:` and a `golden:` line.
  - Commit this file, byte-identical, as docs/specs/prompts/013-buryat-sources.md. That is the first
    commit. `cmp` it against /home/kitaeza/zurkhai-prompts/spec-013-buryat-sources.md before every later
    commit.
  - Verify that 013 is free in the tree. Read the Read-first list, and list it in the report.
  - Run `date` once and use that value for every date you write (METHOD rule 51). If a later `date`
    shows a new day, the records written after it carry the new date, and the report says where.
  - Choose your round's marker (METHOD rule 58). The report calls it "the round's marker" and never
    spells it.
  - Re-measure the OPEN count in FORKS.md with the grep its header defines. The advisor counted 5 on
    main at 01f4d09 (RELAYED). Re-measure it; do not copy it.
  - FORKS.md: five new rows, numbered from the next free F id (derive it; the advisor expects F32 to
    F36, RELAYED), each filed already RULED, in the register's shape. Fields, one per line as given:
      row: R4
        subject: What shows that a datsan belongs to the Buddhist Traditional Sangha of Russia
        Raised: 2026-11-12, by the advisor, after spec 012; recorded by spec 013 Item 0
        Question: Spec 012 reached no register and no Sangha list. What counts as proof of membership
          (001 §1)?
        Options: (a) the publisher names the Sangha as its own body, on the calendar or its site, the
          line quoted in the meta file, register evidence added later; (b) only a register entry or the
          Sangha's own list, every holding a candidate until then; (c) (a) and a second, independent
          confirmation.
        Status: RULED
        Ruled: 2026-11-12, (a). The human: "Agree with all recommendations, go".
        Built in: spec 013 (the meta files of golden/buryat/)
      row: R5
        subject: The admission of the Ivolginsky datsan's 2021 calendar under its site's terms
        Raised: 2026-11-12, by the advisor, after spec 012; recorded by spec 013 Item 0
        Question: The site allows its material for non-commercial activity only and requires written
          permission otherwise; golden/ is CC BY 4.0. Admit it?
        Options: (a) admit, facts only, the footer quoted in `rights`; (b) ask the datsan first, no rows
          until it answers; (c) do not admit; (d) (a) now, and the human asks the datsan, a refusal
          being the TRIGGER to withdraw the rows.
        Status: RULED
        Ruled: 2026-11-12, (d). The human: "Agree with all recommendations, go". The DEBT.md entry is
          <the id Item 0 gives it>.
        Built in: spec 013
      row: R6
        subject: The admission of the Aginsky datsan's 2025 and 2026 editions, no terms found
        Raised: 2026-11-12, by the advisor, after spec 012; recorded by spec 013 Item 0
        Question: No terms of reuse were found on four saved site pages or in either PDF's text layer.
          Admit both editions?
        Options: (a) admit both (T1, F30), `rights` saying no terms were found and naming what was read;
          (b) wait until the site's «Лунный календарь» page is saved and read.
        Status: RULED
        Ruled: 2026-11-12, (a). The human: "Agree with all recommendations, go".
        Built in: spec 013
      row: R7
        subject: Golden rows from the Gunzechoinei and Khoymorsky calendars
        Raised: 2026-11-12, by the advisor, after spec 012; recorded by spec 013 Item 0
        Question: The library holds one to three legible months of each. Transcribe them now?
        Options: (a) no, a DEBT.md entry with the TRIGGER "a full printed calendar in the library";
          (b) the legible months of Gunzechoinei only; (c) the legible months of both.
        Status: RULED
        Ruled: 2026-11-12, (a). The human: "Agree with all recommendations, go". The DEBT.md entry is
          <the id Item 0 gives it>.
        Built in: not built; DEBT.md
      row: R8
        subject: What the golden month column holds for a Buryat source
        Raised: 2026-11-12, by the advisor, after spec 012; recorded by spec 013 Item 0
        Question: 001 §5.1 makes month a number 1 to 12; the Buryat calendars print a season label per
          Gregorian month, or none. What does the column hold?
        Options: (a) the number the label implies in the seasonal order, by lunation, the label in notes,
          «Дополнительный» leap with an empty month, empty where nothing is printed; (b) empty for every
          Buryat source, the labels in notes only; (c) normalised to Janson v1's numbering.
        Status: RULED
        Ruled: 2026-11-12, (a). The human: "Agree".
        Built in: spec 013
    The "row:" names are this prompt's labels for the rulings, not ids; each row takes its F id.
  - DEBT.md: five new entries, numbered from the next free D id (the advisor expects D-33 to D-37,
    RELAYED), Pinned by: not pinned, Status: OPEN, each Statement short (DEBT.md's own rule):
      (a) PROVENANCE — What the 2026-11-12 saves did not reach. Statement: Sangha membership rests on the
          publishers' own statements (R4) until these are in the library: EGRUL extracts, searched by
          name, for the Ivolginsky datsan «Хамбын Хурээ», the Aginsky datsan «Дэчен Лхундублинг», the
          Gunzechoinei datsan, the Khoymorsky datsan and the Central Khurul of Elista; the Ministry of
          Justice's register of religious organisations; the Sangha's own site, sangharussia.ru (named
          in the infobox of sangha-secondary/ruwiki-BTSR.html; not tried); the official text of the
          Civil Code, part 4; the Swiss commercial register's (zefix) extract for Rabten Choeling; and
          the Aginsky site's «Лунный календарь» page (R6's denominator). TRIGGER: the first of: the
          human saves any of them into the library; the q1-verdict prompt is written.
      (b) COVERAGE — No golden rows from the Gunzechoinei and Khoymorsky calendars (R7). TRIGGER: a full
          printed calendar of either is in the library.
      (c) PROVENANCE — The Ivolginsky rows stand on the human's acceptance of R5. Statement: R5's text,
          with the footer's own words cited by path and line; the human writes to the datsan for written
          permission; rows already pushed stay in the public history even if withdrawn. TRIGGER: the
          datsan's answer, recorded by the human; on a refusal, the rows of
          golden/buryat/ivolginsky-datsan-2021.csv are withdrawn with an in-place `[CORRECTED …]` record.
      (d) METHOD — R11. Statement: METHOD rule 9 ("The clear line and the starter prompt for the fresh
          session are ONE message") and docs/ADVISOR.md §8 r4's mechanical check disagree with the
          human's instruction of 2026-11-12 that the clear line names the starter file and its sha256
          instead. TRIGGER: the next Process round (it revises ADVISOR §8; METHOD changes only by an
          amendment spec).
      (e) COVERAGE — whether any test of the gate reads golden/buryat/. Check it (`git grep` over
          crates/ and tools/ for `golden/buryat` and for any test that walks golden/), and file this
          entry only if none does: the format, the step check and the meta presence of golden/buryat/
          were checked by this round's scripts, outside the tree, and nothing re-runs them. TRIGGER: the
          q1-verdict round, or the next round that writes golden/buryat/, whichever first. If a test
          does read it, file nothing, report what reads it, and expect its count in Item 7.
    D-32: Status RETIRED, with your date and the reason "superseded by <(a)'s id>: the pages spec 012
    could not reach were saved on 2026-11-12 (spec 013 R1), and what is still missing is (a)'s".
  - docs/BACKLOG.md: one new entry, numbered from the next free B id (the advisor expects B-5), with a
    Revisions line:
        name: «Онцар Шелжи Мелон», the treatise the Aginsky datsan's about page names
        Origin: 2026-11-12, advisor conversation (from the saved page
          aginsky-datsan/saved-2026-11-12/agin-o-daczane.html)
        Would buy: a named astrological treatise (1908, by Габжа лама Лубсан Бальжир of the Aginsky
          datsan) that the page says lamas of Mongolia and Russia use today for zurkhai calculation: a
          candidate primary statement of which arithmetic Buryat calendars compute, for the Q1 verdict.
        State: idea
        Evidence date: 2026-11-12
    Quote the page's own sentence (at most two) in the report, with your text conversion's line.
  - The open-forks audit: the count expected at your final commit is the count you measured at the
    base, plus one for each STOP AND ASK row STOP RULE (2) or (3) files; the five new rows are RULED and
    add nothing. Show the arithmetic at every commit.

ITEM 1 — Re-verify the evidence the rulings rest on (R2). Before any transcription.
  For each saved page of ARTIFACT (B) that this item names: convert it to text in a `mktemp -d`
  directory by one script you write (the charset from its `.headers` file's content-type, or from the
  page's own meta charset; the Ivolginsky page is windows-1251 by the advisor's reading, RELAYED; drop
  script and style elements, then tags; unescape entities; one text line per non-empty line). Record
  the script verbatim in the report. Then find, quote (at most two sentences) and cite by your
  conversion's line number each of the following. Each was found by the advisor on 2026-11-12 (RELAYED;
  the advisor's line numbers come from its own conversion and need not match yours):
  (a) Ivolginsky, ivolg-home.html: the footer's sentences on the use of the site's material (the advisor
      read: «Использование материалов с сайта разрешено для некоммерческой деятельности. Для всех других
      случаев требуется письменное разрешение дацана или указанного автора.»); the legal name «Местная
      религиозная организация буддистов Иволгинский дацан «Хамбын Хурээ»…»; and the line «Центр
      Буддийской традиционной Сангхи России» (R4).
  (b) Aginsky, the four pages: the line «Буддийская Традиционная Сангха России» on each (R4); and that
      none of the four states terms for reusing the site's material (R6). Show the grep that shows the
      absence (at least: использ, разреш, копир, ©, права, лиценз, соглашен), and say what the «©» line
      and the «Пользовательское соглашение» link lead to. Also the text layers of both PDFs (`pdftotext
      -layout`): the Sangha heading, and no terms (the same grep). Also the about page's sentence on
      «Онцар Шелжи Мелон» (for B-5).
  (c) Gunzechoinei, the four pages: the attribution line («При использовании любых материалов …»), and
      every line naming the Sangha. Then the R4 check: read the calendar covers in the library
      (gunzechoinei-datsan/dazanspb/, dazanspb2024/, dazanspb2026/: the images, not readings/) for a
      line naming the Sangha as the publisher's body (the advisor read «БУДДИЙСКАЯ ТРАДИЦИОННАЯ САНГХА
      РОССИИ / Дацан Гунзэчойнэй» on a 2025 cover on 2026-11-11, RELAYED). Report found or not found;
      no golden rows either way (R7).
  (d) Khoymorsky, dharma-oferta.html: the seller's identity line, and clause 8.3 with the sentence
      before it (the advisor read: electronic goods for personal non-commercial purposes only; no
      passing on, no posting online, no copying for distribution). Say whether the clause names the
      shop's product images. No golden rows (R7).
  (e) «Ринпоче Багша»: rb-o-datsane.html's footer sentence, compared with line 137 of the library's
      datsan-about/o-datsane.txt by a script, character by character after collapsing whitespace (the
      advisor found them equal, 230 characters, RELAYED). The EGRUL extract (`pdftotext` into scratch,
      then grep; never copy a line naming a natural person or a personal ИНН): the full name of the
      organisation, the ОГРН and its date, the КПП, the registration before 1 July 2002 (number, date,
      authority), whether a termination record exists, and whether any line names a founder or a
      centralised religious organisation (grep: учредител, централиз, в состав, Сангх). Compare with
      docs/sources.md's row and spec 012 §1, which give "КПП 032301001" and "ОГРН 2020300001181" from
      the site and call 1020300001171 a certificate number: state what the extract says each number is.
      F22 is unchanged (STOP RULE (4)).
  (f) Kalmyk Central Khurul, the two pages: every © line, and whether any line names a Buddhist body.
  (g) Rabten Choeling, the two pages: the legal-form line with CH-550-0167454-3, and every © or reuse
      line.
  (h) sangha-secondary/ruwiki-BTSR.html (secondary, a Wikipedia page as saved; its `.headers` file
      gives a last-modified date): the infobox's site (sangharussia.ru); the sentence on «параллельных
      буддийских организаций» that names the «Центр тибетской культуры» built by Еше-Лодой Ринпоче
      (whether that centre is «Ринпоче Багша» is the advisor's INFERRED reading only: say what the page
      itself says); and whether the page's list of datsans is a list of the Sangha's members or a
      navigation list of Buddhism in Russia (the advisor read the latter, RELAYED).
  (i) law-switzerland/: each PDF's "Stand am" line (`pdftotext`, first page). Nothing more is read this
      round; no provision is quoted.
  If any of (a), (b) or the PDFs' text layers fails as STOP RULE (2) describes, that source goes to a
  STOP AND ASK; the rest continue.

ITEM 2 — docs/sources.md.
  (a) The rows of the seven holdings: add the saved-2026-11-12/ files to each Contents description (name,
      what it is, sha256); add the new manifest and file count of ARTIFACT (C) beside the 2026-11-11 one,
      each dated, and say that the 2026-11-11 one still holds over the older files (ARTIFACT (C)).
      Obtained: "2026-11-12, fetched by the advisor with curl and copied into the library by the human;
      the EGRUL extract obtained by the human (RELAYED, R1); the files' modification times from `stat`,
      not retrieval dates".
  (b) Licence cells: Ivolginsky — Item 1(a)'s quotation, and "spec 013: admitted under R5 (FORKS.md
      <R5's id>; DEBT.md <its id>)". Aginsky — "no terms found", with Item 1(b)'s denominator, and
      "admitted under R6". Gunzechoinei, Khoymorsky, Kalmyk, Rabten, «Ринпоче Багша» — add Item 1's
      findings; 001 §8 stays as it is for them.
  (c) Trusted-for cells: Ivolginsky and Aginsky — "Buryat practice, T1 (F29, F30); golden/buryat/<file>"
      once Item 6's files are committed. Gunzechoinei and Khoymorsky — "Nothing yet; no golden rows (R7,
      DEBT.md <id>)". «Ринпоче Багша» — unchanged (F22 (b)). Kalmyk and Rabten — unchanged.
  (d) The Source cells: one sentence each with Item 1's membership finding under R4, its strongest
      citation, and for «Ринпоче Багша» the EGRUL facts of Item 1(e) (no personal data).
  (e) Two new library rows, in the table's shape: `law-switzerland/` (SR 231.1 and SR 241, German, the
      "Stand am" lines of Item 1(i); obtained 2026-11-12 by the advisor from the official URLs of spec
      012's link table, copied by the human, RELAYED; Licence: an official publication of federal law,
      as found, quoted only if the PDF states it; Trusted for: the statutes' text at that version, for
      a later round; nothing read this round) and `sangha-secondary/` (the Wikipedia page as saved,
      obtained the same way on 2026-11-12, RELAYED; Trusted for: nothing as evidence; a pointer only,
      secondary).
  (f) The Bibliography's "Buryat golden sources" row: name the two admitted holdings and their golden
      files. Change nothing else in the Bibliography.

ITEM 3 — The Ivolginsky calendar, 2021: golden/buryat/ivolginsky-datsan-2021.csv.
  The PDF is one A2 page whose text layer holds no calendar text (spec 012: 239 bytes; re-measure). Its
  days are read from renderings.
  (a) Render: `pdftoppm` the page into your `mktemp -d` directory at a resolution where every printed
      number reads clearly (state it), and crop one image per Gregorian month and one of the panel that
      lists the year's hurals (the advisor read there «2 февраля 2022 г. — Сагаалха», RELAYED). Do not
      use the library's crops/ or overview-1.png: make your own. Record the commands.
  (b) Pass 1, by you: for each Gregorian month, one raw line per day, in the page's order: the Gregorian
      day and the tshes number printed beside it. Note every cell whose reading is uncertain, before any
      comparison. Note the weekday column each day sits in (Item 5(c)).
  (c) Pass 2, by one delegated reader (GRANTS), blind: it reads only your crops, never pass 1, never the
      library's readings/, and writes the same raw format into a file in your scratch. Its brief is
      recorded verbatim in the report.
  (d) Compare the two passes by a script, cell by cell, not by eye. Settle every disagreement against
      the rendering, at a higher resolution if needed, and record each: date, pass 1, pass 2, final, and
      the crop it was settled on. Never settle by majority with anything else.
  (e) Only after (d)'s result is committed (in the report, with the CSV): compare the final reading with
      the library's ivolginsky-datsan/readings/ files (the advisor's and a delegated reader's, made
      2026-11-11: not a source). Report agreement and every difference; a difference is re-read on the
      rendering, and a changed cell is recorded as a correction of your own with its reason. This is a
      third witness, never the authority.
  (f) Build the CSV by a script from the final raw lines: one row per printed day, from the first to
      the last day the calendar prints (expected: 2021-01-01 to 2022-02-02, RELAYED), in date order.
      Columns, exactly 001 §6.2's: gregorian_date,tshes,month,is_leap_month,is_doubled,skipped,notes.
        - is_doubled: true on the second of two consecutive days with the same tshes (001 §5.1), else
          false.
        - skipped: the tshes number omitted immediately before the day (F9 (b)); the calendar prints no
          skip marker (check it), so every value is derived from the neighbouring days, which the meta
          says; empty on the first row, with a note.
        - month and is_leap_month: empty on every row (R8: the calendar prints no month label; check it
          on the whole page, and report what you find); one note in the meta's `notes` covers every row,
          not repeated per row.
        - notes: empty unless a row needs one.
  (g) The meta file, golden/buryat/ivolginsky-datsan-2021.meta.yaml, one `key: value` per line, these
      keys in this order (001 §6.2's fields and golden/reference/README.md's): tier (T1), source (the
      full citation: title, year name as printed, compiler as printed, publisher's legal name as its
      site gives it, the URL from docs/sources.md, the library path), publisher, datsan, lama (the
      compiler as printed), location (the library path and "page 1"), sha256 (of the PDF), retrieval
      (the date and route docs/sources.md gives), edition_year, membership (R4: the quoted line, its
      file and line), method (both passes, their routes, the comparison, the settlements' count, the
      third-witness result), transcribed_or_generated_by (spec 013, the model as the session reports
      it), date, covers (first and last date, the row count, "every printed day"), rights (R5: the
      footer quoted verbatim, the PDF's own lack of a notice, "the project's use is non-commercial;
      the rows are facts only; admitted under FORKS.md <R5's id>, DEBT.md <id>"), licence (CC BY 4.0,
      with golden/LICENSE.md's sentence that it grants no right a source's terms withhold), notes
      (R8's empty month, F9's derived skipped, the digital original not a scan (F29), every reading
      made, and "Discrepancies between the passes:" with the count).
  Any field value holding a comma uses ";" instead, and the README says so.

ITEM 4 — The Aginsky calendars: golden/buryat/aginsky-datsan-2025.csv and aginsky-datsan-2026.csv.
  Each PDF is one A2 page with a text layer: per Gregorian month a heading line with the month and its
  label, then rows «Григорианский», «Лунный», «Дни недели» and «Символы дня» (RELAYED, the advisor's
  reading; in the 2026 edition the text layer splits some labels across two lines, RELAYED). The day
  symbols (Б, Д, М, Т and others) are L2 matter: not transcribed.
  (a) Route A: `pdftotext -layout` into scratch; a script parses each month's Gregorian, lunar and
      weekday rows and its label into raw lines (date, tshes, weekday as printed, label of its block).
      Record the script verbatim.
  (b) Route B: `pdftoppm` renderings, cropped per month; read by you or by one delegated reader
      (GRANTS), blind to route A's output: the Gregorian day, the tshes, the printed weekday, and the
      label, in the same raw format.
  (c) If route B is delegated, its brief is recorded verbatim. If you read it yourself, say so, and that
      you had not opened route A's output for that month.
  (d) Compare A and B by a script, cell by cell. Settle each disagreement against the rendering; record
      each as in Item 3(d).
  (e) Only after (d), the third witness: the library's aginsky-datsan/aga/text.txt and the
      aga-20NN-textlayer.txt files are the advisor's derivatives of the same PDFs, not readings; compare
      and report, as in Item 3(e).
  (f) Build each CSV as in Item 3(f), with R8 in full:
        - find the lunations by R8's first rule, across the whole edition;
        - establish, from the edition itself, the rule by which a printed label names a lunation (for
          example: the lunation holding most of the Gregorian month's days, or the one that begins in
          it; test every candidate against every label; do not take either as given), state the rule
          that holds for every label in the meta and the report, with the counts;
        - month from the label by R8's seasonal order; «Дополнительный …» gives is_leap_month true and
          an empty month; a label naming any other month gives is_leap_month false; an edge lunation
          with no label gives both empty, with a note;
        - the label as printed in notes on each lunation's first day;
        - the Сагаалган line: find it in the edition's own text (the advisor read «1 марта –
          Сагаалган» in the 2025 edition, RELAYED) and show that it falls on the first day of the
          lunation labelled «Первый весенний». If it does not, STOP RULE (3).
        - F30: on every day of 2026-01-01 to 2026-02-17 where the two editions disagree (Item 5(e)),
          a note in both files naming the other edition's value; neither is corrected.
  (g) Each meta file as in Item 3(g), with: tier T1 (F30), the compilers as printed, the edition's
      first and last date, membership (R4: the Sangha heading of the PDF and of the site pages, quoted
      with file and line), rights (R6: "no terms of reuse found", the denominator of Item 1(b), and
      that the «Лунный календарь» page was not read), the label rule and R8's seasonal order, the
      source's own convention for the extra month (F31: which lunation it names «Дополнительный», and
      Janson v1's convention beside it, leg ③), and the Сагаалган check.

ITEM 5 — The checks. Each runs by a script over the committed CSV (re-read from the repository, not
  from scratch), and each first shows it can fail on a planted defect in a scratch copy (METHOD rule
  31); report the planted defect, the failing output, then the real run.
  (a) The pass comparison of Items 3(d) and 4(d): plant one wrong tshes in a copy of one pass; the
      script reports exactly that cell.
  (b) The tshes step check (F28; 001 §6.3: "a 30-slot cycle with skips and doubles"): for each pair of
      consecutive rows, the next tshes is the previous plus 1, or plus 2 (one skipped), or equal (a
      doubled day, never twice in a row), all modulo 30 with 30 followed by 1. Plant a step of +3; it
      fails. Report per file the counts of doubled days and of skipped days, and per lunation its first
      and last tshes.
  (c) The weekday check: every row's date gives the weekday the calendar prints for it (Ivolginsky: its
      column; Aginsky: the «Дни недели» row). Plant a one-day shift in one month; it fails.
  (d) The label check (Aginsky, R8): every label is assigned to exactly one lunation, by the rule
      Item 4(f) established, and no lunation gets two. Plant a label moved to the neighbouring month;
      the check fails.
  (e) The overlap of the two Aginsky editions, 2026-01-01 to 2026-02-17: per day, equal or different,
      and the count of each. The disagreements are recorded (F30), never corrected.
  (f) The CSV format (golden/reference/README.md's rules for every file, which this round adopts for
      golden/buryat/): UTF-8, LF, one header line, no quoting, ISO dates, `true`/`false`, the column
      list exactly 001 §6.2's; each CSV has its meta; each meta has every key of Item 3(g).
  Record every script verbatim in the report.

ITEM 6 — golden/buryat/README.md.
  The formats of this directory, in golden/reference/README.md's shape: the columns and their meaning
  (001 §6.2; is_doubled per 001 §5.1; skipped per F9 (b); month and is_leap_month per R8 and F31, with
  the seasonal order); the meta keys; the rules for every file (Item 5(f)); the three files, each with
  its tier, publisher, first and last date and row count; and a coverage table: per file, the first and
  the last date, the days, the gaps (none expected), the lunations, the doubled and skipped days.
  Then M0's done-when, 001 §10 ("≥ 12 months of Buryat tshes data from ≥ 2 independent T1–T3 sources"),
  set beside these files: the sources, their months each, and whether the two datsans are independent
  as far as the files show (different datsans, different compilers, no common dates). State it as a
  measured comparison; whether M0 is met is the human's to say at the landing.

ITEM 7 — The close.
  - Run tools/gate.sh once, with nothing in flight, in both profiles (CLAUDE.md hard rule 12; METHOD
    rule 65), on the last commit that changes more than the round's spec file, its INDEX.md row and its
    docs/ROADMAP.md row (docs/ROUNDS.md §3). Compare its count with spec 012's recorded count (RECALLED),
    and say whether any leg read golden/buryat/ (Item 0's (e)).
  - In the final commit: set docs/ROADMAP.md's buryat-sources row to LANDED, with a Revisions line; add
    the round's row to docs/specs/INDEX.md: 013, `013-buryat-sources.md`, "Buryat sources: the admission
    of three datsan calendars, and their tshes as golden rows", draft; finish the report. Show that `git
    diff --name-only <the gated commit> <the tip>` lists only those three paths.
  - Write the squash commit message. After the final commit the branch stands still: a reviewer reads
    its tip, because the round changes golden/ (docs/ADVISOR.md §6).

PREDICTIONS — written by the advisor on 2026-11-12, before this round measures anything (METHOD rule 3).
Their basis: the advisor's own readings of 2026-11-11 and 2026-11-12 (one reading of the Ivolginsky
sheet by the advisor and one by a delegated reader; its own parse of the Aginsky text layers; its text
conversion of the saved pages). All RELAYED; none is a premise. Score each HELD or FAILED, with the
measured value; a prediction scored HELD cites your own measurement, never this prompt.
  1. Every PRECONDITION and ARTIFACT check passes, the 2026-11-11 manifests included.
  2. Item 1 finds every quoted line as relayed; no STOP RULE (2) fires. The Gunzechoinei cover line is
     found. The EGRUL extract gives ОГРН 1020300001171 (17.10.2002), КПП 032601001, and no line naming a
     founder or a centralised religious organisation.
  3. The new ids are F32 to F36 (RULED), D-33 to D-37, B-5; D-32 is RETIRED; OPEN is 5 at the base and
     at the final commit.
  4. No test of the gate reads golden/buryat/, so D-37 is filed; the gate's count equals spec 012's
     (147 in each profile).
  5. Ivolginsky: 398 rows, 2021-01-01 to 2022-02-02; the step check passes on every row, with 10
     doubled days and 17 skipped days; the two passes disagree on fewer than 10 cells before settling;
     the third witness agrees on all 398 final rows.
  6. Aginsky 2025: 413 rows, 2025-01-01 to 2026-02-17. Aginsky 2026: 402 rows, 2026-01-01 to
     2027-02-06. Routes A and B agree on every tshes after settling at most a handful of cells; the
     step and weekday checks pass on every row of both.
  7. The label rule that holds for every label is "the lunation holding most of the Gregorian month's
     days"; the 2025 edition labels 2025-01-30 to 2025-02-28 «Дополнительный зимний месяц»; its
     Сагаалган line is 1 March 2025, the first day of «Первый весенний».
  8. The two Aginsky editions agree on 40 of the 48 days of 2026-01-01 to 2026-02-17 and differ on 8,
     all between 2026-01-09 and 2026-01-16.
  9. Every planted defect of Item 5 fails its check.
  10. The diff touches 15 files: DEBT.md, FORKS.md, docs/BACKLOG.md, docs/ROADMAP.md, docs/sources.md,
      docs/specs/013-buryat-sources.md, docs/specs/prompts/013-buryat-sources.md, docs/specs/INDEX.md,
      and golden/buryat/README.md with the three CSVs and their three meta files.

THE REPORT — per CLAUDE.md's report contract and docs/ROUNDS.md §4, in docs/specs/013-buryat-sources.md.
It holds:
  - the head block, with its `arc:` line and a `golden:` line (rows added, per file);
  - a report-shaped spec (docs/specs/TEMPLATE.md's header lines: Status: draft; Resolves: the five new
    rows; Milestone: M0; Governs: golden/buryat/, docs/sources.md), then the end marker;
  - then the report.
  Lead with the decisive item: one table, one row per golden file: tier, publisher, first and last date,
  rows, lunations, doubled and skipped days, disagreements between the passes before settling, the
  third witness's result, and every check of Item 5 with its planted defect and its result. Then:
    - Item 1's findings, one row per holding, each quotation with file and line, and STOP RULE (2)'s
      result;
    - R1 to R11, each beside the record that implements it;
    - the four legs, each reported, the empty ones with what was searched;
    - the label rule of Item 4(f), with the counts for every candidate rule tested;
    - M0's comparison (Item 6);
    - the predictions, scored;
    - the open-forks audit, with its arithmetic at every commit;
    - the marker greps, at every commit;
    - every other section of the contract, "What the prompt got wrong" included;
    - the diff scope, with its totals summing against `git diff --stat main`, stating the commit it was
      measured at;
    - every STOP AND ASK, if any;
    - last before the squash message: the delegated readers' briefs, the commands and the scripts,
      verbatim.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 013 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════
