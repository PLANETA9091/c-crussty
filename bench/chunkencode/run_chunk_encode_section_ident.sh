#!/usr/bin/env bash
# TASK-149 phase-1 — nativeEncodeSectionData ABI identification (headless, no server).
# Reference = REAL vanilla PalettedContainer.write + LevelChunkSection.write order
# from the mojang-mapped kernel jar /home/z/server/versions/1.21.10/purpur-1.21.10.jar.
# Exclusive BENCH.lock (whole run). Kernel jar direct classpath — no renamed jar needed.
set -u
cd "$(dirname "$0")"    # bench/chunkencode

REPO_ROOT="$(cd ../.. && pwd)"
CHUNK_SO="$(realpath "$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so")"
KERNEL_JAR=/home/z/server/versions/1.21.10/purpur-1.21.10.jar
LIBRARIES=/home/z/server/libraries

[ -f "$CHUNK_SO" ] || { echo "FATAL: $CHUNK_SO missing" >&2; exit 2; }
[ -f "$KERNEL_JAR" ] || { echo "FATAL: $KERNEL_JAR missing" >&2; exit 2; }

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
else JAVAC=javac; JAVA=java; fi

NCP=$(find "$LIBRARIES" -name '*.jar' | tr '\n' ':')

mkdir -p classes results logs
$JAVAC -cp "$KERNEL_JAR:$NCP" -d classes \
  ../p500/java/net/minecraft/network/protocol/game/PaperNativeChunkPacketEncode.java \
  ChunkEncodeSectionIdent.java || exit 1

RAW=results/CHUNKENCODE_SECTION_IDENT_RAW.tsv
: > "$RAW"

exec 9>/home/z/BENCH.lock
flock 9
echo "BENCH.lock held $(date -u +%Y-%m-%dT%H:%M:%SZ)" >&2

OUT="logs/chunkencode_section_ident.out"; : > "$OUT"
timeout 240 "$JAVA" -Xms512m -Xmx1g \
  -Dchunkencode.libs="$CHUNK_SO" \
  -cp "classes:$KERNEL_JAR:$NCP" ChunkEncodeSectionIdent \
  > "$OUT" 2> "logs/chunkencode_section_ident.log"
rc=$?
flock -u 9

if [ $rc -ne 0 ]; then
  echo -e "IDENT_CRASH\texit=$rc" >> "$RAW"
  tail -15 "logs/chunkencode_section_ident.log" | sed 's/^/  log> /' >&2
else
  grep -E "^(IDENT|HEXDIFF|SECTION|SINK)" "$OUT" >> "$RAW" || echo -e "NO-LINES\trc=0" >> "$RAW"
fi
echo "--- raw ---"
cat "$RAW"
exit $rc
