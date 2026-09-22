#!/usr/bin/env bash
# check_blobs_sync.sh — javap-gate against the ×93 blob-sync lesson
# (TASK-414-B: cv3b-1 boot-log 'items_restplane ARMED' absent, items lane 34.23%
# — java gate flags were extended on the rust side but the java-side gates of
# ItemEntityManager/MobAiOps/MobScanOps did not accept the composite flag).
#
# For every lever bridge class:
#   1. the .class blob must exist and be a valid Java 21 (major 65) classfile;
#   2. javap of the blob must contain the expected ARM marker strings;
#   3. javap of the blob must contain the expected flag strings (the composite
#      gate lives in the CONSTANT POOL of the blob — a source-only edit without
#      a rebuild fails here);
#   4. expected native method declarations must be present (javap -p).
#
# Usage: scripts/check_blobs_sync.sh [javac-home]   (exit 0 = in sync)
set -uo pipefail
cd "$(dirname "$0")/.."

JAVAP=/home/z/tools/jdk-21.0.12.1+1/bin/javap
[ -x "$JAVAP" ] || JAVAP=$(command -v javap)
[ -x "$JAVAP" ] || { echo "FAIL: no javap" >&2; exit 1; }

FAIL=0
note() { printf '  %s\n' "$1"; }
die()  { printf 'FAIL: %s\n' "$1" >&2; FAIL=1; }

check_class() { # blob expected_native... — then markers via MARKERS_<n>
  local blob="$1"; shift
  if [ ! -f "$blob" ]; then die "missing blob: $blob"; return; fi
  # major version gate (--release 21 => major 65)
  local major
  major=$(python3 - "$blob" << 'PY'
import sys
b = open(sys.argv[1], 'rb').read(8)
print((b[6] << 8) | b[7])
PY
)
  [ "$major" = "65" ] || die "$blob: major $major != 65 (rebuild with --release 21)"
  local javap_out
  javap_out=$("$JAVAP" -p -c "$blob" 2>&1) || die "$blob: javap failed"
  for marker in "$@"; do
    if [[ "$javap_out" == *"$marker"* ]]; then
      note "$blob: OK marker '$marker'"
    else
      die "$blob: expected marker/flag '$marker' NOT in javap output (stale blob or missing gate)"
    fi
  done
}

echo "== javap-gate: lever bridge blobs vs ARM markers / gate flags =="

check_class \
  "entityinside/build/net/minecraft/world/entity/ItemEntityManager.class" \
  "items_restplane ARMED" "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" \
  "native int idxProbe" "static void indexAdd" "native int lifetimeDue"

check_class \
  "mobai/build/net/minecraft/world/entity/MobAiOps.class" \
  "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" "cmp416_gsel3" \
  "native"

check_class \
  "gsel/build/net/minecraft/world/entity/ai/goal/GoalBatchOps.class" \
  "cmp416_gsel3" "tickGate" \
  "native int gselProbe" "native int gselRegister" "native int gselEpoch"

check_class \
  "sscan/build/net/minecraft/world/entity/MobScanOps.class" \
  "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" \
  "native"

check_class \
  "mobpush/build/net/minecraft/world/entity/MobPushOps.class" \
  "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" \
  "native"

check_class \
  "entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class" \
  "cmp414_cvs" "cmp412_eqsnapv3" \
  "native int eqProbe"

# gate-flag consistency: every flag string accepted by the SOURCE gate must
# also be present in the BLOB constant pool (covers the ×93 rebuild lesson).
for pair in \
  "mobai/net/minecraft/world/entity/MobAiOps.java:mobai/build/net/minecraft/world/entity/MobAiOps.class" \
  "sscan/net/minecraft/world/entity/MobScanOps.java:sscan/build/net/minecraft/world/entity/MobScanOps.class" \
  "mobpush/net/minecraft/world/entity/MobPushOps.java:mobpush/build/net/minecraft/world/entity/MobPushOps.class" \
  "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java:entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class" \
  "entityinside/net/minecraft/world/entity/ItemEntityManager.java:entityinside/build/net/minecraft/world/entity/ItemEntityManager.class" \
  "gsel/net/minecraft/world/entity/ai/goal/GoalBatchOps.java:gsel/build/net/minecraft/world/entity/ai/goal/GoalBatchOps.class"
do
  src="${pair%%:*}"; blob="${pair##*:}"
  flags=$(grep -o '"cmp[0-9_a-z]*"' "$src" | tr -d '"' | sort -u)
  jp=$("$JAVAP" -p -c "$blob" 2>/dev/null)
  for f in $flags; do
    if [[ "$jp" == *"$f"* ]]; then
      :
    else
      die "$blob: source gate flag '$f' missing from blob constant pool — REBUILD"
    fi
  done
  note "$src <-> $blob: $(printf '%s\n' "$flags" | wc -l) gate flags in sync"
done

if [ "$FAIL" = "0" ]; then
  echo "check_blobs_sync: ALL IN SYNC"
  exit 0
else
  echo "check_blobs_sync: BLOB-SYNC VIOLATION (see FAIL lines)" >&2
  exit 1
fi
