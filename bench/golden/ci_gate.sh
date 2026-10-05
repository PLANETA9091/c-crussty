#!/usr/bin/env bash
# bench/golden/ci_gate.sh — GitHub Actions driver for the NCF Phase 0 golden
# gate.
#
# Owner directive (2026-10-05): "do everything now; test on GH CI". The
# sandbox rig (/home/z/server) dies on every sandbox reset, so the only
# machine-checkable Phase 0 invariant — the ORDER-TEST CONTROL (same seed +
# same canonical order, two boots => all chunks semantically equal) — must be
# replayable on a fresh CI runner with zero /home/z state. This script is
# that replay, fully self-contained:
#
#   1. PROVISION a throwaway server dir under the workspace:
#        - download Purpur 1.21.10, PINNED to PURPUR_BUILD (default 2535 —
#          the build the sandbox corpora were dumped with);
#        - materialize the mojang-mapped server image via paperclip
#          (`-Dpaperclip.patchOnly=true`, plus a mapped-jar poll + hard kill:
#          purpur's paperclip ignores patchOnly at the EULA wall, so eula.txt
#          must NOT exist during unpack);
#        - write eula.txt + server.properties (canon seed, RCON on localhost
#          25575 / bench-ab-2301, sync-chunk-writes=true).
#   2. BUILD GoldenDumper.jar from SOURCE via build_golden.sh (javac path on
#      CI, ECJ fallback locally) — proves the committed jar matches sources,
#      and guards the mojang-mapped compile surface against drift.
#   3. GATE: two FRESH pure-vanilla boots, canon seed 3053459, canonical
#      PLAN=spiral order, labels ci_gate_A / ci_gate_B ->
#      `ncfdiff --manifest` MUST be equal on all 256 pairs (exit 0).
#      This replays the control row of results/ORDER_TEST_2026-10-05.md
#      (spiral vs spiral2: 256/256 SEMANTIC_EQUAL) on CI hardware.
#   4. P0.4 SEED MATRIX: GATE_SEEDS (default 4 extra seeds), one FRESH spiral
#      boot each -> vanilla corpora for future Phase 2 gates; uploaded as CI
#      artifacts (the sandbox cannot keep /home/z/server state).
#
# Purity law holds by construction: dump_corpus.sh is the only boot path —
# plain Purpur, NO CRUSSTY agent, no JVM flags beyond heap. Canonical dump
# order = PLAN=spiral (P0.5 decision); comparisons only between same-order
# corpora.
#
# Env:
#   SERVER_DIR    default <repo>/ci-server (gitignored)
#   PURPUR_BUILD  default 2535
#   SEED          default 3053459 (canon; gate boots)
#   GATE_SEEDS    default "90210 424242 8675309 133700"; set '' to skip P0.4
#   DUMP_TIMEOUT  forwarded to dump_corpus.sh (default 900)
#
# Exit 0 <=> gate equal AND every corpus row 256/0. ncfdiff exit 1
# (diverged) kills this script via set -e -> red CI run. Honest artifact
# trail: results/golden_runs.tsv (CI rows), results/CI_GATE_*.txt,
# results/CI_GATE_budget.tsv, server logs, dump dirs.
set -euo pipefail

GOLDEN_DIR="$(cd "$(dirname "$0")" && pwd)"     # bench/golden
REPO="$(cd "$GOLDEN_DIR/../.." && pwd)"         # c-crussty
SERVER_DIR="${SERVER_DIR:-$REPO/ci-server}"
SEED="${SEED:-3053459}"
PURPUR_BUILD="${PURPUR_BUILD:-2535}"
PURPUR_URL="${PURPUR_URL:-https://api.purpurmc.org/v2/purpur/1.21.10/${PURPUR_BUILD}/download}"
GATE_SEEDS="${GATE_SEEDS-90210 424242 8675309 133700}"
RCON_PORT=25575
RCON_PW=bench-ab-2301
DUMP_TIMEOUT="${DUMP_TIMEOUT:-900}"
RESULTS="$GOLDEN_DIR/results"
SUMMARY="${GITHUB_STEP_SUMMARY:-}"
BUDGET="$RESULTS/CI_GATE_budget.tsv"

log() { printf '[ci_gate] %s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }
now() { date +%s; }

mkdir -p "$RESULTS" "$SERVER_DIR/versions" "$SERVER_DIR/plugins"

# ---- 1. provision -----------------------------------------------------------

if [ ! -f "$SERVER_DIR/versions/purpur-1.21.10.jar" ]; then
    log "downloading Purpur 1.21.10 build $PURPUR_BUILD"
    curl -fsSL --retry 3 -o "$SERVER_DIR/versions/purpur-1.21.10.jar" "$PURPUR_URL"
fi
du -h "$SERVER_DIR/versions/purpur-1.21.10.jar"

cd "$SERVER_DIR"

if [ ! -f versions/1.21.10/purpur-1.21.10.jar ]; then
    # purpur's paperclip ignores patchOnly at the EULA wall (verified
    # 2026-10-05): remap finishes, then it tries to boot and refuses on the
    # missing eula.txt (exit 0). So: NO eula.txt yet, run unpack in the
    # background, poll for the mapped jar, hard-kill when it appears (or let
    # the EULA wall exit on its own).
    rm -f eula.txt
    log "unpacking mojang-mapped image (paperclip patchOnly + mapped-jar poll)"
    java -Dpaperclip.patchOnly=true -jar versions/purpur-1.21.10.jar \
        > "$SERVER_DIR/patchonly.log" 2>&1 &
    PC_PID=$!
    UNPACKED=0
    for _ in $(seq 1 150); do
        if [ -f versions/1.21.10/purpur-1.21.10.jar ]; then UNPACKED=1; break; fi
        kill -0 "$PC_PID" 2>/dev/null || break
        sleep 2
    done
    kill "$PC_PID" 2>/dev/null || true
    sleep 1
    [ "$UNPACKED" = "1" ] || die "mojang-mapped jar not materialized (see $SERVER_DIR/patchonly.log)"
fi
log "mojang-mapped jar: versions/1.21.10/purpur-1.21.10.jar"

printf 'eula=true\n' > eula.txt
cat > server.properties <<EOF
level-seed=$SEED
enable-rcon=true
rcon.port=$RCON_PORT
rcon.password=$RCON_PW
online-mode=false
spawn-protection=0
sync-chunk-writes=true
EOF

# no stray server from a previous attempt
pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true
sleep 1

# ---- 2. build the plugin from source ---------------------------------------

cd "$GOLDEN_DIR"
SERVER_DIR="$SERVER_DIR" ./build_golden.sh
cp -f GoldenDumper.jar "$SERVER_DIR/plugins/GoldenDumper.jar"
log "GoldenDumper.jar installed"

# ---- 3. gate: two fresh same-order boots ------------------------------------

cd "$REPO"

t0=$(now)
SEED="$SEED" LABEL=ci_gate_A PLAN=spiral FRESH=1 SERVER_DIR="$SERVER_DIR" \
    DUMP_TIMEOUT="$DUMP_TIMEOUT" bash "$GOLDEN_DIR/dump_corpus.sh"
t1=$(now)
SEED="$SEED" LABEL=ci_gate_B PLAN=spiral FRESH=1 SERVER_DIR="$SERVER_DIR" \
    DUMP_TIMEOUT="$DUMP_TIMEOUT" bash "$GOLDEN_DIR/dump_corpus.sh"
t2=$(now)

REPORT="$RESULTS/CI_GATE_$(date -u +%Y-%m-%d).txt"
log "GATE: ncfdiff --manifest ci_gate_A ci_gate_B (exit 1 = DIVERGED = red CI)"
python3 "$GOLDEN_DIR/tools/ncfdiff.py" --manifest \
    "$SERVER_DIR/golden/ci_gate_A" "$SERVER_DIR/golden/ci_gate_B" \
    --report "$REPORT"
log "GATE PASS: all pairs equal (EQUAL/SEMANTIC_EQUAL) — report: $REPORT"
tail -5 "$REPORT" >&2 || true

# sanity: both runs 256/0 in golden_runs.tsv (CI-local copy)
for lbl in ci_gate_A ci_gate_B; do
    row=$(awk -F'\t' -v l="$lbl" '$3==l {last=$0} END {print last}' "$RESULTS/golden_runs.tsv")
    n=$(printf '%s' "$row"   | awk -F'\t' '{print $5}')
    f=$(printf '%s' "$row"   | awk -F'\t' '{print $6}')
    [ "$n" = "256" ] && [ "$f" = "0" ] || die "label $lbl: n='$n' failed='$f' (expected 256/0)"
done

printf 'boot_A\t%s s\nboot_B\t%s s\n' "$((t1 - t0))" "$((t2 - t1))" >> "$BUDGET"

# ---- 4. P0.4 seed matrix (one fresh spiral boot per seed) -------------------

if [ -n "$GATE_SEEDS" ]; then
    for s in $GATE_SEEDS; do
        log "P0.4 corpus boot: seed $s (spiral, fresh world)"
        sed -i "s/^level-seed=.*/level-seed=$s/" "$SERVER_DIR/server.properties"
        ts=$(now)
        SEED="$s" LABEL="vanilla_ci_s$s" PLAN=spiral FRESH=1 SERVER_DIR="$SERVER_DIR" \
            DUMP_TIMEOUT="$DUMP_TIMEOUT" bash "$GOLDEN_DIR/dump_corpus.sh"
        te=$(now)
        row=$(awk -F'\t' -v l="vanilla_ci_s$s" '$3==l {last=$0} END {print last}' "$RESULTS/golden_runs.tsv")
        n=$(printf '%s' "$row" | awk -F'\t' '{print $5}')
        f=$(printf '%s' "$row" | awk -F'\t' '{print $6}')
        [ "$n" = "256" ] && [ "$f" = "0" ] || die "seed $s corpus: n='$n' failed='$f' (expected 256/0)"
        printf 'seed_%s\t%s s\n' "$s" "$((te - ts))" >> "$BUDGET"
        log "seed $s corpus done in $((te - ts))s (256/0)"
    done
else
    log "GATE_SEEDS empty — skipping P0.4 matrix"
fi

# ---- 5. CI summary ----------------------------------------------------------

if [ -n "$SUMMARY" ]; then
    {
        echo "## NCF Phase 0 golden gate — PASS"
        echo ""
        echo "- Purpur build: $PURPUR_BUILD (pinned)"
        echo "- Gate: ci_gate_A vs ci_gate_B (seed $SEED, spiral, 2 fresh boots) — all 256 pairs equal"
        echo "- ncfdiff report: bench/golden/results/$(basename "$REPORT")"
        echo ""
        echo "| corpus boot | rows (n/failed) | wall time |"
        echo "|---|---|---|"
        while IFS=$'\t' read -r name secs; do
            row=$(awk -F'\t' -v l="$name" '$3==l {last=$0} END {print last}' "$RESULTS/golden_runs.tsv")
            nf=$(printf '%s' "$row" | awk -F'\t' '{print $5 "/" $6}')
            echo "| $name | $nf | $secs |"
        done < "$BUDGET"
    } >> "$SUMMARY" 2>/dev/null || true
fi

pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true
log "ALL GREEN — gate + P0.4 corpora complete"
