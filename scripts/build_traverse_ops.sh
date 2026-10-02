#!/usr/bin/env bash
# Build the TraverseOps bridge class (S7-163 / FLAT-TRAVERSAL lever #9).
#
# PURE JAVA bridge (no JNI natives), same delivery pattern as BatchCollector
# (S7-161): compiled offline against the runtime kernel jar (Mojang-mapped)
# and defined into the KERNEL loader at activation time (same package
# net.minecraft.world.level as BlockGetter).
#
# Class-file major pinned to 65 = the kernel JVM (Java 21).
#
# Usage: scripts/build_traverse_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
resolve_javac_module() {
  # Sandbox has no javac binary but the jdk.compiler module is alive.
  if [ -n "$JAVAC" ]; then echo "$JAVAC"; return; fi
  if [ -x /tmp/toolchain/jdk-21.0.12.1+1/bin/javac ]; then echo /tmp/toolchain/jdk-21.0.12.1+1/bin/javac; return; fi
  if [ -x /tmp/jdk21/bin/javac ]; then echo /tmp/jdk21/bin/javac; return; fi
  if command -v javac > /dev/null 2>&1; then echo javac; return; fi
  echo "java -m jdk.compiler/com.sun.tools.javac.Main"
}

JAVAC_CMD="$(resolve_javac_module)"

SERVER_JAR="${SERVER_JAR:-/tmp/s7147mat/server/versions/1.21.10/purpur-1.21.10.jar}"
if [ ! -f "$SERVER_JAR" ]; then
  SERVER_JAR="/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar"
fi
if [ ! -f "$SERVER_JAR" ]; then
  # Offline fallback: the leg-artifact kernel jar (Mojang-mapped, contains
  # every net.minecraft class the bridge compiles against).
  SERVER_JAR="$(ls research/*/run-*/patched-kernel.jar 2>/dev/null | head -1 || true)"
fi
if [ ! -f "$SERVER_JAR" ]; then echo "runtime/kernel jar not found" >&2; exit 1; fi

JOML_JAR="${JOML_JAR:-$(find /tmp/pdec/matsrv/libraries/org/joml /tmp/s7147mat/server/libraries/org/joml /tmp/s7151mat/server/libraries/org/joml /tmp/my-project/scripts/f1_batch_rng -name 'joml*.jar' 2>/dev/null | head -1 || true)}"
if [ -z "$JOML_JAR" ]; then echo "joml jar not found" >&2; exit 1; fi
FASTUTIL_JAR="${FASTUTIL_JAR:-$(find /tmp/pdec/matsrv/libraries/it/unimi/dsi/fastutil /tmp/s7147mat/server/libraries/it/unimi/dsi/fastutil /tmp/s7151mat/server/libraries/it/unimi/dsi/fastutil /tmp/my-project/scripts/f1_batch_rng -maxdepth 1 -name 'fastutil*.jar' 2>/dev/null | head -1 || true)}"

OUT_DIR=entityinside/build
mkdir -p "$OUT_DIR"

# The module-based javac fallback may reject --release on some sandbox JVMs;
# the running JVM IS Java 21 so the default target major (65) is already
# correct — try pinned first, fall back to the default.
if ! $JAVAC_CMD --release 21 \
  -cp "$SERVER_JAR:$JOML_JAR${FASTUTIL_JAR:+:$FASTUTIL_JAR}" \
  -d "$OUT_DIR" \
  entityinside/net/minecraft/world/level/TraverseOps.java; then
  echo "note: --release 21 rejected, compiling with the JVM default target" >&2
  $JAVAC_CMD \
    -cp "$SERVER_JAR:$JOML_JAR${FASTUTIL_JAR:+:$FASTUTIL_JAR}" \
    -d "$OUT_DIR" \
    entityinside/net/minecraft/world/level/TraverseOps.java
fi

echo "built: $OUT_DIR/net/minecraft/world/level/TraverseOps.class"
sha256sum "$OUT_DIR/net/minecraft/world/level/TraverseOps.class"

# S7-163 leg#1 TECH-DUD guard: EVERY produced classfile must be embedded in
# src/traversal.rs (TRAVERSE_NESTED) — a plain classpath resolves nested
# classes implicitly, but the kernel loader does NOT; a missing entry
# detonates as NoClassDefFoundError on the first entity tick.
EXPECTED_N=2  # TraverseOps.class + TraverseOps$LongTable.class
BUILT_N=$(ls "$OUT_DIR"/net/minecraft/world/level/TraverseOps*.class 2>/dev/null | wc -l)
if [ "$BUILT_N" -ne "$EXPECTED_N" ]; then
  echo "FAIL: TraverseOps produced $BUILT_N classfiles (expected $EXPECTED_N) — " \
       "update TRAVERSE_NESTED in src/traversal.rs before any dispatch" >&2
  ls "$OUT_DIR"/net/minecraft/world/level/TraverseOps*.class >&2
  exit 1
fi
echo "nested-delivery guard: $BUILT_N/$EXPECTED_N classfiles OK"
