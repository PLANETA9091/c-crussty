#!/usr/bin/env bash
# P500 bench runner — compiles the generated bench and executes one JVM per
# group so a misbehaving closed-source kernel cannot poison the whole run.
# Usage: ./run_p500.sh [gid ...]   (default: all groups from groups.tsv)
set -u
cd "$(dirname "$0")"

REPO_ROOT="$(cd ../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
# legacy name kept for older checkouts; current binary is libpaper_native_chunk_encode_jni.so (see native/)
CHUNK_SO="$REPO_ROOT/native/libpaper_chunk_encode_jni.so"
CHUNK_SO2="$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so"

# locate a JDK with javac (portable JDK preferred, then JAVA_HOME, then PATH)
if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
elif [ -x "$JAVA_HOME/bin/javac" ]; then JAVAC="$JAVA_HOME/bin/javac"; JAVA="$JAVA_HOME/bin/java"
else JAVAC=javac; JAVA=java; fi
echo "using: $JAVAC"

LIBS="$MAIN_SO"
[ -f "$CHUNK_SO" ] && LIBS="$LIBS:$CHUNK_SO"
[ -f "$CHUNK_SO2" ] && LIBS="$LIBS:$CHUNK_SO2"

mkdir -p classes results logs
$JAVAC -d classes $(find java -name '*.java') || exit 1

RAW=results/p500_raw.tsv
# P500_APPEND=1: keep prior rows (chunked foreground runs); default: fresh run
[ "${P500_APPEND:-0}" = "1" ] || : > "$RAW"

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

python3 aggregate_p500.py "$RAW" | tee results/P500_REPORT.md | tail -40
echo "raw TSV: $RAW"
