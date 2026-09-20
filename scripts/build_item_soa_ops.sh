#!/usr/bin/env bash
# build_item_soa_ops.sh — compile the ITEMS_SOA bridge class (MEGA-ROUND-2,
# TASK-397-C: SoA flat primitive-array mirror behind ItemEntity.mergeWithNeighbours).
#
# PURE JAVA bridge, same delivery pattern as FluidOps/InsideBitmaskOps:
# compiled offline against the real patched kernel jar (Mojang-mapped,
# purpur-1.21.10) + paper-api (EntityRemoveEvent$Cause) and defined into the
# KERNEL loader at activation time (item_soa.rs). --release 21 pins the
# class-file major to 65 = the kernel JVM.
#
# Usage: scripts/build_item_soa_ops.sh [javac] [kernel.jar] [paper-api.jar]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if [ -x /home/z/tools/jdk-21.0.12.1+1/bin/javac ]; then JAVAC=/home/z/tools/jdk-21.0.12.1+1/bin/javac
  elif command -v javac > /dev/null 2>&1; then JAVAC=javac
  else echo "no javac found (pass one as arg 1 or install a JDK)" >&2; exit 1; fi
fi
KERNEL="${2:-kernel/patched-kernel.jar}"
PAPI="${3:-/home/z/tools/paper-api.jar}"

OUT=entityinside/build
mkdir -p "$OUT/net/minecraft/world/entity/item"

"$JAVAC" --release 21 -proc:none -cp "$KERNEL:$PAPI" -d "$OUT" \
  entityinside/net/minecraft/world/entity/item/ItemSoaOps.java

echo "built: $OUT/net/minecraft/world/entity/item/ItemSoaOps.class"
sha256sum "$OUT/net/minecraft/world/entity/item/ItemSoaOps.class"
