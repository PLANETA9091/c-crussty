package bench.population;

// ============================================================================
// BenchPopulationPlugin — BENCH-ONLY population-injection fixture (S7-128,
// BENCH-X150K era, docs/BENCH_X150K_SCENARIO.md).
//
// Owner directive (2026-09-17 ~21:10-21:15 +08): the scene is the REAL prime
// — all force-chunks + everything ALIVE (spawn AND despawn lanes running
// "as if players are present"), target 20 TPS / minimal MSPT. The working
// unit hypothesis: x150000 ~= ~150k living entities on the real server.
//
// This plugin is a FIXTURE (scenario), not a gameplay change and not a
// config win: the server runs its own 100% unmodified spawn/despawn/AI/
// item-merge/item-despawn code paths. Injection uses the vanilla addEntity
// path (World.spawnEntity / World.dropItem) and is triggered by the bench
// harness via console (`benchpop inject <target> <seed>`) AFTER the forceload
// sweep and BEFORE the profiler window starts (harness waits for the DONE
// marker) — so the measured window always sees the full living scene.
//
// Determinism: a single java.util.Random(seed) drives type mix, placement
// and cluster selection; loaded chunks are visited in sorted (x,z) order, so
// identical (target, seed, loaded-chunk-set) runs inject identical
// populations (same A/B discipline as BenchFakePlayers task170).
//
// Population model (farm-world steady state):
//   - initial injection: 70% items / 20% hostiles / 10% passives
//     (spec default, docs/BENCH_X150K_SCENARIO.md §2)
//   - items despawn at vanilla age 6000 ticks => a TOPUP task re-injects the
//     despawned share every 600 ticks, so the item tick + merge + despawn
//     lanes run CONTINUOUSLY (documented bench deviation, same class as
//     SUMMON_SWEEPS; markers: POPULATION TOPUP)
//   - hostiles/passives are setPersistent(true): farm-stock baseline that
//     survives the window; the NATURAL despawn lane stays vanilla because
//     natural spawns near the fake players are NOT persistent
//   - ~60% of items concentrate in "farm clusters" (5% of loaded chunks,
//     min 16), the rest spread uniformly — mirrors concentrated farm output
//
// NOT FOR PRODUCTION. Bench harness only (world-bench-3 CI, sanctioned boots).
// ============================================================================

import org.bukkit.Bukkit;
import org.bukkit.Chunk;
import org.bukkit.Location;
import org.bukkit.Material;
import org.bukkit.World;
import org.bukkit.command.Command;
import org.bukkit.command.CommandSender;
import org.bukkit.entity.Entity;
import org.bukkit.entity.EntityType;
import org.bukkit.entity.Item;
import org.bukkit.entity.LivingEntity;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.java.JavaPlugin;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Deque;
import java.util.List;
import java.util.Random;

public final class BenchPopulationPlugin extends JavaPlugin {

    private static final String MARK = "[BenchPopulation]";
    private static final int ITEM_PICKUP_DELAY = 32767;  // short-max: never picked up
    private static final int TOPUP_PERIOD_TICKS = 600;   // 30 s
    private static final int ITEM_LIFETIME_TICKS = 6000; // vanilla ItemEntity age
    private static final int TICK_BUDGET = 1500;         // entities injected per tick
    private static final double SHARE_ITEMS = 0.70;
    private static final double SHARE_HOSTILES = 0.20;   // rest = passives
    private static final double CLUSTER_ITEM_SHARE = 0.60;
    private static final double CLUSTER_CHUNK_SHARE = 0.05;
    private static final int CLUSTER_MIN_CHUNKS = 16;

    private static final Material[] ITEM_POOL = {
            Material.WHEAT, Material.CARROT, Material.POTATO, Material.BEEF,
            Material.IRON_INGOT, Material.COBBLESTONE, Material.STICK
    };
    private static final EntityType[] HOSTILE_POOL = {
            EntityType.ZOMBIE, EntityType.SKELETON, EntityType.SPIDER,
            EntityType.CREEPER, EntityType.HUSK, EntityType.DROWNED
    };
    private static final EntityType[] PASSIVE_POOL = {
            EntityType.COW, EntityType.PIG, EntityType.SHEEP, EntityType.CHICKEN
    };

    private int target = 0;
    private long seed = 42;

    // injection state machine (single-flight; console is the only trigger)
    private boolean injecting = false;
    private int injectedTotal = 0, injectedItems = 0, injectedHostiles = 0, injectedPassives = 0;
    private int planItems = 0, planHostiles = 0, planPassives = 0;
    private List<Chunk> chunkOrder = null;
    private List<Chunk> farmClusters = null;
    private int cursor = 0;         // uniform lane index into chunkOrder
    private int clusterCursor = 0;  // round-robin over farmClusters
    private int lastProgress = 0;
    private long startNanos = 0;
    private long t0FullTime = 0;    // fullTime at injection finish (topup replay anchor, S7-130)
    private int itemsThisSlice = 0; // items spawned by the current injection tick (spawn-log entry per tick)

    // topup bookkeeping: items spawned in the last ITEM_LIFETIME_TICKS are the
    // alive-item estimate (vanilla despawn removes everything older).
    // S7-130 fix: the INITIAL injection is logged here too (one entry per
    // injection tick) — previously only topup spawns were logged, so the first
    // topup saw aliveEst=0 and doubled the item population for ~6600 ticks.
    private final Deque<long[]> itemSpawnLog = new ArrayDeque<>(); // [fullTime, count]

    @Override
    public void onEnable() {
        String tEnv = System.getenv("BENCH_POPULATION_TARGET");
        String sEnv = System.getenv("BENCH_POPULATION_SEED");
        try {
            target = tEnv == null ? 0 : Integer.parseInt(tEnv.trim());
        } catch (NumberFormatException e) {
            target = 0;
        }
        try {
            seed = sEnv == null ? 42L : Long.parseLong(sEnv.trim());
        } catch (NumberFormatException e) {
            seed = 42L;
        }
        if (target <= 0) {
            getLogger().info(MARK + " BENCH_POPULATION_TARGET<=0 -> fixture idle (no injection)");
            return;
        }
        getLogger().info(MARK + " armed: target=" + target + " seed=" + seed
                + " — waiting for console command `benchpop inject`");
    }

    @Override
    public boolean onCommand(CommandSender sender, Command cmd, String label, String[] args) {
        if (!"benchpop".equals(cmd.getName())) {
            return false;
        }
        if (args.length >= 1 && "inject".equalsIgnoreCase(args[0])) {
            int t = target;
            long s = seed;
            try {
                if (args.length >= 2) {
                    t = Integer.parseInt(args[1].trim());
                }
                if (args.length >= 3) {
                    s = Long.parseLong(args[2].trim());
                }
            } catch (NumberFormatException e) {
                sender.sendMessage(MARK + " bad args — usage: benchpop inject <target> [seed]");
                return true;
            }
            startInjection(t, s);
            return true;
        }
        sender.sendMessage(MARK + " usage: benchpop inject <target> [seed]");
        return true;
    }

    private synchronized void startInjection(int t, long s) {
        if (injecting) {
            getLogger().warning(MARK + " injection already in flight — ignored");
            return;
        }
        if (t <= 0) {
            getLogger().warning(MARK + " inject target<=0 — ignored");
            return;
        }
        injecting = true;
        target = t;
        seed = s;
        injectedTotal = injectedItems = injectedHostiles = injectedPassives = 0;
        planItems = planHostiles = planPassives = 0;
        cursor = clusterCursor = 0;
        lastProgress = 0;
        startNanos = System.nanoTime();
        t0FullTime = 0;
        itemSpawnLog.clear();

        World w = Bukkit.getWorlds().get(0);
        Chunk[] loaded = w.getLoadedChunks();
        chunkOrder = new ArrayList<>(loaded.length);
        chunkOrder.addAll(Arrays.asList(loaded));
        chunkOrder.sort((a, b) -> {
            long ka = ((long) a.getX() << 32) | (a.getZ() & 0xffffffffL);
            long kb = ((long) b.getX() << 32) | (b.getZ() & 0xffffffffL);
            return Long.compare(ka, kb);
        });

        Random rng = new Random(seed);
        planItems = (int) Math.round(target * SHARE_ITEMS);
        planHostiles = (int) Math.round(target * SHARE_HOSTILES);
        planPassives = Math.max(0, target - planItems - planHostiles);

        // farm clusters: deterministic subset of loaded chunks
        int clusterN = Math.max(CLUSTER_MIN_CHUNKS,
                (int) (chunkOrder.size() * CLUSTER_CHUNK_SHARE));
        farmClusters = new ArrayList<>(clusterN);
        for (int i = 0; i < clusterN && !chunkOrder.isEmpty(); i++) {
            farmClusters.add(chunkOrder.get(rng.nextInt(chunkOrder.size())));
        }

        getLogger().info(MARK + " POPULATION INJECT START target=" + target + " seed=" + seed
                + " loadedChunks=" + chunkOrder.size()
                + " plan(items=" + planItems + ", hostiles=" + planHostiles
                + ", passives=" + planPassives + ") farmClusters=" + farmClusters.size());

        // spread the addEntity pressure across ticks: a fixed per-tick budget
        // avoids a single giant freeze while finishing well before the window
        // (10k ~= 7 ticks, 150k ~= 100 ticks at 1500/tick)
        Bukkit.getScheduler().runTaskTimer(this, new Runnable() {
            @Override
            public void run() {
                itemsThisSlice = 0;
                int budget = TICK_BUDGET;
                while (budget > 0 && injectedTotal < target) {
                    int placed = injectSlice(budget);
                    if (placed <= 0) {
                        break;
                    }
                    budget -= placed;
                }
                // S7-130: log this tick's item spawns with their own fullTime so the
                // topup alive-estimate covers the initial injection exactly (vanilla
                // despawn removes these items at fullTime + 6000, same as the purge
                // horizon in startTopupTask)
                if (itemsThisSlice > 0) {
                    long now = Bukkit.getWorlds().get(0).getFullTime();
                    itemSpawnLog.addLast(new long[]{now, itemsThisSlice});
                }
                if (injectedTotal >= target) {
                    finishInjection();
                    Bukkit.getScheduler().cancelTasks(BenchPopulationPlugin.this);
                    startTopupTask();
                }
            }
        }, 1L, 1L);
    }

    /** Injects up to {@code budget} entities; returns the placed count. */
    private int injectSlice(int budget) {
        World w = Bukkit.getWorlds().get(0);
        // per-slice RNG seeded by (seed, injectedTotal): the slice boundaries
        // are deterministic (fixed budget, fixed lane order), so the whole
        // injection replay is deterministic given (target, seed, chunk set)
        Random rng = new Random(seed ^ (injectedTotal * 1_000_003L));
        int placed = 0;
        while (placed < budget && injectedTotal < target) {
            Chunk ch = nextChunk(rng);
            if (ch == null) {
                break;
            }
            Location base = centerOf(ch);
            if (base == null) {
                continue; // rare: chunk dropped mid-injection — skip, not fatal
            }
            double roll = rng.nextDouble();
            if (roll < SHARE_ITEMS && injectedItems < planItems) {
                if (spawnItem(w, jitter(base, rng), ITEM_POOL[rng.nextInt(ITEM_POOL.length)])) {
                    injectedItems++;
                    injectedTotal++;
                    placed++;
                    itemsThisSlice++;
                }
            } else if (roll < SHARE_ITEMS + SHARE_HOSTILES && injectedHostiles < planHostiles) {
                if (spawnMob(w, jitter(base, rng), HOSTILE_POOL[rng.nextInt(HOSTILE_POOL.length)])) {
                    injectedHostiles++;
                    injectedTotal++;
                    placed++;
                }
            } else if (injectedPassives < planPassives) {
                if (spawnMob(w, jitter(base, rng), PASSIVE_POOL[rng.nextInt(PASSIVE_POOL.length)])) {
                    injectedPassives++;
                    injectedTotal++;
                    placed++;
                }
            } else {
                // the rolled class plan is full — fall through to any open lane
                // so the total still reaches target exactly
                boolean ok = false;
                if (injectedItems < planItems) {
                    ok = spawnItem(w, jitter(base, rng), ITEM_POOL[rng.nextInt(ITEM_POOL.length)]);
                    if (ok) {
                        injectedItems++;
                        itemsThisSlice++;
                    }
                } else if (injectedHostiles < planHostiles) {
                    ok = spawnMob(w, jitter(base, rng), HOSTILE_POOL[rng.nextInt(HOSTILE_POOL.length)]);
                    if (ok) {
                        injectedHostiles++;
                    }
                } else if (injectedPassives < planPassives) {
                    ok = spawnMob(w, jitter(base, rng), PASSIVE_POOL[rng.nextInt(PASSIVE_POOL.length)]);
                    if (ok) {
                        injectedPassives++;
                    }
                }
                if (ok) {
                    injectedTotal++;
                    placed++;
                } else {
                    break; // every lane full (cannot happen: plans sum to target)
                }
            }
        }
        if (injectedTotal - lastProgress >= 5000) {
            getLogger().info(MARK + " POPULATION INJECT PROGRESS " + injectedTotal + "/" + target);
            lastProgress = injectedTotal;
        }
        return placed;
    }

    /** Uniform lane vs farm-cluster lane: cluster lane feeds the item plan. */
    private Chunk nextChunk(Random rng) {
        boolean useCluster = farmClusters != null && !farmClusters.isEmpty()
                && injectedItems < planItems
                && (injectedItems % 5) < 3
                && rng.nextDouble() < CLUSTER_ITEM_SHARE
                && clusterCursor < farmClusters.size();
        if (useCluster) {
            return farmClusters.get(clusterCursor++ % farmClusters.size());
        }
        if (cursor >= chunkOrder.size()) {
            return null;
        }
        return chunkOrder.get(cursor++);
    }

    private Location centerOf(Chunk ch) {
        World w = ch.getWorld();
        if (!w.isChunkLoaded(ch.getX(), ch.getZ())) {
            return null;
        }
        int wx = ch.getX() * 16 + 8;
        int wz = ch.getZ() * 16 + 8;
        int y = w.getHighestBlockYAt(wx, wz);
        return new Location(w, wx + 0.5, y + 1.0, wz + 0.5);
    }

    private Location jitter(Location base, Random rng) {
        return base.clone().add(rng.nextInt(9) - 4, 0, rng.nextInt(9) - 4);
    }

    private boolean spawnItem(World w, Location loc, Material mat) {
        try {
            Item it = w.dropItem(loc, new ItemStack(mat));
            it.setPickupDelay(ITEM_PICKUP_DELAY);
            return true;
        } catch (Throwable t) {
            getLogger().warning(MARK + " item spawn failed at " + loc + ": " + t);
            return false;
        }
    }

    private boolean spawnMob(World w, Location loc, EntityType type) {
        try {
            Entity e = w.spawnEntity(loc, type);
            if (e instanceof LivingEntity le) {
                le.setPersistent(true);         // farm-stock baseline (see header)
                le.setRemoveWhenFarAway(false); // explicit: no player-distance-despawn
            }
            return true;
        } catch (Throwable t) {
            getLogger().warning(MARK + " mob spawn failed at " + loc + ": " + t);
            return false;
        }
    }

    private void finishInjection() {
        long elapsedMs = (System.nanoTime() - startNanos) / 1_000_000L;
        t0FullTime = Bukkit.getWorlds().get(0).getFullTime(); // topup replay anchor (S7-130)
        boolean valid = injectedTotal >= Math.round(target * 0.9);
        getLogger().info(MARK + " POPULATION INJECT DONE target=" + target
                + " injected=" + injectedTotal
                + " items=" + injectedItems + " hostiles=" + injectedHostiles
                + " passives=" + injectedPassives
                + " elapsedMs=" + elapsedMs
                + " t0FullTime=" + t0FullTime);
        getLogger().info(MARK + " POPULATION FIXTURE-VALIDITY: " + (valid ? "VALID" : "INVALID")
                + " (injected=" + injectedTotal + " target=" + target + ")");
    }

    // --- topup: keep the item population at plan while vanilla despawn runs ---
    private void startTopupTask() {
        Bukkit.getScheduler().runTaskTimer(this, () -> {
            World w = Bukkit.getWorlds().get(0);
            long ft = w.getFullTime();
            long horizon = ft - ITEM_LIFETIME_TICKS;
            while (!itemSpawnLog.isEmpty() && itemSpawnLog.peekFirst()[0] < horizon) {
                itemSpawnLog.removeFirst();
            }
            long aliveEst = 0;
            for (long[] rec : itemSpawnLog) {
                aliveEst += rec[1];
            }
            long deficit = planItems - aliveEst;
            if (deficit <= 0) {
                getLogger().info(MARK + " POPULATION TOPUP tick=" + ft
                        + " deficit=0 aliveEst=" + aliveEst);
                return;
            }
            // S7-130: seed from the scene-relative clock delta = ft - T0 (injection
            // finish anchor), NOT the absolute world time — replaying (target, seed)
            // yields the same topup stream regardless of boot timing drift
            Random rng = new Random(seed ^ (ft - t0FullTime));
            int spawned = 0;
            for (int i = 0; i < deficit; i++) {
                if (chunkOrder.isEmpty()) {
                    break;
                }
                Chunk ch = chunkOrder.get(rng.nextInt(chunkOrder.size()));
                Location base = centerOf(ch);
                if (base == null) {
                    continue;
                }
                if (spawnItem(w, jitter(base, rng), ITEM_POOL[rng.nextInt(ITEM_POOL.length)])) {
                    spawned++;
                }
            }
            if (spawned > 0) {
                itemSpawnLog.addLast(new long[]{ft, spawned});
            }
            getLogger().info(MARK + " POPULATION TOPUP tick=" + ft
                    + " spawned=" + spawned + " deficit=" + deficit
                    + " aliveEst=" + aliveEst);
        }, TOPUP_PERIOD_TICKS, TOPUP_PERIOD_TICKS);
    }
}
