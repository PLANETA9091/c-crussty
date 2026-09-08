#!/usr/bin/env bash
# TASK-95 (queued) — Graal JIT A/B: C2 (deployment JDK21) vs GraalVM CE 21 (UseJVMCICompiler)
# Research origin: OPT_ARCHITECTURE_RESEARCH_2026-09-08.md §7 (R3#2: +23% geomean vs C2 on
# 1,112 JMH runs, strongest on megamorphic dispatch — exactly this stack's census profile).
# Protocol: paired-boot A/B, BENCH-MUTEX, world anchor before EVERY boot,
# n>=5/arm, strict alternation to de-trend boot drift, honest NULL reporting.
# NOTE: dormant module arm only — measures JVM-level JIT difference on the SAME workload;
# no engine env gates are set, the module stays dormant by design.
set -euo pipefail
cd "$(dirname "$0")/../.."   # repo root
JDK_C2="${JDK21:-/home/z/jdk21}"
JDK_GRAAL="${GRAALVM21:-/home/z/graalvm/graalvm-community-openjdk-21.0.2+13.1}"
SERVER=/home/z/server
JAR="$SERVER/versions/purpur-1.21.10.jar"   # top-level paperclip jar (live-boot lesson d03562c)
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="$PWD/bench/graal_ab/RAW_GRAAL_$STAMP"
mkdir -p "$OUT"

# --- guards (census-script canon) ---
if pgrep -f 'purpur-1.21.10\.jar|launcher\.jar' >/dev/null 2>&1; then
    echo "LANE-BUSY: another server JVM is running — aborting (BENCH-MUTEX canon)"; exit 42
fi
exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY: BENCH.lock held"; exit 42; }
echo "start-graal-ab-$STAMP" >> /home/z/BENCH.lock.journal
cleanup() { echo "done-graal-ab-$STAMP" >> /home/z/BENCH.lock.journal; }
trap cleanup EXIT

log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/run.log"; }
N="${N:-5}"   # boots per arm

[ -x "$JDK_GRAAL/bin/java" ] || { echo "FATAL: GraalVM not found at $JDK_GRAAL"; exit 1; }
"$JDK_GRAAL/bin/java" -version 2>&1 | head -1 | tee -a "$OUT/run.log"
"$JDK_C2/bin/java" -version 2>&1 | head -1 | tee -a "$OUT/run.log"

boot_one() {  # $1=jdk path $2=extra jvm flags $3=run id
    local JDKP="$1" FLAGS="$2" RID="$3"
    tar -C "$SERVER" -czf "$OUT/anchor_$RID.tgz" $(cd "$SERVER" && ls -d world world_nether world_the_end)
    ( cd "$SERVER" && exec timeout 300 "$JDKP/bin/java" $FLAGS \
        -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 \
        -jar "$JAR" --nogui >"$OUT/boot_$RID.log" 2>&1 ) &
    local SP=$!; disown "$SP" 2>/dev/null || true
    local D=0
    for i in $(seq 1 180); do grep -q 'Done (' "$OUT/boot_$RID.log" 2>/dev/null && { D=1; break; }; sleep 1; done
    if [ "$D" != 1 ]; then log "FATAL: run $RID no Done( in 180s"; tail -5 "$OUT/boot_$RID.log" | tee -a "$OUT/run.log"; kill "$SP" 2>/dev/null || true; return 1; fi
    local T; T=$(grep -o 'Done ([0-9.]*s)' "$OUT/boot_$RID.log" | tail -1)
    log "run $RID: $T"
    sleep 5                       # short post-Done tail (boot-time metric is the target; Done-time captured above)
    kill -TERM "$SP" 2>/dev/null || true
    for i in $(seq 1 45); do kill -0 "$SP" 2>/dev/null || break; sleep 1; done
    kill -9 "$SP" 2>/dev/null || true
    wait "$SP" 2>/dev/null || true
    tar -C "$SERVER" -xzf "$OUT/anchor_$RID.tgz"   # world restore before EVERY boot
    rm -f "$OUT/anchor_$RID.tgz"
}

# --- strictly alternating boots C2 (A) / Graal (B), n per arm ---
SEQ=""; for k in $(seq 1 "$N"); do SEQ="$SEQ A B"; done
log "graal A/B begin: $N per arm, order:$SEQ"
i=0
for arm in $SEQ; do
    i=$((i+1))
    if [ "$arm" = "A" ]; then boot_one "$JDK_C2" "" "$(printf '%02d' $i)_c2" || true
    else boot_one "$JDK_GRAAL" "-XX:+UnlockExperimentalVMOptions -XX:+EnableJVMCI -XX:+UseJVMCICompiler" "$(printf '%02d' $i)_graal" || true; fi
done

# --- aggregate ---
python3 - "$OUT" <<'PYEOF'
import sys, re, glob, os
out = sys.argv[1]
res = {"c2": [], "graal": []}
for f in sorted(glob.glob(os.path.join(out, "boot_*.log"))):
    arm = "graal" if "_graal" in f else "c2"
    txt = open(f, errors="ignore").read()
    m = re.findall(r"Done \(([0-9.]+)s\)", txt)
    if m: res[arm].append(float(m[-1]))
print("arm\tn\tmedian_s\tmin\tmax")
for arm in ("c2", "graal"):
    v = sorted(res[arm]); n = len(v)
    med = v[n//2] if n else float("nan")
    print(f"{arm}\t{n}\t{med:.3f}\t{min(v) if v else float('nan'):.3f}\t{max(v) if v else float('nan'):.3f}")
if res["c2"] and res["graal"]:
    a = sorted(res["c2"]); b = sorted(res["graal"])
    ma = a[len(a)//2]; mb = b[len(b)//2]
    print(f"delta_med_pct\t{(mb-ma)/ma*100:+.1f}")
PYEOF
log "graal A/B complete"
