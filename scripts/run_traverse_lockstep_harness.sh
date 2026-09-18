#!/usr/bin/env bash
# Build & run the TraverseLockstepHarness (S7-163) — OFFLINE bit-exact
# lockstep between vanilla BlockGetter.forEachBlockIntersectedBetween and
# the flat TraverseOps.forEachFlat (lever #9 FLAT-TRAVERSAL).
#
# Gate: random + degenerate scenarios (stationary boundary, sign-zero
# axes, block-aligned coords, multi-block marches, clip-empty cells)
# across 4 visitor policies — (posLong, step) sequence + return value
# bit-in-bit. Mismatch on ANY scenario = FAIL (exit 2).
#
# INJECTS-ONLY: plain JVM, real kernel classes, NO server boot.
set -euo pipefail
cd "$(dirname "$0")/.."   # repo root

resolve_java() {
  if [ -n "${JAVA:-}" ]; then echo "$JAVA"; return; fi
  if [ -x /tmp/toolchain/jdk-21.0.12.1+1/bin/java ]; then echo /tmp/toolchain/jdk-21.0.12.1+1/bin/java; return; fi
  if [ -x /tmp/jdk21/bin/java ]; then echo /tmp/jdk21/bin/java; return; fi
  echo java
}
resolve_javac() {
  if [ -n "${JAVAC:-}" ]; then echo "$JAVAC"; return; fi
  if [ -x /tmp/toolchain/jdk-21.0.12.1+1/bin/javac ]; then echo /tmp/toolchain/jdk-21.0.12.1+1/bin/javac; return; fi
  if [ -x /tmp/jdk21/bin/javac ]; then echo /tmp/jdk21/bin/javac; return; fi
  if command -v javac > /dev/null 2>&1; then echo javac; return; fi
  echo "java -m jdk.compiler/com.sun.tools.javac.Main"
}

JAVAC="$(resolve_javac)"
JAVA="$(resolve_java)"

KERNEL="${KERNEL:-/tmp/s7147mat/server/versions/1.21.10/purpur-1.21.10.jar}"
if [ ! -f "$KERNEL" ]; then
  KERNEL="$(ls research/*/run-*/patched-kernel.jar 2>/dev/null | head -1 || true)"
fi
if [ ! -f "$KERNEL" ]; then echo "kernel jar not found" >&2; exit 1; fi

LIBS="${LIBS:-/tmp/s7147mat/server/libraries}"
if [ -d "$LIBS" ]; then
  LIBCP=$(find "$LIBS" -name '*.jar' | tr '\n' ':')
else
  # Offline fallback (sandbox): the f1_batch_rng library set + the clip
  # META-INF libraries shipped with the bench tooling.
  FB=/tmp/my-project/scripts/f1_batch_rng
  LIBCP="$(find "$FB/mojang-libs" -name '*.jar' 2>/dev/null | tr '\n' ':')$(find "$FB/clip/META-INF/libraries" -name '*.jar' 2>/dev/null | tr '\n' ':')$FB/dfs.jar:$FB/guava.jar:$FB/gson.jar:$FB/authlib.jar:$FB/brigadier.jar:$FB/slf4j-api.jar:$FB/joml.jar:$FB/fastutil.jar:$FB/commons-lang3.jar:$FB/failureaccess.jar:"
fi

HARNESS_BUILD=entityinside/build-harness-traverse
mkdir -p "$HARNESS_BUILD"

$JAVAC --release 21 -proc:none \
  -cp "$KERNEL:$LIBCP:entityinside/build" \
  -d "$HARNESS_BUILD" \
  entityinside/harness/TraverseLockstepHarness.java

exec $JAVA -cp "$HARNESS_BUILD:$KERNEL:$LIBCP:entityinside/build" \
  harness.TraverseLockstepHarness "$@"
