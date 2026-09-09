#!/usr/bin/env bash
# Rebuild the noise bridge classes (ImprovedNoiseNativeOps + Handle).
#
# These are include_bytes!'d into the plugin (src/improved_noise.rs) and
# defined at runtime INTO THE KERNEL JVM, so their class-file version must
# never exceed the running kernel JVM's. A previous toolchain mismatch
# (javac 25 without --release) shipped major-69 classes and broke the patch
# with UnsupportedClassVersionError — this script pins --release so that
# cannot recur.
#
# Why --release 11 (major 55) instead of 8 (major 52):
#   the bridge now uses java.lang.ref.Cleaner (Java 9+ API) for handle
#   lifecycle instead of finalize(). Cleaner cannot be referenced at all
#   under --release 8 (compile error) and a reflective loader would trade
#   compile-time safety for brittleness. Major 55 is safe because the ONLY
#   consumer is the kernel JVM: Minecraft 1.21.x requires Java 21 (major
#   65), and src/improved_noise.rs guards activation by comparing the
#   embedded bridge major against the live JVM's java.class.version — an
#   older kernel keeps the hook dormant instead of crashing. NOISE_RELEASE
#   may override for experiments, but anything > 65 fails the guard below
#   and > 65/55/52 fails the runtime guard on Java 21/11/8 kernels.
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

RELEASE="${NOISE_RELEASE:-8}"   # major 52: phantom-reaper bridge needs nothing from Java 9+
# (a Cleaner-based variant would require --release 11; the runtime guard in
# src/improved_noise.rs would still keep it dormant on older kernels)
SRC_DIR=noise/net
OUT_DIR=noise/build

mkdir -p "$OUT_DIR"
# -d gives us the exact package layout; stubs (RuntimeStubs.java + the
# crussty/batch bridge stub) provide the kernel shapes at compile time only
# — stub classes are discarded, not shipped. ImprovedNoiseBatchOps.java is
# the G4 demonstrator helper (docs/G4_SITE_PATCH_DESIGN.md §5.1): same-
# descriptor retarget target, 4th embedded class in src/improved_noise.rs.
"$JAVAC" --release "$RELEASE" -nowarn \
  -d "$OUT_DIR" \
  noise/net/minecraft/world/level/levelgen/synth/RuntimeStubs.java \
  noise/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps.java \
  noise/net/minecraft/world/level/levelgen/synth/ImprovedNoiseBatchOps.java \
  noise/net/minecraft/world/level/levelgen/synth/PerlinNoiseNativeOps.java \
  noise/net/minecraft/world/level/levelgen/synth/ImprovedNoise.java \
  noise/net/minecraft/world/level/levelgen/synth/PerlinNoise.java \
  noise/net/minecraft/world/level/levelgen/synth/PaperNativePerlinNoise.java \
  noise/net/minecraft/world/level/levelgen/synth/NormalNoise.java \
  noise/net/minecraft/world/level/levelgen/synth/PaperNativeNormalNoise.java \
  noise/net/minecraft/world/level/levelgen/DensityStubs.java \
  noise/net/minecraft/world/level/levelgen/NormalNoiseBatchOps.java \
  noise/net/minecraft/world/level/levelgen/DensityArrayInterpreter.java \
  noise/net/minecraft/util/RandomSource.java \
  noise/net/minecraft/util/Mth.java \
  noise/net/minecraft/util/KeyDispatchDataCodec.java \
  noise/net/minecraft/core/Holder.java \
  noise/net/it/unimi/dsi/fastutil/doubles/DoubleList.java \
  noise/net/crussty/batch/PaperNativeBatchDispatch.java

# Drop the compile-time stub class files; only the real bridge classes ship.
rm -f "$OUT_DIR"/net/minecraft/world/level/levelgen/synth/ImprovedNoise.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/synth/PerlinNoise.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/synth/PaperNativeImprovedNoise.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/synth/PaperNativePerlinNoise.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/synth/NormalNoise.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/synth/PaperNativeNormalNoise.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/DensityFunction.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/DensityFunction\$*.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/DensityFunctions.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/DensityFunctions\$*.class \
      "$OUT_DIR"/net/minecraft/util/RandomSource.class \
      "$OUT_DIR"/net/minecraft/util/KeyDispatchDataCodec.class \
      "$OUT_DIR"/net/minecraft/util/Mth.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/NoiseChunk.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/NoiseChunk\$*.class \
      "$OUT_DIR"/net/minecraft/core/Holder.class \
      "$OUT_DIR"/it/unimi/dsi/fastutil/doubles/DoubleList.class \
      "$OUT_DIR"/crussty/batch/PaperNativeBatchDispatch.class
rmdir "$OUT_DIR"/net/minecraft/util "$OUT_DIR"/net/minecraft/core 2>/dev/null || true
# Stale synthetics from prior builds must not linger (javac never cleans;
# the ship audit below treats any stray as fatal).
rm -f "$OUT_DIR"/net/minecraft/world/level/levelgen/NormalNoiseBatchOps\$[0-9]*.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps\$[0-9]*.class \
      "$OUT_DIR"/net/minecraft/world/level/levelgen/synth/PerlinNoiseNativeOps\$[0-9]*.class

python3 - <<'EOF'
import glob, struct, sys

# The runtime embed contract: src/improved_noise.rs include_bytes!s and
# defines EXACTLY these four classes into the kernel loader. Anything else
# left in noise/build would silently not ship -> NoClassDefFoundError on
# first use inside the kernel (e.g. a stray synthetic $1 from a private
# nested ctor under pre-nestmates targets). The phantom-reaper design ships
# an explicit named $Reaper class (lambda-with-state cannot drain a queue
# across timeouts without capturing mutable state, which lambdas forbid).
# ImprovedNoiseBatchOps is the G4 demonstrator helper (4th embed).
SHIP = {"ImprovedNoiseNativeOps.class", "ImprovedNoiseNativeOps$Handle.class",
        "ImprovedNoiseNativeOps$Reaper.class", "ImprovedNoiseBatchOps.class",
        "PerlinNoiseNativeOps.class", "PerlinNoiseNativeOps$Handle.class",
        "PerlinNoiseNativeOps$Reaper.class"}
# TASK-108: src/noise_fill.rs embeds exactly these (levelgen package).
SHIP_FILL = {"NormalNoiseBatchOps.class", "NormalNoiseBatchOps$Handle.class",
             "NormalNoiseBatchOps$Reaper.class", "NormalNoiseBatchOps$Recorder.class",
             "NormalNoiseBatchOps$RecorderTL.class", "NormalNoiseBatchOps$RecOutTL.class",
             "NormalNoiseBatchOps$Census.class", "NormalNoiseBatchOps$TestProvider.class",
             "DensityArrayInterpreter.class"}

bad = 0
files = sorted(glob.glob('noise/build/net/minecraft/world/level/levelgen/synth/*.class'))
for f in files:
    d = open(f, 'rb').read(8)
    major = struct.unpack('>H', d[6:8])[0]
    name = f.rsplit('/', 1)[-1]
    keep = name in SHIP
    print(f"{name}: major {major} {'(ship)' if keep else '(dropped stub)'}")
    if keep and major > 65:
        print(f"  ERROR: {name} major {major} exceeds kernel support (65 = Java 21)", file=sys.stderr)
        bad = 1
    if not keep and (name.startswith('ImprovedNoiseNativeOps') or name.startswith('ImprovedNoiseBatchOps')):
        print(f"  ERROR: {name} is an unembedded bridge class — src/improved_noise.rs defines only {sorted(SHIP)}; "
              "keep the bridge at exactly 4 class files (Ops, $Handle, $Reaper, BatchOps); "
              "anonymous inner classes / lambdas-that-capture would mint synthetics the define loop never defines",
              file=sys.stderr)
        bad = 1
fill_files = sorted(glob.glob('noise/build/net/minecraft/world/level/levelgen/*.class'))
for f in fill_files:
    d = open(f, 'rb').read(8)
    major = struct.unpack('>H', d[6:8])[0]
    name = f.rsplit('/', 1)[-1]
    keep = name in SHIP_FILL
    print(f"{name}: major {major} {'(ship)' if keep else '(dropped stub)'}")
    if keep and major > 65:
        print(f"  ERROR: {name} major {major} exceeds kernel support (65 = Java 21)", file=sys.stderr)
        bad = 1
    if not keep:
        print(f"  ERROR: {name} is an unembedded bridge class — src/noise_fill.rs defines only {sorted(SHIP_FILL)}; "
              "anonymous inner classes / lambdas-that-capture would mint synthetics the define loop never defines",
              file=sys.stderr)
        bad = 1
shipped = {f.rsplit('/', 1)[-1] for f in files if f.rsplit('/', 1)[-1] in SHIP}
missing = SHIP - shipped
if missing:
    print(f"  ERROR: expected bridge class(es) missing from build output: {sorted(missing)}", file=sys.stderr)
    bad = 1
shipped_fill = {f.rsplit('/', 1)[-1] for f in fill_files if f.rsplit('/', 1)[-1] in SHIP_FILL}
missing_fill = SHIP_FILL - shipped_fill
if missing_fill:
    print(f"  ERROR: expected fill bridge class(es) missing from build output: {sorted(missing_fill)}", file=sys.stderr)
    bad = 1
sys.exit(bad)
EOF

echo "noise bridge rebuilt OK"
