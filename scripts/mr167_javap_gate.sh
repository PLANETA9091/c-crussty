#!/usr/bin/env bash
# mr167_javap_gate.sh — Moonrise #167 parity-debt gate (S75, ROUND-468).
#
# Verifies the EntityCollectionBySection query-box Y-expansion state against
# vanilla (Tuinity/Moonrise #167, upstream fix PR #188 @d71d640a, merged
# 2026-08-07):
#   debt  (pre-fix):  4x "ldc2_w double 2.0d" — minY-2.0/dsub + maxY+2.0/dadd
#                     in BOTH getEntities and getEntitiesLimited
#   fixed (post-fix): minY-4.0 (ldc2_w 4.0d + dsub) + maxY+0.0 (ldc2_w 0.0d
#                     + dadd), pc-layout preserved (getEntities if_icmpgt 209,
#                     getEntitiesLimited if_icmpgt 223)
#
# Modes:
#   scripts/mr167_javap_gate.sh pre  [kernel.jar]   # expect DEBT (4x 2.0d)
#   scripts/mr167_javap_gate.sh post <patched-dir> [kernel.jar]  # expect FIXED
#
# Exit 0 = gate GREEN; exit 2 = ALREADY-FIXED upstream (kernel rebased past
# #188 — rebase mr167 lever away, do NOT patch); exit 1 = FAIL/drift.
set -uo pipefail
cd "$(dirname "$0")/.."

JAVAP=/home/z/tools/jdk-21.0.12.1+1/bin/javap
[ -x "$JAVAP" ] || JAVAP=$(command -v javap)
[ -x "$JAVAP" ] || { echo "FAIL: no javap" >&2; exit 1; }
KERNEL="${2:-/home/z/tools/patched-kernel.jar}"
MODE="${1:?usage: mr167_javap_gate.sh pre|post ...}"
FQ='ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection'

if [ "$MODE" = "pre" ]; then
  [ -f "$KERNEL" ] || { echo "FAIL: kernel jar $KERNEL missing" >&2; exit 1; }
  OUT=$("$JAVAP" -p -c -cp "$KERNEL" "$FQ" 2>&1) || { echo "FAIL: javap $FQ" >&2; exit 1; }
  N2=$(grep -c "double 2.0d" <<<"$OUT")
  N4=$(grep -c "double 4.0d" <<<"$OUT")
  N0=$(grep -c "double 0.0d" <<<"$OUT")
  echo "kernel $KERNEL: ldc2_w 2.0d=$N2, 4.0d=$N4, 0.0d=$N0"
  if [ "$N4" != "0" ] || [ "$N0" != "0" ]; then
    echo "GATE: ALREADY-FIXED upstream (kernel contains 4.0/0.0 constants) — rebase mr167 lever" >&2
    exit 2
  fi
  if [ "$N2" = "4" ]; then
    echo "GATE pre: DEBT CONFIRMED (4x 2.0d sites = Moonrise #167 present)"
    exit 0
  fi
  echo "FAIL: expected 4x 2.0d sites, got $N2 — kernel drift, patcher will refuse" >&2
  exit 1
fi

if [ "$MODE" = "post" ]; then
  DIR="${2:?usage: mr167_javap_gate.sh post <dir-with-nested-path> [kernel.jar]}"
  KERNEL="${3:-/home/z/tools/patched-kernel.jar}"
  P="$DIR/ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices\$EntityCollectionBySection.class"
  [ -f "$P" ] || { echo "FAIL: patched class missing: $P" >&2; exit 1; }
  OUT=$("$JAVAP" -p -c -cp "$DIR:$KERNEL" "$FQ" 2>&1) || { echo "FAIL: javap cannot load patched class" >&2; exit 1; }
  OK=1
  # exact site semantics: 4.0d+dsub (minY) x2, 0.0d+dadd (maxY) x2
  for pat in "double 4.0d" "double 0.0d"; do
    C=$(grep -c "$pat" <<<"$OUT")
    [ "$C" = "2" ] || { echo "FAIL: expected 2x '$pat', got $C" >&2; OK=0; }
  done
  C42=$(grep -A1 "double 4.0d" <<<"$OUT" | grep -c "dsub")
  C02=$(grep -A1 "double 0.0d" <<<"$OUT" | grep -c "dadd")
  [ "$C42" = "2" ] || { echo "FAIL: 4.0d not followed by dsub ($C42/2) — INVERTED constants" >&2; OK=0; }
  [ "$C02" = "2" ] || { echo "FAIL: 0.0d not followed by dadd ($C02/2) — INVERTED constants" >&2; OK=0; }
  # pc-layout preservation (LDC2_W->LDC2_W, no size drift)
  grep -q "if_icmpgt     209" <<<"$OUT" || { echo "FAIL: getEntities pc-layout drift (want if_icmpgt 209)" >&2; OK=0; }
  grep -q "if_icmpgt     223" <<<"$OUT" || { echo "FAIL: getEntitiesLimited pc-layout drift (want 223)" >&2; OK=0; }
  # no residual debt
  grep -q "double 2.0d" <<<"$OUT" && { echo "FAIL: residual 2.0d site present" >&2; OK=0; }
  if [ "$OK" = "1" ]; then
    echo "GATE post: FIXED = vanilla (-4.0/+0.0), pc-layout preserved, loadable"
    exit 0
  fi
  exit 1
fi

echo "FAIL: unknown mode $MODE" >&2
exit 1
