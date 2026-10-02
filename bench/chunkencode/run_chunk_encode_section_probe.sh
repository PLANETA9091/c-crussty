#!/usr/bin/env bash
# TASK-149 phase-1 — differential rc probe for nativeEncodeSectionData/-Sized.
set -u
cd "$(dirname "$0")"
REPO_ROOT="$(cd ../.. && pwd)"
CHUNK_SO="$(realpath "$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so")"
KERNEL_JAR=/home/z/server/versions/1.21.10/purpur-1.21.10.jar
LIBRARIES=/home/z/server/libraries
[ -f "$CHUNK_SO" ] || { echo "FATAL: $CHUNK_SO missing" >&2; exit 2; }
JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
NCP=$(find "$LIBRARIES" -name '*.jar' | tr '\n' ':')
mkdir -p classes results logs
$JAVAC -nowarn -cp "$KERNEL_JAR:$NCP" -d classes \
  ../p500/java/net/minecraft/network/protocol/game/PaperNativeChunkPacketEncode.java \
  ChunkEncodeSectionProbe.java ChunkEncodeSectionProbe2.java || exit 1
RAW=results/CHUNKENCODE_SECTION_PROBE_RAW.tsv
: > "$RAW"
exec 9>/home/z/BENCH.lock
flock 9
OUT="logs/chunkencode_section_probe.out"; : > "$OUT"
timeout 120 "$JAVA" -Xms256m -Xmx1g \
  -Dchunkencode.libs="$CHUNK_SO" \
  -cp "classes:$KERNEL_JAR:$NCP" ChunkEncodeSectionProbe ChunkEncodeSectionProbe2.java \
  > "$OUT" 2> "logs/chunkencode_section_probe.log"
rc=$?
flock -u 9
if [ $rc -ne 0 ]; then
  echo -e "PROBE_CRASH\texit=$rc" >> "$RAW"
  tail -15 "logs/chunkencode_section_probe.log" | sed 's/^/  log> /' >&2
else
  grep -oE "(PROBE|SINK)\t.*" "$OUT" >> "$RAW" || echo -e "NO-LINES\trc=0" >> "$RAW"
fi
echo "--- raw ---"
cat "$RAW"
exit $rc
