# R3: DFU-Schema Cache — Design Line (S7-36)

Status: DESIGN (no implementation attempted this session)
Evidence rank: the largest UNADRESSED Java island in the boot window after
CDS v3 (classloading) is DFU schema-join work.

## 1. Evidence chain (measured, two independent censuses)

| Source | Instrument | Finding |
|---|---|---|
| S7-33 1ms JFR boot census | jdk.ExecutionSample, 2793 in-window | DFU joins ~207 samples = 7.4% of boot-window CPU |
| S7-35 W1 native/Java attribution | analyze_boot_windows.py, R1 gate | DFU = 11% of the 5s registry window (one of 4 islands) |
| S7-32 10ms census | first JFR pass | "+3.0s = DFU Schema joins + 1461 recipes + 1574 advancements" |

Sampler-starvation caveat (S7-35): 2-core JFR undercounts absolute CPU
~1.4-1.5x; relative composition is valid. DFU share is therefore
7-11% of window CPU, i.e. ~0.5-0.9s of the 13.35s default boot.

## 2. Mechanism anatomy

DataFixerUpper (com.mojang.datafixers) validation during boot:

- `DataFixers` static init builds the full fixer stack (~500 schema
  versions V1..V5052, each a class whose static init constructs rules).
- During registry/datapack load every typed element passes
  `TypeRewriteRule` joins; the first use of each (schema, dataType) pair
  compiles an optimized view (`OpticZipper`/`RewriteResult` caches).
- All of this is per-JVM state: every cold boot rebuilds it from zero.
- Vanilla has no persistence hook for the compiled state.

Key structural facts (bytecode/static analysis, running jar 1.21.10):

- The compiled views are held in `DataFixerUpper`'s concurrent hash maps
  keyed by `Type` instances whose equality is structural, NOT stable
  across JVM runs (identity-heavy intern pools, lambda `key()`s).
- Many keys/values are `invokedynamic` lambda instances ⇒ any
  serialization-based persistence hits the **lambda-relinker problem**
  (same blocker already recorded for registry-persistence in
  BOOT_SUBSECOND_FEASIBILITY ADDENDUM-2).

## 3. Options ledger (honest ceilings)

| Option | Mechanism | Est. ceiling | Risk | Verdict |
|---|---|---|---|---|
| R3-a serialize compiled DFU graph | capture + relink lambdas at boot | ~0.3-0.5s | lambda relinker = new engine subsystem; graph stability across Paper updates unproven | design-line only |
| R3-b skip-join short-circuit when world DV == jar DV | agent intercepts DFU entry, precomputed identity rewrite | ~0.2-0.4s | SEMANTIC: bypassing validation changes failure behavior on corrupt data; violates "no gameplay/behavior change" spirit unless gated to byte-identical result proof | gated design-line |
| R3-c warm-up parallelization (schema init on side threads at VMInit) | force `DataFixers.<clinit>` + top-level schema `<clinit>`s concurrently off-main | ~0.2-0.3s on 2 cores | class-init ordering: schema classes form a dependency DAG; JVM class-init locks handle cross-deps but a wrong partition risks init-deadlock; needs island analysis first | candidate for the R2 prewarm framework if R2 ships |
| R3-d upstream | Paper/DFU change | n/a | out of our lane | n/a |

## 4. Pre-registered gates (if R3 is ever attempted)

1. Census-first: dedicated 1ms JFR arm with `jdk.ExecutionSample` +
   `-Xlog:class+init=info` proving ≥0.4s of DFU-join CPU remains AFTER
   CDS v3 banked (v3 may already shrink DFU's classloading share but
   NOT the join compute — the compute is object construction).
2. Amdahl: net win must exceed 0.3s mean over 5+5 boots, p<0.05,
   hs_err 4/0 preserved, zero behavioral deltas (recipe/advancement
   counts, world checksum parity on a fixed seed).
3. One variable per arm; BENCH-MUTEX; anchor-restore per boot.

## 5. Ranking after CDS v3 (S7-36 state)

1. ~~CDS v3~~ → measured this session (see BOOT_CENSUS doc addendum).
2. collision-join island (18% W1) — runtime-surface first (TASK-84 probe).
3. **R3 DFU** — this doc; option R3-c is the only low-risk line, and it
   depends on the R2 prewarm framework existing.
4. R2 serve-at-load (V2 design line, honest ceiling ~0.2s boot).

Conclusion: R3 stays a design line. The cheapest REAL next lever for the
remaining Java islands is the R2-style prewarm framework (R3-c reuses it),
and collision-join attribution via the already-built dirty-census probe.
