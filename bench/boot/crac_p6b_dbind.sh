#!/usr/bin/env bash
# CLEAN discriminator (S7-62): bundled-agent premain registration on PLAIN JVM.
# Harness law (NEW): `A && B && java ... &` makes $! = SUBSHELL pid (bash) — jcmd
# attaches to bash, results garbage (S7-62 discovered; invalidates inline probes).
# Fix: java launch on its OWN line (single command + &).
set -u
JAVA=/home/z/crac-jdk/bin/java
JCMD=/home/z/crac-jdk/bin/jcmd
W=/tmp/crac_bind2
cd "$W"
rm -rf imgE; mkdir -p imgE
echo "=== run E $(date +%T)" >> /tmp/crac_p6b/agent.log

"$JAVA" -Djava.library.path=/tmp/crac_p6b -javaagent:/tmp/crac_p6b/hookv2.jar \
  -XX:CRaCCheckpointTo="$W/imgE" -cp "$W" MainD > "$W/probeE.log" 2>&1 < /dev/null 9>&- &
JP=$!
sleep 2.5
"$JCMD" "$JP" JDK.checkpoint > "$W/jcmdE.out" 2>&1; JRC=$?
DEAD=""
for i in $(seq 1 14); do kill -0 "$JP" 2>/dev/null || { DEAD=1; break; }; sleep 0.5; done
if [ -z "$DEAD" ]; then echo "E: SURVIVED jcmd_rc=$JRC"; kill -9 "$JP" 2>/dev/null; wait "$JP" 2>/dev/null
else wait "$JP" 2>/dev/null; echo "E: DIED jcmd_rc=$JRC wait_rc=$? img=$(ls "$W/imgE" 2>/dev/null | wc -l)"; fi
head -3 "$W/jcmdE.out"
echo "HOOK-MARKERS: $(grep -cE 'SURGERY-V2' /tmp/crac_p6b/agent.log)"