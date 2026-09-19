#!/usr/bin/env bash
# Build the ZERO-CURSOR bridge classes (lever #11 v1, TASK-330):
#   net/minecraft/core/ZeroCursorIter  — pooled bit-exact replica of BlockPos$6
#   net/minecraft/core/ZeroCursorOps   — redirect target (thread-local ring)
# plus entityinside/harness/CursorLockstepHarness.java for the offline
# bit-exact lockstep vs the REAL vanilla lambda$betweenCornersInDirection$8.
#
# Same delivery pattern as ZeroAllocOps (S7-164): compiled offline against the
# kernel jar (Mojang-mapped), defined into the KERNEL loader at activation.
# Class-file major pinned to 65 = kernel JVM (Java 21). ZERO nested classes.
#
# Usage: scripts/build_zerocursor_ops.sh [javac]
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
  SERVER_JAR="$(ls research/*/run-*/patched-kernel.jar 2>/dev/null | head -1 || true)"
fi
if [ ! -f "$SERVER_JAR" ]; then echo "runtime/kernel jar not found" >&2; exit 1; fi

OUT_DIR=entityinside/build
mkdir -p "$OUT_DIR/net/minecraft/core" "$OUT_DIR/harness"
rm -f "$OUT_DIR/net/minecraft/core/ZeroCursor"*.class "$OUT_DIR/harness/CursorLockstepHarness"*.class

# --- bridge classes (no nested classfiles allowed)
if ! $JAVAC_CMD --release 21 -cp "$SERVER_JAR" -d "$OUT_DIR" \
  entityinside/net/minecraft/core/ZeroCursorIter.java \
  entityinside/net/minecraft/core/ZeroCursorOps.java 2>/dev/null; then
  echo "pinned --release 21 failed, retrying with module javac defaults" >&2
  $JAVAC_CMD -cp "$SERVER_JAR" -d "$OUT_DIR" \
    entityinside/net/minecraft/core/ZeroCursorIter.java \
    entityinside/net/minecraft/core/ZeroCursorOps.java
fi

PRODUCED=$(ls "$OUT_DIR/net/minecraft/core/ZeroCursor"*.class 2>/dev/null | wc -l)
if [ "$PRODUCED" != "2" ]; then
  echo "ZERO-CURSOR delivery guard FAILED: produced $PRODUCED classfiles (expected 2: Iter+Ops, no nested)" >&2
  ls -la "$OUT_DIR/net/minecraft/core/" >&2
  exit 2
fi
echo "zerocursor bridge build OK: $PRODUCED classfiles"

# --- harness (compiled separately; NOT part of the delivery set)
$JAVAC_CMD --release 21 -cp "$SERVER_JAR:$OUT_DIR" -d "$OUT_DIR" \
  entityinside/harness/CursorLockstepHarness.java 2>/dev/null \
|| $JAVAC_CMD -cp "$SERVER_JAR:$OUT_DIR" -d "$OUT_DIR" \
  entityinside/harness/CursorLockstepHarness.java
echo "zerocursor harness build OK"
