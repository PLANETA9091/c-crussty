#!/usr/bin/env bash
# S7-158 recon: javap dumps for tracker-race + UUID root-cause (INJECTS-ONLY offline).
set -uo pipefail
JAVA=/tmp/jdk21/bin/javap
cd /tmp/s7158recon
JAR=/tmp/s7147mat/server/versions/1.21.10/purpur-1.21.10.jar

echo "===== 1. ChunkMap.newTrackerTick (full bytecode + line table) ====="
$JAVA -p -c -l -cp "$JAR" net.minecraft.server.level.ChunkMap > chunkmap_full.txt 2>&1
# extract the newTrackerTick method block
awk '/void newTrackerTick|newTrackerTick\(/{flag=1} flag{print} flag && /^$/{exit}' chunkmap_full.txt > newTrackerTick.txt
wc -l newTrackerTick.txt

echo "===== 2. ServerLevel\$EntityCallbacks surface ====="
$JAVA -p -c -l -cp "$JAR" 'net.minecraft.server.level.ServerLevel$EntityCallbacks' > callbacks_full.txt 2>&1
grep -nE "public|onTicking|onEntity|onSection" callbacks_full.txt | grep -vE "^\s*[0-9]+:" | head -30

echo "===== 3. EntityLookup.remove callers census (grep in ChunkMap/ServerLevel disasm) ====="
grep -c "EntityLookup.remove" chunkmap_full.txt || true
$JAVA -p -c -l -cp "$JAR" net.minecraft.server.level.ServerLevel > serverlevel_full.txt 2>&1
grep -c "EntityLookup" serverlevel_full.txt || true
$JAVA -p -c -l -cp "$JAR" net.minecraft.world.level.entity.EntityLookup > entitylookup_full.txt 2>&1
grep -nE "public void remove|public void add" entitylookup_full.txt | head

echo "===== 4. UUID: Entity ctor ====="
$JAVA -p -c -l -cp "$JAR" net.minecraft.world.entity.Entity > entity_full.txt 2>&1
grep -n "createInsecureUUID\|UUIDUtil\|randomUUID" entity_full.txt | head -20

echo "===== 5. RandomSource.create / seedUniquifier ====="
$JAVA -p -c -l -cp "$JAR" net.minecraft.util.RandomSource > randomsource_full.txt 2>&1
cat randomsource_full.txt | head -60

echo "===== 6. UUIDUtil ====="
unzip -o -q "$JAR" 'net/minecraft/core/UUIDUtil.class' 2>/dev/null
$JAVA -p -c -l -cp "$JAR" net.minecraft.core.UUIDUtil > uuidutil_full.txt 2>&1
grep -nE "createInsecureUUID|createInsecureUUID|public static" uuidutil_full.txt | head -20
echo DONE
