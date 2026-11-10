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

### D-4 — 003 §7 names no non-Claude model
- **Filed:** 2026-11-10, spec 004 ADDENDUM 1 (R1, FORKS.md F12).
- **Kind:** METHOD
- **Statement:** 003 §7's model table names only Claude models. Round 004 runs outside that table under a one-round trial (DeepSeek V4 Pro, in CodeWhale). 003's next revision decides the general policy for non-Claude models, with round 004's landing as its evidence.
- **Pinned by:** not pinned
- **TRIGGER:** the next revision of 003.
- **Status:** OPEN

### D-5 — The pin 1.99.0 predates the FFI `bool` miscompilation fix in 1.99.1
- **Filed:** 2026-11-10, spec 004 ADDENDUM 2 (R3, FORKS.md F13).
- **Kind:** TOOLING
- **Statement:** The pinned toolchain, Rust 1.99.0, predates 1.99.1, which fixes "rustc: fix miscompilation of FFI `bool` return values on x86_64" (RELEASES.md, "Version 1.99.1 (2026-10-15)"; FETCHED by the advisor, 2026-11-10). The core has no FFI, so the risk is confined to the future FFI wrapper.
- **Pinned by:** not pinned
- **TRIGGER:** the round that creates crates/tshes-ffi. That round raises the pin to at least 1.99.1, deliberately, in its own PR (002 §2), or it states why not. It also states whether the MSRV moves with the pin.
- **Status:** OPEN

### D-6 — CodeWhale writes session state inside the repository
- **Filed:** 2026-11-10, spec 004 ADDENDUM 1 H4 (observed this round: .codewhale/state/).
- **Kind:** TOOLING
- **Statement:** The CodeWhale harness writes its own session state into the working directory (`.codewhale/state/`: subagents.v1.json, subagents.v1.lock, .gitignore). These files are never committed and never added to the repository's .gitignore (H4); every pre-commit `git status --porcelain` accounts for them.
- **Pinned by:** not pinned
- **TRIGGER:** the next round run in CodeWhale.
- **Status:** OPEN

### D-7 — The gate self-test leg is not in 002 §7's list
- **Filed:** 2026-11-10, spec 004 Item 4 (the self-test leg, Item 5, extends 002 §7).
- **Kind:** DOCS
- **Statement:** tools/gate.sh runs one leg beyond 002 §7's list: the gate self-test (spec 004 Item 5). 002's next revision should list it, so the spec's gate list matches the gate as built.
- **Pinned by:** not pinned
- **TRIGGER:** the next revision of 002.
- **Status:** OPEN
