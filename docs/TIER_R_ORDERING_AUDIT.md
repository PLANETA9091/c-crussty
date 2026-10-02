# Tier-R Ordering Audit — parallel-weaving feasibility of the mojang construction island (TASK-98, S7-41)

Method: static bytecode audit (javap-fidelity) on the RUNNING runtime jar
`/home/z/server/versions/1.21.10/purpur-1.21.10.jar` (mojang-mapped, `net/minecraft/**`
at root), two parallel scans (B1+B2 / B3+B4), artifacts `/tmp/tierR_audit_{A,B}/`.
No boots, no server interaction. Boot call-chain anchor: `net.minecraft.server.Main.main` —
offset 3 `SharedConstants.tryDetectVersion()` → 93 `Bootstrap.bootStrap()` (B2) → 96
`validate()` → **623 `LevelStorageSource.createDefault(...DataFixers.getDataFixer())`
(B1 fires here, on main)** → 849 pack repo / `WorldStem.load` → B3 registry load →
B4 resource reload.

## Verdict table

| Block | What | State measured | Verdict | Ceiling (2-core Amdahl, census-anchored) |
|---|---|---|---|---|
| B1 | DFU DataFixer build (`DataFixers.<clinit>`: 279 schemas, 405 fixers, 332 eagerly-`new`-ed fix objects) | pure function of `SharedConstants.getCurrentVersion().dataVersion().version()` (int); constant-pool scan of ALL 406 datafix classes: **zero** refs to BuiltInRegistries/Bootstrap/Blocks/Items/java.io/java.nio | **OFFLOADABLE-EARLY** — premain can trigger `<clinit>` on a worker after version detect; main joins completed `DATA_FIXER` at Main:623 | ~0.9-1.0s (DFU 7.2% census share) + hides ~700-class DFU load under B2 |
| B2 | vanilla registry bootstrap (`BuiltInRegistries.<clinit>` 90 empty registries → `Bootstrap.bootStrap` fill via content-class `<clinit>` DFS + `LOADERS.forEach` tail → freeze) | measured cross-registry couplings: `Items.<clinit>`→`Blocks` **1070 getstatic** refs + `builtInRegistryHolder().key()`; `BlockEntityType`→Blocks ×201; `VillagerProfession`→PoiTypes+Items+Blocks; `PoiTypes`→Blocks ×46; `Item$Properties`/`ToolMaterial`→ENTITY_TYPE/BLOCK HolderGetter lookups DURING fill; Paper `injectFluidRegister` FLUID↔blockstates; clinit-monitor re-entry (parallel clinit = deadlock, not speedup) | **SERIAL-FORCED (design-NO-GO for per-registry threads)** — matches pre-registered suspicion; only sound variant = phased restructure (declare-all→resolve), out of scope | ~0 (addressable only by unsafe restructure) |
| B3 | datapack/worldgen registries (`net.minecraft.resources.RegistryDataLoader` — NOT core/, 1.21.10 layout) | `load()` = `List.forEach` → `loadContentsFromManager` = plain while-loop over JSON files, **0 hits** CompletableFuture/Executor/parallelStream; `WorldLoader.load` passes backgroundExecutor but does not use it until line 203 — B3 runs 100% main-thread | **SERIAL, main-thread; per-ELEMENT (per JSON file) parallelizable** (read+Gson+Codec.decode independent); per-REGISTRY needs topological order (biome↔noise/feature/dimension chains via shared RegistryInfoLookup) — parse-fan-out/ordered-apply = the real R2 impl line | unknown until parse/apply split measured (targeted JFR probe = NEXT); this is the +5s window core |
| B4 | recipes (1461) / advancements (1574) / loot | vanilla prepare-async/apply-sync fully intact (`SimplePreparableReloadListener`: prepare on `CompletableFuture.supplyAsync`, apply on main; `RecipeMap` built in prepare; apply = putfield+log) — **ALREADY-PARALLEL, but starved**: `Util.maxAllowedExecutorThreads()` = cores/2 = **1** on this box → background pool parallelism 1 | **ALREADY-PARALLEL (starved)**; zero-code probe = `-DPaper.WorkerThreadCount=N` sysprop | limited by 2 physical cores; batched-micro arm candidate |

## GO-gate resolution (from claim: ≥1.5s addressable AND ordering-proof plan)

- **B1 = GO-worthy**: mechanism fully proven static (purity scan + existing threaded
  `DataFixers.optimize` hook shape Mojang ships uncalled); delivery vehicle = the
  TASK-97 v2 prewarm agent (loader-capture, dormant-by-default) with trigger on first
  `net.minecraft` load + poll-until-version + `Class.forName("...DataFixers", true, cl)`.
  Honest ceiling ~1.0s; risk: `Util.<clinit>` thread-pool side-effect (happens at boot
  anyway), tryDetectVersion race (poll from worker, main does its own call — idempotent
  static).
- **B2 = design-NO-GO closed** (measured, not guessed — 8 concrete coupling points).
- **B3 = measure-first**: impl only after a JFR probe splits parse vs apply inside the
  +5s window; Paper registry-event layer (`Conversions`, `LoadingFunction`,
  `PaperRegistryAccess`) threads through every element load — any fan-out must respect it.
- **B4 = zero-code probe** (`-DPaper.WorkerThreadCount`), only as combined-batch arm
  (owner rule: many-at-once).

Boot-program state after this audit: classloading CLOSED (TASK-95), generic-prewarm
DEAD (TASK-97), B2 serial-forced (this audit) — the honest remaining boot levers are
exactly B1 (agent-prewarm GO) and B3-parse (measure→maybe-impl), plus batched micro.
Combined realistic ceiling ≈ 1.5-2.5s of the ~12.5s idle boot; <1s cold boot remains
blocked on snapshot (env-NO-GO ×2) — unchanged.
