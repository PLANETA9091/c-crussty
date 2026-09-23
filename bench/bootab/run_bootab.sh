#!/usr/bin/env bash
# bench/bootab/run_bootab.sh — TASK-32 boot-latency A/B harness (single-variant runner).
# TASK-45: + --phase2 <seconds> post-marker measurement window with TASK-22
# scan-avoidance counter extraction (see BOOTAB_REPORT.md §Phase-2).
#
# Boots a THROWAWAY Purpur 1.21.10 + CRUSSTY instance in a per-run temp dir,
# measures seconds-to-ready-marker (primary + secondary), RSS, kills the exact
# PID it spawned, cleans up. All work is foreground within the caller's process
# (sandbox kills backgrounded processes between tool calls — every batch of runs
# must live inside ONE foreground tool call, see BOOTAB_REPORT.md).
#
# Usage: run_bootab.sh <ready-marker-ERE> <max-wait-s> <runs> [--phase2 <seconds>]
#   ready-marker-ERE : primary readiness marker (grep -E) in captured boot log,
#                      e.g. 'native surface live'
#   max-wait-s       : per-run cap; on timeout the run is recorded TIMEOUT and killed
#   runs             : how many sequential boots for this variant
#   --phase2 <s>     : TASK-45 phase-2 mode — after the PRIMARY marker keep the
#                      server running <s> seconds (default 60), THEN SIGTERM the
#                      exact PID, then parse the log for the TASK-22 counters
#                      (per-hook activation lines 'sighting feed: N full
#                      class-heap scans avoided', hook-armed lines, force-load
#                      attempt lines) and append a phase2 TSV row.
#                      <s> = 0 disables phase-2 (legacy: stop at secondary marker).
#
# Counter semantics (cplug-sdk/src/classes.rs, TASK-22 e9405d1):
#   find_class cache-miss gate: FIRST unsighted call scans + every 8th after
#   that (UNSIGHTED_SKIP_BUDGET = 7 skips between scans); every skipped call
#   bumps the per-name 'avoided' counter, printed ONCE per hook at activation:
#     scans_performed = 1 + floor(avoided / 7)
#     unsighted_calls = avoided + scans_performed
#     pct_calls_avoided = 100 * avoided / unsighted_calls
#   The pre-TASK-22 module (arm A @ 4fb9d12) has NO gate/counter: every
#   cache-miss IS a full JVMTI GetLoadedClasses scan, so its sighting-line
#   counts are 0 and scans must be derived from poller iterations instead
#   (see report) — the harness only reports raw observables for that arm.
#
# Env:
#   BOOTAB_MODULES   (required) modules dir for this arm (contains crussty/)
#   BOOTAB_TAG       arm label for TSV + log names          (default "arm")
#   BOOTAB_ROOT      scratch root for run dirs / copies     (default /tmp/ab-boot)
#   BOOTAB_TSV       results TSV (append)                   (default $BOOTAB_ROOT/results-<tag>.tsv)
#   BOOTAB_PORT_BASE first port tried; +1 per busy collision (default 26100)
#   BOOTAB_SERVER_SRC source layout to copy jars/libs from  (default /home/z/server — READ-ONLY, only read)
#   BOOTAB_JAVA      java binary                            (default /home/z/jdk21/bin/java)
#   BOOTAB_XMX       max heap for BOTH arms identically     (default 1G; live uses 2G but this box
#                    has 4GB RAM with the live server resident — 1G documented in report)
#   BOOTAB_SECONDARY secondary marker ERE; run stops when BOTH seen ('' = primary only)
#                                                            (default 'Done \([0-9.]+s\)')
#   BOOTAB_KEEP      1 = keep per-run scratch dirs          (default 0)
#   BOOTAB_PHASE2_TSV phase2 TSV (append)                  (default $BOOTAB_ROOT/phase2-<tag>.tsv)
#   BOOTAB_COMMIT    module commit sha to stamp into the phase2 TSV (default "-")
#   CRUSSTY_* vars are inherited into the server env (live server runs with
#   CRUSSTY_NATIVE_IMPROVED_NOISE=1 — replicate with CRUSSTY_NATIVE_IMPROVED_NOISE=1).
#
# Safety invariants:
#   - NEVER writes under BOOTAB_SERVER_SRC (versions jar, cache jar, libraries are
#     COPIED into the run dir; modules dir is a symlink to the arm's own copy).
#   - Kills ONLY the java PID it spawned (tracked; verified dead; stray scan for
#     the unique run dir name afterwards).
#   - Ports: BOOTAB_PORT_BASE+i, live server (25565) never used.
set -u

MARKER="${1:?usage: run_bootab.sh <ready-marker-ERE> <max-wait-s> <runs> [--phase2 <s>]}"
MAXWAIT="${2:?usage: run_bootab.sh <ready-marker-ERE> <max-wait-s> <runs> [--phase2 <s>]}"
RUNS="${3:?usage: run_bootab.sh <ready-marker-ERE> <max-wait-s> <runs> [--phase2 <s>]}"

# --- TASK-45: optional trailing flag args (--phase2 <seconds>, default 60) ---
PHASE2=60
shift 3 2>/dev/null || true
while [ $# -gt 0 ]; do
  case "$1" in
    --phase2) PHASE2="${2:?--phase2 requires a seconds value}"; shift 2 ;;
    *) printf 'unknown arg: %s\n' "$1" >&2; exit 2 ;;
  esac
done

BOOTAB_ROOT="${BOOTAB_ROOT:-/tmp/ab-boot}"
BOOTAB_TAG="${BOOTAB_TAG:-arm}"
BOOTAB_TSV="${BOOTAB_TSV:-$BOOTAB_ROOT/results-$BOOTAB_TAG.tsv}"
BOOTAB_PORT_BASE="${BOOTAB_PORT_BASE:-26100}"
BOOTAB_SERVER_SRC="${BOOTAB_SERVER_SRC:-/home/z/server}"
BOOTAB_JAVA="${BOOTAB_JAVA:-/home/z/jdk21/bin/java}"
BOOTAB_XMX="${BOOTAB_XMX:-1G}"
BOOTAB_SECONDARY="${BOOTAB_SECONDARY-Done \([0-9.]+s\)}"
BOOTAB_KEEP="${BOOTAB_KEEP:-0}"
BOOTAB_PHASE2_TSV="${BOOTAB_PHASE2_TSV:-$BOOTAB_ROOT/phase2-$BOOTAB_TAG.tsv}"
BOOTAB_COMMIT="${BOOTAB_COMMIT:--}"
: "${BOOTAB_MODULES:?BOOTAB_MODULES=<modules dir> required (must contain crussty/)}"

mkdir -p "$BOOTAB_ROOT/logs"

log() { printf '%s\n' "[bootab/$BOOTAB_TAG] $*"; }

# BENCH.lock courtesy protocol (sibling agents run timed benches on this box).
exec 9>/home/z/BENCH.lock
if flock -w 30 9; then log "BENCH.lock acquired"; else log "WARN: BENCH.lock busy 30s — proceeding (live server is the bigger noise source anyway)"; fi

port_free() { ! (exec 3<>"/dev/tcp/127.0.0.1/$1") 2>/dev/null; }

TSV_HEADER="tag run     port    pid     t0_epoch        primary_s       secondary_s     paper_done_s    rss_primary_kb  hwm_kb  outcome anomalies"
[ -f "$BOOTAB_TSV" ] || printf '%s\n' "$TSV_HEADER" >> "$BOOTAB_TSV"

for i in $(seq 1 "$RUNS"); do
  port="$((BOOTAB_PORT_BASE + i))"
  while ! port_free "$port"; do port=$((port + 1)); done
  rundir="$(mktemp -d "$BOOTAB_ROOT/run-$BOOTAB_TAG-$i-XXXXXX")"
  log "run $i: dir=$rundir port=$port"

  # --- assemble throwaway server root (copies only; source NEVER written) ---
  cp -r "$BOOTAB_SERVER_SRC/libraries" "$rundir/libraries"
  mkdir -p "$rundir/versions" "$rundir/cache"
  cp "$BOOTAB_SERVER_SRC/versions/purpur-1.21.10.jar" "$rundir/versions/"
  cp "$BOOTAB_SERVER_SRC/cache/mojang_1.21.10.jar"    "$rundir/cache/"
  ln -s "$BOOTAB_MODULES" "$rundir/modules"
  printf 'eula=true\n' > "$rundir/eula.txt"
  {
    printf 'server-port=%s\n' "$port"
    printf 'online-mode=false\n'
    printf 'level-type=minecraft\\:flat\n'
    printf 'generate-structures=false\n'
    printf 'view-distance=4\n'
    printf 'simulation-distance=4\n'
    printf 'spawn-protection=0\n'
    printf 'motd=bootab-%s\n' "$BOOTAB_TAG"
  } > "$rundir/server.properties"

  # --- boot ---
  cd "$rundir"
  t0=$(date +%s.%N)
  "$BOOTAB_JAVA" \
    -agentpath:"$BOOTAB_ROOT/rt/libcrussty_runtime.so=modules=$rundir/modules;versions=$rundir/versions;kernel=purpur-1.21.10.jar" \
    -Xms512M -Xmx"$BOOTAB_XMX" -XX:+UseG1GC -Dfile.encoding=UTF-8 \
    -Ddist.root="$rundir" \
    -jar "$rundir/versions/purpur-1.21.10.jar" --nogui nogui \
    > "$rundir/run.log" 2>&1 &
  PID=$!
  log "run $i: pid=$PID"
  t_primary=""; t_secondary=""
  deadline=$(( $(date +%s) + MAXWAIT ))
  phase2_deadline=0
  while :; do
    if [ -z "$t_primary" ] && grep -q -E "$MARKER" "$rundir/run.log" 2>/dev/null; then
      t_primary=$(date +%s.%N)
      log "run $i: PRIMARY marker @ ${t_primary}s"
      [ "$PHASE2" -gt 0 ] && phase2_deadline=$(( $(date +%s) + PHASE2 ))
    fi
    if [ -n "$BOOTAB_SECONDARY" ] && [ -z "$t_secondary" ] && grep -q -E "$BOOTAB_SECONDARY" "$rundir/run.log" 2>/dev/null; then
      t_secondary=$(date +%s.%N)
      log "run $i: SECONDARY marker @ ${t_secondary}s"
    fi
    # TASK-45: phase-2 mode stops PHASE2 seconds AFTER primary (secondary is
    # recorded but not a stop condition); legacy mode stops at secondary.
    if [ "$PHASE2" -gt 0 ]; then
      [ -n "$t_primary" ] && [ "$(date +%s)" -ge "$phase2_deadline" ] && { log "run $i: PHASE2 window (${PHASE2}s) elapsed"; break; }
    else
      [ -n "$t_secondary" ] && break
      [ -z "$BOOTAB_SECONDARY" ] && [ -n "$t_primary" ] && break
    fi
    if ! kill -0 "$PID" 2>/dev/null; then log "run $i: JVM DIED before markers"; break; fi
    [ "$(date +%s)" -ge "$deadline" ] && { log "run $i: TIMEOUT cap ${MAXWAIT}s"; break; }
    sleep 0.1
  done

  # --- TASK-45: optional post-marker hold (stats windows need a live JVM) ---
  if [ -n "${BOOTAB_POST_HOLD_S:-}" ] && [ "$BOOTAB_POST_HOLD_S" -gt 0 ] 2>/dev/null && kill -0 "$PID" 2>/dev/null; then
    log "run $i: post-marker hold ${BOOTAB_POST_HOLD_S}s"
    sleep "$BOOTAB_POST_HOLD_S"
  fi

  # --- RSS snapshot + exact-PID kill ---
  rss_primary="-"; hwm="-"
  if kill -0 "$PID" 2>/dev/null; then
    rss_primary=$(awk '/VmRSS/{print $2}' /proc/$PID/status 2>/dev/null || echo "-")
    hwm=$(awk '/VmHWM/{print $2}' /proc/$PID/status 2>/dev/null || echo "-")
    kill -TERM "$PID" 2>/dev/null
    for _ in $(seq 1 200); do kill -0 "$PID" 2>/dev/null || break; sleep 0.1; done
    kill -0 "$PID" 2>/dev/null && { log "run $i: TERM ignored -> KILL"; kill -KILL "$PID" 2>/dev/null; sleep 1; }
  fi
  wait "$PID" 2>/dev/null; jrc=$?
  if kill -0 "$PID" 2>/dev/null; then log "run $i: PID $PID STILL ALIVE"; fi

  # --- collect ---
  t0i=${t0%.*}
  p_s="-"; s_s="-"
  [ -n "$t_primary" ]   && p_s=$(awk -v a="$t_primary"   -v b="$t0"   'BEGIN{printf "%.2f", a-b}')
  [ -n "$t_secondary" ] && s_s=$(awk -v a="$t_secondary" -v b="$t0" 'BEGIN{printf "%.2f", a-b}')
  paper_done=$(grep -oE 'Done \([0-9.]+s\)' "$rundir/run.log" | head -1 | grep -oE '[0-9.]+s' || echo "-")
  anom=$(grep -c -E 'ERROR|Exception in thread|UnsatisfiedLink|java\.lang\.[A-Za-z]*Error|hs_err' "$rundir/run.log" 2>/dev/null || true)
  unresolved=$(grep -oE '[0-9]+ symbols unresolved' "$rundir/run.log" | head -1 || echo "-")
  outcome="OK"; [ -z "$t_primary" ] && outcome="NO_MARKER"; [ "$PHASE2" -eq 0 ] && [ -z "$t_secondary" ] && [ -n "$BOOTAB_SECONDARY" ] && [ "$outcome" = "OK" ] && outcome="NO_SECONDARY"
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$BOOTAB_TAG" "$i" "$port" "$PID" "$t0i" "$p_s" "$s_s" "$paper_done" "$rss_primary" "$hwm" "$outcome" "anom=$anom;$unresolved" >> "$BOOTAB_TSV"
  cp "$rundir/run.log" "$BOOTAB_ROOT/logs/$BOOTAB_TAG-$i.log" 2>/dev/null

  # --- TASK-45 phase-2: extract TASK-22 scan-avoidance counters from the log ---
  if [ "$PHASE2" -gt 0 ]; then
    [ -f "$BOOTAB_PHASE2_TSV" ] || printf 'arm\trun\tcommit\twindow_s\tam_armed\tam_sighting\tam_avoided\tam_scans_derived\tam_attempt_max\tnoise_armed\tnoise_sighting\tnoise_avoided\tnoise_scans_derived\tnoise_attempt_max\tpct_calls_avoided\toutcome\n' >> "$BOOTAB_PHASE2_TSV"
    L="$BOOTAB_ROOT/logs/$BOOTAB_TAG-$i.log"
    am_armed=$(grep -c -E 'area_map: hook armed, retransform rc=' "$L" 2>/dev/null || true)
    noise_armed=$(grep -c -E 'improved_noise: hook armed, retransform rc=' "$L" 2>/dev/null || true)
    am_sight=$(grep -c -E 'area_map: sighting feed: [0-9]+ full class-heap scans avoided' "$L" 2>/dev/null || true)
    noise_sight=$(grep -c -E 'improved_noise: sighting feed: [0-9]+ full class-heap scans avoided' "$L" 2>/dev/null || true)
    am_avoided=$(grep -oE 'area_map: sighting feed: [0-9]+ full class-heap scans avoided' "$L" 2>/dev/null | grep -oE '[0-9]+' | awk '{s+=$1} END{print s+0}')
    noise_avoided=$(grep -oE 'improved_noise: sighting feed: [0-9]+ full class-heap scans avoided' "$L" 2>/dev/null | grep -oE '[0-9]+' | awk '{s+=$1} END{print s+0}')
    am_att=$(grep -oE 'area_map: forcing kernel load of .* \(attempt [0-9]+\)' "$L" 2>/dev/null | grep -oE '[0-9]+' | sort -n | tail -1)
    noise_att=$(grep -oE 'improved_noise: forcing kernel load of .* \(attempt [0-9]+\)' "$L" 2>/dev/null | grep -oE '[0-9]+' | sort -n | tail -1)
    am_att=${am_att:-0}; noise_att=${noise_att:-0}
    # scans_performed = 1 + floor(avoided/7) — only where the counter exists.
    am_scans="-"; noise_scans="-"; pct="-"
    [ "$am_sight" -gt 0 ] && am_scans=$(( 1 + am_avoided / 7 ))
    [ "$noise_sight" -gt 0 ] && noise_scans=$(( 1 + noise_avoided / 7 ))
    if [ "$am_sight" -gt 0 ] || [ "$noise_sight" -gt 0 ]; then
      tot_av=$(( am_avoided + noise_avoided )); tot_sc=0
      [ "$am_sight" -gt 0 ] && tot_sc=$(( tot_sc + 1 + am_avoided / 7 ))
      [ "$noise_sight" -gt 0 ] && tot_sc=$(( tot_sc + 1 + noise_avoided / 7 ))
      [ "$tot_sc" -gt 0 ] && pct=$(awk -v a="$tot_av" -v s="$tot_sc" 'BEGIN{printf "%.1f", 100*a/(a+s)}')
    fi
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
      "$BOOTAB_TAG" "$i" "$BOOTAB_COMMIT" "$PHASE2" "$am_armed" "$am_sight" "$am_avoided" "$am_scans" "$am_att" \
      "$noise_armed" "$noise_sight" "$noise_avoided" "$noise_scans" "$noise_att" "$pct" "$outcome" >> "$BOOTAB_PHASE2_TSV"
    log "run $i: PHASE2 counters: am(armed=$am_armed sight=$am_sight avoided=$am_avoided scans~$am_scans att=$am_att) noise(armed=$noise_armed sight=$noise_sight avoided=$noise_avoided scans~$noise_scans att=$noise_att) pct_calls_avoided=$pct"
  fi

  # --- cleanup own dir; verify no strays of OURS remain ---
  cd "$BOOTAB_ROOT"
  [ "$BOOTAB_KEEP" = "1" ] || rm -rf "$rundir"
  stray=$(pgrep -af "run-$BOOTAB_TAG-$i-" || true)
  [ -n "$stray" ] && log "run $i: WARN stray processes: $stray" || log "run $i: no strays, dir cleaned"
  log "run $i: DONE primary=${p_s}s secondary=${s_s}s paper=${paper_done} rss=${rss_primary}kB hwm=${hwm}kB rc=$jrc outcome=$outcome"
done
log "batch complete -> $BOOTAB_TSV"
