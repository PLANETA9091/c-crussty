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

## H1 [HYBRID MEASUREMENT] [ ]
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

## SUB1 [SUBSTANCE+INTERP kernel share] [ ]
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

## NP4 [PIPELINE PARENT-ASSIST, recover exposed fill residual] [ ]
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
