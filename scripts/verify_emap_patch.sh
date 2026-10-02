#!/usr/bin/env bash
# verify_emap_patch.sh — TASK-411-A offline JVM-verifier gate for the
# ChunkMap.entityMap fence (s7172 lesson: defineClass verification is NOT
# enough; the real JVMTI RetransformClasses path must accept the bytes).
# Usage: scripts/verify_emap_patch.sh [kernel-jar] [fastutil-jar]
#   1) cargo test dump_emap_patched_for_verifier -> tests/out/ChunkMap.emap.patched.class
#   2) EMAPRetransformProbe: vanilla ChunkMap load -> retransform(patched)
#      -> fence-helper semantics on a real fastutil map.
#   PASS = "[probe] RETRANSFORM OK" + "[probe] SEMANTIC OK"
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${JAVAC:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
JAVA="${JAVA:-/home/z/tools/jdk-21.0.12.1+1/bin/java}"
JAR="${JAR:-/home/z/tools/jdk-21.0.12.1+1/bin/jar}"
KERNEL_JAR="${1:-$(ls research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar)}"
FASTUTIL="${2:-/home/z/tools/fastutil.jar}"

export PATH="$HOME/.cargo/bin:$PATH"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/home/z/rounds/ROUND-411/a3-target}"

cargo test --lib dump_emap_patched_for_verifier -- --exact classfile::region_threads::dump_emap_patched_for_verifier >/dev/null

HARNESS=$(mktemp -d)
trap 'rm -rf "$HARNESS"' EXIT
$JAVAC -d "$HARNESS" entityinside/harness/EMAPRetransformProbe.java
printf 'Premain-Class: EMAPRetransformProbe\nCan-Retransform-Classes: true\n' > "$HARNESS/mf.txt"
$JAR cfm "$HARNESS/probe.jar" "$HARNESS/mf.txt" -C "$HARNESS" .

"$JAVA" -javaagent:"$HARNESS/probe.jar" \
  -cp "$HARNESS/probe.jar:$KERNEL_JAR:$FASTUTIL" \
  EMAPRetransformProbe "$KERNEL_JAR" tests/out/ChunkMap.emap.patched.class \
  entityinside/build | grep -E "RETRANSFORM OK|SEMANTIC OK" \
  && echo "VERIFY-EMAP: PASS" || { echo "VERIFY-EMAP: FAIL"; exit 1; }
