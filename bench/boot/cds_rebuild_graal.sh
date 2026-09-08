#!/usr/bin/env bash
# TASK-112 (S7-51): Graal-aware dynamic CDS archive re-dump + sidecar MANIFEST contract.
#
# WHY: bench/boot/cds_rebuild.sh is Temurin-hardcoded (/home/z/jdk21/bin/java ->
#   /tmp/crussty_boot_v2.jsa). A CDS archive is version- AND JVM-build-bound
#   (TASK-87/109 law): a Temurin-dumped archive FAILS validation under GraalVM.
#   After any Paper/engine update, running only the old runbook silently downgrades
#   the operator best-state ladder (scripts/e2e_beststate_boot.sh) L1 -> L2 with no
#   warning = silent -20.5% boot regression (banked TASK-109 COMPOSE-GO channel lost).
#
# WHAT: dumps the dynamic archive under GRAALVM (auto-detected), agent-free per the
#   TASK-87 law (JDK forbids ArchiveClassesAtExit with an agent attached; the produced
#   archive IS usable at production boots that DO attach the agent — TASK-110 triple
#   pass), to the persistent production path, and writes a sidecar MANIFEST so
#   cds_archive_check.sh can mechanically detect staleness later.
#
# MANIFEST CONTRACT (sidecar $JSA.manifest, key=value):
#   java_vendor / java_version  — JVM that dumped the archive (pairing proof)
#   jsa_sha256                  — archive content hash
#   jar_sha256 / jar_mtime      — purpur jar identity AT DUMP TIME (staleness anchor)
#   dumped_at                   — UTC timestamp
#
# Usage:
#   bash bench/boot/cds_rebuild_graal.sh                 # real dump (~60-90s, BENCH-MUTEX)
#   DRY_RUN=1 bash bench/boot/cds_rebuild_graal.sh       # print plan, touch NOTHING
# Env overrides: JAVA_BIN, JSA (default /home/z/server/crussty_boot_graal.jsa)
set -u

SERVER=/home/z/server
JSA=${JSA:-/home/z/server/crussty_boot_graal.jsa}
E2E=/home/z/ccrussty/c-crussty/scripts/e2e_orchestrate.sh
RCON=/home/z/ccrussty/c-crussty/scripts/rcon.py
JAR="$SERVER/versions/purpur-1.21.10.jar"

# --- JAVA_BIN resolution: env > canonical Graal > graalvm-dl glob > Temurin fallback
resolve_java() {
    if [ -n "${JAVA_BIN:-}" ] && [ -x "$JAVA_BIN" ]; then echo "$JAVA_BIN"; return 0; fi
    if [ -x /home/z/graalvm/bin/java ]; then echo /home/z/graalvm/bin/java; return 0; fi
    local g
    g=$(ls -1 /home/z/graalvm-dl/graalvm-*/bin/java 2>/dev/null | head -1)
    if [ -n "$g" ] && [ -x "$g" ]; then echo "$g"; return 0; fi
    if [ -x /home/z/jdk21/bin/java ]; then echo /home/z/jdk21/bin/java; return 0; fi
    echo ""; return 1
}

JAVA_BIN_RESOLVED=$(resolve_java) || { echo "FATAL: no usable java found"; exit 4; }
JVER=$("$JAVA_BIN_RESOLVED" -version 2>&1 | head -1)

if [ "${DRY_RUN:-0}" = "1" ]; then
    echo "DRY_RUN plan (nothing touched):"
    echo "  JAVA_BIN = $JAVA_BIN_RESOLVED"
    echo "  JAVA_VER = $JVER"
    echo "  JSA      = $JSA (manifest: $JSA.manifest)"
    echo "  CMD      = exec $JAVA_BIN_RESOLVED -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER -XX:ArchiveClassesAtExit=$JSA -jar $JAR --nogui"
    exit 0
fi

# --- pair sanity: warn loudly if a non-Graal JVM targets the Graal production archive
case "$JVER" in
    *GraalVM*) : ;;
    *) [ "$JSA" = "/home/z/server/crussty_boot_graal.jsa" ] && \
       echo "WARN: dumping with NON-Graal JVM ($JVER) to the Graal production archive path — that archive will NOT map under GraalVM (version-bound law); ladder will fall L1->L2" ;;
esac

# --- BENCH-MUTEX (real run only)
exec 200>/home/z/BENCH.lock
flock -n 200 || { echo "BENCH-LOCK HELD - abort"; exit 3; }
echo "$(date -u +%FT%TZ) main-s7-51 cds-rebuild-graal in-progress" > /home/z/BENCH.lock
trap 'echo "$(date -u +%FT%TZ) done main-s7-51 cds-rebuild-graal (trap rc=$?)" > /home/z/BENCH.lock' EXIT

"$E2E" shutdown >/dev/null 2>&1 || true
sleep 2
rm -f "$JSA"
# vanilla dump boot (NO agent — TASK-87 law); world restore kept for parity with cds_rebuild.sh
rm -rf "$SERVER/world" "$SERVER/world_nether" "$SERVER/world_the_end"
tar xzf "$SERVER/world_census_seed.tar.gz" -C "$SERVER"
CRUSSTY_BOOT_CMD="exec $JAVA_BIN_RESOLVED -Xms512M -Xmx2G -XX:+UseG1GC -Dfile.encoding=UTF-8 -Ddist.root=$SERVER -XX:ArchiveClassesAtExit=$JSA -jar $JAR --nogui" \
E2E_LOG=/tmp/cds_rebuild_graal_e2e.log "$E2E" boot >/dev/null 2>&1
python3 "$RCON" 127.0.0.1 25575 "stop" >/dev/null 2>&1 || true
for i in $(seq 1 60); do pgrep -x java >/dev/null || break; sleep 1; done   # pgrep -x (TASK-110 self-match lesson)
sleep 2

if [ ! -s "$JSA" ]; then
    echo "DUMP FAILED - see /tmp/cds_rebuild_graal_e2e.log"
    exit 1
fi

# --- sidecar manifest (staleness contract for cds_archive_check.sh)
JSUM=$(sha256sum "$JSA" | cut -d' ' -f1)
JSUM_JAR=$(sha256sum "$JAR" 2>/dev/null | cut -d' ' -f1 || echo "unknown")
JMT_JAR=$(stat -c %Y "$JAR" 2>/dev/null || echo "0")
cat > "$JSA.manifest" << EOF
java_bin=$JAVA_BIN_RESOLVED
java_version=$JVER
jsa_sha256=$JSUM
jar_sha256=$JSUM_JAR
jar_mtime=$JMT_JAR
dumped_at=$(date -u +%FT%TZ)
EOF

echo "ARCHIVE OK: $(du -h "$JSA" | cut -f1) -> $JSA"
echo "MANIFEST:   $JSA.manifest"
echo "  $JVER"
echo "  jsa_sha256=$JSUM"
echo "  jar_sha256=$JSUM_JAR (mtime $JMT_JAR)"
echo "Boot with: -XX:SharedArchiveFile=$JSA  (verify: add -Xlog:cds=info, expect 6 Mapped regions = 3 static + 3 dynamic)"
echo "Preflight: bash bench/boot/cds_archive_check.sh   (exit 0=CURRENT 1=STALE 2=NO-MANIFEST 3=MISSING)"
