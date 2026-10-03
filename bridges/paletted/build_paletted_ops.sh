#!/usr/bin/env bash
# Build the PALETTED-DEMUX artifacts (S7-131):
#   1. ASM build-time patcher (COMPUTE_FRAMES — the JVM verifier rejected the
#      hand-rolled StackMapTable; ASM recomputes frames natively)
#   2. stub jar: materialized kernel with PalettedContainer.class REPLACED by
#      the patched image (compile-time shapes for the Ops class)
#   3. ECJ build of PalettedContainerOps against the stub jar
#   4. JVM parity harness: vanilla (L1) vs patched (L2) lockstep, 20k random
#      ops, resize ladder, demux lifecycle, concurrency smoke
#
# NOT a boot: plain JVM class loading + reflection only (INJECTS-ONLY safe).
set -euo pipefail
HERE="$(cd "$(dirname "$0")/.." && pwd)"   # c-crussty root
KERNEL_JAR="${KERNEL_JAR:-/tmp/pdec/matsrv/versions/1.21.10/purpur-1.21.10.jar}"
ASM_DIR="${ASM_DIR:-/home/z/my-project/scripts/f1_batch_rng}"
ECJ="${ECJ:-$HERE/randomtick/ecj.jar}"
OUT="$HERE/paletted/build"
WORK="$(mktemp -d /tmp/paletted.XXXXXX)"

command -v java >/dev/null || { echo "java not found" >&2; exit 1; }
[ -f "$KERNEL_JAR" ] || { echo "KERNEL_JAR not found: $KERNEL_JAR" >&2; exit 1; }
[ -f "$ECJ" ] || { echo "ECJ not found: $ECJ" >&2; exit 1; }
[ -f "$ASM_DIR/asm.jar" ] || { echo "asm.jar not found in $ASM_DIR" >&2; exit 1; }

TOOL_CP="$ASM_DIR/asm.jar:$ASM_DIR/asm-commons.jar:$ASM_DIR/asm-tree.jar:$ASM_DIR/asm-util.jar"

echo "== 1. ASM patcher =="
mkdir -p "$HERE/tests/out" "$OUT" "$WORK/tools"
java -jar "$ECJ" -source 21 -target 21 -cp "$TOOL_CP" -d "$WORK/tools" \
  "$HERE/paletted/tools/PalettedPatchTool.java"
java -cp "$TOOL_CP:$WORK/tools:$KERNEL_JAR" PalettedPatchTool \
  "$HERE/tests/fixtures/PalettedContainer.class" \
  "$HERE/tests/out/PalettedContainer.patched.class"
PATCHED="$HERE/tests/out/PalettedContainer.patched.class"
[ -s "$PATCHED" ] || { echo "patched class artifact missing" >&2; exit 1; }
cp "$PATCHED" "$HERE/paletted/build/PalettedContainer.patched.class"   # runtime embed

echo "== 2. stub jar (kernel with PalettedContainer replaced by the patched image) =="
python3 - "$KERNEL_JAR" "$PATCHED" "$WORK/stub.jar" <<'PY'
import sys, zipfile, os
src, patched, out = sys.argv[1], sys.argv[2], sys.argv[3]
target = "net/minecraft/world/level/chunk/PalettedContainer.class"
with zipfile.ZipFile(src) as zin, zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as zout:
    for item in zin.infolist():
        if item.filename != target:
            zout.writestr(item, zin.read(item.filename))
    with open(patched, "rb") as f:
        zout.writestr(target, f.read())
print("stub jar rewritten")
PY
echo "stub jar: $(du -h "$WORK/stub.jar" | cut -f1)"

echo "== 3. ECJ build PalettedContainerOps =="
java -jar "$ECJ" -source 21 -target 21 -cp "$WORK/stub.jar" -d "$OUT" \
  "$HERE/paletted/net/minecraft/world/level/chunk/PalettedContainerOps.java"
OPS_CLASS="$OUT/net/minecraft/world/level/chunk/PalettedContainerOps.class"
[ -s "$OPS_CLASS" ] || { echo "Ops class build failed" >&2; exit 1; }
echo "built: $OPS_CLASS ($(stat -c%s "$OPS_CLASS") B)"

echo "== 4. JVM parity harness =="
java -jar "$ECJ" -source 21 -target 21 -cp "$WORK/stub.jar" -d "$WORK/harness" \
  "$HERE/paletted/harness/ParityHarness.java"
java -cp "$WORK/stub.jar:$OUT:$WORK/harness" ParityHarness "$KERNEL_JAR" "$WORK/stub.jar" "$OUT" "${1:-424242}" "${2:-/tmp/pdec/matsrv/libraries}"
