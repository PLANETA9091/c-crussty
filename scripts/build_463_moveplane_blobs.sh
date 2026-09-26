#!/usr/bin/env bash
# build_463_moveplane_blobs.sh — TASK-463-69a: compile MovePlaneOps.java
# (navmath P44 bridge, lever cmp463_move) and install the blob NESTED first
# (include_bytes! contract) then the FLAT legacy copy (check_blobs_sync
# flat==nested byte identity). Recipe = build_460_swarx_blobs.sh: ONE javac
# pass, full cp = kernel round-396-a + fastutil + paper-api 1.21.10 +
# adventure-api/key 4.24.0 + joml, --release 21 (major 65).
#
# Usage: scripts/build_463_moveplane_blobs.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
KERNEL="${KERNEL_JAR:-research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL="$(find research -path '*round-396-a/patched-kernel.jar' -size +10M 2>/dev/null | head -1 || true)"
[ -n "$KERNEL" ] && [ -f "$KERNEL" ] || KERNEL=/home/z/tools/patched-kernel.jar
[ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

CP="$KERNEL"
for j in /home/z/tools/fastutil.jar /home/z/tools/paper-api-1.21.10.jar \
         /home/z/tools/adventure-api-4.24.0.jar /home/z/tools/adventure-key-4.24.0.jar; do
  [ -f "$j" ] && CP="$CP:$j"
done
JOML="${JOML_JAR:-/home/z/tools/joml-1.10.7.jar}"
[ -f "$JOML" ] && CP="$CP:$JOML"

BUILD=$(mktemp -d)
trap 'rm -rf "$BUILD"' EXIT

"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$BUILD" \
  moveplane/net/minecraft/world/entity/ai/control/MovePlaneOps.java \
  moveplane/selftest/MovePlaneSelfTestMain.java

CLS="net/minecraft/world/entity/ai/control/MovePlaneOps"
BASE=$(basename "$CLS")
NESTED="moveplane/build/$CLS.class"
mkdir -p "$(dirname "$NESTED")"
cp "$BUILD/$CLS.class" "$NESTED"
cp "$BUILD/$CLS.class" "moveplane/build/$BASE.class"
echo "blob: $NESTED ($(stat -c%s "$NESTED") bytes) + flat moveplane/build/$BASE.class"

# javap C0-gate: the 12 constant groups must be in the blob's constant pool
JAVAP_DIR=$(dirname "$JAVAC")
"$JAVAP_DIR/javap" -p -c -cp "$BUILD" net.minecraft.world.entity.ai.control.MovePlaneOps > /tmp/moveplane_ops.txt
for needle in "6910469410427058090" "4805340802404319232" "10430.378" "16384.0" "65535" "2.500000277905201" "3.1415927410125732" "0.16666666666666666" "1.5707963267948966" "0.017453292"; do
  grep -q "$needle" /tmp/moveplane_ops.txt || { echo "C0-GATE FAIL: constant $needle missing from javap" >&2; exit 1; }
done
echo "build_463_moveplane_blobs: C0 javap-gate 10/10 constant needles OK — run scripts/check_blobs_sync.sh next"

# Selftest against the REAL Mth (in-JVM oracle): rc != 0 = NOT-A-BENCH.
# Runtime cp adds the kernel library tree (guava-33.3.1/slf4j-2.0.17/
# mojang-logging-1.5.10 ... LEDGER-37 oracle classpath discipline).
RUN_CP="$CP:$BUILD"
if [ -d /tmp/libraries ]; then
  LIBS=$(find /tmp/libraries -name '*.jar' | tr '\n' ':')
  RUN_CP="$RUN_CP:$LIBS"
fi
"$JAVAP_DIR/java" -XX:+UnlockExperimentalVMOptions -cp "$RUN_CP" MovePlaneSelfTestMain
