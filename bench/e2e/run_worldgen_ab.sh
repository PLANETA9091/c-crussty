#!/usr/bin/env bash
# TASK-63 (agent-7625532f, 2026-09-09): paired A/B — proven noise-native bridge
# under REAL worldgen load. First real-load A/B of the bridge (boot channel was
# refuted by TASK-58; fresh worldgen identified as the addressable channel by
# TASK-62 F2: 4.12 CPU-s ImprovedNoise/PerlinNoise per 128-chunk burst).
#
# Derived harness — scripts/e2e_orchestrate.sh is used UNTOUCHED (boot/shutdown).
#   Arms: A dormant (no CRUSSTY_* overrides, production default)
#         B CRUSSTY_NATIVE_IMPROVED_NOISE=1
#   SAME deployed module .so in both arms; the differential is the env var only.
#   JFR is EXCLUDED from timed runs (TASK-58 discipline); the mechanism-proof
#   appendage (run_jfr_proof) is a separate non-timed run.
#
# Burst protocol per run (identical work every run):
#   boot -> Done + armed-marker validation -> settle
#   -> forceload add 1600 1600 1727 1727   (chunks 100..107 x 100..107, 64)
#   -> forceload add -1728 -1728 -1601 -1601 (chunks -108..-101, 64)  [TASK-62 coords]
#   -> completion detector: /proc/<child>/stat total CPU rate (all threads)
#      falls below IDLE_RATE for IDLE_STREAK consecutive samples after a
#      >ACTIVE_RATE phase was observed
#   -> forceload query (relay-alive proof) -> forceload remove all
#   -> graceful fifo stop (TASK-59 path) -> world restored from seed tar.
#
# Metrics per run: t_burst (wall, send->detector), cpu_burst (child utime+stime
# delta over the same window), boot Done time (secondary, expected flat per
# TASK-58), watchdog-fire covariate (confound guard: the stall dump itself burns
# CPU; recorded so stats can be recomputed excluding watchdog runs).
#
# Interleaving: ABBA blocks, N runs per arm (claims-grade §6: live server DOWN
# between runs — each run boots a fresh instance on a byte-identical world).
#
# Hygiene: /home/z/BENCH.lock flock holder for the whole session; world+nether+
# end seed tar BEFORE the session (server stopped); byte-identical restore after
# EVERY run; final diff -r verification vs seed extraction; token never exposed.
set -u

SERVER_DIR=/home/z/server
LOGS_DIR=$SERVER_DIR/logs
LATEST=$LOGS_DIR/latest.log
E2E_LOG=$LOGS_DIR/crussty_e2e_boot.log
E2E=/home/z/c-crussty/scripts/e2e_orchestrate.sh
RES=/home/z/c-crussty/bench/e2e/results
RAW=$RES/WORLDGEN_AB_RAW_2026-09-09
SEED_TAR=$SERVER_DIR/world_ab_seed_task63.tar.gz
N_PER_ARM=5
ACTIVE_RATE=0.5   # cores: burst phase threshold (total child CPU, all threads)
IDLE_RATE=0.15    # cores: idle tick baseline ~0.08 (TASK-62 minute profile)
IDLE_STREAK=3     # consecutive 0.5 s samples below IDLE_RATE = burst over
BURST_TIMEOUT=420 # hard cap per burst (s)

log() { printf '%s %s\n' "$(date -u +%H:%M:%S)" "$*"; }

# Signal telemetry (survivability forensics): if the series vanishes again,
# the trap log says WHICH signal arrived; a bare EXIT line = internal exit.
TRAP_LOG=/tmp/task63_trap.log
echo "$(date -u +%H:%M:%S) START pid=$$" >>"$TRAP_LOG"
trap 'echo "$(date -u +%H:%M:%S) EXIT rc=$? pid=$$" >>"$TRAP_LOG"' EXIT
trap 'echo "$(date -u +%H:%M:%S) TERM pid=$$" >>"$TRAP_LOG"; exit 143' TERM
trap 'echo "$(date -u +%H:%M:%S) HUP pid=$$" >>"$TRAP_LOG"; exit 129' HUP
trap 'echo "$(date -u +%H:%M:%S) INT pid=$$" >>"$TRAP_LOG"; exit 130' INT

lock_on()  { setsid bash -c 'exec 9>>/home/z/BENCH.lock; flock 9; while :; do sleep 60; done' & }
lock_off() { pgrep -f '9>>/home/z/BENCH.lock' | xargs -r kill 2>/dev/null; return 0; }

# one-call mode lock: acquire inside THIS process, auto-released at process end
# (sandbox background processes were SIGKILL'd non-deterministically during the
# 03:51-04:10 session — three tree patterns lost; series now driven as one run
# per agent tool call, lock lifetime == call lifetime)
lock_acquire_inline() {
    exec 9>>/home/z/BENCH.lock
    flock -n 9 || { log "BENCH.lock held by someone else — refusing"; exit 1; }
}

server_pid() { pgrep -f 'agentpath:[^ ]*libcrussty_runtime\.so' | head -1; }
cpu_ticks()  { awk '{print $14+$15}' "/proc/$1/stat" 2>/dev/null; }

world_backup_seed() {
    # BACKUP FIRST, ALWAYS. INCIDENT 2026-09-09 (root-caused): the first draft
    # ran selfheal (which restores) BEFORE creating the seed tar -> world dirs
    # were rm -rf'd and the boot generated a fresh random-seed world. Nether/end
    # had no backup (never accessed by design; Paper recreates them at boot).
    # Guards below make restore IMPOSSIBLE without a verified seed tar.
    local dirs=(world)
    [ -d "$SERVER_DIR/world_nether" ] && dirs+=(world_nether)
    [ -d "$SERVER_DIR/world_the_end" ] && dirs+=(world_the_end)
    rm -f "$SEED_TAR"
    tar -czf "$SEED_TAR" -C "$SERVER_DIR" "${dirs[@]}"
    local sz; sz=$(stat -c %s "$SEED_TAR")
    if [ "$sz" -lt 102400 ]; then
        rm -f "$SEED_TAR"
        log "FATAL: seed tar suspiciously small (${sz}B) — refusing to proceed"
        exit 1
    fi
    [ -f "$SERVER_DIR/world/level.dat" ] || { log "FATAL: level.dat missing pre-backup"; exit 1; }
    log "seed tar: $(du -h "$SEED_TAR" | cut -f1) (${#dirs[*]} dirs)"
}

world_restore() {
    # Hard guard: never destroy live world dirs without a verified seed tar.
    if [ ! -s "$SEED_TAR" ] || [ "$(stat -c %s "$SEED_TAR")" -lt 102400 ]; then
        log "FATAL: world_restore without valid seed tar — ABORT (live dirs untouched)"
        exit 1
    fi
    rm -rf "$SERVER_DIR/world" "$SERVER_DIR/world_nether" "$SERVER_DIR/world_the_end"
    tar -xzf "$SEED_TAR" -C "$SERVER_DIR"
    [ -f "$SERVER_DIR/world/level.dat" ] || { log "FATAL: restore produced no level.dat"; exit 1; }
}

selfheal() { # if a previous run died mid-flight: stop + restore (seed-tar guarded)
    if [ -n "$(server_pid)" ]; then
        log "selfheal: server running — shutting down"
        "$E2E" shutdown >>"$RAW/selfheal_shutdown.txt" 2>&1
        sleep 2
    fi
    # Only restore when a seed tar already exists (post-backup). Before the
    # session's first backup this is a no-op on the world (INCIDENT guard).
    [ -s "$SEED_TAR" ] && world_restore
}

boot_arm() { # $1 = A|B ; boots via canonical orchestrate, stdout to RAW
    local arm=$1 out=$2
    if [ "$arm" = B ]; then
        CRUSSTY_NATIVE_IMPROVED_NOISE=1 "$E2E" boot >"$out" 2>&1
    else
        env -u CRUSSTY_NATIVE_IMPROVED_NOISE "$E2E" boot >"$out" 2>&1
    fi
}

armed_markers() { # $1 = A|B -> rc 0 iff arm state as expected (loud on surprise)
    local arm=$1
    if [ "$arm" = B ]; then
        grep -qE 'hook armed|self-test passed|defining bridge' "$E2E_LOG"
    else
        if grep -qE 'hook armed|self-test passed' "$E2E_LOG"; then return 1; fi
        return 0
    fi
}

fifo_send() { # $1 = command text (blocking-open guarded by timeout)
    timeout 5 bash -c 'printf "%s\n" "$1" > "$2"' _ "$1" "$FIFO"
}

run_one() { # $1 = idx, $2 = A|B
    local idx=$1 arm=$2
    local tag="run${idx}_${arm}"
    local pid fifo t0 cpu0
    log "=== $tag boot (arm $arm)"
    boot_arm "$arm" "$RAW/${tag}_boot.txt" || { log "$tag BOOT FAIL"; return 1; }
    pid=$(server_pid)
    [ -n "$pid" ] || { log "$tag no child pid after boot"; return 1; }

    local done_s
    done_s=$(grep -o 'Done ([0-9.]*s' "$LATEST" 2>/dev/null | head -1 | sed 's/.*(\([0-9.]*\)s/\1/')

    # ---- arming gate (POST-Done!) — burst may only start AFTER the marker
    # attempt-1 lesson: checking at Done+0s raced the arming chain (its class
    # force-load can retry up to attempt 6+); run2_B had "hook armed" only in
    # the log afterwards — burst-start vs arming-completion ordering UNPROVEN.
    local armv=ok armwait=0
    if [ "$arm" = B ]; then
        # run7 lesson: the force-load retry loop can be slower than 25 s
        # (attempt 4 at cap) — 90 s cap; a still-unarmed run must NOT be
        # benchmarked as B (it would silently measure the dormant arm).
        while ! grep -qE 'hook armed|self-test passed' "$E2E_LOG" && [ "$armwait" -lt 90 ]; do
            sleep 1; armwait=$((armwait + 1))
        done
        if ! grep -qE 'hook armed|self-test passed' "$E2E_LOG"; then
            log "$tag NO_ARM after ${armwait}s — INVALID B-run, aborting (shutdown+restore)"
            grep -E 'crussty|improved_noise' "$E2E_LOG" >"$RAW/${tag}_markers.txt" 2>/dev/null
            "$E2E" shutdown >>"$RAW/${tag}_shutdown.txt" 2>&1
            sleep 2
            world_restore
            printf '%s,%s,%s,%s,%s,%s\n' "$idx" "$arm" "NA" "NA" "${done_s:-NA}" "NO_ARM@${armwait}" \
                >>"$RAW/summary.csv"
            return 1
        fi
    else
        sleep 6   # dormancy cross-check window
        grep -qE 'hook armed|self-test passed' "$E2E_LOG" && armv=UNEXPECTED_ARM
    fi
    # per-run marker evidence (fresh E2E_LOG = this boot only)
    grep -E 'crussty|improved_noise|hook|self-test|kernel' "$E2E_LOG" \
        >"$RAW/${tag}_markers.txt" 2>/dev/null
    log "$tag arm-validation=$armv wait=${armwait}s done=${done_s}s pid=$pid"
    # Canary: a FRESH-world boot (not the restored seed) takes ~27 s here vs
    # ~16-18 s for the pre-generated world. If seen: something destroyed the
    # world between backup and boot — abort loudly instead of benchmarking a
    # different world.
    if [ -n "${done_s:-}" ] && awk -v d="$done_s" 'BEGIN{exit !(d>22)}'; then
        log "FATAL: $tag Done=${done_s}s > 22s — fresh-world symptom (INCIDENT guard)"
        "$E2E" shutdown >/dev/null 2>&1; sleep 2; world_restore
        return 1
    fi

    sleep 4   # uniform post-boot settle (both arms)

    fifo=$(cat "$LOGS_DIR/crussty_e2e.state" 2>/dev/null)
    if [ ! -p "$fifo" ]; then log "$tag no fifo in state file"; "$E2E" shutdown >/dev/null 2>&1; return 1; fi
    FIFO=$fifo

    # ---- T0: send the two fresh-gen regions (same coords as TASK-62)
    cpu0=$(cpu_ticks "$pid"); t0=$(date +%s.%N)
    fifo_send "forceload add 1600 1600 1727 1727"      || log "$tag send1 rc=$?"
    fifo_send "forceload add -1728 -1728 -1601 -1601"  || log "$tag send2 rc=$?"

    # ---- completion detector: total-CPU rate monitor
    local trace="$RAW/${tag}_cputrace.txt"; : >"$trace"
    local tprev=$t0 cpuprev=$cpu0 now c dt cores active=0 streak=0 tend=0 cpuend=0 elapsed
    while :; do
        sleep 0.5
        now=$(date +%s.%N)
        c=$(cpu_ticks "$pid")
        if [ -z "$c" ]; then log "$tag child vanished mid-burst"; break; fi
        dt=$(awk -v a="$now" -v b="$tprev" 'BEGIN{print a-b}')
        cores=$(awk -v x=$((c - cpuprev)) -v d="$dt" 'BEGIN{print x/(d*100)}')
        echo "$now $cores" >>"$trace"
        tprev=$now; cpuprev=$c
        elapsed=$(awk -v a="$now" -v b="$t0" 'BEGIN{print a-b}')
        if awk -v r="$cores" 'BEGIN{exit !(r>0.5)}'; then active=1; streak=0; fi
        if [ "$active" = 1 ]; then
            if awk -v r="$cores" 'BEGIN{exit !(r<0.15)}'; then
                streak=$((streak + 1))
                if [ "$streak" -ge "$IDLE_STREAK" ] && awk -v e="$elapsed" 'BEGIN{exit !(e>5)}'; then
                    tend=$now; cpuend=$c; break
                fi
            else
                streak=0
            fi
        fi
        if awk -v e="$elapsed" "BEGIN{exit !(e>$BURST_TIMEOUT)}"; then
            log "$tag BURST TIMEOUT after ${BURST_TIMEOUT}s — aborting window"
            tend=$now; cpuend=$c; break
        fi
    done
    [ "$tend" = 0 ] && { tend=$(date +%s.%N); cpuend=$(cpu_ticks "$pid"); }
    local t_burst cpu_burst
    t_burst=$(awk -v a="$tend" -v b="$t0" 'BEGIN{printf "%.2f", a-b}')
    cpu_burst=$(awk -v a="${cpuend:-0}" -v b="${cpu0:-0}" 'BEGIN{printf "%.2f", (a-b)/100}')

    # ---- relay-alive proof + teardown
    local qout="$RAW/${tag}_forceload_query.txt"
    fifo_send "forceload query" && sleep 3 && tail -c 2000 "$LATEST" >"$qout" 2>/dev/null
    fifo_send "forceload remove all" || true
    sleep 2
    grep -c 'Watchdog' "$LATEST" 2>/dev/null | head -1 >"$RAW/${tag}_watchdog_count.txt"
    "$E2E" shutdown >"$RAW/${tag}_shutdown.txt" 2>&1
    sleep 2
    world_restore
    printf '%s,%s,%s,%s,%s,%s\n' "$idx" "$arm" "$t_burst" "$cpu_burst" "${done_s:-NA}" "${armv}@${armwait}" \
        >>"$RAW/summary.csv"
    log "$tag DONE t_burst=${t_burst}s cpu_burst=${cpu_burst}s"
    return 0
}

# fsync-friendly progress marker: the resume logic trusts summary.csv rows only
runs_done() { # -> number of completed timed runs recorded so far
    local n=0
    [ -f "$RAW/summary.csv" ] && n=$(( $(wc -l <"$RAW/summary.csv") - 1 ))
    [ "$n" -lt 0 ] && n=0
    echo "$n"
}

run_jfr_proof() { # non-timed arm-B run with JFR (mechanism proof, TASK-57 recipe)
    local jfrdir=$LOGS_DIR/jfr_task63
    mkdir -p "$jfrdir"
    log "=== JFR proof run (arm B + JFR)"
    JAVA_TOOL_OPTIONS="-XX:StartFlightRecording=filename=$jfrdir/server_%p.jfr,dumponexit=true,settings=profile,maxsize=128M" \
        CRUSSTY_NATIVE_IMPROVED_NOISE=1 "$E2E" boot >"$RAW/jfrproof_boot.txt" 2>&1 \
        || { log "JFR-proof BOOT FAIL"; return 1; }
    local pid; pid=$(server_pid)
    sleep 6   # arming settle
    grep -E 'hook armed|self-test passed' "$E2E_LOG" >"$RAW/jfrproof_markers.txt" 2>/dev/null || true
    date -u +%Y-%m-%dT%H:%M:%S >"$RAW/jfrproof_burst_start_utc.txt"
    local fifo; fifo=$(cat "$LOGS_DIR/crussty_e2e.state" 2>/dev/null); FIFO=$fifo
    fifo_send "forceload add 1600 1600 1727 1727"
    fifo_send "forceload add -1728 -1728 -1601 -1601"
    sleep 40  # mid-burst
    "/home/z/jdk21/bin/jcmd" "$pid" JFR.dump filename="$jfrdir/task63_b_midburst.jfr" \
        >>"$RAW/jfrproof_jcmd.txt" 2>&1
    sleep 30
    fifo_send "forceload remove all" || true
    sleep 2
    "$E2E" shutdown >"$RAW/jfrproof_shutdown.txt" 2>&1
    sleep 2
    world_restore
    date -u +%H:%M:%S >"$RAW/jfrproof_done"
    log "JFR proof run complete"
}

main() {
    mkdir -p "$RAW"
    # ---- one-run mode: `run_worldgen_ab.sh one <idx> <arm>` (agent-call driven)
    if [ "${1:-}" = one ]; then
        local idx=$2
        local arm=$3
        local tag="run${idx}_${arm}"
        [ -n "$(server_pid)" ] && { log "REFUSING: server already running"; exit 1; }
        lock_acquire_inline
        # Seed tar = SESSION ANCHOR: create only if absent. Re-tarring a dirty
        # world would poison every later restore (INCIDENT guard).
        { [ -s "$SEED_TAR" ] && [ "$(stat -c %s "$SEED_TAR")" -ge 102400 ]; } || world_backup_seed
        selfheal
        [ -f "$RAW/summary.csv" ] || \
            echo "idx,arm,t_burst_s,cpu_burst_s,done_s,arm_validation" >"$RAW/summary.csv"
        run_one "$idx" "$arm"
        exit $?
    fi
    # ---- JFR mechanism-proof one-mode: `run_worldgen_ab.sh jfr` (non-timed)
    if [ "${1:-}" = jfr ]; then
        [ -n "$(server_pid)" ] && { log "REFUSING: server already running"; exit 1; }
        lock_acquire_inline
        { [ -s "$SEED_TAR" ] && [ "$(stat -c %s "$SEED_TAR")" -ge 102400 ]; } || world_backup_seed
        selfheal
        [ -f "$RAW/jfrproof_done" ] && { log "JFR proof already done"; exit 0; }
        run_jfr_proof
        exit $?
    fi
    # ---- full-series mode (background; NOTE: fragile in this sandbox, kept for
    #      environments without the reaper; the one-mode is the proven driver)
    [ -n "$(server_pid)" ] && { log "REFUSING: server already running"; exit 1; }
    lock_on; sleep 1
    world_backup_seed          # MUST precede any selfheal/restore (INCIDENT guard)
    selfheal
    [ -f "$RAW/summary.csv" ] || \
        echo "idx,arm,t_burst_s,cpu_burst_s,done_s,arm_validation" >"$RAW/summary.csv"
    local skip; skip=$(runs_done)
    [ "$skip" -gt 0 ] && log "RESUME: $skip timed run(s) already recorded — skipping"

    # ABBA blocks, N per arm
    local -a order=()
    local i half=$((N_PER_ARM / 2))
    for ((i = 0; i < half; i++)); do order+=(A B B A); done   # half blocks of 4
    if [ $((N_PER_ARM % 2)) -eq 1 ]; then order+=(A B); fi    # odd tail: A B
    # N=5 -> A B B A | A B (5 per arm, execution-order interleaved)
    log "order: ${order[*]}"

    local idx=1 rc failcnt=0
    for arm in "${order[@]}"; do
        if [ "$idx" -le "$skip" ]; then
            log "run $idx ($arm) already recorded — skip"
            idx=$((idx + 1)); continue
        fi
        selfheal   # defensive: previous run must have left a restored, stopped box
        run_one "$idx" "$arm" || { failcnt=$((failcnt + 1)); log "run $idx ($arm) FAILED"; }
        idx=$((idx + 1))
    done

    if [ ! -f "$RAW/jfrproof_done" ]; then
        run_jfr_proof || log "JFR proof failed (non-fatal for timed series)"
    else
        log "JFR proof already done — skip"
    fi

    # ---- final byte-identical verification
    local ref=$SERVER_DIR/.world_ab_ref_task63
    rm -rf "$ref"; mkdir -p "$ref"
    tar -xzf "$SEED_TAR" -C "$ref"
    if diff -r "$SERVER_DIR/world" "$ref/world" >/dev/null 2>&1; then
        log "WORLD VERIFY: world/ byte-identical to seed"
        echo "world_verify=IDENTICAL" >"$RAW/world_verify.txt"
    else
        log "WORLD VERIFY: DIFF DETECTED (see $RAW/world_verify.txt)"
        { diff -rq "$SERVER_DIR/world" "$ref/world" 2>&1 | head -20; } >"$RAW/world_verify.txt"
    fi
    rm -rf "$ref"

    lock_off
    log "series complete: $failcnt failed runs; summary: $RAW/summary.csv"
}

main "$@"
