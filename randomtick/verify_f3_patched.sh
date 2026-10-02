#!/bin/bash
# F3 runtime verifier gate (S7-116): run the Rust-patched LevelTicks and the
# COMPOSED (F1+F3) ServerLevel through a REAL HotSpot link-time verification
# (resolveClass) — no server boot (INJECTS-ONLY intact). Complements the
# Rust byte-level tests: only the JVM type-checking verifier can prove the
# emitted bytes are legal for major 65.
#
# Loader topology mirrors src/tickhook.rs exactly: patched LevelTicks +
# composed ServerLevel + helper TickBlockOps child-defined into ONE loader;
# everything else parent-first from the kernel jar (VerifyF3.java). The
# ServerLevel image is the F1+F3 COMPOSITION — the exact bytes the runtime
# hook chain produces (F3 re-applies the idempotent F1 patch; see
# classfile::real_noise::f3_serverlevel_composes_with_f1).
#
# Inputs:
#   patched LevelTicks      = $TMPDIR/ccrussty_patched_LevelTicks.class —
#                             dumped by cargo test f3_patch_roundtrip_verified
#   composed  ServerLevel   = $TMPDIR/ccrussty_patched_ServerLevel_F1F3.class
#                             — dumped by cargo test f3_serverlevel_composes_with_f1
#                             (run `cargo test --release f3` first)
#   kernel jar              = run21 materialized kernel (same fixture source
#                             the Rust tests patch — pairing exact by construction)
#   ops class               = randomtick/build/net/minecraft/server/level/TickBlockOps.class
#                             (ECJ parity build, kernel-first classpath —
#                             same bytes the hook defines)
#
# PASS = "VERIFY-OK" line, exit 0. Any VerifyError => exit 1.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
KERNEL_JAR="${KERNEL_JAR:-/home/z/my-project/scripts/bench3_research/run21/patched-kernel.jar}"
PATCHED_LT="${PATCHED_LT:-$(ls /tmp/ccrussty_patched_LevelTicks_F3full.class 2>/dev/null || ls /tmp/ccrussty_patched_LevelTicks.class 2>/dev/null || true)}"
PATCHED_SL="${PATCHED_SL:-$(ls /tmp/ccrussty_patched_ServerLevel_F1F3.class 2>/dev/null || true)}"
OPS_CLASS="${OPS_CLASS:-$HERE/build/net/minecraft/server/level/TickBlockOps.class}"

if [ -z "$PATCHED_LT" ] || [ ! -f "$PATCHED_LT" ]; then
  echo "patched LevelTicks not found — run: cargo test --release f3_patch_roundtrip" >&2
  exit 2
fi
if [ -z "$PATCHED_SL" ] || [ ! -f "$PATCHED_SL" ]; then
  echo "composed ServerLevel not found — run: cargo test --release f3_serverlevel" >&2
  exit 2
fi
if [ ! -f "$KERNEL_JAR" ]; then
  echo "KERNEL_JAR not found: $KERNEL_JAR" >&2
  exit 2
fi
if [ ! -f "$OPS_CLASS" ]; then
  echo "helper class missing: $OPS_CLASS" >&2
  exit 2
fi

mkdir -p "$HERE/build-verify"
JAVA_BIN="${JAVA_BIN:-$(dirname "$(ls /tmp/jdk21/bin/javap 2>/dev/null || which javap)")}"
JAVA="${JAVA_BIN}/java"
"$JAVA" -jar "$HERE/ecj.jar" -source 21 -target 21 -d "$HERE/build-verify" \
  "$HERE/src-verify/VerifyF3.java" 2>/dev/null

"$JAVA" -cp "$HERE/build-verify" VerifyF3 "$KERNEL_JAR" "$PATCHED_LT" "$PATCHED_SL" "$OPS_CLASS"
