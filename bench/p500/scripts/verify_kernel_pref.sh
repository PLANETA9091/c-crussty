#!/usr/bin/env bash
# TASK-13: kernel-policy gate full-coverage verification (CRUSSTY_KERNEL_PREF=old).
#
# For every remap candidate — the four DO_NOT_WIRE kernels in
# src/kernel_policy.rs (g19/g20/g27/g34, confirmed scale-invariant in
# bench/p500/results/P500_SCALING.md and P500_REPORT_v2.md §"Regressions
# (do-not-wire)") — this script runs the P500 harness (p500.Bench) four ways:
#
#   layer=agent mode=default   CRUSSTY runtime agent attached, env unset
#   layer=agent mode=pref-old  agent + CRUSSTY_KERNEL_PREF=old
#   layer=plain mode=default   bare JVM: bridge stubs bind via dlsym (no agent)
#   layer=plain mode=pref-old  bare JVM + CRUSSTY_KERNEL_PREF=old
#
# WHY TWO LAYERS: the remap is a REGISTRATION-time binding
# (kernel_policy::registration_fallback, called from lib.rs::define_and_register
# when the CRUSSTY runtime injects the 283-native surface: bootstrap-defined
# bridge classes + RegisterNatives). A bare `java p500.Bench` never runs that
# path — the env var alone cannot (and must not) change a dlsym-bound bench.
# The plain layer documents exactly that (expected: zero delta, zero remap
# lines = gate fail-safe/inert outside the runtime); the agent layer is the
# actual gate surface (expected: `kernel_pref:` remap lines on stderr and the
# alt kernel collapsing to its paired old kernel's ns/op).
#
# ORDER: ALL agent-layer rows run first (the gate evidence this task exists
# for), then the plain-layer rows (cheap inertness documentation). If the
# wall-clock budget dies mid-run, the least informative rows are what gets
# lost.
#
# MECHANICS: scripts/KernelPrefDriver.java (compiled alongside) makes the
# agent-attached runs wait until the injected BOOTSTRAP copies of all four
# bridge classes exist — via a NON-poisoning bootstrap-only probe
# (Class.forName(n,false,null)); a naive Class.forName would permanently bind
# the app-loader classpath stub for any not-yet-injected probe (round-1
# lesson) — and prints a classloader report as evidence.
#
# MODULE .so REQUIREMENT: the plugin build that carries the TASK-04 gate
# (target/release/libcrussty.so). The DEPLOYED server module .so
# (/home/z/server/modules/crussty/libcrussty.so) PREDATES TASK-04 (no
# kernel_pref strings) and is deliberately NOT usable as a fallback.
#
# USAGE (BENCH.lock holder only — see /home/z/BENCH.lock protocol):
#   KPREF_MODULE_SO=/path/to/target/release/libcrussty.so ./scripts/verify_kernel_pref.sh
# Env overrides:
#   KPREF_N           p500.n arg size            (default 16 — short runs;
#                     batches are time-bounded 120ms in Bench.java, so N does
#                     not drive wall time, it drives the per-call arg shape)
#   KPREF_MODULE_SO   module .so carrying the TASK-04 gate   (REQUIRED unless
#                     target/release/libcrussty.so exists relative to repo root)
#   KPREF_RUNTIME_SO  runtime agent .so          (default /home/z/server/libcrussty_runtime.so)
#   KPREF_SERVER      deployed server dir for runtime + module scaffolding
#                                                (default /home/z/server; READ-ONLY)
#   KPREF_STAGE       private staging dir        (default /tmp/crussty-kpref-modstage)
#   KPREF_TIMEOUT     per-JVM timeout seconds    (default 100)
# Output: results/kernel_pref_coverage.tsv + logs/kpref_<layer>_<mode>_g<gid>.{out,err}
set -u
cd "$(dirname "$0")/.."                     # bench/p500

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
elif [ -x "${JAVA_HOME:-}/bin/javac" ]; then JAVAC="$JAVA_HOME/bin/javac"; JAVA="$JAVA_HOME/bin/java"
else JAVAC=javac; JAVA=java; fi

REPO_ROOT="$(cd ../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
CHUNK_SO="$REPO_ROOT/native/libpaper_native_chunk_encode_jni.so"
LIBS="$MAIN_SO:$CHUNK_SO"                   # canonical name: libpaper_native_chunk_encode_jni.so (no legacy variant ships)
N="${KPREF_N:-16}"
RUNTIME_SO="${KPREF_RUNTIME_SO:-/home/z/server/libcrussty_runtime.so}"
MODULE_SO="${KPREF_MODULE_SO:-$REPO_ROOT/target/release/libcrussty.so}"
SERVER="${KPREF_SERVER:-/home/z/server}"
STAGE="${KPREF_STAGE:-/tmp/crussty-kpref-modstage}"
RUNTO="${KPREF_TIMEOUT:-100}"
OUT_TSV="results/kernel_pref_coverage.tsv"

# gid:class:alt_kernel:paired_old_kernel:P500_REPORT_v2 alt/old ratio.
# QUOTED single-line array elements + quoted expansion: immune to IFS/word
# splitting (a multiline literal var once expanded to garbage "1001" under
# the sandbox bash — never again; the assert below guards the parse anyway).
ROWS=(
"19:PaperNativeLevelChunkHeightmap:newCombinedUpdateSummary:oldFourUpdateSummary:5.70"
"20:PaperNativeMarkerCache:cachedSummary:oldSummary:4.54"
"27:PaperNativePalettedReencodeScratch:directPackedSummary:oldNewArraySummary:2.35"
"34:PaperNativeProtoChunkHeightmap:newCachedContainsSummary:oldEnumSetForeachSummary:1.78"
)

log() { echo "[verify_kernel_pref] $*"; }

# --- parse self-test (fail BEFORE burning the bench lock on garbage) ---------
[ "${#ROWS[@]}" -eq 4 ] || { log "FATAL: expected 4 rows, got ${#ROWS[@]}"; exit 1; }
for row in "${ROWS[@]}"; do
  IFS=':' read -r gid class alt old v2ratio <<< "$row"
  case "$gid" in 19|20|27|34) ;; *) log "FATAL: bad gid [$gid] from row [$row]"; exit 1;; esac
  [ -n "$class" ] && [ -n "$alt" ] && [ -n "$old" ] && [ -n "$v2ratio" ] \
    || { log "FATAL: incomplete row [$row]"; exit 1; }
done
log "parse self-test OK: ${#ROWS[@]} remap candidates"

# --- preflight ---------------------------------------------------------------
for f in "$JAVA" "$MAIN_SO" "$CHUNK_SO" "$RUNTIME_SO" "$SERVER/modules/crussty/module.json"; do
  [ -e "$f" ] || { log "FATAL missing: $f"; exit 1; }
done
if [ ! -e "$MODULE_SO" ]; then
  log "FATAL: module .so with the TASK-04 gate not found: $MODULE_SO"
  log "       build it (cargo build --release) or set KPREF_MODULE_SO."
  log "       NOTE: the deployed server module .so predates TASK-04 (no gate) — not a fallback."
  exit 1
fi

# --- build bench classes into a PRIVATE dir (never touches shared classes/) ---
BUILD=build-kpref
rm -rf "$BUILD" "$STAGE"; mkdir -p "$BUILD" logs results
"$JAVAC" -d "$BUILD" $(find java -name '*.java') scripts/KernelPrefDriver.java || exit 1

# --- stage a private module tree (module.json + native payloads are copied
#     READ-ONLY from the deployed server; the .so is the TASK-04 build) -------
mkdir -p "$STAGE/crussty/native"
cp "$MODULE_SO" "$STAGE/crussty/libcrussty.so" || exit 1
cp "$SERVER/modules/crussty/module.json" "$STAGE/crussty/" || exit 1
cp "$SERVER/modules/crussty/native/"* "$STAGE/crussty/native/" || exit 1
log "module staged: $STAGE (so=$MODULE_SO)"

: > "$OUT_TSV"
echo -e "layer\tmode\tgid\tclass\talt_kernel\tpaired_old\tp500v2_ratio\tremap_lines_total\tclass_remap\tbootstrap_probes\talt_ns_op\told_ns_op\trun_rc" >> "$OUT_TSV"

parse_ns() { # $1=stdout-file $2=method -> median ns/op
  local v
  v=$(awk -F'\t' -v m="$2" '$1=="RESULT" && $5==m {print $7; exit}' "$1")
  echo "${v:-NaN}"
}

AGENT_OK=1   # flipped to 0 if the first agent run proves injection never happens

run_one() { # $1=layer $2=mode $3=gid $4=class $5=alt $6=old $7=v2ratio
  local layer="$1" mode="$2" gid="$3" class="$4" alt="$5" old="$6" v2ratio="$7"
  local tag="${layer}_${mode}_g${gid}"
  local pref=()
  [ "$mode" = "pref-old" ] && pref=(CRUSSTY_KERNEL_PREF=old)
  local args=(-Xms512m -Xmx1g -XX:+AlwaysPreTouch -Xbatch -XX:+UseG1GC
              -Dp500.n="$N" -Dp500.libs="$LIBS")
  if [ "$layer" = "agent" ]; then
    # single argv element, mirrors the LIVE server cmdline exactly:
    #   -agentpath:<so>=modules=<dir>;versions=<dir>;kernel=purpur-1.21.10.jar
    args+=("-agentpath:$RUNTIME_SO=modules=$STAGE;versions=$SERVER/versions;kernel=purpur-1.21.10.jar"
           -Dp500.kprefwait=true)
  fi
  if [ "$layer" = "agent" ] && [ "$AGENT_OK" = "0" ]; then
    echo -e "$layer\t$mode\t$gid\t$class\t$alt\t$old\t$v2ratio\t0\t0\t0\tSKIPPED\tSKIPPED\tskip" >> "$OUT_TSV"
    log "  $tag: SKIPPED (no bootstrap injection in bare JVM — see first agent run)"
    return
  fi
  log "run $tag (n=$N, pref=${pref[*]:-unset})"
  env "${pref[@]}" timeout "$RUNTO" "$JAVA" "${args[@]}" -cp "$BUILD" KernelPrefDriver "$gid" \
      > "logs/kpref_$tag.out" 2> "logs/kpref_$tag.err"
  local rc=$?
  local remaps classremap probes alt_ns old_ns
  remaps=$(grep -c 'kernel_pref: .* bound to old kernel' "logs/kpref_$tag.err" 2>/dev/null) || remaps=0
  classremap=$(grep -c "kernel_pref: $class\\.$alt bound to old kernel" "logs/kpref_$tag.err" 2>/dev/null) || classremap=0
  probes=$(grep -o 'bootstrap(injected)' "logs/kpref_$tag.err" 2>/dev/null | wc -l)
  alt_ns=$(parse_ns "logs/kpref_$tag.out" "$alt")
  old_ns=$(parse_ns "logs/kpref_$tag.out" "$old")
  echo -e "$layer\t$mode\t$gid\t$class\t$alt\t$old\t$v2ratio\t$remaps\t$classremap\t$probes\t$alt_ns\t$old_ns\t$rc" >> "$OUT_TSV"
  log "  $tag: rc=$rc remaps=$remaps class_remap=$classremap injected_probes=$probes alt=$alt_ns ns/op old=$old_ns ns/op"
  # adaptive bail-out: first agent run decides whether the layer is viable
  if [ "$layer" = "agent" ] && [ "${probes:-0}" -eq 0 ]; then
    AGENT_OK=0
    log "  agent layer NOT viable in a bare bench JVM (0 bootstrap-injected probes) — skipping remaining agent runs"
  fi
}

# pass 1: the gate surface (agent layer) for every candidate
for row in "${ROWS[@]}"; do
  IFS=':' read -r gid class alt old v2ratio <<< "$row"
  run_one agent default    "$gid" "$class" "$alt" "$old" "$v2ratio"
  run_one agent pref-old   "$gid" "$class" "$alt" "$old" "$v2ratio"
done

# pass 2: inertness documentation (plain layer, no runtime attached)
for row in "${ROWS[@]}"; do
  IFS=':' read -r gid class alt old v2ratio <<< "$row"
  run_one plain default    "$gid" "$class" "$alt" "$old" "$v2ratio"
  run_one plain pref-old   "$gid" "$class" "$alt" "$old" "$v2ratio"
done

log "done — TSV: $OUT_TSV"
cat "$OUT_TSV"
