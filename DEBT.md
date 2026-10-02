# DEBT — what the project owes, and what it must not do yet

Deliberate debt, negative debt (a constraint that must NOT be relaxed until a condition holds), and accepted risks. This is not a bug list (fix those) and not a list of open questions (those are `FORKS.md`). **Filing means an entry here**, with an id and a TRIGGER. A code comment or a spec's prose is not filed (METHOD rule 56).

**Entry shape:**

```
### D-n — <one-line statement>
- **Filed:** <date>, <where it was found>
- **Kind:** CORRECTNESS | PROVENANCE | COVERAGE | NEGATIVE | TOOLING | DOCS | METHOD
- **Statement:** <what is owed or forbidden, and why>
- **Pinned by:** <test name> | not pinned
- **TRIGGER:** <a checkable condition under which the entry is acted on>
- **Status:** OPEN | PAID <date, commit> | RETIRED <date, reason>
```

**How entries are kept:**

- **Numbers.** `D-n` numbers are taken from this file when the entry is filed and are never reused.
- **No deletions.** An entry is never deleted. When it is paid or retired, its `Status:` changes and is dated.
- **Corrections.** A correction is made in place, with `[CORRECTED yyyy-mm-dd: …]`.
- **Length.** An entry stays short. A long account belongs in the spec that found it, cited here.
- **Forks.** A gated finding whose fix shapes are named and none chosen also gets an OPEN row in `FORKS.md`.

---

### D-1 — No client defaults `System` before the Q1 verdict
- **Filed:** 2026-10-02, 001 §4, 003 §3.
- **Kind:** NEGATIVE
- **Statement:** No client, wrapper or rule may default the computation system (Phugpa, Genden, Tsurphu) until the Q1 verdict spec exists. Until then the engine takes `System` as an explicit parameter everywhere. A default chosen earlier would assert an answer to Q1 that only the data can give.
- **Pinned by:** not pinned
- **TRIGGER:** the Q1 verdict spec is accepted (M3).
- **Status:** OPEN

### D-2 — The glossary does not exist
- **Filed:** 2026-10-02, 002 §8.
- **Kind:** DOCS
- **Statement:** 002 §8 requires calendar terms to use the transliteration given in `docs/glossary.md`, one spelling per term. The file does not exist yet, and its spellings must come from sources, not from memory.
- **Pinned by:** not pinned
- **TRIGGER:** the first round that names a calendar term in code (M1) creates it.
- **Status:** OPEN

### D-3 — Two naming checks before the first client
- **Filed:** 2026-10-02, 001 §11 (F3).
- **Kind:** PROVENANCE
- **Statement:** (a) A Buryat speaker confirms the spelling of "Хүрдэн". (b) The name is run past a lama when Гунзэчойнэй datsan is approached (F4). If either objects, only the app brand changes.
- **Pinned by:** not pinned
- **TRIGGER:** before the first client is released (M5).
- **Status:** OPEN
