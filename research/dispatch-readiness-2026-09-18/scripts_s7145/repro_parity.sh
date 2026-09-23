#!/usr/bin/env bash
# repro_parity.sh — S7-145: reproduce PALETTED-DEMUX ParityHarness from a
# CLEAN post-WIPE state. Stub jar = patched PalettedContainer (child-first
# L2), ops dir = paletted/build, libRoot = paperclip-deployed libraries.
# NO boot, INJECTS-ONLY.
set -euo pipefail
export PATH=/tmp/toolchain/jdk-21.0.12.1+1/bin:$PATH
REPO=/home/z/c-crussty
KERNEL=/tmp/kernelout/patched-kernel.jar
LIBS=/tmp/kernelmat/server/libraries
WORK=/tmp/repro_parity_s7145
rm -rf "$WORK"; mkdir -p "$WORK/net/minecraft/world/level/chunk"

# stub jar: FULL PalettedContainer* family at canonical paths — patched
# container REPLACES the vanilla copy, vanilla RO/nested stay so L2 keeps
# its interface/method-signature closure consistent (LinkageError lesson:
# a lone patched class itable-links against the parent loader's RO).
unzip -o -q "$KERNEL" "net/minecraft/world/level/chunk/PalettedContainer*.class" -d "$WORK"
# same-runtime-package closure (protected/abstract boundary is loader-scoped):
# Strategy (+nested proxies) and Configuration must share L2's loader with the
# patched container, else IllegalAccessError on Strategy.getConfigurationForBitCount
unzip -o -q "$KERNEL" "net/minecraft/world/level/chunk/Strategy*.class" -d "$WORK"
unzip -o -q "$KERNEL" "net/minecraft/world/level/chunk/Configuration*.class" -d "$WORK"
cp "$REPO/tests/out/PalettedContainer.patched.class" \
   "$WORK/net/minecraft/world/level/chunk/PalettedContainer.class"
cd "$WORK"
jar cf stub.jar net/
echo "stub jar built: $(du -h stub.jar | cut -f1); classes: $(find net -name '*.class' | wc -l)"

# harness compile (default package, self-contained ChildFirstClassLoader)
mkdir -p "$REPO/paletted/harness/build"
javac -proc:none -cp "$KERNEL" -d "$REPO/paletted/harness/build" \
  "$REPO/paletted/harness/ParityHarness.java"
echo "JAVAC OK"

java -cp "$REPO/paletted/harness/build" ParityHarness \
  "$KERNEL" "$WORK/stub.jar" "$REPO/paletted/build" 424242 "$LIBS" | tail -8
