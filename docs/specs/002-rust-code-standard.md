# 002 — Engine language: Rust, and the code standard

Status: accepted (r2), 2026-10-02
Resolves: 001 §8 fork F1 (option A), and F2 for code.

## 1. Decision

The engine and all bindings are written in **Rust, edition 2024**. Android uses UniFFI bindings. Web and the Telegram Mini App use wasm-bindgen. Clients may be written in other languages, but they never compute calendar facts (001 §3).

## 2. Toolchain

- **Edition**: `edition = "2024"` in every crate.
- **Toolchain pin**: `rust-toolchain.toml` pins the stable version current when the repo is created, with `rustfmt`, `clippy`, and the `wasm32-unknown-unknown` target. Bump it deliberately in a PR, never implicitly.
- **MSRV**: declared as `rust-version` in the workspace `Cargo.toml`, equal to the pinned toolchain at start. CI builds on both MSRV and latest stable.
- **Licence**: `MIT OR Apache-2.0` for all code, the Rust ecosystem convention. Data licensing stays open (001 F2).

## 3. Workspace layout

```
Cargo.toml                 workspace (repo: zurkhai)
crates/tshes               L1 engine — the only crate that computes
crates/tshes-ffi           UniFFI wrapper for Android (thin)
crates/tshes-wasm          wasm-bindgen wrapper for web / TG (thin)
crates/zurkhai-rules       L2 schema, loader, validator
tools/zurkhai-diff         Q1 differential report (001 §6.5)
```

Wrappers contain conversion code only. If logic appears in a wrapper, it moves to core.

## 4. Rules for `tshes` (the engine crate)

Each rule states the failure it prevents. Rules marked **[CI]** are enforced mechanically.

### 4.1 No floating point **[CI]**
The calendar is exact rational arithmetic. Floats introduce rounding that can move a boundary by one day, which is invisible on most dates and wrong on a few.
→ `#![deny(clippy::float_arithmetic)]`. Fractions are represented as integer numerator/denominator pairs or as mixed-radix integers, following the paper's own representation.

### 4.2 Floor division and modulo are explicit **[CI]**
Janson's formulas use mathematical floor and mod. Rust's `/` and `%` truncate toward zero, which gives a different answer for negative operands. Negative values do occur, for example in epoch offsets.
→ `#![deny(clippy::integer_division, clippy::modulo_arithmetic)]`. All division goes through `div_euclid` / `rem_euclid`, or through named helpers (`floor_div`, `fmod`) with doc tests covering negative inputs. Where a formula uses the ceiling or the fractional part, the helper names that explicitly.

### 4.3 Overflow is checked in every profile **[CI]**
Debug builds panic on overflow and release builds wrap silently. Without this rule, a test suite that passes in debug says nothing about release.
→ `overflow-checks = true` in `[profile.release]`. CI runs the full test suite in **both** debug and release. Intermediate values use `i64`. Use `i128` only where a bound proves `i64` insufficient, and state that bound in a comment.

### 4.4 No panics on valid input **[CI]**
The engine runs inside an Android app and a browser, and a panic there is a crash.
→ `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]` in core. Out-of-range input returns `Err`. Internal invariants use `debug_assert!` *and* are covered by property tests, because a debug_assert alone enforces nothing in release.

### 4.5 No unsafe, no I/O, no clock, no locale **[CI]**
→ `#![forbid(unsafe_code)]`. Core takes dates as arguments and never reads "today". It has no `std::fs`, no `std::time`, and no environment access. Localised names live in a separate module as data, keyed by enum.

### 4.6 `no_std` (fork F6)
Core has no need for `std`, and `#![no_std]` with zero dependencies makes 4.5 mechanical and the WASM output smaller. The cost is that some ergonomics need `alloc`. **Default: `no_std` + no `alloc` in core**, unless M1 finds a concrete need for allocation, recorded in a spec.

### 4.7 Newtypes for every calendar quantity
Mixing up a tshes number, a month number and a lunar-mansion index is the most likely silent bug, since all three are small integers.
→ `Jdn(i64)`, `Tshes(u8)` (valid range 1..=30), `MonthNo(u8)`, `Mansion(u8)` and so on. Constructors validate and return `Result`. Fields are private. There are no `From<u8>` conversions that skip validation.

### 4.8 Every formula cites its source
→ A comment at each formula gives the source and its exact location, for example `// Janson (arXiv:1401.6285v?) §5, eq. (5.3)`. If a constant has no citation, CI review rejects it. "Derived" constants show their derivation in a doc comment or a test.

### 4.9 Dependencies
→ Core has **zero dependencies**. Dev-dependencies are allowed (`proptest`, a CSV reader for fixtures). `cargo-deny` runs in CI for licences and advisories across the workspace.

## 5. Error handling

- Core defines one `#[non_exhaustive] enum Error` with variants such as `OutOfRange`, `SkippedDay`, and `InvalidField`. No `thiserror` in core (4.9). It implements `core::fmt::Display` and `core::error::Error`.
- Wrappers map `Error` to their platform's error type 1:1, without collapsing variants.
- `to_gregorian` on a skipped tshes returns a typed `Err(SkippedDay)` rather than `None`, so that callers have to handle it.

## 6. Testing standard

- **Property tests** (`proptest`) for 001 P1–P4 and P7, over the full 1900–2100 range, with deterministic seeds committed for any failure found.
- **Fixture tests** for P5 and P6 read from `golden/reference/`. A fixture test that loads zero rows fails, so that an empty or missing file can't produce a silent pass.
- **Seeded control**: at least one test feeds a deliberately wrong fixture row and asserts that it fails. This proves the harness can fail.
- **Both profiles**: `cargo test` and `cargo test --release` (4.3).
- **WASM smoke test**: `wasm-pack test --node` on a handful of dates, checking that the output matches native.

## 7. CI gate (every PR)

```
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test  --workspace
cargo test  --workspace --release
cargo build -p tshes --target wasm32-unknown-unknown
wasm-pack test --node crates/tshes-wasm      (§6 WASM smoke test; once the crate exists)
cargo deny check
MSRV job: cargo +<msrv> test --workspace
```

## 8. Style

- `rustfmt` defaults. No `rustfmt.toml` unless a rule is argued in a PR.
- Clippy: `pedantic` at warn workspace-wide, plus the deny list in §4 for core.
- Public items in core have doc comments. Calendar terms use the transliteration given in `docs/glossary.md`, with one spelling per term (tshes, not tithi/tsé interchangeably). The glossary is created by the first round that names a calendar term in code (`DEBT.md` D-2).
- Commit messages reference the spec number.

## 9. Open forks

- F6 `no_std` without `alloc` in core (default above). Confirm in M1.
- F7 UniFFI vs hand-written JNI for Android. Default: UniFFI.
- F8 Whether `zurkhai-rules` evaluates conditions in Rust (shared across clients) or only validates them. Default: evaluates, so Android and web cannot diverge.

## Revisions

- r1: initial.
- r2: engine crates renamed `tshes-*` (001 F3). The engine name is tradition-neutral; the Buryat-specific crates (`zurkhai-rules`, `zurkhai-diff`) keep the project name.

— spec text ends here —
