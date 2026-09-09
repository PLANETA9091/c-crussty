#!/usr/bin/env bash
# P501 SIDESURFACE bench runner — TASK-156 per TASK-155 §103 pre-registration.
# Measures the 94 zero-evidence exports (37 classes, 49 groups) in a
# SELF-CONTAINED sidecar tree. Canonical bench/p500/ is NEVER touched:
# no groups.tsv/G*.java regeneration, no writes outside this directory.
# Rig-pattern: one JVM per group, time-bounded batches (~120 ms), median,
# DCE-proof sink, 600 s timeout, crash-retry ladder (fresh JVM, N=16, then N=1).
#
# Usage: ./run_p501.sh [gid ...]   (default: all groups from java/p500/groups.tsv)
# Env:   P501_APPEND=1  keep prior RAW rows (chunked foreground runs)
#        P501_N=<int>   override default arg-array size (default 256)
set -u
cd "$(dirname "$0")"

REPO_ROOT="$(cd ../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
CHUNK_SO="$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so"

# locate a JDK with javac — portable jdk21, then JAVA_HOME, then the post-wipe
# ToolProvider shim at /home/z/bin/javac (sandbox wipe #2 removed the portable
# JDK; system OpenJDK 21 has no javac binary — TASK-151 shim discipline), then PATH.
if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVAROOT=/home/z/jdk21
elif [ -x "${JAVA_HOME:-}/bin/javac" ]; then JAVAC="$JAVA_HOME/bin/javac"; JAVAROOT="$JAVA_HOME"
elif [ -x /home/z/bin/javac ]; then JAVAC=/home/z/bin/javac; JAVAROOT="${JAVA_HOME:-/usr/lib/jvm/java-21-openjdk-amd64}"
else JAVAC=javac; JAVAROOT="${JAVA_HOME:-}"; fi
JAVA="$JAVAROOT/bin/java"; [ -x "$JAVA" ] || JAVA=java
echo "using: $JAVAC / $JAVA"

LIBS="$MAIN_SO"
[ -f "$CHUNK_SO" ] && LIBS="$LIBS:$CHUNK_SO"

mkdir -p classes results logs
$JAVAC -d classes $(find java -name '*.java') || exit 1

# artifact date follows the cron timeline (+08), matching TASK-154/155 doc names
DATE="$(TZ=Asia/Shanghai date +%F)"
RAW="results/P501_SIDESURFACE_RAW_${DATE}.tsv"
REPORT="results/P501_SIDESURFACE_REPORT_${DATE}.md"
# keep the first report of the day; later invocations bank a numbered suffix
[ -f "$REPORT" ] && REPORT="results/P501_SIDESURFACE_REPORT_2_${DATE}.md"
[ "${P501_APPEND:-0}" = "1" ] || : > "$RAW"

# env provenance block (embedded by the aggregator, first writer wins)
if [ ! -f results/P501_ENV.txt ]; then
  {
    echo "date: $(date -Is)"
    echo "jvm: $("$JAVA" -version 2>&1 | head -1)"
    echo "javac: $JAVAC"
    echo "host: $(uname -srm)"
    echo "N: ${P501_N:-256}"
    echo "libs: $LIBS"
  } > results/P501_ENV.txt
fi

GIDS="$*"
if [ -z "$GIDS" ]; then
  GIDS=$(seq 0 $(( $(wc -l < java/p500/groups.tsv) - 1 )))
fi

for gid in $GIDS; do
  echo "=== group $gid ==="
  OUT="logs/g$gid.out"; : > "$OUT"
  timeout 600 "$JAVA" -Xms1g -Xmx1g -XX:+AlwaysPreTouch -Xbatch -XX:+UseG1GC \
    -Dp500.groups=java/p500/groups.tsv -Dp500.libs="$LIBS" \
    -cp classes p500.Bench "$gid" > "$OUT" 2> "logs/g$gid.log"
  rc=$?
  if [ $rc -ne 0 ]; then
    # Some closed-source kernels abort() (Rust panic => SIGABRT) on shapes
    # they did not expect. Retry the whole group in a fresh JVM with smaller
    # synthetic batches: N=16, then N=1. First attempt that survives wins.
    for NTRY in 16 1; do
      echo "retry group $gid with -Dp500.n=$NTRY (previous exit $rc)"
      : > "$OUT"
      timeout 600 "$JAVA" -Xms1g -Xmx1g -XX:+AlwaysPreTouch -Xbatch -XX:+UseG1GC \
        -Dp500.n=$NTRY -Dp500.groups=java/p500/groups.tsv -Dp500.libs="$LIBS" \
        -cp classes p500.Bench "$gid" > "$OUT" 2> "logs/g$gid.log"
      rc=$?
      [ $rc -eq 0 ] && break
    done
  fi
  if [ $rc -eq 0 ]; then
    cat "$OUT" >> "$RAW"
  else
    echo -e "CRASH\t$gid\texit=$rc" >> "$RAW"
    tail -5 "logs/g$gid.log" | sed 's/^/  log> /'
  fi
done

python3 aggregate_p501.py "$RAW" | tee "$REPORT" | tail -40
echo "raw TSV: $RAW"
