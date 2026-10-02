# NNN — <name>

```
spec: NNN
prompt: docs/specs/prompts/NNN-short-name.md
branch: spec/NNN-short-name (cut from main at <sha>)
merge SHA: not yet merged
pushed: no
model: <model>, effort <level>
gate: <N> passed, <M> failed (debug) / <N> passed, <M> failed (release) — <failed names>; skipped legs: <names, reasons>
```

<!-- The head block above opens every round's file (docs/ROUNDS.md §4.1). A spec drafted outside a round leaves it out. -->

Status: draft | accepted | superseded by NNN
Depends on: <specs>
Resolves: <F-n>, if any
Milestone: <M-n>
Governs: <crates or paths>

## Purpose

<One paragraph: what this spec decides, and what it does not.>

## Properties

```
P1  <a machine-checkable property, e.g. to_gregorian(to_zdate(d)) == d for every d in range>
    Test: <test name>
    Shapes: <the special shapes the test's data must contain>
P2  ...
```

Every property is checkable by one of:
- a property test;
- a fixture test against golden data, with its tier named;
- the Q1 differential;
- a seeded control.

If a property can only be verified by reading the code, it is prose, not a property: rewrite it, or move it to Non-properties.

## Non-properties

- <What this spec deliberately does not cover, and where it is covered instead.>

## Forks

### F-n — <subject>

<The question, the options with their costs, and the spec's own reading if it has one. A reading is not a ruling. Every fork has a `FORKS.md` row.>

## Stop rules

- <A named condition and a named action.>

## Reference

<Sources, with their `docs/sources.md` rows and citation labels.>

## Revisions

- r1 (YYYY-MM-DD): initial.

— spec text ends here —

## Report

<A round's report follows the end marker, in the order of `docs/ROUNDS.md` §4.2. It ends with `## The squash commit message`.>
