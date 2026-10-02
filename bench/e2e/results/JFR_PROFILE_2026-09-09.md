# JFR LIVE-BOOT PROFILE — g9 amplification evidence + hotspot discovery

* TASK-57, 2026-09-09, agent-7625532f. First-ever JFR profile of this box (dump→analyze→optimize cycle).
* Profile files: `/home/z/server/logs/jfr/` — `server_31469.jfr` (child, 6.0 MB, full 544 s incl. dumponexit),
  `child_steady_dump.jfr` (5.1 MB, mid-session jcmd dump), `server_31443.jfr` / `launcher_steady_dump.jfr` (launcher JVM).
* Analysis scripts: `/home/z/my-project/scripts/jfr_analyze.py` + `jfr_deep.py` (frame-level aggregation, bucketing, caller-chain queries); raw sample dump `exec_samples_full.txt` (927 samples).
* Server state after session: STOPPED (as found). World untouched (forceload never delivered — I1). Token not exposed. Live proofs not required (dormant env, no env-gated surfaces exercised).

## 1. Method

* Boot: `scripts/e2e_orchestrate.sh boot`, dormant env (no `CRUSSTY_*` overrides), deployed module at HEAD,
  boot `Done (17.075s)` at 01:27:22 UTC; recording window 544 s (01:27:04–01:35:36, covers boot + ~8 min idle-tick + graceful stop).
* JFR injection: `JAVA_TOOL_OPTIONS="-XX:StartFlightRecording=filename=…/server_%p.jfr,dumponexit=true,settings=profile,maxsize=128M"`
  — reaches the launcher-spawned child JVM automatically (documented working recipe; `%p` pid-substitution verified).
  Both `dumponexit=true` at SIGTERM-shutdown AND explicit mid-session `jcmd <pid> JFR.dump` produce valid files
  (belt-and-suspenders; while running, 0-byte `.jfr` files are normal — chunks flush at dump).
* Sampling: `settings=profile` ⇒ `jdk.ExecutionSample` period **10 ms** (self-validated: 927 samples ≈ 9.3 s runnable-Java
  CPU over 544 s ≈ observed avg 14.4 % JVM CPU); `jdk.NativeMethodSample` 20 ms (26 299 — dominated by parked-in-epoll
  netty/idle threads, NOT compute); `minecraft.ServerTickTime` 529 events captured.
* Honest limits: single boot, idle workload (no players, no client bot, console load-injection unavailable — I1),
  n=927 Java samples (small for rare paths), sampling measures CPU-share not call counts, `sampledThread` names
  not attributed by the parser (executor context recovered from stack frames instead). Dormant-env profile =
  production default configuration.

## 2. F1 — g9 `DensityFunctions$Ap2.fillArray` is DEAD BY MEASUREMENT

The g9 whole-method hook design doc (`docs/G9_WHOLE_METHOD_HOOK_DESIGN.md`, S7-13) listed as mandatory blocker
(a) JFR amplification proof ≥ ~18.5k `fillArray` calls/tick. The profile measures the stronger fact:

* **0 of 927 samples contain `fillArray` anywhere in the stack.**
* All `Ap2` traffic flows through the **single-value `compute()`** path: `Ap2.compute` frame-hits 261 + 146 across
  samples; top caller above `Ap2.compute` = `Climate$Sampler.sample` (117 hits) — biome sampling drives the tree.
* The entire noise stack appears in one **5-second boot window** (01:27:21.4–01:27:26.4, 287 samples):
  `ChunkGeneratorStructureState.lambda$generateRingPositions$6` → `BiomeSource.findBiomeHorizontal` →
  `MultiNoiseBiomeSource.getNoiseBiome` → `Climate$ParameterList.findValueIndex` → `Climate$RTree.search` (58 leaf)
  → `DensityFunctions$*` (ShiftedNoise 124+81, PureTransformer 475, Spline 146, ShiftA/B 156) →
  `PerlinNoise.getValue` (279+231) → `ImprovedNoise.noise` (230) / `sampleAndLerp` (193) → `Mth.lerp3` (165 leaf).
  Per-minute buckets: `01:27 → 287`, every later minute → 0. This is structure-ring generation at world load,
  on the ForkJoin worldgen executor.

Consequences: (1) the batch/hook **target method itself never runs** on this server/version/workload — the ≥18.5k/tick
bar is not merely unmet, it is unsatisfiable for `fillArray`; (2) g9 NO-GO is now **measured on both legs**
(scalar-mapping undocumented AND amplification absent — the method is absent); (3) if anyone ever revisits the
Ap2 surface, the live traffic is object-context single-value `compute()` — not expressible by any existing batch
shape (multi-object-ref), and the R-band premium math (B.9 §3) still applies. Verdict: g9 stays closed;
revisit trigger = a profile with real player-driven worldgen showing `fillArray` frames (the doc's §8 open
question is answered in the negative for idle/boot profiles).

## 3. F2 — boot-window noise stack quantifies the native-noise opportunity

The 287-sample / 5 s structure-gen burst ≈ **2.87 CPU-s** (×10 ms) — ~0.57 cores average across the worldgen
executor during the burst — is precisely the stack the existing closed-.so native noise bridge
(`CRUSSTY_NATIVE_IMPROVED_NOISE`, improved_noise.rs) replaces per-call (~2x under contention, PROVEN_WINS).
Boot primary marker (BOOTAB) = 3.10 s ±0.04; a boot A/B (dormant vs noise-native) now has a sample-backed
hypothesis: the Java-side `ImprovedNoise.noise` leaf cost (35 leaf samples + its lerp3/sampleAndLerp callees)
during structure-gen is the removable term. **Candidate TASK-58 (proposal, not claimed here): BOOTAB-style
paired boot with noise-native armed, n≥5, BENCH.lock.** Not run this tick — task boundary kept honest.

## 4. F3 — idle-server main thread: Paper TPS accounting is the top leaf cluster

* Steady-state leaf leaders: `TickData.getTPSAverage` → `ArrayDeque.inc` **142** + `nonNullElementAt` **17**,
  `TickTime.differenceFromLastTick` **23**, plus lerp2/lerp3 residues of the (already accounted) boot burst.
* Arithmetic: 142 × 10 ms = 1.42 s CPU over ~10,880 ticks (544 s @ 20 TPS) ≈ **~0.13 ms/tick main-thread**
  (cluster incl. adjacent frames ≈ 0.17 ms/tick) — every-tick deque iteration for TPS averages, on an idle box.
* Framing: dominant ONLY because the idle main thread has nothing else to run (hosting-density relevance);
  under player load this is noise. Candidate (P3, design-first): moonrise `TickData` window/iteration patch via
  the existing class-retransform machinery (same class as area_map patch) — or config-side tick-window review.
  **Not implemented**; recorded as HOTSPOT_CANDIDATES_V2 addendum D6.

## 5. F4/F5 — tick health and module surface

* `minecraft.ServerTickTime` (529 events): tick durations 0.64–2.24 ms — healthy vs the 50 ms budget; spikes
  coincide with the structure-gen window. No lag introduced by the module.
* CRUSSTY-patched surface cost: area_map patched `update()` — 0 samples (no maps ticked, idle); moonrise
  `ChunkHolderManager.removeTicketAtLevel` 12 samples (ticket churn at boot); module runtime overhead is
  invisible at this sample size (consistent with previous JNI-floor measurements: transition floor 35–90 ns).

## 6. Infra findings

* **I1 — launcher stdin forwarding is broken.** Console commands written to the launcher stdin fifo
  (`forceload`, `tps`, `list`) never reach the server (no log response, no region changes, `enable-rcon=false`).
  Consequence: `e2e_orchestrate.sh shutdown`'s primary graceful path is dead — the working path is the SIGTERM
  fallback (exit 143; JVM shutdown hooks run; `dumponexit=true` JFR dump fires; world save intact). Precedent
  explained: S7-13's "graceful stop 143" was the same fallback, not fifo delivery. Candidate follow-up: debug
  launcher stdin loop or enable rcon (server.properties change = ops decision, needs its own task).
  A comment was added to `scripts/e2e_orchestrate.sh` documenting this.
* **I2 — stale courtesy lock.** `/tmp/crussty_bench.lock` (mtime 00:35Z, zero holder processes — flock had
  auto-released on process exit) blocked `boot`/`all` modes (existence-based guard). Removed before boot;
  recreated nothing — sessions must clean up their lock files after finishing.
* **I3 — JFR recipe validated end-to-end** on this box (JAVA_TOOL_OPTIONS → child, `%p`, explicit dump,
  dumponexit at SIGTERM, 128 MB cap ⇒ 6.0 MB actual). Reuse as-is for follow-up profiles.

## 7. Docs synced by this task

* `docs/G9_WHOLE_METHOD_HOOK_DESIGN.md` — measured-update section: both blockers measured; fillArray absent.
* `docs/HOTSPOT_CANDIDATES_V2.md` — addendum D6 (TPS deque, P3) + boot-noise pointer (F2 → TASK-58 proposal).
* `scripts/e2e_orchestrate.sh` — stdin-broken comment (I1).
* CLAIMS.md + worklogs — TASK-57 closure.
