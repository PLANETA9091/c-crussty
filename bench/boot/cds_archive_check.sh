#!/usr/bin/env bash
# TASK-112 (S7-51): instant file-level CDS archive freshness preflight — ZERO boots.
#
# Contract (exit codes):
#   0 = CURRENT      manifest exists AND manifest jar_sha256 == live purpur jar sha256
#   1 = STALE        manifest exists AND jar hash mismatch (purpur changed since dump)
#   2 = NO-MANIFEST  archive exists but has no sidecar manifest (pre-contract archive,
#                    e.g. dumped by graal_cds_compose.sh before this contract) — usable,
#                    but staleness unprovable; mtime heuristic reported
#   3 = MISSING      archive file absent (ladder will fall through; expect L2+)
#   4 = NO-JAR       live purpur jar itself missing (sandbox broken)
#
# Usage:
#   bash bench/boot/cds_archive_check.sh                      # default production pair
#   JSA=/path/x.jsa JAR=/path/y.jar bash cds_archive_check.sh # overrides (tests/fixtures)
# Machine-readable line on stdout: VERDICT=<CURRENT|STALE|NO-MANIFEST|MISSING|NO-JAR>
set -u
SERVER=/home/z/server
JSA=${JSA:-/home/z/server/crussty_boot_graal.jsa}
JAR=${JAR:-$SERVER/versions/purpur-1.21.10.jar}
MAN="$JSA.manifest"

if [ ! -s "$JSA" ]; then
    echo "VERDICT=MISSING archive=$JSA (absent or empty) — expect best-state ladder L2+ fallback"
    exit 3
fi
if [ ! -s "$JAR" ]; then
    echo "VERDICT=NO-JAR jar=$JAR missing — sandbox broken, preflight meaningless"
    exit 4
fi

LIVE_JAR_SHA=$(sha256sum "$JAR" | cut -d' ' -f1)

if [ ! -f "$MAN" ]; then
    # pre-contract archive: report mtime heuristic, do not fail the ladder
    J_MTIME=$(stat -c %Y "$JAR")
    A_MTIME=$(stat -c %Y "$JSA")
    if [ "$A_MTIME" -ge "$J_MTIME" ]; then
        H="archive mtime >= jar mtime (plausibly fresh)"
    else
        H="archive OLDER than jar (probably stale)"
    fi
    echo "VERDICT=NO-MANIFEST archive=$JSA has no .manifest sidecar — staleness unprovable; heuristic: $H; re-dump via cds_rebuild_graal.sh to adopt the contract"
    exit 2
fi

MAN_JAR_SHA=$(sed -n 's/^jar_sha256=//p' "$MAN")
if [ -z "$MAN_JAR_SHA" ]; then
    echo "VERDICT=NO-MANIFEST manifest=$MAN malformed (no jar_sha256 key)"
    exit 2
fi

DUMPED_JVER=$(sed -n 's/^java_version=//p' "$MAN")
if [ "$MAN_JAR_SHA" = "$LIVE_JAR_SHA" ]; then
    echo "VERDICT=CURRENT archive=$JSA matches live purpur jar (sha256 ${LIVE_JAR_SHA:0:12}..., dumped under: ${DUMPED_JVER:-unknown})"
    exit 0
else
    echo "VERDICT=STALE archive=$JSA was dumped for a DIFFERENT purpur jar (manifest ${MAN_JAR_SHA:0:12}... vs live ${LIVE_JAR_SHA:0:12}...) — dynamic regions will fail validation; expect L1->L2 fallback; re-dump via cds_rebuild_graal.sh"
    exit 1
fi
