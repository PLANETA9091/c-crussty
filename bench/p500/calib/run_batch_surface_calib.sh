#!/usr/bin/env bash
# TASK-51 — batch-surface calibration runner (PROVEN_WINS_SYNC §4.1 closure).
# One JVM per group (crash containment), canonical P500 flags, exclusive BENCH.lock.
# Usage: ./run_batch_surface_calib.sh [group ...]   (default: all four)
set -u
cd "$(dirname "$0")/.."   # bench/p500

REPO_ROOT="$(cd ../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
CHUNK_SO="$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so"

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
elif [ -x "$JAVA_HOME/bin/javac" ]; then JAVAC="$JAVA_HOME/bin/javac"; JAVA="$JAVA_HOME/bin/java"
else JAVAC=javac; JAVA=java; fi
echo "using: $JAVAC" >&2

LIBS="$MAIN_SO"
[ -f "$CHUNK_SO" ] && LIBS="$LIBS:$CHUNK_SO"

CALIB_DIR=calib
mkdir -p "$CALIB_DIR/classes" results logs

$JAVAC -d "$CALIB_DIR/classes" $(find java -name '*.java') || exit 1

RAW=results/BATCH_SURFACE_CALIBRATION_RAW.tsv
[ "${CALIB_APPEND:-0}" = "1" ] || : > "$RAW"

GIDS="$*"
if [ -z "$GIDS" ]; then GIDS="ticketset fractions rtree rtree-full"; fi

for g in $GIDS; do
  echo "=== calib group $g ===" >&2
  OUT="logs/calib_$g.out"; : > "$OUT"
  timeout 300 "$JAVA" -Xms1g -Xmx1g -XX:+AlwaysPreTouch -Xbatch -XX:+UseG1GC \
    -Dp500.libs="$LIBS" -cp "$CALIB_DIR/classes" BatchSurfaceCalib "$g" \
    > "$OUT" 2> "logs/calib_$g.log"
  rc=$?
  if [ $rc -ne 0 ]; then
    echo -e "CALIB_CRASH\t$g\texit=$rc" >> "$RAW"
    tail -5 "logs/calib_$g.log" | sed 's/^/  log> /' >&2
  else
    grep -E "^(CALIB|SINK)" "$OUT" >> "$RAW"
  fi
done

echo "--- raw ---"
cat "$RAW"
