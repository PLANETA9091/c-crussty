#!/usr/bin/env bash
# bench/lifecycle/run_soak.sh — TASK-17: 10-minute lifecycle soak of the
# phantom-reaper bridge (ImprovedNoiseNativeOps, TASK-01/09) under GC
# pressure on a small heap.
#
#   - driver:  SoakBench (this dir) — waves of 50k noise-handle objects
#     built+dropped for 10 min; EVEN waves = GC-pressure windows (parallel
#     junk allocator + explicit System.gc() every ~500ms), ODD waves =
#     quiet windows (no explicit gc) to exercise the quiet-reclaim path;
#   - JVM:     -Xms256m -Xmx256m -XX:+UseG1GC (small heap => GC pressure),
#              -Xlog:gc -> results/soak_gc.log (pause stats in the report);
#   - asserts: 0 double-free (no JNI fatal error, freed == built exactly —
#              the bridge's AtomicBoolean CAS release() keeps free
#              at-most-once), handle count bounded, no OOME, throughput
#              drift first-vs-last wave;
#   - lock:    poll-acquires /tmp/crussty_bench.lock (short benches by
#              other agents have priority — they are quick, we are 10+ min).
#              If the lock stays busy > SOAK_LOCK_WAIT seconds the soak runs
#              ANYWAY and the report carries an interference caveat.
#
# Usage: ./run_soak.sh [duration_sec] [wave_size]
#   ./run_soak.sh --summarize   (re-run verdict on existing results only)
#   SOAK_DURATION (600), SOAK_WAVE (50000), SOAK_LOCK_WAIT (2400),
#   SOAK_SKIP_COMPILE=1 (classes_soak/ prebuilt by a previous call) env.
# Outputs in results/: soak_raw.tsv, soak_stderr.log, soak_gc.log,
#                      soak_rc.txt, soak_summary.txt
set -uo pipefail
cd "$(dirname "$0")"

OUT=results
mkdir -p "$OUT"

if [ "${1:-}" = "--summarize" ]; then
  RC_FH=$(cat "$OUT/soak_rc.txt" 2>/dev/null || echo "1 1 0")
  set -- $RC_FH
  exec python3 soak_summary.py "$OUT" "$1" "$2" "$3" \
      "${SOAK_DURATION:-600}" "${SOAK_WAVE:-50000}" "0"
fi

DURATION=${1:-${SOAK_DURATION:-600}}
WAVE=${2:-${SOAK_WAVE:-50000}}
LOCK_WAIT=${SOAK_LOCK_WAIT:-2400}

echo "== TASK-17 lifecycle soak @ $(date -u '+%FT%TZ') =="
echo "config: duration=${DURATION}s wave=${WAVE} lock_wait=${LOCK_WAIT}s"
echo "context_loadavg=$(cat /proc/loadavg 2>/dev/null || echo 'n/a')"
if pgrep -f 'purpur-1.21.10.jar' >/dev/null 2>&1; then
  echo "context_purpur_server=RUNNING (live server shares the 2 vCPUs — interference expected)"
else
  echo "context_purpur_server=absent"
fi

# --- toolchain (same pattern as bench/noise_ab/run_noise_ab.sh) ---
if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
elif [ -x "${JAVA_HOME:-}/bin/javac" ]; then JAVAC="$JAVA_HOME/bin/javac"; JAVA="$JAVA_HOME/bin/java"
else JAVAC=javac; JAVA=java; fi
echo "using: $JAVAC"

REPO_ROOT="$(cd ../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
[ -f "$MAIN_SO" ] || { echo "FAIL: missing $MAIN_SO" >&2; exit 2; }

# --- compile BEFORE the lock: keep lock-held time = pure bench time ---
PKG=net/minecraft/world/level/levelgen/synth
if [ "${SOAK_SKIP_COMPILE:-0}" != "1" ]; then
  rm -rf classes_soak
  mkdir -p classes_soak
  "$JAVAC" -nowarn -d classes_soak \
    "$PKG/ImprovedNoise.java" \
    "$PKG/PaperNativeImprovedNoise.java" \
    "$REPO_ROOT/noise/$PKG/ImprovedNoiseNativeOps.java" \
    "$PKG/SoakBench.java" \
    || { echo "FAIL: javac" >&2; exit 2; }
  echo "compile OK"
fi

# --- bench exclusivity: poll-acquire (short benches of others go first) ---
exec 9>/tmp/crussty_bench.lock
t0=$(date +%s); got=0
while :; do
  if flock -n 9; then got=1; break; fi
  if [ $(( $(date +%s) - t0 )) -ge "$LOCK_WAIT" ]; then break; fi
  sleep 10
done
if [ "$got" = 1 ]; then
  echo "bench lock acquired (waited $(( $(date +%s) - t0 ))s)"
else
  echo "WARN: /tmp/crussty_bench.lock BUSY > ${LOCK_WAIT}s — running the soak ANYWAY; INTERFERENCE CAVEAT applies to all numbers"
fi
LOCK_WAITED=$(( $(date +%s) - t0 ))

# --- the soak ---
GCLOG="$OUT/soak_gc.log"
rm -f "$GCLOG"
echo "soak start @ $(date -u '+%FT%TZ')"
"$JAVA" -Xms256m -Xmx256m -XX:+UseG1GC \
  -Xlog:gc:file="$GCLOG":time,uptime:filecount=0 \
  -Dcrussty.native="$MAIN_SO" -cp classes_soak \
  net.minecraft.world.level.levelgen.synth.SoakBench "$DURATION" "$WAVE" \
  > "$OUT/soak_raw.tsv" 2> "$OUT/soak_stderr.log"
RC=$?
echo "soak end @ $(date -u '+%FT%TZ')  java_rc=$RC"

# --- fatal-error / double-free proxies (JVM-level) ---
FATAL=0
if grep -q -e 'A fatal error has been detected' -e 'SIGSEGV' -e 'EXCEPTION_ACCESS_VIOLATION' \
       -e 'Internal Error' -e 'double free' -e 'corrupted' "$OUT/soak_stderr.log" 2>/dev/null; then
  FATAL=1
fi
HSERR=$(ls hs_err_pid*.log 2>/dev/null | wc -l)
echo "fatal_error_markers=$FATAL hs_err_files=$HSERR"
echo "$RC $FATAL $HSERR" > "$OUT/soak_rc.txt"

# --- summary (python3; no sed/bracket-traps) ---
python3 soak_summary.py "$OUT" "$RC" "$FATAL" "$HSERR" "$DURATION" "$WAVE" "$LOCK_WAITED" \
  | tee "$OUT/soak_summary.txt"
SUMMARY_RC=${PIPESTATUS[0]}
echo "summary_rc=$SUMMARY_RC"
exit $SUMMARY_RC
