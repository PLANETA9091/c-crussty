package bench.dimforceload;

import java.io.File;

import org.bukkit.Bukkit;
import org.bukkit.World;
import org.bukkit.command.Command;
import org.bukkit.command.CommandSender;
import org.bukkit.plugin.Plugin;
import org.bukkit.plugin.java.JavaPlugin;

/**
 * DimForceload — BENCH-V2 fixture plugin (AG-248, wave-515).
 *
 * WHY THIS EXISTS (fixture-integrity audit AG-248, finding F1):
 *   vanilla /forceload is Overworld-only; "execute in the_nether run forceload
 *   add ..." fails and the bench harness cmd() swallows errors — nether/end
 *   silently load 0 chunks and the "61,347 chunks in 3 dims" stand collapses
 *   to overworld-only with NO gate noticing. Cross-dimension chunk tickets on
 *   Paper/Purpur require a plugin (Bukkit World.addPluginChunkTicket).
 *
 * WHAT IT DOES:
 *   - Every 10 ticks, IF the start-file "dimload.start" exists in the server
 *     working dir, add plugin chunk tickets for the square region
 *     [-R..R]x[-R..R] chunks (R from env DIM_RADIUS_CHUNKS, default 71 =>
 *     143x143 = 20,449 chunks/dim) in every world named in env
 *     DIM_WORLDS (default "world_nether,world_the_end").
 *   - BATCHED MARKING (blocker #14 fix, AG-301 wave-520): CraftWorld
 *     .addPluginChunkTicket(x,z,plugin) SYNC-LOADS the chunk on the main
 *     thread (CraftWorld.java:582 -> getChunkAt -> ServerChunkCache.syncLoad
 *     -> managedBlock). Adding all 20449x3 tickets in ONE tick hangs the
 *     Server thread >60 s => Paper watchdog dumps + kills the server ~2 min
 *     after boot (evidence: vallegs run-36805200406 / run-36805421425,
 *     "Server thread dump" x13, Stopping server at 02:25:12). So marking is
 *     amortized: <= dimload.batch (default 128) ticket-adds per invocation,
 *     per-world cursor, ~160 invocations to drain 20449 => worst case ~1.3 s
 *     of sync loads per tick, no watchdog hit.
 *   - MARKED EMITTER (blocker #12 fix, AG-301; credit AG-70 for the design):
 *     when a world's cursor completes, exactly ONE line
 *     "[DimForceload] Marked 20449 chunks world=<name>" is logged; repeated
 *     invocations are NOT re-announced, so report_benchv2.py
 *     sum("Marked (d+) chunks") == 3 x 20449 = 61,347 >= 58,272 (G4 PASS).
 *     Keep-alive re-asserts are skipped once a world is fully marked
 *     (idempotent anyway, and cheaper than the old re-add-every-60t loop).
 *   - Command "/dimchunks": prints one G-DIM line per world with the loaded
 *     chunk count, e.g. "[DimForceload] G-DIM world=world_nether loaded=20449"
 *     — the bash fixture gate parses these (no silent empty dims possible).
 *
 * The start-file gate keeps chunk GENERATION out of the boot window so the
 * harness can timestamp GEN FIRST_TS exactly (ch/s drain-definition).
 *
 * NOT FOR PRODUCTION. Bench harness only (bench-v2 CI, sanctioned boots).
 */
public final class DimForceloadPlugin extends JavaPlugin {

    static final String START_FILE = "dimload.start";

    private int radiusChunks() {
        String v = System.getenv("DIM_RADIUS_CHUNKS");
        if (v == null || v.isEmpty()) {
            return 71; // 1136 blocks / 16 + center => 143x143 = 20,449 chunks
        }
        try {
            return Math.max(1, Integer.parseInt(v.trim()));
        } catch (NumberFormatException e) {
            return 71;
        }
    }

    private java.util.Set<String> enabledWorlds() {
        String v = System.getenv("DIM_WORLDS");
        if (v == null || v.isEmpty()) {
            v = "world_nether,world_the_end";
        }
        return new java.util.HashSet<>(java.util.Arrays.asList(v.split(",")));
    }

    private int batchPerTick() {
        String v = System.getenv("DIM_MARK_BATCH");
        if (v == null || v.isEmpty()) {
            return 128; // amortize sync chunk-loads, keep tick << watchdog 60 s
        }
        try {
            return Math.max(1, Integer.parseInt(v.trim()));
        } catch (NumberFormatException e) {
            return 128;
        }
    }

    @Override
    public void onEnable() {
        final java.util.Set<String> worlds = enabledWorlds();
        final int r = radiusChunks();
        final int side = 2 * r + 1;
        final int total = side * side;
        final int batch = batchPerTick();
        // per-world marking cursor: next linear index in row-major (x-major) order
        final java.util.Map<String, Integer> cursor = new java.util.HashMap<>();
        final java.util.Set<String> announced = new java.util.HashSet<>();
        Bukkit.getScheduler().runTaskTimer(this, () -> {
            if (!new File(START_FILE).exists()) {
                return; // harness opens the GEN window by touching dimload.start
            }
            for (World w : Bukkit.getWorlds()) {
                if (!worlds.contains(w.getName())) {
                    continue;
                }
                final String name = w.getName();
                final int done = cursor.getOrDefault(name, 0);
                if (done >= total) {
                    continue; // fully marked — cheap keep-alive skip (idempotent)
                }
                final int end = Math.min(total, done + batch);
                for (int i = done; i < end; i++) {
                    final int x = (i / side) - r;
                    final int z = (i % side) - r;
                    w.addPluginChunkTicket(x, z, (Plugin) this);
                }
                cursor.put(name, end);
                if (end >= total && announced.add(name)) {
                    // blocker #12: exactly one Marked line per world, ever
                    getLogger().info("[DimForceload] Marked " + total
                            + " chunks world=" + name);
                }
            }
        }, 40L, 10L);
        getLogger().info("[DimForceload] armed worlds=" + worlds + " radius_chunks=" + r
                + " batch=" + batch + " start_file=" + START_FILE);
    }

    @Override
    public boolean onCommand(CommandSender sender, Command command, String label, String[] args) {
        for (World w : Bukkit.getWorlds()) {
            getLogger().info("[DimForceload] G-DIM world=" + w.getName()
                    + " loaded=" + w.getLoadedChunks().length);
        }
        return true;
    }
}
