#!/usr/bin/env bash
# check_javap_loadability_449c.sh — TASK-449-C δ-gate (урок ×448 DELIVERY-FAIL).
# javap-гейт ЗАГРУЖАЕМОСТИ: EVERY .class under */build/ (ops bridges AND their
# inner companions, e.g. ColpushOps$ConstSlot) must be parseable by
# javap -p -cp <build-dir-root> <fqcn>. NCDFE at runtime = DELIVERY-FAIL;
# this gate catches stale/missing inner-class blobs offline. Fixture capture
# artifacts (*.patched.class etc.) are not classfile names and are skipped.
# Usage: scripts/check_javap_loadability_449c.sh   (exit 0 = PASS)
set -uo pipefail
cd "$(dirname "$0")/.."

JAVAP="${JDK21:-/home/z/tools/jdk-21.0.12.1+1}/bin/javap"
[ -x "$JAVAP" ] || JAVAP=$(command -v javap)
[ -x "$JAVAP" ] || { echo "FAIL: no javap" >&2; exit 1; }
export JAVAP

python3 - <<'PY'
import os, subprocess, sys
JAVAP = os.environ["JAVAP"]
fails, count = [], 0
for root, dirs, files in os.walk("."):
    if ".git" in root:
        continue
    if os.path.basename(root) != "build":
        continue
    for r2, d2, f2 in os.walk(root):
        for fn in f2:
            if not fn.endswith(".class"):
                continue
            base = fn[:-6]
            if base.endswith(".patched") or base.endswith("_patched_ssb") or ".patched." in base:
                continue  # fixture capture artifacts, not classfile names
            p = os.path.join(r2, fn)
            fq = p[len(root) + 1:-6].replace("/", ".")
            count += 1
            r = subprocess.run([JAVAP, "-p", "-cp", root, fq], capture_output=True, text=True)
            if r.returncode != 0 or "Error" in r.stderr or not r.stdout.strip():
                fails.append((p, fq, (r.stderr or r.stdout).strip()[:120]))
print(f"javap loadability gate: {count} classes checked")
for p, fq, err in fails:
    print(f"FAIL {p} ({fq}): {err}", file=sys.stderr)
print("javap-loadability:", "PASS (all op classes + inner companions loadable)" if not fails else f"FAIL ({len(fails)})")
sys.exit(1 if fails else 0)
PY
