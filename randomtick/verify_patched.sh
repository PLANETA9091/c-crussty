#!/bin/bash
# F1 runtime verifier gate (S7-112): run the Rust-patched ServerLevel through
# a REAL HotSpot link-time verification (resolveClass) — no server boot
# (INJECTS-ONLY intact). Complements the Rust byte-level tests: only the JVM
# type-checking verifier can prove the emitted bytes are legal for major 65.
#
# Inputs:
#   patched class = $TMPDIR/ccrussty_patched_ServerLevel.class — dumped by the
#   cargo test f1_patch_roundtrip_verified (run `cargo test --release f1` first).
#   kernel jar    = run21 materialized kernel (same fixture source the Rust
#   test patches — the pairing is exact by construction).
#
# PASS = "VERIFY-OK" line, exit 0. Any VerifyError => exit 1.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
KERNEL_JAR="${KERNEL_JAR:-/home/z/my-project/scripts/bench3_research/run21/patched-kernel.jar}"
PATCHED="${PATCHED:-$(ls /tmp/ccrussty_patched_ServerLevel.class 2>/dev/null || true)}"

if [ -z "$PATCHED" ] || [ ! -f "$PATCHED" ]; then
  echo "patched class not found — run: cargo test --release f1_patch_roundtrip" >&2
  exit 2
fi
if [ ! -f "$KERNEL_JAR" ]; then
  echo "KERNEL_JAR not found: $KERNEL_JAR" >&2
  exit 2
fi

mkdir -p "$HERE/build-verify"
java -jar "$HERE/ecj.jar" -source 21 -target 21 -d "$HERE/build-verify" \
  "$HERE/src-verify/VerifyPatched.java" 2>/dev/null

java -cp "$HERE/build-verify" VerifyPatched "$KERNEL_JAR" "$PATCHED"
