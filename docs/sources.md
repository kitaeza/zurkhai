# Sources — what each source is, and what it is trusted for

The bibliography of 001 §9, the index of the paper library, and the reference facts research rounds establish.

**The library lives outside git** (`~/zurkhai-papers/`; the path is in `docs/ADVISOR.md` §2). A public repository redistributes nothing whose licence has not been checked. Golden data stores facts, never a calendar's published text or design (001 §2).

**The rule:**
- **Read before fetching.** Read this file before any fetch (METHOD rule 50).
- **One row per source, same commit.** A round that obtains a source adds its row here in the same commit.
- **Labelled citations.** Every citation carries one of FETCHED, SNIPPET-VERIFIED, or RECALLED / UNVERIFIED.

## The library

| File | Source (full citation, version) | Obtained (date, from) | sha256 | Licence | Trusted for |
|---|---|---|---|---|---|
| `janson-1401.6285v1.pdf` | S. Janson, "Tibetan calendar mathematics", arXiv:1401.6285v1 [math.HO], 24 Jan 2014 (v1; the only version arXiv lists); 86 pages, page 1 carries the identifier arXiv:1401.6285v1 (verified by spec 005) | 2026-11-10 17:20:32 +0300 — the file's modification date, from `stat`, not a retrieval date; placed in the library by the human | `7cafc7df563a3020849c86f4e397daa18b7f7f5ff46244403180aca90b9d0f77` | arXiv non-exclusive distribution licence 1.0 (arxiv.org/licenses/nonexclusive-distrib/1.0/); the PDF stays outside git (CLAUDE.md hard rule 17) | Phugpa in full; Tsurphu, Mongolian / New Genden and Bhutanese as differences. The primary algorithmic source, and the T5 tables. |

## Bibliography

| Source | Trusted for | Not trusted for | In the library |
|---|---|---|---|
| S. Janson, *Tibetan Calendar Mathematics*, arXiv:1401.6285 (latest version) | Phugpa in full; Tsurphu, Mongolian / New Genden and Bhutanese as differences. The primary algorithmic source, and the T5 tables. | Buryat practice | yes (v1) |
| E. Henning, *Kalacakra and the Tibetan Calendar*, AIBS, 2007; kalacakra.org | Cross-checking Janson. 001 §5.2 names it as a possible source for the TO-VERIFY attributes. | Buryat practice | no |
| E. Reingold, N. Dershowitz, *Calendrical Calculations*, chapter on the Tibetan calendar | Phugpa, as an independent statement of the algorithm | Buryat practice | no |
| O. Salmi, *Mongolian Calendar* (tsalo.fi) | Working notes on the Mongolian variant: the field leg | Buryat practice; anything not checked against Janson | no |
| Lotsawa House Phugpa calculator and its free-software library | T4: the engine against Phugpa arithmetic only (001 §6.1) | Buryat practice; anything but Phugpa | — |
| Buryat golden sources, to be collected in M0. Candidates: Гунзэчойнэй datsan (St. Petersburg) annual calendar; Kurumkan datsan weekly zurkhai (via KP-Irkutsk); Ivolginsky datsan and Sangha publications; Sagaalgan dates as announced by the Sangha | Buryat practice, at the tier each earns (T1–T3) | Anything before its source passes the admission checks of 001 §6.3 | — |

## Reference facts

Facts about Buryat practice established by research leg ① (`docs/METHOD.md` P3). A spec that cites such a fact points here, never at someone's notes. Each fact is tagged one of three ways:
- **DOC**: a T1–T3 source, cited with its location;
- **ASKED**: a dated question to a datsan or a зурхайч, with the answer;
- **UNVERIFIED**: stated, together with what would verify it.

None yet.
