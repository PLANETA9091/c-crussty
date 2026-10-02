#!/usr/bin/env bash
# TASK-105 JFR profile analyzer (agent-7625532f)
# Per-run: sample totals, top leaf frames, category shares (bridge / noise / other),
# GC window stats, allocation-sample weights. Differential armed-vs-dormant readout.
set -uo pipefail
JFR=/home/z/jdk21/bin/jfr
RAW=/home/z/c-crussty/bench/graal_ab/RAW_TASK105

for RUN in JFRAD1 JFRA2 JFRA3 JFRAD4; do
    D="$RAW/run_$RUN"; F="$D/burst.jfr"
    [ -s "$F" ] || { echo "!! $RUN missing jfr"; continue; }
    echo "================== $RUN =================="
    # --- ExecutionSample extraction: block per sample, top-6 frames
    "$JFR" print --events jdk.ExecutionSample --stack-depth 6 "$F" 2>/dev/null > "$D/exec.txt"
    TOTAL=$(rg -c '^jdk\.ExecutionSample \{' "$D/exec.txt" || echo 0)
    echo "ExecutionSamples: $TOTAL"
    # --- top-15 LEAF frames
    echo "--- top-15 leaf frames (depth1) ---"
    awk '
        /^    [a-z]/ && !inhdr { next }
        /^jdk\.ExecutionSample \{/ { depth=0; inhdr=0; next }
        inhdr && /stackTrace = \[/ { inhdr=0; depth=0; next }
        { if ($0 ~ /^  [a-zA-Z]/) inhdr=1 }
        /^    [^ ]/ { } # noop
        /line: |line:$/ { }
    ' /dev/null 2>/dev/null  # placeholder; real parsing below
    python3 - "$D/exec.txt" << 'PYEOF'
import sys, re, collections
leaf = collections.Counter()
deep = collections.Counter()
bridge = 0; noise_leaf = 0; total = 0
BRIDGE = re.compile(r'crussty|JNI|MethodHandle|java\.lang\.invoke|striped|handle|Unsafe|nativeMethod|NativeMethod', re.I)
NOISE  = re.compile(r'levelgen\.|Noise|noise|DensityFunction|NoiseChunk|Terrain|Aquifer|Carvers?|Surface', re.I)
blocks = open(sys.argv[1]).read().split('jdk.ExecutionSample {')[1:]
for b in blocks:
    frames = []
    for line in b.splitlines():
        m = re.match(r'^    ([a-zA-Z][^\s(]*\([^\n]*)', line)
        if m: frames.append(m.group(1))
        elif frames and line.startswith('  ') is False: break
    if not frames: continue
    total += 1
    leaf[frames[0].split('(')[0]] += 1
    for f in frames[:6]: deep[f.split('(')[0]] += 1
    if any(BRIDGE.search(f) for f in frames[:6]): bridge += 1
    if NOISE.search(frames[0]): noise_leaf += 1
if total:
    print(f"TOTAL={total}  bridge-exposure(top6)={bridge} ({100*bridge/total:.1f}%)  noise-leaf={noise_leaf} ({100*noise_leaf/total:.1f}%)")
    print("--- top-15 leaf ---")
    for k, v in leaf.most_common(15):
        print(f"{v:5d} ({100*v/total:5.1f}%)  {k[:110]}")
    print("--- top-12 any-of-top6 ---")
    for k, v in deep.most_common(12):
        print(f"{v:5d} ({100*v/total:5.1f}%)  {k[:110]}")
else:
    print("NO SAMPLES PARSED")
PYEOF
    # --- GC
    echo "--- GC in window ---"
    "$JFR" print --events jdk.GarbageCollection "$F" 2>/dev/null | rg -o 'duration = [0-9.]+' | awk '{s+=$3; n++} END{printf "count=%d sum=%.0fms\n", n, s*1000}'
    # --- allocation
    echo "--- allocation top-8 (sampled weight) ---"
    "$JFR" print --events jdk.ObjectAllocationSample "$F" 2>/dev/null > "$D/alloc.txt"
    rg -o 'objectClass = [A-Za-z.\[\]]+' "$D/alloc.txt" | awk '{print $3}' | sort | uniq -c | sort -rn | head -8
    rg -o 'weight = [0-9.]+' "$D/alloc.txt" | awk '{s+=$3; n++} END{printf "alloc-events=%d sampled-weight-sum=%.1fMB\n", n, s/1048576}'
done
