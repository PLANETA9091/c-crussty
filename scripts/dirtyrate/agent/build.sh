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

# Shade ASM into the agent jar (single artifact for -javaagent).
cd build/classes
"$JDK/bin/jar" xf ../../lib/asm-9.7.jar
printf 'Premain-Class: agent.DirtyCensusAgent\nCan-Redefine-Classes: false\nCan-Retransform-Classes: false\n' > manifest.mf
"$JDK/bin/jar" cfm ../jar/dirty_census.jar manifest.mf -C . .
cd ..
ls -la jar/dirty_census.jar
echo "BUILD-OK"
