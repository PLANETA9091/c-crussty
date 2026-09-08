#!/usr/bin/env bash
# TASK-65 (agent-7625532f, 2026-09-09): worldgen parallelism probe — TASK-62 F3
# follow-up. TASK-62 measured 86% of burst-minute runnable CPU on ONE Paper
# Common Worker (Server thread parked in chunk wait) and flagged per-area
# serialization as a possible ceiling. This probe decides it:
#
#   Arm S: ONE contiguous 16x8 region  -> forceload add 1600 1600 1855 1727
#          (chunks 100..115 x 100..107 = 128 fresh chunks)
#   Arm P: pair of distant 8x8 regions -> forceload add 1600 1600 1727 1727
#          + forceload add -1728 -1728 -1601 -1601 (the exact TASK-62/63 coords)
#
# Hypothesis: if per-area serialization dominates the Paper chunk scheduler, P
# engages more workers in parallel -> shorter wall; if serialization is global,
# S and P are statistically indistinguishable (F3 closes NEGATIVE, no ops lever).
#
# KEY METRIC beyond wall/CPU: per-thread CPU distribution over the burst window,
# sampled fork-free from /proc/<child>/task/*/stat (utime+stime per comm, 1 Hz,
# bash-builtin read — no subprocesses inside the timed window).
#
# Derived harness (run_worldgen_ab.sh lineage; e2e_orchestrate.sh UNTOUCHED):
#   - one-run-per-agent-call driver (`one <idx> <arm>`) — TASK-63 F4 lesson:
#     background process trees are SIGKILL'd by the sandbox reaper here;
#   - /proc/<child>/stat total-CPU-rate completion detector (2 Hz);
#   - seed tar = session anchor created once; world restored after EVERY run;
#     final diff -r verification; fresh-world canary (Done > 22 s aborts);
#   - dormant-only (no CRUSSTY_* involvement) -> no arming gate needed;
#   - JFR excluded (zero-touch timed runs);
#   - inline /home/z/BENCH.lock flock, lifetime = the tool-call lifetime.
set -u

SERVER_DIR=/home/z/server
LOGS_DIR=$SERVER_DIR/logs
LATEST=$LOGS_DIR/latest.log
E2E=/home/z/c-crussty/scripts/e2e_orchestrate.sh
RES=/home/z/c-crussty/bench/e2e/results
RAW=$RES/WORLDGEN_PARALLEL_RAW_2026-09-09
SEED_TAR=$SERVER_DIR/world_par_seed_task65.tar.gz
ACTIVE_RATE=0.5
IDLE_RATE=0.15
IDLE_STREAK=3
BURST_TIMEOUT=420

log() { printf '%s %s\n' "$(date -u +%H:%M:%S)" "$*"; }

TRAP_LOG=/tmp/task65_trap.log
echo "$(date -u +%H:%M:%S) START pid=$$" >>"$TRAP_LOG"
trap 'echo "$(date -u +%H:%M:%S) EXIT rc=$? pid=$$" >>"$TRAP_LOG"' EXIT
trap 'echo "$(date -u +%H:%M:%S) TERM pid=$$" >>"$TRAP_LOG"; exit 143' TERM
trap 'echo "$(date -u +%H:%M:%S) HUP pid=$$" >>"$TRAP_LOG"; exit 129' HUP

lock_acquire_inline() {
    exec 9>>/home/z/BENCH.lock
    flock -n 9 || { log "BENCH.lock held by someone else — refusing"; exit 1; }
}

server_pid() { pgrep -f 'agentpath:[^ ]*libcrussty_runtime\.so' | head -1; }
cpu_ticks()  { awk '{print $14+$15}' "/proc/$1/stat" 2>/dev/null; }

world_backup_seed() {
    local dirs=(world)
    [ -d "$SERVER_DIR/world_nether" ] && dirs+=(world_nether)
    [ -d "$SERVER_DIR/world_the_end" ] && dirs+=(world_the_end)
    rm -f "$SEED_TAR"
    tar -czf "$SEED_TAR" -C "$SERVER_DIR" "${dirs[@]}"
    local sz; sz=$(stat -c %s "$SEED_TAR")
    [ "$sz" -lt 102400 ] && { rm -f "$SEED_TAR"; log "FATAL: seed tar too small (${sz}B)"; exit 1; }
    log "seed tar: $(du -h "$SEED_TAR" | cut -f1) (${#dirs[*]} dirs)"
}

world_restore() {
    if [ ! -s "$SEED_TAR" ] || [ "$(stat -c %s "$SEED_TAR")" -lt 102400 ]; then
        log "FATAL: world_restore without valid seed tar — ABORT"
        exit 1
    fi
    rm -rf "$SERVER_DIR/world" "$SERVER_DIR/world_nether" "$SERVER_DIR/world_the_end"
    tar -xzf "$SEED_TAR" -C "$SERVER_DIR"
    [ -f "$SERVER_DIR/world/level.dat" ] || { log "FATAL: restore produced no level.dat"; exit 1; }
}

selfheal() {
    if [ -n "$(server_pid)" ]; then
        log "selfheal: server running — shutting down"
        "$E2E" shutdown >>"$RAW/selfheal_shutdown.txt" 2>&1
        sleep 2
    fi
    [ -s "$SEED_TAR" ] && world_restore
}

fifo_send() {
    timeout 5 bash -c 'printf "%s\n" "$1" > "$2"' _ "$1" "$FIFO"
}

# One fork-free sample pass over all child threads: appends "tid ticks comm"
# lines to $1. comm may contain spaces — parsed between the outer parentheses.
# NOTE: `set --` clobbers positional params — args captured into locals FIRST
# (bug found on run1_S: the first tid's sample went to a file named by the
# ppid field and the glob died from tid 2 on).
thread_sample() {
    local st line rest tid pid=$1 out=$2
    for st in /proc/$pid/task/*/stat; do
        tid=${st#/proc/$pid/task/}; tid=${tid%/stat}
        IFS= read -r line <"$st" 2>/dev/null || continue
        rest=${line##*) }
        set -- $rest
        # $12 = utime (f14), $13 = stime (f15) in full-stat numbering
        printf '%s %s %s\n' "$tid" "$(( ${12:-0} + ${13:-0} ))" "${line#*(}" | \
            sed 's/) .*//' >>"$out"
    done
}

run_one() { # $1 = idx, $2 = S|P
    local idx=$1
    local arm=$2
    local tag="run${idx}_${arm}"
    log "=== $tag boot (arm $arm)"
    "$E2E" boot >"$RAW/${tag}_boot.txt" 2>&1 || { log "$tag BOOT FAIL"; return 1; }
    local pid; pid=$(server_pid)
    [ -n "$pid" ] || { log "$tag no child pid after boot"; return 1; }

    local done_s
    done_s=$(grep -o 'Done ([0-9.]*s' "$LATEST" 2>/dev/null | head -1 | sed 's/.*(\([0-9.]*\)s/\1/')
    log "$tag done=${done_s}s pid=$pid"
    # Fresh-world canary, refined after a false positive (04:53 boot: Done=32.1s
    # was a SLOW CLASSLOADING phase — spawn lines all showed instant 100%; a
    # genuinely fresh world prints sub-100% "Preparing spawn area" lines).
    if grep -qE 'Preparing spawn area: [0-9]{1,2}%$' "$LATEST" 2>/dev/null; then
        log "FATAL: $tag shows sub-100% spawn gen — FRESH WORLD, aborting"
        "$E2E" shutdown >/dev/null 2>&1; sleep 2; world_restore
        return 1
    fi
    if [ -n "${done_s:-}" ] && awk -v d="$done_s" 'BEGIN{exit !(d>22)}'; then
        log "WARN: $tag Done=${done_s}s > 22s but spawn 100% instant — slow boot, not fresh world (continuing)"
    fi

    sleep 4   # uniform settle

    local fifo; fifo=$(cat "$LOGS_DIR/crussty_e2e.state" 2>/dev/null)
    if [ ! -p "$fifo" ]; then log "$tag no fifo in state file"; "$E2E" shutdown >/dev/null 2>&1; return 1; fi
    FIFO=$fifo

    # ---- T0: send burst geometry (S = one contiguous region; P = distant pair)
    cpu0=$(cpu_ticks "$pid"); t0=$(date +%s.%N)
    local thlog="$RAW/${tag}_threads.txt"; : >"$thlog"
    if [ "$arm" = S ]; then
        fifo_send "forceload add 1600 1600 1855 1727" || log "$tag send rc=$?"
    else
        fifo_send "forceload add 1600 1600 1727 1727"      || log "$tag send1 rc=$?"
        fifo_send "forceload add -1728 -1728 -1601 -1601"  || log "$tag send2 rc=$?"
    fi

    # ---- completion detector + 1 Hz per-thread CPU accumulation
    local trace="$RAW/${tag}_cputrace.txt"; : >"$trace"
    local tprev=$t0 cpuprev=$cpu0 now c dt cores active=0 streak=0 tend=0 cpuend=0 elapsed
    local tick=0
    while :; do
        sleep 0.5
        now=$(date +%s.%N)
        c=$(cpu_ticks "$pid")
        if [ -z "$c" ]; then log "$tag child vanished mid-burst"; break; fi
        dt=$(awk -v a="$now" -v b="$tprev" 'BEGIN{print a-b}')
        cores=$(awk -v x=$((c - cpuprev)) -v d="$dt" 'BEGIN{print x/(d*100)}')
        echo "$now $cores" >>"$trace"
        tprev=$now; cpuprev=$c
        tick=$((tick + 1)); [ $((tick % 2)) -eq 0 ] && thread_sample "$pid" "$thlog"
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
            log "$tag BURST TIMEOUT — aborting window"
            tend=$now; cpuend=$c; break
        fi
    done
    [ "$tend" = 0 ] && { tend=$(date +%s.%N); cpuend=$(cpu_ticks "$pid"); }
    local t_burst cpu_burst
    t_burst=$(awk -v a="$tend" -v b="$t0" 'BEGIN{printf "%.2f", a-b}')
    cpu_burst=$(awk -v a="${cpuend:-0}" -v b="${cpu0:-0}" 'BEGIN{printf "%.2f", (a-b)/100}')

    # ---- relay-alive proof + teardown
    fifo_send "forceload query" && sleep 3 && tail -c 2000 "$LATEST" >"$RAW/${tag}_forceload_query.txt" 2>/dev/null
    fifo_send "forceload remove all" || true
    sleep 2
    grep -c 'Watchdog' "$LATEST" 2>/dev/null | head -1 >"$RAW/${tag}_watchdog_count.txt"
    "$E2E" shutdown >"$RAW/${tag}_shutdown.txt" 2>&1
    sleep 2
    world_restore
    printf '%s,%s,%s,%s,%s\n' "$idx" "$arm" "$t_burst" "$cpu_burst" "${done_s:-NA}" \
        >>"$RAW/summary.csv"
    log "$tag DONE t_burst=${t_burst}s cpu_burst=${cpu_burst}s"
    return 0
}

main() {
    mkdir -p "$RAW"
    if [ "${1:-}" = one ]; then
        local idx=$2
        local arm=$3
        [ -n "$(server_pid)" ] && { log "REFUSING: server already running"; exit 1; }
        lock_acquire_inline
        { [ -s "$SEED_TAR" ] && [ "$(stat -c %s "$SEED_TAR")" -ge 102400 ]; } || world_backup_seed
        selfheal
        [ -f "$RAW/summary.csv" ] || echo "idx,arm,t_burst_s,cpu_burst_s,done_s" >"$RAW/summary.csv"
        run_one "$idx" "$arm"
        exit $?
    fi
    if [ "${1:-}" = verify ]; then
        local ref=$SERVER_DIR/.world_par_ref_task65
        rm -rf "$ref"; mkdir -p "$ref"; tar -xzf "$SEED_TAR" -C "$ref"
        if diff -r "$SERVER_DIR/world" "$ref/world" >/dev/null 2>&1; then
            log "WORLD VERIFY: IDENTICAL"; echo "world_verify=IDENTICAL" >"$RAW/world_verify.txt"
        else
            diff -rq "$SERVER_DIR/world" "$ref/world" 2>&1 | head -20 >"$RAW/world_verify.txt"
            log "WORLD VERIFY: DIFF — see RAW/world_verify.txt"
        fi
        rm -rf "$ref"; exit 0
    fi
    log "usage: run_worldgen_parallel.sh one <idx> <S|P> | verify"
    exit 2
}

main "$@"
