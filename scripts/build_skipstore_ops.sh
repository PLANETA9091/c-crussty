#!/usr/bin/env bash
# Build the SkipStoreOps bridge class (S7-166 / #13-SBB SKIP-STORE-BB).
#
# PURE JAVA bridge (no JNI natives), same delivery pattern as ZeroAllocOps
# (S7-164): compiled offline against the runtime kernel jar (Mojang-mapped)
# and defined into the KERNEL loader at activation time (same package
# net.minecraft.world.level).
#
# Class-file major pinned to 65 = the kernel JVM (Java 21).
# ZERO nested classes allowed (S7-163 leg#1 lesson) — the cargo guard
# src/skip_store.rs enforces the same set.
#
# Usage: scripts/build_skipstore_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
resolve_javac_module() {
  if [ -n "$JAVAC" ]; then echo "$JAVAC"; return; fi
  if [ -x /tmp/toolchain/jdk-21.0.12.1+1/bin/javac ]; then echo /tmp/toolchain/jdk-21.0.12.1+1/bin/javac; return; fi
  if [ -x /tmp/jdk21/bin/javac ]; then echo /tmp/jdk21/bin/javac; return; fi
  if command -v javac > /dev/null 2>&1; then echo javac; fi
  echo "java -m jdk.compiler/com.sun.tools.javac.Main"
}

JAVAC_CMD="$(resolve_javac_module)"

SERVER_JAR="${SERVER_JAR:-/tmp/s7147mat/server/versions/1.21.10/purpur-1.21.10.jar}"
if [ ! -f "$SERVER_JAR" ]; then
  SERVER_JAR="/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar"
fi
if [ ! -f "$SERVER_JAR" ]; then
  SERVER_JAR="$(ls research/*/run-*/patched-kernel.jar 2>/dev/null | tail -1 || true)"
fi
if [ ! -f "$SERVER_JAR" ]; then echo "runtime/kernel jar not found" >&2; exit 1; fi

OUT_DIR=entityinside/build
mkdir -p "$OUT_DIR/net/minecraft/world/level"

# Remove any stale output first (delivery-set discipline).
rm -f "$OUT_DIR/net/minecraft/world/level/SkipStoreOps"*.class

if ! $JAVAC_CMD --release 21 \
  -cp "$SERVER_JAR" \
  -d "$OUT_DIR" \
  entityinside/net/minecraft/world/level/SkipStoreOps.java 2>/dev/null; then
  echo "pinned --release 21 failed, retrying with module javac defaults" >&2
  $JAVAC_CMD \
    -cp "$SERVER_JAR" \
    -d "$OUT_DIR" \
    entityinside/net/minecraft/world/level/SkipStoreOps.java
fi

# Delivery-set guard: EXACTLY one classfile must be produced (no nested).
PRODUCED=$(ls "$OUT_DIR/net/minecraft/world/level/SkipStoreOps"*.class 2>/dev/null | wc -l)
if [ "$PRODUCED" != "1" ]; then
  echo "SKIP-STORE delivery guard FAILED: produced $PRODUCED classfiles (expected 1)" >&2
  ls -la "$OUT_DIR/net/minecraft/world/level/" >&2
  exit 2
fi
echo "skipstore_ops build OK: $(ls "$OUT_DIR/net/minecraft/world/level/SkipStoreOps"*.class)"
