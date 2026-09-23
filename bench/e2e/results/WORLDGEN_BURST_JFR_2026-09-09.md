# WORLDGEN-BURST JFR PROFILE — real-load profile; g9 revisit trigger FIRED

* TASK-62, 2026-09-09, agent-7625532f. First REAL-WORKLOAD profile of this box (TASK-57 was idle/boot-only).
* Profile files: `/home/z/server/logs/jfr_task62/` — `task62_burst_dump.jfr` (6.1 MB, mid-burst explicit dump,
  window boot→03:28, PRIMARY), `server_15834.jfr` (7.7 MB, full session incl. dumponexit at graceful stop),
  `server_15808.jfr` (launcher), raw sample dump `exec_samples_task62.txt` (2610 samples).
* Analysis scripts: `/home/z/my-project/scripts/jfr_task62_analyze.py` (frame/stack/thread aggregation).
* Server state after session: STOPPED (as found). **World RESTORED byte-identical** from pre-task tar
  (`diff -r` clean) — the burst deliberately generated 128 fresh chunks, then `forceload remove all` +
  graceful stop + backup restore returned the tree to as-found state. Token not exposed.

## 1. Verdict

**The g9 revisit trigger (TASK-57 §2) has FIRED.** Criterion was: "a profile with real player-driven
worldgen showing `fillArray` frames". Measured: **120 of 2610 samples contain `fillArray` in stack
(6.1% of the burst minute)** — `DensityFunctions$PureTransformer.fillArray ← DensityFunctions$Ap2.fillArray
(×2 nested) ← NoiseChunk.selectCellYZ ← NoiseBasedChunkGenerator.doFill` on the Paper Common Worker pool.
TASK-57's "0 of 927 — method is absent" verdict was true for idle/boot profiles and was an artifact of the
world being pre-generated: terrain-fill never ran there. It is NOT true under fresh worldgen. The g9
assessment reopens with new data (§4 F1); the other NO-GO leg (scalar-mapping semantics undocumented) is
unchanged and remains the blocker for any hook design.

## 2. Method

* Boot: `scripts/e2e_orchestrate.sh boot` (TASK-59 stdin lifeline active), dormant env (no `CRUSSTY_*`
  overrides — production default, noise-native bridge DORMANT), deployed module at HEAD 966e0c4,
  `Done (17.337s)`. JFR injection per validated TASK-57 recipe:
  `JAVA_TOOL_OPTIONS="-XX:StartFlightRecording=filename=/home/z/server/logs/jfr_task62/server_%p.jfr,dumponexit=true,settings=profile,maxsize=128M"`
  (both JVMs recorded; 0-byte files while running is normal).
* Relay-alive proof pre-burst: `forceload query` via fifo → correct response in 3 s (TASK-59 fix working live).
* Burst: two distant fresh-generation regions far from the pre-generated spawn area —
  `forceload add 1600 1600 1727 1727` (chunks 100..107 × 100..107, 64 chunks) and
  `forceload add -1728 -1728 -1601 -1601` (chunks -108..-101, 64 chunks) = **128 fresh chunks**, full
  density-function terrain pipeline. Severity evidence: Watchdog fired at 03:27:23 — "The server has not
  responded for 10 seconds" — Server thread parked in `ChunkTaskScheduler` chunk wait (102,101) while
  worldgen ran on the worker pool. Both regions marked 64/64.
* Explicit mid-burst `jcmd <child> JFR.dump` (6.1 MB) + dumponexit final at graceful fifo `stop`
  (launcher exited cleanly — second live validation of the TASK-59 fix).

## 3. Evidence

Timeline (jdk.ExecutionSample, 10 ms period; ~26.1 CPU-s runnable-Java total over boot→03:28):

| minute | samples | fillArray samples | share |
|--------|---------|-------------------|-------|
| 03:25 (boot) | 122 | 0 | 0.0% |
| 03:26 (idle tick) | 468 | 0 | 0.0% |
| **03:27 (burst)** | **1959** | **120** | **6.1%** |
| 03:28 (settle/dump) | 61 | 0 | 0.0% |

Thread distribution: burst-minute runnable CPU is **86% on one thread** — `Paper Common Worker #0`
(1690/1959); Watchdog 189 (printing dumps during the stall), Server thread 75 (parked in chunk wait).
Whole-dump: Worker #0 = 1693/2610 (65%). All 120 fillArray samples are on Worker #0 (7.1% of its runnable time).

Ap2 surface split under real load: **fillArray = 88 samples, compute = 138 samples** — TASK-57 saw ONLY
compute (261+146) because terrain-fill never ran; now both legs are alive, compute still leads
(biome/climate sampling runs concurrently inside the same burst).

fillArray stack anatomy (leaf-inside-fillArray-subtree → entry):
* Leaves inside the fillArray subtree: `Ap2.compute` (13), `Mth.lerp3` (12), `NoiseChunk$NoiseInterpolator.compute`
  (12), `NoiseChunk.fillAllDirectly` (7), `RangeChoice.compute` (7) — the fillArray path funnels straight
  into the same noise-leaf cluster as the single-value path.
* Entry frames: `Ap2.fillArray` (45), `PureTransformer.fillArray` (27),
  `NoiseBasedChunkGenerator.lambda$fillFromNoise$11` (21), `doFill` (11), `selectCellYZ` (8) — i.e. the
  worldgen tasks themselves ARE the noise fill (no unrelated caller).

Hot leaves overall (burst): `Mth.lerp3` 3.40 CPU-s, `NoiseChunk.updateForZ` 1.37 s,
`WatchdogThread.run` 1.02 s, `Climate$RTree$SubTree.search` 0.72 s, `ImprovedNoise.noise` 0.71 s,
`PerlinNoise.getValue` 0.64 s, `Aquifer.computeSubstance` 0.61 s.

Anywhere-in-stack (burst window): `NoiseChunk` 7.22 s (`doFill` 5.73 s), `ImprovedNoise`/`PerlinNoise`
4.12 s, `Mth.lerp` 3.42 s, `Ap2` 2.14 s, `Climate` 1.62 s, `NormalNoise` 1.40 s, `BlendedNoise` 0.87 s,
`PureTransformer` 1.21 s, `selectCellYZ` 0.46 s. `libcrussty` frames: 0 samples (dormant env — expected).

## 4. Findings

**F1 — g9 reopen assessment (trigger fired, one blocker remains).** The amplification leg of the g9 NO-GO
is invalidated for real worldgen: the target method runs at ~6% of burst CPU. Measured ceiling for any
whole-method fillArray hook: **1.2 CPU-s per 128-chunk fresh-gen burst (6.1% of burst-minute CPU)** — an
upper bound, since a hook removes only the batched-op cost inside fillArray, not the whole subtree. The
scalar-mapping leg (batch semantics of the target op undocumented — TASK-57 §2) is UNCHANGED and still
blocks any hook design. Comparison that actually decides priorities: the **existing proven noise-native
bridge targets 4.12 CPU-s of the same burst** (ImprovedNoise/PerlinNoise anywhere-in-stack) — 3.4× the
entire fillArray surface — and is already landed and P500-proven. Verdict: g9 stays effectively behind the
documented blocker, but the "method is absent" wording in G9_WHOLE_METHOD_HOOK_DESIGN §8 must be corrected
to "absent in idle/boot; ~6% of fresh-worldgen burst CPU (TASK-62 F1)"; revisit economics now measured.

**F2 — the proven bridge's real-load opportunity, quantified.** Dormant-config burst profile = direct
measure of what `CRUSSTY_NATIVE_IMPROVED_NOISE=1` replaces under real load: 4.12 CPU-s ImprovedNoise/
PerlinNoise over a 128-chunk burst (21% of burst-minute runnable CPU), consistent with P500 ~2x-under-
contention premium → ~2 CPU-s recoverable per 128-chunk burst equivalent. This is the first real-load
corroboration of the bridge's production value; boot A/B (TASK-58) already showed the boot channel is
not addressable — **fresh worldgen is the addressable channel**.

**F3 — worldgen parallelism observation (honest caveat).** 86% of burst-minute runnable CPU ran on a
single Paper Common Worker while the Server thread was parked in chunk wait. Single 64-chunk forceload
commands may serialize per-area dependencies (neighbour-required chain), so this is NOT a definitive
parallelism ceiling — but the observation flags real headroom for multi-area bursts. Follow-up candidate
only, not claimed here.

**F4 — ops guidance: 64-chunk single forceload commands stall the Server thread 10 s+ (Watchdog fired).**
Legitimate worldgen, no crash, world safe — but future load experiments should use ≤16-chunk regions or
staged commands to keep the watchdog quiet and tick pacing measurable.

## 5. Implications unlocked / next candidates

* TASK-57 §2 and G9_WHOLE_METHOD_HOOK_DESIGN §8 wording correction (F1) — docs-only follow-up.
* Noise-native bridge real-load A/B: paired boots with `CRUSSTY_NATIVE_IMPROVED_NOISE=1` + identical
  forceload burst script (F2 gives the hypothesis: ~2 CPU-s/128-chunk delta) — the worldgen-burst channel
  TASK-58 could not test on boot. Requires a scripted burst harness (repeatable seed/coords + clean world
  per run) — candidate TASK-63.
* D6 P3 design remains open; ck_cap verify-strings (S7-13 NEXT-5) remain open.

## 6. Honest limits

* n=120 fillArray samples (small for rare sub-branches; leaf taxonomy within fillArray subtree is coarse).
* Single boot, single seed, one region-pair geometry; `forceload` worldgen equals the player-driven pipeline
  (same chunk system) but ticket priority/spread may differ from a walking player.
* Worker-pool width during bursts not independently varied (single worker observation, F3 caveat).
* Sampling measures CPU-share, not call counts; the 18.5k-calls/tick bar from the g9 doc remains untested
  by sampling — the measured CPU-share ceiling (1.2 s/burst) supersedes it as the decision-grade number.
* Noise-native bridge was dormant (production default) — no interaction effects measured.

## 7. Hygiene

* BENCH.lock acquired (flock holder) and released via holder kill after session.
* Stale `/tmp/crussty_bench.lock` removed before boot (boot-refusal guard); stale orphan `sleep 3600`
  children from previous sessions left alive deliberately (harmless, PPID 1) — session holder killed,
  fifo scrubbed.
* Server stopped as found (graceful fifo stop, launcher clean exit). World restored byte-identical
  (`diff -r` against pre-task tar). Token not exposed; no src changes in this task.
