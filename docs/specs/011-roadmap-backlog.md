spec: 011
prompt: docs/specs/prompts/011-roadmap-backlog.md
arc: Process
branch: spec/011-roadmap-backlog (cut from main at 0d1d2db, whose tree is 8d33ee8117a4fea5e7692a3614a2309c2b4b7c97)
merge SHA: not yet merged
pushed: no
model: Sonnet 5.5 (claude-sonnet-5-5), effort High (two sessions: the second began after a harness restart, read git status, git log and the tree before it continued; the switch point to the fallback model, Opus 5.5, was not reached and no design choice fell to it)
gate: 147 tests passed, 0 failed (debug) / 147 passed, 0 failed (release) — no failure; run on the clean committed tree at c58ccee20a6c53afcd315414413b40833caf9aad (11 legs run, 11 passed, 0 failed; the msrv leg also 147 / 0); the final commit changes docs/specs/011-roadmap-backlog.md, docs/specs/INDEX.md and docs/ROADMAP.md only, which no leg reads; skipped legs: wasm-pack (crates/tshes-wasm does not exist, 002 §7)

# 011 — The roadmap, the backlog and arcs; ADVISOR r4 and ROUNDS r4

Status: draft
Depends on: 001, 003
Resolves: F27 (the new row, filed already RULED)
Milestone: none
Governs: docs/ROADMAP.md, docs/BACKLOG.md, docs/ADVISOR.md, docs/ROUNDS.md

A records round (arc: Process). It writes down the human's rulings R1–R7 of the prompt: the roadmap, the backlog, arcs, the advisor running the human's git commands, the advisor's starter as a file, and the first client as a website. It records two latitude rulings (R8, R9), files five `DEBT.md` entries, and decides no fork.

— spec text ends here —

# Report — spec 011

Dates come from `date`, run at the top of the round: `чт 12 нояб 2026 08:56:33 MSK`. The second session ran `date` twice more (13:36:08 and 14:03:57 MSK, the same day), so every record carries 2026-11-12 and no record carries a new date. Evidence tags (docs/ROUNDS.md §4.3): MEASURED (this round), RECALLED, INFERRED, RELAYED. The prompt's own rulings are quoted from the prompt, `docs/specs/prompts/011-roadmap-backlog.md`. "The round's marker" is the one token the round wrote to itself (a placeholder it never needed in the end); the greps below name it that way and never spell it.

## 0. Precondition, read-first list, session history

**PRECONDITION** (MEASURED, quoted):

```
git -C /home/kitaeza/github/zurkhai status --porcelain            → (no output)
git -C /home/kitaeza/github/zurkhai rev-parse --abbrev-ref HEAD   → main
git -C /home/kitaeza/github/zurkhai rev-parse main                → 0d1d2dbad3ca25854991865ac26d4b75f50910b3
git -C /home/kitaeza/github/zurkhai rev-list --count main         → 12
```

Spec number: `ls docs/specs/` held 001 to 010, so 011 was free. Branch `spec/011-roadmap-backlog` was cut from main at 0d1d2db (tree 8d33ee8117a4fea5e7692a3614a2309c2b4b7c97) before any edit.

**Read first, and how much** (all MEASURED reads this round): CLAUDE.md (in full; re-read when the second session began); docs/ROUNDS.md (in full); docs/ADVISOR.md (in full); docs/specs/003-project-method.md (in full, 180 lines); docs/specs/001-init-spec.md (§4, lines 41–55; §§9–12, lines 209–250, which holds §10 and §11 in full); docs/METHOD.md rules 1, 3, 8, 9 (lines 35–114), 31 (307–321), 36 (359–365, in full), 50 (485–498), 51 (501–507), 56 (549–555), 58 (565–577), 60 (587–591), 61 (593–599), 65 (629–635) and 67 (649–655), each with its Rule, Why, Here and Cost; GATED.md (in full); FORKS.md (in full, so F4, F5, F7, F10, F11, F22 and F26 and every status line); DEBT.md (in full, so D-1, D-8, D-19, D-23 and D-24 and every heading); docs/specs/INDEX.md and docs/specs/TEMPLATE.md (in full); docs/specs/010-publication.md (lines 1–40, the head block; 135–139 and 320–328, the by-catch on actions/checkout); docs/sources.md (in full: its first section and the library table; it has rows for kalmyk-central-khurul and henning-kalacakra and none for rabten-choeling, see D-29); .github/workflows/ci.yml (in full). Beyond the list, for Item 4(c): tools/gate.sh (in full) and tools/gate-self-test.sh (by grep only).

**Sessions.** Session 1 ran Items 0 to 6 and started the gate on c58ccee; the harness then restarted and wiped the session's scratch directory, including the gate log and the two check scripts. Session 2 found the branch at c58ccee with a clean tree (`git status --porcelain` printed nothing), recreated the scripts with the same text, and ran the gate again on the same clean commit (the first run's exit status was 0, by the harness's completion notice; its counts were lost, so they are not used). Nothing from session 1 was committed unverified: every commit of session 1 was made with its checks, and session 2 started from a clean tree.

## 1. The decisive item: docs/ROADMAP.md

### 1.1 The file and its check (MEASURED)

`docs/ROADMAP.md`: an opening (what the file is; 001 §10 authoritative, the roadmap tracks order and state only; why its tables name no spec number, date or model; TRIGGERs; status replaced and never appended to; the eight arcs with their coverage and the boundary caveat; how a new row enters), Table 1 (14 rows, six cells each), Table 2 (2 rows, four cells each), Revisions (r1, then r2 for row 8).

Instrument: `check_roadmap.py` (last section). It reads the dictated rows of Items 1(b) and 1(c) from the committed copy of the prompt, not from a copy typed by the round; it checks cell counts, the eight arcs, M0–M7 or none, the `— LANDED` / `— PLANNED` ending, non-empty TRIGGERs, no date and no spec number from 004 up in Table 1; then it compares every cell with the dictated field.

**The controls, first** (METHOD rules 31, 36). Each is a scratch copy of the file with one ordinary wrong cell, as the prompt asks; the script was told nothing of the seed.

| Seeded | Expected | Observed (session 1, on the file at c66ce21 and the scripts' first text) | Observed (session 2, on the final file, with `--landed 8`) |
|---|---|---|---|
| row 6 `achieves`: "1900–2100" → "1900–2101" | exactly Table 1 row 6 achieves | `FINDING Table 1 row 6 achieves: differs from the dictated field …` and `TOTAL: 1 cell(s) with a finding` | the same single finding; 91 of 92 cells equal |
| row 11 `arc`: "Rules" → "Governance" | exactly Table 1 row 11 arc | `FINDING Table 1 row 11 arc: not one of R4's eight arcs; differs from the dictated field: expected 'Rules', got 'Governance'`, `TOTAL: 1` | the same single finding |
| (extra) row 3 `opens`: a date appended | exactly row 3 opens | `holds a date; differs from the dictated field`, `TOTAL: 1` | not repeated |
| (extra) row 7 `opens`: "(spec 010)" appended | exactly row 7 opens | `holds a spec number 010; …`, `TOTAL: 1` | not repeated |
| (extra) row 14 `TRIGGER` emptied | exactly row 14 TRIGGER | `empty TRIGGER; differs from the dictated field`, `TOTAL: 1` | not repeated |

The two required controls report exactly the seeded cell and the seeded arc and nothing else, in both sessions. The three extra controls are latitude (§10). Each fires on its own rule, so a rule that never fires is not trusted.

**The real file.** At c58ccee: `Table 1: 14 rows; Table 2: 2 rows; cells compared with the prompt: 92, equal: 92`, `TOTAL: 0 cell(s) with a finding` (14 × 6 + 2 × 4 = 92). At the final tree, with row 8 at LANDED: without `--landed` the script reports exactly one finding, `Table 1 row 8 round: differs from the dictated field: expected 'roadmap-backlog — PLANNED', got 'roadmap-backlog — LANDED'` (so it can see the change); with `--landed 8` it prints `ALLOWED DIFFERENCE (--landed 8): row 8 round: dictated 'roadmap-backlog — PLANNED', expected 'roadmap-backlog — LANDED'`, `cells compared with the prompt: 92, equal: 92`, `TOTAL: 0`. Row 12 carries F27: Item 0 filed the new row as F27, so no substitution.

**Table 2 against 003 §4's P4 row and 001 §4** (the check Item 1(c) asks for, MEASURED by reading both). 003 §4, P4: "Gate 1: the Q1 verdict (M3). Gate 2: the first public release of a client, which checks correctness, provenance display, tradition labelling, and a reviewer's sign-off." Gate 2's "what it checks" is the same words, in the same order. Gate 1: P4 says only "the Q1 verdict (M3)"; 001 §4 asks "Which computation system do Buryat datsans' calendars follow?"; the cell says "which computation system the Buryat calendars follow". The cell's "Buryat calendars" is shorter than 001 §4's "Buryat datsans' calendars" and means the same. **No difference found; STOP RULE (2) did not fire.** Gate 1's state, "NOT REACHED: M0 has no admitted source yet; D-1 holds", agrees with `docs/sources.md` (every datsan holding is "a candidate; admission and tier not ruled") and with D-1 (OPEN).

### 1.2 The arc agrees three ways, for this round itself

| Where | Reads |
|---|---|
| this prompt's `ARC:` line | `Process — the arc's label in docs/ROADMAP.md, which this round creates (Item 1).` |
| this file's head block, `arc:` line | `Process` |
| docs/ROADMAP.md, Table 1 row 8 (roadmap-backlog) | arc cell `Process`, round cell `roadmap-backlog — LANDED` (set by the final commit) |

Agreed (docs/ADVISOR.md §6, step 3, r4). The head block carries `arc: Process` on the line after `prompt:`, the first head block to carry one.

## 2. docs/BACKLOG.md and its seeds (Item 2)

`docs/BACKLOG.md`: an opening (R2's distinction, the shape of an entry, the states, stale and dropped kept, the evidence date, re-verifying before promoting), B-1 to B-4, Revisions r1. An entry is the heading (the short name) plus Origin, Would buy, State, and an Evidence date line: the four fields of R2, and the date every entry carries (the check's wording: "exactly the four fields and an evidence date").

**The greps that decided each seed** (MEASURED, over DEBT.md and FORKS.md at the base):

| Seed | Grep (case-insensitive, extended) | Hits | Covered? | Filed as |
|---|---|---|---|---|
| 1 a Telegram Mini App client | `telegram\|TG Mini\|mini app\|wasm-bindgen` | FORKS.md:28, F1's options text only | No. F1 is RULED (a) on the language; it names the Mini App among the web targets and decides no client. F27 (filed here) rules which client M5 builds first: (a) a website; the Mini App is its declined option (b), not a record that a Telegram client is a possibility. Reading: F27 decides "first", B-1 keeps a possible later second client | B-1 |
| 2 real-world Phugpa controls | `kalmyk\|khurul\|rabten\|positive control\|phugpa control\|real-world` | none | No | B-2 |
| 3 approaching the other datsans | `older edition\|written permission\|approach\|other datsan\|how .* is computed` | F4 (FORKS.md:47, 49: one datsan, Гунзэчойнэй, before or after M3); D-3 (DEBT.md:46: the name run past a lama when F4's datsan is approached) | No. Both concern the one datsan; B-3 is "beyond F4's one datsan" | B-3 |
| 4 Henning's Tsurphu lists as the variant's oracle | `oracle\|henning\|kalacakra\|karana` | D-19 (DEBT.md:174–180), F23 (FORKS.md:182–185), and unrelated hits | **My reading of D-19: not covered.** D-19 names Henning's published Tsurphu calendars as RELAYED evidence that the karana-Sun variant exists, and that his 2012 file differs from Janson v1's Table 8 in months 4, 5 and 11. It does not name them as the variant's oracle: its Statement ends "Ruling out A.2 alone does not rule out the variant", and none of its three TRIGGERs mentions an oracle. F23's Ruled line gives the same RELAYED evidence | B-4 |

Result: **no seed was covered; B-1 to B-4 are filed** (STOP RULE (3) did not fire). Every seed's fields are copied as dictated; "would buy" needed no shortening (seed 2's is exactly two sentences, the others one), so there is no second text to show. Each origin is the dictated one (seed 1) or the default "2026-11-11, advisor conversation" (seeds 2 to 4); each state is `idea`.

**The check** (`check_backlog.py`, last section; same control-first shape as Item 1). Controls: B-3's State changed to `someday` → `FINDING B-3 State: 'someday' does not begin with one of R2's five states; differs from the dictated field: expected 'idea', got 'someday'`, `TOTAL: 1`; B-3's Would buy line removed → `FINDING B-3 Would buy: field missing; differs from the dictated field …`, `TOTAL: 1`. Both exactly the seeded pair, in both sessions. The real file: `entries: 4; fields compared with the prompt's seeds: 20, equal: 20`, `TOTAL: 0` (4 × 5 = 20).

**A claim this round could not check:** B-2 says Rabten Choeling's 2026 calendar is "in the paper library" (the seed's words, RELAYED). `docs/sources.md` has no row for it (MEASURED, a case-insensitive grep over the tracked `.md` files outside `docs/specs/prompts/` printed nothing before B-2 was written), and the round had no grant to read the library. B-2 is copied as dictated; D-29 records the gap.

## 3. The rulings, each beside the sentence that implements it (METHOD rule 61)

R8 and R9 are recorded in §3.8 and nowhere else (they are not forks).

### 3.1 R1 — the roadmap

Ruling (prompt R1): "docs/ROADMAP.md holds two tables. Table 1, the plan: one row per round, landed or planned, with the columns arc | milestone | round | achieves | opens | TRIGGER. … The plan names no spec numbers, no dates and no model. Table 2, the gates: 003 §4 P4's two gates, each with its state. A status cell is replaced when the state changes, never appended to; the old state goes into a one-line entry under the file's Revisions. 001 §10 stays authoritative for what each milestone delivers: the roadmap tracks order and state only."

Implemented (docs/ROADMAP.md): "`docs/specs/001-init-spec.md` §10 stays authoritative for what each milestone delivers and when it is done; this file tracks **order and state only**." / "Its two tables name no spec number, no date and no model (the Revisions below carry dates, and the spec that made each change)." / "**A status is replaced, never appended to.** When a round lands, or a gate's state changes, the cell is overwritten, and the old state goes into a one-line entry under Revisions." Table 1's header is `arc | milestone | round | achieves | opens | TRIGGER`; Table 2's is `gate | what it checks | state | TRIGGER`; r2 is the one-line entry for row 8's PLANNED → LANDED.

What the ruling reaches and leaves alone: it reaches the two tables and the opening; it leaves 001 §10 untouched (no milestone deliverable is edited).

### 3.2 R2 — the backlog

Ruling: "DEBT.md is what is owed, FORKS.md is a decision open, BACKLOG.md is a possibility; one item is filed in only one of them. Each entry has four fields and no more: a short name; where it came from …; what it would buy, in one or two sentences; its state. States: idea → sized → promoted (naming the spec) | stale (…) | dropped (…). Stale and dropped entries are kept, never deleted. Each entry carries the date its evidence was gathered, and promoting a stale entry requires re-verifying its facts first."

Implemented (docs/BACKLOG.md): "`DEBT.md` is what is **owed** … `FORKS.md` is a decision **open** and waiting for the human. `BACKLOG.md` is something that is **possible** … **one item is filed in only one of them**." / "It has four fields and no more: a short name (the heading); where it came from …; what it would buy, in one or two sentences; its state. Each entry also carries the date its evidence was gathered." / "Stale and dropped entries are kept, never deleted" / "**Before promoting a stale entry**, re-verify its facts".

### 3.3 R3 — following the roadmap

Ruling: "At each landing three things agree: the prompt's arc, the arc in the spec's head block, and the arc of the round's row in docs/ROADMAP.md. A mismatch is a finding. The advisor checks the roadmap, the fork register and the backlog for drift at each landing. A round that is not on the roadmap gets its row, agreed with the human, before its prompt is written."

Implemented: docs/ADVISOR.md §6 step 3: "**The arc agrees three ways**: the prompt's `ARC:` line, the head block's `arc:` line, and the round's row in `docs/ROADMAP.md`, which the round's final commit set to LANDED … A mismatch is a finding." / step 7: "**The drift check**, after step 6: read `docs/ROADMAP.md`, `FORKS.md` and `docs/BACKLOG.md` on the merged `main`, and add to the adjudication a closing note that names every planned row whose TRIGGER has now fired, every gate whose state has changed, and every backlog entry the round promoted." / §5: "**The roadmap row.** A round that is not on `docs/ROADMAP.md` gets its row, agreed with the human, before its prompt is written." / docs/ROADMAP.md: "A round that is not on the roadmap gets its row, agreed with the human, before its prompt is written."

Note: the ruling says the drift check is made "at each landing"; Item 3(d) puts it as a step after the merge, which comes after the adjudication is delivered; the "closing note" wording is this round's reading (§8, fork 1).

### 3.4 R4 — arcs

Ruling: "The prompt carries an `ARC:` line and the head block an `arc:` line. The advisor assigns the arc with the spec number; the human may overrule it. A round that landed before this one gets its arc only in its roadmap row; no landed spec file is edited. The arcs, each with what it covers: …"

Implemented: docs/ROUNDS.md §2.2: "`ARC:          <one of the eight arcs of docs/ROADMAP.md>, assigned by the advisor with the spec number; the human may overrule it.`" after `SPEC NUMBER:`; §4.1: `arc: <one of the eight arcs of docs/ROADMAP.md>` after `prompt:`; docs/ADVISOR.md §5: "**The arc.** … The advisor assigns the arc with the spec number; the human may overrule it."; docs/ROADMAP.md: "A round that landed before arcs existed has its arc only in its row here; no landed spec file is edited." and the eight arcs with their coverage, as listed in R4, and "The weakest boundary is Engine against Validation (the advisor's caveat)." No landed spec file or prompt was edited (MEASURED: `git diff --name-only main c58ccee` lists none).

### 3.5 R5 — the advisor runs the human's git commands

Ruling: "The advisor runs the docs/ROUNDS.md §9 merge …, the pushes …, the branch clean-up and, once, the first public push. It runs each set only after the human's explicit word in chat for THAT set, and shows the exact commands first. A force-push to the public origin is asked a second time, separately, right before it runs. Never while an engine session is running in the engine repository … Not covered: writing file content; committing anything a round did not write; reading code (docs/ADVISOR.md §3 is unchanged). The backstop: a permissions `ask` list …"

Implemented, docs/ADVISOR.md §1, one sentence per clause: "the advisor runs the commands, on the human's word" / "**The word is per set.** The advisor runs a set only after the human's explicit word in chat for THAT set." / "**The commands come first.**" / "**A force-push to the public origin is asked twice.** It is asked a second time, separately, right before it runs." / "**Never while an engine session runs** in the engine repository: the merge checks out `main` there." / "**Not covered:** writing file content; committing anything a round did not write; reading code (§3 is unchanged)." / "**The backstop** is a permissions `ask` list in the advisor clone's `.claude/settings.local.json`, which git ignores …" with the same sixteen write commands for the engine repository and `reset` and `checkout` for the clone. §2: "In the clone it commits nothing; the only commit the advisor makes anywhere is a round's squash, in the engine repository (§1)." docs/ROUNDS.md §1 (loop and bullets) and §9 (title "The merge and the pushes (the human's word, the advisor's commands)", its opening, and step 4's "the advisor does not push and asks the human"). docs/ROUNDS.md §9 step 5 holds no "the human" to change; nothing else in §9 changed (MEASURED: the diff of §9 is the title, the opening paragraph and step 4's one clause).

Reaches / leaves alone: it reaches who runs the commands; it leaves alone what the commands are and the F11 and F26 rulings they rest on.

### 3.6 R6 — the advisor's starter is a file

Ruling: "At EVERY handoff, whether the advisor's session should be cleared or not, the advisor overwrites /home/kitaeza/zurkhai-prompts/advisor-starter.txt with the docs/ADVISOR.md §8 starter, composed from files, and shows it with `cat`. The human's shell has an alias, zadv-start, that prints that file; a fresh advisor session starts with it. The starter ends with a stale-check paragraph: main's full commit id, what the public origin's main should print, and the sha256 of every running or prompted prompt file. … METHOD rule 9 is unchanged: the clear line and the starter are still one message."

Implemented, docs/ADVISOR.md §8: "**The starter is a file, written at every handoff.** At EVERY handoff, whether the advisor's session should be cleared or not, the advisor overwrites `/home/kitaeza/zurkhai-prompts/advisor-starter.txt` with the starter below, composed from files, and shows it with `cat`. The human's shell has an alias, `zadv-start`, that prints that file; a fresh advisor session starts with it." / the template's second paragraph, "Check first that this starter is not stale. …" with the three items, and "If any of these differs, the starter is stale: say so, and rebuild the state from the files and your own notes" / "the clear line and the starter are one message, METHOD rule 9" / "The human opens a fresh session in `/home/kitaeza/github/zurkhai-advisor`, on Opus, effort xhigh, and runs `! zadv-start`." The mechanical check is reworded (Item 3(f)): "appear together only in a message that also contains the starter block …, or in exactly the second form of the last line".

### 3.7 R7 — the first client is a website

Ruling: "The human chose the website, and on 2026-10-02 registered the domain zurkhai.org for it (RELAYED). Nothing was served there on 2026-10-02 (RELAYED; not re-checked since)."

Implemented: FORKS.md F27, "### F27 — The first client", Status RULED, Ruled "2026-10-02, (a), a website. The human registered the domain zurkhai.org for it the same day. The human's words were not recorded.", Built in "not yet built; docs/ROADMAP.md's website row (spec 011)"; ROADMAP row 12 "the first client, a website at zurkhai.org (F27) (001 §10 M5)". The ruling's second RELAYED sentence ("Nothing was served there") is not copied into any file: neither the row nor the roadmap says anything is or is not served, and the round did not check (no network grant).

### 3.8 R8 and R9 — recorded

- **R8** (prompt: "Round 010's latitude (2026-11-12, 'Agree with all recommendations, go merge'): its Latitude 1–5, and the read-only or process commands it ran outside its GRANTS (kill, pkill, pgrep, timeout, sleep, git remote -v, git merge-base), are accepted as reported; no grant after the fact."): recorded here. This round took no grant from it (§10 lists the commands this round ran outside its own list).
- **R9** (prompt: "Round 009 session 3's latitude (2026-11-12, 'approve'): its Latitude 1–6, and its unlisted rewording of the head block's model line ('either:' became 'sessions 1 and 2:'), are accepted as reported; no grant after the fact."): recorded here.

## 4. The five DEBT entries, and the fetch (Item 5)

Next free D id: D-25 (MEASURED: the last heading in DEBT.md before this round is D-24). Each of (a) to (d) was searched for in DEBT.md and FORKS.md first and none was covered: `node\.js\|node 20\|deprecat\|ubuntu` → no hit; `midnight\|rule 51\|later day\|resumed on` → no hit; `INDEX\|status of a merged\|LANDED` → three unrelated hits (FORKS.md:118 in F14, FORKS.md:148 in F18, DEBT.md:137 in D-14; none about a round's status); `ROADMAP\|BACKLOG\|arc line\|human merges\|003 §11` → only F27's own Built in.

| Id | Kind | Subject | TRIGGER (summary) |
|---|---|---|---|
| D-25 | TOOLING | actions/checkout v4 runs on a deprecated Node.js 20 | a removal date from GitHub; a CI failure or warning of one; the next round that edits `.github/workflows/` |
| D-26 | METHOD | rule 51 says nothing of a round crossing midnight or resumed on a later day | the next amendment spec to docs/METHOD.md |
| D-27 | DOCS | the INDEX status of a merged round is never changed (7 rows, 004–010, say `draft`: MEASURED `grep -c '\| draft \|'` → 7) | the next spec that revises 003 or TEMPLATE.md, or the first record that reads an INDEX status as a round's state |
| D-28 | DOCS | gated texts lag the rulings of 2026-11-12: 003 §11's list; TEMPLATE.md's head block; 003 lines 23, 30, 174 (human merges and pushes; the advisor's 23, 30 and 174 confirmed; 163 does not contradict R5); plus 003:162 (INFERRED) | the next accepted spec that revises 003 or TEMPLATE.md |
| D-29 | DOCS | by-catch: no `docs/sources.md` row for Rabten Choeling (B-2's claim) | the next round that edits sources.md, or the Sources round |

All have `Pinned by: not pinned` and `Status: OPEN`. D-26's statement quotes the interim clause from the prompts: it is in the prompts of specs 008, 009, 010 and 011 (MEASURED by grep over docs/specs/prompts/), not "since spec 010" (§9, item 1).

**The fetch of Item 5(a)** (FETCHED, 2026-11-12, the one command GRANTS allows, `gh run view 43107437324 -R kitaeza/zurkhai`; it succeeded, so the RELAYED fallback text was not used). Header lines: `✓ main CI · 43107437324`; jobs `✓ build and test on latest stable (002 §2) in 40s` and `✓ gate (pinned toolchain, every leg of tools/gate.sh) in 19m54s`. The two annotations, quoted from the output (each appears under both jobs):

> `! Node.js 20 is deprecated. The following actions target Node.js 20 but are being forced to run on Node.js 24: actions/checkout@11d5960a326750d5838078e36cf38b85af677262.` (followed in the output by a link to a GitHub changelog of 2025-09-19, not read)

> `- "The ubuntu-latest label will migrate to Ubuntu 26 beginning October 19, 2026. For more information, see https://github.com/actions/runner-images/issues/14748"`

D-25's statement quotes the first in full and the second without the link. The output does not say which runner image the run used (D-25 says so). The prompt's claim that spec 010 "found v5, v6 and v7 published (RECALLED, its by-catch)" was checked against 010's text (lines 137 and 324): it says so.

## 5. The consistency read (Item 6) and the Item 4(h) grep

### 5.1 The read

docs/ROADMAP.md, docs/BACKLOG.md, docs/ADVISOR.md, docs/ROUNDS.md and CLAUDE.md were read start to end as committed at 44eb950 (MEASURED: `Read` of each file). Four contradictions were in the round's own text and were fixed in c58ccee (CLAUDE.md hard rule 19 within the round's own new text and its own edits):

1. ROADMAP: the opening said the file "names no spec number, no date" while its Status line and Revisions carry dates and "spec 011" → now "Its two tables name no spec number, no date and no model".
2. ROADMAP: the opening said the file "never restates a milestone's deliverable in other words" while its `achieves` cells paraphrase 001 §10 → that clause is removed ("order and state only", as R1 says).
3. ROADMAP: "A landed row's TRIGGER is '—'" while row 8, once LANDED, keeps "publication landed" (the prompt allows the status as the only difference) → now "The rows that had landed before this file was written show '—' …".
4. ADVISOR §1: "It never … runs a merge or a push" while §2 has the advisor run the clone's `pull --ff-only` → "in the engine repository".

One more edit is a rule-19 fix made earlier, in 44eb950, beside Item 4(c)'s edit: ROUNDS §3's "The final commit" bullet said it adds the INDEX row only, while the gate bullet beside it says the final commit changes the spec file, the INDEX row and the ROADMAP row → it now also sets the ROADMAP row to LANDED ("a halt does not").

**Flags, not fixed** (each beyond the round's own edits, or in a file the round may not change):

- F-a. CLAUDE.md hard rule 12 (line 22): "CI must be green in **both** debug and release, on the final committed tree." ROUNDS §3 (r4) and Item 7 of this prompt run the gate on the last commit before the records-only final commit. The two trees differ only in files no gate leg reads (§6), but the sentences differ. CLAUDE.md may change here only in two places (Item 4(g)), so this is flagged. No `DEBT.md` entry: a CLAUDE.md edit is in the advisor's gift, and the next round that touches CLAUDE.md can take it; recorded here only.
- F-b. ROUNDS §4.1's `gate:` line has no place for the commit the gate ran on, which §3 (r4) requires the line to name. This report's head block names it.
- F-c. CLAUDE.md's "Files a round keeps current" says an entry the prompt tells you to promote is updated "in the same commit"; the same commit as what is not said (Item 4(g)'s dictated words).
- F-d. B-1 and F27: see §2, seed 1.

### 5.2 Item 4(h): the grep

Phrases (case-insensitive): the human merges, human merges, squash-merges, pushes main, pushes or merges, commits nothing; and MILESTONE (case-sensitive). Searched: docs/, CLAUDE.md and the root `.md` files.

**At the base (main)** the non-record hits were exactly: CLAUDE.md:12; docs/ADVISOR.md:11 and 31; docs/ROUNDS.md:18 and 63; docs/specs/003-project-method.md:23, 30, 163 and 174. No hit for MILESTONE.

**After this round's edits** the non-record hits are: docs/ADVISOR.md:40 ("In the clone it commits nothing; the only commit the advisor makes anywhere is a round's squash …"), which is the round's own sentence and agrees with the other §1 and §2 sentences (not a contradiction); and docs/specs/003-project-method.md:23, 30, 163 and 174, which are gated and go into D-28 (23, 30, 174), with 163 ("The engine session … never pushes or merges") not contradicting R5. CLAUDE.md:12, ADVISOR 11 and 31, ROUNDS 18 and 63 were the round's own edits and no longer match. No hit for MILESTONE outside the prompt.

**Records** (counted by lines, listed none): docs/specs/prompts/010-publication.md 1; docs/specs/prompts/011-roadmap-backlog.md 9 (and 2 lines with MILESTONE, the grep list in Item 4(h) itself); docs/specs/prompts/004 to 009 0 each; the reports docs/specs/004 to 010 0 each.

**Listed, not edited:** FORKS.md:98, F11's Ruled line, "… after a merge's tree check the human pushes the branch there …", a record of a ruling (R5 changes who runs the push, not what F11 ruled).

## 6. Item 4(c): what the gate reads (MEASURED)

`tools/gate.sh` (read in full): a `grep -i docs` over it and over `tools/gate-self-test.sh` prints nothing. The legs: fmt, clippy, test, test_release, build_wasm, deny, msrv, no_std_check and no_alloc_check (both `grep -r` over `crates/tshes/src` only), zero_deps_check (`cargo metadata` of the workspace), self_test (a `tar` copy of the whole tree into a scratch directory, then plants driven through the legs on the copy; the copy takes `docs/` along, no leg reads it). The tests read files at two places: `crates/tshes/tests/common/fixtures.rs` (a path built on `CARGO_MANIFEST_DIR`, to `golden/`) and `crates/tshes/src/phugpa.rs:1479` (`include_str!` of `golden/reference/janson-v1-table6-epochs-806.csv`). `grep -rn -E 'docs/|ROADMAP|BACKLOG|INDEX\.md|specs/'` over `crates/` and the manifests finds two comments (`worked_examples.rs:5`, `lib.rs:12`) and no read. **No gate leg reads anything under docs/**; the final commit's three files are therefore not read by it. Prediction 8 held.

## 7. The predictions, scored (METHOD rule 3)

| # | Prediction | Score | Measured value |
|---|---|---|---|
| 1 | Both PRECONDITION checks pass | HELD | §0: clean status, `main`, 0d1d2dbad3ca25854991865ac26d4b75f50910b3, 12 |
| 2 | OPEN is 5 at the base and 5 at the final commit; the new row is F27 | HELD | `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md` → 5 at the base (lines 51, 57, 70, 76, 90 = F4, F5, F7, F8, F10) and 5 at every commit (§11); the last F id was F26, the new row is F27 |
| 3 | The new DEBT entries are D-25 to D-28; none covered | HELD, with one more | The last id was D-24; D-25 to D-28 filed, none covered (greps in §4). D-29 is a fifth, by-catch |
| 4 | Seeds 1 to 3 filed as B-1 to B-3; seed 4 as B-4 because D-19 names Henning's calendars as evidence, not as the variant's oracle | HELD | §2: B-1 to B-4; D-19 read in DEBT.md:174–180 |
| 5 | Every control of Items 1 and 2 reports exactly what was seeded | HELD | §1.1 and §2: 5 roadmap and 2 backlog controls, each reported exactly its seeded cell, in both sessions for the required ones |
| 6 | Table 2's rows agree with 003 §4's P4 row | HELD | §1.1: the same words for Gate 2; Gate 1 matches 001 §4's question |
| 7 | Item 4(h)'s grep hits CLAUDE.md:12, ADVISOR 11 and 31, ROUNDS 18 and 63, and 003 lines 23, 30, 163 and 174; 163 does not contradict R5; no MILESTONE | HELD | §5.2: `git grep` on main printed exactly those nine lines; no MILESTONE |
| 8 | tools/gate.sh reads no file under docs/ | HELD | §6 |
| 9 | The gate's count equals the last count recorded on main | HELD | 147 / 0 (debug) and 147 / 0 (release), against spec 010's recorded 147 / 0 and 147 / 0 (RECALLED, its head block's gate line). The count is not re-derived for the base: no gate was run on main |
| 10 | The diff touches 10 files | HELD (see §12) | 8 files at c58ccee, plus this file and INDEX.md at the tip = 10, the ten the prediction names |

## 8. Forks, resolved and unresolved

No STOP AND ASK. No fork was raised; FORKS.md gained only F27, filed RULED.

1. **Where the drift check runs.** Considered (a) a STOP AND ASK, because R3 says the advisor checks "at each landing" while Item 3(d) puts the check as a step after the merge and says it names its findings "in the adjudication", which is delivered before the merge; (b) wording it as a closing note to the adjudication, after step 6; (c) running it before the merge. Chose (b): the human's ruling is "at each landing" and Item 3(d) fixes the position, so what the engine does is unchanged, and per ROUNDS §6 only the advisor's own packaging is in question (a forced answer is not a fork). The advisor reads the wording at the landing.
2. **The evidence-date line.** R2 says "four fields and no more" and "each entry carries the date its evidence was gathered". The check's wording ("exactly the four fields and an evidence date") shows the date is carried beside the four. Chose a fifth line, `Evidence date:`, with the short name as the heading; considered folding the date into State, which would break the state check.
3. **Setting row 8 to LANDED while its TRIGGER stays.** The prompt allows the status as the only difference; after landing, row 8's TRIGGER still reads "publication landed". Chose to follow the prompt and reword the opening of ROADMAP so that it claims nothing the table contradicts (§5.1, item 3).
4. **What to do with the CLAUDE.md rule 12 mismatch** (F-a): considered editing it (STOP RULE (4) forbids it), chose to flag.

## 9. What the prompt got wrong

1. "Prompts since spec 010 carry an interim clause in Item 0" (Item 5(b)): MEASURED, the clause (run `date` once; "If a later `date` shows a new day …") is in the prompts of specs 008, 009, 010 and 011. D-26 says so.
2. Item 3(d) puts the drift check after the merge and has it name its findings "in the adjudication"; the adjudication (step 5) comes before the merge (step 6). Resolved in §8, fork 1.
3. Item 4(e): "In steps 4 and 5, 'the human' becomes whoever runs the step": step 5 holds no such phrase; only step 4 did.
4. CLAUDE.md hard rule 12 ("on the final committed tree") against Item 4(c) and Item 7 (the gate on the last non-record commit): two standing texts of the prompt's own world disagree (F-a). Also Item 7 says "Run tools/gate.sh once"; the harness restart made it two runs on the same commit (§0).
5. Seed 2 says Rabten Choeling's calendar is in the library; `docs/sources.md` has no row for it (D-29). The Read-first list hedged ("if present").
6. Item 5 is titled "Four DEBT.md entries"; five were filed (D-29 by-catch).
7. R2's "four fields and no more" and its evidence date: the prompt does not say whether the date is one of the four (§8, fork 2).
8. The prompt asks the report to hold "the commands and the scripts, verbatim" last, and ROUNDS §4.2 and TEMPLATE.md put the squash commit message last; this report ends with the message (as spec 010 did).

Nothing else found that a check contradicted. The dictated Table 1 and Table 2 fields were consistent with 001 §10, 003 §4 and the tree wherever they could be checked (row 12's F27; row 8's TRIGGER; D-18, D-19, D-23, D-24 exist and say what rows 4, 6, 10 cite).

## 10. Latitude taken

1. Three extra controls for `check_roadmap.py` (a date, a spec number, an empty TRIGGER; §1.1), each on its own rule.
2. D-29, a fifth DEBT entry (by-catch; §4).
3. Wording beyond the dictated text: ROADMAP's Status line goes to r2 with a Revisions entry r2 for row 8 (the prompt asks for "a Revisions line"); the closing-note wording of ADVISOR §6 step 7; "in the engine repository" in ADVISOR §1; the ROUNDS §3 final-commit bullet (§5.1, rule 19); ROADMAP's four rewordings (§5.1).
4. Extra reads: 001 §4 and §9 to §12; 003 in full; the rest of tools/gate.sh; a grep of `crates/` for reads of docs paths (read-only, to answer Item 4(c)).
5. The gate ran twice on c58ccee (§0).
6. Commands outside the prompt's tool list, none of which raised a permission prompt, all read-only or process: `sleep` and `seq` (wait loops for the background gate), `test` and `printf` (in the commit chains), `od` (the prompt file's last bytes), `env` (to look for RUSTUP_TOOLCHAIN and PROPTEST_CASES, D-11), `rustc -V`, `mktemp -d -p <scratchpad>` (the granted `mktemp -d`, with a parent directory), `git --no-optional-locks diff`, `git grep`. One read outside the repository and the round's own scratch: an `ls` of the scratchpad's parent directory in session 2, which printed the names of two session directories (one of them another session's, empty); nothing was opened. No permission was denied and no grant was worked around.
7. The squash message section is last (§9, item 8).

## 11. Open-forks audit and the marker greps, at every commit

Expected OPEN = previous OPEN count + forks raised − rulings recorded (METHOD rule 60). F27 is filed RULED: it is neither an OPEN row nor a ruling that closes an OPEN row.

| Commit | Previous | Raised | Rulings recorded on OPEN rows | Expected | Grep `'^- \*\*Status:\*\* OPEN$'` FORKS.md |
|---|---|---|---|---|---|
| 9493bc1 (prompt) | 5 (base, MEASURED) | 0 | 0 | 5 | 5 |
| c66ce21 (Items 0–2; F27 RULED) | 5 | 0 | 0 | 5 | 5 |
| 44eb950 (Items 3–5) | 5 | 0 | 0 | 5 | 5 |
| c58ccee (Item 6, the gated commit) | 5 | 0 | 0 | 5 | 5 |
| the final commit | 5 | 0 | 0 | 5 | 5 (grep run in the commit's chain) |

**Marker greps:** at each commit, `grep -rn '<the round's marker>' <repo> --include='*.md' --exclude-dir=prompts --exclude-dir=target --exclude-dir=.git` (the frozen prompt copy is excluded, as METHOD rule 58 says) printed nothing, in a chain that continues only on no match: after 9493bc1's own check, c66ce21, 44eb950 and c58ccee, and in the final commit's chain. `git status --porcelain` before each chain matched the files the commit meant to change (the exact expected listing was compared in the chain): the prompt file; FORKS.md, ROADMAP.md, BACKLOG.md; CLAUDE.md, DEBT.md, ADVISOR.md, ROUNDS.md; ADVISOR.md and ROADMAP.md; and for the final commit INDEX.md, ROADMAP.md and this file. No `proptest-regressions` seed and no other file the test framework wrote exists (§12). The prompt `cmp` against `/home/kitaeza/zurkhai-prompts/spec-011-roadmap-backlog.md` passed in every chain (sha256 50a2601e3642b0258458ac5e014c6e6152df1d37590970a6a577bd6533ed86c2).

## 12. Diff scope, totals that must sum

Measured at the gated commit c58ccee20a6c53afcd315414413b40833caf9aad, `git diff --stat main c58ccee`, `--numstat`:

| File | + | − |
|---|---|---|
| CLAUDE.md | 2 | 2 |
| DEBT.md | 40 | 0 |
| FORKS.md | 8 | 0 |
| docs/ADVISOR.md | 26 | 7 |
| docs/BACKLOG.md | 53 | 0 |
| docs/ROADMAP.md | 56 | 0 |
| docs/ROUNDS.md | 16 | 10 |
| docs/specs/prompts/011-roadmap-backlog.md | 544 | 0 |
| **8 files** | **745** | **19** |

Check: 2 + 40 + 8 + 26 + 53 + 56 + 16 + 544 = 745; 2 + 0 + 0 + 7 + 0 + 0 + 10 + 0 = 19; `git diff --stat` printed "8 files changed, 745 insertions(+), 19 deletions(-)".

**Gated paths touched: no.** At c58ccee the paths of crates/, golden/, rules/, clients/, tools/, .github/, GATED.md, docs/METHOD.md, docs/sources.md, docs/glossary.md, docs/specs/TEMPLATE.md and docs/specs/INDEX.md: 0 files (`git diff --name-only main c58ccee -- <those>` counted 0). The round's own spec file, its prompt copy and its INDEX row are not changes to an accepted spec (GATED.md). **Files the test framework wrote: none** (`git status --porcelain` printed nothing after the gate; the gate's self-test deleted its own scratch directory).

**The final commit** changes three paths, as ROUNDS §3 (r4) says: this file (new), docs/specs/INDEX.md (+1 line, the 011 row) and docs/ROADMAP.md (row 8's round cell `PLANNED` → `LANDED`, the Status line r1 → r2, one Revisions line: 57 lines in the file at the tip against 56 at c58ccee, three lines touched). `git diff --name-only c58ccee <the tip>` lists those three and no other (shown in the closing message of the session, because a report cannot hold the diff of the commit that holds it, and not here). So the tip's `git diff --stat main` is 10 files: the 8 above, this file and INDEX.md; its totals are 745 + 1 (ROADMAP) + 1 (INDEX) + this file's length insertions, and 19 deletions, the spec file's own length being the one number this report cannot hold.

## 13. Held

Nothing is held for a later round that has no identifier: the flags F-a to F-d of §5.1 are recorded here, F-a, F-b and F-c as reading notes for the next round that touches those files, and the DEBT entries D-25 to D-29 carry the rest.

## The commands and the scripts, verbatim

The round's marker is shown as `<the round's marker>`; every other command is as run. `B=/home/kitaeza/github/zurkhai`, `S` = the scratch directory made by `mktemp -d -p /tmp/claude-1000/-home-kitaeza-github-zurkhai/48ea1bb1-83f9-48bd-b492-7ee210322eb1/scratchpad r011.XXXXXX` (session 1) and `r011b.XXXXXX` (session 2), `P=$B/docs/specs/prompts/011-roadmap-backlog.md`.

```
# precondition, date
git -C $B status --porcelain
git -C $B rev-parse --abbrev-ref HEAD
git -C $B rev-parse main
git -C $B rev-list --count main
date
# the open-forks count, the prompt file's shape
grep -c '^- \*\*Status:\*\* OPEN$' $B/FORKS.md
tail -c 20 /home/kitaeza/zurkhai-prompts/spec-011-roadmap-backlog.md | od -c ; wc -l -c <that file>; sha256sum <that file>
# the branch, the prompt copy (written with the Write tool), cmp
git -C $B switch -c spec/011-roadmap-backlog main
cmp $B/docs/specs/prompts/011-roadmap-backlog.md /home/kitaeza/zurkhai-prompts/spec-011-roadmap-backlog.md
# every commit's chain (the expected listing differs per commit)
test "$(git -C $B status --porcelain)" = "<the expected listing>" && cmp $P /home/kitaeza/zurkhai-prompts/spec-011-roadmap-backlog.md && ! grep -rn '<the round's marker>' $B --include='*.md' --exclude-dir=prompts --exclude-dir=target --exclude-dir=.git && test "$(grep -c '^- \*\*Status:\*\* OPEN$' $B/FORKS.md)" = "5" && python3 -I $S/check_roadmap.py $B/docs/ROADMAP.md $P && python3 -I $S/check_backlog.py $B/docs/BACKLOG.md $P && git -C $B add <files> && git -C $B diff --cached --stat && git -C $B commit -q -m "<message>"
# the CI run (Item 5(a))
gh run view 43107437324 -R kitaeza/zurkhai
# the seed greps
grep -n -i -E 'telegram|TG Mini|mini app|wasm-bindgen' $B/DEBT.md $B/FORKS.md
grep -n -i -E 'kalmyk|khurul|rabten|positive control|phugpa control|real-world' $B/DEBT.md $B/FORKS.md
grep -n -i -E 'older edition|written permission|approach|other datsan|how .* is computed' $B/DEBT.md $B/FORKS.md
grep -n -i -E 'oracle|henning|kalacakra|karana' $B/DEBT.md $B/FORKS.md
grep -rn -i 'rabten' $B --include='*.md' --exclude-dir=prompts --exclude-dir=target -l
# the DEBT coverage greps
grep -n -i -E 'node\.js|node 20|node20|deprecat|ubuntu' $B/DEBT.md $B/FORKS.md
grep -n -i -E 'midnight|rule 51|later day|resumed on' $B/DEBT.md $B/FORKS.md
grep -n -i -E 'INDEX|status of a merged|LANDED' $B/DEBT.md $B/FORKS.md
grep -n -i -E 'ROADMAP|BACKLOG|`arc:|arc line|human merges|003 §11' $B/DEBT.md $B/FORKS.md
grep -n -i -E 'later `date`|new day|crossing midnight' $B/docs/specs/prompts/*.md
grep -c '| draft |' $B/docs/specs/INDEX.md
# Item 4(h)
grep -n -i -E 'the human merges|human merges|squash-merges|pushes main|pushes or merges|commits nothing' <the docs/, CLAUDE.md and root .md files>
grep -n 'MILESTONE' <the same files>
git -C $B grep -n -i -E '<the same phrases>' main -- CLAUDE.md '*.md' ':(exclude)docs/specs/prompts' ':(exclude)docs/specs/004-*' … ':(exclude)docs/specs/010-*'
# Item 4(c)
grep -n -i 'docs' $B/tools/gate.sh $B/tools/gate-self-test.sh
grep -rn -E 'docs/|ROADMAP|BACKLOG|INDEX\.md|specs/' $B/crates $B/Cargo.toml $B/deny.toml $B/rust-toolchain.toml --include='*.rs' --include='*.toml'
grep -rn -E 'include_str|include_bytes|read_to_string|File::open|fs::read|CARGO_MANIFEST_DIR|Command::new' $B/crates --include='*.rs'
# the gate (session 2; the first run's log was lost)
env | grep -i -E 'RUSTUP|CARGO|PROPTEST|RUSTFLAGS'    # only PATH matched (it contains .cargo/bin)
bash $B/tools/gate.sh > $S/gate.log 2>&1
awk '/^=== leg: /{leg=$3} /^test result:/{p[leg]+=$4; f[leg]+=$6; ig[leg]+=$8} END{for(l in p) print l, "passed="p[l], "failed="f[l], "ignored="ig[l]}' $S/gate.log | sort
#   → msrv passed=147 failed=0 ignored=0 / test passed=147 failed=0 ignored=0 / test_release passed=147 failed=0 ignored=0
#   → "GATE: 11 legs run — 11 passed, 0 failed, 1 skipped"; 6 toolchain lines, all rustc 1.99.0 (b940084d7 2026-09-28)
# the controls (copies in $S, one Edit each) and the final checks
python3 -I $S/check_roadmap.py $S/rm-cell.md $P --landed 8
python3 -I $S/check_roadmap.py $S/rm-arc.md $P --landed 8
python3 -I $S/check_backlog.py $S/bl-state.md $P
python3 -I $S/check_backlog.py $S/bl-field.md $P
python3 -I $S/check_roadmap.py $B/docs/ROADMAP.md $P --landed 8
python3 -I $S/check_backlog.py $B/docs/BACKLOG.md $P
# diff scope
git -C $B --no-optional-locks diff --stat main c58ccee
git -C $B --no-optional-locks diff --numstat main c58ccee
git -C $B --no-optional-locks diff --name-only main c58ccee -- crates golden rules clients tools .github GATED.md docs/METHOD.md docs/sources.md docs/glossary.md docs/specs/TEMPLATE.md docs/specs/INDEX.md | wc -l
```

`check_roadmap.py`:

```python
"""Spec 011 Item 1(e): check docs/ROADMAP.md.

usage: python3 -I check_roadmap.py ROADMAP.md PROMPT.md [--landed N]

Reads the dictated rows of Item 1(b) and 1(c) from PROMPT.md (the committed
copy of the prompt), and compares every cell of ROADMAP.md with them. Also
checks the structural rules. --landed N: row N of Table 1 is expected to read
LANDED where the prompt dictates PLANNED (the only allowed difference).
Prints one FINDING line per offending cell (all reasons for a cell on that
line) and exits 1 if there is any.
"""
import re
import sys

ARCS = ["Engine", "Validation", "Sources", "Verdict", "Rules", "Clients",
        "Outreach", "Process"]
MILESTONES = ["M%d" % i for i in range(8)] + ["none"]
T1_KEYS = ["arc", "milestone", "round", "achieves", "opens", "TRIGGER"]
T2_KEYS = ["gate", "what it checks", "state", "TRIGGER"]


def read_lines(path):
    with open(path, encoding="utf-8") as f:
        return f.read().split("\n")


def dictated(lines, start_prefix, end_prefix, keys):
    s = next(i for i, l in enumerate(lines) if l.startswith(start_prefix))
    e = next(i for i, l in enumerate(lines) if i > s and l.startswith(end_prefix))
    keyre = re.compile(r"^\s+(%s): (.*)$" % "|".join(re.escape(k) for k in keys))
    rows, cur = [], None
    for l in lines[s + 1:e]:
        if re.match(r"^\s+row: \d+$", l):
            cur = {}
            rows.append(cur)
            continue
        m = keyre.match(l)
        if m and cur is not None:
            cur[m.group(1)] = m.group(2).rstrip()
    return rows


def table(lines, heading, header):
    s = next(i for i, l in enumerate(lines) if l.startswith(heading))
    raw = []
    for l in lines[s + 1:]:
        if l.startswith("## "):
            break
        if l.startswith("|"):
            raw.append([c.strip() for c in l.strip().strip("|").split("|")])
    assert raw[0] == header, "header of %s is %r" % (heading, raw[0])
    assert all(set(c) <= set("-: ") for c in raw[1]), "no separator row"
    return raw[2:]


def main():
    roadmap_path, prompt_path = sys.argv[1], sys.argv[2]
    landed = None
    if "--landed" in sys.argv:
        landed = int(sys.argv[sys.argv.index("--landed") + 1])
    rl = read_lines(roadmap_path)
    pl = read_lines(prompt_path)
    d1 = dictated(pl, "  (b) Table 1", "  (c) Table 2", T1_KEYS)
    d2 = dictated(pl, "  (c) Table 2", "  (d) Revisions", T2_KEYS)
    assert len(d1) == 14 and all(set(r) == set(T1_KEYS) for r in d1), "dictated Table 1 unreadable"
    assert len(d2) == 2 and all(set(r) == set(T2_KEYS) for r in d2), "dictated Table 2 unreadable"
    t1 = table(rl, "## Table 1", T1_KEYS)
    t2 = table(rl, "## Table 2", T2_KEYS)

    findings = {}

    def flag(key, why):
        findings.setdefault(key, []).append(why)

    for name, rows, ncell, keys in (("Table 1", t1, 6, T1_KEYS), ("Table 2", t2, 4, T2_KEYS)):
        for i, cells in enumerate(rows, 1):
            if len(cells) != ncell:
                flag((name, i, "*"), "has %d cells, not %d" % (len(cells), ncell))
                continue
            for k, c in zip(keys, cells):
                if k == "TRIGGER" and c == "":
                    flag((name, i, k), "empty TRIGGER")
    for i, cells in enumerate(t1, 1):
        if len(cells) != 6:
            continue
        arc, ms, rnd = cells[0], cells[1], cells[2]
        if arc not in ARCS:
            flag(("Table 1", i, "arc"), "not one of R4's eight arcs")
        if ms not in MILESTONES:
            flag(("Table 1", i, "milestone"), "not M0-M7 or none")
        if not re.search(r" — (LANDED|PLANNED)$", rnd):
            flag(("Table 1", i, "round"), "does not end in '— LANDED' or '— PLANNED'")
        for k, c in zip(T1_KEYS, cells):
            if re.search(r"\b\d{4}-\d{2}-\d{2}\b", c):
                flag(("Table 1", i, k), "holds a date")
            for m in re.finditer(r"\b(\d{3})\b", c):
                if int(m.group(1)) >= 4:
                    flag(("Table 1", i, k), "holds a spec number %s" % m.group(1))

    compared = equal = 0
    allowed = []
    for name, rows, dic, keys in (("Table 1", t1, d1, T1_KEYS), ("Table 2", t2, d2, T2_KEYS)):
        if len(rows) != len(dic):
            flag((name, 0, "*"), "has %d rows, the prompt dictates %d" % (len(rows), len(dic)))
        for i, (cells, want) in enumerate(zip(rows, dic), 1):
            if len(cells) != len(keys):
                continue
            for k, c in zip(keys, cells):
                exp = want[k]
                if name == "Table 1" and i == landed and k == "round":
                    assert exp.endswith("— PLANNED")
                    new = exp[: -len("PLANNED")] + "LANDED"
                    allowed.append("row %d round: dictated %r, expected %r" % (i, exp, new))
                    exp = new
                compared += 1
                if c == exp:
                    equal += 1
                else:
                    flag((name, i, k), "differs from the dictated field: expected %r, got %r" % (exp, c))

    print("Table 1: %d rows; Table 2: %d rows; cells compared with the prompt: %d, equal: %d"
          % (len(t1), len(t2), compared, equal))
    for a in allowed:
        print("ALLOWED DIFFERENCE (--landed %d): %s" % (landed, a))
    for key in sorted(findings, key=lambda k: (k[0], k[1], str(k[2]))):
        print("FINDING %s row %d %s: %s" % (key[0], key[1], key[2], "; ".join(findings[key])))
    print("TOTAL: %d cell(s) with a finding" % len(findings))
    sys.exit(1 if findings else 0)


main()
```

`check_backlog.py`:

```python
"""Spec 011 Item 2(c): check docs/BACKLOG.md.

usage: python3 -I check_backlog.py BACKLOG.md PROMPT.md

Every entry ("### B-n — name") must have exactly the fields Origin, Would buy,
State, Evidence date (no other bold-labelled field), each non-empty; its State
must begin with one of R2's five states (promoted / stale / dropped need a
reason or a spec after the word); ids are B-1.. in order, never repeated; the
evidence date begins with yyyy-mm-dd; "would buy" has at most two sentences.
Then every entry is compared with the seed of the same number dictated in the
prompt's Item 2(b): name, origin, would buy, evidence date, and state 'idea'.
One FINDING line per offending (entry, field).
"""
import re
import sys

FIELDS = ["Origin", "Would buy", "State", "Evidence date"]
STATES = ["idea", "sized", "promoted", "stale", "dropped"]


def read_lines(path):
    with open(path, encoding="utf-8") as f:
        return f.read().split("\n")


def seeds(pl):
    s = next(i for i, l in enumerate(pl) if l.startswith("  (b) The seeds."))
    e = next(i for i, l in enumerate(pl) if i > s and l.startswith("  (c) The check"))
    region = pl[s:e]
    text = "\n".join(region)
    default_origin = re.search(r'origin is "([^"]+)"', text).group(1)
    keyre = re.compile(r"^\s+(short name|origin|would buy|evidence date): (.*)$")
    out, cur = [], None
    for l in region:
        if re.match(r"^\s+seed: \d+$", l):
            cur = {}
            out.append(cur)
            continue
        m = keyre.match(l)
        if m and cur is not None:
            cur[m.group(1)] = m.group(2).rstrip()
    for sd in out:
        sd.setdefault("origin", default_origin)
    return out


def entries(bl):
    out, cur = [], None
    for l in bl:
        m = re.match(r"^### B-(\d+) — (.*)$", l)
        if m:
            cur = {"id": int(m.group(1)), "name": m.group(2).strip(), "fields": {}, "extra": []}
            out.append(cur)
            continue
        if l.startswith("#"):
            cur = None
            continue
        if cur is None:
            continue
        m = re.match(r"^- \*\*([^*]+):\*\* ?(.*)$", l)
        if m:
            k, v = m.group(1), m.group(2).strip()
            if k in FIELDS and k not in cur["fields"]:
                cur["fields"][k] = v
            else:
                cur["extra"].append(k)
    return out


def main():
    bl = read_lines(sys.argv[1])
    pl = read_lines(sys.argv[2])
    sd = seeds(pl)
    assert len(sd) == 4 and all(set(s) == {"short name", "origin", "would buy", "evidence date"} for s in sd), \
        "dictated seeds unreadable: %r" % sd
    es = entries(bl)
    findings = {}

    def flag(key, why):
        findings.setdefault(key, []).append(why)

    ids = [e["id"] for e in es]
    if ids != list(range(1, len(ids) + 1)):
        flag(("B-*", "ids"), "ids are %r, not B-1..B-n in order, once each" % ids)
    if len(es) != len(sd):
        flag(("B-*", "count"), "%d entries, the prompt dictates %d seeds" % (len(es), len(sd)))
    compared = equal = 0
    for e in es:
        eid = "B-%d" % e["id"]
        for k in FIELDS:
            if k not in e["fields"]:
                flag((eid, k), "field missing")
            elif e["fields"][k] == "":
                flag((eid, k), "field empty")
        for k in e["extra"]:
            flag((eid, k), "a field beyond the four (and the evidence date), or a repeated one")
        if e["name"] == "":
            flag((eid, "name"), "no short name")
        st = e["fields"].get("State")
        if st is not None and st != "":
            word = re.match(r"^([a-z]+)\b(.*)$", st)
            if not word or word.group(1) not in STATES:
                flag((eid, "State"), "%r does not begin with one of R2's five states" % st)
            elif word.group(1) == "idea" or word.group(1) == "sized":
                if word.group(2).strip():
                    flag((eid, "State"), "%r: '%s' takes nothing after it" % (st, word.group(1)))
            elif not re.match(r"^ \(.+\)$", word.group(2)):
                flag((eid, "State"), "%r: '%s' needs its spec or reason in parentheses" % (st, word.group(1)))
        ev = e["fields"].get("Evidence date")
        if ev and not re.match(r"^\d{4}-\d{2}-\d{2}\b", ev):
            flag((eid, "Evidence date"), "does not begin with yyyy-mm-dd")
        wb = e["fields"].get("Would buy")
        if wb:
            n = len(re.split(r"(?<=[.!?])\s+(?=[A-ZА-Я])", wb.strip()))
            if n > 2:
                flag((eid, "Would buy"), "%d sentences, not one or two" % n)
        if 1 <= e["id"] <= len(sd):
            want = sd[e["id"] - 1]
            pairs = [("name", e["name"], want["short name"]),
                     ("Origin", e["fields"].get("Origin"), want["origin"]),
                     ("Would buy", e["fields"].get("Would buy"), want["would buy"]),
                     ("Evidence date", e["fields"].get("Evidence date"), want["evidence date"]),
                     ("State", e["fields"].get("State"), "idea")]
            for k, got, exp in pairs:
                compared += 1
                if got == exp:
                    equal += 1
                else:
                    flag((eid, k), "differs from the dictated field: expected %r, got %r" % (exp, got))
    print("entries: %d; fields compared with the prompt's seeds: %d, equal: %d" % (len(es), compared, equal))
    for key in sorted(findings, key=lambda k: (k[0], k[1])):
        print("FINDING %s %s: %s" % (key[0], key[1], "; ".join(findings[key])))
    print("TOTAL: %d (entry, field) pair(s) with a finding" % len(findings))
    sys.exit(1 if findings else 0)


main()
```

## The squash commit message

```
spec 011: the roadmap, the backlog and arcs; ADVISOR r4 and ROUNDS r4

A records round (arc: Process); no code, no golden data, no gated path.

docs/ROADMAP.md (new): the plan as 14 rows (arc, milestone, round, achieves,
opens, TRIGGER) and the two gates of 003 section 4 P4 with their state;
row 8 (this round) is LANDED. docs/BACKLOG.md (new): possibilities, apart
from DEBT (owed) and FORKS (open); B-1 to B-4. Both are checked cell by cell
against the prompt's dictated fields, each check after a seeded control.

docs/ADVISOR.md r4 (sections 1, 2, 5, 6, 8): the advisor runs the human's
git commands on the human's word, per set, commands shown first, a second
ask before a force-push to the public origin; the arc; the arc agrees three
ways and the drift check at a landing; the starter is a file, rewritten at
every handoff, with a stale check. docs/ROUNDS.md r4 (sections 1, 2.2, 3,
4.1, 9): ARC: and arc: lines; the gate runs on the last commit that changes
more than the spec file, its INDEX row and its ROADMAP row; the merge and
the pushes are the human's word and the advisor's commands. CLAUDE.md: two
edits.

FORKS.md: F27 (the first client: a website), filed RULED; OPEN stays at 5.
DEBT.md: D-25 (actions/checkout on Node.js 20), D-26 (rule 51 and midnight),
D-27 (the INDEX status of a merged round), D-28 (gated texts that lag the
rulings of 2026-11-12), D-29 (no sources.md row for Rabten Choeling).

Gate on c58ccee: 147 passed, 0 failed (debug) / 147 passed, 0 failed
(release); the final commit changes only the spec file, its INDEX row and
its ROADMAP row.

Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>
```
