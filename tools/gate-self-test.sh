#!/usr/bin/env bash
# Spec 004 Item 5 — the gate self-test (METHOD rule 31: prove the instrument
# can fail before trusting that it didn't).
#
# Plants otherwise-valid violations into scratch copies of the workspace,
# each copy with its own target directory (METHOD rule 64), and asserts the
# diagnostic NAME each must produce — a lint name or an error code, never
# "the build failed", which a typo also produces. The scratch directory is
# created under the system temp directory and deleted at the end (METHOD
# rule 49); the working tree is never touched (spec 004 GRANTS).
#
# Order: a clean control first (Item 5a — a self-test whose clean copy fails
# proves nothing), then the [CI] plants (Item 5b), the overflow control
# (Item 5c), and the ruled-but-unmarked plants (Item 5d).
#
# The legs each copy drives: everything except the self-test itself (a gate
# run inside a copy must not recurse into its own self-test) and cargo deny
# (it needs the advisory database; the workspace has zero third-party
# crates, so no planted shape can change what deny checks).
set -u

SELF_DIR="$(dirname "$(readlink -f "$0")")"
REPO_ROOT="$SELF_DIR/.."
SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/zurkhai-gate-self-test.XXXXXX")"
LEGS="fmt clippy test test_release build_wasm msrv no_std_check no_alloc_check zero_deps_check"

PLANTS=0
FAILURES=0

cleanup() {
    rm -rf "$SCRATCH"
}
trap cleanup EXIT

copy_workspace() { # $1: copy name
    mkdir -p "$SCRATCH/$1"
    tar -C "$REPO_ROOT" --exclude=.git --exclude=target --exclude=.codewhale -cf - . | tar -C "$SCRATCH/$1" -xf -
}

run_gate() { # $1: copy name — runs the copy's gate, logs to "$SCRATCH/$1.log"
    export CARGO_TARGET_DIR="$SCRATCH/target-$1"
    bash "$SCRATCH/$1/tools/gate.sh" $LEGS > "$SCRATCH/$1.log" 2>&1
}

plant_lines() { # plant_lines <copy> <line>... — appends to the copy's lib.rs
    local copy="$1"
    shift
    printf '%s\n' "$@" >> "$SCRATCH/$copy/crates/tshes/src/lib.rs"
}

expect_fail() { # expect_fail <copy> <expected-diagnostic-ERE>
    local copy="$1"
    local pattern="$2"
    PLANTS=$((PLANTS + 1))
    local rc=0
    run_gate "$copy"
    rc=$?
    if [ "$rc" -eq 0 ]; then
        echo "FAIL plant $copy: the gate passed; expected it to reject with: $pattern"
        FAILURES=$((FAILURES + 1))
    elif grep -E "$pattern" "$SCRATCH/$copy.log"; then
        echo "PASS plant $copy: the gate rejected it with: $pattern"
    else
        echo "FAIL plant $copy: the diagnostic '$pattern' is not in the gate output"
        FAILURES=$((FAILURES + 1))
    fi
}

# --- Item 5a: the clean control -----------------------------------------
copy_workspace control
if run_gate control; then
    echo "PASS control: the unmodified copy passes every driven leg"
else
    echo "FAIL control: the unmodified copy does not pass every driven leg"
    FAILURES=$((FAILURES + 1))
fi

# --- Item 5b: the [CI] rules ---------------------------------------------
# 002 §4.1 — no floating point.
copy_workspace b1_float
plant_lines b1_float 'pub fn planted_float() { let _ = 1.0_f32 + 2.0_f32; }'
expect_fail b1_float 'clippy::float_arithmetic'

# 002 §4.2 — `/` on signed operands.
copy_workspace b2_div
plant_lines b2_div 'pub fn planted_div(a: i32, b: i32) { let _ = a / b; }'
expect_fail b2_div 'clippy::integer_division'

# 002 §4.2 — `%` on signed operands.
copy_workspace b3_mod
plant_lines b3_mod 'pub fn planted_mod(a: i32, b: i32) { let _ = a % b; }'
expect_fail b3_mod 'clippy::modulo_arithmetic'

# 002 §4.4 — unwrap.
copy_workspace b4_unwrap
plant_lines b4_unwrap 'pub fn planted_unwrap() { let _ = Some(1_u8).unwrap(); }'
expect_fail b4_unwrap 'clippy::unwrap_used'

# 002 §4.4 — expect.
copy_workspace b5_expect
plant_lines b5_expect 'pub fn planted_expect() { let _ = Some(1_u8).expect("planted"); }'
expect_fail b5_expect 'clippy::expect_used'

# 002 §4.4 — panic.
copy_workspace b6_panic
plant_lines b6_panic 'pub fn planted_panic() { panic!("planted"); }'
expect_fail b6_panic 'clippy::panic'

# 002 §4.4 — indexing.
copy_workspace b7_index
plant_lines b7_index 'pub fn planted_index(i: usize) { let _ = [0_u8; 4][i]; }'
expect_fail b7_index 'clippy::indexing_slicing'

# 002 §4.5 — an unsafe block.
copy_workspace b8_unsafe
plant_lines b8_unsafe 'pub fn planted_unsafe() { let x = 0_i32; let p = &x as *const i32; unsafe { let _ = *p; } }'
expect_fail b8_unsafe 'unsafe_code'

# 002 §4.5 — the core reaching std. Every route this round knows, each
# planted and asserted by the diagnostic it must produce:
copy_workspace b9_clock
plant_lines b9_clock 'pub fn planted_clock() { let _ = std::time::SystemTime::now(); }'
expect_fail b9_clock 'E0433'

copy_workspace b10_fs
plant_lines b10_fs 'pub fn planted_fs() { let _ = std::fs::read; }'
expect_fail b10_fs 'E0433'

copy_workspace b11_env
plant_lines b11_env 'pub fn planted_env() { let _ = std::env::var("planted"); }'
expect_fail b11_env 'E0433'

# The route `#![no_std]` alone does NOT close: `extern crate std;` compiles at
# the crate root (the sysroot crate is reachable even though no_std removed
# std from the extern prelude), std paths resolve against it, and the 002 §7
# legs pass — so the gate's own no-std-check must reject it.
copy_workspace b12_extern_std
plant_lines b12_extern_std \
    'extern crate std;' \
    '' \
    'pub fn planted_root_std() { let _ = std::time::SystemTime::now(); }'
expect_fail b12_extern_std 'no-std-check'

# The same route, confined to cfg(test) — also silent to the 002 §7 legs.
copy_workspace b13_cfg_test_std
plant_lines b13_cfg_test_std \
    '#[cfg(test)]' \
    'extern crate std;' \
    '' \
    '#[cfg(test)]' \
    'mod planted_std_reach {' \
    '    use std::time::SystemTime;' \
    '    #[test]' \
    '    fn planted_probe() {' \
    '        let _ = SystemTime::now();' \
    '    }' \
    '}'
expect_fail b13_cfg_test_std 'no-std-check'

# The std prelude macro route (rustc renders this without an error code).
copy_workspace b14_println
plant_lines b14_println 'pub fn planted_println() { println!("planted"); }'
expect_fail b14_println 'cannot find macro `println` in this scope'

# Removing the attribute is the most direct route of all: the crate becomes
# a std crate and the 002 §7 legs pass; no-std-check must reject it.
copy_workspace b15_no_attr
sed -i '/^#!\[no_std\]$/d' "$SCRATCH/b15_no_attr/crates/tshes/src/lib.rs"
expect_fail b15_no_attr 'no-std-check'

# --- Item 5c: the overflow control ---------------------------------------
copy_workspace c_overflow_off
sed -i 's/^overflow-checks = true$/overflow-checks = false/' "$SCRATCH/c_overflow_off/Cargo.toml"
expect_fail c_overflow_off 'test overflow_tripwire - should panic \.\.\. FAILED'

# --- Item 5d: ruled or stated, but not marked [CI] ------------------------
# F6, ruled (a): no alloc. `extern crate alloc;` compiles fine in a no_std
# crate, so the 002 §7 legs pass and the gate's no-alloc-check rejects it.
copy_workspace d1_alloc
plant_lines d1_alloc \
    'extern crate alloc;' \
    '' \
    'pub fn planted_alloc() { let _ = alloc::vec::Vec::<u8>::new(); }'
expect_fail d1_alloc 'no-alloc-check'

# The alloc macro route, without the extern crate: the compiler rejects it
# (rustc renders this without an error code).
copy_workspace d2_vec_macro
plant_lines d2_vec_macro 'pub fn planted_vec_macro() { let _ = vec![1_u8]; }'
expect_fail d2_vec_macro 'cannot find macro `vec` in this scope'

# The alloc type route, without the extern crate: the compiler rejects it.
copy_workspace d3_box
plant_lines d3_box 'pub fn planted_box() { let _ = Box::new(0_u8); }'
expect_fail d3_box 'E0433'

# 002 §8, stated but not marked [CI]: clippy pedantic at warn, workspace-wide.
# Measured (spec 004): without the member's [lints] workspace = true opt-in,
# this shape compiles clean — the pedantic group was not live in the member.
# With the opt-in, must_use_candidate fires (it is an allow-by-default lint
# that the pedantic group turns on), so the gate must reject this plant.
copy_workspace s8_pedantic
plant_lines s8_pedantic 'pub fn planted_must_use() -> u32 { 0 }'
expect_fail s8_pedantic 'clippy::must_use_candidate'

# 002 §4.9: zero dependencies. A path dependency needs no network, so the
# 002 §7 legs pass and the gate's zero-deps-check rejects it.
copy_workspace d4_path_dep
printf '%s\n' '' '[dependencies]' 'planted-dep = { path = "../planted-dep" }' >> "$SCRATCH/d4_path_dep/crates/tshes/Cargo.toml"
mkdir -p "$SCRATCH/d4_path_dep/crates/planted-dep/src"
printf '%s\n' \
    '[package]' \
    'name = "planted-dep"' \
    'version = "0.1.0"' \
    'edition = "2024"' \
    'rust-version = "1.99.0"' \
    'license = "MIT OR Apache-2.0"' \
    > "$SCRATCH/d4_path_dep/crates/planted-dep/Cargo.toml"
printf '%s\n' '// A planted path dependency (spec 004 Item 5d).' > "$SCRATCH/d4_path_dep/crates/planted-dep/src/lib.rs"
expect_fail d4_path_dep 'zero-deps-check'

# --------------------------------------------------------------------------
echo
echo "SELF-TEST: $PLANTS plants — $((PLANTS - FAILURES)) rejected with their named diagnostic, $FAILURES failures"
if [ "$FAILURES" -ne 0 ]; then
    echo "SELF-TEST: failed plants; see the gate output above"
    exit 1
fi
echo "SELF-TEST: every plant rejected with its named diagnostic; the scratch directory is deleted on exit"
exit 0
