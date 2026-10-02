# Area-map same-tick run() coalescing — feasibility study (TASK-38)

**Verdict: NEGATIVE — plugin-side same-tick coalescing of `SingleUserAreaMapOps.run()`
is moot for the real call pattern. Direction closed at the Java/plugin layer. No
implementation shipped (per the investigate-first gate: implement only if provably
semantics-preserving AND profitable — neither holds).**

Agent: agent-7625532f (TASK-38-w5) · 2026-09-08 · base commit `29aa1c7` (origin/master)

---

## 1. The question

`bench/areamap/results/APPLY_BENCH.md` (TASK-20, `beaf374`) showed the production
native apply is bound by FULL JNI copy-in/out of the ops+keys buffers (~12 GB/s,
cost ∝ total buffer bytes; 4× buffer ⇒ 7–11.5× cost) while the actual diff is
~2–4 columns. The full fix needs a bridge/`.so` change (out of scope). The
residual plugin-side idea: if multiple `run()` invocations happen **per tick**,
accumulate ops within one tick and flush ONCE, cutting the *number* of native
calls (each carries the fixed per-call copy overhead). Does the real call
pattern justify it?

**Gate applied (from the task brief):** if `run()` is effectively called ≤1
native time per tick per map instance, coalescing is MOOT → document and skip.

## 2. Method (evidence, not guesswork)

Disassembled the exact server the live PIDs 26544/26562 run (Purpur
`1.21.10-2535-HEAD@0a2dc04`, Moonrise inlined): paperclip patch+reobf executed
in a throwaway copy dir (`/tmp/amprobe`, copies only — `/home/z/server` never
written), then `javap -c` on the remapped jar
(`io.papermc.paperclip`-output `versions/1.21.10/purpur-1.21.10.jar`). Every
edge below is a verified `invokevirtual/static` site, not recalled behavior.
The plugin's own pollers (TASK-20-R note: 2s/10s activation cadence in
`src/area_map.rs`) are unrelated — they only arm the hook; the call pattern
below lives entirely in the server tick loop.

## 3. Call graph and cadence of `SingleUserAreaMap.update(III)` (→ patched → `run()`)

The ONLY caller chain into `SingleUserAreaMap.update` is
`NearbyPlayers.tickPlayer(ServerPlayer)` ([M] sole non-own reference besides
the abstract class itself; 6 call sites inside it):

```
PER-TICK anchor:
  ServerPlayer.tick()                          [1×/tick/player — player entity tick]
    └─ ServerChunkCache.move(player)           (bytecode @196 in tick())
         └─ ChunkMap.move(player)
              └─ BaseChunkSystemHooks.updateMaps
                   └─ RegionizedPlayerChunkLoader.updatePlayer(player)
                        └─ NearbyPlayers.tickPlayer(player)
                             └─ 6 × TrackedPlayer.update(III)   ← 6 DISTINCT map instances

EVENT-driven callers of the same NearbyPlayers.tickPlayer(player):
  ServerEntityLookup.entitySectionChangeCallback(...)   [player crosses a chunk section]
  ServerGamePacketListenerImpl.handleMoveVehicle(...) → ServerChunkCache.move  [vehicle move]
  ServerPlayer.setCamera(Entity)                        [spectator camera switch]
```

`tickPlayer` updates one `NearbyPlayers$TrackedPlayer` (subclass of
`SingleUserAreaMap`) **per `NearbyMapType`** — 6 separate map instances per
player, with distances [M] from the disassembly (live config: view-distance=10,
simulation-distance=10 from `server.properties`):

| # | NearbyMapType       | distance            | maxOps(d,d)=2·(2d+1)² | JNI copy bytes/call (in+out, 18·cap) | est. @~12 GB/s |
|---|---------------------|---------------------|-----------------------|--------------------------------------|----------------|
| 1 | GENERAL             | MAX_VIEW_DISTANCE+1 = 33 | 8 978            | ~158 KB                              | ~13.2 µs       |
| 2 | GENERAL_SMALL       | 10 (hardcoded)      | 882                   | ~15.5 KB                             | ~1.3 µs        |
| 3 | GENERAL_REALLY_SMALL| 3 (hardcoded)       | 98                    | ~1.7 KB                              | ~0.15 µs       |
| 4 | TICK_VIEW_DISTANCE  | sim distance = 10   | 882                   | ~15.5 KB                             | ~1.3 µs        |
| 5 | VIEW_DISTANCE       | view distance = 10  | 882                   | ~15.5 KB                             | ~1.3 µs        |
| 6 | SPAWN_RANGE         | 8 (hardcoded)       | 578                   | ~10.2 KB                             | ~0.9 µs        |

(Copy-byte math from APPLY_BENCH: in+out = 2·(cap·1 + cap·8) = 18·cap; the last
column is [E]stimate from the measured ~12 GB/s probe, not a new measurement.)

## 4. Per-instance per-tick analysis — why coalescing has nothing to do

Key mechanism ([M], `SingleUserAreaMapOps.run()` + patched `update()` body):
the patched `update()` writes `lastChunkX/Z/distance` **before** invoking
`run()`; `run()` skips the native call entirely when `from==to && oldD==newD`
(same-state fast path, 0 native cost) or `fromX==MIN_VALUE` (post-`remove()`).

Therefore, for ONE map instance (player, type) within ONE tick:

- **Idle player (no section crossing this tick):** 1 update from
  `ServerPlayer.tick()` → same-state → fast path, **0 native calls**.
- **Player crossed sections this tick:** the section-change event fires
  `tickPlayer` first (mid packet handling) → 1 **native call** per changed map;
  the same tick's `ServerPlayer.tick()` then re-runs `tickPlayer` → every map
  is now same-state → fast path, **0 native calls**. (Order of the two
  invocations within the tick is irrelevant: whichever fires second is
  same-state, because the first already wrote the fields.)
- **>1 native call per instance per tick** requires the map state to change
  ≥2× within one tick — i.e. two border-crossing movement packets in the same
  tick, or a move plus a view-distance change in the same tick. Uncommon, and
  — decisively — **not coalescable semantics-preservingly** (§5).

So the same-tick window per map instance contains **at most one native call in
practice**; all duplicate same-tick invocations are already the O(1) fast path
that was measured at 24.7–25.3 ns/update with 0 native calls
(`docs/BOOST_SWEEP.md` #1, live-verified). **There is nothing left to
coalesce.** The premise "multiple run() invocations per tick" is true only in
the count-of-invocations sense (6–12 per player per tick); in the sense that
matters — multiple *native* calls per tick on the same data — it is false.

## 5. Why even the rare multi-native-call tick is rejected

Merging sequential updates (A→B)+(B→C) into one native call (A→C) elides the
intermediate `remove(B∖A)`+`add(B∖A)` callback churn. Callbacks are
side-effecting and read mid-tick: `TrackedPlayer.add/removeCallback` mutate the
per-chunk `TrackedChunk` player lists that spawn/ticking/tracking logic consults
between updates within the same tick. Delivering a different callback sequence
(elided churn, reordered ops) violates the task's hard rule
("same results visible at the same tick boundaries"). Deferring callbacks to a
tick-end flush is likewise deferred visibility across tick phases — rejected.

## 6. Cross-instance batching is the only real lever — and it is out of scope

The 6 per-player native calls are on 6 *distinct* map instances (different
state, different callbacks). Coalescing those would need a multi-map batch
native API = bridge + `.so` change — explicitly out of scope for this task,
and already tracked as the wave-3 candidate (`APPLY_BENCH.md` §Negative
result: diff-budget window ~8·d, 64–256× fewer bytes/call at d=511; plus
`docs/BATCH_WIRING_PLAN.md`). Note the dominant instance there is GENERAL
(d=33): its `long[]` keys buffer dominates the copy cost, so the diff-budget
direction also subsumes most of what a 6→1 merge could theoretically save.

## 7. Upper-bound math (one-liner, honest)

[A] Per border-crossing player-tick, all 6 native calls total ≈ 17 µs of
buffer-copy (table §3, GENERAL d=33 dominates at ~13 µs); a *hypothetical
impossible* 6→1 cross-instance merge would cap the win at ≈ 4 µs per
crossing player-tick (≈0.008% of a 50 ms tick) — and would still require the
out-of-scope bridge+`.so` change — while the **in-scope same-instance variant
saves exactly 0**: the typical tick has ≤1 native call per map instance and the
duplicate invocations are already the 0-native-call fast path.

## 8. Verdict and follow-ups

- **NEGATIVE.** Same-tick coalescing at the Java/plugin layer: moot call
  pattern (≤1 native call per map instance per tick), and the residual rare
  case is semantics-violating. No code shipped; `.so`, `/home/z/CRUSSTY`, live
  server untouched. No benchmark claims (nothing to benchmark).
- The profitable directions remain the already-tracked native-side ones:
  diff-budget-sized buffers / direct buffers (APPLY_BENCH §Negative result,
  wave-3) and batch-API wiring (TASK-28 `BATCH_WIRING_PLAN.md`). Those need
  bridge+`.so` work and are out of scope here.
- TASK-30 tie-in: the TASK-30 oracle (645 vs 374 ops/call) was a bench-stream
  artifact, not a production cadence measurement; production cadence is the
  call graph in §3 — idle players never reach the native kernel at all.

## Evidence index

- Remapped jar: `/tmp/amprobe/j/versions/1.21.10/purpur-1.21.10.jar` (throwaway;
  produced by paperclip patch+reobf from COPIES of the live server's
  `versions/`+`cache/` jars; boot log line "Loading Purpur 1.21.10-2535-HEAD@0a2dc04").
- `javap -c` evidence: `ServerPlayer.tick()` → `ServerChunkCache.move` (@196);
  `ChunkMap.move` → `PlatformHooks.updateMaps`; `BaseChunkSystemHooks.updateMaps`
  → `RegionizedPlayerChunkLoader.updatePlayer` → `NearbyPlayers.tickPlayer`
  (6 × `TrackedPlayer.update(III)` @57/79/100/129/158/180, distances above);
  `ServerEntityLookup.entitySectionChangeCallback` → `tickPlayer`;
  `MoonriseConstants.MAX_VIEW_DISTANCE` default 32 (`Integer.getInteger(...,32)`).
- Distances/copy-cost model: APPLY_BENCH.md (12 GB/s probe, 18·cap bytes/call);
  live `server.properties` view/sim = 10/10.
