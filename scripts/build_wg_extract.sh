#!/bin/bash
# Rebuild the jar-fresh vanilla worldgen extract locally (protocol of
# bench/golden/ci_gate_p2.sh, minus the remap step: data/*.json + structure
# .nbt are mapping-independent, so the paperclip-patched jar is sufficient).
# Output: /tmp/wg-extract (bench default NCF_WG).
set -euo pipefail
SERVER_DIR=/tmp/purpur-server
EXTRACT=/tmp/wg-extract
PURPUR_URL="https://api.purpurmc.org/v2/purpur/1.21.10/2535/download"
mkdir -p "$SERVER_DIR/versions"
if [ ! -f "$SERVER_DIR/versions/purpur-1.21.10.jar" ]; then
    echo "[extract] downloading purpur 2535..."
    curl -fsSL --retry 3 -o "$SERVER_DIR/versions/purpur-1.21.10.jar" "$PURPUR_URL"
fi
if [ ! -f "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" ]; then
    echo "[extract] paperclip patchOnly (java)..."
    cd "$SERVER_DIR"
    java -Dpaperclip.patchOnly=true -jar versions/purpur-1.21.10.jar > patchonly.log 2>&1 &
    for i in $(seq 1 120); do
        [ -f versions/1.21.10/purpur-1.21.10.jar ] && break
        sleep 2
    done
    [ -f versions/1.21.10/purpur-1.21.10.jar ] || { echo "[extract] FATAL no patched jar"; tail -5 patchonly.log; exit 1; }
fi
mkdir -p "$EXTRACT"
python3 - "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" "$EXTRACT" <<'PY'
import zipfile, os, sys
z = zipfile.ZipFile(sys.argv[1])
n = 0
for name in z.namelist():
    is_json = name.endswith('.json') and (
        name.startswith('data/minecraft/worldgen/') or name.startswith('data/minecraft/tags/block/')
        or name.startswith('data/minecraft/tags/worldgen/')
    )
    is_nbt = name.endswith('.nbt') and name.startswith('data/minecraft/structure/')
    if is_json or is_nbt:
        dest = os.path.join(sys.argv[2], *name.split('/'))
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        open(dest, 'wb').write(z.read(name))
        n += 1
print(f"extracted {n} json files")
PY
echo "[extract] DONE: $EXTRACT"
