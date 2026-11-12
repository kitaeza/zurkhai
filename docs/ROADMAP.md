# ROADMAP — the order of the rounds, and the state of the two gates

Status: r2 (2026-11-12). Not gated; it changes by a revision entry below, in the commit that changes it. Where it disagrees with an accepted spec, the spec wins and the disagreement is a flag.

This file is the project's plan in one place: which rounds have landed, which are planned, what each opens, and what must hold before the next one may start. `docs/specs/001-init-spec.md` §10 stays authoritative for what each milestone delivers and when it is done; this file tracks **order and state only**.

**What it leaves out, and why.** Its two tables name no spec number, no date and no model (the Revisions below carry dates, and the spec that made each change). The advisor assigns a spec number when it writes the round's prompt (`docs/ROUNDS.md` §2), so a number written here earlier would be a guess; a date or a model is a claim a plan cannot keep. Those live where they are measured: the prompt, the round's head block, `docs/specs/INDEX.md`.

**Every row carries a TRIGGER**: a condition someone can check against the tree or a ruling, never an intention ("when we are ready"). The rows that had landed before this file was written show "—": there was nothing left to wait for when they were recorded.

**A status is replaced, never appended to.** When a round lands, or a gate's state changes, the cell is overwritten, and the old state goes into a one-line entry under Revisions. A round's own final commit sets its row to LANDED; a halted round does not.

**Arcs.** Every round has one arc, the line of work it belongs to. The prompt carries it as `ARC:`, the head block as `arc:`, and the round's row here as its first cell; at each landing the three agree (`docs/ADVISOR.md` §6). The advisor assigns the arc with the spec number, and the human may overrule it. A round that landed before arcs existed has its arc only in its row here; no landed spec file is edited.

- **Engine**: `crates/tshes`, the computation systems (milestones M1, M2).
- **Validation**: fixtures, oracles and differentials.
- **Sources**: Buryat calendars: the library, admission, golden data (M0).
- **Verdict**: the Q1 verdict (M3).
- **Rules**: the L2 rules (M4).
- **Clients**: the website and Android (M5, M6).
- **Outreach**: datsans, a зурхайч, the Buryat strings (M7, F4, F5).
- **Process**: method, records and publication (no milestone).

The weakest boundary is Engine against Validation (the advisor's caveat). A round that sits on it takes the arc the advisor assigns, and the human may overrule, as for any arc.

**How a new row enters.** A round that is not on the roadmap gets its row, agreed with the human, before its prompt is written.

## Table 1 — the plan

| arc | milestone | round | achieves | opens | TRIGGER |
|---|---|---|---|---|---|
| Engine | M1 | workspace-gate — LANDED | The workspace and the gate | every later round's gate (002 §7) | — |
| Engine | M1 | phugpa-core — LANDED | The Phugpa core | the reference fixtures | — |
| Validation | M1 | reference-fixtures — LANDED | The reference fixtures: T5 tables from Janson v1, the T4 oracle, P5 and P6 | Phugpa checked on every day of 1900–2100; the other systems checkable against Janson's tables | — |
| Engine | M1 | to-verify-attributes — LANDED | The TO-VERIFY attributes of 001 §5.2: what Janson v1 defines; rows 6–8 deferred (D-18) | M1's last condition: the §5.2 rows resolved or explicitly deferred | — |
| Engine | M2 | genden-tsurphu — LANDED | Genden and Tsurphu: the two other candidate systems, Gregorian ↔ ZDate from Janson v1; P5 for both | all three candidate systems, for the Q1 differential | — |
| Validation | none | henning-differential — LANDED | The Henning differential: TCG 1.06 against the engine's three systems, 1900–2100 | D-23 and D-24 | — |
| Process | none | publication — LANDED | Before the first public push: the CI pin, the history audit and map, the private mirror | the public repository, and CI on hosted runners | — |
| Process | none | roadmap-backlog — LANDED | The roadmap, the backlog and arcs; ADVISOR r4 and ROUNDS r4; the first client recorded as a website | an arc on every later prompt; the Sources round | publication landed |
| Sources | M0 | buryat-sources — PLANNED | the admission of the Buryat datsan calendars in the paper library, and their tshes as golden rows (001 §10 M0) | the Q1 verdict | roadmap-backlog landed, and the human has answered the questions on admission and tier that the round's prompt needs |
| Verdict | M3 | q1-verdict — PLANNED | the Q1 differential report, and the verdict spec or a recorded stop-and-consult (001 §10 M3) | D-1 is lifted only by the verdict spec; the TRIGGERs of D-19, D-23 and D-24 name this round | M0's done-when is met (001 §10), and genden-tsurphu landed |
| Rules | M4 | l2-schema — PLANNED | the L2 schema, its validator and the first sourced table (001 §10 M4) | sourced rules for the first client | the human orders it; 001 §10 sets it no dependency, and it is not yet ordered |
| Clients | M5 | website — PLANNED | the first client, a website at zurkhai.org (F27) (001 §10 M5) | Android; the second gate of 003 §4 P4 | the Q1 verdict spec exists (D-1), and l2-schema landed |
| Clients | M6 | android — PLANNED | Android, on the same engine through bindings (001 §10 M6) | — | website landed, and F7 is ruled |
| Outreach | M7 | zurkhaich-review — PLANNED | a review by a зурхайч: written feedback recorded, an endorsement if given (001 §10 M7) | — | F4 is ruled |

## Table 2 — the gates

| gate | what it checks | state | TRIGGER |
|---|---|---|---|
| Gate 1, the Q1 verdict (003 §4 P4; M3) | which computation system the Buryat calendars follow | NOT REACHED: M0 has no admitted source yet; D-1 holds | the Q1 verdict spec is accepted |
| Gate 2, the first public release of a client (003 §4 P4) | correctness, provenance display, tradition labelling, and a reviewer's sign-off | NOT REACHED: no client exists | the website's first release candidate |

## Revisions

- r1 (2026-11-12): initial, on the human's rulings R1, R3, R4 and R7 (spec 011).
- r2 (2026-11-12): Table 1, row 8 (roadmap-backlog): PLANNED → LANDED, by the round's own final commit (spec 011).
