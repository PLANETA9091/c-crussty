#!/usr/bin/env bash
# check_blobs_sync.sh — TASK-416-A javap-gate against the ×93 blob-sync lesson
# (ported from 1aec4f8 @round-414, extended: queryplane + gsel + item_merge +
# auto flag-consistency for cmp416_mcomp; ×93 ×4 повтора за 2 тика:
# merge усыпил плоскости — mc1/mc2/mc3 RED @round-415-a-mcomp).
#
# For every lever bridge class:
#   1. the NESTED .class blob (the path include_bytes! actually embeds) must
#      exist, be major 65, and match its FLAT sibling byte-for-byte (the
#      round-415 rebuild-script bug: flat refreshed, nested stale);
#   2. javap of the blob must contain the expected ARM marker strings;
#   3. javap of the blob must contain EVERY flag string accepted by the
#      SOURCE gate (composite gate lives in the CONSTANT POOL — a source-only
#      edit without a rebuild fails here);
#   4. expected native declarations must be present (javap -p).
#
# Usage: scripts/check_blobs_sync.sh   (exit 0 = in sync)
set -uo pipefail
cd "$(dirname "$0")/.."

JAVAP=/home/z/tools/jdk-21.0.12.1+1/bin/javap
[ -x "$JAVAP" ] || JAVAP=$(command -v javap)
[ -x "$JAVAP" ] || { echo "FAIL: no javap" >&2; exit 1; }

FAIL=0
note() { printf '  %s\n' "$1"; }
die()  { printf 'FAIL: %s\n' "$1" >&2; FAIL=1; }

check_class() { # blob marker... — nested blob + javap markers
  local blob="$1"; shift
  if [ ! -f "$blob" ]; then die "missing blob: $blob"; return; fi
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

check_flat_matches_nested() { # fqcn-dir fqcn — flat copy == nested copy
  local dir="$1" fqcn="$2"
  local nested="$dir/$fqcn.class" flat="$dir/$(basename "$fqcn").class"
  if [ ! -f "$nested" ]; then die "missing nested blob: $nested"; return; fi
  if [ ! -f "$flat" ]; then die "missing flat blob: $flat (legacy path)"; return; fi
  if ! cmp -s "$nested" "$flat"; then
    die "$nested != $flat — rebuild via scripts/build_mcomp_blobs_all.sh (×93: nested is what include_bytes! embeds)"
  else
    note "$nested == flat: byte-identical"
  fi
}

echo "== javap-gate: lever bridge blobs vs ARM markers / gate flags (lever cmp416_mcomp) =="

check_class \
  "entityinside/build/net/minecraft/world/entity/ItemEntityManager.class" \
  "items_restplane ARMED" "cmp416_mcomp" "cmp409_multi" "cmp415_mcomp" \
  "native int idxProbe" "static void indexAdd" "native int lifetimeDue"

check_class \
  "mobai/build/net/minecraft/world/entity/MobAiOps.class" \
  "cmp416_mcomp" "cmp409_multi" "cmp415_mcomp" "native"

check_class \
  "sscan/build/net/minecraft/world/entity/MobScanOps.class" \
  "cmp416_mcomp" "cmp409_multi" "cmp415_mcomp" "native"

check_class \
  "mobpush/build/net/minecraft/world/entity/MobPushOps.class" \
  "cmp416_mcomp" "cmp409_multi" "cmp415_mcomp" "native"

check_class \
  "queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class" \
  "cmp416_mcomp" "cmp412_b2p1" "selfTest" "isHardCollidingProbe"

check_class \
  "gsel/build/net/minecraft/world/entity/ai/goal/GoalBatchOps.class" \
  "cmp416_mcomp" "cmp414_pfb" "tickGate" "native int gselEpoch"

check_class \
  "items/build/net/minecraft/world/entity/item/ItemMergeOps.class" \
  "mergeWithNeighbours" "tryToMerge"

# gate-flag consistency: every flag string accepted by the SOURCE gate must
# also be present in the BLOB constant pool (covers the ×93 rebuild lesson).
for pair in \
  "mobai/net/minecraft/world/entity/MobAiOps.java:mobai/build/net/minecraft/world/entity/MobAiOps.class" \
  "sscan/net/minecraft/world/entity/MobScanOps.java:sscan/build/net/minecraft/world/entity/MobScanOps.class" \
  "mobpush/net/minecraft/world/entity/MobPushOps.java:mobpush/build/net/minecraft/world/entity/MobPushOps.class" \
  "entityinside/net/minecraft/world/entity/ItemEntityManager.java:entityinside/build/net/minecraft/world/entity/ItemEntityManager.class" \
  "queryplane/net/minecraft/world/entity/QueryPlaneOps.java:queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class" \
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

# flat-vs-nested byte identity (round-415 rebuild-script bug root-cause)
check_flat_matches_nested "mobpush/build" "net/minecraft/world/entity/MobPushOps"
check_flat_matches_nested "sscan/build" "net/minecraft/world/entity/MobScanOps"
check_flat_matches_nested "mobai/build" "net/minecraft/world/entity/MobAiOps"
check_flat_matches_nested "entityinside/build" "net/minecraft/world/entity/ItemEntityManager"
check_flat_matches_nested "queryplane/build" "net/minecraft/world/entity/QueryPlaneOps"
check_flat_matches_nested "gsel/build" "net/minecraft/world/entity/ai/goal/GoalBatchOps"
check_flat_matches_nested "items/build" "net/minecraft/world/entity/item/ItemMergeOps"

if [ "$FAIL" = "0" ]; then
  echo "check_blobs_sync: ALL IN SYNC"
  exit 0
else
  echo "check_blobs_sync: BLOB-SYNC VIOLATION (see FAIL lines)" >&2
  exit 1
fi
