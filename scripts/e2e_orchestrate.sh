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
#
# STDIN FORWARDING — FIXED (TASK-59, 2026-09-09, agent-7625532f; finding bench/e2e/results/JFR_PROFILE_2026-09-09.md §6 I1):
#   Root cause (forensics STDIN_FORENSICS_2026-09-09.md): the launcher.jar relay thread
#   ("launcher-stdin") is correct code (read→write→flush per chunk) but exits on fifo EOF —
#   and the boot invocation's `exec 9<> $FIFO` write-end hold DIES with the boot bash process
#   (boot returns right after "Done ("). From that moment every writer is gone, the relay's
#   read() returns 0, the thread exits silently, and commands written later to the fifo are
#   never read by anyone. Live proof (2026-09-09 boot): launcher thread dump has NO
#   launcher-stdin thread; injecting "list" straight into the child stdin pipe
#   (echo list > /proc/<launcher>/fd/<child-stdin-write-end>) executed instantly — child
#   console leg 100% healthy, only the relay lifeline was missing.
#   Fix: do_boot spawns a detached holder (setsid bash, fd9 <> fifo, sleep loop) that keeps
#   a write end open for the launcher's lifetime; do_shutdown/do_boot kill holders (stale
#   cleanup) and scrub old fifo files. S7-13's "graceful stop 143" was the SIGTERM fallback
#   firing because this lifeline was dead, not the fifo write itself.
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

# ALL SESSION-scoped matching lines (grep_markers returns only the first): the
# attempt-frequency row must aggregate EVERY per-attempt capture line in the
# boot's scope to derive the max attempt k, not merely the first one. Same
# offset logic as grep_markers; dedupe is left to the caller (console.log and
# server.log are teed copies of the same stream, so identical lines appear in
# both and must not double-count).
grep_markers_all() { # grep_markers_all <ERE> -> all matching SESSION lines across marker files
    local pat="$1" f off
    if [ -f "$OFFS_FILE" ]; then
        while IFS=: read -r f off; do
            [ -n "$f" ] && [ -f "$f" ] || continue
            tail -n +"$((off+1))" -- "$f" 2>/dev/null | grep -h -E "$pat" 2>/dev/null
        done < "$OFFS_FILE"
    else
        marker_files | xargs -r grep -h -E "$1" 2>/dev/null
    fi
}

# Module .so marker capability: the plugin can only emit a log marker if the
# marker's string constant is embedded in the deployed module binary. A build
# predating a marker's code can NEVER produce the line, so its absence from the
# logs is then EXPECTED — not a regression (S7-5 false alarm: "dormant-строка
# ABSENT (маркер новее .so)"). Read-only probe: fixed substring via strings(1),
# grep -aF fallback when binutils is absent.
so_has_marker() { # so_has_marker <fixed substring> -> rc 0 when embedded in the module .so
    local so="$SERVER_DIR/modules/crussty/libcrussty.so"
    [ -f "$so" ] || return 1
    if command -v strings >/dev/null 2>&1; then
        strings "$so" 2>/dev/null | grep -qF -- "$1"
    else
        grep -aqF -- "$1" "$so" 2>/dev/null
    fi
}

# Hook life-sign EREs for ck_cap(): every log line the hook's activation/serve
# pipeline can emit (eprintln! sites in src/improved_noise.rs / src/area_map.rs).
# ck_cap uses them to tell "hook alive, the row's own line is config-gated or
# not yet emitted" (INFO) apart from "marker-capable .so but ZERO life-signs"
# (FAIL = dead byte-hook pipeline).
#   improved_noise: armed/dormant are env-gated (CRUSSTY_NATIVE_IMPROVED_NOISE).
#   area_map: arms on EVERY boot, but asynchronously — poller with negative
#   backoff + forced Class.forName, so its lines can land well after Done;
#   any life-sign means late arming, not a dead pipeline.
# Capture-phase shapes (S7-8 hardening, module 625c564): when the kernel class
# predates the hook, the activation worker logs "class predates hook, capturing
# current bytes via no-op retransform" then per-attempt "capture retransform
# rc=<n> (attempt k)" lines and, on the fallback path, "retransform capture
# empty after 3 attempts…" / "resource-stream capture <n> bytes" — a boot in
# that phase has NO armed/dormant/pristine line yet, so without these tokens
# the capable-.so life-sign check would misread a live capture as a dead
# pipeline. The trailing '|improved_noise: .*dormant' mirrors AMAP_LIFESIGNS:
# several dormant verdicts carry the token mid-line ("{NOISE_CLASS} not loaded
# within 180s, hook stays dormant", "no original bytes captured even after
# retransform …, hook stays dormant"), so it can not sit in the anchored group.
NOISE_LIFESIGNS='improved_noise: (hook armed|hook serve|pristine sighting|forcing kernel load|Class\.forName|self-test|dormant|class predates hook|capture retransform rc=|retransform capture empty|resource-stream capture)|improved_noise: .*dormant'
# NOTE the trailing '|area_map: .*dormant' — the 180s-timeout line carries
# "dormant" mid-line ("... not loaded within 180s, hook stays dormant"), so the
# token can not sit in the prefix-anchored group; SELF-TEST FAIL is all-caps.
AMAP_LIFESIGNS='area_map: (hook armed|sighting feed|patched|patch failed|forcing kernel load|force load|Class\.forName|[Ss]elf-test|SELF-TEST|defined|define_class|helper definition aborted)|area_map: .*dormant'

server_pid() { # child JVM = the one carrying the -agentpath runtime
    pgrep -f 'agentpath:[^ ]*libcrussty_runtime\.so' | head -1
}

# --- stdin lifeline (TASK-59): the launcher relay dies on fifo EOF; keep one
# --- write end open in a detached holder so the relay thread stays alive.
start_stdin_holder() {
    # No final exec: the cmdline must keep the fifo path visible for kill_stdin_holders.
    # TASK-95 S7-36 fd hygiene: the holder must NOT inherit the bench rig's
    # flock fd (200) — inherited holders kept BENCH.lock locked after clean
    # rig exits (leak class seen 4x this session). Close every fd >9.
    local _hfd _hclose=""
    for _hfd in /proc/self/fd/*; do
        [ "$(basename "$_hfd")" -gt 9 ] 2>/dev/null && _hclose="$_hclose $(basename "$_hfd")>&-"
    done
    setsid bash -c "exec $_hclose 9<> '$FIFO'; while :; do sleep 3600; done" </dev/null >/dev/null 2>&1 &
    log "stdin holder started (write-end lifeline on $FIFO)"
}

kill_stdin_holders() { # kill every holder of any crussty_e2e_stdin fifo (session is single-tenant)
    local hp cp
    pgrep -f 'crussty_e2e_stdin.*sleep 3600' 2>/dev/null | while read -r hp; do
        # S7-31 fix: the holder's `sleep 3600` child has no fifo path in its own cmdline,
        # survives the parent kill, and stays orphaned while HOLDING INHERITED FDS
        # (incl. any flock'd BENCH-MUTEX fd) -> next session deadlocks on the mutex.
        # Kill children FIRST, then the holder (observed: 6 orphaned sleeps in one day).
        for cp in $(pgrep -P "$hp" 2>/dev/null); do
            kill "$cp" 2>/dev/null || true
        done
        kill "$hp" 2>/dev/null && log "killed stdin holder pid $hp" || true
    done
}

scrub_stale_stdin() { # holders first, then their fifo leftovers (22 accumulated pre-TASK-59)
    kill_stdin_holders
    local f
    for f in "$LOGS_DIR"/crussty_e2e_stdin.*; do
        [ -e "$f" ] || return 0
        rm -f "$f" && log "scrubbed stale fifo $f"
    done
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
    scrub_stale_stdin            # dead sessions' holders + fifo leftovers (TASK-59)
    [ -p "$FIFO" ] || mkfifo "$FIFO" || fail "mkfifo $FIFO failed"
    start_stdin_holder           # relay lifeline: must outlive THIS invocation (TASK-59)

    # mv (not cp): latest.log must not carry the previous boot's "Done (" line,
    # else the wait loop below returns instantly. Fresh latest.log is created by
    # log4j on startup; the moved file is the backup.
    mv "$LATEST" "$LATEST.pre-e2e.$$" 2>/dev/null || log "no latest.log to backup"
    : > "$E2E_LOG"
    capture_offsets

    if [ -n "${CRUSSTY_BOOT_CMD:-}" ]; then
        log "booting via CRUSSTY_BOOT_CMD override"
    else
        # TASK-88 (S7-32, owner directive: boot <1s, engine repo authorized): default boot
        # = DIRECT purpur jar (bundler bypass; launcher only resolved java.home and spawned
        # a child with exactly this cmdline — byte-verified against hs_err "Command Line")
        # + AppCDS v2 archive when present (S7-31: -18.4% boot, p=0.0079, safe degradation
        # proven both directions: missing archive -> baseline-speed boot, zero crash).
        # NO server.properties / paper / gameplay config touched — JVM flags are CLI-only
        # and replicate the launcher child's own production flags 1:1.
        local CDS_FLAG=""
        local JSA="$SERVER_DIR/crussty_boot.jsa"
        if [ -s "$JSA" ]; then
            CDS_FLAG="-XX:SharedArchiveFile=$JSA"
            log "AppCDS v2 default: mapping $JSA ($(du -h "$JSA" | cut -f1))"
        else
            log "AppCDS archive absent -> baseline-speed boot (safe degradation)"
        fi
        CRUSSTY_BOOT_CMD="exec '$JAVA_BIN/java' $CDS_FLAG \
'-agentpath:$SERVER_DIR/libcrussty_runtime.so=modules=$SERVER_DIR/modules;versions=$SERVER_DIR/versions;kernel=purpur-1.21.10.jar' \
-Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER_DIR \
-jar '$SERVER_DIR/versions/purpur-1.21.10.jar' --nogui"
        log "booting default: direct purpur jar (bundler bypass) + AppCDS-v2-if-present + agent"
    fi
    # TASK-95 S7-36: the boot subshell (and thus the server JVM) must not
    # inherit the rig's flock fd either — a live server holding BENCH.lock
    # made holder-purges kill live boots (S7-32/37 incident class).
    ( cd "$SERVER_DIR" && { _bfd=""; _bclose=""
        for _bfd in /proc/self/fd/*; do
            [ "$(basename "$_bfd")" -gt 9 ] 2>/dev/null && _bclose="$_bclose $(basename "$_bfd")>&-"
        done
        eval "exec $_bclose" 2>/dev/null
        eval "$CRUSSTY_BOOT_CMD" < "$FIFO" >> "$E2E_LOG" 2>&1; } ) &
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
    # Attempt-frequency monitor (Task 2-b, S7-10): implements the accepted-risk
    # watch item from E2E_LIVE_2026-09-08.md ADDENDUM S7-9. The S7-8 hardening
    # (module 625c564) retries the no-op-retransform capture up to 3 attempts
    # ("capture retransform rc=<rc> (attempt k)", 250ms apart) and then falls
    # back to the kernel-loader resource stream. SESSION 009 boot#3 showed the
    # residual delivery race is real: attempt 1 dropped, attempt 2 delivered.
    # This row makes k visible per boot so multiple boots accumulate a frequency
    # baseline — a rising k>1 rate means the race is worsening and earns the
    # JVMTI-level look the addendum reserved. FAIL only for real breakage.
    ck_noise_attempts() { # ck_noise_attempts <name> — capture attempt>1 frequency row
        local name="$1" dead attempts k nlines ev empty rs armed served
        # Ultimate failure first (highest priority): bytes WERE captured but the
        # kernel loader never registered — the hook stays dormant forever, the
        # patch pipeline is dead regardless of how cleanly capture went.
        dead="$(grep_markers 'improved_noise: no kernel loader captured')"
        if [ -n "$dead" ]; then
            printf '%-34s %-6s %s\n' "$name" "FAIL" "$dead (dead patch pipeline)"; fails=$((fails+1)); return
        fi
        # All capture-attempt lines in the session scope (deduped across the teed
        # console.log/server.log copies). The full-line ERE is anchored on
        # "capture retransform rc=" so the UNRELATED force-load kick lines
        # ("forcing kernel load ... (attempt 1..4)") can never poison k.
        attempts="$(grep_markers_all 'improved_noise: capture retransform rc=[0-9]+ \(attempt [0-9]+\)' | awk '!seen[$0]++')"
        if [ -n "$attempts" ]; then
            k="$(printf '%s\n' "$attempts" | sed -n 's/.*attempt \([0-9][0-9]*\)).*/\1/p' | sort -n | tail -1)"
            nlines="$(printf '%s\n' "$attempts" | grep -c .)"
            ev="$(printf '%s\n' "$attempts" | grep -E "attempt ${k}\)" | tail -1)"
            empty="$(grep_markers 'improved_noise: retransform capture empty after 3 attempts')"
            if [ "$k" -le 2 ] || [ -z "$empty" ]; then
                # No "empty after 3" line => the last logged attempt DID deliver
                # (the capture loop breaks on delivery), so "retry recovered" is
                # literal for k<=3 here.
                if [ "$k" -eq 1 ]; then
                    printf '%-34s %-6s %s\n' "$name" "PASS" "capture attempt k=1 (clean first-attempt delivery; $nlines attempt line(s) | $ev)"
                else
                    printf '%-34s %-6s %s\n' "$name" "INFO" "capture attempt k=$k — delivery race hit, retry recovered (accepted-risk; monitor frequency; $nlines attempt line(s) | $ev)"
                fi
                return
            fi
            # k==3 AND "empty after 3 attempts" seen: retransform delivery never
            # carried the bytes — recovery hinged on the resource-stream fallback.
            rs="$(grep_markers 'improved_noise: resource-stream capture [0-9]+ bytes')"
            if [ -n "$rs" ]; then
                printf '%-34s %-6s %s\n' "$name" "INFO" "capture attempt k=3 — retransform delivery fully raced out, resource-stream fallback recovered (accepted-risk; monitor frequency | $rs)"
            else
                printf '%-34s %-6s %s\n' "$name" "INFO" "capture attempt k=3 — all retransform attempts raced out, resource-stream fallback absent/failed — hook stays dormant (fail-safe; see armed/dormant rows)"
            fi
            return
        fi
        rs="$(grep_markers 'improved_noise: resource-stream capture [0-9]+ bytes')"
        if [ -n "$rs" ]; then
            printf '%-34s %-6s %s\n' "$name" "INFO" "capture via resource-stream fallback (retransform delivery fully raced out) | $rs"
            return
        fi
        # Pristine-load path: armed with no capture phase means the kernel class
        # loaded AFTER hook registration, so the original bytes arrived through
        # ClassFileLoadHook delivery at class-load time ("pristine sighting" at
        # load, replayed later as "hook serve") — no retransform capture needed.
        armed="$(grep_markers 'improved_noise: hook armed, retransform rc=[0-9]+')"
        served="$(grep_markers 'improved_noise: (pristine sighting|hook serve)')"
        if [ -n "$armed" ] && [ -n "$served" ]; then
            printf '%-34s %-6s %s\n' "$name" "INFO" "pristine-load path (bytes via ClassFileLoadHook at class-load time, no capture phase needed) | $served"
            return
        fi
        if ! so_has_marker 'improved_noise: capture retransform rc='; then
            printf '%-34s %-6s %s\n' "$name" "INFO" "(expected-absent — module .so predates capture markers)"
            return
        fi
        printf '%-34s %-6s %s\n' "$name" "INFO" "(no capture-phase evidence this session — dormant boot or armed line not yet emitted)"
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
    ck_cap() { # ck_cap <name> <ERE> <so-substring> <lifesigns-ERE> [mode] — capability-aware hook row:
        #   line found in logs                        -> INFO
        #     mode "pass":                            -> PASS (core row: hook arms on every boot)
        #     mode "rc":   rc=0                       -> PASS (armed retransform succeeded)
        #                  rc!=0                      -> FAIL (retransform itself failed — honest
        #                       capture; the old rc=0-pinned ERE shoved a real failure into
        #                       the generic INFO branch where the actual rc was invisible)
        #   .so lacks the marker constant             -> INFO "expected-absent (pre-marker .so)"
        #   .so capable + <hook> life-sign lines      -> INFO (hook alive; the row's line is
        #        config-gated — armed/dormant are mutually exclusive — or not yet
        #        emitted: armed lands post-Done on the activation worker)
        #   .so capable + ZERO <hook> life-sign lines -> FAIL (dead byte-hook pipeline,
        #        the S7-5 crash-boot signature — a real regression)
        local v; v="$(grep_markers "$2")"
        if [ -n "$v" ]; then
            if [ "${5:-}" = "rc" ]; then
                local rc="${v##*retransform rc=}"; rc="${rc%%[!0-9]*}"
                if [ "$rc" = "0" ]; then printf '%-34s %-6s %s\n' "$1" "PASS" "$v"
                else printf '%-34s %-6s %s\n' "$1" "FAIL" "$v (retransform rc!=0 — patch NOT applied)"; fails=$((fails+1)); fi
            elif [ "${5:-}" = "pass" ]; then
                printf '%-34s %-6s %s\n' "$1" "PASS" "$v"
            else
                printf '%-34s %-6s %s\n' "$1" "INFO" "$v"
            fi
        elif ! so_has_marker "$3"; then
            printf '%-34s %-6s %s\n' "$1" "INFO" "(expected-absent — module .so predates marker)"
        elif grep_markers "$4" | grep -q .; then
            printf '%-34s %-6s %s\n' "$1" "INFO" "(absent — hook alive, line config-gated/not yet emitted)"
        else
            printf '%-34s %-6s %s\n' "$1" "FAIL" "(marker-capable .so, 0 hook life-sign lines)"; fails=$((fails+1))
        fi
    }
    ck "runtime loaded"          'crussty-runtime\] v[0-9.]+ loaded \(options:'
    ck "cplugin_init injected"   'cplugin_init: injecting Crussty CE native surface'
    ck "module init rc=0"        'crussty-runtime\] module crussty -> init rc=0'
    ck "pipeline ready"          'pipeline ready: [0-9]+ module hook'
    ck "SIGUSR1 trigger armed"   'SIGUSR1 reload trigger armed'
    ck "native surface live"     'native surface live: [0-9]+ bridge classes'
    ck "batch kernels resolved"  'batch: [0-9]+ kernels resolved'
    # NOTE area_map rows: capability-aware like improved_noise (same ck_cap), but
    # area_map arms on EVERY boot (no env gate) — hence found -> PASS. Armed is
    # rc-aware: rc!=0 means the armed retransform itself failed (FAIL), rc=0 is
    # the healthy PASS. Late arming (poller backoff) shows as INFO when any
    # area_map life-sign exists, FAIL only on a truly silent capable .so.
    ck_cap "area_map armed"          'area_map: hook armed, retransform rc=[0-9]+' 'area_map: hook armed, retransform rc=' "$AMAP_LIFESIGNS" rc
    ck_cap "area_map scans-avoided"  'area_map: sighting feed: [0-9]+ full class-heap scans avoided' 'area_map: sighting feed: ' "$AMAP_LIFESIGNS" pass
    # TASK-68: call-level probe (drives the patched update() via the embedded
    # Java driver, both ops arms). Diagnostics-only in the module, but a green
    # boot on a capable .so must emit the OK line — capability-aware row like
    # scans-avoided; the FAILED variant is a hard bad-line below.
    ck_cap "area_map call-level"     'area_map: call-level self-test OK' 'area_map: call-level self-test ' "$AMAP_LIFESIGNS" pass
    ck_bad "area_map call-level FAILED" 'area_map: call-level self-test FAILED'
    ck "live proof nativeCheck"  'live proof: normalNoise.nativeCheck\(\) = 1'
    ck "boot complete (Done)"    'Done \([0-9.]+s\)!'
    ck_bad "batch policy REFUSED" 'batch: kernel .* REFUSED by kernel-policy'
    ck_bad "WIRE REFUSED"        'kernel-policy:.*REFUSED'
    ck_bad "policy BYPASSED"     'kernel-policy:.*BYPASSED'
    ck_bad "symbols unresolved>0" 'native surface live: [0-9]+ bridge classes, [0-9]+ natives registered \([1-9][0-9]* symbols unresolved\)'
    # NOTE dormant ERE: the real line is "...dormant (set CRUSSTY_NATIVE_IMPROVED_NOISE=1
    # to enable)" — the old pattern ended with "1\)" and could NEVER match (actual
    # root cause of the S7-5 "dormant-строка ABSENT" false alarm; the string is in
    # the .so since 1ab0af4). The <so-substring> args are the capability probes,
    # the <lifesigns-ERE> args scope the alive-check to the row's own hook.
    # Armed rows run in rc-mode: the ERE captures ANY rc and the row verdict is
    # driven by the captured value (rc=0 PASS / rc!=0 FAIL) instead of pinning
    # rc=0 in the ERE and letting a real rc!=0 vanish into the INFO branch.
    ck_cap "improved_noise armed" 'improved_noise: hook armed, retransform rc=[0-9]+' 'improved_noise: hook armed, retransform rc=' "$NOISE_LIFESIGNS" rc
    ck_cap "improved_noise dormant" 'improved_noise: dormant \(set CRUSSTY_NATIVE_IMPROVED_NOISE=1' 'improved_noise: dormant (set CRUSSTY_NATIVE_IMPROVED_NOISE=1' "$NOISE_LIFESIGNS"
    ck_cap "improved_noise scans-avoided" 'improved_noise: sighting feed: [0-9]+ full class-heap scans avoided' 'improved_noise: sighting feed: ' "$NOISE_LIFESIGNS"
    # Attempt>1 frequency monitor (Task 2-b, S7-10): per-boot max capture attempt k,
    # so boots accumulate the accepted-race frequency baseline (SESSION 009 boot#3:
    # attempt 1 dropped, attempt 2 delivered — that boot must read INFO k=2, not
    # invisible). See E2E_LIVE_2026-09-08.md ADDENDUM S7-9/S7-10.
    ck_noise_attempts "improved_noise capture attempts"
    # G4 site-arming row (S7-12, docs/G4_SITE_PATCH_DESIGN.md §7.3): the
    # dormant boot shows the arm line ABSENT (PASS-by-absence, INFO — the
    # rollout gate off means silent refusal by design); a CRUSSTY_BATCH=auto|on
    # boot shows the arm marker plus the retarget evidence line (retargeted /
    # skipped / FAILED — all three are honest states, only the refusal is a
    # hard FAIL because a kernel-policy refusal for the proven nativeNoise
    # pair would mean a demoted kernel).
    local site_arm_line retarget_line site_refusal
    site_refusal="$(grep_markers 'batch: site .* not armed: kernel-policy refuses')"
    if [ -n "$site_refusal" ]; then
        printf '%-34s %-6s %s\n' "batch site arm" "FAIL" "$site_refusal"; fails=$((fails+1))
    else
        site_arm_line="$(grep_markers 'batch: arm [A-Za-z0-9_/.]+ id=(none|[0-9]+) T=[0-9]+ site=')"
        retarget_line="$(grep_markers 'batch: site [a-z_]+ retarget')"
        if [ -n "$site_arm_line" ] && [ -n "$retarget_line" ]; then
            printf '%-34s %-6s %s\n' "batch site arm" "PASS" "$site_arm_line | $retarget_line"
        elif [ -n "$site_arm_line" ]; then
            printf '%-34s %-6s %s\n' "batch site arm" "INFO" "$site_arm_line (no retarget evidence line)"
        else
            printf '%-34s %-6s %s\n' "batch site arm" "INFO" "(absent — dormant PASS-by-absence: rollout gate off, site unretargeted)"
        fi
    fi
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
    # S7-31 fix #2: early-return used to skip kill_stdin_holders entirely —
    # after an RCON-side stop (server already dead) the session's stdin holders
    # survived holding inherited flock'd fds -> next session deadlocked on
    # BENCH-MUTEX. Holders MUST be reaped on every shutdown path.
    [ -n "$pid" ] || { kill_stdin_holders; log "no server running — nothing to stop"; return 0; }
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
    kill_stdin_holders           # session over — relay lifeline down (TASK-59)
    rm -f "$PID_FILE" "$STATE_FILE" "$OFFS_FILE"
    rm -f "$LOGS_DIR"/crussty_e2e_stdin.* 2>/dev/null || true
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
