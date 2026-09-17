#!/bin/bash
# F2 runtime verifier gate (S7-114): run the Rust-patched Brain through a REAL
# HotSpot link-time verification (resolveClass) — no server boot (INJECTS-ONLY
# intact). Complements the Rust byte-level tests: only the JVM type-checking
# verifier can prove the emitted bytes are legal for major 65.
#
# Loader topology mirrors src/brainhook.rs exactly: patched Brain + lens
# helper trio (BrainOps$IdKey, BrainOps$Snapshot, BrainOps — nested first)
# child-defined into ONE loader; everything else parent-first from the kernel
# jar (VerifyBrain.java).
#
# Inputs:
#   patched class = $TMPDIR/ccrussty_patched_Brain.class — dumped by the cargo
#   test f2_patch_roundtrip_verified (run `cargo test --release f2` first).
#   kernel jar    = run21 materialized kernel (same fixture source the Rust
#   test patches — the pairing is exact by construction).
#   ops classes   = randomtick/build/net/minecraft/world/entity/ai/ (ECJ
#   parity build, kernel-first classpath — same bytes the hook defines).
#
# PASS = "VERIFY-OK" line, exit 0. Any VerifyError => exit 1.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
KERNEL_JAR="${KERNEL_JAR:-/home/z/my-project/scripts/bench3_research/run21/patched-kernel.jar}"
PATCHED="${PATCHED:-$(ls /tmp/ccrussty_patched_Brain.class 2>/dev/null || true)}"
OPS_DIR="${OPS_DIR:-$HERE/build/net/minecraft/world/entity/ai}"

if [ -z "$PATCHED" ] || [ ! -f "$PATCHED" ]; then
  echo "patched class not found — run: cargo test --release f2_patch_roundtrip" >&2
  exit 2
fi
if [ ! -f "$KERNEL_JAR" ]; then
  echo "KERNEL_JAR not found: $KERNEL_JAR" >&2
  exit 2
fi
for f in 'BrainOps.class' 'BrainOps$IdKey.class' 'BrainOps$Snapshot.class'; do
  if [ ! -f "$OPS_DIR/$f" ]; then
    echo "helper class missing: $OPS_DIR/$f" >&2
    exit 2
  fi
done

mkdir -p "$HERE/build-verify"
java -jar "$HERE/ecj.jar" -source 21 -target 21 -d "$HERE/build-verify" \
  "$HERE/src-verify/VerifyBrain.java" 2>/dev/null

java -cp "$HERE/build-verify" VerifyBrain "$KERNEL_JAR" "$PATCHED" "$OPS_DIR"
