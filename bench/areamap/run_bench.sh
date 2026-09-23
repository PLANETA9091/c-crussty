#!/usr/bin/env bash
# TASK-20: area-map apply-loop vs same-state fast-path micro-bench (ns/px at
# nominal 128/512/1024 grids). Mirrors run_smoke.sh: compiles the bench
# classes in both modes (fake = pure-Java enumeration reference, real = the
# shipped libpaper_native_jni.so) and runs the driver; raw TSV-ish lines to
# results/APPLY_BENCH.log. NOT a CI gate; light local bench (~30-60 s).
#
# Usage: ./run_bench.sh [jdk-bin-dir]   (default /home/z/jdk21/bin)
set -euo pipefail
cd "$(dirname "$0")/../.."   # repo root

JAVA_BIN="${1:-/home/z/jdk21/bin}"
[ -f area-map/build/ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps.class ] || scripts/build_area_map.sh

cd bench/areamap
rm -rf classes-bench-fake classes-bench-real results; mkdir -p classes-bench-fake classes-bench-real results
"$JAVA_BIN/javac" -nowarn -cp ../../area-map/build -d classes-bench-fake \
  ca/spottedleaf/moonrise/common/misc/PaperNativeAreaMap.java \
  ca/spottedleaf/moonrise/common/misc/SingleUserAreaMap.java \
  ca/spottedleaf/moonrise/common/misc/AreaMapApplyBench.java
cp -r classes-bench-fake/* classes-bench-real/
"$JAVA_BIN/javac" -nowarn -cp ../../area-map/build -d classes-bench-real \
  realdecl/ca/spottedleaf/moonrise/common/misc/PaperNativeAreaMap.java

{
  echo "== FAKE mode (pure-Java enumeration reference; fast-path call-count observable) =="
  "$JAVA_BIN/java" -Xms512m -Xmx512m -cp classes-bench-fake:../../area-map/build \
    ca.spottedleaf.moonrise.common.misc.AreaMapApplyBench
  echo "== REAL mode (production path: libpaper_native_jni.so enumerate + apply) =="
  "$JAVA_BIN/java" -Xms512m -Xmx512m \
    -Dcrussty.native="$(cd ../.. && pwd)/native/libpaper_native_jni.so" \
    -cp classes-bench-real:../../area-map/build \
    ca.spottedleaf.moonrise.common.misc.AreaMapApplyBench
} 2>&1 | tee results/APPLY_BENCH.log
