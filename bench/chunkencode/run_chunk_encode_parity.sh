#!/usr/bin/env bash
# TASK-78 part B — chunk-encode .so light-data parity + timing (headless, no server).
# Parity = vanilla decoder-constructor round trip (see ChunkEncodeParity.java javadoc).
# Exclusive BENCH.lock (timing section). Regenerating vanilla_mojang.jar:
#   see /tmp/expl/jp.sh recipe (NeoForged AutoRenamingTool over
#   /home/z/server/cache/mojang_1.21.10.jar nested 1.21.10 server jar, mappings 1.21.10.json).
set -u
cd "$(dirname "$0")"    # bench/chunkencode

REPO_ROOT="$(cd ../.. && pwd)"
CHUNK_SO="$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so"
MOJANG_JAR="${MOJANG_JAR:-/tmp/expl/vanilla_mojang.jar}"
LIBRARIES=/home/z/server/libraries

[ -f "$CHUNK_SO" ] || { echo "FATAL: $CHUNK_SO missing" >&2; exit 2; }
[ -f "$MOJANG_JAR" ] || { echo "FATAL: $MOJANG_JAR missing (rebuild via /tmp/expl/jp.sh recipe)" >&2; exit 2; }

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
else JAVAC=javac; JAVA=java; fi

NCP=$(find "$LIBRARIES" -name '*.jar' | tr '\n' ':')

mkdir -p classes results logs
$JAVAC -cp "$MOJANG_JAR:$NCP" -d classes \
  ../p500/java/net/minecraft/network/protocol/game/PaperNativeChunkPacketEncode.java \
  ChunkEncodeParity.java || exit 1

RAW=results/CHUNKENCODE_LIGHT_RAW.tsv
: > "$RAW"

exec 9>/home/z/BENCH.lock
flock 9
echo "BENCH.lock held $(date -u +%Y-%m-%dT%H:%M:%SZ)" >&2

OUT="logs/chunkencode_parity.out"; : > "$OUT"
timeout 240 "$JAVA" -Xms512m -Xmx1g -XX:+UseG1GC \
  -Dp500.libs="$CHUNK_SO" \
  -cp "classes:$MOJANG_JAR:$NCP" ChunkEncodeParity \
  > "$OUT" 2> "logs/chunkencode_parity.log"
rc=$?
flock -u 9

if [ $rc -ne 0 ]; then
  echo -e "PARITY_CRASH\texit=$rc" >> "$RAW"
  tail -15 "logs/chunkencode_parity.log" | sed 's/^/  log> /' >&2
else
  grep -E "^(PARITY|MISMATCH|TIMING|RATIO|SINK)" "$OUT" >> "$RAW" || echo -e "NO-LINES\trc=0" >> "$RAW"
fi
echo "--- raw ---"
cat "$RAW"
exit $rc
