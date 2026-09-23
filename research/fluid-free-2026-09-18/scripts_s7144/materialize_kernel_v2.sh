#!/usr/bin/env bash
# materialize_kernel_v2.sh — S7-144: variant that lets paperclip DOWNLOAD the
# mojang jars itself (cache wiped by WIPE 03:43). Kill-before-main = NOT a boot.
# Usage: materialize_kernel_v2.sh <paperclip.jar> <workdir> <out_dir>
set -euo pipefail
PAPERCLIP="${1:?paperclip.jar}"
WORK="${2:?work dir}"
OUT="${3:?output dir}"
export PATH=/tmp/toolchain/jdk-21.0.12.1+1/bin:$PATH
mkdir -p "$WORK/server/cache" "$OUT"
cp "$PAPERCLIP" "$WORK/server/"
cd "$WORK/server"
(java -jar "$(basename "$PAPERCLIP")" nogui > paperclip.log 2>&1; echo "PAPERCLIP_EXIT=$?" >> paperclip.log) &
PCPID=$!
JAR=""
for _ in $(seq 1 240); do
  JAR="$(find "$WORK/server/versions" -name '*.jar' -size +10M 2>/dev/null | head -1 || true)"
  [ -n "$JAR" ] && break
  sleep 1
done
pkill -f "$(basename "$PAPERCLIP")" 2>/dev/null || true
kill "$PCPID" 2>/dev/null || true
wait "$PCPID" 2>/dev/null || true
[ -n "$JAR" ] || { echo "ERROR: versions jar never materialized; log tail:"; tail -20 paperclip.log; exit 1; }
cp "$JAR" "$OUT/patched-kernel.jar"
echo "materialized: $OUT/patched-kernel.jar ($(du -h "$OUT/patched-kernel.jar" | cut -f1))"
# mojang jars are now in cache — bank them for future runs without download
ls "$WORK/server/cache/" 2>/dev/null | head -5 || true
