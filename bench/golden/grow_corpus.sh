#!/usr/bin/env bash
# bench/golden/grow_corpus.sh — PROACTIVE corpus growth around terrain-adapting
# structure starts (owner directive 2026-10-07: "Корпус пополняйте чанками
# рядом со структурами ЗАРАНЕЕ, а не после того, как гейт покраснел").
#
# The T38-B lesson (addendum 19): the staged corpus was chunk-biased AWAY from
# structures (225-chunk region had no terrain-adapting starts), so the gate
# corpus never exercised the one layer that diverged. This tool enumerates the
# I8 fallback starts (stagediff --fallback-list, the Beardifier-radius prescan)
# and grows BOTH corpus sides around them BEFORE any gate needs them:
#
#   --plan  SEED X0 X1 Z0 Z1 EXTRACT OUT_DIR
#       no server needed: writes the start list + the dump square plan
#       (results/corpus/plan_seed_<seed>.txt) + the shadow key list.
#   --exec  SEED X0 X1 Z0 Z1 EXTRACT SERVER_DIR OUT_DIR [STATUS]
#       requires a BOOTED server (RCON up; e.g. the ci_density/ci_gate_p2
#       boot): for each start P, RCON "goldendump <px> <pz> <R> status <st>"
#       (the plugin dumps the spiral square around P) + the matching rust
#       gen-batch square. Corpus lands under OUT_DIR/seed_<seed>/.
#
# The dump square radius defaults to the largest fallback radius (8) so each
# start's whole beardifier reach is inside the corpus (Beardifier reach per
# structure: R = floor((maxdist+11+15)/16), villages 6, ancient_city 8).
set -euo pipefail

GOLDEN_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE="$GOLDEN_DIR/../../chunk-factory"
MODE="${1:?usage: grow_corpus.sh --plan|--exec SEED X0 X1 Z0 Z1 EXTRACT DIR [SERVER_DIR] [STATUS]}"
SEED="${2:?seed}"
X0="${3:?x0}"; X1="${4:?x1}"; Z0="${5:?z0}"; Z1="${6:?z1}"
EXTRACT="${7:?worldgen extract}"
OUT_DIR="${8:?out dir}"
SERVER_DIR="${9:-}"
STATUS="${10:-noise}"
DUMP_R="${NCF_CORPUS_DUMP_R:-8}"
RCON_PORT="${RCON_PORT:-25575}"
RCON_PW="${RCON_PW:-bench-ab-2301}"

log() { printf '[grow_corpus %s] %s %s\n' "$SEED" "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }

mkdir -p "$OUT_DIR"
cd "$CRATE"

# 1. the I8 prescan: starts on stderr, shadow keys on stdout
FBL_OUT="$OUT_DIR/fallback_seed_${SEED}.txt"
FBL_LOG="${FBL_OUT%.txt}.log"
cargo run --release --bin stagediff -- --fallback-list "$SEED" "$X0" "$X1" "$Z0" "$Z1" "$EXTRACT" \
    > "$FBL_OUT" 2> "$FBL_LOG"
STARTS=$(grep -a 'FALLBACK start ' "$FBL_LOG" || true)
N_STARTS=$(printf '%s' "$STARTS" | grep -c . || true)
N_SHADOW=$(wc -l < "$FBL_OUT" | tr -d ' ')
log "prescan: $N_STARTS starts, $N_SHADOW shadow chunks in [$X0..$X1]x[$Z0..$Z1]"
[ -n "$STARTS" ] || { log "no terrain-adapting starts in region — nothing to grow"; exit 0; }

if [ "$MODE" = "--plan" ]; then
    PLAN="$OUT_DIR/plan_seed_${SEED}.txt"
    {
        echo "# proactive corpus plan; seed=$SEED region=[$X0..$X1]x[$Z0..$Z1] status=$STATUS"
        echo "# starts (structure cx cz r):"
        printf '%s\n' "$STARTS" | sed 's/^FALLBACK start //'
        echo "# dump squares (goldendump <px> <pz> $DUMP_R status $STATUS):"
        printf '%s\n' "$STARTS" | sed -n 's/.*cx=\(-\?[0-9]*\) cz=\(-\?[0-9]*\).*/goldendump \1 \2 '"$DUMP_R"' status '"$STATUS"'/p'
        echo "# gen-batch squares:"
        printf '%s\n' "$STARTS" | sed -n 's/.*cx=\(-\?[0-9]*\) cz=\(-\?[0-9]*\) r=\(-\?[0-9]*\).*/\1 \2 \3/p' | while read -r px pz r; do
            lo_x=$((px - r)); hi_x=$((px + r)); lo_z=$((pz - r)); hi_z=$((pz + r))
            echo "--gen-batch $SEED $lo_x $hi_x $lo_z $hi_z (rust side of start $px,$pz)"
        done
    } > "$PLAN"
    log "plan written: $PLAN"
    exit 0
fi

# --exec: a booted server is REQUIRED for the java side
[ -n "$SERVER_DIR" ] || die "--exec needs SERVER_DIR (booted server)"
[ -f "$SERVER_DIR/eula.txt" ] || die "no server at $SERVER_DIR"
RCON="$GOLDEN_DIR/../ab/rcon.py"

# 2. java side: one spiral square per start (label corpus_grow_<seed>)
printf '%s\n' "$STARTS" | sed -n 's/.*cx=\(-\?[0-9]*\) cz=\(-\?[0-9]*\).*/\1 \2/p' | while read -r px pz; do
    log "java dump square: goldendump $px $pz $DUMP_R status $STATUS"
    python3 "$RCON" "$RCON_PORT" "$RCON_PW" "goldendump $px $pz $DUMP_R status $STATUS" \
        || die "goldendump failed at $px $pz"
done
log "java side done (watch server log for GOLDEN STAGED DUMP COMPLETE per square)"

# 3. rust side: the matching squares
printf '%s\n' "$STARTS" | sed -n 's/.*cx=\(-\?[0-9]*\) cz=\(-\?[0-9]*\) r=\(-\?[0-9]*\).*/\1 \2 \3/p' | while read -r px pz r; do
    lo_x=$((px - r)); hi_x=$((px + r)); lo_z=$((pz - r)); hi_z=$((pz + r))
    RUST_DIR="$OUT_DIR/seed_${SEED}/rust_${STATUS}"
    mkdir -p "$RUST_DIR"
    log "rust gen-batch around start $px,$pz: [$lo_x..$hi_x]x[$lo_z..$hi_z]"
    cargo run --release --bin stagediff -- --gen-batch "$SEED" "$lo_x" "$hi_x" "$lo_z" "$hi_z" \
        "$EXTRACT" "$RUST_DIR" --status "$STATUS" || die "gen-batch failed at $px $pz"
done
log "corpus growth complete under $OUT_DIR/seed_${SEED}/"
