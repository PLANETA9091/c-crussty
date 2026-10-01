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
 *   - Every 60 ticks, IF the start-file "dimload.start" exists in the server
 *     working dir, add plugin chunk tickets for the square region
 *     [-R..R]x[-R..R] chunks (R from env DIM_RADIUS_CHUNKS, default 71 =>
 *     143x143 = 20,449 chunks/dim) in every world named in env
 *     DIM_WORLDS (default "world_nether,world_end"). Re-asserting an existing
 *     plugin ticket is idempotent, so the loop is a cheap keep-alive.
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
            v = "world_nether,world_end";
        }
        return new java.util.HashSet<>(java.util.Arrays.asList(v.split(",")));
    }

    @Override
    public void onEnable() {
        final java.util.Set<String> worlds = enabledWorlds();
        final int r = radiusChunks();
        Bukkit.getScheduler().runTaskTimer(this, () -> {
            if (!new File(START_FILE).exists()) {
                return; // harness opens the GEN window by touching dimload.start
            }
            for (World w : Bukkit.getWorlds()) {
                if (!worlds.contains(w.getName())) {
                    continue;
                }
                for (int x = -r; x <= r; x++) {
                    for (int z = -r; z <= r; z++) {
                        w.addPluginChunkTicket(x, z, (Plugin) this);
                    }
                }
            }
        }, 40L, 60L);
        getLogger().info("[DimForceload] armed worlds=" + worlds + " radius_chunks=" + r
                + " start_file=" + START_FILE);
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
