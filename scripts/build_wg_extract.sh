#!/bin/bash
# Rebuild the jar-fresh vanilla worldgen extract locally (protocol of
# bench/golden/ci_gate_p2.sh, minus the remap step: data/*.json + structure
# .nbt are mapping-independent, so the paperclip-patched jar is sufficient).
# Output: /tmp/wg-extract (bench default NCF_WG).
set -euo pipefail
SERVER_DIR="${NCF_SERVER_DIR:-/home/z/my-project/wg-build/purpur-server}"
EXTRACT="${NCF_OUT:-/tmp/wg-extract}"
PURPUR_URL="https://api.purpurmc.org/v2/purpur/1.21.10/2535/download"
mkdir -p "$SERVER_DIR/versions"
if [ ! -f "$SERVER_DIR/versions/purpur-1.21.10.jar" ] || [ "$(stat -c%s "$SERVER_DIR/versions/purpur-1.21.10.jar" 2>/dev/null || echo 0)" -lt 50000000 ]; then
    echo "[extract] downloading purpur 2535..."
    curl -fsSL --retry 3 -o "$SERVER_DIR/versions/purpur-1.21.10.jar" "$PURPUR_URL"
    SZ=$(stat -c%s "$SERVER_DIR/versions/purpur-1.21.10.jar" 2>/dev/null || echo 0)
    [ "$SZ" -gt 50000000 ] || { echo "[extract] FATAL jar truncated ($SZ bytes)"; exit 1; }
fi
if [ ! -f "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" ] || [ "$(stat -c%s "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar" 2>/dev/null || echo 0)" -lt 50000000 ]; then
    echo "[extract] paperclip patchOnly (java)..."
    command -v java >/dev/null || { echo "[extract] FATAL no java"; exit 1; }
    rm -f "$SERVER_DIR/versions/1.21.10/purpur-1.21.10.jar"
    cd "$SERVER_DIR"
    java -Dpaperclip.patchOnly=true -jar versions/purpur-1.21.10.jar > patchonly.log 2>&1
    [ -f versions/1.21.10/purpur-1.21.10.jar ] || { echo "[extract] FATAL no patched jar"; tail -5 patchonly.log; exit 1; }
    SZ=$(stat -c%s versions/1.21.10/purpur-1.21.10.jar)
    [ "$SZ" -gt 20000000 ] || { echo "[extract] FATAL patched jar truncated ($SZ)"; tail -5 patchonly.log; exit 1; }
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
