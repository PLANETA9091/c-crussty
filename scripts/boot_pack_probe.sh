#!/bin/bash
# Local PACK oracle boot: fresh world with the pack zip installed as a WORLD
# datapack (dimension/overworld.json applies ONLY at world creation — addendum
# 50 lesson), then /goldendump + /goldenrefs on selected chunks. Mirrors the
# ci_gate_p2.sh pack-cell protocol (fresh boot, sync-chunk-writes, RCON,
# watchdog off) and the boot_goldenrefs.sh command loop.
# usage: boot_pack_probe.sh <pack_slug> <seed> <port> <cx1,cz1> [cx2,cz2 ...]
#        (targets are "cx,cz" for goldenrefs+goldendump pairs or "cmd:<cmd>")
set -u
SLUG="$1"; SEED="$2"; RCON_PORT="$3"; shift 3
CRUS=/home/z/my-project/c-crussty
SRV="$CRUS/ci-server"
DP_ROOT="$CRUS/ci-datapacks"
PACK_ZIP="$DP_ROOT/$SLUG.zip"
[ -f "$PACK_ZIP" ] || { echo "DIE: $PACK_ZIP missing"; exit 1; }
cd "$SRV" || exit 1

# stop any stale server
pkill -f 'purpur-1.21.10.jar' 2>/dev/null && sleep 5

if [ "${KEEP_WORLD:-0}" != "1" ]; then
    rm -rf world world_nether world_the_end
fi
mkdir -p world/datapacks logs config
cp "$PACK_ZIP" world/datapacks/
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

echo "[boot] pack=$SLUG seed=$SEED port=$RCON_PORT"
nohup setsid java -Xms512M -Xmx1536m -jar versions/purpur-1.21.10.jar --nogui \
    </dev/null > "$SRV/boot_probe.log" 2>&1 &
disown || true

waited=0; done_line=""
while [ $waited -lt 240 ]; do
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
        sh:*)
            shcmd="${target#sh:}"
            echo "[sh] $shcmd"
            ( cd "$SRV" && eval "$shcmd" ) || echo "sh failed: $shcmd"
            ;;
        *)
            cx="${target%,*}"; cz="${target#*,}"
            echo "[goldenrefs] chunk ($cx,$cz)"
            python3 "$CRUS/bench/ab/rcon.py" "$RCON_PORT" goldenpw "goldenrefs $cx $cz" || echo "rcon failed for ($cx,$cz)"
            sleep 2
            ;;
    esac
done

if [ "${NO_STOP:-0}" != "1" ]; then
sleep 4
python3 "$CRUS/bench/ab/rcon.py" "$RCON_PORT" goldenpw "stop" >/dev/null 2>&1 || true
for _ in $(seq 1 60); do
    pgrep -f 'purpur-1.21.10.jar' >/dev/null || break
    sleep 1
done
pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true
fi
echo "[probe] done — outputs above; full log: logs/latest.log"
