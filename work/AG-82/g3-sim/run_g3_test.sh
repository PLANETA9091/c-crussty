#!/usr/bin/env bash
# $1 = block file, $2 = server marker-delay (s), $3 = log file
set -u
BLOCK="$1"; DELAY="$2"; LOGF="$3"
log() { echo "[t $(date -u +%H:%M:%S)] $*"; }
FAIL=0
cmd() { true; }
DP_ENABLED=0
rm -f "$LOGF"; touch "$LOGF"
( sleep "$DELAY"; printf 'There are 7 data pack(s) enabled: [vanilla], [file/bukkit], [file/terralith.zip (world)], [file/tectonic.zip (world)], [file/incendium.zip (world)], [file/stellarity.zip (world)]\n' >> "$LOGF" ) >/dev/null 2>&1 &
SRV=$!
T0=$(date +%s.%N)
source "$BLOCK"
T1=$(date +%s.%N)
kill $SRV 2>/dev/null; wait $SRV 2>/dev/null
echo "RESULT block=$(basename $BLOCK) delay=${DELAY}s DP_ENABLED=$DP_ENABLED FAIL=$FAIL elapsed=awk "BEGIN{printf \"%.1f\", $T1 - $T0}""
