# NCF WORKLIST (owner standing order, 2026-10-09)

Loop: take the FIRST open item; one step per tick: probe / fix / measure /
commit. Status: [ ] open, [~] in progress, [x] done (same-commit LOG
evidence), [!] blocked (one line "BLOCKED: <reason>", next item same tick).
After each finished item: ncf_profile build, list top-3 exclusive-time
functions, append up to 3 new items (HYP + probe + target) ordered by
expected ms saved / effort. The list must never be empty.
All levers obey docs/NCF_RULES.md (R1-R5). Ledger: docs/NCF_SPEED.md.

## S1 [SURFACE, lazy biome] [ ]
update_y (surface_rules.rs ~754) calls get_biome_voted_region
(biomes.rs:340 — returns String, 8-corner vote) for EVERY solid block in
every column; ctx.biome is read only at ~857 and ~905. Java memoizes
lazily (Suppliers.memoize per updateY; worklog ~1193 states
value-identical). HYP: dominates the 21.0 ms surface stage.
PROBE: count get_biome_voted_region calls per chunk (expect tens of
thousands vs 256 needed).
FIX: store the pending (x,y,z) in update_y, compute on first read, cache
until the next update_y.
TARGET: surface <= 12 ms after S1 alone.

## S2 [SURFACE, block class table] [ ]
is_air_id/is_fluid_id/is_stone_id (surface_rules.rs ~1081) compare
block-name strings per block, several times per step, plus
HeightmapKind::is_opaque_state in set_block.
FIX: classification flags (AIR/FLUID/OPAQUE...) stored in StateTable
(filler.rs:72) at intern time, looked up by state id.
TARGET: surface <= 8 ms.

## S3 [SURFACE, no String] [ ]
biome as an interned id (u16) instead of String in probe, ctx.biome and
the condition at ~905.
TARGET: surface <= 4 ms total. After S1-S3 re-profile and write the next
3 surface items here.

## N1 [NOISE 19.0 ms -> <= 8.27] [ ]
Addendum 78: AVX2 gather SIMD was slower on this KVM rig and was reverted.
Do NOT retry gather SIMD.
PROBE FIRST: perlin-core time vs tree-walk time split. Then HYP candidates,
each proven by a counter before coding:
(a) duplicate evaluation of the same noise at the same coordinates across
    router subtrees (memoize per node id + quart coords),
(b) weaker caching than Java's Cache2D/FlatCache/CacheOnce/CacheAllInCell
    markers (count node evaluations per chunk vs the minimum),
(c) scalar-gradient loads with 4-lane arithmetic only.
Anything skipping work must be bit-exact; gate-p2 decides.

## H1 [HYBRID MEASUREMENT] [ ]
The 3.32x figure is a model (unported stages at 1x, incl. jvm_other
48.9 ms). Measure the real hybrid: same machine, fixed 1024-chunk region
(the E2E region bench exists), pure Java vs native-then-Java-completion,
world restored byte-identically (I10), report chunks/s and CPU-ms/chunk
(JFR). Question: does jvm_other shrink when native stages replace Java
work? If the hybrid hook is not wired, wire it default-off with the I8
fallback. Label results with core count (2 vCPU rig).

## C1 [CORRECTNESS AT SCALE, always available] [~]
checklist line "0 diffs on >= 10^4 chunks x >= 5 seeds x {vanilla,
Terralith, Tectonic}" is still [~]. Add seeds and chunk counts to CI
until it is [x] with evidence.

## F1 [FEATURES, demoted] [ ]
Features are 4.8 of 363 ms Java (1.3%). Do one feature increment only
after every 3 speed commits, and prefer increments that unlock the most
biomes in the census. Next per addendum 94: weighted_state_provider as
to_place in compact simple_block (39 biomes), then fancy_trunk_placer (7),
glowstone_blob (5) / vines (4).

## OWNER-Q (recorded worklog addendum 95; continue without waiting)
OWNER-Q1: what does x100 mean (per core, or total pregen on N cores)?
OWNER-Q2: N = cores on the target machine?
