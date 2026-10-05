#!/usr/bin/env bash
# bench/golden/ci_datapacks.sh — NCF P0.4 tail: worldgen-datapack corpora on
# CI (owner spec: {vanilla, Terralith, Tectonic} corpora for the Phase 2
# zero-diff GATE).
#
# For each datapack (Terralith, Tectonic — resolved from the Modrinth API,
# datapack loader builds compatible with 1.21.x):
#   1. download the pack zip,
#   2. run ircheck over its data/ tree (Phase 1 IR coverage on non-vanilla
#      datapacks — the parser must understand Terralith/Tectonic routers),
#   3. ONE fresh boot with the pack injected into world/datapacks (P0.5
#      canonical PLAN=spiral), dump 256 chunks -> corpus artifact.
#
# Corpora are the REFERENCE side of future native comparisons; determinism of
# the rig itself is proven by the vanilla ci_gate control. Artifacts:
# golden-datapack-corpora.
set -euo pipefail

GOLDEN_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO="$(cd "$GOLDEN_DIR/../.." && pwd)"
CRATE="$REPO/chunk-factory"
SERVER_DIR="${SERVER_DIR:-$REPO/ci-server}"
SEED="${SEED:-3053459}"
PURPUR_BUILD="${PURPUR_BUILD:-2535}"
PURPUR_URL="${PURPUR_URL:-https://api.purpurmc.org/v2/purpur/1.21.10/${PURPUR_BUILD}/download}"
RCON_PORT=25575
RCON_PW=bench-ab-2301
RESULTS="$GOLDEN_DIR/results"
DP_ROOT="${DP_ROOT:-$REPO/ci-datapacks}"

log() { printf '[ci_datapacks] %s %s\n' "$(date -u +%H:%M:%S)" "$*" >&2; }
die() { log "FATAL: $*"; exit 1; }

mkdir -p "$RESULTS" "$SERVER_DIR/versions" "$SERVER_DIR/plugins" "$DP_ROOT"

# ---- provision (same protocol as ci_gate.sh) --------------------------------

if [ ! -f "$SERVER_DIR/versions/purpur-1.21.10.jar" ]; then
    log "downloading Purpur 1.21.10 build $PURPUR_BUILD"
    curl -fsSL --retry 3 -o "$SERVER_DIR/versions/purpur-1.21.10.jar" "$PURPUR_URL"
fi
cd "$SERVER_DIR"
if [ ! -f versions/1.21.10/purpur-1.21.10.jar ]; then
    has_craftworld() {
        python3 -c "import zipfile,sys; sys.exit(0 if 'org/bukkit/craftbukkit/CraftWorld.class' in zipfile.ZipFile(sys.argv[1]).namelist() else 1)" "$1" 2>/dev/null
    }
    rm -f eula.txt
    log "unpacking mojang-mapped image"
    java -Dpaperclip.patchOnly=true -jar versions/purpur-1.21.10.jar > "$SERVER_DIR/patchonly.log" 2>&1 &
    PC_PID=$!
    UNPACKED=0
    for _ in $(seq 1 180); do
        if [ -f versions/1.21.10/purpur-1.21.10.jar ] && has_craftworld versions/1.21.10/purpur-1.21.10.jar; then
            UNPACKED=1; break
        fi
        kill -0 "$PC_PID" 2>/dev/null || break
        sleep 2
    done
    kill "$PC_PID" 2>/dev/null || true
    sleep 1
    [ "$UNPACKED" = "1" ] || die "mojang-mapped jar not materialized"
fi
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
pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true
sleep 1

cd "$GOLDEN_DIR"
SERVER_DIR="$SERVER_DIR" ./build_golden.sh
cp -f GoldenDumper.jar "$SERVER_DIR/plugins/GoldenDumper.jar"

# ---- resolve + fetch datapacks ----------------------------------------------

fetch_pack() { # $1 = modrinth slug, $2 = dest zip
    python3 - "$1" "$2" <<'PY'
import json, sys, urllib.request

slug, dest = sys.argv[1], sys.argv[2]

import os
MC_VERSION = os.environ.get("MC_VERSION", "1.21.10")

def get(url):
    req = urllib.request.Request(url, headers={"User-Agent": "crussty-ncf-ci/1.0"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.loads(r.read().decode())

versions = get(f"https://api.modrinth.com/v2/project/{slug}/version")
datapack_versions = [v for v in versions if "datapack" in v.get("loaders", [])]
if not datapack_versions:
    datapack_versions = versions
# exact target version first (a pack built for a NEWER Minecraft can crash
# the server on world creation — seen with Terralith 2.6.0/1.21.11 on
# 1.21.10), then any 1.21.x, then any.
exact = [v for v in datapack_versions
         if MC_VERSION in v.get("game_versions", [])]
family = [v for v in datapack_versions
          if any(g.startswith("1.21") for g in v.get("game_versions", []))]
pick = (exact or family or datapack_versions)[0]
files = pick.get("files") or []
if not files:
    raise SystemExit(f"no files for {slug} {pick.get('version_number')}")
url = files[0]["url"]
print(f"[ci_datapacks] {slug}: {pick['version_number']} -> {url}", file=sys.stderr)
req = urllib.request.Request(url, headers={"User-Agent": "crussty-ncf-ci/1.0"})
with urllib.request.urlopen(req, timeout=300) as r, open(dest, "wb") as f:
    f.write(r.read())
PY
}

run_pack() { # $1 = label prefix, $2 = slug
    local label="$1" slug="$2"
    local zip="$DP_ROOT/${slug}.zip"
    log "fetching $slug (Modrinth)"
    fetch_pack "$slug" "$zip"
    du -h "$zip"

    # IR gate over the datapack's own data/ tree (extract zip -> data/).
    # The vanilla worldgen is extracted FIRST into the same tree: datapack
    # routers legitimately reference vanilla registry entries
    # (minecraft:shift_x, minecraft:overworld/continents, ...) that ship
    # with the server jar, not with the pack.
    local extract="$DP_ROOT/${slug}-extract"
    rm -rf "$extract"
    python3 - "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" "$extract" <<'PY'
import zipfile, os, sys
z = zipfile.ZipFile(sys.argv[1])
n = 0
for name in z.namelist():
    if name.startswith('data/minecraft/worldgen/') and name.endswith('.json'):
        dest = os.path.join(sys.argv[2], *name.split('/'))
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        open(dest, 'wb').write(z.read(name))
        n += 1
print(f"extracted {n} vanilla worldgen json files (ref-fallback base)")
PY
    python3 - "$zip" "$extract" <<'PY'
import zipfile, os, sys
z = zipfile.ZipFile(sys.argv[1])
n = 0
for name in z.namelist():
    if name.endswith('.json'):
        rest = name[len('data/'):] if name.startswith('data/') else name
        parts = rest.split('/')
        if len(parts) < 4 or parts[1] != 'worldgen':
            continue
        dest = os.path.join(sys.argv[2], *name.split('/'))
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        open(dest, 'wb').write(z.read(name))
        n += 1
print(f"extracted {n} datapack worldgen json files")
PY
    if [ -d "$extract/data" ]; then
        log "IR gate: ircheck over $slug datapack worldgen"
        ( cd "$CRATE" && cargo run --bin ircheck -- "$extract" --ns '' ) \
            | tee "$RESULTS/ircheck_${slug}_$(date -u +%Y-%m-%d).log" \
            || die "ircheck failed on $slug datapack"
    fi

    # corpus boot
    mkdir -p "$DP_ROOT/${slug}-datapacks"
    cp -f "$zip" "$DP_ROOT/${slug}-datapacks/${slug}.zip"
    log "corpus boot: $slug (seed $SEED, spiral, fresh world)"
    SEED="$SEED" LABEL="${label}" PLAN=spiral FRESH=1 SERVER_DIR="$SERVER_DIR" \
        DATAPACKS_DIR="$DP_ROOT/${slug}-datapacks" \
        bash "$GOLDEN_DIR/dump_corpus.sh" || {
            log "--- boot/dump log tail ($label) ---"
            tail -40 "$RESULTS/golden_${label}_boot.log" >&2 || true
            tail -40 "$SERVER_DIR/logs/latest.log" >&2 || true
            die "dump failed for $slug"
        }

    row=$(awk -F'\t' -v l="$label" '$3==l {last=$0} END {print last}' "$RESULTS/golden_runs.tsv")
    n=$(printf '%s' "$row" | awk -F'\t' '{print $5}')
    f=$(printf '%s' "$row" | awk -F'\t' '{print $6}')
    [ "$n" = "256" ] && [ "$f" = "0" ] || die "$slug corpus: n='$n' failed='$f' (expected 256/0)"
    log "$slug corpus done (256/0)"
}

run_pack "terralith_s$SEED" "terralith"
run_pack "tectonic_s$SEED" "tectonic"

pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true
log "ALL GREEN — datapack corpora complete (terralith + tectonic)"
