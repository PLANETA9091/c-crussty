#!/usr/bin/env bash
# Headless unit-smoke for the PATCHED SingleUserAreaMap.update() body
# (task 4-a: same-state fast path + Scratch init, verified via stubs).
#
# What this does, in order:
#   0. rebuilds the shipped bridge classes via scripts/build_area_map.sh
#      (also re-validates the Scratch newarray guard) and records sha256,
#   1. patches tests/fixtures/SingleUserAreaMap.class (real Paper kernel
#      bytes, major 65) with tests/area_map_smoke/patch_tool.py -- a Python
#      mirror of src/classfile.rs::patch_update. If the cargo-produced gold
#      file exists (/tmp/ccrussty_patched_SingleUserAreaMap.class, dumped by
#      `cargo test classfile::tests::patch_roundtrip`) the outputs are
#      byte-compared (exit 2 on mismatch),
#   2. compiles the harness: counting-fake native + SmokeProbe (variant A),
#      real-native declaration + SmokeProbe/System.load (variant B),
#      SmokeMain/RecordingMap/NaiveDiff, Boot,
#   3. variant A: JVM drives the patched update() with the counting stub:
#      CHK-1 Scratch init / no NPE on fresh threads (+ grow-only doubling),
#      CHK-2 same-state fast path (native invocation count delta == 0),
#      CHK-3 changed state (native delta == 1, callbacks == naive
#      set difference), NOT_SET + IllegalArgumentException guards,
#   4. variant B: same harness against the REAL native/libpaper_native_jni.so
#      bound by exact JNI symbol name (no counting; difference correctness
#      through the closed native).
#
# Exit codes: 0 = variant A PASS and variant B PASS
#             1 = variant A (mandatory) FAILED
#             2 = patch/byte-parity failure
#             3 = variant A PASS but variant B (real .so) FAILED
#
# Usage: tests/area_map_smoke/run.sh [runs]        (default 1; each run is a
#        fresh JVM; the whole pipeline is idempotent)
set -euo pipefail
cd "$(dirname "$0")/../.."

JAVA_BIN="${JAVA_HOME:-/home/z/jdk21}"
JAVA="$JAVA_BIN/bin/java"
JAVAC="$JAVA_BIN/bin/javac"
SMOKE=tests/area_map_smoke
BUILD="$SMOKE/build"
FIXTURE=tests/fixtures/SingleUserAreaMap.class
MISC_PKG=ca/spottedleaf/moonrise/common/misc

RUNS="${1:-1}"

# --- step 0: shipped bridge classes --------------------------------------
echo "== [0/4] rebuild shipped bridge classes (scripts/build_area_map.sh)"
bash scripts/build_area_map.sh
sha256sum area-map/build/$MISC_PKG/SingleUserAreaMapOps*.class

# --- step 1: patched kernel bytes ----------------------------------------
echo "== [1/4] patch $FIXTURE (patch_tool.py mirror of classfile::patch_update)"
GOLD=/tmp/ccrussty_patched_SingleUserAreaMap.class
GOLD_ARGS=()
if [ -f "$GOLD" ]; then
  GOLD_ARGS=(--verify "$GOLD")
else
  echo "note: gold bytes not found ($GOLD) -- byte-parity vs Rust SKIPPED;"
  echo "      produce with: cargo test classfile::tests::patch_roundtrip"
fi
python3 "$SMOKE/patch_tool.py" --fixture "$FIXTURE" --out "$BUILD/patched_SingleUserAreaMap.class" "${GOLD_ARGS[@]}"

python3 - "$FIXTURE" "$BUILD/patched_SingleUserAreaMap.class" <<'EOF'
import sys
orig = open(sys.argv[1], "rb").read()
patched = open(sys.argv[2], "rb").read()
assert b"SingleUserAreaMapOps" not in orig, "original fixture must not reference the Ops class"
assert b"SingleUserAreaMapOps" in patched, "patched bytes must reference the Ops class"
print(f"canary OK: original {len(orig)} B has no Ops reference; patched {len(patched)} B references Ops")
EOF

# --- step 2: compile the harness ------------------------------------------
echo "== [2/4] compile harness"
mkdir -p "$BUILD/misc/$MISC_PKG" "$BUILD/misc_real/$MISC_PKG" "$BUILD/harness" "$BUILD/boot"
# shipped bridge bytes (include_bytes!'d by src/area_map.rs) into both variants
for cls in "SingleUserAreaMapOps.class" 'SingleUserAreaMapOps$Scratch.class' 'SingleUserAreaMapOps$1.class'; do
  cp "area-map/build/$MISC_PKG/$cls" "$BUILD/misc/$MISC_PKG/"
  cp "area-map/build/$MISC_PKG/$cls" "$BUILD/misc_real/$MISC_PKG/"
done
# variant A: counting fake native + probe; variant B: real native decl + probe
"$JAVAC" --release 8 -d "$BUILD/misc" "$SMOKE"/java/fake_native/$MISC_PKG/*.java
"$JAVAC" --release 8 -d "$BUILD/misc_real" "$SMOKE"/java/real_so/$MISC_PKG/*.java
# compile-only view of the real kernel class shape (major 65 fixture): the
# harness must type-check against the REAL SingleUserAreaMap (update(III)Z,
# getLastChunkX/...), never against a stub shape. At RUNTIME the loader
# defines the PATCHED bytes instead -- this dir is not on the runtime URLs.
mkdir -p "$BUILD/fixture_ca/$MISC_PKG"
cp "$FIXTURE" "$BUILD/fixture_ca/$MISC_PKG/"
# harness + boot
"$JAVAC" --release 17 -d "$BUILD/harness" -cp "$BUILD/misc:$BUILD/fixture_ca" "$SMOKE"/java/harness/areamapsmoke/*.java
"$JAVAC" --release 17 -d "$BUILD/boot" "$SMOKE"/java/boot/Boot.java

# --- steps 3+4: the two variants (fresh JVM each) --------------------------
run_variant() { # $1 = variant dir name, $2 = extra JVM args
  local dir="$1"; shift
  "$JAVA" "$@" -cp "$BUILD/boot" Boot "$SMOKE" "$dir"
}

rc_a=0; rc_b=0
for run in $(seq 1 "$RUNS"); do
  echo "== [3/4] variant A: stub-counting native (run $run/$RUNS, fresh JVM)"
  if run_variant misc; then rc_a=0; else rc_a=$?; fi
  echo "== [4/4] variant B: real native/libpaper_native_jni.so (run $run/$RUNS, fresh JVM)"
  if run_variant misc_real -Damsmoke.so="$PWD/native/libpaper_native_jni.so"; then rc_b=0; else rc_b=$?; fi
  if [ "$rc_a" -ne 0 ] || [ "$rc_b" -ne 0 ]; then break; fi
done

echo "== verdict"
if [ "$rc_a" -ne 0 ]; then
  echo "RESULT: FAIL (variant A / mandatory checks failed, rc=$rc_a)"
  exit 1
fi
if [ "$rc_b" -ne 0 ]; then
  echo "RESULT: PARTIAL (variant A PASS; variant B real-.so failed, rc=$rc_b)"
  exit 3
fi
echo "RESULT: PASS (variant A stub + variant B real-.so, $RUNS run(s), all checks green)"
exit 0
