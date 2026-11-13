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
- **Status:** RULED
- **Ruled:** 2026-11-13, (b), after M3: the approach carries spec 014's verdict, its 6 open days and the two Aginsky editions' disagreement as its questions; the contact is the human's. The human: "Agree with all recommendations, go." (spec 014 ADDENDUM 1, R9)

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
- **Status:** RULED
- **Ruled:** 2026-11-11, (c) — merged round branches go to a private mirror, the git remote `rounds`, and never to the public origin; after a merge's tree check the human pushes the branch there, checks that the mirror holds its tip, then deletes the local branch; the human: "agree with recommendations, go" (spec 010 R1).
- **Built in:** spec 010 (`docs/ROUNDS.md` §9, r3)
- Until ruled, each branch is kept locally, neither pushed nor deleted. (superseded: RULED 2026-11-11)

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

### F23 — Which Tsurphu the engine implements (spec 008 R3)
- **Raised:** 2026-11-11, by the advisor; filed, already ruled, by spec 008 Item 9.
- **Question:** Janson v1 describes one Tsurphu (A.2, pp. 38–40) and notes that some Tsurphu almanacs take the true solar longitude, and in some the true date, from the karana calculation (Remark 22, p. 40). Which does the engine's `System::Tsurphu` compute?
- **Options:** (a) A.2, the variant deferred; (b) both, the variant as a fourth system; (c) the variant only.
- **Status:** RULED
- **Ruled:** 2026-11-11, (a); the human: "Agree with all recommendations" (spec 008 R3). The advisor's reasons, RELAYED (measured 2026-11-11): Henning's published Tsurphu calendars (kalacakra.org) take the Sun's longitude from the karana calculation; his 2012 file differs from Janson v1's Table 8 Tsurphu column in months 4, 5 and 11, with every month length equal. The variant is `DEBT.md` D-19.
- **Built in:** spec 008

### F24 — What Henning's TCG output becomes in the tree (spec 009 Fork 1)
- **Raised:** 2026-11-11, spec 009 Fork 1 (Item 8, question a).
- **Question:** Spec 009 built Henning's TCG 1.06 from its published source and compared it with the engine on every day of 1900–2100 for all three systems (53 disagreeing days in all, one cause: F25). What does that output become in the tree, and at which tier of 001 §6.1 if it becomes a fixture? 001 §6.1 as written: T4 is "Engine vs Phugpa only"; T5 is "Tables printed in Janson / Henning".
- **Options:** (a) nothing beyond the spec 009 report; (b) a fixture under `golden/reference/`, for Phugpa only or for all three versions, as full rows or as the disagreement lists only, at a tier that needs an amendment of 001 §6.1 for every reading except Phugpa-as-T4; (c) a re-runnable tool under `tools/` that rebuilds TCG from the library and runs the differential; (d) (b) and (c); (e) something not listed. The costs are in spec 009's Fork 1.
- **Status:** RULED
- **Ruled:** 2026-11-11, (a) — nothing beyond spec 009's report: no tier, no golden row, no tool; `DEBT.md` D-24 names when the differential is run again; the human: "Agree with all recommendations" (spec 009 ADDENDUM 1, R6).

### F25 — Which anomaly increment a2 the engine's three systems compute (spec 009 Fork 2)
- **Raised:** 2026-11-11, spec 009 Fork 2 (Item 8, question d).
- **Question:** Janson v1 gives the Moon's anomaly increment per lunar day as a2 = 1/28, eq. (7.13), "usually used when calculating calendars", and as a2 = (1 + a1)/30 = 3781/105840, eq. (7.24), which Henning uses (Remark 14, pp. 25–26). The engine computes (7.13) for Phugpa, Tsurphu and Genden; TCG computes (7.24) for all three. They give different Tibetan dates on 17, 13 and 23 days of 1900–2100 (53 in all), and Janson's printed Tables 1, 7, 8 and 9 do not separate them. Which does each system of the engine compute?
- **Options:** (a) keep (7.13) for all three, and record the 53 days (a `DEBT.md` entry with a TRIGGER); (b) switch all three to (7.24); (c) both, as a named variant of each system (not a default, `DEBT.md` D-1); (d) decide at the Q1 differential, with the engine unchanged and an entry that names the 53 days; (e) something not listed. The costs are in spec 009's Fork 2.
- **Status:** RULED
- **Ruled:** 2026-11-11, (a) — a2 = 1/28, Janson v1 (7.13), for Phugpa, Tsurphu and Genden; the 53 days on which (7.24) gives other dates are `DEBT.md` D-23; the human: "Agree with all recommendations" (spec 009 ADDENDUM 1, R7). The advisor's reasons, RELAYED: (1) 001 §6.4's P6 requires Phugpa to reproduce the T4 output exactly over 1900–2100 ("Phugpa reproduces T4 output exactly over at least 1900–2100"), and the T4 fixture equals the engine's (7.13) dump on every day (spec 009's report, §8); (2) Janson v1 A.2 (p. 38) and A.3 (p. 42) give Tsurphu and Genden the Phugpa values of the mean motions, a2 included, each with "see Remark 14" (FETCHED, spec 009 session 2, from the library copy through `pdftotext -layout`: A.2, p. 38, "the same values as given there for the Phugpa version of the constants m1, s1 and a1 (and m2, s2 and a2, see Remark 14)"; A.3, p. 42, "the same in the New Genden version as in the Phugpa version, see Section 7, and thus so are m2, s2 and a2, see Remark 14"); (3) Remark 14 calls (7.13) the value usually used when calculating calendars (FETCHED, p. 25: "the simpler (7.13) is usually used when calculating calendars, for example in the almanac [14]").

### F26 — The commit email address on main, before the first public push
- **Raised:** 2026-11-11, by the advisor, before the first public push; recorded by spec 010 Item 0.
- **Question:** main's commits carried a personal email address of the human's, as author and committer. Publish them as they were, or rewrite them first?
- **Options:** (a) publish them as they were; (b) rewrite every commit on main, the initial commit included, to the human's GitHub no-reply address, keep the old history in the private mirror, and commit a map from old commit ids to new.
- **Status:** RULED
- **Ruled:** 2026-11-11, (b), the initial commit included. The human: "keep mail address private"; "yes force-push".
- **Built in:** the rewrite, by the human, before spec 010; the map, spec 010 (Item 3).

### F27 — The first client
- **Raised:** 2026-10-02, by the advisor: 001 §10's M5 reads "web or TG Mini App"; recorded by spec 011 Item 0
- **Question:** Which client does M5 build first?
- **Options:** (a) a website; (b) a Telegram Mini App.
- **Status:** RULED
- **Ruled:** 2026-10-02, (a), a website. The human registered the domain zurkhai.org for it the same day. The human's words were not recorded.
- **Built in:** not yet built; docs/ROADMAP.md's website row (spec 011)

### F28 — The reading of 001 §6.3's admission clause for a datsan's lunar calendar
- **Raised:** 2026-11-12, by the advisor, before the Sources round; recorded by spec 012 Item 0
- **Question:** None of the held datsan calendars names itself zurkhai or the Tibetan calendar; each says «Лунный календарь». Does 001 §6.3's "states it is zurkhai / Tibetan calendar" admit them?
- **Options:** (a) yes, for a Sangha datsan's own Buddhist lunar calendar whose numbers behave like tshes, shown by a step check, with a DEBT.md entry to clarify §6.3; (b) no, only a source that names зурхай or the Tibetan calendar; (c) case by case, in each source's meta file.
- **Status:** RULED
- **Ruled:** 2026-11-12, (a). The human: "1 agree". The DEBT.md entry is D-31.
- **Built in:** spec 013 (the step check of golden/buryat/, run by its scripts; under the gate from spec 014, D-37)

### F29 — The tier of the Ivolginsky datsan's 2021 calendar
- **Raised:** 2026-11-12, by the advisor, before the Sources round; recorded by spec 012 Item 0
- **Question:** The calendar is the datsan's own print file, on its website. T1 or T2 (001 §6.1)?
- **Options:** (a) T1, an official calendar of a Sangha datsan, with edition and compiler; its meta file says it is the digital original, not a scan; (b) T2, the datsan's own web publication.
- **Status:** RULED
- **Ruled:** 2026-11-12, (a), T1. Admission still waits on the terms check of 001 §8 (spec 012). The human: "2 agree".
- **Built in:** spec 013 (golden/buryat/ivolginsky-datsan-2021.csv, T1)

### F30 — The tier, and the editions, of the Aginsky datsan's calendars
- **Raised:** 2026-11-12, by the advisor, before the Sources round; recorded by spec 012 Item 0
- **Question:** The 2025 and 2026 editions disagree on 9–16 January 2026. What tier, and how are the editions recorded?
- **Options:** (a) both T1, one golden file per edition, the disagreement recorded in both and neither corrected; (b) the 2025 edition only; (c) both, with the disagreeing days left out.
- **Status:** RULED
- **Ruled:** 2026-11-12, (a). Admission still waits on the terms check of 001 §8 (spec 012). The human: "3 agree".
- **Built in:** spec 013 (golden/buryat/aginsky-datsan-2025.csv and aginsky-datsan-2026.csv, T1, the disagreements noted in both)

### F31 — What the golden is_leap_month column records
- **Raised:** 2026-11-12, by the advisor, before the Sources round; recorded by spec 012 Item 0
- **Question:** A source may name the extra month by a different convention from Janson v1's. Does the column record the source's label, or a normalised one?
- **Options:** (a) what the source prints, its convention stated in the meta file, the cell empty with a note where nothing is printed, and the Q1 round mapping conventions; (b) normalised to Janson v1's convention at transcription; (c) empty for every Buryat source.
- **Status:** RULED
- **Ruled:** 2026-11-12, (a). The human: "4 agree".
- **Built in:** spec 013 (is_leap_month per the source's label; golden/buryat/README.md)

### F32 — What shows that a datsan belongs to the Buddhist Traditional Sangha of Russia
- **Raised:** 2026-11-12, by the advisor, after spec 012; recorded by spec 013 Item 0
- **Question:** Spec 012 reached no register and no Sangha list. What counts as proof of membership (001 §1)?
- **Options:** (a) the publisher names the Sangha as its own body, on the calendar or its site, the line quoted in the meta file, register evidence added later; (b) only a register entry or the Sangha's own list, every holding a candidate until then; (c) (a) and a second, independent confirmation.
- **Status:** RULED
- **Ruled:** 2026-11-12, (a). The human: "Agree with all recommendations, go".
- **Built in:** spec 013 (the meta files of golden/buryat/)

### F33 — The admission of the Ivolginsky datsan's 2021 calendar under its site's terms
- **Raised:** 2026-11-12, by the advisor, after spec 012; recorded by spec 013 Item 0
- **Question:** The site allows its material for non-commercial activity only and requires written permission otherwise; golden/ is CC BY 4.0. Admit it?
- **Options:** (a) admit, facts only, the footer quoted in `rights`; (b) ask the datsan first, no rows until it answers; (c) do not admit; (d) (a) now, and the human asks the datsan, a refusal being the TRIGGER to withdraw the rows.
- **Status:** RULED
- **Ruled:** 2026-11-12, (d). The human: "Agree with all recommendations, go". The DEBT.md entry is D-35.
- **Built in:** spec 013

### F34 — The admission of the Aginsky datsan's 2025 and 2026 editions, no terms found
- **Raised:** 2026-11-12, by the advisor, after spec 012; recorded by spec 013 Item 0
- **Question:** No terms of reuse were found on four saved site pages or in either PDF's text layer. Admit both editions?
- **Options:** (a) admit both (T1, F30), `rights` saying no terms were found and naming what was read; (b) wait until the site's «Лунный календарь» page is saved and read.
- **Status:** RULED
- **Ruled:** 2026-11-12, (a). The human: "Agree with all recommendations, go".
- **Built in:** spec 013

### F35 — Golden rows from the Gunzechoinei and Khoymorsky calendars
- **Raised:** 2026-11-12, by the advisor, after spec 012; recorded by spec 013 Item 0
- **Question:** The library holds one to three legible months of each. Transcribe them now?
- **Options:** (a) no, a DEBT.md entry with the TRIGGER "a full printed calendar in the library"; (b) the legible months of Gunzechoinei only; (c) the legible months of both.
- **Status:** RULED
- **Ruled:** 2026-11-12, (a). The human: "Agree with all recommendations, go". The DEBT.md entry is D-34.
- **Built in:** not built; DEBT.md

### F36 — What the golden month column holds for a Buryat source
- **Raised:** 2026-11-12, by the advisor, after spec 012; recorded by spec 013 Item 0
- **Question:** 001 §5.1 makes month a number 1 to 12; the Buryat calendars print a season label per Gregorian month, or none. What does the column hold?
- **Options:** (a) the number the label implies in the seasonal order, by lunation, the label in notes, «Дополнительный» leap with an empty month, empty where nothing is printed; (b) empty for every Buryat source, the labels in notes only; (c) normalised to Janson v1's numbering.
- **Status:** RULED
- **Ruled:** 2026-11-12, (a). The human: "Agree".
- **Built in:** spec 013

### F37 — Whether M0's done-when is met by the three golden/buryat/ files
- **Raised:** 2026-11-12, by the advisor at spec 013's landing; recorded by spec 014 Item 0
- **Question:** 001 §10's M0 asks for "≥ 12 months of Buryat tshes data from ≥ 2 independent T1–T3 sources". The tree holds the Ivolginsky datsan's 2021 calendar and the Aginsky datsan's 2025 and 2026 editions, all T1 (spec 013). Is M0 met?
- **Options:** (a) met, unconditionally; (b) not met until the Ivolginsky datsan answers the request for written permission (DEBT.md D-35); (c) met, on the condition that a refusal under D-35 which withdraws the Ivolginsky rows reopens M0, because only one datsan would remain.
- **Status:** RULED
- **Ruled:** 2026-11-13, (c). The human: "Agree with all recommendations, go merge".
- **Built in:** spec 014 (DEBT.md D-35; docs/ROADMAP.md, Gate 1's state cell)

### F38 — The outcome of the Q1 differential: a verdict, or stop-and-consult (spec 014 Fork 1)
- **Raised:** 2026-11-13, spec 014 Fork 1 (Item 6, the halt)
- **Question:** Spec 014 measured the three golden/buryat/ files against the engine's Phugpa, Tsurphu and Genden (1,213 rows; Genden 1,199 matches and 14 mismatches, Tsurphu 29, Phugpa 68; Genden matches the Ivolginsky file on 398 of 398 rows). Which outcome of 001 §10 M3 does the verdict spec record: a verdict, or the stop-and-consult outcome of 001 §6.5?
- **Options:** (1) a verdict naming one system, with its scope (which datsans and dates the evidence covers) and how its 14 listed mismatches are recorded; (2) the stop-and-consult outcome of 001 §6.5 (the data to a зурхайч; F4 is OPEN); (3) more data before any verdict (DEBT.md D-33, D-34); (4) something the numbers suggest, as spec 014's report lists it. The costs are in spec 014's report, "STOP AND ASK — spec 014 Fork 1".
- **Status:** RULED
- **Ruled:** 2026-11-13, (1), in the shape of spec 014 ADDENDUM 1's R5: Genden, scoped to the three golden/buryat/ files of two datsans, under 001 §6.5's second rule; the 2026 edition's 8 days recorded as a disagreement between editions, the 2025 edition's 6 as open; four reopening conditions (spec 014's verdict, its Reopening section). The human: "Agree with all recommendations, go."
- **Built in:** spec 014 (the verdict spec, Item 13)

### F39 — What a client shows as the advice for a day
- **Raised:** 2026-11-13, the advisor's question I2 to the human (recorded by spec 015 Item 0)
- **Question:** When a client shows a day, how are the interpretations that hold on it presented?
- **Options:** (a) every sourced rule whose condition holds that day, each with its source and tradition, contradicting rules side by side; a combined verdict only where a source itself states how its rules combine (001 §7, §10 M5); (b) a combined verdict per category; (c) prose generated from the rules (against 001 §2, "No authored interpretations")
- **Status:** RULED
- **Ruled:** 2026-11-13, (a). The human: "Agree with all recommendations" (spec 015, I2)

### F40 — Whether a Tibetan interpretive rule set may serve as an oracle against Buryat datsans' printed advice
- **Raised:** 2026-11-13, the advisor's question A to the human (recorded by spec 015 Item 0); the human ruled that it is decided after spec 015 lands, on its numbers
- **Question:** May the rules of a Tibetan (non-Buryat) interpretive source, such as M. Erlewine's "Tibetan Astrology" (Tsurphu), be applied to dates and compared with what Buryat datsan calendars print, as a measurement kept outside rules/?
- **Options:** (a) yes, as an interpretation oracle outside rules/, which needs CLAUDE.md hard rule 8 and 001 §2 amended first; a match would not show that a datsan uses that source; (b) no
- **Status:** OPEN

### F41 — A second interpretive tradition beside the Buryat one
- **Raised:** 2026-11-13, the advisor's question B to the human (recorded by spec 015 Item 0)
- **Question:** Should clients offer, beside the Buryat rules, a separately labelled Tibetan tradition (for example recommendations labelled as Erlewine's "Tibetan Astrology"), computed on its own system's dates?
- **Options:** (a) not now: 001 §1–2 stand; revisit no earlier than M4/M5; (b) yes, which needs a revision of 001 §1–2, CLAUDE.md hard rule 8 changed, and the publisher's written permission; (c) revisit at M5
- **Status:** RULED
- **Ruled:** 2026-11-13, (a). The human: "Agree with all recommendations" (spec 015, B)
