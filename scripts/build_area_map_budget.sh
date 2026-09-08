#!/usr/bin/env bash
# Rebuild the BUDGETED area-map bridge classes (TASK-64 variant C).
#
# Produces the same three-class set as scripts/build_area_map.sh
# (SingleUserAreaMapOps + $Scratch + $1) but from the budgeted-scratch source
# (area-map/budget-ca/.../SingleUserAreaMapOpsBudget.java, class name
# SingleUserAreaMapOps — see that file for the file/class-name trick).
# Output: area-map/build-budget/ — include_bytes!'d by src/area_map.rs and
# selected at activation time by the CRUSSTY_AREAMAP_BUDGET gate (default
# off = legacy bytes from area-map/build/, zero behavioral delta).
#
# Same invariants as the legacy build: class-file major pinned via --release
# 8 (major 52 — a toolchain mismatch must not recur), Scratch must ship with
# its array fields initialized (newarray guard), stub classes discarded.
#
# Usage: scripts/build_area_map_budget.sh [javac]   (default: PATH javac or /home/z/jdk21)
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

RELEASE="${AREA_MAP_RELEASE:-8}"  # major 52: bootstrap/loader DefineClass target
STUBS=area-map/ca/spottedleaf/moonrise/common/misc/RuntimeStubs.java
SRC=area-map/budget-ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOpsBudget.java
OUT_DIR=area-map/build-budget/ca/spottedleaf/moonrise/common/misc

mkdir -p "$OUT_DIR"
"$JAVAC" --release "$RELEASE" -nowarn \
  -d area-map/build-budget \
  "$STUBS" \
  "$SRC"

# Drop the compile-time stub class files; only the 3 real bridge classes ship
# (the exact set src/area_map.rs include_bytes!'s — same names as legacy).
rm -f "$OUT_DIR/PaperNativeAreaMap.class" "$OUT_DIR/SingleUserAreaMap.class"

python3 - <<'EOF'
import struct, sys

SHIP = {
    "SingleUserAreaMapOps.class",
    "SingleUserAreaMapOps$Scratch.class",
    "SingleUserAreaMapOps$1.class",
}
OUT = "area-map/build-budget/ca/spottedleaf/moonrise/common/misc/"

bad = 0
for name in sorted(SHIP):
    try:
        d = open(OUT + name, "rb").read()
    except FileNotFoundError:
        print(f"ERROR: {name} missing from budget build output", file=sys.stderr)
        bad = 1
        continue
    major = struct.unpack(">H", d[6:8])[0]
    print(f"{name}: major {major}")
    if major > 65:
        print(f"  ERROR: {name} major {major} exceeds kernel support (65 = Java 21)", file=sys.stderr)
        bad = 1

# Scratch NPE guard (same as legacy build): <init> must contain newarray
# byte[] (0xbc 0x08) and long[] (0xbc 0x0b) — proof the arrays are allocated
# in the constructor, not left null.
scratch = open(OUT + "SingleUserAreaMapOps$Scratch.class", "rb").read()
if b"\xbc\x08" not in scratch or b"\xbc\x0b" not in scratch:
    print("ERROR: budget Scratch <init> does not allocate its arrays — null-array "
          "NPE on first update() (see A3 audit P0). Scratch fields MUST be initialized.",
          file=sys.stderr)
    bad = 1

sys.exit(bad)
EOF

echo "area-map budget bridge rebuilt OK"
