#!/usr/bin/env bash
# The CI gate — spec 002 §7, defined once (spec 004 Item 4).
# The hosted workflow (.github/workflows/ci.yml) and local runs call this
# same script, so the two cannot drift.
#
# Usage: tools/gate.sh [LEG ...] — no args runs every leg; named args run
# only those legs. The gate self-test (tools/gate-self-test.sh) drives a
# named subset on scratch copies, so a gate run inside a scratch copy never
# recurses into its own self-test leg.
#
# Every leg prints the toolchain it ran on (METHOD rule 23) and its own
# PASS / FAIL; skipped legs are named with their reason (METHOD rule 65).
# Exit status: 0 iff every run leg passed.
set -u

GATE_ROOT="$(cd "$(dirname "$(readlink -f "$0")")/.." && pwd)"
MANIFEST="$GATE_ROOT/Cargo.toml"
# The caller's directory must not matter (spec 005 Item 1b). rustup chooses
# the toolchain from the current directory (rust-toolchain.toml, searched
# upward from the cwd), never from `--manifest-path`; measured, spec 005: run
# from outside the repository, the cargo legs ran on the machine's default
# stable instead of the pin. So the gate enters its own tree first.
cd "$GATE_ROOT" || exit 2
# The MSRV is read from the workspace manifest, never typed twice (002 §2:
# rust-version equals the pinned toolchain at start).
MSRV="$(sed -n 's/^rust-version = "\([0-9][0-9.]*\)".*/\1/p' "$MANIFEST" | head -n 1)"
CORE_SRC="$GATE_ROOT/crates/tshes/src"
CORE_TOML="$GATE_ROOT/crates/tshes/Cargo.toml"

RUN_LEGS=0
PASS_LEGS=0
FAIL_LEGS=0
SKIP_LEGS=0
FAILED_NAMES=""

run_leg() {
    local name="$1"
    shift
    echo "=== leg: $name"
    RUN_LEGS=$((RUN_LEGS + 1))
    if "$@"; then
        PASS_LEGS=$((PASS_LEGS + 1))
        echo "PASS $name"
    else
        FAIL_LEGS=$((FAIL_LEGS + 1))
        FAILED_NAMES="$FAILED_NAMES $name"
        echo "FAIL $name"
    fi
}

skip_leg() {
    echo "=== leg: $1"
    SKIP_LEGS=$((SKIP_LEGS + 1))
    echo "SKIP $1: $2"
}

leg_fmt() {
    echo "toolchain: $(rustc -V)"
    cargo fmt --manifest-path "$MANIFEST" --all --check
}

leg_clippy() {
    echo "toolchain: $(rustc -V)"
    cargo clippy --manifest-path "$MANIFEST" --workspace --all-targets -- -D warnings
}

leg_test() {
    echo "toolchain: $(rustc -V)"
    cargo test --manifest-path "$MANIFEST" --workspace
}

leg_test_release() {
    echo "toolchain: $(rustc -V)"
    cargo test --manifest-path "$MANIFEST" --workspace --release
}

leg_build_wasm() {
    echo "toolchain: $(rustc -V)"
    cargo build --manifest-path "$MANIFEST" -p tshes --target wasm32-unknown-unknown
}

leg_deny() {
    echo "tool: $(cargo deny --version)"
    cargo deny check
}

leg_msrv() {
    echo "toolchain: $(rustc +"$MSRV" -V)"
    cargo +"$MSRV" test --manifest-path "$MANIFEST" --workspace
}

# Spec 004 Item 5b: `#![no_std]` alone does not make 002 §4.5 mechanical —
# `#[cfg(test)] extern crate std;` compiles and passes the 002 §7 legs, so
# the gate checks the core's source itself.
leg_no_std_check() {
    echo "check: #![no_std] present, and no reach to std, in crates/tshes/src (002 §4.5, §4.6; F6 (a))"
    local ok=0
    local hits
    if ! grep -q '^#!\[no_std\]' "$CORE_SRC/lib.rs"; then
        echo "ERROR no-std-check: crates/tshes/src/lib.rs is missing #![no_std] (002 §4.6)"
        ok=1
    fi
    hits="$(grep -nE '\bextern[[:space:]]+crate[[:space:]]+std\b|\bstd::' -r "$CORE_SRC" 2>/dev/null || true)"
    if [ -n "$hits" ]; then
        echo "ERROR no-std-check: the core reaches std (002 §4.5):"
        echo "$hits"
        ok=1
    fi
    return "$ok"
}

# Spec 004 Item 5d: F6, ruled (a) — no_std and no alloc. `extern crate
# alloc;` compiles fine in a no_std crate, so the gate checks for it.
leg_no_alloc_check() {
    echo "check: no reach to alloc in crates/tshes/src (F6, ruled (a): no alloc)"
    local hits
    hits="$(grep -nE '\bextern[[:space:]]+crate[[:space:]]+alloc\b|\balloc::' -r "$CORE_SRC" 2>/dev/null || true)"
    if [ -n "$hits" ]; then
        echo "ERROR no-alloc-check: the core uses alloc (F6 (a)):"
        echo "$hits"
        return 1
    fi
    return 0
}

# Spec 004 Item 5d, rewritten by spec 005 Item 1a: 002 §4.9 — core has zero
# dependencies; dev-dependencies are allowed. A path dependency needs no
# network, so cargo-deny does not catch it. The leg reads the manifest as
# Cargo reads it (`cargo metadata --no-deps`): the text-based leg of spec 004
# passed a `[dependencies.<name>]` table and a `[target.'cfg(unix)'.dependencies]`
# section unseen (measured, spec 005). Cargo gives each dependency a kind:
# null (normal), "build" or "dev"; only "dev" passes.
leg_zero_deps_check() {
    echo "check: cargo metadata --no-deps lists no normal or build dependency for tshes; dev-dependencies allowed (002 §4.9)"
    local meta bad dev
    if ! command -v jq > /dev/null; then
        echo "ERROR zero-deps-check: jq is required to read cargo metadata and was not found"
        return 1
    fi
    if ! meta="$(cargo metadata --manifest-path "$MANIFEST" --no-deps --format-version 1)"; then
        echo "ERROR zero-deps-check: cargo metadata failed"
        return 1
    fi
    if ! printf '%s' "$meta" | jq -e '[.packages[] | select(.name == "tshes")] | length == 1' > /dev/null; then
        echo "ERROR zero-deps-check: cargo metadata does not list exactly one package named tshes"
        return 1
    fi
    bad="$(printf '%s' "$meta" | jq -r '.packages[] | select(.name == "tshes") | .dependencies[] | select(.kind != "dev") | "ERROR zero-deps-check: crates/tshes has a \(.kind // "normal") dependency on \(.name) (target: \(.target // "any")) (002 §4.9)"')"
    if [ -n "$bad" ]; then
        echo "$bad"
        return 1
    fi
    dev="$(printf '%s' "$meta" | jq -r '[.packages[] | select(.name == "tshes") | .dependencies[] | select(.kind == "dev") | .name] | if length == 0 then "none" else join(",") end')"
    echo "zero-deps ok: no normal or build dependency; dev-dependencies: $dev"
    return 0
}

leg_self_test() {
    echo "tool: bash $(bash --version | head -n 1)"
    bash "$GATE_ROOT/tools/gate-self-test.sh"
}

LEGS="${*:-fmt clippy test test_release build_wasm wasm_pack deny msrv no_std_check no_alloc_check zero_deps_check self_test}"
for leg in $LEGS; do
    case "$leg" in
        fmt) run_leg fmt leg_fmt ;;
        clippy) run_leg clippy leg_clippy ;;
        test) run_leg test leg_test ;;
        test_release) run_leg test_release leg_test_release ;;
        build_wasm) run_leg build_wasm leg_build_wasm ;;
        wasm_pack) skip_leg wasm-pack "crates/tshes-wasm does not exist (002 §7: this leg runs once the crate exists; METHOD rule 65)" ;;
        deny) run_leg deny leg_deny ;;
        msrv) run_leg msrv leg_msrv ;;
        no_std_check) run_leg no_std_check leg_no_std_check ;;
        no_alloc_check) run_leg no_alloc_check leg_no_alloc_check ;;
        zero_deps_check) run_leg zero_deps_check leg_zero_deps_check ;;
        self_test) run_leg self_test leg_self_test ;;
        *) echo "unknown leg: $leg" && exit 2 ;;
    esac
done

echo
echo "GATE: $RUN_LEGS legs run — $PASS_LEGS passed, $FAIL_LEGS failed, $SKIP_LEGS skipped"
if [ -n "$FAILED_NAMES" ]; then
    echo "GATE: failed legs:$FAILED_NAMES"
    exit 1
fi
exit 0
