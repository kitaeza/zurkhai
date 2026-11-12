# 012 — Source checks: who publishes each calendar in the library, and on what terms

```
spec: 012
prompt: docs/specs/prompts/012-source-checks.md
arc: Sources
branch: spec/012-source-checks (cut from main at c4af28d0214bb035b2d5252d9570234544254174, tree 01cc8fb8982037fee41839c0727660393d7387c9)
merge SHA: not yet merged
pushed: no
model: Sonnet 5.5 (claude-sonnet-5-5), effort High (one session; no switch to the fallback model)
gate: 147 tests passed, 0 failed (debug) / 147 passed, 0 failed (release) — no failure; run once, on the clean committed tree at 0d61fef0f8e49ef2c40cb79693bbdef69b96eec5 (11 legs run, 11 passed, 0 failed; the msrv leg also 147 / 0); the final commit changes docs/specs/012-source-checks.md, docs/specs/INDEX.md and docs/ROADMAP.md only, which no leg reads; skipped legs: wasm-pack (crates/tshes-wasm does not exist, 002 §7)
```

Status: draft
Depends on: 001 (§1, §6, §8), 003 (§4, P3)
Resolves: F28, F29, F30, F31 (filed already RULED by Item 0)
Milestone: M0
Governs: docs/sources.md

## Purpose

For each of the seven calendars in the paper library, this round looks on the web for who publishes it, whether the publisher is a datsan of the Buddhist Traditional Sangha of Russia (001 §1), what the publisher's terms say about reuse, and what the governing law says about reusing a calendar's facts (001 §8); it also files the human's rulings of 2026-11-12 on the coming Sources round. It transcribes nothing, admits nothing, decides no tier and draws no legal conclusion. It does not decide any admission; «Ринпоче Багша» stays a candidate only (F22).

## Non-properties

- Admission, tier and any permission to reuse: the human's, at the Sources round (`docs/ROADMAP.md`, buryat-sources).
- Any golden row, any transcription, any change under `crates/`, `golden/`, `rules/`, `clients/`, `tools/`, `.github/`.

## Forks

None raised. F28 to F31 are the human's rulings of 2026-11-12, filed already RULED.

## Stop rules

- STOP RULE (2): Item 1's control failed; the round continued, and every WebFetch quotation is "wording not verified (Item 1)". This governed the round.
- STOP RULE (4): no legal conclusion is drawn anywhere in this file.
- STOP RULE (7): two PDFs that WebFetch saved outside the round's grants were not read.

## Reference

`docs/sources.md` (seven library rows, one new). Citations carry FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED.

## Revisions

- r1 (2026-11-12): initial.

— spec text ends here —

## Report

The date is 2026-11-12 throughout: `date` printed `чт 12 нояб 2026 14:40:51 MSK` at the start and `15:02:59 MSK` later; no day change.

### 0. How to read this report

Every claim is MEASURED, RECALLED, INFERRED or RELAYED (`docs/ROUNDS.md` §4.3), and every citation FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED. "Library copy" means a file in `/home/kitaeza/zurkhai-papers/`, saved on 2026-11-11 and read this round. Item 1's control did not run, so every quotation that came through WebFetch (rabten.eu, ru.wikipedia.org, www.wipo.int) is marked "wording not verified (Item 1)"; a quotation from a library copy is not affected. A search result's text is a search tool's summary; where it quotes a statute or a page, the quotation is SNIPPET-VERIFIED and its wording is likewise not verified.

**What the round could not reach** (one attempt per host, no retry; D-32): datsan-rinpoche-bagsha.ru, ivolgdatsan.ru, aginskydatsan.ru, khurul.ru, egrul.nalog.ru, nco.minjust.gov.ru, legalacts.ru, dharma.ru, gkodeksrf.ru and rulaws.ru returned `connect ECONNREFUSED`; dazanspb.ru and pravo.gov.ru returned "Command failed with no output"; lawbrary.ch returned HTTP 403. The cause is not established (INFERRED only that it is the network path, not a decision of those sites). The result: **no register entry and no Sangha list was obtained, so every membership verdict below is UNDETERMINED**, and **no provision of Item 4(b) is FETCHED from an official publication**.

### 1. The decisive item: the seven holdings

Verdict words are as Item 2(c) defines them. "Law" is Item 4(a): the country as far as the terms or the seat show it, INFERRED. All quotations of the terms are from library copies unless a URL is given.

| Holding | Publisher (legal name as given) and identifiers | Membership verdict; strongest evidence FOR / AGAINST | Terms found (quoted) | Law (INFERRED) | Open |
|---|---|---|---|---|---|
| `datsan-rinpoche-bagsha/` | «Местная религиозная буддийская организация «Буддийский центр «Ринпоче Багша»», Улан-Удэ (`datsan-about/o-datsane.txt` lines 112–113). ИНН 0323082410, КПП 032301001, ОГРН 2020300001181 (EGRUL entry "18 октября 2002"), certificate number 1020300001171 of 05.05.2006 (line 136) | **UNDETERMINED.** FOR: none from a register or the Sangha. AGAINST: the About page never names the Sangha (`grep -i -E 'сангх\|БТСР'` over the file prints nothing, MEASURED); the legal name carries no Sangha words (but see 2(d): nor do two Sangha datsans' names) | Footer, line 137: "Любое использование либо копирование материалов или подборки материалов сайта, элементов дизайна и оформления допускается лишь с письменного разрешения правообладателя и только со ссылкой на источник: www.datsan-rinpoche-bagsha.ru". Live page not fetched | Russia: seat "670024, Россия, Республика Бурятия, г. Улан-Удэ, ул. Стрелецкая, 1" (line 136); line 136 cites ст. 24.1 and 24.2 of Federal Law 125-FZ | the tax register and the Ministry of Justice register entry; a Sangha list |
| `ivolginsky-datsan/` | Not read in full on the print. The advisor's reading (RELAYED, in `docs/sources.md`) is that the footer names the Sangha and "the local religious organisation of Buddhists of the Ivolginsky datsan". A search result (SNIPPET-VERIFIED, an MChS certificate) names «Местная религиозная организация буддистов Иволгинский дацан "Хамбын Хурээ"», с. Верхняя Березовка. No ИНН/ОГРН found | **UNDETERMINED.** FOR: the publisher's own print, whose footer appears to open with «БУДДИЙСКАЯ ТРАДИЦИОННАЯ САНГХА РОССИИ» (INFERRED from `overview-1.png`, too small to read in full); ru.wikipedia (FETCHED, secondary, wording not verified (Item 1)): the Sangha's card names the residence «Иволгинский дацан». AGAINST: nothing found | The PDF's text layer is 239 bytes and holds none; no notice found by grep. ivolgdatsan.ru UNREACHABLE | Russia: Иволгинский район, Бурятия (search result) | the register entry; a legible footer; the site's own footer and terms |
| `aginsky-datsan/` | «Агинский дацан «Дэчен Лхундублинг»», headed «Буддийская Традиционная Сангха России»; "Составители: Жапов А. Т., Дондоков Б. Д." (both PDFs, `pdftotext`, MEASURED). No legal name, ИНН or ОГРН found | **UNDETERMINED.** FOR: the publisher's own print, both PDFs: «Буддийская Традиционная Сангха России» above «Агинский дацан» (MEASURED). AGAINST: ru.wikipedia's Sangha article lists it only in a navigation list and says nothing of its membership (FETCHED, secondary, wording not verified (Item 1)) | Neither PDF prints a notice (`pdftotext`: 19,193 and 19,671 bytes; grep for права, копир, использован, ©, copyright, лиценз, разреш printed no match). aginskydatsan.ru UNREACHABLE | Russia (a search result, a news item on khurul.ru, the Kalmyk khurul's site, says the datsan turns 210; seat not shown by the calendars) | the register entry; the site's about page; the legal name |
| `gunzechoinei-datsan/` | «Санкт-Петербургский Дацан Гунзэчойнэй» (`dazanspb/page.txt` line 44: "выпускаются по заказу Санкт-Петербургского Дацана Гунзэчойнэй"); kiosk address "г. Санкт-Петербург , Приморский пр-кт , д. 91" (line 57). The legal entity's name, ИНН and ОГРН were not found | **UNDETERMINED.** FOR: the datsan's own page says its lunar data follow the tradition "используемой Буддийской Сангхой (Общиной) России" (2025 page, line 45) and "Буддийской Традиционной Сангхой (Общиной) России" (2026 page, line 45): a statement about the tradition, not of membership. AGAINST: nothing found | Each of three pages ends "© Дацан Гунзэчойнэй 2026" and "При использовании любых материалов указание автора текста (при наличии) и ссылка на dazanspb.ru обязательны" (2025 page lines 60–61; 2024 lines 52–53; 2026 lines 64–65). dazanspb.ru UNREACHABLE; no printed calendar in the library | Russia: St Petersburg | the legal entity's name and register entry; the printed calendars' own notices |
| `khoymorsky-datsan/` | Brand on the shop page: Хойморский дацан "Дэчен Равжалин" (`page.txt` line 114); the poster's own panel: "Республика Бурятия, Тункинский район, Хойморский дацан «Дэчен Равжалин», лама астролог Лобсан Чойнзин (Сотников И.В.)" (read on `overview.png`). The shop's legal name was not reached (its «Оферта» page, `/page/oferta`, is UNREACHABLE). Warehouse: "г. Санкт-Петербург, ул. Крупской, д. 55А" (line 152) | **UNDETERMINED**, and the publisher's identity is itself unsettled. FOR: nothing. AGAINST / OPEN: ru.wikipedia's Sangha article names the Khoymorsky datsan «Бодхидхарма» (FETCHED, secondary, wording not verified (Item 1)); a search result says Wikipedia gives «Дэчен Рабжалин» to a datsan of Dzhidinsky district (SNIPPET-VERIFIED, wording not verified (Item 1)) | The shop (separately): the page copy carries no copyright or reuse notice; its terms are UNREACHABLE. The datsan (separately): the poster prints none (read on `overview.png`). | The shop: Russia, St Petersburg (INFERRED from the warehouse address); the datsan: Russia, Buryatia (the poster) | whether the poster's datsan is the Khoymorsky one; the shop's terms; the datsan's own site |
| `kalmyk-central-khurul/` | «Центральный хурул «Золотая обитель Будды Шакьямуни»», Элиста; "Республика Калмыкия, г. Элиста, ул. Юрия Клыкова, 63" (`page.txt` lines 28, 30, 34). Compiler on the print: "Анджа-гэлюнг". No ИНН/ОГРН found | The body it belongs to: **UNDETERMINED.** FOR (a Sangha link): nothing. AGAINST: ru.wikipedia's Sangha article says the Sangha has "не имеет значительного контроля" over Kalmykia and Tuva (FETCHED, secondary, wording not verified (Item 1)). No page naming a Kalmyk body was found | Page, line 34: "© 2026 Центральный хурул «Золотая обитель Будды Шакьямуни»". The 2025 and 2026 calendars print at the foot a line that begins "©" and names the khurul complex (read on the overview images, small print, INFERRED). khurul.ru UNREACHABLE | Russia: Kalmykia | which Buddhist body of Kalmykia, if any; the register entry |
| `rabten-choeling/` | Rabten Choeling, Centre for Higher Tibetan Studies, CH-1801 Le Mont-Pèlerin (the PDF's last page). The editorial page (FETCHED 2026-11-12, wording not verified (Item 1)): "Legal form: association, commercial register (Switzerland): CH-550-0167454-3"; address "Chemin du Dérochoz 2, 1801 Le Mont-Pèlerin". The UID CHE-107.951.227, "Rechtsform Verein" is from a search result of an aggregator (onlineinfo.ch), SNIPPET-VERIFIED | The body it belongs to: **the Rabten network** of monasteries and study centres, named on the calendar's own pages (FETCHED through `pdftotext`: "all the Rabten monasteries and study centres, as well as the Edition Rabten publishing service"). No Russian Sangha link found. The register (Zefix) not reached | The PDF's text layer: no notice (80,030 bytes; the pages not read as images). Site footer (FETCHED, wording not verified (Item 1)): "© 12.11.26 Rabten Choeling". The calendar page says nothing on reuse | Switzerland: Le Mont-Pèlerin (canton of Vaud, INFERRED from the postal code) | the commercial-register extract (the identifier to search is in the link table) |

**Item 2(d), «Ринпоче Багша», each part of the advisor's reading checked.**
(1) The About page names that legal entity and ИНН 0323082410: MEASURED, `o-datsane.txt` lines 112–113, 118 and 136 (line 136 writes «Буддийский центр "Ринпоче-багша"», with a hyphen). (2) The name lacks the words "Буддийской традиционной Сангхи России": MEASURED. (3) The page never names the Sangha: MEASURED, the grep above prints nothing. (4) The expectation itself: the legal names of two datsans a source puts in the Sangha carry no such words either: «Местная религиозная организация буддистов Иволгинский дацан "Хамбын Хурээ"» (an MChS certificate, SNIPPET-VERIFIED) and «Местная религиозная организация буддистов «Загустайский дацан "Дэчен Рабжалинг"»» (a Ministry of Justice report, ОГРН 1120327016590, ИНН 0318034121, SNIPPET-VERIFIED; that the Zagustaysky datsan belongs to the Sangha is a search result's summary of a Wikipedia page, wording not verified). So the expectation is **not supported** by these two names: a local religious organisation's name, in these two cases, does not carry the Sangha's name. That does not show the Rinpoche Bagsha organisation is or is not a member. A search for a source placing it in either body (the Sangha or «Объединение буддистов Бурятии») found none naming it (SNIPPET-VERIFIED: the one phrase about "Не входит в Буддийскую традиционную сангху России; относится к … «Объединение буддистов Бурятии»" concerns another monastery, as the result itself says).

### 2. Item 1: the control on WebFetch

- Library text, `datsan-rinpoche-bagsha/datsan-about/o-datsane.txt` line 137 (MEASURED 2026-11-12 by `sed -n '137p'`): «Любое использование либо копирование материалов или подборки материалов сайта, элементов дизайна и оформления допускается лишь с письменного разрешения правообладателя и только со ссылкой на источник: www.datsan-rinpoche-bagsha.ru».
- WebFetch of https://datsan-rinpoche-bagsha.ru/o-datsane: no text came back. The tool returned `connect ECONNREFUSED 176.57.65.16:443`. No second attempt (rule 50).
- Result: **the control did not run**, so no script comparison was made. STOP RULE (2) applies: every WebFetch quotation is "wording not verified (Item 1)", and the page is in the link table.
- What the control cannot show, even when it runs: that WebFetch is faithful on a page it has not already been checked on. This one never ran. Every WebFetch rendering in this report stands unchecked, so none of them is used as the only basis of a claim that matters: the memberships rest on library prints and are UNDETERMINED, and the statutes rest on snippets and are not firm.

### 3. Item 4: the law, by country

**4(a), 4(c) first.** The governing law of each holding's terms, as far as the terms or the seat show it, is in the "Law" column above: Russia for six holdings, Switzerland for Rabten Choeling. No terms page of any holding names a governing law or a jurisdiction (MEASURED: none of the quotations above does). What the project would take from each calendar (001 §6.2, `docs/sources.md`): the Gregorian date, the tshes number, the month, the leap-month and doubled/skipped marks; never a calendar's text, images or design. That is stated beside the provisions; no conclusion follows from it (STOP RULE (4)).

**4(b), Russia.** The official text (pravo.gov.ru, the Civil Code part 4, `nd=102110716`) could not be fetched ("Command failed with no output"). Non-official hosts were refused (legalacts.ru, gkodeksrf.ru, rulaws.ru). What exists is search-result text, SNIPPET-VERIFIED, from non-official hosts, wording not verified (Item 1):

| Provision | What was found | Label |
|---|---|---|
| Art. 1259 п. 5 | "Авторские права не распространяются на идеи, концепции, принципы, методы, процессы, системы, способы, решения технических, организационных или иных задач, открытия, факты, языки программирования, геологическую информацию о недрах." (quoted by a search result from gkodeksrf.ru) | SNIPPET-VERIFIED, not official, version date not shown |
| Art. 1259 п. 6 | a search result says it excludes official documents, symbols, folk works and reports of events of an informational nature; it is a paraphrase, not a quotation | SNIPPET-VERIFIED, paraphrase |
| Art. 1334 | the maker of a database whose creation took substantial costs has the right to extract and use its materials; a database of at least 10,000 independent materials is presumed to have required them; the right exists independently of rights in the materials and in the database as a composite work (paraphrase of search results from gkodeksrf.ru, legalacts.ru and others) | SNIPPET-VERIFIED, paraphrase, not official |
| Art. 1335 | a lawyer's analysis in a search result says protection runs 15 years, renewed on update; the article's own text was not seen | SNIPPET-VERIFIED, secondary |
| Arts. 1333, 1335.1, 1336 | no text found. A search result's section listing names 1334, 1335 and 1335.1 and not 1336, which this round could not confirm | not found; 1336 unconfirmed |
| Art. 1260 (compilations), art. 1274 (quotation), unfair competition (Law 135-FZ) | not searched | RECALLED / UNVERIFIED that they exist; nothing about their text |

Found **for** reuse (the leg's own rule): art. 1259 п. 5, as quoted, excludes facts and methods from copyright. Found **against** reuse: art. 1259 п. 1's protection of works generally was not fetched; art. 1334's database right exists as a category, and the presumption of substantial costs begins at 10,000 materials (INFERRED, arithmetic only: a yearly calendar lists some 365 days, far fewer; whether any substantial-cost claim could still be made was not found). Nothing about a publisher's written-permission clause as a matter of law was found.

**4(b), Switzerland.** fedlex.admin.ch's page for SR 231.1 needs JavaScript and returned no text. Two official PDFs (SR 231.1 at res.cloudinary.com/adminch, "Bundespublikationen", signed 2026-05-07; SR 241 at fedlex.data.admin.ch, version 20250101, whose bookmarks name "Art. 2 Grundsatz" and "Art. 5 Verwertung fremder Leistung") came back from WebFetch as unreadable bytes, and the harness saved each at a path outside this round's grants. STOP RULE (7): they were not read, and no other tool or route was tried. What exists:

| Provision | What was found | Label |
|---|---|---|
| CopA (URG) Art. 2 para. 1 | "Werke sind, unabhängig von ihrem Wert oder Zweck, geistige Schöpfungen der Literatur und Kunst" [the sentence continues "die individuellen Charakter haben", per the rendering] | FETCHED from www.wipo.int/wipolex/fr/text/475781, German, "stand am 23. März 2004" (an old version, not official, wording not verified (Item 1)) |
| CopA Art. 4 | "Sammlungen sind selbständig geschützt, sofern es sich bezüglich Auswahl oder Anordnung um geistige Schöpfungen" [continues "mit individuellem Charakter handelt"] | same, 2004 version |
| CopA Art. 5 para. 1 | "Durch das Urheberrecht nicht geschützt sind:" with item a, "Gesetze, Verordnungen, völkerrechtliche Verträge und andere amtliche Erlasse" | same, 2004 version |
| CopA Art. 25 para. 1 | "Veröffentlichte Werke dürfen zitiert werden, wenn das Zitat zur Erläuterung, als Hinweis oder zur Veranschaulichung dient" [continues with a proportion requirement] | same, 2004 version |
| UWG (SR 241) Art. 5 lit. c | "handelt insbesondere unlauter, wer das marktreife Arbeitsergebnis eines andern ohne angemessenen eigenen Aufwand durch technische Reproduktionsverfahren als solches übernimmt und verwertet" (quoted by a search result) | SNIPPET-VERIFIED, wording not verified (Item 1), version not shown |
| No right in a database as such | three commentary pages say Switzerland has no sui generis database right and databases are protected by copyright, if original, or by unfair competition law (primerus.com, legal500.com, swlegal.com) | SNIPPET-VERIFIED, secondary: not a statute |

Found **for** reuse: Art. 4's protection attaches to the selection or arrangement of a collection when it is an intellectual creation of individual character, and the commentaries read no separate database right; Art. 5 para. 1 excludes official acts. Found **against** reuse: Art. 5 lit. c UWG is a rule of unfair competition that a publisher could invoke against taking a market-ready work result without adequate own effort (the search result says the Federal Supreme Court reads it narrowly, citing BGE 131 III 384, not verified); Art. 25 as found is the quotation provision and its conditions were not read through. The 2004 version of the CopA is 22 years old; a later amendment is not excluded.

**What is not met:** Item 4(b)'s "FETCHED, from an official publication" is met for **no** provision. The human saves the official texts into the library (link table), and a later round reads them with `pdftotext`.

### 4. The four legs (003 §4, P3)

1. **① Reference** (datsan publications; a зурхайч where reachable): Items 2 and 3 above. A зурхайч was not approached (STOP RULE (5); F4, B-3). What it found against the approach: nothing makes any holding a member (all UNDETERMINED), and the terms found are a footer of one datsan restricting use "лишь с письменного разрешения" and the Gunzechoinei datsan's attribution line.
2. **② Field** (other implementations): searched "Tibetan calendar software calendar data licence terms reuse Lotsawa House Henning TCG Mongolian zurkhai app data copyright". Found: Lotsawa House's texts are under CC BY-NC 4.0 and its metadata under CC0, and its calculator page calls its conversion code Free Software and cites Janson (SNIPPET-VERIFIED, wording not verified); nothing about Henning's TCG, a Mongolian app, or any datsan calendar's data. Relevant to this round's question: none. (The repository already holds Henning's licence text, `docs/sources.md`.)
3. **③ Science** (Janson, Henning, Schuh, Reingold–Dershowitz): searched "Janson Henning Schuh Reingold Dershowitz Tibetan calendar tables copyright of calendar dates facts reuse". Found no statement on whether converted dates or tables are protected (SNIPPET-VERIFIED); the one relevant note is that Reingold–Dershowitz's code (CALENDRICA 4.0) is Apache 2.0 and the book carries a standard notice. "None relevant" to the question of reusing a Buryat calendar's facts.
4. **④ Does this contradict anything already shipped?** No contradiction with `docs/sources.md` as it stood, with F22 (the round changes neither its row nor its ruling), or with F28 to F31. One tension is reported beside F22, not resolved (STOP RULE (3)): `docs/sources.md` already named the Khoymorsky poster "Хойморский дацан «Дэчен Равжалин»"; Wikipedia's Sangha article names a Khoymorsky datsan «Бодхидхарма», so the identity of the poster's datsan is unsettled (D-32). F29 and F30 each wait on the terms check of 001 §8: the check found the Aginsky PDFs print no notice and the Ivolginsky PDF's footer is unreadable, and the two sites are unreachable; the admission stays the human's.

### 5. The rulings R1 to R7, each beside its record

- **R1** (the human: "lets make web search round to find out what is jurisdiction"; then "Agree with all recommendations, go"): this round; the roadmap row is `docs/ROADMAP.md`'s source-checks row (r3). «Ринпоче Багша» stays as F22 rules it; the round did not touch F22.
- **R2** (the human: "Lets add them to source, like for Rinpoche Bagsha make a web search for it"): `docs/sources.md` has a new row for `rabten-choeling/calendarEN.pdf` and the Kalmyk row's Trusted-for cell is updated; DEBT D-29 is PAID. No golden row.
- **R3** (the human: "1 agree"): FORKS F28; DEBT D-31 (001 is gated).
- **R4** (the human: "2 agree; 3 agree"): FORKS F29 (Ivolginsky) and F30 (Aginsky). Each admission waits on the terms check, whose findings are in §1.
- **R5** (the human: "4 agree"): FORKS F31.
- **R6** (round 011's Latitude 1–7, "Agree with all recommendations, go merge"): accepted as reported; no grant after the fact. Recorded here only.
- **R7** (round 011's flags, same words): DEBT D-30, with file and line re-measured: `CLAUDE.md:22`, `docs/ROUNDS.md:108`, `docs/ROUNDS.md:129`, `CLAUDE.md:48`, `docs/ROUNDS.md:222`.

### 6. The link table (Item 6)

Pages the human should open or save into the library. The round writes nothing into the library. "UNREACHABLE" is this round's attempt, one per host.

| URL | What it is for | What to search or save | Item |
|---|---|---|---|
| https://datsan-rinpoche-bagsha.ru/o-datsane | Item 1's control page; UNREACHABLE (ECONNREFUSED) | save the page; its footer sentence is line 137 of the library copy | 1, 3 |
| https://egrul.nalog.ru/ | the tax service's register of legal entities; UNREACHABLE (ECONNREFUSED), not a form this round passed | search ИНН 0323082410 («Ринпоче Багша»); and by name «Иволгинский дацан "Хамбын Хурээ"», «Агинский дацан «Дэчен Лхундублинг»», «Гунзэчойнэй», «Дэчен Равжалин», «Центральный хурул» (no ИНН is known for them) | 2 |
| https://nco.minjust.gov.ru/ | the Ministry of Justice's register and reports of non-commercial and religious organisations (the host appeared in search results; UNREACHABLE) | the same names and ИНН; the register's own address is RECALLED / UNVERIFIED | 2 |
| the Sangha's own list of datsans | no address of the Sangha's own site was found | save any page of the Sangha's that lists its datsans | 2 |
| https://ru.wikipedia.org/wiki/Буддийская_традиционная_сангха_России | secondary; its navigation lists the Aginsky, Khoymorsky, Rinpoche Bagsha and Gunzechoinei datsans, and it says the Sangha has no significant control over Kalmykia (rendering, wording not verified) | save the page | 2, 4 |
| https://ivolgdatsan.ru/ | the Ivolginsky site's footer and terms; UNREACHABLE | save the home page and any terms page; also a legible crop of the calendar's footer line | 2, 3 |
| https://aginskydatsan.ru/ | the Aginsky site's about page, footer, terms; UNREACHABLE | save the same | 2, 3 |
| https://dazanspb.ru/ and the three news URLs in `docs/sources.md` | the Gunzechoinei footer "© Дацан Гунзэчойнэй 2026" and its attribution line (a ruling may rest on it); UNREACHABLE ("Command failed with no output") | save the home page and the footer of one news page | 2, 3 |
| https://khurul.ru/materials/buddijskij-kalendar | the Kalmyk khurul's footer and any body it names; UNREACHABLE | save it, and its «О монастыре» page | 2, 3 |
| https://dharma.ru/page/oferta | the shop's terms; UNREACHABLE | save it | 3 |
| http://pravo.gov.ru/proxy/ips/?doc_itself=&infostr=xO7q8+zl7fIg7vLu4fDg5uDl8vH%2FIO3lIOIg7+7x6+Xk7eXpIPDl5ODq9ujo&nd=102110716&page=1&rdk=1 | the Civil Code part 4, official; UNREACHABLE | save arts. 1259, 1260, 1274, 1333 to 1336 with the edition date | 4 |
| https://www.fedlex.admin.ch/eli/cc/1993/1798_1798_1798/en | SR 231.1; needs JavaScript | save arts. 2, 4, 5, 25 with the "Stand am" date (a PDF, so `pdftotext` can read it) | 4 |
| https://res.cloudinary.com/adminch/image/private/s--ga863q07--/v1719842041/Bundespublikationen/231-1d.pdf | SR 231.1 in German, fetched but not readable by WebFetch | save the file | 4 |
| https://fedlex.data.admin.ch/filestore/fedlex.data.admin.ch/eli/cc/1988/223_223_223/20250101/de/pdf-a/fedlex-data-admin-ch-eli-cc-1988-223_223_223-20250101-de-pdf-a-1.pdf | SR 241 (UWG), version 20250101 | save the file | 4 |
| https://www.rabten.eu/en/page/editorial | Rabten Choeling's legal form and commercial-register number (a ruling may rest on it) | save the page | 2, 3 |
| https://www.rabten.eu/en/page/iDevCalendar and https://www.rabten.eu/downloads/calendarEN.pdf | the calendar page and the PDF's published URL | save both | 3, 5 |
| https://www.zefix.ch/ | the Swiss commercial register (a search form; not attempted: RECALLED / UNVERIFIED address) | search "Rabten Choeling"; CHE-107.951.227 (an aggregator's UID) and CH-550-0167454-3 (the site's number) | 2 |

### 7. Properties

All tasks and the property list:

| Item | State | Evidence |
|---|---|---|
| 0: branch, prompt, rows | DONE | branch cut at c4af28d; prompt `cmp` printed CMP-OK before each commit; F28 to F31, D-30, D-31, ROADMAP row (diff shown in the session) |
| 1: the control | DEFERRED (did not run) | `connect ECONNREFUSED 176.57.65.16:443` |
| 2: publishers and membership | DONE for (a), (b), (d); (c) UNDETERMINED for all seven | §1 |
| 3: the terms | DONE where reachable; the live sites of five holdings UNREACHABLE | §1 |
| 4: the law | (a), (c) DONE; (b) DEFERRED: no provision FETCHED from an official publication | §3 |
| 5: `docs/sources.md` | DONE | new Rabten row; six rows' licence and Source cells; D-29 PAID |
| 6: the link table | DONE | §6 |
| 7: the close | DONE | §8 (gate), the squash message below; the final commit changes the three paths named in §8 |

### 8. The gate (Item 7)

`tools/gate.sh` ran once, in the background, on the clean committed tree at `0d61fef0f8e49ef2c40cb79693bbdef69b96eec5` (`git status --porcelain` printed nothing before it and after it), the last commit that changes more than this file, `docs/specs/INDEX.md` and `docs/ROADMAP.md`. MEASURED from its log: `GATE: 11 legs run — 11 passed, 0 failed, 1 skipped`; the sums of the `test result:` lines are 147 passed and 0 failed in the debug leg, 147 and 0 in the release leg, and 147 and 0 in the msrv leg; the self-test leg printed `26 plants — 26 rejected with their named diagnostic, 0 failures`. The skipped leg is wasm-pack (`crates/tshes-wasm` does not exist, 002 §7). Spec 011's recorded count is 147 / 147 (RECALLED, from its head block): **equal**. The final commit should differ from `0d61fef` in the three paths `docs/specs/012-source-checks.md`, `docs/specs/INDEX.md` and `docs/ROADMAP.md` only; `git diff --name-only 0d61fef <the tip>` cannot be written into the commit it describes, so it is run after the final commit and shown in the session's closing message. No reviewer runs at this close: the round changes neither `crates/` nor `golden/` (`docs/ADVISOR.md` §6).

### 9. Diff scope (contract item 5)

Measured at `0d61fef` by `git diff --numstat main 0d61fef`: DEBT.md +24 −0; FORKS.md +32 −0; docs/ROADMAP.md +4 −2; docs/sources.md +7 −6; docs/specs/012-source-checks.md +169 −0; docs/specs/prompts/012-source-checks.md +385 −0. Sum: 24 + 32 + 4 + 7 + 169 + 385 = 621 insertions, and 2 + 6 = 8 deletions, which equals `git diff --stat main 0d61fef`, "6 files changed, 621 insertions(+), 8 deletions(-)". The final commit adds `docs/specs/INDEX.md` (one row), changes this file and sets the ROADMAP row to LANDED, so the tip touches **7 files**. Gated paths touched: **no** (this round's own new spec file, its committed prompt and its INDEX row are not changes to an accepted spec, `GATED.md`). Files the test framework wrote: **none** (`git status --porcelain` printed nothing after the gate). `crates/`, `golden/`, `rules/`, `clients/`, `tools/`, `.github/`, `docs/METHOD.md`, `GATED.md`, `CLAUDE.md`, `docs/ADVISOR.md`, `docs/ROUNDS.md` and `docs/BACKLOG.md`: unchanged.

### 10. The predictions, scored

| # | Prediction | Score | Measured value |
|---|---|---|---|
| 1 | Every PRECONDITION and ARTIFACT check passes | HELD | `status --porcelain` printed nothing, HEAD `main`, `rev-parse main` printed `c4af28d0214bb035b2d5252d9570234544254174`, `ls` of the library succeeded, `sha256sum` of `calendarEN.pdf` printed `13aa4256…2898`, line 137 read |
| 2 | Item 1's control holds | FAILED | the control did not run: `connect ECONNREFUSED 176.57.65.16:443` |
| 3 | The tax register cannot be searched by WebFetch and goes into the link table | HELD for the outcome, not the reason | `egrul.nalog.ru` returned `connect ECONNREFUSED 213.24.64.183:443`; the prediction's reason (a form or a captcha) was not what happened, because no page came back; it is in the link table |
| 4 | Ivolginsky, Aginsky and Gunzechoinei each shown a member by a primary source or a Sangha page | FAILED | none was shown: no register entry and no Sangha page was reached; all three are UNDETERMINED (the Aginsky print and the Ivolginsky print are the publishers' own statements, which Item 2(c) ranks below a register and the Sangha's list) |
| 5 | «Ринпоче Багша» ends UNDETERMINED or NOT A MEMBER; no source names it a member | HELD | UNDETERMINED; no source found names it a member (the Wikipedia article on the Sangha names it only in a navigation list, FETCHED, wording not verified (Item 1)) |
| 6 | The Central Khurul belongs to a Buddhist body of Kalmykia, not to the Sangha | FAILED | not shown: no page naming a Kalmyk body was found; the only item is Wikipedia's remark that the Sangha has no significant control over Kalmykia |
| 7 | ivolgdatsan.ru and aginskydatsan.ru each carry a notice about use of the site's material; neither PDF prints one | FAILED | the first half is unmeasured (both sites UNREACHABLE); the second half held in part: the Aginsky PDFs print none (`pdftotext`, grep), the Ivolginsky PDF's text layer holds none and its footer line is unreadable in the library's image |
| 8 | Art. 1259 found and quoted with a clause that facts are not objects of copyright; Swiss law has no right in a database as such | HELD on SNIPPET-VERIFIED evidence only | p. 5 quoted from a search result of gkodeksrf.ru (not official); the Swiss statement rests on three commentary snippets, not on a statute; no official text FETCHED (§3) |
| 9 | OPEN is 5 at the base and at the final commit; new rows F28 to F31; new DEBT entries D-30 and D-31; D-29 PAID | HELD in part, FAILED in part | OPEN is 5 at the base and at every commit (§11); F28 to F31; D-29 PAID; the new DEBT entries are **D-30, D-31 and D-32** (D-32 is this round's by-catch) |
| 10 | The diff touches 7 files | HELD | §9: the 6 files at `0d61fef` plus `docs/specs/INDEX.md`; the tip's name-only list is shown in the session's closing message (§8) |
| 11 | The gate's count equals spec 011's | HELD | 147 / 147, §8 |

### 11. The open-forks audit and the marker greps, at every commit

The audit is `expected = previous OPEN + forks raised + gated findings filed with options and none chosen − rulings recorded` (METHOD rule 60), measured with `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`. The round's marker is a token written nowhere in the tree; the grep is `git grep` for it over everything but the frozen prompt copy, and the round writes no placeholder, so each grep must print nothing. Control (rule 31): the same `grep -c` over a scratch file that holds the marker printed 1.

| Commit | Previous OPEN | Raised | Filed with options, none chosen | Rulings recorded | Expected | Measured OPEN | Marker grep |
|---|---|---|---|---|---|---|---|
| 85084c3 (the prompt) | 5 (base, measured) | 0 | 0 | 0 | 5 | 5 | no output |
| 0c88d05 (Item 0) | 5 | 0 | 0 | 0 (F28 to F31 are filed already RULED: a row born RULED neither raises nor records) | 5 | 5 | no output |
| 0d61fef (Item 5) | 5 | 0 | 0 | 0 | 5 | 5 | no output |
| the final commit | 5 | 0 | 0 | 0 | 5 | 5 | no output |

The OPEN rows are F4, F5, F7, F8 and F10 (the status lines read at the start). The base count the advisor relayed was 5; this round measured 5.

### 12. The remaining contract sections

**Prompt questions answered.** Every item of the prompt is answered above: Item 1 in §2, Items 2 and 3 in §1, Item 4 in §3, Item 5 in the `docs/sources.md` diff and D-29, Item 6 in §6, Item 7 in §8 and §9. The four legs are in §4.

**The stop rule that governed the round:** STOP RULE (2): Item 1's control did not run (§2). STOP RULE (7) also bound once: two PDFs that WebFetch saved outside the grants were not read.

**Forks encountered, resolved by the round.** (1) Whether to read the two PDFs that WebFetch saved under the harness's directory: considered reading them with `pdftotext`, which would have given the Swiss provisions; chose not to, because that directory is outside GRANTS and STOP RULE (7) says never to work around it; they go in the link table instead. (2) Whether to follow the server-supplied redirect from admin.ch to fedlex.admin.ch and on to fedlex.data.admin.ch: considered stopping, chose to follow the two redirects once each, because they are the official host the prompt names for the statute. (3) Whether to file the unreached sources as a DEBT entry: considered leaving them in the prose, chose D-32, because CLAUDE.md hard rule 13 says prose is not filing. No STOP AND ASK was needed: none of the three changes what the project means.

**Latitude taken beyond the prompt.** (1) `docs/ROADMAP.md`'s Status line goes from r2 to r3, and from r3 to r4 at the final commit, beside the Revisions lines, so that the file does not contradict its own entry (hard rule 19). (2) D-32 was filed, which the prompt did not list (prediction 9 scores it). (3) The prompt names no search; twenty-two were run (listed in §13) to find the Sangha's list, the registers' entries, the statute texts and the legs ② and ③. (4) The Kalmyk calendars' footers were read on the library's overview images, and the Khoymorsky poster's panel on its overview image, for Item 3's "for images, read them"; the Ivolginsky overview image and its footer crop were looked at too, and its footer stayed unreadable (no grant to enlarge it: `pdftoppm` is not in GRANTS). The Aginsky PDFs were read through their text layers only, and their images were not viewed.

**Totals that must sum.** §9 (621 insertions, 8 deletions, 6 files at `0d61fef`, 7 at the tip); §8 (147 + 0 in each of three profiles); §11 (OPEN 5 at every commit); the seven holdings are 5 datsan holdings (Rinpoche Bagsha, Ivolginsky, Aginsky, Gunzechoinei, Khoymorsky) + the Kalmyk khurul + Rabten Choeling = 7.

**What the prompt got wrong.**
1. Item 2(c) and prediction 3 expect the registers to need "a form, a login or a captcha": the registers did not answer at all (ECONNREFUSED), so there was nothing to pass; and the prompt's whole membership ladder rests on reaching a register or the Sangha's list, which this machine could not do (D-32).
2. Item 4(b) expects the official statute texts to be FETCHED: fedlex's page needs JavaScript and its PDFs came back as unreadable bytes saved outside the grants; pravo.gov.ru did not answer. The prompt had no route for a binary result.
3. Item 4(b) names "articles 1333 to 1336": a search result's listing of the chapter's section names 1333 to 1335.1 and not 1336; this round could not confirm what 1336 is (UNVERIFIED).
4. The prompt's `GRANTS:` does not include `pdftoppm`, so the Ivolginsky footer (the one print that may carry the Sangha's name) could not be enlarged; the prompt does not say how a too-small print is to be read.
5. Prediction 2 and the Item 1 control assume the site answers the tool; the control had no fallback for a refused connection beyond STOP RULE (2).
6. Item 0 asks for a Revisions line "with the old TRIGGER text": done; the prompt did not say the file's Status line must change with it (latitude 1).
Nothing else was found wrong in the prompt's dictated rows: F28 to F31, D-30 and D-31 were written as given, and the ids the advisor expected (F28 to F31, D-30, D-31) were the next free ones in the tree.

**By-catch.** (1) The reach pattern: all twelve Russian-hosted sites tried (the ten that returned ECONNREFUSED and the two that returned no output, §0) failed, and every other host answered except lawbrary.ch (HTTP 403); ru.wikipedia.org answered (D-32). (2) The Khoymorsky identity discrepancy (D-32). (3) A search result's INN for the St Petersburg datsan is 12 digits (121521717700), which is not an organisation's form; it is recorded nowhere (not an identifier); the legal entity's name stays unknown (D-32). (4) The Ivolginsky calendar's compiler is "Ширээтэ лама Ацайского дацана" (`docs/sources.md`), a different datsan from the publisher; whether the Ivolginsky datsan is the print's publisher or only its distributor is not shown by anything read (no DEBT id; it is the human's to ask in the Sources round, F29's terms check).

**Debt and forks.** Filed: F28, F29, F30, F31 (RULED); D-30, D-31, D-32 (OPEN). Paid: D-29. Updated: none. Audit: §11. Marker greps: §11.

**Held** (identified items only): the unreached sources (D-32); the clarification of 001 §6.3 (D-31); round 011's flags (D-30).

### 13. The searches and fetches, and the commands

**Fetches (WebFetch), one line each: URL — host — result.**
1. https://datsan-rinpoche-bagsha.ru/o-datsane — datsan-rinpoche-bagsha.ru — ECONNREFUSED 176.57.65.16:443 (the control)
2. https://ivolgdatsan.ru/ — ivolgdatsan.ru — ECONNREFUSED 45.130.41.254:443
3. https://aginskydatsan.ru/ — aginskydatsan.ru — ECONNREFUSED 5.45.80.36:443
4. https://dazanspb.ru/ — dazanspb.ru — "Command failed with no output"
5. https://khurul.ru/materials/buddijskij-kalendar — khurul.ru — ECONNREFUSED 200.165.232.98:443
6. https://ru.wikipedia.org/wiki/Буддийская_традиционная_сангха_России — ru.wikipedia.org — a rendering; no Sangha membership statement for five of the six datsans; the Ivolginsky datsan as the residence
7. https://www.rabten.eu — www.rabten.eu — a rendering; footer "© 12.11.26 Rabten Choeling"; editorial and data-protection links
8. https://www.fedlex.admin.ch/eli/cc/1993/1798_1798_1798/en — www.fedlex.admin.ch — a notice that JavaScript is required; no statute text
9. https://www.rabten.eu/en/page/editorial — www.rabten.eu — legal form association, "CH-550-0167454-3", address Chemin du Dérochoz 2
10. https://egrul.nalog.ru/ — egrul.nalog.ru — ECONNREFUSED 213.24.64.183:443
11. https://nco.minjust.gov.ru/api/public/nko-reports/archive/70f47d25-4a37-4fef-9001-89012ca4bb8c/download — nco.minjust.gov.ru — ECONNREFUSED 188.254.16.6:443
12. http://pravo.gov.ru/proxy/ips/?doc_itself=…&nd=102110716&page=1&rdk=1 — pravo.gov.ru — "Command failed with no output"
13. https://res.cloudinary.com/adminch/image/private/…/231-1d.pdf — res.cloudinary.com — a signed PDF as bytes, unreadable by the renderer; saved by the harness outside the grants, not read
14. https://legalacts.ru/kodeks/GK-RF-chast-4/razdel-vii/glava-70/statja-1259/ — legalacts.ru — ECONNREFUSED 213.159.208.105:443
15. https://lawbrary.ch/law/art/URG-v2023.07-en-art-4 — lawbrary.ch — HTTP 403
16. https://dharma.ru/page/oferta — dharma.ru — ECONNREFUSED 5.189.239.92:443
17. https://www.rabten.eu/en/page/iDevCalendar — www.rabten.eu — "Calendar download: Tibetan Calendar (pdf)", link to `calendarEN.pdf`, footer "© 12.11.26 Rabten Choeling • Editorial • Data protection"
18. https://www.wipo.int/wipolex/fr/text/475781 — www.wipo.int — the CopA in German, "stand am 23. März 2004"; Arts. 2, 4, 5, 25 present
19. https://gkodeksrf.ru/ch-4/rzd-7/gl-70/st-1259-gk-rf — gkodeksrf.ru — ECONNREFUSED 87.236.16.155:443
20. https://rulaws.ru/gk-rf-chast-4/Razdel-VII/Glava-71/paragraph-5/ — rulaws.ru — ECONNREFUSED 82.202.247.11:443
21. https://www.admin.ch/opc/de/classified-compilation/19860391/201407010000/241.pdf — www.admin.ch — a 301 to fedlex.admin.ch (not auto-followed)
22. https://www.fedlex.admin.ch/redirect?url=… — www.fedlex.admin.ch — a 301 to fedlex.data.admin.ch
23. https://fedlex.data.admin.ch/filestore/…/fedlex-data-admin-ch-eli-cc-1988-223_223_223-20250101-de-pdf-a-1.pdf — fedlex.data.admin.ch — a PDF as bytes, unreadable by the renderer; bookmarks "Art. 2 Grundsatz", "Art. 5 Verwertung fremder Leistung"; saved by the harness outside the grants, not read

**Searches (WebSearch, standard mode), one line each.**
1. "Буддийская традиционная сангха России список дацанов Иволгинский Агинский Гунзэчойнэй Хоймор" — a summary: no full list; Ivolginsky as the Sangha's centre
2. "Rabten Choeling Tibetan Calendar 2026 pdf calendarEN rabten.eu" — no 2026 PDF found
3. "fedlex.admin.ch Bundesgesetz über das Urheberrecht … SR 231.1 pdf" (allowed domain fedlex.admin.ch) — no links
4. "Гражданский кодекс статья 1259 … pravo.gov.ru" — art. 1259 p. 5 and 6 as summaries from legalacts.ru, gkodeksrf.ru and others
5. "ИНН 0323082410 Буддийский центр Ринпоче Багша Улан-Удэ" — no source tying the ИНН to the centre; a Ministry of Justice download URL
6. "Rabten Choeling Fondation Mont-Pèlerin registre du commerce CHE fondation Rabten" — onlineinfo.ch: UID CHE-107.951.227, Verein
7. "Гражданский кодекс Российской Федерации часть четвертая 230-ФЗ официальный текст" (allowed domains pravo.gov.ru, publication.pravo.gov.ru) — the pravo.gov.ru URL with `nd=102110716`
8. "Bundesgesetz … Art. 2 individuellen Charakter Art. 25 Zitate SR 231.1" — droit-bilingue.ch, wipo.int, lawbrary.ch, the cloudinary PDF
9. "Хойморский дацан Дэчен Равжалин Тункинский район религиозная организация …" — the Khoymorsky name discrepancy
10. "Центральный хурул … ЦДУБК" — nothing naming a Kalmyk body
11. "Иволгинский дацан «Хамбын Хурэ» Местная религиозная организация буддистов ИНН ОГРН …" — the MChS certificate's name
12. "Агинский дацан «Дэчен Лхундублинг» религиозная организация ИНН ОГРН …" — another organisation's numbers (Zagustaysky), not used for Aginsky
13. "Санкт-Петербургский дацан Гунзэчойнэй религиозная организация «Буддийская община» ИНН ОГРН …" — no ОГРН; a 12-digit ИНН not recorded
14. "Rabten Choeling Tibetan Calendar download rabten.eu …" (allowed domains rabten.eu, rabten.ch, editionrabten.*, apps.rabten.eu) — the PDF's URL `https://www.rabten.eu/downloads/calendarEN.pdf`
15. "Switzerland copyright database protection no sui generis right collections Art. 4 URG …" — commentary snippets
16. "Гражданский кодекс статья 1259 … "Авторские права не распространяются на идеи, концепции, …"" — p. 5 quoted from gkodeksrf.ru
17. "статья 1334 ГК РФ "изготовитель базы данных" … статья 1335 срок действия 1336" — art. 1334, 1335 summaries
18. "Буддийская традиционная сангха России официальный сайт дацаны входящие в состав БТСР список" — no official site
19. "Bundesgesetz gegen den unlauteren Wettbewerb UWG Art. 5 Verwertung fremder Leistung …" — Art. 5 lit. c wording
20. "Ринпоче Багша дацан Бурятия входит в Буддийскую традиционную сангху или Объединение буддистов Бурятии" — nothing naming Rinpoche Bagsha in either
21. "Tibetan calendar software calendar data licence terms reuse Lotsawa House Henning TCG Mongolian zurkhai app data copyright" — leg ②
22. "Janson Henning Schuh Reingold Dershowitz Tibetan calendar tables copyright of calendar dates facts reuse" — leg ③

**Commands (Bash; absolute paths; one `date` run at the start, one later).**
`git -C /home/kitaeza/github/zurkhai status --porcelain`; `git … rev-parse --abbrev-ref HEAD`; `git … rev-parse main`; `ls /home/kitaeza/zurkhai-papers/`; `date`; `sha256sum /home/kitaeza/zurkhai-papers/rabten-choeling/calendarEN.pdf`; `sed -n '137p' …/o-datsane.txt`; `git … switch -c spec/012-source-checks`; `git … rev-parse HEAD^{tree}`; `cmp /home/kitaeza/zurkhai-prompts/spec-012-source-checks.md /home/kitaeza/github/zurkhai/docs/specs/prompts/012-source-checks.md` before each commit; `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`; `git … grep` for the marker; `pdftotext -layout` and `pdfinfo` on the four PDFs into a `mktemp -d` directory, with `grep -n -i -E 'права|копир|использован|©|copyright|лиценз|разреш|licen|reprod|permission|all rights|www\.|http|@|ИНН|ОГРН|регистр'`; `grep -n -i -E 'сангх|Сангха|БТСР' …/o-datsane.txt`; `stat -c … calendarEN.pdf`; `grep -n` for the five flag lines of D-30; `tools/gate.sh` once, output to the scratchpad; `git … diff --numstat main 0d61fef`. No script was written; no `python3` was run; no `curl`, `wget`, `gh` or `git fetch`/`push`/`pull` was run.

## The squash commit message

```
spec 012: source checks: who publishes each calendar in the library, and on what terms

The Sources arc's first round: leg 1 of 003 section 4's P3 for the seven
holdings of the paper library (five Buryat datsans, the Kalmyk Central Khurul,
Rabten Choeling). It transcribes nothing, admits nothing and draws no legal
conclusion.

Findings. The tax and Justice registers, the Sangha's own list and most live
sites could not be reached from this machine (D-32), and WebFetch's control on
datsan-rinpoche-bagsha.ru did not run (ECONNREFUSED). All seven membership
verdicts are therefore UNDETERMINED; the strongest items are the Aginsky PDFs'
own heading "Буддийская Традиционная Сангха России" and the Ivolginsky print's
footer (as best read). Terms found: the Rinpoche Bagsha footer, the
Gunzechoinei attribution line, the Kalmyk (c) lines; the Aginsky PDFs and the
Khoymorsky poster print none; Rabten's PDF text layer holds none. Provisions
of the Russian Civil Code part 4 and the Swiss CopA and UWG were found as
snippets and a 2004 WIPO Lex copy, not from an official publication.

Records. docs/sources.md: new row for rabten-choeling/calendarEN.pdf, and the
six other rows' licence cells and one Source sentence each. FORKS.md: F28 to
F31, filed already RULED (001 6.3's reading; Ivolginsky T1; Aginsky T1 with
both editions; is_leap_month records what the source prints); OPEN stays 5.
DEBT.md: D-29 PAID; D-30 (round 011's four flags), D-31 (001 6.3's clause),
D-32 (the sources not reached) filed OPEN. docs/ROADMAP.md: source-checks row
(r3, then LANDED, r4), buryat-sources' TRIGGER replaced. docs/specs/INDEX.md:
row 012. The prompt is committed byte-identical.

Gate: 147 passed, 0 failed (debug) / 147 passed, 0 failed (release), 11 legs
run, 11 passed, 1 skipped (wasm-pack), on 0d61fef0f8e49ef2c40cb79693bbdef69b96eec5.
No file under crates/, golden/, rules/, clients/, tools/ or .github/ changed.

Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>
```
