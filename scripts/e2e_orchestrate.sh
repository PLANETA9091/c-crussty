#!/usr/bin/env bash
# SESSION-007/S7-2: live-server E2E orchestration for /home/z/server (Purpur 1.21.10
# + CRUSSTY runtime + c-crussty module). PREP ONLY — never executed by its author.
#
# Modes:
#   boot       backup latest.log, start server detached (setsid, fifo stdin),
#              wait for "Done (" boot-complete marker (latest.log).
#   verify     PASS/FAIL checklist: armed-strings, scans-avoided counters,
#              kernel-policy gate, batch init, live proof. Non-zero on any FAIL.
#   hotreload  SIGUSR1 the child JVM, wait for "reloaded (dlopen'd fresh + re-init)"
#              + "hook purge" markers, report.
#   shutdown   graceful stop ("stop" via launcher stdin fifo; fallback kill -TERM,
#              30s grace), report final/native lines.
#   all        boot -> verify -> hotreload -> shutdown (sequential).
#
# Boot command discovered (previous boot, logs/console.log 2026-09-07):
#   cd /home/z/server && java -jar launcher/launcher.jar
# launcher.jar resolves java.home of ITS jvm and spawns the child:
#   java -agentpath:/home/z/server/libcrussty_runtime.so=modules=...;versions=...;kernel=purpur-1.21.10.jar
#        -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=...
#        -jar versions/purpur-1.21.10.jar --nogui
# It forwards stdin -> server stdin (so "stop" works via fifo) and tees
# stdout/stderr into logs/server.log + console.log. CRUSSTY *plugin* markers are
# eprintln(stderr) => console.log/server.log, NOT logs/latest.log (log4j only).
#
# Env overrides:
#   SERVER_DIR        (default /home/z/server)
#   JAVA_BIN          (default /home/z/jdk21/bin — previous boot used system java)
#   CRUSSTY_BOOT_CMD  full override of the boot command (default documented above)
#   BOOT_TIMEOUT      (default 300 s)
#   E2E_LOG           (default $SERVER_DIR/logs/crussty_e2e_boot.log)
#   Improved-noise patch needs CRUSSTY_NATIVE_IMPROVED_NOISE=1 exported before boot.
#
# Courtesy guard: refuses boot/all while /tmp/crussty_bench.lock exists
# (concurrent timed benchmarks own the box).
set -u

SERVER_DIR="${SERVER_DIR:-/home/z/server}"
JAVA_BIN="${JAVA_BIN:-/home/z/jdk21/bin}"
BOOT_TIMEOUT="${BOOT_TIMEOUT:-300}"
E2E_LOG="${E2E_LOG:-$SERVER_DIR/logs/crussty_e2e_boot.log}"
LOGS_DIR="$SERVER_DIR/logs"
LATEST="$LOGS_DIR/latest.log"
PID_FILE="$LOGS_DIR/crussty_e2e.pid"
STATE_FILE="$LOGS_DIR/crussty_e2e.state"
FIFO="$LOGS_DIR/crussty_e2e_stdin.$$"
OFFS_FILE="$LOGS_DIR/crussty_e2e.offsets"
RELOAD_WAIT="${RELOAD_WAIT:-60}"

log()  { printf '%s\n' "[e2e] $*"; }
fail() { printf '%s\n' "[e2e] FAIL: $*" >&2; exit 1; }

# Candidate logs that contain [crussty-*] markers (console.log survives across
# boots; the boot log is per-session; latest.log only carries log4j lines).
marker_files() {
    for f in "$E2E_LOG" "$LOGS_DIR/console.log" "$LOGS_DIR/server.log"; do
        [ -f "$f" ] && printf '%s\n' "$f"
    done
}

# Boot-time line offsets per candidate log: everything at/below the offset is a
# PREVIOUS session (console.log/server.log survive across boots and would else
# false-PASS the verify table with stale markers). E2E_LOG/latest.log are fresh
# (truncated / moved away at boot) so their offsets are 0/absent.
capture_offsets() {
    : > "$OFFS_FILE"
    local f
    for f in "$E2E_LOG" "$LATEST" "$LOGS_DIR/console.log" "$LOGS_DIR/server.log"; do
        if [ -f "$f" ]; then printf '%s:%s\n' "$f" "$(wc -l < "$f")" >> "$OFFS_FILE"
        else printf '%s:0\n' "$f" >> "$OFFS_FILE"; fi
    done
}

grep_markers() { # grep_markers <ERE>  -> first matching SESSION line across marker files
    local pat="$1" f off m
    if [ -f "$OFFS_FILE" ]; then
        while IFS=: read -r f off; do
            [ -n "$f" ] && [ -f "$f" ] || continue
            m="$(tail -n +"$((off+1))" -- "$f" 2>/dev/null | grep -h -m1 -E "$pat" 2>/dev/null)"
            [ -n "$m" ] && { printf '%s\n' "$m"; return 0; }
        done < "$OFFS_FILE"
    else
        marker_files | xargs -r grep -h -m1 -E "$1" 2>/dev/null | head -1
    fi
}

server_pid() { # child JVM = the one carrying the -agentpath runtime
    pgrep -f 'agentpath:[^ ]*libcrussty_runtime\.so' | head -1
}

guard_bench_lock() {
    # The lock FILE is a permanent fixture (flock leaves it behind) — mere
    # existence says nothing. Probe: non-blocking acquire fails => a timed
    # benchmark holds the box right now.
    if ! flock -n /tmp/crussty_bench.lock true 2>/dev/null; then
        fail "/tmp/crussty_bench.lock HELD — timed benchmarks own the box; boot refused"
    fi
}

do_boot() {
    guard_bench_lock
    [ -d "$SERVER_DIR" ] || fail "server dir missing: $SERVER_DIR"
    [ -f "$SERVER_DIR/launcher/launcher.jar" ] || fail "launcher.jar missing"
    [ -f "$SERVER_DIR/libcrussty_runtime.so" ] || fail "libcrussty_runtime.so missing"
    pgrep -f 'agentpath:[^ ]*libcrussty_runtime\.so' >/dev/null && \
        fail "server already running ($(server_pid))"
    [ -p "$FIFO" ] || mkfifo "$FIFO" || fail "mkfifo $FIFO failed"

    # mv (not cp): latest.log must not carry the previous boot's "Done (" line,
    # else the wait loop below returns instantly. Fresh latest.log is created by
    # log4j on startup; the moved file is the backup.
    mv "$LATEST" "$LATEST.pre-e2e.$$" 2>/dev/null || log "no latest.log to backup"
    : > "$E2E_LOG"
    capture_offsets

    if [ -n "${CRUSSTY_BOOT_CMD:-}" ]; then
        log "booting via CRUSSTY_BOOT_CMD override"
    else
        CRUSSTY_BOOT_CMD="exec '$JAVA_BIN/java' -jar '$SERVER_DIR/launcher/launcher.jar'"
        log "booting default: cd $SERVER_DIR && java -jar launcher/launcher.jar"
    fi
    ( cd "$SERVER_DIR" && eval "$CRUSSTY_BOOT_CMD" < "$FIFO" >> "$E2E_LOG" 2>&1 ) &
    local lpid=$!
    echo "$lpid" > "$PID_FILE"
    echo "$FIFO" > "$STATE_FILE"
    exec 9<> "$FIFO"   # hold fifo writer open so the server stdin never EOFs
    log "launcher pid $lpid (fifo $FIFO); waiting up to ${BOOT_TIMEOUT}s for 'Done ('"

    local deadline=$(( $(date +%s) + BOOT_TIMEOUT ))
    while :; do
        grep -q 'Done (' "$LATEST" 2>/dev/null && { log "boot complete: $(grep -h -m1 'Done (' "$LATEST")"; return 0; }
        kill -0 "$lpid" 2>/dev/null || fail "launcher exited early; tail: $(tail -c 1200 "$E2E_LOG")"
        [ "$(date +%s)" -ge "$deadline" ] && \
            fail "timeout after ${BOOT_TIMEOUT}s; latest tail: $(tail -c 800 "$LATEST" 2>/dev/null)"
        sleep 5
    done
}

do_verify() {
    local fails=0
    printf '%-34s %-6s %s\n' "MARKER" "STATE" "CAPTURED VALUE"
    printf '%s\n' "--------------------------------------------------------------------------------"
    ck() { # ck <name> <ERE>  — PASS if any marker file matches
        local v; v="$(grep_markers "$2")"
        if [ -n "$v" ]; then printf '%-34s %-6s %s\n' "$1" "PASS" "$v"
        else printf '%-34s %-6s %s\n' "$1" "FAIL" "(not found)"; fails=$((fails+1)); fi
    }
    ck_opt() { # status-only marker (legitimately absent in some configs)
        local v; v="$(grep_markers "$2")"
        printf '%-34s %-6s %s\n' "$1" "$([ -n "$v" ] && echo INFO || echo ABSENT)" "$v"
    }
    ck_bad() { # PASS when absent (policy refusals / bypass are regressions)
        local v; v="$(grep_markers "$2")"
        if [ -z "$v" ]; then printf '%-34s %-6s %s\n' "$1" "PASS" "(absent — healthy)"
        else printf '%-34s %-6s %s\n' "$1" "FAIL" "$v"; fails=$((fails+1)); fi
    }
    ck "runtime loaded"          'crussty-runtime\] v[0-9.]+ loaded \(options:'
    ck "cplugin_init injected"   'cplugin_init: injecting Crussty CE native surface'
    ck "module init rc=0"        'crussty-runtime\] module crussty -> init rc=0'
    ck "pipeline ready"          'pipeline ready: [0-9]+ module hook'
    ck "SIGUSR1 trigger armed"   'SIGUSR1 reload trigger armed'
    ck "native surface live"     'native surface live: [0-9]+ bridge classes'
    ck "batch kernels resolved"  'batch: [0-9]+ kernels resolved'
    ck "area_map armed"          'area_map: hook armed, retransform rc=0'
    ck "area_map scans-avoided"  'area_map: sighting feed: [0-9]+ full class-heap scans avoided'
    ck "live proof nativeCheck"  'live proof: normalNoise.nativeCheck\(\) = 1'
    ck "boot complete (Done)"    'Done \([0-9.]+s\)!'
    ck_bad "batch policy REFUSED" 'batch: kernel .* REFUSED by kernel-policy'
    ck_bad "WIRE REFUSED"        'kernel-policy:.*REFUSED'
    ck_bad "policy BYPASSED"     'kernel-policy:.*BYPASSED'
    ck_bad "symbols unresolved>0" 'native surface live: [0-9]+ bridge classes, [0-9]+ natives registered \([1-9][0-9]* symbols unresolved\)'
    ck_opt "improved_noise armed" 'improved_noise: hook armed, retransform rc=0'
    ck_opt "improved_noise dormant" 'improved_noise: dormant \(set CRUSSTY_NATIVE_IMPROVED_NOISE=1\)'
    ck_opt "improved_noise scans-avoided" 'improved_noise: sighting feed: [0-9]+ full class-heap scans avoided'
    ck_opt "kernel_pref old-bind" 'kernel_pref: .* bound to old kernel'
    printf '%s\n' "--------------------------------------------------------------------------------"
    [ "$fails" -eq 0 ] && { log "verify: ALL PASS"; return 0; }
    log "verify: $fails FAIL(s)"; return 1
}

do_hotreload() {
    local pid; pid="$(server_pid)"
    [ -n "$pid" ] || fail "no server JVM found (pattern: agentpath:*libcrussty_runtime.so)"
    # NOTE: reload only takes effect if modules/crussty/libcrussty.so was replaced
    # on disk (new inode via mv) BEFORE the trigger — mirror engine e2e.sh.
    log "sending SIGUSR1 to $pid"
    kill -USR1 "$pid" || fail "kill -USR1 $pid failed"
    local deadline=$(( $(date +%s) + RELOAD_WAIT ))
    while [ "$(date +%s)" -lt "$deadline" ]; do
        if grep_markers "reloaded \(dlopen.d fresh \+ re-init\)" | grep -q .; then
            printf '%-34s %-6s %s\n' "hotreload reloaded" "PASS" "$(grep_markers "reloaded \(dlopen.d fresh")"
            local purge; purge="$(grep_markers 'hook purge')"
            [ -n "$purge" ] && printf '%-34s %-6s %s\n' "hook purge" "PASS" "$purge" \
                || printf '%-34s %-6s %s\n' "hook purge" "WARN" "(not seen yet)"
            local skip; skip="$(grep_markers 'reload skipped')"
            [ -z "$skip" ] && { log "hotreload: OK (module reloaded, hooks purged)"; return 0; }
            fail "reload skipped: $skip"
        fi
        kill -0 "$pid" 2>/dev/null || fail "server died during hot-reload"
        sleep 2
    done
    fail "no 'reloaded (dlopen'd fresh + re-init)' marker within ${RELOAD_WAIT}s"
}

do_shutdown() {
    local pid; pid="$(server_pid)"
    [ -n "$pid" ] || { log "no server running — nothing to stop"; return 0; }
    log "stopping server pid $pid"
    local stop_fifo=""
    [ -n "${STOP_FIFO_FD9:-}" ] && stop_fifo="fd9"   # set by `all` (fd 9 open)
    [ -z "$stop_fifo" ] && [ -f "$STATE_FILE" ] && read -r stop_fifo < "$STATE_FILE"
    if [ "$stop_fifo" = "fd9" ]; then
        echo "stop" >&9 && log "sent 'stop' via launcher stdin fifo (fd9)" || true
    elif [ -p "${stop_fifo:-}" ]; then
        # no reader-open race: timeout guards the blocking open for write
        timeout 5 bash -c 'echo stop > "$1"' _ "$stop_fifo" \
            && log "sent 'stop' via launcher stdin fifo ($stop_fifo)" \
            || log "fifo write failed/timeout — falling back to SIGTERM"
    else
        log "no stdin fifo known — falling back to SIGTERM"
    fi
    local i=0
    while kill -0 "$pid" 2>/dev/null && [ "$i" -lt 30 ]; do sleep 1; i=$((i+1)); done
    if kill -0 "$pid" 2>/dev/null; then
        log "graceful stop timed out after 30s — sending SIGTERM"
        kill -TERM "$pid" 2>/dev/null || true
        sleep 5
        kill -0 "$pid" 2>/dev/null && fail "server pid $pid still alive after SIGTERM"
    fi
    log "server exited; final crussty/native lines:"
    grep -h -E 'crussty|Done|exited' "$LATEST" "$E2E_LOG" 2>/dev/null | tail -8
    exec 9>&- 2>/dev/null || true
    rm -f "$PID_FILE" "$STATE_FILE" "$FIFO" "$OFFS_FILE"
}

main() {
    local mode="${1:-all}"
    case "$mode" in
        boot)      do_boot ;;
        verify)    do_verify ;;
        hotreload) do_hotreload ;;
        shutdown)  do_shutdown ;;
        all)       guard_bench_lock; do_boot || exit 1; do_verify || VERIFY_RC=1
                   STOP_FIFO_FD9=1 do_hotreload || HOTRELOAD_RC=1; do_shutdown
                   exit $(( ${VERIFY_RC:-0} + ${HOTRELOAD_RC:-0} )) ;;
        *) fail "usage: $0 {boot|verify|hotreload|shutdown|all}" ;;
    esac
}
main "$@"
