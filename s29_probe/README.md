# s29_probe — aquifer candidate-selection bit-exactness probe (R468-S29)

C2ME #558 top-k equivalence methodology (their 204,096 cases) applied to the
LOCAL ground truth: `javap` of `Aquifer$NoiseBasedAquifer` in
/home/z/tools/patched-kernel.jar (Purpur 1.21.10, 2025-12-11 build).

Findings encoded in the probe (see source header):
- 12-cell scan: offX {0,1} x offY {-1,0,1} x offZ {0,1}; gridX=(x-5)>>4,
  gridY=floorDiv(y+1,12), gridZ=(z-5)>>4
- TOP-4 mutually-exclusive `>=` cascade (bytecode 387..502) — NOT top-3:
  the C2ME #558 diff targets C2ME's own top-3 impl on ver/1.21.1
- per-NoiseChunk FULL-GRID caches already exist in vanilla 1.21.10:
  aquiferCache[FluidStatus] + aquiferLocationCache[long] (bytecode 207..231,
  240..337) — the 16-slot ring-buffer thrash of 1.19.x is gone
- barrier noise = aquifer_barrier 1 octave @ firstOctave -3 (NormalNoise,
  2 Perlin evals/sample), memoized per block via MutableDouble NaN sentinel
  (bytecode 266..312); `new MutableDouble` remains per pressure-path block

Variants: V0 = vanilla per-block scan; V1 = C2ME #558 mechanics (candidates
cached per grid cell, hot top-2, deep top-4 replay from stored distances).

Result (PROBE_LOG.txt): 3 seeds x 4,259,840 blocks = 12,779,520 cases,
0 mismatches, laneA(top4) and laneB(top2) checksums bit-identical per seed;
BlockPos unpacks -91.54%, location-array reads -91.54% (per-block model).
