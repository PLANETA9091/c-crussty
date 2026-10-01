package bench.dimforceload;

import java.io.File;

import org.bukkit.Bukkit;
import org.bukkit.World;
import org.bukkit.command.Command;
import org.bukkit.command.CommandSender;
import org.bukkit.plugin.Plugin;
import org.bukkit.plugin.java.JavaPlugin;

/**
 * DimForceload — BENCH-V2 fixture plugin (AG-248, wave-515; v3 pregen redesign AG-496, wave-522).
 *
 * HISTORY:
 *   v1 (AG-93/248): one-shot addPluginChunkTicket for ALL cells — main-thread sync
 *       loads >60s => watchdog kills server (blocker #14).
 *   v2 (AG-301): amortized marking, <=DIM_MARK_BATCH ticket-adds/tick. addPluginChunkTicket
 *       STILL sync-loads fresh chunks on main (0.4-5s/chunk under jigsaw worldgen) =>
 *       watchdog OR DRAIN-TIMEOUT with marked=0/58272 (canary-4 RED ×2, wave-521).
 *       async-ticket-in-callback v1 (getChunkAtAsync -> addPluginChunkTicket inside
 *       whenComplete) = silent no-op: marked=0/58272 3/3 (runs 36817518129/36817520384/
 *       36817796398; AG-473/477/496/499) — REFUTED, do not resurrect.
 *
 * v3 PREGEN REDESIGN (this file; AG-496 wave-522, chain AG-459/463/473/496):
 *   1. GEN phase (off-main): for every cell fire world.getChunkAtAsync(x, z) —
 *      Paper generates/loads the chunk on worker threads, main thread never
 *      sync-loads (watchdog-safe by construction). GEN-ERR telemetry logs every
 *      failed future (AG-475: old whenComplete chains swallowed errors).
 *   2. MARK phase (register-only): a poller task every 20 ticks registers
 *      addPluginChunkTicket ONLY for cells already loaded (isChunkLoaded) —
 *      a ticket on a loaded chunk never sync-loads. Unloaded cells are retried
 *      next poll (no unbounded futures, no callback starvation).
 *   3. TELEMETRY (AG-475 markers, grep-able from server-stdout.log):
 *        [DF] WAIT      — start-file seen, GEN firing
 *        [DF] GEN-START — per world, cells=N
 *        [DF] PROGRESS  — every 10s: world=.. marked=X/Y loaded=Z
 *        [DF] GEN-ERR   — per failed future with exception class+message
 *        [DF] Marked N chunks world=.. — v2-compatible line (report parser keeps working)
 *        [DF] GEN-DONE  — all worlds marked, elapsed seconds
 *   4. "/dimchunks" G-DIM census unchanged.
 *
 * Back-compat: env DIM_MARK_MODE=legacy restores exact v2 amortized-sync behavior
 * (rollback switch for A/B isolation). Default mode = pregen.
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
            return 128; // v2 legacy mode only
        }
        try {
            return Math.max(1, Integer.parseInt(v.trim()));
        } catch (NumberFormatException e) {
            return 128;
        }
    }

    private boolean pregenMode() {
        String v = System.getenv("DIM_MARK_MODE");
        return v == null || v.isEmpty() || !v.equalsIgnoreCase("legacy");
    }

    @Override
    public void onEnable() {
        if (pregenMode()) {
            armPregen();
        } else {
            armLegacy();
        }
    }

    // ------------------------------------------------------------------ v3 ---
    private void armPregen() {
        final java.util.Set<String> worlds = enabledWorlds();
        final int r = radiusChunks();
        final int side = 2 * r + 1;
        final int total = side * side;
        final long armTs = System.currentTimeMillis();
        getLogger().info("[DF] armed mode=pregen-v3 worlds=" + worlds + " radius_chunks=" + r
                + " cells_per_world=" + total);
        final java.util.Set<String> genFired = new java.util.HashSet<>();
        // per-world set of cells not yet ticket-marked (removed as they are registered)
        final java.util.Map<String, java.util.Deque<int[]>> pending = new java.util.HashMap<>();
        final java.util.Map<String, Integer> marked = new java.util.HashMap<>();
        final java.util.Set<String> announced = new java.util.HashSet<>();
        final long[] pollCount = {0};

        Bukkit.getScheduler().runTaskTimer(this, () -> {
            if (!new File(START_FILE).exists()) {
                return; // harness opens the GEN window by touching dimload.start
            }
            // ---- fire GEN futures once (main thread: just scheduling, no IO) ----
            for (World w : Bukkit.getWorlds()) {
                if (!worlds.contains(w.getName()) || !genFired.add(w.getName())) {
                    continue;
                }
                getLogger().info("[DF] WAIT start-file seen — firing GEN t=+"
                        + ((System.currentTimeMillis() - armTs) / 1000) + "s");
                java.util.Deque<int[]> q = new java.util.ArrayDeque<>();
                for (int i = 0; i < total; i++) {
                    final int x = (i / side) - r;
                    final int z = (i % side) - r;
                    q.add(new int[]{x, z});
                    final String wn = w.getName();
                    // getChunkAtAsync: generation + load on worker threads.
                    // AG-475 telemetry: whenComplete MUST NOT swallow errors.
                    w.getChunkAtAsync(x, z).whenComplete((ch, ex) -> {
                        if (ex != null) {
                            getLogger().warning("[DF] GEN-ERR world=" + wn
                                    + " x=" + x + " z=" + z
                                    + " err=" + ex.getClass().getName() + ": " + ex.getMessage());
                        }
                    });
                }
                pending.put(w.getName(), q);
                marked.put(w.getName(), 0);
                getLogger().info("[DF] GEN-START world=" + w.getName() + " cells=" + total);
            }
            // ---- MARK phase: register-only tickets on already-loaded cells ----
            for (World w : Bukkit.getWorlds()) {
                final String name = w.getName();
                if (!worlds.contains(name)) {
                    continue;
                }
                java.util.Deque<int[]> q = pending.get(name);
                if (q == null || q.isEmpty()) {
                    continue;
                }
                int m = marked.getOrDefault(name, 0);
                int before = q.size();
                for (int tries = 0; tries < before; tries++) {
                    int[] c = q.pollFirst();
                    if (w.isChunkLoaded(c[0], c[1])) {
                        w.addPluginChunkTicket(c[0], c[1], (Plugin) this); // loaded => NO sync load
                        m++;
                    } else {
                        q.addLast(c); // still generating — retry next poll
                    }
                }
                marked.put(name, m);
                if ((pollCount[0]++ % 2) == 0 || q.isEmpty()) { // ~every 10s of 20t-polls... poll is 10t => /1
                    getLogger().info("[DF] PROGRESS world=" + name + " marked=" + m + "/" + total
                            + " loaded=" + w.getLoadedChunks().length);
                }
                if (m >= total && announced.add(name)) {
                    // v2-compatible marked emitter (report_benchv2.py parser)
                    getLogger().info("[DimForceload] Marked " + total + " chunks world=" + name);
                }
            }
            boolean allDone = marked.size() >= worlds.size()
                    && marked.values().stream().allMatch(v -> v >= total);
            if (allDone && !announced.contains("__done__")) {
                announced.add("__done__");
                getLogger().info("[DF] GEN-DONE all_marked=" + (total * worlds.size())
                        + " elapsed=" + ((System.currentTimeMillis() - armTs) / 1000) + "s");
            }
        }, 40L, 10L);
    }

    // ------------------------------------------------------------------ v2 ---
    /** Exact legacy v2 behavior (AG-301 amortized sync marking) — rollback switch. */
    private void armLegacy() {
        final java.util.Set<String> worlds = enabledWorlds();
        final int r = radiusChunks();
        final int side = 2 * r + 1;
        final int total = side * side;
        final int batch = batchPerTick();
        final java.util.Map<String, Integer> cursor = new java.util.HashMap<>();
        final java.util.Set<String> announced = new java.util.HashSet<>();
        getLogger().info("[DF] armed mode=legacy-v2 worlds=" + worlds + " radius_chunks=" + r
                + " batch=" + batch);
        Bukkit.getScheduler().runTaskTimer(this, () -> {
            if (!new File(START_FILE).exists()) {
                return;
            }
            for (World w : Bukkit.getWorlds()) {
                if (!worlds.contains(w.getName())) {
                    continue;
                }
                final String name = w.getName();
                final int done = cursor.getOrDefault(name, 0);
                if (done >= total) {
                    continue;
                }
                final int end = Math.min(total, done + batch);
                for (int i = done; i < end; i++) {
                    final int x = (i / side) - r;
                    final int z = (i % side) - r;
                    w.addPluginChunkTicket(x, z, (Plugin) this);
                }
                cursor.put(name, end);
                if (end >= total && announced.add(name)) {
                    getLogger().info("[DimForceload] Marked " + total + " chunks world=" + name);
                }
            }
        }, 40L, 10L);
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
