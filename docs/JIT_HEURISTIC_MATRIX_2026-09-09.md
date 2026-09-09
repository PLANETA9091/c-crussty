# TASK-120 — OPT_ARCH §6 JIT-heuristic matrix: CLOSED, no size-based action (2026-09-09)

agent-7625532f. §6 (formalized JIT heuristics, GO "zero-risk documentation +
tooling work") closure via bands + hot-census cross-reference. Static,
zero boots. Scanner: bench/jitflags/scan_jit_heuristics.py (derived from
TASK-85's huge_method_scan.py; same minimal classfile-parser discipline).
Reports: bench/jitflags/JIT_HEURISTIC_SCAN_FULL_2026-09-09.txt,
JIT_HEURISTIC_SCAN_WORLDGEN_2026-09-09.txt.

## Published constants encoded (R3#7)

| constant | value | meaning |
|----------|-------|---------|
| HugeMethodLimit | 8000B | C2 refuses larger methods (DontCompileHugeMethods) |
| MaxInlineSize | 35B | cold-callee inline ceiling |
| FreqInlineSize | 325B | hot-callee inline ceiling |
| TypeProfileWidth | 2 | >2 receiver types = megamorphic, no devirtualization |

## Bands scanned (jar: versions/1.21.10/purpur-1.21.10.jar, 9809 classes)

Parser validated: full-jar HUGE=16 exactly reproduces TASK-85's scan.

- **HUGE (>8000): 16 methods** — all datafix/bootstrap/clinit/recipe/loot
  one-time init (BlockStateData.bootstrap*, Blocks.<clinit>, Items.<clinit>,
  ...). Zero on any measured-hot path.
- **NEAR_HUGE (4000-8000]: 20 methods** (new data) — top: EntityType.<clinit>
  7408B, VillagerTrades lambda 7093B, PurpurWorldConfig 6803B/5804B,
  BlockStateData bootstraps, VanillaEntityLoot.generate, ProcessorLists.
  bootstrap. ALL one-time init/config — zero hot-path.
- **WORLDGEN scope (levelgen|chunk), all bands >=2000B: only 3 methods** —
  DesertPyramidPiece/JungleTemplePiece/OceanMonument postProcess (structure
  piece placement, cold per-chunk, not in the noise census). ZERO in
  HUGE/NEAR_HUGE.

## Hot-census size profile (the decisive finding)

Largest method per measured-hot census class (DFC_STATIC_AUDIT + TASK-108
v3 census tokens), floor 300B:

| census class | largest method | size |
|---|---|---|
| synth.ImprovedNoise | sampleWithDerivative | 729B |
| NoiseChunk | <init> | 620B |
| DensityFunctions$TwoArgumentSimpleFunction | create | 485B |
| synth.BlendedNoise | compute | 410B |
| synth.PerlinNoise | <init> | 397B |
| EndIslandDensityFunction | getHeightValue | 361B |
| DensityFunctions$Ap2 | fillArray | 302B |

**Every measured-hot method is <=729B — 5.5x below the near-huge floor,
11x below HugeMethodLimit.** The whole measured-hot worldgen area lives
comfortably inside C2's compile+inline sweet spot (hot-callee inlining
ceiling FreqInlineSize=325B is the relevant boundary for leaf math like
noise getValue/sample — those leaf kernels are 100-400B class, so inlining
behaviour is size-healthy too).

Bonus mechanistic note (feeds TASK-108's closed file): NoiseChunk$
NoiseInterpolator.fillArray does not even reach the 300B floor — the v3
whole-body target is itself a tiny per-point loop wrapper, which further
explains why whole-body bridging could not win (nothing to batch inside a
body this small; the work lives in the delegated subtree).

## Decision matrix (standing protocol)

| band x hot | decision |
|---|---|
| HUGE x hot | NATIVE-SWAP CANDIDATE (TASK-74 law: 11KB barrier -> native swap 5-8x) |
| NEAR_HUGE x hot | INLINE-PRESSURE WATCH (consider inline-barrier pattern) |
| FAT x hot | NO-ACTION |
| HUGE x not-hot | NO-ACTION (one-time init cost) |

**Verdict: zero cells populated.** The size dimension of §6 has NO
unexploited application targets in our profile; the TASK-74 mechanism has
no remaining candidate bodies. The megamorphic dimension (TypeProfileWidth)
is runtime-measurable only — recorded as a scope boundary (agent
instrumentation lane), not attempted statically.

Re-scan trigger: any kernel jar bump (Paper/Purpur update) — rerun
scan_jit_heuristics.py before assuming the old census holds.

## Scope boundaries (honest)

Static scan cannot measure receiver counts or callsite hotness — the
megamorphic half of §6 needs runtime TypeProfile data (agent
instrumentation; separate lane). Method-level Code.length only; no
bytecode disassembly. The 259-class launcher jar
(versions/purpur-1.21.10.jar) is not the server code; the real jar is
versions/1.21.10/purpur-1.21.10.jar (TASK-85 path confirmed).
