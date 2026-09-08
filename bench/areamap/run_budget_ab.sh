#!/usr/bin/env bash
# TASK-64 variant C — TASK-30 oracle re-run on the BUDGETED scratch policy
# (FAKE contract-emulating + REAL closed native + LEGACY REAL control) plus
# the A/B bench legacy-vs-budgeted (docs/AREAMAP_DENSE_APPLY_DESIGN.md
# §11.4 rows 4-5).
#
# File-disjoint from the sibling rigs (classes-budget-*, classes-legacy-*,
# classes-ab-*, results/areamap_budget_*): the shared smoke/oracle fake, the
# legacy build and every foreign WIP dir are NOT touched. The budgeted
# bridge is selected purely by classpath (area-map/build-budget bytes).
#
# ORACLE = the parity gate before any arm of this lever may go live
# (§11.2 falsifier 2: hidden len-dependent semantics of the closed native).
# The script aborts (set -e) if any oracle arm fails parity; the LEGACY
# REAL control re-runs the canonical TASK-30 streams to prove the rig.
#
# Usage: bench/areamap/run_budget_ab.sh [java-bin]   (default /home/z/jdk21/bin)
set -euo pipefail
cd "$(dirname "$0")/../.."
JAVA_BIN="${1:-/home/z/jdk21/bin}"

PKG=ca/spottedleaf/moonrise/common/misc
SO="$(pwd)/native/libpaper_native_jni.so"

[ -f "area-map/build/$PKG/SingleUserAreaMapOps.class" ] || scripts/build_area_map.sh
[ -f "area-map/build-budget/$PKG/SingleUserAreaMapOps.class" ] || scripts/build_area_map_budget.sh

cd bench/areamap
mkdir -p results
rm -rf classes-budget-oracle-fake classes-budget-oracle-real classes-legacy-oracle-real classes-ab-legacy classes-ab-budget
mkdir -p classes-budget-oracle-fake classes-budget-oracle-real classes-legacy-oracle-real classes-ab-legacy classes-ab-budget
for d in classes-budget-oracle-fake classes-budget-oracle-real classes-legacy-oracle-real classes-ab-legacy classes-ab-budget; do
  mkdir -p "$d/$PKG"
done

# bridge classes into each classpath (bench classes compile against them)
cp "../../area-map/build/$PKG/"SingleUserAreaMapOps*.class "classes-legacy-oracle-real/$PKG/"
cp "../../area-map/build/$PKG/"SingleUserAreaMapOps*.class "classes-ab-legacy/$PKG/"
cp "../../area-map/build-budget/$PKG/"SingleUserAreaMapOps*.class "classes-budget-oracle-fake/$PKG/"
cp "../../area-map/build-budget/$PKG/"SingleUserAreaMapOps*.class "classes-budget-oracle-real/$PKG/"
cp "../../area-map/build-budget/$PKG/"SingleUserAreaMapOps*.class "classes-ab-budget/$PKG/"

# --- compile the four arms ---
"$JAVA_BIN/javac" -nowarn -cp classes-budget-oracle-fake -d classes-budget-oracle-fake \
  budgetab/$PKG/PaperNativeAreaMap.java \
  $PKG/SingleUserAreaMap.java \
  benchjava/$PKG/OracleBench.java

"$JAVA_BIN/javac" -nowarn -cp classes-budget-oracle-real -d classes-budget-oracle-real \
  realdecl/$PKG/PaperNativeAreaMap.java \
  $PKG/SingleUserAreaMap.java \
  benchjava/$PKG/OracleBench.java

"$JAVA_BIN/javac" -nowarn -cp classes-legacy-oracle-real -d classes-legacy-oracle-real \
  realdecl/$PKG/PaperNativeAreaMap.java \
  $PKG/SingleUserAreaMap.java \
  benchjava/$PKG/OracleBench.java

"$JAVA_BIN/javac" -nowarn -cp classes-ab-legacy -d classes-ab-legacy \
  realdecl/$PKG/PaperNativeAreaMap.java \
  $PKG/SingleUserAreaMap.java \
  budgetab/benchjava/$PKG/BudgetAbBench.java

"$JAVA_BIN/javac" -nowarn -cp classes-ab-budget -d classes-ab-budget \
  realdecl/$PKG/PaperNativeAreaMap.java \
  $PKG/SingleUserAreaMap.java \
  budgetab/benchjava/$PKG/BudgetAbBench.java

# --- oracle battery (exit gate: parity on every call, both modes) ---
TSV=results/areamap_budget_oracle_raw.tsv
: > "$TSV"

echo "== BUDGET oracle FAKE (contract-emulating fake, budgeted scratch) ==" | tee -a "$TSV"
"$JAVA_BIN/java" -Xms512m -Xmx512m -Dcrussty.areamap.budget=true \
  -cp classes-budget-oracle-fake "$PKG.OracleBench" 2>&1 | tee -a "$TSV"

echo "== BUDGET oracle REAL (closed native, budgeted scratch) — PARITY GATE ==" | tee -a "$TSV"
"$JAVA_BIN/java" -Xms512m -Xmx512m -Dcrussty.areamap.budget=true \
  -Dcrussty.native="$SO" -cp classes-budget-oracle-real "$PKG.OracleBench" 2>&1 | tee -a "$TSV"

echo "== LEGACY oracle REAL control (closed native, cap scratch) ==" | tee -a "$TSV"
"$JAVA_BIN/java" -Xms512m -Xmx512m -Dcrussty.native="$SO" \
  -cp classes-legacy-oracle-real "$PKG.OracleBench" 2>&1 | tee -a "$TSV"

echo "oracle raw TSV: $TSV"

# --- A/B bench under BENCH-MUTEX (REAL .so timing runs) ---
LOCK=/home/z/BENCH.lock
STAMP="$(date -u +%FT%TZ)"
echo "lock agent-7625532f TASK-64-budget-ab $STAMP" >> "$LOCK"
cleanup() { echo "done agent-7625532f TASK-64-budget-ab" >> "$LOCK"; }
trap cleanup EXIT

AB=results/budget_ab_raw.tsv
: > "$AB"
for arm in legacy budget; do
  for d in 33 63 255 511; do
    case "$d" in
      33|63) CALLS=2000 ;;
      255)   CALLS=200 ;;
      511)   CALLS=100 ;;
    esac
    for shape in move mix; do
      "$JAVA_BIN/java" -Xms512m -Xmx512m \
        -Dcrussty.native="$SO" -Dcrussty.arm="$arm" \
        -cp "classes-ab-$arm" "$PKG.BudgetAbBench" "$d" "$shape" "$CALLS" 2>&1 | tee -a "$AB"
    done
  done
done

# --- cross-arm multiset parity (identical native + streams) ---
PARITY=results/budget_ab_parity.txt
: > "$PARITY"
ALL_OK=1
for d in 33 63 255 511; do
  for shape in move mix; do
    a="$(grep -E "^BUDGETAB arm=legacy d=$d shape=$shape " "$AB" | grep -oE "fold_sum=[-0-9]+ fold_xor=[-0-9]+" | head -1 || true)"
    b="$(grep -E "^BUDGETAB arm=budget d=$d shape=$shape " "$AB" | grep -oE "fold_sum=[-0-9]+ fold_xor=[-0-9]+" | head -1 || true)"
    if [ -n "$a" ] && [ "$a" = "$b" ]; then
      echo "PARITY-OK d=$d shape=$shape ($a)" >> "$PARITY"
    else
      echo "PARITY-DIFF d=$d shape=$shape legacy=[$a] budget=[$b]" >> "$PARITY"
      ALL_OK=0
    fi
  done
done
cat "$PARITY"

echo ""
echo "A/B raw TSV: $AB"
if [ "$ALL_OK" -eq 1 ]; then
  echo "BUDGET-AB VERDICT: cross-arm multiset parity OK on all 8 (d,shape) pairs"
else
  echo "BUDGET-AB VERDICT: PARITY-DIFF present — investigate before any rollout"
  exit 1
fi
