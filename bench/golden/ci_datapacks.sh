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
    python3 - "$zip" "$extract" "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" <<'PY'
import json, os, sys, zipfile

zip_path, dest_root, server_jar = sys.argv[1], sys.argv[2], sys.argv[3]
z = zipfile.ZipFile(zip_path)

# The data-pack format the SERVER speaks (version.json in the mapped jar).
data_format = json.loads(zipfile.ZipFile(server_jar).read('version.json'))[
    'pack_version']['data_major']

def fmt_range(f):
    # formats: int | [min,max] | {min_inclusive,max_inclusive}
    if isinstance(f, dict):
        return int(f.get('min_inclusive', 0)), int(f.get('max_inclusive', 1 << 30))
    if isinstance(f, (list, tuple)):
        return int(f[0]), int(f[-1])
    return int(f), int(f)

# Overlay entries (in pack.mcmeta order) that match our format; LAST match
# wins — vanilla applies overlays on top of the base in list order.
mcmeta = None
if 'pack.mcmeta' in z.namelist():
    try:
        mcmeta = json.loads(z.read('pack.mcmeta'))
    except Exception:
        mcmeta = None
matching_overlays = []
overlay_section = {}
if mcmeta:
    # overlays can sit at the TOP level or inside 'pack' (both seen in the
    # wild); vanilla reads the top-level 'overlays'.
    overlay_section = mcmeta.get('overlays') or (
        mcmeta.get('pack', {}).get('overlays') if isinstance(mcmeta.get('pack'), dict) else None
    ) or {}
for entry in overlay_section.get('entries', []):
        lo, hi = fmt_range(entry.get('formats', entry.get('min_format')))
        if lo <= data_format <= hi:
            matching_overlays.append(entry['directory'].strip('/'))

def extract_worldgen(prefix):
    # prefix: '' for base data/, or '<overlay>/' — overlay files land on the
    # SAME relative paths as the base (that is what an overlay means).
    # T39: dimension/ overrides are extracted too — packs inject their
    # multi-noise biome table via data/minecraft/dimension/overworld.json.
    n = 0
    for name in z.namelist():
        # P5.3 increment 4: structure TEMPLATE .nbt files ride along with the
        # pack (data/<ns>/structure/**/*.nbt) — required by the jigsaw
        # assembly for pack adapting structures (Beardifier feed).
        if name.endswith('.nbt') and name.startswith(prefix + 'data/') and '/structure/' in name:
            parts = name[len(prefix + 'data/'):].split('/')
            dest = os.path.join(dest_root, "data", *parts)
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            open(dest, 'wb').write(z.read(name))
            n += 1
            continue
        if not name.endswith('.json') or not name.startswith(prefix + 'data/'):
            continue
        parts = name[len(prefix + 'data/'):].split('/')
        if len(parts) < 3 or parts[1] not in ('worldgen', 'dimension', 'tags'):
            # 'tags': pack biome tags (#<pack>:has_structure/...) feed the
            # P5.3-pre fallback prescan (structure_scan.rs reads them from
            # <extract>/data/<ns>/tags/worldgen/biome).
            continue
        if parts[1] == 'worldgen' and len(parts) < 4:
            continue
        dest = os.path.join(dest_root, "data", *parts)
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        open(dest, 'wb').write(z.read(name))
        n += 1
    return n

n = extract_worldgen('')
print(f"extracted {n} base worldgen json files")
for d in matching_overlays:
    n = extract_worldgen(d + '/')
    print(f"applied overlay '{d}': {n} worldgen json files (format {data_format})")
if not matching_overlays:
    print("WARNING: no overlay matched the server data format")
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

# ci_gate_p2.sh helper: `ci_datapacks.sh --extract-only <slug>` provisions the
# mapped jar and builds the merged (vanilla-base + pack-overlay) worldgen
# extract WITHOUT booting or dumping.
if [ "${1:-}" = "--extract-only" ]; then
    SLUG_X="${2:?--extract-only needs a slug}"
    ZIP_X="$DP_ROOT/${SLUG_X}.zip"
    if [ ! -f "$ZIP_X" ]; then
        log "fetching $SLUG_X (extract-only)"
        fetch_pack "$SLUG_X" "$ZIP_X"
    fi
    EXTRACT_X="$DP_ROOT/${SLUG_X}-extract"
    rm -rf "$EXTRACT_X"
    python3 - "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" "$EXTRACT_X" <<'PY'
import zipfile, os, sys
z = zipfile.ZipFile(sys.argv[1])
n = 0
for name in z.namelist():
    is_json = name.endswith('.json') and (
        name.startswith('data/minecraft/worldgen/') or name.startswith('data/minecraft/tags/block/')
        or name.startswith('data/minecraft/tags/worldgen/')
    )  # + tags/worldgen: biome tags for the P5.3-pre fallback prescan
    # P5.3 increment 4: structure TEMPLATE .nbt files — the jigsaw assembly
    # (piece_feed/template_cache) reads data/<ns>/structure/<path>.nbt from
    # the extract; without them pack/vanilla adapting starts fail
    # resource-missing and the Beardifier feed degrades to I8 fallback.
    is_nbt = name.endswith('.nbt') and name.startswith('data/minecraft/structure/')
    if is_json or is_nbt:
        dest = os.path.join(sys.argv[2], *name.split('/'))
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        open(dest, 'wb').write(z.read(name))
        n += 1
print(f"extracted {n} base json files")
PY
    python3 - "$ZIP_X" "$EXTRACT_X" "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" <<'PY'
import json, os, sys, zipfile

zip_path, dest_root, server_jar = sys.argv[1], sys.argv[2], sys.argv[3]
z = zipfile.ZipFile(zip_path)

data_format = json.loads(zipfile.ZipFile(server_jar).read('version.json'))[
    'pack_version']['data_major']

def fmt_range(f):
    if isinstance(f, dict):
        return int(f.get('min_inclusive', 0)), int(f.get('max_inclusive', 1 << 30))
    if isinstance(f, (list, tuple)):
        return int(f[0]), int(f[-1])
    return int(f), int(f)

mcmeta = None
if 'pack.mcmeta' in z.namelist():
    try:
        mcmeta = json.loads(z.read('pack.mcmeta'))
    except Exception:
        mcmeta = None
matching_overlays = []
overlay_section = {}
if mcmeta:
    overlay_section = mcmeta.get('overlays') or (
        mcmeta.get('pack', {}).get('overlays') if isinstance(mcmeta.get('pack'), dict) else None
    ) or {}
for entry in overlay_section.get('entries', []):
        lo, hi = fmt_range(entry.get('formats', entry.get('min_format')))
        if lo <= data_format <= hi:
            matching_overlays.append(entry)
        elif entry.get('formats') is None and entry.get('min_format') is not None:
            lo2, hi2 = fmt_range(entry.get('min_format'))
            if lo2 <= data_format <= hi2:
                matching_overlays.append(entry)

def extract_worldgen(prefix):
    # T40 FIX: this extractor fed the gate-P2 pack cells and was doubly broken:
    #   (1) dest lacked the "data/" segment (files landed at <extract>/minecraft/
    #       ... instead of <extract>/data/minecraft/...), so WorldgenDir::load
    #       (which walks <extract>/data/) saw NONE of the pack overrides ->
    #       build_overworld silently took the pure-vanilla path (vanilla router
    #       + vanilla preset table) -> every pack cell produced vanilla biomes
    #       (rust=minecraft:deep_dark vs java=terralith:cave/*, equal=0/2401).
    #   (2) the minecraft-only namespace filter starves the pack chain: pack
    #       routers reference their own namespace (terralith:overworld/cliff/
    #       spline -> "density_function file not found"). The real merged
    #       datapack loads worldgen/dimension under EVERY namespace.
    # Mirrors the run_pack extractor above: worldgen/dimension under any
    # namespace, dest under <extract>/data/.
    n = 0
    for name in z.namelist():
        # P5.3 increment 4: structure TEMPLATE .nbt files (same rationale as
        # the run_pack extractor above).
        if name.endswith('.nbt') and name.startswith(prefix + 'data/') and '/structure/' in name:
            parts = name[len(prefix) + len('data/'):].split('/')
            dest = os.path.join(dest_root, "data", *parts)
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            open(dest, 'wb').write(z.read(name))
            n += 1
            continue
        if not name.endswith('.json') or not name.startswith(prefix + 'data/'):
            continue
        parts = name[len(prefix) + len('data/'):].split('/')
        if len(parts) < 3:
            continue
        # P5.3 increment 6 FIX (pack-cell beardifier feed): 'tags' was missing
        # here — the pack's data/<ns>/tags/worldgen/biome/has_structure/*.json
        # files never reached the extract, so "#terralith:has_structure/*"
        # candidate biome predicates resolved as unknown and EVERY pack
        # adapting start was biome-rejected (feed 0/N for pack structures;
        # java placed them -> missing beard_thin/bury fill -> block flips).
        # The main run_pack extractor above already carries the 3-tuple.
        if parts[1] not in ('worldgen', 'dimension', 'tags'):
            continue
        if parts[1] == 'worldgen' and len(parts) < 4:
            continue
        dest = os.path.join(dest_root, 'data', *parts)
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        open(dest, 'wb').write(z.read(name))
        n += 1
    return n

n = extract_worldgen('')
print(f"extracted {n} base worldgen json files")
for oi, entry in enumerate(matching_overlays):
    prefix = entry.get('directory', '').strip('/') + '/'
    n = extract_worldgen(prefix)
    print(f"overlay[{oi}] '{prefix}': {n} json files (last match wins)")
PY
    log "extract-only complete: $EXTRACT_X"
    exit 0
fi

run_pack "terralith_s$SEED" "terralith"
run_pack "tectonic_s$SEED" "tectonic"
# task 5 (P0.4 tail): Structory ships as a mod jar whose data/ tree is a
# worldgen datapack — same extraction path. Exact-version pin picks 1.3.17
# (the 1.21.10 build). BACAP is NOT on Modrinth (CurseForge-only) — kept as
# a known gap in docs/NCF_WORKLOG.txt P0.4.
run_pack "structory_s$SEED" "structory"

pkill -f 'purpur-1.21.10.jar' 2>/dev/null || true
log "ALL GREEN — datapack corpora complete (terralith + tectonic + structory)"
