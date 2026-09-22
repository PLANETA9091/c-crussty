#!/usr/bin/env bash
# Build the NavPoolOps bridge class (TASK-410-A k5 node-pool, cmp405_navplane).
#
# PURE JAVA bridge + ONE native (navPoolTick telemetry), delivery pattern of
# NavPlaneOps (TASK-405-A): compiled offline against the runtime kernel jar
# (Mojang-mapped) and defined into the KERNEL loader at region_threads
# activation time (same package net.minecraft.world.level.pathfinder as
# NodeEvaluator — protected-field access is package access).
#
# Class-file major pinned to 65 = the kernel JVM (Java 21).
# ZERO nested classes allowed (S7-163 leg#1 lesson) — the cargo guard
# src/region_threads.rs enforces the same set.
#
# Usage: scripts/build_navpool_ops.sh [javac]
set -euo pipefail
cd "$(dirname "$0")/.."

JAVAC="${1:-/home/z/tools/jdk-21.0.12.1+1/bin/javac}"

SERVER_JAR="${SERVER_JAR:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
if [ ! -f "$SERVER_JAR" ]; then
  SERVER_JAR="$(ls research/*/run-*/patched-kernel.jar 2>/dev/null | tail -1 || true)"
fi
if [ ! -f "$SERVER_JAR" ]; then echo "runtime/kernel jar not found" >&2; exit 1; fi

FASTUTIL_JAR="${FASTUTIL_JAR:-/home/z/tools/fastutil.jar}"

OUT_DIR=entityinside/build
mkdir -p "$OUT_DIR/net/minecraft/world/level/pathfinder"

# Remove any stale output first (delivery-set discipline).
rm -f "$OUT_DIR/net/minecraft/world/level/pathfinder/NavPoolOps"*.class

if ! $JAVAC --release 21 \
  -cp "$SERVER_JAR:$FASTUTIL_JAR" \
  -d "$OUT_DIR" \
  entityinside/net/minecraft/world/level/pathfinder/NavPoolOps.java 2>/dev/null; then
  $JAVAC \
    -cp "$SERVER_JAR:$FASTUTIL_JAR" \
    -d "$OUT_DIR" \
    entityinside/net/minecraft/world/level/pathfinder/NavPoolOps.java
fi

# Delivery-set guard: EXACTLY one classfile must be produced (no nested).
PRODUCED=$(ls "$OUT_DIR/net/minecraft/world/level/pathfinder/NavPoolOps"*.class 2>/dev/null | wc -l)
if [ "$PRODUCED" != "1" ]; then
  echo "NAVPOOL delivery guard FAILED: produced $PRODUCED classfiles (expected 1)" >&2
  ls -la "$OUT_DIR/net/minecraft/world/level/pathfinder/" >&2
  exit 2
fi
echo "navpool_ops build OK: $(ls "$OUT_DIR/net/minecraft/world/level/pathfinder/NavPoolOps"*.class)"
