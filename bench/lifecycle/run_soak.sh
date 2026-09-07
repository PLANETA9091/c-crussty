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
#   SOAK_DURATION (600), SOAK_WAVE (50000), SOAK_LOCK_WAIT (2400) env.
# Outputs in results/: soak_raw.tsv, soak_stderr.log, soak_gc.log,
#                      soak_summary.txt   (console log is the caller's tee)
set -uo pipefail
cd "$(dirname "$0")"

DURATION=${1:-${SOAK_DURATION:-600}}
WAVE=${2:-${SOAK_WAVE:-50000}}
LOCK_WAIT=${SOAK_LOCK_WAIT:-2400}
OUT=results
mkdir -p "$OUT"

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
rm -rf classes_soak
mkdir -p classes_soak
"$JAVAC" -nowarn -d classes_soak \
  "$PKG/ImprovedNoise.java" \
  "$PKG/PaperNativeImprovedNoise.java" \
  "$REPO_ROOT/noise/$PKG/ImprovedNoiseNativeOps.java" \
  "$PKG/SoakBench.java" \
  || { echo "FAIL: javac" >&2; exit 2; }
echo "compile OK"

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

# --- summary (python3; no sed/bracket-traps) ---
python3 - "$OUT" "$RC" "$FATAL" "$HSERR" "$DURATION" "$WAVE" "$LOCK_WAITED" <<'PYEOF' | tee "$OUT/soak_summary.txt"
import sys, re, os

out, java_rc, fatal, hserr, duration, wave, lock_waited = sys.argv[1:8]
java_rc, fatal, hserr = int(java_rc), int(fatal), int(hserr)
duration, wave, lock_waited = int(duration), int(wave), int(lock_waited)

sums = {}
reclaim_lines = 0
with open(os.path.join(out, "soak_raw.tsv")) as f:
    for line in f:
        parts = line.rstrip("\n").split("\t")
        if not parts or not parts[0]:
            continue
        # sum lines:  "sum\tmetric\tvalue"   (3 fields)
        if parts[0] == "sum" and len(parts) >= 3:
            try:
                sums[parts[1]] = int(parts[2])
            except ValueError:
                sums[parts[1]] = parts[2]
        # wave lines: "wave\tidx\tphase\tkey\tvalue" (5 fields)
        elif parts[0] == "wave" and len(parts) >= 5 and parts[3] == "reclaim_ms":
            reclaim_lines += 1

def num(k, d=-1):
    v = sums.get(k)
    return d if v is None else v

built   = num("handles_built_total")
freed   = num("freed_total")
unfreed = num("native_unfreed")
live_max = num("live_handles_max")
live_end = num("live_handles_after_settle")
oome    = num("oome")
waves   = num("waves")
waves_p = num("waves_pressure")
waves_q = num("waves_quiet")
p_first, p_last = num("ns_build_pressure_first"), num("ns_build_pressure_last")
q_first, q_last = num("ns_build_quiet_first"), num("ns_build_quiet_last")
done_marker = 1 if any(l.startswith("soak\tdone") for l in open(os.path.join(out, "soak_raw.tsv"))) else 0

drift_p = (p_last / p_first) if (p_first and p_first > 0 and p_last > 0) else -1
drift_q = (q_last / q_first) if (q_first and q_first > 0 and q_last > 0) else -1

# ---- GC log summary ----
type_re = re.compile(r"\bPause (\w+)")
ms_re = re.compile(r"(\d+(?:\.\d+)?)ms\s*$")
gcount, gtotal, gmax, gfull = 0, 0.0, 0.0, 0
gclog_path = os.path.join(out, "soak_gc.log")
if os.path.exists(gclog_path):
    with open(gclog_path, errors="replace") as f:
        for line in f:
            if "Pause" not in line:
                continue
            mt, md = type_re.search(line), ms_re.search(line)
            if not mt or not md:
                continue
            ms = float(md.group(1))
            gcount += 1
            gtotal += ms
            gmax = max(gmax, ms)
            if mt.group(1) == "Full":
                gfull += 1

results = []
def check(name, ok, detail):
    results.append((name, "PASS" if ok else "FAIL", detail))

check("S0 jvm_completed",      done_marker == 1 and java_rc == 0,
      f"done_marker={done_marker} java_rc={java_rc} (0 = JVM alive at exit, no crash)")
check("A1 no_double_free",     fatal == 0 and hserr == 0 and unfreed == 0 and freed == built and built > 0,
      f"fatal_markers={fatal} hs_err={hserr} freed={freed} built={built} native_unfreed={unfreed}")
check("A2 handles_bounded",    0 <= live_max <= 2 * wave and live_end == 0,
      f"live_max={live_max} bound=2x{wave}={2*wave} live_after_settle={live_end}")
check("A3 no_oome",            oome == 0, f"oome={oome}")
check("A4a pressure_drift",    0 < drift_p <= 3.0, f"first={p_first} last={p_last} last/first={drift_p:.2f}x (threshold 3.0x)")
check("A4b quiet_drift",       0 < drift_q <= 3.0, f"first={q_first} last={q_last} last/first={drift_q:.2f}x (threshold 3.0x)")
check("A5 coverage",           waves >= 3 and waves_p >= 1 and waves_q >= 1,
      f"waves={waves} (pressure={waves_p} quiet={waves_q})")

overall = "PASS" if all(r[1] == "PASS" for r in results) else "FAIL"

def g(k, d="n/a"):
    v = sums.get(k)
    return str(v) if v is not None else d

print("=" * 72)
print("TASK-17 SOAK SUMMARY — phantom-reaper under GC pressure (small heap)")
print("=" * 72)
print(f"lock_waited_s                {lock_waited}")
print(f"wall_s                       {g('wall_s')} (configured {duration})")
print(f"waves                        {g('waves')} (pressure {g('waves_pressure')} / quiet {g('waves_quiet')})")
print(f"handles_built_total          {built}")
print(f"freed_total                  {freed}")
print(f"native_unfreed               {unfreed}")
print(f"live_handles_max             {live_max}  (bound {2*wave})")
print(f"live_handles_after_settle    {live_end}")
print(f"reclaim_pressure_ms p50/p95/max  {g('reclaim_pressure_p50_ms')} / {g('reclaim_pressure_p95_ms')} / {g('reclaim_pressure_max_ms')}  (n={g('reclaim_pressure_count')}, timeouts={g('reclaim_pressure_timeouts')})")
print(f"reclaim_quiet_ms    p50/p95/max  {g('reclaim_quiet_p50_ms')} / {g('reclaim_quiet_p95_ms')} / {g('reclaim_quiet_max_ms')}  (n={g('reclaim_quiet_count')})")
print(f"ns_per_build pressure first/last  {p_first} / {p_last}  (drift {drift_p:.2f}x)")
print(f"ns_per_build quiet    first/last  {q_first} / {q_last}  (drift {drift_q:.2f}x)")
print(f"gc_collections (MXBean)      {g('gc_collections')}   gc_time_ms {g('gc_time_ms')}")
print(f"gc.log pauses                count={gcount} total={gtotal:.0f}ms max={gmax:.1f}ms full={gfull}")
print(f"oome                         {oome}")
print("-" * 72)
for name, verdict, detail in results:
    print(f"{verdict}  {name:24s} {detail}")
print("-" * 72)
print(f"SOAK_VERDICT: {overall}")
print("reclaim_wave_lines_observed  %d" % reclaim_lines)
sys.exit(0 if overall == "PASS" else 1)
PYEOF
SUMMARY_RC=${PIPESTATUS[0]}
echo "summary_rc=$SUMMARY_RC"
exit $SUMMARY_RC
