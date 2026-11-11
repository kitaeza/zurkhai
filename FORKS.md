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
- **Status:** RULED
- **Ruled:** Code `MIT OR Apache-2.0` (002 §2). Data: 2026-11-10, option (a), CC BY 4.0 — the human: "1, CC BY 4.0" (spec 006 R1).
- **Built in:** spec 006 (`golden/LICENSE.md`)

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
- **Status:** RULED
- **Ruled:** 2026-11-11, (b) — the tshes number omitted immediately before the day, mirroring ZDate.skipped_before; where a source prints no skip marker, derived from the neighbouring days, and the meta says "derived"; empty, with a note, where the day's predecessor is not in the source; the human: "Ok agree" (spec 007 ADDENDUM 1, R4).
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

### F14 — The zone regime of the T4 fixture (spec 006 Fork 1)
- **Raised:** 2026-11-10, spec 006 Fork 1 (STOP RULE (5): the generator's output differs between time zones).
- **Question:** The library's Gregorian dates depend on the machine's time zone (`dist/index.js:344` and the local-time getters of `getDateStr`, lines 72–77): under `TZ=Pacific/Kiritimati` every date 1900–1994 is one day early. Under which regime is the T4 fixture generated, and what does "byte-identical across zones" then mean?
- **Options:** (a) the generator sets `process.env.TZ = 'UTC'` for itself before loading the library, so its output does not depend on the caller's zone; the three-zone runs then show the generator is zone-independent, not the library; the Kiritimati defect is reported as a finding. (b) the generator refuses to run in any zone whose offset is not 0 across the range, and the fixture is defined as the output under `TZ=UTC`; Pago_Pago is shown equal as evidence only. (c) the generator cancels the library's offset by reading `new Date().getTimezoneOffset()` itself (the prompt forbids the clock). (d) another regime the human names.
- **Status:** RULED
- **Ruled:** 2026-11-11, (a) — the generator pins its own zone to UTC, checks it on every day, and refuses otherwise; the human: "agree" (spec 006 ADDENDUM 1, R4).
- **Built in:** spec 006 (tools/t4-tdc.mjs)

### F15 — The kind of day that 001 §5.2's rows 6–8 name (spec 007 Fork 1)
- **Raised:** 2026-11-11, spec 007 Fork 1 (Item 7, question c).
- **Question:** Where Janson v1 defines a day attribute for both the calendar day (a function of the JD) and the lunar day (a function of the tshes and the month), which does 001 §5.2's "day" name?
- **Options:** (a) both, as separately named attributes; (b) the calendar day only; (c) the lunar day only; (d) defer the choice, and with it rows 6–8, until a Buryat source shows which kind it uses (none is in the library).
- **Status:** RULED
- **Ruled:** 2026-11-11, (d) defer — deferred with rows 6–8 (D-18); the round that reopens them puts this question again; the human: "Agree with all recommendations" (spec 007 ADDENDUM 1, R3).

### F16 — Which of several quantities 001 §5.2's rows name for one unit (spec 007 Fork 2)
- **Raised:** 2026-11-11, spec 007 Fork 2 (Item 7, question c).
- **Question:** Janson v1 defines three numbers for a year (central, life, power) and five elements for a calendar day (power, life, fortune, body, spirit); which does 001 §5.2 name in rows 6 and 7?
- **Options:** (a) the smallest set: a year's central number and a day's power element; (b) all of them; (c) defer; (d) another set the human names.
- **Status:** RULED
- **Ruled:** 2026-11-11, (c) defer — deferred with rows 6–8 (D-18); the round that reopens them puts this question again; the human: "Agree with all recommendations" (spec 007 ADDENDUM 1, R3).

### F17 — The identification of "mewa" and "parkha" (spec 007 Fork 3)
- **Raised:** 2026-11-11, spec 007 Fork 3 (Item 7, question d).
- **Question:** The words "mewa" and "parkha" occur in no source in the library; by structure (cycle length, colours, directions, names) they are Janson's "number" (Table 16, p. 74) and "trigram" (Table 15, p. 74). Does the project take them to be those quantities?
- **Options:** (a) accept both as INFERRED working identifications, recorded as such, the code spelled in Janson's words; (b) accept neither until a source names the words; (c) accept one only; (d) defer.
- **Status:** RULED
- **Ruled:** 2026-11-11, (d) defer — deferred with rows 6–8 (D-18); the round that reopens them puts this question again; the human: "Agree with all recommendations" (spec 007 ADDENDUM 1, R3).

### F18 — The numbering the engine exposes for trigrams and for a day's element and animal (spec 007 Fork 4)
- **Raised:** 2026-11-11, spec 007 Fork 4 (Item 7, question e).
- **Question:** Janson v1 says its numbering of the trigrams is "perhaps not traditional" (p. 74) and that Henning numbers the day cycles differently (Remark 36, p. 82). What numbering, if any, does the engine expose?
- **Options:** (a) typed enums only, no numeric index in the public API; (b) Janson's numbers, with his caveat documented; (c) Janson's and Henning's (needs Henning's book); (d) defer.
- **Status:** RULED
- **Ruled:** 2026-11-11, (d) defer — deferred with rows 6–8 (D-18); the round that reopens them puts this question again; the human: "Agree with all recommendations" (spec 007 ADDENDUM 1, R3).

### F19 — Row 6, day element / animal: implement, defer or find a source (spec 007 Fork 5)
- **Raised:** 2026-11-11, spec 007 Fork 5 (Item 7, questions a and b).
- **Question:** Janson v1 defines the day's element and animal for the calendar day (App. E.4, p. 82) and the lunar day (App. E.3, p. 81), VERIFIED under 001 §5.3. Does the resume implement them from Janson v1, defer them, or first obtain a further source?
- **Options:** (a) implement what Forks 1–4 admit, from Janson v1, under `System::Phugpa`; (b) first obtain an independent day-cycle source (Yap, Janson v1's ref. [20]; or Henning's archive, ref. [24]), then implement; (c) defer, with a `DEBT.md` entry; (d) another course.
- **Status:** RULED
- **Ruled:** 2026-11-11, (c) defer — T1 and T2 are not implemented (D-18); the human: "Agree with all recommendations" (spec 007 ADDENDUM 1, R3).

### F20 — Row 7, mewa: implement, defer or find a source (spec 007 Fork 6)
- **Raised:** 2026-11-11, spec 007 Fork 6 (Item 7, questions a and b).
- **Question:** For the nine-numbers quantity Janson v1 defines the year (E.3–E.5), the calendar day (E.4) and the lunar day (E.11), and gives a month number for Tsurphu only (E.8): NOT IN JANSON V1 for Phugpa. What does the resume do with each line?
- **Options:** (a) implement the year and day lines that Forks 1–4 admit, from Janson v1; defer the month line with a `DEBT.md` entry; (b) first obtain Henning's book (where Janson sends for details), then decide, the month line included; (c) defer all four lines; (d) another course.
- **Status:** RULED
- **Ruled:** 2026-11-11, (c) defer all four lines, T3–T6 (D-18); the human: "Agree with all recommendations" (spec 007 ADDENDUM 1, R3).

### F21 — Row 8, parkha: implement, defer or find a source (spec 007 Fork 7)
- **Raised:** 2026-11-11, spec 007 Fork 7 (Item 7, questions a and b).
- **Question:** For the trigram quantity Janson v1 defines the calendar day (E.4) and the lunar day (E.10), and defines none for a year or a month: NOT IN JANSON V1. What does the resume do with each line?
- **Options:** (a) implement the day lines that Forks 1–4 admit, from Janson v1; defer the year and month lines with a `DEBT.md` entry; (b) first obtain Tseng (Janson v1's refs. [17, 18]) or Henning's book (ref. [7]), then decide, the year and month lines included; (c) defer all four lines; (d) another course.
- **Status:** RULED
- **Ruled:** 2026-11-11, (c) defer all four lines, T7–T10 (D-18); the human: "Agree with all recommendations" (spec 007 ADDENDUM 1, R3).

### F22 — The admission of the datsan «Ринпоче Багша»'s lunar calendar as a Buryat source
- **Raised:** 2026-11-11, by the advisor between rounds 006 and 007; filed by spec 007 ADDENDUM 1.
- **Question:** May the lunar calendar of the datsan «Ринпоче Багша» (Ulan-Ude) supply Buryat golden rows?
- **Options:** (a) admit it now as T2, the datsan's own website; (b) a candidate only: its `docs/sources.md` row and the library, and no golden rows until the M0 round's leg ① checks Sangha membership (001 §1), a statement that it is a zurkhai / Tibetan calendar (001 §6.3), and the publisher's terms (001 §8); (c) do not use it.
- **Status:** RULED
- **Ruled:** 2026-11-11, (b) — a candidate only; the human: "Ok agree" (spec 007 ADDENDUM 1, R5).
