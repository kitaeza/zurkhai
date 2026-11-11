spec: 009
prompt: docs/specs/prompts/009-henning-differential.md
branch: spec/009-henning-differential (cut from main at 07b5bbf)
merge SHA: not yet merged
pushed: no
model: session 1 (Items 0–8, the halt): Sonnet 5.5 (claude-sonnet-5-5), effort High; session 2 (ADDENDUM 1, Items 9–13): Sonnet 5.5 (claude-sonnet-5-5), effort High; the switch point to the fallback model (Opus 5.5) was never reached in sessions 1 and 2: no design choice fell to it; session 3 (ADDENDUM 2, Items 14–16): Sonnet 5.5 (claude-sonnet-5-5), effort High; the switch point was not reached and no design choice arose
gate: 147 tests passed, 0 failed (debug) / 147 passed, 0 failed (release) — no failure; run once by session 2 on the gated commit f63ad2f (no tracked source changed since 07b5bbf), 11 legs run, 11 passed, 0 failed; skipped legs: wasm-pack (crates/tshes-wasm does not exist, 002 §7). Session 1's halt ran none: halt by design, no tracked source changed; session 3 ran none: every leg skipped, because no file the gate reads changed after f63ad2f (Session 3)

Status: draft — the round is complete; status is set to accepted at landing
Depends on: 001, 002, 003, 005, 006, 008
Resolves: F24 and F25 (raised by this round; RULED (a) each)
Milestone: M2, its cross-check before M3
Governs: nothing built (F24 (a), F25 (a))

— spec text ends here —

# Report — spec 009 (the Henning differential)

Dates come from `date`, run at the top of the round: 2026-11-11 (the machine's clock stayed on 2026-11-11 to the end, METHOD rule 51). Evidence tags (docs/ROUNDS.md §4.3): MEASURED (this round), RELAYED (the human or the advisor, never re-measured here), INFERRED. Janson v1 means S. Janson, "Tibetan calendar mathematics", arXiv:1401.6285v1, the library copy (86 pages, sha256 `7cafc7df…0f77`); every Janson citation below is **FETCHED** (read this round from the library copy through `pdftotext -layout` and the Read tool). "TCG" is E. Henning's program, version 1.06, built here from its published source; Henning's source is cited by file:line and described, never copied [CORRECTED 2026-11-11 (spec 009 session 2, Item 11): untrue at the halt commit `6248886`, as the records at the §4 heading and at the opening of §21 say; this is a third statement of the same kind, which ADDENDUM 1 does not name, and S2.3 lists the copies that were removed]. The round ran in one session on one branch [CORRECTED 2026-11-11 (session 2): this describes the halt. The round ran in two sessions on one branch: session 1 halted at Item 8 (`6248886`), session 2 resumed it under ADDENDUM 1; see the Session 2 section]. Research legs: not applicable. 003 §4 sets their precondition as Q1, every L2 rule category and any new attribute; this round measures an existing computation against a second implementation. Leg ② of 003 §4 names "Henning's software" for the rounds that need a field leg; this is not one of them.

## 1. The decisive item: Item 6's counts and Item 7's table, summing

### 1.1 Item 6 — TCG against the engine on every day of 1900-01-01..2100-12-31 (73,414 days per version)

The engine's rows are `to_zdate` with the system named explicitly, from a `git archive` copy of `2739375` (Item 5); TCG's are read from its "Print calendar" files (Item 4); both are one row per Gregorian day in the T4 fixture's columns. MEASURED, `diffrows.py` (last section).

| Version | Days that differ, by field: year / month / leap flag / tshes | Disagreeing days | Runs of consecutive disagreeing days | Longest run | Tibetan years (of 202) whose set of months / leap month / first day (201 compared) differs |
|---|---|---|---|---|---|
| Phugpa | 0 / 1 / 0 / 17 | **17** | 17 (all of length 1) | 1 | 0 / 0 / 0 |
| Tsurphu | 0 / 0 / 0 / 13 | **13** | 13 (all of length 1) | 1 | 0 / 0 / 0 |
| Genden | 0 / 0 / 0 / 23 | **23** | 23 (all of length 1) | 1 | 0 / 0 / 0 |
| **Sum** | | **53** = 17 + 13 + 23 | 53 | 1 | 0 / 0 / 0 |

Every disagreeing day is a run of length 1, and every one (53 of 53) has a doubled or skipped tshes, in at least one of the two calendars, on that day or the day next to it (days −1, 0, +1). **Base rate (METHOD rule 22), MEASURED on the engine's dumps:** the share of all 73,414 days lying within one day of a doubled or skipped tshes is 13,176 (0.1795) for Phugpa, 13,166 (0.1793) for Tsurphu and 13,183 (0.1796) for Genden; 53 of 53 is far above 0.18, and the reason is the mechanism of §1.2 (a lunar day ending a hair before or after midnight moves a boundary between a doubled or skipped day and its neighbour). STOP RULE (5) (more than 300 days) did not fire: the largest count is 23.

**Which fields a disagreement can show, and which it cannot (METHOD rule 25).** Neither side prints the true date in these rows. A row carries the year, month, leap flag and tshes of a Gregorian day, so a disagreement shows only which lunar day names a calendar day; the true date (the fraction of a day that decides the label) is not on either side's row. The Item 7 script, which computes it, is the instrument that sees it. The rows cannot show the day of week, the Sun's or Moon's longitude, the time a lunar day ends, or any astrological field (TCG prints some of them; the engine computes none of them). The first day of a Tibetan year is the first row of the year, a derived value that both sides determine in the same way.

**For Phugpa, TCG against the T4 fixture directly** (`golden/reference/tdc-97d4a00-phugpa-1900-2100.csv`): the same 17 days; the list is identical to the list against the engine's dump (`diff` of the two list files silent), as it must be, because the dump check (Item 5) passed. **The advisor's RELAYED paragraph, re-derived (CLAUDE.md hard rule 10):** the six years of Item 2 hold 2,159 day-rows in TCG's output (355 + 355 + 384 + 355 + 355 + 355) and in the fixture; 2 of the 2,159 differ from the fixture (MEASURED), and both days are in this round's Phugpa list: **2001-02-10 and 2025-11-19** are both there. No conclusion is drawn from the paragraph itself.

### 1.2 Item 7 — which side follows Janson v1, where the two computations part, and the class table

**(i) The reference script** (`janson_ref.py`, last section; written from the paper only, exact `Fraction` arithmetic, no floating point). Its instruments, before it was trusted (METHOD rule 31):

| Control | What was run | Expected | Observed (MEASURED) |
|---|---|---|---|
| Its constants against the paper | each printed fraction against its printed radix form (10 pairs) and the relations the paper states ($m_1 = 30 m_2$, $s_1 = 30 s_2$, (7.24) $= (1 + a_1)/30$, $\beta + \beta^* \equiv 6, 52 \pmod{65}$, (5.15)); the forward month table against (5.19)–(5.22) with the printed β, for 1880..2130 | all equal | all equal; 0 of 3,105 months disagree per version. (A first run printed one BAD line: Tsurphu's $m_0$ radix form is 4 + 0.2351…, the integer 4 being part of the JD offset 2353741 + 4 = 2353745 that the paper adds, Remark 8; the check compared fractional parts wrongly. The check was changed to compare fractional parts, the paper's own integer split being the cause, not the paper) |
| Table 8 of the T5 files (2012) | all three versions | reproduced | **reproduced**: Phugpa 23 of 23 events, Tsurphu 21 of 21, Genden 21 of 21, 0 missing and 0 extra each (and under (7.24) too) |
| 30 days listed before the run (`thirty-days.txt`, written before the first run of the script, which `date` timestamped 18:35:33) | 10 per version: 2 or 3 doubled days, 3 skipped, 3 leap-month days, 2 ordinary | all reproduce the engine's dump | **30 of 30** |
| Every day, against the engine's dump | 73,414 days × 3 versions | (not required) | **0 days differ** in each version (13.9 s) |
| One constant seeded wrong | `--seed` on one constant of one version; expected: Table 8 of that version fails and so do that version's listed days | fails both | **trials, all reported:** Phugpa $m_0$ + 1/5656 d: Table 8 passes, days pass (**cannot resolve** a shift of 15 s on these 2012 events and these 10 days, METHOD rule 18). Tsurphu $a_0$ + 1/392: Table 8 FAILS (one event, skipped 6 for skipped 5 in month 9), days pass. Genden $s_0$ − 1/402: neither fails. Phugpa $m_0$ + 1/100 and Phugpa $a_0$ + 100/3528: Table 8 fails (that version only), days pass. Tsurphu $m_0$ + 1/100: Table 8 fails, 1 of 10 Tsurphu days fails. Genden $m_0$ + 1/50: Table 8 fails, 2 of 10 Genden days fail. **Genden $s_0$ − 40/402: both fail**: Table 8 of Genden reports 15 events missing and 15 extra, 5 of the 10 Genden days differ (1937-01-04, 2023-03-02, 2007-09-24, 2079-08-24, 1984-06-23); Phugpa's and Tsurphu's Table 8 and all 20 of their days still reproduce, so the seed reaches only the version it was planted in |

**The script against the two implementations, on all 73,414 days of each version (MEASURED):**

| Script with | against the engine | against TCG |
|---|---|---|
| $a_2 = 1/28$, eq. (7.13), p. 23 (the paper's main text) | 0 / 0 / 0 days differ (Phugpa / Tsurphu / Genden) | 17 / 13 / 23 differ |
| $a_2 = 3781/105840$, eq. (7.24), p. 25 (Remark 14: Henning's value) | (17 / 13 / 23 differ, the same days) | **0 / 0 / 0 differ** |

So **the engine is Janson v1's formulas with (7.13), and TCG is Janson v1's formulas with (7.24)**, on every one of the 220,242 days; nothing else differs between the three implementations on this range. Which side "follows Janson v1" is therefore not a question with one answer: both do, each at a different one of two formulas the paper prints for the same quantity (Remark 14, pp. 25–26: "the simpler (7.13) is usually used when calculating calendars, for example in the almanac [14]"; Henning "uses instead the exact value", "also used in his computed calendars"). Janson computed this very difference for Phugpa: "9 examples in 1900–1999 and 8 examples in 2000–2099", listing 10 February 2001, 10 May 2006 and 19 November 2025 (p. 26). This round's Phugpa list has 9 days in 1900–1999 and 8 in 2000–2099 and contains those three dates.

**For each disagreeing day, the true dates of the lunar days that decide its label** (`settle`, last section). For every one of the 53 days exactly one lunar day N (of the lunar days within one day of it) has $\lfloor \text{true date} \rfloor$ different under (7.13) and (7.24); the table gives its true date relative to the integer boundary k (the JD at which the lunar day ends under one value of $a_2$ and one day earlier under the other), in days, by (7.13) and by (7.24). The script's label under (7.13) equals the engine's and under (7.24) equals TCG's on 53 of 53 days. "Marks" are for days −1, 0, +1 (D: the second day of a repeated number; S: a number skipped immediately before; . neither), engine / TCG; a leap month is written 2L; cells are year/month/tshes.

**Phugpa (17):**

| Date | Engine | TCG | Marks | N | td(7.13) − k | td(7.24) − k |
|---|---|---|---|---|---|---|
| 1914-02-17 | 1913/12/22 | 1913/12/23 | ..S / .S. | 411112 | +0.0002054 | -0.0002796 |
| 1922-08-14 | 1922/6/22 | 1922/6/21 | ..D / .D. | 414261 | -0.0003449 | +0.0001181 |
| 1929-02-01 | 1928/12/23 | 1928/12/22 | ..D / .D. | 416662 | -0.0001868 | +0.0001042 |
| 1946-05-30 | 1946/3/29 | 1946/3/30 | ..S / .S. | 423089 | +0.0005636 | -0.0000758 |
| 1956-04-09 | 1956/2/29 | 1956/2/28 | ..D / .D. | 426748 | -0.0002579 | +0.0001125 |
| 1967-09-25 | 1967/7/22 | 1967/7/21 | ..D / .D. | 431001 | -0.0003255 | +0.0001374 |
| 1968-03-22 | 1968/1/23 | 1968/1/24 | ..S / .S. | 431183 | +0.0003978 | -0.0001093 |
| 1968-09-12 | 1968/7/21 | 1968/7/20 | ..D / .D. | 431360 | -0.0003283 | +0.0001126 |
| 1977-07-12 | 1977/5/27 | 1977/5/26 | ..D / .D. | 434636 | -0.0002455 | +0.0003277 |
| 2001-02-10 | 2000/12/17 | 2000/12/18 | ..S / .S. | 443387 | +0.0002896 | -0.0000852 |
| 2006-05-10 | 2006/3/13 | 2006/3/12 | ..D / .D. | 445332 | -0.0001473 | +0.0001172 |
| 2025-11-19 | 2025/9/30 | 2025/9/29 | ..D / .D. | 452579 | -0.0002594 | +0.0003799 |
| 2045-11-06 | 2045/9/27 | 2045/9/28 | ..S / .S. | 459987 | +0.0000201 | -0.0005752 |
| 2046-07-18 | 2046/5/16 | 2046/5/15 | S.D / SD. | 460245 | -0.0000567 | +0.0000756 |
| 2046-10-18 | 2046/8/19 | 2046/8/18 | ..D / .D. | 460338 | -0.0001197 | +0.0001977 |
| 2064-07-19 | 2064/6/5 | 2064/6/6 | ..S / .S. | 466925 | +0.0000579 | -0.0000523 |
| 2099-10-14 | 2099/8/30 | 2099/9/1 | ..S / .S. | 480000 | +0.0006508 | -0.0000106 |

**Tsurphu (13):**

| Date | Engine | TCG | Marks | N | td(7.13) − k | td(7.24) − k |
|---|---|---|---|---|---|---|
| 1936-01-27 | 1935/12/3 | 1935/12/4 | ..S / .S. | 75633 | +0.0000004 | -0.0000657 |
| 1953-12-01 | 1953/10/26 | 1953/10/25 | ..D / .D. | 82255 | -0.0004293 | +0.0001218 |
| 1967-07-31 | 1967/6/25 | 1967/6/24 | ..D / .D. | 87324 | -0.0004461 | +0.0000830 |
| 1975-12-26 | 1975/11/23 | 1975/11/24 | ..S / .S. | 90443 | +0.0000864 | -0.0004206 |
| 1994-12-26 | 1994/11/23 | 1994/11/24 | ..S / .S. | 97493 | +0.0001602 | -0.0001440 |
| 2014-10-03 | 2014/8/9 | 2014/8/10 | ..S / .S. | 104829 | +0.0000388 | -0.0001596 |
| 2022-06-28 | 2022/4/30 | 2022/4/29 | ..D / .D. | 107699 | -0.0003466 | +0.0002927 |
| 2038-09-25 | 2038/7/27 | 2038/7/26 | ... / .DS | 113726 | -0.0000384 | +0.0000762 |
| 2049-11-19 | 2049/9/25 | 2049/9/24 | ..D / .D. | 117864 | -0.0003663 | +0.0001628 |
| 2052-06-25 | 2052/4/28 | 2052/4/29 | ..S / .S. | 118828 | +0.0000920 | -0.0005253 |
| 2055-06-23 | 2055/4/29 | 2055/4/28 | ..D / .D. | 119938 | -0.0001354 | +0.0002349 |
| 2079-10-18 | 2079/8/23 | 2079/8/24 | ..S / .S. | 128963 | +0.0002922 | -0.0002148 |
| 2089-02-02 | 2088/12/23 | 2088/12/22 | .SD / ... | 132412 | -0.0000700 | +0.0000270 |

**Genden (23):**

| Date | Engine | TCG | Marks | N | td(7.13) − k | td(7.24) − k |
|---|---|---|---|---|---|---|
| 1903-11-01 | 1903/9/12 | 1903/9/11 | ..D / .D. | 58091 | -0.0000417 | +0.0002008 |
| 1920-06-26 | 1920/5/11 | 1920/5/10 | ..D / .D. | 64270 | -0.0000254 | +0.0001069 |
| 1926-04-08 | 1926/2/25 | 1926/2/26 | ..S / .S. | 66415 | +0.0000809 | -0.0004703 |
| 1934-06-10 | 1934/4/29 | 1934/4/28 | S.D / SD. | 69448 | -0.0001752 | +0.0000717 |
| 1934-12-25 | 1934/11/20 | 1934/11/19 | ..D / .D. | 69649 | -0.0000299 | +0.0003889 |
| 1943-02-19 | 1943/1/15 | 1943/1/14 | ..D / .D. | 72674 | -0.0002040 | +0.0001047 |
| 1958-01-14 | 1957/11/24 | 1957/11/25 | ..S / .S. | 78204 | +0.0000837 | -0.0004454 |
| 1962-08-10 | 1962/7/10 | 1962/7/9 | ..D / .D. | 79899 | -0.0000208 | +0.0000982 |
| 1971-03-25 | 1971/1/28 | 1971/1/29 | ..S / .S. | 83098 | +0.0001130 | -0.0005043 |
| 1988-11-19 | 1988/10/10 | 1988/10/11 | ..S / .S. | 89650 | +0.0001463 | -0.0000741 |
| 1990-04-04 | 1990/2/9 | 1990/2/10 | ..S / .S. | 90159 | +0.0000560 | -0.0000234 |
| 2006-04-26 | 2006/2/28 | 2006/2/29 | ..S / .S. | 96118 | +0.0004321 | -0.0001852 |
| 2013-07-21 | 2013/6/13 | 2013/6/14 | ..S / .S. | 98803 | +0.0002512 | -0.0000353 |
| 2024-09-19 | 2024/8/16 | 2024/8/17 | ..S / .S. | 102946 | +0.0002914 | -0.0000613 |
| 2033-03-17 | 2033/2L/17 | 2033/2L/16 | ..D / .D. | 106096 | -0.0000922 | +0.0002605 |
| 2033-11-05 | 2033/9/13 | 2033/9/14 | ..S / .S. | 106333 | +0.0001784 | -0.0001082 |
| 2043-12-28 | 2043/11/28 | 2043/11/27 | ..D / .D. | 110097 | -0.0004235 | +0.0001718 |
| 2045-01-07 | 2044/11/19 | 2044/11/20 | ..S / .S. | 110479 | +0.0001679 | -0.0002510 |
| 2049-09-09 | 2049/7/12 | 2049/7/13 | ..S / .S. | 112212 | +0.0001094 | -0.0001551 |
| 2061-01-06 | 2060/11/15 | 2060/11/16 | ..S / .S. | 116415 | +0.0001848 | -0.0000797 |
| 2073-02-25 | 2073/1/18 | 2073/1/19 | ..S / .S. | 120918 | +0.0000771 | -0.0003198 |
| 2077-06-15 | 2077/4/24 | 2077/4/25 | ..S / .S. | 122514 | +0.0002121 | -0.0003170 |
| 2087-04-20 | 2087/3/18 | 2087/3/17 | ..D / .D. | 126167 | -0.0003440 | +0.0000308 |

In days, the true date's distance from the deciding boundary is at most 0.00065 d (about 56 s) under (7.13) and at most 0.00058 d under (7.24); the smallest is 0.0000004 d (Tsurphu 1936-01-27, under (7.13)) and 0.0000106 d under (7.24) (Phugpa 2099-10-14). Janson's remark (p. 25) that the two values of $a_2$ give true dates that differ by at most 1/1512 d = 0.000661 d is consistent with every row: the largest difference between a row's two true dates is Phugpa 2099-10-14, 0.0006508 + 0.0000106 = 0.0006614 d, which reaches that maximum.

**(ii) Where the two computations part.** In TCG: `t2.c:637` in `gza_dag`: the lunar day `tt` enters, a second time and as a count of 105840ths, the product that is converted to a big decimal there, beside its entry into the 28ths at `t2.c:623` (the first anomaly digit `rilcha[0]`): $a_2 = 1/28 + 1/105840 = 3781/105840$, Janson's (7.24). In the engine: `crates/tshes/src/phugpa.rs:81` (`A2`, with the comment citing (7.13)) used at `phugpa.rs:421`, $a_2 = 126/3528 = 1/28$. Two probes, each changing one thing on a copy of the side concerned (METHOD rule 27); the second is the "second way" of rule 28.

| Probe | The one change | Phugpa (17) | Tsurphu (13) | Genden (23) | Everything else it changed (MEASURED) |
|---|---|---|---|---|---|
| **P-TCG**: a copy of TCG's source, `-O0` and `-O2` | `t2.c:637`: the second entry of the lunar day into that product removed, so $a_2 = 1/28$ (one line differs from a fresh extraction; `cmp` and `diff`) | removes 16, leaves 1 (2099-10-14) | removes 13 | removes 23 | the days it changes in TCG's output are exactly the 17, 13 and 23 days of the lists, no other day; **on 2099-10-14 it prints two rows** (2099/8/30 and 2099/9/1) because TCG's test for a skipped day 1 (`t4.c:486–494`, `530–537`) compares day 1 with "day 0" of the same month, which equals the previous month's day 30 only when the anomaly has no jump at a month's end; under (7.13) it does, by $-6.6 \cdot 10^{-4}$ d (`monthjump`: td(n, 30) = 2487991.0006508, td(n+1, 0) = 2487990.9999894, floors 2487991 and 2487990), under (7.24) it does not (difference 0). So the probe leaves a TCG artifact of its own on that day, not a disagreement of the original; the 606 output files of the probe are byte-identical between `-O0` and `-O2` |
| **P-ENG**: a copy of the engine (`git archive` of `2739375`) | `phugpa.rs:421–426` and `446, 452`: $a_2 = 3781/105840$ in place of 1/28, the moon term in units 30 times finer and the other two terms and the denominator scaled by 30 | removes 17 | removes 13 | removes 23 | the days it changes in the engine's output are exactly the 17, 13 and 23 days of the lists (`diff` of the date columns), no other; the probe engine equals the original TCG on **all 73,414 days of all three versions** |

**Rule 29 (a variation of a suspected cause that moves nothing eliminates it).** The build: `-O0` and `-O2` give byte-identical output on 606 files (3 versions × 202 years) for the original and for the probe. Mechanism: TCG's calendar path is exact integer and big-decimal arithmetic (`bcd.c`, `utils.c`), with no `double` in `t2.c`, `t3.c`, `t4.c`, `tests.c` and none reached by "Print calendar" (Item 1d), so an optimiser can change nothing but undefined behaviour, and it changes nothing; the compiler is **eliminated**. The weekday-based way TCG finds a calendar day (`spi_zagf`, `t2.c:415–439`) rather than $\lfloor\text{true date}\rfloor$: the script (which uses the floor, (8.1)) with (7.24) equals TCG on 73,414 days of each version with no residue, so that route contributes no difference; **eliminated** (mechanism: it picks the JD whose weekday is the true weekday's integer part, which is the floor by construction). The epoch constants: Item 3, equal on every digit; **eliminated**. The Sun (Tsurphu): the script's siddhanta Sun reproduces TCG's Tsurphu on every day, so a karana Sun (D-19) is not what TCG computes; **eliminated**. The month rules (Tsurphu 0 and 1, Genden 46 and 47): the script's month table reproduces TCG's months on every day, **eliminated**.

**The class table (every disagreement of Item 6 is in exactly one row; the counts sum to Item 6's):**

| Cause | Class | Settles (count; dates) | Probe | What else the probe changed |
|---|---|---|---|---|
| The Moon's anomaly increment per lunar day: TCG $a_2 = 3781/105840$ (7.24), the engine $a_2 = 1/28$ (7.13); `t2.c:637` against `phugpa.rs:81, 421` | **H**, with the qualifier below | **53** = 17 (Phugpa) + 13 (Tsurphu) + 23 (Genden); the dates are the 17, 13 and 23 rows of the three tables above | P-ENG removes 53 of 53; P-TCG removes 52 of 53 (16 + 13 + 23) and on the 53rd, 2099-10-14, removes the engine's disagreement in the label of tshes 30 and adds a TCG artifact (two rows on that date) | nothing else on any other day (both probes); see the table above |
| E: the engine departs from Janson v1 | E | 0 | | |
| U: not settled | U | 0 | | |
| C: a rule Janson v1 does not state | C | 0 | | |
| **Sum** | | **53** (= Item 6's 17 + 13 + 23) | | |

**The qualifier on H.** The class definition reads "TCG departs from Janson v1's formulas, at a place in its source that the probe confirms". The place is confirmed (P-TCG removes 52 of 53 on its own; P-ENG removes all 53 from the other side). But the formula TCG uses is Janson v1's (7.24), so TCG departs from the paper's *customary* (7.13), not from the paper. The prompt's four classes have no cell for "a variant the paper states", and the prompt's rule for H ("the engine follows the paper … CLAUDE.md hard rule 7 forbids changing it to agree with TCG") answers a different question from the one measured here: hard rule 7 forbids tuning a constant to fit data; it does not choose between two formulas the paper prints. Filing the row as class H with no fork would hide a real decision, so Item 8 raises it (F25, §13). Neither side is "wrong"; no class E disagreement exists.

## 2. Item 0 — branch, prompt, number, records (MEASURED 2026-11-11)

- **PRECONDITION.** `git -C /home/kitaeza/github/zurkhai log -1 --format='%h %T %s' main` printed `07b5bbf cdc937a95e657ba99e8d2e3dd7f34311d02caab7 spec 008: Genden and Tsurphu: the two other candidate systems, Gregorian <-> ZDate from Janson v1, and P5 for both`: the commit and the tree the prompt names.
- **ARTIFACT (A).** `sha256sum` of `/home/kitaeza/zurkhai-papers/janson-1401.6285v1.pdf` printed `7cafc7df563a3020849c86f4e397daa18b7f7f5ff46244403180aca90b9d0f77`; `pdfinfo` printed `Pages: 86`.
- **ARTIFACT (B).** `/home/kitaeza/zurkhai-papers/henning-tcg/` holds one file, `tcg1309.zip`, 93,207 bytes (`ls -la`); `sha256sum` printed `8389de8efcd18f7a66a7cef9b6277c222e5193b2acec440f721096a5ad1e83d2`; `unzip -l` lists 15 files, 265,616 bytes: makefile, bcd.h, bcd.ext, tc.h, tc.ext, tests.c, tcgnotes.txt, tc.def, bcd.c, utils.c, t1.c, t2.c, t3.c, t4.c, tcg.exe (all dated 2013-09-03 13:34).
- **ARTIFACT (C).** The manifest command printed `659b9347ae860b6d68ca74ad3482bfe39a439dcb4614232b6c49b654351bacf1`; `find … -type f | wc -l` printed 17.
- **Branch and prompt.** `spec/009-henning-differential` cut from `main` at `07b5bbf` before any edit. The prompt is the branch's first commit, `e48feb5`, byte-identical (`cmp` against `/home/kitaeza/zurkhai-prompts/spec-009-henning-differential.md` silent; the file was written with the Write tool, not copied by a shell command; `cmp` run before every later commit).
- **Spec number.** `ls docs/specs/*.md` showed 001–008 and no 009: free.
- **Date.** `date` run once, after the branch was cut: `2026-11-11 18:14:41 MSK`; later `date` calls (18:35, 18:46) showed the same day, so no record carries another date.
- **The round's marker** (METHOD rule 58) is chosen; this report never spells it in prose. Its only appearance is the single placeholder line of §14 (the report's own diff-stat line), replaced before the halt commit.
- **OPEN forks, re-measured** (`grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md` on `07b5bbf`): **6** (F4, F5, F7, F8, F10, F11), equal to the advisor's relayed count. The next free ids, verified in the registers: **D-20** and **F24**.
- **The gate** does not run in this half: no tracked source changes (docs/ROUNDS.md §3; METHOD rule 65). Head block: "not run: halt by design, no tracked source changed". Skipped legs: all of them (fmt, clippy, test, test_release, build_wasm, deny, msrv, no_std_check, no_alloc_check, zero_deps_check, self_test, wasm-pack); the engine was run only in `git archive` copies under the scratchpad, never in the working tree.
- **Read-first list, and how much of each was read.** Read in full with the Read tool: `CLAUDE.md` (the session's loaded copy, 19 hard rules), `docs/ROUNDS.md` (267 lines), `docs/specs/001-init-spec.md` (302 lines, the whole file, so §1, §4, §5.1, §6, §9, §10, §11, §12 included), `docs/specs/003-project-method.md` (181 lines, the whole file), `docs/specs/006-reference-fixtures.md` (475 lines, the whole file), `docs/METHOD.md` (709 lines, the whole file, which includes rules 1, 3, 11, 18, 25, 27, 28, 29, 31, 32, 35, 36, 39, 50, 51, 58, 59, 60, 64, 65 and 67 each in full), `GATED.md`, `FORKS.md` (the whole file; F10 and F23 in full), `DEBT.md` (the whole file; D-1, D-12 and D-19 in full), `docs/sources.md` (the whole file), `golden/reference/README.md`. **Read in part:** `docs/specs/008-genden-tsurphu.md`: §1 (lines 22–110), §2 (111–146), §3 and §4 (147–190), §12 (243–275) and §15 (322–331), the sections the prompt names, not the other 400 lines of its 724. Also read: `docs/specs/INDEX.md`, `docs/specs/TEMPLATE.md`, the engine's `phugpa.rs` lines 396–460 and 905–975 and `tests/p6_t4_fixture.rs` lines 1–120 and `tests/common/mod.rs` (Item 5 and Item 7 (ii) allow reading the engine's code; the Item 7 script was written before the engine's code was read for it, from the paper's pages, and never from it).

## 3. R1, and the three DEBT entries of Item 0

**R1** (2026-11-11, "Agree with all recommendations"): round 008's "Latitude taken", items 5, 6, 7 and 11, are accepted as reported, with no grant after the fact. They were: (5) python3 scripts used to edit repository files, beyond its GRANTS; (6) five Bash calls that began with `cd`; (7) the first transcription pass typed after the pdftotext text had been seen; (11) parts of the Read-first list read by their headings only. Round 008 is closed and its file is not edited. From this round on, repository files are edited only with the Edit and Write tools, and METHOD rules 31 and 36 are explicit checks on this round's own controls (Items 2, 4, 5, 6 and 7): each control shows its instrument can fail, and reports exactly the seeded difference and no other (§9). **Kept this round, MEASURED:** every edit to a file in the repository (`DEBT.md`, `docs/sources.md`, `FORKS.md`, this file, the prompt copy) was made with the Edit or Write tool; no script or shell redirection wrote into the repository. **Not kept: four Bash calls began with `cd`** (hard rule 16 and the prompt's `PATHS:` line; they are listed under Latitude, item 7), so R1's "no `cd`" was not met in full.

**R2 and R3, filed in the records commit `328febb`, all RELAYED from the reviewer's report on 39318f5 (2026-11-11), none re-checked here (the reviewer's report was not read):**

| Id | Kind | Subject | TRIGGER |
|---|---|---|---|
| **D-20** | COVERAGE | Round 008's tests are weaker than its report says (five statements) | the next round that edits `crates/tshes/` |
| **D-21** | DOCS | Round 008's citations and descriptions disagree with its code in six places | the next round that edits `crates/tshes/` |
| **D-22** | CORRECTNESS | Public functions panic on inputs their documentation admits | the next round that edits `crates/tshes/src/`, or before the crate's first publish, whichever first |

None of the three TRIGGERs fires in this round: nothing under `crates/` changes.

## 4. Item 1 — the holding, and what the program computes (MEASURED 2026-11-11; Henning's source is cited by file:line, never copied)

[CORRECTED 2026-11-11 (spec 009 session 2, Item 11): the heading's "never copied" was untrue at the halt commit `6248886`. This section then reproduced statements and expressions of the program's source: two calls in the day loop of `prn_cal` (§4c), the format-string call at `t4.c:700` (§5), and condition and assignment expressions on the epoch flags and on the file name (§4b). Each is now a description with file:line, and §4b also no longer quotes the program's prompts and error text. What remains in the section is identifiers and numbers cited with file:line, the licence notice, and the menu text as `tcgnotes.txt` documents it, as CITATIONS allows.]

**The records.** `docs/sources.md` gained one library row for `henning-tcg/` (source, contents as `unzip -l` lists them, Obtained as the prompt words it, sha256 and size, Licence, Trusted for) and the `henning-kalacakra/` row's "Trusted for" cell gained the one sentence the prompt gives, nothing else in it changed (commit `2739375`). **An ordering slip, reported:** the prompt says the row lands "in the commit that first cites ARTIFACT (B)"; the Item 0 records commit (`328febb`) already cited the zip's size, hash and file list in this report, and the row landed in the next commit. The date tcgnotes.txt gives was checked: line 1 reads "Version 1.06. 3 September 2013." The licence is stated by 12 of the 15 files (the seven `.c` files, `tc.h`, `bcd.h`, `tc.ext`, `bcd.ext`, `tc.def`; counted with `grep -a`), as an MIT-style permission notice with "Copyright (c) 2009-2013 Edward Henning" (e.g. `t2.c:3–24`); `makefile`, `tcgnotes.txt` and `tcg.exe` carry none.

**a. The menu entries for the three versions** (`tcgnotes.txt:5–11`, the same strings in `tc.def:114–118`, `e_str`): "1. Generalised Phugpa, -1000.", "2. Generalised Tsurphu, -1000.", "5. New Genden Calculations, 1747 CE." (entries 3 and 4, Error Correction and mkhas pa'i snying nor, are out of scope). The key chosen sets `epch` to key − 1 (`t2.c:113–114`): Phugpa 0, Tsurphu 1, Genden 4. The main menu: `tcgnotes.txt:20–28`, handled by `t1.c:344–407` ("2. Print calendar" → `prn_cal`, `t1.c:376–379`).

**b. How "Print calendar" is asked for a range of years, and the file it writes.** From the main menu key `2`, `prn_cal` (`t4.c:255`) asks for the starting year and the finish year with two integer reads (`scanf`, `t4.c:281–289`) and runs one Tibetan year after another (`t4.c:910–913`), writing **one file per year**: `pl_<year>.txt` when the epoch is Phugpa, `ts_<year>.txt` when the flag `tsurlug` is 1 (Tsurphu), `ec_<year>.txt` for Error Correction (`t4.c:298–303`; `tcgnotes.txt:32`). **Two findings about TCG (MEASURED, both builds).** (1) *No branch names a file for Genden.* For the Genden epoch (`epch` 4) with `tsurlug` 0 none of the three conditions holds, so `filenam` (a global character array of 14, `tc.def:87`) keeps its previous contents, empty in a fresh process: opening a file with the empty name fails, the program prints an error message about the target file and waits for a key with `getch` (`t4.c:305–310`), then writes to the null stream. A fresh process given `5`, `2`, `2012`, `2012`, `9` ends with exit status 139 (SIGSEGV) at `-O0` and `-O2`, and writes no file. (2) *Selecting Genden after Tsurphu leaves Tsurphu's flags set.* `set_epoch` sets `tsurlug` and `zlapure` only for keys 1 and 2 (`t2.c:140–141`, `165–166`; for keys 3, 4, 5 those lines are commented out, `t2.c:197`, `223`, `249`). MEASURED: the sequence `2`,`2`,2033,2033,`1`,`5`,`2`,2033,2033,`9` (Tsurphu, print, set epoch Genden, print) writes Genden's calendar under Tsurphu's leap-month rule: for Tibetan year 2012 it prints 384 day-rows where Genden has 355 (a leap month where Genden has none; 89 of its rows are not rows of the correct calendar), for 2033 295 of its 384 rows are not rows of the correct 384, and for 2035 it prints 354 rows against the correct 384, 89 of them not rows of the correct calendar; identical under `-O2`. **How the round drives Genden:** a Phugpa print of the same year first (it sets `filenam` to `pl_<year>.txt` and leaves `tsurlug` and `zlapure` both at 0, the values of a fresh process), then `1`, `5`, then a print of that one year: the Genden calendar overwrites `pl_<year>.txt`. One process per year, 202 per build. MEASURED, the dependence on the pre-print year: the Genden output after a pre-print of 1900 equals the output after a pre-print of the year itself, for 1899, 1927, 1987, 2012 and 2100, `-O0` and `-O2`. Both defects are TCG 1.06's, not the port's (the code that causes them is unchanged Henning code, and the pl_ files that exist for Phugpa were written by the unaffected path); they go to Item 8 as part of fork F24's cost.

**c. Which Tsurphu the program computes: the siddhanta Sun, as Janson v1's A.2 does (prediction 4: HELD).** The true date of a lunar day in TCG is the true weekday `gzadag`, and it is built from the mean weekday `tsebar` and the solar equation of the mean Sun at the end of the lunar day. For every epoch alike: the day loop of `prn_cal` calls the Sun routine `nyi_dag` on the mean Sun `nyibar` and then the weekday routine `gza_dag` on the mean weekday `tsebar` (`t4.c:477–481`; a comment of Henning's there says the first must run before the second); `nyi_dag` (`t2.c:572–612`) takes the mean Sun `nyibar`, the sum of the month's value `nyidru` and the lunar day's `nyilon` (`t4.c:477–479`, `t2.c:545–563`), subtracts the 6;45 offset `nyifac` (`t2.c:48`, `576`), reads the Sun table `nyibye`, `nyidom` (`t2.c:50–51`, `593–606`) and returns the solar equation `listc`; `gza_dag` (`t2.c:615–678`) applies the Moon's equation (`gzadom`, `gzabye`, `t2.c:53–56`, `623–657`) and then `add_gen`/`sub_gen` of `listc` (`t2.c:667–670`). The Sun's slopes are the same constants for every epoch (`set_grub`, `t2.c:86–99`, `nyidmg` at `t2.c:43`); the epoch changes only the starting values `gda2`, `nda2` and the flags (`t2.c:65–66`, `143–167`). A case-insensitive search for `karana`, `sbyor` and `byedpa` over the other six `.c` files finds nothing, and in `t4.c` the yoga and the karana are computed after `nyidag` is fixed and only to print their names (`t4.c:558–584`). Henning's page says the same in its own words: "There is a minor difference with recent Tsurphu calendars, due to their use of a different value for the longitude of the Sun. That is implemented in the software for an extensive Tsurphu calendar that is available here." (`os_tib.htm:73–74`, FETCHED); that program (`os_tsurphu.htm`) is out of scope (STOP RULE (7)). MEASURED beside the reading: the script of §1.2, which is the siddhanta calculation of A.2, reproduces TCG's Tsurphu on all 73,414 days when it uses TCG's $a_2$ (0 differ). STOP AND ASK for this item did not arise.

**d. Where each version's epoch data is held, and the arithmetic.** In `t2.c`: the mean-weekday epoch value `gda<k>` and the mean-Sun epoch value `nda<k>` for k = 1 (Phugpa), 2 (Tsurphu) and 5 (Genden) at `t2.c:60–61`, `65–66`, `80–81`; per-epoch scalars in `set_epoch` (`t2.c:103–251`): the epoch year `epch_yr` (−1000, −1000, 1747), the intercalation index `eyr_a` (7, 11, 10; used by `zla_dag`, `t2.c:298–317`), the anomaly constants `ril_a`, `ril_b` (6/61, 6/75, 24/22; `rilchaf`, `t2.c:403–410`), the general-day constants `spz_b`, `spz_c`, `spz_f`, `spz_j` (`spi_zagf`, `t2.c:415–439`), the leap-month tests `zlasho1`, `zlasho2` (48/49 for Phugpa; 46/47 for Genden; `zlapure = 1` for Tsurphu: a leap month where the index is 0 or 1; `adj_zla`, `t2.c:320–400`), and the monthly slopes `nyidmg` (Sun), `gzadmg` (weekday), `tsedmg`, `nyilmg` (per lunar day) at `t2.c:43–46`. **Arithmetic:** exact integer arithmetic only on the calendar path. Quantities are radix lists of five ints (weekday: radices 7, 60, 60, 6, 707; Sun: 27, 60, 60, 6, 67); `mul_gen`, `add_gen`, `sub_gen` (`utils.c:237–374`) multiply and carry through arbitrary-precision decimal digit arrays (`bcd.c`, `BCDMAX 20`, `bcd.h:24`; `signed char` digits, `l2bcd`/`bcd2l` convert from and to `int`), so no product overflows a 32-bit int in the multiplications; the anomaly correction `gza_dag` uses `bcd` for its products and quotients (`t2.c:637–640`, `659–662`, `674–677`). **Floating point (`double`) is used only to print and in the planetary routines**: `prn_lstd` and `conv2degs` (`utils.c:52–65`, `286–312`) for the degree figures, `cal_cyc`'s Julian tithi times (`t1.c:224–254`), and `do_plans` in `t3.c`; a search for `double`, `float`, `sqrt`, `fmod`, `floor`, `ceil`, `pow` in `t2.c`, `t3.c`, `t4.c` and `tests.c` finds none, and `prn_cal` calls none of the routines above (`t4.c:255–916`). So no floating-point value reaches a tshes, a month or a date in "Print calendar". One more point where the date is not Janson's (8.1): `spi_zagf` (`t2.c:415–439`) finds the day by the general-day count and then moves it by up to two so that its weekday equals the true weekday's integer part `gzadag[0]` (`t2.c:425–434`), and `jul2date` (`t2.c:485–542`) converts a JD to a date; both are integer arithmetic, and the elimination of §1.2 shows they add no difference.

## 5. Item 2 — the build, and its check

**Extraction and stand-ins.** `tcg1309.zip` was extracted with `unzip` into a `mktemp -d` directory under the session's scratchpad; Henning's files were not edited, and after the builds a fresh extraction into a second `mktemp -d` directory was compared file by file with `cmp`: 15 of 15 files identical. Three stand-in headers of this round's own are in a separate directory: `conio.h` (one function, `getch`, reading a byte from standard input and exiting with status 3 at end of input), `dos.h` and `bios.h` (empty: nothing from them is used). Their text is in the last section.

**Compiler, data model, flags.** `gcc (Ubuntu 15.2.0-16ubuntu1) 15.2.0`, x86-64, LP64 (`int` 32 bits, `long` and pointers 64 bits), against the original's 32-bit Win32 model (Open Watcom, `wcl386`, makefile flags `/c /cc /ms /W3 /3 /fpi /omax /zp1 /bt=NT`). The code uses `int`, `signed char` and `double` only, so the difference in `long` and pointer width does not enter its arithmetic (INFERRED from the sources; the build check is the measurement that settles it). Flags: `-std=gnu89 -O0` and `-std=gnu89 -O2`, with `-Wall -Wextra -I <stand-ins>`; no `-ffast-math`. `-std=gnu89` because the sources use K&R function definitions (`utils.c:424`), which gcc 15's default C23 mode rejects. No `-m32` build was tried. **Warnings, counted from the compiler's output** (each line with `warning:`): **-O0: 26**, **-O2: 41**. By kind, -O0: unused-but-set variable 15, array-parameter bound mismatch 6, unused parameter 1, `main` not returning `int` 1 (`t1.c:344`), format 1 (`t4.c:1429`, `%ld` with an `int`), too many arguments for a format 1 and an embedded NUL in a format 1 (both `t4.c:700`). At -O2 the same, plus 8 ignored `scanf` results and 7 may-be-uninitialised (`yr_gender`, `yr_elemx`, `mt_elemx` in `t4.c:425–445`, and `wwd`, `wwm`, `wwy`, `dow`, which the compiler reports at an inlined `sprintf`). **One warning is a finding about the program, not the port:** `t4.c:700` writes the extra line that follows the first of two days that share a tshes with a format string whose first character is written as the octal escape for NUL (the author appears to have meant the hexadecimal escape for a space: INFERRED); a NUL ends a C string, so the format is empty, and the line (the yoga, karana, animal, trigram and number line) is never written for that day. The published files show the same absence (day "5: Sun." of 2012 has no such line), so the build reproduces the author's output there. The exit status of every run is 25: `main` is declared to return nothing (`t1.c:344`), and the value left in the return register is the length of the last `printf`, the one that reports the highest number of figures used (11 in these runs), not a status.

**THE BUILD CHECK (METHOD rule 64): per year, normalised only for line endings and trailing spaces.** The six files (`pl_1900`, `pl_1950`, `pl_2000`, `pl_2012`, `pl_2025`, `pl_2049`) were produced by one process per build, driven by the input in the last section, and compared by `cmpbuild.py` with Henning's `tdata/pl_<year>.txt`:

| Year | Lines | Equal lines | Differing lines: calendar field | Differing lines: other field |
|---|---|---|---|---|
| 1900 | 1,470 | 1,470 | 0 | 0 |
| 1950 | 1,470 | 1,470 | 0 | 0 |
| 2000 | 1,590 | 1,590 | 0 | 0 |
| 2012 | 1,470 | 1,470 | 0 | 0 |
| 2025 | 1,470 | 1,470 | 0 | 0 |
| 2049 | 1,470 | 1,470 | 0 | 0 |
| **Total** | **8,940** | **8,940** | **0** | **0** |

Both builds (-O0 and -O2) give this table, and their six files are byte-identical to each other (`cmp`). Lines are counted with `wc -l`'s rule; 1,470 + 1,470 + 1,590 + 1,470 + 1,470 + 1,470 = 8,940. STOP RULE (4) did not fire. **Predictions 2 and 3: HELD.** A published file is not an oracle for the engine (the `docs/sources.md` sentence of commit `2739375`): this check shows only that this build computes what the author's build computed for six years of Phugpa; it does not show the Tsurphu or Genden paths, which the pl_ files do not cover (Item 4's reading check and Item 3 stand in for them).

**Every run of the round was made with both builds and compared (METHOD rule 32's question):** the six years (above), the three full ranges (606 files: 202 Phugpa, 202 Tsurphu, 202 Genden, byte-identical, and the standard output of the Phugpa and Tsurphu runs identical), the probe copy of TCG (606 files, byte-identical), the fresh Genden run (exit 139 in both), the stale-state runs (rows equal) and the pre-print runs (equal). Differences found: none.

## 6. Item 3 — the epochs, before any day is compared (MEASURED, `epochs.py`)

TCG's numbers are typed from the lines named; Janson's are as printed. A TCG weekday list is read in radices (7, 60, 60, 6, 707), a Sun list in (27, 60, 60, 6, 67), as `set_grub` sets them for every epoch (`t2.c:97–98`). The carrying offset K is the number of true months from TCG's epoch to Janson's n = 0; it is found by requiring each quantity to be equal, and the one K that fits all of them is the same for all three (22338 for E806, 33792 for E1732, 0 for E1747). K = 22338 is TCG's count of the nominal epoch month 3 of year 806; Janson's n = 0 is month 2 (his Remark 5), whose traditional count is the one obtained by calculating for month 3 (his Remark 4), which is the same number.

| Version | Quantity | TCG (file:line, as written) | Janson v1 (equation, page) | Carried value | Verdict |
|---|---|---|---|---|---|
| all | monthly mean weekday | `gzadmg` = 1;31,50,0,480 (`t2.c:44`) | $m_1 - 28$, (7.2), p. 21: 8657/5656 | 8657/5656 | EQUAL |
| all | per lunar day mean weekday | `tsedmg` = 0;59,3,4,16 (`t2.c:45`) | $m_2$, (7.3), p. 21: 11135/11312 | 11135/11312 | EQUAL |
| all | monthly mean Sun | `nyidmg` = 2,10,58,1,17 (`t2.c:43`) | $s_1$, (7.6), p. 23: 65/804 | 65/804 | EQUAL |
| all | per lunar day mean Sun | `nyilmg` = 0,4,21,5,43 (`t2.c:46`) | $s_2$, (7.7), p. 23: 13/4824 | 13/4824 | EQUAL |
| all | monthly anomaly | `rilchaf` (`t2.c:403–410`): 2 twenty-eighths and 1/126 of one | $a_1$, (7.12), p. 23: 253/3528 | 253/3528 | EQUAL |
| all | per-lunar-day anomaly | `gza_dag` (`t2.c:623`, `637`): $1/28 + 1/105840$ | $a_2$: (7.13) 1/28; (7.24) 3781/105840 | 3781/105840 | **EQUAL to (7.24); DIFFERS from (7.13) by 1/105840** |
| Phugpa | $\beta^*$ | `eyr_a` = 7 at year −1000 (`t2.c:124`, `123`) | E806, (5.4), p. 14: 61 | $(7 + 2 \cdot 21672) \bmod 65 = 61$ | EQUAL |
| Phugpa | $m_0$ | `gda1` = 5;35,36,4,160 (`t2.c:60`) | E806, (7.4), p. 21: 2015501 + 4783/5656; weekday $(JD + 2) \bmod 7 = 0$ | fraction 4783/5656, weekday 0 | EQUAL |
| Phugpa | $s_0$ | `nda1` = 26,45,53,4,26 (`t2.c:61`) | (7.8), p. 23: 743/804 | 743/804 | EQUAL |
| Phugpa | $a_0$ | `ril_a`, `ril_b` = 6, 61 (`t2.c:125–126`) | (7.14), p. 23: 475/3528 | 475/3528 | EQUAL |
| Phugpa | Table 6, JD 2015531 (k = 1, K′ = 22339) | | $m_0$ 2:22:34:2:518 = 2.376238; $s_0$ 0:8:3:3:33 = 0.004975; $a_0$ 5:98 = 0.206349 | 240/101 = 2.376238; 0.004975124; 13/63 | EQUAL on all three (exact fractions) |
| Tsurphu | $\beta^*$ | `eyr_a` = 11 at −1000 (`t2.c:151`) | E1732, A.2, p. 39: 59 | $(11 + 2 \cdot 32784) \bmod 65 = 59$ | EQUAL |
| Tsurphu | $m_0$ | `gda2` = 5;38,22,3,701 (`t2.c:65`) | (A.3), p. 39: 2353745 + 1795153/7635600; weekday 4 | fraction 1795153/7635600, weekday 4 | EQUAL (the coarse radices carry to Janson's finer fraction exactly) |
| Tsurphu | $s_0$ | `nda2` = 0,7,25,0,30 (`t2.c:66`) | (A.4), p. 39: −5983/108540 ≡ 102557/108540 | 102557/108540 | EQUAL |
| Tsurphu | $a_0$ | `ril_a`, `ril_b` = 6, 75 (`t2.c:152–153`) | (A.5), p. 39: 207/392 | 207/392 | EQUAL |
| Tsurphu | Table 6 (k = −11453, K′ = 22339) | | 2:25:20:2:352 = 2.422338; 0:29:34:5:37 = 0.018261; 5:112 | 2.422338; 0.018260549; 53/252 | EQUAL on all three |
| Genden | $\beta^*$ | `eyr_a` = 10 at 1747 (`t2.c:232–233`) | E1747, A.3, p. 42 (and the paragraph after (A.16), p. 42): 10 | 10 | EQUAL |
| Genden | $m_0$ | `gda5` = 1;55,13,3,333 (`t2.c:80`) | (A.11), p. 42: 2359237 + 2603/2828 (Remark 23: "1; 55, 13, 3, 333"); weekday 1 | fraction 2603/2828, weekday 1 | EQUAL |
| Genden | $s_0$ | `nda5` = 26,39,51,0,18 (`t2.c:81`) | (A.12), p. 42: 397/402 | 397/402 | EQUAL |
| Genden | $a_0$ | `ril_a`, `ril_b` = 24, 22 (`t2.c:234–235`) | (A.13), p. 42: 1523/1764 | 1523/1764 | EQUAL |
| Genden | Table 6 (k = −11639, K′ = −11639) | | 2:25:6:3:327 = 2.418494; 0:38:17:0:6 = 0.023632; 5:101 | 2.418494; 0.023631841; 731/3528 | EQUAL on all three |

**This item says whether a later disagreement can come from the constants: only through $a_2$.** Every epoch value and five of the six mean motions are equal on every digit; the sixth, $a_2$, is the cause of §1.2. **Prediction 5: FAILED as worded.** It reads "TCG's epoch data and mean motions, carried to E806, E1732 and E1747, equal Janson v1's on every digit". The epoch data does, and so do $m_1, m_2, s_1, s_2, a_1$; $a_2$ equals Janson's (7.24) and differs from his (7.13) by 1/105840. It is scored FAILED because the engine, whose constants round 008 matched to the paper, carries (7.13). The prediction's author could not know that TCG holds the Remark 14 variant. The leap-month rules (ix = 48 or 49; 0 or 1; 46 or 47) are not constants of the epoch tables and were checked by the month reading (Item 4: leap months equal Table 7 in every version) and by the script's month table (0 differences).

## 7. Item 4 — run, read, and check the reading

**What was run.** "Print calendar" for each version over Tibetan years 1899..2100 (the first, 1899, contains 1900-01-01, which is tshes 30 of a leap month 11; the last, 2100, contains 2100-12-31), with both builds: Phugpa one process (key 1), Tsurphu one process (key 2), Genden one process per year (§4b). Each file was read into rows by `readtcg.py`; 73,796 day-lines per version, 73,414 inside 1900-01-01..2100-12-31 (382 are days of the first and last files outside the range). The rows of the two builds are identical.

**The mapping from TCG's fields to the T4 fixture's columns, each with its evidence:**

| Column | TCG's field | Evidence |
|---|---|---|
| `tibetan_year` | the year number `prn_cal` asked for, printed on the "New Year: <year>, <element>-<gender>-<animal>" line that opens each file | `t4.c:281–285, 294, 366–370`; `os_tib.htm:116` and `tcgnotes.txt:32` (the file `pl_1992.txt` is "phug lugs for 1992"); MEASURED: every file's New Year line carries the file's own year (202 of 202 per version), and the numbering is Janson's Y, the Gregorian year in which the Tibetan year begins (the reading check against Tables 1 and 9 is the oracle: 0 mismatches) |
| `month` | N of the heading "Tibetan Lunar Month: N …" | `t4.c:448–465`; `mthnum` is the corrected month number (`t4.c:401–404`) |
| `is_leap_month` | true when the heading says "(Intercalary)"; "(Delayed)" is the regular month (second of the two), false | `t4.c:448–462` (`intercal` from `adj_mth < 0`, `delaymth`); `tcgnotes.txt:57` ("negative value … intercalary month (zla lhag) … the second of these is called the delayed month (zla bshol)"); MEASURED: the leap months read equal Table 7 in all three versions |
| `tshes` | T of the day line "T: Dow. mansion. element-element; D Mon YYYY" | `t4.c:685, 755, 815` (`tt`) |
| `gregorian_date` | D Mon YYYY of the same line | `t4.c:691, 761, 821` (`wd`, `wmonths[wm-1]`, `wy`); `tcgnotes.txt:95` ("Julian day … followed by the western date, in dd/mm/yyyy format") |

A repeated tshes is two day-lines with the same T and consecutive dates (the first printed with the JD − 1, `t4.c:674–691`); an omitted tshes is a line "T. Omitted: …" with no date (`t4.c:607–615`), so doubled and skipped tshes follow from consecutive rows, as in the T4 file.

**Checks on TCG's own output** (`checkrows.py`; the same checks pass on the committed T4 fixture and on the engine's three dumps, MEASURED; the engine's skipped and doubled counts equal TCG's in every version: 2,800 and 1,632, 2,799 and 1,631, 2,802 and 1,634):

| Check | Phugpa | Tsurphu | Genden |
|---|---|---|---|
| rows | 73,414 | 73,414 | 73,414 |
| first / last date; duplicate dates; gaps | 1900-01-01 / 2100-12-31; 0; 0 | same; 0; 0 | same; 0; 0 |
| tshes steps between consecutive days (mod 30): 0 / 1 / 2 | 1,632 / 68,981 / 2,800 | 1,631 / 68,983 / 2,799 | 1,634 / 68,977 / 2,802 |
| months, all / complete (not the first or last of the range) | 2,488 / 2,486 | 2,488 / 2,486 | 2,488 / 2,486 |
| complete months of 29 / 30 days (no others) | 1,168 / 1,318 | 1,168 / 1,318 | 1,168 / 1,318 |
| leap months, all / complete | 75 / 74 | 75 / 75 | 75 / 75 |
| a number present 3 or more times in a month | 0 | 0 | 0 |
| skipped − doubled = 30 × (complete months) − (days in them) | 2,800 − 1,632 = **1,168** = 30 × 2,486 − 73,412 = 74,580 − 73,412 | 2,799 − 1,631 = **1,168** = 1,168 | 2,802 − 1,634 = **1,168** = 1,168 |
| TCG's own "Omitted" lines against the numbers absent from the rows, in the 2,486 complete months | 2,800 = 2,800, 0 months differ | 2,799 = 2,799, 0 differ | 2,802 = 2,802, 0 differ |

(The leap-month counts "75 / 74" for Phugpa and "75 / 75" for the others are as in the T4 fixture and spec 008's census: the Phugpa leap month 11 of 1899 straddles 1900-01-01, the range's first day, so it is not a complete month in the range.)

**THE READING CHECK against the paper (`readcheck.py`, the T5 files by their own columns):**

| Version | Table 7 (leap month per year, 2000–2020) | Table 8 (repeated and skipped numbers of 2012) | Table 9 (first day of the year, 2000–2030) | Table 1 (first day and name, 1927–2046) |
|---|---|---|---|---|
| Phugpa | 21 rows, 0 mismatches | 23 events, 0 | 31 rows, 0 | 120 rows, dates and names, 0 |
| Tsurphu | 21, 0 | 21, 0 | 31, 0 | (Phugpa only) |
| Genden | 21, 0 | 21, 0 | 31, 0 | (Phugpa only) |

0 mismatches in 341 table rows (Phugpa 21 + 23 + 31 + 120 = 195, Tsurphu 21 + 21 + 31 = 73, Genden 73; Table 1 read for date and for name); no reading slip occurred (the first reading passed); no finding of TCG against Janson's tables exists. **Prediction 6: HELD.** **The reading check cannot separate (7.13) from (7.24)** (METHOD rules 18, 25): the engine passes the same tables under (7.13) in P5 (spec 006, 008), and the script passes Table 8 under both values; the printed tables decide neither variant.

## 8. Item 5 — the engine's side (MEASURED)

`git -C /home/kitaeza/github/zurkhai diff --stat main HEAD -- crates Cargo.toml Cargo.lock rust-toolchain.toml` printed nothing (the branch carries no change to the engine; at `2739375` the tree object `crates` is `5c4f27812a1e2cd748d4b9df292c964be5bf0d93`). On a `git archive HEAD` copy (taken at `2739375`; the second copy, for the engine probe of Item 7, was taken at `c3a52bd`; the `crates` tree object is `5c4f2781…` at `2739375`, `c3a52bd` and the halt commit alike), a probe test of this round's own (`zz_spec009_dump.rs`, last section) writes one row per day for each of `System::Phugpa`, `System::Tsurphu` and `System::Genden`, named explicitly (DEBT.md D-1), through the public API only. Run in debug and in release (`cargo +1.99.0 test --offline`, `CARGO_TARGET_DIR` inside a `mktemp -d` directory): the two outputs are **byte-identical** for each of the three systems (sha256 of the Phugpa dump `8bfd140735bf33b724abe56b0bbb6bb9b63f20164c07cf570622f70f66444ce4`, Tsurphu `b14570470d6b3a486928fba4e38dea05e72049390a5be1f40628c7a6f9bbfa23`, Genden `f602429f8a3ba7b8b1ca2514465c060d51a5c780b86a3bc5e212402876a551cf`).

**THE DUMP CHECK:** the Phugpa dump equals `golden/reference/tdc-97d4a00-phugpa-1900-2100.csv` on all 73,414 rows: `diffrows.py` reports 0 disagreeing days in all four fields, and `cmp` of the two files is silent (byte-identical; the sha256 is the fixture's own, recorded in spec 006). Its control, METHOD rules 31 and 36: a copy of the dump with one row's tshes changed (1982-02-20, 27 → 28) is reported as exactly one disagreeing day (0 → 1, only that date, the tshes field 0 → 1); a copy with one day's month changed (2009-07-08) is reported as exactly that day (month field 0 → 1). As expected.

## 9. Every METHOD rule 31 control of Items 2, 4, 5, 6 and 7, together

| Item | Instrument | What was seeded | Expected | Observed |
|---|---|---|---|---|
| 2 | `cmpbuild.py` | a copy of `pl_2012.txt` with line 26's tshes 5 → 6 and line 34's day 29 → 30 | exactly 2 differing lines, both calendar, in tshes and in the western date | exactly 2 (`['tshes']` at line 26, `['wday']` at line 34), 1,468 equal |
| 2 | `cmpbuild.py` | the same file with line 14 (`  Solar: Wood-Tiger. Jiao 6`) removed | exactly that line missing, class other | exactly 1, `missing in ours: published line 14`, other, 1,469 equal |
| 2 | `cmpbuild.py` | line 26 (a day line) removed (added control) | exactly that line missing, class calendar | exactly 1, calendar |
| 2 | `cmpbuild.py` | an unchanged copy | 0 | 0 |
| 4 | `checkrows.py` | one row deleted; one tshes changed to a value that breaks the step | FAIL on the row count and the gap; FAIL on the step counts | `rows: 73413`, `gaps: 1`; `step counts … {4: 1, 28: 1}` |
| 4 | `readcheck.py` | a copy of the read rows with one leap month and one New Year changed (per version) | Phugpa: 1 (Table 7) and 2 (Tables 9 and 1) for the New Year of 2006, 3 together; Tsurphu and Genden: 1, 1, 2 together | Phugpa: 1, 2, 3; Tsurphu: 1, 1, 2; Genden: 1, 1, 2; each message names its table and year |
| 5 | `diffrows.py` | one tshes (1982-02-20) of a copy of the dump changed; one month (2009-07-08) of a copy of the fixture changed | exactly that day each time | exactly that day, the field count 0 → 1 |
| 6 | `diffrows.py` | the same two seeds in the TCG-against-engine comparison (17 disagreeing days at the start) | one new day each, and only that day | 17 → 18, new day 1982-02-20 only; month-field count 1 → 2, new day 2009-07-08 only |
| 7 | `janson_ref.py` | the seeded constants of §1.2 | fails Table 8 and the listed days | see §1.2: Genden $s_0$ − 40/402 fails both; smaller seeds fail Table 8 only or neither, reported as "cannot resolve" |

## 10. The predictions, scored

Written by the advisor before any measurement (METHOD rule 3); scored with this round's own measured values.

| # | Prediction | Score | Measured |
|---|---|---|---|
| 1 | Every ARTIFACT check passes | HELD | §2: sha256 of the PDF and the zip, `pdfinfo` 86 pages, 15 files in the zip, manifest `659b9347…acf1`, 17 files |
| 2 | TCG builds with gcc from Henning's files unchanged, with header stand-ins only | HELD | §5: 15 of 15 files identical to a fresh extraction after the build; the only non-default choice is the flag `-std=gnu89` |
| 3 | The build check passes on every line of all six years, in both builds; `-O0` and `-O2` identical everywhere | HELD | 8,940 of 8,940 lines equal; 606 files identical between the builds (and 606 for the probe) |
| 4 | TCG's Tsurphu true date uses the siddhanta Sun | HELD | §4c, with file:line; and 0 of 73,414 days differ between the siddhanta script (with TCG's $a_2$) and TCG's Tsurphu |
| 5 | TCG's epoch data and mean motions equal Janson v1's on every digit | **FAILED** (stays FAILED after C2) | §6: the epoch data and five of six mean motions are equal; $a_2$ is (7.24), not (7.13). **C2 (ADDENDUM 1, the advisor's correction, not a ruling):** the prediction was ambiguous as written, because Janson v1 prints two values of $a_2$ and TCG's equals (7.24) on every digit; the advisor meant the values the engine carries, and under that reading it FAILED, as session 1 scored it. The ambiguity is the advisor's defect, and the score is unchanged |
| 6 | The read output passes the reading check for all three versions | HELD | §7: 0 mismatches in 341 table rows |
| 7 | Phugpa: TCG and the engine disagree on between 10 and 200 days, 2001-02-10 and 2025-11-19 among them | HELD | 17 days; both dates in the list |
| 8 | Tsurphu and Genden: each between 10 and 200 days | HELD | 13 and 23 |
| 9 | Months and leap months agree in all 202 years; the first day differs in at most 3 years per version | HELD | 0 years, 0 years, 0 years (and 0 for the first day) |
| 10 | Every disagreeing day is in a run of at most 2 days, beside a doubled or skipped tshes in at least one calendar | HELD | 53 runs, all of length 1; 53 of 53 beside one (base rate 0.18) |
| 11 | No disagreement is class E or U | HELD | 0 and 0 |
| 12 | One cause settles every Phugpa disagreement, and the same cause every Tsurphu and Genden disagreement | HELD, with one qualification | $a_2$ settles all 53 (the script and P-ENG); P-TCG removes 52 of 53 and leaves a TCG artifact on 2099-10-14 (§1.2) |
| 13 | Item 8 raises between 1 and 4 forks, and OPEN at the halt commit is 6 + the forks raised | HELD | 2 forks (F24, F25); OPEN 6 + 2 = 8, measured (§12) |

**ADDENDUM 1's predictions 14–18** (written by the advisor on 2026-11-11, before session 2 measured anything; RELAYED, none a premise; scored with session 2's own values):

| # | Prediction | Score | Measured (session 2) |
|---|---|---|---|
| 14 | ARTIFACT (A) and (B) pass, and HEAD is 6248886 | HELD | S2.1: the PDF's sha256 `7cafc7df…0f77` and 86 pages; the zip's 93,207 bytes, sha256 `8389de8e…83d2` (in full in S2.1), and 15 files; `git rev-parse HEAD` printed `62488863abc2ca86482a6bd32ac2b0551e9107a8`; `git status --porcelain` empty |
| 15 | After Item 10, FORKS.md has 6 OPEN rows, and D-23 and D-24 are the ids filed | HELD | the pre-commit grep printed 6 (F4, F5, F7, F8, F10, F11); `git show f12d565:FORKS.md` has 6; D-23 and D-24 are the entries filed in `f12d565` (the last id before them was D-22) |
| 16 | Item 11's check, on the halt's report, lists 12 lines: 133, 184, 186, 190, 198, 652, 653, 694, 1570, 1572, 1574 and 1608; of these 133, 190, 198, 694 and 1608 are class (a) | HELD | S2.3: the check printed exactly those 12 line numbers; the class (a) lines are those five (by my reading line 198 holds two class (a) substrings) |
| 17 | After Item 11, the check lists 7 lines, none in class (a): one licence line, one line of menu text, two `#include` lines and three lines of `epochs.py`'s values | HELD | S2.3: 7 lines (186 licence, 188 menu, 720 and 721 `#include`, 1606, 1608 and 1610 the values of `epochs.py`; line numbers at Item 11's commit `cb39543`); (a) 0 |
| 18 | Every gate leg that runs passes with round 008's counts: 147 passed, 0 failed (debug) and 147 passed, 0 failed (release), with wasm-pack skipped | HELD | S2.5: the gate on `f63ad2f`: 147 passed, 0 failed (debug) and 147 passed, 0 failed (release); 11 legs run, 11 passed, 0 failed; wasm-pack skipped. Round 008's head block says 147 / 147 (RELAYED); session 2 re-measured the 147s from the gate's own output |

## 11. Every spec task and every prompt question, answered

| Task | Verdict | Evidence |
|---|---|---|
| Item 0: branch, prompt, number, records, three DEBT entries, R1, marker, OPEN count, date, Read-first list | DONE | §2, §3; commits `e48feb5`, `328febb` |
| Item 1: the sources.md rows; statements a–d | DONE | §4; commit `2739375` |
| Item 2: the build, warnings by count and kind, the build check, its controls, the commands | DONE | §5, §9, last section |
| Item 3: the epochs and mean motions, carried, in a table | DONE | §6 |
| Item 4: runs, rows, mapping, checks, reading check, controls | DONE | §7, §9 |
| Item 5: the engine's side, the dump, the dump check, its control | DONE | §8, §9 |
| Item 6: the differential, per field, per year, the list, T4 directly, rule 25, control, the RELAYED paragraph | DONE | §1.1, §9 |
| Item 7: the script and its controls, the true dates, where they part, the probes, the class table | DONE | §1.2, §9 |
| Item 8: one STOP AND ASK per fork, in FORKS.md, the rule 59 grep, the rule 60 arithmetic | DONE | §13, §12 |
| The prompt's questions in Item 1 (a–d), Item 6 ("state whether those two days are in your Phugpa list"), Item 7 ("which side follows Janson v1", "where do the two computations part") | answered | §4, §1.1, §1.2 |
| Research legs | not applicable | head of this report |

## 12. The open-forks audit, and the marker greps (METHOD rules 58, 59, 60)

**The audit**, `expected = previous OPEN count + forks raised + gated findings filed with options named and none chosen − rulings recorded`, then `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`. Re-measured at the start (6: F4, F5, F7, F8, F10, F11), not copied. The "measured" column is read from each commit's own tree with `git show <sha>:FORKS.md`; for the halt commit it is the pre-commit grep of the staged tree.

| Commit | What it does to forks | Expected | Measured |
|---|---|---|---|
| `e48feb5` (the prompt) | nothing | 6 + 0 + 0 − 0 = 6 | 6 |
| `328febb` (Item 0 records: D-20, D-21, D-22, R1) | nothing (no fork is filed there) | 6 | 6 |
| `2739375` (Items 1–2, sources.md) | nothing | 6 | 6 |
| `c3a52bd` (progress record) | nothing | 6 | 6 |
| the halt commit `6248886` | F24 and F25 raised, both OPEN | 6 + 2 + 0 − 0 = **8** | **8** |
| *session 2* — `f12d565` (Item 10: F24 and F25 RULED (a); D-23 and D-24 filed) | two rulings recorded; no fork raised | 8 + 0 + 0 − 2 = **6** | **6** (the pre-commit grep of the staged file, and `git show f12d565:FORKS.md`) |
| `cb39543` (Item 11: the report only) | nothing | 6 + 0 + 0 − 0 = **6** | **6** (both) |
| the Item 12 commit (the report only) | nothing | 6 | **6** (the pre-commit grep of the staged tree; `FORKS.md` is unchanged in it) |
| the final commit (Item 13: the gate line, INDEX.md, the diff scope, the squash message) | nothing | 6 | see the audit line of the Session 2 section, written in that commit |

No DEBT.md entry in this round has "fix shapes named and none chosen" (D-20 to D-22 name their fixes and are not forks); a gated finding with options named and none chosen is F25, counted among the two forks.

**The grep of 001 and 002 for forks with no ruling (METHOD rule 59):** `grep -nE '\bF[0-9]+\b' docs/specs/001-init-spec.md docs/specs/002-rust-code-standard.md`, compared with `FORKS.md`. Hits: F1 (RULED), F2 (RULED; 001 §11's line is stale, D-14), F3 (RULED), F4 (OPEN; no datsan contact here), F5 (OPEN; no localised name written), F6 (RULED (a); this round adds no `std` or `alloc` to the engine), F7 (OPEN; no FFI crate), F8 (OPEN; no rules crate), F9 (RULED (b); governs `golden/buryat/`, which this round does not touch), F10 (OPEN; it concerns software whose system is unknown, which TCG's is not). No fork was found that the round built through; nothing under `crates/` changed.

**The marker greps (METHOD rule 58).** The grep: `grep -rn --exclude-dir=.git --exclude-dir=target --exclude-dir=prompts -e <the round's marker> /home/kitaeza/github/zurkhai`. `e48feb5` (the prompt) was committed after its `cmp` only: no tracked file could hold the marker yet, and the OPEN count was not grepped before it (the count 6 for that commit in the audit below is read from its tree afterwards, not run before it). Before `328febb`, `2739375` and `c3a52bd` the chain asserted the marker count `= 0` and the OPEN count `= 6`; re-measured now with `git grep` on each of the four trees: 0 hits each. Before the halt commit: the single placeholder line of §14 exists while the diff is measured, then is replaced by the measured line, and the chain of the halt commit asserts 0.

**Session 2's marker greps** (the same grep with session 2's marker, which this report never spells; the chain of each commit asserts 0 before `git commit`, and the results below are read from the chains and from each commit's tree): `f12d565`: the chain asserted 0 hits; `git grep` of the commit's tree (excluding `docs/specs/prompts`) finds 0. `cb39543`: the chain asserted 0; the tree has 0. The Item 12 commit: the chain asserts 0. The final commit: the diff-scope line of the Session 2 section is written with the marker as a placeholder while the diff is measured and replaced by the measured line, and the chain asserts 0. Session 1's marker is not spelled anywhere in the report, and session 2 does not know it, so session 2 did not re-run session 1's grep on any commit; the rows above are session 1's own, as it recorded them. Where session 1 says a grep was not run before a commit (`e48feb5`), that stands.

## 13. Item 8 — the halt, by design (STOP RULE (2))

Item 8's questions, each a fork or a forced answer:

- **a. What TCG's output becomes in the tree:** a fork, **F24 = spec 009 Fork 1** (below).
- **b. Each class E disagreement:** none exists (§1.2: the engine equals the script with (7.13) on all 220,242 days). No fork.
- **c. Each class U disagreement:** none exists (every one of the 53 is settled by the script and by P-ENG). No fork.
- **d. Each class C rule:** none is class C as the prompt defines it (a rule Janson v1 does not state). The one rule on which the two sides differ is a variant Janson v1 does state; its choice is a real decision, **F25 = spec 009 Fork 2** (below). The rule that would decide it, CLAUDE.md hard rule 7, does not: it forbids changing a constant to make two sides agree, and the question here is which of two printed formulas a system is.
- **e. Each finding of Items 2–4 carried forward:** TCG against the paper's tables: none (0 mismatches in 341 table rows). A constant that differs (Item 3): $a_2$, which is F25. A non-calendar field of the build check: none differs (8,940 of 8,940 lines equal). The two TCG defects of §4b (no file name for Genden; the epoch flags `tsurlug` and `zlapure` not reset) and the empty format at `t4.c:700` are facts about TCG 1.06, with no decision of the project in them beyond the cost they add to F24's options (a tool must work around the first two); forced answer: no separate fork, they are carried in F24.
- **Class H disagreements need no fork** (the prompt): the 53 are listed in §1.2 under the qualifier stated there, and a fork is raised because the qualifier changes the answer. Hard rule 7 is respected: nothing was changed under `crates/`, `golden/`, `rules/` or any constant, on either side, to make them agree; the probe copies stay in the scratchpad.

Dates of the lists inside 2021–2026, as arithmetic only (they say nothing about which system any calendar follows, and no Buryat source was read): Tsurphu 2022-06-28, Genden 2024-09-19, Phugpa 2025-11-19.

```
STOP AND ASK — spec 009 Fork 1, what TCG's output becomes in the tree

DONE AND GREEN
  Items 0–7 are done and committed. TCG 1.06 builds from Henning's unchanged source with gcc (-O0 and
  -O2 byte-identical on 606 files per run) and reproduces his six published Phugpa years on all 8,940
  lines. Its read output equals Janson v1's Tables 7, 8, 9 (and 1 for Phugpa) in all three versions
  (0 mismatches in 283 cells). The engine's dump equals the T4 fixture on 73,414 of 73,414 rows. TCG and
  the engine disagree on 53 days of 1900–2100 (17 Phugpa, 13 Tsurphu, 23 Genden), all settled (Fork 2).
  Nothing under crates/, golden/, rules/ or tools/ changed.

THE FINDING
  Henning's TCG, built here, is a second independent implementation of all three systems (it is not
  independent of Janson: the script of Item 7 reproduces it exactly with Janson's (7.24)). Its output is
  held nowhere in the tree except as counts, 53 listed days and the commands of this report.
  001 §6.1, as written: T4 is "Third-party software (e.g. Lotsawa House's Phugpa calculator, built on
  Janson) | Engine vs Phugpa only — never Buryat practice"; T5 is "Tables printed in Janson / Henning |
  Engine vs the paper's own system". TCG's Phugpa rows fit T4 as written. TCG's Tsurphu and Genden rows do
  not fit T4 ("Engine vs Phugpa only"). Whether either TCG's output built in this round, or Henning's own
  published lists (the six tdata/pl_*.txt files; his other lists are not in this round's grants), are T5
  ("Tables printed in … Henning") is a reading of 001 §6.1 that this round cannot make.
  TCG has two defects that a re-runnable use must work around (§4b): "Print calendar" writes no file for
  Genden as a first action (exit 139), and selecting Genden after Tsurphu leaves Tsurphu's leap-month
  rule on (for 2012 it prints 384 days where Genden has 355); and a line of its output (the yoga/karana line of the first of
  two days that share a tshes) is never written (t4.c:700), which a published file shows too.

EVIDENCE
  MEASURED: the counts above (§1.1, §5, §7, §8); TCG's output rows per version 73,414 each; three full
  runs and a probe, each with two builds; the Genden defects (exit 139; 384 rows for 2012 against 355).
  INFERRED: that a fixture or tool would be used by M3's Q1 differential only as an extra check on the
  engine's three systems; no Buryat source is involved.
  RECALLED: none.   RELAYED: the origin of tcg1309.zip (kalacakra.org over HTTP; the human placed it in
  the library, docs/sources.md).

THE QUESTION
  What does TCG's output become in the tree, and at which tier if it becomes a fixture? A fixture under
  golden/ and any tier beyond T4-as-written for Phugpa change what the project's oracles mean; a tool
  under tools/ adds a maintained dependency on a third-party program. Both are decisions of the human
  (docs/ROUNDS.md §6) and cannot be made inside this round's scope (STOP RULE (7)).

OPTIONS — a starting set, not a menu
  1. Nothing beyond this report. Cost: zero in the tree; the differential cannot be re-run from the tree
     (the commands, stand-ins, probes and scripts are in this report's last section, so a later session
     can re-run it by hand); M3 may cite this report only (a dated report, METHOD rule 26).
  2. A fixture under golden/reference/. Sub-choices: (i) Phugpa only, or all three versions; (ii) the full
     rows (73,414 per version, about 1.98 MB each, 5.9 MB for three) or only the disagreement lists (53
     lines) with the epoch comparison. Real cost: golden/** is gated and needs an accepted spec naming it;
     a tier: T4 as written fits Phugpa only (and TCG's Phugpa rows add no oracle strength: the engine
     already equals the T4 fixture, and TCG differs from both on 17 days); T5 as written is "Tables printed
     in Janson / Henning", which fits Henning's published lists (the six Phugpa files; his Tsurphu lists
     follow a different Sun, D-19) only if "printed" includes his website's calendars, and does not fit
     software output built here; for Tsurphu and Genden any tier needs an amendment of 001 §6.1 (an
     accepted spec, changed only by revision); each tolerated mismatch needs a D-n (METHOD rule 44), and
     F25's answer decides whether the 53 days are mismatches at all; the meta carries the zip's sha256,
     the gcc flags and the Genden workaround.
  3. A re-runnable tool under tools/ that rebuilds TCG from the library and runs the differential. Real
     cost: tools/ is not gated, but the tool needs the library (outside git), so CI cannot run it and its
     tripwire (METHOD rule 38) checks only preconditions; about 1,300 lines of Python, 47 of Rust and 31 of C
     headers (this report's last section: the readers, the checks, the reference script, the probe
     comparison, the Rust dump probe, the stand-ins; the Genden driver is a shell loop); Python in a tree whose tools are shell and node, a language decision; maintenance
     whenever the engine's or the fixtures' formats change.
  4. Options 2 and 3 together.
  The adjudicator may answer with something not listed (for example, option 2 for Phugpa now and options
  for the other versions after F25).

IF THE ANSWER IS DELAYED
  Nothing. The round has ended by design; the branch stands still (docs/ROUNDS.md §3).
```

[CORRECTED 2026-11-11 (spec 009 session 2, Item 12): the block above says "0 mismatches in 283 cells". The count measured in §7 and §17 is 0 mismatches in **341 table rows**: Table 7, 21 rows × 3 versions = 63; Table 9, 31 × 3 = 93; Table 1, 120; Table 8 events, 23 + 21 + 21 = 65; 63 + 93 + 120 + 65 = 341. This session re-counted the data rows of the ten T5 files the reading check reads (Table 1; Tables 7, 8 and 9, three versions each) (`wc -l` less one header line each): 120 + (21 × 3) + (23 + 21 + 21) + (31 × 3) = 341 again. What "283" counted cannot be derived from the round's measurements: no sum of these groups gives it (all 341; without Table 1 221; without Table 8 276; Phugpa and Tsurphu only 268), and the halt's session left no record of how it was reached. The block above stays as written, as the halt's history.]

```
STOP AND ASK — spec 009 Fork 2, which anomaly increment a2 the engine's three systems compute

DONE AND GREEN
  The 53 disagreements are settled to one cause by measurement: the Moon's anomaly increment per lunar
  day. The engine computes a2 = 1/28, Janson v1 eq. (7.13), p. 23; TCG computes a2 = 3781/105840,
  eq. (7.24), p. 25 (Remark 14). The script of Item 7 reproduces the engine with (7.13) and TCG with
  (7.24) on all 73,414 days of each version (0 differ); a probe of the engine with (7.24) reproduces TCG
  on all 73,414 days of all three versions and changes exactly the 53 days; a probe of TCG with (7.13)
  removes 52 of the 53 (§1.2). No class E and no class U disagreement exists.

THE FINDING
  Janson v1 prints both values for one quantity. Remark 14 (pp. 25–26): (7.13) "is usually used when
  calculating calendars, for example in the almanac [14]"; (7.24) is Henning's exact value, "also used in
  his computed calendars", which Janson calls "mathematically more natural". The two give different
  Tibetan dates on 17 days of Phugpa, 13 of Tsurphu and 23 of Genden in 1900–2100 (Janson found 9 + 8
  for Phugpa; this round's list has 9 + 8, with his three dates 2001-02-10, 2006-05-10, 2025-11-19).
  Janson's printed Tables 1, 7, 8, 9 do not separate them (the engine under (7.13) and TCG under (7.24)
  both reproduce every printed cell), and the T4 library equals the engine, so it uses (7.13) (P6, spec
  006). The engine chose (7.13) in spec 005 (Example I) and spec 008 (citation row 21) without filing
  a fork or a debt entry (the choice was recorded only in report prose, METHOD rule 56). The true date
  of every one of the 53 lunar days that decide the labels lies within 0.00066 d (56 s) of a day
  boundary, so the choice matters only on those days.

EVIDENCE
  MEASURED: §1.2 (the 53 rows with both true dates; the two probes; the script runs); the engine's
  phugpa.rs:81, 421 (A2 = 126/3528); TCG's t2.c:623, 637; the T4 fixture equals the engine on 73,414
  rows. INFERRED: that a Buryat calendar made with either value would differ from the other only on
  these 53 days (and on days outside 1900–2100); no Buryat source was read. RECALLED: none.
  RELAYED: the advisor's observation that Henning's Phugpa lists differ from the T4 fixture on
  2001-02-10 and 2025-11-19, re-derived here (2 of 2,159, both in the 17).

THE QUESTION
  Which anomaly increment does each system of the engine compute? The choice defines what "Phugpa",
  "Tsurphu" and "Genden" compute on 53 days of 1900–2100. M3 (the Q1 verdict) compares Buryat calendars
  with all three, and on those days the choice can decide a match. It is not local and not cheap to
  undo (it changes crates/tshes, golden expectations and the meaning of three systems), and no rule or
  ruling decides it: 001 §9 names Janson as the primary source and Janson prints both; hard rule 7
  forbids tuning a constant to data, not choosing between two printed formulas (the prompt's class
  H, "no fork", does not fit, §1.2).

OPTIONS — a starting set, not a menu
  1. Keep (7.13) for all three systems, and file the 53 days (a DEBT.md entry with a TRIGGER: the Q1
     differential finds a source covering one of them). Cost: engine and fixtures unchanged; the engine
     agrees with the T4 library and Janson's main text, and disagrees with Henning's software and his
     published lists on 53 days.
  2. Switch all three systems to (7.24). Cost: a fix round in the gated crate; the Phugpa P6 test
     against the T4 fixture then fails on 17 rows (the fixture was generated with (7.13)), so it needs
     a tolerated-mismatch record with D-ids (METHOD rule 44) or a new oracle; spec 005's and spec 008's
     Remark 14 tests are re-audited; the engine then agrees with Henning and disagrees with the library.
  3. Both, as a named variant of each system (never a default: DEBT.md D-1). Cost: a public API change
     (crates/tshes is gated and published API is a promise, D-12); Q1 then has six candidates; a
     variant needs its own fixtures (TCG's rows are one oracle for it, F24); for the days outside the 53
     the variants are identical.
  4. Decide at the Q1 differential: leave the engine unchanged and record the 53 days (as option 1), with
     the explicit statement that the choice is open and is to be taken when a Buryat source covers one of
     them (the lists have 3 days in 2021–2026: Tsurphu 2022-06-28, Genden 2024-09-19, Phugpa 2025-11-19).
  The adjudicator may answer with something not listed.

IF THE ANSWER IS DELAYED
  Nothing. The round has ended by design; the engine keeps (7.13), by D-1 not as a default and not as a
  ruling, and the decision stays visibly open (CLAUDE.md hard rule 18).
```

Both forks are filed as OPEN rows in `FORKS.md` (F24, F25) in the halt commit.

## 14. Diff scope, and the stop rule that governed the round

**The stop rule that governed the round: STOP RULE (2), Item 8 is a STOP AND ASK by design.** Not fired: (1) every ARTIFACT and PRECONDITION check passed; (3) the program built from unchanged files with gcc and header stand-ins; (4) the build check had no difference; (5) the largest disagreement count is 23, below 300; (6) no class E disagreement; (7) nothing out of scope was written (no change under `crates/`, `golden/`, `rules/`, `clients/`, `tools/`, `docs/glossary.md`, `docs/METHOD.md`, `GATED.md` or an accepted spec; no Henning program other than TCG's three epochs; no Buryat source; no fork decided); (8) no read, run or install outside GRANTS was refused; the reads and runs the grants do not name are under Latitude.

**Diff scope** (`git diff --numstat main` measured on the staged tree of the halt commit, the commit that carries this line; the report's own line is the line below):

`git diff --numstat main` (insertions, deletions, file): 24 0 DEBT.md; 12 0 FORKS.md; 2 1 docs/sources.md; 2185 0 docs/specs/009-henning-differential.md; 508 0 docs/specs/prompts/009-henning-differential.md; TOTAL (5 files) 2731 insertions = 24 + 12 + 2 + 2185 + 508, 1 deletion = 0 + 0 + 1 + 0 + 0 (the 1 deletion is the `henning-kalacakra/` row of `docs/sources.md`, which gained a sentence; the new row is an insertion).

- **Gated paths touched: no.** The round's own `docs/specs/009-henning-differential.md` and `docs/specs/prompts/009-henning-differential.md` are not changes to an accepted spec (GATED.md). Not gated, changed as the items say: `DEBT.md` (D-20, D-21, D-22), `FORKS.md` (F24, F25), `docs/sources.md` (one row, one sentence). `docs/specs/INDEX.md`: not changed at the halt (the row lands in the final commit, after the resume). `crates/`, `golden/`, `tools/`, `rules/`, `clients/`, `Cargo.toml`, `Cargo.lock`, `docs/METHOD.md`, `GATED.md`: untouched.
- **Files the test framework wrote:** none in the working tree (no `proptest-regressions` file; the probe test ran in a copy with no property test).
- **Commits:** `e48feb5` (the prompt), `328febb` (Item 0 records), `2739375` (Items 1–2), `c3a52bd` (progress record), and the halt commit.

## 15. Forks encountered, including the ones resolved

- **Two STOP AND ASKs (§13): F24 and F25.**
- **Resolved without escalating** ("considered X, chose Y because Z"):
  1. *How to run a program that writes into its working directory when every Bash call has absolute paths and no `cd`.* Considered a wrapper script, a `cd` in a subshell (rule 16), a python `subprocess` call, and `env --chdir=<dir>`. Chose `env --chdir`, the convention of round 006, because it changes nothing but the directory and the program's own interface is untouched; it is not named in GRANTS (Latitude).
  2. *How to get a Genden calendar out of TCG* (§4b). Considered editing `t4.c` to name a file (forbidden: STOP RULE (3) allows no edit of Henning's files), an `LD_PRELOAD` or a macro for `fopen` (a workaround by another route, METHOD rule 67), and the program's own interface. Chose the interface: a Phugpa print first (which sets the file name and leaves the flags at their fresh values), shown independent of the pre-print year.
  3. *The class of the 53 disagreements* (§1.2). Considered H, C, and a new cell. Chose H with a stated qualifier, because the probe confirms the place in TCG's source (H's test) and the other classes' definitions fail more clearly (C: "a rule Janson v1 does not state"; the rule is stated); and raised F25 so that the qualifier does not hide a decision.
  4. *The treatment of 2099-10-14.* Considered counting it as unsettled by P-TCG (U), or as settled by the script and P-ENG. Chose settled: P-ENG removes it, the script settles it, and the residue of P-TCG is explained by a measured mechanism (`monthjump`).
  5. *How to carry TCG's epochs to Janson's.* Considered carrying by the true-month count alone (off by one at E806: Remark 4) and finding K by the quantities themselves. Chose the latter and checked it against the count (§6).
  6. *Fork F25's number of options.* Considered splitting "keep (7.13)" from "defer to Q1". Kept two options because the engine is unchanged in both and the difference is the entry, not the decision; the adjudicator may answer otherwise.

## 16. Latitude taken beyond or differently from the prompt

1. `env --chdir=<dir>` ran the built TCG and the two probe builds (GRANTS lists "running the program built there … with its input from a file or a pipe", not `env`); it was the only way to give a program that writes relative file names a directory without `cd`. No denial occurred.
2. `-std=gnu89` was chosen (GRANTS allows choosing compiler flags).
3. A third seeded control in Item 2 (a day line removed) and a fourth (an unchanged copy), beyond the two the prompt names; the same for several extra seeded trials for the Item 7 script (§1.2).
4. Two probes in Item 7 (P-TCG, P-ENG) where the prompt asks for a probe on "a copy of the side concerned"; P-ENG is the second way of METHOD rule 28.
5. The seeded-constant control of Item 7 was found only after eight trials (§1.2); all trials are reported, the choice of the reported control was made after seeing them.
6. Python scripts were written first into a plain directory (`scratchpad/scripts`, made by the Write tool) and then moved by `cp` into a `mktemp -d` directory; the first directory was removed with `rm -r` (GRANTS: `rm -r` only inside `mktemp -d` directories). The first `cmpbuild.py` counted a final empty element as a line; it was changed to drop it, and the build check was re-run (the first run's totals were 8,946 lines; the corrected 8,940).
7. **Four Bash calls began with a `cd`**, against hard rule 16 and the prompt's `PATHS:` line: one `cd /tmp 2>/dev/null; grep -n … t4.c` (listing `t4.c`'s functions), and three `cd /home/kitaeza/github/zurkhai 2>/dev/null; …` (a `grep` of `FORKS.md` for the end of F23, a `git grep` for earlier statements about $a_2$, and the loop that measured the OPEN count and the marker at four commits). None changed anything, and each continued with absolute paths in the same line. R1's account in §3 says so.
8. The `gcc` option `-std=gnu89` and the run of 202 processes per build for Genden are the only departures from "Run 'Print calendar' … over enough Tibetan years" (§4b).
9. The Item 1 sources.md row landed one commit after the first citation of ARTIFACT (B) (§4).
10. The progress record commit `c3a52bd` (a fallback-clause commit at an item boundary) is replaced by this complete report.

**R8 (ADDENDUM 1, 2026-11-11; the human: "Agree with all recommendations"), item by item.** No grant is made after the fact for any of them.

| Session 1's item | R8 | What it was |
|---|---|---|
| 1 | accepted as reported | `env --chdir=<dir>` ran the built TCG and the probe builds, a tool GRANTS does not name |
| 2 | no ruling needed (a choice inside GRANTS and the items) | `-std=gnu89`, a compiler flag |
| 3 | no ruling needed | a third and fourth seeded control in Item 2 and extra seeded trials in Item 7 |
| 4 | no ruling needed | two probes (P-TCG, P-ENG) in Item 7 |
| 5 | accepted as reported | the seeded control of the Item 7 script was chosen after eight trials, all reported |
| 6 | accepted as reported | `rm -r` of a plain scratch directory that was not a `mktemp -d` directory (GRANTS allows `rm -r` only inside `mktemp -d` directories) |
| 7 | accepted as reported | four Bash calls began with `cd` |
| 8 | no ruling needed | the `gcc` option and the 202 processes per build for Genden |
| 9 | accepted as reported | the `docs/sources.md` row for `henning-tcg/` landed one commit after the first citation of ARTIFACT (B) |
| 10 | no ruling needed | the progress record commit `c3a52bd` |

Five accepted (1, 5, 6, 7, 9) and five that need no ruling (2, 3, 4, 8, 10): 5 + 5 = 10 = §16's ten items.

**Session 2's own latitude** (MEASURED; nothing here is a read, run, fetch or install outside ADDENDUM 1's GRANTS; no permission was denied):

11. The prompt copy in the tree was compared with `cmp -n 41868` (its length), not whole-file `cmp`, because it is a byte-for-byte prefix of the prompt file and does not hold the addendum (S2.1). The copy was not changed. **R10 (ADDENDUM 2, 2026-11-11; the human: "Agree with your recommendations"): accepted as reported (ADDENDUM 2, R10); no grant.** [Resolved 2026-11-11 by session 3 (ADDENDUM 2, R9): ADDENDUM 1 and 2 are appended; see Session 3.]
12. Item 11 asked for the places the advisor found and "any others you find". I replaced, as class (a), the conditions, assignment, call and quoted strings found by reading (§4b, §4c, §5, §1.2) and the entry-point declaration of `main` (§5), which could be read as class (d); I chose to describe it (S2.3). The report's opening paragraph carried a third "never copied" statement, which ADDENDUM 1 does not name; I added the `[CORRECTED …]` record there, and one on its sentence "The round ran in one session".
13. The first attempt of the Item 10 commit chain stopped at one of its own guards (a locale-dependent `sort` in the `git status --porcelain` comparison): nothing was staged or committed, and the chain was re-run with `LC_ALL=C sort`.
14. `git show 6248886:docs/specs/009-henning-differential.md` was redirected into a `mktemp -d` directory, as Item 11 words it ("written into $T"); `awk` and `grep -F` read it there.

## 17. Numbers that must sum

- **Disagreements:** 17 + 13 + 23 = **53**; by field 17 + 1 (month on 2099-10-14) in Phugpa, so Phugpa's 17 disagreeing days = 17 tshes days, of which 1 also differs in month (the month field counts 1, the tshes field 17: the disagreeing day is counted once); 13; 23. Class table: 53 = 53.
- **Runs:** 17 + 13 + 23 = 53 runs, longest 1.
- **Days compared:** 73,414 = days from 1900-01-01 to 2100-12-31 inclusive, per version; 3 × 73,414 = 220,242 days against the script, per implementation.
- **Rows in the T4 shape:** skipped − doubled = 30 × 2,486 − 73,412: Phugpa 2,800 − 1,632 = 1,168; Tsurphu 2,799 − 1,631 = 1,168; Genden 2,802 − 1,634 = 1,168; 74,580 − 73,412 = 1,168.
- **Steps:** 1,632 + 68,981 + 2,800 = 73,413 (= 73,414 − 1) for Phugpa; 1,631 + 68,983 + 2,799 = 73,413; 1,634 + 68,977 + 2,802 = 73,413.
- **Months:** 1,168 + 1,318 = 2,486 complete months per version; 2,486 + 2 = 2,488.
- **Build check:** 1,470 + 1,470 + 1,590 + 1,470 + 1,470 + 1,470 = 8,940.
- **Output files:** 3 versions × 202 years = 606 per build; 606 = 606 equal between the builds, for the original and for the probe.
- **Day-lines:** 73,796 per version = 73,414 + 382.
- **The reading check:** table rows compared = Table 7: 21 × 3 = 63; Table 9: 31 × 3 = 93; Table 1: 120; Table 8 events: 23 + 21 + 21 = 65; 63 + 93 + 120 + 65 = **341** = 195 (Phugpa) + 73 + 73; 0 mismatches.
- **The class table:** 53 = 53; the probes: P-ENG 17 + 13 + 23 = 53 removed; P-TCG 16 + 13 + 23 = 52 removed, 1 left.
- **Latitude:** see §16: session 1's ten items + session 2's four (11 to 14) = 14; R8: 5 accepted + 5 needing no ruling = 10.
- **Session 2, Item 11 (S2.3):** the 12 lines the check listed on the halt's report = 5 class (a) + 3 (b) + 2 (c) + 2 (d); the 7 lines on the working file = 0 (a) + 3 (b) + 2 (c) + 2 (d).
- **Session 2, the open forks:** 8 at the halt − 2 rulings (F24, F25) + 0 raised = 6.
- **The T5 rows of the reading check, re-counted by session 2:** 120 (Table 1) + 63 (Table 7) + 65 (Table 8) + 93 (Table 9) = 341.

## 18. What the prompt got wrong

- **The class scheme of Item 7 has no cell for a variant the paper states.** H, C and the rule "no fork for H" assume that if TCG differs from the engine, one side departs from the paper; here both follow the paper at different printed formulas (§1.2, F25).
- **Item 4 ("Run 'Print calendar' for each version") assumes it works for each.** It does not for Genden as a first action, and its state is not reset between epochs (§4b); the prompt's "STOP RULE (3)" would have stopped the round if it had been read as a build failure; it is a use failure of the program, handled through the program's own interface.
- **Prediction 5** (constants equal on every digit) fails on $a_2$ (§6), and the prediction's basis note ("the advisor … did not build or run the program") shows why the author could not know; Janson's Remark 14 already says it.
- **Item 0 puts the sources.md row "in the commit that first cites ARTIFACT (B)"** while Item 0's own records commit cites it (§4).
- **Item 6's RELAYED paragraph** was right (2 of 2,159; both days in the list); re-derived.
- **Item 2's controls say "exactly the missing line" on a copy with one line removed**, and the first copy chosen (a day line) changes the class of the difference; two copies (a non-calendar line and a day line) were run to show both.
- **"F24 at 07b5bbf", "D-20 at 07b5bbf", "OPEN count 6", "009 free"** were all right.
- **"No cause is named by the run that reproduces a disagreement, only by the run that removes it"** was followed, and the probe P-TCG left 2099-10-14 not removed; the rule did not tell what to do with a probe that removes the disagreement and adds an artifact. Handled by the second probe and by the measured mechanism, reported as such.

**Session 2** (ADDENDUM 1 and what this session found):

- **ADDENDUM 1's Item 11 names two "never copied" statements** (the heading of §4 and the opening of §21). A third stood in the report's opening paragraph, found by `grep` for the phrase; it has its `[CORRECTED …]` record too.
- **The committed prompt copy does not hold the addendum.** Item 0 of the original prompt and hard rule 2 say `cmp` the copy before every later commit; the copy is the prompt as the first commit stored it (41,868 bytes) and the file now holds 64,264. ADDENDUM 1 does not say whether to append itself to the copy (ROUNDS.md §2: "Changes after a session has read the prompt are dated ADDENDA appended to the same file"). Session 2 used the prefix `cmp` and left the copy unchanged (S2.1); a flag for the human, not a fork and not a `DEBT.md` entry (the prompt orders exactly two entries, hard rule 1). [Resolved 2026-11-11 by session 3 (ADDENDUM 2, R9): ADDENDUM 1 and 2 are appended; see Session 3.]
- **Remark 14 begins on p. 24**, not p. 25: its opening lines ($m_2 = m_1/30$ and $s_2 = s_1/30$) are on p. 24, and the passage on $a_2$ ((7.24), "usually used when calculating calendars", the examples) is on pp. 25–26. The prompt's "Remark 14 (pp. 25–26)" names the pages of the passage; D-23 keeps the prompt's wording as ADDENDUM 1 orders (S2.2).
- **The "283 cells" of Fork 1** cannot be derived; the measured count is 341 table rows (the `[CORRECTED …]` record under Fork 1).
- **ADDENDUM 1's other claims held when measured:** the status was empty and HEAD was `6248886`; the OPEN count was 8; D-23 was the next free id; the check's 12 lines and 7 lines were the ones predicted; the A.2 (p. 38) and A.3 (p. 42) sentences say what the advisor said; both pinning tests exist (`phugpa.rs:991`, `:1012`). No next-free-id mismatch (STOP RULE (12)) occurred. A hook that rejects `cd` (PATHS addition) did not fire: no call of session 2 ran `cd`.

## 19. By-catch

- **TCG's `t4.c:700` format** (a format string that begins with the octal escape for NUL, hence empty; §5) loses one line for the first of two days with one tshes, in every published Phugpa file. A fact about Henning's program, recorded here; no `DEBT.md` entry (the project owes nothing).
- **TCG's two use defects** (§4b), recorded here and in F24.
- **Spec 005's and spec 008's choice of (7.13)** was recorded only in report prose (METHOD rule 56: not filed). It is filed now, by F25 (a fork, not a debt, because options are named).
- **Round 008's report says "Table 8's events 23 / 21 / 21"** and P5 tests exist for all three; consistent with the 23 / 21 / 21 read here.
- Nothing else found.

**Session 2's by-catch:** the committed prompt copy lacks ADDENDUM 1 (a flag; no `DEBT.md` id, see §18). The `henning-tcg/` row's licence cell in `docs/sources.md` is the one other place the Item 11 patterns match outside this report (the licence notice, class (c); S2.3). Nothing else found. [Resolved 2026-11-11 by session 3 (ADDENDUM 2, R9): ADDENDUM 1 and 2 are appended; see Session 3.]

## 20. Held

Identified items only (METHOD rule 6): D-1 (respected: every engine call in the probe names its `System`), D-12 (OPEN, unchanged), D-19 (OPEN: TCG's Tsurphu is A.2's siddhanta Sun; Henning's published Tsurphu lists are the karana variant, RELAYED, not read), D-20, D-21, D-22 (filed, OPEN); F4, F5, F7, F8, F10, F11 (OPEN); F24, F25 (raised, OPEN). The squash commit message is written by the resume, not at the halt.

[CORRECTED 2026-11-11 (spec 009 session 2): at the end of the round the held items are D-1 (respected, unchanged), D-12, D-19 (unchanged), D-20, D-21, D-22 (filed by session 1, OPEN), **D-23 and D-24 (filed by session 2, OPEN)**; F4, F5, F7, F8, F10, F11 (OPEN); **F24 and F25 are RULED (a), not OPEN**. The squash commit message is the section of that name in this file, written in the final commit.]

## Session 2 — the resume (ADDENDUM 1; Items 9–13)

Session 2 is a fresh session on the same branch, `spec/009-henning-differential`, on the same model: Sonnet 5.5 (`claude-sonnet-5-5`), effort High. No switch point to the fallback model was reached. The sections above (§1 to §21) are session 1's report as it stood at the halt commit `6248886`, corrected only where a `[CORRECTED …]` record says so. This section is session 2's.

**Dates.** Session 1 ran `date` at the top of its round: `2026-11-11 18:14:41 MSK`. Session 2 ran `date` once at the top: `ср 11 нояб 2026 20:12:36 MSK` (`2026-11-11T17:12:36Z`). Both are 2026-11-11. Every record that carries a date and was written by session 1 stays as written; every record written by session 2 carries 2026-11-11 from its `date`. The rulings' date, R6 to R8, is the human's, 2026-11-11. Which records each session covers: session 1 wrote Items 0–8 (commits `e48feb5`, `328febb`, `2739375`, `c3a52bd`, `6248886`: D-20 to D-22, F24 and F25 raised, the `docs/sources.md` rows); session 2 wrote the commits listed under "Session 2's commits" below (D-23 and D-24, the rulings on F24 and F25, the `docs/sources.md` cell, and the report's corrections).

### S2.1 Item 9 — the resume (MEASURED 2026-11-11)

- **Read:** the prompt through ADDENDUM 1 (lines 1–787 of `/home/kitaeza/zurkhai-prompts/spec-009-henning-differential.md`, in full), `git log`, the report so far (this file, all 2,185 lines at `6248886`; the scripts of §21.5 by their headings and by the lines Item 11 names, not line by line), `docs/ROUNDS.md` (266 lines, in full), `docs/METHOD.md` rules 31, 36, 50, 51, 56, 58, 59, 60, 64, 65 and 67 (in full), `FORKS.md` (header, F23 to F25 and the status lines), `DEBT.md` (header, D-16 to D-22), `docs/sources.md` (the `henning-kalacakra/` and `henning-tcg/` rows and the headings), `docs/specs/INDEX.md`, 001 §6.4's P6 line, and `crates/tshes/src/phugpa.rs` lines 975–1034 (the two Remark 14 tests, for D-23's "Pinned by"). The Read-first list of the first session's header is not re-read in full by this session: ADDENDUM 1 replaces the session's reads with its Item 9 and its GRANTS.
- **`git status --porcelain`:** empty (exit 0). **`git rev-parse HEAD`:** `62488863abc2ca86482a6bd32ac2b0551e9107a8`, the value ITEM 9 names. STOP RULE (9) did not fire.
- **ARTIFACT (A):** `sha256sum` printed `7cafc7df563a3020849c86f4e397daa18b7f7f5ff46244403180aca90b9d0f77`; `pdfinfo` printed `Pages: 86`. **ARTIFACT (B):** `ls -l` shows one file, `tcg1309.zip`, 93,207 bytes; `sha256sum` printed `8389de8efcd18f7a66a7cef9b6277c222e5193b2acec440f721096a5ad1e83d2`; `unzip -l` lists the 15 files (makefile, bcd.h, bcd.ext, tc.h, tc.ext, tests.c, tcgnotes.txt, tc.def, bcd.c, utils.c, t1.c, t2.c, t3.c, t4.c, tcg.exe), 265,616 bytes. Both pass. (C) is not used by this session (ADDENDUM 1).
- **Session 2's marker** (METHOD rule 58) is chosen. This report never spells it; it is the token of the one placeholder line this session writes into this file (the final commit's diff-scope line), replaced before that commit.
- **The OPEN count, re-measured** with the grep `FORKS.md` defines (`grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`) on the tree at `6248886`: **8** (F4, F5, F7, F8, F10, F11, F24, F25), equal to the halt's relayed count. The next free ids, verified in the registers: **D-23** (the last entry is D-22) and, if a fork were raised, F26; none is.
- **The prompt copy in the tree.** `docs/specs/prompts/009-henning-differential.md` is 41,868 bytes and ends at line 508, the end banner of the original prompt; `/home/kitaeza/zurkhai-prompts/spec-009-henning-differential.md` is 64,264 bytes and holds ADDENDUM 1 after it. `cmp` of the two files is not silent (`EOF on … after byte 41868, line 508`); `cmp -n 41868` of the two is silent (the committed copy is a byte-for-byte prefix of the file). Hard rule 2 and Item 0 of the original prompt say `cmp` the copy before every commit; the committed copy was never meant to hold an addendum that did not exist when it was committed, and no item of ADDENDUM 1 asks this session to append it. This session ran the prefix `cmp` before each commit and changed nothing in the copy; the choice to append the addendum to the tree copy, or to leave it outside the tree, is the human's (a flag, repeated under "What the prompt got wrong"). [Resolved 2026-11-11 by session 3 (ADDENDUM 2, R9): ADDENDUM 1 and 2 are appended; see Session 3.]
- **The gate** did not run before Item 13 (ROUNDS.md §3 asks a resume to run the workspace's tests before building anything; this session builds nothing and no tracked source changed since `07b5bbf`, as ADDENDUM 1's Item 9 says).

### S2.2 Item 10 — the rulings onto the registers (R6, R7)

- **`FORKS.md` F24:** Status OPEN → RULED, with the row `Ruled: 2026-11-11, (a) — nothing beyond spec 009's report: no tier, no golden row, no tool; DEBT.md D-24 names when the differential is run again; the human: "Agree with all recommendations" (spec 009 ADDENDUM 1, R6).` **F25:** Status OPEN → RULED, with the row `Ruled: 2026-11-11, (a) — a2 = 1/28, Janson v1 (7.13), …; the 53 days … are DEBT.md D-23; the human: "Agree with all recommendations" (spec 009 ADDENDUM 1, R7)`, and the advisor's three reasons, RELAYED. Neither row gains a `Built in:` line: nothing is built.
- **The A.2 and A.3 sentences, FETCHED this session** (the library copy through `pdftotext -layout` into a `mktemp -d` directory; pages by the running headers of the text): A.2, **p. 38**: "The Tsurphu tradition uses the astronomical functions in Section 7 with the same values as given there for the Phugpa version of the constants m1, s1 and a1 (and m2, s2 and a2, see Remark 14) for mean motions, while the epoch values m0, s0 and a0 are different." A.3, **p. 42**: "The constants m1, s1 and a1 for mean motions are the same in the New Genden version as in the Phugpa version, see Section 7, and thus so are m2, s2 and a2, see Remark 14, while the epoch values m0, s0 and a0 are different". Reason (3), **p. 25**: "Nevertheless, the simpler (7.13) is usually used when calculating calendars, for example in the almanac [14]." (7.13) is on **p. 23**, (7.24) on **p. 25**. Reason (1): 001 §6.4 reads "P6. Phugpa reproduces T4 output exactly over at least 1900–2100." (FETCHED from the tree, `docs/specs/001-init-spec.md:137`), and the dump check of §8 above is the evidence that the T4 fixture equals the engine's (7.13) dump on all 73,414 days. All three reasons are checked against their sources and stand as the advisor relayed them. **A page note:** the text of Remark 14 begins on **p. 24** (its first lines, on $m_2 = m_1/30$ and $s_2 = s_1/30$); the passage on $a_2$, (7.24) and the examples is on pp. 25–26. The register entries keep the prompt's wording "Remark 14, pp. 25–26", as ADDENDUM 1 orders ("no … page … may change").
- **`DEBT.md`:** two entries filed, numbered from the next free id (D-23, verified): **D-23** (COVERAGE; the 53 days, TRIGGER at the Q1 differential) and **D-24** (COVERAGE; no full-range oracle for Tsurphu and Genden, TRIGGER at the first round after spec 009 that edits `crates/tshes/src/`, or the Q1 verdict spec). The fields are written as given; the statements are reworded only for the register's style (backticks around file names, the ids and the F-numbers). **D-23's "Pinned by"** reads the grep ADDENDUM 1 orders: `git grep -n -E` for `remark_14_the_two_versions_of_a2_differ_on_the_days_janson_lists` and `remark_14_footnote_27_the_two_versions_differ_on_9_and_8_days`, at HEAD under `crates/tshes`, found **both**: `crates/tshes/src/phugpa.rs:991` and `:1012`, each on `Epoch::E806` (Phugpa only), so the entry names them. Tsurphu and Genden carry no test that pins (7.13) against (7.24); the entry says "Phugpa only".
- **`docs/sources.md`:** the `henning-tcg/` row's "Trusted for" cell: "for the spec 009 differential only. No tier and no golden row until the human rules at the halt (spec 009 Item 8)." became "for the spec 009 differential and its re-runs (`DEBT.md` D-24) only. No tier and no golden row (`FORKS.md` F24, RULED (a), 2026-11-11)." The cell's last sentence (on `tcg.exe`) is unchanged; nothing else in the file changed (`git diff --numstat`: 1 insertion, 1 deletion).
- **The open-forks audit for this commit** (METHOD rule 60): expected = 8 (previous OPEN) + 0 (raised) + 0 (gated findings with options named and none chosen) − 2 (F24 and F25 ruled) = **6**; the pre-commit grep of the working `FORKS.md`, which is the staged file, printed **6**. The marker grep of this commit: 0 hits (the grep excludes `.git`, `target` and `prompts`). `git status --porcelain` before the chain: `DEBT.md`, `FORKS.md`, `docs/sources.md` and this file, nothing else; no `proptest-regressions` file.

### S2.3 Item 11 — the report: what it must not carry (C3; CITATIONS) (MEASURED 2026-11-11)

**The check**, as ADDENDUM 1 gives it, with `$T` and `$X` two `mktemp -d` directories under the scratchpad and `$X` an extraction of `tcg1309.zip` (`unzip -q` into it; 15 files):

```
awk '{sub(/\r$/,""); gsub(/[ \t]/,""); sub(/;$/,""); if (length($0)>=12) print}' $X/t1.c $X/t2.c $X/t3.c $X/t4.c $X/utils.c $X/tests.c $X/bcd.c $X/tc.h $X/bcd.h $X/tc.ext $X/bcd.ext $X/tc.def $X/makefile | LC_ALL=C sort -u > $T/patterns.txt
awk '{gsub(/[ \t]/,""); print}' <the report> > $T/report-nows.md
grep -n -F -f $T/patterns.txt $T/report-nows.md
```

The patterns file holds **2,076** patterns from the 13 text files. The halt's report was written with `git show 6248886:docs/specs/009-henning-differential.md` into `$T` (2,185 lines); the working file was checked after the fix.

**The control (METHOD rule 31; STOP RULE (10)): PASSED.** On the halt's report the check lists **12 lines: 133, 184, 186, 190, 198, 652, 653, 694, 1570, 1572, 1574 and 1608**, among them **line 694, the line of §21.3's P-TCG block that copies `t2.c:637`**. The check is able to find a copy; STOP RULE (10) did not fire. (Prediction 16: HELD, the 12 lines are the 12 the advisor listed.)

**Classification of the 12 lines at the halt** (14 matched substrings; line numbers are the halt's):

| Line | Section | What matched | Class | Action |
|---|---|---|---|---|
| 133 | §1.2 (ii) | a call and an assignment, both at `t2.c:623` and `t2.c:637` (two substrings) | (a) | replaced by a description with file:line |
| 184 | §4 | the copyright notice line, quoted with the licence under Item 1 | (c) | stays |
| 186 | §4 a | a menu entry, as `tcgnotes.txt` documents it | (c) | stays |
| 190 | §4 c | a call in `t4.c`'s day loop | (a) | replaced |
| 198 | §5 | the format-string call at `t4.c:700`, and the entry-point declaration of `main` (two substrings) | (a), (a) | both replaced (see below on `main`) |
| 652, 653 | §21.2 | two `#include` lines of a standard header in this round's own `conio.h` stand-in | (d) | stay |
| 694 | §21.3 | the line of the P-TCG diff that copies `t2.c:637` | (a) | replaced |
| 1570, 1572, 1574 | §21.5 | the `TCG` dictionary's epoch year, one per version, in lines that hold numbers cited with file:line | (b) | stay |
| 1608 | §21.5 | the print string of `epochs.py`, which copies `t2.c:623` | (a) | replaced |

By class, at the halt: **(a) 5 lines** (133, 190, 198, 694, 1608; the five prediction 16 names), **(b) 3**, **(c) 2**, **(d) 2**; 5 + 3 + 2 + 2 = 12.

**Places the check cannot see, confirmed by reading (the blind spots of ADDENDUM 1).** §19, halt line 574: the format string of `t4.c:700`, a shorter piece than the source line (class (a)); §21.3, halt line 696: the second line of the P-TCG diff, a copied line with a term removed (class (a)); §21.5, halt line 1609: the continuation of the print string, which holds the term of `t2.c:637` (class (a)). Each is replaced. **Others I found by reading the halt's prose** (ADDENDUM 1: "add any others you find"), all class (a), all replaced by descriptions with file:line: the `+ tt` fragment in §1.2's P-TCG row; in §4b, the conditions on the flag `tsurlug` and on the epoch `epch`, the assignment that leaves two flags at 0, the call that opens a file with an empty name, and the program's two prompts and its error message (text of the source's strings); in §4c, the expression that builds the mean Sun from two values, and a quoted comment of Henning's; in §5, a quotation of the message that gives the highest figures used. The entry-point declaration of `main` (`t1.c:344`) is, strictly, a line of Henning's source that is also a textbook C form; I classed it (a) and replaced it (§16, item 12). The P-ENG block of §21.3, a diff of the engine's `phugpa.rs`, was replaced by a description with file:line, as ADDENDUM 1 orders (C3); Item 5's dump probe, through the public API, stays verbatim.

**Two corrections and one note.** `[CORRECTED 2026-11-11: …]` records stand at the §4 heading and at the opening of §21; each says what was removed and where, and quotes none of it. The `epochs.py` block carries the note that one string of the script as recorded differs from the script that ran: the print string after the loop that reports the anomaly per lunar day is now a description with the cites `t2.c:623` and `t2.c:637`; the arithmetic after it is unchanged.

**After the fix, on the working file** (2,221 lines, as committed at Item 11; the line numbers shift in later commits): the check lists **7 lines: 186, 188, 720, 721, 1606, 1608, 1610**: 186 the licence line **(c)**, 188 the menu line **(c)**, 720 and 721 the two `#include` lines **(d)**, 1606, 1608 and 1610 the three lines of `epochs.py`'s values **(b)**. **(a) 0**; (b) 3 + (c) 2 + (d) 2 = 7. I read each of the seven lines and each of the places named above: none copies a statement, a call, a format string or a diff line of the program; no place of the advisor's list remains. (Prediction 17: HELD, 7 lines, none in class (a), the seven the prediction names.) Run on the round's other changed files with the same patterns: `DEBT.md` 0 lines, `FORKS.md` 0, the committed prompt copy 0, `docs/sources.md` 1 line (the licence notice in the `henning-tcg/` row, class **(c)**, written by session 1 under Item 1).

**Blind spots that remain** (ADDENDUM 1's own, and mine): a copy shorter than a whole source line, and a copied line that was then changed. I read the prose and the two scripts' comments for both; the readers' regular expressions in §21.5 match the program's printed output lines (`readtcg.py`), which are the data the round reads, not its source.

**Item 11's pre-commit measures:** OPEN count 6 (expected 6 + 0 + 0 − 0 = 6); marker grep 0 hits; `cmp -n 41868` of the prompt copy against the prompt file silent; `git status --porcelain` before the chain: this file only.

### S2.4 The rulings, the advisor's corrections, the hand-over, and session 2's commits

**The rulings** (the human, 2026-11-11; at the halt the advisor sent its recommendations and the human answered "Agree with all recommendations"). **R6:** F24 (spec 009 Fork 1) is RULED (a): nothing beyond this report; TCG's output gets no tier, no golden row and no tool under `tools/`; whether Henning's published lists, or output of software built from his source, could be T5 under 001 §6.1 stays undecided, since no fixture needs the answer. Recorded in `FORKS.md` F24 and `DEBT.md` D-24 (S2.2). **R7:** F25 (spec 009 Fork 2) is RULED (a): the engine computes a2 = 1/28, Janson v1 (7.13), for Phugpa, Tsurphu and Genden, and (7.24) is not implemented; the 53 days are `DEBT.md` D-23. Recorded in `FORKS.md` F25 with the advisor's three reasons (RELAYED; the A.2 and A.3 sentences and reason (3) FETCHED, S2.2). **R8:** session 1's latitude items 1, 5, 6, 7 and 9 are accepted as reported, with no grant after the fact; recorded in §16.

**The advisor's corrections** (not rulings). **C1:** Item 7's classes and Item 8's line "Class H disagreements need no fork" assumed one printed formula for each quantity; for a2 the paper prints two. Session 1 was right to raise F25, and §18 of the halt's report ("The class scheme of Item 7 has no cell for a variant the paper states") stands. **C2:** prediction 5 was ambiguous; recorded beside its score in §10, which stays FAILED. **C3:** "verbatim" for a probe of Henning's code collides with CITATIONS, and for a probe of the engine puts the implementation into a report (`docs/ADVISOR.md` §6); Item 11 removed both (S2.3). This round's own dump probe (Item 5), through the public API, stays verbatim in §21.3, because D-24's re-run needs it.

**The hand-over.** Session 1 ended at the halt commit `6248886` (Item 8: both STOP AND ASKs written, F24 and F25 filed OPEN). Session 2 began there, on the same branch, with an empty working tree and nothing uncommitted (S2.1). Session 1's report was complete at the halt; session 2 corrected it where a `[CORRECTED …]` record says so and did not rewrite the rest. The STOP AND ASK blocks of §13 stay as written, as the round's history.

**For a re-run (D-24).** A re-run of the differential needs from this report: §21.1 (the build, the runs, the program's input, the Genden work-around of §4b), §21.2 (the stand-in headers), §21.3's dump probe, and the scripts `readtcg.py`, `checkrows.py`, `omitcheck.py`, `readcheck.py` and `diffrows.py` of §21.5, with `janson_ref.py` to settle a disagreement. The probes P-TCG and P-ENG were for naming the cause; they are described in §21.3, with file:line, and not copied (C3), so a session that wants them makes them from the description.

**Session 2's commits:** `f12d565` (Item 10: the registers), `cb39543` (Item 11: the report's copies removed), the Item 12 commit (this section and the corrections above; its hash is in `git log`), and the final commit (Item 13: the gate, `docs/specs/INDEX.md`, the diff scope, the squash message).

### S2.5 Item 13 — the gate and the close (MEASURED 2026-11-11)

**The gate**, `/home/kitaeza/github/zurkhai/tools/gate.sh` with no arguments, run once, with nothing in flight: on the Item 12 commit **`f63ad2f8d3605dcd570ace886f811db75ceedafd`** (`git status --porcelain` empty before and after; the tree equals `07b5bbf` in `crates/`, `golden/`, `tools/` and the manifests). Toolchain `rustc 1.99.0 (b940084d7 2026-09-28)` on every leg that prints one. Exit status 0; the gate's last line: `GATE: 11 legs run — 11 passed, 0 failed, 1 skipped`.

| Leg | Result |
|---|---|
| fmt | PASS |
| clippy | PASS |
| test (debug) | PASS: **147 passed, 0 failed**, 0 ignored (15 `test result` lines: the workspace's test binaries and doc-tests, summed from the gate's output) |
| test_release | PASS: **147 passed, 0 failed**, 0 ignored (15 lines) |
| build_wasm | PASS |
| wasm-pack | SKIP: `crates/tshes-wasm` does not exist (002 §7; METHOD rule 65) |
| deny | PASS |
| msrv | PASS |
| no_std_check | PASS |
| no_alloc_check | PASS |
| zero_deps_check | PASS |
| self_test | PASS: 26 plants rejected with their named diagnostic, 0 failures |

11 passed + 0 failed = 11 run; 11 + 1 skipped = 12 legs. The counts equal round 008's head block (147 and 147, RELAYED), re-measured here from the gate's output. Prediction 18: HELD. The gate wrote nothing into the tree (`git status --porcelain` empty after it).

**That the final commit changes nothing the gate reads.** Both commands were run, the second on the staged final commit, and both printed nothing:

```
git -C /home/kitaeza/github/zurkhai diff --stat f63ad2f HEAD -- crates golden tools Cargo.toml Cargo.lock rust-toolchain.toml deny.toml
git -C /home/kitaeza/github/zurkhai diff --cached --stat -- crates golden tools Cargo.toml Cargo.lock rust-toolchain.toml deny.toml
```

Output of the first: (empty). Output of the second: (empty). So the gate's result on `f63ad2f` holds for the branch tip: the final commit changes only `docs/specs/009-henning-differential.md` and `docs/specs/INDEX.md`.

**The open-forks audit at the final commit** (METHOD rule 60): expected = 6 (previous) + 0 (raised) + 0 (gated findings with options named and none chosen) − 0 (rulings recorded) = **6**; the pre-commit grep of the staged tree printed **6** (F4, F5, F7, F8, F10, F11). The grep of 001 and 002 for forks with no ruling (METHOD rule 59) is §12's, unchanged: no new fork, and nothing under `crates/` changed. The marker greps of session 2: see §12.

**Diff scope at the final commit** (`git diff --stat main`, measured on the staged tree of the final commit, which carries this line; the gated commit is `f63ad2f`):

`git diff --numstat main` (insertions, deletions, file): 40 0 DEBT.md; 14 0 FORKS.md; 2 1 docs/sources.md; 2343 0 docs/specs/009-henning-differential.md; 1 0 docs/specs/INDEX.md; 508 0 docs/specs/prompts/009-henning-differential.md; TOTAL (6 files) 2908 insertions = 40 + 14 + 2 + 2343 + 1 + 508, 1 deletion = 0 + 0 + 1 + 0 + 0 + 0 (the deletion is the `henning-kalacakra/` row of `docs/sources.md`, which gained a sentence); `git diff --shortstat main`: 6 files changed, 2908 insertions(+), 1 deletion(-). [CORRECTED 2026-11-11 (spec 009 session 3, Item 16): this diff scope was measured on the staged tree of session 2's final commit `cf2167e`, when the committed prompt copy held 508 lines; the round's diff scope at its final commit is the Session 3 section's (S3.9).]

- **Gated paths touched: no.** Nothing under `crates/`, `golden/`, `rules/`, `clients/`, `tools/`, `Cargo.toml`, `Cargo.lock`, `docs/METHOD.md`, `GATED.md` or an accepted spec changed. This round's own `docs/specs/009-henning-differential.md` and `docs/specs/prompts/009-henning-differential.md` are not changes to an accepted spec. Not gated, changed as the items say: `DEBT.md` (D-20 to D-24), `FORKS.md` (F24, F25 raised and ruled), `docs/sources.md` (one row, one sentence, one cell), `docs/specs/INDEX.md` (this round's row).
- **Files the test framework wrote:** none (no `proptest-regressions` file; `git status --porcelain` was empty after the gate).
- **Commits of the branch** (both sessions): `e48feb5`, `328febb`, `2739375`, `c3a52bd`, `6248886` (session 1); `f12d565`, `cb39543`, `f63ad2f`, and the final commit (session 2). [CORRECTED 2026-11-11 (spec 009 session 3, Item 16): session 2's final commit is `cf2167e`; session 3's commits (`4742692` and the final commit) are listed in the Session 3 section (S3.9).]
- **The stop rule that governed session 2:** STOP RULE (9) (status, HEAD and ARTIFACT checks) and (11) (a failing gate leg) were the live ones; neither fired. (10) fired as a test, not as a failure: the control found the line it should. (12) did not fire.

## Session 3 — the prompt-copy resume (ADDENDUM 2; Items 14–16)

Session 3 is a fresh session on the same branch, `spec/009-henning-differential`, on the same model: Sonnet 5.5 (`claude-sonnet-5-5`), effort High. No switch point to the fallback model was reached and no design choice arose. It builds nothing, runs no gate, no cargo, no python3 and no awk, reads no source (ADDENDUM 2: no ARTIFACT), and edits two files only: the committed prompt copy and this report. The sections above are the report as it stood at `cf2167e`, changed only where a `[CORRECTED …]` record or a bracketed `[Resolved …]` note says so, and in the head block and the squash message as Item 16 orders.

**Dates.** `date` ran once at the top: `ср 11 нояб 2026 23:45:04 MSK`, that is 2026-11-11; every record session 3 wrote carries 2026-11-11. (One later call, `date +%F`, ran inside a byte-inspection command and printed 2026-11-11 again; see S3.7, latitude 1.) The rulings' date is the human's, 2026-11-11. The three sessions' dates are all 2026-11-11: session 1 `18:14:41 MSK`, session 2 `20:12:36 MSK`, session 3 `23:45:04 MSK`; no record of any session crosses a day. Which records each session covers: session 1, Items 0–8 (`e48feb5`, `328febb`, `2739375`, `c3a52bd`, `6248886`); session 2, Items 9–13 (`f12d565`, `cb39543`, `f63ad2f`, `cf2167e`); session 3, Items 14–16 (`4742692` and the final commit).

**What session 3 read** (READ FIRST, replaced by ADDENDUM 2): the prompt file through ADDENDUM 2, 955 lines, in full (the Read tool returned lines 1–694, then 695–956, a size cap splitting the first call); `CLAUDE.md`, in full as the session context supplied it, not re-read with the Read tool; `docs/ROUNDS.md` lines 25–229 (§2, §3, §4, §5, §6, §7, §8, §9 and the first lines of §10; §1 and the rest of §10 not read; READ FIRST asks for §2, §3, §4 and §9); `GATED.md`, in full (10 lines); `docs/METHOD.md` rule 51 (lines 501–507) and rules 58–65 (lines 565–635), so rules 51, 58, 60 and 65 each in full, with 59, 61–64 read as well; this report: the head block and opening paragraph (lines 1–19), §12 (356–378), §14 (530–541), §16 to §20 (553–641), the Session 2 section (643–758) and the section "The squash commit message" (760–770); `grep` over the report for the places that describe the committed copy; `git log`, `git status --porcelain`, `git rev-parse HEAD`. §1 to §11, §13, §15 and §21 were not read in session 3 (not asked).

### S3.1 The decisive item: the whole-file `cmp` (R9; MEASURED 2026-11-11)

`cmp /home/kitaeza/zurkhai-prompts/spec-009-henning-differential.md /home/kitaeza/github/zurkhai/docs/specs/prompts/009-henning-differential.md` printed nothing (exit 0) on the committed copy at `4742692`. `wc -c -l`: the prompt file **78,122 bytes, 955 lines**; the copy **78,122 bytes, 955 lines**. Before Item 15 the copy was **41,868 bytes, 508 lines**. The commit of Item 15 shows **447 insertions, 0 deletions** for it (`git diff --cached --numstat`: `447	0	docs/specs/prompts/009-henning-differential.md`; 955 − 508 = 447). The first 41,868 bytes did not change: before the commit, `git show HEAD:docs/specs/prompts/009-henning-differential.md | cmp - <(head -c 41868 <the copy>)` was silent. From `4742692` on, every `cmp` of the copy is the whole-file one (CLAUDE.md hard rule 2).

### S3.2 The rulings, the corrections and the hand-over

**R9** (the human, 2026-11-11; "Agree with your recommendations"): (a), a records-only resume appends ADDENDUM 1 and ADDENDUM 2 to the committed prompt, so that the whole-file `cmp` is silent; it brings the report's diff scope, records and squash message up to date; the merge follows. Done (S3.3, S3.4). The options not taken: the human appending the file on the branch; a later round appending it; leaving it out of the tree. **R10:** session 2's latitude item 11 (the prompt copy compared with `cmp -n 41868`) is accepted as reported, no grant; recorded beside that item in §16.

**The advisor's corrections** (not rulings). **C4:** ADDENDUM 1 did not order session 2 to append itself; the omission is the advisor's, and the tree already decided the question (GATED.md's line on `docs/specs/**`, which allows an appended ADDENDUM; hard rule 2). Session 2's flag (§18, §19) stands as written, with a `[Resolved …]` note. **C5:** the "never copied" statements were three, not two; session 2's report already says so (the opening paragraph's record, S2.3); no change here. **C6:** Remark 14 begins on p. 24 and its passage on a2, (7.24) and the examples is on pp. 25–26; session 2 noted exactly that (S2.2, §18); `DEBT.md` is not changed (it is out of scope, STOP RULE (15)).

**The hand-over.** Session 2 ended at `cf2167e` (Item 13), a complete report, with the branch standing still. Session 3 began there with an empty working tree (S3.3) and nothing uncommitted. The sections of sessions 1 and 2 stand as written except where a `[CORRECTED …]` or `[Resolved …]` record says so.

### S3.3 Item 14 — the resume (MEASURED 2026-11-11)

- **`git status --porcelain`:** empty (no output). **`git rev-parse HEAD`:** `cf2167e2c42a8ccea96a5b4547be15c48d31cb68`, the value ITEM 14 names. STOP RULE (13) did not fire.
- **`git log`:** `cf2167e`, `f63ad2f`, `cb39543`, `f12d565`, `6248886` (the top five of the nine commits the branch then had over `main`'s `07b5bbf`; `git log --oneline` showed the top eight).
- **Session 3's marker** (METHOD rule 58) is chosen. This report never spells it; it is the token of the one placeholder line this session writes into this file (the diff-scope line of S3.9), replaced before the final commit.
- **The OPEN count, re-measured** with the grep `FORKS.md` defines (`grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`): **6** (F4, F5, F7, F8, F10, F11), equal to the relayed 6.
- **The copy and the prompt file:** `wc -c -l` printed `508 41868` for the copy and `955 78122` for the prompt file. `head -c 41868 <the prompt file> | cmp - <the copy>` was silent: the copy is a byte prefix of the prompt file.
- **Date and marker** are as above. The paper library is not read; no ARTIFACT is checked (ADDENDUM 2).

### S3.4 Item 15 — the committed prompt (R9)

The tail of the prompt file after its first 41,868 bytes (lines 509–955: 447 lines, 36,254 bytes) was appended with one Edit (old_string: the copy's last three lines, the original prompt's end banner; new_string: those lines and the 447 lines). Properties of the prompt file, re-measured by `grep` and `tail -c 1 | od -c`: no tab, no carriage return, no trailing space, one final newline (`\n`); no non-breaking space. The non-ASCII characters of the appended tail, counted: `═` 600, `§` 45, `—` 39, `–` 20, `▼` 12, `▲` 12, `…` 5, `→` 4, `−` 2 (the 600 `═` are the 8 rule lines of 75 characters).

**The first `cmp` was not silent**: `differ: byte 70786, line 866` (the sizes were equal, 78,122 bytes, the line counts 954 against 955). I had typed the `PATHS — unchanged from ADDENDUM 1` line of ADDENDUM 2 unbroken; the prompt file breaks it after "the call" (`od -c` on both files at line 866 showed `\n` where my copy had a space). One more Edit restored the break; the second whole-file `cmp` was silent. STOP RULE (14) did not fire: nothing was committed before the `cmp` was silent. **The pre-commit chain** of `4742692` (one `&&`-chain, no `;`): the whole-file `cmp`; the prefix `cmp` above; `git status --porcelain` equal to ` M docs/specs/prompts/009-henning-differential.md` (that file only); `git diff --numstat` of the copy equal to `447`, `0`; the OPEN count equal to 6; the marker grep (`git grep -n -I <marker> -- . ':!docs/specs/prompts'`) with no match. All passed, then `git add` and `git commit`. The commit message states the sizes and counts and carries the `Co-Authored-By` trailer.

### S3.5 Item 16 — what the report changed (MEASURED)

1. The head block: `model:` gains session 3's entry; `gate:` gains "; session 3 ran none: every leg skipped, because no file the gate reads changed after f63ad2f (Session 3)" (METHOD rule 65: the skipped legs are named).
2. This section.
3. Two `[CORRECTED 2026-11-11 …]` records, in S2.5: the diff scope ("Diff scope at the final commit", its numbers line) and "Commits of the branch". Four `[Resolved 2026-11-11 by session 3 (ADDENDUM 2, R9): …]` notes, one at the end of each of session 2's four statements that the copy lacks the addendum or was left unchanged: S2.1 ("The prompt copy in the tree"), §16 item 11, §18 (the session 2 bullet "The committed prompt copy does not hold the addendum") and §19 (session 2's by-catch). §16 item 11 also carries R10 beside it.
4. The squash message: two changes, as Item 16.9 orders: the gate sentence gains "(run on f63ad2f; no file it reads changed after it)"; the paragraph that names the registers ends with "The committed prompt holds the round prompt and its two addenda." The block is still the first and only fenced block under the heading; there is one heading of that name and one `Co-Authored-By` line.
5. `docs/specs/INDEX.md` is not changed (`git diff --stat cf2167e HEAD -- docs/specs/INDEX.md` printed nothing; its row for 009 stands).

### S3.6 Properties, questions, predictions

**Properties** (DONE / DEFERRED / N-A, with evidence):

| Property | Verdict | Evidence |
|---|---|---|
| The committed copy equals the prompt file, whole file | DONE | `cmp` silent; 78,122 bytes and 955 lines each (S3.1) |
| The first 41,868 bytes unchanged | DONE | `git show HEAD:… \| cmp - <(head -c 41868 …)` silent before `4742692` |
| Item 15's commit holds the copy only, insertions only | DONE | `git status --porcelain` ` M <the copy>`; numstat `447 0` |
| Head block: `model:` and `gate:` | DONE | S3.5 item 1 |
| Session 3 section, each contract item present | DONE | this section; "none" where empty |
| Two `[CORRECTED …]` records and four `[Resolved …]` notes | DONE | S3.5 item 3 |
| R10 beside §16 item 11 | DONE | §16 item 11 |
| Open-forks arithmetic, marker greps, diff scope, gate inputs | DONE | S3.8, S3.9 |
| Squash message edited in place, two changes only | DONE | S3.5 item 4 |
| `INDEX.md` unchanged | DONE | S3.5 item 5 |
| Gate run | N-A | none; ADDENDUM 2 forbids it |

**Every prompt question answered.** ADDENDUM 2 puts no question. Its one doubt, whether the committed copy holds the addenda, is decided by R9 and done.

**The predictions, scored** (the measurement is session 3's own, never the prompt's):

- **19. HELD.** HEAD `cf2167e2c42a8ccea96a5b4547be15c48d31cb68`, the status empty, the OPEN count 6 (S3.3).
- **20. HELD.** Before Item 15 the copy was 41,868 bytes and 508 lines (`wc -c -l`), and `head -c 41868 <the prompt file> | cmp - <the copy>` was silent.
- **21. HELD.** After Item 15 the copy is 78,122 bytes and 955 lines, equal to the prompt file, and the commit shows 447 insertions and 0 deletions (S3.1). The first `cmp` was not silent; the prediction is about the state after Item 15, which holds.
- **22.** Scored in S3.9, with its measurement (it needs the final line count of this file).
- **23.** Scored in S3.9, with the two commands' output.

### S3.7 The stop rule, forks, latitude, what the prompt got wrong, by-catch

- **The stop rule that governed session 3:** STOP RULE (13) (the status and HEAD check) and (14) (the whole-file `cmp` made silent with the Edit and Write tools, nothing committed before it). (13) did not fire. (14) did not fire: the first `cmp` differed (S3.4), the fix was one Edit, the second was silent, and nothing was committed before. (15) (out of scope) did not fire: the only files edited are the two named; the first 41,868 bytes of the copy are unchanged; session 1's and 2's records changed only by `[CORRECTED …]` records, bracketed notes and the squash message.
- **Forks encountered:** none. Considered, resolved by a ruling or a rule: whether the committed prompt holds the addenda (R9, GATED.md, hard rule 2). Considered "Write the whole file" against "Edit, appending": chose Edit, because then the first 41,868 bytes cannot change by construction.
- **Latitude taken:** (1) `date +%F` ran a second time, inside a byte-inspection command (ADDENDUM 2: "Run `date` once"); it printed 2026-11-11, and no record uses it. (2) The bracketed notes and `[CORRECTED …]` records are appended at the end of the existing line; I considered a new line below each, and chose the end of the line because the targets are bullets, a numbered item and table-free paragraphs, where a new line would be a separate paragraph or a continuation. (3) The chains used `test` and `printf` (shell built-ins), which GRANTS' list of read-only tools does not name, to compare `git status --porcelain` and `git diff --numstat` with their expected strings; no file was written by them. (4) `od -c`, `sed -n`, `grep -o -P`, `sort`, `uniq -c`, `wc` and `tail -c` are on the GRANTS list; no program outside it ran. (5) Session 2's other statement about the copy, in S2.3's "Item 11's pre-commit measures" (`cmp -n 41868` ... silent), got no `[Resolved …]` note: Item 16 names four places (S2.1, §16 item 11, §18, §19); that statement is a record of what session 2 ran, true as written. (6) The text appended to the copy was typed by hand from the Read tool's display of the prompt file, as Item 15 allows (the Edit tool); the first `cmp` failure (S3.4) is a transcription defect of mine, found by the instrument ADDENDUM 2 names, and fixed.
- **What the prompt got wrong:** (1) Item 15 says to append "ADDENDUM 1 and ADDENDUM 2, each from its opening rule line to its closing rule line, with no line between them and none added", but the prompt file holds one blank line (line 509) between the original prompt's end banner and ADDENDUM 1's opening rule line, and prediction 21's 447 lines count it. The two sentences of Item 15 differ by that one line; the other sentence ("everything in the prompt file after its first 41,868 bytes") and the whole-file `cmp` decide it, and I followed them. (2) The advisor's relayed check, that ADDENDUM 1's Item 11 patterns list no line on the prompt file, was not re-run (ADDENDUM 2 gives session 3 no ARTIFACT and no `awk`); it stays RELAYED. Nothing else found: the sizes (41,868; 508; 78,122; 955), the 447 insertions, HEAD, the status and the OPEN count were all as predicted, and the numstat of the six files was as predicted (S3.9).
- **By-catch:** none found. (The re-run check on the copy by a session with `awk` and the library belongs to whichever round next has an ARTIFACT; not filed: the prompt names no `DEBT.md` entry, hard rule 1.)

### S3.8 The open-forks audit and the marker greps (METHOD rules 58, 60)

| Commit | What it does to forks | Expected | Measured |
|---|---|---|---|
| `4742692` (Item 15: the committed prompt) | nothing | 6 + 0 + 0 − 0 = 6 | 6 (the chain asserted `grep -c` = 6 before `git commit`) |
| the final commit (Item 16: the report only) | nothing | 6 + 0 + 0 − 0 = 6 | 6 (asserted in its chain; `FORKS.md` is unchanged in it) |

**Session 3's marker greps** (`git grep -n -I <session 3's marker> -- . ':!docs/specs/prompts'`, which this report never spells; the chain of each commit asserts "no match" before `git commit`): `4742692`: the chain asserted no match; the tree has none. The final commit: the diff-scope line of S3.9 is written with the marker as a placeholder while the diff is measured, then replaced by the measured line, and the chain asserts no match. Sessions 1's and 2's markers are not spelled in the report and session 3 does not know them, so session 3 did not re-run their greps; the rows in §12 are theirs.

### S3.9 Diff scope at the final commit, and the gate inputs

`git diff --cached --numstat main` (insertions, deletions, file), measured on the staged tree of the final commit, which carries this line (its parent is `4742692`; the commit's own hash is in `git log`): 40 0 DEBT.md; 14 0 FORKS.md; 2 1 docs/sources.md; 2450 0 docs/specs/009-henning-differential.md; 1 0 docs/specs/INDEX.md; 955 0 docs/specs/prompts/009-henning-differential.md; TOTAL (6 files) 3462 insertions = 40 + 14 + 2 + 2450 + 1 + 955, 1 deletion = 0 + 0 + 1 + 0 + 0 + 0 (the deletion is the `henning-kalacakra/` row of `docs/sources.md`, which gained a sentence). `git diff --shortstat main`: 6 files changed, 3462 insertions(+), 1 deletion(-). Against session 2's scope (`cf2167e`: 2,908 insertions, 1 deletion): the prompt copy 508 → 955 lines (+447) and this report 2,343 → 2,450 lines (+107); 2,908 + 447 + 107 = 3,462.

- **Gated paths touched: no.** Nothing under `crates/`, `golden/`, `rules/`, `clients/`, `tools/`, `Cargo.toml`, `Cargo.lock`, `docs/METHOD.md`, `GATED.md` or an accepted spec changed. The round's own `docs/specs/009-henning-differential.md` and `docs/specs/prompts/009-henning-differential.md` are not changes to an accepted spec (GATED.md allows a round's committed prompt to gain an appended ADDENDUM). `docs/specs/INDEX.md`, `DEBT.md`, `FORKS.md` and `docs/sources.md` are unchanged by session 3.
- **Files the test framework wrote:** none (no cargo ran; `git status --porcelain` was empty at the start of the session and after Item 15's commit).
- **Commits of the branch** (11, the three sessions): `e48feb5`, `328febb`, `2739375`, `c3a52bd`, `6248886` (session 1); `f12d565`, `cb39543`, `f63ad2f`, `cf2167e` (session 2); `4742692` (Item 15) and the final commit (Item 16) (session 3); 5 + 4 + 2 = 11.

**That the final commit changes nothing the gate reads.** The gate ran on `f63ad2f` (S2.5). Both commands were run in the final commit's chain, the second on the staged final commit, and both printed nothing:

```
git -C /home/kitaeza/github/zurkhai diff --stat f63ad2f HEAD -- crates golden tools Cargo.toml Cargo.lock rust-toolchain.toml deny.toml
git -C /home/kitaeza/github/zurkhai diff --cached --stat -- crates golden tools Cargo.toml Cargo.lock rust-toolchain.toml deny.toml
```

Output of the first: (empty). Output of the second: (empty). So the gate's result on `f63ad2f` (147 passed, 0 failed in debug, 147 passed, 0 failed in release, S2.5) holds for the branch tip: `git diff --stat f63ad2f HEAD` for the paths the gate reads is empty, and the final commit changes only this report.

**Predictions 22 and 23.** **22. HELD.** The numstat lists the six files it lists at `cf2167e`; DEBT.md 40 0, FORKS.md 14 0, `docs/sources.md` 2 1 and `docs/specs/INDEX.md` 1 0 are the counts they have there; the copy has 955 0; the report has 2,450 insertions (more than 2,343) and 0 deletions. **23. HELD.** Both commands printed nothing.

## The squash commit message

```
spec 009: the Henning differential: TCG 1.06 against the engine's three systems, 1900-2100

Henning's TCG 1.06 (tcg1309.zip, from kalacakra.org), built with gcc from its published source with header stand-ins only, at -O0 and -O2 with byte-identical output, and checked against Henning's own six published Phugpa years (1900, 1950, 2000, 2012, 2025, 2049): 8,940 of 8,940 lines equal. Run for Phugpa, Tsurphu and Genden over every day of 1900-01-01..2100-12-31 (73,414 days each) and compared with the engine day by day: 17, 13 and 23 disagreeing days (53 in all), each a single day beside a doubled or skipped tshes; the sets of months and the leap months agree in all 202 Tibetan years. One cause, removed by one-change probes on a copy of TCG and on a copy of the engine, and reproduced by a script written from Janson v1 alone: the Moon's anomaly increment per lunar day, a2 = 1/28 in the engine (Janson v1 eq. (7.13)) and 3781/105840 in TCG (eq. (7.24), Remark 14). Nothing under crates/ or golden/ changed, and no constant was changed on either side. The gate on the tip (run on f63ad2f; no file it reads changed after it): 147 passed, 0 failed in debug and in release.

F24 (what TCG's output becomes in the tree) and F25 (which a2 each system computes) were raised and RULED (a): TCG's output stays in the report, with no tier, no golden row and no tool; the engine keeps a2 = 1/28 for all three systems. DEBT.md: D-20, D-21 and D-22 (relayed from the reviewer of round 008) and D-23 (the 53 days on which (7.24) gives other dates) and D-24 (Tsurphu and Genden have no full-range oracle in the tree; the differential is re-run from the report's section 21) filed. docs/sources.md: a row for the henning-tcg/ library holding and one sentence in the henning-kalacakra/ row. The report carries no code of Henning's and no diff of the engine. The committed prompt holds the round prompt and its two addenda.

Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>
```

## 21. The commands, the stand-ins, the program's input, the probes and the scripts, verbatim

[CORRECTED 2026-11-11 (spec 009 session 2, Item 11): the sentence below says Henning's source is never copied here. At the halt commit `6248886` this section copied it in two places: the P-TCG block of §21.3, a diff whose two lines were the changed line of `t2.c` before and after, and one print string of `epochs.py` in §21.5, which copied text from two places of `t2.c`. Both are now descriptions with file:line, and so is the P-ENG block, which was a diff of the engine's `phugpa.rs` (docs/ADVISOR.md §6; ADDENDUM 1, C3). The rest of the section is this round's own code (the stand-in headers, the dump probe, the scripts) and numbers cited with file:line.]

(Henning's source is cited by file:line above and never copied here. Paths below are the session's scratchpad directories; `$S` is the scratchpad, `$X` the extraction of `tcg1309.zip`, `$W` the work directory with `inc/`, `o0/`, `o2/` and the runs, `$SC` the scripts directory, `$E` the engine copy. Each block below was extracted from this file and compared with the scratch file it was typed from, `cmp` silent, before the commit.)

### 21.1 The build and the runs (shell, as run)

```
# extraction (a mktemp -d directory), and the stand-ins in $W/inc/ (below)
unzip -q /home/kitaeza/zurkhai-papers/henning-tcg/tcg1309.zip -d $X
# build, once per optimisation level (0 and 2); the original's flags are in its makefile line 2
for opt in 0 2; do
  for f in t1 t2 t3 t4 utils tests bcd; do
    gcc -std=gnu89 -O$opt -Wall -Wextra -I $W/inc -c $X/$f.c -o $W/o$opt/$f.o 2> $W/o$opt/$f.warn
  done
  gcc -O$opt -o $W/o$opt/tcg $W/o$opt/t1.o $W/o$opt/t2.o $W/o$opt/t3.o $W/o$opt/t4.o \
      $W/o$opt/utils.o $W/o$opt/tests.o $W/o$opt/bcd.o -lm
done
# the program is run in a directory of its own, with its input from a file
env --chdir=$W/<run dir> $W/o<opt>/tcg < $W/<run dir>/in.txt > .../stdout.txt 2> .../stderr.txt
# the build check: the input (one process per build)
printf '1\n2\n1900\n1900\n2\n1950\n1950\n2\n2000\n2000\n2\n2012\n2012\n2\n2025\n2025\n2\n2049\n2049\n9\n'
python3 -I $SC/cmpbuild.py real $W/bc-o<opt> /home/kitaeza/zurkhai-papers/henning-kalacakra/tdata 1900 1950 2000 2012 2025 2049
python3 -I $SC/cmpbuild.py controls /home/kitaeza/zurkhai-papers/henning-kalacakra/tdata
# the full ranges: Phugpa and Tsurphu, one process each, the input
printf '1\n2\n1899\n2100\n9\n'      # Phugpa   (key 1 = epoch 1)
printf '2\n2\n1899\n2100\n9\n'      # Tsurphu  (key 2 = epoch 2)
# Genden, one process per year Y in 1899..2100 (a Phugpa print first, then epoch 5; see section 4b)
printf '1\n2\n%s\n%s\n1\n5\n2\n%s\n%s\n9\n' $Y $Y $Y $Y
# the files are pl_<Y>.txt (Phugpa and Genden) and ts_<Y>.txt (Tsurphu)
python3 -I $SC/readtcg.py <dir> <pl|ts> <rows.csv> <omitted.txt> 1899 2100
python3 -I $SC/checkrows.py <rows.csv>
python3 -I $SC/omitcheck.py <rows.csv> <omitted.txt>
python3 -I $SC/readcheck.py <phugpa|tsurphu|genden> <rows.csv> <dir> <pl|ts> /home/kitaeza/github/zurkhai/golden/reference
python3 -I $SC/readcheck.py control <version> <rows.csv> <dir> <pl|ts> /home/kitaeza/github/zurkhai/golden/reference
# the engine side: a git archive copy, the probe test added to the copy only, debug and release
git -C /home/kitaeza/github/zurkhai archive HEAD | tar -x -C $E
ZURKHAI_009_DUMP_DIR=<debug dir>   CARGO_TARGET_DIR=<dir>/debug   cargo +1.99.0 test --offline --manifest-path $E/Cargo.toml -p tshes --test zz_spec009_dump
ZURKHAI_009_DUMP_DIR=<release dir> CARGO_TARGET_DIR=<dir>/release cargo +1.99.0 test --offline --release --manifest-path $E/Cargo.toml -p tshes --test zz_spec009_dump
cmp <debug dir>/engine-<v>.csv <release dir>/engine-<v>.csv
cmp <debug dir>/engine-phugpa.csv /home/kitaeza/github/zurkhai/golden/reference/tdc-97d4a00-phugpa-1900-2100.csv
# the differential
python3 -I $SC/diffrows.py diff <engine dump> <tcg rows> engine tcg <list file>
python3 -I $SC/diffrows.py control <engine dump or fixture> <tcg rows or dump>
# the reference script
python3 -I $SC/janson_ref.py selfcheck
python3 -I $SC/janson_ref.py table8 /home/kitaeza/github/zurkhai/golden/reference [--seed v:const:delta] [--a2 exact]
python3 -I $SC/janson_ref.py days $SC/thirty-days.txt <debug dump dir> [--seed v:const:delta]
python3 -I $SC/janson_ref.py full <debug dump dir> [--a2 exact]
python3 -I $SC/janson_ref.py vsrows <version> <tcg rows.csv> [--a2 exact]
python3 -I $SC/janson_ref.py settle <version> <list file> <tcg rows.csv> <engine dump.csv>
python3 -I $SC/janson_ref.py monthjump phugpa 15999
python3 -I $SC/epochs.py /home/kitaeza/github/zurkhai/golden/reference
python3 -I $SC/probecmp.py <label> <engine dump> <tcg original rows> <tcg probe rows>
python3 -I $SC/mdtable.py <version> <list file> <settle output>
```

### 21.2 The stand-in headers (this round's own code)

`conio.h`:

```
/* Stand-in for Open Watcom's <conio.h>, written for spec 009 (not Henning's code).
   TCG uses one function from it: getch(), one key from the console without echo.
   Here it reads one byte from standard input. At end of input it stops the program
   with exit status 3 and a message, so that a loop waiting for a key cannot spin. */
#ifndef ZURKHAI_009_STANDIN_CONIO_H
#define ZURKHAI_009_STANDIN_CONIO_H

#include <stdio.h>
#include <stdlib.h>

static inline int getch(void)
{
    int c = getchar();
    if (c == EOF) {
        fputs("stand-in getch(): end of input\n", stderr);
        exit(3);
    }
    return c;
}

#endif
```

`dos.h`:

```
/* Stand-in for Open Watcom's <dos.h>, written for spec 009 (not Henning's code).
   TCG includes it (utils.c:30) and, as far as the build shows, uses nothing from it. Empty on purpose. */
#ifndef ZURKHAI_009_STANDIN_DOS_H
#define ZURKHAI_009_STANDIN_DOS_H
#endif
```

`bios.h`:

```
/* Stand-in for Open Watcom's <bios.h>, written for spec 009 (not Henning's code).
   TCG includes it (utils.c:31) and, as far as the build shows, uses nothing from it. Empty on purpose. */
#ifndef ZURKHAI_009_STANDIN_BIOS_H
#define ZURKHAI_009_STANDIN_BIOS_H
#endif
```

### 21.3 The probes

**P-TCG** (a fresh extraction in its own `mktemp -d` directory, one line changed with the Edit tool; `diff` of the two `t2.c` after removing carriage returns). The change, described: at `t2.c:637`, in the call that converts the anomaly's 105840ths product to a big decimal, the first factor was the lunar day `tt` added to thirty times the second anomaly digit `rilcha[1]`; the probe left thirty times that digit alone, so the lunar day no longer enters that product a second time (it keeps its entry into the 28ths at `t2.c:623`). The `diff` printed the one hunk `637c637` and nothing else. Henning's source is not reproduced here (CITATIONS; CLAUDE.md hard rule 17): a later session re-makes the probe from this description and `t2.c:637`.

**P-ENG** (a second `git archive` copy of the tree; `diff` of `crates/tshes/src/phugpa.rs` against the unchanged copy). The change, described, with the hunks the `diff` printed (`421,426c421,426`, `446c446,448`, `452c454`): (1) `phugpa.rs:421–426`: the Moon's anomaly argument computed with a2 = 3781/105840, Janson v1 (7.24), p. 25, in place of 1/28, as a numerator over 105840 (7590 times n, plus 3781 times d, plus 30 times the epoch's a0), its integer part and remainder taken in units of 1/3780, which is 30 times finer than the table's unit of 1/126, and the linear interpolation between table entries scaled to match; (2) `phugpa.rs:446`: the mean-date and Sun terms of the true-date numerator multiplied by 30, the Moon term unchanged, because the Moon term is now in the finer units; (3) `phugpa.rs:452`: the denominator of `end_day`'s division multiplied by 30. Every other term is unchanged. The engine's implementation is not reproduced here (`docs/ADVISOR.md` §6: a report never carries quoted implementation); §1.2's probe table gives the same description.

**The engine dump probe** `crates/tshes/tests/zz_spec009_dump.rs` (added to the copies only):

```rust
//! Spec 009 Item 5 probe (a copy of the tree only; never in the working tree).
//!
//! Writes one row per day of 1900-01-01..=2100-12-31 for each of the three systems, through the public API
//! only (`to_zdate` with the system named explicitly, DEBT.md D-1), in the T4 fixture's five columns:
//! gregorian_date,tibetan_year,month,is_leap_month,tshes
//! The output directory is the environment variable ZURKHAI_009_DUMP_DIR (a `mktemp -d` directory).

use std::fmt::Write as _;
use std::fs;
use tshes::{GregorianDate, Jdn, System, to_zdate};

fn dump(system: System) -> String {
    let first = GregorianDate::new(1900, 1, 1).expect("date").to_jdn().get();
    let last = GregorianDate::new(2100, 12, 31).expect("date").to_jdn().get();
    let mut out = String::from("gregorian_date,tibetan_year,month,is_leap_month,tshes\n");
    for jdn in first..=last {
        let date = Jdn::new(jdn).expect("jdn").to_gregorian().expect("date");
        let z = to_zdate(date, system).expect("every supported day maps");
        writeln!(
            out,
            "{:04}-{:02}-{:02},{},{},{},{}",
            date.year(),
            date.month(),
            date.day(),
            z.year().number(),
            z.month().get(),
            z.is_leap_month(),
            z.tshes().get()
        )
        .expect("write to a String");
    }
    out
}

#[test]
fn write_the_three_dumps() {
    let dir = std::env::var("ZURKHAI_009_DUMP_DIR").expect("ZURKHAI_009_DUMP_DIR");
    for (name, system) in [
        ("phugpa", System::Phugpa),
        ("tsurphu", System::Tsurphu),
        ("genden", System::Genden),
    ] {
        let text = dump(system);
        assert_eq!(text.lines().count(), 73_415, "{name}: header + 73,414 rows");
        fs::write(format!("{dir}/engine-{name}.csv"), text).expect("write");
    }
}
```

### 21.4 The 30 days listed before the reference script was run (`thirty-days.txt`)

```
# Spec 009 Item 7 (i): the 30 days listed BEFORE the reference script was run against any engine dump.
# version date    kind (D = second day of a repeated number, S = day after a skipped number, L = a day of a leap month, O = ordinary)
phugpa 1936-11-15 D
phugpa 2023-01-11 D
phugpa 1935-10-14 S
phugpa 2007-06-18 S
phugpa 2079-05-06 S
phugpa 1905-05-14 L
phugpa 1986-08-28 L
phugpa 2070-07-16 L
phugpa 2012-02-22 O
phugpa 1999-07-04 O
tsurphu 1937-01-04 D
tsurphu 2023-11-09 D
tsurphu 1935-11-29 S
tsurphu 2007-11-21 S
tsurphu 2079-12-14 S
tsurphu 1903-03-10 L
tsurphu 1984-06-23 L
tsurphu 2068-05-10 L
tsurphu 2012-07-04 O
tsurphu 1960-03-15 O
genden 1937-01-04 D
genden 2023-03-02 D
genden 1935-11-29 S
genden 2007-09-24 S
genden 2079-08-24 S
genden 1903-03-10 L
genden 1984-06-23 L
genden 2068-05-10 L
genden 2012-07-04 O
genden 1960-03-15 O
```

### 21.5 The scripts

#### cmpbuild.py

```python
"""Spec 009 Item 2: compare a built TCG's "Print calendar" files with Henning's published pl_<year>.txt.

Normalisation: line endings (CR removed) and trailing spaces/tabs only.
Each differing line is classed as a CALENDAR line (a difference in the year, the month heading, the tshes
number or the western date) or an OTHER line (any other field), or "missing"/"extra" (a line that exists on one
side only, classed by what kind of line it is).
Usage: python3 -I cmpbuild.py real <ours_dir> <theirs_dir> year...     (real comparison)
       python3 -I cmpbuild.py controls <theirs_dir>                    (seeded controls, METHOD rules 31, 36)
"""
import difflib
import re
import sys

RE_DAY = re.compile(r"^(\d+): (\w+)\. (.+?)\. ([A-Za-z]+)-([A-Za-z]+); (\d+) (\w+) (\d+)$")
RE_OMIT = re.compile(r"^(\d+)\. Omitted: (.*)$")
RE_NEWYEAR = re.compile(r"^New Year: .*$")
RE_MONTH = re.compile(r"^Tibetan Lunar Month: .*$")


def norm(path):
    with open(path, "rb") as f:
        data = f.read().decode("latin-1")
    lines = [ln.rstrip(" \t") for ln in data.replace("\r", "").split("\n")]
    if lines and lines[-1] == "":
        lines.pop()  # the empty element after the final newline is not a line
    return lines


def kind(line):
    """The kind of a line: which calendar fields it carries."""
    if RE_DAY.match(line) or RE_OMIT.match(line):
        return "day-line (tshes, western date)"
    if RE_NEWYEAR.match(line):
        return "New Year line (year)"
    if RE_MONTH.match(line):
        return "month heading"
    return "other"


def classify_pair(a, b):
    """Class of a replaced line pair: 'calendar' or 'other', with the fields that differ."""
    ma, mb = RE_DAY.match(a), RE_DAY.match(b)
    if ma and mb:
        names = ["tshes", "weekday", "mansion", "elem1", "elem2", "wday", "wmonth", "wyear"]
        diff = [n for n, x, y in zip(names, ma.groups(), mb.groups()) if x != y]
        cal = [n for n in diff if n in ("tshes", "wday", "wmonth", "wyear")]
        return ("calendar" if cal else "other"), diff
    oa, ob = RE_OMIT.match(a), RE_OMIT.match(b)
    if oa and ob:
        diff = [n for n, x, y in zip(["tshes", "rest"], oa.groups(), ob.groups()) if x != y]
        return ("calendar" if "tshes" in diff else "other"), diff
    ka, kb = kind(a), kind(b)
    if ka in ("New Year line (year)", "month heading") or kb in ("New Year line (year)", "month heading"):
        return "calendar", [ka if ka != "other" else kb]
    if ka != kb:
        # one side is a calendar-kind line and the other not: a calendar field differs
        return "calendar", [ka + " vs " + kb]
    return "other", ["text"]


def compare(ours, theirs):
    """Return (n_ours, n_theirs, n_equal, diffs). diffs: list of dicts."""
    sm = difflib.SequenceMatcher(None, theirs, ours, autojunk=False)
    diffs = []
    equal = 0
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal":
            equal += i2 - i1
        elif tag == "replace" and (i2 - i1) == (j2 - j1):
            for k in range(i2 - i1):
                cls, fields = classify_pair(ours[j1 + k], theirs[i1 + k])
                diffs.append(dict(op="differs", theirs_line=i1 + k + 1, ours_line=j1 + k + 1,
                                  ours=ours[j1 + k], theirs=theirs[i1 + k], cls=cls, fields=fields))
        else:
            for k in range(i1, i2):
                kd = kind(theirs[k])
                diffs.append(dict(op="missing in ours", theirs_line=k + 1, ours_line=None, ours=None,
                                  theirs=theirs[k], cls="calendar" if kd != "other" else "other", fields=[kd]))
            for k in range(j1, j2):
                kd = kind(ours[k])
                diffs.append(dict(op="extra in ours", theirs_line=None, ours_line=k + 1, ours=ours[k],
                                  theirs=None, cls="calendar" if kd != "other" else "other", fields=[kd]))
    return len(ours), len(theirs), equal, diffs


def report(label, ours, theirs):
    no, nt, eq, diffs = compare(ours, theirs)
    print(f"{label}: lines ours {no}, published {nt}, equal {eq}, differing {len(diffs)} "
          f"(calendar {sum(d['cls'] == 'calendar' for d in diffs)}, other {sum(d['cls'] == 'other' for d in diffs)})")
    for d in diffs:
        print(f"   [{d['cls']}] {d['op']}: published line {d['theirs_line']} {d['theirs']!r}; "
              f"ours line {d['ours_line']} {d['ours']!r}; fields {d['fields']}")
    return no, nt, eq, diffs


def main():
    mode = sys.argv[1]
    if mode == "real":
        ours_dir, theirs_dir = sys.argv[2], sys.argv[3]
        tot = dict(lines=0, equal=0, diffs=0)
        for y in sys.argv[4:]:
            no, nt, eq, diffs = report(f"pl_{y}.txt",
                                       norm(f"{ours_dir}/pl_{y}.txt"), norm(f"{theirs_dir}/pl_{y}.txt"))
            tot["lines"] += nt
            tot["equal"] += eq
            tot["diffs"] += len(diffs)
        print(f"TOTAL published lines {tot['lines']}, equal {tot['equal']}, differing {tot['diffs']}")
    elif mode == "controls":
        theirs_dir = sys.argv[2]
        base = norm(f"{theirs_dir}/pl_2012.txt")
        # control 1: one tshes and one western date changed
        i_t = next(i for i, ln in enumerate(base) if ln == "5: Mon. tha skar. Water-Wind; 27 Feb 2012")
        i_d = next(i for i, ln in enumerate(base) if ln == "7: Wed. bra nye. Water-Fire; 29 Feb 2012")
        c1 = list(base)
        c1[i_t] = "6: Mon. tha skar. Water-Wind; 27 Feb 2012"
        c1[i_d] = "7: Wed. bra nye. Water-Fire; 30 Feb 2012"
        print("CONTROL 1 (one tshes, one western date changed; expected exactly 2 differences, both calendar):")
        _, _, _, d1 = report("control1", c1, base)
        ok1 = len(d1) == 2 and all(d["cls"] == "calendar" for d in d1) and \
            [d["fields"] for d in d1] == [["tshes"], ["wday"]]
        print("  control 1 verdict:", "AS EXPECTED" if ok1 else "NOT AS EXPECTED")
        # control 2: one line removed (a non-calendar line)
        i_s = next(i for i, ln in enumerate(base) if ln == "  Solar: Wood-Tiger. Jiao 6")
        c2 = base[:i_s] + base[i_s + 1:]
        print("CONTROL 2 (one non-calendar line removed; expected exactly 1 difference: that line missing, class other):")
        _, _, _, d2 = report("control2", c2, base)
        ok2 = len(d2) == 1 and d2[0]["op"] == "missing in ours" and d2[0]["theirs"] == "  Solar: Wood-Tiger. Jiao 6" \
            and d2[0]["cls"] == "other" and d2[0]["theirs_line"] == i_s + 1
        print("  control 2 verdict:", "AS EXPECTED" if ok2 else "NOT AS EXPECTED")
        # control 3: one calendar line removed
        c3 = base[:i_t] + base[i_t + 1:]
        print("CONTROL 3 (one day-line removed; expected exactly 1 difference: that line missing, class calendar):")
        _, _, _, d3 = report("control3", c3, base)
        ok3 = len(d3) == 1 and d3[0]["op"] == "missing in ours" and d3[0]["cls"] == "calendar" \
            and d3[0]["theirs_line"] == i_t + 1
        print("  control 3 verdict:", "AS EXPECTED" if ok3 else "NOT AS EXPECTED")
        # control 4: the unchanged copy gives zero differences
        print("CONTROL 4 (unchanged copy; expected 0 differences):")
        _, _, _, d4 = report("control4", list(base), base)
        print("  control 4 verdict:", "AS EXPECTED" if len(d4) == 0 else "NOT AS EXPECTED")
        sys.exit(0 if (ok1 and ok2 and ok3 and len(d4) == 0) else 1)


if __name__ == "__main__":
    main()
```

#### readtcg.py

```python
"""Spec 009 Item 4: read TCG's "Print calendar" files into one row per Gregorian day.

Usage: python3 -I readtcg.py <dir> <prefix: pl|ts> <out.csv> <omitted_out.txt> <first_year> <last_year>

Columns, as the T4 fixture's: gregorian_date,tibetan_year,month,is_leap_month,tshes
The mapping (evidence in the report):
  tibetan_year   the year of the file (the number prn_cal asks for; New Year: <year> line)
  month          N of the heading "Tibetan Lunar Month: N ..."
  is_leap_month  true when the heading says "(Intercalary)"; "(Delayed)" is the regular month, false
  tshes          T of the day line "T: Dow. mansion. e-e; D Mon YYYY"
  gregorian_date D Mon YYYY of the same line
"Omitted" lines ("T. Omitted: ...") are not days; they are written to <omitted_out.txt> as
year,month,is_leap_month,tshes (a cross-check of the skipped numbers the rows imply).
Only rows with 1900-01-01 <= date <= 2100-12-31 are written.
"""
import re
import sys

MON = {m: i + 1 for i, m in enumerate("Jan Feb Mar Apr May Jun Jul Aug Sep Oct Nov Dec".split())}
RE_DAY = re.compile(r"^(\d+): (\w+)\. (.+?)\. ([A-Za-z]+)-([A-Za-z]+); (\d+) (\w+) (\d+)$")
RE_OMIT = re.compile(r"^(\d+)\. Omitted: (.*)$")
RE_MONTH = re.compile(r"^Tibetan Lunar Month: (\d+)( \((Intercalary|Delayed)\))? - (.*)$")
RE_NY = re.compile(r"^New Year: (\d+), (.*)$")
LO, HI = "1900-01-01", "2100-12-31"


def lines_of(path):
    with open(path, "rb") as f:
        data = f.read().decode("latin-1")
    return [ln.rstrip(" \t") for ln in data.replace("\r", "").split("\n")]


def main():
    d, prefix, out_csv, out_omit, y0, y1 = sys.argv[1:7]
    rows, omitted = [], []
    stats = dict(newyear=0, headings=0, daylines=0, omitlines=0, unknown=0)
    for y in range(int(y0), int(y1) + 1):
        cur = None
        ny_years = []
        for ln in lines_of(f"{d}/{prefix}_{y}.txt"):
            if ln == "":
                continue
            m = RE_NY.match(ln)
            if m:
                stats["newyear"] += 1
                ny_years.append(int(m.group(1)))
                continue
            m = RE_MONTH.match(ln)
            if m:
                stats["headings"] += 1
                cur = (int(m.group(1)), m.group(3) == "Intercalary")
                continue
            m = RE_OMIT.match(ln)
            if m:
                stats["omitlines"] += 1
                omitted.append((y, cur[0], cur[1], int(m.group(1))))
                continue
            m = RE_DAY.match(ln)
            if m:
                stats["daylines"] += 1
                t, _dow, _mans, _e1, _e2, dd, mon, yy = m.groups()
                date = f"{int(yy):04d}-{MON[mon]:02d}-{int(dd):02d}"
                rows.append((date, y, cur[0], cur[1], int(t)))
                continue
            # lines that are not calendar-field lines: yoga/karana, longitudes, Solar:, special-day notes
            stats["unknown"] += 1
        assert ny_years == [y], f"{prefix}_{y}.txt: New Year lines {ny_years}"
    kept = [r for r in rows if LO <= r[0] <= HI]
    with open(out_csv, "w", newline="") as f:
        f.write("gregorian_date,tibetan_year,month,is_leap_month,tshes\n")
        for date, y, mo, lp, t in kept:
            f.write(f"{date},{y},{mo},{'true' if lp else 'false'},{t}\n")
    with open(out_omit, "w") as f:
        for y, mo, lp, t in omitted:
            f.write(f"{y},{mo},{'true' if lp else 'false'},{t}\n")
    print(f"read {len(rows)} day lines, kept {len(kept)} in {LO}..{HI}; omitted lines {len(omitted)}; stats {stats}")


if __name__ == "__main__":
    main()
```

#### checkrows.py

```python
"""Spec 009 Item 4: the checks on one version's rows (TCG's read output or an engine dump).

Usage: python3 -I checkrows.py <rows.csv>
Rows: gregorian_date,tibetan_year,month,is_leap_month,tshes (one per Gregorian day).
Prints each check with its measured value and PASS/FAIL, and the identity with its arithmetic.
"""
import csv
import datetime
import sys
from collections import Counter, OrderedDict


def load(path):
    with open(path, newline="") as f:
        r = csv.reader(f)
        hdr = next(r)
        assert hdr == ["gregorian_date", "tibetan_year", "month", "is_leap_month", "tshes"], hdr
        return [(a, int(b), int(c), d, int(e)) for a, b, c, d, e in r]


def groups(rows):
    """Month groups in order: key (year, month, leap) -> list of tshes, keeping order; consecutive rows."""
    out = []
    for date, y, m, lp, t in rows:
        key = (y, m, lp)
        if out and out[-1][0] == key:
            out[-1][1].append(t)
        else:
            out.append((key, [t]))
    return out


def main():
    rows = load(sys.argv[1])
    ok = True

    def check(name, value, expected_ok, text=""):
        nonlocal ok
        ok &= expected_ok
        print(f"  {'PASS' if expected_ok else 'FAIL'}  {name}: {value} {text}")

    d0 = datetime.date(1900, 1, 1)
    check("rows", len(rows), len(rows) == 73414, "(expected 73414)")
    dates = [datetime.date.fromisoformat(r[0]) for r in rows]
    check("first / last date", f"{dates[0]} / {dates[-1]}", dates[0] == d0 and dates[-1] == datetime.date(2100, 12, 31))
    dup = len(dates) - len(set(dates))
    check("duplicate dates", dup, dup == 0)
    gaps = sum(1 for a, b in zip(dates, dates[1:]) if (b - a).days != 1)
    check("gaps (consecutive dates not 1 day apart)", gaps, gaps == 0)
    steps = Counter(((b[4] - a[4]) % 30) for a, b in zip(rows, rows[1:]))
    check("tshes step counts (mod 30) between consecutive days", dict(sorted(steps.items())),
          set(steps) <= {0, 1, 2})
    g = groups(rows)
    # a month group must be a single run: the same key must not re-occur later
    keys = [k for k, _ in g]
    check("month keys occurring in more than one run", len(keys) - len(set(keys)), len(keys) == len(set(keys)))
    complete = g[1:-1]
    lens = Counter(len(t) for _, t in complete)
    check("months (all / complete)", f"{len(g)} / {len(complete)}", True)
    check("complete months by length in days", dict(sorted(lens.items())), set(lens) <= {29, 30})
    leap = sum(1 for (y, m, lp), _ in g if lp == "true")
    leap_c = sum(1 for (y, m, lp), _ in complete if lp == "true")
    check("leap months (all / complete)", f"{leap} / {leap_c}", True)
    # skipped and doubled numbers inside complete months, by counting days per number
    skipped = doubled = bad = 0
    for _, ts in complete:
        c = Counter(ts)
        for n in range(1, 31):
            k = c.get(n, 0)
            if k == 0:
                skipped += 1
            elif k == 2:
                doubled += 1
            elif k != 1:
                bad += 1
        if any(n < 1 or n > 30 for n in c):
            bad += 1
    check("numbers present 3+ times, or outside 1..30, in complete months", bad, bad == 0)
    days_in_complete = sum(len(t) for _, t in complete)
    lhs = skipped - doubled
    rhs = 30 * len(complete) - days_in_complete
    check("identity skipped - doubled = 30 x (complete months) - (days in them)",
          f"{skipped} - {doubled} = {lhs}; 30 x {len(complete)} - {days_in_complete} = {rhs}", lhs == rhs)
    years = OrderedDict()
    for (y, m, lp), _ in g:
        years.setdefault(y, 0)
        years[y] += 1
    check("Tibetan years (distinct)", f"{len(years)} ({min(years)}..{max(years)})", len(years) == 202)
    print("ALL PASS" if ok else "SOME FAIL")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
```

#### omitcheck.py

```python
"""Spec 009 Item 4: cross-check TCG's own "Omitted" lines against the skipped numbers its rows imply.

Usage: python3 -I omitcheck.py <rows.csv> <omitted.txt>
For every complete month of the rows (not the first or last month of the range), the set of tshes 1..30 with no row must
equal the set of "Omitted" lines TCG printed for that month key (year, month, leap).
"""
import csv
import sys
from collections import Counter, defaultdict


def main():
    rows_path, omit_path = sys.argv[1:3]
    groups = []
    with open(rows_path, newline="") as f:
        r = csv.reader(f)
        next(r)
        for date, y, m, lp, t in r:
            key = (int(y), int(m), lp == "true")
            if groups and groups[-1][0] == key:
                groups[-1][1].append(int(t))
            else:
                groups.append((key, [int(t)]))
    omit = defaultdict(set)
    n_omit = 0
    with open(omit_path) as f:
        for ln in f:
            y, m, lp, t = ln.strip().split(",")
            omit[(int(y), int(m), lp == "true")].add(int(t))
            n_omit += 1
    complete = groups[1:-1]
    bad = []
    implied = 0
    for key, ts in complete:
        absent = {n for n in range(1, 31) if n not in set(ts)}
        implied += len(absent)
        if absent != omit.get(key, set()):
            bad.append((key, sorted(absent), sorted(omit.get(key, set()))))
    in_complete = sum(len(omit.get(k, set())) for k, _ in complete)
    print(f"  complete months {len(complete)}: numbers absent from the rows {implied}; 'Omitted' lines for those months {in_complete}; "
          f"months where the two sets differ: {len(bad)} {bad[:5]}; 'Omitted' lines in the files in total {n_omit} (the rest are in the two edge months and in days outside 1900-2100)")


if __name__ == "__main__":
    main()
```

#### readcheck.py

```python
"""Spec 009 Item 4: THE READING CHECK. TCG's read rows against the paper's printed tables (the T5 files).

Usage: python3 -I readcheck.py <version: phugpa|tsurphu|genden> <rows.csv> <tcg_dir> <prefix pl|ts> <golden/reference dir>
       python3 -I readcheck.py control <version> <rows.csv> <tcg_dir> <prefix> <golden dir>   (seeded controls)

Per version: Table 7 (leap month per year, 2000-2020), Table 8 (repeated and skipped numbers, Tibetan year 2012),
Table 9 (first day of the year, 2000-2030); for Phugpa also Table 1 (first day and name, 1927-2046).
The first day of Tibetan year Y is the first row whose tibetan_year is Y (the year begins with its month 1, or
with a leap month 1: Janson v1 p. 9; "1 + JD of the last day of month 12 of the preceding year", p. 27: day 2
when day 1 is skipped, which is what the first row is).
"""
import csv
import re
import sys
from collections import Counter

RE_NY = re.compile(r"^New Year: (\d+), (\w+)-(\w+)-(\w+)$")


def load_rows(path):
    with open(path, newline="") as f:
        r = csv.reader(f)
        next(r)
        return [(a, int(b), int(c), d == "true", int(e)) for a, b, c, d, e in r]


def t5(golden, name):
    with open(f"{golden}/{name}", newline="") as f:
        r = csv.DictReader(f)
        return list(r)


def first_days(rows):
    fd = {}
    for date, y, m, lp, t in rows:
        fd.setdefault(y, date)
    return fd


def leap_months(rows):
    lm = {}
    for date, y, m, lp, t in rows:
        if lp:
            lm.setdefault(y, set()).add(m)
    return lm


def t8_events(rows, year):
    """Repeated and skipped numbers per month of `year`, by counting the days per number in each month run.
    Months of the year that are not complete inside the rows (none for 2012) would be excluded by the caller."""
    ev = set()
    runs = []
    for date, y, m, lp, t in rows:
        if y != year:
            continue
        key = (m, lp)
        if runs and runs[-1][0] == key:
            runs[-1][1].append(t)
        else:
            runs.append((key, [t]))
    for (m, lp), ts in runs:
        c = Counter(ts)
        for n in range(1, 31):
            k = c.get(n, 0)
            if k == 0:
                ev.add((m, n, "skipped"))
            elif k == 2:
                ev.add((m, n, "repeated"))
            elif k > 2:
                ev.add((m, n, f"x{k}"))
    return ev


def tcg_new_year_names(tcg_dir, prefix, years):
    names = {}
    for y in years:
        with open(f"{tcg_dir}/{prefix}_{y}.txt", "rb") as f:
            for ln in f.read().decode("latin-1").replace("\r", "").split("\n"):
                m = RE_NY.match(ln.rstrip())
                if m:
                    names[y] = (m.group(2), m.group(4))
                    break
    return names


def run_checks(version, rows, tcg_dir, prefix, golden):
    mism = []
    n = dict()
    # Table 7
    t7 = t5(golden, f"janson-v1-table7-leap-months-{version}.csv")
    lm = leap_months(rows)
    n["table7"] = len(t7)
    for r in t7:
        y = int(r["year"])
        exp = {int(r["leap_month"])} if r["leap_month"] else set()
        got = lm.get(y, set())
        if exp != got:
            mism.append(f"Table 7, year {y}: leap month expected {sorted(exp)} (table), TCG {sorted(got)}")
    # Table 8
    t8 = t5(golden, f"janson-v1-table8-2012-{version}.csv")
    exp8 = {(int(r["month"]), int(r["tshes"]), r["kind"]) for r in t8}
    got8 = t8_events(rows, 2012)
    n["table8"] = len(t8)
    for e in sorted(exp8 - got8):
        mism.append(f"Table 8, 2012: {e[2]} {e[1]} in month {e[0]} is in the table and not in TCG")
    for e in sorted(got8 - exp8):
        mism.append(f"Table 8, 2012: {e[2]} {e[1]} in month {e[0]} is in TCG and not in the table")
    # Table 9
    t9 = t5(golden, f"janson-v1-table9-losar-{version}.csv")
    fd = first_days(rows)
    n["table9"] = len(t9)
    for r in t9:
        y = int(r["tibetan_year"])
        if fd.get(y) != r["gregorian_date"]:
            mism.append(f"Table 9, Tibetan year {y}: first day expected {r['gregorian_date']} (table), TCG {fd.get(y)}")
    # Table 1 (Phugpa only)
    if version == "phugpa":
        t1 = t5(golden, "janson-v1-table1-losar-phugpa.csv")
        n["table1"] = len(t1)
        names = tcg_new_year_names(tcg_dir, prefix, [int(r["tibetan_year"]) for r in t1])
        for r in t1:
            y = int(r["tibetan_year"])
            if fd.get(y) != r["gregorian_date"]:
                mism.append(f"Table 1, Tibetan year {y}: first day expected {r['gregorian_date']} (table), TCG {fd.get(y)}")
            if names.get(y) != (r["element"], r["animal"]):
                mism.append(f"Table 1, Tibetan year {y}: name expected {(r['element'], r['animal'])} (table), TCG {names.get(y)}")
    return mism, n


def main():
    if sys.argv[1] == "control":
        _, _, version, rowsf, tcg_dir, prefix, golden = sys.argv
        rows = load_rows(rowsf)
        base, n = run_checks(version, rows, tcg_dir, prefix, golden)
        print(f"CONTROL ({version}): unchanged copy: {len(base)} mismatches (expected 0)")
        assert len(base) == 0
        # change one leap month: the leap month of Tibetan year 2005's group gets month number + 1 (Table 7 only)
        lm = leap_months(rows)
        ly = next(y for y in sorted(lm) if 2000 <= y <= 2020 and y not in (2012,) and 2005 <= y)
        mut = [(d, y, (m + 1 if (lp and y == ly) else m), lp, t) for d, y, m, lp, t in rows]
        mism, _ = run_checks(version, mut, tcg_dir, prefix, golden)
        print(f"  one leap month changed (Tibetan year {ly}, month {sorted(lm[ly])[0]} -> {sorted(lm[ly])[0] + 1}):")
        for s in mism:
            print("    ", s)
        ok1 = len(mism) == 1 and mism[0].startswith(f"Table 7, year {ly}:")
        print(f"  expected exactly 1 (Table 7); observed {len(mism)}: {'AS EXPECTED' if ok1 else 'NOT AS EXPECTED'}")
        # change one New Year: the first row of Tibetan year Y gets the next day's date (a copy edit of one cell)
        Y = 2005 if ly != 2005 else 2006
        done = False
        mut2 = []
        for d, y, m, lp, t in rows:
            if y == Y and not done:
                done = True
                import datetime
                d = (datetime.date.fromisoformat(d) + datetime.timedelta(days=1)).isoformat()
            mut2.append((d, y, m, lp, t))
        mism2, _ = run_checks(version, mut2, tcg_dir, prefix, golden)
        exp_n = 2 if version == "phugpa" else 1  # Phugpa: Table 9 and Table 1 both print the year
        print(f"  one New Year changed (Tibetan year {Y}, first day + 1 day):")
        for s in mism2:
            print("    ", s)
        ok2 = len(mism2) == exp_n and all(s.startswith(("Table 9", "Table 1,")) for s in mism2)
        print(f"  expected exactly {exp_n} ({'Tables 9 and 1' if version == 'phugpa' else 'Table 9'}); observed {len(mism2)}: "
              f"{'AS EXPECTED' if ok2 else 'NOT AS EXPECTED'}")
        # both changed together
        mut3 = [(d, y, (m + 1 if (lp and y == ly) else m), lp, t) for d, y, m, lp, t in mut2]
        mism3, _ = run_checks(version, mut3, tcg_dir, prefix, golden)
        exp3 = 1 + exp_n
        print(f"  one leap month and one New Year changed together: expected exactly {exp3}; observed {len(mism3)}: "
              f"{'AS EXPECTED' if len(mism3) == exp3 else 'NOT AS EXPECTED'}")
        sys.exit(0 if (ok1 and ok2 and len(mism3) == exp3) else 1)
    version, rowsf, tcg_dir, prefix, golden = sys.argv[1:6]
    rows = load_rows(rowsf)
    mism, n = run_checks(version, rows, tcg_dir, prefix, golden)
    print(f"READING CHECK ({version}): tables read {n}")
    print(f"  mismatches: {len(mism)}")
    for s in mism:
        print("    ", s)
    sys.exit(0 if not mism else 1)


if __name__ == "__main__":
    main()
```

#### diffrows.py

```python
"""Spec 009 Items 5 and 6: compare two row files (one row per Gregorian day) day by day.

Usage: python3 -I diffrows.py diff <A.csv> <B.csv> <labelA> <labelB> <list_out.txt>
       python3 -I diffrows.py control <A.csv> <B.csv>      (seeded controls: METHOD rules 31, 36)
Rows: gregorian_date,tibetan_year,month,is_leap_month,tshes
A is the left side (the engine, or the T4 fixture); B the right side (TCG, or the engine dump).
"""
import csv
import sys
from collections import Counter, OrderedDict

FIELDS = ["tibetan_year", "month", "is_leap_month", "tshes"]


def load(path):
    with open(path, newline="") as f:
        r = csv.reader(f)
        hdr = next(r)
        assert hdr == ["gregorian_date", "tibetan_year", "month", "is_leap_month", "tshes"], hdr
        return [(a, int(b), int(c), d, int(e)) for a, b, c, d, e in r]


def marks(rows):
    """Per day: 'D' second day of a repeated number (tshes equal to the previous day's), 'S' a number skipped
    immediately before (step 2 modulo 30), '.' neither. The first row has no predecessor and gets '.'."""
    out = ["."]
    for a, b in zip(rows, rows[1:]):
        step = (b[4] - a[4]) % 30
        out.append("D" if step == 0 else ("S" if step == 2 else "."))
    return out


def compare(A, B):
    assert [r[0] for r in A] == [r[0] for r in B], "the two files do not cover the same dates in the same order"
    n = len(A)
    per_field = Counter()
    bad = []
    for i, (a, b) in enumerate(zip(A, B)):
        diff = [f for f, x, y in zip(FIELDS, a[1:], b[1:]) if x != y]
        for f in diff:
            per_field[f] += 1
        if diff:
            bad.append(i)
    return n, per_field, bad


def runs_of(idx):
    runs = []
    for i in idx:
        if runs and runs[-1][-1] == i - 1:
            runs[-1].append(i)
        else:
            runs.append([i])
    return runs


def year_info(rows):
    months = OrderedDict()  # year -> set of (month, leap)
    leap = {}
    first = {}
    for date, y, m, lp, t in rows:
        months.setdefault(y, set()).add((m, lp))
        if lp == "true":
            leap.setdefault(y, set()).add(m)
        first.setdefault(y, date)
    return months, leap, first


def main():
    mode = sys.argv[1]
    if mode == "diff":
        pa, pb, la, lb, out = sys.argv[2:7]
        A, B = load(pa), load(pb)
        n, per_field, bad = compare(A, B)
        mA, mB = marks(A), marks(B)
        runs = runs_of(bad)
        print(f"{la} vs {lb}: {n} days compared")
        print("per field, days that differ (out of %d): %s" % (n, {f: per_field.get(f, 0) for f in FIELDS}))
        print(f"disagreeing days: {len(bad)}; runs of consecutive disagreeing days: {len(runs)}; "
              f"longest run: {max((len(r) for r in runs), default=0)}; run lengths: {dict(sorted(Counter(len(r) for r in runs).items()))}")
        # per Tibetan year
        yA, lA, fA = year_info(A)
        yB, lB, fB = year_info(B)
        years = sorted(set(yA) | set(yB))
        months_diff = [y for y in years if yA.get(y) != yB.get(y)]
        leap_diff = [y for y in years if lA.get(y, set()) != lB.get(y, set())]
        first_diff = [y for y in years if y not in (1899,) and fA.get(y) != fB.get(y)]
        print(f"Tibetan years compared: {len(years)} ({years[0]}..{years[-1]}); first day of the year compared for "
              f"{len(years) - 1} (1899 begins before the range)")
        print(f"  years whose set of months (month, leap) differs: {len(months_diff)} {months_diff}")
        print(f"  years whose leap month differs: {len(leap_diff)} {leap_diff}")
        print(f"  years whose first day differs: {len(first_diff)} {[(y, fA.get(y), fB.get(y)) for y in first_diff]}")
        # the list
        beside = 0
        with open(out, "w") as f:
            f.write(f"# {la} vs {lb}; one line per disagreeing day\n")
            f.write(f"# date | {la} (year, month, leap, tshes) | {lb} (same) | marks at days -1,0,+1 for {la} and {lb} "
                    f"(D = second day of a repeated number, S = a number skipped immediately before, . = neither) | "
                    f"beside a doubled or skipped tshes in either calendar\n")
            for i in bad:
                a, b = A[i], B[i]
                ma = "".join(mA[j] if 0 <= j < n else "-" for j in (i - 1, i, i + 1))
                mb = "".join(mB[j] if 0 <= j < n else "-" for j in (i - 1, i, i + 1))
                flag = any(c in "DS" for c in ma + mb)
                beside += flag
                f.write(f"{a[0]} | {a[1:]} | {b[1:]} | {la}:{ma} {lb}:{mb} | {'yes' if flag else 'NO'}\n")
        print(f"disagreeing days beside a doubled or skipped tshes (days -1,0,+1, either calendar): {beside} of {len(bad)}")
        print(f"list written: {out}")
    elif mode == "control":
        pa, pb = sys.argv[2:4]
        A, B = load(pa), load(pb)
        n0, f0, bad0 = compare(A, B)
        # control 1: one day's tshes changed in a copy of B
        k = 30000
        B1 = list(B)
        r = B1[k]
        B1[k] = (r[0], r[1], r[2], r[3], r[4] % 30 + 1)
        n1, f1, bad1 = compare(A, B1)
        new = sorted(set(bad1) - set(bad0))
        ok = new == [k] and len(bad1) == len(bad0) + 1 and f1["tshes"] == f0["tshes"] + 1
        print(f"CONTROL: one day's tshes changed in a copy of B (row {k}, {r[0]}, tshes {r[4]} -> {r[4] % 30 + 1}):")
        print(f"  before: {len(bad0)} disagreeing days; after: {len(bad1)}; new disagreeing days: {[A[i][0] for i in new]}; "
              f"tshes-field count {f0['tshes']} -> {f1['tshes']}: {'AS EXPECTED (exactly that day, nothing else)' if ok else 'NOT AS EXPECTED'}")
        # control 2: one month number changed on one day of a copy of A -> only the month field, only that day
        A2 = list(A)
        r = A2[40000]
        A2[40000] = (r[0], r[1], r[2] % 12 + 1, r[3], r[4])
        n2, f2, bad2 = compare(A2, B)
        new2 = sorted(set(bad2) - set(bad0))
        ok2 = new2 == [40000] and f2["month"] == f0["month"] + 1
        print(f"CONTROL: one day's month changed in a copy of A (row 40000, {r[0]}): new disagreeing days {[A[i][0] for i in new2]}, "
              f"month-field count {f0['month']} -> {f2['month']}: {'AS EXPECTED' if ok2 else 'NOT AS EXPECTED'}")
        sys.exit(0 if (ok and ok2) else 1)


if __name__ == "__main__":
    main()
```

#### epochs.py

**One string of this script as recorded below differs from the script that ran (Item 11).** At the halt the recorded script was equal to the script that ran (the halt's `cmp`, said in the opening of this section). The `print` call after the loop over `rows`, which reports the anomaly per lunar day, printed a string that copied the text of `t2.c:623` and of a term of `t2.c:637`. Here that string is a description with the same two cites. Only the printed words differ; the arithmetic after it (`a2t`, and the comparison with (7.24) and (7.13)) is unchanged, and the results of §6 are unaffected. The numbers typed into `GZADMG`, `NYIDMG`, `TSEDMG`, `NYILMG` and `TCG` are values cited with file:line (class (b)).

```python
"""Spec 009 Item 3: TCG's epoch data and mean motions, carried by exact rational arithmetic to Janson v1's epochs
(E806, E1732, E1747) and to Table 6's JD 2015531, and compared digit for digit.

TCG's numbers are typed here from the lines named (file:line of tcg1309.zip's sources, read this round); Janson's are
as printed in the paper (equations and pages named). Nothing is copied from the engine.
Usage: python3 -I epochs.py <golden/reference dir>
"""
import csv
import sys
from fractions import Fraction as F

# ------------------------------------------------------------------ TCG, as written
# a TCG radix list is [d0, d1, d2, d3, d4, d5]: weekday lists use radices (7, 60, 60, 6, 707) and the
# sun lists (27, 60, 60, 6, 67); set_grub (t2.c:86-99) sets sun_f = 67 and gza_f = 707 for every epoch.


def wk(lst):
    d = lst
    return d[0] + F(d[1], 60) + F(d[2], 3600) + F(d[3], 21600) + F(d[4], 21600 * 707)


def sun(lst):
    d = lst
    return (F(d[0], 27) + F(d[1], 27 * 60) + F(d[2], 27 * 3600) + F(d[3], 27 * 21600) + F(d[4], 27 * 21600 * 67))


# monthly and daily slopes (t2.c:43-46)
GZADMG = [1, 31, 50, 0, 480, 0]     # t2.c:44  weekday per month
NYIDMG = [2, 10, 58, 1, 17, 0]      # t2.c:43  sun per month
TSEDMG = [0, 59, 3, 4, 16, 0]       # t2.c:45  weekday per lunar day
NYILMG = [0, 4, 21, 5, 43, 0]       # t2.c:46  sun per lunar day

TCG = {
    "phugpa": dict(where="t2.c:60-61 (gda1, nda1), 123-126 (epch_yr, eyr_a, ril_a, ril_b)",
                   gda=[5, 35, 36, 4, 160, 0], nda=[26, 45, 53, 4, 26, 0], epch_yr=-1000, eyr_a=7, ril_a=6, ril_b=61),
    "tsurphu": dict(where="t2.c:65-66 (gda2, nda2), 150-153",
                    gda=[5, 38, 22, 3, 701, 0], nda=[0, 7, 25, 0, 30, 0], epch_yr=-1000, eyr_a=11, ril_a=6, ril_b=75),
    "genden": dict(where="t2.c:80-81 (gda5, nda5), 232-235",
                   gda=[1, 55, 13, 3, 333, 0], nda=[26, 39, 51, 0, 18, 0], epch_yr=1747, eyr_a=10, ril_a=24, ril_b=22),
}

# ------------------------------------------------------------------ Janson v1, as printed
M1, M2 = F(167025, 5656), F(11135, 11312)   # (7.2), (7.3)
S1, S2 = F(65, 804), F(13, 4824)            # (7.6), (7.7)
A1, A2C = F(253, 3528), F(1, 28)            # (7.12), (7.13)
A2E = F(3781, 105840)                       # (7.24)
JANSON = {
    # version: epoch label, Y0, beta*, JD (integer part of m0), m0 frac, s0, a0 and the equations
    "phugpa": dict(label="E806", Y0=806, bstar=61, jd=2015501, m0f=F(4783, 5656), s0=F(743, 804), a0=F(475, 3528),
                   eq="(5.4), (7.4), (7.8), (7.14), pp. 14, 21-23"),
    "tsurphu": dict(label="E1732", Y0=1732, bstar=59, jd=2353745, m0f=F(1795153, 7635600), s0=F(-5983, 108540) % 1,
                    a0=F(207, 392), eq="A.2 (A.3)-(A.5), p. 39"),
    "genden": dict(label="E1747", Y0=1747, bstar=10, jd=2359237, m0f=F(2603, 2828), s0=F(397, 402), a0=F(1523, 1764),
                   eq="A.3 (A.11)-(A.13), p. 42"),
}


def main():
    golden = sys.argv[1]
    with open(f"{golden}/janson-v1-table6-epochs-806.csv", newline="") as f:
        t6 = {r["system"]: r for r in csv.DictReader(f)}
    print("MEAN MOTIONS: TCG's slopes against Janson's")
    rows = [
        ("weekday per month (gzadm, t2.c:44) vs m1 - 28 (7.2), p. 21", wk(GZADMG), M1 - 28),
        ("weekday per lunar day (tsedm, t2.c:45) vs m2 (7.3), p. 21", wk(TSEDMG), M2),
        ("sun per month (nyidm, t2.c:43) vs s1 (7.6), p. 23", sun(NYIDMG), S1),
        ("sun per lunar day (nyilm, t2.c:46) vs s2 (7.7), p. 23", sun(NYILMG), S2),
        ("anomaly per month (rilchaf, t2.c:403-410: 2 in 28ths and 1 in 126ths of those) vs a1 (7.12), p. 23",
         F(2, 28) + F(1, 28 * 126), A1),
    ]
    for name, tcg, jan in rows:
        print(f"  {'EQUAL' if tcg == jan else 'DIFFER'}  {name}: TCG {tcg} ({float(tcg):.9f}); Janson {jan}")
    print("  anomaly per lunar day: gza_dag (t2.c:623-641) adds the lunar day to the 28ths (t2.c:623) and, a second time, "
          "to the 105840ths (t2.c:637): a2 = 1/28 + 1/105840")
    a2t = F(1, 28) + F(1, 28 * 126 * 30)
    print(f"  {'EQUAL to (7.24)' if a2t == A2E else 'NOT (7.24)'}: TCG a2 = {a2t}; Janson (7.13) = {A2C}, (7.24) = {A2E}; "
          f"{'EQUAL to (7.13)' if a2t == A2C else 'DIFFERS from (7.13) by ' + str(a2t - A2C)}")
    print()
    print("EPOCH DATA carried to Janson's own epochs (K = the number of true months from TCG's epoch to Janson's n = 0)")
    for v in ("phugpa", "tsurphu", "genden"):
        t, j = TCG[v], JANSON[v]
        Mdiff = 12 * (j["Y0"] - t["epch_yr"])
        bstar = (t["eyr_a"] + 2 * Mdiff) % 65
        print(f"== {v}: TCG {t['where']}; Janson {j['label']} {j['eq']}")
        print(f"   beta*: TCG holds eyr_a = {t['eyr_a']} at year {t['epch_yr']}; carried by 2 x {Mdiff} months mod 65 = {bstar}; "
              f"Janson {j['bstar']}: {'EQUAL' if bstar == j['bstar'] else 'DIFFER'}")
        W = wk(t["gda"])
        N = sun(t["nda"])
        a0t = F(t["ril_a"], 28) + F(t["ril_b"], 28 * 126)
        # candidate K: the counts around the TCG count of the nominal epoch month
        Kc = Mdiff + (2 * Mdiff + t["eyr_a"]) // 65
        found = {}
        for K in range(Kc - 3, Kc + 4):
            m0c = W + K * (M1 - 28)       # mod 7
            s0c = (N + K * S1) % 1
            a0c = (a0t + K * A1) % 1
            found[K] = dict(m0=m0c, s0=s0c, a0=a0c)
        # the integer part of Janson's m0 modulo 7 in TCG's form: (JD + 2) mod 7
        jw = (j["jd"] + 2) % 7
        print(f"   K candidates {Kc - 3}..{Kc + 3} (TCG's count of the nominal epoch month {Kc} +-3); per quantity, the K that makes it EQUAL:")
        for q, jv in (("a0", j["a0"]), ("s0", j["s0"])):
            ks = [K for K, d in found.items() if d[q] == jv]
            print(f"     {q}: Janson {jv}; equal for K in {ks}")
        ksm = [K for K, d in found.items() if d["m0"] % 1 == j["m0f"] and int(d["m0"] // 1) % 7 == jw]
        ksm_f = [K for K, d in found.items() if d["m0"] % 1 == j["m0f"]]
        print(f"     m0: Janson fraction {j['m0f']} and weekday (JD + 2) mod 7 = {jw}; "
              f"equal fraction for K in {ksm_f}; equal fraction and weekday for K in {ksm}")
        # the K that fits a0 and s0 (the exact quantities) is the carrying offset; report every component at it
        cand = [K for K, d in found.items() if d["a0"] == j["a0"] and d["s0"] == j["s0"]]
        if not cand:
            # coarse radices cannot hold Tsurphu's finer s0: report the K that fits a0 and the nearest s0/m0
            cand = [K for K, d in found.items() if d["a0"] == j["a0"]]
        K = cand[0]
        d = found[K]
        print(f"   carried at K = {K}:")
        print(f"     m0 (traditional, mod 7): carried {d['m0'] % 7} -> fraction {d['m0'] % 1}; Janson fraction {j['m0f']} "
              f"({float(j['m0f']):.9f}); difference carried - Janson = {d['m0'] % 1 - j['m0f']} = {float(d['m0'] % 1 - j['m0f']):+.3e} days; "
              f"weekday carried {int(d['m0'] // 1) % 7} vs Janson (JD+2) mod 7 {jw}")
        print(f"     s0: carried {d['s0']} ({float(d['s0']):.9f}); Janson {j['s0']}; difference {d['s0'] - j['s0']} = {float(d['s0'] - j['s0']):+.3e} revolutions: "
              f"{'EQUAL' if d['s0'] == j['s0'] else 'DIFFER'}")
        print(f"     a0: carried {d['a0']}; Janson {j['a0']}: {'EQUAL' if d['a0'] == j['a0'] else 'DIFFER'}")
        # Table 6: JD 2015531
        r = t6[v]
        D, d1, d2, d3, d4 = map(int, r["m0_radix"].split(":"))
        V6 = D + F(d1, 60) + F(d2, 3600) + F(d3, 21600) + F(d4, 21600 * 707)
        s6 = sun(list(map(int, r["s0_radix"].split(":"))) + [0])
        a6 = F(int(r["a0_radix"].split(":")[0]), 28) + F(int(r["a0_radix"].split(":")[1]), 28 * 126)
        # k months from Janson's n = 0 to the mean new moon on JD 2015531
        m0J = j["jd"] + j["m0f"]
        k = -((-(2015531 - m0J)) // M1)  # ceil
        k = int(k)
        assert 2015531 <= m0J + k * M1 < 2015532
        Kt = K + k
        m06 = (W + Kt * (M1 - 28)) % 7
        s06 = (N + Kt * S1) % 1
        a06 = (a0t + Kt * A1) % 1
        print(f"   Table 6 (JD 2015531, k = {k} months from Janson's n = 0, so K' = {Kt}): printed m0 {r['m0_radix']} = {V6} ({float(V6):.6f}), "
              f"s0 {r['s0_radix']}, a0 {r['a0_radix']}")
        print(f"     m0 carried {m06} ({float(m06):.6f}) vs printed {V6}: {'EQUAL' if m06 == V6 else 'DIFFER by ' + str(float(m06 - V6))}; "
              f"s0 carried {float(s06):.9f} vs printed {float(s6):.9f}: {'EQUAL' if s06 == s6 else 'DIFFER by ' + str(float(s06 - s6))}; "
              f"a0 carried {a06} vs printed {a6}: {'EQUAL' if a06 == a6 else 'DIFFER'}")


if __name__ == "__main__":
    main()
```

#### probecmp.py

```python
"""Spec 009 Item 7 (ii): what a one-change probe of TCG removes, and everything else it changes.

Usage: python3 -I probecmp.py <label> <engine.csv> <tcg_original.csv> <tcg_probe.csv>
Rows may repeat a date in the probe (a day printed twice); rows are compared by date.
Reports, for the 73,414 dates of the engine file:
  - the original TCG's disagreements with the engine (D0), and the probe's (D1);
  - which of D0 the probe removes, which it leaves, which it adds;
  - the dates where the probe prints more than one row, or none;
  - every date on which the probe's output differs from the original TCG's, and whether that date is in D0.
"""
import csv
import sys
from collections import defaultdict


def load(path):
    out = defaultdict(list)
    with open(path, newline="") as f:
        r = csv.reader(f)
        next(r)
        for a, b, c, d, e in r:
            out[a].append((int(b), int(c), d, int(e)))
    return out


def main():
    label, pe, po, pp = sys.argv[1:5]
    eng, orig, prob = load(pe), load(po), load(pp)
    dates = sorted(eng)
    d0 = [d for d in dates if orig.get(d) != eng[d]]
    d1 = [d for d in dates if prob.get(d) != eng[d]]
    removed = sorted(set(d0) - set(d1))
    left = sorted(set(d0) & set(d1))
    added = sorted(set(d1) - set(d0))
    multi = [d for d in prob if len(prob[d]) > 1]
    missing = [d for d in dates if d not in prob]
    changed = sorted(d for d in set(orig) | set(prob) if orig.get(d) != prob.get(d))
    print(f"{label}: engine days {len(dates)}; original TCG disagrees on {len(d0)}; probe TCG disagrees on {len(d1)}")
    print(f"  removed by the probe: {len(removed)}; left: {len(left)} {left}; added: {len(added)} {added}")
    print(f"  dates the probe prints more than once: {len(multi)} {sorted(multi)}; dates it does not print: {len(missing)}")
    print(f"  dates where the probe's output differs from the original TCG's: {len(changed)}; "
          f"in the original disagreement list: {len([d for d in changed if d in set(d0)])}; "
          f"not in it: {[ (d, orig.get(d), prob.get(d)) for d in changed if d not in set(d0)]}")
    for d in d1:
        print(f"    probe vs engine: {d}: engine {eng[d]} probe {prob.get(d)} original {orig.get(d)}")


if __name__ == "__main__":
    main()
```

#### mdtable.py

```python
"""Spec 009: print the markdown rows of THE LIST (Item 6) joined with Item 7's measured true dates.

Usage: python3 -I mdtable.py <version> <diff list> <settle output>
Row: date | engine year/month/tshes | TCG year/month/tshes | marks at days -1,0,+1 (engine / TCG) | N | td13 - k | td24 - k
(a leap month is written 2L). The numbers are copied from the two files, not recomputed.
"""
import re
import sys


def cell(t):
    y, m, lp, d = (x.strip() for x in t.strip("() ").split(","))
    return f"{y}/{m}{'L' if lp == 'True' else ''}/{d}"


def main():
    v, listf, settlef = sys.argv[1:4]
    marks = {}
    for ln in open(listf):
        if ln[0].isdigit():
            parts = [p.strip() for p in ln.split(" | ")]
            m = re.match(r"engine:(\S+) tcg:(\S+)", parts[3])
            marks[parts[0]] = (m.group(1), m.group(2), parts[4])
    sec = False
    rows = []
    for ln in open(settlef):
        if ln.startswith("=== "):
            sec = ln.strip() == f"=== {v}"
            continue
        if sec and re.match(r"\s+\d{4}-\d\d-\d\d \|", ln):
            p = [x.strip() for x in ln.split(" | ")]
            date = p[0]
            eng = cell(p[1][4:])
            tcg = cell(p[2][4:])
            m = re.search(r"N=(\d+) .*td13 - k = ([+-][\d.]+), td24 - k = ([+-][\d.]+), k = JD (\d+)", p[5])
            ma, mb, beside = marks[date]
            rows.append(f"| {date} | {eng} | {tcg} | {ma} / {mb} | {m.group(1)} | {m.group(2)} | {m.group(3)} |")
    print("\n".join(rows))
    print(f"# {len(rows)} rows", file=sys.stderr)


if __name__ == "__main__":
    main()
```

#### janson_ref.py

```python
"""Spec 009 Item 7 (i): an independent reference computation of the Tibetan calendar, from Janson v1 only.

Written from the paper's text (S. Janson, "Tibetan calendar mathematics", arXiv:1401.6285v1), never from the
engine's code or TCG's. Exact rational arithmetic (fractions.Fraction) throughout; no floating point.

Sources inside the paper (page numbers of the arXiv v1 PDF):
  true month count and the leap rule: Section 5, eqs. (5.2), (5.3), (5.7), (5.8), (5.9), (5.10), pp. 14-16;
      Tsurphu: A.2 (A.9) and the sentences after it, p. 39-40; New Genden: A.3 (A.17) and the sentence after it, p. 43
  (Y, M, leap) from n: (5.19)-(5.22), p. 17 (used as a cross-check of the forward month table)
  mean date, mean sun, anomaly, true date: Section 7, (7.1)-(7.22), pp. 21-24
  the labelling of calendar days: Section 6, p. 20; (8.1), p. 27
  epoch constants: (7.2)-(7.8), (7.12)-(7.14) p. 21-23 (Phugpa, E806); (A.1)-(A.5) p. 39 (Tsurphu, E1732);
      (A.11)-(A.13) p. 42 (Genden, E1747)
  the alternative a2: Remark 14, (7.24), p. 25

Usage:
  python3 -I janson_ref.py selfcheck
  python3 -I janson_ref.py table8 <golden/reference dir> [--seed v:const:delta] [--a2 exact]
  python3 -I janson_ref.py days <days.txt> <dump dir> [--seed ...] [--a2 exact]
  python3 -I janson_ref.py full <dump dir> [--a2 exact]       (every day 1900-01-01..2100-12-31 against a dump)
  python3 -I janson_ref.py settle <version> <list.txt> <tcg rows.csv> <engine dump.csv>
"""
import csv
import datetime
import sys
from collections import Counter
from fractions import Fraction as F

# ---------------------------------------------------------------- constants as printed in the paper
M1 = F(167025, 5656)        # (7.2)
M2 = F(11135, 11312)        # (7.3)
S1 = F(65, 804)             # (7.6)
S2 = F(13, 4824)            # (7.7)
A1 = F(253, 3528)           # (7.12)
A2_CUSTOMARY = F(1, 28)     # (7.13)
A2_EXACT = F(3781, 105840)  # (7.24), Remark 14

VERSIONS = {
    # Phugpa, epoch E806: (5.4) beta* = 61, (7.4), (7.8), (7.14); rule (5.8) ix = 48 or 49; "48 = 65 - 17" (5.10)
    "phugpa": dict(Y0=806, M0=3, bstar=61, c=17, leap=(48, 49),
                   m0=2015501 + F(4783, 5656), s0=F(743, 804), a0=F(475, 3528)),
    # Tsurphu, epoch E1732: (A.3)-(A.5), beta* = 59; rule (A.9) ix = 0 or 1; regular months rounded down (c = 0)
    "tsurphu": dict(Y0=1732, M0=3, bstar=59, c=0, leap=(0, 1),
                    m0=2353745 + F(1795153, 7635600), s0=F(-5983, 108540), a0=F(207, 392)),
    # New Genden, epoch E1747: (A.11)-(A.13), beta* = 10; rule (A.17) ix = 46 or 47; "48 replaced by 46": c = 65 - 46
    "genden": dict(Y0=1747, M0=3, bstar=10, c=19, leap=(46, 47),
                   m0=2359237 + F(2603, 2828), s0=F(397, 402), a0=F(1523, 1764)),
}
# beta for the inverse formulas (5.19)-(5.22): (5.15) 184 - beta* (Phugpa); (A.10) and C.3: 142 (E1732); A.3: 172
BETA = {"phugpa": 184 - 61, "tsurphu": 142, "genden": 172}

# the radix forms as printed, to cross-check each fraction (a misreading of either would show)
RADIX_CHECKS = [
    ("phugpa m0 frac (7.4)", F(4783, 5656), [0, 50, 44, 2, 38], [60, 60, 6, 707], False),
    ("phugpa s0 (7.8)", F(743, 804), [24, 57, 5, 2, 16], [27, 60, 60, 6, 67], True),
    ("phugpa a0 (7.14)", F(475, 3528), [3, 97], [28, 126], True),
    ("m1 mod 28 (7.2)", F(167025, 5656) - 29, [0, 31, 50, 0, 480], [60, 60, 6, 707], False),
    ("tsurphu m0 frac (A.3)", F(1795153, 7635600), [4, 14, 6, 2, 2, 666], [60, 60, 6, 13, 707], False),
    ("tsurphu s0 (A.4)", F(5983, 108540), [1, 29, 17, 5, 6, 1], [27, 60, 60, 6, 13, 67], True),
    ("tsurphu a0 (A.5)", F(207, 392), [14, 99], [28, 126], True),
    ("genden m0 frac (A.11)", F(2603, 2828), [1, 55, 13, 3, 31, 394], [60, 60, 6, 67, 707], False),
    ("genden s0 (A.12)", F(397, 402), [26, 39, 51, 0, 18], [27, 60, 60, 6, 67], True),
    ("genden a0 (A.13)", F(1523, 1764), [24, 22], [28, 126], True),
]


def radix_value(digits, radices, first_is_fraction_of_radix0):
    """Value of a printed radix form. Form 'd0; d1, d2.. (r1, r2..)' (time): d0 + d1/r1 + d2/(r1 r2) + ...
    Form 'd0, d1.. (r0, r1..)' (longitude, anomaly): d0/r0 + d1/(r0 r1) + ... (a fraction of a full circle)."""
    if first_is_fraction_of_radix0:
        val, den = F(0), F(1)
        for d, r in zip(digits, radices):
            den *= r
            val += F(d, 1) / den
        return val
    val, den = F(digits[0]), F(1)
    for d, r in zip(digits[1:], radices):
        den *= r
        val += F(d, 1) / den
    return val


def selfcheck():
    ok = True
    for name, printed, digits, radices, frac in RADIX_CHECKS:
        v = radix_value(digits, radices, frac)
        # the fractional parts must be equal; the integer part of m0 is the JD offset the paper adds (Remark 8)
        good = (v % 1 == printed % 1)
        ok &= good
        print(f"  {'OK ' if good else 'BAD'} {name}: radix form = {v} ({float(v):.9f}); printed {printed}")
    # derived relations the paper states
    rel = [("m1 = 30 m2", M1 == 30 * M2), ("s1 = 30 s2", S1 == 30 * S2),
           ("a2 exact = (1 + a1)/30 (7.24)", A2_EXACT == (1 + A1) / 30),
           ("a2 exact = 1/28 + 1/105840", A2_EXACT == F(1, 28) + F(1, 105840)),
           ("beta + beta* = 6 mod 65 (A.10), tsurphu", (BETA['tsurphu'] + VERSIONS['tsurphu']['bstar']) % 65 == 6),
           ("beta + beta* = 52 mod 65 (A.18), genden", (BETA['genden'] + VERSIONS['genden']['bstar']) % 65 == 52),
           ("beta = 184 - beta* (5.15), phugpa", BETA['phugpa'] == 184 - VERSIONS['phugpa']['bstar'])]
    for name, good in rel:
        ok &= good
        print(f"  {'OK ' if good else 'BAD'} {name}")
    return ok


# ---------------------------------------------------------------- the astronomical functions
MOON_TAB = [0, 5, 10, 15, 19, 22, 24, 25]  # (7.18)
SUN_TAB = [0, 6, 10, 11]                   # (7.21)


def moon_tab_int(i):
    i %= 28
    if i <= 7:
        return MOON_TAB[i]
    if i <= 14:
        return MOON_TAB[14 - i]   # moon_tab(14 - i) = moon_tab(i)
    return -moon_tab_int(i - 14)  # moon_tab(14 + i) = -moon_tab(i)


def sun_tab_int(i):
    i %= 12
    if i <= 3:
        return SUN_TAB[i]
    if i <= 6:
        return SUN_TAB[6 - i]     # sun_tab(6 - i) = sun_tab(i)
    return -sun_tab_int(i - 6)    # sun_tab(6 + i) = -sun_tab(i)


def interp(tabfun, x):
    """Linear interpolation between integer arguments."""
    i = x.numerator // x.denominator  # floor
    frac = x - i
    a, b = tabfun(i), tabfun(i + 1)
    return a + (b - a) * frac


def make(version, a2_exact=False, overrides=None):
    p = dict(VERSIONS[version])
    if overrides:
        p.update(overrides)
    a2 = A2_EXACT if a2_exact else A2_CUSTOMARY
    return p, a2


def parts(p, a2, n, d):
    """The quantities at the end of lunar day d in true month count n (Section 7)."""
    mean_date = n * M1 + d * M2 + p["m0"]                      # (7.1)
    mean_sun = n * S1 + d * S2 + p["s0"]                       # (7.5)
    anomaly_moon = n * A1 + d * a2 + p["a0"]                   # (7.11)
    xm = (28 * anomaly_moon) % 28                              # anomaly counted modulo 1
    moon_equ = interp(moon_tab_int, xm)                        # (7.17)
    anomaly_sun = mean_sun - F(1, 4)                           # (7.19)
    xs = (12 * anomaly_sun) % 12
    sun_equ = interp(sun_tab_int, xs)                          # (7.20)
    true_date = mean_date + moon_equ / 60 - sun_equ / 60       # (7.22)
    return dict(mean_date=mean_date, moon_equ=moon_equ, sun_equ=sun_equ, true_date=true_date)


def true_date(p, a2, N):
    n, d = divmod(N - 1, 30)
    return parts(p, a2, n, d + 1)["true_date"]


# ---------------------------------------------------------------- the month table, Section 5
def month_table(version, p, y_from, y_to):
    """Forward: for each (Y, M) the true month count of the regular month and of the leap month if any.
    Returns dict n -> (Y, M, leap)."""
    Y0, M0, bstar, c, leap = p["Y0"], p["M0"], p["bstar"], p["c"], p["leap"]
    table = {}
    for Y in range(y_from, y_to + 1):
        for M in range(1, 13):
            Mstar = 12 * (Y - Y0) + M - M0                                # (5.2)
            ix = (67 * Mstar + bstar) % 65                                # (5.7)
            n_reg = (67 * Mstar + bstar + c) // 65                        # (5.10) with the version's c
            if ix in leap:                                                 # (5.8), (A.9), (A.17)
                table[n_reg - 1] = (Y, M, True)
                table[n_reg] = (Y, M, False)
            else:
                table[n_reg] = (Y, M, False)
    return table


def beta_crosscheck(version, p, table):
    """(5.19)-(5.22) with the paper's printed beta must reproduce the forward table."""
    beta, Y0 = BETA[version], p["Y0"]
    bad = 0
    for n, (Y, M, leap) in table.items():
        x = -((-(65 * n + beta)) // 67)                                    # ceil((65n + beta)/67)   (5.19)
        Mx = x % 12 or 12                                                  # x amod 12               (5.20)
        Yx = (x - Mx) // 12 + Y0                                           # (5.21)
        lx = ((65 * n + beta) % 67) in (1, 2)                              # (5.22)
        if (Yx, Mx, lx) != (Y, M, leap):
            bad += 1
    return bad


# ---------------------------------------------------------------- labels, Section 6 and (8.1)
def floor(fr):
    return fr.numerator // fr.denominator


def jd_of(y, m, d):
    a = (14 - m) // 12
    yy = y + 4800 - a
    mm = m + 12 * a - 3
    return d + (153 * mm + 2) // 5 + 365 * yy + yy // 4 - yy // 100 + yy // 400 - 32045


def date_of(jd):
    return datetime.date(1970, 1, 1) + datetime.timedelta(days=jd - 2440588)


def labels(version, jd_first, jd_last, a2_exact=False, overrides=None, keep_td=False):
    """Label of every calendar day jd_first..jd_last: day J gets the lunar day N = min{N : floor(td(N)) >= J}
    (Section 6: a calendar day is labelled by the lunar day current at its beginning; a lunar day names the
    calendar day where it ends, (8.1); a day on which no lunar day ends takes the name of the following day;
    when two lunar days end on one day the first names it)."""
    p, a2 = make(version, a2_exact, overrides)
    N0 = int((jd_first - p["m0"]) / M2) - 8
    N1 = int((jd_last - p["m0"]) / M2) + 8
    tds = {N: true_date(p, a2, N) for N in range(N0, N1 + 1)}
    fl = {N: floor(t) for N, t in tds.items()}
    assert fl[N0] < jd_first and fl[N1] > jd_last
    y_from = date_of(jd_first).year - 2
    y_to = date_of(jd_last).year + 2
    table = month_table(version, p, y_from - 1, y_to + 1)
    out = {}
    N = N0
    for J in range(jd_first, jd_last + 1):
        while fl[N] < J:
            N += 1
        n, d = divmod(N - 1, 30)
        Y, M, leap = table[n]
        out[J] = (Y, M, leap, d + 1, N)
    return (out, tds, p, a2, table) if keep_td else out


def load_rows(path):
    with open(path, newline="") as f:
        r = csv.reader(f)
        next(r)
        return [(a, int(b), int(c), d == "true", int(e)) for a, b, c, d, e in r]


def parse_overrides(argv):
    over = {}
    a2e = "--a2" in argv and argv[argv.index("--a2") + 1] == "exact"
    if "--seed" in argv:
        spec = argv[argv.index("--seed") + 1]  # version:const:delta   e.g. phugpa:m0:1/5656
        v, c, delta = spec.split(":")
        over[v] = {c: VERSIONS[v][c] + F(delta)}
    return over, a2e


def table8_events(rows, year):
    runs = []
    for date, y, m, lp, t in rows:
        if y != year:
            continue
        key = (m, lp)
        if runs and runs[-1][0] == key:
            runs[-1][1].append(t)
        else:
            runs.append((key, [t]))
    ev = set()
    for (m, lp), ts in runs:
        c = Counter(ts)
        for k in range(1, 31):
            if c.get(k, 0) == 0:
                ev.add((m, k, "skipped"))
            elif c.get(k, 0) == 2:
                ev.add((m, k, "repeated"))
    return ev


def script_rows(version, first, last, a2_exact=False, overrides=None):
    jf, jl = jd_of(*first), jd_of(*last)
    lab = labels(version, jf, jl, a2_exact, overrides)
    return [(date_of(J).isoformat(), lab[J][0], lab[J][1], lab[J][2], lab[J][3]) for J in range(jf, jl + 1)]


def main():
    mode = sys.argv[1]
    if mode == "selfcheck":
        ok = selfcheck()
        for v in VERSIONS:
            p, a2 = make(v)
            tb = month_table(v, p, 1880, 2130)
            bad = beta_crosscheck(v, p, tb)
            ns = sorted(tb)
            contiguous = ns == list(range(ns[0], ns[-1] + 1))
            print(f"  {v}: month table 1880..2130 has {len(tb)} months, n contiguous: {contiguous}; "
                  f"(5.19)-(5.22) with the printed beta disagree on {bad} months")
            ok &= contiguous and bad == 0
        print("SELFCHECK", "PASS" if ok else "FAIL")
        sys.exit(0 if ok else 1)
    over, a2e = parse_overrides(sys.argv)
    if mode == "table8":
        golden = sys.argv[2]
        allgood = True
        for v in VERSIONS:
            rows = script_rows(v, (2011, 12, 1), (2013, 3, 31), a2e, over.get(v))
            got = table8_events(rows, 2012)
            with open(f"{golden}/janson-v1-table8-2012-{v}.csv", newline="") as f:
                exp = {(int(r["month"]), int(r["tshes"]), r["kind"]) for r in csv.DictReader(f)}
            miss, extra = sorted(exp - got), sorted(got - exp)
            good = not miss and not extra
            allgood &= good
            print(f"  {v}: Table 8 (2012): {len(exp)} printed events, script {len(got)}; "
                  f"in table not script {miss}; in script not table {extra}: {'REPRODUCED' if good else 'NOT REPRODUCED'}")
        print("TABLE8", "PASS" if allgood else "FAIL")
        sys.exit(0 if allgood else 1)
    if mode == "days":
        daysf, dumpdir = sys.argv[2], sys.argv[3]
        want = []
        for ln in open(daysf):
            ln = ln.strip()
            if ln and not ln.startswith("#"):
                v, date = ln.split()[:2]
                want.append((v, date))
        dumps = {v: {r[0]: r for r in load_rows(f"{dumpdir}/engine-{v}.csv")} for v in VERSIONS}
        bad = 0
        for v, date in want:
            y, m, d = map(int, date.split("-"))
            J = jd_of(y, m, d)
            lab = labels(v, J - 1, J + 1, a2e, over.get(v))[J]
            eng = dumps[v][date]
            sc = (date, lab[0], lab[1], lab[2], lab[3])
            good = sc == eng
            bad += not good
            print(f"  {'OK ' if good else 'DIFF'} {v:8s} {date}: script {sc[1:]} engine {eng[1:]}")
        print(f"DAYS: {len(want)} listed, {len(want) - bad} reproduced, {bad} not")
        sys.exit(0 if bad == 0 else 1)
    if mode == "settle":
        # python3 -I janson_ref.py settle <version> <list.txt> <tcg rows.csv> <engine dump.csv>
        v, listf, tcgf, engf = sys.argv[2:6]
        tcg = {r[0]: r for r in load_rows(tcgf)}
        eng = {r[0]: r for r in load_rows(engf)}
        dates = [ln.split(" | ")[0] for ln in open(listf) if ln[0].isdigit()]
        agree_eng = agree_tcg = 0
        print(f"  {len(dates)} disagreeing days of {v}")
        print("  date | engine | TCG | script (7.13) | script (7.24) | lunar day N, true date by (7.13) and (7.24) "
              "relative to the integer boundary k (days) | straddling")
        for date in dates:
            y, m, d = map(int, date.split("-"))
            J = jd_of(y, m, d)
            l13, td13, p, a13, _ = labels(v, J - 3, J + 3, False, None, True)
            l24, td24, _, a24, _ = labels(v, J - 3, J + 3, True, None, True)
            s13, s24 = (date,) + l13[J][:4], (date,) + l24[J][:4]
            e, t = eng[date], tcg[date]
            agree_eng += (s13 == e)
            agree_tcg += (s24 == t)
            stra = [N for N in td13 if N in td24 and floor(td13[N]) != floor(td24[N])]
            # the straddling lunar days within one day of this calendar day
            near = [N for N in stra if abs(floor(td13[N]) - J) <= 1 or abs(floor(td24[N]) - J) <= 1]
            for N in near:
                k = max(floor(td13[N]), floor(td24[N]))
                n_, d_ = divmod(N - 1, 30)
                print(f"  {date} | eng {e[1:]} | tcg {t[1:]} | s13 {s13[1:]} | s24 {s24[1:]} | N={N} (n={n_}, d={d_ + 1}) "
                      f"td13 - k = {float(td13[N] - k):+.7f}, td24 - k = {float(td24[N] - k):+.7f}, k = JD {k} = {date_of(k)} | "
                      f"{len(near)} straddling lunar day(s)")
        print(f"  script (7.13) reproduces the engine on {agree_eng} of {len(dates)}; script (7.24) reproduces TCG on {agree_tcg} of {len(dates)}")
        sys.exit(0)
    if mode == "monthjump":
        # python3 -I janson_ref.py monthjump <version> <n> : lunar day 30 of month n against lunar day "0" of month n+1
        # (the same instant in the month-by-month frame); equal under (7.24), not under (7.13) (Remark 14, p. 25)
        v, n = sys.argv[2], int(sys.argv[3])
        for name, e in (("a2 = 1/28 (7.13)", False), ("a2 = 3781/105840 (7.24)", True)):
            p, a2 = make(v, e)
            t30 = parts(p, a2, n, 30)["true_date"]
            t0 = parts(p, a2, n + 1, 0)["true_date"]
            print(f"  {v} n={n} {name}: td(n, 30) = {float(t30):.7f}, td(n+1, 0) = {float(t0):.7f}, difference {float(t0 - t30):+.3e} days; "
                  f"floors {floor(t30)} and {floor(t0)}")
        sys.exit(0)
    if mode == "vsrows":
        # python3 -I janson_ref.py vsrows <version> <rows.csv> [--a2 exact]: the script against any row file
        v, path = sys.argv[2], sys.argv[3]
        rows = script_rows(v, (1900, 1, 1), (2100, 12, 31), a2e, over.get(v))
        other = load_rows(path)
        diff = [(a, b) for a, b in zip(rows, other) if a != b]
        print(f"  {v}: script (a2 {'exact (7.24)' if a2e else 'customary (7.13)'}) vs {path.split('/')[-1]}: "
              f"{len(other)} days, {len(diff)} days differ")
        for a, b in diff[:60]:
            print(f"     {a[0]} script {a[1:]} file {b[1:]}")
        sys.exit(0)
    if mode == "full":
        dumpdir = sys.argv[2]
        for v in VERSIONS:
            rows = script_rows(v, (1900, 1, 1), (2100, 12, 31), a2e, over.get(v))
            eng = load_rows(f"{dumpdir}/engine-{v}.csv")
            diff = [(a, b) for a, b in zip(rows, eng) if a != b]
            print(f"  {v}: script vs engine dump, {len(eng)} days, script rows {len(rows)}: {len(diff)} days differ")
            for a, b in diff[:40]:
                print(f"     {a[0]} script {a[1:]} engine {b[1:]}")
        sys.exit(0)


if __name__ == "__main__":
    main()
```
