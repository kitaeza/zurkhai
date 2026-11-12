# ROUNDS — how a round runs: the prompt, the session, the report, the merge

Status: r4 (2026-11-12). Not gated; it changes by a revision entry below, in the commit that changes it. Where it disagrees with an accepted spec, the spec wins and the disagreement is a flag.

This document is the working form of the loop in 003 §2 and §12. `docs/ADVISOR.md` covers what the advisor reads and does; `docs/METHOD.md` holds the rules cited here by number.

## 1. The loop, and who does what

```
the human + the advisor:  research legs → numbered forks → the human rules each one
the advisor:              writes the round prompt to ~/zurkhai-prompts/spec-NNN-name.md
the human:                starts a FRESH engine session with the handoff line (§7)
the engine session:       branch spec/NNN-name → commits the prompt unchanged → one reviewed unit
                          → the report in docs/specs/NNN-name.md → never pushes, never merges
the human:                "spec NNN landed" → the advisor adjudicates (ADVISOR.md §6)
the advisor + the human:  if the round touched crates/ or golden/: a read-only reviewer reads the tip
                          first (§10) → "review landed" → the adjudication is delivered
the human:                gives the word for each merge and push (§9)
the advisor:              runs the merge and the pushes, each on that word (§9)
```

- **The human** owns direction, every fork, every ruling, and the decision on every merge and every push: the human gives the word for each, and the advisor runs the commands (§9).
- **The advisor** (an Opus session in the read-only clone, `docs/ADVISOR.md`) drafts forks, rulings and prompts and adjudicates reports. It writes no file content to either clone; it runs the merge and push commands only on the human's word (`docs/ADVISOR.md` §1).
- **The engine session** (Sonnet by default, 003 §7) reads the tree and the code, does one reviewed unit, and reports. It decides implementation shape inside its scope; it asks about anything that changes what the project means (§6).

## 2. The prompt file

- The advisor writes it to `~/zurkhai-prompts/spec-NNN-short-name.md` (outside both clones, so neither repository's git state is touched). The human hands the engine one line naming the file (§7); a file removes the copy-paste channel, which silently truncates text.
- **The advisor assigns the spec number** in the prompt, from the tree (`ls docs/specs/*.md`, METHOD rule 8). The round verifies the number is still free and STOPs if it is not. A number is assigned by one party, because two rounds deriving it from one tree pick the same number.
- **One runnable prompt per spec number, never two.** When options need comparing, the advisor sends the fork first and writes the one prompt after the human picks.
- **The prompt is public.** The round commits it into `docs/specs/prompts/`, and the repository is public: a prompt carries no private data, no credentials, no third-party text beyond a two-sentence quotation, and no one's verbatim words the human has not agreed to publish.
- **Changes after a session has read the prompt are dated ADDENDA appended to the same file**, never edits to text a session has read. An addendum is not appended while a session is running (its `cmp` would fail); rulings for a running session go through the answer to its STOP AND ASK, and are appended afterwards. A prompt may be edited in place only if no session has read it — check for its branch, its committed copy, and ask the human, before editing.

### 2.1 The banner

The prompt file's content, and any prompt shown in a terminal, sits between banners. Covering text outside them is advisor → human, and is never phrased as an instruction an engine session could act on.

```
═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC NNN — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

                        ... the prompt ...

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC NNN — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════
```

Never two banner pairs in one message. No bracketed notes to the human inside a banner ("confirm this line"): the human sends prompts verbatim, so anything inside is read by the round as an instruction. A ruling still open when the prompt is due is asked first; the prompt is written after the answer.

### 2.2 The header

Every line is required unless marked conditional. `PATHS:` and `ASKING:` are reminders at the point of use; the rules behind them are in `CLAUDE.md`.

```
RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  NNN — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
ARC:          <one of the eight arcs of docs/ROADMAP.md>, assigned by the advisor with the spec number; the human may overrule it.
BRANCH:       spec/NNN-short-name, cut from main at <sha> before any edit; commit to it, the halt included.
              No merge. No push.
MODEL:        <model>. EFFORT: <effort>. Reason: <the 003 §7 branch, derived — ADVISOR.md §5>.
              FALLBACK: <what a session on a different model does — §8>.
PRECONDITION: main at <sha>, carrying spec <MMM>. If main lacks it, STOP and report: the merge is §9's.
              Read first, in this order: <tree paths>.
ARTIFACT:     <every input file or URL>. If absent, STOP and report — do not reconstruct, infer,
              or proceed on reported figures.
STOP RULE:    <a named condition and a named action>. (METHOD rule 1)
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; no fourth kind;
              an unobtainable source says PAYWALLED or UNREACHABLE. Read docs/sources.md before any
              fetch. Never curl/wget. (METHOD rule 50)
GATED:        yes: <GATED.md paths this round may change, under accepted spec MMM> | no.
GRANTS:       (conditional — only when the round reads outside WORKDIR, beyond this prompt file)
              <directories the human adds with /add-dir; the tools that read them>. (METHOD rule 67)
ASKING:       Every adjudication in the STOP AND ASK form (§5). Never an interactive widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec NNN — <the spec's name>

<One paragraph, three to five sentences: what the round is, why now, and what it does NOT do.>
```

On this machine `grep` is ugrep: a bounded repetition of 100 or more, such as `.\{0,120\}`, fails with "exceeds complexity limits"; keep bounds below 100.

Then the items. **Item 0** is always: create the branch; commit this prompt byte-identical to `docs/specs/prompts/NNN-short-name.md` as the first commit, and `cmp` it against `~/zurkhai-prompts/spec-NNN-short-name.md` before every later commit; verify the spec number; read the "Read first" list. The prompt ends with the round's predictions (METHOD rule 3, written before any measurement) and THE REPORT (what the report leads with).

**What a round may read outside WORKDIR.** Its own prompt file is read by the human's handoff line, which names it; anything else (the paper library) needs `GRANTS:`.

### 2.3 Standing clauses, every prompt

- **The decisive item first**, and a falsifiable prediction where one exists (METHOD rule 3).
- **No mechanism offered.** State the symptom, not a suspected cause. The advisor's account of a mechanism goes in as an instruction to check, never as a premise the round may build on; a probe preserves the failing input's shape and changes exactly one thing (METHOD rules 27–29).
- **Re-derive inherited figures.** A count from an earlier report is re-measured, not copied (`CLAUDE.md` hard rule 10, METHOD rules 4, 21).
- **Both profiles** wherever a check could differ (002 §4.3).
- **Read lists hold only tree paths**, external directories the session can reach, and named URLs. The round cannot read the advisor's memory: whatever it needs goes into the prompt.
- **Research legs are copied from 003 §4's canonical list**, never retyped: four legs, each reporting what it found AGAINST the approach, every absence naming its search.
- **Dictated data is one field per line.** Tabular data handed to a round for transcription (a golden row, a table from a paper) is written as `key: value` lines, never as aligned columns, which do not survive copying.
- **A guarantee is scoped to where it holds**, and the prompt says where it does not. A property that holds by accident of implementation is recorded as accidental where it is relied on. An exhaustive `match` proves every arm was written, never that any arm is right.
- **A prompt's list adds to a standing rule, never narrows it** (an "expected failing tests" list does not excuse an unlisted red).

## 3. The engine session

- **Fresh session per round.** A new spec, a change of model, or a resume after a halt: always a fresh session. An addendum to a round still running is pasted into that running session.
- **One reviewed unit** (`CLAUDE.md` hard rule 1). A second problem is filed in `DEBT.md`, or reported as a flag; not fixed.
- **Commits**: on the round's branch only, as many as the round needs. Every commit is preceded by: the prompt `cmp`; `git status --porcelain` compared with the files the round meant to change; the marker grep (METHOD rule 58); the open-forks audit arithmetic (METHOD rule 60); for a commit touching `crates/`, `cargo fmt --check` and `cargo clippy` for what it touches. Edits, their assertions, the greps and `git commit` run as one `&&`-chain with no `;`.
- **The gate** (METHOD rule 65) runs LAST, on the committed tree, with no edits in flight — also on a halt, if any tracked source changed. It runs on **the last commit that changes any path other than the round's own spec file, its `docs/specs/INDEX.md` row and its `docs/ROADMAP.md` row**. The final commit changes only those three, which no gate leg reads (spec 011 checked `tools/gate.sh` and the files its legs read: they read `crates/`, `Cargo.toml`, `deny.toml`, `rust-toolchain.toml` and `golden/`, and nothing under `docs/`). The report's gate line names the commit the gate ran on, and `git diff --name-only <that commit> <the tip>` shows that the tip differs from it in those three paths only. The reason: a report cannot hold a gate run on the commit that holds the report. Always: 002 §7. Only when reached: the WASM smoke test, the Q1 report, the rules validator. Every skipped leg is named.
- **The final commit** adds the round's row to `docs/specs/INDEX.md` (METHOD rule 8) and sets the round's row in `docs/ROADMAP.md` to LANDED; a halt does not.
- **A halt commits.** A round that stops on a STOP AND ASK commits its work-so-far plus the STOP AND ASK (in its spec file, and as an OPEN row in `FORKS.md`) to its branch, so the advisor reads the fork from the tree, not from a paste. `main` stays clean.
- **A resume** is a fresh session on the same branch. It reads the prompt and its addendum, `git log` and the report so far, and runs the workspace's tests once before building anything, because a round that halted before its gate left its failures unseen.
- **Never** push, merge, rebase `main`, or delete a branch.
- **After the final commit the branch stands still.** A reviewer may be reading its tip (§10); a later change comes only through an addendum, and its commit names the new tip.

## 4. The report

Every round has its own number and its own file, `docs/specs/NNN-short-name.md`: spec-shaped when the round drafts a spec, report-shaped when it executes one. Either way the file opens with the head block and closes with the report. A spec's normative text ends at "— spec text ends here —"; the report follows that line in the same file.

### 4.1 The head — fixed lines, first in the file

```
spec: NNN
prompt: docs/specs/prompts/NNN-short-name.md
arc: <one of the eight arcs of docs/ROADMAP.md>
branch: spec/NNN-short-name (cut from main at <sha>) | NONE + what happened instead
merge SHA: <sha> | not yet merged
pushed: yes | no
model: <model as the session reports it>, effort <level> (each session, if more than one)
gate: <N> tests passed, <M> failed (debug) / <N> passed, <M> failed (release) — <failed names, each tagged new or pre-existing>; skipped legs: <names, reasons>
golden: <rows added / corrected, per source> (omit if golden/ untouched)
```

Fixed lines, not prose, because a grep for `^pushed:` finds the state in one pass, where state buried in a paragraph is state a reader scanning for it never lands on. `gate:` is a count and a named failure set, never the word "green": a suite that quietly got smaller is visible only as a count. Tests are counted passed + failed, ignored excluded.

### 4.2 The body, in this order (003 §5, extended)

1. **The decisive item**, first, labelled as such.
2. **Every property**: DONE / DEFERRED / N-A, each with evidence — expected value, actual value, assertion text and test name (METHOD rules 36, 7). Every task: DONE / DEFERRED / REFUSED, each with its reason.
3. **Every prompt question answered**, including "already covered by X".
4. **The stop rule that governed the round**, named (METHOD rule 2).
5. **Diff scope**: files, lines ±, gated paths touched yes/no, and every file the test framework wrote (METHOD rule 58).
6. **Forks encountered**, including the ones resolved ("considered X, chose Y because Z"). An unresolved one is a STOP AND ASK with its `FORKS.md` row.
7. **Latitude taken** beyond or differently from the prompt.
8. **Numbers that must sum, shown summing.**
9. **What the prompt got wrong** — a required section; "none found" is a valid entry, silence is not. Claims the prompt made with the same confidence as everything else are exactly the ones to check.
10. **By-catch** — what was found but not asked for, with its `DEBT.md` id. "Nothing found" is a valid entry.
11. **Debt and forks**: ids filed, updated, paid; the open-forks audit with its arithmetic; the marker grep and its result.
12. **Held**: identified items only (METHOD rule 6).
13. **The squash commit message**, as a fenced block under the heading `## The squash commit message`, written from the final tree, ending with every model's `Co-Authored-By` trailer.

**If a precondition failed:** the condition, the evidence, "zero diff", and stop. The property table is omitted, so a clean no-op cannot look like a partial failure.

**Every session's report is complete**, a resume's included.

### 4.3 Evidence tags

Every claim in a report is one of: **MEASURED** (read, run or counted this round, against the current state), **RECALLED** (carried forward from an earlier round's record, not re-derived), **INFERRED** (a reasoned reading where no exact figure exists, disclosed as one), **RELAYED** (measured by someone outside this round — the human, the advisor — with source and date; it never decays into RECALLED). These tag how *this round* knows a claim. The source tiers T1–T5 (001 §6.1) tag where a *golden fact* comes from. The two axes are independent and both are stated.

"Reproduces" is not "is correct" (METHOD rule 35).

## 5. STOP AND ASK — a round that needs a decision mid-flight

A round that needs an adjudication it cannot make inside its scope writes it out in this shape, verbatim headings, in its response and in its spec file, files the `FORKS.md` row in the same commit, and commits. Never an interactive widget: a closed option set biases the answer toward its own options, the right answer is sometimes none of them, and widget text can arrive garbled in relay.

```
STOP AND ASK — spec <NNN> Fork <K>, <one-line subject>

DONE AND GREEN
  <what is already banked, so it is not re-litigated>

THE FINDING
  <root cause, sourced to file:line; what was reproduced>

EVIDENCE
  MEASURED: <...>   INFERRED: <...>   RECALLED: <...>   RELAYED: <...>

THE QUESTION
  <the decision, and why it cannot be made inside this round's scope>

OPTIONS — a starting set, not a menu
  1. <option> — real cost
  2. <option> — real cost
  <the adjudicator may answer with something not listed; say so>

IF THE ANSWER IS DELAYED
  <what the round will do meanwhile: nothing, or which independent items continue>
```

"OPTIONS — a starting set, not a menu" is required text, verbatim. The fork's identifier is `spec NNN Fork K`; when it is ruled, the ruling is written onto its `FORKS.md` row.

## 6. Ask or decide

**ASK** (STOP AND ASK): anything that changes what the project means or what a user observes — a computation system's behaviour, a golden row's interpretation, a source's admission or tier, an L2 rule's meaning, a gated path's contract, a scope widening beyond the prompt; and any fork a spec leaves open (`CLAUDE.md` hard rule 4).

**DECIDE**, and report as "considered X, chose Y because Z": implementation shape, which file, test strategy, a sibling fix inside the named scope.

**Round shape and scope are the advisor's call, not the human's.** An engine session does not escalate "should this round measure or build"; the advisor does not escalate round shape, effort, task order or which stop rules to name. The test: would a wrong answer change what the engine DOES, or only how the round is packaged? Only the first is a fork. A forced answer is not a fork either: if an existing rule, spec clause or ruling already decides it, say so and move on.

**Where the answer lands**: on the fork's `FORKS.md` row, and in the task text of the next prompt or addendum, marked as the human's ruling. A ruling binds from the moment it is given; its text lands in the next round's Item 0 or in its own amendment spec.

## 7. The handoff line

Every handoff of engine work ends with two things: **fresh session or continue**, and **one line to paste**.

- A new spec → a FRESH session, which reads the tree and `CLAUDE.md` anew. A change of model is always a fresh session.
- An ADDENDUM to a round still running → pasted into the RUNNING session.
- A resume after a halt → a FRESH session on the model the addendum names; it reads the prompt, its addendum, `git log` and the report first.
- The line: `**<Model>, effort <level>:** Read /home/kitaeza/zurkhai-prompts/spec-NNN-short-name.md and execute it` — with a short tail only when needed (", and execute ADDENDUM N — the round resumes on its own branch").

The model is set by how the session is STARTED, not by the prompt's `MODEL:` line; naming it in the handoff closes that gap. Effort is always stated: defaults differ between sessions.

## 8. Model and effort

003 §7's two questions decide it: is the answer DISCOVERABLE (in the tree, a source, a measurement) or INVENTED; and if invented, how expensive is it to redo. Discoverable → Sonnet. Invented and expensive → Opus. No local precedent and ripples through everything → Fable, rarely. Effort High unless a reason is stated.

A sharper reading of the same question: **is a wrong answer LOUD or SILENT?** Loud — a property test, a golden row, a differential or the compiler fails visibly — is safe on the smaller model. Silent — it fails no test and compounds — is not: transcribing or interpreting a datsan source into `golden/` or `rules/`, citations to Janson, Henning or Schuh, rule-schema design, the Q1 verdict. A silent round runs on the smaller model only if its prompt mandates an instrument that makes wrongness loud (for a transcription: the seeded wrong row and a second-pass cross-check, 001 §6.3).

**Fallback clause** (in the header, when a round might be resumed on another model): commit at every step boundary; a continuing session reads `git status`, the log and the report first; it never commits a previous session's uncommitted work unverified — it finishes and proves it by the step's oracle, or redoes it; a design choice that would fall to the fallback model is a STOP AND ASK; the report names the switch point.

## 9. The merge and the pushes (the human's word, the advisor's commands)

The human gives the word for each set of commands below, and the advisor runs them (`docs/ADVISOR.md` §1): the word is per set, the commands are shown first, and a force-push to the public origin is asked a second time, separately, right before it runs. After the advisor's go, on "spec NNN landed" (`docs/ADVISOR.md` §6):

1. Extract the squash message from the round's spec into a file first, and read it: `git -C /home/kitaeza/github/zurkhai show "spec/NNN-name:docs/specs/NNN-name.md" | awk '/^## The squash commit message/{f=1;next} f&&/^```/{if(g)exit; g=1; next} g' > <scratch>/NNN-squash.txt`. Its tail is the `Co-Authored-By` trailers; if one is missing, add it with `git commit --trailer`.
2. `git -C /home/kitaeza/github/zurkhai checkout main && git -C /home/kitaeza/github/zurkhai merge --squash spec/NNN-name && git -C /home/kitaeza/github/zurkhai commit -F <scratch>/NNN-squash.txt`
3. **The tree check:** `git -C /home/kitaeza/github/zurkhai rev-parse 'main^{tree}' 'spec/NNN-name^{tree}'` — the two hashes are equal, or the squash lost something.
4. **Push `main` to the public origin after every merge** (the human's ruling of 2026-11-11, R2 of spec 010). First the check: `git -C /home/kitaeza/github/zurkhai log --format='%ae%n%ce' origin/main..main | sort -u` must print only `8291740+kitaeza@users.noreply.github.com`, the human's GitHub no-reply address. If it prints anything else, the advisor does not push and asks the human. Then `git -C /home/kitaeza/github/zurkhai push origin main`. The first push is the exception to both halves: the public origin still holds the old initial commit, which the rewrite replaced (F26, below), so the range is all of `main`, and the check also prints `noreply@github.com`, the committer of the initial commit (it was made in GitHub's web interface); that one extra line is expected, and the push replaces origin's commit by a force-push, as the human ruled.
5. **The branch goes to the private mirror, then is deleted** (F11 (c), RULED 2026-11-11). After step 3's tree check, run `git -C /home/kitaeza/github/zurkhai push rounds main spec/NNN-name`. Then check that `git -C /home/kitaeza/github/zurkhai ls-remote rounds refs/heads/spec/NNN-name` prints the commit that `git -C /home/kitaeza/github/zurkhai rev-parse spec/NNN-name` prints. Then run `git -C /home/kitaeza/github/zurkhai branch -D spec/NNN-name`. `-D` and not `-d`, because `git branch -d` always refuses after a squash (the tip is not an ancestor of `main`); the check that matters is step 3's, and the mirror's tip is step 5's. A round branch is never pushed to the public origin. The mirror is the git remote `rounds`, named here by its remote only.

`main`'s history was rewritten once, before the first public push (F26): every commit's email address became the human's GitHub no-reply address. A commit id that the tree cites from before then is translated by `docs/specs/010-publication.md`, "The history map".

**A stacked pair** (a round cut from an unmerged predecessor, only on the human's hold): the predecessor first by `merge --squash`; the successor second by `git read-tree -u --reset spec/<second> && git commit`, which takes its tree exactly; each tree-checked.

## 10. The reviewer's brief

A read-only engine session that checks a landed round's committed tip before the advisor delivers its adjudication. When it runs, on which model, and what it may report: `docs/ADVISOR.md` §6. It is not a round: no spec number, no branch, no commit, nothing filed.

The advisor writes the brief to `~/zurkhai-prompts/review-NNN-<tag>.md`. The tag is `close` for the first review of a round, then `close2` and so on; a tag is never reused. The brief sits between banners, as a prompt does (§2.1), with `REVIEW BRIEF — SPEC NNN` in place of `ROUND PROMPT — SPEC NNN`. Its header, every line required:

```
RECIPIENT:    a READ-ONLY reviewer of spec NNN at commit <sha>. Not a round: it files nothing, commits
              nothing, starts no branch, and edits nothing in either clone.
WORKDIR:      /home/kitaeza/zurkhai-review-NNN-<tag>/ — empty, made by the human; the session's working
              directory. The repository /home/kitaeza/github/zurkhai is added with /add-dir and is read
              only through `git -C` at the commits below, never its working tree.
MODEL:        <model>. EFFORT: <effort>. Reason: the round ran on <model> (the trailers of <first> …
              <last>); a reviewer on the author's model shares its blind spots.
COMMITS:      base <sha> (main before the round); tip <sha> (the round's final commit).
GRANTS:       git, read-only, at the named commits only: `show`, `log`, `diff <a> <b>`, and `archive <sha>`
              extracted into WORKDIR. No git command that reads or writes the index, the working tree or a
              ref: no status, add, checkout, switch, stash, reset, restore, worktree, branch, tag, fetch or
              commit, and no diff without two commits. The Read tool and read-only shell tools on WORKDIR.
              cargo and tools/gate.sh only on an extraction inside WORKDIR. <the paper library, when a check
              needs it>. Network: none. (METHOD rule 67)
CHECKS:       numbered. Each names the fact to establish, its oracle (a source, a file, a run), and what
              counts as a FINDING. A check asks; it never hands the reviewer the advisor's suspected cause as
              a premise (§2.3).
REPORT:       /home/kitaeza/zurkhai-prompts/review-NNN-<tag>-report.md — the only file written outside
              WORKDIR; never overwritten. Per check: VERIFIED, FINDING, or NOT CHECKED with the reason; the
              commands and their output; file:line. Verdicts, test names, assertion text and run output —
              never quoted implementation (ADVISOR.md §3). The final message ends with the report's path
              and its sha256.
PATHS:        Every Bash call uses absolute paths and no cd. One command per call where it can be.
```

- **The handoff line**: `**<Model>, effort <level>:** Read /home/kitaeza/zurkhai-prompts/review-NNN-<tag>.md and execute it`, for a fresh session opened in its WORKDIR, with `/add-dir /home/kitaeza/github/zurkhai` (and the paper library when GRANTS name it). The human makes WORKDIR first, and says "review landed" once the report file exists.
- **Its findings reach the engine** only through the next prompt or addendum, quoted there. The round never reads the report.
- **It is never the next round's session.**

## Revisions

- r1 (2026-10-02): initial.
- r2 (2026-11-11): §10, the reviewer's brief; §1's loop and §3 name the reviewer (`docs/ADVISOR.md` r3, the human's ruling of 2026-11-11).
- r3 (2026-11-12): §9 steps 4 and 5, and the history note; on the human's rulings of 2026-11-11 (F11 (c), the new row F26, and R2).
- r4 (2026-11-12): §1, §2.2, §3, §4.1 and §9; on the human's rulings R4 and R5 (spec 011), and the gate-on-the-tip practice.
