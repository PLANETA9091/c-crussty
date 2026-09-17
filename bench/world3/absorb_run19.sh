#!/usr/bin/env bash
# =============================================================================
# absorb_run17.sh — download + validate BENCH-4 validation-run artifacts
# (run 35156292165, fake_players=4, dispatch S7-99). Idempotent.
#
# Usage: bash absorb_run17.sh [RUN_ID]  (default 35156292165)
# Lands in /home/z/my-project/scripts/bench3_research/run19/
# =============================================================================
set -uo pipefail
RUN_ID="${1:-35163894978}"
DEST="/home/z/my-project/scripts/bench3_research/run19"
CREDS="$HOME/.git-credentials"
TOKEN="$(sed -n 's|^https://agent-7625532f:\([^@]*\)@github.com$|\1|p' "$CREDS" | head -1)"
[ -n "$TOKEN" ] || { echo "FATAL: no token in $CREDS (run bootstrap_tick.sh first)"; exit 1; }

mkdir -p "$DEST"
cd "$DEST" || exit 1

if [ -s ".absorbed" ]; then echo "run $RUN_ID already absorbed at $DEST"; exit 0; fi

echo "querying artifact for run $RUN_ID..."
ART_JSON="$(curl -s -H "Authorization: Bearer $TOKEN" \
  "https://api.github.com/repos/PLANETA9091/c-crussty/actions/runs/$RUN_ID/artifacts")"
ART_URL="$(echo "$ART_JSON" | python3 -c "
import json,sys
d=json.load(sys.stdin)
for a in d.get('artifacts', []):
    if a['name'] == 'world3-bench':
        print(a['archive_download_url']); break
")"
if [ -z "$ART_URL" ]; then
  echo "artifact 'world3-bench' not found — run not finished yet? full list:"
  echo "$ART_JSON" | python3 -c "import json,sys; [print(' -',a['name'],a['size_in_bytes']) for a in json.load(sys.stdin).get('artifacts',[])]" 2>/dev/null
  exit 2
fi

echo "downloading world3-bench..."
curl -sL -H "Authorization: Bearer $TOKEN" -o artifact.zip "$ART_URL"
ls -la artifact.zip
unzip -o -q artifact.zip -d x && rm artifact.zip
mv x/* . 2>/dev/null; rmdir x 2>/dev/null
ls -la

# --- validation gates (honest absorb) --------------------------------------
FAIL=0
[ -s BOTTLENECKS_3.md ] || { echo "MISSING BOTTLENECKS_3.md"; FAIL=1; }
[ -s run-env.txt ] || { echo "MISSING run-env.txt"; FAIL=1; }
[ -s cpu-collapsed.txt ] || echo "WARN: cpu-collapsed.txt absent"
grep -q "FIXTURE-VALIDITY: VALID" BOTTLENECKS_3.md 2>/dev/null \
  && echo "GATE: FIXTURE-VALIDITY VALID" \
  || { echo "GATE: FIXTURE-VALIDITY not VALID (check BOTTLENECKS_3.md)"; FAIL=1; }
grep -q "fake_players: 4" run-env.txt 2>/dev/null && echo "GATE: run-env fake_players=4" || { echo "GATE: run-env fake_players!=4"; FAIL=1; }
grep -q "BenchFakePlayers] registered" server-stdout.log 2>/dev/null \
  && echo "GATE: plugin registered players" || { echo "GATE: no BenchFakePlayers registration lines"; FAIL=1; }

if [ "$FAIL" = "0" ]; then
  echo "run $RUN_ID absorbed OK: $(date -u +%FT%TZ)" > .absorbed
  echo "NEXT: python3 /home/z/c-crussty/bench/world3/recon_lanes.py $DEST --diff <run16-dir>"
else
  echo "VALIDATION FAILED — artifact present but gates failed (inspect files above)"
fi
