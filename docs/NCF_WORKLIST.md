# NCF Worklist — owner standing order, 2026-10-09 (replaces Job 441690 payloads)

Status: `[ ]` open | `[~]` in progress | `[x]` done (same-commit LOG evidence
only) | `[!]` blocked (one `BLOCKED: <reason>` line directly under the item).
Take the FIRST open item, top-down; one step per tick: probe, fix, measure,
or commit. After each finished item: run the ncf_profile build, list the top-3
exclusive-time functions, append up to 3 new items at the bottom (HYP + probe
+ target), ordered by expected ms saved / effort. The list must never be empty.
Tick ends with no commit and no measured number = failure. No zero-action;
never wait for the owner (OWNER-Q lines go to the worklog).

---

## S1 [SURFACE, lazy biome] [x]
surface_rules.rs update_y (line ~754) calls get_biome_voted_region
(biomes.rs:340, returns String, 8-corner vote) for EVERY solid block in every
column. ctx.biome is read only at lines ~857 (Cond::BiomeIs) and ~905
(Cond::Temperature). Java memoizes lazily (worklog ~line 1193 already states
value-identical). HYP: this dominates the 21.0 ms surface stage.
Probe first: count get_biome_voted_region calls per chunk (expect tens of
thousands vs 256 needed). Fix: store the pending (x,y,z) in update_y, compute
on first read, cache until the next update_y.
Target: surface <= 12 ms after S1 alone.
DONE (2026-10-09): probe b986eecd = 30,274 votes/chunk. Fix: biome_pending +
ensure_biome() (memoize-per-updateY, Java-null mirroring pre-first-updateY),
&str read path (Temperature clone killed). Surface 21.19/21.08 -> 8.71/8.54
(2.4x; target <=12 beaten), ported 42.47 -> 30.36, Amdahl 3.31 -> 3.72x,
probe 30,274 -> 4,822 votes/chunk (remaining = Java-faithful memoize reads).
IDENTITY: stagediff --gen-batch 256 chunks surface+carvers vs parent
b986eecd BYTE-IDENTICAL (diff -rq); 194/194 cargo test.

## S2 [SURFACE, block class table] [x]
is_air_id/is_fluid_id/is_stone_id (surface_rules.rs ~1081) compare block-name
strings per block, several times per step, plus HeightmapKind::is_opaque_state
in set_block. Fix attempted (intern-time flags in StateTable): measured only
0.6-1.3% surface gain (8.51/8.53 -> 8.46/8.42) — REVERTED per R2 (gain < 5%),
numbers recorded in NCF_SPEED.md history. PREMISE FALSIFIED: 188k classify
calls/chunk but the String work totals ~0.05-0.1 ms (interned cache-hot
names, few-cycle memcmp rejects). Probe (NCF_S2_PROBE): rule.apply is ~78%
of the stage — the surface lever moved to S4.
DONE-closed (2026-10-09, falsified-and-reverted; target <= 8 ms abandoned —
not reachable via classify flags; evidence: NCF_SPEED.md history row + S2
probe line, same commit).

## S3 [SURFACE, no String] [x]
DONE (2026-10-10, commit 837905d): u16 biome registry — String-free resolve +
integer BiomeIs/probe compares. Surface 2.46 -> 2.17 ms/chunk; stagediff
surface 64/64 + carvers 16/16 EQUAL; 217 tests. Evidence: commit message +
NCF_SPEED.md history row. S1-S4 re-profile: surface has no >= 5% lever left
(R3-S4-RESEARCH 2026-10-10): remaining set_block/y-loop classify id-flags
measure 2-5% = R2-revert territory; per-column probe vote 0.35 ms is
Java-faithful BiomeSource cache — untouchable.

## N1 [NOISE 17.23 ms -> next] [x]
DECOMPOSED (2026-10-10, NCF_N1_PROBE 3-counter probe, commit ed9a255 + this):
profile-build COARSE clocks scaled to clean 17.23: slice fills ~9.9 ms (58%),
per-block drive callback ~3.2 (19%), substance final_density fill 3.11 raw /
~2.1 clean (12%), climate+biomes+rest ~1.9 (11%). Slice leaf calls 13,550/chunk
100% YDEP (yfree=0: flag-2 leaves never reach compute_body — last-slot front
memo intercepts). L1/L2/L3 DEAD (R3-N1B research: wrap_memo+intern already
dedup shared subtrees -> 0 ms; all 6 Java markers handled -> 0 ms; vein
toggle/ridged are O(1) interp reads + gap lazy ~1-2k evals -> <=0.2 ms).
L4 SoA y-corner batching: gate PASSES (slices >= 2.5 ms, ydep 100%) but
ceiling 0.8-0.9 ms = AT the R2 5% bar — only if slice-fill noise kernel share
>= 2.4 ms holds. The per-y-dispatch lever (arms for MulOrAdd/Clamp/
BlendDensity, flag-2-guarded) was MEASURED 2026-10-10 tick-2318 and
FALSIFIED: A/B byte-identical (surface 256/256 + carvers 16/16, NCF_TILE
_CACHE 0/1, 4/4) but parent noise 16.82/16.82 (PORTED 20.61/20.59) vs
arms 16.39/16.73 (20.25/20.53) = Δ 0.09-0.43 ms of 16.82 (0.5-2.6%) —
BELOW the 0.86 ms R2 bar -> REVERTED per R2, no code commit (attempt
diff archived: wg-build/n1-arms-attempt.diff). N1-R probe attribution:
after the committed Mapped/RangeChoice/CacheOnce/Cache2D/Interp arms,
the remaining pfd pool in slice fills = YClampedGradient ONLY (203
calls, 9,973 elems, 0.095 ms/chunk) and cell fills = 0 — the heavy
subtrees sit behind cache wrappers whose fills are per-element scalar
compute BY DESIGN, so the 1-3 ms estimate conflated fill_array visits
(147k) with harvestable per-y pfd time (~0.1-0.4 ms). Linear-arm lever
CLOSED.
Hypotheses (a)/(b) ANSWERED 2026-10-10 (see R3-N1B + DECOMPOSED above):
(a) no duplicate copies exist (wrap_memo+intern dedup; markers complete);
(b) no missing markers — Java parity is complete. (c) gather forbidden;
octave-SoA weak (grad_dot table). Live levers AFTER the arm falsification:
NP1 noise-parallel (below — biggest), SUB1 substance/interp kernel share,
L4 y-corner SoA (borderline). Anything skipping work must be bit-exact;
gate-p2 decides.

## H1 [HYBRID MEASUREMENT] [!]
BLOCKED: no Java hybrid E2E rig on this sandbox (JAVA_WARM_* are model
constants; the mod harness + fixed 1024-chunk Java region bench do not
exist here). Unblocks when a rig with the Java side is available.
The 3.32x figure is a model (unported stages at 1x, incl. jvm_other 48.9 ms).
Measure the real hybrid: same machine, fixed 1024-chunk region (the E2E region
bench exists), pure Java vs native-then-Java-completion, world restored
byte-identically (I10), report chunks/s and CPU-ms/chunk (JFR). Question to
answer: does jvm_other shrink when native stages replace Java work? If the
hybrid hook is not wired, wire it default-off with the I8 fallback. Label
results with core count (2 vCPU rig).

## C1 [CORRECTNESS AT SCALE, always available] [~]
Checklist line "0 diffs on >= 10^4 chunks x >= 5 seeds x {vanilla, Terralith,
Tectonic}" is still [~]. Add seeds and chunk counts to CI until it is [x]
with evidence.

## F1 [FEATURES, demoted] [ ]
Features are 4.8 of 363 ms Java (1.3%). Do one feature increment only after
every 3 speed commits, and prefer increments that unlock the most biomes in
the census. Queue (addendum 93 §6, inc.15 block_column already landed in
9ca0ad6): weighted_state_provider as to_place in compact simple_block (39
biomes) -> fancy_trunk_placer (7) -> glowstone_blob (5) / vines (4) ->
stagediff features-status on the special corpus (3x3-replay spiral
GoldenDumper).

## S4 [SURFACE, rule.apply internals] [x]
DONE (2026-10-10, commit 3cb07cb): hit path -> u32 slot ids + per-node hit
memo + FxHash StateTable — kills per-hit parse/format/SipHash. Probe baseline
(NCF_S2B): try 5.54, intern 4.65, set 0.61 ms/chunk, intern_calls 16,974 ->
after: intern 0.005, intern_calls 4/chunk. Surface 8.5 -> 2.46 ms (S4) then
2.17 (S3, 837905d). Target <= 6.5 beaten 2.9x. Evidence: commit messages +
NCF_SPEED.md rows + R3-S4-RESEARCH re-measure at HEAD 5ddfbbf (surface
2.23/2.29/2.24 x3, stagediff A/B vs 2bfd0ec: vanilla surface 64/64,
carvers 16/16, seed 90210 64/64, Terralith 36/36 BYTE-IDENTICAL).

## NP1 [NOISE-PARALLEL, slice rows + cell fills, std::thread::scope] [~]
HYP: the 15 roots x 5 rows slice fills and the per-cell cache fills are
mutually independent units — RNG is position-derived (xoroshiro at(x,y,z),
no cross-unit draw sequence), so evaluating units in parallel keeps the
per-element op order bit-exactly (I2-safe by construction; crate is
std-only, use std::thread::scope, precedent region.rs write_region_parallel
~:433). Slice fills ~9.9 ms COARSE of noise 16.5-17.2 (58-60%); the 2-vCPU
rig caps the wall win at ~2x on the parallel share -> expect noise ~10-12.
Probe FIRST (R5): scope-parallel row fills behind an env flag (default off),
stagediff byte-identical A/B (NCF flags 0/1) + ledger x2; verify no TLS
state crosses the boundary (N1 counter TLS markers, debug EntryGuard) and
note 2-vCPU contention in the ledger labels. Target: noise <= 12 ms/chunk
(R2 bar 0.86 ms of noise stage). Effort: medium. NEXT TICK.

PHASE-1 LANDED (2026-10-11, commit 2cd78b3): fill_slice row loop extracted
verbatim into fill_slice_rows; parallel arm behind NCF_PAR_FILL (OnceLock,
exactly "1") forks ONE clone worker on the upper row half via
std::thread::scope (precedent region.rs), copy-back = worker slice rows
memcpy + counter merge + final scalars + epoch-shifted CacheOnce copy-back;
tile.rs RefCell->Mutex + AtomicU64 (disabled path still pre-lock); gate
cell_count_xz>=2 excludes height_feed. I2: zero float-op change; memos are
position/epoch-keyed pure memos -> replicas value-transparent. EVIDENCE:
stagediff A/B surface 256 + carvers 16 x NCF_TILE_CACHE 0/1 = 4/4
BYTE-IDENTICAL + parent-parity empty; 227/0 tests (+T1 bit-equal, +T2
counters/CacheOnce equal); paired ledger seed 3053459 256 chunks: OFF noise
17.06 PORTED 20.94 vs ON noise 14.22/14.33 (0.8% spread) PORTED
18.05/18.20 = noise-stage gain 2.7-2.8 ms >= 0.86 bar KEEP. Flag default
OFF = zero regression when unset. 2-vCPU rig, contention noted. Target
noise <= 12 NOT yet reached (14.2-14.3) -> phase-2 refinement = NP2 below.

PROBE FALSIFIED (2026-10-11, commit 55929de N2 clocks, cfg ncf_profile):
MEASURED (contended rig): fill_calls 5/chunk; fixed clone+spawn/join-merge =
0.168 ms/chunk (tiny); imbalance_loss 1.344 ms, ratio |m-w|/(m+w) 0.117;
interp[0] = 87.8% of unit mass, ic-delta 49-98 per unit (data-dependent,
merge stays delta-sum); unit clocks cross-validate rows clocks (21.453 vs
21.466 ms/chunk). CONTENTION-FREE ARITHMETIC (phase-1 paired deltas
OFF-ON = 2.33-2.84 = 0.3988*M - 0.14): M = 6.5-7.5 ms/chunk -> realizable
gains: C adaptive contiguous split (best unit cut 57.7% vs 60.1% wall
factor) = 0.15-0.25 ms; B heavy-first dynamic cursor (52.7% wall factor) =
0.48-0.55 ms; A persistent pool adds only ~0.17 ms. ALL < R2 bar 0.70 ms
(5% of noise 14.02) -> do NOT implement. CacheOnce merge under
non-contiguous ownership (B) would be the highest-complexity code in the
crate for a sub-bar win. Successor with real headroom: NP3 overlap
pipeline (parent-only walk window ~60% of drive wall).

DONE (2026-10-11, commit 1ced27b): NCF_PAR_FILL=2 pipelined drive — worker
fills col k+2 (pipeline_worker_fill: slice1/slice2 handle swap reuses
fill_slice_rows verbatim) while parent walks col k; 3-buffer rotate
(slice2 + rotate_slices) since walk reads 100% of both live buffers; merge
= phase-1 shape (ic/aic delta-sum F+W+G serial-exact, fill-end scalars incl
cell_start_block_x/in_cell_x, epoch-shifted CacheOnce, slice2 handle
handoff); last iteration serial swap_slices. EVIDENCE: stagediff 5/5 diff
-rq EMPTY (surface 256 + carvers 16 x TC 0/1 mode0-vs-mode2 + parent-parity
+ mode1-vs-mode2); 230/0 tests (+T4 bit-identical 2 origins, +T5
counters/CacheOnce, +T6 degenerate cc=1); ledger 256ch: mode1 14.22 vs
mode2 11.93/11.47 noise, PORTED 15.36-15.81, paired gain +2.3-2.8 >= 0.70
bar KEEP; target noise <= 12 REACHED. N3 probe: walk 6.686 worker 9.132
join_block 2.695 -> exposed residual 2.519 ms/chunk -> NP4 assist queued.

## SUB1 [SUBSTANCE+INTERP kernel share] [x]
PIPELINE NOTE (2026-10-11): under mode2 the noise wall is worker-bound
(F > W on quiet rig), so walk-side substance cuts have ~0 wall leverage —
SUB1 serves the SERIAL default path (mode0, substance ~2.1 of 16.68) and
re-enters the pipeline ladder only after NP5 lands or when F <= W. Order:
run AFTER NP5.
HYP: substance final_density fill 3.16 ms COARSE / ~2.1 clean (98,304
elems, 3,840 visits/chunk) is dominated by per-element 8-corner
interpolation arithmetic + cache reads (dispatch already array-wise); a
lane-parallel SoA form across the 128-elem cell (lane = independent
element, scalar loads + permutation, NO gather — P2.13 constraint) is
I2-safe (same ops per element). Probe: ncf_profile exclusive-time split
(interp-lerp arithmetic vs noise evals vs cache read/write) inside the
substance fill; gate the SoA rework ONLY if the arithmetic share >= 1.2 ms
(5% of noise stage). Target: substance <= 1.5 ms clean. Effort: high —
run only after NP1 lands or is falsified.
PROBE LANDED (2026-10-11, this commit: SUB1 probe clocks cfg ncf_profile
+ NCF_SUB1_PROBE gate — 10 scope statics tiling the N1_FILL_NANOS envelope
pairwise-disjointly (N2 unit-clock precedent) + 2 expect-0 counters;
sub1-research agent mapped the tree FIRST per owner order: vanilla fd top
= Add(fd,Beardifier) -> Min -> Mapped(squeeze) -> MulOrAdd(0.64, interp0),
so the interp0 trilerp lives in the `_`-arm fillAllDirectly of MulOrAdd,
NOT the Interp fill_array arm; Beardifier pfd = zero-lerp dispatch-floor
control, mulora-beard per element = the harvestable trilerp share).
EVIDENCE (64ch mode0 x2, profile build): E = 3.338/3.260 probe-scale
(2.19/2.14 clean via s = 16.71/25.45 = 0.657 — matches the ed9a255
3.16-COARSE/~2.1-clean precedent); squeeze 0.149/0.145, add 0.034/0.034,
min_loop 1.314/1.280, mulora_pfd 1.082/1.049, beard_pfd 0.564/0.555
(5.7/5.6 ns/elem), interp_pfd 0, pfd_other 0, mul/max/rc 0 — vanilla
shape EXACTLY as researched; noise_leaves = 0 AND cachewraps = 0
(premises verified: no ImprovedNoise, no cache wrapper inside the fill);
cross-check sum(scopes) 3.142/3.063 vs E (residual 5.9/6.0% =
dispatch+scratch+probe). VERDICT: ARITH_HIGH 2.578/2.508 -> 1.69/1.65
clean; ARITH_LOW 1.451/1.399 -> 0.95/0.92 clean. BAND vs the written 1.2
gate (HIGH passes, LOW fails) -> per the pre-committed template decide on
GATE_HIGH -> GO on the full-chain SoA track. Honest notes: (a) the written
1.2 gate = 5% of PRE-NP noise ~24; the live R2 5% bar for mode0 = 0.834
and even conservative LOW 0.92-0.95 exceeds it; (b) LOW subtracts the FULL
beard floor from both mixed scopes — right for a kernel-only partial
rewrite (n1-arms precedent, sub-bar), but a FULL-CHAIN SoA rewrite of the
fd top also removes the per-element dispatch itself (beard 0.37 + residual
0.13 clean + in-scope dispatch), so the realistic harvest window is
~1.0-1.7 clean = 6-10% of mode0 noise = 2-3x the R2 bar. NEXT (one step
per tick): full-chain SoA design research FIRST (vanilla-shape detection
at wrap time; batched kernels: corners-shared trilerp + rc lane-select +
noodle lane-select + squeeze/min/add; I2 = identical per-element op
sequence, no gather), then worktree prototype, stagediff 4/4 + paired
mode0 ledger x2; land only if >= 5% of stage. mode2 leverage stays ~0
while F > W — serves mode0 (default) only.

LANDED (2026-10-11, this commit): full-chain SoA via NCF_SUB1_SOA=1 (default
OFF), research-first (sub1soa-research agent design doc -> orchestrator
implementation in worktree sub1-soa). Two research corrections: (a) the
substance fill runs on the WALK side in EVERY mode (select_cell_yz is called
by drive_blocks AND drive_blocks_pipelined) — one branch serves mode0/1/2;
(b) the Min loop compares the squeezed fd array against a per-element freshly
computed NOODLE value (RangeChoice(Interp(i_main), -1e6, 0, Const(64),
Add(Interp(i_thick), MulOrAdd(1.5, Max(Mapped(Abs, I_ra), Mapped(Abs,
I_rb)))))), not two array halves. Detector at SimTemplate::build (once per
RandomState): structural match + constants read from nodes + NO flag-2 node
in the matched region (first draft required flags==0 — WRONG: the in_range
Const is flag-1 in vanilla; t7 caught it, fixed to flags!=2 — flag 0/1 = no
memo interaction in compute()). Kernels: per-cell passes in visit order —
hoisted z-plane lerp2 tables (manual LICM over pure loads: corners shared
per cell, fracs from the fixed tables; 4x fewer lerps, same bits), squeeze
via mapped_transform (all 7 arms), noodle chain straight-line (a1-before-a2,
Abs inlined, java_max bound shortcircuit, NO zero-check on the folded
MulOrAdd, java_min at the Min loop), Beardifier pfd its own pass, root add
UNCONDITIONAL (-0.0 quirk), end machine state (in_cell_*/array_index) set
explicitly. EVIDENCE: stagediff A/B 5/5 EMPTY (surface 256 x NCF_TILE_CACHE
0/1 + carvers 16 x 0/1 + mode2 surface); t7 bit-identical (drive values +
slice0/1 + substance_cache bits + ic/aic + csby/icy/csbz/icz/array_index, 2
origins) + t8 detector negatives (nether top, non-Const in_range, flag-2) +
flag-1 positive, 229/0 BOTH builds (+2 tests); probe honesty: soa_cells =
768/chunk, all SUB_* scopes 0.000, envelope E 3.338/3.260 -> 0.862
probe-scale = ~2.14 -> ~0.57 clean (substance target <= 1.5 BEATEN); paired
ledger mode0 OFF 16.96/17.05 (0.5%) -> ON 14.78/14.85 (0.5%) = gain
2.18-2.20 ms = 12.7-12.9% of the stage = 2.6x the live R2 bar 0.834 -> KEEP;
PORTED 20.90/20.67 -> 18.49/18.58. SURPRISE: mode2 OFF 11.97/11.68 -> ON
10.23/10.14 = gain 1.54-1.74 — the "walk-side leverage ~0 while F > W"
premise is FALSIFIED at the current regime (mode2 wall = walk + blocked
join: cutting walk-side work also cuts the join block and gives the contended
worker more core time); N3 probe SOA=1: walk 5.367 (was 7.279), join_block
4.174 (was 2.948), residual 4.002, worker 9.321 (was 9.897) — the worker is
no longer the binding side; NEW mode2 RECORD 10.14-10.23, the NP5-era
"noise <= 10.5" target REACHED by SUB1. PORTED 15.89/15.39 -> 14.13/13.91.
NEXT: re-rank the ladder at the new regime (worker fill kernel F 9.3 vs walk
5.4 probe-scale — the pipeline window moved; NP5-style worker-side levers
regain priority), then per-section serialization / carvers rework toward the
1 ms mandate.

## NP2 [NOISE-PARALLEL PHASE 2, persistent workers + dynamic partition] [x]
HYP: phase-1 NP1 leaves 4 of 5 spawn pairs per chunk and a fixed 3/2 heavy-row
split (interp[0] rows dominate): a persistent worker pool per drive_blocks
(scope once per drive, generation-gated) with heavy-first dynamic partition
(AtomicUsize fetch_add over the 40 (row,interp) units) should cut spawn/join
~0.1-0.4 ms and balance loss ~0.5-0.8 ms -> noise 14.2 -> ~12.5-13.3; adding
overlap of next-column fill_slice with current-column substance+callback
(software pipeline at swap_slices boundaries) targets the substance share
(~2.1 ms) for the final push to <= 12. Probe FIRST (R5): ncf_profile clock on
spawn/join total per chunk + per-unit exclusive times (balance ratio); fix
only if overhead+imbalance >= 0.86 ms combined. I2: same unit independence
proof as NP1 (position-pure op sequences); byte-identity gate = stagediff 4/4
A/B + ledger x2 paired. Target: noise <= 12.5 ms/chunk. Effort: medium.

## NP3 [SUBSTANCE+CALLBACK OVERLAP PIPELINE, worker pre-fills next column] [x]
HYP: during select_cell_yz + per-block callback the NP1 worker is idle
(phase-1 joins before the walk); the parent-only window per column =
substance (~2.1 ms/chunk) + callback (~3.2 COARSE) is ~60% of drive wall.
Pipelined worker (ping-pong slice buffers + index remap in
select_cell_yz/swap_slices) pre-fills column k+1 while parent walks column
k: hides up to min(next-fill wall, walk wall) = 2-4 ms/chunk — 4-7x the
NP2 realizable win. Value-safety: stale-clone speculative pre-fill is
value-safe (fork epochs < future epochs, no false hits; misses recompute
identical bits); buffers + measured counter delta handed back, parent
skips own fill; fill_slice becomes copy-back+merge consumer; non-pipelined
serial fallback under NCF_PAR_FILL=0 unchanged. I2: same unit independence
proof (position-pure op sequences; N2 probe shows ic-deltas data-dependent
-> delta-sum merges only). Probe FIRST (R5): ncf_profile worker-idle
ms/column + walk-wall ms/column (N2_* clocks extended); fix only if
hideable window >= 0.70 ms (5% of noise 14.02). Target: noise <= 12
ms/chunk. Effort: high.

## NP4 [PIPELINE PARENT-ASSIST, recover exposed fill residual] [x]
HYP: N3 probe shows worker fill (9.132 ms/chunk) > walk (6.686): parent
idles 2.519 ms/chunk at h.join() (exact per-scope residual). Parent-assist:
before walking column k the parent runs p prefix rows of the hidden fill
(slice2 — safe, walk never reads slice2), worker takes the suffix; adaptive
p via EWMA unit costs (N2_UNIT_NANOS infra exists). Wall =
max(W + pF, (1-p)F); optimal p ~ (F-W)/2F -> recovers up to ~1.2-2.5
ms/chunk. NP3 pre-enabled the structure (pipeline_worker_fill extendable
to a rows range; merge is phase-1-proven delta-sum). Probe FIRST (R5): N3
residual clock already exact (2.519 on 64-ch probe); re-measure paired on
quiet rig; fix only if residual >= 0.70 ms paired. I2: unchanged protocol.
Target: noise <= 10.5 ms/chunk. Effort: medium.

PROBE FALSIFIED (2026-10-11, this commit: N2 per-row unit print, R5 gate
run BEFORE any fix code): quiet rig x2 (64ch mode2): walk 6.584/7.054,
worker 8.971/9.519 (ratio 1.346-1.363, 4th+5th independent reproduction),
exposed residual 2.410/2.546. f0 = row-0 fill cost (all 8 interps) =
1.810/2.001 ms/chunk over 5 fill executions per chunk = 0.724-0.800 ms/fill;
unit-clock overhead ~0 (unit sum 17.191 == rows sum 17.20 ms/chunk
cross-check). F-W = 0.778-0.822 ms/column -> p=1 wall gain = (F-W) - f0 =
0.054/0.022 ms/column = 0.02-0.16 ms/chunk over 3 hidden fills << R2 bar
0.70 (also < strict 5% = 0.57). p>=2 regresses (prefix 1.5+ ms > window).
Sub-row split REJECTED on correctness: boundary-row halves share one aic ->
CacheOnce array-memo len-mismatch (assert) or epoch divergence; whole rows
are the only safe granularity and one row (0.72-0.80) > the whole F-W
window (0.78-0.82). Residual is real but not harvestable at row granularity
-> NP4 CLOSED per R2 (arithmetic falsification, NP2/df28217 precedent).
PIPELINE ECONOMICS (carried forward): mode2 is worker-bound (F > W), so
WALK-side cuts (SUB1 substance) have ~0 wall leverage while F > W;
WORKER fill-kernel speedups convert ~1:1 into wall (window F-W = 2.33-2.77
ms/chunk) -> NP5 opened, SUB1 re-ordered below it.

## NP5 [FILL-KERNEL SPEEDUP UNDER PIPELINE, worker-side wall leverage] [x]
HYP: mode2 is worker-bound (F = 3.025-3.173 vs W = 2.247-2.351 ms/column,
quiet rig x2): the 3 overlapped columns contribute 3xF wall, so worker
fill-kernel speedups land ~1:1 on noise (window F-W = 2.33-2.77 ms/chunk) —
unlike pre-pipeline where they were Amdahl-diluted. interp[0] = 87.9% of
fill unit mass (N2 unit table x2 runs) = wrapped per-element scalar compute;
L4 SoA y-corner batching was gated pre-pipeline at ceiling 0.8-0.9 ms = AT
bar, now converts fully while F > W. Probe FIRST (R5): re-measure the L4
premise under mode2 (slice-fill kernel share via existing unit clocks +
scoped arm prototype in an isolated worktree, byte-identical A/B stagediff +
paired ledger x2); implement only if projected paired gain >= 0.70 ms.
Target: noise <= 10.5 ms/chunk. Effort: medium-high.

PROBE FALSIFIED (2026-10-11, this commit: N5_FCM_HITS/MISSES counters at
the FlatCacheW arm + bench [N5-probe] print; R5 gate BEFORE any fix code).
RESEARCH (np5-research agent): interp[0] = sloped-cheese root BlendDensity
-> Add -> Mul(slope_lower) -> Add(-0.1) -> Add(-1) -> Mul(slope_upper) ->
Min(REF base_terrain, REF caves); FlatCache = write-once priming at
instantiate, hit = 10 int ops, miss recomputes WITHOUT store; the
window-redundancy hypothesis (adjacent cell columns re-evaluating a shared
x-boundary plane) REFUTED in code — the shared plane is filled exactly once
per chunk, no per-cell window reset exists. PROBE (64ch mode2): FlatCacheW
visits 6,283/chunk, miss share 0.0% -> window-geometry lever = 0 ms.
LEVER LEDGER vs R2 bar 0.70: (a) window geometry 0 (proven dead);
(b) dispatch-removal array arms MEASURED 0.09-0.43 sub-bar (n1-arms
tick-2318; N1-R pfd pool 0.095 ms); (c) leaf-math SoA/vectorization
forbidden/weak (P2.13 no-gather, octave-SoA grad_dot, leaves 100% YDEP
with y-free front memo complete); (d) interp[0] mass = irreducible
per-element spline+noise math short of interpreter->compiled-passes
rework (SUB1-scale+ effort, ceiling unproven, not a one-tick lever).
VERDICT: no >= 0.70 worker-fill lever exists within R3 constraints ->
NP5 CLOSED (NP2/NP4 precedent). N3 residual 2.471 (6th reproduction).
Worker-fill wall is now MEASURED-irreducible: pipeline shape (NP3) +
kernel economics are at their R3 optimum; next live item = SUB1 probe
(mode0 serial path value; mode2 walk-side leverage still ~0 while F > W).
