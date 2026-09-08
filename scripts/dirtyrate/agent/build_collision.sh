#!/usr/bin/env bash
# TASK-104 collision dirty-census agent build (S7-46 main, 2026-09-09)
# Spec: docs/COLLISION_CENSUS_DESIGN.md §5 steps 1-3. Pattern per TASK-90 build.sh.
set -euo pipefail
cd "$(dirname "$0")"
JDK="${JDK21:-/home/z/jdk21}"

rm -rf build/classes build/jar && mkdir -p build/classes build/jar
"$JDK/bin/javac" --release 17 -cp lib/asm-9.7.jar -d build/classes \
    src/collision/CollisionCensusAgent.java

# Self-test FIRST (compiles fake probe-target classes, weaves bytes in-process,
# loads through the verifier, asserts exact counter deltas): failure here = no jar.
"$JDK/bin/javac" --release 17 -cp "lib/asm-9.7.jar:build/classes" \
    -d build/selftest src/collision/fake/*.java src/collision/CollisionSelfTest.java
"$JDK/bin/java" -cp "lib/asm-9.7.jar:build/classes:build/selftest" collision.CollisionSelfTest

# Integration: collision TSV emission contract x THEIR analyzer (unmodified).
python3 - <<'PYEOF'
import subprocess, sys
rows = []
for ep, q, m, mv, bi, sa in [(100, 5000, 40, 6000, 60000, 15000), (130, 8000, 50, 9000, 90000, 22000)]:
    rows += [f"{ep}\tcollision\tmutation\t{m}",
             f"{ep}\tcollision\tquery\t{q}",
             f"{ep}\tcollision-blockiter\tquery\t{bi}",
             f"{ep}\tcollision-shapes-axis\tquery\t{sa}",
             f"{ep}\tcollision-move\tquery\t{mv}",
             f"{ep}\tMETA\tphase\ttick"]
open("/tmp/collision_fixture.tsv", "w").write("\n".join(rows) + "\n")
out = subprocess.run([sys.executable, "../analyze_dirtyrate.py", "/tmp/collision_fixture.tsv"],
                     capture_output=True, text=True).stdout
# window delta (row2-row1, analyzer semantics): query 3000, mutation 10 -> dirty 0.333%
assert "| 0.333% |" in out, "analyzer contract mismatch:\n" + out
assert "collision-move" in out and "collision-blockiter" in out \
    and "collision-shapes-axis" in out, "extension rows lost:\n" + out
assert "collision: best dirty%" in out, "CPU-share ceiling line lost:\n" + out
print("[collision-selftest] ANALYZER-CONTRACT-OK (collision fixture -> their analyzer, unmodified)")
PYEOF

# Shade ASM into the agent jar (single artifact for -javaagent).
cd build/classes
"$JDK/bin/jar" xf ../../lib/asm-9.7.jar
printf 'Premain-Class: collision.CollisionCensusAgent\nCan-Redefine-Classes: false\nCan-Retransform-Classes: false\n' > manifest.mf
"$JDK/bin/jar" cfm ../jar/collision_census.jar manifest.mf -C . .
cd ..
echo "[collision-build] OK -> jar/collision_census.jar ($(du -h jar/collision_census.jar | cut -f1))"
