# GOLDEN HARNESS — canonical chunk NBT oracle (NCF Phase 0)

Owner spec: `/home/z/my-project/download/worklog.txt` (NCF worklog), items
P0.3 (dumper), P0.4 (corpus), P0.5 (order test), P0.6 (diff tool), feeding
invariant I1 (semantic equivalence to vanilla Paper 1.21.10, DataVersion 4556)
and the P1 gate (golden vectors).

## 1. Purpose

The golden corpus is THE oracle for every future zero-diff gate in the Native
Chunk Factory: a Rust-generated chunk is accepted only when `ncfdiff` proves it
semantically equal to the corresponding corpus dump produced by the real
vanilla server on the same seed / datapack / DataVersion. Byte-identity is
desired but not required (I1); semantic identity of blocks, biomes, heightmaps,
light, structure starts/references, block entities, PostProcessing and fluid
ticks is mandatory. Every gate claim must link a corpus run + diff report.

## 2. Purity law (hard rule)

Corpus-generation boots MUST be plain Purpur: `java -jar
versions/purpur-1.21.10.jar --nogui`, NO CRUSSTY agent, no `-agentpath`, no
extra plugins beyond GoldenDumper itself.

Reason: the Perlin whole-body native bridge is default-ON since TASK-148, so
any agent-armed boot silently changes worldgen output and would poison the
corpus — every future zero-diff gate would then compare against contaminated
data. `dump_corpus.sh` enforces this by construction (it boots the server
itself and never attaches the agent). GoldenDumper's header says the same; if
the plugin is ever found on an agent-armed server, the dump is INVALID and must
be regenerated.

## 3. Components

| file | role |
|---|---|
| `src/dev/crussty/golden/GoldenDumperPlugin.java` | Bukkit plugin; console/RCON command `goldendump`; forces FULL generation and writes canonical NBT per chunk (main thread, <=8 chunks/tick, progress every 100) |
| `src/dev/crussty/golden/VectorHook.java` | Phase 2 vector-capture stub — documented, NOT implemented in v1 |
| `src/plugin.yml` | plugin descriptor (name GoldenDumper, api-version '1.21') |
| `build_golden.sh` | compiles + packages `GoldenDumper.jar` (reads the mojang-mapped server jar from SERVER_DIR; never writes there) |
| `dump_corpus.sh` | one server lifecycle per invocation: boot PURE vanilla, poll `Done (`, issue `goldendump <label> manifest <plan>` over RCON, poll `GOLDEN DUMP COMPLETE`, stop, append results line |
| `tools/ncfdiff.py` | semantic NBT diff (P0.6); selftest without a server |

Build: `./build_golden.sh` (JDK `javac` if present; this sandbox only has a JRE
so it falls back to `tools/ecj.jar` — same repo precedent as
`scripts/build_noise.sh`). Install: copy `GoldenDumper.jar` into the corpus
server's `plugins/` dir.

## 4. Verified Mojang-mapped APIs (Purpur 1.21.10)

All names below were verified 2026-10-05 by method-table/constant-pool
inspection of the mojang-mapped server jar
`/home/z/server/versions/1.21.10/purpur-1.21.10.jar` (the paperclip launcher at
`versions/purpur-1.21.10.jar` contains no classes; the mapped image appears
under `versions/<mcver>/` after the first boot):

| API | verified shape |
|---|---|
| `org.bukkit.craftbukkit.CraftWorld.getHandle()` | `-> net.minecraft.server.level.ServerLevel` |
| `ServerLevel.getChunkSource()` | `-> ServerChunkCache` |
| `ServerChunkCache.getChunk(int,int,ChunkStatus,boolean)` | `-> ChunkAccess` |
| `net.minecraft.world.level.chunk.status.ChunkStatus.FULL` | package `...chunk.status` confirmed in 1.21.x |
| `net.minecraft.world.level.chunk.storage.SerializableChunkData.copyOf(ServerLevel, ChunkAccess)` | `-> SerializableChunkData` (a `Record`) |
| `SerializableChunkData.write()` | `-> CompoundTag` |
| `net.minecraft.nbt.NbtIo.writeCompressed(CompoundTag, java.nio.file.Path)` | also `(..., OutputStream)` overload |
| `ServerLevel.getSeed()` | `-> long` |
| `LevelChunk.getPersistedStatus()` | `-> ChunkStatus` |
| `CompoundTag.getString(String)` / `getInt(String)` | return `java.util.Optional` (1.21.5+ getter shape) |
| `SharedConstants.getCurrentVersion()` | `-> net.minecraft.WorldVersion` |
| `WorldVersion.dataVersion()` | `-> net.minecraft.world.level.storage.DataVersion` |
| `DataVersion.version()` | `-> int` |
| `ServerLevel.getServer()` / `MinecraftServer.getWorldData()` / `WorldData.getLevelName()` | level name chain |
| `Level.dimension()` / `ResourceKey.location()` | dimension key chain |

IMPORTANT spec correction: the owner spec's
`ChunkSerializer.write(level, level.getPoiManager(), chunk)` does NOT exist in
1.21.10 — `net.minecraft.world.level.chunk.storage.ChunkSerializer` was
refactored into the `SerializableChunkData` record (POI data no longer part of
the chunk payload; `PoiManager` is not a parameter). The dumper uses the
verified 1.21.10 shape.

Remaining runtime-only VERIFY-1.21.10 items (marked in the code):
- whether `SerializableChunkData.write()` bytes == the `.mca` region payload
  minus container framing (check when the first corpus lands);
- heightmap bit width 9 and block/biome palette packing details in
  `ncfdiff.py` (verify against the first REAL dumps; record findings in the NCF
  worklog LOG, do not silently patch).

## 5. Output layout

Output root: system property `goldendump.out`, default `<serverdir>/golden`
(server dir resolved from the plugin data folder).

```
<out>/<label>/
  manifest.tsv                      # one row per dumped chunk (append order)
  meta.properties                   # once per dump (last write wins per label+seed)
  seed_<seed>/
    c_<chunkX>_<chunkZ>.nbt         # gzipped NBT (NbtIo.writeCompressed)
```

`manifest.tsv` schema (TAB-separated, `#` comment header lines):

```
chunkX <TAB> chunkZ <TAB> Status <TAB> DataVersion <TAB> file <TAB> bytes
```

`file` is relative to the label dir (`seed_<seed>/c_x_z.nbt`); `Status` and
`DataVersion` are read from the serialized tag, so the manifest describes file
contents, not memory state.

`meta.properties` keys: `seed` (level seed, long), `dataVersion` (int),
`levelName`, `dimension` (dimension key location, e.g. `minecraft:overworld`),
`timestamp` (ISO-8601 UTC, second precision), `serverJar` (best-effort path of
the jar the CraftBukkit classes were loaded from).

Console/RCON markers (grepped by the driver):
- start: `GOLDEN DUMP start label=... chunks=... seed=... dir=...`
- progress: `GOLDEN DUMP progress <done>/<total> failed=<M>` (every 100)
- per-chunk failure: `GOLDEN DUMP FAILED chunk <x> <z>: <throwable>` (never aborts)
- final: `GOLDEN DUMP COMPLETE n=<N> failed=<M> dir=<dir>`

## 5b. STAGED mode (task 5-a — oracle for the Rust stagediff gate 5-b/5-c)

Command forms (all legacy forms keep working):
- `/goldendump <x> <z> <radius> status <noise|surface>` — label derived
  (`staged_<status>`)
- `/goldendump <label> <x> <z> <radius> status <noise|surface>` — explicit
  label (used by `dump_corpus.sh STAGED_STATUS=<noise|surface>`)

For each chunk, obtained in the SAME inward-out clockwise spiral order as
PLAN=spiral (`spiralCoords`, byte-compatible with `plan_spiral_range`):
`ServerChunkCache.getChunkFuture(x, z, ChunkStatus, load=true).join()` on the
main thread, up to 8 chunks per tick (same budget as the FULL path). The dump
is a DECODED contract (NOT vanilla bit-packing) written gzipped to
`<out>/<label>/seed_<seed>/c_<x>_<z>.nbt`:

```
root { ChunkX:Int, ChunkZ:Int, Status:String("minecraft:noise"|...),
       DataVersion:Int, MinY:Int, Height:Int,
       Sections: List of 24 (DENSE, minSectionY..maxSectionY) {
         Y: Byte, Palette: List of {Name, Properties}  == NbtUtils.writeBlockState,
         Data: IntArray(4096), index i = sy*256 + sz*16 + sx },
       Biomes: List matching Sections 1:1 {
         Y: Byte, Palette: List of String, Data: IntArray(64),
         index q = by*16 + bz*4 + bx },
       Heightmaps: compound of raw long[] AS-IS (getRawData().clone()),
       block_ticks / fluid_ticks: only if non-empty ({i,t,p} SavedTick codec),
       PostProcessing: List of 24 lists of Short (packOffsets shape) }
```

Index orders VERIFIED against CFR 0.152 decompile of Purpur 1.21.10:
`Strategy.getIndex(x,y,z) = (y << bitsPerAxis | z) << bitsPerAxis | x`
(blocks bitsPerAxis=4, biomes bitsPerAxis=2) and
`PalettedContainer.get(x,y,z) = get(strategy.getIndex(x,y,z))`;
`LevelChunkSection.getBlockState/getNoiseBiome(x,y,z)` delegate with (x,y,z)
in that order.

Heightmap priming (ChunkStatus decompile + live corpus):
- statuses EMPTY..SURFACE prime `WORLDGEN_HEIGHTMAPS = {OCEAN_FLOOR_WG,
  WORLD_SURFACE_WG}` (`ChunkStatus.heightmapsAfter` ctor arg)
- CARVERS..FULL switch to `FINAL_HEIGHTMAPS = {OCEAN_FLOOR, WORLD_SURFACE,
  MOTION_BLOCKING, MOTION_BLOCKING_NO_LEAVES}`
- the staged dump copies exactly the types of `chunk.getPersistedStatus()
  .heightmapsAfter()` — the same filter as `SerializableChunkData.copyOf`
  (line-verified). CAUTION: if the chunk was ALREADY generated past the
  requested status in the same boot, the returned chunk is an
  ImposterProtoChunk and the heightmap set reflects the HIGHER status —
  request staged statuses before any FULL request.

Staged manifest.tsv: `chunkX<TAB>chunkZ<TAB>file<TAB>status`; meta.properties
gains `status`, `worldMinY`, `worldHeight`. Markers:
- start: `GOLDEN STAGED DUMP start label=... status=... chunks=... dir=...`
- per-chunk failure: `GOLDEN STAGED DUMP FAILED chunk <x> <z> status=...: ...`
- final: `GOLDEN STAGED DUMP COMPLETE <status> n=<N> failed=<M> dir=<dir>`

Inspector: `tools/stagedump_inspect.py` (summary, `--block X Y Z`, `--biome`,
`--scan N`, `--verify-index`, `--heightmaps`, `--selftest`).

Corpora (rig, vanilla seed 3053459, radius 7 -> 15x15 = 225 chunks, center
107,107 -> chunks 100..114, one FRESH boot each):
- `/home/z/server/golden/staged_vanilla_s3053459_noise/`
- `/home/z/server/golden/staged_vanilla_s3053459_surface/`
Receipts: `results/STAGED_DUMP_2026-10-05.txt`.

## 6. Corpus plan (P0.4)

1. First corpus: vanilla seed **3053459** (the repo canon seed), PLAN=quadrants
   (TASK-63 canon: two 8x8-chunk squares, block coords 1600..1727 and
   -1728..-1601, 128 chunks) and PLAN=full16 (16x16 = 256 chunks).
2. Then >=5 distinct vanilla seeds (fresh worlds, FRESH=1).
3. Then datapack corpora: `world466-stress-v1`
   (Terralith + Tectonic + BACAP + Structory) and `worldgen-stand s35-dp1`
   (Trek) — same procedure; label the dumps by seed + datapack so gates can
   select the right oracle.

Env for `dump_corpus.sh`: `SEED` (verified against `level-seed` in
server.properties, warn-only), `LABEL` (default `vanilla_s<SEED>`), `PLAN`
(`quadrants | full16 | spiral | spiral_rev`), `FRESH=1` for fresh generation
(default 0 with a loud warning — chunks loaded from disk are NOT a fresh-
generation corpus), `DUMP_TIMEOUT` (default 900 s).

P0.5 region-order test:

```
FRESH=1 PLAN=spiral     LABEL=order_spiral     ./dump_corpus.sh
FRESH=1 PLAN=spiral_rev LABEL=order_spiral_rev ./dump_corpus.sh
python3 tools/ncfdiff.py --manifest /home/z/server/golden/order_spiral \
                                   /home/z/server/golden/order_spiral_rev
```

Same region, same seed, two different generation orders (manifest lines are the
dump order). If any pair diverges: investigate the first divergent field
(ncfdiff reports section / block / biome / heightmap + coordinates); prime
suspect is cross-chunk feature placement (features spilling into neighbours
depending on generation order). Fix = pin a canonical order for ALL future
comparisons (record it here and in the NCF worklog LOG). If all pairs are
equal, record "order-independent confirmed on <corpus>" in the LOG and use any
order.

## 7. ncfdiff (P0.6)

```
python3 tools/ncfdiff.py a.nbt b.nbt [--report FILE]   # single pair
python3 tools/ncfdiff.py --manifest dirA dirB          # all c_*.nbt pairs
python3 tools/ncfdiff.py --selftest                    # no server needed
```

Verdicts: `EQUAL` (byte-identical), `SEMANTIC_EQUAL (bytes differ)` (decoded
semantics identical; byte-only fields listed, e.g. InhabitedTime/LastUpdate),
`DIVERGED` (first divergent top-level key; inside sections the first divergent
block/biome/heightmap value with coordinates). Exit codes: 0 equal (either
kind), 1 diverged, 2 usage/IO error. stdlib only; reads gzip, zlib and raw NBT.

Honest limits: MVP decoders (9-bit heightmaps, yzx block palette packing, quart
biome packing) must be confirmed against the first real corpus dumps; findings
go to the NCF worklog LOG section. List-valued payloads (block entities,
entities, ticks, PostProcessing, structure starts) are compared order-sensitively.

## 8. Vector-mode TODO (Phase 2 prep — NOT implemented)

`VectorHook.java` is a stub. Phase 2 plan: retransform-based capture of
random draws / noise samples / density-function values out of the real Paper
1.21.10 classes for a fixed coordinate vector set, written as
`<out>/<label>/vectors/*.jsonl`. The P1 gate requires the scalar IR evaluator
(P1.5) to reproduce these vectors bit-in-bit. The capture machinery must stay
OUT of the corpus dumper (purity law) — separate module/plugin on a separate
boot if needed.
