#!/usr/bin/env bash
# Build the BlockUpdateOps bridge class (S7-168 / STEAL v2 defect-fix).
#
# PURE JAVA bridge (no JNI natives), delivery pattern of SkipStoreOps
# (S7-166): compiled offline against the runtime kernel jar (Mojang-mapped)
# and defined into the KERNEL loader at region_threads activation time
# (same package net.minecraft.server.level as ServerLevel).
#
# Class-file major pinned to 65 = the kernel JVM (Java 21).
# ZERO nested classes allowed (S7-163 leg#1 lesson) — the cargo guard
# src/region_threads.rs enforces the same set.
#
# Usage: scripts/build_blockupd_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if [ -x /tmp/toolchain/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/tmp/toolchain/jdk-21.0.12.1+1/bin/javac
  elif [ -x /tmp/jdk21/bin/javac ]; then JAVAC=/tmp/jdk21/bin/javac
  elif command -v javac > /dev/null 2>&1; then JAVAC=javac
  else echo "no javac found (pass one as arg 1)" >&2; exit 1; fi
fi

SERVER_JAR="${SERVER_JAR:-/tmp/s7147mat/server/versions/1.21.10/purpur-1.21.10.jar}"
if [ ! -f "$SERVER_JAR" ]; then
  SERVER_JAR="/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar"
fi
if [ ! -f "$SERVER_JAR" ]; then
  SERVER_JAR="$(ls research/*/run-*/patched-kernel.jar 2>/dev/null | tail -1 || true)"
fi
if [ ! -f "$SERVER_JAR" ]; then echo "runtime/kernel jar not found" >&2; exit 1; fi

FASTUTIL_JAR="${FASTUTIL_JAR:-$(find /tmp/alllibs2 /tmp/pdec /tmp/s7147mat -name 'fastutil-*.jar' 2>/dev/null | head -1 || true)}"

OUT_DIR=entityinside/build
mkdir -p "$OUT_DIR/net/minecraft/server/level"

# Remove any stale output first (delivery-set discipline).
rm -f "$OUT_DIR/net/minecraft/server/level/BlockUpdateOps"*.class

# RegionTickOps must be on the compile classpath (BlockUpdateOps calls
# isWorker/deferBlockUpdate) — use the current build output.
RT_DIR="$OUT_DIR"

if ! $JAVAC --release 21 \
  -cp "$SERVER_JAR:${FASTUTIL_JAR:+$FASTUTIL_JAR}:$RT_DIR" \
  -d "$OUT_DIR" \
  entityinside/net/minecraft/server/level/BlockUpdateOps.java 2>/dev/null; then
  $JAVAC \
    -cp "$SERVER_JAR:${FASTUTIL_JAR:+$FASTUTIL_JAR}:$RT_DIR" \
    -d "$OUT_DIR" \
    entityinside/net/minecraft/server/level/BlockUpdateOps.java
fi

# Delivery-set guard: EXACTLY one classfile must be produced (no nested).
PRODUCED=$(ls "$OUT_DIR/net/minecraft/server/level/BlockUpdateOps"*.class 2>/dev/null | wc -l)
if [ "$PRODUCED" != "1" ]; then
  echo "BU-DEFER delivery guard FAILED: produced $PRODUCED classfiles (expected 1)" >&2
  ls -la "$OUT_DIR/net/minecraft/server/level/" >&2
  exit 2
fi
echo "blockupd_ops build OK: $(ls "$OUT_DIR/net/minecraft/server/level/BlockUpdateOps"*.class)"
