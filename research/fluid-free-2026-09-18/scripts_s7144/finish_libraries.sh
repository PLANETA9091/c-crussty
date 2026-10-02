#!/usr/bin/env bash
# finish_libraries.sh — S7-144: let paperclip finish downloading ALL libraries
# (first materialization was killed right after versions jar appeared, so the
# mojang-logging needed by paper classes was still missing). Kill at "Starting
# ...Main" line = before main() runs; no eula.txt exists so even a racing main
# exits immediately without booting the server (INJECTS-ONLY).
set -euo pipefail
export PATH=/tmp/toolchain/jdk-21.0.12.1+1/bin:$PATH
WORK=/tmp/kernelmat/server
cd "$WORK"
java -jar purpur-1.21.10.jar nogui >> paperclip2.log 2>&1 &
PCPID=$!
# Kill the moment paperclip tries to start the server main, or after 240s
for _ in $(seq 1 240); do
  if rg -q "^Starting " paperclip2.log 2>/dev/null; then
    break
  fi
  if ! kill -0 "$PCPID" 2>/dev/null; then
    break
  fi
  sleep 1
done
kill "$PCPID" 2>/dev/null || true
pkill -f "purpur-1.21.10.jar" 2>/dev/null || true
wait "$PCPID" 2>/dev/null || true
sleep 1
pgrep -af java || echo "NO java processes (INJECTS-ONLY clean)"
ls "$WORK/eula.txt" 2>/dev/null && cat "$WORK/eula.txt" || echo "no eula.txt (no boot)"
tail -5 paperclip2.log
echo "libraries now: $(find "$WORK/libraries" -name '*.jar' | wc -l)"
