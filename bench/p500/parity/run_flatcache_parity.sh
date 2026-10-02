#!/usr/bin/env bash
# TASK-53 — FlatCacheContext semantic parity gate runner (§4.2 promotion step 1).
# Loads the REAL closed .so, byte-compares old vs new pair outputs on
# randomized + edge inputs, emits FIXTURE vectors for the plugin live self-test.
# Exclusive BENCH.lock (timing section shares the machine with nothing else).
set -u
cd "$(dirname "$0")/.."   # bench/p500

REPO_ROOT="$(cd ../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
CHUNK_SO="$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so"

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
else JAVAC=javac; JAVA=java; fi
echo "using: $JAVAC" >&2

LIBS="$MAIN_SO"
[ -f "$CHUNK_SO" ] && LIBS="$LIBS:$CHUNK_SO"

PARITY_DIR=parity
mkdir -p "$PARITY_DIR/classes" results logs

$JAVAC -d "$PARITY_DIR/classes" $(find java -name 'PaperNativeNoiseChunkFlatCacheContext.java') \
       "$PARITY_DIR/FlatCacheParity.java" || exit 1

RAW=results/FLATCACHE_PARITY_RAW.tsv
: > "$RAW"

exec 9>/home/z/BENCH.lock
flock 9
echo "BENCH.lock held $(date -u +%Y-%m-%dT%H:%M:%SZ)" >&2

OUT="logs/flatcache_parity.out"; : > "$OUT"
timeout 300 "$JAVA" -Xms1g -Xmx1g -XX:+AlwaysPreTouch -Xbatch -XX:+UseG1GC \
  -Dp500.libs="$LIBS" -cp "$PARITY_DIR/classes" FlatCacheParity \
  > "$OUT" 2> "logs/flatcache_parity.log"
rc=$?
flock -u 9

if [ $rc -ne 0 ]; then
  echo -e "PARITY_CRASH\texit=$rc" >> "$RAW"
  tail -10 "logs/flatcache_parity.log" | sed 's/^/  log> /' >&2
else
  grep -E "^(PARITY|FIXTURE|TIMING|SINK)" "$OUT" >> "$RAW"
fi

echo "--- raw ---"
cat "$RAW"
