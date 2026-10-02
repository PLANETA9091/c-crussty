#!/usr/bin/env bash
# TASK-95 phase-3 (S7-38): canonical ABBA x3/arm — v3 vs e2e default.
# D = e2e default (paperclip + v2 jsa auto-map, banked mean 13.351s)
# V = explicit-cp + v3 jsa + agent (12.954s preliminary n=1)
# Order D V D V D V, anchor-restore before EVERY boot, orphan-holder hygiene,
# hs_err passive. Output: per-boot Done times -> mean/separation/p.
set -u
SERVER=/home/z/server
SEED=$SERVER/world_census_seed.tar.gz
E2E=/home/z/ccrussty/c-crussty/scripts/e2e_orchestrate.sh
RCON=/home/z/ccrussty/c-crussty/scripts/rcon.py
JAVA=/home/z/jdk21/bin/java
V3=/tmp/crussty_boot_v3.jsa
OUT=/tmp/cds_v3
mkdir -p "$OUT"
CP="$SERVER/versions/1.21.10/purpur-1.21.10.jar"
while IFS= read -r j; do CP="$CP:$j"; done < <(find "$SERVER/libraries" -name '*.jar' | sort)
AGENTQ="'-agentpath:$SERVER/libcrussty_runtime.so=modules=$SERVER/modules;versions=$SERVER/versions;kernel=purpur-1.21.10.jar'"

reap_holders() {  # canon S7-31/37: kill orphaned stdin holders + children
    for p in /proc/[0-9]*/fd; do
        pid=${p#/proc/}; pid=${pid%/fd}
        [ -r "/proc/$pid/cmdline" ] || continue
        if ls -l "$p" 2>/dev/null | grep -q 'BENCH.lock'; then
            CL=$(tr '\0' ' ' < "/proc/$pid/cmdline" 2>/dev/null)
            case "$CL" in *crussty_e2e_stdin*|*"sleep 3600"*) kill -9 "$pid" 2>/dev/null;; esac
        fi
    done
}
exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-38 cds-v3-abba in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-38 cds-v3-abba (trap rc=$?)" > /home/z/BENCH.lock' EXIT
HS0=$(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l)
echo "hs_err baseline: $HS0"
anchor_restore() {
    "$E2E" shutdown >/dev/null 2>&1 || true; sleep 1
    rm -rf "$SERVER/world" "$SERVER/world_nether" "$SERVER/world_the_end"
    tar xzf "$SEED" -C "$SERVER"
}
stop_server() {
    python3 "$RCON" 127.0.0.1 25575 "stop" >/dev/null 2>&1 || true
    for i in $(seq 1 45); do pgrep -f "purpur-1.21.10.jar" >/dev/null || break; sleep 1; done
    sleep 1; reap_holders
}
boot_once() { # $1 = arm tag
    anchor_restore
    if [ "$1" = D ]; then
        E2E_LOG="$OUT/abba_${1}_e2e.log" "$E2E" boot >"$OUT/abba_${1}_bootcall.log" 2>&1
    else
        CRUSSTY_BOOT_CMD="exec '$JAVA' -XX:SharedArchiveFile=$V3 -Xlog:cds=info:file=$OUT/abba_${1}_cds.txt $AGENTQ \
-Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER \
-cp '$CP' org.bukkit.craftbukkit.Main --nogui" \
E2E_LOG="$OUT/abba_${1}_e2e.log" "$E2E" boot >"$OUT/abba_${1}_bootcall.log" 2>&1
    fi
    T=$(grep -h -m1 -oE 'Done \([0-9]+\.[0-9]+s\)' "$SERVER/logs/latest.log" 2>/dev/null | grep -oE '[0-9]+\.[0-9]+')
    M=$(grep -cE "Mapped" "$OUT/abba_${1}_cds.txt" 2>/dev/null || true)
    echo "ARM $1: ${T:-NO-DONE} mapped=${M:-0}"
    stop_server
}
for ROUND in 1 2 3; do
    boot_once D
    boot_once V
done
echo "hs_err after: $(ls "$SERVER"/hs_err_pid*.log 2>/dev/null | wc -l) (baseline $HS0)"
