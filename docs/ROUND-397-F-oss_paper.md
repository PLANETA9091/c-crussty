# ROUND-397-F — vector `oss_paper`: Canvas item-merge optimizations (TASK-397-F)

Branch `round-397-f-oss_paper` · flag `CRUSSTY_LEVER_FLAG=items_paper` (`CRUSSTY_LEVER_ARG` accepted, unused).
Research proofs: `/home/z/rounds/ROUND-397/RESEARCH-F.md` (Canvas ItemEntity.java.patch +
WorldConfig.java, PaperMC/Paper ItemEntity.java.patch, kernel javap ground truth).

## What is ported (2 mechanics, ONE class, ZERO byte growth, no ops bridge, no MethodHandles)

`src/item_paper.rs` performs size-neutral bytecode surgery on the kernel class
`net/minecraft/world/entity/item/ItemEntity` (byte hook + retransform, fluid_guard discipline):

| # | Upstream (Canvas, Paper ecosystem) | Patch | Deviation |
|---|---|---|---|
| P1 | `itemEntitiesWaitTwoSecondsForMergeCheckAlways=true` (Canvas WorldConfig.java L340–353) | `tick()V` @434: `ifeq +7` → `goto +7` (opcode 0x99→0xa8) — moved-path `iconst_2` dead, merge-check `rate` ≡ 40 | **DOC-DEV-1** below |
| P2 | Canvas merge-loop exit (`if (!this.isMergable()) break;`) | `mergeWithNeighbours()V` @156: `invokevirtual isRemoved` → `invokevirtual isMergable` (CP-u2 swap to existing #459) + `ifeq`→`ifne` | **None** |

Exactly 4 bytes differ from the pristine kernel class (verifier-checked in `verify_patch`).

## DOC-DEV-1 (P1, documented superiority deviation)

Vanilla 1.21.x cadence is `rate = moved ? 2 : 40`: an item that crossed a block boundary this
tick merge-scans every 2 ticks. With P1 armed the scan is intervaled to 40 ticks for moved items
too — exactly the semantic of the upstream Canvas config
`itemEntitiesWaitTwoSecondsForMergeCheckAlways=true` ("This forces the interval to always be 2
ticks, reducing the amount of times item entities check to merge" — Canvas WorldConfig docs).

Consequences (all merge/pickup/despawn RESULTS are unchanged):
- merge latency for a moving (falling/tumbling) item becomes ≤2s instead of ≤1 tick;
- resting items keep the vanilla 40-tick cadence unchanged;
- merge with neighbours on a scan tick remains the vanilla all-candidates loop (P2 only skips
  iterations that are provable no-ops after `this` became full — `tryToMerge`'s own guard
  `areMergable(other.count + this.count <= max)` cannot pass when `this` is full, and removed
  `this` is not `isAlive()` hence not mergeable either — vanilla break-on-removed preserved).

## Parity discipline

- Flag OFF (empty/foreign `CRUSSTY_LEVER_FLAG`): `register()` no-ops — no byte hook, no
  retransform, module byte-indistinguishable from master. EXACT vanilla path by construction.
- Flag ON: 4-byte patch, statically re-verified (`verify_patch`: both patterns re-scanned,
  CP operands re-resolved, byte-diff count == 4) and verified against the REAL kernel class
  bytes (`cargo test --lib item_paper`, live `ItemEntity.class` from purpur-1.21.10) and by
  HotSpot's verifier (`defineClass` + link of the patched bytes, JDK21, `-Xverify:all`).
- Size neutrality ⇒ Code length, exception table, StackMapTable byte-identical; no new branch
  targets (P1 dead region carries no frames; P2 keeps targets 165/168 and their stack shapes).

## Boot log markers

`[crussty-plugin] item_paper: dormant ...` / `pristine sighting` / `computed size-neutral patch`
/ `hook armed, retransform rc=` — grep these in server stdout to confirm arming.
