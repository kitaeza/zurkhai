# BACKLOG — possibilities, not commitments

Status: r2 (2026-11-12). Not gated; it changes by a revision entry below, in the commit that changes it.

This file keeps things the project might do. It is **a list of possibilities, not a queue**: nothing here is planned, owed or decided. The plan is `docs/ROADMAP.md`. Three records stay apart, and **one item is filed in only one of them**:

- `DEBT.md` is what is **owed**: a shortcut or a constraint, with a TRIGGER.
- `FORKS.md` is a decision **open** and waiting for the human.
- `BACKLOG.md` is something that is **possible**: it would buy something, and nobody has committed to it.

**An entry** is numbered `B-n` from this file, in filing order, and the number is never reused. It has four fields and no more: a short name (the heading); where it came from (the date, and whether from an advisor conversation or a spec's aside); what it would buy, in one or two sentences; its state. Each entry also carries the date its evidence was gathered.

```
### B-n — <short name>
- **Origin:** <date, advisor conversation | a spec's aside>
- **Would buy:** <one or two sentences>
- **State:** idea | sized | promoted (<spec>) | stale (<reason>) | dropped (<reason>)
- **Evidence date:** <date the facts behind the entry were gathered>
```

**States.** An entry begins as an `idea`, may become `sized` once someone has estimated what it costs, and ends as `promoted` (naming the spec that took it up), `stale` (its evidence aged or was superseded; the reason is written beside it) or `dropped` (decided against; the reason is written beside it). Stale and dropped entries are kept, never deleted: a later reader should find that the idea was considered, and why it left.

**Before promoting a stale entry**, re-verify its facts: its evidence date says how old they are, and a promoted entry rests on facts as they are now.

---

### B-1 — a Telegram Mini App client
- **Origin:** 2026-10-02, advisor conversation; 001 §10's M5 reads "web or TG Mini App"
- **Would buy:** a second client, inside Telegram, on the same engine through wasm-bindgen (F1's option (a) names the Telegram Mini App)
- **State:** idea
- **Evidence date:** 2026-10-02

### B-2 — real-world Phugpa controls
- **Origin:** 2026-11-11, advisor conversation
- **Would buy:** published calendars that follow Phugpa — the Kalmyk Central Khurul's printed calendars (2023–2026) and Rabten Choeling's 2026 calendar, both in the paper library — as a positive control: the Q1 comparison should find them Phugpa, which shows it can tell Phugpa from the system the Buryat datsans follow. Their role and tier are undecided.
- **State:** idea
- **Evidence date:** 2026-11-11 (the advisor's reading of three months and a delegated reader's of nine: the Khurul's 2025 calendar equals Phugpa on all 365 days; RELAYED)

### B-3 — approaching the other datsans
- **Origin:** 2026-11-11, advisor conversation
- **Would buy:** beyond F4's one datsan: asking the datsans whose calendars the library holds for older editions, for written permission to publish their calendars' facts (001 §8), and for how their calendar is computed
- **State:** idea
- **Evidence date:** 2026-11-11

### B-4 — Henning's published Tsurphu lists as the variant's oracle
- **Origin:** 2026-11-11, advisor conversation
- **Would buy:** an oracle, already in the paper library (henning-kalacakra), for the karana-Sun Tsurphu variant of D-19, if that variant is ever built
- **State:** idea
- **Evidence date:** 2026-11-11 (his 2012 list differs from Janson v1's Table 8 in months 4, 5 and 11; RELAYED, as D-19 says)

### B-5 — «Онцар Шелжи Мелон», the treatise the Aginsky datsan's about page names
- **Origin:** 2026-11-12, advisor conversation (from the saved page `aginsky-datsan/saved-2026-11-12/agin-o-daczane.html`)
- **Would buy:** a named astrological treatise (1908, by Габжа лама Лубсан Бальжир of the Aginsky datsan) that the page says lamas of Mongolia and Russia use today for zurkhai calculation: a candidate primary statement of which arithmetic Buryat calendars compute, for the Q1 verdict.
- **State:** idea
- **Evidence date:** 2026-11-12

## Revisions

- r1 (2026-11-12): initial, on the human's ruling R2 (spec 011); B-1 to B-4 filed from the advisor's seeds.
- r2 (2026-11-12): B-5 filed, on the human's R10 (spec 013).
