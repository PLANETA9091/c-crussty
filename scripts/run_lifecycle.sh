#!/usr/bin/env bash
# P500-style lifecycle A/B runner: old (finalize + global map) vs new
# (phantom reaper + stripes) over the REAL libpaper_native_jni.so kernels.
#
#   scripts/run_lifecycle.sh [javac-java-prefix]   (default: /home/z/jdk21/bin)
#
# Outputs: bench/lifecycle/results/{<impl>_<scenario>.tsv, LIFECYCLE_REPORT.md}
set -euo pipefail
cd "$(dirname "$0")/.."

JAVA_BIN="${1:-/home/z/jdk21/bin}"
NATIVE="$(pwd)/native/libpaper_native_jni.so"
CLS=bench/lifecycle/classes
RES=bench/lifecycle/results
SC_DIR=bench/lifecycle/net/minecraft/world/level/levelgen/synth

mkdir -p "$RES"
rm -f "$RES"/*.tsv "$RES"/LIFECYCLE_REPORT.md

# ---- build (shipped bridge classes + bench sources) ----
if [ ! -f noise/build/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps.class ]; then
  scripts/build_noise.sh
fi
rm -rf "$CLS"; mkdir -p "$CLS"
"$JAVA_BIN/javac" -nowarn -cp noise/build -d "$CLS" "$SC_DIR"/*.java

# ---- run: one JVM per (impl, scenario) — P500 isolation methodology ----
for impl in old new; do
  for sc in hotpath churn gcchurn; do
    echo "=== $impl $sc ==="
    "$JAVA_BIN/java" -Xms512m -Xmx512m \
      -Dcrussty.native="$NATIVE" \
      -cp "$CLS:noise/build" \
      net.minecraft.world.level.levelgen.synth.LifecycleBench \
      "$impl" "$sc" 2>"$RES/${impl}_${sc}.err" \
      | tee "$RES/${impl}_${sc}.tsv"
  done
done

# ---- aggregate ----
python3 - "$RES" <<'EOF'
import glob, os, sys
res = sys.argv[1]
rows = {}
for f in sorted(glob.glob(os.path.join(res, '*.tsv'))):
    base = os.path.basename(f)[:-4]
    impl, sc = base.rsplit('_', 1)
    for line in open(f):
        parts = line.rstrip('\n').split('\t')
        if len(parts) == 4 and parts[0] == impl:
            rows[(impl, sc, parts[2])] = parts[3]

def g(sc, metric, impl):
    return rows.get((impl, sc, metric), 'n/a')

def pct(new, old):
    try:
        n, o = float(new), float(old)
        return f"{n/o:.2f}x" if o else 'n/a'
    except ValueError:
        return 'n/a'

L = []
L.append('# LIFECYCLE REPORT — ImprovedNoiseNativeOps old vs new (P500-style)')
L.append('')
L.append('- OLD: Handle.finalize() + ONE global synchronized WeakHashMap (pre-TASK-01 shipped code)')
L.append('- NEW: PhantomReference reaper thread + 16 identity-striped WeakHashMaps (TASK-01/09)')
L.append('- Same native kernels (nativeBuildHandle/nativeNoise/nativeFreeHandle, libpaper_native_jni.so)')
L.append('- -Xms512m -Xmx512m, one JVM per (impl, scenario), best-of-5 batches (hotpath)')
L.append('')
for sc, title in [('hotpath', 'M1 hot path: ns per noise() under contention'),
                  ('churn',   'M2/M4 churn: 20k handles built then dropped'),
                  ('gcchurn', 'M3 churn + parallel junk allocator (GC pressure)')]:
    L.append(f'## {title} — `{sc}`')
    L.append('')
    L.append('| metric | old | new | new/old |')
    L.append('|--------|-----|-----|---------|')
    metrics = [m for (i, s, m) in rows if s == sc and i == 'old']
    seen = set()
    for m in metrics:
        if m in seen: continue
        seen.add(m)
        o, n = g(sc, m, 'old'), g(sc, m, 'new')
        L.append(f'| {m} | {o} | {n} | {pct(n, o)} |')
    L.append('')

out = os.path.join(res, 'LIFECYCLE_REPORT.md')
open(out, 'w').write('\n'.join(L))
print(f'report -> {out}')
EOF
echo "lifecycle bench done"