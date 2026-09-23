#!/usr/bin/env bash
# run_fluid_free_harness.sh — S7-144 dispatch-readiness audit: full
# reproduction of the S7-143 FluidFreeHarness verification from a CLEAN
# environment (post-WIPE). Plain JVM, real materialized kernel, NO boot.
set -euo pipefail
export PATH=/tmp/toolchain/jdk-21.0.12.1+1/bin:$PATH
REPO=/home/z/c-crussty
KERNEL=/tmp/kernelout/patched-kernel.jar
CPDIR=/tmp/ff_audit_s7144/cp
HARNESS_SRC=$REPO/entityinside/harness/FluidFreeHarness.java
HARNESS_BUILD=$REPO/entityinside/harness/build

CHUNK=net/minecraft/world/level/chunk
ENTITY=net/minecraft/world/entity

rm -rf "$CPDIR"; mkdir -p "$CPDIR/$CHUNK" "$CPDIR/$ENTITY"
# Nested libs have versioned patches — the FINAL resolved set is what
# paperclip itself deployed to <work>/libraries during materialization
LIBS=/tmp/kernelmat/server/libraries
[ -d "$LIBS" ] || { echo "ERROR: $LIBS missing (run materialize_kernel_v2.sh first)"; exit 1; }
LIBCP=$(find "$LIBS" -name '*.jar' | tr '\n' ':')
echo "LIBS: $(find "$LIBS" -name '*.jar' | wc -l) jars"
# Shadow patched classes AHEAD of the kernel (single loader space = runtime retransform model)
cp "$REPO/tests/out/LevelChunkSection.patched.class"   "$CPDIR/$CHUNK/LevelChunkSection.class"
cp "$REPO/tests/out/PalettedContainer.patched.class"    "$CPDIR/$CHUNK/PalettedContainer.class"
cp "$REPO/paletted/build/$CHUNK/PalettedContainerOps.class" "$CPDIR/$CHUNK/"
cp "$REPO/entityinside/build/$ENTITY/FluidOps.class"    "$CPDIR/$ENTITY/"
# (plus any nested classes shipped with FluidOps, if present)
cp "$REPO"/entityinside/build/$ENTITY/FluidOps*.class   "$CPDIR/$ENTITY/" 2>/dev/null || true

# ORDER MATTERS: shadow-classes first, then the PATCHED kernel (it carries
# paper-shaded classes like LogUtils.getClassLogger that must beat the clean
# mojang-logging in libraries/), then the nested libraries.
CP="$CPDIR:$REPO/entityinside/build:$REPO/paletted/build:$KERNEL:$LIBCP"

# Clean recompile of the harness against the shadow classpath (verifier = structural gate)
rm -rf "$HARNESS_BUILD"; mkdir -p "$HARNESS_BUILD"
javac -proc:none -cp "$CP" -d "$HARNESS_BUILD" "$HARNESS_SRC"
echo "JAVAC OK (harness recompiled against patched shadow classpath)"

# Run: exit 0 = FLUID-FREE OFFLINE PASS
java -cp "$CP:$HARNESS_BUILD" net.minecraft.world.entity.FluidFreeHarness
