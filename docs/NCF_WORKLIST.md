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

## N1 [NOISE 17.23 ms -> next] [~]
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
>= 2.4 ms holds. BIGGER lever found: per-y tree dispatch — fill_array has
array-wise arms ONLY for Const/Ap2(Add/Mul); everything else (MulOrAdd,
Mapped, Clamp, RangeChoice, ...) walks PER-Y via provider_fill_all_directly
(interpolator.rs:1949, 15 roots x 5 rows x 49 y = 3,675 full-tree walks/chunk
~ 147k node visits). Extending the array-wise pattern to the linear node
kinds is the same proven bit-exact template (same ops per element, op order
per element unchanged) — candidate for 1-3 ms. NEXT TICK: array-wise arms
for MulOrAdd/Mapped/Clamp in isolated worktree -> stagediff 256/256 ->
ledger x2 (R2 5% bar = 0.86 ms of noise stage).
Hypotheses (a)/(b) ANSWERED 2026-10-10 (see R3-N1B + DECOMPOSED above):
(a) no duplicate copies exist (wrap_memo+intern dedup; markers complete);
(b) no missing markers — Java parity is complete. (c) gather forbidden;
octave-SoA weak (grad_dot table). Live levers: array-wise fill arms for the
linear node kinds (above), L4 y-corner SoA (borderline), substance-fill
kernel share (3.16 raw). Anything skipping work must be bit-exact; gate-p2
decides.

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
