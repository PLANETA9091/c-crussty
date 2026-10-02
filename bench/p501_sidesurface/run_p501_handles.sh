#!/usr/bin/env bash
# P501 HANDLE-CHAIN probe runner — TASK-157 (§ TASK-156 parked observation).
# One JVM per FAMILY (perlin, rtree) with an in-JVM build→consume→free chain.
# Sidecar-own artifact; canonical bench/p500/ + generated trees untouched.
# Family JVM isolation: an abort in one family cannot poison the other.
set -u
cd "$(dirname "$0")"

REPO_ROOT="$(cd ../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
CHUNK_SO="$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so"

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVAROOT=/home/z/jdk21
elif [ -x "${JAVA_HOME:-}/bin/javac" ]; then JAVAC="$JAVA_HOME/bin/javac"; JAVAROOT="$JAVA_HOME"
elif [ -x /home/z/bin/javac ]; then JAVAC=/home/z/bin/javac; JAVAROOT="${JAVA_HOME:-/usr/lib/jvm/java-21-openjdk-amd64}"
else JAVAC=javac; JAVAROOT="${JAVA_HOME:-}"; fi
JAVA="$JAVAROOT/bin/java"; [ -x "$JAVA" ] || JAVA=java
echo "using: $JAVAC / $JAVA"

LIBS="$MAIN_SO"
[ -f "$CHUNK_SO" ] && LIBS="$LIBS:$CHUNK_SO"

mkdir -p classes results logs
$JAVAC -d classes $(find java -name '*.java') HandleChainProbe.java || exit 1

# artifact date follows the cron timeline (+08)
DATE="$(TZ=Asia/Shanghai date +%F)"
RAW="results/P501_HANDLECHAIN_RAW_${DATE}.tsv"
[ "${P501_APPEND:-0}" = "1" ] || : > "$RAW"

if [ ! -f results/P501_HANDLECHAIN_ENV.txt ]; then
  {
    echo "date: $(date -Is)"
    echo "jvm: $("$JAVA" -version 2>&1 | head -1)"
    echo "javac: $JAVAC"
    echo "host: $(uname -srm)"
    echo "libs: $LIBS"
  } > results/P501_HANDLECHAIN_ENV.txt
fi

for FAMILY in perlin rtree; do
  echo "=== family $FAMILY ==="
  OUT="logs/handlechain_$FAMILY.out"; : > "$OUT"
  timeout 600 "$JAVA" -Xms1g -Xmx1g -XX:+AlwaysPreTouch -Xbatch -XX:+UseG1GC \
    -Dp501.libs="$LIBS" -cp classes HandleChainProbe "$FAMILY" > "$OUT" 2> "logs/handlechain_$FAMILY.log"
  rc=$?
  if [ $rc -eq 0 ]; then
    cat "$OUT" >> "$RAW"
  else
    echo -e "CRASH\t$FAMILY\texit=$rc" >> "$RAW"
    tail -5 "logs/handlechain_$FAMILY.log" | sed 's/^/  log> /'
  fi
done

python3 aggregate_handlechain.py "$RAW" | tee "results/P501_HANDLECHAIN_REPORT_${DATE}.md" | tail -30
echo "raw TSV: $RAW"
