#!/usr/bin/env bash
# TASK-90 dirty-census agent build (agent-7625532f, 2026-09-09)
# Spec: docs/DIRTY_RATE_CENSUS_TOOLING.md §1 — pure-java counter agent, ASM entry-probes.
set -euo pipefail
cd "$(dirname "$0")"
JDK="${JDK21:-/home/z/jdk21}"

rm -rf build/classes build/jar && mkdir -p build/classes build/jar
"$JDK/bin/javac" --release 17 -cp lib/asm-9.7.jar -d build/classes src/agent/DirtyCensusAgent.java

# Self-test FIRST (compiles fake probe-target classes, weaves bytes in-process,
# loads through the verifier, asserts counters): failure here = no jar.
"$JDK/bin/javac" --release 17 -cp "lib/asm-9.7.jar:build/classes" -d build/selftest \
    src/agent/SelfTest.java src/agent/fake/BlockEntity.java
"$JDK/bin/java" -cp "lib/asm-9.7.jar:build/classes:build/selftest" agent.SelfTest

# Integration: agent TSV emission contract x THEIR analyzer (unmodified).
python3 - <<'PYEOF'
import subprocess, sys
rows = []
for ep, q, m, pt in [(100, 1000, 5, 1000), (130, 2000, 8, 1150)]:
    rows += [f"{ep}\thopper-inventory\tmutation\t{m}",
             f"{ep}\thopper-inventory\tquery\t{q}",
             f"{ep}\thopper-push-tick\tquery\t{pt}",
             f"{ep}\tMETA\tphase\ttick"]
open("/tmp/dirty_fixture.tsv", "w").write("\n".join(rows) + "\n")
out = subprocess.run([sys.executable, "../analyze_dirtyrate.py", "/tmp/dirty_fixture.tsv"],
                     capture_output=True, text=True).stdout
# window delta: query 1000, mutation 3 -> dirty 0.300% -> candidate band
assert "| 0.300% |" in out and "GUARD-CANDIDATE->100x" in out, "analyzer contract mismatch:\n" + out
assert "hopper-push-tick" in out, "push-tick extension row lost"
print("[selftest] ANALYZER-CONTRACT-OK (fixture -> their analyzer, unmodified)")
PYEOF

# Shade ASM into the agent jar (single artifact for -javaagent).
cd build/classes
"$JDK/bin/jar" xf ../../lib/asm-9.7.jar
printf 'Premain-Class: agent.DirtyCensusAgent\nCan-Redefine-Classes: false\nCan-Retransform-Classes: false\n' > manifest.mf
"$JDK/bin/jar" cfm ../jar/dirty_census.jar manifest.mf -C . .
cd ..
ls -la jar/dirty_census.jar
echo "BUILD-OK"
