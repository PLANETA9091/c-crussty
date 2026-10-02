#!/usr/bin/env bash
# Noise-bridge A/B bench: legacy bridge (git HEAD = Finalizer + synchronizedMap
# on every call) vs working-tree bridge (Cleaner + per-thread direct-mapped
# identity cache). The legacy variant is GENERATED from git HEAD at run time,
# so the baseline always tracks the committed state — a true A/B of exactly
# the change under test.
#
# Usage: ./run_noise_ab.sh [profile ...]   (default: all profiles)
# Requires: javac + java (JDK 21), native/libpaper_native_jni.so (in repo).
set -euo pipefail
cd "$(dirname "$0")"

REPO_ROOT="$(cd ../.. && pwd)"
MAIN_SO="$REPO_ROOT/native/libpaper_native_jni.so"
[ -f "$MAIN_SO" ] || { echo "missing $MAIN_SO" >&2; exit 1; }

if [ -x /home/z/jdk21/bin/javac ]; then JAVAC=/home/z/jdk21/bin/javac; JAVA=/home/z/jdk21/bin/java
elif [ -x "$JAVA_HOME/bin/javac" ]; then JAVAC="$JAVA_HOME/bin/javac"; JAVA="$JAVA_HOME/bin/java"
else JAVAC=javac; JAVA=java; fi
echo "using: $JAVAC"

PKG_DIR=gen/net/minecraft/world/level/levelgen/synth
mkdir -p "$PKG_DIR" classes

# 1) regenerate the legacy baseline from git HEAD (must exist: this bench is
#    exactly the A/B of the hot-path change, so the committed bridge IS the
#    legacy side). Rename the class so both variants coexist in one JVM.
git -C "$REPO_ROOT" show HEAD:noise/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps.java \
  | sed 's/ImprovedNoiseNativeOps/ImprovedNoiseNativeOpsLegacy/g' \
  > "$PKG_DIR/ImprovedNoiseNativeOpsLegacy.java"
echo "legacy baseline regenerated from git HEAD ($(git -C "$REPO_ROOT" rev-parse --short HEAD))"

# 2) compile: runtime shape stubs + legacy bridge + current bridge + driver
$JAVAC -nowarn -d classes \
  src/net/minecraft/world/level/levelgen/synth/ImprovedNoise.java \
  src/net/minecraft/world/level/levelgen/synth/PaperNativeImprovedNoise.java \
  "$PKG_DIR/ImprovedNoiseNativeOpsLegacy.java" \
  "$REPO_ROOT"/noise/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps.java \
  src/net/minecraft/world/level/levelgen/synth/NoiseAbBench.java

# 3) run (forward order: legacy first, current second; the driver itself
#    also runs both orders internally and takes min-of-medians)
"$JAVA" -Xms256m -Xmx1g -XX:+UseG1GC -cp classes net.minecraft.world.level.levelgen.synth.NoiseAbBench "$MAIN_SO" "$@"
