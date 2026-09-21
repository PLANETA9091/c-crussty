#!/usr/bin/env bash
# build_mobhash_ops.sh — compile the MOBHASH bridge class (TASK-401-A,
# vector spatial-hash neighbors, lever cmp401_mobhash).
#
# Pattern: scripts/build_entity_query_ops.sh — compiled offline against the
# runtime kernel jar (Mojang-mapped, moonrise patches public) and defined into
# the KERNEL loader at activation time (same package net.minecraft.world.entity
# as Entity). --release 21 pins the class-file major to 65 = the kernel JVM;
# src/mobhash_manager.rs refuses to arm if the embedded major exceeds the live
# JVM's.
#
# Usage: scripts/build_mobhash_ops.sh [javac] [kernel.jar] [fastutil.jar]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-}"
if [ -z "$JAVAC" ]; then
  if [ -x /home/z/tools/jdk-21.0.12.1+1/bin/javac ]; then
    JAVAC=/home/z/tools/jdk-21.0.12.1+1/bin/javac
  elif command -v javac > /dev/null 2>&1; then
    JAVAC=javac
  else
    echo "no javac found (pass one as arg 1)" >&2
    exit 1
  fi
fi

KERNEL="${2:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-j2b/patched-kernel.jar}"
FASTUTIL="${3:-/home/z/tools/fastutil.jar}"

OUT_DIR=mobhash/build
mkdir -p "$OUT_DIR/net/minecraft/world/entity"

"$JAVAC" --release 21 -nowarn -proc:none \
  -classpath "$KERNEL:$FASTUTIL" \
  -d "$OUT_DIR" \
  mobhash/net/minecraft/world/entity/MobHashOps.java

echo "built:"
ls -la "$OUT_DIR/net/minecraft/world/entity/MobHashOps.class"
sha256sum "$OUT_DIR/net/minecraft/world/entity/MobHashOps.class"
