#!/bin/bash
# SENSE runtime verifier gate (TASK-438-A2): run the Rust-patched
# ServerEntityGetter through a REAL HotSpot link-time verification
# (resolveClass) — no server boot (INJECTS-ONLY). Complements the Rust
# byte-level tests: only the JVM type-checking verifier can prove the emitted
# 14-byte body is legal for major 65 (incl. invokestatic
# SenseOps.nearestEntityGate resolution from sense/build).
#
# Inputs:
#   patched class = /tmp/ccrussty_patched_ServerEntityGetter.class — dumped by
#   the cargo test sense_patch_roundtrip_verified (run `cargo test sense_patch`
#   first).
#   kernel jar    = round-396-a patched-kernel.jar (same fixture source the
#   Rust test patches — the pairing is exact by construction).
#
# PASS = "SENSE VERIFY-OK" line, exit 0. Any VerifyError => exit 1.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/.." && pwd)"
cd "$ROOT"
KERNEL_JAR="${KERNEL_JAR:-research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL_JAR" ] || KERNEL_JAR="$(find research -path '*round-396-a/patched-kernel.jar' -size +10M 2>/dev/null | head -1)"
[ -f "$KERNEL_JAR" ] || { echo "kernel jar not found" >&2; exit 1; }
PATCHED="${PATCHED:-$(ls /tmp/ccrussty_patched_ServerEntityGetter.class 2>/dev/null || true)}"

if [ -z "$PATCHED" ] || [ ! -f "$PATCHED" ]; then
  echo "patched class not found — run: cargo test sense_patch_roundtrip" >&2
  exit 2
fi
[ -f "$ROOT/sense/build/net/minecraft/world/entity/SenseOps.class" ] || {
  echo "sense blob missing — run scripts/build_430b_blobs.sh first" >&2
  exit 2
}

JAVAC="${JAVAC_BIN:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
JAVA="${JAVA_BIN:-/home/z/tools/jdk-21.0.12.1+1/bin/java}"

mkdir -p "$HERE/build-verify"
"$JAVAC" --release 21 -nowarn -d "$HERE/build-verify" "$HERE/src-verify/VerifyPatchedSense.java"

"$JAVA" -cp "$HERE/build-verify" VerifyPatchedSense "$KERNEL_JAR" "$PATCHED"
