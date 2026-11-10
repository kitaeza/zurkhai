═══════════════════════════════════════════════════════════════════════════
▼▼▼ ROUND PROMPT — SPEC 004 — BEGINS HERE. NOTHING ABOVE IS PART OF IT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

RECIPIENT:    zurkhai-engine
WORKDIR:      /home/kitaeza/github/zurkhai
SPEC NUMBER:  004 — ASSIGNED by the advisor. Verify it is free in the tree (ls docs/specs/*.md); STOP if not.
BRANCH:       spec/004-workspace-gate, cut from main at the commit PRECONDITION names, before any edit;
              commit to it, the halt included. No merge. No push.
MODEL:        Sonnet 5.5. EFFORT: High. Reason: 003 §7, first branch — discoverable: 002 states every
              setting this round makes, and a wrong one is made LOUD by the gate self-test Item 5
              mandates (docs/ROUNDS.md §8).
              FALLBACK: commit at every item boundary. A continuing session reads git status, git log
              and the report first; it never commits a previous session's uncommitted work unverified —
              it finishes it and proves it by the item's own check, or redoes it. A design choice that
              would fall to the fallback model is a STOP AND ASK. The report names the switch point.
PRECONDITION: main at 1e89a5c (specs 001–003 accepted; F6 ruled), or exactly one commit past it whose
              subject is "Add docs/ROADMAP.md; file F12 (first client: a website); ADVISOR.md r3" and
              which changes only docs/ROADMAP.md, FORKS.md and docs/ADVISOR.md. Anything else: STOP and
              report, zero diff — merging is the human's.
              Read first, in this order: CLAUDE.md; docs/ROUNDS.md; docs/specs/002-rust-code-standard.md;
              docs/specs/001-init-spec.md (§3, §8, §10, §11 and Appendix A); docs/specs/003-project-method.md;
              docs/METHOD.md (all of Part 1 — rules 7, 17, 23, 31, 32, 38, 49, 51, 56, 58–60, 63–65
              and 67 bind this round directly); GATED.md; FORKS.md; DEBT.md; docs/specs/TEMPLATE.md;
              .gitignore.
ARTIFACT:     None outside the tree: the inputs are the files in the Read-first list. No paper is
              needed, because no calendar arithmetic lands in this round. If a listed file is absent,
              STOP and report.
STOP RULE:    (1) A requirement of 002 §2–§4, §7 or §8 that cannot be met as written on the pinned
              toolchain — a lint that does not exist under the name 002 gives, a gate leg that cannot
              run: STOP at that point; report the clause and the exact error; substitute nothing.
              (2) A planted violation of a rule 002 marks [CI] (Item 5b, 5c) that the gate does not
              reject, and that no check you can add to the gate rejects: STOP AND ASK — a [CI] rule
              would be unenforced.
              (3) An install named in GRANTS fails, or a permission is denied: STOP and report each
              denial with its reason; never work around it by another tool or route (METHOD rule 67).
              (4) Writing a calendar term in code beyond the crate name F3 fixed, or computing anything
              calendrical, is out of scope: do not write it. If something seems to need it, report it
              under "What the prompt got wrong".
CITATIONS:    Every citation is FETCHED, SNIPPET-VERIFIED or RECALLED / UNVERIFIED; no fourth kind; an
              unobtainable source says PAYWALLED or UNREACHABLE. Read docs/sources.md before any fetch.
              Never curl/wget. (METHOD rule 50)
GATED:        yes: crates/tshes/**, under accepted spec 002 (with 001 §8's layout). Also your own
              docs/specs/004-workspace-gate.md, docs/specs/prompts/004-workspace-gate.md and your row in
              docs/specs/INDEX.md, which are not changes to an accepted spec. No other gated path.
GRANTS:       No reads outside WORKDIR beyond this prompt file. Installs, each command approved by the
              human as it runs: the toolchain you pin (Item 1), with rustfmt, clippy and the
              wasm32-unknown-unknown target (rustup, into ~/.rustup); cargo-deny
              (`cargo install --locked cargo-deny`, into ~/.cargo/bin). Network: those installs and
              cargo-deny's advisory database. Scratch: the gate self-test (Item 5) may create and delete
              its own directory under the system temp directory (mktemp -d) or under target/.
              Nothing else.
ASKING:       Every adjudication in the STOP AND ASK form (docs/ROUNDS.md §5). Never an interactive widget.
PATHS:        Every Bash call uses absolute paths and no cd; git uses `git -C`. One command per call
              where it can be.

Spec 004 — the workspace and the gate

Round 004 is M1's first round (001 Appendix A, Task 1). It sets up the Cargo workspace, the toolchain
pin, the lints and the 002 §7 gate on an empty `tshes` core, so that the gate is green, and shown able
to fail, before any calendar code lands. It runs now because every later round builds under this gate.
It does NOT implement calendar arithmetic, create the other crates of 002 §3, fetch any paper, or write
anything under golden/.

ITEM 0 — Branch, prompt, number.
  Create spec/004-workspace-gate from the commit PRECONDITION names. Commit this file, byte-identical,
  as docs/specs/prompts/004-workspace-gate.md — the first commit — and `cmp` it against
  /home/kitaeza/zurkhai-prompts/spec-004-workspace-gate.md before every later commit. Verify 004 is free
  in the tree. Read the Read-first list. Run `date` once and use that value for every date you write
  (METHOD rule 51). Choose your round's marker (METHOD rule 58). Re-measure the OPEN count in FORKS.md
  with the grep its header defines: the advisor counted 8 at 1e89a5c (RELAYED — re-measure it, do not
  copy it).

ITEM 1 — The toolchain to pin.
  002 §2 pins "the stable version current when the repo is created"; the repository's first commit is
  dated 2026-10-02. Find the current stable release from rustup itself, not from memory, and quote the
  command and its output. If rustup reports a stable released after 2026-10-02, STOP AND ASK which to
  pin. Install it per GRANTS. Quote `rustc -Vv` and `cargo -V` of the pinned toolchain.

ITEM 2 — The workspace (002 §2, §3, §8).
  - Root Cargo.toml as a workspace: edition 2024 for every member; `rust-version` equal to the pin (the
    MSRV, 002 §2); licence `MIT OR Apache-2.0` (002 §2); `overflow-checks = true` in [profile.release]
    (002 §4.3); clippy `pedantic` at warn, workspace-wide (002 §8).
  - rust-toolchain.toml pinning the exact version, with rustfmt, clippy and the wasm32-unknown-unknown
    target (002 §2).
  - Members: crates/tshes only. The other 002 §3 crates are created by the rounds that need them, by
    milestone (001 §10); crates/tshes-ffi also waits for F7, which is OPEN.
  - No rustfmt.toml (002 §8). Cargo.lock is committed (.gitignore says why).

ITEM 3 — The core, empty (002 §4, F6).
  - crates/tshes with no dependencies (002 §4.9); `#![no_std]` with no `alloc` (F6, RULED 2026-10-02,
    option (a)); `#![forbid(unsafe_code)]`; the deny list of 002 §4.1, §4.2 and §4.4, applied to the
    core.
  - No calendar code, no Error type, no arithmetic helpers: they land with the first calendar round. No
    dev-dependencies are needed; if you find you need one, say why.
  - The 002 §4.3 tripwire: a test that fails unless arithmetic overflow panics, in the profile each of
    `cargo test` and `cargo test --release` builds it with. It asserts the overflow panic itself — the
    mechanism, not a symptom (METHOD rule 31). State which Cargo profile `cargo test --release` builds
    it with and how that profile relates to [profile.release], from Cargo's documentation or output,
    labelled per CITATIONS.
  - D-2: the only calendar term this round writes in code is the crate name `tshes`, fixed by the
    human's ruling F3 (FORKS.md). The glossary is created by the first round that writes calendar code,
    from a source. D-2 stays OPEN; the report says so.

ITEM 4 — The gate, defined once (002 §7).
  - One script under tools/ defines every gate leg. The hosted workflow (.github/workflows/) and your
    local runs call that same script, so the two cannot drift.
  - Legs: 002 §7's list, plus Item 5's self-test. The wasm-pack leg is SKIPPED this round and named with
    its reason: crates/tshes-wasm does not exist (METHOD rule 65). The MSRV leg runs
    `cargo +<msrv> test --workspace`. The hosted workflow also builds and tests on latest stable
    (002 §2).
  - The report lists every third-party action the workflow uses and how each is pinned.
  - You cannot run the hosted workflow, because rounds never push: it is "reasoned, not run" (METHOD
    rule 17) until the human's first push, and the report says so. "CI green" in this round means every
    leg of the script passing locally on the final committed tree.
  - The self-test leg is not in 002 §7's list: file a DEBT.md entry (Kind: DOCS) so that 002's next
    revision lists it. TRIGGER: the next revision of 002.

ITEM 5 — The gate self-test (METHOD rule 31).
  Committed under tools/, run by the gate, and run by you. It works on a scratch copy of the workspace
  (GRANTS), each copy with its own target directory (METHOD rule 64), deleted at the end (METHOD rule 49)
  — never on the working tree. Each planted violation is otherwise-valid code, and is asserted by the
  diagnostic NAME it must produce (a lint name or an error code), never by "the build failed", which a
  typo also produces.
  a. Clean control first: the unmodified copy passes every leg the self-test drives. A self-test whose
     clean copy fails proves nothing.
  b. The [CI] rules: 002 §4.1 (float arithmetic); §4.2 (integer `/`, and `%`, each on signed operands);
     §4.4 (unwrap, expect, panic, indexing); §4.5 (an unsafe block; the core reaching std — the clock,
     the filesystem, the environment). METHOD rule 31 names "`no_std` making 002 §4.5 mechanical" as a
     by-construction claim: plant every route you know by which a `#![no_std]` crate can still reach
     std, and report which the gate rejects. A shape it does not reject: add a check to the gate that
     rejects it, and plant again. If no check can, STOP RULE (2).
  c. The overflow control: in the copy, overflow-checks off in [profile.release]; Item 3's tripwire must
     go red under `cargo test --release`.
  d. Ruled or stated, but not marked [CI]: F6's "no alloc", and 002 §4.9's "zero dependencies" (a path
     dependency needs no network). Plant each and report. Where the gate does not reject it, add a check
     if that is local and cheap; otherwise file a DEBT.md entry with a TRIGGER. Either way, report it as
     "considered X, chose Y because Z".
  Report a table: rule; planted shape; expected diagnostic; observed diagnostic; verdict.

ITEM 6 — The gate, last.
  Before the final commit, grep 002 for forks with no ruling and compare with FORKS.md (METHOD rule 59);
  name the grep. Then run the gate on the final committed tree, both profiles, no edits in flight
  (METHOD rule 65; CLAUDE.md hard rule 12). The gate line follows docs/ROUNDS.md §4.1: counts, never
  "green", every skipped leg named. Quote, from each tool's own output, the toolchain each leg ran on
  (METHOD rule 23). A `cargo deny` failure on a network fetch is classified by METHOD rule 63's
  procedure, never by feeling. Commit messages reference spec 004 (002 §8).

PREDICTIONS — written by the advisor before any measurement (METHOD rule 3). Score each HELD or FAILED,
with the measured value.
  1. The pinned toolchain is Rust 1.99.0. The advisor derived it from the six-week release schedule,
     counted from 1.85.0 on 2025-02-20 (RECALLED / UNVERIFIED); rustup's own report decides.
  2. Every planted violation in Item 5b is rejected, each with the diagnostic named for it.
  3. At least one shape in Item 5b reaches std past `#![no_std]` and passes the gate as 002 §7 lists it,
     so `#![no_std]` alone does not make 002 §4.5 mechanical and the gate needs a check of its own. The
     same holds for alloc in Item 5d.
  4. The overflow tripwire passes in debug and in release, and goes red in Item 5c's control.
  5. The final gate: every leg passes, cargo deny included; wasm-pack SKIPPED; the only tests counted
     are Item 3's tripwires, each passing in both profiles.
  6. The MSRV leg runs the same toolchain as the pin (002 §2 sets the MSRV equal to it at creation), so
     this round it adds no independent evidence; the report says so.

THE REPORT — per CLAUDE.md's report contract and docs/ROUNDS.md §4, in docs/specs/004-workspace-gate.md:
the head block; a report-shaped spec (docs/specs/TEMPLATE.md's header lines — Milestone M1; Governs: the
workspace root, crates/tshes, tools/, .github/workflows/ — then the end marker); then the report.
  - Lead with the decisive item: Item 5's table and the final gate line.
  - Then every clause of 002 §2–§8, scored DONE / DEFERRED / N-A with evidence (expected value, actual
    value, the assertion or command, the test or leg name); 001 §6.4 P1–P7 as N-A, with the reason.
  - The predictions, scored. The toolchain inventory. D-2's state. The open-forks audit with its
    arithmetic. Every other section of the contract, "What the prompt got wrong" included.
  - Your row in docs/specs/INDEX.md in the final commit, and the squash commit message (docs/ROUNDS.md
    §4.2, item 13).

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ROUND PROMPT — SPEC 004 — ENDS HERE. NOTHING BELOW IS PART OF IT.   ▲▲▲
═══════════════════════════════════════════════════════════════════════════

═══════════════════════════════════════════════════════════════════════════
▼▼▼ ADDENDUM 1 — SPEC 004 — 2026-11-10 — BEGINS HERE. PART OF THE PROMPT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

Written before any session ran this round. Where it differs from the text above, it governs.
The banners above and below it are part of the file the round commits byte-identical (Item 0).

THE HUMAN'S RULINGS (2026-10-02 rulings unchanged; these are new, given 2026-11-10):
  R1. This round runs on a model outside 003 §7's table, as a one-round trial: DeepSeek V4 Pro,
      in the CodeWhale terminal agent. 003 §7's general policy on such models is decided by 003's
      next revision, with this round's landing as its evidence.
  R2. Of DeepSeek V4 Pro and V4 Flash, V4 Pro.

HEADER CHANGES — these lines replace the header's MODEL line:
MODEL:        DeepSeek V4 Pro (`deepseek-v4-pro`), in CodeWhale. EFFORT: thinking on, at CodeWhale's
              highest reasoning setting. Reason: 003 §7, first branch — unchanged from the header
              (discoverable; a wrong setting is made LOUD by Item 5). The change is the model family,
              by the human's ruling R1; it is not a new branch of the table. V4 Pro over V4 Flash by
              R2: the model and the harness are both untried here, so the round does not also take
              the smallest model.
              FALLBACK: as in the header. The fallback model is Sonnet 5.5, effort High, in Claude
              Code; the switch is a fresh session, and the report names the switch point.

THE HARNESS — CodeWhale. Each of these is checked, not assumed:
  H1. Instruction file. CodeWhale loads AGENTS.md by default; this repository's rules are in
      CLAUDE.md, imported through CodeWhale's `project_instruction_imports` setting in its user
      config, outside the repository. Before Item 0, state whether CodeWhale loaded CLAUDE.md,
      quoting what it shows. If it reports CLAUDE.md as found but not loaded: STOP and report,
      zero diff. Never create AGENTS.md or any other instruction file in the repository.
  H2. Approval mode: "Ask" only (`permission_posture = "ask"`; CodeWhale 0.10.2 accepts ask,
      auto-review or full-access). Never "Auto-Review" or "Full Access": GRANTS has every install
      approved by the human, not by a reviewer model. The report names the mode and quotes
      `codewhale --version`.
  H3. Network. CodeWhale's web search and fetch tools are network, and GRANTS does not grant them.
      Item 3's Cargo citation comes from local output (`cargo help`, `cargo test -v`) or is
      RECALLED / UNVERIFIED.
  H4. Files the harness writes. Before creating the branch, quote `git status --porcelain`; it is
      empty. Any file CodeWhale itself writes inside WORKDIR later (snapshots, session state) is
      never committed and never added to .gitignore. The report lists it under diff scope, and a
      DEBT.md entry (Kind: TOOLING) records it, with TRIGGER: the next round run in CodeWhale.
  H5. Trailers. Every commit this session makes, and the squash message, ends with exactly this
      model trailer:
          Co-Authored-By: DeepSeek V4 Pro <noreply@deepseek.com>
      No other model's trailer, including any CodeWhale adds by itself; remove it before
      committing. A fallback session writes its own model's trailer on its own commits.
  H6. The report's `model:` line gives the model ID as CodeWhale displays it, and the reasoning
      setting by its name and value as CodeWhale displays it — not as the model describes itself.
  H7. Sub-agents. CLAUDE.md hard rule 11 applies unchanged: each brief carries the GRANTS line
      verbatim, with H2–H3. The report names the model each sub-agent ran on.

ITEM 0 — ADDITION.
  File the ruling R1 in FORKS.md as a RULED row, in the round's first commit after the prompt
  commit. Its number is the next free F-n in FORKS.md on the branch: F12 if main is at 1e89a5c,
  F13 if the roadmap commit PRECONDITION names is present (that commit files F12).
    - Raised: 2026-11-10, spec 004 ADDENDUM 1 (the advisor's fork 1).
    - Question: May a round run on a model outside 003 §7's table, and how is it recorded?
    - Options: (a) a one-round trial on round 004 (DeepSeek V4 Pro, in CodeWhale), recorded in this
      row; 003's next revision decides the general policy. (b) Revise 003 §7 first. (c) Keep 004 on
      Sonnet 5.5, in Claude Code.
    - Status: RULED
    - Ruled: 2026-11-10, (a). V4 Pro over V4 Flash, also ruled 2026-11-10.
  In the same commit, a DEBT.md entry (Kind: METHOD): 003 §7 names no non-Claude model, and this
  round ran outside its table under that row. TRIGGER: the next revision of 003.
  The row is RULED, so it does not change the OPEN count; re-measure that count anyway.

PREDICTION 7 — written by the advisor before any measurement (METHOD rule 3):
  7. At landing, the advisor's mechanical checks (ADVISOR.md §6, step 3) all pass: the prompt
     `cmp` through this addendum, the model three ways, the scope against GATED, the open-forks
     audit, the marker grep, the INDEX row. The report predicts nothing here; the advisor scores
     it at landing.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ADDENDUM 1 — SPEC 004 — ENDS HERE. NOTHING BELOW IS PART OF IT.      ▲▲▲
═══════════════════════════════════════════════════════════════════════════

═══════════════════════════════════════════════════════════════════════════
▼▼▼ ADDENDUM 2 — SPEC 004 — 2026-11-10 — BEGINS HERE. PART OF THE PROMPT. ▼▼▼
═══════════════════════════════════════════════════════════════════════════

Appended after the halt at 6c5fe19 (STOP AND ASK spec 004 Fork 1, FORKS.md F13). Where it differs
from the text above, it governs. MODEL, the harness rules H1–H7 and GRANTS are unchanged. This is a
resume: a fresh session on spec/004-workspace-gate. The model is the one ADDENDUM 1 names.

THE HUMAN'S RULING (2026-11-10, "agree with all recommends"):
  R3. F13 (spec 004 Fork 1): option (a). Pin Rust 1.99.0, and set the MSRV (`rust-version`) to
      1.99.0. 002 §2's text decides it: "the stable version current when the repo is created".
      Rust's RELEASES.md gives "Version 1.99.0 (2026-10-01)"; the repository's first commit is
      2026-10-02. (The advisor FETCHED this from
      https://raw.githubusercontent.com/rust-lang/rust/master/RELEASES.md on 2026-11-10. The
      round does not re-fetch it: GRANTS gives no network for it.)

ITEM 0 — RESUME.
  - Read this prompt through ADDENDUM 2, then `git log`, then the report so far. Quote
    `git status --porcelain` (it is empty) and `codewhale --version`. Re-check H1 and H2 for this
    session and quote what CodeWhale shows. The workspace's tests cannot be run once before you
    build anything, because no workspace exists at the halt. Say so; do not skip it in silence.
  - Write R3 onto F13's row: `Status: RULED`, and
    `Ruled: 2026-11-10, (a) — pin 1.99.0, MSRV 1.99.0 (002 §2's reading).`
  - File a DEBT.md entry (Kind: TOOLING). Statement: the pin, 1.99.0, predates 1.99.1, which
    fixes "rustc: fix miscompilation of FFI `bool` return values on x86_64" (RELEASES.md,
    "Version 1.99.1 (2026-10-15)"; FETCHED by the advisor, 2026-11-10). TRIGGER: the round that
    creates crates/tshes-ffi. That round raises the pin to at least 1.99.1, deliberately, in its
    own PR (002 §2), or it states why not. It also states whether the MSRV moves with the pin.
  - The expected OPEN count after this commit is 9 − 1 = 8. Re-measure it with the grep FORKS.md
    defines.

ITEM 1 — RESOLVED BY R3.
  Install 1.99.0 by its exact version, per GRANTS, with rustfmt, clippy and the
  wasm32-unknown-unknown target. Quote `rustc -Vv` and `cargo -V` of the pinned toolchain. Then
  continue with Items 2–6 as written.

THE REPORT — FIXES TO THE HALT'S DRAFT, made in the same file:
  1. The marker (METHOD rule 58). Report the greps for the round's marker, including the greps
     before the three commits already made. If a grep was not run before a commit, say so; do not
     reconstruct it.
  2. The open-forks arithmetic at every commit (METHOD rule 60), the halt commit included
     (8 + 1 = 9), and this resume's commit (9 − 1 = 8).
  3. The `model:` line. Explain "roster route" in one line, or remove it. The line stays as
     CodeWhale displays it (H6), with one entry per session.
  4. H7. For each session, name the model each sub-agent ran on, or write "no sub-agents ran".
  5. A date correction, in place, marked `[CORRECTED 2026-11-10: …]`, both in the report and on
     F13's Question line. "2026-10-13" in rustup's output "(58cf8f9c3 2026-10-13)" is the commit
     date in the version string, not a release date. 1.99.1 was released on 2026-10-15
     (RELEASES.md, as in R3).
  6. The STOP AND ASK block stays as written, as the round's history. The report records R3 below
     it and names the switch point between the two sessions (the FALLBACK clause).
  Prediction 1 is scored on the version actually pinned.

═══════════════════════════════════════════════════════════════════════════
▲▲▲ ADDENDUM 2 — SPEC 004 — ENDS HERE. NOTHING BELOW IS PART OF IT.      ▲▲▲
═══════════════════════════════════════════════════════════════════════════
