#!/usr/bin/env bash
# Build the INSIDE-DIET bridge classes (lever #12 v1, TASK-332):
#   net/minecraft/world/entity/InsideDietOps     — glue-free bit-exact walk replica
#   net/minecraft/world/entity/InsideDietVisitor — visit transcription (1 object/call)
# Same delivery pattern as ZeroCursorOps (TASK-330): compiled offline against
# the kernel jar (Mojang-mapped), defined into the KERNEL loader at activation.
# Class-file major pinned to 65 (Java 21). ZERO nested classfiles.
#
# Usage: scripts/build_inside_diet_ops.sh [javac]
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
mkdir -p "$OUT_DIR/net/minecraft/world/entity" "$OUT_DIR/harness"
rm -f "$OUT_DIR/net/minecraft/world/entity/InsideDiet"*.class

# --- bridge classes (no nested classfiles allowed)
LIBS="$(ls /tmp/my-project/scripts/f1_batch_rng/mojang-libs/*.jar 2>/dev/null | tr '\n' ':' | sed 's/:$//')"
[ -z "$LIBS" ] && LIBS="$SERVER_JAR"
if ! $JAVAC_CMD --release 21 -cp "$SERVER_JAR:$LIBS" -d "$OUT_DIR" \
  entityinside/net/minecraft/world/entity/InsideDietOps.java \
  entityinside/net/minecraft/world/entity/InsideDietVisitor.java 2>/dev/null; then
  echo "pinned --release 21 failed, retrying with module javac defaults" >&2
  $JAVAC_CMD -cp "$SERVER_JAR:$LIBS" -d "$OUT_DIR" \
    entityinside/net/minecraft/world/entity/InsideDietOps.java \
    entityinside/net/minecraft/world/entity/InsideDietVisitor.java
fi

PRODUCED=$(ls "$OUT_DIR/net/minecraft/world/entity/InsideDiet"*.class 2>/dev/null | wc -l)
if [ "$PRODUCED" != "2" ]; then
  echo "INSIDE-DIET delivery guard FAILED: produced $PRODUCED classfiles (expected 2: Ops+Visitor, no nested)" >&2
  ls -la "$OUT_DIR/net/minecraft/world/entity/" >&2
  exit 2
fi
echo "inside-diet bridge build OK: $PRODUCED classfiles"

echo "inside-diet delivery complete"
