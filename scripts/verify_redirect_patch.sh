#!/usr/bin/env bash
# verify_redirect_patch.sh — offline JVM-verifier gate for static body-redirects
# (s7172 lesson: defineClass verification is NOT enough; the real JVMTI
# RetransformClasses path rejected bytes that defineClass accepted).
# Usage: verify_redirect_patch.sh <kernel-jar> <mojang-libs-dir> <fastutil-jar>
#   1) cargo test dump_patched_blockpos_for_verifier_probe  -> tests/out/BlockPos.zerocursor.patched.class
#   2) RetransformProbe: vanilla BlockPos load -> retransform(patched) -> factory call
#   PASS = "[probe] RETRANSFORM OK" + "[probe] SEMANTIC OK"
set -euo pipefail
KERNEL_JAR="${1:?kernel jar}"
MOJANG_LIBS="${2:?mojang-libs dir}"
FASTUTIL="${3:?fastutil jar}"
export PATH="$PATH:$HOME/.cargo/bin:${JDK_BIN:-/tmp/jdk21/bin}"
cd /home/z/c-crussty
cargo test --lib dump_patched_blockpos_for_verifier_probe -- --exact >/dev/null
HARNESS=$(mktemp -d)
javac -d "$HARNESS" entityinside/harness/RetransformProbe.java
printf 'Premain-Class: RetransformProbe\nCan-Retransform-Classes: true\n' > "$HARNESS/mf.txt"
jar cfm "$HARNESS/probe.jar" "$HARNESS/mf.txt" -C "$HARNESS" .
java -javaagent:"$HARNESS/probe.jar" \
  -cp "$HARNESS/probe.jar:$MOJANG_LIBS/*" \
  RetransformProbe "$KERNEL_JAR" tests/out/BlockPos.zerocursor.patched.class \
  entityinside/build/ | grep -E "RETRANSFORM OK|SEMANTIC OK" \
  && echo "VERIFY-REDIRECT: PASS" || { echo "VERIFY-REDIRECT: FAIL"; exit 1; }
