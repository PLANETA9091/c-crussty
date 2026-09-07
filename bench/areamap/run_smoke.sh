#!/usr/bin/env bash
# Compile + run the area-map smoke in both modes; TSV results to results/.
set -euo pipefail
cd "$(dirname "$0")/../.."   # repo root

JAVA_BIN="${1:-/home/z/jdk21/bin}"
[ -f area-map/build/ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps.class ] || scripts/build_area_map.sh

cd bench/areamap
rm -rf classes-fake classes-real results; mkdir -p classes-fake classes-real results
"$JAVA_BIN/javac" -nowarn -cp ../../area-map/build -d classes-fake \
  ca/spottedleaf/moonrise/common/misc/PaperNativeAreaMap.java \
  ca/spottedleaf/moonrise/common/misc/SingleUserAreaMap.java \
  ca/spottedleaf/moonrise/common/misc/RecAreaMap.java \
  ca/spottedleaf/moonrise/common/misc/AreaMapSmoke.java
cp -r classes-fake/* classes-real/
"$JAVA_BIN/javac" -nowarn -cp ../../area-map/build -d classes-real \
  realdecl/ca/spottedleaf/moonrise/common/misc/PaperNativeAreaMap.java

{
  echo "== FAKE mode (fast-path observable) =="
  "$JAVA_BIN/java" -cp classes-fake:../../area-map/build \
    ca.spottedleaf.moonrise.common.misc.AreaMapSmoke
  echo "== REAL mode (native parity) =="
  "$JAVA_BIN/java" -Dcrussty.native="$(cd ../.. && pwd)/native/libpaper_native_jni.so" \
    -cp classes-real:../../area-map/build \
    ca.spottedleaf.moonrise.common.misc.AreaMapSmoke
} 2>&1 | tee results/AREAMAP_SMOKE.log
