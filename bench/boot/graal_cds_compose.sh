#!/usr/bin/env bash
# TASK-109 (S7-47): Graal x AppCDS-v2 COMPOSABILITY A/B — the two banked GOs
# (TASK-96/100 Graal JIT -12.5% steady; TASK-87/88 CDS v2 boot -19.9%) have never
# been measured TOGETHER. Operator best-state claim (Graal + dormant) is incomplete
# without this: if CDS composes under Graal, boot stacks; if not, the v2 default
# is Temurin-specific and operator must choose one.
# Design: dump boot under GRAALVM (agent-free — JDK forbids ArchiveClassesAtExit
# with agent, TASK-87 law), then 3 ABBA pairs: A = Graal+SharedArchiveFile,
# B = Graal without archive. No agent in A/B arms (S7-30 R4: dormant agent boot
# cost = noise; isolates the pure JVM-level CDSxGraal question; agent+archive
# composition under Temurin already proven S7-31 "Mapped dynamic region x3").
# PRE-REGISTERED GATE (CLAIMS.md TASK-109, pushed before run):
#   COMPOSE-GO iff all 3 pairs d(A-B) < 0 AND mean <= -1.0s
#   COMPOSE-NULL if mean >= -0.3s or sign-unstable
#   between -> honest NO-VERDICT (underpowered n=3)
# 0 src/ Rust, 0 config/gameplay, flags CLI-only, prod e2e untouched.
set -u
SERVER=/home/z/server
GRAAL=$(ls -d /home/z/graalvm-dl/graalvm-*/bin/java 2>/dev/null | head -1)
[ -z "$GRAAL" ] && GRAAL=$(ls -d /home/z/graalvm/graalvm-*/bin/java 2>/dev/null | head -1)
[ -x "$GRAAL" ] || { echo "FATAL: GraalVM java not found"; exit 1; }
GRAAL_BIN=$(dirname "$GRAAL")
JAR="$SERVER/versions/purpur-1.21.10.jar"
JSA=/tmp/graal_boot_compose.jsa
REPO=/home/z/ccrussty/c-crussty
STAMP=$(date +%Y%m%d_%H%M%S)
OUT="$REPO/bench/boot/RAW_GRAALCDS_$STAMP"
mkdir -p "$OUT"

exec 9>/home/z/BENCH.lock
flock -n 9 || { echo "LOCK-BUSY"; exit 42; }
if pgrep -f 'purpur-1.21.10\.jar|launcher\.jar' >/dev/null 2>&1; then echo "LANE-BUSY"; exit 42; fi
echo "start-task109-graalcds-$STAMP" >> /home/z/BENCH.lock.journal
trap 'echo "done-task109-graalcds-$STAMP" >> /home/z/BENCH.lock.journal' EXIT
log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$OUT/run.log"; }
"$GRAAL" -version 2>&1 | head -1 | tee -a "$OUT/run.log"

stop_server() {
    python3 "$REPO/scripts/rcon.py" 127.0.0.1 25575 "stop" >/dev/null 2>&1 || true
    for i in $(seq 1 60); do pgrep -f "purpur-1.21.10.jar" >/dev/null || break; sleep 1; done
    sleep 2
}

# ---------- phase 1: DUMP boot (Graal, NO agent, dump at graceful exit) ----------
log "phase1: dump boot -> $JSA"
rm -f "$JSA"
tar xzf "$SERVER/world_census_seed.tar.gz" -C "$SERVER"
( cd "$SERVER" && exec timeout 300 "$GRAAL_BIN/java" -Xms512M -Xmx2G -XX:+UseG1GC \
    -Dfile.encoding=UTF-8 -XX:ArchiveClassesAtExit="$JSA" -Xlog:cds=info \
    -jar "$JAR" --nogui >"$OUT/dump_boot.log" 2>&1 ) &
SP=$!; disown $SP 2>/dev/null || true
D=0; for i in $(seq 1 240); do grep -q 'Done (' "$OUT/dump_boot.log" 2>/dev/null && { D=1; break; }; sleep 1; done
if [ "$D" != 1 ]; then log "DUMP-BOOT FAILED (no Done in 240s) — flag-acceptance gate evidence:"; tail -15 "$OUT/dump_boot.log" | tee -a "$OUT/run.log"; stop_server; exit 11; fi
DT=$(grep -o 'Done ([0-9.]*s)' "$OUT/dump_boot.log" | tail -1)
log "dump boot Done: $DT; graceful stop for dump write"
sleep 5; stop_server
if [ -s "$JSA" ]; then log "ARCHIVE OK: $(du -h "$JSA" | cut -f1)"; else log "DUMP FAILED (archive empty) — COMPOSE-NULL by flag-acceptance gate"; exit 12; fi
grep -E 'Mapped' "$OUT/dump_boot.log" | tail -3 | tee -a "$OUT/run.log" || true

# ---------- phase 2: 3 ABBA pairs, A=Graal+CDS, B=Graal ----------
PAIRS=0
run_boot() { # $1=RID $2=arm(A|B)
    local RID="$1" ARM="$2" EXTRA=""
    [ "$ARM" = "A" ] && EXTRA="-XX:SharedArchiveFile=$JSA"
    ( cd "$SERVER" && exec timeout 300 "$GRAAL_BIN/java" -Xms512M -Xmx2G -XX:+UseG1GC \
        -Dfile.encoding=UTF-8 $EXTRA -Xlog:cds=info \
        -jar "$JAR" --nogui >"$OUT/boot_$RID.log" 2>&1 ) &
    local SP=$!; disown $SP 2>/dev/null || true
    local D=0
    for i in $(seq 1 240); do grep -q 'Done (' "$OUT/boot_$RID.log" 2>/dev/null && { D=1; break; }; sleep 1; done
    if [ "$D" != 1 ]; then log "BOOT $RID ($ARM) FAILED"; tail -8 "$OUT/boot_$RID.log" | tee -a "$OUT/run.log"; kill -9 $SP 2>/dev/null || true; echo "$RID $ARM INVALID" >> "$OUT/boots.tsv"; return 1; fi
    local T; T=$(grep -o 'Done ([0-9.]*s)' "$OUT/boot_$RID.log" | tail -1)
    local M; M=$(grep -cE 'Mapped (static|dynamic)' "$OUT/boot_$RID.log" || true)
    echo -e "$RID\t$ARM\t$T\tmapped=$M" | tee -a "$OUT/boots.tsv"
    sleep 5; stop_server
    return 0
}
# order: A B | B A | A B  (3 pairs, position-balanced)
for ARM in A B B A A B; do
    PAIRS=$((PAIRS+1)); RID="p${PAIRS}_${ARM}"
    tar -C "$SERVER" -czf "$OUT/anchor.tgz" $(cd "$SERVER" && ls -d world world_nether world_the_end)
    run_boot "$RID" "$ARM" || true
    tar -C "$SERVER" -xzf "$OUT/anchor.tgz"   # restore world before next boot
done
rm -f "$OUT/anchor.tgz"
log "A/B complete — boots.tsv above; hs_err count: $(ls "$SERVER"/hs_err_* 2>/dev/null | wc -l)"
