#!/usr/bin/env bash
# check_blobs_sync.sh — TASK-417-C javap-gate against the ×93 blob-sync lesson
# (ported from 1aec4f8 @round-414, extended: queryplane + entitygoalquery +
# cmp417_bq gate flags + flat==nested byte identity. ×93 discipline:
# include_bytes! embeds the NESTED path; a flat-only refresh leaves the
# embedded blob stale = dormant plane).
#
# For every lever bridge class:
#   1. the NESTED .class blob (the path include_bytes! actually embeds) must
#      exist, be major 65, and match its FLAT sibling byte-for-byte;
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
    elif python3 -c "import sys; sys.exit(0 if b'$marker' in open(sys.argv[1],'rb').read() else 1)" "$blob"; then
      # indy-recipe lesson (x93/420a): concat constants fold into bootstrap
      # method recipes — invisible to javap -c, alive in raw constant pool.
      note "$blob: OK marker '$marker' (raw-byte cp grep, indy recipe)"
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
    die "$nested != $flat — rebuild via scripts/build_417c_blobs_all.sh (×93: nested is what include_bytes! embeds)"
  else
    note "$nested == flat: byte-identical"
  fi
}

echo "== javap-gate: lever bridge blobs vs ARM markers / gate flags (lever cmp417_bq / cmp421_brain) =="

check_class \
  "entityinside/build/net/minecraft/world/entity/ItemEntityManager.class" \
  "items_restplane ARMED" "cmp417_bq" "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" \
  "native int idxProbe" "static void indexAdd" "native int lifetimeDue"

check_class \
  "goalops/build/net/minecraft/world/entity/ai/goal/GoalOps.class" \
  "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "goal-selector EFFECT armed" "goalCleanup" "goalUpdate" \
  "goal-selector running EFFECT armed" \
  "tickGate" "tickRunningGate" "availableGoals" "lockedFlags" "goalTypes"

check_class \
  "queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class" \
  "cmp417_bq" "cmp420_colpush" "cmp412_b2p1" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "selfTest" "isHardCollidingProbe"

check_class \
  "mobai/build/net/minecraft/world/entity/MobAiOps.class" \
  "cmp417_bq" "cmp420_colpush" "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" \
  "native"

check_class \
  "sscan/build/net/minecraft/world/entity/MobScanOps.class" \
  "cmp417_bq" "cmp420_colpush" "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" \
  "native"

check_class \
  "sense/build/net/minecraft/world/entity/SenseOps.class" \
  "cmp438_sense" "cmp430_inside" "cmp451_senseins" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "nearestEntityGate" "sense EFFECT" "selfTest" \
  "native int senseProbe" "native int senseEpoch"

check_class \
  "randomtick/build/net/minecraft/world/entity/ai/BrainOps.class" \
  "cmp438_sense" "cmp439_sense_scan" "cmp451_senseins" "cmp452_mega" "selfTestTickEach" "tickEachRunning" \
  "sense tick2 EFFECT armed"

check_class \
  "mobpush/build/net/minecraft/world/entity/MobPushOps.class" \
  "cmp417_bq" "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" "cmp420_colpush" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" \
  "native int mobProbe" "boxFor" "colpushSweep"

check_class \
  "colpush/build/net/minecraft/world/entity/ColpushOps.class" \
  "cmp420_colpush" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "pushEntities" "bulkTick" "selfTest" "armed" \
  "native int colpushProbe" "native int colpushTick"

check_class \
  "entityinside/build/net/minecraft/world/entity/RegionTickOps.class" \
  "COLPUSH_ON" "COLPUSH_BROKEN" "ColpushOps.bulkTick:()V"

check_class \
  "entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class" \
  "cmp414_cvs" "cmp412_eqsnapv3" "cmp420_colpush" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" \
  "native int eqProbe" "native int senseArena"

# TASK-420-C chunk-pipeline plane (cmp420_chunk2): the bridge must carry the
# lever marker + the parse-cache effect strings in its constant pool, and
# declare the redirect entry points (descriptor pinned by build script javap
# grep; flat-only pinned by build script '$' guard + rust delivery test).
# TASK-430-B inside-plane subsystem (cmp430_inside): the snapshot-plane
# bridge declares the two retarget statics (snapGet/secWrite) + the native
# table (snapProbe/snapCollect) + selfTest; the bitmask pre-gate bridge
# declares its entry (checkInsideBlocksGated) + armState probe. Lever wiring
# lives rust-side (STRICT eq cmp430_inside) — these blobs carry no lever id.
check_class \
  "entityinside/build/net/minecraft/world/entity/InsideSnapOps.class" \
  "snapGet" "secWrite" "selfTest" "inside_snap: first gate HIT served" \
  "native int snapProbe" "native int snapCollect"

check_class \
  'entityinside/build/net/minecraft/world/entity/InsideSnapOps$Snap.class' \
  "builtAtGen"

check_class \
  "entityinside/build/net/minecraft/world/entity/InsideBitmaskOps.class" \
  "checkInsideBlocksGated" "armState" "sweptHullInto"

# TASK-432-B inside-plane deepening: the inside_cache gate bridge carries the
# fusion hook (SNAP_ARMED note) + the fused read helper + the widened slot
# space; its bytes must show the snapGet ref (fusion) and the 2^18 slot cap.
check_class \
  "entityinside/build/net/minecraft/world/entity/InsideBlockOps.class" \
  "noteSnapArmed" "snapGet" "bstate" "SNAP_ARMED" "mirror"

check_class \
  "chunkparse/build/net/minecraft/world/level/chunk/storage/ChunkParseOps.class" \
  "cmp420_chunk2" "cmp420_colpush" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp453_diet" "parse-cache first hit" "parse-cache selftest" \
  "biomes-cache first hit" "biomes selftest" \
  "public static void init" "parseSection" "parseBiomesSection"

# TASK-438-C chunk-send serialization snapshot (cmp437_chunk4): the bridge
# must carry the lever + carrier ids and the effect-marker strings, and
# declare the redirect entry points (descriptor pinned by the build script
# javap grep; flat-only pinned by build script '$' guard + rust delivery test).
check_class \
  "chunksend/build/net/minecraft/server/network/ChunkSendOps.class" \
  "cmp437_chunk4" "cmp435_chunk3" "cmp444_chunk5" "cmp450_chunk" "cmp453_diet" "chunk4 send-snapshot first hit" "chunk4 snapshot selftest PASS" "chunk4 stats" \
  "public static void sendChunk" "public static boolean selfTest"

# TASK-444-B chunk-packet encode cache (cmp444_chunk5): stage-2 bridge on top
# of the chunk4 send plane — encode-once capture + byte[] replay per player,
# keyed by packet instance. Flat-only pinned by build script '$' guard + rust
# delivery test; descriptor pinned by the build script javap grep.
check_class \
  "chunksend/build/net/minecraft/server/network/ChunkPacketEncodeOps.class" \
  "cmp444_chunk5" "cmp450_chunk" "cmp437_chunk4" "cmp435_chunk3" "chunk5 payload-cache first hit" "chunk5 payload selftest PASS" "chunk5 stats" \
  "public static void write" "public static boolean selfTest"

# TASK-421-C noise-blob coverage: the GEN-axis bridge family (noise/build,
# NOISE_RELEASE=8 => major 52) was OUTSIDE this gate — the only lever family
# whose blobs check_blobs_sync never audited (the x93 gate hole). The blobs
# are include_bytes!'d by src/noise_fill.rs (levelgen package) and
# src/improved_noise.rs (synth package); the embed-set audit lives in
# build_noise.sh, here we pin: major version + the bridge entry methods that
# the whole-body redirects target (a stale/partial rebuild loses them).
noise_check_class() { # blob marker... — major-52 variant of check_class
  local blob="$1"; shift
  if [ ! -f "$blob" ]; then die "missing blob: $blob"; return; fi
  local major
  major=$(python3 - "$blob" << 'PY'
import sys
b = open(sys.argv[1], 'rb').read(8)
print((b[6] << 8) | b[7])
PY
)
  [ "$major" = "52" ] || die "$blob: major $major != 52 (noise bridge = NOISE_RELEASE 8; >65 would fail the runtime guard)"
  local javap_out
  javap_out=$("$JAVAP" -p -c "$blob" 2>&1) || die "$blob: javap failed"
  for marker in "$@"; do
    if [[ "$javap_out" == *"$marker"* ]]; then
      note "$blob: OK marker '$marker' (major 52)"
    else
      die "$blob: expected method marker '$marker' NOT in javap output (stale blob or missing gate)"
    fi
  done
}

noise_check_class \
  "noise/build/net/minecraft/world/level/levelgen/NormalNoiseBatchOps.class" \
  "fillNoise" "fillShift" "selfTest" "nativeFillScaledPositions"
noise_check_class \
  "noise/build/net/minecraft/world/level/levelgen/DensityArrayInterpreter.class" \
  "interpFillArray"
noise_check_class \
  "noise/build/net/minecraft/world/level/levelgen/synth/ImprovedNoiseBatchOps.class" \
  "public static double noise" "selfTestFlush"
noise_check_class \
  "noise/build/net/minecraft/world/level/levelgen/synth/PerlinNoiseNativeOps.class" \
  "native"

# gate-flag consistency: every flag string accepted by the SOURCE gate must
# also be present in the BLOB constant pool (covers the ×93 rebuild lesson).
for pair in \
  "mobai/net/minecraft/world/entity/MobAiOps.java:mobai/build/net/minecraft/world/entity/MobAiOps.class" \
  "sscan/net/minecraft/world/entity/MobScanOps.java:sscan/build/net/minecraft/world/entity/MobScanOps.class" \
  "mobpush/net/minecraft/world/entity/MobPushOps.java:mobpush/build/net/minecraft/world/entity/MobPushOps.class" \
  "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java:entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class" \
  "entityinside/net/minecraft/world/entity/ItemEntityManager.java:entityinside/build/net/minecraft/world/entity/ItemEntityManager.class" \
  "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java:entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class" \
  "queryplane/net/minecraft/world/entity/QueryPlaneOps.java:queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class" \
  "goalops/net/minecraft/world/entity/ai/goal/GoalOps.java:goalops/build/net/minecraft/world/entity/ai/goal/GoalOps.class" \
  "colpush/net/minecraft/world/entity/ColpushOps.java:colpush/build/net/minecraft/world/entity/ColpushOps.class" \
  "entityinside/net/minecraft/world/entity/RegionTickOps.java:entityinside/build/net/minecraft/world/entity/RegionTickOps.class" \
  "chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java:chunkparse/build/net/minecraft/world/level/chunk/storage/ChunkParseOps.class"
do
  src="${pair%%:*}"; blob="${pair##*:}"
  flags=$(grep -o '"cmp[0-9_a-z]*"' "$src" | tr -d '"' | sort -u)
  # TASK-420-A: raw-byte grep instead of javap output — gate strings inside
  # indy makeConcatWithConstants recipes never show in javap -c, but ARE in
  # the classfile constant pool (raw bytes = cp truth).
  for f in $flags; do
    if python3 - "$blob" "$f" << 'PY'
import sys
b = open(sys.argv[1], 'rb').read()
sys.exit(0 if sys.argv[2].encode() in b else 1)
PY
    then
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
check_flat_matches_nested "sscan/build" "net/minecraft/world/entity/MobPushOps"
check_flat_matches_nested "mobai/build" "net/minecraft/world/entity/MobAiOps"
check_flat_matches_nested "entityinside/build" "net/minecraft/world/entity/ItemEntityManager"
check_flat_matches_nested "entitygoalquery/build" "net/minecraft/world/entity/EntityGoalQueryOps"
check_flat_matches_nested "queryplane/build" "net/minecraft/world/entity/QueryPlaneOps"
check_flat_matches_nested "goalops/build" "net/minecraft/world/entity/ai/goal/GoalOps"
check_flat_matches_nested "colpush/build" "net/minecraft/world/entity/ColpushOps"
check_flat_matches_nested "entityinside/build" "net/minecraft/world/entity/RegionTickOps"
check_flat_matches_nested "sense/build" "net/minecraft/world/entity/SenseOps"
check_flat_matches_nested "chunksend/build" "net/minecraft/server/network/ChunkSendOps"
check_flat_matches_nested "chunksend/build" "net/minecraft/server/network/ChunkPacketEncodeOps"

# TASK-451-D x449-lesson javap-LOADABILITY gate: javap must locate+parse EVERY
# op class (outer AND inner) through its blobs dir via a real classpath — the
# pre-dispatch catch for blob-set holes (a class javap cannot load from the
# blobs dir is exactly the class the kernel loader would NCDFE on).
gate_load() { # dir fqcn-slash — fail if javap cannot load
  local dir="$1" fq="$2" fq_dots
  fq_dots="${fq//\//.}"
  if "$JAVAP" -p -cp "$dir" "$fq_dots" >/dev/null 2>&1; then
    note "javap-load OK: $fq_dots"
  else
    die "javap loadability FAIL: $fq_dots (cp=$dir) — blob missing/stale"
  fi
}
gate_load entityinside/build   net/minecraft/world/entity/ItemEntityManager
gate_load entityinside/build   net/minecraft/world/entity/InsideSnapOps
gate_load entityinside/build   'net/minecraft/world/entity/InsideSnapOps$Snap'
gate_load goalops/build        net/minecraft/world/entity/ai/goal/GoalOps
gate_load queryplane/build     net/minecraft/world/entity/QueryPlaneOps
gate_load mobai/build          net/minecraft/world/entity/MobAiOps
gate_load sscan/build          net/minecraft/world/entity/MobScanOps
gate_load sscan/build          net/minecraft/world/entity/MobPushOps
gate_load mobpush/build        net/minecraft/world/entity/MobPushOps
gate_load colpush/build        net/minecraft/world/entity/ColpushOps
gate_load entitygoalquery/build net/minecraft/world/entity/EntityGoalQueryOps
gate_load sense/build          net/minecraft/world/entity/SenseOps
gate_load randomtick/build     net/minecraft/world/entity/ai/BrainOps
gate_load chunkparse/build     net/minecraft/world/level/chunk/storage/ChunkParseOps
gate_load chunksend/build      net/minecraft/server/network/ChunkSendOps
gate_load chunksend/build      net/minecraft/server/network/ChunkPacketEncodeOps

if [ "$FAIL" = "0" ]; then
  echo "check_blobs_sync: ALL IN SYNC"
  exit 0
else
  echo "check_blobs_sync: BLOB-SYNC VIOLATION (see FAIL lines)" >&2
  exit 1
fi
