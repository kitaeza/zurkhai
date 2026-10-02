# 003 — Project method: spec-driven development

Status: draft (r1)
Depends on: 001, 002

## 1. Decision

The project is built in **rounds**: units of work governed by specs, executed by an engine session, adjudicated by an advisor, and decided by a human. Four documents carry the method, and this spec records the decisions behind them:

- **`docs/METHOD.md`** — the rules, each with its reason, where it applies here, and its cost.
- **`docs/ROUNDS.md`** — the shapes: a round prompt, a report, a STOP AND ASK, the handoff line, the merge.
- **`docs/ADVISOR.md`** — the advisor's role, what it may read, and its routines.
- **`CLAUDE.md`** — the operational summary every session reads first.

Where one of them disagrees with this spec, this spec wins and the disagreement is a flag.

## 2. Workflow

```
research legs  →  numbered forks  →  human adjudicates every fork
   →  spec (ends with "— spec text ends here —")
   →  gated session: one reviewed unit, flag-don't-fix, no push
   →  report per contract  →  advisor adjudicates  →  human merges and pushes
```

- **Specs** live in `docs/specs/NNN-slug.md`. The advisor assigns a round's number in its prompt, from the **tree**, not from `INDEX.md`, because an index goes stale; the round verifies the number is free and stops if it is not.
- **Every fork is adjudicated by the human**, including forks the session thinks are obvious. The session reports a resolved fork as "considered X, chose Y because Z". Forks are registered in `FORKS.md`, numbered `F-n`; it is the human's queue.
- **A declined question is not a delegated question.** If the human leaves a question unanswered, the decision stays visibly open; no plausible default fills it.
- **One reviewed unit per session.** A session that finds a second problem files it in `DEBT.md` or as a flag and does not fix it.
- **One branch per round.** A round works on `spec/NNN-slug`, commits only there, and never pushes or merges. The human squash-merges it: one commit per round on `main`. In a public repository this is also the line between a draft and something published.
- **Corrections to a dated record are made in place** with an inline `[CORRECTED yyyy-mm-dd: …]` marker, never as a silent rewrite.
- **Revisions**: each spec has a `Revisions` section. A revision prompt says "next revision per the file's Revisions section", never a literal number.
- **Positions live in the tree.** A decision that exists only in a chat, or in an assistant's memory, does not exist for the project. When a position is formed, it is written into a spec, `DEBT.md` or `FORKS.md` in the same round.

## 3. How the method applies here

`docs/METHOD.md` Part 1 states every rule with where it bites in this project. The ones this project leans on hardest:

- **A passing test proves only the shapes its data carries** (METHOD rules 11, 13, 39, 41). Fixtures and property strategies cover leap months, skipped tshes, doubled tshes, year boundaries, and the 1900/2100 edges; a clean year with none of these proves almost nothing. Reports give coverage per shape, not only a total.
- **Provenance** (rule 12). Every figure says which golden tier (T1–T5, 001 §6.1) it rests on.
- **Seeded controls** (rule 31). A deliberately wrong fixture row must fail; the Q1 diff is also run on a known-mismatching system, to prove it can report a mismatch.
- **Self-agreement is not correctness** (rule 35). P2's round trip and P6's agreement with an oracle built on the same paper are consistency checks; Janson's printed tables (T5) and the Buryat T1–T3 data are the independent ones.
- **Negative debt** (rule 53). For example: "must NOT default `System` in clients until the Q1 verdict spec exists". Each entry needs a trigger condition, or it is just an opinion.
- **Null results are reported** (rule 11). A source that matches no system is reported in full. So is a TO-VERIFY attribute Janson does not define.
- **Documentation is not evidence, in both directions** (rule 46). A datsan calendar *saying* it follows a system does not prove it does. A paper not mentioning an attribute does not prove the attribute is undefined.

## 4. Rules with a precondition

`docs/METHOD.md` Part 2, and where this project stands on each:

| Rule | Precondition met? | Application |
|---|---|---|
| P1 Differential testing against a reference | **Yes.** The reference is published Buryat calendars. Lotsawa House is a secondary oracle for Phugpa only. | **Mechanics from the oracle, semantics from the reference.** Lotsawa House can validate Phugpa arithmetic, but only Buryat sources can say what Buryat practice is. |
| P2 Real-usage corpus as demand evidence and fuzzer | **Partly.** The weekly zurkhai issues are a corpus of what users look up. | Use it to rank L2 categories. A corpus count is censored data: a category's absence can mean the source didn't cover it. |
| P3 Research legs before genuinely new work | **Yes**, for Q1, every L2 rule category, and any new attribute. | Legs: ① **reference**: datsan publications, and a зурхайч where reachable. ② **field**: other implementations (Lotsawa House, Henning's software, existing Mongolian apps); pin where their conventions differ. ③ **science**: Janson, Henning, Schuh, Reingold–Dershowitz, or an explicit "none relevant". ④ **Does this contradict anything already shipped?** Every leg is reported, including empty ones, with what it found against the approach. A research round that lowers a spec's advertised value has succeeded. Citations resolve to a tree path (`docs/sources.md`, `golden/…`), never to someone's notes. |
| P4 Explicit readiness gate | **Yes, twice.** | Gate 1: the Q1 verdict (M3). Gate 2: the first public release of a client, which checks correctness, provenance display, tradition labelling, and a reviewer's sign-off. Without a date driving it, a gate is allowed to say no. |
| P5 Reachability tracked separately from existence | **Yes, from M1.** The engine computes attributes before any rule or client reads them. | `docs/REACHABILITY.md` is created at the first `tshes-ffi` or `tshes-wasm` export: an attribute computed in core but displayed by no client, or a rule that is valid but unreachable from any UI path. |

## 5. Session report contract

Every session report — a resumed session's included — contains, in this order:

1. **The decisive item first**, as named in the prompt.
2. **Every property in the spec**: DONE / DEFERRED / N-A, each with evidence. Evidence means expected value, actual value, and the assertion text, not "tests pass". An unmentioned property reads as a forgotten one.
3. **Every question in the prompt answered**, including "already covered by X".
4. **The stop rule that governed the round**, named.
5. **Diff scope**: files touched, lines ±, whether any gated path was touched, and every file the test framework wrote.
6. **Forks encountered**, including forks the session resolved itself.
7. **Latitude taken**: anything done beyond or differently from the prompt.
8. **Numbers that must sum, shown summing**: for example matched + mismatched + no-data = days in range.
9. **What the prompt got wrong** — "none found" is an entry; silence is not.
10. **By-catch**: what was found but not asked for, with its `DEBT.md` id.
11. **If a precondition failed**: name the condition and the evidence, confirm zero diff, and stop. In that case the per-property table is omitted, so that a clean no-op cannot look like a partial failure.

The report opens with the fixed head block of `docs/ROUNDS.md` §4.1 and lives in the round's own `docs/specs/NNN-slug.md`, after the spec's end marker.

## 6. Prompt header

Every round prompt opens with the header in `docs/ROUNDS.md` §2.2: `RECIPIENT`, `WORKDIR`, `SPEC NUMBER`, `BRANCH`, `MODEL` (with its reason), `PRECONDITION`, `ARTIFACT`, `STOP RULE`, `CITATIONS`, `GATED`, `GRANTS` (when the round reads outside the repository), `ASKING`, `PATHS`.

- **PRECONDITION** names the commit `main` must be at. If `main` lacks the predecessor round, the round STOPs and reports: merging is the human's.
- **ARTIFACT** lists every input file and URL. If one is absent, STOP and report — do not reconstruct, infer, or proceed on reported figures.

Standing clauses most prompts need (`docs/ROUNDS.md` §2.3):

- **No mechanism offered.** State the symptom, not a suspected cause. ("Phugpa Losar 2031 differs from the T5 table by one day", not "probably the intercalation index".)
- **Re-derive inherited figures.** A count from an earlier report is re-measured, not copied.
- **Report the decisive item first.**
- **Both profiles** wherever a check could differ.
- **Read lists contain only tree paths**, external directories the session can reach, and named URLs. If a session needs to know something, the content goes into the prompt.

## 7. Model and effort policy

Two questions, in order:

1. Is the correct answer **discoverable** in the tree, a source, or a measurement, or must it be **invented**?
2. If invented, how expensive is it to redo?

| Answer | Model |
|---|---|
| Discoverable: implement per spec, transcribe, measure, verify, trace, run the Q1 diff | **Sonnet** |
| Invented but cheap to redo: a local design choice inside a round | Sonnet or Opus |
| Invented and expensive to redo: spec drafting, fork adjudication, rule-schema design | **Opus** |
| No local precedent and ripples through everything | **Fable**, rarely. Candidates here: the L2 condition language (002 F8), and the Q1 verdict if the data is ambiguous. |

Default to Sonnet and make escalation argue for itself. Effort defaults to **High**. Effort is a quality-floor knob, not a throughput knob.

The prompt's `MODEL:` line is derived, never defaulted, and names the branch of the table it took; the handoff line names the model too, because a session's model is set when it is started (`docs/ROUNDS.md` §7–§8). A sharper form of question 1: is a wrong answer **loud** (a test, a golden row, a differential fails) or **silent** (it fails nothing and compounds)? Transcribing a datsan source, interpreting it into `rules/`, and citing a paper are silent; they run on the smaller model only with an instrument that makes wrongness loud.

The adjudicator's own rule: when a fork is high-stakes and the adjudicator is not confident, say so and route it, rather than producing a confident answer.

## 8. Gated paths

Changes to these require an accepted spec named in the round's prompt. They are listed in `GATED.md`:

- `crates/tshes/**`
- `golden/**`: both adding rows (which needs provenance meta) and changing rows (which needs an in-place `[CORRECTED]` record)
- `docs/specs/**` for accepted specs: only via a revision. A round's own new spec file, its committed prompt and its `INDEX.md` row are not changes to an accepted spec.
- `docs/METHOD.md`: only via an amendment spec

Not gated: `rules/**`. Data PRs follow the citation requirement in 001 §7 and pass the validator. `clients/**` is also not gated, under normal review.

## 9. What the method does not cover

- **Performance measurement machinery.** There is no performance claim to defend. If one appears (WASM bundle size, app cold start), its measurement rules land first, by an amendment spec.
- **A separate series for process findings.** Forks are `F-n` in `FORKS.md`, numbered globally; a finding against the method itself is a fork or a `DEBT.md` entry.
- **Batch amendments.** This project is small enough that an amendment lands as its own spec when needed.

## 10. Cost, and when to skip

The method is not free. Stretches of rounds can pass with no visible number moving, until one correctness fix unblocks a real batch (`docs/METHOD.md`, warning (a)). The rigour is real, and so is the cost.

Scaled to this project:

- **Always**: the report contract, the prompt header, stop rules, provenance, seeded controls, no push. These are cheap and catch the most.
- **Only for genuinely new ground**: full research legs. A transcription round, a CI fix, or a client UI change does not need them; say so in one line in the spec.
- **Only at the two gates**: readiness-gate documents.
- **External contributors** to a public repo are not held to the session contract. Their PRs go through normal review. Data PRs must meet the citation rule regardless.

## 11. Repository files

Created with this spec:

- `CLAUDE.md`: session rules. `GATED.md`: the §8 list.
- `docs/METHOD.md`, `docs/ROUNDS.md`, `docs/ADVISOR.md`.
- `DEBT.md`: debt and negative-debt entries, each with a TRIGGER.
- `FORKS.md`: the fork register, the human's queue.
- `docs/sources.md`: the bibliography, with what each source is trusted for, and the index of the paper library. The papers themselves live outside git.
- `docs/specs/TEMPLATE.md`: the spec skeleton.
- `docs/specs/INDEX.md`: convenience only; the tree is authoritative.
- `docs/specs/prompts/`: every round's prompt, committed by the round, unchanged.

Created later, at their trigger:

- `docs/glossary.md`: by the first round that names a calendar term in code (`DEBT.md` D-2).
- `docs/REACHABILITY.md`: at the first `tshes-ffi` or `tshes-wasm` export (§4, P5).
- Readiness documents: at the two gates (§4, P4).

## 12. Roles and the round loop

- **The human** owns direction, every fork, every ruling, every merge and every push.
- **The advisor** is an Opus session working in a read-only clone of this repository. It drafts forks, rulings and round prompts, and adjudicates reports. It never writes to the repository and never reads the engine's source (`docs/ADVISOR.md`).
- **The engine session** reads the tree and the code, executes one round on its own branch, and reports. It never pushes or merges.

Three places on disk, and the reason for each:

| Place | What lives there | Why separate |
|---|---|---|
| `~/github/zurkhai` | The repository; engine sessions work here | — |
| `~/github/zurkhai-advisor` | A clone of it, without `crates/` and `clients/` | Independence: the engine session reads the code and reports; the advisor adjudicates from the report and the evidence. Two readers of the same source are not independent, and an advisor who reads the code writes assertions where one who asks writes stop rules. |
| `~/zurkhai-prompts/` | Round prompt files | Outside both clones, so writing a prompt touches neither repository's state. |
| `~/zurkhai-papers/` | The paper library | Outside git: a public repository redistributes nothing whose licence has not been checked. |

The loop: the advisor writes a prompt file; the human starts a fresh engine session with the handoff line; the round commits the prompt unchanged as its first commit, works, and reports in its spec file; on "spec NNN landed" the advisor checks the branch and adjudicates; the human merges. A halted round commits its work-so-far and its STOP AND ASK; the human rules; the advisor appends a dated ADDENDUM to the same prompt file; a fresh session resumes on the same branch.

## Revisions

- r1: initial.

— spec text ends here —
