# CALLBACK COST C — measured (TASK-72, S7-20, 2026-09-08)

Resolves re-open trigger 3 of `docs/AREAMAP_DENSE_DECISION.md` ("Measured
callback cost C << 40 ns via the TASK-68 rig, making the §7.2 upper bands
credible") — the last open unknown listed in `AREAMAP_DENSE_APPLY_DESIGN.md`
§7.1 ("C — главный источник неопределённости").

## Verdict (TL;DR)

| quantity | d=8 family | d=16 family | threshold |
|---|---|---|---|
| C_dispatch (apply loop + virtual dispatch floor) | **1.78 ns/op** | **1.85 ns/op** | ≪ 40 ✓ |
| C_full, shared-chunk corner (cheap production corner) | **53.2 ns/op** | **57.1 ns/op** | > 40 ✗ |
| C_full, solo-chunk corner (expensive production corner) | **61.8 ns/op** | **63.8 ns/op** | > 40 ✗ |

**Trigger 3 is REFUTED in the direction that STRENGTHENS the DEFER.** The
dispatch machinery is essentially free (≈ 1.8 ns — the Ops-bridge "minimal
apply loop" design claim is confirmed by measurement), but the REAL
production-shaped callback body costs **≈ 53-64 ns/op**, ABOVE the 40 ns
pessimism tier of the design's §7.2 band table. The §7.2 upper bands
(8-30× / 80-350× / 150-700×, computed at C = bench 1-3 ns) are therefore NOT
credible for production, and even the table's "до 2.5× / до 15× / до 30×"
(C = 40 ns) rows are optimistic by ≈ 30-60 % in the callback member.

## Method

Headless CPU-only rig (session tooling `/home/z/my-project/scripts/callbackcost/`,
preflight pattern; no server boot, no deploy, no `src/` change, BENCH.lock
`TASK-72-callbackcost`, JIT warmup 6000 updates per config, 5 windows × 2048
updates, median, fwd+rev arm-order passes, min-of-passes, least-squares over
n; JVM `-Xms1g -Xmx1g -Xmn512m`; `.so` sha256 `d8f821aa…` = the canonical
build; patched kernel bytes 3320 B regenerated green by the smoke rig rc=0).

Five arms over the SAME alternating rect stream (square (0,0,d) → (m,0,d) →
back; exactly n = 2·(2d+1)·m ops per move, verified per config on a fresh
CountMap before any timing — 22/22 VERIFY ok):

1. `direct` — `nativeUpdateOpsBatch` driven directly (NativeGate bridge, no
   apply loop): isolates the native enum+copy member.
2. `empty` — full patched `update()` with empty callback bodies: adds loop +
   virtual dispatch floor.
3. `count` — one counter increment per callback: minimal rig body.
4. `chunk_solo` — disassembly-faithful single-type `TrackedPlayer` steady-state
   shape (javap of the patched jar): CoordinateUtils-shaped key compute,
   real fastutil `Long2ReferenceOpenHashMap.get/put/remove`, real
   moonrise `ReferenceList` (map-backed, allocations included), `updateCount++`,
   empty `type.addTo/removeFrom` (verified empty in bytecode), and the
   `removeCallback` isEmpty → `byChunk.remove` + `directByChunk.remove`
   cleanup. EXPENSIVE production corner (chunk tracked by this map only).
5. `chunk_shared` — same class with a phantom second type keeping every chunk
   non-empty: no by-chunk map mutation, per-type `ReferenceList` still
   recreated on re-add (real `removePlayer` nulls the slot). CHEAP production
   corner.

Decomposition: `C_dispatch = b(empty) − b(direct)`; `C_body = b(tier) −
b(empty)`; `C_full = b(tier) − b(direct)` — slopes only, so the per-call
native copy member (L-const within each family) cancels.

Families: d=8 (m = 1,2,3,4,6,8,12 → n = 34..408) and d=16 (m = 1,2,4,8 →
n = 66..528). Two full runs (before/after an escalation fix, see caveats):
slope deltas < 7 %, all R² ≥ 0.998.

## Results (final run, min-of-passes linear fits)

```
---- family d=8 ----            b (ns/op)   a (ns)     R2
  direct                          4.170     1078.0   0.9994
  empty                           5.954     1121.2   0.9983
  count                           6.208     1135.3   0.9979
  chunk_solo                     65.953     1223.5   0.9999
  chunk_shared                   57.352      970.5   0.9993
---- family d=16 ----
  direct                          4.242     1049.2   0.9998
  empty                           6.091     1089.7   1.0000
  count                           6.513     1079.6   1.0000
  chunk_solo                     68.053     1196.1   0.9999
  chunk_shared                   61.345      569.6   0.9996

C_dispatch: 1.78 / 1.85          C_count: 0.25 / 0.42
C_shared:  51.40 / 55.25         C_solo:  60.00 / 61.96   (d=8 / d=16)
```

Intercept sanity (same-L after the escalation fix): `a_empty ≈ a_direct +
~40 ns` — the patched per-call overhead (ThreadLocal get, guards, field
writes) over the raw native call; `a_direct ≈ 1.05 µs` = JNI crossing +
L=2312 bulk copy — independently consistent with the PROBE copy model
(~0.5 ns/entry·byte+long × 2312) and with the same-state fast-path claim
(skipping ≈ 115 ns transition + buffer touch).

## Why the body costs ~55 ns (disassembly-backed)

Per callback op, alternating moves hit BOTH production branches:

- **remove** (steady-state cell leaving): `byChunk.get` (open-addressing
  probe) + `updateCount++` + `ReferenceList.remove` (Reference2IntOpenHashMap
  lookup + array swap) + size==0 → slot null + `directByChunk.remove` +
  (solo corner) `byChunk.remove` ≈ 45-60 ns.
- **add** (cell entering): `byChunk.get` (hit or miss) + (empty slot) NEW
  `ReferenceList` allocation + `directByChunk.put` + `byChunk.put` (solo
  corner, after prior remove) + `ReferenceList.add` ≈ 55-90 ns.

The real `TrackedPlayer` callbacks also pay a checkcast bridge
(`removeCallback(Object,int,int)` → cast → typed delegate, ≈ 1 ns) and, on
the solo-corner remove, a server-only
`ChunkSystemLevel.moonrise$releaseChunkData` call that is NOT emulable
headlessly — the solo tier is therefore a slight UNDERESTIMATE of that
corner.

## Impact on the DENSE decision (docs/AREAMAP_DENSE_DECISION.md updated)

- Production shapes (d ≤ 33, n ≈ 6d+2 ≈ 200 ops per crossing): dense total ≈
  n·C_full ≈ 11 µs vs today ≈ T_native(≈ 13-17 µs GENERAL copy-bound) +
  n·C_full(≈ 11 µs) ≈ 24-28 µs → **win ≈ 1.5-2.5×**, not the 3.5-15×
  projected at C = bench 1-3 ns. µs-scale, same order as variant C's already
  shipped win on the same path (TASK-64: move 3.1× at d=33) — dense adds
  nothing decisive there.
- Synthetic large grids (d ≥ 255): the optimistic 80-350× / 150-700× tiers
  collapse toward the C = 40 ns rows (до 15× / до 30×) and, at the measured
  C ≈ 53-64 ns, below them.
- **DEFER is re-affirmed with measured data; re-open trigger 3 is CLOSED**
  (refuted). Remaining re-open triggers: operator d ≥ 255 requirement;
  live-JFR frequency evidence.

## Bonus findings

1. **Callbacks dominate the patched path's post-native cost**: n·C_full ≈ 55
   ns/op vs native enum ≈ 4.2 ns/op + ≈ 1.05 µs L-copy. Any future
   area-map lever must target callback CHURN itself — coalescing is already
   rejected as semantics-violating (`AREAMAP_COALESCING_FEASIBILITY.md` §5),
   so this is closed as out of scope for the plugin.
2. The bench-tier inference in §7.1 ("residual ≈ 0.1 µs on ~380 ops" for the
   APPLY_BENCH CountingMap) is confirmed: the measured count-tier body
   (0.25-0.42 ns) + dispatch floor (1.8 ns) sits exactly in that range — the
   bench maps were cheap-body, production maps are not.

## Honest caveats

- Emulation, not the real `TrackedPlayer`: real fastutil + real
  `ReferenceList` + real branch structure, but synthetic param objects and no
  ServerLevel/ChunkData interactions (release/request paths on remove are
  missing → solo tier underestimates that corner; add path fully covered).
- Production has 4 map types sharing `byChunk`; the shared/solo tiers bracket
  the real per-callback cost at [≈ 53, ≈ 64] ns/op. Both are > 40 ns, so the
  verdict does not depend on where production actually sits in the bracket.
- GC: the chunk tiers allocate (faithful); `-Xmn512m` + median-of-5 windows +
  min-of-passes keep window spread tight (worst windows shown in RAW); slopes
  reproduced across two runs < 7 % delta.
- Rig lesson recorded: a same-state `update()` pre-escalation is a no-op (the
  fast path early-outs BEFORE the native call and never grows the Scratch) —
  the first run's d=8 family therefore ran at L=578 vs direct-arm L=2312;
  slopes were unaffected (L constant within each family) but the rig was
  re-run with a real-move escalation so intercepts are cross-arm comparable.

RAW: `bench/p500/results/CALLBACK_COST_RAW.log` (full TSV),
`bench/p500/results/CALLBACK_COST_VERDICT.txt` (regression output).
Rig: `/home/z/my-project/scripts/callbackcost/` (session tooling, preflight
pattern, not part of the repo).
