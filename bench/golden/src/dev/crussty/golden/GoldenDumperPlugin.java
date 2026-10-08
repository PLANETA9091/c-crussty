package dev.crussty.golden;

// ============================================================================
// GoldenDumperPlugin — canonical chunk NBT oracle dumper (NCF P0.3).
//
// PURITY LAW: this plugin must ONLY run on a corpus-generation server booted
// WITHOUT the CRUSSTY agent (plain `java -jar purpur-1.21.10.jar --nogui`).
// The golden corpus must be pure vanilla semantics. Since TASK-148 the Perlin
// whole-body native bridge is default-ON, so any agent-armed boot would
// contaminate the corpus and silently poison every future zero-diff gate
// (NCF invariant I1, see bench/golden/GOLDEN_HARNESS.md).
//
// What it does (NCF P0.3):
//   /goldendump <label> <chunkX> <chunkZ> <radius>   — square (2r+1)^2, r<=32
//   /goldendump <label> manifest <file>              — lines "chunkX<TAB>chunkZ",
//                                                      '#' comments / blanks ignored, cap 20000
//   /goldendump <label> raw <chunkX> <chunkZ> <radius> — session-4 RAW mode
//   /goldendump raw <chunkX> <chunkZ> <radius>         — shorthand, label "raw"
//
// RAW mode (NCF P3.1 input): everything the default mode does (compressed
// seed_<seed>/c_<x>_<z>.nbt + manifest.tsv row — BACKWARD COMPATIBLE, the
// manifest keeps its v1 columns) PLUS:
//   1. <out>/<label>/raw/chunk.<cx>.<cz>.nbt — UNCOMPRESSED NBT of the same
//      CompoundTag: NbtIo.write(tag, DataOutput) — the DataOutput overload
//      writes UNCOMPRESSED (javap-verified 2026-10-05; writeCompressed is the
//      gzip path). The Rust mcaforge tool repackages these into .mca region
//      files (1:1 container framing). DataVersion inside the tag is NOT
//      touched — ChunkSerializer/SerializableChunkData output is used as-is.
//   2. <out>/<label>/raw/probes.tsv — TWO deterministic probe rows per chunk
//      (dump order): columns (minX+3, minZ+5) and (minX+10, minZ+12),
//      minX = cx*16, minZ = cz*16; scan y from level.getMaxY()-1 DOWN to
//      level.getMinY(); first BlockState with !isAir() ->
//      "<x>,<y>,<z>,<block registered name>". If no non-air exists (should
//      not happen): "<x>,<minY>,<z>,minecraft:air". Header: "# x,y,z,block".
//      These give mcaforge a fast bit-plausibility oracle per chunk.
//   3. <out>/<label>/raw/manifest.tsv — separate raw manifest (does not touch
//      the v1 manifest.tsv): chunkX, chunkZ, Status, DataVersion, rawFile,
//      rawBytes, compressedBytes.
//   Completion adds a marker line BEFORE the standard one (drivers grep the
//   standard "GOLDEN DUMP COMPLETE n="): "GOLDEN DUMP RAW COMPLETE ...".
//
// ----------------------------------------------------------------------------
// STAGED mode (NCF task 5-a — oracle for the Rust stagediff gate 5-b/5-c):
//   /goldendump <chunkX> <chunkZ> <radius> status <noise|surface>
//     — NO label slot: the label is derived canonically as
//       "staged_<statusName>" (e.g. staged_noise). All previous command forms
//       keep working unchanged.
//   /goldendump <label> <chunkX> <chunkZ> <radius> status <noise|surface>
//     — 6-arg LABELED form (dump_corpus.sh STAGED_STATUS): the label is used
//       verbatim so the corpus driver can target staged_vanilla_s<seed>_<status>.
//   Semantics: for each chunk of the square (2r+1)^2, in the SAME inward-out
//   clockwise SPIRAL order as the FULL dumper's PLAN=spiral (mirrors
//   bench/golden/dump_corpus.sh plan_spiral exactly), obtain a ChunkAccess
//   generated UP TO the requested ChunkStatus via
//   ServerChunkCache.getChunkFuture(x, z, status, true) and serialize a
//   DECODED (NOT vanilla bit-packed) NBT:
//
//   root compound, gzipped via NbtIo.writeCompressed to
//   <outdir>/<label>/seed_<seed>/c_<x>_<z>.nbt:
//     ChunkX: Int, ChunkZ: Int              (chunk coords)
//     Status: String                        (registry key, "minecraft:noise" /
//                                           "minecraft:surface" — same string
//                                           vanilla writes into .mca "Status")
//     DataVersion: Int, MinY: Int, Height: Int
//     Sections: List (one entry per section, minSectionY..maxSectionY, ALWAYS
//               dense/uniform — no empty-section elision), each:
//         Y: Byte (section Y, worldY>>4)
//         Palette: List of compound  {Name: String, Properties: {k: v}}  —
//                  EXACTLY NbtUtils.writeBlockState(state) (vanilla palette form)
//         Data: IntArray(4096) of palette indices,
//               index i = sy*256 + sz*16 + sx  (local coords, sy = section-local y)
//               VERIFIED from CFR decompile 2026-10-05:
//               net.minecraft.world.level.chunk.Strategy.getIndex ->
//                 "(y << bitsPerAxis | z) << bitsPerAxis | x",
//               block states bitsPerAxis=4 -> (y<<4|z)<<4|x = y*256+z*16+x.
//     Biomes: List matching Sections 1:1 (same Y order), each:
//         Y: Byte
//         Palette: List of String (biome id, e.g. "minecraft:ocean")
//         Data: IntArray(64) of palette indices,
//               index q = by*16 + bz*4 + bx  (quart coords in section)
//               VERIFIED same Strategy.getIndex, biomes bitsPerAxis=2 ->
//               (y<<2|z)<<2|x = y*16+z*4+x.
//     Heightmaps: compound of raw long[] COPIED AS-IS (NOT decoded), one key
//               per primed type (getSerializationKey()). Primes verified:
//               statuses up to SURFACE track WORLDGEN_HEIGHTMAPS =
//               {OCEAN_FLOOR_WG, WORLD_SURFACE_WG} (ChunkStatus ctor arg,
//               CFR-verified); OCEAN_FLOOR_WG/WORLD_SURFACE_WG are updated
//               in-place by NoiseBasedChunkGenerator.doFill (heightmap.update
//               per placed block) and by ProtoChunk.setBlockState for any
//               status in heightmapsAfter; CARVERS..FULL switch to the four
//               FINAL heightmaps (primeHeightmaps at FEATURES). NOTE: if the
//               chunk was ALREADY generated past the requested status in this
//               boot, the returned chunk is an ImposterProtoChunk and
//               getPersistedStatus() delegates to the wrapped chunk — the
//               heightmap set then reflects the HIGHER status. Dumps that
//               request a status before any higher request are immune.
//     block_ticks / fluid_ticks: written ONLY if non-empty at that status
//               (SavedTick codec shape {i,t,p} via SavedTick.codec().listOf();
//               empty at NOISE/SURFACE in practice — observed on the corpus).
//     PostProcessing: List of Lists of Short, one inner list per section
//               (mirrors SerializableChunkData.packOffsets exactly: empty
//               inner lists included for every section). NON-EMPTY at NOISE:
//               aquifer fluid updates call markPosForPostprocessing during
//               doFill (verified in NoiseBasedChunkGenerator decompile).
//
//   manifest.tsv (in <outdir>/<label>/): columns chunkX<TAB>chunkZ<TAB>
//   file<TAB>status (relative seed_<seed>/c_<x>_<z>.nbt; status = registry key).
//   meta.properties gains staged-only keys: status, worldMinY, worldHeight.
//   Completion marker (grepped by dump_corpus.sh STAGED_STATUS mode):
//     GOLDEN STAGED DUMP COMPLETE <status> n=<N> failed=<M> dir=<dir>
//
//   Main-thread discipline: same 8-chunks-per-tick budget as the FULL path;
//   each staged tick BLOCKS in getChunkFuture(...).join() for up to 8 chunks
//   (Paper/Moonrise getChunkFuture already managedBlock()s on the main thread
//   — decompile-verified: scheduleChunkLoad(..., Priority.HIGHER) then
//   managedBlock(isDone)); the join lets the worker pool run while the main
//   thread waits, one batch per tick, so the chunk system never starves.
// ----------------------------------------------------------------------------
// For each requested chunk (one at a time, on the MAIN server thread):
//   1. force full generation: ServerChunkCache.getChunk(x, z, ChunkStatus.FULL, true)
//      — the standard vanilla force path; brings the chunk to FULL/ticking state.
//   2. serialize: SerializableChunkData.copyOf(level, chunk).write() -> CompoundTag
//      (1.21.10 shape; the pre-1.21.2 ChunkSerializer.write was refactored into
//      this Record, see bench/golden/GOLDEN_HARNESS.md "verified APIs").
//   3. write gzipped NBT to <out>/<label>/seed_<seed>/c_<x>_<z>.nbt via
//      NbtIo.writeCompressed(tag, path).
//   4. append a manifest.tsv row (chunkX, chunkZ, Status, DataVersion, file, bytes).
//
// Main-thread discipline: chunks are processed up to CHUNKS_PER_TICK (8) per
// tick via repeated Bukkit.getScheduler().runTask(...) rounds, so the server
// never stalls in one giant frame; progress logs every 100 processed chunks.
// Per-chunk exceptions are logged with coordinates, counted as failed, and
// NEVER abort the run. Completion marker (grepped by dump_corpus.sh):
//   GOLDEN DUMP COMPLETE n=<N> failed=<M> dir=<dir>
//
// API verification status (2026-10-05): every Mojang-mapped name below was
// verified by constant-pool inspection of the mojang-mapped server jar
// /home/z/server/versions/1.21.10/purpur-1.21.10.jar (methods + descriptors).
// Remaining runtime-only risks are marked VERIFY-1.21.10 with the reason.
//
// Session-4 additions (javap 2026-10-05):
//   NbtIo.write(CompoundTag, java.io.DataOutput)                public static
//     — UNCOMPRESSED NBT (the writeCompressed overloads are the gzip path;
//       NbtIo.write(CompoundTag, Path) also exists but the DataOutput form is
//       used to make the framing explicit for the Rust repackager)
//   Level.getMinY() / Level.getMaxY()                          public
//     (Level implements LevelHeightAccessor; getMaxY is a default method =
//      minY + height - 1)
//   LevelChunk.getBlockState(BlockPos)                          public
//     (absolute world coords inside the chunk; ChunkAccess/BlockGetter shape)
//   BlockBehaviour$BlockStateBase.isAir()                       public final
//   BlockBehaviour$BlockStateBase.getBlock()                    public
//   Registries.BLOCK / Registries.BIOME                         public constants
//   RegistryAccess.lookupOrThrow(ResourceKey) -> Registry<E>    public default
//     (registryOrThrow does NOT exist in 1.21.10 — see VectorCapture header)
//   Registry.getKey(T) -> ResourceLocation                      public abstract
// ============================================================================

import java.io.BufferedOutputStream;
import java.io.BufferedWriter;
import java.io.DataOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;
import java.security.CodeSource;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.Properties;
import java.util.concurrent.CompletableFuture;

import it.unimi.dsi.fastutil.shorts.ShortList;

import net.minecraft.SharedConstants;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Holder;
import net.minecraft.core.Registry;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.nbt.IntArrayTag;
import net.minecraft.nbt.ListTag;
import net.minecraft.nbt.LongArrayTag;
import net.minecraft.nbt.NbtIo;
import net.minecraft.nbt.ShortTag;
import net.minecraft.nbt.NbtUtils;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.level.ChunkResult;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.biome.Biome;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.LevelChunk;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.chunk.storage.SerializableChunkData;
import net.minecraft.world.level.levelgen.Heightmap;
import net.minecraft.world.ticks.SavedTick;

import com.mojang.serialization.Codec;

import org.bukkit.Bukkit;
import org.bukkit.World;
import org.bukkit.command.Command;
import org.bukkit.command.CommandExecutor;
import org.bukkit.command.CommandSender;
import org.bukkit.command.ConsoleCommandSender;
import org.bukkit.command.RemoteConsoleCommandSender;
import org.bukkit.craftbukkit.CraftServer;
import org.bukkit.craftbukkit.CraftWorld;
import org.bukkit.plugin.java.JavaPlugin;

public final class GoldenDumperPlugin extends JavaPlugin implements CommandExecutor {

    /** Chunks generated+serialized per server tick (kept small so the main thread breathes). */
    static final int CHUNKS_PER_TICK = 8;
    /** Max radius for the square form: (2*32+1)^2 = 4225 chunks. */
    static final int MAX_RADIUS = 32;
    /** Max coordinate lines accepted via the manifest file form. */
    static final int MAX_MANIFEST_LINES = 20000;
    /** Log a progress line every N processed chunks. */
    static final int PROGRESS_EVERY = 100;

    private volatile boolean busy = false;

    @Override
    public void onEnable() {
        if (getCommand("goldendump") == null || getCommand("goldenvec") == null) {
            // plugin.yml is malformed or not packaged — fail loudly, the harness is unusable.
            throw new IllegalStateException("goldendump/goldenvec command missing (plugin.yml not packaged?)");
        }
        getCommand("goldendump").setExecutor(this);
        getCommand("goldenvec").setExecutor(this);
        if (getCommand("goldensurface") != null) {
            getCommand("goldensurface").setExecutor(this);
        }
        getLogger().info("GoldenDumper ready: corpus dumps + vector captures are vanilla-only (no CRUSSTY agent boots)");
    }

    // ------------------------------------------------------------------
    // Command entry (main thread: Paper dispatches console/RCON commands here)
    // ------------------------------------------------------------------

    @Override
    public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
        // /goldenvec has its own dispatch (synchronous, no chunk gen).
        if (command.getName().equals("goldenvec")) {
            getLogger().info("goldenvec invoked by " + sender.getClass().getName()
                    + " args=" + java.util.Arrays.toString(args));
            boolean trustedVec = sender instanceof ConsoleCommandSender
                    || sender instanceof RemoteConsoleCommandSender
                    || sender.getClass().getName().contains("RemoteConsole")
                    || sender.getClass().getName().contains("CraftConsoleCommandSender");
            if (!trustedVec) {
                sender.sendMessage("goldenvec: refused — console/RCON senders only");
                return true;
            }
            if (args.length != 1 || !args[0].matches("[A-Za-z0-9._-]{1,80}")) {
                sender.sendMessage("usage: /goldenvec <label> (allowed [A-Za-z0-9._-])");
                return true;
            }
            VectorCapture.run(this, args[0], sender);
            return true;
        }
        // session 6 bisect rig: /goldensurface <cx> <cz> — replicate the
        // buildSurface walk on the REAL generated chunk and dump the
        // SurfaceRules.Context fields per block (reflection into the
        // package-private context).
        if (command.getName().equals("goldensurface")) {
            getLogger().info("goldensurface invoked args=" + java.util.Arrays.toString(args));
            if (args.length != 2) {
                sender.sendMessage("usage: /goldensurface <chunkX> <chunkZ>");
                return true;
            }
            try {
                VectorCapture.captureSurface(this, Integer.parseInt(args[0]), Integer.parseInt(args[1]), sender);
            } catch (Throwable t) {
                getLogger().warning("GOLDENSURFACE FAILED: " + t);
                t.printStackTrace();
                sender.sendMessage("goldensurface: FAILED: " + t);
            }
            return true;
        }
        // NCF P5.3 piece-engine oracle: /goldenpieces <chunkX> <chunkZ> —
        // force-generate to STRUCTURE_STARTS, dump StructureStart pieces
        // (boxes / ground level delta / junctions) as JSON for the Rust
        // piece_dump bit-compare.
        if (command.getName().equals("goldenpieces")) {
            getLogger().info("goldenpieces invoked args=" + java.util.Arrays.toString(args));
            if (args.length != 2) {
                sender.sendMessage("usage: /goldenpieces <chunkX> <chunkZ>");
                return true;
            }
            try {
                VectorCapture.capturePieces(this,
                        Integer.parseInt(args[0]), Integer.parseInt(args[1]), sender);
            } catch (Throwable t) {
                getLogger().warning("GOLDENPIECES FAILED: " + t);
                t.printStackTrace();
                sender.sendMessage("goldenpieces: FAILED: " + t);
            }
            return true;
        }
        // P5.3 inc6: /goldenrefs <chunkX> <chunkZ> — the chunk's reference map
        // + the exact Beardifier input starts (references semantics oracle).
        if (command.getName().equals("goldenrefs")) {
            getLogger().info("goldenrefs invoked args=" + java.util.Arrays.toString(args));
            if (args.length != 2) {
                sender.sendMessage("usage: /goldenrefs <chunkX> <chunkZ>");
                return true;
            }
            try {
                VectorCapture.captureRefs(this,
                        Integer.parseInt(args[0]), Integer.parseInt(args[1]), sender);
            } catch (Throwable t) {
                getLogger().warning("GOLDENREFS FAILED: " + t);
                t.printStackTrace();
                sender.sendMessage("goldenrefs: FAILED: " + t);
            }
            return true;
        }
        // T38-B bisect rig: /goldendensity <blockX> <blockZ> <y0> <y1> <label>
        // — dump interp/aquifer/density vector families at an ARBITRARY chunk
        // column (negative coords fine, no JVM flags: coords are args).
        if (command.getName().equals("goldendensity")) {
            getLogger().info("goldendensity invoked args=" + java.util.Arrays.toString(args));
            if (args.length != 5) {
                sender.sendMessage("usage: /goldendensity <blockX> <blockZ> <y0> <y1> <label>");
                return true;
            }
            try {
                VectorCapture.captureDensity(this,
                        Integer.parseInt(args[0]), Integer.parseInt(args[1]),
                        Integer.parseInt(args[2]), Integer.parseInt(args[3]),
                        args[4], sender);
            } catch (Throwable t) {
                getLogger().warning("GOLDENDENSITY FAILED: " + t);
                t.printStackTrace();
                sender.sendMessage("goldendensity: FAILED: " + t);
            }
            return true;
        }
        // T38-B resid bisect: /aquafields <blockX> <blockY> <blockZ> — dump the
        // machine-WRAPPED aquifer field values (the NoiseBasedAquifer's private
        // DensityFunction fields — these are noiseRouter1 = NoiseChunk.mapAll(wrap)
        // versions, NOT the raw randomState router) + the real computeFluid
        // status at ONE position. The decision-trace oracle for the status layer.
        if (command.getName().equals("aquafields")) {
            getLogger().info("aquafields invoked args=" + java.util.Arrays.toString(args));
            if (args.length != 3) {
                sender.sendMessage("usage: /aquafields <blockX> <blockY> <blockZ>");
                return true;
            }
            try {
                VectorCapture.aquaFields(this,
                        Integer.parseInt(args[0]), Integer.parseInt(args[1]),
                        Integer.parseInt(args[2]), sender);
            } catch (Throwable t) {
                getLogger().warning("AQUAFIELDS FAILED: " + t);
                t.printStackTrace();
                sender.sendMessage("aquafields: FAILED: " + t);
            }
            return true;
        }
        // Executor/console-only guard. RCON commands arrive as
        // RemoteConsoleCommandSender (used by dump_corpus.sh via bench/ab/rcon.py),
        // the real console as ConsoleCommandSender; everything else is refused.
        // NOTE: the instanceof pair is belt-and-suspenders; the name-based
        // fallback exists because the plugin jar goes through Paper's plugin
        // remapper and interface identity must not depend on remap artifacts.
        // EVERY refusal path logs to the server log (getLogger) — an RCON-only
        // refusal must never be invisible to the harness driver.
        getLogger().info("goldendump invoked by " + sender.getClass().getName()
                + " args=" + java.util.Arrays.toString(args));
        boolean trusted = sender instanceof ConsoleCommandSender
                || sender instanceof RemoteConsoleCommandSender
                || sender.getClass().getName().contains("RemoteConsole")
                || sender.getClass().getName().contains("CraftConsoleCommandSender");
        if (!trusted) {
            sender.sendMessage("goldendump: refused — console/RCON senders only (got "
                    + sender.getClass().getName() + ")");
            return true;
        }
        if (args.length < 2) {
            getLogger().warning("goldendump refused: usage (args=" + args.length + ")");
            sender.sendMessage("usage: /goldendump <label> (<chunkX> <chunkZ> <radius>"
                    + "|raw <chunkX> <chunkZ> <radius>|manifest <file>)"
                    + "|<chunkX> <chunkZ> <radius> status <noise|surface|carvers>"
                    + "|<label> <chunkX> <chunkZ> <radius> status <noise|surface>)");
            return true;
        }
        final String outLabel = args[0];
        if (!outLabel.matches("[A-Za-z0-9._-]{1,80}")) {
            // label becomes a path component: no slashes, no traversal.
            getLogger().warning("goldendump refused: bad label '" + outLabel + "'");
            sender.sendMessage("goldendump: bad label '" + outLabel + "' — allowed [A-Za-z0-9._-], max 80 chars");
            return true;
        }
        if (busy) {
            getLogger().warning("goldendump refused: busy");
            sender.sendMessage("goldendump: another dump is already running — wait for GOLDEN DUMP COMPLETE");
            return true;
        }

        final boolean raw;
        final ChunkStatus stagedStatus;
        boolean stagedLabeled = false; // 6-arg form: label <x> <z> <r> status <status>
        final long[] coords;
        try {
            if (args.length == 6 && args[4].equalsIgnoreCase("status")) {
                // task 5-a STAGED mode, LABELED form (dump_corpus.sh STAGED_STATUS):
                // /goldendump <label> <x> <z> <radius> status <noise|surface>
                // The label is used verbatim as the output dir component so the
                // corpus driver can target staged_vanilla_s<seed>_<status>.
                raw = false;
                stagedLabeled = true;
                int cx = Integer.parseInt(args[1]);
                int cz = Integer.parseInt(args[2]);
                int r = Integer.parseInt(args[3]);
                requireRadius(r);
                stagedStatus = parseStagedStatus(args[5]);
                coords = spiralCoords(cx, cz, r);
            } else if (args.length == 5 && args[3].equalsIgnoreCase("status")) {
                // task 5-a STAGED mode: /goldendump <x> <z> <radius> status <noise|surface>
                // No label slot — startDump derives "staged_<statusName>".
                raw = false;
                stagedLabeled = false;
                int cx = Integer.parseInt(args[0]);
                int cz = Integer.parseInt(args[1]);
                int r = Integer.parseInt(args[2]);
                requireRadius(r);
                stagedStatus = parseStagedStatus(args[4]);
                coords = spiralCoords(cx, cz, r);
            } else if (args.length == 5 && args[1].equalsIgnoreCase("raw")) {
                // session-4 RAW mode, labeled form: /goldendump <label> raw <cx> <cz> <r>
                raw = true;
                stagedStatus = null;
                int cx = Integer.parseInt(args[2]);
                int cz = Integer.parseInt(args[3]);
                int r = Integer.parseInt(args[4]);
                requireRadius(r);
                coords = squareCoords(cx, cz, r);
            } else if (args.length == 4 && outLabel.equalsIgnoreCase("raw")
                    && !args[1].equalsIgnoreCase("manifest")) {
                // session-4 RAW mode shorthand: /goldendump raw <cx> <cz> <r>
                // (label defaults to "raw"); NOTE this repurposes the old
                // "compressed dump with label raw" 4-arg form — nobody uses it
                // (all corpus labels are vanilla_s*), documented in the header.
                raw = true;
                stagedStatus = null;
                stagedLabeled = false;
                int cx = Integer.parseInt(args[1]);
                int cz = Integer.parseInt(args[2]);
                int r = Integer.parseInt(args[3]);
                requireRadius(r);
                coords = squareCoords(cx, cz, r);
            } else if (args.length == 4 && !args[1].equalsIgnoreCase("manifest")) {
                raw = false;
                stagedStatus = null;
                stagedLabeled = false;
                int cx = Integer.parseInt(args[1]);
                int cz = Integer.parseInt(args[2]);
                int r = Integer.parseInt(args[3]);
                requireRadius(r);
                coords = squareCoords(cx, cz, r);
            } else if (args.length == 3 && args[1].equalsIgnoreCase("manifest")) {
                raw = false;
                stagedStatus = null;
                stagedLabeled = false;
                coords = readManifest(Paths.get(args[2]));
            } else {
                sender.sendMessage("usage: /goldendump <label> (<chunkX> <chunkZ> <radius>"
                        + "|raw <chunkX> <chunkZ> <radius>|manifest <file>)"
                        + "|<chunkX> <chunkZ> <radius> status <noise|surface|carvers>"
                        + "|<label> <chunkX> <chunkZ> <radius> status <noise|surface>)");
                return true;
            }
        } catch (NumberFormatException e) {
            getLogger().warning("goldendump refused: non-integer coords");
            sender.sendMessage("goldendump: chunk coords and radius must be integers");
            return true;
        } catch (IllegalArgumentException e) {
            getLogger().warning("goldendump refused: " + e.getMessage());
            sender.sendMessage("goldendump: " + e.getMessage());
            return true;
        } catch (IOException e) {
            getLogger().warning("goldendump refused: manifest read failed: " + e);
            sender.sendMessage("goldendump: manifest read failed: " + e);
            return true;
        }
        if (coords.length == 0) {
            getLogger().warning("goldendump refused: empty coordinate plan");
            sender.sendMessage("goldendump: empty coordinate plan — nothing to do");
            return true;
        }

        try {
            final String dumpLabel;
            if (stagedStatus != null) {
                // STAGED mode label: the 6-arg form carries an explicit label;
                // the 5-arg form derives it canonically
                // (getKey -> "minecraft:noise"; getPath -> "noise").
                dumpLabel = stagedLabeled ? outLabel
                        : "staged_" + BuiltInRegistries.CHUNK_STATUS.getKey(stagedStatus).getPath();
            } else {
                dumpLabel = outLabel;
            }
            startDump(dumpLabel, coords, raw, stagedStatus, sender);
        } catch (IllegalStateException e) {
            sender.sendMessage("goldendump: " + e.getMessage());
        }
        return true;
    }

    private static void requireRadius(int r) {
        if (r < 0 || r > MAX_RADIUS) {
            throw new IllegalArgumentException("radius must be 0.." + MAX_RADIUS);
        }
    }

    /** STAGED mode status word -> registry ChunkStatus ("minecraft:" prefix tolerated). */
    private static ChunkStatus parseStagedStatus(String arg) {
        String want = arg.toLowerCase(Locale.ROOT);
        if (want.startsWith("minecraft:")) want = want.substring("minecraft:".length());
        if (want.equals("noise")) {
            return ChunkStatus.NOISE;
        }
        if (want.equals("surface")) {
            return ChunkStatus.SURFACE;
        }
        // session 6 (P2.8): CARVERS-status oracle dumps for the carver gate
        if (want.equals("carvers")) {
            return ChunkStatus.CARVERS;
        }
        throw new IllegalArgumentException("unknown staged status '" + arg
                + "' — supported: noise | surface | carvers");
    }

    /**
     * Spiral plan for the STAGED mode: (2r+1)^2 square centered at (cx, cz),
     * inward-out clockwise spiral. VERIFIED byte-compatible mirror of
     * bench/golden/dump_corpus.sh plan_spiral (the canonical PLAN=spiral
     * order, NCF P0.5): first the north edge west->east at z=z0, then the
     * east edge north->south, then (if x1>x0) the south edge east->west, then
     * (if z1>z0) the west edge south->north, then shrink the ring.
     */
    static long[] spiralCoords(int cx, int cz, int r) {
        int x0 = cx - r, x1 = cx + r, z0 = cz - r, z1 = cz + r;
        List<Long> list = new ArrayList<>((2 * r + 1) * (2 * r + 1));
        while (x0 <= x1 && z0 <= z1) {
            for (int x = x0; x <= x1; x++) list.add(pack(x, z0));
            for (int z = z0 + 1; z <= z1; z++) list.add(pack(x1, z));
            if (x1 > x0) {
                for (int x = x1 - 1; x >= x0; x--) list.add(pack(x, z1));
            }
            if (z1 > z0) {
                for (int z = z1 - 1; z > z0; z--) list.add(pack(x0, z));
            }
            x0++; x1--; z0++; z1--;
        }
        long[] out = new long[list.size()];
        for (int i = 0; i < out.length; i++) out[i] = list.get(i);
        return out;
    }

    /** Square plan, row-major: x ascending outer loop, z ascending inner loop (deterministic order). */
    static long[] squareCoords(int cx, int cz, int r) {
        List<Long> list = new ArrayList<>((2 * r + 1) * (2 * r + 1));
        for (int x = cx - r; x <= cx + r; x++) {
            for (int z = cz - r; z <= cz + r; z++) {
                list.add(pack(x, z));
            }
        }
        long[] out = new long[list.size()];
        for (int i = 0; i < out.length; i++) out[i] = list.get(i);
        return out;
    }

    /**
     * Manifest file: lines "chunkX<TAB>chunkZ"; blank lines and lines starting
     * with '#' are ignored; hard cap MAX_MANIFEST_LINES (refuse, do not truncate).
     * The ORDER of lines is the dump order — this is what makes the P0.5
     * region-order test possible (spiral vs reversed-spiral plans).
     */
    static long[] readManifest(Path file) throws IOException {
        if (!Files.isRegularFile(file)) {
            throw new IllegalArgumentException("manifest file not found: " + file);
        }
        List<Long> list = new ArrayList<>();
        int lineNo = 0;
        for (String line : Files.readAllLines(file, StandardCharsets.UTF_8)) {
            lineNo++;
            String t = line.trim();
            if (t.isEmpty() || t.startsWith("#")) continue;
            String[] parts = t.split("\t");
            if (parts.length < 2) {
                throw new IllegalArgumentException("manifest line " + lineNo + " is not \"chunkX<TAB>chunkZ\": " + line);
            }
            try {
                list.add(pack(Integer.parseInt(parts[0].trim()), Integer.parseInt(parts[1].trim())));
            } catch (NumberFormatException e) {
                throw new IllegalArgumentException("manifest line " + lineNo + " has non-integer coords: " + line);
            }
            if (list.size() > MAX_MANIFEST_LINES) {
                throw new IllegalArgumentException("manifest exceeds " + MAX_MANIFEST_LINES + " entries — refused");
            }
        }
        long[] out = new long[list.size()];
        for (int i = 0; i < out.length; i++) out[i] = list.get(i);
        return out;
    }

    static long pack(int x, int z) {
        return ((long) x << 32) | (z & 0xFFFFFFFFL);
    }

    static int unpackX(long v) { return (int) (v >> 32); }

    static int unpackZ(long v) { return (int) v; }

    // ------------------------------------------------------------------
    // Dump job (state machine scheduled tick-by-tick on the main thread)
    // ------------------------------------------------------------------

    private DumpJob job = null;

    private static final class DumpJob {
        final ServerLevel level;
        final Path labelDir;   // <out>/<label>
        final Path seedDir;    // <out>/<label>/seed_<seed>
        final long seed;
        final long[] coords;
        final boolean raw;     // session-4 RAW mode flag
        final Path rawDir;     // <out>/<label>/raw (non-null only when raw)
        final ChunkStatus stagedStatus; // task 5-a STAGED mode target (null = full/raw modes)
        final String stagedName;        // registry key of stagedStatus ("minecraft:noise"), null otherwise
        int next = 0;
        int ok = 0;
        int failed = 0;
        int probes = 0;
        BufferedWriter manifest;
        BufferedWriter probeWriter;    // raw/probes.tsv (raw mode only)
        BufferedWriter rawManifest;    // raw/manifest.tsv (raw mode only)

        DumpJob(ServerLevel level, Path labelDir, Path seedDir, long seed, long[] coords,
                boolean raw, Path rawDir, ChunkStatus stagedStatus) {
            this.level = level;
            this.labelDir = labelDir;
            this.seedDir = seedDir;
            this.seed = seed;
            this.coords = coords;
            this.raw = raw;
            this.rawDir = rawDir;
            this.stagedStatus = stagedStatus;
            this.stagedName = stagedStatus == null ? null
                    : BuiltInRegistries.CHUNK_STATUS.getKey(stagedStatus).toString();
        }
    }

    private void startDump(String label, long[] coords, boolean raw, ChunkStatus stagedStatus,
                           CommandSender ack) {
        if (job != null) throw new IllegalStateException("another dump is already running");

        ServerLevel level = mainServerLevel();

        // Output root: system property goldendump.out wins; default <serverdir>/golden.
        Path root;
        String prop = System.getProperty("goldendump.out");
        if (prop != null && !prop.isBlank()) {
            root = Paths.get(prop);
        } else {
            // getDataFolder() may be RELATIVE (e.g. "plugins/GoldenDumper") —
            // getParentFile() on the relative form loses the server dir
            // (parent == null) and silently killed the first corpus runs
            // (message reached the RCON client only). Resolve ABSOLUTE first.
            Path pluginsDir = getDataFolder().getAbsoluteFile().toPath().getParent();
            Path serverDir = pluginsDir == null ? null : pluginsDir.getParent();
            if (serverDir == null) {
                throw new IllegalStateException("cannot resolve server dir from " + pluginsDir
                        + " — set -Dgoldendump.out=<dir>");
            }
            root = serverDir.resolve("golden");
        }

        long seed = level.getSeed(); // verified: ServerLevel.getSeed() -> J (jar inspection 2026-10-05)
        Path labelDir = root.resolve(label);
        Path seedDir = labelDir.resolve("seed_" + seed);
        Path rawDir = labelDir.resolve("raw");
        try {
            Files.createDirectories(seedDir);
            if (raw) {
                Files.createDirectories(rawDir);
            }
        } catch (IOException e) {
            throw new IllegalStateException("cannot create output dir " + seedDir + ": " + e);
        }

        DumpJob j = new DumpJob(level, labelDir, seedDir, seed, coords, raw, rawDir, stagedStatus);
        try {
            writeMeta(j);
            j.manifest = Files.newBufferedWriter(labelDir.resolve("manifest.tsv"),
                    StandardCharsets.UTF_8, StandardOpenOption.CREATE, StandardOpenOption.TRUNCATE_EXISTING);
            if (stagedStatus != null) {
                j.manifest.write("# GoldenDumper staged manifest (NCF task 5-a — DECODED staged chunk contract)");
                j.manifest.newLine();
                j.manifest.write("# columns: chunkX<TAB>chunkZ<TAB>file<TAB>status");
                j.manifest.newLine();
            } else {
                j.manifest.write("# GoldenDumper manifest v1 (NCF P0.3)");
                j.manifest.newLine();
                j.manifest.write("# columns: chunkX<TAB>chunkZ<TAB>Status<TAB>DataVersion<TAB>file<TAB>bytes");
                j.manifest.newLine();
            }
            if (raw) {
                j.probeWriter = Files.newBufferedWriter(rawDir.resolve("probes.tsv"),
                        StandardCharsets.UTF_8, StandardOpenOption.CREATE, StandardOpenOption.TRUNCATE_EXISTING);
                j.probeWriter.write("# x,y,z,block");
                j.probeWriter.newLine();
                j.rawManifest = Files.newBufferedWriter(rawDir.resolve("manifest.tsv"),
                        StandardCharsets.UTF_8, StandardOpenOption.CREATE, StandardOpenOption.TRUNCATE_EXISTING);
                j.rawManifest.write("# GoldenDumper RAW manifest v1 (NCF P3.1 input — uncompressed NBT)");
                j.rawManifest.newLine();
                j.rawManifest.write("# columns: chunkX<TAB>chunkZ<TAB>Status<TAB>DataVersion<TAB>"
                        + "rawFile<TAB>rawBytes<TAB>compressedBytes");
                j.rawManifest.newLine();
            }
        } catch (IOException e) {
            throw new IllegalStateException("cannot open manifest/meta under " + labelDir + ": " + e);
        }
        job = j;
        // BUSY-FLAG FIX (task 5-a trap): `busy` was checked in onCommand since
        // session 1 but never SET — concurrent goldendump commands would have
        // interleaved manifest writes. Set/clear it around the job lifecycle.
        busy = true;
        if (stagedStatus != null) {
            ack.sendMessage("goldendump: start label=" + label + " mode=staged status=" + j.stagedName
                    + " chunks=" + coords.length + " seed=" + seed + " dir=" + labelDir);
            // T38-A: spawn block coords ride on the start marker — the gate
            // driver excludes the spawn-chunk square (Paper keeps it FULL,
            // a staged NOISE dump then reads full chunks) from both corpora.
            org.bukkit.Location sl = Bukkit.getWorlds().get(0).getSpawnLocation();
            getLogger().info("GOLDEN STAGED DUMP start label=" + label + " status=" + j.stagedName
                    + " chunks=" + coords.length + " seed=" + seed + " dir=" + labelDir
                    + " spawn=" + sl.getBlockX() + "," + sl.getBlockZ());
        } else {
            ack.sendMessage("goldendump: start label=" + label + " mode=" + (raw ? "raw" : "compressed")
                    + " chunks=" + coords.length + " seed=" + seed + " dir=" + labelDir);
            getLogger().info("GOLDEN DUMP start label=" + label + " mode=" + (raw ? "raw" : "compressed")
                    + " chunks=" + coords.length + " seed=" + seed + " dir=" + labelDir);
        }
        Bukkit.getScheduler().runTask(this, this::processTick);
    }

    /** World 0 of the server (overworld for standard server.properties). */
    private ServerLevel mainServerLevel() {
        World w = Bukkit.getWorlds().get(0);
        // CraftWorld.getHandle() -> ServerLevel (verified, jar inspection 2026-10-05)
        return ((CraftWorld) w).getHandle();
    }

    private void writeMeta(DumpJob j) {
        Properties meta = new Properties();
        meta.setProperty("seed", Long.toString(j.seed));
        meta.setProperty("timestamp", Instant.now().truncatedTo(ChronoUnit.SECONDS).toString());
        meta.setProperty("serverJar", serverJarPath());
        // Best-effort NMS accessors: 1.21.10 internals drift (and the plugin
        // remapper) can throw NoSuchMethodError at any single accessor —
        // meta.properties is auxiliary and must NEVER kill a dump. Each
        // accessor gets its own catch; failures are recorded in the file.
        try {
            // SharedConstants.getCurrentVersion() -> WorldVersion; dataVersion()
            // -> DataVersion; version() -> int (record accessors).
            meta.setProperty("dataVersion", Integer.toString(
                    SharedConstants.getCurrentVersion().dataVersion().version()));
        } catch (Throwable t) {
            meta.setProperty("dataVersion", "unavailable: " + t.getClass().getSimpleName());
        }
        try {
            meta.setProperty("levelName", j.level.getServer().getWorldData().getLevelName());
        } catch (Throwable t) {
            // NoSuchMethodError observed here pre-namespace-header (remapper);
            // Bukkit API fallback — the world name, which is what meta needs.
            try {
                meta.setProperty("levelName", Bukkit.getWorlds().get(0).getName());
            } catch (Throwable t2) {
                meta.setProperty("levelName", "unavailable: " + t.getClass().getSimpleName());
            }
        }
        try {
            meta.setProperty("dimension", j.level.dimension().location().toString());
        } catch (Throwable t) {
            meta.setProperty("dimension", "unavailable: " + t.getClass().getSimpleName());
        }
        if (j.stagedStatus != null) {
            // task 5-a staged-only metadata (contract inputs for the Rust gate)
            meta.setProperty("status", j.stagedName);
            meta.setProperty("worldMinY", Integer.toString(j.level.getMinY()));
            meta.setProperty("worldHeight", Integer.toString(j.level.getHeight()));
        }
        try (var out = Files.newBufferedWriter(j.seedDir.resolve("meta.properties"), StandardCharsets.UTF_8)) {
            meta.store(out, "GoldenDumper dump metadata (NCF P0.3); one file per dump+seed");
        } catch (IOException e) {
            getLogger().warning("GOLDEN DUMP meta.properties write failed: " + e);
        }
    }

    /** Best-effort server jar path (CraftServer lives inside the mojang-mapped server jar). */
    private static String serverJarPath() {
        try {
            CodeSource cs = CraftServer.class.getProtectionDomain().getCodeSource();
            return cs == null ? "unknown" : cs.getLocation().toString();
        } catch (Throwable t) {
            return "unknown (" + t.getClass().getSimpleName() + ")";
        }
    }

    /** One scheduler round: up to CHUNKS_PER_TICK chunks, then re-schedule or finish. */
    private void processTick() {
        DumpJob j = job;
        if (j == null) return;
        if (j.stagedStatus != null) {
            stagedTick(j);
            return;
        }
        int doneThisTick = 0;
        while (j.next < j.coords.length && doneThisTick < CHUNKS_PER_TICK) {
            long packed = j.coords[j.next++];
            doneThisTick++;
            int x = unpackX(packed);
            int z = unpackZ(packed);
            try {
                dumpOne(j, x, z);
                j.ok++;
            } catch (Throwable t) {
                // Per-chunk failure: log with coordinates, count, NEVER abort the run.
                j.failed++;
                getLogger().warning("GOLDEN DUMP FAILED chunk " + x + " " + z + ": " + t);
            }
        }
        int processed = j.next;
        if (processed % PROGRESS_EVERY == 0 || j.next >= j.coords.length) {
            getLogger().info("GOLDEN DUMP progress " + processed + "/" + j.coords.length
                    + " failed=" + j.failed);
        }
        if (j.next >= j.coords.length) {
            try { j.manifest.flush(); j.manifest.close(); } catch (IOException e) {
                getLogger().warning("GOLDEN DUMP manifest close failed: " + e);
            }
            if (j.raw) {
                try {
                    j.probeWriter.flush();
                    j.probeWriter.close();
                    j.rawManifest.flush();
                    j.rawManifest.close();
                } catch (IOException e) {
                    getLogger().warning("GOLDEN DUMP raw manifest/probes close failed: " + e);
                }
                // logged BEFORE the standard marker; raw drivers can grep this
                getLogger().info("GOLDEN DUMP RAW COMPLETE chunks=" + j.ok + " probes=" + j.probes
                        + " dir=" + j.rawDir);
            }
            job = null;
            busy = false;
            // Final marker line — the driver greps exactly this.
            getLogger().info("GOLDEN DUMP COMPLETE n=" + j.ok + " failed=" + j.failed + " dir=" + j.labelDir);
        } else {
            // T41: compressed/raw dumps pump WITHIN ONE TICK (runTask, same-tick
            // re-entry) — the pre-T37-b shape. Tick gaps here are a CORRECTNESS
            // bug, not a timing detail: across a 2-tick gap the world ticks
            // (chunk GC unloads/discards proto dependency chunks, ticker state
            // advances) and the corpus content becomes a function of the
            // gameTime at which the dump runs. Evidence: run 37560066762
            // golden-harness ORDER-TEST control (two pure-vanilla boots, same
            // seed/order) diverged 32/256 with feature-level diffs (a dirt
            // blob, a granite blob, a whole spruce tree present in one boot
            // and absent in the other) while boot B was byte-identical across
            // runs and boot A moved (LastUpdate 9 vs 23 for the same chunk);
            // per-chunk LastUpdate climbs inside one dump (9..61) proving the
            // dump spans real ticks since T37-b. The watchdog rationale of
            // T37-b does NOT apply here: compressed/raw dumps are <=256 chunks
            // (~25s inside one tick, well under the 60s no-tick kill; green
            // for weeks pre-T37-b). The staged path keeps runTaskLater(2) —
            // 2401-chunk gate-P2 dumps genuinely exceed the watchdog window.
            // INVARIANT: a compressed/raw plan must stay completable inside
            // one tick; larger corpora must use the staged path or smaller plans.
            Bukkit.getScheduler().runTask(this, this::processTick);
        }
    }

    /**
     * One STAGED scheduler round (task 5-a): up to CHUNKS_PER_TICK chunks,
     * each obtained by BLOCKING on getChunkFuture(x, z, status, true).join().
     * Paper/Moonrise already managedBlock()s inside getChunkFuture when called
     * on the main thread (scheduleChunkLoad(..., Priority.HIGHER) + main-thread
     * task pumping), so the worker pool progresses while the main thread waits
     * — the same shape the FULL path uses via getChunk(..., FULL, true). One
     * batch of 8 per tick keeps the tick loop alive ("join futures on
     * subsequent ticks": each batch is joined in its own scheduler round).
     */
    private void stagedTick(DumpJob j) {
        int doneThisTick = 0;
        while (j.next < j.coords.length && doneThisTick < CHUNKS_PER_TICK) {
            long packed = j.coords[j.next++];
            doneThisTick++;
            int x = unpackX(packed);
            int z = unpackZ(packed);
            try {
                dumpStagedOne(j, x, z);
                j.ok++;
            } catch (Throwable t) {
                // Per-chunk failure: log with coordinates, count, NEVER abort the run.
                j.failed++;
                getLogger().warning("GOLDEN STAGED DUMP FAILED chunk " + x + " " + z
                        + " status=" + j.stagedName + ": " + t);
            }
        }
        int processed = j.next;
        if (processed % PROGRESS_EVERY == 0 || j.next >= j.coords.length) {
            getLogger().info("GOLDEN STAGED DUMP progress " + processed + "/" + j.coords.length
                    + " failed=" + j.failed);
        }
        if (j.next >= j.coords.length) {
            try { j.manifest.flush(); j.manifest.close(); } catch (IOException e) {
                getLogger().warning("GOLDEN STAGED DUMP manifest close failed: " + e);
            }
            job = null;
            busy = false;
            // Final marker line — the STAGED driver greps exactly this prefix.
            getLogger().info("GOLDEN STAGED DUMP COMPLETE " + j.stagedName + " n=" + j.ok
                    + " failed=" + j.failed + " dir=" + j.labelDir);
        } else {
            // T37-b: schedule 2 ticks ahead so the current tick can COMPLETE.
            // Evidence (runs 37544917131 vs 37549651622): the whole dump runs
            // inside one tick — managedBlock() pumps scheduler rounds without
            // closing the tick — and Paper's watchdog (org.spigotmc
            // .WatchdogThread) kills after ~60s of no-tick REGARDLESS of
            // server.properties max-tick-time=-1. runTask (next-scheduler-
            // round) was re-entering the same tick; runTaskLater(2) is not
            // due until the tick loop advances. Dump cost: ~2 ticks/chunk.
            Bukkit.getScheduler().runTaskLater(this, this::processTick, 2);
        }
    }

    /**
     * Generate one chunk UP TO the job's ChunkStatus and write its DECODED NBT.
     * Blocks the calling (main) thread until the chunk reaches the status.
     */
    private void dumpStagedOne(DumpJob j, int x, int z) throws IOException {
        CompletableFuture<ChunkResult<ChunkAccess>> future =
                j.level.getChunkSource().getChunkFuture(x, z, j.stagedStatus, true);
        ChunkResult<ChunkAccess> res = future.join();
        ChunkAccess chunk = res.isSuccess() ? res.orElse(null) : null;
        if (chunk == null) {
            throw new IOException("chunk not available at " + j.stagedName + ": " + res.getError());
        }
        CompoundTag tag = buildStagedTag(j, chunk);
        Path file = j.seedDir.resolve("c_" + x + "_" + z + ".nbt");
        NbtIo.writeCompressed(tag, file);
        long bytes = Files.size(file);
        try {
            j.manifest.write(x + "\t" + z + "\t" + "seed_" + j.seed + "/c_" + x + "_" + z + ".nbt"
                    + "\t" + j.stagedName);
            j.manifest.newLine();
        } catch (IOException e) {
            throw new IOException("staged manifest append failed: " + e, e);
        }
    }

    /**
     * DECODED staged chunk contract (task 5-a) — see the class header for the
     * full schema and the VERIFIED index orders. Everything here runs on the
     * main server thread (chunk access is not thread-safe).
     */
    private static CompoundTag buildStagedTag(DumpJob j, ChunkAccess chunk) {
        ServerLevel level = j.level;
        CompoundTag root = new CompoundTag();
        root.putInt("ChunkX", chunk.getPos().x);
        root.putInt("ChunkZ", chunk.getPos().z);
        root.putString("Status", j.stagedName);
        root.putInt("DataVersion", SharedConstants.getCurrentVersion().dataVersion().version());
        root.putInt("MinY", level.getMinY());
        root.putInt("Height", level.getHeight());

        LevelChunkSection[] sections = chunk.getSections();
        int minSectionY = chunk.getMinSectionY();

        // ---------------- Sections (blocks) + Biomes ----------------
        // Index orders VERIFIED against the mojang-mapped jar decompile
        // (CFR 0.152, 2026-10-05), net.minecraft.world.level.chunk.Strategy:
        //   getIndex(x, y, z) = (y << bitsPerAxis | z) << bitsPerAxis | x;
        //   blocks  bitsPerAxis = 4 -> i  = sy*256 + sz*16 + sx (4096 entries)
        //   biomes  bitsPerAxis = 2 -> q  = by*16  + bz*4  + bx (  64 entries)
        ListTag sectionsTag = new ListTag();
        ListTag biomesTag = new ListTag();
        Map<BlockState, Integer> blockPaletteIndex = new HashMap<>();
        Map<String, Integer> biomePaletteIndex = new HashMap<>();
        for (int s = 0; s < sections.length; s++) {
            LevelChunkSection sec = sections[s];
            int secY = minSectionY + s;

            // blocks: encounter-order palette (scan order == index order),
            // so the palette order is deterministic for a fixed content.
            ListTag palette = new ListTag();
            int[] data = new int[4096];
            blockPaletteIndex.clear();
            for (int i = 0; i < 4096; i++) {
                int sy = i >> 8, sz = (i >> 4) & 15, sx = i & 15; // i = sy*256+sz*16+sx
                BlockState st = sec.getBlockState(sx, sy, sz);
                Integer idx = blockPaletteIndex.get(st);
                if (idx == null) {
                    // EXACT vanilla palette entry form (same compound the
                    // vanilla .mca palettes carry): {Name, Properties{...}}
                    palette.add(NbtUtils.writeBlockState(st));
                    idx = palette.size() - 1;
                    blockPaletteIndex.put(st, idx);
                }
                data[i] = idx;
            }
            CompoundTag sTag = new CompoundTag();
            sTag.putByte("Y", (byte) secY);
            sTag.put("Palette", palette);
            sTag.put("Data", new IntArrayTag(data));
            sectionsTag.add(sTag);

            // biomes: same scan discipline over 4x4x4 quart space.
            ListTag biomePalette = new ListTag();
            int[] biomeData = new int[64];
            biomePaletteIndex.clear();
            for (int q = 0; q < 64; q++) {
                int by = q >> 4, bz = (q >> 2) & 3, bx = q & 3; // q = by*16+bz*4+bx
                Holder<Biome> h = sec.getNoiseBiome(bx, by, bz);
                // The biome strategy holds Reference holders (registry-backed,
                // see PalettedContainerFactory.create(): registry.asHolderIdMap()),
                // so unwrapKey() is always present in practice. If it ever is
                // not, FAIL THE CHUNK (throw) — a silent "unknown" biome in a
                // golden oracle would be worse than a counted failure.
                ResourceLocation biomeLoc = h.unwrapKey()
                        .map(k -> k.location())
                        .orElseThrow(() -> new IllegalStateException(
                                "staged dump: biome holder without registry key at " + secY));
                String id = biomeLoc.toString();
                Integer idx = biomePaletteIndex.get(id);
                if (idx == null) {
                    biomePalette.add(net.minecraft.nbt.StringTag.valueOf(id));
                    idx = biomePalette.size() - 1;
                    biomePaletteIndex.put(id, idx);
                }
                biomeData[q] = idx;
            }
            CompoundTag bTag = new CompoundTag();
            bTag.putByte("Y", (byte) secY);
            bTag.put("Palette", biomePalette);
            bTag.put("Data", new IntArrayTag(biomeData));
            biomesTag.add(bTag);
        }
        root.put("Sections", sectionsTag);
        root.put("Biomes", biomesTag);

        // ---------------- Heightmaps (raw, NOT decoded) ----------------
        // Mirror SerializableChunkData.copyOf exactly: only the types tracked
        // by the chunk's CURRENT persisted status (heightmapsAfter), raw long[]
        // clones via Heightmap.getRawData(). At NOISE/SURFACE these are
        // WORLD_SURFACE_WG + OCEAN_FLOOR_WG (see class header priming facts).
        CompoundTag hms = new CompoundTag();
        for (Map.Entry<Heightmap.Types, Heightmap> e : chunk.getHeightmaps()) {
            if (!chunk.getPersistedStatus().heightmapsAfter().contains(e.getKey())) continue;
            hms.put(e.getKey().getSerializationKey(), new LongArrayTag(e.getValue().getRawData().clone()));
        }
        if (!hms.isEmpty()) {
            root.put("Heightmaps", hms);
        }

        // ---------------- ticks (only when non-empty) ----------------
        // SavedTick codec shape == vanilla block_ticks/fluid_ticks ({i,t,p}).
        // Empty at NOISE/SURFACE in practice; guarded try/catch so a codec
        // linkage surprise can never kill a dump (omitted + warned).
        try {
            ChunkAccess.PackedTicks ticks = chunk.getTicksForSerialization(level.getGameTime());
            if (ticks != null && !ticks.blocks().isEmpty()) {
                root.store("block_ticks", SavedTick.codec(BuiltInRegistries.BLOCK.byNameCodec()).listOf(), ticks.blocks());
            }
            if (ticks != null && !ticks.fluids().isEmpty()) {
                root.store("fluid_ticks", SavedTick.codec(BuiltInRegistries.FLUID.byNameCodec()).listOf(), ticks.fluids());
            }
        } catch (Throwable t) {
            java.util.logging.Logger log = java.util.logging.Logger.getLogger("GoldenDumper");
            log.warning("GOLDEN STAGED DUMP ticks serialization skipped: " + t);
        }

        // ---------------- PostProcessing (vanilla packOffsets shape) -----
        // List of 24 inner lists of Short (one per section, empty included);
        // mirrors SerializableChunkData.packOffsets exactly. NON-EMPTY at
        // NOISE: aquifer fluid updates mark positions during doFill.
        ShortList[] postProcessing = chunk.getPostProcessing();
        ListTag ppTag = new ListTag();
        for (ShortList list : postProcessing) {
            ListTag inner = new ListTag();
            if (list != null) {
                for (int i = 0; i < list.size(); i++) {
                    inner.add(ShortTag.valueOf(list.getShort(i)));
                }
            }
            ppTag.add(inner);
        }
        root.put("PostProcessing", ppTag);

        return root;
    }

    /**
     * Force FULL generation of one chunk and write its canonical NBT.
     * Everything here must run on the main server thread.
     */
    private void dumpOne(DumpJob j, int x, int z) throws IOException {
        ServerLevel level = j.level;

        // (a) Force full generation / load. getChunk(x, z, ChunkStatus.FULL, true) is the
        // standard vanilla force path: blocks the main thread until the chunk pipeline
        // reaches FULL (ticking) state. Verified signature (jar inspection 2026-10-05):
        //   ServerChunkCache.getChunk(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Lnet/minecraft/world/level/chunk/ChunkAccess;
        // (1.21.x package: net.minecraft.world.level.chunk.status.ChunkStatus — confirmed.)
        net.minecraft.world.level.chunk.ChunkAccess access =
                level.getChunkSource().getChunk(x, z, ChunkStatus.FULL, true);

        // For FULL status the instance is always LevelChunk (vanilla guarantee);
        // if Paper ever returns a ProtoChunk here the cast fails into the
        // per-chunk failure path (counted, logged, run continues).
        LevelChunk chunk = (LevelChunk) access;

        // (b) Serialize. 1.21.10 renamed the old ChunkSerializer flow:
        //   ChunkSerializer.write(level, poiManager, chunk)
        // became the Record SerializableChunkData with static copyOf(ServerLevel, ChunkAccess)
        // and instance write() -> CompoundTag. Both verified by jar inspection 2026-10-05
        // (descriptors dumped from the mojang-mapped server jar). The POI manager is no
        // longer a parameter — POI data lives outside the chunk payload.
        // VERIFY-1.21.10 (runtime): byte layout vs .mca region payload (container framing)
        // must be checked once the first real corpus dump lands — see GOLDEN_HARNESS.md.
        CompoundTag tag = SerializableChunkData.copyOf(level, chunk).write();

        // (c) Write gzipped NBT. Verified: NbtIo.writeCompressed(CompoundTag, Path) (public static).
        Path file = j.seedDir.resolve("c_" + x + "_" + z + ".nbt");
        NbtIo.writeCompressed(tag, file);
        long bytes = Files.size(file);

        // (d) Manifest row. Status/DataVersion read from the SERIALIZED tag so the
        // manifest reflects file contents, not in-memory state.
        // CompoundTag.getString/getInt return Optional in 1.21.10 (verified).
        String status = tag.getString("Status").orElseGet(() ->
                chunk.getPersistedStatus().toString());
        int dataVersion = tag.getInt("DataVersion").orElse(-1);
        String relFile = "seed_" + j.seed + "/c_" + x + "_" + z + ".nbt";
        try {
            j.manifest.write(x + "\t" + z + "\t" + status + "\t" + dataVersion + "\t" + relFile + "\t" + bytes);
            j.manifest.newLine();
        } catch (IOException e) {
            throw new IOException("manifest append failed: " + e, e);
        }

        // (e) session-4 RAW mode extras (P3.1 input). Runs in addition to (b)-(d)
        // so the compressed artifacts stay byte-identical to the default mode.
        if (j.raw) {
            // (e1) UNCOMPRESSED NBT: NbtIo.write(CompoundTag, DataOutput) — the
            // DataOutput overload writes UNCOMPRESSED (javap-verified 2026-10-05).
            // The tag is used EXACTLY as SerializableChunkData produced it
            // (DataVersion untouched).
            Path rawFile = j.rawDir.resolve("chunk." + x + "." + z + ".nbt");
            try (DataOutputStream out = new DataOutputStream(
                    new BufferedOutputStream(Files.newOutputStream(rawFile)))) {
                NbtIo.write(tag, out);
            }
            long rawBytes = Files.size(rawFile);

            // (e2) probes: two deterministic columns per chunk, top-down first non-air.
            writeProbe(j, level, chunk, (x << 4) + 3, (z << 4) + 5);
            writeProbe(j, level, chunk, (x << 4) + 10, (z << 4) + 12);

            // (e3) raw manifest row (separate file; the v1 manifest.tsv above is
            // backward compatible and untouched in shape).
            j.rawManifest.write(x + "\t" + z + "\t" + status + "\t" + dataVersion + "\t"
                    + "chunk." + x + "." + z + ".nbt\t" + rawBytes + "\t" + bytes);
            j.rawManifest.newLine();
        }
    }

    /** One probes.tsv row: top-down scan for the first non-air block state. */
    private static void writeProbe(DumpJob j, ServerLevel level, LevelChunk chunk,
                                   int px, int pz) throws IOException {
        String found = null;
        int foundY = level.getMinY();
        for (int y = level.getMaxY() - 1; y >= level.getMinY(); y--) {
            BlockState st = chunk.getBlockState(new BlockPos(px, y, pz));
            // Skip random-tickable blocks (mushrooms/saplings/crops...): the
            // probe is executed on a LATER boot after forceload, and a random
            // tick between load and probe would pop them (observed: a brown
            // mushroom vanished on boot 2 — 17/18 PASS became 17 probes with
            // one env-flake). A non-ticking block is stable end to end.
            if (!st.isAir() && !st.isRandomlyTicking()) {
                foundY = y;
                // BuiltInRegistries.BLOCK is a static DefaultedRegistry —
                // stable linkage (the RegistryAccess.lookupOrThrow overload
                // set failed to link at runtime, see VectorCapture header).
                found = net.minecraft.core.registries.BuiltInRegistries.BLOCK
                        .getKey(st.getBlock()).toString();
                break;
            }
        }
        String row = px + "," + foundY + "," + pz + "," + (found != null ? found : "minecraft:air");
        j.probeWriter.write(row);
        j.probeWriter.newLine();
        j.probes++;
    }

    @Override
    public void onDisable() {
        DumpJob j = job;
        if (j != null) {
            getLogger().warning("GOLDEN DUMP ABORTED (plugin disable) processed=" + j.next
                    + "/" + j.coords.length + " failed=" + j.failed + " dir=" + j.labelDir);
            job = null;
            busy = false;
        }
    }
}
