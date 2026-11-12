# CLAUDE.md — rules for every session in this repo

Read this file, then the prompt and the spec it names, then `docs/ROUNDS.md` (the shapes of a round) and `docs/specs/003-project-method.md` if anything here is unclear. The rules behind these, each with its reason: `docs/METHOD.md`.

**If your working directory is the advisor's clone (`zurkhai-advisor`), you are the advisor: `docs/ADVISOR.md` governs you, not the engine rules below.**

## What this project is
An open-source **Buryat** zurkhai calendar (tradition: Buryat; computation system: open until the Q1 verdict, see 001 §4). Layers: `crates/tshes` (the engine, the only code that computes calendar facts), `rules/` (sourced interpretations), `clients/` (UI). See spec 001.

## Hard rules
1. **One reviewed unit per session.** If you find a second problem, file it in `DEBT.md` or report it as a flag. Do not fix it.
2. **Do not push. Do not merge.** Work on your round's branch, `spec/NNN-short-name`, cut from `main`. Its first commit is your prompt, byte-identical, in `docs/specs/prompts/NNN-short-name.md`; `cmp` it against the prompt file before every later commit. The merge is the human's decision (`docs/ROUNDS.md` §9).
3. **Gated paths** (`GATED.md`) change only under an accepted spec named in your prompt.
4. **Stop at forks.** If the spec does not decide something and the choice is not local and cheap to undo, stop and write it out as a STOP AND ASK (`docs/ROUNDS.md` §5) — never through an interactive option widget. File its row in `FORKS.md` and commit the work-so-far with it. A forced answer is not a fork: if a rule, a spec or a ruling already decides it, say so and move on.
5. **Stop on missing inputs.** If an ARTIFACT named in the prompt is absent, stop and report. Never reconstruct a source, a table or a figure from memory.
6. **Every formula cites its source** (paper, section, equation). Every golden row has provenance meta. Every rule has a source. Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED (`docs/METHOD.md` rule 50).
7. **Never tune constants to fit data.** A mismatch with a published calendar is reported, not absorbed.
8. **Never admit Mongolian or Tibetan interpretations into `rules/`.** Their calendars may be used only as arithmetic oracles (T4/T5).
9. **Never mix Western "lunar days" with tshes.** If a source does, report it and exclude the whole source.
10. **Re-derive inherited figures.** Do not copy a number from an earlier report; re-measure it.
11. **Re-verify delegated claims** against the source before reporting them. When two subagents disagree, read the source to decide. A subagent's brief carries the prompt's `GRANTS:` line verbatim.
12. Code standard: spec 002. CI must be green in **both** debug and release, on the final committed tree.
13. **Filing means a record.** An accepted risk or deferred item is a `DEBT.md` entry with a TRIGGER; an open decision is a `FORKS.md` row. A code comment or a spec's prose is not filed.
14. **Before every commit:** read `git status --porcelain` and account for every file (a `proptest-regressions` seed is named in the commit message, or removed); grep for your round's marker; compute the expected count of OPEN forks, then grep it; verify the commit message against the staged diff. Edits, their checks and `git commit` run as one `&&`-chain with no `;`.
15. **Dates come from `date`**, run this round — never from the session header or an earlier round's timestamps.
16. **Every Bash call uses absolute paths and no `cd`**; git uses `git -C`.
17. **Third-party material never enters git.** Papers and scans live in the paper library outside the repository (`docs/sources.md`); golden data stores facts, never a calendar's text or design.
18. **A declined question is not a delegated question.** If the human leaves a question unanswered, the decision stays visibly open; no default fills it.
19. **An edit that contradicts an adjacent sentence** may fix that sentence to restore consistency, and says so in the report. Anything farther away is flagged, not fixed.

## Report contract (in this order)
The report lives in the round's own `docs/specs/NNN-short-name.md`, opens with the fixed head block (`docs/ROUNDS.md` §4.1), and holds:
1. The decisive item named in the prompt.
2. Every spec property: DONE / DEFERRED / N-A, with evidence (expected value, actual value, assertion text, test name).
3. Every prompt question answered, including "already covered by X".
4. The stop rule that governed the round, named.
5. Diff scope: files, lines ±, gated paths touched (yes/no), files the test framework wrote.
6. Forks encountered, including ones you resolved yourself ("considered X, chose Y because Z").
7. Latitude taken beyond the prompt.
8. Totals that must sum, shown summing.
9. What the prompt got wrong ("none found" is an entry; silence is not).
10. By-catch: what you found but were not asked for, with its `DEBT.md` id.
11. If a precondition failed: the condition, the evidence, "zero diff", and stop. Omit the property table.

Null results are reported, not dropped. Every session's report is complete, a resumed session's included.

## Files a round keeps current
`DEBT.md` and `FORKS.md` (filed in the same commit that raises an item); `docs/specs/INDEX.md` (your own row, in your final commit); `docs/ROADMAP.md` (your own row set to LANDED, in your final commit); `docs/BACKLOG.md` (an entry the prompt tells you to promote, in the same commit); `docs/sources.md` (a row for every source you obtain, in the same commit).
