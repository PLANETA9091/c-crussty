#!/usr/bin/env bash
# selftest_queryplane_gate.sh — TASK-416-A recipe 3 ПРЕГИСТ-ГЕЙТ ДО диспатча:
# QueryPlaneOps.selfTest()==true locally (fresh blob, real kernel jar).
# Root-cause mc1-3: stale blob constant pool (only cmp412_b2p1) => ENABLED=false
# under cmp415_mcomp => selfTest false => queryplane dormant. This gate catches
# that class of failure BEFORE a leg is burned.
set -uo pipefail
cd "$(dirname "$0")/.."
JAVAC=/home/z/tools/jdk-21.0.12.1+1/bin/javac
JAVA=/home/z/tools/jdk-21.0.12.1+1/bin/java
KERNEL="${KERNEL_JAR:-/home/z/c-crussty/research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar}"
[ -f "$KERNEL" ] || { echo "FAIL: kernel jar missing" >&2; exit 1; }
OUT=$(mktemp -d); trap 'rm -rf "$OUT"' EXIT
$JAVAC --release 21 -cp "$KERNEL:queryplane/build" -d "$OUT" \
  scripts/bench4_recon/QueryPlaneSelfTestMain.java || { echo "FAIL: compile" >&2; exit 1; }
RC=0
run_case() { # lever expected
  CRUSSTY_LEVER_FLAG="$1" EXPECTED="$2" "$JAVA" -cp "$KERNEL:queryplane/build:$OUT" \
    QueryPlaneSelfTestMain 2>/dev/null | tee /dev/stderr | grep -q GATE-OK || RC=1
}
run_case cmp416_mcomp true   # era lever — MUST be true (fresh blob gate)
run_case cmp415_mcomp true   # legacy era composite stays armed
run_case cmp412_b2p1 true    # original queryplane lever stays armed
run_case cmp403_tickplane false # foreign flag => false (strict gate)
run_case "" false            # empty flag => false (vanilla)
if [ "$RC" = "0" ]; then echo "selftest_queryplane_gate: PASS (selfTest==true for cmp416_mcomp)"; else echo "selftest_queryplane_gate: FAIL" >&2; fi
exit $RC
