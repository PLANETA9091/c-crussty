#!/usr/bin/env python3
"""TASK-17 soak summary: parse results/soak_raw.tsv + results/soak_gc.log,
print the metric table + per-assertion verdicts, exit 0 iff all PASS.

Usage: soak_summary.py OUT_DIR JAVA_RC FATAL HSERR DURATION WAVE LOCK_WAITED
(standalone so run_soak.sh --summarize can regenerate the verdict without
re-running the soak.)
"""
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
