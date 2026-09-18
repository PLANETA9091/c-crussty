#!/usr/bin/env bash
# Build & run the FLUID-DIRTY offline harness (S7-151, TASK-290).
#
# Prereqs:
#   - entityinside/build/FluidPushOps*.class (scripts/build_fluid_push_ops.sh)
#   - tests/out/{Entity,LevelChunk}.fluiddirty.patched.class
#     (cargo test --lib fluid_dirty::dump_patched_for_verifier)
#
# INJECTS-ONLY: plain JVM, real kernel classes, NO server boot.
set -euo pipefail
cd "$(dirname "$0")/.."   # repo root

JAVA="${JAVA:-/tmp/toolchain/jdk-21.0.12.1+1/bin/java}"
JAVAC="${JAVAC:-/tmp/toolchain/jdk-21.0.12.1+1/bin/javac}"

KERNEL="${KERNEL:-/tmp/s7147mat/server/versions/1.21.10/purpur-1.21.10.jar}"
# Nested libs have versioned patches — the FINAL resolved set is what
# paperclip itself deployed to <work>/libraries during materialization
# (run_fluid_free_harness.sh S7-144 lesson: brigadier/Dfu/etc. needed).
LIBS="${LIBS:-/tmp/s7147mat/server/libraries}"
LIBCP=$(find "$LIBS" -name '*.jar' | tr '\n' ':')
JOML_JAR=$(find "$LIBS/org/joml" -name 'joml-*.jar' | head -1)
FASTUTIL_JAR=$(find "$LIBS/it/unimi/dsi/fastutil" -name 'fastutil-*.jar' | head -1)

HARNESS_BUILD=entityinside/build-harness-fluidpushops
rm -rf "$HARNESS_BUILD"
mkdir -p "$HARNESS_BUILD"

# Compile the harness against kernel + bridge build (one loader space at runtime).
"$JAVAC" -proc:none --release 21 \
  -cp "entityinside/build:$KERNEL:$JOML_JAR:$FASTUTIL_JAR:$LIBCP" \
  -d "$HARNESS_BUILD" \
  entityinside/harness/FluidDirtyHarness.java

# Classpath: bridge build + harness build FIRST (patched/bridge classes shadow),
# then the PATCHED kernel (paper-shaded LogUtils must beat mojang-logging),
# then the nested libraries (S7-144 order discipline).
exec "$JAVA" \
  -cp "$HARNESS_BUILD:entityinside/build:$KERNEL:$LIBCP" \
  net.minecraft.world.entity.FluidDirtyHarness \
  tests/out/Entity.fluiddirty.patched.class \
  tests/out/LevelChunk.fluiddirty.patched.class
