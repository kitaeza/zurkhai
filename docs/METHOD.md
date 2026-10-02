# METHOD — how this project works, and why

Status: r1 (2026-10-02). A gated path (`GATED.md`): it changes only by an amendment spec.

These are the rules every round, the advisor and the human work by. `CLAUDE.md` is their operational summary for an engine session; `docs/ROUNDS.md` gives the shapes of a prompt, a report and a STOP AND ASK; `docs/ADVISOR.md` gives the advisor's role. Spec 003 records the decisions behind all four.

**Three parts, and the split is the point.**

- **Part 1** — rules that hold for every round.
- **Part 2** — rules with a named precondition ("IF you have X, THEN…"): wrong without it, not merely unnecessary.
- **Part 3** — what this method deliberately does not cover, so a reader knows what they are not getting.

**Per rule:** the rule; **why** — the failure it prevents, because a rule stripped of its reason is a rule nobody remembers under pressure; **here** — where it bites in this project; and, where it matters, its **cost**, including when *not* to use it.

**How the method grows.** A rule enters only through an amendment spec, carrying the finding that earned it, and takes the next number in its part when it lands — never reserved in advance. A finding against a rule is flagged, never fixed in place by the round that meets it (`CLAUDE.md` hard rule 1).

## Index by subject

| Subject | Rules |
|---|---|
| A. Rounds and reports | 1–9 |
| B. Evidence and measurement | 10–26 |
| C. Naming a cause | 27–30 |
| D. Tests and instruments | 31–45 |
| E. Sources and golden data | 46–52 |
| F. Records and the tree | 53–62 |
| G. Builds and the gate | 63–65 |
| H. Delegation | 66–67 |
| Part 2 | P1–P5 |

---

# Part 1 — rules for every round

## A. Rounds and reports

### 1. Stop rules are named in advance, not discovered in the moment

**Rule.** Every round that could plausibly hit a wall states, before work starts, what the wall looks like and what happens when it is hit — not "use judgment", but a specific condition and a specific action (usually: stop, report, don't guess past it).

**Why.** Without a named rule, a round facing an ambiguous result reasons its way past the ambiguity in whichever direction keeps the work moving — the exact bias a stop rule removes, because the one who hits the wall is also the one who wants to keep going.

**Here.** Every prompt carries a `STOP RULE:` line (`docs/ROUNDS.md` §2.2). Standing stop rules: 001 §12.

**Cost.** A stop rule stated too broadly ("stop if anything looks wrong") is an incantation, not a rule: it must name the condition. A low-stakes task does not need one.

### 2. The report contract

**Rule.** Every governed unit of work reports: every property DONE / DEFERRED / N-A (never silently absent — an unmentioned property reads as a forgotten one); every question in the prompt, including "already covered by X"; diff scope (files, lines, whether a gated path was touched); every fork encountered, including the ones resolved without escalating; and the stop rule that governed the round, named. A precondition failure is reported as one — the condition, the evidence, zero diff — never dressed as a partial success.

**Why.** A report that lists only what shipped is indistinguishable, to a reader who was not there, from one that silently dropped three properties and a fork nobody wanted to mention. "Trust me, I checked" does not survive the second reviewer.

**Here.** The full order and the fixed head block: `docs/ROUNDS.md` §4; 003 §5.

**Cost.** A full report on a five-minute task is theatre. It earns its cost wherever a silent gap would be expensive to find later — which is most real work, so err toward using it.

### 3. The decisive item first, and a falsifiable prediction

**Rule.** A request for work that will be checked is built for disproof: the stop condition and the report shape come before the task list, the decisive item is asked for first, and where there is a real expectation, the specific number or outcome is written down BEFORE the work starts.

**Why.** A request with no prediction lets the worker land on whatever answer is easiest to defend; a falsifiable prediction gives a target to disprove instead. An exact predicted outcome that lands exactly is itself part of what makes a result trustworthy; a vague one ("this should improve coverage") lets a partial win be reported as success.

**Here.** "Expect Phugpa Losar 2027 = <date from Janson's table>". The prompt's header holds the stop rule (`docs/ROUNDS.md` §2.2); its predictions section is written before any measurement.

**Cost.** A numeric prediction can be wrong, publicly. That exposure is the mechanism. When there is no model to predict from, say so instead of inventing a number.

### 4. A prediction stated against another round's numbers is scored only after its baseline is re-measured

**Rule.** A target stated as "at least as good as X", with X measured earlier, is scored only after X is re-measured in this round's own conditions. Both values are reported with the difference named. The prompt's number is a hint, never the comparator.

**Why.** A baseline re-measured on the same build can differ widely from the one a prompt quotes, and a result between the two gets opposite verdicts depending on which one it is scored against.

**Here.** "Phugpa matches ≥ N rows, per report K" is re-measured on this round's tree, golden snapshot and tool build before it is scored. This extends `CLAUDE.md` hard rule 10 from figures to predictions.

### 5. Know an instrument's repeat spread before writing a threshold against it

**Rule.** Before a prediction is written against a measurement, the measurement is run twice and its spread stated. A threshold inside the spread is written UNSCOREABLE at this shape — not scored after the fact — and the prediction names the shape that could resolve it.

**Why.** A threshold set against a single sample is set against nothing: if the measurement's own repeat moves more than the threshold, no outcome could score the prediction either way.

**Here.** The deterministic engine and the Q1 diff have zero spread. Unseeded property-test shape counts and two transcription passes of one source (001 §6.3) do not.

### 6. A carried-forward list holds only identified items

**Rule.** Anything a report or a prompt carries to the next round is an identified item: a `D-n` (`DEBT.md`), an `F-n` (`FORKS.md`), or `spec NNN Fork K`. An item without an identifier is FILED — an entry with a TRIGGER — before it is carried; a phrase never is. An identified item may leave the list without comment, because its own entry records when it was done or retired. No item travels under a nickname.

**Why.** Items carried as phrases get shortened to "the long tail", then dropped by a round that stops copying them forward; numbered items survive because their entry is their register. A nickname beside a perfectly good number splits one item into two names.

**Here.** Flags, DEFERRED properties and a report's Held list name an id.

### 7. Before the last build, every test the spec names is looked up in the tree

**Rule.** Before a round's final build and gate, every test name its spec's properties carry is looked up in the tree mechanically, with the output in the report. A name with no matching `fn` is a property whose oracle was never built: it is built now, or the property is scored DEFERRED naming the missing test — never DONE on the strength of another test.

**Why.** A property with two named oracles can be scored as held for many builds on one of them while the other was never written.

**Here.** P1–P7, the seeded controls and the fixture tests: a property marked DONE names a test that exists.

### 8. The specs index is checked against the tree, and a round adds its own row in its final commit

**Rule.** `docs/specs/INDEX.md` is a convenience, never the source of the next free number; the tree is (`ls docs/specs/*.md`). Where they disagree the tree wins, and the index is corrected in the same session. A round adds its own row in its final commit, when its spec can at last describe itself.

**Why.** An index goes stale for a structural reason — a spec cannot describe itself while it is being written — and a number taken from a stale index collides.

**Here.** The advisor assigns a round's number in its prompt; the round verifies in the tree that it is free and stops if not (`docs/ROUNDS.md` §2).

### 9. The advisor asks for its own session to be cleared, and the tree holds what a fresh one needs

**Rule.** Engine rounds are self-contained; the advisor's judgment spans rounds. So the advisor's session is cleared only at a CLEAN boundary — a round merged and the next prompt written, or an addendum written with its rulings — never mid-adjudication, and never while a ruling exists only in the conversation. The advisor says so unprompted, as the last line of a handoff: every two or three rounds, at the first clean boundary after its context was compacted, before a heavy adjudication, and whenever it catches itself citing a stale figure or re-asking something settled. **The clear line and the starter prompt for the fresh session are ONE message**; the starter is composed from files, never from the conversation, which makes writing it the last check that everything decided is durable.

**Why.** A long session costs more per turn and loses detail at every compaction. A clear line sent without its starter hands the human a chore instead of a handoff; a starter composed from memory of the conversation carries forward whatever was never written down.

**Here.** `docs/ADVISOR.md` §8 holds the triggers and the starter's exact shape.

## B. Evidence and measurement

### 10. Fix lists are ordered by evidence × impact × cost, and impact is measured, not estimated

**Rule.** A candidate fix carries three independent numbers, never collapsed into one "priority": how strong is the evidence it is real; how much it matters on real data — measured, not reasoned about; and what it costs. A zero on any one collapses the ranking, however strong the other two: a real, cheap fix that touches nothing real still ranks last.

**Why.** Ranking by evidence and cost alone lets a well-evidenced, trivial fix with zero effect on any real data rank above work that matters. The ranking feels right and is wrong, because the missing axis is the one that mattered.

**Here.** Impact = golden rows changed, mismatches closed, or attributes unblocked; "real data" means T1–T3 golden rows. Count them; don't estimate.

**Cost.** Measuring impact can cost more than the fix. For a long list, triage on evidence × cost first, then measure impact only on what survives.

### 11. The measurement doctrine

Five compressed rules, because "measure it" is not itself a method.

1. **Same-instrument comparison, always.** A figure from a different run, build profile or harness is not a comparison; it is two unrelated facts. When two figures disagree, name what built each instrument — which session, which dataset, which row count — before concluding the code changed: two harnesses can differ in a way indistinguishable from a regression until the instruments are checked.
2. **State the expected effect before measuring.** Post-hoc rationalisation of a surprising result is indistinguishable from a correct explanation to a reader who wasn't there; a prediction written first is falsifiable.
3. **Impact is measured, not estimated** (rule 10, stated again because it stands on its own as a measurement principle).
4. **A passing test proves the property only for the shapes its own data contains** — never more. A clean differential over data that never includes a given shape proves nothing about that shape, however often it passes. Name the shapes a check read, and read the shape the product sits in most, not only the special shapes a collection effort produces.
5. **A null result is reported, not discarded — because the reader's trust in a POSITIVE finding depends on it.** A report that records only what needed fixing cannot be told apart from one whose author never looked; once that ambiguity exists, every finding in the report inherits it.

**Why.** A wrong rule can stay provably green through several reviews when every fixture that could disagree with it happens to use the one shape the rule gets right. Nobody reasons wrongly; the suite's data is simply blind to the bug's shape.

**Here.** Mostly correctness-shaped, not performance. Item 4 is central: fixtures contain leap months, skipped tshes, doubled tshes, year boundaries and the 1900/2100 edges, and the Q1 report and every fixture test name the shapes their rows covered — as well as ordinary days of current years, the shape users meet most. Item 5: a source that matches no system is reported in full; so is a TO-VERIFY attribute Janson does not define.

**Cost.** A rough sanity check is not a measurement and need not pretend to be one.

### 12. Provenance marking: who benefits from this number being favourable

**Rule.** Every claim in a document meant to inform a real decision is tagged by where it came from: a party with no stake in the answer (an external oracle, a published source, a citation), or the people who want the decision to go one way. Tag it plainly; don't editorialise about what the tag means.

**Why.** Without the tag, "our own check says we match" and "an independent source computed the same answer and we matched it" read as equally confident prose, and only one of them is hard to fake.

**Here.** Golden tiers T1–T5 (001 §6.1): every figure in a report says which tier it rests on. The engine's own output is the stakeholder-produced kind. The Q1 verdict spec and the client release gate are the documents this rule is for.

**Cost.** Tagging every sentence is noise; reserve it for documents whose purpose is a go/no-go call.

### 13. A case count is not a domain statement

**Rule.** A measurement's stated RANGE is a claim in itself, separate from how many cases inside it were checked or how cleanly they passed. A boundary value just outside the tested range is not "probably fine by extrapolation"; it is unmeasured, and is reported that way.

**Why.** A grid that cleanly covers lengths 2 through 64 says nothing about length 1 — and a fix's own regression can live exactly at the one boundary the grid's range never reached, however rigorous the grid is inside it.

**Here.** "Tested 10,000 dates" means nothing unless the dates cover the special shapes; report coverage per shape, not just the total. The boundaries that matter: the 1900 and 2100 edges and behaviour outside them (P7), tshes 1 and 30, the month 12 → 1 rollover, leap months, negative JDN arithmetic (002 §4.2).

**Cost.** Pay it at boundaries a type or domain makes structurally special (empty, one, the edges, saturation) — not as exhaustiveness, which is a different and much more expensive property.

### 14. Pipeline reach: ask the unblocking count, not the local-stage count

**Rule.** "N more cases now succeed at step K" is a fact about step K; it says nothing about the final output unless every step after K is checked too. The number that matters for a decision is how many end-to-end outcomes changed.

**Why.** A change can make hundreds more inputs pass one stage while none of them gets further, each stopped by a different, already-known failure one stage later. The stage count is real, correctly measured, and says nothing a user would notice.

**Here.** "N more mean-date terms match" is not "N more golden rows match end to end" (tshes, month and leap flag together). For L2: how many rules exposing attribute X actually unblocks, not how often X is mentioned.

**Cost.** End-to-end measurement costs more — worth it whenever the local number will be quoted as the whole-system one, which is almost always.

### 15. Completeness is checked against something the claim did not produce

**Rule.** Two forms. **A dataset** that is internally consistent can still be silently incomplete: before it is taken as ground truth, its count is compared with a total known independently. **An enumeration** — "I listed every X" — is usually established by the same reading that produced it; it is checked against a list the tree already keeps, or an execution that would fail if it were incomplete. Where no independent source exists, the report says the claim rests on the search alone.

**Why.** An export taken by an account with limited access is complete from that account's point of view: no error, no partial flag, nothing that looks broken from inside. And an author who searched, read the hits and concluded the list was whole has produced the claim and its only witness in one act.

**Here.** Before a golden file, a fetched oracle output or a transcribed table is ground truth, its row count is compared with the days in its range, the issues the series published, the rows the paper's table prints; 002 §6's zero-row check catches only the empty case. "All of Janson's Phugpa formulas are implemented" or "every TO-VERIFY row is resolved" is checked against the paper's sections and tables, not the implementer's reading.

**Cost.** One comparison. What is expensive is remembering to distrust a clean-looking result at all.

### 16. Re-checking a named failure: ask the fact directly

**Rule.** When the question is "does THIS already-named failure still occur", re-running a multi-stage pipeline that reports only the FIRST problem it meets is unsound: fixing the original cause can unmask a second one, which then reads as the first changing identity or as a new failure. Ask the underlying fact directly. Where only the re-run is available, check its arithmetic against an independent invariant — a subgroup's count can never exceed its supergroup's — before reporting it.

**Why.** An input with two problems, one now fixed, reports only the remaining one, so the fixed one is silently counted as still failing. An impossible subtotal is often the only symptom.

**Here.** To learn whether a known mismatch (a date, a source) is fixed, compute that date and compare it with its golden row directly. An instrument that feeds a verdict lists every mismatch (001 §6.5), and its counts are shown summing (003 §5).

### 17. A claim about a real object is not correct until it has run, and its shape is checked, not read off its name

**Rule.** Two failures, two remedies. **Not executed:** a query, formula or script that reasons correctly about every fact it uses can still be wrong in ways no re-reading catches — run it once, for real; until then it is "reasoned to be correct", a weaker claim, and labelled so. **A name trusted over a shape:** describing an object's properties from what its name implies — look at its actual shape first. That is strictly cheaper, never skipped, and not replaced by execution: a query can run cleanly against a misdescribed object, or against a same-named twin.

**Why.** Text corrected twice on paper can still carry defects that one real run exposes at once; and a thing named like a history can turn out to be a snapshot, which one count would have shown.

**Here.** A formula, script or diff that has not run against real data is reported "reasoned, not run". Before describing a source — "it is tshes", "it is Phugpa", "it is a full year", "it is the 2024 edition" — check its shape: its numbers behave as tshes (a 30-slot cycle with skips and doubles, 001 §6.3); its coverage is what its title says. Name the exact twin: the arXiv version of Janson, the printing of a calendar, Lotsawa House's calculator or its library.

### 18. An instrument shown unable to discriminate is a result, never "unchanged"

**Rule.** When an instrument cannot resolve the difference being looked for, the report says "cannot resolve" and why — never "unchanged" or "both match", which read as a measured equality. The claim then has to come from somewhere else, or not be made.

**Why.** It is rule 31's counterpart: an instrument shown failing to discriminate is itself a finding, and writing its silence as agreement turns a gap in the evidence into a fact.

**Here.** When the golden data cannot separate candidate systems — Genden and Tsurphu share leap months and differ only on some days (001 §4) — the Q1 report says "cannot resolve on this coverage" and names the dates or shapes that would resolve it.

### 19. When a change moves an intermediate number and not the outcome, the outcome's cause is something you did not change

**Rule.** Measure the outcome and at least one intermediate quantity the change acts on directly. If the intermediate moves and the outcome does not, stop arguing about the change: the outcome has a different cause. Name it, and prove it with a test that takes the changed rule out of the picture. Do not tune the change.

**Why.** "No improvement" is tempting to read as "real but small", followed by tuning and re-measuring. A moving intermediate beside a flat outcome forecloses that reading — and also rules out the other explanation, that the change never took effect.

**Here.** A change that moves the leap-month set or the intercalation index while the Buryat mismatch count stays flat is not "real but small": the mismatches have another cause. Tuning again is exactly what `CLAUDE.md` hard rule 7 forbids.

**Cost.** One extra column. Choose the intermediate before running.

### 20. A per-unit figure that depends on the count is never multiplied by the count

**Rule.** Before multiplying a per-unit figure by a count, or extrapolating a rate from a sampled window to a whole range, ask whether the figure depends on position or count. If it does, measure the curve, state its form and residual, and sum or integrate it.

**Why.** Multiplying by the first measurement under-estimates a rising curve badly, and multiplying by the last — which feels conservative — still does. An extrapolation whose shape nobody named is a number, not a measurement.

**Here.** Extrapolating a mismatch or system-disagreement rate from the covered golden months to 1900–2100 (disagreements cluster by intercalation cycle); estimating transcription effort per weekly issue.

### 21. An aggregate is split before its dominant term is named, and an inherited term is re-measured, never re-cited

**Rule.** A figure that sums several terms is split into its terms before any one is called dominant. A term whose name arrived through a prompt, a debt entry or a report is measured on its own before it is cited again: a citation of a citation is not a measurement. A ratio whose denominator the change also moves does not measure its numerator.

**Why.** A cause name read once off the code and carried through several prompts can be wrong while the number under it is right; split, the named term may be a small fraction and an unnamed one the rest — and a fix built for the named one leaves the cost in place.

**Here.** The Q1 mismatch count is split by cause — transcription, source error, alignment, genuine variant — before any cause is called dominant (001 §6.5). A cause inherited from a prompt or `DEBT.md` is re-checked (`CLAUDE.md` hard rule 10). A match RATE is not evidence of matches when the change also moved coverage.

### 22. A correlation is reported with its base rate, or not at all

**Rule.** "N of M events of kind A fall near an event of kind B" means nothing until set beside the share of the whole window that lies near B's events. Report both. A share at or below the base rate is no correlation.

**Why.** When B is dense, almost any A falls near one; a coincidence reads as a mechanism, and the next round builds against it.

**Here.** "Mismatches cluster near skipped or doubled tshes", or "near leap months", needs the share of all covered days that lie near such events.

### 23. A run's settings are read from what ran it, never restated from memory

**Rule.** A sentence about a run's settings is a claim about its script or command. The report's settings come from the script or command itself — copied, or printed by the tool at start — never from memory of the project's practice. A setting with a default is written with what its absence means, because "not set" and "set to 0" can be different runs.

**Why.** A report can state a setting its runs never had, and a setting nobody chose can be inherited, unstated, through many rounds of measurements.

**Here.** The Q1 differential report prints its own settings at the top: systems, range, tiers included, golden snapshot, commit, profile. A property-test run's case count, seed and profile are quoted from the command that ran.

### 24. A refusal's cost is reported beside the integrity it buys, from the same run

**Rule.** When a round adds something that refuses or excludes work to stop a correctness defect, it reports both sides, per case, from one run: what is refused, and the integrity of what the same run left behind. A rate measured across a change of regime is not one rate.

**Why.** The refusal is visible and lands on whoever runs the workload; the defect it prevents is invisible and lands on whoever reads the result later. A measurement that reports only the visible side prices a correct fix as a catastrophe.

**Here.** A new `Err` path, a source-admission filter or a validator rule reports what it rejects beside what it prevents. A match rate pooled across sources or periods of different provenance is not one rate.

### 25. A differential is a check only where both sides can see the evidence that decides it

**Rule.** Before trusting a differential between two derivations, name the field that decides the question and check that both sides read it: "the two answers differ when X; side A reads X at …; side B reads X at …". If one side cannot see X, the differential is a test of that side, and its message says nothing about the data. A refusal that is also an action needs its own bound: ask what makes the next attempt different.

**Why.** A differential whose second side is blind to the deciding field still fires, still names a real disagreement, and is read as "the data is wrong" when it says "the impoverished side disagreed". It is independent in provenance and asymmetric in evidence, so an independence check passes it every time.

**Here.** A source that does not mark doubled or skipped days, or leap months, cannot decide a disagreement about them. Comparing it with the engine on those days tests the source's notation, and the Q1 report says so instead of counting a mismatch.

### 26. A dated report is frozen, and every figure cites one

**Rule.** A committed, dated measurement report is never regenerated or hand-edited, and its measured numbers are never edited or annotated as wrong; a wrong measurement is superseded by a new report. An error in its prose gets an inline `[CORRECTED yyyy-mm-dd: …]` with the original text kept. A figure quoted anywhere cites the report and its date (and its tier, rule 12), and a report's own counts are derived from the artifact, never recomputed separately.

**Why.** A report and its artifact diverge whenever their numbers are produced twice; and a silently edited measurement destroys the record of what was known when.

**Here.** Committed Q1 and coverage reports. "Phugpa 1,204 of 1,210 rows, T1–T3, `<report path>`, <date>".

## C. Naming a cause

### 27. A cause is NAMED by a run that removes one variable, never by the run that reproduces the defect

**Rule.** A reproduction shows THAT something happens, never WHY, and an instrument attached to it shows only the values on the path it took — consistent with several stories. Before writing "the cause is X", name the run that would be different if the cause were Y, and run it. Until then the cause is INFERRED, however rich the instrument's output.

**Why.** A rich instrument is the more dangerous case, because its output feels like proof. Two plausible stories can both fit the same trace, and one controlled run that changes one variable decides between them. A wrong cross-reference is copied by every next reader with no reason to doubt it: check the number, not the sentence around it.

**Here.** The cause of a Q1 mismatch (transcription, edition, system, epoch) and of an engine bug is named by a controlled run.

### 28. A necessary condition found by subtraction is necessary only among the variables that round varied

**Rule.** Removing one ingredient at a time from a reproduction finds conditions only among the ingredients it already has. After listing the conditions, write for each "the mechanism needs X; this condition is one way to get X", name a second way, and run it. Where no second way exists, say so and name the search.

**Why.** A condition can be true of the reproduction because of how it was built while the mechanism needs something more general; a fix scoped to the listed condition then closes a fraction of the defect.

**Here.** A bound such as "the mismatches occur only in leap years" is restated from the mechanism before anyone builds on it.

### 29. A run that varies a story's own variable and does NOT move the outcome eliminates the story; write the mechanism beside it

**Rule.** A run that varies the story's load-bearing variable and leaves the outcome identical is the falsifier — not "no evidence either way". The elimination is written beside a MECHANISM sentence, about what the system can and cannot represent, because the next reader will otherwise meet the same output and re-derive the same story.

**Why.** An elimination by a table persuades the round that ran it and nobody afterwards; a one-sentence mechanism is what the next reader cannot argue their way around.

**Here.** A mismatch "caused by the epoch" that reproduces unchanged under three different epoch treatments is eliminated, and the report says what the epoch can and cannot affect.

### 30. A retry, fallback or discriminator is named for what it does NOT read; check by a run that its input changes

**Rule.** A remedy is worth something only if its input differs where the failure's did, and a mode's NAME is not its input. Before a retry, fallback or discriminator is ruled — at the latest before it is built — one run shows that its input differs: the failure induced, the remedy run on it, one variable changed to show which input decided. A remedy's premise stated in words ("it reads no X") is DERIVED until that run exists, and the fork that proposes it says so.

**Why.** A ruling built on what a mode's name promises fails, loudly, on exactly the cause it was ruled for.

**Here.** A Q1 discriminator — "leap months separate Genden from Phugpa" — is shown by a run to differ between the systems on the covered data before a verdict rests on it. Genden and Tsurphu share leap months (001 §4).

## D. Tests and instruments

### 31. Prove the instrument can fail before trusting that it didn't

**Rule.** Before believing a detector's "clean" result, deliberately feed it something that should trip it, and confirm it does. Three sharpenings:

- **When two mechanisms can produce the same observable outcome, a test that asserts the outcome tests neither.** Name the mechanism in the assertion, not the benefit: if the assertion would still pass with the mechanism removed and its neighbour left in place, it is a test of the neighbour.
- **A property claimed "by construction" is a claim, not a proof.** It gets one measurement on the shape that would falsify it before it is recorded as held — "by construction" is where a property goes to stop being checked.
- **A level is a rate in disguise.** An assertion whose truth depends on how much a background process got done inside a window is measuring the machine; assert that the event happened, never how much finished in time.

**Why.** A detector that has never caught anything is indistinguishable from one that is silently broken. A harness can run clean over a full matrix of states and become trustworthy only once a deliberately seeded bug, injected at the same points, is caught.

**Here.** 002 §6: a deliberately wrong fixture row must fail; the Q1 diff is also run on a known-mismatching system, to prove it can report a mismatch. Genden and Phugpa share mean-motion constants and agree on most days, so a Genden test on a day where they agree tests the shared path, not Genden: Genden and Tsurphu fixtures include days where the systems differ. "By construction" claims here — P1's totality, "no floats so no rounding", `no_std` making 002 §4.5 mechanical, location independence (001 §5.3) — each get one check on the shape that would break them. The third sharpening binds client tests from M5 on; the engine has no threads and no clock.

**Cost.** The control is work up front and needs updating when the detector changes — worth it for detectors whose "all clean" will be relied on.

### 32. Debug and release are different programs

**Rule.** A check that exists only as a debug assertion is absent from the build users run, if release disables debug assertions. Treat a debug-only check as documentation of an invariant, never as its enforcement.

**Why.** An overflow guard written as a debug assertion fires correctly in debug; in release it compiles to nothing, and a large result silently wraps with no error and no signal.

**Here.** 002 §4.3–4.4: overflow checks on in release, `debug_assert!` backed by property tests, the whole suite in both profiles.

**Cost.** A real runtime check has a cost; pay it at any boundary a caller does not fully control, not for an invariant enforced by construction one line above.

### 33. A test that pins a known gap says so in its own name

**Rule.** A test asserting a LIMITATION (excluded, unreachable, refused) reads, to a green run and to a later grep, exactly like a test asserting a REQUIREMENT — unless its own name or doc comment says which. Name the gap in the test: `_a_gap`, `_not_yet_supported`, or a doc comment naming the future fix.

**Why.** A limitation test that reads as a requirement sits green in exactly the place a sweep for that gap would look, and the sweep moves on.

**Here.** Tests that assert a system is not yet implemented, that a TO-VERIFY attribute returns `Err`, or that a known golden mismatch is tolerated carry the gap in their name and the `DEBT.md` id in their doc comment (rule 44 for any tolerance).

### 34. A test of a for-any claim needs a fixture that reaches the edge the derivation assumed away

**Rule.** A property asserting "this holds for any magnitude, position or count" proves that only if its fixture reaches the edge the derivation quietly assumed away. The claim names the dimension the fixture must stress. Where a real corpus exists, run the property over it once: that is cheaper than reasoning out an adversarial fixture, and catches what a hand-built one misses.

**Why.** A small, convenient fixture can pass for the wrong reason — indistinguishable, by its green result, from one that stresses the claim.

**Here.** 002 §4.3's i64-headroom claim is tested at the extreme JDN of the supported range and beyond it. Intercalation and skip/double claims are tested across at least one full cycle of the system's rule, not one convenient year. And once, the property runs over every row of the real golden corpus.

### 35. Self-agreement, and reproducing your own prior output, are not correctness checks

**Rule.** A test proving two of your own derivations agree, or a script proving it reproduces its own earlier output, checks CONSISTENCY, not correctness — worth nothing the moment both sides share the defect. Use either in place of a check against something OUTSIDE the system (an oracle, a from-scratch re-derivation, an extended measurement) only after confirming the two sides cannot share a blind spot.

**Why.** Two derivers that collapse the same cases identically agree perfectly. A script re-run verbatim reproduces a figure that was wrong all along. A fix and its own acceptance test, both reading through the same faulty path, verify each other for as long as nobody checks against something outside. When a measurement is extended and checked against the extension and still holds, that survival is worth stating.

**Here.** P2's round trip `to_gregorian(to_zdate(d)) == d` is self-agreement: a shared wrong epoch passes it. Re-running the Q1 diff proves only that it is reproducible. Lotsawa House (T4) and `tshes` both derive from Janson, so P6 agreement can share a misreading of Janson; the independent checks are Janson's printed tables (T5), read directly, and the Buryat T1–T3 data.

**Cost.** Reserve the external check for claims a reader will rely on.

### 36. Prove the assertion, not the survival

**Rule.** A regression test asserts the SPECIFIC value that was wrong — never merely that the call completed, returned something, or did not panic. A defect reproduced live before the fix cites that reproduction (date, commit) as the pre-change evidence. A defect found by inspection proves detection by a targeted disable of the fix's own lines; a whole-diff stash proves only "something in this diff matters". A test that cannot fail pre-change is a negative control and says so — and then the claim it was meant to support is recorded **UNVERIFIED**: the disclosure must change what the surrounding text is allowed to claim. Two corollaries: narrowing a broad invariant re-audits every test whose reproduction relied on the broad form; and a test of property P reaches P by the most ordinary means, never by borrowing an unrelated defect as its trigger — it dies, looking like a regression, the day that defect is fixed.

**Why.** A fix that asserts only success ships the next defect on the same path. A test whose fixture cannot fail, honestly disclosed, still leaves "verified" standing in the summary above it. Reusing many pre-existing tests as the control for a new capability proves nothing about the new path, however many of them there are.

**Here.** 003 §5 defines evidence as expected value, actual value and assertion text. Re-running the Phugpa suite proves nothing about Genden. A seeded control reaches its failure through an ordinary wrong row. When 002 §5 narrows `to_gregorian` from `Option` to `Err(SkippedDay)`, every test written against the old contract is re-audited.

### 37. A doc comment asserting a property nothing tests is not a record

**Rule.** A positive claim in a doc comment — "never collides", "a cheap no-op", "X cannot happen" — is backed by a test that would fail if it stopped being true, or labelled "assumed (spec N, unchecked)". A claim later falsified is corrected in place, citing the spec that falsified it — never deleted without trace.

**Why.** An unqualified claim nothing enforces reads as settled fact to every later round, and quietly becomes the justification for a design.

**Here.** `tshes`'s public items carry doc comments (002 §8), and they will assert "never yields a skipped tshes", "valid 1900–2100", "location-independent". Each is tested or labelled assumed.

**Cost.** Not a test per sentence; untested claims must LOOK untested.

### 38. A check that runs outside the gate gets a tripwire inside it

**Rule.** Every acceptance test excluded from the ordinary gate (`#[ignore]`d, or run by hand) gets a cheap, always-run companion that checks its PRECONDITION at trivial scale, named for what it guards, with a doc comment stating what it does NOT check.

**Why.** A check nobody runs rots invisibly: a convention change can orphan its inputs, and nothing notices until someone needs it.

**Here.** Applies the day a check leaves `cargo test`: a full-range P6 run against Lotsawa House output, or the Q1 report. 002 §6's "a fixture test that loads zero rows fails" is already a tripwire of this kind.

**Cost.** A tripwire proves the acceptance can run, never that its numbers hold; say so in its doc comment.

### 39. A generated oracle states its approximations' direction and counts what it reaches

**Rule.** A model or generator that grades one rule must also model the rules around it, and where it errs toward refusing or restricting MORE, the properties pass vacuously. State each approximation's direction in code; list the rules the oracle OMITS, each with its direction; assert a non-zero reachability count for every shape the properties depend on. A known-failure count that falls to zero on a change to the oracle, not to the system, is a warning.

**Why.** An over-restrictive oracle does not fail — it passes, because the cases that would catch the rule never reach it, and "the defect is gone" is indistinguishable from "the corpus stopped reaching it".

**Here.** A proptest strategy that rarely draws a leap month, and a Q1 diff whose alignment drops ambiguous rows into "no data", both pass vacuously. Each counts the shapes it reached — leap months, skipped and doubled tshes, year boundaries, per system — and asserts them non-zero.

### 40. A control is re-derived for what it guards

**Rule.** A seeded control copied from a sibling copies the sibling's claim about which step is load-bearing — a property of the sibling's procedure, not of the step. Before copying a control, ask what makes the failure visible in THIS procedure, and which step is the last one guarding against it.

**Why.** A copied control can find nothing because a later step covers the one it strips — reading as a broken instrument — while the step the new procedure really depends on goes untested.

**Here.** A seeded control written for Phugpa is re-derived for Genden and for Tsurphu from their own differences (epoch, intercalation), never copied.

### 41. A property over a structure with internal boundaries draws its positions AT the boundaries, and a planted off-by-one proves the draw reaches them

**Rule.** A structure that changes its arithmetic at internal edges is tested with positions drawn AT those edges, as a strategy of its own beside the uniform draw. Before the property is trusted, an off-by-one is planted at an edge, and the property must go red in every run, not in some.

**Why.** A uniform draw lands on a given edge about once in the piece's size, so an edge bug fails one run in many — a real bug that reads as a flake, and a property that teaches its reader the boundary is covered.

**Here.** The calendar is all internal boundaries: month ends (tshes 29/30), skipped and doubled tshes, leap months, the 12 → 1 rollover, Losar, the 60-year cycle, the 1900/2100 range edges. P1–P4 and P7 draw these explicitly beside uniform dates, and a planted edge bug goes red three runs in three.

**Cost.** One strategy and one planted bug per boundary.

### 42. A random search that must FIND a planted defect runs from a fixed seed

**Rule.** A property whose job is to HOLD keeps proptest's own seeding, where a new seed is new coverage. A search whose job is to FIND a planted defect takes a fixed seed and enough cases to find it in the tree as committed, so its counterexample is the same every run and can be quoted. Every such search is paired with a by-hand input that is red on its own.

**Why.** A seeded control found by a fresh-seeded search is a fact about the run: a rare shape is sometimes missed, and the control reads green for no reason in the tree.

**Here.** A 002 §6 seeded control that relies on a random search uses `TestRng::deterministic_rng`; P1–P4 keep random seeds.

### 43. A test of a migration includes an item nothing touches

**Rule.** A test of a migration, a rewrite, or any writer that decides per item what to carry forward includes at least one item no change touches, and asserts that item's resulting form — not only that everything reads back equal.

**Why.** A fixture in which every item is touched is exactly the shape under which everything is rewritten anyway; an untouched item carried forward in the old form passes it.

**Here.** When the `rules/` schema, the golden layout, or the FFI/WASM types change under shipped data or clients.

### 44. A test that relaxes its oracle cites the debt id of the gap it hides, and that entry names the test

**Rule.** A test that relaxes a constraint its oracle states — a sort where order matters, a tolerance where the oracle is exact, a subset where it is whole — cites the `D-n` of the gap it hides, and that entry names the test.

**Why.** A comment calling a gap "already reported", with no id beside it, means nowhere it was reported; and a fix proven against a new test leaves the old, relaxed oracle relaxed unless the entry says which test to tighten.

**Here.** Any P5, P6 or Q1 test that tolerates a mismatch, skips a subset, or compares a set where the oracle is ordered. It backs `CLAUDE.md` hard rule 7: a tolerated mismatch is reported, never absorbed.

### 45. A round writes tests for what it built; nobody writes tests for what should work

**Rule.** Keep a standing correctness corpus, independent of any one round's diff, derived from the domain's semantics, its published sources and the specs — never from reading the implementation. Author it where the implementation is not one `cat` away, so the rule is the easy path rather than a discipline to self-police, and state plainly what the corpus is not.

**Why.** Each round tests the surface it touched; nobody's job is "does the rest still hold". A tester who reads the implementation tests what the code DOES rather than what it SHOULD do.

**Here.** The standing corpus is `golden/` plus the expected values in property and fixture tests, authored from published sources and specs, never from `crates/tshes` or its output. A transcription round does not run the engine on the dates it transcribes. The advisor's clone, which has no `crates/`, is the natural place to author it. `golden/` states what it does not cover.

**Cost.** Front-loaded design per category; once built, the corpus is reused indefinitely.

## E. Sources and golden data

### 46. Documentation about behaviour is not evidence the behaviour exists — in both directions

**Rule.** A comment, a spec or a manifest describing what something does is a claim, not a proof: verify it before trusting it. This applies equally to a claim of ABSENCE ("this doesn't work yet"), because absence claims go stale exactly as fast, and nobody re-checks a claim that reads as bad news.

**Why.** A document saying a feature "fails, pending a later round" can keep saying so, in several places, for days after the feature starts working.

**Here.** A datsan calendar *saying* it follows a system does not prove it does. A paper not mentioning an attribute does not prove the attribute is undefined. TO-VERIFY rows (001 §5.2) are claims in both directions.

**Cost.** Re-verify the claims a decision depends on, and correct a stale one the moment it is noticed.

### 47. A known-incomplete source is quarantined, not merely annotated

**Rule.** The moment a source is confirmed incomplete (limited, truncated, stale, contaminated), stop drawing NEW conclusions from it — not "draw them and caveat them". The guard is procedural: retire it, rename it, mark it DO-NOT-CITE, replace every live reference before moving on.

**Why.** An annotation records that a limitation was noticed; it does not stop the next finding — even one in the same document — from reaching for the same file.

**Here.** 001 §12 already applies it to Western lunar-day contamination: the whole source leaves, with the reason recorded. The same holds for a source found truncated, mistranscribed beyond repair, or of unknown edition: it leaves `golden/buryat/` (or its meta records a dated quarantine), the Q1 diff refuses it, and every reference is replaced in the same round.

**Cost.** Quarantine can look like discarding good rows; trusting the source claim by claim requires every future author to remember its blind spot, which an annotation cannot enforce.

### 48. An entity is identified by its structure, never by its name

**Rule.** A name is a label someone chose; it is never the object. Using a name, a suffix or a convention as the KEY that decides "is this the one I mean" fails both ways: a different object matches because it shares the name, and the intended one is missed because it does not follow the convention. Identify by a structural property — never by a name alone, nor by a name plus a check that itself only inspects the name.

**Why.** Name-based matching produces false positives and false negatives at once, and in practice the structural property that would have avoided both is usually already there, unused.

**Here.** Identify a calendar entity by a structural key — JDN; rabjung cycle plus year index; system, month number and leap flag; mansion index — never by a label. An element-animal year name repeats every 60 years; Mongolian and Tibetan cycle numbering differ by an offset (001 §5.1); transliterations vary. Golden rows join engine output on the Gregorian date. L2 conditions reference typed attributes, never strings (002 §4.7).

### 49. Shared data is never a scratchpad

**Rule.** Throwaway objects made to demonstrate or reproduce something live in scratch state the round also destroys — never in shared data every round depends on; where shared data must be used, every object created there is named and removed, or filed as left behind. A helper that writes fixture or placeholder files opens with `create_new` and refuses a file that already has bytes; a test that runs on real data calls no fixture writer.

**Why.** A probe left in shared data collides, rounds later, with an unrelated round that has no way to know it is there. A fixture writer reached by a test on real data empties real files, and the hollowed copy goes on serving as "the real data".

**Here.** Throwaway rows and files go in a test's temp dir or a scratch path the round deletes — never in `golden/`, `rules/` or `docs/`. 002 §6's seeded wrong row lives in test code, never in a golden file. Tests over T1–T3 golden data call no fixture writer.

### 50. A source is looked for in the library before it is fetched, and every citation carries one of three labels

**Rule.** A research leg follows a ladder and stops at the first rung that yields the source:

1. Read `docs/sources.md`, the index of the paper library (kept outside git), before any fetch.
2. One attempt per remaining source per host — never a second and third try at a host that refused.
3. Never `curl` or `wget`: they return HTML error pages or empty files, and leave junk behind.
4. Hand the human ONE batched link table, EARLY, and continue with everything that does not depend on it. What the human fetches goes into the library and gains its `docs/sources.md` row in the same round.
5. Whatever never arrives is labelled. Every citation carries exactly one of: **FETCHED** (read this round; a quotation of at most two sentences beside the claim, with URL or DOI; add **SECONDARY** if only a summary was read), **SNIPPET-VERIFIED** (a verbatim phrase search resolves to the page, which was not itself read), or **RECALLED / UNVERIFIED**. An unobtainable source also says **PAYWALLED** or **UNREACHABLE**.

**Why.** A paper fetched by one round into a place the next round cannot see gets labelled unverified while it sits on the machine, and every round re-learns which hosts refuse.

**Here.** The two-sentence cap also serves 001 §2: no reproduction of copyrighted text.

**Cost.** One file read per research leg, one index row per source obtained.

### 51. A dated record takes its date from the machine's clock, read this round

**Rule.** Before writing the first date into any record, run `date` and use that value everywhere — once, at the top. Never the session header's date, which can be stale by the time a long round writes its conclusions, and never a date inferred from the previous round's timestamps.

**Why.** Dates here are load-bearing: "the human ruled on <date>" is the audit trail that a human ruled at all, and a wrong date in a register is never recomputed by anything downstream.

**Here.** Golden `.meta.yaml` retrieval dates, `[CORRECTED yyyy-mm-dd: …]` markers, `DEBT.md` and `FORKS.md` entries, revision lines.

### 52. A count of a shape is not a claim about the engine until one instance has run

**Rule.** A census counts things by their shape, and often names what the engine would do with them. That is a claim about the engine, settled by running one instance of the shape on the current build before a round is built for it.

**Why.** A consequence read off a single reproduction can be the wrong one: the real shape, run once, can fail differently, elsewhere, and more often.

**Here.** A claim drawn from the weekly-issue corpus about what the engine or the rules validator does with some shape is run once before a round builds for it.

## F. Records and the tree

### 53. Negative debt entries: recording what must NOT happen yet

**Rule.** Not every debt entry is "build this later"; some are "do not relax this constraint until condition X holds", recorded with the same rigour. A negative entry prevents someone from "fixing" a deliberate narrowness by widening it before its precondition is met.

**Why.** A deliberately narrow check, flagged only as "seems restrictive", is exactly what a well-meaning later change loosens without knowing why it was narrow.

**Here.** `DEBT.md` D-1: "must NOT default `System` in clients until the Q1 verdict spec exists".

**Cost.** A "never do X" entry with no trigger condition is an opinion wearing a debt entry's clothes.

### 54. Fix at the definition site, not every call site

**Rule.** When a bug is a wrong invariant in a shared function's own contract, fix the function and let the compiler force every caller into correctness — never patch call sites one at a time while the shared function still lies about its contract.

**Why.** A call-site patch needs someone to remember every caller; a corrected signature makes missing one impossible.

**Here.** Fix the formula once in core, never patch a date in a wrapper or a client (002 §3: logic found in a wrapper moves to core).

**Cost.** A shared signature change has blast radius — the right trade whenever "every caller must change" is the property you want enforced.

### 55. A debt entry's stated blocker is a claim, and claims decay

**Rule.** A debt entry's reason for NOT building something was true when written, not forever. Before citing that blocker as the reason to defer again, grep the tree for whether the missing capability has appeared, and re-read the entry's reasoning against the current tree.

**Why.** A blocker can be true about dependencies and false about the work itself — the capability already in the tree under another name — and defers rounds of work that could have started at once.

**Here.** Blockers such as "not in Janson", "needs a T1 source" or "waits for F-n" are re-checked against `docs/sources.md`, `golden/` and `FORKS.md` before a round defers on them again.

**Cost.** One grep, before the blocker is used as load-bearing.

### 56. An accepted risk recorded only in a comment, or in a spec's prose, is not recorded

**Rule.** Accepting a known defect or shortcut means filing it in `DEBT.md`, with an id and a TRIGGER a later round can hit. A doc comment is read only by someone already in that function — the reader who least needs it; a spec's "noted, not decided" prose is the same failure in a different container. **Filing means a `DEBT.md` entry, and nowhere else counts.** Grep for the entry before writing "filed as debt" anywhere.

**Why.** A risk recorded where only its author stands stays invisible until something unrelated, written by someone who never read the comment, collides with it.

**Here.** Accepted risks go to `DEBT.md`; open questions go to `FORKS.md`. A comment may cite the entry, never replace it.

### 57. A declared property nothing enforces is never a premise

**Rule.** A declared property that nothing enforces is metadata for a human reader. It may be shown, explained or handed to a person; no computation may rest on it to change a result, unless a spec states the justification and a test checks that it holds.

**Why.** A system that trusts an unenforced declaration changes its results silently on the day the declaration is false.

**Here.** A source's stated computation system, a TO-VERIFY attribute, 001 §5.3's location independence, an L2 rule's `reviewed_by`: each may be shown to a reader; none is a premise the engine, the rules evaluator or a client default rests on.

### 58. Grep for your own markers before the commit, and verify the commit message against the staged diff

**Rule.** A round that writes a token to itself — a placeholder for a number not yet measured, a `TODO`, a status field to flip after a ruling — greps for it immediately before committing, and names the greps in its report. One marker word per round, so one grep finds every instance; the round's spec never spells it (it writes "the round's marker"), so the grep excludes only the frozen prompt copy. Further:

- **The check is the audit's own pattern**, fixed in advance from the field's defined shape (a register's header, an existing correct row) — never retyped from the line under test, which inherits its errors and passes because it was built to.
- **A commit message is a claim about the staged diff**, verified against that diff. An edit that fails aborts the commit: each edit asserts its anchor, and the edits, their assertions, the greps and `git commit` run as one `&&`-chain with no `;` in it. At a squash merge, `main`'s message is written from the final tree, never stitched from branch messages.
- **`git status --porcelain` is read BEFORE the chain** and compared with the files the round meant to change. An artifact the test framework wrote — a `proptest-regressions` seed, a snapshot — is named in the commit message and the report's diff scope, or removed.

**Why.** A placeholder standing where a measured number belongs makes a document read as complete. A message written alongside a script whose edit failed describes a change the tree does not have; a single `;` discards a failing test's exit code before `git commit` runs.

**Here.** Markers get written into `DEBT.md`, `FORKS.md`, spec status fields and TO-VERIFY rows. 002 §6 commits property-test failure seeds on purpose: each one is named.

**Cost.** One grep and one `git status` per commit.

### 59. Before the commit, grep the specs that own what you touched for forks nobody ruled

**Rule.** A round landing code under a crate greps every spec owning a contract there for forks with no ruling — over the whole file, and against `FORKS.md` — and names the grep in its report. A hit is put to the human, or filed in `DEBT.md` with the de-facto choice named as the finding.

**Why.** A spec's rule that forks are adjudicated binds the round that raises the fork, not the round months later that builds straight through it without reading it. And a fork's ruling often sits at the end of its spec, beyond any fixed window after its heading.

**Here.** A "Default:" line is not a ruling. 002's F6 (`no_std` without `alloc`), F7 (UniFFI) and F8 (rules evaluate) carry defaults an M1 round could build straight through; they stay OPEN in `FORKS.md` until the human rules.

### 60. The open-forks audit is two-sided

**Rule.** Before each commit, write `expected = previous OPEN count + forks raised + gated findings filed with options named and none chosen − rulings recorded`, run the audit grep (defined in `FORKS.md`'s header), and compare. A mismatch either way is a finding: too low is a row not filed, too high a ruling not recorded.

**Why.** Read one-sided — "does the count equal what I expected?" — the audit passes the round that forgot to file a row, because "unchanged" looks exactly like a pass.

### 61. A ruling is implemented against its own sentence, and a proxy for its condition is not the condition

**Rule.** Before implementing a ruling as a clear, drain, remove or reset, enumerate what the structure carries — for a map, its keys and its values separately — and name in the diff what the ruling reaches and what it must leave alone; re-read the ruling's own sentence as the checklist. When a ruling names a condition (a kind, a mode, a level), test that condition's own value; if a proxy is unavoidable, state where the two are equal and where they diverge.

**Why.** A one-line clear of a structure that carries two meanings implements more than the ruling; a proxy borrowed from a neighbouring line, where it was right for different stakes, diverges exactly where nobody looked.

**Here.** When a ruling names a tradition, a system or a tier, the code tests that value, never a correlate: `System::Genden` is not "Buryat" (001 §1).

### 62. A change the previous version would mishandle bumps a version it refuses on

**Rule.** A change after which the previous version would read or write data wrongly bumps a version that the previous version refuses on, with its migration and compatibility test. Every round that changes what is written or exchanged says, in its report, what the previous version does with the new form.

**Why.** Without a version that refuses, an older reader does not fail — it reads the new form wrongly and says nothing.

**Here.** Once data or clients ship: the `rules/` schema, the golden file layout, and the FFI/WASM types a shipped client reads.

## G. Builds and the gate

### 63. Flakes are classified by procedure, not by feeling

**Rule.** A failure that looks like a flake is classified by a fixed procedure: rerun it in isolation; if it still fails, stash the round's diff and rerun on pre-change code before classifying it. A failure that survives both is a finding, and is filed. Only a failure that clears the first isolated rerun may be reported "confirmed clean on isolated rerun". A red in the final gate is re-run instead of halting the round only if it is a FILED flake (a `DEBT.md` entry naming the test or its class) whose failure MATCHES that entry, with the circumstances recorded and the re-run stated with its counts. A stop rule naming one flake does not widen to another by resemblance.

**Why.** "Confirmed clean on isolated rerun" otherwise covers one rerun, three, and a stash comparison alike — and a real, pre-existing defect gets waved through as one more flake. More reruns only narrow confidence; only the pre-change comparison asks whether the round's diff caused it.

**Here.** The engine is deterministic: a property-test counterexample is never a flake — its seed is committed (002 §6) and it is a finding. Real flake sources here are network fetches in CI (`cargo deny`, `wasm-pack`) and, later, client tests.

### 64. A built artifact's provenance is verified before its result is trusted

**Rule.** Before a result taken from a built binary is believed, check that the file that ran is the build you think: from the tree you mean, under the plain configuration, after the last relevant change. A result that arrived faster, stranger or cleaner than expected is a provenance question first. A test spawns the binary cargo built FOR it (`CARGO_BIN_EXE_*`), never a path it merely finds. Every tree that builds binaries has its own target directory; a run of a built binary records its checksum beside the result; and after building the same target from more than one tree, the checksums are compared — they must DIFFER — before anything is compared.

**Why.** A stale binary left behind by a configuration change fails, or passes, for reasons that have nothing to do with the code. Two worktrees sharing a target directory can each be told the other's build is fresh, so a "comparison across trees" compares one binary with itself.

**Here.** The Q1 differential report, `wasm-pack` output and a client showing a date are all results of built artifacts: each records the commit and profile that built it, and a surprising result gets a plain rebuild before it is believed.

**Cost.** One plain rebuild or one checksum per measurement session.

### 65. The gate is proportioned to the diff, and every skipped leg is named

**Rule.** One written rule says which gate legs a diff owes: some always, others only when the diff reaches what they test. Every skipped leg is named in the gate line with its reason, and a count not comparable to the previous gate's says so. What a gate needs to run and to be reconciled lives in the tree, never only in a round's scratch.

**Why.** A gate run whole for every diff costs time on legs the diff cannot reach; a gate trimmed silently hides what it skipped; and tooling kept in scratch is lost at the first sweep.

**Here.** Always: 002 §7's gate. Only when reached: the WASM smoke test (when `crates/tshes-wasm` is touched), the Q1 report (when `golden/` or a system is touched), the rules validator (when `rules/` is touched). A docs-only round names the legs it skipped.

## H. Delegation

### 66. Delegation: re-verify before reporting as your own

**Rule.** A claim produced by someone or something else on your behalf is a claim to check, not to relay. Before it enters a record under your name, re-verify it against the source yourself. When two delegates disagree, resolve it by reading the source — never by averaging or splitting the difference: both may be right about different links of one chain, and a blended answer is a third, unverified claim wearing their credibility.

**Why.** Two investigations of one code path can reach opposite verdicts — "this can only fail silently" and "this eventually panics" — and both be right about different frames of the same call chain. Averaging them is confidently wrong; reading the chain end to end shows both.

**Here.** `CLAUDE.md` hard rule 11.

**Cost.** Reserve it for claims that go into a permanent record or are acted on directly.

### 67. A grant is named before a read outside the working directory, and a delegate's brief carries it

**Rule.** A prompt that sends a round to read anything outside its working directory names the GRANT in its header: the directories the human adds, the tools that read them, any script whose approval the human states. A round that meets a permission denial names each denied read with the reason given, and never works around it by another tool or route; where the prompt needs the read, it halts with a STOP AND ASK. A round that delegates work puts the `GRANTS:` line into the delegate's brief verbatim (`GRANTS: None` included), and the delegate's report lists every read it made outside the working directory, or says it made none; the round checks that list before using anything returned. A grant is a list of what may be used, not a promise of what exists.

**Why.** A read the prompt did not name is a read the harness may refuse, which halts a session for nothing; and a fence the harness does not enforce is kept only by whoever knows it — a delegate knows only what its brief says.

**Here.** The paper library and the prompts directory sit outside the repository (`docs/ROUNDS.md` §2.2).

---

# Part 2 — rules with a named precondition

### P1. IF you have a real reference to differential-test against, THEN every correctness claim is checked against its real output, not against your own understanding of the spec

Your own model and the reference's actual behaviour diverge in ways neither side notices until they are compared; the costliest defects can trace to a documented rule nobody actually ran against the system it describes. **Without a real reference** this degrades to "test against your own understanding" — exactly the thing worth suspecting most. Cost: a differential needs real, representative inputs; one against sanitised data proves less than it looks.

**Here — precondition met.** The reference is the published Buryat calendars; Lotsawa House is a secondary oracle for Phugpa only. **Mechanics from the oracle, semantics from the reference:** Lotsawa House can validate Phugpa arithmetic, but only Buryat sources can say what Buryat practice is. This is why T4 output, generated by software, never counts toward Q1.

### P2. IF you have a real-usage corpus, THEN it is both your demand evidence and your fuzzer

A sample of real usage answers two questions at once: what to build next (ranked by actual frequency, not guessed importance), and what is already broken (fed through the system, whatever fails is a real, prioritised list). **Without one**, demand is estimated from intuition, and synthetic data only finds the bugs its author already imagined. Cost: real noise — budget time to separate signal from it, and don't treat every failure as equally worth fixing (rule 10).

**Here — precondition partly met.** The weekly zurkhai issues are a corpus of what users look up: they rank L2 categories, and their tshes rows feed the engine through the Q1 diff. A corpus count is censored data: a category's absence can mean the source didn't cover it. Noise here includes Western lunar-day contamination (001 §6.3).

### P3. IF a piece of work is genuinely new, THEN research precedes it, across independent legs, reported even when empty

Before design starts: what does the authoritative reference say (including its own rationale, not only its behaviour); what do the field's practice and known failures say; what does the theory say, if any is relevant. Every leg is attempted and reported even when it finds nothing — a leg skipped because "it probably won't matter" is the one most likely to have mattered — and each says what it found AGAINST the approach, with the search named when nothing turned up. **Without genuine novelty** (a straightforward extension of a settled pattern), full research is waste: "this is settled, cite the precedent" is the correct, fast answer. Cost: real time; reserve it for decisions that would be expensive to redo.

**Here — precondition met** for Q1, every L2 rule category, and any new attribute. Legs: ① **reference**: datsan publications, and a зурхайч where reachable. ② **field**: other implementations (Lotsawa House, Henning's software, existing Mongolian apps); pin where their conventions differ. ③ **science**: Janson, Henning, Schuh, Reingold–Dershowitz, or an explicit "none relevant". ④ **Does this contradict anything already shipped?** A research round that lowers a spec's advertised value has succeeded. Citations resolve to a tree path (`docs/sources.md`, `golden/`), never to someone's notes. Rule 50 is the fetch-and-label procedure every leg follows.

### P4. IF you need a stop/go decision with real consequences, THEN build an explicit readiness gate, separate from the work that produced its evidence

A gate document states verdicts against criteria agreed in advance, cites its evidence, and refuses to make the final call itself — it measures; a human decides. A document that tries to do both shades the evidence toward the answer it wants to give. **Without a real stop/go decision**, a formal gate is theatre. Cost: a gate must be kept current by every round that changes what it measures, or it drifts stale — worse than none, because it looks authoritative.

**Here — precondition met, twice.** Gate 1: the Q1 verdict (M3). Gate 2: the first public release of a client — correctness, provenance display, tradition labelling, a reviewer's sign-off. Without a date driving it, a gate is allowed to say no.

### P5. IF machinery can be built ahead of its consumer, THEN track reachability separately from existence

"The code exists" and "a user can reach it" are different facts, and any layering — a mechanism built before the entry point that calls it — accumulates gaps between the two unless something checks automatically. **Without such layering** (a small system wired whole at birth), a reachability matrix is ceremony. Cost: the matrix and its tripwire need updating with every new capability, or they rot into false comfort.

**Here — precondition met from M1, not only once clients exist.** The engine will compute attributes (yoga, mewa, parkha…) before any L2 rule or client reads them, and 001 §7 blocks rules on attributes not yet exposed. The matrix (`docs/REACHABILITY.md`) is created at the first `tshes-wasm` or `tshes-ffi` export.

---

# Part 3 — what this method does not cover

- **Performance measurement.** The project makes no performance claim, so the method has no rules for timing, load, throughput or benchmark comparisons. If a claim appears (WASM bundle size, app cold start), its measurement rules land by an amendment spec first.
- **Concurrency and persistence.** The engine has neither (002 §4.5–4.6); there are no rules for locks, durability or crash recovery.
- **A separate process-flag series.** Forks are numbered `F-n`, globally, in `FORKS.md`; there is no second register for findings against the method — those are forks or `DEBT.md` entries.
- **Batch amendments.** An amendment lands as its own spec when it is needed (003 §9).

## Two warnings

**(a) Expect stretches with nothing visible to show.** Rounds of rigour — closing gaps, correcting stale claims, hardening instruments — can move no user-visible number for a long time, until one correctness fix unblocks a real, countable batch. The rigour finds real defects; the stretch is a real cost, and a reader deciding whether to adopt this method deserves to see that shape first.

**(b) The most frequent failure is the adjudicator's own, not the work's.** The likeliest mistake is mis-estimating how much a finding MATTERS — reasoning about what the domain plausibly contains instead of checking what the pipeline, as built, can actually reach. A theoretically large concern the pipeline cannot hit yet is not the same size as a small one in its live path (rule 14).

## Revisions

- r1 (2026-10-02): initial.
