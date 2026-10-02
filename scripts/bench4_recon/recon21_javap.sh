#!/usr/bin/env bash
# recon21_javap.sh — javap-контракт #14 v2 TravelDietOps (дампы реального kernel jar)
set -euo pipefail
JAR=/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7178-recal/patched-kernel.jar
OUT=/home/z/c-crussty/research/gc-recon-2026-09-19/contract-traveldiet-s7182b
mkdir -p "$OUT"
JP=/tmp/jdk21/bin/javap
cd "$OUT"
for C in net.minecraft.world.entity.Entity net.minecraft.world.entity.LivingEntity \
         net.minecraft.world.phys.Vec3 net.minecraft.world.phys.AABB \
         net.minecraft.world.entity.EntityDimensions net.minecraft.world.phys.shapes.CollisionUtil; do
  F=$(echo "$C" | tr '.' '_').txt
  [ -s "$F" ] || "$JP" -c -p -cp "$JAR" "$C" > "$F"
done
wc -c *.txt
echo "== EntityDimensions.makeBoundingBox signatures =="
rg -n "makeBoundingBox" EntityDimensions.txt | head
echo "== call-sites of EntityDimensions.makeBoundingBox in Entity/LivingEntity =="
rg -n "EntityDimensions.makeBoundingBox" Entity.txt LivingEntity.txt | head
echo "== AABB.inflate signature =="
rg -n "public net.minecraft.world.phys.AABB inflate" AABB.txt | head -3
echo "== AABB.inflate call-sites in Entity =="
rg -n "inflate" Entity.txt | head -8
echo "== Vec3.add/multiply call-sites in LivingEntity (travel chain) =="
rg -n "Vec3.(add|multiply|scale|subtract)" LivingEntity.txt | head -20
echo "== travel/move/collide method offsets =="
rg -n "public void travel|public boolean move|private net.minecraft.world.phys.Vec3 collide|getAllowedMovement|handleRelativeFrictionAndCalculateMovement|getInputVector" LivingEntity.txt Entity.txt | head -12
