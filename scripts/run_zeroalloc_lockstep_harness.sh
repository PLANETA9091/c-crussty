#!/usr/bin/env bash
# Build & run the ZeroAllocLockstepHarness (S7-164) — OFFLINE bit-exact
# lockstep between the vanilla kernel classes (AABB / EntityDimensions)
# and the scalar ZeroAllocOps tails (lever #10 ZERO-ALLOC-INSIDE), plus a
# HotSpot verification of the fully redirected Entity.class produced by
# the classfile body-redirect.
#
# Gate: 50k makeBox scenarios + 300k collidedAlongVector scenarios
# (random + degenerate: zero deltas, sub-eps deltas, empty lists,
# straddling/thin/far boxes). Any mismatch = FAIL (exit 2).
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

export PATH="$HOME/.cargo/bin:$PATH"
CARGO="$(command -v cargo || echo /home/z/.cargo/bin/cargo)"

KERNEL="${KERNEL:-/tmp/s7147mat/server/versions/1.21.10/purpur-1.21.10.jar}"
if [ ! -f "$KERNEL" ]; then
  KERNEL="$(ls research/*/run-*/patched-kernel.jar 2>/dev/null | head -1 || true)"
fi
if [ ! -f "$KERNEL" ]; then echo "kernel jar not found" >&2; exit 1; fi

LIBS="${LIBS:-/tmp/s7147mat/server/libraries}"
if [ -d "$LIBS" ]; then
  LIBCP=$(find "$LIBS" -name '*.jar' | tr '\n' ':')
else
  FB=/tmp/my-project/scripts/f1_batch_rng
  LIBCP="$(find "$FB/mojang-libs" -name '*.jar' 2>/dev/null | tr '\n' ':')$(find "$FB/clip/META-INF/libraries" -name '*.jar' 2>/dev/null | tr '\n' ':')$FB/dfs.jar:$FB/guava.jar:$FB/gson.jar:$FB/authlib.jar:$FB/brigadier.jar:$FB/slf4j-api.jar:$FB/joml.jar:$FB/fastutil.jar:$FB/commons-lang3.jar:$FB/failureaccess.jar:"
fi

# Step 0: build the ZeroAllocOps bridge (guards: exactly ONE classfile).
bash scripts/build_zeroalloc_ops.sh > /dev/null

# Step 1: rust body-redirect -> Entity_patched.class (HotSpot will verify).
echo "== emitting redirected Entity.class via cargo test =="
CRUSSTY_EMIT_PATCHED_ENTITY=entityinside/build/Entity_patched.class \
  $CARGO test --release zeroalloc_emit_patched_entity_for_harness 2>&1 | tail -2
test -f entityinside/build/Entity_patched.class || { echo "no patched entity emitted" >&2; exit 1; }

HARNESS_BUILD=entityinside/build-harness-zeroalloc
mkdir -p "$HARNESS_BUILD"

JAVAC="$(resolve_javac)"
JAVA="$(resolve_java)"

# Step 2: compile the harness (package net.minecraft.world.level for
# package-private access to the scalar tails).
if ! $JAVAC --release 21 -proc:none \
  -cp "$KERNEL:$LIBCP:entityinside/build" \
  -d "$HARNESS_BUILD" \
  entityinside/harness/ZeroAllocLockstepHarness.java; then
  echo "note: --release 21 rejected, compiling with the JVM default target" >&2
  $JAVAC -proc:none \
    -cp "$KERNEL:$LIBCP:entityinside/build" \
    -d "$HARNESS_BUILD" \
    entityinside/harness/ZeroAllocLockstepHarness.java
fi

# Step 3: run the lockstep oracle.
exec $JAVA -Xss32m -cp "$KERNEL:$LIBCP:entityinside/build:$HARNESS_BUILD" \
  net.minecraft.world.level.ZeroAllocLockstepHarness
