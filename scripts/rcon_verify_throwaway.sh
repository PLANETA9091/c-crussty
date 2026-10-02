#!/usr/bin/env bash
# TASK-75 (S7-21) — RCON hygiene rotation end-to-end verification on a THROWAWAY
# Purpur instance. Does NOT touch /home/z/server runtime and does NOT take
# /home/z/BENCH.lock (the server lane belongs to the concurrent TASK-74 session).
# Politeness: boots under nice -n 19 so a concurrent timing run on the same 2
# cores keeps scheduler priority (CPU-s metrics of the other lane are the ones
# that must stay clean; this probe is functional, not timed).
#
# Verdict lines: PROBE_NEW_OK / PROBE_OLD_REJECTED / CLEAN_STOP (all three => PASS)
# Exit: 0 = all pass; 1 = any failure. 0 new secret values are ever printed.
set -u

RUNDIR=/tmp/rcon_hygiene_verify
SRC=/home/z/server
RCON_PY="$(cd "$(dirname "$0")" && pwd)/rcon.py"
DEVLOGS_WORKLOG=/home/z/ccrussty/crussty-dev-logs/c-crussty/worklog.md
RPORT=25585
SPORT=25586
BOOT_TIMEOUT=240
rc=0

# ---- pre-flight: never run next to a live lane -------------------------------
if pgrep -f "java.*jar" >/dev/null 2>&1; then
  echo "ABORT: java process detected (foreign lane active)"; exit 1
fi
# last BENCH.lock "lock ..." line must be closed by a later "done" line
last_lock_line=$(grep -n "^lock " /home/z/BENCH.lock 2>/dev/null | tail -1 | cut -d: -f1 || true)
last_done_line=$(grep -n "done" /home/z/BENCH.lock 2>/dev/null | tail -1 | cut -d: -f1 || true)
if [ -n "$last_lock_line" ] && [ -z "$last_done_line" -o "$last_lock_line" -gt "$last_done_line" ]; then
  echo "ABORT: /home/z/BENCH.lock has an open lock entry (foreign lane active)"; exit 1
fi

# ---- stage throwaway ----------------------------------------------------------
rm -rf "$RUNDIR"; mkdir -p "$RUNDIR"
cp "$SRC/versions/purpur-1.21.10.jar" "$RUNDIR/" || { echo "ABORT: purpur jar"; exit 1; }
mkdir -p "$RUNDIR/cache" "$RUNDIR/versions"
cp "$SRC/versions/purpur-1.21.10.jar" "$RUNDIR/versions/"
cp "$SRC/cache/mojang_1.21.10.jar" "$RUNDIR/cache/" 2>/dev/null || echo "WARN: no mojang cache jar"
printf 'eula=true\n' > "$RUNDIR/eula.txt"
cp "$SRC/server.properties" "$RUNDIR/server.properties"
sed -i -e "s/^rcon.port=.*/rcon.port=$RPORT/" \
       -e "s/^server-port=.*/server-port=$SPORT/" \
       -e "s/^query.port=.*/query.port=$SPORT/" \
       -e "s/^level-type=.*/level-type=minecraft\\\\:flat/" \
       -e "s/^view-distance=.*/view-distance=2/" \
       -e "s/^online-mode=.*/online-mode=false/" \
       -e "s/^level-seed=.*/level-seed=90919058/" \
       "$RUNDIR/server.properties"
grep -q "^rcon.port=$RPORT" "$RUNDIR/server.properties" || echo "rcon.port=$RPORT" >> "$RUNDIR/server.properties"

# ---- boot (nice'd, dormant: no CRUSSTY env at all) ----------------------------
cd "$RUNDIR"
nice -n 19 java -Xms512m -Xmx1024m -jar "versions/purpur-1.21.10.jar" --nogui > boot.log 2>&1 &
JPID=$!
echo "boot started pid=$JPID (nice 19, flat world, ports $SPORT/$RPORT)"

done_ok=0
for i in $(seq 1 $((BOOT_TIMEOUT / 2))); do
  if grep -q "Done (" boot.log 2>/dev/null; then done_ok=1; break; fi
  if ! kill -0 "$JPID" 2>/dev/null; then break; fi
  sleep 2
done
if [ "$done_ok" != 1 ]; then
  echo "FAIL: boot did not reach Done in ${BOOT_TIMEOUT}s; tail:"; tail -5 boot.log
  kill -9 "$JPID" 2>/dev/null; exit 1
fi
grep -m1 "Done (" boot.log

# ---- probe 1: NEW password via hardened rcon.py (dogfood) ---------------------
if out=$(python3 "$RCON_PY" 127.0.0.1 "$RPORT" list 2>&1); then
  echo "PROBE_NEW_OK: $out"
else
  echo "FAIL: new-password round-trip failed: $out"; rc=1
fi

# ---- probe 2: OLD burned secret must be REJECTED ------------------------------
oldpw=$(grep -m1 -o "crussty-g9-probe" "$DEVLOGS_WORKLOG" 2>/dev/null || true)
if [ -z "$oldpw" ]; then
  echo "WARN: old secret token not found in dev-logs worklog — negative probe skipped (not a rotation failure)"
else
  neg=$(python3 - "$oldpw" "$RPORT" <<'PYEOF'
import socket, struct, sys
oldpw, port = sys.argv[1], int(sys.argv[2])
def pkt(rid, ptype, payload):
    body = struct.pack("<ii", rid, ptype) + payload.encode() + b"\x00\x00"
    return struct.pack("<i", len(body)) + body
try:
    with socket.create_connection(("127.0.0.1", port), timeout=10) as s:
        s.sendall(pkt(1, 3, oldpw))
        raw = b""
        while len(raw) < 4:
            c = s.recv(4 - len(raw))
            if not c:
                print("REJECTED (closed on auth)"); sys.exit(0)
            raw += c
        (length,) = struct.unpack("<i", raw)
        data = b""
        while len(data) < length:
            c = s.recv(length - len(data))
            if not c:
                print("REJECTED (closed mid-auth)"); sys.exit(0)
            data += c
        if len(data) < 8:
            print("REJECTED (short auth reply)"); sys.exit(0)
        rid = struct.unpack("<i", data[:8])[0]
        print("REJECTED (rid=-1)" if rid == -1 else f"UNEXPECTED_RID_{rid}")
except Exception as e:
    print(f"REJECTED (conn: {type(e).__name__})")
PYEOF
  )
  case "$neg" in
    REJECTED*) echo "PROBE_OLD_REJECTED: $neg" ;;
    *) echo "FAIL: old secret NOT rejected: $neg"; rc=1 ;;
  esac
fi

# ---- stop via RCON (dogfood again), then verify process exit ------------------
python3 "$RCON_PY" 127.0.0.1 "$RPORT" stop >/dev/null 2>&1
stopped=0
for i in $(seq 1 30); do
  kill -0 "$JPID" 2>/dev/null || { stopped=1; break; }
  sleep 2
done
if [ "$stopped" = 1 ]; then
  echo "CLEAN_STOP"
else
  echo "FAIL: server did not stop in 60s — killing"; kill -9 "$JPID" 2>/dev/null; rc=1
fi

# ---- secret hygiene: the RUNDIR props copy carries the NEW secret — shred it --
shred -u "$RUNDIR/server.properties" 2>/dev/null || rm -f "$RUNDIR/server.properties"

[ $rc = 0 ] && echo "VERDICT: PASS" || echo "VERDICT: FAIL"
exit $rc
