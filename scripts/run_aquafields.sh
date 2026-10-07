#!/usr/bin/env bash
# scripts/run_aquafields.sh — boot the capture server, run the /aquafields
# probes immediately after Done, then stop. One-shot bisect helper (Job 441690).
set -u
SERVER=/home/z/my-project/c-crussty/ci-server
RCON="python3 /home/z/my-project/c-crussty/bench/ab/rcon.py 25575 bench-ab-2301"
cd "$SERVER" || exit 1

pkill -f 'purpur-1.21.10.jar' 2>/dev/null
sleep 1
nohup setsid java -Xms512M -Xmx1024m -jar versions/purpur-1.21.10.jar --nogui </dev/null > /tmp/aqua_boot.log 2>&1 &
disown

# wait for a FRESH Done( marker (log mtime after boot start)
start=$(date +%s)
while true; do
  sleep 1
  now=$(date +%s)
  [ $((now - start)) -gt 240 ] && { echo "TIMEOUT waiting for boot"; exit 1; }
  line=$(tail -c 2000 logs/latest.log 2>/dev/null | grep -a 'Done (' | tail -1)
  [ -n "$line" ] && grep -aq "Done (" logs/latest.log 2>/dev/null && {
      # ensure the log was written after boot start
      m=$(stat -c %Y logs/latest.log)
      [ "$m" -ge "$start" ] && break
  }
  pgrep -f 'purpur-1.21.10.jar' >/dev/null || { echo "SERVER DIED during boot"; exit 1; }
done
echo "boot done: $line"

# probe immediately, retry each up to 5 times
probe() {
  local pos="$1"
  for _ in 1 2 3 4 5; do
    $RCON "aquafields $pos" >/dev/null 2>&1 && return 0
    sleep 0.5
  done
  echo "PROBE FAILED: $pos"
}

for pos in "257 -20 -61" "261 -31 -62" "248 -22 -73" "260 -34 -77" "265 -16 -74" "247 -43 -76"; do
  probe "$pos"
done

grep -a "aquafields @" logs/latest.log
$RCON "stop" >/dev/null 2>&1
sleep 3
pkill -f 'purpur-1.21.10.jar' 2>/dev/null
echo "=== done ==="
