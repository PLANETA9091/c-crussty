# ORDER TEST — does vanilla Paper 1.21.10 worldgen depend on generation order?

NCF P0.5 (Q1). Executed 2026-10-05, sandbox rig: Purpur 1.21.10-2535,
seed 3053459, 2 vCPU, fresh world per boot, pure vanilla (no CRUSSTY agent —
purity law). Harness: bench/golden (GoldenDumper + ncfdiff).

## Method

1. Boot A: fresh world, dump the 16x16-chunk region (chunk 100..115 x 100..115,
   256 chunks) in INWARD-OUT CLOCKWISE SPIRAL order (PLAN=spiral).
2. Boot B: fresh world (same seed), same region, the SAME plan REVERSED
   (PLAN=spiral_rev) — reversed spiral = maximally different generation order.
3. Compare all 256 pairs semantically: bench/golden/tools/ncfdiff.py --manifest
   (decodes palettes/heightmaps/light before comparing; reports first
   divergent block per section).

CONTROL (determinism of the rig itself): Boot C = same PLAN=spiral, fresh
world, compared against Boot A.

## Results

| comparison | pairs | semantic_equal | diverged |
|---|---:|---:|---:|
| spiral vs spiral2 (control: same order, 2 boots) | 256 | **256** | 0 |
| spiral vs spiral_rev (order flipped) | 256 | 0 | **256** |

Control verdict: the rig is deterministic across boots — same seed + same
order reproduces content exactly (byte diffs are benign fields only:
InhabitedTime etc.). Therefore the spiral-vs-spiral_rev divergence is caused
by the generation ORDER, not by boot nondeterminism.

Divergence anatomy (order-flipped pairs):

- mean 1.6 sections per chunk carry a block divergence (min 0, max 7 of 24);
  most of each chunk is identical.
- **86.2% of first-divergence block positions sit at chunk borders**
  (local x or z in 0..2 / 13..15): 344 border vs 55 inner.
- affected fields (chunks affected, of 256): Heightmaps 802 field-diffs,
  sections 801, block_ticks 248, PostProcessing 143, fluid_ticks 7,
  block_entities 5.
- typical content diffs at borders: ore/stone swaps (deepslate_gold_ore vs
  clay, andesite vs granite), tree-leaf/mushroom differences — consistent
  with cross-chunk feature spill + light/heightmap recompute at different
  neighbor states.

## Answers / decisions

- **Q1: YES — vanilla Paper 1.21.10 worldgen output depends on generation
  order.** The effect is real but localized (border blocks and the fields
  derived from them: heightmaps, light arrays, tick lists, PostProcessing).
- **Canonical order (P0.5 mandate): all golden corpora are generated with
  PLAN=spiral (inward-out clockwise spiral over the 16x16 region, manifest
  order = generation order), single boot per corpus, fresh world per boot.**
  Zero-diff comparisons are only valid between same-order corpora.
- Implication for ladder B (virtual region files): a Rust-generated region
  later surrounded by Java generation will interact at borders exactly like
  the order-flip here. The factory must replicate vanilla's border behavior
  bit-exactly (feature spill into neighbours, light across region seams) —
  the 86%-border finding makes this the central risk of ladder B, not a
  corner case.

## Artifacts

- Raw corpora (NOT in repo, rig-local): /home/z/server/golden/
  vanilla_s3053459_{spiral,spiral_rev,spiral2}/seed_3053459/c_*.nbt
  (256 files each, ~40-60 KB per chunk).
- Plans: results/plans/vanilla_s3053459_spiral{,_rev}.tsv (order = the test).
- Full first-divergence report: results/ORDER_TEST_2026-10-05_pairs.txt
  (rc summary: pairs=256 equal=0 semantic_equal=0 diverged=256).
- Runs ledger: results/golden_runs.tsv.
