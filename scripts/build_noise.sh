#!/usr/bin/env bash
# Rebuild the noise bridge classes (ImprovedNoiseNativeOps + Handle).
#
# These are include_bytes!'d into the plugin (src/improved_noise.rs) and
# defined at runtime INTO THE KERNEL JVM, so their class-file version must
# never exceed the oldest supported kernel JVM (Java 21 = major 65).
# A previous toolchain mismatch (javac 25 without --release) shipped major-69
# classes and broke the patch with UnsupportedClassVersionError — this script
# pins --release so that cannot recur.
#
# Usage: scripts/build_noise.sh [javac]   (default: JAVA javac or /home/z/jdk21)
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if command -v javac > /dev/null 2>&1; then JAVAC=javac
  elif [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi

RELEASE="${NOISE_RELEASE:-8}"   # major 52: runs on every kernel JVM we support
SRC_DIR=noise/net
OUT_DIR=noise/build

mkdir -p "$OUT_DIR"
# -d gives us the exact package layout; stubs (RuntimeStubs.java) provide the
# kernel shapes at compile time only — stub classes are discarded, not shipped.
"$JAVAC" --release "$RELEASE" -nowarn \
  -d "$OUT_DIR" \
  noise/net/minecraft/world/level/levelgen/synth/RuntimeStubs.java \
  noise/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps.java

# Drop the compile-time stub class files; only the two real bridge classes ship.
rm -f "$OUT_DIR"/net/minecraft/world/level/levelgen/synth/ImprovedNoise.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/synth/PaperNativeImprovedNoise.class

python3 - <<'EOF'
import glob, struct, sys
bad = 0
for f in sorted(glob.glob('noise/build/net/minecraft/world/level/levelgen/synth/*.class')):
    d = open(f, 'rb').read(8)
    major = struct.unpack('>H', d[6:8])[0]
    name = f.rsplit('/', 1)[-1]
    keep = name.startswith(('ImprovedNoiseNativeOps',))
    print(f"{name}: major {major} {'(ship)' if keep else '(dropped stub)' if not keep else ''}")
    if keep and major > 65:
        print(f"  ERROR: {name} major {major} exceeds kernel support (65 = Java 21)", file=sys.stderr)
        bad = 1
sys.exit(bad)
EOF

echo "noise bridge rebuilt OK"
