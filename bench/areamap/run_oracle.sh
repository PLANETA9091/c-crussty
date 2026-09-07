#!/usr/bin/env bash
# TASK-30 — per-call correctness oracle (run_oracle.sh).
# Correctness-only: multiset parity of pipeline-emitted ops vs the naive
# set difference on deterministic move/resize streams, both JVM modes.
# File-disjoint from the sibling benches (classes-oracle-*, results/TASK30_ORACLE.md).
set -euo pipefail
cd "$(dirname "$0")/../.."   # repo root

JAVA_BIN="${1:-/home/z/jdk21/bin}"
[ -f area-map/build/ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps.class ] || scripts/build_area_map.sh

cd bench/areamap
rm -rf classes-oracle-fake classes-oracle-real
mkdir -p classes-oracle-fake classes-oracle-real results

PKG=ca/spottedleaf/moonrise/common/misc

"$JAVA_BIN/javac" -nowarn -cp ../../area-map/build -d classes-oracle-fake \
  $PKG/PaperNativeAreaMap.java \
  $PKG/SingleUserAreaMap.java \
  benchjava/$PKG/OracleBench.java
cp -r classes-oracle-fake/* classes-oracle-real/
"$JAVA_BIN/javac" -nowarn -cp ../../area-map/build -d classes-oracle-real \
  realdecl/$PKG/PaperNativeAreaMap.java

RAW=results/task30_oracle_raw.tsv
: > "$RAW"

echo "== FAKE mode (counting stub, capacity observable) ==" | tee -a "$RAW"
"$JAVA_BIN/java" -Xms512m -Xmx512m -cp classes-oracle-fake:../../area-map/build \
  $PKG.OracleBench 2>&1 | tee -a "$RAW"

echo "== REAL mode (native .so) ==" | tee -a "$RAW"
"$JAVA_BIN/java" -Xms512m -Xmx512m \
  -Dcrussty.native="$(cd ../.. && pwd)/native/libpaper_native_jni.so" \
  -cp classes-oracle-real:../../area-map/build \
  $PKG.OracleBench 2>&1 | tee -a "$RAW"

echo "raw TSV: $RAW"
