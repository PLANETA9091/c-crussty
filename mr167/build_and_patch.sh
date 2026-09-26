#!/usr/bin/env bash
# build_and_patch.sh — S75 one-shot: pre-gate -> extract ECBS from kernel ->
# Mr167Patch (4 sites) -> post-gate -> load-check. Idempotent, drift-refusing.
# Usage: mr167/build_and_patch.sh [kernel.jar]   (default /home/z/tools/patched-kernel.jar)
set -euo pipefail
cd "$(dirname "$0")/.."

KERNEL="${1:-/home/z/tools/patched-kernel.jar}"
JDK=/home/z/tools/jdk-21.0.12.1+1/bin
NESTED='ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices$EntityCollectionBySection'
WORK="$(mktemp -d /home/z/tmp.mr167.XXXXXX)"
trap 'rm -rf "$WORK"' EXIT

echo "== [1/5] pre-gate: debt present? =="
bash scripts/mr167_javap_gate.sh pre "$KERNEL"

echo "== [2/5] extract $NESTED =="
mkdir -p "$WORK/in/ca/spottedleaf/moonrise/patches/chunk_system/level/entity"
unzip -o -q "$KERNEL" "$NESTED.class" -d "$WORK/in"
IN="$WORK/in/$NESTED.class"
[ -f "$IN" ] || { echo "FAIL: extraction" >&2; exit 1; }

echo "== [3/5] Mr167Patch =="
"$JDK/javac" -cp mr167/asm-9.7.1.jar mr167/Mr167Patch.java
mkdir -p "$WORK/out"
java -cp mr167/asm-9.7.1.jar:. Mr167Patch "$IN" "$WORK/out/patched.class"

echo "== [4/5] install nested layout + post-gate =="
mkdir -p "mr167/patched/ca/spottedleaf/moonrise/patches/chunk_system/level/entity"
cp "$WORK/out/patched.class" "mr167/patched/$NESTED.class"
cp "$WORK/out/patched.class" "mr167/ChunkEntitySlices\$EntityCollectionBySection.mr167.patched.class"
bash scripts/mr167_javap_gate.sh post mr167/patched "$KERNEL"

echo "== [5/5] load-check vs kernel jar =="
cat > "$WORK/LoadCheck.java" <<'EOF'
import java.net.*; import java.nio.file.*;
public class LoadCheck {
  public static void main(String[] a) throws Exception {
    URL[] urls = { Paths.get(a[0]).toUri().toURL(), Paths.get(a[1]).toUri().toURL() };
    URLClassLoader cl = new URLClassLoader(urls, ClassLoader.getPlatformClassLoader());
    Class<?> c = Class.forName("ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection", false, cl);
    System.out.println("LOAD OK: " + c.getName());
  }
}
EOF
"$JDK/javac" -d "$WORK" "$WORK/LoadCheck.java"
java -cp "$WORK" LoadCheck "$(pwd)/mr167/patched" "$KERNEL"

echo "mr167 build_and_patch: ALL GREEN (kernel=$KERNEL)"
