#!/usr/bin/env bash
# TASK-52 — refused-id e2e runner (runbook G8/A.6).
# Two arms, one JVM each, REAL closed-source kernels, exclusive BENCH.lock:
#   shipped — libcrussty.so built from origin/master: -3 rows (R0/R1/R2/R4/R5);
#             R3 SKIPs (drift-guard invariant: every table id allowed).
#   rig     — detached worktree @origin/master + 1-entry hand-patch appending
#             DO_NOT_WIRE LevelChunkHeightmap.newCombinedUpdateSummary (shape A,
#             id 15) to KERNELS: true ERR_KERNEL_REFUSED (-10) e2e, incl. the
#             mixed-batch no-partial-execution proof. Rig never lands; the
#             patch diff is recorded in the report. Worktree removed on exit.
# Raw: bench/batch/refused_e2e/results/BATCH_REFUSED_ID_E2E_RAW.tsv (+ logs/).
set -u
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"   # repo root
cd "$ROOT"

JDK="${JDK:-/home/z/jdk21}"
NATIVE_LIB="$ROOT/native/libpaper_native_jni.so"
LOCK=/home/z/BENCH.lock
WT=/tmp/w-t52-rig

[[ -x "$JDK/bin/java" ]] || { echo "JDK missing: $JDK" >&2; exit 2; }
[[ -f "$NATIVE_LIB" ]]   || { echo "native lib missing: $NATIVE_LIB" >&2; exit 2; }
source "$HOME/.cargo/env" 2>/dev/null || export PATH="$HOME/.cargo/bin:$PATH"

# ---- compile fixture on the fly (never committed) --------------------------
CLASSES="$ROOT/bench/batch/refused_e2e/classes"
rm -rf "$CLASSES"; mkdir -p "$CLASSES"
"$JDK/bin/javac" -d "$CLASSES" \
  "$ROOT"/bench/batch/java/crussty/batch/PaperNativeBatchDispatch.java \
  "$ROOT"/bench/batch/refused_e2e/java/RefusedIdE2E.java || exit 2

RES="$ROOT/bench/batch/refused_e2e/results"
LOGS="$RES/logs"; mkdir -p "$LOGS"
RAW="$RES/BATCH_REFUSED_ID_E2E_RAW.tsv"
: > "$RAW"

run_arm() { # $1 label, $2 module .so, $3 kernels, $4 rigRefusedId(-1 shipped)
  local label="$1" lib="$2" kn="$3" rid="$4"
  echo "=== arm $label (lib=$lib kernels=$kn rig_id=$rid) ===" >&2
  CRUSSTY_BATCH_NATIVE_LIB="$NATIVE_LIB" \
  "$JDK/bin/java" -Xms512m -Xmx512m -XX:+UseG1GC \
    -cp "$CLASSES" RefusedIdE2E --lib "$lib" --native "$NATIVE_LIB" \
    --kernels "$kn" --rig-refused-id "$rid" \
    > "$LOGS/refused_$label.out" 2> "$LOGS/refused_$label.log"
  local rc=$?
  grep -E "^E2E" "$LOGS/refused_$label.out" >> "$RAW"
  echo "# arm=$label exit=$rc" >> "$RAW"
  if [ $rc -ne 0 ]; then
    echo "arm $label: FIXTURE FAILED (exit $rc)" >&2
    tail -5 "$LOGS/refused_$label.log" | sed 's/^/  log> /' >&2
  fi
  return $rc
}

# ---- shipped arm: build module .so from origin/master if absent ------------
SHIPPED_LIB="$ROOT/target/release/libcrussty.so"
if [[ ! -f "$SHIPPED_LIB" ]]; then
  echo "building shipped module .so (origin/master worktree)..." >&2
  SHIPPED_WT=/tmp/w-t52-shipped
  rm -rf "$SHIPPED_WT"
  git worktree add --detach "$SHIPPED_WT" origin/master >/dev/null 2>&1
  (cd "$SHIPPED_WT" && cargo build --release 2>&1 | tail -1) >&2
  SHIPPED_LIB="$SHIPPED_WT/target/release/libcrussty.so"
fi
[[ -f "$SHIPPED_LIB" ]] || { echo "shipped lib missing: $SHIPPED_LIB" >&2; exit 2; }

# ---- rig arm: worktree + 1-entry hand-patch --------------------------------
rig_build() {
  rm -rf "$WT"
  git worktree add --detach "$WT" origin/master >/dev/null 2>&1 || return 1
  python3 - "$WT/src/batch_table.rs" << 'PYEOF'
import sys
p = sys.argv[1]
s = open(p).read()
# fixed-size array must grow with the appended entry
old_decl = "pub const KERNELS: [BatchKernel; 15] = ["
new_decl = "pub const KERNELS: [BatchKernel; 16] = ["
if old_decl not in s:
    print("KERNELS decl not found", file=sys.stderr); sys.exit(1)
s = s.replace(old_decl, new_decl, 1)
anchor = "];\n\n/// Slice view of the compile-time table"
rig = """    // TASK-52 RIG ENTRY (never lands): DO_NOT_WIRE kernel, refused by the
    // batch gate -> true ERR_KERNEL_REFUSED e2e. Shape A matches its JNI sig.
    BatchKernel {
        id: 15,
        shape: Shape::A,
        class: "PaperNativeLevelChunkHeightmap",
        method: "newCombinedUpdateSummary",
        sig: "(I[J)I",
        symbol: "Java_PaperNativeLevelChunkHeightmap_newCombinedUpdateSummary",
    },
];
"""
if anchor not in s:
    print("anchor not found", file=sys.stderr); sys.exit(1)
s = s.replace(anchor, rig + "\n/// Slice view of the compile-time table", 1)
open(p, "w").write(s)
print("rig entry appended, KERNELS 15->16")
PYEOF
  [[ $? -eq 0 ]] || return 1
  rg -c "id: 15," "$WT/src/batch_table.rs" >/dev/null || { echo "rig patch verify failed" >&2; return 1; }
  (cd "$WT" && cargo build --release 2>&1 | tail -1) >&2
  [[ -f "$WT/target/release/libcrussty.so" ]]
}

# ---- exclusive bench lock over both arms -----------------------------------
exec 9>"$LOCK"
flock 9
echo "# BENCH.lock acquired $(date -u +%FT%TZ) by agent-7625532f (TASK-52 refused-id e2e)" >&2
FAIL=0
run_arm shipped "$SHIPPED_LIB" 15 -1 || FAIL=1
if rig_build; then
  run_arm rig "$WT/target/release/libcrussty.so" 16 15 || FAIL=1
else
  echo "# arm=rig BUILD_FAILED" >> "$RAW"; FAIL=1
fi
flock -u 9

# ---- cleanup rig worktree (shipped worktree only if we created it) ---------
git worktree remove --force "$WT" 2>/dev/null
[[ "$SHIPPED_LIB" == /tmp/* ]] && git worktree remove --force /tmp/w-t52-shipped 2>/dev/null

echo "--- raw ---"; cat "$RAW"
exit $FAIL
