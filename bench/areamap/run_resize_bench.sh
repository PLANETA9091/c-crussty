#!/usr/bin/env bash
# TASK-20 — area-map apply-loop micro-bench (run_apply_bench.sh).
# Timing companion of run_smoke.sh: same fixtures, no-log sink, 3 sizes.
# Does NOT touch classes-fake/classes-real/results beyond its own files, so a
# smoke rerun cannot nuke bench artifacts and vice versa.
set -euo pipefail
cd "$(dirname "$0")/../.."   # repo root

JAVA_BIN="${1:-/home/z/jdk21/bin}"
[ -f area-map/build/ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps.class ] || scripts/build_area_map.sh

cd bench/areamap
rm -rf classes-bench-resize-fake classes-bench-resize-real
mkdir -p classes-bench-resize-fake classes-bench-resize-real results

PKG=ca/spottedleaf/moonrise/common/misc

"$JAVA_BIN/javac" -nowarn -cp ../../area-map/build -d classes-bench-resize-fake \
  $PKG/PaperNativeAreaMap.java \
  $PKG/SingleUserAreaMap.java \
  benchjava/$PKG/AreaMapBench.java
cp -r classes-bench-resize-fake/* classes-bench-resize-real/
"$JAVA_BIN/javac" -nowarn -cp ../../area-map/build -d classes-bench-resize-real \
  realdecl/$PKG/PaperNativeAreaMap.java

RAW=results/apply_bench_raw.tsv
: > "$RAW"

echo "== FAKE mode (fast-path observable, counting stub) ==" | tee -a "$RAW"
"$JAVA_BIN/java" -Xms512m -Xmx512m -XX:+UseG1GC -cp classes-bench-resize-fake:../../area-map/build \
  $PKG.AreaMapBench 2>&1 | tee -a "$RAW"

echo "== REAL mode (native .so) ==" | tee -a "$RAW"
"$JAVA_BIN/java" -Xms512m -Xmx512m -XX:+UseG1GC \
  -Dcrussty.native="$(cd ../.. && pwd)/native/libpaper_native_jni.so" \
  -cp classes-bench-resize-real:../../area-map/build \
  $PKG.AreaMapBench 2>&1 | tee -a "$RAW"

echo "raw TSV: $RAW"
