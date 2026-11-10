spec: 004
prompt: docs/specs/prompts/004-workspace-gate.md
branch: spec/004-workspace-gate (cut from main at 1e89a5c)
merge SHA: not yet merged
pushed: no
model: deepseek-v4-pro (DeepSeek V4 Pro), reasoning "thinking at the highest setting" as CodeWhale displays it — halt session 2026-11-10 and resume session 2026-11-10, the same model and setting for both (R1, R2)
gate: 1 passed, 0 failed (debug) / 1 passed, 0 failed (release) — no failed tests; skipped legs: wasm-pack (crates/tshes-wasm does not exist)

Status: draft — the round is complete; status is set to accepted at landing
Depends on: 001, 002, 003
Resolves: none
Milestone: M1
Governs: the workspace root, crates/tshes, tools/, .github/workflows/

— spec text ends here —

STOP AND ASK — spec 004 Fork 1, which Rust stable to pin

DONE AND GREEN
  Item 0 complete on the branch: the prompt committed byte-identical (a0bac5e, `cmp` clean); 004
  verified free in the tree; FORKS.md F12 (RULED, R1) and DEBT.md D-4 (METHOD) filed (254c9c4); the
  OPEN count re-measured at 8 and audited. The Read-first list is read. Harness checks: H1 CLAUDE.md
  loaded (CodeWhale `project_instruction_imports`, quoting its hard rules); H2 permission posture
  Ask, `codewhale 0.10.2 (ff466b2ed0bf)`; H4 `git status --porcelain` empty before the branch.

THE FINDING
  Item 1's stop condition is met. `rustup check` reports the current stable as 1.99.1, released
  2026-10-13 [CORRECTED 2026-11-10: the date in rustup's version string "(58cf8f9c3 2026-10-13)" is
  the commit date, not a release date; 1.99.1 was released on 2026-10-15 per RELEASES.md, as in R3]
  — after 2026-10-02, the repository's first commit date. On 2026-10-02 the current stable
  was 1.99.0 (the initial 1.99 release; 1.99.1 is its patch). The prompt names this exact situation:
  "If rustup reports a stable released after 2026-10-02, STOP AND ASK which to pin." 002 §2 pins "the
  stable version current when the repo is created", which reads as 1.99.0; rustup's current stable
  reads as 1.99.1. The two readings differ, and the prompt reserves the choice for the human.

EVIDENCE
  MEASURED: `rustup check` (exit 100, "update available"):
    "stable-x86_64-unknown-linux-gnu - update available: 1.97.1 (8bab26f4f 2026-07-14) ->
    1.99.1 (58cf8f9c3 2026-10-13)"
  MEASURED: the repository's first commit: 2026-10-02 11:41:56 +0300, fda72eb "Initial commit"
    (`git log --reverse --format='%ai %h %s' | head -n 1`).
  INFERRED: 1.99.0 was the initial release of the 1.99 line and the stable current on 2026-10-02;
    rustup's own report dates 1.99.1, its patch, at 2026-10-13 [CORRECTED 2026-11-10: that string
    date is the commit date, not a release date; 1.99.1 was released on 2026-10-15 per RELEASES.md,
    as in R3].
  RECALLED / UNVERIFIED: the advisor's prediction 1 derived 1.99.0 from the six-week cadence counted
    from 1.85.0 (2025-02-20); it did not anticipate the 1.99.1 patch.

THE QUESTION
  Which version does the pin — and therefore the MSRV — name: 1.99.0, the stable current at the
  repository's creation (002 §2's literal reading), or 1.99.1, rustup's current stable now? The
  prompt's Item 1 makes this a STOP AND ASK, so it cannot be decided inside the round's scope.

OPTIONS — a starting set, not a menu
  1. Pin 1.99.0 — 002 §2's literal "current when the repo is created". Cost: the pin sits one patch
     behind the 1.99 line's fixes; rustup installs it by exact version only (the `stable` channel now
     serves 1.99.1).
  2. Pin 1.99.1 — rustup's current stable, the same 1.99 minor line with its patch. Cost: released
     after the repository was created, so the pin does not literally match 002 §2.
  3. Something else the adjudicator names — cost depends on the choice (e.g. the machine's installed
     1.97.1 predates the repository and would need edition-2024 checks).

IF THE ANSWER IS DELAYED
  Nothing continues. Every remaining item (2–6) names the pin: `rust-version`, `rust-toolchain.toml`,
  the MSRV gate leg and every local gate run. The halt commit holds Item 0's state and this fork.

THE RULING — R3 (2026-11-10, ADDENDUM 2; the human: "agree with all recommends")
  F13 is ruled: (a). Pin Rust 1.99.0, and set the MSRV (`rust-version`) to 1.99.0. 002 §2's text
  decides it: "the stable version current when the repo is created". Rust's RELEASES.md gives
  "Version 1.99.0 (2026-10-01)"; the repository's first commit is 2026-10-02 (RELAYED — the advisor
  FETCHED the RELEASES.md line on 2026-11-10; GRANTS give this round no network to re-fetch it).
  The ruling is written on F13's row and D-5 records the pin's cost (the FFI `bool` fix in 1.99.1).
  Switch point (the FALLBACK clause): the halt session stopped at 6c5fe19 with the STOP AND ASK
  above; a fresh session on the same model resumed on the same branch per ADDENDUM 2. The fallback
  model (Sonnet 5.5) was not needed; no design choice fell to it.

# Report — spec 004 (the resume's report, complete; the halt's is the block above)

## The decisive item

Item 5's table — rule; planted shape; expected diagnostic; observed diagnostic; verdict. Every plant
is otherwise-valid code planted into a scratch copy of the workspace (never the working tree); the
assertion is the diagnostic NAME (lint name, error code, or rustc's exact message where it renders
no code), never "the build failed". The clean control ran first and passed every driven leg.

| Plant | Rule | Planted shape | Expected diagnostic | Observed | Verdict |
|---|---|---|---|---|---|
| control | — | unmodified copy | every driven leg passes | every driven leg passed | PASS (5a) |
| b1 | 002 §4.1 | `let _ = 1.0_f32 + 2.0_f32;` | `clippy::float_arithmetic` | same | REJECTED |
| b2 | 002 §4.2 | `let _ = a / b;` (i32) | `clippy::integer_division` | same | REJECTED |
| b3 | 002 §4.2 | `let _ = a % b;` (i32) | `clippy::modulo_arithmetic` | same | REJECTED |
| b4 | 002 §4.4 | `Some(1_u8).unwrap()` | `clippy::unwrap_used` | same | REJECTED |
| b5 | 002 §4.4 | `Some(1_u8).expect("planted")` | `clippy::expect_used` | same | REJECTED |
| b6 | 002 §4.4 | `panic!("planted")` | `clippy::panic` | same | REJECTED |
| b7 | 002 §4.4 | `[0_u8; 4][i]` | `clippy::indexing_slicing` | same | REJECTED |
| b8 | 002 §4.5 | `unsafe { let _ = *p; }` | `unsafe_code` | same | REJECTED |
| b9 | 002 §4.5 clock | `std::time::SystemTime::now()` | `E0433` | same | REJECTED |
| b10 | 002 §4.5 filesystem | `std::fs::read` | `E0433` | same | REJECTED |
| b11 | 002 §4.5 environment | `std::env::var("planted")` | `E0433` | same | REJECTED |
| b12 | 002 §4.5 std route | `extern crate std;` + `std::time` at the crate root — compiles past `#![no_std]`; the 002 §7 legs pass it | `no-std-check` | same | REJECTED |
| b13 | 002 §4.5 std route | `#[cfg(test)] extern crate std;` + std in a test module — compiles; the 002 §7 legs pass it | `no-std-check` | same | REJECTED |
| b14 | 002 §4.5 std route | `println!("planted")` | `cannot find macro \`println\` in this scope` | same | REJECTED |
| b15 | 002 §4.5 std route | `#![no_std]` removed — the crate becomes a std crate; the 002 §7 legs pass it | `no-std-check` | same | REJECTED |
| c | 002 §4.3 | `overflow-checks = false` in `[profile.release]` | `test overflow_tripwire - should panic ... FAILED` | same | REJECTED (tripwire red) |
| d1 | F6 no alloc | `extern crate alloc;` + `alloc::vec::Vec::<u8>::new()` — compiles; the 002 §7 legs pass it | `no-alloc-check` | same | REJECTED |
| d2 | F6 no alloc | `vec![1_u8]` without `extern crate alloc` | `cannot find macro \`vec\` in this scope` | same | REJECTED |
| d3 | F6 no alloc | `Box::new(0_u8)` without `extern crate alloc` | `E0433` | same | REJECTED |
| d4 | 002 §4.9 | a path dependency added to crates/tshes (no network needed) | `zero-deps-check` | same | REJECTED |
| s8 | 002 §8 | `pub fn planted_must_use() -> u32 { 0 }` — pedantic at warn must be live in the member | `clippy::must_use_candidate` | same | REJECTED |

Corrections made during development, recorded as history: b12's first expectation was `E0463` — the
measurement showed `extern crate std;` compiles at the crate root of a `#![no_std]` crate (the
sysroot crate is reachable even though no_std removed std from the extern prelude), so the assertion
moved to the gate's own check (prediction 3's mechanism). b14 and d2 first expected `E0425` — rustc
1.99.0 renders the unresolved-macro error without a code, so the assertion is the exact message. c's
first pattern omitted " - should panic" from the passing line's shape. d1's first shape
(`Vec::new()` + `push`) tripped `clippy::vec_init_then_push` (a default-warn lint), so the shape was
made pedantic-clean; the no-alloc reach itself is silent to the 002 §7 legs, as measured.

The final gate line (Item 6, below): 1 passed, 0 failed (debug) / 1 passed, 0 failed (release) —
no failed tests; skipped legs: wasm-pack (crates/tshes-wasm does not exist).

## Every clause of 002 §2–§8, scored

Evidence shape: expected value, actual value, the assertion/command, the test or leg name. MEASURED
means read or run this round, against this tree.

- **§2 edition**: DONE. Expected `edition = "2024"` in every crate; actual: root `[workspace.package]
  edition = "2024"` and the member's `edition.workspace = true`; the release test build's rustc
  invocation (quoted from `cargo test --release --no-run -v`, MEASURED) carries `--edition=2024`.
- **§2 toolchain pin**: DONE. Expected: `rust-toolchain.toml` pinning the stable current at the
  repo's creation with rustfmt, clippy, wasm32. Actual: `channel = "1.99.0"`, `components =
  ["rustfmt", "clippy"]`, `targets = ["wasm32-unknown-unknown"]`, `profile = "minimal"` (file
  content); the toolchain quotes: `rustc 1.99.0 (b940084d7 2026-09-28)`, `cargo 1.99.0 (5f94df478
  2026-08-27)`; `rustup +1.99.0 component list --installed` shows clippy and rustfmt,
  `rustup +1.99.0 target list --installed` shows `wasm32-unknown-unknown`. Item 1, complete.
- **§2 MSRV**: DONE. Expected `rust-version` equal to the pin at start; actual:
  `[workspace.package] rust-version = "1.99.0"` (member inherits); the gate's msrv leg reads the
  value from the manifest (never typed twice) and runs `cargo +1.99.0 test --workspace` — it prints
  the same `rustc 1.99.0 (b940084d7 2026-09-28)` as the pin legs (prediction 6: no independent
  evidence this round).
- **§2 licence**: DONE. Expected `MIT OR Apache-2.0`; actual: `[workspace.package] license =
  "MIT OR Apache-2.0"`, member `license.workspace = true`; the deny leg reports `licenses ok`.
- **§3 layout**: DONE for what exists: `crates/tshes` exists and builds. N-A this round, with the
  reason, for: `crates/tshes-ffi` (waits for F7, which is OPEN — 002 §9; created by the round that
  needs it, by milestone — 001 §10), `crates/tshes-wasm` (same; the wasm-pack gate leg is SKIPPED
  because it does not exist), `crates/zurkhai-rules` (M4's round), `tools/zurkhai-diff` (the Q1
  report's round). `tools/` now holds `gate.sh` and `gate-self-test.sh` — the gate machinery this
  round builds, not `zurkhai-diff`.
- **§4.1 no floats [CI]**: DONE. Expected `#![deny(clippy::float_arithmetic)]`; actual: the
  attribute is in crates/tshes/src/lib.rs; assertion: plant b1 — clippy leg output contains
  `clippy::float_arithmetic` and the leg FAILs.
- **§4.2 div/mod explicit [CI]**: DONE. Expected deny `clippy::integer_division,
  clippy::modulo_arithmetic`; actual: the attributes are in lib.rs; assertion: plants b2 and b3.
- **§4.3 overflow checked [CI]**: DONE. Expected `overflow-checks = true` in `[profile.release]`;
  actual: the root Cargo.toml line; the mechanism test: `overflow_tripwire`
  (crates/tshes/tests/overflow_tripwire.rs) — `#[should_panic(expected = "attempt to add with
  overflow")]` over `black_box(i32::MAX) + black_box(1)`; it passes in both profiles
  (`test overflow_tripwire - should panic ... ok` in the debug AND the release runs, MEASURED); the
  operands go through `core::hint::black_box` because a literal `i32::MAX + 1` is a compile error
  via the `arithmetic_overflow` lint, and the mechanism must be a runtime overflow (METHOD rule 31).
  Plant c flips the setting off and the tripwire goes red. Which profile `cargo test --release`
  builds it with, from Cargo's own output: `cargo test --help`: "-r, --release — Build artifacts in
  release mode, with optimizations"; the rustc invocation for the test (from
  `cargo test --release --no-run -v`, fresh target dir, MEASURED) carries `-C opt-level=3 -C
  overflow-checks=on --test` on `/home/kitaeza/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/
  bin/rustc` — i.e. the release profile, whose defaults Cargo's docs name ("optimizations") and
  whose `overflow-checks` value comes from `[profile.release]` (release's default is off; the
  setting turns it on). No network was used for this citation (H3); it is local output.
- **§4.4 no panics on valid input [CI]**: DONE. Expected deny `clippy::unwrap_used,
  clippy::expect_used, clippy::panic, clippy::indexing_slicing`; actual: the attributes are in
  lib.rs; assertion: plants b4–b7.
- **§4.5 no unsafe, no I/O, no clock, no locale [CI]**: DONE. Expected `#![forbid(unsafe_code)]`
  plus a core that reaches none of the standard library's clock, filesystem or environment; actual:
  the attribute is in lib.rs (plant b8 → `unsafe_code`), and the std routes are planted and
  rejected: b9–b11 (compiler, E0433), b14 (compiler, the macro message), b12/b13/b15 (the 002 §7
  legs pass them — `#![no_std]` alone does NOT make §4.5 mechanical, MEASURED, prediction 3 held at
  the crate root too; the gate's `no-std-check` leg rejects each). No locale code exists anywhere.
- **§4.6 no_std (F6)**: DONE — F6, ruled (a), confirmed in M1 as 002 §9 asks. Expected `#![no_std]`
  with no alloc; actual: `#![no_std]` in lib.rs, zero uses of alloc; `no-alloc-check` asserts the
  source; plants d1–d3 show alloc reaches the core past `#![no_std]` and is rejected (d1 by the
  check, d2/d3 by the compiler). No concrete need for alloc was found: the core has zero items.
- **§4.7 newtypes**: N-A — no calendar quantity exists yet; no newtype is written (no calendar code
  lands this round).
- **§4.8 formula citations**: N-A — no formula and no constant land this round.
- **§4.9 zero dependencies**: DONE. Expected zero `[dependencies]` and zero dev-dependencies in
  core; actual: crates/tshes/Cargo.toml has none (the `zero-deps-check` leg asserts
  `[dependencies]`/`[dev-dependencies]`/`[build-dependencies]` are absent); no dev-dependency was
  needed — the tripwire needs only `core::hint::black_box`; `cargo deny check` passes
  (`advisories ok, bans ok, licenses ok, sources ok`, cargo-deny 0.20.2) with deny.toml committed.
  Plant d4 shows a path dependency (no network) passes the 002 §7 legs and is rejected only by the
  check.
- **§5 error handling**: N-A — no Error type, no conversion, no skipped-tshes path exists; Item 3
  says they land with the first calendar round.
- **§6 testing standard**: N-A — property tests, fixtures, the seeded wrong row and the WASM smoke
  test are the calendar rounds' instruments; this round's only test is the §4.3 tripwire. The
  seeded-control idea is realized where it applies: the gate self-test itself is this round's
  instrument, proven able to fail (Item 5).
- **§7 CI gate**: DONE as written this round. Every leg lives in one script, tools/gate.sh — fmt,
  clippy (`-D warnings`), test, test --release, build wasm32, wasm-pack, deny, MSRV — and the hosted
  workflow (.github/workflows/ci.yml) and local runs call that same script, so the two cannot drift.
  The wasm-pack leg is SKIPPED and named with its reason: crates/tshes-wasm does not exist (METHOD
  rule 65). The MSRV leg runs `cargo +<msrv> test --workspace` with the MSRV read from the manifest.
  The hosted workflow also builds and tests on latest stable (002 §2; `cargo +stable`, both
  profiles). Third-party actions the workflow uses, and their pinning: `actions/checkout@v4` — pinned
  by major-version tag; pinning by commit SHA needs a network lookup this round's GRANTS do not
  include, so the tag pin is the honest one and is stated here. No other third-party action: rustup
  and cargo come with the runner, and the toolchain itself is pinned by the repo's
  rust-toolchain.toml (rustup's shim installs it). The workflow is reasoned, not run (METHOD rule
  17) — rounds never push; it first executes at the human's first push. "CI green" in this round
  means every leg of the script passing locally on the final committed tree (Item 6, below). The
  self-test leg is not in 002 §7's list: filed as D-7 (Kind: DOCS), TRIGGER: the next revision of
  002.
- **§8 style**: DONE. rustfmt defaults, no rustfmt.toml (none exists in the tree; `cargo fmt --all
  --check` passes). Clippy pedantic at warn, workspace-wide: `[workspace.lints.clippy] pedantic =
  "warn"` in the root and `[lints] workspace = true` in the member — MEASURED that the workspace
  table alone does NOT reach members (plant s8 fired nothing before the opt-in and fires
  `clippy::must_use_candidate` after; the self-test pins this with s8). Public items have doc
  comments: N-A — the core has no public items. Calendar terms: D-2 stays OPEN (below); the only
  calendar term written in code is the crate name `tshes`, fixed by F3 (ruled 2026-10-02). Commit
  messages reference the spec number: DONE — every commit subject on the branch starts "spec 004".

## 001 §6.4 P1–P7

N-A, all seven, one reason: they are properties of the calendar engine, and no calendar arithmetic
lands this round (Item 0 says so in terms). The only test is the overflow tripwire, which tests the
build's overflow mechanism, not a calendar fact. P1–P7 are the M1 calendar rounds' properties.

## The predictions

1. HELD — the pin is Rust 1.99.0 (`rustc 1.99.0 (b940084d7 2026-09-28)`; rustup: "latest update on
   2026-10-01 for version 1.99.0"), scored on the version actually pinned (ADDENDUM 2).
2. HELD — every Item 5b plant (b1–b15) is rejected, each with its named diagnostic (table above).
3. HELD, and stronger than predicted — three shapes reach std past `#![no_std]` and pass the gate as
   002 §7 lists it (b12 root `extern crate std;`, b13 the cfg(test) route the advisor predicted, b15
   the attribute removed), and alloc does the same (d1); the gate needed checks of its own —
   `no-std-check` and `no-alloc-check` — and each plant was planted again and rejected by name.
4. HELD — the tripwire passes in debug and in release ("test overflow_tripwire - should panic ...
   ok" in both profiles, MEASURED) and goes red in Item 5c's control (plant c).
5. HELD — the final gate (Item 6): every leg passes, deny included; wasm-pack SKIPPED; the only
   tests counted are the tripwires, one passing in each profile (counts in the gate line).
6. HELD — the MSRV leg runs the same toolchain as the pin (both print
   `rustc 1.99.0 (b940084d7 2026-09-28)`), so this round it adds no independent evidence; the
   report says so.
7. Not scored here — the advisor's mechanical checks (ADVISOR.md §6, step 3); the report predicts
   nothing and the advisor scores it at landing.

## The toolchain inventory

- `rustc 1.99.0 (b940084d7 2026-09-28)` — `rustc +1.99.0 -Vv` in full: binary rustc, commit-hash
  b940084d7eb6a299eb4bfeb8e34901bc051e7ac4, commit-date 2026-09-28, host x86_64-unknown-linux-gnu,
  release 1.99.0, LLVM 23.1.1.
- `cargo 1.99.0 (5f94df478 2026-08-27)`.
- rustup 1.29.0; its self-update to 1.29.3 failed ("failed to set permissions for
  '/home/kitaeza/.cargo/bin/rustup-init': No such file or directory (os error 2)") — the named
  install (the toolchain) succeeded; this failure is reported here as a flag, not worked around.
- Installed on 1.99.0: rustc, cargo, rustfmt, clippy, rust-std (host), rust-std-wasm32; targets
  installed: x86_64-unknown-linux-gnu, wasm32-unknown-unknown.
- cargo-deny 0.20.2 (`cargo install --locked cargo-deny`; the workflow pins `--version 0.20.2`).
- The gate's legs print their own toolchain (METHOD rule 23): the final gate's per-leg lines are
  quoted in the gate section below.

## D-2's state

OPEN (DEBT.md D-2). This round writes the crate name `tshes` only — fixed by the human's ruling F3
(FORKS.md). The glossary is created by the first round that writes calendar code, from a source; the
report says so, as Item 3 requires.

## The open-forks audit, with its arithmetic (METHOD rule 60)

Audit grep (the one FORKS.md's header defines): `grep -c '^- \*\*Status:\*\* OPEN$' FORKS.md`.
Expected = previous count + raised − ruled, at every commit:

- a0bac5e (prompt): expected 8 (the advisor counted 8 at 1e89a5c; re-measured, not copied:
  `git show a0bac5e:FORKS.md | grep -c ...` = 8) → 8 ✓
- 254c9c4 (F12 filed as RULED, D-4): 8 + 0 − 0 = 8 → 8 ✓
- 6c5fe19 (F13 filed OPEN, the halt): 8 + 1 − 0 = 9 → 9 ✓
- 5653e3c (R3 onto F13): 9 + 0 − 1 = 8 → 8 ✓
- 7db2f95, 1ae3c0b, the Item 5 commit, the final commit: 8 + 0 − 0 = 8 → 8 ✓ each

The halt-era counts were re-measured from the tree this session (git show), never copied from the
halt's prose. The rows: F2, F4, F5, F7, F8, F9, F10, F11 OPEN; F1, F3, F6, F12, F13 RULED.

Item 6's 002 grep, named: `grep -nE '\bF[0-9]+\b' docs/specs/002-rust-code-standard.md` — hits:
F1 (ruled), F2 (code half ruled; data licence OPEN), F6 (ruled (a); "Confirm in M1" — confirmed by
this round: no_std, no alloc, no need found), F7 (OPEN, matches FORKS.md), F8 (OPEN, matches), F3
(ruled). No fork without a ruling is built through: F7 and F8 govern crates this round does not
create, and F6's confirmation is this round's Item 3 + no-alloc-check.

## The marker (METHOD rule 58)

The round's marker is the word `MARK004`; it does not occur in the prompt file, so the grep finds
only what this round wrote (the spec never spells it). The grep, named:
`grep -rn --exclude-dir=.git --exclude-dir=.codewhale --exclude-dir=target 'MARK004' <WORKDIR>`.

- Before the three halt-era commits (a0bac5e, 254c9c4, 6c5fe19): the tree records no marker word
  and no grep evidence from that session. Per ADDENDUM 2 fix #1: I say so; I do not reconstruct a
  grep that was not recorded.
- Before 5653e3c (the resume's first commit): 0 hits — the marker was chosen this session and
  nothing had been written yet.
- Before 7db2f95, 1ae3c0b, the Item 5 commit: exactly 1 hit — the placeholder on this report's
  `gate:` line.
- Before the final commit: exactly 2 hits — both in this section (the word itself, and the grep
  that found it). The three gate placeholders were flipped to the measured gate line; no instance
  of the token stands for anything unfilled.

## Diff scope

Against main at 1e89a5c, through the Item 5 commit: +746/−0 across 13 files, plus the Item 5 commit
and the final commit, shown summing:

- `.github/workflows/ci.yml` +48, `Cargo.lock` +7, `Cargo.toml` +20, `DEBT.md` +32, `FORKS.md` +14,
  `crates/tshes/Cargo.toml` +9 (+5 in the Item 5 commit), `crates/tshes/src/lib.rs` +23,
  `crates/tshes/tests/overflow_tripwire.rs` +18, `deny.toml` +20,
  `docs/specs/004-workspace-gate.md` +73 (this report, replaced at the final commit),
  `docs/specs/prompts/004-workspace-gate.md` +308 (the prompt plus ADDENDUM 2, byte-identical),
  `rust-toolchain.toml` +9, `tools/gate.sh` +165, `tools/gate-self-test.sh` +230 (the Item 5
  commit), `docs/specs/INDEX.md` +1 (the final commit). Lines −: 4 in this report across the
  rewrites; nothing else deleted.
- Gated paths touched: `crates/tshes/**` — YES, under accepted spec 002, named in the prompt's
  GATED line. This round's own `docs/specs/004-workspace-gate.md`, its committed prompt and its
  `INDEX.md` row — the prompt's GATED line says these are not changes to an accepted spec. No other
  gated path is touched; `golden/**` and `docs/METHOD.md` are untouched.
- Files the harness wrote: `.codewhale/state/` (subagents.v1.json, subagents.v1.lock, .gitignore) —
  never committed, never added to .gitignore (H4); accounted for in every pre-commit
  `git status --porcelain`; D-6 filed (Kind: TOOLING, TRIGGER: the next round run in CodeWhale).
  Also, scratch probe copies under `/home/kitaeza/github/zurkhai/target/` (`zz-pedantic-probe`,
  `zz-alloc-probe`) created per GRANTS ("or under target/") when the guardian refused /tmp copies;
  gitignored build scratch; their removal was refused by the guardian (`rm` denied) and they are
  listed here as left behind (METHOD rule 49's "or filed as left behind") — they are deleted at the
  first permitted cleanup.
- Files the test framework wrote: none (no proptest, no seeds, no snapshots).

## Forks encountered

- **F13 (spec 004 Fork 1)** — raised by the halt (block above), RULED by R3, (a), recorded on the
  row in 5653e3c.
- **F12** — filed RULED (R1) at 254c9c4; not reopened.
- Resolved without escalating, as "considered X, chose Y because Z":
  1. The three std shapes that pass the 002 §7 legs (b12/b13/b15): considered whether `#![no_std]`
     makes 002 §4.5 mechanical — no, measured; chose the `no-std-check` leg (Item 5b's own mandate:
     "add a check to the gate that rejects it, and plant again") because a grep over the core's
     source is local, cheap and catches all three shapes by name.
  2. Alloc (d1): F6's "no alloc" is ruled, not [CI]; the 002 §7 legs pass `extern crate alloc;` —
     chose `no-alloc-check` (grep for `extern crate alloc` / `alloc::`), local and cheap; d2/d3 show
     the compiler already rejects the macro/type routes without the extern crate.
  3. Zero dependencies (d4): a path dependency needs no network and carries a licence field, so
     cargo-deny does not see it — chose `zero-deps-check` (grep for the dependency sections), local
     and cheap.
  4. The pedantic opt-in: measured that `[workspace.lints.clippy]` alone does not reach members —
     chose the member's `[lints] workspace = true` (002 §8's "workspace-wide" must actually apply)
     and pinned it with self-test plant s8 (latitude, reported below).
  5. The tripwire's literal `i32::MAX + 1` is a compile error (`arithmetic_overflow`) — chose
     `core::hint::black_box` operands so the overflow is a runtime event, the mechanism itself
     (METHOD rule 31).
  6. cargo-deny 0.20.2 removed `[licenses]` keys (unlicensed/copyleft/default/allow-osi-fsf-free)
     and the `version` key in `[bans]`/`[sources]` — chose the tool's own template shape
     (`cargo deny init`), allow list `MIT`, `Apache-2.0`, defaults elsewhere.
  7. The latest-stable workflow job runs both debug and release tests — 002 §2 says "builds and
     tests", §4.3 says both profiles.
  8. `actions/checkout` pinned by major tag — SHA pinning needs a network lookup GRANTS do not
     include; stated in the workflow's own header comment.
  9. The workflow installs nothing but cargo-deny (pinned version); the toolchain comes from
     rust-toolchain.toml via rustup's shim, so the pin is defined once.
  10. The self-test's scratch copies exclude `.git`, `target`, `.codewhale`; each copy builds into
     its own `CARGO_TARGET_DIR` (METHOD rule 64); the self-test drives every leg except itself
     (recursion) and deny (the advisory database; with zero third-party crates no planted shape can
     change what deny checks) — documented in the script's header.

## Latitude taken

1. The three added gate legs (no-std-check, no-alloc-check, zero-deps-check) — mandated by Item
   5b/5d but beyond 002 §7's list; D-7 files the self-test leg for 002's next revision.
2. Self-test plant s8 — pins 002 §8's pedantic-at-warn, not in Item 5's named list; same mechanism.
3. The member `[lints] workspace = true` — required to make 002 §8 true; measured, not assumed.
4. The Item 3 citation comes from local output (`cargo test --help`, `-v` rustc args), per H3.
5. The latest-stable job runs both profiles (002 §2 + §4.3).
6. deny.toml is minimal and template-derived.
7. gate.sh prints each leg's toolchain (METHOD rule 23) — the report quotes those lines.
8. gate.sh uses a subshell cd for `cargo deny check` (the tool has no --manifest-path flag); the
   session's own calls keep the no-cd rule.
9. Probe scratch moved from /tmp to `target/` when the guardian refused /tmp copies — within GRANTS.
10. The overflow tripwire uses `core::hint::black_box` — reported above (fork 5).

## Numbers that must sum, shown summing

- Forks per commit: 8, 8, 9, 8, 8, 8, 8, 8 — shown with the arithmetic in the audit section.
- Gate legs: 11 defined (fmt, clippy, test, test_release, build_wasm, wasm-pack, deny, msrv,
  no_std_check, no_alloc_check, zero_deps_check, self_test — 12 named in the case list, wasm-pack
  counts as skipped, so 11 run + 1 skipped on a full run). The Item 4 subset run: 10 run, 10
  passed, 0 failed, 1 skipped. The final run's counts are in the gate section.
- Self-test: 20 plants at the first full run (16 passed, 4 corrected, then 20/20); the committed
  script has 21 plants (s8 added): control + 21 = 22 gate runs in the final run, 21 rejections
  expected.
- Tests: 1 test (overflow_tripwire) — 1 passed in debug, 1 passed in release; the msrv leg runs the
  same debug binary (prediction 6), not counted again.
- 002 clauses: 8 sections scored; §4 and §7 each split into their clauses; the DONE + N-A entries
  cover every clause of §2–§8.
- Diff: +746 through the Item 5 commit + 230 (self-test) + 5 (member Cargo.toml) + 1 (INDEX row) +
  this report's replacement — every file listed in diff scope.

## What the prompt got wrong

- The halt's "1.99.1, released 2026-10-13" — the date in rustup's version string is the commit
  date, not a release date; 1.99.1 was released on 2026-10-15 (RELEASES.md, R3). Corrected in place
  here and on F13's Question line, marked `[CORRECTED 2026-11-10: …]` (ADDENDUM 2 fix #5).
- Nothing else found: the four stop rules were hit exactly once (the Item 1 stop, now ruled), the
  [CI] lints all exist under 002's names on 1.99.0, prediction 3's mechanism held (and held at the
  crate root, beyond the advisor's prediction), and every GRANTS-named install succeeded. "None
  found" beyond the dated correction.

## By-catch

- `[workspace.lints.clippy]` does not reach members without the member's `[lints] workspace = true`
  (measured) — fixed this round; not filed (fixed, not a gap).
- `clippy::vec_init_then_push` fires on `Vec::new()` + `push` by default — shaped plant d1; not
  filed.
- rustup's self-update failure (1.29.0 → 1.29.3) — the user's machine, not the repo; reported as a
  flag; not filed.
- cargo-deny 0.20.2's config-key removals — reflected in deny.toml; not filed.
- `.codewhale/state/` — D-6 (TOOLING).
- The Auto-Review guardian denials mid-resume (cargo on copies, the untracked self-test script, rm,
  git commit) — recorded under the harness section below; every denial was reported, none worked
  around; not repo debt.

## Debt and forks

- D-5 (TOOLING) filed — the pin 1.99.0 predates 1.99.1's FFI `bool` fix; TRIGGER: the round that
  creates crates/tshes-ffi.
- D-6 (TOOLING) filed — CodeWhale's session state inside the repo; TRIGGER: the next round run in
  CodeWhale.
- D-7 (DOCS) filed — the self-test leg is not in 002 §7's list; TRIGGER: the next revision of 002.
- D-2 stays OPEN (the glossary; the report says so).
- The open-forks audit with arithmetic and the marker greps: sections above.

## Held

D-2 (OPEN), D-5, D-6, D-7 (filed this round, OPEN); F7 and F8 (002 §9, OPEN — govern crates later
rounds create); F11 (OPEN — the branch stays local, neither pushed nor deleted); the wasm-pack leg
stays SKIPPED until crates/tshes-wasm exists (named in the gate line).

## The harness (H1–H7, ADDENDUM 1)

- **H1**: CLAUDE.md is loaded — CodeWhale's `project_instruction_imports` shows it, quoting its own
  content; e.g. "CLAUDE.md — rules for every session in this repo" and "1. **One reviewed unit per
  session.**". Re-checked at the resume: still loaded. No AGENTS.md or other instruction file was
  created.
- **H2**: at each session start the posture was Ask (quoted: "Current permission posture: Ask");
  `codewhale 0.10.2 (ff466b2ed0bf)`. GRANTS' installs were approved by the human as they ran
  (rustup 1.99.0, cargo-deny 0.20.2, the advisory database). During the resume the human adjusted
  the posture (observed: Full Access, then Auto-Review); under Auto-Review the guardian denied
  `cargo clippy` on scratch copies, the untracked self-test script, `rm` of the probe scratch, and
  the git commit chain — each denial is recorded here with its stated reason, and none was worked
  around; the denied steps were re-attempted only after the human's "approve". The report names the
  modes as observed.
- **H3**: no web search or fetch tool was used; the Cargo citation in §4.3 is local output
  (`cargo test --help`, `cargo test --release --no-run -v`), labelled MEASURED.
- **H4**: `.codewhale/state/` — never committed, never added to .gitignore; listed in diff scope;
  D-6 (TOOLING) with TRIGGER: the next round run in CodeWhale. `git status --porcelain` was empty
  before the branch (halt session) and accounted for the files at every resume commit.
- **H5**: every commit this session — and the squash message below — ends with exactly
  `Co-Authored-By: DeepSeek V4 Pro <noreply@deepseek.com>`; no other trailer was added, and none
  that the harness might add was kept.
- **H6**: the `model:` line above gives the model ID as CodeWhale displays it and the reasoning
  setting by its name as CodeWhale displays it — one entry per session; "roster route" removed
  (ADDENDUM 2 fix #3).
- **H7**: no sub-agents ran in the halt session (its report names none) and none ran in the resume
  session — all work was done in-session.

## The stop rules (named)

The prompt's four: (1) a 002 §2–§4/§7/§8 requirement unmet on the pinned toolchain — not hit; every
clause met as written (the deny.toml shape followed the tool's own 0.20 template, no 002 requirement
was substituted). (2) a [CI] plant the gate could not reject — not hit; every plant rejected with a
named diagnostic, the three silent std routes and alloc closed by gate checks added per Item 5. (3)
an install or permission denied — the halt hit Item 1's stop (STOP AND ASK F13, ruled R3); in the
resume, the guardian's denials were each reported and none worked around; the named installs
succeeded (the rustup self-update failure is a machine-level flag, reported). (4) calendar terms
beyond the crate name — not hit; the only term written is `tshes` (F3). The FALLBACK clause: the
switch point is named above the report.

## Questions the prompt asked, answered

- "State which Cargo profile `cargo test --release` builds it with and how that profile relates to
  `[profile.release]`" — §4.3's evidence (release profile; `-C opt-level=3 -C overflow-checks=on`
  from Cargo's own output).
- "Quote `rustc -Vv` and `cargo -V` of the pinned toolchain" — toolchain inventory.
- "The report lists every third-party action the workflow uses and how each is pinned" — §7.
- "A `cargo deny` failure on a network fetch is classified by METHOD rule 63's procedure" — the one
  deny failure this round was the sandbox's read-only lock path on a restricted run, not a network
  fetch; it passed in isolation under the GRANTS environment and again in every subsequent run —
  classified by that procedure, not by feeling.
- "The gate line follows docs/ROUNDS.md §4.1: counts, never 'green', every skipped leg named" — the
  gate section below.
- "grep 002 for forks with no ruling and compare with FORKS.md; name the grep" — the audit section.
- Item 5d's "considered X, chose Y because Z" for F6's alloc and §4.9's zero dependencies — forks 2
  and 3.
- ADDENDUM 2 fixes 1–6 — the marker section (1), the audit arithmetic (2), the model line (3), H7
  (4), the `[CORRECTED]` markers (5), the STOP AND ASK block kept as history with R3 below it and
  the switch point named (6).
- The workspace's tests could not be run once before building, because no workspace existed at the
  halt — said, not skipped in silence (ADDENDUM 2 Item 0; the resume's first build was the
  workspace's first build).

## The gate (Item 6 — the final run, both profiles, on the final committed tree, no edits in flight)

The gate ran with the working tree clean, on commit e355e4a (the final commit changes only this
report file, which no gate leg reads). One command: `bash tools/gate.sh` — every leg, in order.
Summary line, quoted from the gate's own output:

    GATE: 11 legs run — 11 passed, 0 failed, 1 skipped

The skipped leg, named with its reason: wasm-pack — crates/tshes-wasm does not exist (002 §7: this
leg runs once the crate exists; METHOD rule 65).

The toolchain each leg ran on, quoted from the gate's own output (METHOD rule 23): every cargo leg
(fmt, clippy, test, test_release, build_wasm, msrv) printed
`toolchain: rustc 1.99.0 (b940084d7 2026-09-28)` — the pinned toolchain, both profiles. The deny
leg printed `tool: cargo-deny 0.20.2` and then `advisories ok, bans ok, licenses ok, sources ok`.
The self-test leg printed `tool: bash GNU bash, version 5.3.9(1)-release (x86_64-pc-linux-gnu)`.
The three source-check legs are greps; their patterns are quoted in the leg output itself.

The test counts, both profiles, quoted from the runs: debug and release each ran the same one test
and each reported `test overflow_tripwire - should panic ... ok` and
`test result: ok. 1 passed; 0 failed; 0 ignored; ...` — the only test in the workspace is Item 3's
tripwire, passing in both profiles. The msrv leg runs the same toolchain as the pin and therefore
the same binary (prediction 6): its 1 passing test is not counted again.

The self-test leg, quoted from its own summary: `SELF-TEST: 21 plants — 21 rejected with their
named diagnostic, 0 failures` — the clean control copy passed every driven leg first, and the
scratch directory was deleted at the end (METHOD rules 31, 49, 64). Every row of the Item 5 table
above is confirmed by this run.

The hosted workflow (.github/workflows/ci.yml) runs this same script: reasoned, not run, until the
human's first push (METHOD rule 17). "CI green" in this round is the run quoted here, on the final
committed tree.

A `cargo deny` failure on a network fetch would be classified by METHOD rule 63's procedure; the
only deny failure this round was the sandbox's read-only lock path on a restricted run, and it
passed in isolation under the GRANTS environment and in every subsequent run — classified by that
procedure, not by feeling.

## The squash commit message

```
spec 004: the workspace and the gate

Cut the Cargo workspace for the zurkhai engine: edition 2024, the Rust 1.99.0
pin (rust-toolchain.toml with rustfmt, clippy, wasm32; MSRV 1.99.0, ruled on
F13), and the empty #![no_std] crates/tshes core with the 002 §4 deny list and
the §4.3 overflow tripwire. Defined the 002 §7 gate once, in tools/gate.sh —
every leg (wasm-pack SKIPPED: crates/tshes-wasm does not exist), the same
script hosted (.github/workflows/ci.yml, plus latest-stable) and local — plus
the no-std, no-alloc and zero-deps checks the gate self-test
(tools/gate-self-test.sh, 21 plants) proved load-bearing: extern crate std
compiles past #![no_std], and pedantic-at-warn needs the member lints opt-in.
Both profiles green on the pinned toolchain; cargo deny clean; the MSRV leg
runs the same toolchain as the pin. FORKS.md F13 ruled (a); DEBT D-5, D-6, D-7.

Co-Authored-By: DeepSeek V4 Pro <noreply@deepseek.com>
```
