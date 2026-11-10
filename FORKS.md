# FORKS — the fork register

Every decision the human must rule on, in one place: the human's queue. A fork raised in a spec, in a round's STOP AND ASK, or by a `DEBT.md` entry whose fix shapes are named and none chosen gets a row here **in the same commit that raises it**. When the human rules, the ruling is written onto the same row, and its `Status:` line changes in the same edit.

**Identifiers.** `F-n`, numbered globally in filing order; the number is taken from this file when the row is filed. A fork a round raises in a STOP AND ASK is also `spec NNN Fork K`; its row carries both.

**Row shape** — fixed, because the audit reads it:

```
### F-n — <subject>
- **Raised:** <date>, <where: spec and section, or spec NNN Fork K>
- **Question:** <one line>
- **Options:** (a) … (b) … — a spec's own "Default:" is listed as its reading, never as a ruling
- **Status:** OPEN | RULED | SUPERSEDED | WITHDRAWN
- **Ruled:** <date, the option, the human's words where short>        (RULED rows)
- **Built in:** <spec NNN>                                             (once built)
```

SUPERSEDED and WITHDRAWN carry their reason. Rows are never deleted.

**The audit** (METHOD rule 60): compute the expected count first, then `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`.

---

### F1 — Engine language
- **Raised:** 001 §8.
- **Question:** Which language and toolchain does the engine use?
- **Options:** (a) Rust core with UniFFI (Android) and wasm-bindgen (web, Telegram Mini App); (b) Kotlin Multiplatform; (c) TypeScript only.
- **Status:** RULED
- **Ruled:** (a), Rust, edition 2024 — 001 §8, spec 002.

### F2 — Licences
- **Raised:** 001 §8.
- **Question:** Which licences cover the code, and which cover golden data and rule tables?
- **Options:** Code: `MIT OR Apache-2.0` (ruled in 002 §2). Data: (a) CC BY 4.0, so attributions travel with the data (001's proposal); (b) another licence.
- **Status:** OPEN
- **Ruled:** In part — code `MIT OR Apache-2.0` (002 §2). The data licence stays open; it is decided before the first data commit.

### F3 — Naming
- **Raised:** 001 §11.
- **Question:** What are the repository, engine crates and app brand called?
- **Options:** As ruled below.
- **Status:** RULED
- **Ruled:** Repository `zurkhai`; engine crates `tshes-*` (tradition-neutral, because the engine implements several systems); app brand "Хүрдэн — зурхай" (001 §11). Two checks remain before the first client and can change only the app brand: `DEBT.md` D-3.

### F4 — When to approach Гунзэчойнэй datsan
- **Raised:** 001 §11.
- **Question:** Approach the datsan before or after the Q1 verdict (M3)?
- **Options:** (a) Before: access to T1 data. (b) After: a working engine to show.
- **Status:** OPEN

### F5 — Buryat-language strings
- **Raised:** 001 §11.
- **Question:** Who provides the Buryat-language strings, and who reviews them?
- **Options:** None proposed yet.
- **Status:** OPEN

### F6 — `no_std` without `alloc` in the core
- **Raised:** 002 §4.6, §9.
- **Question:** Is `tshes` `#![no_std]` with no `alloc`?
- **Options:** (a) `no_std` and no `alloc`, unless M1 finds a concrete need, recorded in a spec (002's reading); (b) `no_std` with `alloc`; (c) `std`.
- **Status:** RULED
- **Ruled:** 2026-10-02, (a): `no_std` with no `alloc` in `tshes`, unless M1 finds a concrete need, recorded in a spec. The human: "agree".

### F7 — Android bindings
- **Raised:** 002 §9.
- **Question:** UniFFI or hand-written JNI for Android?
- **Options:** (a) UniFFI (002's reading); (b) hand-written JNI.
- **Status:** OPEN

### F8 — Rule evaluation
- **Raised:** 002 §9.
- **Question:** Does `zurkhai-rules` evaluate rule conditions in Rust, shared by every client, or only validate them?
- **Options:** (a) Evaluate, so Android and web cannot diverge (002's reading); (b) validate only.
- **Status:** OPEN

### F9 — The golden `skipped` column
- **Raised:** 001 §6.2, §11.
- **Question:** What does the golden CSV's `skipped` column hold?
- **Options:** (a) A boolean; (b) the tshes number omitted immediately before this day, mirroring `ZDate.skipped_before` (001 §5.1).
- **Status:** OPEN
- Decided before the first golden row is transcribed (M0).

### F10 — The tier of third-party software whose system is unknown
- **Raised:** 001 §11.
- **Question:** At what tier are the outputs of an app whose computation system is unknown recorded (the RuStore app named in 001 §11)?
- **Options:** (a) T4 as written, although T4 is defined as validating the engine against Phugpa only (001 §6.1); (b) a tier of its own; (c) a research-leg finding only, until its system is identified.
- **Status:** OPEN
- Decided before its outputs are recorded.

### F11 — Round branches and the public remote
- **Raised:** `docs/ROUNDS.md` §9.
- **Question:** After a round is squash-merged, where does its branch — with its halts, resumes and STOP AND ASKs — live?
- **Options:** (a) Pushed to the public origin, publishing the drafts. (b) Kept locally only, so the history lives on one machine. (c) Pushed to a private mirror.
- **Status:** OPEN
- Until ruled, each branch is kept locally, neither pushed nor deleted.

### F12 — A round running on a model outside 003 §7's table
- **Raised:** 2026-11-10, spec 004 ADDENDUM 1 (the advisor's fork 1).
- **Question:** May a round run on a model outside 003 §7's table, and how is it recorded?
- **Options:** (a) a one-round trial on round 004 (DeepSeek V4 Pro, in CodeWhale), recorded in this row; 003's next revision decides the general policy. (b) Revise 003 §7 first. (c) Keep 004 on Sonnet 5.5, in Claude Code.
- **Status:** RULED
- **Ruled:** 2026-11-10, (a). V4 Pro over V4 Flash, also ruled 2026-11-10.

### F13 — Which Rust stable does round 004 pin
- **Raised:** 2026-11-10, spec 004 Fork 1 (Item 1's stop: rustup's current stable is newer than the repository's creation date).
- **Question:** Pin 1.99.0 (stable current at the repository's creation, 2026-10-02) or 1.99.1 (rustup's current stable, released 2026-10-13 [CORRECTED 2026-11-10: the date in rustup's version string "(58cf8f9c3 2026-10-13)" is the commit date, not a release date; 1.99.1 was released on 2026-10-15 per RELEASES.md, as in R3])?
- **Options:** (a) 1.99.0 — 002 §2's literal reading. (b) 1.99.1 — rustup's current stable. (c) another version the human names.
- **Status:** RULED
- **Ruled:** 2026-11-10, (a) — pin 1.99.0, MSRV 1.99.0 (002 §2's reading).
