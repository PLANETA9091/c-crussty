#!/usr/bin/env bash
# Rebuild the area-map bridge classes (SingleUserAreaMapOps + $Scratch + $1).
#
# These are include_bytes!'d into the plugin (src/area_map.rs) and defined at
# runtime INTO THE KERNEL JVM (the map's own loader), so their class-file
# version must never exceed the running kernel JVM's. This script pins
# --release 8 (major 52) so a toolchain mismatch cannot recur (see the
# noise major-69 incident in the worklog).
#
# CRITICAL INVARIANT: SingleUserAreaMapOps$Scratch must ship with its array
# fields initialized (non-null). A null field survived one release and would
# NPE on the first patched SingleUserAreaMap.update() of every thread. The
# guard below fails the build if the embedded Scratch constant pool lacks the
# expected shape (newarray instructions in <init>).
#
# Usage: scripts/build_area_map.sh [javac]   (default: PATH javac or /home/z/jdk21)
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

RELEASE="${AREA_MAP_RELEASE:-8}"  # major 52: bootstrap/loader DefineClass target
SRC_DIR=area-map/ca/spottedleaf/moonrise/common/misc
OUT_DIR=area-map/build/ca/spottedleaf/moonrise/common/misc

mkdir -p "$OUT_DIR"
# Stubs (RuntimeStubs.java) provide kernel shapes at compile time only —
# stub classes are discarded, not shipped.
"$JAVAC" --release "$RELEASE" -nowarn \
  -d area-map/build \
  "$SRC_DIR/RuntimeStubs.java" \
  "$SRC_DIR/SingleUserAreaMapOps.java"

# Drop the compile-time stub class files; only the 3 real bridge classes ship
# (the exact set src/area_map.rs include_bytes!'s).
rm -f "$OUT_DIR/PaperNativeAreaMap.class" "$OUT_DIR/SingleUserAreaMap.class"

python3 - <<'EOF'
import struct, sys

SHIP = {
    "SingleUserAreaMapOps.class",
    "SingleUserAreaMapOps$Scratch.class",
    "SingleUserAreaMapOps$1.class",
}
OUT = "area-map/build/ca/spottedleaf/moonrise/common/misc/"

bad = 0
for name in sorted(SHIP):
    try:
        d = open(OUT + name, "rb").read()
    except FileNotFoundError:
        print(f"ERROR: {name} missing from build output", file=sys.stderr)
        bad = 1
        continue
    major = struct.unpack(">H", d[6:8])[0]
    print(f"{name}: major {major}")
    if major > 65:
        print(f"  ERROR: {name} major {major} exceeds kernel support (65 = Java 21)", file=sys.stderr)
        bad = 1

# Scratch NPE guard: <init> must contain newarray byte[] (0xbc 0x08) and
# long[] (0xbc 0x0b) — proof the arrays are allocated in the constructor,
# not left null.
scratch = open(OUT + "SingleUserAreaMapOps$Scratch.class", "rb").read()
if b"\xbc\x08" not in scratch or b"\xbc\x0b" not in scratch:
    print("ERROR: Scratch <init> does not allocate its arrays — null-array NPE on "
          "first update() (see A3 audit P0). Scratch fields MUST be initialized.",
          file=sys.stderr)
    bad = 1

sys.exit(bad)
EOF

echo "area-map bridge rebuilt OK"
