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

## S2 [SURFACE, block class table] [~]
is_air_id/is_fluid_id/is_stone_id (surface_rules.rs ~1081) compare block-name
strings per block, several times per step, plus HeightmapKind::is_opaque_state
in set_block. Fix: classification flags (AIR/FLUID/OPAQUE...) stored in
StateTable (filler.rs:72) at intern time, looked up by state id.
Target: <= 8 ms.
[~] next step (S1 landed: surface now 8.71, the String classifies are the
next dominant cost inside it)

## S3 [SURFACE, no String] [ ]
biome as an interned id (u16) instead of String in probe, ctx.biome and the
condition at ~905. Target: <= 4 ms total.
After S1-S3 re-profile and write the next 3 surface items.

## N1 [NOISE 19.0 ms -> <= 8.27] [ ]
Addendum 78: AVX2 gather SIMD was slower on this KVM rig and was reverted.
Do NOT retry gather SIMD. First probe: perlin-core time vs tree-walk time
split. Then HYP candidates, each proven by a counter before coding:
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
