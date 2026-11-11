spec: 010
prompt: docs/specs/prompts/010-publication.md
branch: spec/010-publication (cut from main at 56ddcb7, whose tree is 337c44827b8faaee4cee14eb13b1a13b3aa14637)
merge SHA: not yet merged
pushed: no
model: Sonnet 5.5 (claude-sonnet-5-5), effort High (one session so far; the switch point to the fallback model, Opus 5.5, has not been reached and no design choice has fallen to it)
gate: 147 tests passed, 0 failed (debug) / 147 passed, 0 failed (release) — no failure; run once, on the clean committed tree at 723dd38 (11 legs run, 11 passed, 0 failed); the final commit changes docs/specs/010-publication.md and docs/specs/INDEX.md only, which no leg reads; skipped legs: wasm-pack (crates/tshes-wasm does not exist, 002 §7)

Status: draft
Resolves: F11 and F26 (F11 ruled by this round's records, F26 filed already RULED)
Milestone: none, the publication of the repository
Governs: .github/workflows/ci.yml and docs/ROUNDS.md §9

— spec text ends here —

# Report — spec 010 (before the first public push)

Dates come from `date`, run once at the top of the round: 2026-11-12 00:12:05 +0300. The human's rulings carry their own date, 2026-11-11, and keep it. Evidence tags (docs/ROUNDS.md §4.3): MEASURED (this round), RELAYED (the human or the advisor, never re-measured here), INFERRED. The two network lookups of Item 2 are cited **FETCHED**, each with its command and output (METHOD rule 50). No email address outside THE ALLOWLIST appears in this file: an address the audit found outside it is never spelled (there was none on main), and the control's planted address is shown only as the parts it is built from.

## 1. The decisive items

### 1.1 Item 1: the audit of main's history, four checks (MEASURED)

Scope: all 11 commits of main, 134 blobs reachable (cross-check, a count the audit did not produce: `git rev-list --objects main | cut -d' ' -f1 | git cat-file --batch-check='%(objecttype)'` gave 134 blob, 63 tree, 11 commit; the audit's own `git ls-tree -r` walk found 134 unique blobs). Instrument: `audit.py` (last section), the same script that ran on the control (§2.1).

| Check | Count | Result |
|---|---|---|
| (a) email addresses | 4 distinct matches (regex of the prompt), 0 outside THE ALLOWLIST | `noreply@deepseek.com` 21 in files, 1 in commit objects, in 7 commits; `noreply@anthropic.com` 18 in files, 8 in commit objects, in 8 commits; `8291740+kitaeza@users.noreply.github.com` 0 in files, 21 in commit objects, in 11 commits; `noreply@github.com` 0 in files, 1 in commit objects, in 1 commit. No listed match to judge. |
| (b) secrets | 0 hits for each of the six patterns | no FINDING |
| (c) blobs | 1 listed of 134 | `golden/reference/tdc-97d4a00-phugpa-1900-2100.csv`, 1,976,432 bytes (over 1,000,000), no NUL byte, first added by `eb5c760` (spec 006), in main's tip, accounted for by spec 006 (`docs/specs/006-reference-fixtures.md` line 264: "The fixture's size: `golden/reference/tdc-97d4a00-phugpa-1900-2100.csv`: 1,976,432 bytes, 73,415 lines", and its `.meta.yaml`). It is the T4 fixture of 001 §6.1: a column of facts, one row per Gregorian day, generated from a library whose `package.json` says MIT (`docs/sources.md`, the `TibetanDateCalcualtor/` row); it holds no calendar's text or design, so CLAUDE.md hard rule 17 does not reach it. Not a FINDING. No blob contains a NUL byte (0 of 134) and none has a listed extension. |
| (d) paths deleted along main | 1 | `LICENSE`, removed by `0ac42b2` ("Initialize the project documents"); `git show 0ac42b2 --stat` prints `LICENSE => LICENSE-MIT | 0`: a rename with no content change, a 1,064-byte text file, the MIT licence text with the human's GitHub handle as the copyright holder (the initial commit's `LICENSE`, read through `git show a0bca9d:LICENSE`). It held nothing (a), (b) or (c) catches (the audit's own cross-check: 0 of 1 deleted paths caught), and it is the project's own licence file, not third-party material. Not a FINDING. |

**No FINDING in (a)-(d). STOP RULE (2) did not fire.** The audit's output, verbatim:

```
repo commits scanned: 11 (oldest first)

== (a) email addresses ==
distinct matches: 4
  on THE ALLOWLIST: 4; outside it: 0
  allowlisted noreply@deepseek.com: 21 in files, 1 in commit objects, in 7 commits
  allowlisted noreply@anthropic.com: 18 in files, 8 in commit objects, in 8 commits
  allowlisted 8291740+kitaeza@users.noreply.github.com: 0 in files, 21 in commit objects, in 11 commits
  allowlisted noreply@github.com: 0 in files, 1 in commit objects, in 1 commits
  outside-allowlist hits (commit,place,line): 0

== (b) secrets ==
  pattern gh[pousr]_ (36)      hits: 0
  pattern github_pat_ (22+)    hits: 0
  pattern sk- (20+)            hits: 0
  pattern AKIA (16)            hits: 0
  pattern BEGIN..PRIVATE KEY   hits: 0
  pattern xox[abprs]-          hits: 0
  secret hits total: 0

== (c) blobs ==
unique blobs reachable: 134; with a NUL byte: 0
  BLOB 1f4e33877e paths=['golden/reference/tdc-97d4a00-phugpa-1900-2100.csv'] size=1976432 first-commit=eb5c760 in-tip=True why=size>1e6
  listed blobs: 1

== (d) paths deleted along the history ==
  DELETED: LICENSE removed-by 0ac42b2 blob 2d3aaba0d5 size 1064 text; caught by (a)/(b)/(c): False
  deleted paths: 1; of them caught by (a), (b) or (c): 0

SUMMARY a-distinct=4 a-outside-hits=0 b-hits=0 c-listed=1 d-deleted=1
```

How (a) was scanned: `git grep -a -n -F -e @ <commit>` per commit (every address holds an `@`), then the prompt's regular expression on those lines in Python, plus the same regular expression on each commit object (`git cat-file commit`: author and committer lines, any `gpgsig`, the message). The prompt's regular expression run directly through `git grep -E` was quadratic on a very long line and did not finish on the control's 500,000-byte line (the first version of the script; killed, then changed as above). The same prefilter ran on the control and on main.

### 1.2 Item 2: the pin of actions/checkout (FETCHED 2026-11-12)

**(a) The `uses:` lines before the edit** (`grep -rn 'uses:' .github/`, MEASURED): `.github/workflows/ci.yml:29:        uses: actions/checkout@v4` and `.github/workflows/ci.yml:42:        uses: actions/checkout@v4`. It is the only file under `.github/`. D-8 named lines 29 and 42 before the rewrite; they are still 29 and 42. After the edit they are lines 30 and 43 (the header comment grew by one line).

**Route A** (FETCHED, 2026-11-12): `git ls-remote https://github.com/actions/checkout 'refs/tags/v4*'` printed 16 lines and no line ending `^{}`, so every v4 tag is a lightweight tag that points at a commit:

```
11d5960a326750d5838078e36cf38b85af677262	refs/tags/v4
1e31de5234b9f8995739874a8ce0492dc87873e2	refs/tags/v4.0.0
8ade135a41bc03ea155e62e844d188df1ea18608	refs/tags/v4.1.0
b4ffde65f46336ab88eb53be808477a3936bae11	refs/tags/v4.1.1
9bb56186c3b09b4f86b1c65136769dd318469633	refs/tags/v4.1.2
1d96c772d19495a3b5c517cd2bc0cb401ea0529f	refs/tags/v4.1.3
0ad4b8fadaa221de15dcec353f45205ec38ea70b	refs/tags/v4.1.4
44c2b7a8a4ea60a981eaca3cf939b5f4305c123b	refs/tags/v4.1.5
a5ac7e51b41094c92402da3b24376905380afc29	refs/tags/v4.1.6
692973e3d937129bcbf40652eb9f2f61becf3332	refs/tags/v4.1.7
d632683dd7b4114ad314bca15554477dd762a938	refs/tags/v4.2.0
eef61447b9ff4aafe5dcd4e0bbf5d482be7e7871	refs/tags/v4.2.1
11bd71901bbe5b1630ceea73d27597364c9af683	refs/tags/v4.2.2
08eba0b27e820071cde6df949e0beb9ba4906955	refs/tags/v4.3.0
34e114876b0b11c390a56381ad16ebd13914f8d5	refs/tags/v4.3.1
11d5960a326750d5838078e36cf38b85af677262	refs/tags/v4.4.0
```

**Route B** (FETCHED, 2026-11-12): `gh api repos/actions/checkout/git/matching-refs/tags/v4 --jq '.[] | [.ref, .object.type, .object.sha] | @tsv'` printed the same 16 refs, each with `object.type` = `commit` and the same 40 hex digits as Route A (compared line by line: 16 of 16 equal; the two lines that matter are below). The prompt's second step of Route B (`gh api repos/actions/checkout/git/tags/<tag object id>`) applies to an annotated tag and there is none: every `object.type` is `commit`, so there is no tag object to look up, and I made no such call.

```
refs/tags/v4	commit	11d5960a326750d5838078e36cf38b85af677262
refs/tags/v4.4.0	commit	11d5960a326750d5838078e36cf38b85af677262
```

**The releases** (FETCHED, 2026-11-12): `gh api 'repos/actions/checkout/releases?per_page=100' --paginate --jq '.[] | [.tag_name, .draft, .prerelease, .published_at] | @tsv'`, sorted by publication date, trimmed to the v4.* lines and the five newest others (tag, draft, prerelease, published_at):

```
v2.8.0	false	false	2026-07-20T15:43:41Z      (the five newest others: v2.8.0, v3.7.0, v5.1.0, v6.1.0, v7.0.1)
v3.7.0	false	false	2026-07-20T15:40:05Z
v4.4.0	false	false	2026-07-20T15:36:10Z
v5.1.0	false	false	2026-07-20T15:27:58Z
v6.1.0	false	false	2026-07-20T15:23:28Z
v7.0.1	false	false	2026-07-20T15:10:05Z
v4.3.1	false	false	2025-11-17T16:06:48Z
v4.3.0	false	false	2025-08-11T10:32:57Z
v4.2.2	false	false	2024-10-23T14:46:00Z
v4.2.1	false	false	2024-10-07T16:44:53Z
v4.2.0	false	false	2024-09-25T17:52:55Z
v4.1.7	false	false	2024-06-12T19:05:21Z
v4.1.6	false	false	2024-05-16T18:11:28Z
v4.1.5	false	false	2024-05-06T17:39:01Z
v4.1.4	false	false	2024-04-24T13:33:24Z
v4.1.3	false	false	2024-04-19T14:56:44Z
v4.1.2	false	false	2024-03-12T11:22:04Z
v4.1.1	false	false	2023-10-17T15:53:17Z
v4.0.0	false	false	2023-10-17T15:41:17Z
v4.1.0	false	false	2023-09-22T17:42:49Z
```

All 15 v4.x.y tags of Route A are published releases (none a draft, none a prerelease). The listing I printed was cut at 40 lines (`head -40`); the 15 v4.x.y releases are all within it, and Route A's 15 v4.x.y tags equal them one for one.

**(d) The choice.** The highest v4.x.y in numeric order is **v4.4.0**; it is a published release; its commit by Route A and by Route B is `11d5960a326750d5838078e36cf38b85af677262`; the tag `v4` points to the same commit by both routes. No other v4.x.y tag shares that commit (the 15 other ids in the Route A listing are all different), so the tag `v4` points to the commit of exactly one release, the highest. As a third witness that the id is a commit of actions/checkout itself and not of a fork: `gh api repos/actions/checkout/git/commits/11d5960a326750d5838078e36cf38b85af677262` (a GET under `repos/actions/checkout/`) returned that sha, dated 2026-07-16T19:43:47Z, first line of its message "backport fixes to releases-v4 (#2524)". **STOP RULE (3) did not fire.**

**(e) The edit.** Both lines are now `uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262 # v4.4.0`; the header comment states the commit pin, the release, the two routes, the date and D-8.

**(f) The check** (`pincheck.py`, last section). Control first, on `git show main:.github/workflows/ci.yml` copied to the scratch directory: it reported `FAIL` on exactly lines 29 and 42 (`2 uses: lines, failing line numbers: [29, 42]`, exit status 1). On the edited file: `PASS` on lines 30 and 43, `failing line numbers: []`, exit status 0. `git diff main -- .github/` shows three hunks only: the header comment (3 lines out, 4 in), line 29's `uses:` line, line 42's `uses:` line (the diff is in the commit). `python3 -I -c 'import yaml'` succeeded (exit status 0), so `parseyaml.py` (last section) parsed the edited file: `parsed OK; top-level keys: [True, 'jobs', 'name']` (YAML 1.1 reads the key `on` as `True`; that is PyYAML's reading, and the file is unchanged there), job `gate` with `actions/checkout@11d5960a…`, job `latest-stable` with the same.

**(g) DEBT.md D-8** is `Status: PAID 2026-11-12, spec 010`. By-catch, FETCHED from the release notes of `actions/checkout` (`gh api repos/actions/checkout/releases/tags/<tag> --jq .body`, 2026-11-12), and nothing was changed for any of it:

- **A newer major version exists:** v5 (v5.0.0, 2025-08-11; v5.1.0, 2026-07-20), v6 (v6.0.0, 2025-11-20; v6.1.0, 2026-07-20), and v7 (v7.0.0, 2026-06-18; v7.0.1, 2026-07-20). Not adopted (STOP RULE (5)).
- **v4.4.0's note** (FETCHED): "**[BREAKING]** backport `allow-unsafe-pr-checkout` to v4", with a link to a GitHub changelog of 2026-06-18 titled "safer pull_request_target defaults for github actions checkout". The v7.0.0 note says "block checking out fork pr for pull_request_target and workflow_run". INFERRED: `ci.yml` triggers on `push` to main and on `pull_request`, and uses neither `pull_request_target` nor `workflow_run`, so that breaking change does not reach it; I did not read the linked changelog.
- **Support of v4:** v4.4.0 was released on 2026-07-20 together with v2.8.0, v3.7.0, v5.1.0, v6.1.0 and v7.0.1, all six between 15:10 and 15:44 UTC that day. None of the notes I read (v4.4.0, v7.0.0, v7.0.1) says anything about the end of support of v4, and none gives a date on which v4 stops working. So no `DEBT.md` entry is filed (the prompt's condition for one is a note that gives such a date).

(h) `docs/sources.md` takes no row for this lookup (the prompt's ruling); the citation is in `ci.yml`'s comment and in this section.

## 2. The controls (METHOD rules 31 and 36)

### 2.1 Item 1's control: a scratch repository with planted items

Seeded (`control1.py`, last section; the planted strings are built from parts at run time and exist only in the scratch directory): commit 1 adds `planted.txt` holding one address on a reserved test domain (built from `"probe" + ".person"`, `"@"` and `"example" + ".test"`), one string of the `ghp_` shape (built from `"gh"`, `"p_"` and `"A1b2C3" * 6`, 36 letters and digits after the prefix), and `noreply@anthropic.com` (on THE ALLOWLIST); and adds `planted.bin`, 1,100,000 bytes with one NUL byte (500,000 `A`, a NUL, 599,999 `B`); commit 2 deletes `planted.txt` (by removing the file and `git add planted.txt`: `git rm` is not in GRANTS). The commits' author and committer are `noreply@github.com`, on THE ALLOWLIST.

| Check | Expected (and nothing else) | Observed |
|---|---|---|
| (a) | one address, and not the allowlisted one | distinct matches 3: `noreply@anthropic.com` and `noreply@github.com` on THE ALLOWLIST, 1 outside it, listed as `commit e50049a path planted.txt line 1 (after the @: example.test)`; outside-allowlist hits: 1 |
| (b) | one match | `gh[pousr]_ (36)` hits 1, listed as `commit e50049a path planted.txt line 2`; the other five patterns 0; secret hits total 1 |
| (c) | `planted.bin` | one blob listed: `planted.bin`, 1,100,000 bytes, first commit `e50049a`, in tip, `why=NUL+size>1e6`; unique blobs 2, with a NUL byte 1 |
| (d) | the text file | `planted.txt` removed by `a485335`, caught by (a)/(b)/(c): True; deleted paths 1 |

The control reports exactly the seeded difference and no other (SUMMARY `a-distinct=3 a-outside-hits=1 b-hits=1 c-listed=1 d-deleted=1`). **What the control does not reach, said plainly:** the extension branch of (c) (`planted.bin` is listed for its NUL and its size, not its extension) and five of the six secret patterns (`github_pat_`, `sk-`, `AKIA`, `BEGIN … PRIVATE KEY`, `xox`) were not seeded, because the prompt seeds one `ghp_` string and one binary only. Those branches reported 0 on main, and a 0 from a branch that was never shown able to fire is a weaker 0 than the `ghp_` one (METHOD rule 31).

### 2.2 Item 2's control

The pin check's control (§1.2 (f)): the unedited file fails on exactly its two tag-pinned lines, 29 and 42, and the edited file on none.

### 2.3 Item 3's controls

The shifted-pair control and the transcription check's control are in the sections below.

## 3. Item 0: the preconditions and the records (MEASURED)

**The PRECONDITION, every output quoted:**

- (a) `git status --porcelain` printed nothing; `git rev-parse --abbrev-ref HEAD` printed `main`.
- (b) `git log --format='%h %T %s' main` listed 11 commits: newest `56ddcb7 337c44827b8faaee4cee14eb13b1a13b3aa14637 spec 009: the Henning differential: TCG 1.06 against the engine's three systems, 1900-2100`; oldest `a0bca9d a96dcf3b2a8679c9d7ed428ab918ba0c00255e3a Initial commit`; `git rev-list --max-parents=0 main` printed one commit, `a0bca9d071bc646847f2a5ea6e6bced7b92e8222`. (The full listing is the history map's table.)
- (c) `git log --format='%ae%n%ce' main | sort -u` printed `8291740+kitaeza@users.noreply.github.com` and `noreply@github.com`, both on THE ALLOWLIST.
- (d) `git rev-list --count archive/main-pre-rewrite` printed 11; `git rev-parse 'main^{tree}' 'archive/main-pre-rewrite^{tree}'` printed `337c44827b8faaee4cee14eb13b1a13b3aa14637` twice; `git ls-remote rounds refs/heads/archive/main-pre-rewrite` printed `f4c796cd0f8d7054de192e21bd1b4b9f9d818e75	refs/heads/archive/main-pre-rewrite`, and `git rev-parse archive/main-pre-rewrite` printed `f4c796cd0f8d7054de192e21bd1b4b9f9d818e75`.
- (e) `git rev-list --count main ^archive/main-pre-rewrite` printed 11.
- (f) `git var GIT_AUTHOR_IDENT` printed `Kitaeza <8291740+kitaeza@users.noreply.github.com> 1794431522 +0300`, which names the first address of THE ALLOWLIST.

All six passed. **The branch** `spec/010-publication` was cut from `56ddcb7` (tree `337c44827b8faaee4cee14eb13b1a13b3aa14637`); its first commit, `9fdac81`, is the prompt, `cmp`-identical to `/home/kitaeza/zurkhai-prompts/spec-010-publication.md` (the first write of the copy differed in three lines' indentation; `cmp` said so, three `Edit`s fixed them, and `cmp` printed nothing before the commit). **010 is free** (`ls docs/specs/*.md` listed 001-009, `INDEX`, `TEMPLATE`).

**The Read-first list, each read this round:** CLAUDE.md (in full, 48 lines); docs/ROUNDS.md (in full, 266 lines at the base); docs/specs/003-project-method.md (in full, 180 lines, which holds §2, §6, §7, §11, §12); docs/METHOD.md lines 1-66 (header, index and rules 1 and 3) and rules 8, 15, 31, 35, 36, 50, 51, 56, 58, 60, 61, 65 and 67, each in full (lines 99-106, 173-182, 309-322, 349-366, 485-514, 549-577, 587-599, 629-636, 649-656); GATED.md (in full, 9 lines); FORKS.md (in full, 198 lines at the base); DEBT.md (lines 1-20, the header, and D-8, lines 83-89, in full); .github/workflows/ci.yml (in full, 48 lines); docs/specs/INDEX.md (in full, 15 lines); docs/specs/TEMPLATE.md (in full, 68 lines); docs/specs/009-henning-differential.md (lines 1-40: the head block and the report's opening); docs/sources.md (in full, 46 lines).

**The date:** `date` printed `2026-11-12 00:12:05 +0300`; it is used for every date written by this round (the human's rulings keep 2026-11-11). **The round's marker** is chosen and is not spelled here; its grep result is in §10.

**The records Item 0 wrote:**

- `FORKS.md` F11: `Status:` RULED; `Ruled:` 2026-11-11, (c), the private mirror, the git remote `rounds`, the human's words "agree with recommendations, go" (spec 010 R1); `Built in:` spec 010; the interim line stays and ends "(superseded: RULED 2026-11-11)".
- `FORKS.md` F26, new, filed already RULED, with the fields as given (subject, Raised, Question, Options, Status, Ruled, Built in). The next free id was derived from the file: the last row was F25, so F26.
- **The OPEN count at the base, re-measured:** `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md` on the base tree printed **6** (F4, F5, F7, F8, F10, F11), equal to the advisor's RELAYED 6. The expected count at every commit after the `FORKS.md` edit is 6 + 0 raised + 0 gated findings − 1 ruling (F11) = **5**; F26 is filed RULED and adds nothing.

## The history map

The human rewrote main's history before this round so that every commit, the initial commit included, carries their GitHub no-reply address where it carried a personal address (`FORKS.md` F26, RULED 2026-11-11, option (b)). The old history is kept in the private mirror, the git remote `rounds`, as the branch `archive/main-pre-rewrite`, which a public reader cannot reach. This round did not make the rewrite; it paired the two histories commit by commit, oldest first, and compared every field except the email addresses: for all 11 pairs, the tree, the author name, the author date, the committer name, the committer date and the raw message are equal, and each new commit's parent is the new commit paired with the old commit's parent (the initial commits have none); one thing could not be kept, the signature of the old initial commit (the new one is unsigned; no other commit on either side is signed). A tree id is therefore the same on both sides, and a commit id that the tree cites from before the rewrite (a 7-character or a 40-digit prefix of an "old commit" below) is translated by finding the row whose old commit begins with it and reading the new commit in the same row.

| Position | Old commit | New commit | Tree | Subject |
|---|---|---|---|---|
| 1 | `fda72ebc171bc0d6ef97a24195925e1cfa52010c` | `a0bca9d071bc646847f2a5ea6e6bced7b92e8222` | `a96dcf3b2a8679c9d7ed428ab918ba0c00255e3a` | Initial commit |
| 2 | `73aca121e27f19e49d093f00c4c3c766abce78fc` | `0ac42b2e50f744ae297c6891b17ddb2879305f9d` | `43a0340edb5739693e574a05399b1d84dc3f949b` | Initialize the project documents |
| 3 | `f33467eae45ff0a802cbde2f29ea22751f596555` | `cfc604d78213c23a0154d088b03b73c25161e1d1` | `dbdf3418d77df05751efed2abb4aba3c81948a9d` | ADVISOR.md r2: the advisor runs at effort xhigh |
| 4 | `1e89a5c868527fad6e34e7a1f55bfb5c63a02f7f` | `62da8b4b3d09b6368fd0712f696aaebac05b054f` | `3b931847636c864339b96b524ae7de5849241711` | Accept specs 001–003; rule F6 (a) |
| 5 | `59bad84088b56ee484a76a655f2d177d8b58511c` | `aaf55a6234b7095c4535b0aa42f2872e7ea5e9a3` | `24d4398480b6345a620c11fd9061da4f5d5c8ed6` | spec 004: the workspace and the gate |
| 6 | `0ebaaf8a9822d6f49c11bec47367742b4555c72f` | `68540381eca72b7b12a1d121a437aa11bfb5f0b2` | `4406c74daf75ab147eabcba4560afca24926418f` | spec 005: the Phugpa core, Gregorian <-> ZDate from Janson v1, and the gate hardened |
| 7 | `45efbbce3c6bccb21e361d9f709ad629fe6d914c` | `eb5c7605fcc5b0b36404b9856f7a882b19ea10f2` | `73e135b4a1d9ae950addcebfa4568f31966ee92c` | spec 006: the reference fixtures: T5 tables from Janson v1, the T4 oracle, P5 and P6 |
| 8 | `62413d193bd2df623da5ba3e1062999e4a7af501` | `44147b3c888c72a66e4d401b9a1ae2f147ca42c9` | `b5f8e943c194745308638232715dcdf1458428d3` | spec 007: the TO-VERIFY attributes of 001 §5.2 — what Janson v1 defines; rows 6-8 deferred |
| 9 | `34430fb4674206a1bcebdb1b3a1da4525d0feb51` | `5da1c5c3e03728ea93931420f68ff7852efaf063` | `46735e32ba90f018fc07114c0aeece43b5d30031` | ADVISOR.md r3, ROUNDS.md r2: the reviewer session at a round's close |
| 10 | `07b5bbf0632debab15a32175970fbd6c83a70cce` | `d174965afc95cee0e7e0091261922c88aba56974` | `cdc937a95e657ba99e8d2e3dd7f34311d02caab7` | spec 008: Genden and Tsurphu: the two other candidate systems, Gregorian <-> ZDate from Janson v1, and P5 for both |
| 11 | `f4c796cd0f8d7054de192e21bd1b4b9f9d818e75` | `56ddcb7b05ce693b565abb013d662ec4dbf30324` | `337c44827b8faaee4cee14eb13b1a13b3aa14637` | spec 009: the Henning differential: TCG 1.06 against the engine's three systems, 1900-2100 |

## 4. Item 3: the comparison, the signatures, the citations (MEASURED)

**(a) Linear.** `git rev-list --min-parents=2 --count` printed 0 for `main` and 0 for `archive/main-pre-rewrite`; 11 commits each. Paired by position, oldest first (`git rev-list --reverse`).

**(b) The comparison** (`map.py`, last section; no format string in it holds `%ae`, `%aE`, `%ce` or `%cE`): for each of the 11 pairs, the tree (`%T`), the author name (`%an`), the author date (`%ad`, `--date=raw`), the committer name (`%cn`), the committer date (`%cd`, `--date=raw`), the raw message (the bytes after the first blank line of `git cat-file commit`), the message as `%B`, and the parent link: all 8 comparisons equal on all 11 pairs, **0 rows with any differing field of 11**. STOP RULE (4) did not fire.

**(c) Signatures** (`git cat-file commit`, a header line starting `gpgsig`): old side, signed: position 1, `fda72eb` (the old initial commit); the other 10 are unsigned. New side: 0 signed (position 1, `a0bca9d`, is unsigned). Reported, not a FINDING: a rewrite cannot keep a signature.

**(d) The shifted-pair control:** old commit i against new commit i+1 for i = 1..10, the tree comparison: it failed on **10 of 10** shifted pairs, as it must. (The control reaches the tree comparison only; the other fields were not shifted.)

**(g) The transcription check** (`transcribe.py`, last section): see §9 for its control and its result on the committed table.

**(h) Where the tree cites old ids** (`git grep -F -o -e <7 characters> main --`, MEASURED at main's tip; the count is of literal occurrences of the 7 characters, and includes 3 that are the start of a full 40-digit id: one of `45efbbc` in `docs/specs/007-to-verify-attributes.md` line 102 and two of `62413d1`, in `docs/specs/008-genden-tsurphu.md` line 443 and `docs/specs/prompts/008-genden-tsurphu.md` line 32; with `-w` they are not counted, and the totals are 66 against 69):

| Position | Old id | Mentions | Files (mentions) |
|---|---|---|---|
| 1 | `fda72eb` | 1 | `docs/specs/004-workspace-gate.md` (1) |
| 2 | `73aca12` | 0 | none |
| 3 | `f33467e` | 0 | none |
| 4 | `1e89a5c` | 7 | `docs/specs/004-workspace-gate.md` (3), `docs/specs/prompts/004-workspace-gate.md` (3), `docs/specs/007-to-verify-attributes.md` (1) |
| 5 | `59bad84` | 8 | `docs/specs/005-phugpa-core.md` (4), `docs/specs/prompts/005-phugpa-core.md` (4) |
| 6 | `0ebaaf8` | 13 | `docs/specs/006-reference-fixtures.md` (8), `docs/specs/prompts/006-reference-fixtures.md` (5) |
| 7 | `45efbbc` | 13 | `docs/specs/007-to-verify-attributes.md` (6), `docs/specs/prompts/007-to-verify-attributes.md` (7) |
| 8 | `62413d1` | 2 | `docs/specs/008-genden-tsurphu.md` (1), `docs/specs/prompts/008-genden-tsurphu.md` (1) |
| 9 | `34430fb` | 6 | `DEBT.md` (2), `docs/specs/008-genden-tsurphu.md` (2), `docs/specs/prompts/009-henning-differential.md` (2) |
| 10 | `07b5bbf` | 20 | `DEBT.md` (2), `docs/specs/009-henning-differential.md` (10), `docs/specs/prompts/009-henning-differential.md` (8) |
| 11 | `f4c796c` | 0 | none |
| | **The nine after the initial commit** | **69** = 0 + 0 + 7 + 8 + 13 + 13 + 2 + 6 + 20 | **13 files** |
| | **The initial commit** | **1** | **1 file** |

Positions 2 and 3 are not cited by any file, and position 11 (the old tip, the old squash of spec 009) cannot be cited by a file of its own squash. The advisor's RELAYED count before round 009 and the rewrite was 45 in 10 files and 1 in 1 file; this tree has 69 in 13 files and 1 in 1 file (round 009's files add the rest). None of the mentions is edited: the committed prompts and the reports are records.

## 5. Item 4: docs/ROUNDS.md §9, revision r3

The round's wording (the advisor reads it at the landing). Step 4 is R2, the push of `main` after every merge, with the check first (the check as the prompt gives it, with one sentence for the first push, below). Step 5 is R1: `push rounds main spec/NNN-name`, then the `ls-remote` check against `rev-parse`, then `branch -D`, with the sentence on why `-d` refuses after a squash kept, and the mirror named by its remote only. One sentence after step 5 gives the history note: `main`'s history was rewritten once, before the first public push (F26), and a commit id that the tree cites from before then is translated by `docs/specs/010-publication.md`, "The history map". The Status line is r3 (2026-11-12) and the Revisions section has the r3 entry. (The tree names the mirror by its remote only: no URL was read or written.)

**(e) The grep** of docs/, CLAUDE.md and the root files for "kept locally", "F11" and "when the human decides to publish" (`git grep -n -i` on the base tree, then on the edited files): "when the human decides to publish" had one hit, `docs/ROUNDS.md` §9 step 4, which the edit replaced. "kept locally" had hits in `docs/ROUNDS.md` §9 step 5 (replaced) and `FORKS.md` F11 (the interim line, which the prompt keeps). **Sentences fixed beside the edit (CLAUDE.md hard rule 19): none.** Every other hit is a flag and is not fixed: 19 lines in dated reports of accepted or draft specs that name F11 as OPEN at the time they were written (`docs/specs/004-workspace-gate.md` lines 282 and 439; `005-phugpa-core.md` lines 346 and 498; `006-reference-fixtures.md` lines 217 and 364; `007-to-verify-attributes.md` lines 1282, 1303 and 1396; `008-genden-tsurphu.md` lines 259 and 341; `009-henning-differential.md` lines 164, 335, 358, 639, 641, 655, 749 and 785). They are frozen records (METHOD rule 26), correct as of their dates. The other hits are `FORKS.md` lines 93, 96 and 100 (F11 itself) and this round's own ROUNDS.md edits.

## 6. The predictions, scored

| # | Prediction | Score | Measured |
|---|---|---|---|
| 1 | Every PRECONDITION check passes | HELD | §3: (a)-(f) all passed, outputs quoted |
| 2 | Every control reports exactly what was seeded | HELD | §2: Item 1 `a-distinct=3 a-outside-hits=1 b-hits=1 c-listed=1 d-deleted=1`, exactly the seeded items; Item 2 failed on exactly lines 29 and 42; Item 3 failed on 10 of 10 shifted pairs, and the transcription control reported exactly row 7. The Item 1 control leaves five secret patterns and the extension rule unseeded (§2.1) |
| 3 | Item 1 finds no FINDING: no address outside THE ALLOWLIST, no secret, no binary, document, image or archive blob, and no path deleted along main | FAILED as worded, HELD in substance | 0 addresses outside, 0 secret hits, 0 blobs with a NUL byte or a listed extension: held. "No path deleted along main": 1 path is in some commit and not in the tip, `LICENSE`, and it is a rename to `LICENSE-MIT` with 0 changed lines (`0ac42b2`). It is not a FINDING (§1.1 (d)), but the clause as written is false |
| 4 | Every blob over 1,000,000 bytes is under golden/reference/, accounted for by spec 006 | HELD | exactly 1 blob over 1,000,000 bytes (of 134): `golden/reference/tdc-97d4a00-phugpa-1900-2100.csv`, 1,976,432 bytes; spec 006 line 264 gives the same size |
| 5 | The tag v4 points to the commit of exactly one published v4.x.y release, the highest, and the two routes agree | HELD | v4 and v4.4.0 both `11d5960a…7262` by Route A and Route B; v4.4.0 is the highest of 15 v4.x.y releases, published, not a draft, not a prerelease; the other 14 v4.x.y tags have other commits |
| 6 | actions/checkout has published a release with a major version above 4 | HELD | v5.1.0, v6.1.0 and v7.0.1, all 2026-07-20 (and v5.0.0, v6.0.0, v7.0.0 before them) |
| 7 | All 11 pairs agree on every field of Item 3(b); the old initial commit is signed and its new commit is not; no other commit on either side is signed | HELD | 0 of 11 rows with a differing field; signed old: 1 (`fda72eb`, position 1); signed new: 0 |
| 8 | The tree cites old 7-character ids at least 46 times | HELD | 70 = 69 (the nine after the initial commit) + 1 (the initial commit); 66 + 1 = 67 if the 3 starts of a full 40-digit id are left out. Both are at least 46 |
| 9 | The gate's count equals the last count recorded on main | HELD | 147 passed, 0 failed (debug) and 147 passed, 0 failed (release); spec 009's head block records 147 / 147 (RECALLED) |
| 10 | OPEN at the final commit equals OPEN at the base minus 1 | HELD | base 6, final 5 (§8) |
| 11 | The diff touches 7 files | HELD | `git diff --name-only main HEAD` at the final commit lists exactly the 7 files named (checked in the pre-commit chain: the count is asserted to be 7, and the commit aborts if it is not) |

## 7. R1 to R4, and the records Item 0 wrote, by id

- **R1** (F11 = (c), the private mirror): written onto `FORKS.md` F11 (`Status:` RULED, `Ruled:`, `Built in:` spec 010, the interim line marked superseded) and built in `docs/ROUNDS.md` §9 step 5. The tree names the mirror by its remote `rounds` only; the round read no URL of it and wrote none. `git remote -v` was run once for the remote names, with its output cut to names by `awk` before it was displayed (a read-only git command outside the list of GRANTS, see §11).
- **R2** (`main` pushed after every merge): `docs/ROUNDS.md` §9 step 4, with the check first. See §11 for the flaw in the check's range at the first push.
- **R3** (the rewrite): checked, never made. The history map is the section above; F26 records the ruling. No format string run on `archive/main-pre-rewrite` holds `%ae`, `%aE`, `%ce` or `%cE`: the only commands run on that branch were `rev-list --count`, `rev-parse` of the branch and its tree, `ls-remote rounds`, and `map.py` (fields `%T %an %ad %cn %cd %P %s %B` and a raw message cut from `git cat-file commit`, whose header, with its address, was held in the script's memory to look for `gpgsig` and never printed or written). The audit ran on `main` only. No email address outside THE ALLOWLIST is written anywhere.
- **R4** (files edited only with Edit and Write): every file in the repository was written with the Write or Edit tool; no script wrote into the repository (`control1.py` writes only into a scratch repository, `mutate.py` only a scratch copy). Controls first, for Items 1, 2 and 3, each showing its instrument can fail (§2).
- **Records:** `FORKS.md` F11 (ruled), F26 (new, RULED); `DEBT.md` D-8 (PAID 2026-11-12, spec 010); `docs/ROUNDS.md` r3; `docs/specs/INDEX.md` row 010 (final commit). No `DEBT.md` entry is filed (§11, by-catch).

## 8. The open-forks audit (METHOD rule 60)

`expected = previous OPEN count + forks raised + gated findings filed with options named and none chosen − rulings recorded`, then `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`.

| Commit | Arithmetic | grep printed |
|---|---|---|
| base (`56ddcb7`), re-measured | 6 (F4, F5, F7, F8, F10, F11) | 6 |
| `9fdac81` (the prompt) | 6 + 0 + 0 − 0 = 6 | 6 |
| `723dd38` (Items 0-4) | 6 + 0 raised (F26 is filed RULED) + 0 − 1 (F11) = 5 | 5 (F4, F5, F7, F8, F10, at lines 51, 57, 70, 76, 90) |
| the final commit | 5 + 0 + 0 − 0 = 5 | 5 (asserted in the pre-commit chain) |

Two-sided: a row not filed would show as a count too low, a ruling not recorded as too high; both agree.

## 9. The transcription check of the history map (Item 3 (g))

`transcribe.py` derives the 11 rows (position, old commit from `git rev-list --reverse archive/main-pre-rewrite`, new commit from `git rev-list --reverse main`, tree from `git rev-parse <new>^{tree}`, subject from `git show -s --format=%s <new>`) and compares each with the row of the table in the spec file. **Control first:** a scratch copy of the file with one hex digit changed in row 7 (the tree's first digit, 7 to 8, by `mutate.py`) made it print `row 7: DISAGREE in tree`, `AGREE` for the other ten rows, and `disagreeing rows: [7]` (exit status 1): exactly that row. **On the table as committed at `723dd38`** (and again in the pre-commit chain of the final commit): `rows derived from git: 11; rows in the table: 11`, 11 of 11 `AGREE`, `disagreeing rows: []` (exit status 0).

## 10. The marker greps

The round's marker is one word, not spelled in this file. The grep before each commit was `git grep --untracked -n -e <the marker> -- . ':!docs/specs/prompts/010-publication.md'` (the frozen prompt copy is the only exclusion), and it printed nothing at `9fdac81`, at `723dd38`, and at the final commit (asserted in the chain: `! git grep …`).

## 11. The rest of the contract

**Properties.** The round has no property list of its own; the prompt's items are the properties.

| Item | State | Evidence |
|---|---|---|
| 0: branch, prompt, number, records, date, marker, OPEN count | DONE | §3; the first commit `9fdac81` `cmp`-identical; OPEN 6 at the base |
| 1: the audit (a)-(d), its control first | DONE | §1.1 and §2.1; 0 FINDINGs |
| 2: the pin (a)-(h) | DONE | §1.2; both `uses:` lines at `11d5960a…7262 # v4.4.0`; D-8 PAID |
| 3: the map (a)-(h) | DONE | the history map, §4, §9 |
| 4: ROUNDS.md §9 r3 (a)-(e) | DONE | §5 |
| 5: the gate, the INDEX row, the report, the squash message | DONE | §14 and the final commit |

**Every prompt question answered.** Does main's history hold anything that must not be published? No: no address outside THE ALLOWLIST, no secret, one blob over the size limit and accounted for, one renamed path (§1.1). Which commit does the pin name, and do the two routes agree? `11d5960a326750d5838078e36cf38b85af677262`, the release v4.4.0; they agree (§1.2). Does the rewritten history match the old one? Yes in every compared field, on all 11 pairs; one signature is lost, as a rewrite must (§4). How many times does the tree cite an old id? 70 (§4 (h)). Does the gate's count equal the last recorded on main? Yes, 147 and 147 against spec 009's 147 and 147 (RECALLED, §14).

**The stop rule that governed the round:** STOP RULE (2) governed Item 1 and (3) governed Item 2 (the two decisive items); neither fired. (4) did not fire (0 of 11 rows differ). (1) did not fire. (5) was respected: nothing under crates/, golden/, rules/, clients/ or tools/, no METHOD.md, GATED.md, CLAUDE.md, ADVISOR.md, sources.md or accepted spec was changed; no newer major of actions/checkout adopted; nothing pushed, fetched, pulled, deleted, tagged or rewritten. (6): one call was denied, and one run needed a command outside GRANTS' list. The denial: the no-cd hook blocked a command of mine that began with `cd /tmp` (CLAUDE.md hard rule 16; my mistake, the `cd` served no purpose); I reran the same command without the `cd`, using absolute paths and `git -C`. The commands outside the list, none of which raised a permission prompt: `kill <pid>` and `pkill -f` (to stop my own runaway `git grep` on the control, §1.1), `pgrep`, `timeout`, and `sleep`; `git remote -v` and `git merge-base origin/main main` (read-only); and `git rev-list --objects main | git cat-file --batch-check` as a cross-check (`cat-file` and `rev-list` are on the list). I report them rather than working around anything.

**Diff scope.** Measured at `723dd38`, `git diff --stat main HEAD`: 6 files changed, 628 insertions(+), 11 deletions(-): `.github/workflows/ci.yml` 6+/5−, `DEBT.md` 1+/1−, `FORKS.md` 12+/2−, `docs/ROUNDS.md` 6+/3−, `docs/specs/010-publication.md` 243+/0−, `docs/specs/prompts/010-publication.md` 360+/0−; the sums: 6 + 1 + 12 + 6 + 243 + 360 = 628 and 5 + 1 + 2 + 3 = 11. The final commit adds the row of `docs/specs/INDEX.md` (+1) and the sections of this file after §5 and the head block's gate line, the 7th file. **Gated paths touched: no** (this round's own spec, prompt and INDEX row are not changes to an accepted spec). **Files the test framework wrote: none** (`git status --porcelain` printed nothing after the gate; `target/` is ignored). A stray file I wrote by mistake outside the repository: `/tmp/claude-1000/-home-kitaeza/github-zurkhai/placeholder-not-used` (2 bytes, a mistyped path of the `Write` tool, not in the scratchpad); I did not delete it, because `rm` outside a `mktemp -d` directory is outside GRANTS; the human may remove it.

**Forks encountered, resolved by the round** ("considered X, chose Y because Z"): (1) the check of §9 step 4 at the first push: considered rewording the check so that it is exact at the first push (naming both addresses), or keeping the prompt's check verbatim and adding one sentence; chose the second, because the prompt gives the check as the human's rule, THE ALLOWLIST already lists `noreply@github.com` for the initial commit, and a sentence decides nothing. (2) D-8's `Status:` line: the register's shape is `PAID <date, commit>` and the prompt says "your date and spec 010"; chose the prompt's form, since the commit does not exist when the line is written. (3) The control's deletion: `git rm` is not in GRANTS; chose removing the file and `git add <path>`. No unresolved fork; no STOP AND ASK; no new `FORKS.md` row besides F26, which the prompt dictated.

**Latitude taken.** (1) Three extra reads in Item 2, all GETs under `repos/actions/checkout/`: `git/commits/<sha>` as a third witness, and the release bodies of v4.4.0, v7.0.0 and v7.0.1 for the by-catch the prompt asks for. (2) In Item 1, a `git grep -F -e @` prefilter before the prompt's regular expression, a scan of the raw commit objects, and a cross-check of the blob count (§1.1). (3) In Item 3, two extra comparisons beyond the prompt's list: the message as `%B` and the parent link of every pair, and a 40-digit id per row of the table. (4) One added sentence in ROUNDS.md §9 step 4 for the first push (the fork above). (5) The squash message section is last, as ROUNDS.md §4.2 and TEMPLATE.md put it, after the commands and scripts, which the prompt calls "last".

**What the prompt got wrong.**
- **The check of Item 4 (a) fails at the first push.** `git log --format='%ae%n%ce' origin/main..main | sort -u` must print only the human's no-reply address. Measured now: `origin/main` is the old initial commit `fda72eb`, which has no common ancestor with `main` (`git merge-base origin/main main` printed nothing, exit status 1), so the range is all of `main`, and the command printed two lines: the human's no-reply address and `noreply@github.com` (the initial commit's committer). The first push would have failed its own check. Handled by one sentence in §9 step 4; the advisor reads the wording. The same first push is a force-push (R3), which step 4's `push origin main` alone would not be.
- **Prediction 3** says no path is deleted along main; `LICENSE` is, by a rename (§6). The prompt's FINDING test for (d) is sound; the prediction's wording was not.
- **Item 1's regular expression as written is quadratic** under `git grep -E` on a very long line (it hung on the control's 500,000-byte line, §1.1); the prompt did not say so, and I changed the instrument, not the expression.
- **Item 2 (c)** tells the round to look up each annotated tag; all 16 v4 tags are lightweight, so that step had nothing to apply to. Not wrong (the prompt says "for each annotated tag you rely on"), but the prompt expected `^{}` lines in Route A and there are none.
- **Item 1's control** does not say whose address the scratch commits carry; I used an address on THE ALLOWLIST (`noreply@github.com`), so the control's commit objects add two allowlisted matches to the "distinct" count (3, not 2). Harmless, and shown in §2.1.
- **The records' dates:** the prompt dates its rulings 2026-11-11 and the machine's clock says 2026-11-12; the rulings keep the human's date, and everything this round wrote carries 2026-11-12. `date` stayed on 2026-11-12 to the end (00:12:05 at the start, 00:48:35 before the report was finished).
- Nothing else found.

**By-catch.**
- v4.4.0 (the pinned release) carries a **[BREAKING]** change (`allow-unsafe-pr-checkout`, §1.2 (g)). INFERRED not to reach `ci.yml`, whose triggers are `push` and `pull_request`. No release note says when v4 stops working, so no `DEBT.md` entry qualifies and none is filed.
- Newer majors exist (v5, v6, v7). Not adopted; the pin is v4 as D-8 and the prompt say. Whether to move to v7 is the human's, a later round's.
- 19 lines in dated reports (listed in §5) still call F11 OPEN. Frozen records; flagged, not fixed.
- `DEBT.md` D-8's statement still reads "uses `actions/checkout@v4`", which describes the debt as it was when filed; left as a record (its `Status:` is PAID).
- The stray file in /tmp (above). No `DEBT.md` id is filed for any of these.

**Held (METHOD rule 6):** none.

## 12. Diff scope: see §11

(One place, §11, "Diff scope".)

## 13. STOP AND ASK

None. No fork was raised, no `FORKS.md` row is OPEN because of this round, and no precondition failed.

## 14. The gate (Item 5)

`bash /home/kitaeza/github/zurkhai/tools/gate.sh`, run once in the background on the clean committed tree at `723dd38` (nothing in flight: `git status --porcelain` printed nothing before it and nothing after). Exit status 0. `GATE: 11 legs run — 11 passed, 0 failed, 1 skipped`. Debug: `test` leg 147 passed, 0 failed (27 + 8 + 5 + 13 + 28 + 1 + 14 + 17 + 5 + 5 + 6 + 1 + 5 + 8 + 4 = 147); release: `test_release` leg 147 passed, 0 failed (the same 15 result lines); the `msrv` leg repeats the 147 on the pinned toolchain `rustc 1.99.0`. The other legs: `fmt`, `clippy`, `build_wasm`, `deny`, `no_std_check`, `no_alloc_check`, `zero_deps_check` and `self_test` (26 plants, 26 rejected with their named diagnostic, 0 failures) all PASS. **Skipped leg:** `wasm-pack` (`crates/tshes-wasm` does not exist, 002 §7; METHOD rule 65). The Q1 report and the rules validator (METHOD rule 65) are not legs of `tools/gate.sh` and were not run: no `golden/` file, system or `rules/` file changed. **Count against the last recorded on main:** spec 009's head block records 147 passed / 147 passed (RECALLED); this gate's 147 / 147 are equal, so the count is comparable. **The final commit** changes `docs/specs/010-publication.md` and `docs/specs/INDEX.md` only; no gate leg reads either (a search of `crates/` and `tools/` found reads of `golden/` only, by `fixtures.rs` and `include_str!`, and none of `docs/` or `.github/`), so the gate's result stands for it.

## 15. The commands and the scripts, verbatim

`S` is the scratch directory `/tmp/claude-1000/-home-kitaeza-github-zurkhai/274141f8-d91e-4c31-ba79-342cc840517c/scratchpad/tmp.xc5D2UeW2K`, `R` is `/home/kitaeza/github/zurkhai`; the control repository is a second `mktemp -d` directory. In the real commands the paths were written out in full.

```
git -C $R status --porcelain ; git -C $R rev-parse --abbrev-ref HEAD
git -C $R log --format='%h %T %s' main ; git -C $R rev-list --max-parents=0 main
git -C $R log --format='%ae%n%ce' main | sort -u
git -C $R rev-list --count archive/main-pre-rewrite
git -C $R rev-parse 'main^{tree}' 'archive/main-pre-rewrite^{tree}'
git -C $R ls-remote rounds refs/heads/archive/main-pre-rewrite ; git -C $R rev-parse archive/main-pre-rewrite
git -C $R rev-list --count main ^archive/main-pre-rewrite ; git -C $R var GIT_AUTHOR_IDENT
date +%F' '%T' '%z
git -C $R switch -c spec/010-publication
cmp /home/kitaeza/zurkhai-prompts/spec-010-publication.md $R/docs/specs/prompts/010-publication.md
grep -c '^- \*\*Status:\*\* OPEN$' $R/FORKS.md
python3 -I $S/control1.py <empty mktemp -d dir> ; python3 -I $S/audit.py <that dir> HEAD
python3 -I $S/audit.py $R main
git -C $R rev-list --objects main | cut -d' ' -f1 | git -C $R cat-file --batch-check='%(objecttype)' | sort | uniq -c
grep -rn 'uses:' $R/.github/
git ls-remote https://github.com/actions/checkout 'refs/tags/v4*'
gh api repos/actions/checkout/git/matching-refs/tags/v4 --jq '.[] | [.ref, .object.type, .object.sha] | @tsv'
gh api 'repos/actions/checkout/releases?per_page=100' --paginate --jq '.[] | [.tag_name, .draft, .prerelease, .published_at] | @tsv' | sort -k4 -r | head -40
gh api repos/actions/checkout/git/commits/11d5960a326750d5838078e36cf38b85af677262 --jq '[.sha, .author.date, (.message|split("\n")[0])] | @tsv'
gh api repos/actions/checkout/releases/tags/v4.4.0 --jq '.body'      # and v7.0.1, v7.0.0
git -C $R show main:.github/workflows/ci.yml > $S/pin/ci-unedited.yml
python3 -I $S/pincheck.py $S/pin/ci-unedited.yml ; python3 -I $S/pincheck.py $R/.github/workflows/ci.yml
git -C $R diff main -- .github/ ; python3 -I -c 'import yaml' ; python3 -I $S/parseyaml.py $R/.github/workflows/ci.yml
python3 -I $S/map.py $R main archive/main-pre-rewrite
git -C $R grep -w -F -o -e <7-character old id> main --          # and without -w, and -c, for the 11 ids
python3 -I $S/mutate.py $R/docs/specs/010-publication.md $S/spec-mutated.md
python3 -I $S/transcribe.py $R main archive/main-pre-rewrite $S/spec-mutated.md
python3 -I $S/transcribe.py $R main archive/main-pre-rewrite $R/docs/specs/010-publication.md
git -C $R grep -n -i -e 'kept locally' -e 'F11' -e 'when the human decides to publish' -- docs CLAUDE.md DEBT.md FORKS.md GATED.md ...
git -C $R log --format='%ae%n%ce' origin/main..main | sort -u ; git -C $R merge-base origin/main main
bash $R/tools/gate.sh
```

`audit.py`:

```python
#!/usr/bin/env python3
"""Spec 010 Item 1: the audit of a repository's history (a)-(d).

usage: python3 -I audit.py <repo> <rev>
Read-only git on <repo>. Never prints the text of a secret, and for an email
address outside THE ALLOWLIST prints only the part after the @.
"""
import re
import subprocess
import sys
from collections import OrderedDict, defaultdict

repo, rev = sys.argv[1], sys.argv[2]

EMAIL = r"[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+(\.[A-Za-z0-9-]+)*\.[A-Za-z]{2,}"
ALLOW = {
    "8291740+kitaeza@users.noreply.github.com",
    "noreply@github.com",
    "noreply@anthropic.com",
    "noreply@deepseek.com",
}
SECRETS = [
    ("gh[pousr]_ (36)", r"gh[pousr]_[A-Za-z0-9]{36}"),
    ("github_pat_ (22+)", r"github_pat_[A-Za-z0-9_]{22,}"),
    ("sk- (20+)", r"sk-[A-Za-z0-9_-]{20,}"),
    ("AKIA (16)", r"AKIA[A-Z0-9]{16}"),
    ("BEGIN..PRIVATE KEY", r"BEGIN[A-Z ]*PRIVATE KEY"),
    ("xox[abprs]-", r"xox[abprs]-"),
]
EXTS = tuple((".pdf .djvu .png .jpg .jpeg .gif .webp .tif .tiff .zip .gz .tgz .xz .7z "
              ".rar .tar .doc .docx .xls .xlsx .odt .exe .dll .so .dylib").split())
SIZE_LIMIT = 1_000_000


def git(*args, stdin=None, ok=(0,)):
    r = subprocess.run(["git", "-C", repo, *args], input=stdin, capture_output=True)
    if r.returncode not in ok:
        sys.stderr.write("git %s failed rc=%d\n" % (" ".join(args), r.returncode))
        sys.exit(2)
    return r.stdout


commits = git("rev-list", "--reverse", rev).decode().split()
short = {c: c[:7] for c in commits}
print("repo commits scanned: %d (oldest first)" % len(commits))

LINE = re.compile(rb"^([0-9a-f]{40}):(.*?):(\d+):(.*)$")


def grep_commit(commit, pattern, only_matching):
    args = ["grep", "-a", "-n", "-E"] + (["-o"] if only_matching else []) + ["-e", pattern, commit]
    out = git(*args, ok=(0, 1))
    for line in out.split(b"\n"):
        if not line:
            continue
        m = LINE.match(line)
        if not m:
            sys.stderr.write("unparsed grep line\n")
            sys.exit(2)
        yield m.group(1).decode(), m.group(2).decode("utf-8", "replace"), int(m.group(3)), m.group(4)


def grep_commit_fixed(commit, literal):
    out = git("grep", "-a", "-n", "-F", "-e", literal, commit, ok=(0, 1))
    for line in out.split(b"\n"):
        if not line:
            continue
        m = LINE.match(line)
        if not m:
            sys.stderr.write("unparsed grep line\n")
            sys.exit(2)
        yield m.group(1).decode(), m.group(2).decode("utf-8", "replace"), int(m.group(3)), m.group(4)


# ---------------------------------------------------------------- (a)
print("\n== (a) email addresses ==")
email_files = []   # (commit, path, line, match)
# Prefilter: git grep -F '@' (every address holds one), then the prompt's regex
# on those lines only. The regex alone is quadratic on a very long line with no
# '@' (a 500 KB binary line hung the first version of this script).
rx_line = re.compile(EMAIL.encode())
for c in commits:
    for _c, path, ln, text in grep_commit_fixed(c, "@"):
        for m in rx_line.finditer(text):
            email_files.append((c, path, ln, m.group(0).decode("utf-8", "replace")))
email_objs = []    # (commit, section, line-in-section, match)
rx = re.compile(EMAIL.encode())
for c in commits:
    raw = git("cat-file", "commit", c)
    head, _, msg = raw.partition(b"\n\n")
    for section, blob in (("header", head), ("message", msg)):
        for i, line in enumerate(blob.split(b"\n"), 1):
            for m in rx.finditer(line):
                email_objs.append((c, section, i, m.group(0).decode("utf-8", "replace")))

distinct = OrderedDict()
for c, p, ln, mt in email_files:
    d = distinct.setdefault(mt, {"files": 0, "objs": 0, "commits": set()})
    d["files"] += 1
    d["commits"].add(c)
for c, s, ln, mt in email_objs:
    d = distinct.setdefault(mt, {"files": 0, "objs": 0, "commits": set()})
    d["objs"] += 1
    d["commits"].add(c)
print("distinct matches: %d" % len(distinct))
allowed = [m for m in distinct if m in ALLOW]
outside = [m for m in distinct if m not in ALLOW]
print("  on THE ALLOWLIST: %d; outside it: %d" % (len(allowed), len(outside)))
for m in allowed:
    d = distinct[m]
    print("  allowlisted %s: %d in files, %d in commit objects, in %d commits"
          % (m, d["files"], d["objs"], len(d["commits"])))
email_hit_paths = set()
for c, p, ln, mt in email_files:
    if mt not in ALLOW:
        domain = mt.split("@", 1)[1]
        print("  OUTSIDE: commit %s path %s line %d  (after the @: %s)" % (short[c], p, ln, domain))
        email_hit_paths.add(p)
for c, s, ln, mt in email_objs:
    if mt not in ALLOW:
        domain = mt.split("@", 1)[1]
        print("  OUTSIDE: commit %s object %s line %d  (after the @: %s)" % (short[c], s, ln, domain))
n_out_hits = sum(1 for x in email_files if x[3] not in ALLOW) + sum(1 for x in email_objs if x[3] not in ALLOW)
print("  outside-allowlist hits (commit,place,line): %d" % n_out_hits)

# ---------------------------------------------------------------- (b)
print("\n== (b) secrets ==")
secret_hits = []
secret_hit_paths = set()
for name, pat in SECRETS:
    rxs = re.compile(pat.encode())
    for c in commits:
        for _c, path, ln, _txt in grep_commit(c, pat, False):
            secret_hits.append((c, path, ln, name))
            secret_hit_paths.add(path)
        raw = git("cat-file", "commit", c)
        head, _, msg = raw.partition(b"\n\n")
        for section, blob in (("header", head), ("message", msg)):
            for i, line in enumerate(blob.split(b"\n"), 1):
                if rxs.search(line):
                    secret_hits.append((c, "<commit " + section + ">", i, name))
    print("  pattern %-20s hits: %d" % (name, sum(1 for h in secret_hits if h[3] == name)))
for c, p, ln, name in secret_hits:
    print("  FINDING: commit %s path %s line %d pattern %s" % (short[c], p, ln, name))
print("  secret hits total: %d" % len(secret_hits))

# ---------------------------------------------------------------- (c)
print("\n== (c) blobs ==")
blobs = {}                 # oid -> dict(size, paths, first)
paths_by_commit = []       # list of dict(path -> oid)
for idx, c in enumerate(commits):
    out = git("ls-tree", "-r", "-l", "-z", "--full-tree", c)
    cur = {}
    for ent in out.split(b"\0"):
        if not ent:
            continue
        meta, path = ent.split(b"\t", 1)
        mode, typ, oid, size = meta.decode().split()
        if typ != "blob":
            continue
        path = path.decode("utf-8", "replace")
        cur[path] = oid
        b = blobs.get(oid)
        if b is None:
            b = blobs[oid] = {"size": int(size), "paths": OrderedDict(), "first": c}
        b["paths"][path] = True
    paths_by_commit.append(cur)
tip_oids = set(paths_by_commit[-1].values())
tip_paths = set(paths_by_commit[-1].keys())

oids = list(blobs)
inp = ("\n".join(oids) + "\n").encode()
out = git("cat-file", "--batch", stdin=inp)
pos = 0
has_nul = {}
for oid in oids:
    nl = out.index(b"\n", pos)
    hdr = out[pos:nl].split()
    size = int(hdr[2])
    body = out[nl + 1: nl + 1 + size]
    has_nul[oid] = b"\0" in body
    pos = nl + 1 + size + 1
print("unique blobs reachable: %d; with a NUL byte: %d" % (len(oids), sum(has_nul.values())))
c_hits = []
for oid, b in blobs.items():
    reasons = []
    if has_nul[oid]:
        reasons.append("NUL")
    if b["size"] > SIZE_LIMIT:
        reasons.append("size>1e6")
    if any(p.lower().endswith(EXTS) for p in b["paths"]):
        reasons.append("extension")
    if reasons:
        c_hits.append((oid, b, reasons))
c_hit_paths = set()
for oid, b, reasons in c_hits:
    for p in b["paths"]:
        c_hit_paths.add(p)
    print("  BLOB %s paths=%s size=%d first-commit=%s in-tip=%s why=%s"
          % (oid[:10], list(b["paths"]), b["size"], short[b["first"]], oid in tip_oids, "+".join(reasons)))
print("  listed blobs: %d" % len(c_hits))

# ---------------------------------------------------------------- (d)
print("\n== (d) paths deleted along the history ==")
all_paths = OrderedDict()
for cur in paths_by_commit:
    for p in cur:
        all_paths[p] = True
deleted = [p for p in all_paths if p not in tip_paths]
flagged = 0
for p in deleted:
    removed_at = None
    last_oid = None
    for i in range(1, len(commits)):
        if p in paths_by_commit[i - 1] and p not in paths_by_commit[i]:
            removed_at = commits[i]
            last_oid = paths_by_commit[i - 1][p]
    b = blobs[last_oid]
    caught = (p in email_hit_paths) or (p in secret_hit_paths) or (p in c_hit_paths)
    flagged += 1 if caught else 0
    print("  DELETED: %s removed-by %s blob %s size %d %s; caught by (a)/(b)/(c): %s"
          % (p, short[removed_at], last_oid[:10], b["size"], "binary" if has_nul[last_oid] else "text", caught))
print("  deleted paths: %d; of them caught by (a), (b) or (c): %d" % (len(deleted), flagged))

print("\nSUMMARY a-distinct=%d a-outside-hits=%d b-hits=%d c-listed=%d d-deleted=%d"
      % (len(distinct), n_out_hits, len(secret_hits), len(c_hits), len(deleted)))
```

`control1.py` (the planted strings are the parts shown):

```python
#!/usr/bin/env python3
"""Spec 010 Item 1 control: build the scratch repository with planted items.

usage: python3 -I control1.py <empty-scratch-dir>
The planted strings are built from parts at run time. They are written only
into files inside the scratch directory given as the argument.
"""
import os
import subprocess
import sys

d = sys.argv[1]
if os.listdir(d):
    sys.exit("scratch dir not empty")


def git(*a):
    env = dict(os.environ,
               GIT_AUTHOR_NAME="Control", GIT_COMMITTER_NAME="Control",
               GIT_AUTHOR_EMAIL="noreply" + "@" + "github.com",
               GIT_COMMITTER_EMAIL="noreply" + "@" + "github.com")
    subprocess.run(["git", "-C", d, *a], check=True, env=env, capture_output=True)


git("init", "-q")
local, dom = "probe" + ".person", "example" + ".test"            # the address outside THE ALLOWLIST
token = "gh" + "p_" + ("A1b2C3" * 6)                              # ghp_ + 36 letters/digits
allowed = "noreply" + "@" + "anthropic" + ".com"                  # on THE ALLOWLIST
with open(os.path.join(d, "planted.txt"), "w") as f:
    f.write("contact: " + local + "@" + dom + "\n")
    f.write("token: " + token + "\n")
    f.write("trailer: " + allowed + "\n")
with open(os.path.join(d, "planted.bin"), "wb") as f:
    f.write(b"A" * 500000 + b"\x00" + b"B" * 599999)
assert os.path.getsize(os.path.join(d, "planted.bin")) == 1100000
git("add", "planted.txt", "planted.bin")
git("commit", "-q", "-m", "control 1: add the planted items")
os.remove(os.path.join(d, "planted.txt"))
git("add", "planted.txt")
git("commit", "-q", "-m", "control 2: delete the text file")
print("control repository built")
```

`pincheck.py`:

```python
#!/usr/bin/env python3
"""Spec 010 Item 2 (f): every `uses:` line ends in @ + 40 hex digits, then optionally a comment.

usage: python3 -I pincheck.py <file>...
Prints each uses: line with its verdict, and the line numbers that fail.
"""
import re
import sys

USES = re.compile(r"^\s*(?:-\s+)?uses:\s*(\S+)(.*)$")
GOOD = re.compile(r"@[0-9a-f]{40}$")
COMMENT = re.compile(r"^(\s+#.*)?$")

bad_total = 0
for path in sys.argv[1:]:
    bad = []
    n = 0
    with open(path, encoding="utf-8") as f:
        for i, line in enumerate(f, 1):
            m = USES.match(line.rstrip("\n"))
            if not m:
                continue
            n += 1
            ok = bool(GOOD.search(m.group(1))) and bool(COMMENT.match(m.group(2)))
            print("%s:%d %s %s" % (path, i, "PASS" if ok else "FAIL", line.strip()))
            if not ok:
                bad.append(i)
    print("%s: %d uses: lines, failing line numbers: %s" % (path, n, bad))
    bad_total += len(bad)
sys.exit(1 if bad_total else 0)
```

`parseyaml.py`:

```python
import sys
import yaml

d = yaml.safe_load(open(sys.argv[1], encoding="utf-8"))
print("parsed OK; top-level keys:", sorted(d.keys(), key=str))
for job, body in d["jobs"].items():
    print("job", job, "steps", len(body["steps"]), "uses:", [s.get("uses") for s in body["steps"] if "uses" in s])
```

`map.py`:

```python
#!/usr/bin/env python3
"""Spec 010 Item 3 (a)-(e): pair main with archive/main-pre-rewrite, commit by commit.

usage: python3 -I map.py <repo> <new-ref> <old-ref>
No format string here holds %ae, %aE, %ce or %cE (R3). Raw commit objects are
read only to cut out the message and to test for a gpgsig line; the header is
never printed.
"""
import subprocess
import sys

repo, new_ref, old_ref = sys.argv[1], sys.argv[2], sys.argv[3]


def git(*args):
    r = subprocess.run(["git", "-C", repo, *args], capture_output=True)
    if r.returncode != 0:
        sys.stderr.write("git %s failed\n" % " ".join(args))
        sys.exit(2)
    return r.stdout


def count_merges(ref):
    return int(git("rev-list", "--min-parents=2", "--count", ref).decode().strip())


old = git("rev-list", "--reverse", old_ref).decode().split()
new = git("rev-list", "--reverse", new_ref).decode().split()
print("(a) merges: %s = %d, %s = %d; commits: old %d, new %d"
      % (new_ref, count_merges(new_ref), old_ref, count_merges(old_ref), len(old), len(new)))
assert len(old) == len(new)

FMT = "%T%x00%an%x00%ad%x00%cn%x00%cd%x00%P%x00%s"


def fields(c):
    out = git("show", "-s", "--date=raw", "--format=" + FMT, c).decode("utf-8")
    t, an, ad, cn, cd, p, s = out.rstrip("\n").split("\0")
    return {"tree": t, "an": an, "ad": ad, "cn": cn, "cd": cd, "parents": p.split(), "subject": s}


def raw_message(c):
    raw = git("cat-file", "commit", c)
    head, sep, msg = raw.partition(b"\n\n")
    return msg, any(l.startswith(b"gpgsig") for l in head.split(b"\n"))


def pct_b(c):
    return git("show", "-s", "--format=%B", c)


F_old = [fields(c) for c in old]
F_new = [fields(c) for c in new]
M_old = [raw_message(c) for c in old]
M_new = [raw_message(c) for c in new]

print("\n(b) true pairs, field by field (D = differs):")
bad_rows = 0
print("pos  tree an ad cn cd msgraw msg%B parent")
for i in range(len(old)):
    f_o, f_n = F_old[i], F_new[i]
    r = {
        "tree": f_o["tree"] == f_n["tree"],
        "an": f_o["an"] == f_n["an"],
        "ad": f_o["ad"] == f_n["ad"],
        "cn": f_o["cn"] == f_n["cn"],
        "cd": f_o["cd"] == f_n["cd"],
        "msgraw": M_old[i][0] == M_new[i][0],
        "msgB": pct_b(old[i]) == pct_b(new[i]),
    }
    if i == 0:
        r["parent"] = (f_o["parents"] == [] and f_n["parents"] == [])
    else:
        r["parent"] = (f_o["parents"] == [old[i - 1]] and f_n["parents"] == [new[i - 1]])
    if not all(r.values()):
        bad_rows += 1
    print("%2d   %s" % (i + 1, "  ".join("=" if r[k] else "D" for k in
                                         ("tree", "an", "ad", "cn", "cd", "msgraw", "msgB", "parent"))))
print("rows with any differing field: %d of %d" % (bad_rows, len(old)))

print("\n(c) signatures (a gpgsig header line):")
for i in range(len(old)):
    print("  pos %2d old %s signed=%s   new %s signed=%s"
          % (i + 1, old[i][:7], M_old[i][1], new[i][:7], M_new[i][1]))
print("  signed old: %d; signed new: %d" % (sum(m[1] for m in M_old), sum(m[1] for m in M_new)))

print("\n(d) control: old[i] against new[i+1], tree comparison, i = 1..%d" % (len(old) - 1))
fails = 0
for i in range(len(old) - 1):
    same = F_old[i]["tree"] == F_new[i + 1]["tree"]
    if not same:
        fails += 1
print("  shifted pairs on which the tree comparison fails (as it must): %d of %d" % (fails, len(old) - 1))

print("\nROWS (position | old | new | tree | subject):")
for i in range(len(old)):
    print("| %d | `%s` | `%s` | `%s` | %s |" % (i + 1, old[i], new[i], F_new[i]["tree"], F_new[i]["subject"]))
```

`transcribe.py`:

```python
#!/usr/bin/env python3
"""Spec 010 Item 3 (g): derive the 11 history-map rows from git again and compare them
with the table in a spec file.

usage: python3 -I transcribe.py <repo> <new-ref> <old-ref> <spec-file>
Prints one verdict per row and the numbers of the rows that disagree.
"""
import re
import subprocess
import sys

repo, new_ref, old_ref, spec = sys.argv[1:5]


def git(*a):
    r = subprocess.run(["git", "-C", repo, *a], capture_output=True)
    if r.returncode != 0:
        sys.exit("git failed: " + " ".join(a))
    return r.stdout.decode("utf-8")


old = git("rev-list", "--reverse", old_ref).split()
new = git("rev-list", "--reverse", new_ref).split()
derived = []
for i in range(len(new)):
    tree = git("rev-parse", new[i] + "^{tree}").strip()
    subj = git("show", "-s", "--format=%s", new[i]).rstrip("\n")
    derived.append((i + 1, old[i], new[i], tree, subj))

text = open(spec, encoding="utf-8").read()
section = text.split("\n## The history map\n", 1)
if len(section) != 2:
    sys.exit("no '## The history map' section")
body = section[1].split("\n## ", 1)[0]
ROW = re.compile(r"^\| (\d+) \| `([0-9a-f]{40})` \| `([0-9a-f]{40})` \| `([0-9a-f]{40})` \| (.*) \|$")
rows = []
for line in body.split("\n"):
    m = ROW.match(line)
    if m:
        rows.append((int(m.group(1)), m.group(2), m.group(3), m.group(4), m.group(5)))

print("rows derived from git: %d; rows in the table: %d" % (len(derived), len(rows)))
bad = []
for d in derived:
    t = [r for r in rows if r[0] == d[0]]
    if len(t) != 1:
        print("row %d: DISAGREE (table has %d rows with this position)" % (d[0], len(t)))
        bad.append(d[0])
        continue
    names = ("position", "old", "new", "tree", "subject")
    diff = [names[k] for k in range(5) if t[0][k] != d[k]]
    print("row %d: %s" % (d[0], "AGREE" if not diff else "DISAGREE in " + ",".join(diff)))
    if diff:
        bad.append(d[0])
extra = [r[0] for r in rows if r[0] not in [d[0] for d in derived]]
print("rows in the table with no derived row: %s" % extra)
print("disagreeing rows: %s" % bad)
sys.exit(1 if bad or extra or len(rows) != len(derived) else 0)
```

`mutate.py`:

```python
#!/usr/bin/env python3
"""Spec 010 Item 3 (g) control: copy the spec file to the scratch directory with one hex digit
of one row changed (row 7, the tree: its first hex digit 7 becomes 8).

usage: python3 -I mutate.py <spec-file> <scratch-copy>
"""
import sys

src, dst = sys.argv[1], sys.argv[2]
text = open(src, encoding="utf-8").read()
old = "`73e135b4a1d9ae950addcebfa4568f31966ee92c`"
new = "`83e135b4a1d9ae950addcebfa4568f31966ee92c`"
assert text.count(old) == 1, "anchor must occur once"
open(dst, "w", encoding="utf-8").write(text.replace(old, new))
print("changed 1 hex digit in row 7 (tree)")
```

## The squash commit message

```
spec 010: before the first public push: the checkout pin, the history audit and map, the private mirror

- .github/workflows/ci.yml: both actions/checkout uses lines pin the commit of release v4.4.0
  (11d5960a326750d5838078e36cf38b85af677262), found by two routes that agree (git ls-remote and
  the GitHub API's matching-refs) on 2026-11-12; DEBT.md D-8 is PAID.
- The audit of main's 11 commits found nothing that must not be published: no address outside the
  allowlist, no secret, one blob over 1,000,000 bytes (the T4 fixture, spec 006), one renamed path
  (LICENSE to LICENSE-MIT). Its controls on a scratch repository reported exactly what was planted.
- The history map (docs/specs/010-publication.md): main and archive/main-pre-rewrite paired commit
  by commit; the tree, names, dates and messages are equal on all 11 pairs; only the old initial
  commit's signature is lost. FORKS.md F26 records the human's ruling on the rewrite.
- FORKS.md F11 RULED (c): merged round branches go to a private mirror (the git remote rounds),
  never to the public origin. docs/ROUNDS.md r3: section 9 steps 4 and 5 and the history note.
- Gate: 147 tests passed, 0 failed in debug and in release; 11 legs run, 11 passed, 1 skipped
  (wasm-pack). OPEN forks: 6 before, 5 after.

Co-Authored-By: Claude Sonnet 5.5 <noreply@anthropic.com>
```
