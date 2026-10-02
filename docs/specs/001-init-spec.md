# 001 — Project charter and initial spec

Repository: `zurkhai`. Engine crates: `tshes`, `tshes-ffi`, `tshes-wasm`. App brand: **Хүрдэн — зурхай** (pending F3 checks).
Status: accepted (r3), 2026-10-02
Scope of this spec: what the project is, what it is not, the one question that must be answered before anything else is built, the data model, the verification strategy, and the first milestones.

---

## 1. Purpose

An open, verifiable implementation of the **Buryat zurkhai calendar** (Зурхай, as practised in datsans of the Buddhist Traditional Sangha of Russia), delivered as:

1. **A computation engine** (the calendar itself) — open source, deterministic, tested against published calendars.
2. **Rule tables** (interpretations: haircut days, travel, birth-year compatibility, etc.) — structured data, every rule carrying its source and tradition.
3. **Clients** — Android app, website, and possibly a Telegram Mini App — built on top of 1 and 2.

**Tradition: Buryat.** The project follows the Buryat zurkhai tradition as practised in the datsans of the Buddhist Traditional Sangha of Russia. Mongolian and Tibetan interpretive traditions are out of scope.

**Tradition is not the same thing as computation system.** "Buryat" decides whose calendars are the ancestor and whose interpretations enter L2. It does *not* decide which arithmetic (Phugpa, Genden, …) Buryat calendars use. That remains Q1 (§4), answered only by data. If Buryat datsans turn out to compute with the Mongolian Genden system, the engine uses Genden, and the project is still Buryat.

The project's value proposition is **correctness and provenance**: every date the engine produces can be traced to a published algorithm and checked against published calendars, and every interpretation can be traced to a named source.

## 2. Non-goals

- **No authored interpretations.** The project does not invent or "fill in" astrological meanings. Interpretations enter only from cited sources, and are reviewed by a practising зурхайч where possible.
- **No claim of religious authority.** The app states which tradition and which source it follows. It does not present itself as an official Sangha product unless and until a datsan endorses it.
- **No Mongolian or Tibetan interpretive traditions** in L2. Mongolian and Tibetan *sources* may still serve as oracles for the engine's arithmetic (tiers T4/T5), never as evidence of Buryat practice.
- **No reproduction of copyrighted calendar texts.** Golden data stores *facts* (dates, lunar-day numbers, leap-month flags), not the published prose or design of any calendar.
- **No Western "lunar day" astrology.** The Russian web is flooded with "лунный календарь стрижек" content based on Western astrology. This project's "lunar day" is the Tibetan *tshes* (tithi) number, which is a different quantity. The two must never be mixed, including in golden data collection (see §6.3).

## 3. Layers

| Layer | What | Nature | Changes how |
|---|---|---|---|
| L1 Engine | Gregorian ↔ zurkhai date; per-day computed attributes | Algorithmic, deterministic | Only by spec + tests |
| L2 Rules | Interpretations keyed on L1 attributes | Data, sourced, tradition-tagged | By data PR with citation |
| L3 Clients | Android, web, TG Mini App | UI | Freely |

L1 knows nothing about L2. L2 references only attributes L1 exposes. L3 never computes calendar facts itself.

## 4. The question that gates everything — Q1

**Which computation system do Buryat datsans' calendars follow?**

Candidates, all described in Janson (see §9):

- **Phugpa** — the standard Tibetan calendar (Dalai Lama's tradition).
- **Mongolian / New Genden (Төгс буянт)** — a reformed version used in Mongolia. Same mean-motion constants as Phugpa but different epoch values and a modified intercalation rule; its leap months coincide with Tsurphu's, while individual days can fall one day apart.
- **Tsurphu** — included for completeness; unlikely but cheap to rule out.
- **None of the above / a local variant** — must be allowed as an outcome.

**Rule: Q1 is answered by differential testing against Buryat golden data (§6), not by assumption, by documentation, or by a single matching Sagaalgan date.** A claim that "Buryat = X" needs the same evidence standard as a claim that "Buryat ≠ X".

Until Q1 is answered, L1 implements all candidate systems behind a `System` parameter, and no client ships a default.

## 5. L1 data model

### 5.1 Zurkhai date

```
ZDate {
  system:        Phugpa | Genden | Tsurphu
  year:          { rabjung_cycle, year_in_cycle (1..60),
                   element, gender, animal }
  month:         1..12, is_leap_month: bool
  tshes:         1..30           // lunar day number (tithi)
  is_doubled:    bool            // second occurrence of a repeated tshes
  skipped_before: Option<tshes>  // tshes omitted immediately before this day
}
```

Mapping: `fn to_zdate(gregorian, system) -> ZDate` and `fn to_gregorian(ZDate) -> Result<Date, Error>` (a typed `Err(SkippedDay)` for a skipped tshes, 002 §5). Internally via Julian Day Number.

Year naming: element + animal (and gender) in Russian, Buryat, and Tibetan transliteration. The Mongolian 60-year cycle numbering differs from the Tibetan rabjung numbering by an offset — the engine stores one canonical form and renders both.

### 5.2 Per-day computed attributes (exposed to L2)

Only attributes with a published algorithmic definition belong in L1. Each must cite its source section when implemented.

| Attribute | Notes | Source status |
|---|---|---|
| weekday + planet | trivial | — |
| tshes number | core | Janson |
| lunar mansion (27) | | Janson — verify section |
| yoga (27) | | Janson — verify section |
| karana | | Janson — verify section |
| day element / animal | | **TO VERIFY** source |
| mewa (мэнгэ, 9) for year/month/day | | **TO VERIFY** source (Janson or Henning) |
| parkha (парка, 8) for year/month/day | | **TO VERIFY** source |
| month: leap flag, length | | Janson |
| year: element, gender, animal, mewa, parkha | | Janson / Henning |

Rows marked TO VERIFY are not implemented until a source is cited in the PR. "It is commonly known" is not a source.

### 5.3 Location dependence

Per the published descriptions, the traditional calendar is an arithmetic calendar and does not depend on observer location. **This is a claim to verify in M1, not an assumption to build on.** If any attribute turns out to be location- or timezone-dependent, it gets an explicit parameter, never an implicit default.

## 6. Verification strategy

### 6.1 Oracles, by provenance tier

| Tier | Source | Validates |
|---|---|---|
| T1 | Printed or official calendar from a Sangha datsan (scanned, with edition and date) | Buryat practice |
| T2 | A datsan's own web or social publication | Buryat practice |
| T3 | Media republication of a datsan's zurkhai (e.g. weekly issues by the abbot of Kurumkan datsan in KP-Irkutsk) | Buryat practice, with attribution caveat |
| T4 | Third-party software (e.g. Lotsawa House's Phugpa calculator, built on Janson) | **Engine vs Phugpa only** — never Buryat practice |
| T5 | Tables printed in Janson / Henning | Engine vs the paper's own system |

Every golden row records its tier, source, and retrieval date. T4 and T5 data never count as evidence for Q1.

### 6.2 Golden data format

```
golden/buryat/<source-slug>.csv
gregorian_date, tshes, month, is_leap_month, is_doubled, skipped, notes
```

The meaning of the `skipped` column (a flag, or the tshes number omitted before this day, as in `ZDate.skipped_before`) is open: F9 (§11).

Plus `golden/buryat/<source-slug>.meta.yaml`: tier, publisher, datsan, lama (if named), URL or scan path, retrieval date, edition year, who transcribed it. Facts only, no copied prose.

### 6.3 Collection hazards

- **Western lunar days.** Some publications mix Western astrological "лунные дни" with zurkhai. A source is admitted only if it states it is zurkhai / Tibetan calendar and its numbers behave like tshes (30-slot cycle with skips and doubles).
- **Transcription errors.** Each source is transcribed once and spot-checked by a second pass; disagreements are recorded, not silently fixed.
- **Partial weeks.** Weekly issues give 7 days at a time; a full year needs ~52 issues. Coverage gaps are recorded explicitly.

### 6.4 Engine properties (must hold for every system)

- P1. Every Gregorian day maps to exactly one ZDate.
- P2. `to_gregorian(to_zdate(d)) == d` for every day in the test range.
- P3. Within a month, tshes are non-decreasing; a number is either present once, present twice (doubled), or absent (skipped). No other pattern occurs.
- P4. Months have 29 or 30 days; leap months appear at the frequency the system's intercalation rule implies.
- P5. Reproduces the system's own published tables (T5) exactly.
- P6. Phugpa reproduces T4 output exactly over at least 1900–2100.
- P7. Range: 1900–2100 minimum. Behaviour outside the range is defined, not undefined.

### 6.5 The Q1 differential report

For each candidate system, over all Buryat golden rows:

- match count, mismatch count, coverage (days with data / days in range)
- every mismatch listed with its date, both values, and source tier
- **null results are reported**, not dropped

Verdict rules:

- One system matches 100% of T1–T3 rows with ≥ 12 months of coverage → Q1 answered, record it in a spec.
- Best system matches but with a small number of mismatches → each mismatch is investigated (transcription? source error? genuine variant?) before any verdict.
- No system matches well → **stop**, report, and take the data to a зурхайч. **Do not tune constants to fit the data.** A fitted engine with no published basis is exactly what this project exists to avoid.

## 7. L2 rule tables (schema only in this spec)

```
rule {
  id
  tradition:     "buryat-sangha" | ...
  source:        { tier, publisher, lama, work, page/issue, date }
  category:      haircut | travel | construction | medical | wedding | ...
  condition:     expression over L1 attributes and, optionally,
                 the user's birth-year animal/element
  verdict:       favourable | unfavourable | neutral | text_key
  text:          { ru, bxr?, en? }  // short; paraphrase, not copied prose
  reviewed_by:   optional, name of reviewing зурхайч + date
}
```

- Contradictory rules from different sources coexist, each tagged; clients choose by tradition and source.
- A rule with no source is rejected at CI.
- Rules depending on attributes L1 does not yet expose are blocked, not approximated.

The categories the weekly Kurumkan issues use (haircut consequences by day, travel, favourable/unfavourable activities, per-animal luck) are the first target for the schema — they show what Buryat users actually look for.

## 8. Architecture

Fork F1 — engine language:

| Option | For | Against |
|---|---|---|
| **A. Rust core + UniFFI (Android) + wasm-bindgen (web, TG Mini App)** | one engine, exact integer/rational arithmetic, small fast WASM | two binding toolchains |
| B. Kotlin Multiplatform | native on Android, JS target for web | JS target is heavier; less natural for a pure-math library |
| C. TypeScript only | simplest web + TG; React Native / Capacitor for Android | weakest fit for exact arithmetic; engine and app entangled |

**Decided: A (Rust, edition 2024).** See spec 002 for the code standard.

Fork F2 — licences: code MIT or Apache-2.0 (or both); golden data and rule tables CC BY 4.0 so attributions travel with the data. Before admitting any source, check whether its publisher has terms that restrict reuse of its facts.

### Repository layout (proposed)

```
/crates/tshes      L1 engine (no I/O, no clock, no locale); see spec 002 §3
/crates/tshes/tests property tests P1–P7, T4/T5 fixture tests
/crates/tshes-ffi  UniFFI wrapper (002 §3)
/crates/tshes-wasm wasm-bindgen wrapper (002 §3)
/crates/zurkhai-rules  L2 schema, loader, validator (002 §3)
/golden/buryat     T1–T3 data + meta
/golden/reference  T4/T5 data
/rules             L2 tables (data)
/tools/zurkhai-diff  Q1 differential report generator (002 §3)
/clients/web
/clients/android
/docs/specs        numbered specs, this is 001
/docs/sources.md   bibliography with what each source is trusted for, and
                   the index of the paper library (outside git, 003 §11)
```

## 9. Sources

- S. Janson, *Tibetan Calendar Mathematics*, arXiv:1401.6285 (latest version). Phugpa in full; Tsurphu, Mongolian/New Genden, Bhutanese as differences. **Primary algorithmic source.**
- E. Henning, *Kalacakra and the Tibetan Calendar*, AIBS, 2007; and kalacakra.org.
- E. Reingold, N. Dershowitz, *Calendrical Calculations*, chapter on the Tibetan calendar (Phugpa).
- O. Salmi, *Mongolian Calendar* (tsalo.fi) — working notes on the Mongolian variant.
- Lotsawa House Phugpa calculator and its free-software library — T4 oracle for Phugpa only.
- Buryat golden sources: to be collected in M0. Candidates: Гунзэчойнэй datsan (St. Petersburg) annual calendar; Kurumkan datsan weekly zurkhai (via KP-Irkutsk); Ivolginsky datsan / Sangha publications; Sagaalgan dates as announced by the Sangha.

## 10. Milestones

| M | Deliverable | Done when |
|---|---|---|
| M0 | Golden data: ≥ 12 months of Buryat tshes data from ≥ 2 independent T1–T3 sources | CSVs + meta committed, coverage report |
| M1 | Engine: Phugpa | P1–P7 pass; spec 002 CI gate green; §5.2 TO-VERIFY rows resolved or explicitly deferred |
| M2 | Engine: Genden + Tsurphu | P1–P5 pass for each against Janson's tables |
| M3 | **Q1 verdict** | Differential report committed; verdict spec written, or stop-and-consult outcome recorded |
| M4 | L2 schema + validator + first sourced table (haircut by tshes) | CI rejects unsourced rules |
| M5 | First client (web or TG Mini App) | Shows date, tshes, year, sourced rules, with tradition and source visible |
| M6 | Android | Same engine via bindings |
| M7 | Review by a зурхайч | Written feedback recorded; endorsement if given |

M0 is manual work and can run in parallel with M1–M2. M3 needs both.

## 11. Open forks

- ~~F1 Engine language~~ — decided: Rust (spec 002).
- F2 Licences (§8) — code decided by 002 §2 (`MIT OR Apache-2.0`); the data licence (CC BY 4.0 proposed) is open — decide before the first data commit.
- F3 Naming — **decided** (r3): repo `zurkhai`, engine crates `tshes-*` (tradition-neutral, because the engine implements several systems), app brand "Хүрдэн — зурхай". Still open before the first client: (a) a Buryat speaker confirms the spelling of Хүрдэн; (b) the name is run past a lama when approaching Гунзэчойнэй (F4). If either objects, only the app brand changes.
- Field note for the Q1 research round: RuStore already lists an app titled "Зурхай: Буддийский календарь и атлас" that claims verification against Buryat sources. It goes into research leg ② as a candidate oracle; Its outputs are recorded at the tier F10 decides: T4 as defined in §6.1 validates the engine against Phugpa only, and this app's computation system is unknown.
- F4 Whether to approach Гунзэчойнэй datsan before or after M3. Before gives access to T1 data; after gives a working engine to show.
- F5 Buryat-language strings: who provides and reviews them.
- F9 The golden CSV's `skipped` column (§6.2): a boolean, or the tshes number omitted immediately before this day (mirroring `ZDate.skipped_before`, §5.1). Decide before the first golden row is transcribed (M0).
- F10 The tier for third-party software whose computation system is unknown (the RuStore app above): T4 as written, a separate tier, or recorded only as a research-leg finding until its system is identified. Decide before its outputs are recorded.

## 12. Stop rules for this spec

- If M2 shows Janson's descriptions are insufficient to implement Genden or Tsurphu exactly, stop and record what is missing. Do not fill gaps from unlabelled sources.
- If M3 finds no system matches the Buryat data, stop (see §6.5). Do not tune constants.
- If any golden source turns out to mix Western lunar days with tshes, remove the whole source, not just the offending rows, and record why.

---

## Appendix A — first session prompt (M1, engine: Phugpa)

This appendix is the content of the first round, not a runnable prompt. The advisor writes the runnable prompt to `~/zurkhai-prompts/` in the header shape of `docs/ROUNDS.md`, assigns its spec number, and may split these tasks across more than one round (`CLAUDE.md` hard rule 1).

```
TARGET: CLAUDE CODE SESSION
MODEL: Sonnet.  EFFORT: High.
PRECONDITION: repo contains docs/specs/001–003, CLAUDE.md, GATED.md, DEBT.md.
              If not, STOP and report. Next free spec number from the TREE.
ARTIFACT: Janson, "Tibetan Calendar Mathematics", arXiv:1401.6285, latest
          version. Look in the paper library /home/kitaeza/zurkhai-papers/
          (indexed in docs/sources.md) first; if absent, fetch it there and
          add its row to docs/sources.md. The PDF stays outside git. If it
          cannot be fetched, STOP and report — do not implement from memory
          or secondary pages.
STOP RULE: if any constant or formula needed for Phugpa is not stated in the
           paper, STOP at that point and report the section and the gap.
           Do not substitute values from other sources without listing them.

TASKS
1. Set up the workspace, toolchain pin, lints and CI exactly per spec 002.
   CI must be green on an empty core before any calendar code lands.
2. Implement Phugpa per Janson: Gregorian <-> ZDate (§5.1), via JDN.
   Cite the paper's section number in a comment at every formula.
3. Implement property tests P1–P4 and P7 over 1900–2100.
4. Transcribe the Phugpa Losar / year tables from the paper into
   golden/reference/ as T5 fixtures; implement P5.
5. Build P6: generate Lotsawa House Phugpa output for a sample of years
   (or use their free library directly) as T4 fixtures; compare.
6. For each row in spec §5.2 marked TO VERIFY: find whether Janson defines
   it. Report section or "not in Janson". Implement only those found.
7. Verify or refute §5.3 (location independence) from the paper's text.

REPORT — per CLAUDE.md contract. Decisive item first:
- P5 and P6 result (pass / fail with first mismatch: expected, actual, source row).
- Then every property P1–P7: DONE / DEFERRED / N-A, with evidence.
- Shape coverage of the property-test range: counts of leap months, skipped
  tshes, doubled tshes actually exercised (003 §3, rule 4).
- §5.2 table with each row's source status.
- §5.3 verdict with the quoted section reference.
- Any fork encountered, even if resolved.
```

## Revisions

- r1: initial.
- r2: F1 decided (Rust, spec 002); Appendix A aligned with specs 002 and 003.
- r3: tradition fixed as Buryat (§1, §2), with the tradition / computation-system distinction stated; F3 naming decided; layout uses `crates/tshes`.

— spec text ends here —
