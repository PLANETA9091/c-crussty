#!/usr/bin/env bash
# bench/golden/dump_corpus.sh — golden corpus driver (NCF P0.4/P0.5).
#
# DOCUMENTED, NOT EXECUTED in task 0-c. This script boots a PURITY-CRITICAL
# server: plain Purpur, NO CRUSSTY agent, NO -agentpath, NO extra plugins.
#
#   PURITY LAW: the golden corpus is the oracle for every future zero-diff
#   gate (NCF I1). The CRUSSTY Perlin whole-body bridge is default-ON since
#   TASK-148, so ANY agent-armed boot contaminates the corpus. Never "just
#   reuse" a running bench server: boot a fresh vanilla one with this script.
#
# What it does, one server lifecycle per invocation:
#   0. verify server.properties level-seed matches SEED (warn only, per spec)
#   1. optional FRESH=1: delete world dirs (fresh generation — REQUIRED for
#      corpus semantics; reusing an existing world loads chunks from disk)
#   2. boot: java -Xms512M -Xmx1536m -jar $SERVER_DIR/versions/purpur-1.21.10.jar --nogui
#      (no userjvmargs.txt, no JVM flags beyond heap — owner law I6)
#   3. poll logs/latest.log for the `Done (` marker (BOOT_TIMEOUT 240s),
#      same style as bench/ab/run_paper_ab.sh
#   4. generate the coordinate plan file (see PLAN below) and issue
#      `goldendump <label> manifest <plan-file>` over RCON
#      (python3 ../ab/rcon.py 25575 bench-ab-2301 ...)
#   5. poll for the `GOLDEN DUMP COMPLETE n=... failed=... dir=...` marker
#      (DUMP_TIMEOUT, default 900s), extract n/failed
#   6. rcon `stop`, wait for process exit
#   7. append one summary line to bench/golden/results/golden_runs.tsv
#
# Output root: <serverdir>/golden (the plugin default; no env needed).
# Nothing is exported here to redirect it — corpus lives with the server.
#
# Env:
#   SEED    canonical seed (default 3053459). The caller sets level-seed in
#           server.properties BEFORE boot; this script only verifies + warns.
#   LABEL   dump label (default vanilla_s<SEED>; spiral plans append _<plan>
#           so the two P0.5 boots get distinct labels)
#   PLAN    quadrants  — TASK-63 canon: two 8x8-chunk squares at chunk
#                       100..107 and -108..-101 (the forceload canon coords
#                       1600..1727 / -1728..-1601 block-coord / 16) = 128 chunks
#           full16     — 16x16 contiguous region chunk 100..115 = 256 chunks
#           spiral     — one 16x16 region (chunk 100..115) in spiral order
#           spiral_rev — the SAME region in reversed spiral order (P0.5)
#   FRESH   1 = delete world* before boot (default 0, prints a loud warning
#           if world/ exists and FRESH=0 — corpus would be disk-loaded, not
#           generated)
#   DUMP_TIMEOUT  poll cap for the dump marker (default 900s)
#
# NOTE ON THE SQUARE FORM: the canon 8x8 squares are NOT expressible as the
# (2r+1)^2 square form (8 is even), so this driver always uses the manifest
# form — which is also what makes arbitrary ORDER plans (spiral/rev) possible.
# For ad-hoc use the square form works fine:
#   rcon> goldendump mylabel 100 100 3
#
# Usage:
#   ./dump_corpus.sh                       # PLAN=quadrants SEED=3053459
#   PLAN=spiral  FRESH=1 ./dump_corpus.sh  # P0.5 run 1
#   PLAN=spiral_rev FRESH=1 ./dump_corpus.sh
#   python3 tools/ncfdiff.py --manifest \
#       /home/z/server/golden/vanilla_s3053459_spiral \
#       /home/z/server/golden/vanilla_s3053459_spiral_rev
set -euo pipefail

GOLDEN_DIR="$(cd "$(dirname "$0")" && pwd)"     # bench/golden
REPO="$(cd "$GOLDEN_DIR/../.." && pwd)"         # c-crussty
RCON_PY="$GOLDEN_DIR/../ab/rcon.py"

SERVER_DIR="${SERVER_DIR:-/home/z/server}"
SEED="${SEED:-3053459}"
PLAN="${PLAN:-quadrants}"
LABEL="${LABEL:-}"
FRESH="${FRESH:-0}"
BOOT_TIMEOUT=240
DUMP_TIMEOUT="${DUMP_TIMEOUT:-900}"
RCON_PORT=25575
RCON_PW=bench-ab-2301

RESULTS="$GOLDEN_DIR/results"
PLANS="$RESULTS/plans"
mkdir -p "$RESULTS" "$PLANS"

log() { printf '%s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }

java_pid() { pgrep -f 'purpur-1.21.10.jar' | head -1; }

wait_exit() { # grace wait, then TERM, then KILL (same policy as run_paper_ab.sh)
    local pid=$1 i=0
    while kill -0 "$pid" 2>/dev/null && [ $i -lt 60 ]; do sleep 1; i=$((i+1)); done
    if kill -0 "$pid" 2>/dev/null; then
        kill -TERM "$pid" 2>/dev/null; sleep 10
        kill -KILL "$pid" 2>/dev/null || true
    fi
}

rcon() { python3 "$RCON_PY" "$RCON_PORT" "$RCON_PW" "$1"; }

prop() { python3 - "$SERVER_DIR/server.properties" "$1" <<'PY'
import sys
want = sys.argv[2]
try:
    for line in open(sys.argv[1], encoding='utf-8'):
        line = line.strip()
        if line.startswith(want + '='):
            print(line.split('=', 1)[1]); break
except OSError:
    pass
PY
}

# ---- preflight --------------------------------------------------------------

[ -f "$SERVER_DIR/server.properties" ] || die "no server.properties under $SERVER_DIR"
[ -f "$SERVER_DIR/versions/purpur-1.21.10.jar" ] || die "server jar missing: $SERVER_DIR/versions/purpur-1.21.10.jar"
[ -f "$SERVER_DIR/plugins/GoldenDumper.jar" ] \
    || die "GoldenDumper.jar not installed in $SERVER_DIR/plugins/ (build: bench/golden/build_golden.sh)"

ACTUAL_SEED="$(prop level-seed || true)"
if [ -z "$ACTUAL_SEED" ]; then
    log "WARN: level-seed not set in server.properties — world will use a random seed"
elif [ "$ACTUAL_SEED" != "$SEED" ]; then
    log "WARN: level-seed=$ACTUAL_SEED but SEED=$SEED — corpus seed mismatch, fix server.properties BEFORE boot"
fi

case "$PLAN" in
    quadrants|full16|spiral|spiral_rev) ;;
    *) die "unknown PLAN '$PLAN' (quadrants | full16 | spiral | spiral_rev)" ;;
esac
if [ -z "$LABEL" ]; then
    LABEL="vanilla_s${SEED}"
    case "$PLAN" in spiral|spiral_rev) LABEL="${LABEL}_${PLAN}" ;; esac
fi

if [ -d "$SERVER_DIR/world" ] && [ "$FRESH" != "1" ]; then
    log "WARN: $SERVER_DIR/world exists and FRESH=0 — chunks may LOAD FROM DISK instead of generating;"
    log "WARN: a corpus dump on a loaded world is NOT a fresh-generation corpus. Set FRESH=1 for P0.4/P0.5 runs."
fi

# ---- coordinate plan --------------------------------------------------------
# One line per chunk "chunkX<TAB>chunkZ"; '#' comments allowed; ORDER IS THE
# DUMP ORDER (P0.5 depends on it).

plan_quadrants() { # TASK-63 canon burst: block 1600..1727 & -1728..-1601 / 16
    for x in $(seq 100 107);    do for z in $(seq 100 107);    do printf '%d\t%d\n' "$x" "$z"; done; done
    for x in $(seq -108 -101);  do for z in $(seq -108 -101);  do printf '%d\t%d\n' "$x" "$z"; done; done
}

plan_full16() {
    for x in $(seq 100 115); do for z in $(seq 100 115); do printf '%d\t%d\n' "$x" "$z"; done; done
}

plan_spiral() { # 16x16 region chunk 100..115, inward-out spiral, clockwise
    python3 - <<'PY'
lo, hi = 100, 115
x0, x1, z0, z1 = lo, hi, lo, hi
while x0 <= x1 and z0 <= z1:
    for x in range(x0, x1 + 1): print(f"{x}\t{z0}")
    for z in range(z0 + 1, z1 + 1): print(f"{x1}\t{z}")
    if x1 > x0:
        for x in range(x1 - 1, x0 - 1, -1): print(f"{x}\t{z1}")
    if z1 > z0:
        for z in range(z1 - 1, z0, -1): print(f"{x0}\t{z}")
    x0 += 1; x1 -= 1; z0 += 1; z1 -= 1
PY
}

PLAN_FILE="$PLANS/${LABEL}.tsv"
{
    echo "# plan=$PLAN label=$LABEL seed=$SEED generated=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    case "$PLAN" in
        quadrants)   plan_quadrants ;;
        full16)      plan_full16 ;;
        spiral)      plan_spiral ;;
        spiral_rev)  plan_spiral | tac ;;
    esac
} > "$PLAN_FILE"
N_CHUNKS="$(grep -vc '^#' "$PLAN_FILE" || true)"
log "plan: $PLAN_FILE ($N_CHUNKS chunks)"

# ---- boot (PURE VANILLA — no agent, no flags beyond heap) -------------------

if [ "$FRESH" = "1" ]; then
    log "FRESH=1: deleting $SERVER_DIR/world{,_nether,_the_end} for fresh generation"
    rm -rf "$SERVER_DIR/world" "$SERVER_DIR/world_nether" "$SERVER_DIR/world_the_end"
fi

mkdir -p "$SERVER_DIR/logs"
[ -f "$SERVER_DIR/logs/latest.log" ] && mv "$SERVER_DIR/logs/latest.log" "$SERVER_DIR/logs/latest.prev" 2>/dev/null

BOOTLOG="$RESULTS/golden_${LABEL}_boot.log"
cd "$SERVER_DIR" || die "cannot cd $SERVER_DIR"
log "booting PURE-VANILLA server for label=$LABEL (NO CRUSSTY agent — purity law)"
nohup setsid java \
    -Xms512M -Xmx1536m -jar "$SERVER_DIR/versions/purpur-1.21.10.jar" --nogui \
    </dev/null >"$BOOTLOG" 2>&1 &
disown || true

waited=0; done_line=""
while [ $waited -lt $BOOT_TIMEOUT ]; do
    sleep 2; waited=$((waited+2))
    pid=$(java_pid)
    [ -z "$pid" ] && { sleep 3; pid=$(java_pid); [ -z "$pid" ] && die "server died during boot — see $BOOTLOG"; }
    done_line=$(rg -o 'Done \([0-9.]+s\)!?' "$SERVER_DIR/logs/latest.log" 2>/dev/null | head -1 || true)
    [ -n "$done_line" ] && break
done
[ -n "$done_line" ] || die "no Done( marker within ${BOOT_TIMEOUT}s — see $SERVER_DIR/logs/latest.log"
log "booted: $done_line"

# ---- dump -------------------------------------------------------------------

rcon "goldendump $LABEL manifest $PLAN_FILE" || die "goldendump command failed over RCON"

waited=0; marker=""
while [ $waited -lt $DUMP_TIMEOUT ]; do
    sleep 3; waited=$((waited+3))
    kill -0 "$(java_pid)" 2>/dev/null || die "server died during dump — see $SERVER_DIR/logs/latest.log"
    # fast-fail: the plugin throwing inside onCommand would otherwise stall
    # the poll until DUMP_TIMEOUT (observed on the first corpus runs).
    if grep -q 'Command exception: /goldendump' "$SERVER_DIR/logs/latest.log" 2>/dev/null; then
        die "goldendump command threw — see 'Command exception' in $SERVER_DIR/logs/latest.log"
    fi
    marker=$(rg 'GOLDEN DUMP COMPLETE n=' "$SERVER_DIR/logs/latest.log" 2>/dev/null | head -1 || true)
    [ -n "$marker" ] && break
done
[ -n "$marker" ] || die "no GOLDEN DUMP COMPLETE marker within ${DUMP_TIMEOUT}s"
N="$(printf '%s' "$marker" | sed -n 's/.*n=\([0-9]*\).*/\1/p')"
FAILED="$(printf '%s' "$marker" | sed -n 's/.*failed=\([0-9]*\).*/\1/p')"
DUMPDIR="$(printf '%s' "$marker" | sed -n 's/.*dir=\(.*\)$/\1/p')"
log "dump complete: n=$N failed=$FAILED dir=$DUMPDIR"

# ---- stop + record ----------------------------------------------------------

rcon 'stop' >/dev/null 2>&1 || true
pid=$(java_pid); [ -n "$pid" ] && wait_exit "$pid"
sleep 2
pgrep -f 'purpur-1.21.10.jar' >/dev/null && die "server did not stop"

printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$SEED" "$LABEL" "$PLAN" "${N:-?}" "${FAILED:-?}" "$DUMPDIR" \
    | tee -a "$RESULTS/golden_runs.tsv"
log "corpus at $DUMPDIR ; diff hint:"
log "  python3 $GOLDEN_DIR/tools/ncfdiff.py --manifest $DUMPDIR <other-label-dir>"
if [ "${FAILED:-0}" != "0" ]; then
    log "WARN: $FAILED chunks failed to dump — inspect latest.log 'GOLDEN DUMP FAILED' lines"
fi
