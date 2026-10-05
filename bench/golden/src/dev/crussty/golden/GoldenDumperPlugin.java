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
import java.util.List;
import java.util.Properties;

import net.minecraft.SharedConstants;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Registry;
import net.minecraft.core.registries.Registries;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.nbt.NbtIo;
import net.minecraft.resources.ResourceLocation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.chunk.LevelChunk;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.chunk.storage.SerializableChunkData;

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
                    + "|raw <chunkX> <chunkZ> <radius>|manifest <file>)");
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
        final long[] coords;
        try {
            if (args.length == 5 && args[1].equalsIgnoreCase("raw")) {
                // session-4 RAW mode, labeled form: /goldendump <label> raw <cx> <cz> <r>
                raw = true;
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
                int cx = Integer.parseInt(args[1]);
                int cz = Integer.parseInt(args[2]);
                int r = Integer.parseInt(args[3]);
                requireRadius(r);
                coords = squareCoords(cx, cz, r);
            } else if (args.length == 4 && !args[1].equalsIgnoreCase("manifest")) {
                raw = false;
                int cx = Integer.parseInt(args[1]);
                int cz = Integer.parseInt(args[2]);
                int r = Integer.parseInt(args[3]);
                requireRadius(r);
                coords = squareCoords(cx, cz, r);
            } else if (args.length == 3 && args[1].equalsIgnoreCase("manifest")) {
                raw = false;
                coords = readManifest(Paths.get(args[2]));
            } else {
                sender.sendMessage("usage: /goldendump <label> (<chunkX> <chunkZ> <radius>"
                        + "|raw <chunkX> <chunkZ> <radius>|manifest <file>)");
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
            startDump(outLabel, coords, raw, sender);
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
        int next = 0;
        int ok = 0;
        int failed = 0;
        int probes = 0;
        BufferedWriter manifest;
        BufferedWriter probeWriter;    // raw/probes.tsv (raw mode only)
        BufferedWriter rawManifest;    // raw/manifest.tsv (raw mode only)

        DumpJob(ServerLevel level, Path labelDir, Path seedDir, long seed, long[] coords,
                boolean raw, Path rawDir) {
            this.level = level;
            this.labelDir = labelDir;
            this.seedDir = seedDir;
            this.seed = seed;
            this.coords = coords;
            this.raw = raw;
            this.rawDir = rawDir;
        }
    }

    private void startDump(String label, long[] coords, boolean raw, CommandSender ack) {
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

        DumpJob j = new DumpJob(level, labelDir, seedDir, seed, coords, raw, rawDir);
        try {
            writeMeta(j);
            j.manifest = Files.newBufferedWriter(labelDir.resolve("manifest.tsv"),
                    StandardCharsets.UTF_8, StandardOpenOption.CREATE, StandardOpenOption.TRUNCATE_EXISTING);
            j.manifest.write("# GoldenDumper manifest v1 (NCF P0.3)");
            j.manifest.newLine();
            j.manifest.write("# columns: chunkX<TAB>chunkZ<TAB>Status<TAB>DataVersion<TAB>file<TAB>bytes");
            j.manifest.newLine();
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
        ack.sendMessage("goldendump: start label=" + label + " mode=" + (raw ? "raw" : "compressed")
                + " chunks=" + coords.length + " seed=" + seed + " dir=" + labelDir);
        getLogger().info("GOLDEN DUMP start label=" + label + " mode=" + (raw ? "raw" : "compressed")
                + " chunks=" + coords.length + " seed=" + seed + " dir=" + labelDir);
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
            // Final marker line — the driver greps exactly this.
            getLogger().info("GOLDEN DUMP COMPLETE n=" + j.ok + " failed=" + j.failed + " dir=" + j.labelDir);
        } else {
            Bukkit.getScheduler().runTask(this, this::processTick);
        }
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
        }
    }
}
