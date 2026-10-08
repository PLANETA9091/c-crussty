#!/bin/bash
# Local goldenrefs probe: fresh vanilla boot at a given seed, then /goldenrefs on
# selected chunks. Mirrors bench/golden/ci_gate_p2.sh boot protocol (T37-b
# watchdog note, RCON sender, spawn-protection=0).
# usage: boot_goldenrefs.sh <seed> <port> <cx1,cz1> [cx2,cz2 ...]
set -u
SEED="$1"; RCON_PORT="$2"; shift 2
CRUS=/home/z/my-project/c-crussty
SRV="$CRUS/ci-server"
cd "$SRV" || exit 1

# stop any stale server
pkill -f 'purpur-1.21.10.jar' 2>/dev/null && sleep 5

if [ "${KEEP_WORLD:-0}" != "1" ]; then
    rm -rf world world_nether world_the_end
fi
mkdir -p logs config
[ -f logs/latest.log ] && mv logs/latest.log logs/latest.prev 2>/dev/null

cat > eula.txt <<'EOF'
eula=true
EOF
cat > server.properties <<EOF
level-seed=$SEED
enable-rcon=true
rcon.port=$RCON_PORT
rcon.password=goldenpw
online-mode=false
spawn-protection=0
sync-chunk-writes=true
max-tick-time=-1
EOF
cat > config/paper-global.yml <<'EOF'
_version: 29
watchdog:
  enable: false
EOF
# ensure NO datapack world-gen overrides (vanilla oracle)
rm -rf world/datapacks

echo "[boot] seed=$SEED port=$RCON_PORT"
nohup setsid java -Xms512M -Xmx1536m -jar versions/purpur-1.21.10.jar --nogui \
    </dev/null > "$SRV/boot_probe.log" 2>&1 &
disown || true

waited=0; done_line=""
while [ $waited -lt 180 ]; do
    sleep 2; waited=$((waited+2))
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || { sleep 2; pgrep -f 'purpur-1.21.10.jar' >/dev/null || { echo "DIE: server died during boot"; tail -30 logs/latest.log; exit 1; } ; }
    done_line=$(grep -aoE 'Done \([0-9.]+s\)!?' logs/latest.log 2>/dev/null | head -1 || true)
    [ -n "$done_line" ] && break
done
[ -n "$done_line" ] || { echo "DIE: no Done marker"; tail -30 logs/latest.log; exit 1; }
echo "[boot] $done_line"

sleep 3
for target in "$@"; do
    case "$target" in
        cmd:*)
            cmd="${target#cmd:}"
            echo "[cmd] $cmd"
            python3 "$CRUS/bench/ab/rcon.py" "$RCON_PORT" goldenpw "$cmd" || echo "rcon failed: $cmd"
            sleep 2
            ;;
        *)
            cx="${target%,*}"; cz="${target#*,}"
            echo "[goldenrefs] chunk ($cx,$cz)"
            python3 "$CRUS/bench/ab/rcon.py" "$RCON_PORT" goldenpw "goldenrefs $cx $cz" || echo "rcon failed for ($cx,$cz)"
            sleep 2
            ;;
    esac
done

sleep 4
python3 "$CRUS/bench/ab/rcon.py" "$RCON_PORT" goldenpw "stop" >/dev/null 2>&1 || true
for _ in $(seq 1 60); do
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || break
    sleep 1
done
pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true
echo "[probe] done — goldenrefs output above; full log: logs/latest.log"
