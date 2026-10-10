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

## N1 [NOISE 19.0 ms -> <= 8.27] [~]
PROBE DONE (2026-10-10, NCF_N1_PROBE bench section, this commit): clean ledger
x2 noise 17.23 ms/chunk = 81% of PORTED 21.16 (master 5ddfbbf rig-local).
perlin-core 9.27-9.73 ms/chunk PROBE-INFLATED (129,140 calls/chunk x 2
Instant clock-pairs ~ 5.5 ms pure clock overhead) -> honest perlin ~ 4-4.5
ms = 23-26% of stage, straddling the 25% SoA gate. COARSE (no-inflation)
clocks: substance(final_density CacheAllInCell) fill = 3.16 ms/chunk over
98,304 elems and only 3,840 W-node visits (precompiled ~5-node tree, 768
cells x 128 elems); y-free tile cache 133 hits / 99 misses per chunk (57%)
on 232 lookups — tiles barely engaged. Remaining split (slice fills +
per-block drive + climate + biomes) ~ 9.5-10 ms UNDECOMPOSED — next probe:
clock drive_blocks + slice-fill sites coarse. Gather SIMD FORBIDDEN (P2.13);
grad_dot is GRADIENT-table-based -> octave-SoA needs gather or 32 scalar
loads per 4 octaves = weak. N1(a) duplicate-eval verdict pending focused
research (first attempt timed out; epoch-trap analysis required: CacheOnce
lastArray epochs are call-order-observable, memo must stay within row).
(a) duplicate evaluation of the same noise at the same coordinates across
    router subtrees (memoize per node id + quart coords),
(b) weaker caching than Java's Cache2D/FlatCache/CacheOnce/CacheAllInCell
    markers (count node evaluations per chunk vs the minimum),
(c) scalar-gradient loads with 4-lane arithmetic only.
Anything skipping work must be bit-exact; gate-p2 decides.

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
