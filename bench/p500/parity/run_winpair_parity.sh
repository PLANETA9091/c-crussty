#!/usr/bin/env bash
# TASK-54 — WIN-pair parity gate runner, wave 2 (promotion lifecycle step 1
# for the remaining wire-eligible P500 WIN pairs). Same contract as
# run_flatcache_parity.sh: REAL closed .so, exclusive BENCH.lock, FIXTURE2
# emission for the plugin live self-test.
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
mkdir -p "$PARITY_DIR/classes" "$PARITY_DIR/results" "$PARITY_DIR/logs"

$JAVAC -d "$PARITY_DIR/classes" \
       $(find java -name 'PaperNativeNoiseInterpolatorSlice.java' \
              -o -name 'PaperNativeImprovedNoiseInline.java' \
              -o -name 'PaperNativePalettedReencodeScratch.java') \
       "$PARITY_DIR/WinPairParity.java" || exit 1

RAW="$PARITY_DIR/results/WINPAIR_PARITY_RAW.tsv"
: > "$RAW"

exec 9>/home/z/BENCH.lock
flock 9
echo "BENCH.lock held $(date -u +%Y-%m-%dT%H:%M:%SZ)" >&2

OUT="$PARITY_DIR/logs/winpair_parity.out"; : > "$OUT"
timeout 480 "$JAVA" -Xms1g -Xmx1g -XX:+AlwaysPreTouch -Xbatch -XX:+UseG1GC \
  -Dp500.libs="$LIBS" -cp "$PARITY_DIR/classes" WinPairParity \
  > "$OUT" 2> "$PARITY_DIR/logs/winpair_parity.log"
rc=$?
flock -u 9

if [ $rc -ne 0 ]; then
  echo -e "PARITY_CRASH\texit=$rc" >> "$RAW"
  tail -10 "$PARITY_DIR/logs/winpair_parity.log" | sed 's/^/  log> /' >&2
else
  grep -E "^(PARITY|FIXTURE2|SINK)" "$OUT" >> "$RAW"
fi

echo "--- raw ---"
cat "$RAW"
