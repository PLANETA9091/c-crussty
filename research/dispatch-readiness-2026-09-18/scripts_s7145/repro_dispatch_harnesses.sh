#!/usr/bin/env bash
# repro_dispatch_harnesses.sh — S7-145: reproduce InsideCache / FlushDiet /
# AllocDiet offline harnesses from a CLEAN post-WIPE state (classpath recipe:
# shadow->KERNEL->libraries, proven in S7-144 RUNBOOK). NO boot, INJECTS-ONLY.
set -euo pipefail
export PATH=/tmp/toolchain/jdk-21.0.12.1+1/bin:$PATH
REPO=/home/z/c-crussty
KERNEL=/tmp/kernelout/patched-kernel.jar
LIBS=/tmp/kernelmat/server/libraries
WORK=/tmp/repro_s7145
rm -rf "$WORK"; mkdir -p "$WORK"

LIBCP=$(find "$LIBS" -name '*.jar' | tr '\n' ':')
# ORDER (S7-144 lesson): bridge build dirs -> PATCHED KERNEL -> libraries
CP="$REPO/entityinside/build:$REPO/entityquery/build:$REPO/paletted/build:$KERNEL:$LIBCP"

echo "== javac harnesses =="
mkdir -p "$REPO/entityinside/harness/build" "$REPO/allocdiet/harness/build"
javac -proc:none -cp "$CP" -d "$REPO/entityinside/harness/build" \
  "$REPO/entityinside/harness/InsideCacheHarness.java" \
  "$REPO/entityinside/harness/FlushDietHarness.java"
javac -proc:none -cp "$CP" -d "$REPO/allocdiet/harness/build" \
  "$REPO/allocdiet/harness/AllocDietHarness.java"
echo "JAVAC OK"

echo "== extract nest partner RecordedEffect from kernel =="
unzip -o -q "$KERNEL" "net/minecraft/world/entity/InsideBlockEffectApplier\$StepBasedCollector\$RecordedEffect.class" -d "$WORK"
RE_EFFECT="$WORK/net/minecraft/world/entity/InsideBlockEffectApplier\$StepBasedCollector\$RecordedEffect.class"
[ -f "$RE_EFFECT" ] || { echo "ERROR: RecordedEffect not found"; exit 1; }

echo "== INSIDE-CACHE harness =="
java -cp "$CP:$REPO/entityinside/harness/build" InsideCacheHarness \
  "$REPO/tests/out/Entity.patched.class" "$REPO/entityinside/build" | tail -4

echo "== FLUSH-DIET harness =="
java -cp "$CP:$REPO/entityinside/harness/build" FlushDietHarness \
  "$REPO/tests/out/StepBasedCollector.patched.class" "$RE_EFFECT" | tail -4

echo "== ALLOC-DIET harness =="
java -cp "$CP:$REPO/allocdiet/harness/build" AllocDietHarness \
  "$KERNEL" "$REPO/tests/out/LivingEntity.patched.class" \
  "$REPO/tests/out/CollisionUtil.patched.class" \
  "$REPO/entityquery/build/net/minecraft/world/entity/EntityQueryOps.class" | tail -4

echo "== ALL THREE HARNESS REPRO DONE =="
