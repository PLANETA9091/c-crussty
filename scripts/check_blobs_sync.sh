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
S16_SELF="$(readlink -f "$0")"
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
  "cmp466_c98ai" "items_restplane ARMED" "cmp417_bq" "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" \
  "native int idxProbe" "static void indexAdd" "native int lifetimeDue"

check_class \
  "goalops/build/net/minecraft/world/entity/ai/goal/GoalOps.class" \
  "cmp466_c98ai" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "goal-selector EFFECT armed" "goalCleanup" "goalUpdate" "cmp456_poi" \
  "goal-selector running EFFECT armed" \
  "tickGate" "tickRunningGate" "availableGoals" "lockedFlags" "goalTypes"

check_class \
  "queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class" \
  "cmp466_c98ai" "cmp417_bq" "cmp420_colpush" "cmp412_b2p1" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "selfTest" "isHardCollidingProbe"

check_class \
  "mobai/build/net/minecraft/world/entity/MobAiOps.class" \
  "cmp466_c98ai" "cmp417_bq" "cmp420_colpush" "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp456_poi" \
  "native"

check_class \
  "sscan/build/net/minecraft/world/entity/MobScanOps.class" \
  "cmp466_c98ai" "cmp417_bq" "cmp420_colpush" "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp456_poi" \
  "native"

check_class \
  "sense/build/net/minecraft/world/entity/SenseOps.class" \
  "cmp466_c98ai" "cmp456_poi" "cmp438_sense" "cmp430_inside" "cmp451_senseins" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "nearestEntityGate" "sense EFFECT" "selfTest" \
  "native int senseProbe" "native int senseEpoch"

check_class \
  "randomtick/build/net/minecraft/world/entity/ai/BrainOps.class" \
  "cmp438_sense" "cmp439_sense_scan" "cmp451_senseins" "cmp452_mega" "selfTestTickEach" "tickEachRunning" \
  "sense tick2 EFFECT armed"

check_class \
  "mobpush/build/net/minecraft/world/entity/MobPushOps.class" \
  "cmp466_c98ai" "cmp417_bq" "cmp414_cvs" "cmp412_meganav" "cmp412_eqsnapv3" "cmp420_colpush" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp456_poi" \
  "native int mobProbe" "boxFor" "colpushSweep"

check_class \
  "colpush/build/net/minecraft/world/entity/ColpushOps.class" \
  "cmp466_c98ai" "cmp420_colpush" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "pushEntities" "bulkTick" "selfTest" "armed" "cmp456_poi" \
  "native int colpushProbe" "native int colpushTick"

check_class \
  "entityinside/build/net/minecraft/world/entity/RegionTickOps.class" \
  "COLPUSH_ON" "COLPUSH_BROKEN" "ColpushOps.bulkTick:()V"

check_class \
  "entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class" \
  "cmp466_c98ai" "cmp414_cvs" "cmp412_eqsnapv3" "cmp420_colpush" "cmp421_brain" "cmp422_brain2" "cmp430_inside" "cmp432_inside2" "cmp451_senseins" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp444_chunk5" "cmp450_chunk" "cmp452_mega" "cmp453_diet" "cmp457_eqsnap2" "cmp456_chunkmono" "cmp456_chunkmono_p31snap" "cmp456_poi" \
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
  "cmp420_chunk2" "cmp420_colpush" "cmp434_chunkpl" "cmp435_chunk3" "cmp437_chunk4" "cmp453_diet" "parse-cache first hit" "parse-cache selftest" "cmp456_poi" \
  "biomes-cache first hit" "biomes selftest" \
  "public static void init" "parseSection" "parseBiomesSection"

# ROUND-468 S56 hole-closure (x425 sleeping-gate lesson): 9 build dirs / 20
# tracked blobs lived OUTSIDE this gate while LIVE-wired — fluid_guard=1 is
# in EVERY bank-v5 dispatch (fluid/build), poi/chunksched/entityquery are
# era-carrier lanes (cmp456_poi / chunkmono acceptors), paletted patches the
# kernel classfile, stagger/items are armed-or-dormant planes. redstone/
# and area-map probe outputs are UNTRACKED lab artifacts on c1196321 (no
# include_bytes! site in src) — deliberately NOT gated.
# Also: the ERA CARRIER cmp466_c98ai (MERGE #10) was pinned in ZERO
# check_class lists above — the source-flag loop covered only 8/10 carriers
# (sense/SenseOps.class carried cmp466_c98ai with NO gate site at all).
check_class \
  "poi/build/net/minecraft/world/entity/ai/village/poi/PoiOps.class" \
  "cmp456_poi" "updatePoiGate" "poiTickGate" "selfTest" "CRUSSTY_LEVER_FLAG" \
  "native"

check_class \
  "chunksched/build/net/minecraft/server/level/ChunkSchedOps.class" \
  "cmp456_chunkmono" "ARMED chunk-sched" "selfTest" \
  "native"

check_class \
  "fluid/build/net/minecraft/world/entity/FluidPushGuardHook.class" \
  "CRUSSTY_FLUID_BITMASK" "selfTest"

check_class \
  'fluid/build/net/minecraft/world/entity/FluidPushGuardHook$GuardEntry.class'

check_class \
  "fluid/build/net/minecraft/world/entity/FluidBitmaskOps.class" \
  "clean" "CACHE_CAP" "BUILDS_PER_TICK" "selfTest"

check_class \
  'fluid/build/net/minecraft/world/entity/FluidBitmaskOps$Entry.class'

check_class \
  "stagger/build/net/minecraft/world/entity/PushStaggerOps.class" \
  "CRUSSTY_STAGGER_N" "CRUSSTY_LEVER_ARG" "pushables" "bipush        16"

check_class \
  "stagger/build/net/minecraft/world/entity/ai/goal/target/GoalStaggerOps.class" \
  "canUseGate" "CRUSSTY_STAGGER_N" "CRUSSTY_LEVER_ARG" "bipush        16"

check_class \
  "entityquery/build/net/minecraft/world/entity/EntityQueryOps.class" \
  "pushables" "newPushRing"

check_class \
  "entityquery/build/net/minecraft/world/entity/EntityIndexOps.class" \
  "cmp405_eindex" "ARMED" "ERR_STRUCT" \
  "native"

check_class \
  'entityquery/build/net/minecraft/world/entity/EntityIndexOps$Buf.class'

check_class \
  "paletted/build/net/minecraft/world/level/chunk/PalettedContainerOps.class" \
  "ABORTS" "CAPPED" "selfTest"

# TASK-438-C chunk-send serialization snapshot (cmp437_chunk4): the bridge
# must carry the lever + carrier ids and the effect-marker strings, and
# declare the redirect entry points (descriptor pinned by the build script
# javap grep; flat-only pinned by build script '$' guard + rust delivery test).
check_class \
  "chunksend/build/net/minecraft/server/network/ChunkSendOps.class" \
  "cmp437_chunk4" "cmp435_chunk3" "cmp444_chunk5" "cmp450_chunk" "cmp453_diet" "chunk4 send-snapshot first hit" "chunk4 snapshot selftest PASS" "chunk4 stats" "cmp456_poi" \
  "public static void sendChunk" "public static boolean selfTest"

# TASK-444-B chunk-packet encode cache (cmp444_chunk5): stage-2 bridge on top
# of the chunk4 send plane — encode-once capture + byte[] replay per player,
# keyed by packet instance. Flat-only pinned by build script '$' guard + rust
# delivery test; descriptor pinned by the build script javap grep.
check_class \
  "chunksend/build/net/minecraft/server/network/ChunkPacketEncodeOps.class" \
  "cmp444_chunk5" "cmp450_chunk" "cmp437_chunk4" "cmp435_chunk3" "chunk5 payload-cache first hit" "chunk5 payload selftest PASS" "chunk5 stats" "cmp456_poi" \
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
  "chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java:chunkparse/build/net/minecraft/world/level/chunk/storage/ChunkParseOps.class" \
  "sense/net/minecraft/world/entity/SenseOps.java:sense/build/net/minecraft/world/entity/SenseOps.class" \
  "chunksend/net/minecraft/server/network/ChunkSendOps.java:chunksend/build/net/minecraft/server/network/ChunkSendOps.class" \
  "chunksend/net/minecraft/server/network/ChunkPacketEncodeOps.java:chunksend/build/net/minecraft/server/network/ChunkPacketEncodeOps.class" \
  "poi/net/minecraft/world/entity/ai/village/poi/PoiOps.java:poi/build/net/minecraft/world/entity/ai/village/poi/PoiOps.class" \
  "entityquery/net/minecraft/world/entity/EntityIndexOps.java:entityquery/build/net/minecraft/world/entity/EntityIndexOps.class"
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
check_flat_matches_nested "poi/build" "net/minecraft/world/entity/ai/village/poi/PoiOps"

# TASK-463-88a CP-EXACT gate (lessons ×461/×463): merge 887c4641 union-glued
# "cmp457_paldelta|cmp457_eqsnap2" INSIDE single equals() strings at 11 java
# gate sites — a plain substring-grep for the flag token passes while the gate
# never matches a real flag (7/10 java gate classes slept, NCDFE landmine).
# Gate 1: NO classfile in the repo may carry the exact merge-glue utf8 entry
# (pipe-joined paldelta+eqsnap2 pair; list-style pipe lists are exempt).
# Gate 2: every equals-style gate blob must carry BOTH standalone constants
# (the ItemEntityManager ground-truth shape); BrainOps is list-style
# (TICK2_FLAGS, rust-mirror arming) and is deliberately out of gate 2.
python3 scripts/check_cp_exact.py || die "cp-exact repo scan: merge-glue pipe literal present (see above)"
for blob in \
  entityinside/build/net/minecraft/world/entity/ItemEntityManager.class \
  colpush/build/net/minecraft/world/entity/ColpushOps.class \
  mobpush/build/net/minecraft/world/entity/MobPushOps.class \
  sscan/build/net/minecraft/world/entity/MobPushOps.class \
  entitygoalquery/build/net/minecraft/world/entity/EntityGoalQueryOps.class \
  queryplane/build/net/minecraft/world/entity/QueryPlaneOps.class \
  sscan/build/net/minecraft/world/entity/MobScanOps.class \
  sense/build/net/minecraft/world/entity/SenseOps.class \
  mobai/build/net/minecraft/world/entity/MobAiOps.class \
  goalops/build/net/minecraft/world/entity/ai/goal/GoalOps.class
do
  python3 scripts/check_cp_exact.py --require-standalone "$blob" || \
    die "cp-exact standalone FAIL: $blob"
done

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

# R468-S21 Л180h anti-drift gate: every stagger/build blob must have a
# committed .java SOURCE sibling. Л180h lesson — GoalStaggerOps.class rode
# master since 48475af3 (TASK-401-I, 2026-09-21) with its .java silently
# orphaned on round-403-a-* branches: rebuild from source became impossible
# (javac input missing) while the 2076B blob stayed include_bytes-live in
# src/stagger.rs:48 for 5 days across cmp401_stagger..cmp466_c98ai carriers.
# Contract: source loss = silent drift; this gate turns it into a FAIL.
# S67 (round-470) merge onto post-S56 blobgate v2 + errata: PushStaggerOps
# sibling arg was '.java' under build/ (never exists => gate no-op for Push);
# corrected to the .class blob path so the gate actually guards both.
gate_source_sibling() { # blob_path src_path
  if [ -f "$1" ] && [ ! -f "$2" ]; then
    die "source-sibling FAIL: blob '$1' has no committed source '$2' (Л180h quiet-drift class)"
  fi
  note "source-sibling OK: $2"
}
gate_source_sibling stagger/build/net/minecraft/world/entity/PushStaggerOps.class \
                   stagger/net/minecraft/world/entity/PushStaggerOps.java
gate_source_sibling stagger/build/net/minecraft/world/entity/ai/goal/target/GoalStaggerOps.class \
                   stagger/net/minecraft/world/entity/ai/goal/target/GoalStaggerOps.java
gate_load poi/build            net/minecraft/world/entity/ai/village/poi/PoiOps
gate_load chunksched/build     net/minecraft/server/level/ChunkSchedOps
gate_load fluid/build          net/minecraft/world/entity/FluidPushGuardHook
gate_load fluid/build          'net/minecraft/world/entity/FluidPushGuardHook$GuardEntry'
gate_load fluid/build          net/minecraft/world/entity/FluidBitmaskOps
gate_load entityquery/build    net/minecraft/world/entity/EntityQueryOps
gate_load entityquery/build    net/minecraft/world/entity/EntityIndexOps
gate_load stagger/build        net/minecraft/world/entity/PushStaggerOps
gate_load stagger/build        net/minecraft/world/entity/ai/goal/target/GoalStaggerOps
gate_load paletted/build       net/minecraft/world/level/chunk/PalettedContainerOps

# ROUND-468 S56 dangling-embed guard (x425 adjacent class): an include_bytes!
# path inside a DECLARED module (src/lib.rs|src/main.rs `mod X`) that is
# missing from the tree breaks cargo late or NCDFEs at arm time with the
# blobs gate green. Declared modules only — undeclared lab files (e.g.
# prepare_manager.rs on c1196321, no `mod` site) are intentionally skipped.
python3 - <<'PYEOF' || die "dangling include_bytes! inside a DECLARED module (see stderr)"
import os, re, sys
mods = set()
for f in ("src/lib.rs", "src/main.rs"):
    if os.path.exists(f):
        mods |= set(re.findall(r'^\s*(?:pub\s+)?mod\s+([a-z0-9_]+)\s*;', open(f).read(), re.M))
bad = []
for m in sorted(mods):
    p = "src/%s.rs" % m
    if not os.path.exists(p):
        continue
    for path in re.findall(r'include_bytes!\("([^"]+)"\)', open(p).read()):
        full = os.path.normpath(os.path.join("src", path))
        if not os.path.exists(full):
            bad.append("%s: %s" % (p, path))
for b in bad:
    print("DANGLING:", b, file=sys.stderr)
sys.exit(1 if bad else 0)
PYEOF


# S16 ROUND-471 gap-scan closure (S56-errata-v2): 20 tier-1 LIVE-embed blobs
# (declared module, non-cfg-test include_bytes!, active-lever planes) had ZERO
# gate sites while shipping in the kernel binary — x425 dormant-blob class.
# Existence + major-65 + javap-parse gate here; marker pins land with each
# plane's next rebuild. GAP_REGISTER below machine-tracks the remaining 21.
check_class "entityinside/build/net/minecraft/world/entity/BatchCollector.class"
check_class "entityinside/build/net/minecraft/world/entity/CollideBatchOps.class" "public static boolean blockCollisions(net.minecraft.world.level.Level, net.minecraft.world.entity.Entity, net.minecraft.world.phys.AABB, java.util.List, java.util.List, int, java.util.function.BiPredicate<net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos>)" "private static long mix64(long)" "private static void resetTable(java.lang.Object[])"
check_class "entityinside/build/net/minecraft/world/entity/FlushOps.class"
check_class "entityinside/build/net/minecraft/server/level/NavPlaneOps.class" "public static native int navDecide(int, int, int, int, int[], int[], double[], byte[])" "private static boolean decideJava(int, int, int, int, int, double, double, double, int, int, int)" "public static void handle(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.state.BlockState, int)"
check_class "entityinside/build/net/minecraft/world/level/pathfinder/NavPoolOps.class" "public static native void navPoolTick(int, long, long, long)" "public static void prepare(net.minecraft.world.level.pathfinder.NodeEvaluator, net.minecraft.world.level.PathNavigationRegion, net.minecraft.world.entity.Mob)" "public static net.minecraft.world.level.pathfinder.Node getNode(net.minecraft.world.level.pathfinder.NodeEvaluator, int, int, int)"
check_class "entityinside/build/net/minecraft/server/level/EntityMapOps.class" "public static java.lang.String armState()" "public static boolean containsKey(it.unimi.dsi.fastutil.ints.Int2ObjectMap, int)" "public static boolean refListAdd(ca.spottedleaf.moonrise.common.list.ReferenceList, java.lang.Object)"
check_class "entityinside/build/net/minecraft/server/level/EntityMapSafeItr.class"
check_class "entityinside/build/net/minecraft/server/level/EntityMapSafeValues.class"
check_class "entityinside/build/net/minecraft/server/level/TrackerTickOps.class" "public static void newTrackerTick(net.minecraft.server.level.ChunkMap)" "public static void sweep(ca.spottedleaf.moonrise.patches.chunk_system.level.entity.server.ServerEntityLookup)"
check_class "entityinside/build/net/minecraft/util/RngOps.class" "public static java.util.UUID createInsecureUUID(net.minecraft.util.RandomSource)"
check_class "entityinside/build/net/minecraft/server/level/BlockUpdateOps.class" "public static void vanilla(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.state.BlockState, int)" "private static net.minecraft.world.level.pathfinder.PathTypeCache pathTypes(net.minecraft.server.level.ServerLevel)"
check_class 'entityinside/build/net/minecraft/world/entity/RegionTickOps$Mut.class' 'net.minecraft.world.entity.RegionTickOps$Mut(boolean, net.minecraft.world.entity.Entity)' "final net.minecraft.world.entity.Entity entity;"
check_class 'entityquery/build/net/minecraft/world/entity/EntityIndexOps$Buf.class' 'net.minecraft.world.entity.EntityIndexOps$Buf()' "final java.util.concurrent.atomic.AtomicBoolean busy;" "final double[] bb;"
check_class "randomtick/build/net/minecraft/server/level/TickBlockOps.class"
check_class "randomtick/build/net/minecraft/server/level/RandomTickOps.class"
check_class "paletted/build/PalettedContainer.patched.class" "public volatile transient java.lang.Object[] crusstySnap;" "public volatile int crusstySnapGen;" "public volatile int crusstyGen;"
noise_check_class 'noise/build/net/minecraft/world/level/levelgen/synth/PerlinNoiseNativeOps$Handle.class'
noise_check_class 'noise/build/net/minecraft/world/level/levelgen/synth/PerlinNoiseNativeOps$Reaper.class'
check_class 'randomtick/build/net/minecraft/world/entity/ai/BrainOps$IdKey.class'
check_class 'randomtick/build/net/minecraft/world/entity/ai/BrainOps$Snapshot.class'

# S16 live-embed closure scanner: every non-test include_bytes! path of a
# DECLARED module must be gated somewhere in THIS script or registered below
# as a known gap (else die). Untracked dangling embeds (prepare_manager.rs
# PrepareOps.class) stay excluded per S56 doctrine (undeclared module).
GAP_REGISTER=$(cat <<'S16GAPS'
entityinside/build/net/minecraft/core/ZeroCursorIter.class
entityinside/build/net/minecraft/core/ZeroCursorOps.class
entityinside/build/net/minecraft/world/entity/FluidOps.class
entityinside/build/net/minecraft/world/entity/FluidPushOps.class
entityinside/build/net/minecraft/world/entity/FluidPushOps$ScanOut.class
entityinside/build/net/minecraft/world/entity/InsideBatchOps.class
entityinside/build/net/minecraft/world/entity/InsideBlockOps$Recorder.class
entityinside/build/net/minecraft/world/entity/InsideDietOps.class
entityinside/build/net/minecraft/world/entity/InsideDietVisitor.class
entityinside/build/net/minecraft/world/entity/InsideSnapOps$Lane.class
entityinside/build/net/minecraft/world/entity/InsideSnapRegistryOps.class
entityinside/build/net/minecraft/world/entity/TravelDietOps.class
entityinside/build/net/minecraft/world/level/SkipStoreOps.class
entityinside/build/net/minecraft/world/level/TraverseOps.class
entityinside/build/net/minecraft/world/level/ZeroAllocOps.class
fluid/build/net/minecraft/world/entity/FluidBitmaskOps$Entry.class
items/build/net/minecraft/world/entity/item/ItemMergeOps.class
area-map/build-probe/dev/crussty/areamapprobe/AreaMapProbe.class
area-map/build-probe/dev/crussty/areamapprobe/AreaMapProbe$RecMap.class
tests/fixtures/SerializableChunkData.class
tests/fixtures/SingleUserAreaMap.class
S16GAPS
)
python3 - "$GAP_REGISTER" "$S16_SELF" <<'PYEOF2' || die "live-embed gap not gated and not in GAP_REGISTER (see stderr)"
import os, re, sys
script = open(sys.argv[2]).read()
register = set(l for l in sys.argv[1].splitlines() if l.strip())
lib = open("src/lib.rs").read()
mods = set(re.findall(r'^\s*(?:pub\s+)?mod\s+([a-z0-9_]+)\s*;', lib, re.M))
live = set()
for root, d, files in os.walk("src"):
    for f in files:
        if not f.endswith(".rs"): continue
        if f[:-3] not in mods: continue
        p = os.path.join(root, f); txt = open(p).read()
        for m in re.finditer(r'include_bytes!\("(\.[^"]+)"\)', txt):
            pre = txt[:m.start()]
            t = txt[m.end():]
            seg_t = txt[:m.start()]
            inside_test = False
            for tm in re.finditer(r'#\[cfg\(test\)\]', txt[:m.start()]):
                seg = txt[tm.end():m.start()]
                if seg.count('{') > seg.count('}'): inside_test = True
            if inside_test: continue
            live.add(os.path.normpath(os.path.join(root, m.group(1))))
bad = []
for path in sorted(live):
    quoted = '"%s"' % path
    if quoted in script or ("'%s'" % path) in script: continue
    if path in register: continue
    bad.append(path)
for b in bad: print("UNGATED LIVE-EMBED:", b, file=sys.stderr)
sys.exit(1 if bad else 0)
PYEOF2

if [ "$FAIL" = "0" ]; then
  echo "check_blobs_sync: ALL IN SYNC"
  exit 0
else
  echo "check_blobs_sync: BLOB-SYNC VIOLATION (see FAIL lines)" >&2
  exit 1
fi
