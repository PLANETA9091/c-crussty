#!/usr/bin/env bash
# build_459_roarfix_blobs.sh — TASK-459-86 (ROAR-BLOB-FIX, тик-459, урок-408:
# устаревший блоб = спящий гейт). Rebuild the cmp458_roar eindex bridge blobs
# (EntityIndexOps + nested Buf) from the roar source of round-458k-roaring —
# ONE javac pass (lesson ×93: inter-source deps resolved by javac itself).
#
# cp = kernel round-396-a + fastutil + paper-api 1.21.10 + adventure 4.24.0
# (mandate /home/z/tools, NEVER round-j2b-jar), --release 21 (major 65) —
# pattern: scripts/build_459_p32_blobs.sh (round-459-p32) + the canonical
# build_entity_index_ops.sh (TASK-405-C) kernel contract.
#
# Gates after install: (1) major 65, (2) leverEnabled carries BOTH lever
# strings (cmp405_eindex | cmp458_roar) in bytecode, (3) native signatures
# match the manager RegisterNatives contract (eidxProbe "()I", eidxFlush
# "(I[I[B[D[I)I", eidxFlushQuery "(I[I[B[D[IDDDDDDIIIIII[I)I").
#
# Usage: scripts/build_459_roarfix_blobs.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"
JAVAP="${JAVAP:-${JAVAC%javac}javap}"

KERNEL="${KERNEL_JAR:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || KERNEL="/home/z/tools/patched-kernel.jar"
[ -f "$KERNEL" ] || { echo "kernel jar not found" >&2; exit 1; }

CP="$KERNEL:entityquery/build"
for j in /home/z/tools/fastutil.jar /home/z/tools/paper-api-1.21.10.jar \
         /home/z/tools/adventure-api-4.24.0.jar /home/z/tools/adventure-key-4.24.0.jar; do
  [ -f "$j" ] && CP="$CP:$j"
done

ALL_BUILD=$(mktemp -d)
trap 'rm -rf "$ALL_BUILD"' EXIT

"$JAVAC" --release 21 -nowarn -cp "$CP" -d "$ALL_BUILD" \
  entityquery/net/minecraft/world/entity/EntityIndexOps.java

NESTED="entityquery/build/net/minecraft/world/entity"

install_blob() { # fqcn(slash-form) — nested include_bytes! path (manager contract)
  local cls="$1"; local built="$ALL_BUILD/$cls.class"; local dst="$NESTED/$(basename "$cls").class"
  # gate 0: class-file major must be 65 (Java 21) — manager refuses newer.
  local major
  major=$(od -An -tu1 -j6 -N2 "$built" | awk '{print $1*256+$2}')
  [ "$major" = "65" ] || { echo "FATAL: $cls major $major != 65" >&2; exit 1; }
  cp "$built" "$dst"
  echo "blob: $dst ($(stat -c%s "$dst") bytes, major $major)"
}

install_blob net/minecraft/world/entity/EntityIndexOps
install_blob 'net/minecraft/world/entity/EntityIndexOps$Buf'

# gate 1: lever strings baked into leverEnabled bytecode (STRICT eq both flags).
for flag in cmp405_eindex cmp458_roar; do
  if ! "$JAVAP" -c -p -cp entityquery/build net.minecraft.world.entity.EntityIndexOps \
      | sed -n '/private static boolean leverEnabled/,/^  [a-z]/p' | grep -q "String $flag"; then
    echo "FATAL: leverEnabled() missing $flag" >&2; exit 1
  fi
done
echo "gate1 OK: leverEnabled() carries cmp405_eindex | cmp458_roar"

# gate 2: native signatures match src/entity_index_manager.rs RegisterNatives.
SIGS=$("$JAVAP" -p -cp entityquery/build net.minecraft.world.entity.EntityIndexOps | grep "native int eidx")
echo "$SIGS"
echo "$SIGS" | grep -q "eidxProbe()" || { echo "FATAL: eidxProbe sig" >&2; exit 1; }
echo "$SIGS" | grep -q "eidxFlush(int, int\[\], byte\[\], double\[\], int\[\])" \
  || { echo "FATAL: eidxFlush sig" >&2; exit 1; }
echo "$SIGS" | grep -q "eidxFlushQuery(int, int\[\], byte\[\], double\[\], int\[\], double, double, double, double, double, double, int, int, int, int, int, int, int\[\])" \
  || { echo "FATAL: eidxFlushQuery sig" >&2; exit 1; }
echo "gate2 OK: native signatures match RegisterNatives contract"
echo "ROARFIX BLOBS REBUILT"
