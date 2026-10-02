#!/usr/bin/env bash
# materialize_kernel.sh — offline patched-kernel materialization (task165/S7-96)
# Usage: materialize_kernel.sh <paperclip.jar> <cache_dir_with_mojang_jar> <out_dir>
# Runs paperclip ONLY to apply patches and write versions/<v>/<kernel>.jar,
# then KILLS it before server main starts (NOT a server boot — INJECTS-ONLY safe).
# Resulting jar is byte-identical to the CI booted kernel (same paperclip+patch+vanilla).
set -euo pipefail
PAPERCLIP="${1:?paperclip.jar}"
CACHE="${2:?cache dir containing mojang_*.jar}"
OUT="${3:?output dir}"
WORK="$(mktemp -d /tmp/kernelmat.XXXXXX)"
trap 'pkill -f "java -jar.*$(basename "$PAPERCLIP")" 2>/dev/null || true' EXIT
mkdir -p "$WORK/server/cache"
cp "$CACHE"/mojang_*.jar "$WORK/server/cache/"
cp "$PAPERCLIP" "$WORK/server/"
cd "$WORK/server"
java -jar "$(basename "$PAPERCLIP")" nogui > paperclip.log 2>&1 &
PCPID=$!
for _ in $(seq 1 90); do
  JAR="$(find "$WORK/server/versions" -name '*.jar' -size +10M 2>/dev/null | head -1 || true)"
  [ -n "$JAR" ] && break
  sleep 1
done
kill "$PCPID" 2>/dev/null || true
wait "$PCPID" 2>/dev/null || true
[ -n "${JAR:-}" ] || { echo "ERROR: versions jar never materialized; log tail:"; tail -20 paperclip.log; exit 1; }
mkdir -p "$OUT"
cp "$JAR" "$OUT/patched-kernel.jar"
echo "materialized: $OUT/patched-kernel.jar ($(du -h "$OUT/patched-kernel.jar" | cut -f1))"
echo "verify: contains ServerLevel? $(unzip -l "$OUT/patched-kernel.jar" | grep -c 'net/minecraft/server/level/ServerLevel.class') matches"
