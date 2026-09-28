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
// C61 (ROUND-477, POP-gate re-dispatch fix ×3 worlds dp2/totemA/Trek — runs
// 36346148733/36346160464/36346174230): the heavy-world legs wedged in the
// injection phase. Post-mortem of dp2 36346148733: INJECT START at
// loadedChunks=9786 raced 0→42000 in 18s, then 16385 consecutive
// "spawn failed ... ArrayIndexOutOfBoundsException: Index -1 length 65537"
// (fastutil Int2ObjectOpenHashMap n=65536 state, mask=-1 — the SAME C30-475
// corruption class as the S62 DnT fatals: concurrent rt4-worker fastutil
// mutation vs main-thread access, here on the ChunkMap.entityMap surface).
// After the first storm the per-tick inject loop `while (placed < budget)`
// never reached budget again (0 successes) and SPUN the server thread until
// the JVM kill (183 watchdog dumps in ChunkMap.addEntity→containsKey:349,
// 23% addEntity + 32% containsKey CPU) — the DONE marker never printed and
// the run burned the full 900s as a zombie. C54-fact: the SAME plugin
// completes 150k in 56.5s on the canon world → the injector logic is sound,
// the HARNESS must (a) bound per-tick attempts so the server thread ALWAYS
// returns (wedge-kill), (b) abort the arm loudly on a consecutive-failure
// storm and re-arm once from the live deficit, (c) gate the start on the
// full force-loaded chunk set (shell counts "Marked" forceload lines and
// exports BENCH_POPULATION_MIN_LOADED), (d) cap the DONE-wait at 1800s
// (run_world3.sh) and fail FAST on the ABORTED marker instead of burning
// the cap. Measurement semantics untouched: injection still completes
// strictly before the profiler window; determinism (target, seed,
// loaded-chunk-set) preserved on the happy path (re-arm logs its deviation).
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
import org.bukkit.entity.Animals;
import org.bukkit.entity.Entity;
import org.bukkit.entity.EntityType;
import org.bukkit.entity.Item;
import org.bukkit.entity.LivingEntity;
import org.bukkit.entity.Monster;
import org.bukkit.inventory.ItemStack;
import org.bukkit.plugin.java.JavaPlugin;
import org.bukkit.scheduler.BukkitTask;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Deque;
import java.util.List;
import java.util.Random;

public final class BenchPopulationPlugin extends JavaPlugin {

    private static final String MARK = "[BenchPopulation]";
    private static final int ITEM_PICKUP_DELAY = 32767;  // short-max: never picked up
    private static final int TOPUP_PERIOD_TICKS = 120;   // S7-148: 6 s @20TPS (было 600 — урок leg #2'': при 900+ тиках leg-ранов один скан не успевал)
    private static final int TOPUP_PER_TICK = 20;        // S7-147: min per-tick refill budget (~14ms/tick, profile-invisible)
    private static final int TOPUP_PER_TICK_MAX = 100;   // S7-148: cap дефицит-драйвена (~70ms/тик worst-case при TPS 3+)
    private static final int TOPUP_DRAIN_HORIZON = 50;   // S7-148: тиков на добор дефицита (deficit/HORIZON база бюджета)
    private static final int ITEM_LIFETIME_TICKS = 6000; // vanilla ItemEntity age
    private static final int TICK_BUDGET = 1500;         // entities injected per tick
    // --- C61 harness hardening (POP-gate ×3-world re-dispatch) ----------------
    private static final int ATTEMPT_CAP_SLACK = 128;    // C61: max spawn attempts/tick = budget*2 + slack (wedge-kill)
    private static final int STALL_FAIL_ABORT = 512;     // C61: consecutive spawn failures → abort this arm
    private static final int MAX_INJECT_ARMS = 2;        // C61: initial arm + 1 re-arm, then loud latch
    private static final int REARM_DELAY_TICKS = 100;    // C61: 5 s settle between arms
    private static final int GATE_POLL_TICKS = 40;       // C61: start-gate re-poll period (2 s)
    private static final int GATE_MAX_POLLS = 300;       // C61: start-gate hard wall (300 polls = 10 min) — never deadlock
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
    private int stallWarn = 0;
    private long startNanos = 0;
    // C61 state: start-gate + corruption-guard + failure telemetry
    private int minLoadedChunks = 0;      // BENCH_POPULATION_MIN_LOADED; 0 = gate off
    private BukkitTask gateTask = null;   // C61: start-gate poller handle (self-cancelling)
    private int armsUsed = 0;             // injection arms consumed (initial + re-arms)
    private boolean abortLatched = false; // catastrophic corruption latch (no more arms)
    private int consecutiveFails = 0;     // spawn failures since last success (persists across ticks)
    private long attemptTotal = 0;        // spawn attempts (success + failure), cumulative
    private long failTotal = 0;           // spawn failures, cumulative
    private long failLogged = 0;          // failure WARNs emitted (throttle counter)
    private long t0FullTime = 0;    // fullTime at injection finish (topup replay anchor, S7-130)
    private int itemsThisSlice = 0; // items spawned by the current injection tick (spawn-log entry per tick)

    // topup bookkeeping: items spawned in the last ITEM_LIFETIME_TICKS are the
    // alive-item estimate (vanilla despawn removes everything older).
    // S7-130 fix: the INITIAL injection is logged here too (one entry per
    // injection tick) — previously only topup spawns were logged, so the first
    // topup saw aliveEst=0 and doubled the item population for ~6600 ticks.
    private final Deque<long[]> itemSpawnLog = new ArrayDeque<>(); // [fullTime, count]

    // S7-147 real-count topup state: deficits computed from ACTUAL alive counts
    // every TOPUP_PERIOD_TICKS, drained continuously at TOPUP_PER_TICK/tick.
    // The despawn-schedule model (itemSpawnLog) is blind to merge/lava/explosion/
    // cramming/daylight-burning losses — any lever that raises TPS multiplies
    // ticks-per-wall-second and the scene drains under wall-time parity
    // (evidence: runs 35284069355 / 35314220731 — 148k -> ~71k alive while the
    // model reported deficit=0; topup never fired in the 300s base at TPS 0.7).
    private int pendingItems = 0, pendingHostiles = 0, pendingPassives = 0;
    private long topupSpawnedTotal = 0;
    private boolean topupDrainTaskRunning = false;

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
        String mEnv = System.getenv("BENCH_POPULATION_MIN_LOADED");
        try {
            minLoadedChunks = mEnv == null ? 0 : Integer.parseInt(mEnv.trim());
        } catch (NumberFormatException e) {
            minLoadedChunks = 0;
        }
        if (target <= 0) {
            getLogger().info(MARK + " BENCH_POPULATION_TARGET<=0 -> fixture idle (no injection)");
            return;
        }
        getLogger().info(MARK + " armed: target=" + target + " seed=" + seed
                + " minLoadedChunks=" + minLoadedChunks
                + " — waiting for console command `benchpop inject`");
    }

    @Override
    public void onDisable() {
        // G3.0-fixture (478-A2): never leave the region workers parked if the
        // plugin unloads mid-injection (bench window must run the lever).
        hookInjectQuiesce(false);
    }

    // --- G3.0-fixture (478-A2): INJECT-QUIESCE hook -----------------------
    // While the fixture is being injected, the region-confinement workers are
    // parked (RegionTickOps.forEach takes the vanilla-serial list.forEach
    // tail). Evidence class: dp2 36357022841 / trek 36356982268 / totem
    // 36357157202 — fastutil entityMap AIOOBE Index -1 (len 65537|131073)
    // from main-thread addEntity racing worker-thread entity callbacks.
    // Injection is fixture-prep, NOT the measured window: the serial path is
    // bit-exact vanilla and the lever re-arms at INJECT DONE (fix=0 on the
    // window; C54 PER-WORLD refuted — same world sha a13b353a SUCCEEDED on
    // c30febfb and FAILED on the 640926e9 carrier).
    private boolean quiesceHookBroken = false;

    private void hookInjectQuiesce(boolean on) {
        if (quiesceHookBroken) {
            return;
        }
        try {
            Class<?> ops = Class.forName("net.minecraft.world.entity.RegionTickOps");
            ops.getMethod("setInjectQuiesce", boolean.class).invoke(null, on);
        } catch (Throwable t) {
            // Non-carrier runtime (no RegionTickOps): quiesce structurally
            // unavailable — vanilla serial is already the only path.
            quiesceHookBroken = true;
            getLogger().warning(MARK + " inject-quiesce hook unavailable ("
                    + t.getClass().getSimpleName() + ") — continuing without park");
        }
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
        if (abortLatched) {
            getLogger().severe(MARK + " injection request ignored — corruption-guard latched (see POPULATION INJECT ABORTED)");
            return;
        }
        if (injecting) {
            getLogger().warning(MARK + " injection already in flight — ignored");
            return;
        }
        if (t <= 0) {
            getLogger().warning(MARK + " inject target<=0 — ignored");
            return;
        }
        // C61 start-gate: do not race the force-load tail. The harness exports
        // BENCH_POPULATION_MIN_LOADED = full sweep chunk count; injection arms
        // only once getLoadedChunks() reaches it (re-poll every GATE_POLL_TICKS,
        // hard wall GATE_MAX_POLLS — the gate can never deadlock the run).
        if (minLoadedChunks > 0) {
            World w0 = Bukkit.getWorlds().get(0);
            int loadedNow = w0.getLoadedChunks().length;
            if (loadedNow < minLoadedChunks) {
                getLogger().warning(MARK + " POPULATION INJECT GATE-WAIT loadedChunks=" + loadedNow
                        + " need>=" + minLoadedChunks + " — deferring arm (poll " + GATE_POLL_TICKS + "t)");
                final int gt = t;
                final long gs = s;
                gateTask = Bukkit.getScheduler().runTaskTimer(this, new Runnable() {
                    private int polls = 0;
                    @Override
                    public void run() {
                        polls++;
                        int l = Bukkit.getWorlds().get(0).getLoadedChunks().length;
                        if (l >= minLoadedChunks) {
                            if (gateTask != null) {
                                gateTask.cancel();
                                gateTask = null;
                            }
                            getLogger().info(MARK + " POPULATION INJECT GATE-PASS loadedChunks=" + l
                                    + " (polls=" + polls + ") — arming injection");
                            hookInjectQuiesce(true); // G3.0-fixture: park workers for the injection window
                            beginInjection(gt, gs);
                        } else if (polls >= GATE_MAX_POLLS) {
                            if (gateTask != null) {
                                gateTask.cancel();
                                gateTask = null;
                            }
                            getLogger().warning(MARK + " POPULATION INJECT GATE-TIMEOUT loadedChunks=" + l
                                    + " need>=" + minLoadedChunks + " after " + polls + " polls — arming anyway (never deadlock)");
                            hookInjectQuiesce(true); // G3.0-fixture: park workers for the injection window
                            beginInjection(gt, gs);
                        } else if (polls % 50 == 0) {
                            getLogger().warning(MARK + " POPULATION INJECT GATE-WAIT loadedChunks=" + l
                                    + " need>=" + minLoadedChunks + " (polls=" + polls + ")");
                        }
                    }
                }, GATE_POLL_TICKS, GATE_POLL_TICKS);
                return;
            }
        }
        beginInjection(t, s);
    }

    /** C61: the actual arm — full state reset, snapshot, plan, per-tick timer. */
    private synchronized void beginInjection(int t, long s) {
        injecting = true;
        armsUsed++;
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
        armInjectTimer();
    }

    /** C61: single re-arm — keep injected counters (resume from the live
     *  scene), re-snapshot the loaded-chunk set, reset the per-arm guards.
     *  Deviation vs the happy-path replay is logged and confined to the
     *  corruption path (canon worlds never hit it). */
    private synchronized void reArm() {
        if (abortLatched || injecting) {
            return;
        }
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
        int clusterN = Math.max(CLUSTER_MIN_CHUNKS,
                (int) (chunkOrder.size() * CLUSTER_CHUNK_SHARE));
        farmClusters = new ArrayList<>(clusterN);
        for (int i = 0; i < clusterN && !chunkOrder.isEmpty(); i++) {
            farmClusters.add(chunkOrder.get(rng.nextInt(chunkOrder.size())));
        }
        cursor = 0;
        clusterCursor = 0;
        consecutiveFails = 0;
        injecting = true;
        armsUsed++;
        getLogger().info(MARK + " POPULATION INJECT RE-ARM-START arms=" + armsUsed
                + " loadedChunks=" + chunkOrder.size()
                + " farmClusters=" + farmClusters.size()
                + " resuming injected=" + injectedTotal + "/" + target
                + " (seed replay deviates on the re-snapshot — corruption path only)");
        armInjectTimer();
    }

    /** C61: the per-tick injection timer (extracted so beginInjection and
     *  reArm share one implementation). */
    private void armInjectTimer() {
        Bukkit.getScheduler().runTaskTimer(this, new Runnable() {
            @Override
            public void run() {
                if (abortLatched || !injecting) {
                    return;
                }
                itemsThisSlice = 0;
                int budget = TICK_BUDGET;
                while (budget > 0 && injectedTotal < target) {
                    int placed = injectSlice(budget);
                    if (placed <= 0) {
                        break;
                    }
                    budget -= placed;
                }
                if (itemsThisSlice > 0) {
                    long now = Bukkit.getWorlds().get(0).getFullTime();
                    itemSpawnLog.addLast(new long[]{now, itemsThisSlice});
                }
                if (injectedTotal >= target) {
                    finishInjection();
                    Bukkit.getScheduler().cancelTasks(BenchPopulationPlugin.this);
                    startTopupTask();
                    return;
                }
                if (consecutiveFails >= STALL_FAIL_ABORT) {
                    Bukkit.getScheduler().cancelTasks(BenchPopulationPlugin.this);
                    injecting = false;
                    int injectedSoFar = injectedTotal;
                    long failsSoFar = failTotal;
                    if (armsUsed >= MAX_INJECT_ARMS) {
                        abortLatched = true;
                        hookInjectQuiesce(false); // G3.0-fixture: re-arm the lever, window must stay measured
                        getLogger().severe(MARK + " POPULATION INJECT ABORTED corruption-guard arms=" + armsUsed
                                + " injected=" + injectedSoFar + "/" + target
                                + " fails=" + failsSoFar
                                + " — entity-tracker fastutil state corrupted (C30-class rt4 race, AIOOBE Index -1); fixture INVALID, failing fast");
                    } else {
                        getLogger().severe(MARK + " POPULATION INJECT RE-ARM corruption-guard arms=" + armsUsed
                                + " injected=" + injectedSoFar + "/" + target
                                + " fails=" + failsSoFar
                                + " — re-snapshotting chunks, resuming from live deficit in " + REARM_DELAY_TICKS + "t");
                        Bukkit.getScheduler().runTaskLater(BenchPopulationPlugin.this,
                                () -> reArm(), REARM_DELAY_TICKS);
                    }
                }
            }
        }, 1L, 1L);
    }

    /** Injects up to {@code budget} entities; returns the placed count.
     *  C61 wedge-kill: attempts are hard-capped at budget*2 + ATTEMPT_CAP_SLACK
     *  per tick. dp2 36346148733 proved the uncapped loop can spin FOREVER
     *  (0-success spawn storm keeps placed at 0 while the while-condition
     *  stays true) — the server thread must ALWAYS return from a tick. */
    private int injectSlice(int budget) {
        World w = Bukkit.getWorlds().get(0);
        // per-slice RNG seeded by (seed, injectedTotal): the slice boundaries
        // are deterministic (fixed budget, fixed lane order), so the whole
        // injection replay is deterministic given (target, seed, chunk set)
        Random rng = new Random(seed ^ (injectedTotal * 1_000_003L));
        int placed = 0;
        int attempts = 0;
        int attemptCap = budget * 2 + ATTEMPT_CAP_SLACK;
        int missStreak = 0; // S7-130b: loud diagnostics instead of a silent freeze
        while (placed < budget && injectedTotal < target) {
            if (++attempts > attemptCap) {
                getLogger().warning(MARK + " INJECT ATTEMPT-CAP attempts=" + attempts
                        + " (cap=" + attemptCap + ") placed=" + placed
                        + " injected=" + injectedTotal + "/" + target
                        + " consecutiveFails=" + consecutiveFails
                        + " — yielding tick (C61 wedge-kill)");
                break;
            }
            Chunk ch = nextChunk(rng);
            if (ch == null) {
                if (++missStreak == 1) {
                    getLogger().warning(MARK + " nextChunk=null — chunkOrder empty; injection cannot progress"
                            + " (injected=" + injectedTotal + "/" + target + ")");
                }
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
        } else if (placed <= 0 && injectedTotal < target && ++stallWarn % 20 == 1) {
            // S7-130b: placed=0 with the plan incomplete must NEVER be silent —
            // this exact silence cost run 35238931413 its whole window
            getLogger().warning(MARK + " INJECT STALL placed=0 injected=" + injectedTotal
                    + "/" + target + " (cursor=" + cursor + " clusterCursor=" + clusterCursor
                    + " items=" + injectedItems + "/" + planItems
                    + " hostiles=" + injectedHostiles + "/" + planHostiles
                    + " passives=" + injectedPassives + "/" + planPassives + ")");
        }
        return placed;
    }

    /** Uniform lane vs farm-cluster lane: cluster lane feeds the item plan.
     *  S7-130b fix: the uniform lane WRAPS (modulo) — at 150k scale it needs
     *  ~87k placements over 9948 chunks, so a one-pass cursor that returns
     *  null after exhaustion silently froze the whole injection at ~11k
     *  (run 35238931413: PROGRESS 6000 was the last marker, no DONE/INVALID,
     *  harness stop at POP_INJECT_TIMEOUT). Cluster lane was already wrap- 
     *  around; uniform now matches it. */
    private Chunk nextChunk(Random rng) {
        boolean useCluster = farmClusters != null && !farmClusters.isEmpty()
                && injectedItems < planItems
                && (injectedItems % 5) < 3
                && rng.nextDouble() < CLUSTER_ITEM_SHARE
                && clusterCursor < farmClusters.size();
        if (useCluster) {
            return farmClusters.get(clusterCursor++ % farmClusters.size());
        }
        if (chunkOrder.isEmpty()) {
            return null;
        }
        return chunkOrder.get(cursor++ % chunkOrder.size());
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
            noteSpawnSuccess();
            return true;
        } catch (Throwable t) {
            noteSpawnFailure("item", loc, t);
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
            noteSpawnSuccess();
            return true;
        } catch (Throwable t) {
            noteSpawnFailure("mob", loc, t);
            return false;
        }
    }

    /** C61: failure telemetry + throttled logging. The full failure log of
     *  dp2 36346148733 (16385 identical WARNs) is log-noise; the guard needs
     *  the first few, then a periodic heartbeat. */
    private void noteSpawnFailure(String kind, Location loc, Throwable t) {
        consecutiveFails++;
        failTotal++;
        failLogged++;
        if (failLogged <= 3 || failLogged % 256 == 0) {
            getLogger().warning(MARK + " " + kind + " spawn failed (" + failLogged
                    + ") at " + loc + ": " + t);
        }
    }

    private void noteSpawnSuccess() {
        consecutiveFails = 0;
        attemptTotal++;
    }

    private void finishInjection() {
        hookInjectQuiesce(false); // G3.0-fixture: DONE — unpark workers, measured window runs the lever
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

    // --- topup: keep the POPULATION at plan while vanilla decay lanes run ---
    // S7-147: real-count driven (items + hostiles + passives). Scan every
    // TOPUP_PERIOD_TICKS, drain deficits continuously at TOPUP_PER_TICK/tick
    // so no single-tick spawn spike pollutes the profiled window.
    private void startTopupTask() {
        Bukkit.getScheduler().runTaskTimer(this, () -> {
            World w = Bukkit.getWorlds().get(0);
            long ft = w.getFullTime();

            // real alive counts over all loaded chunks (the 9216 forceloaded
            // chunks hold the whole scene)
            int aliveItems = 0, aliveHostiles = 0, alivePassives = 0;
            for (Entity e : w.getEntities()) {
                if (e instanceof Item) {
                    aliveItems++;
                } else if (e instanceof Monster) {
                    aliveHostiles++;
                } else if (e instanceof Animals) {
                    alivePassives++;
                }
            }

            // model estimate retained for telemetry continuity (age-despawn only)
            long horizon = ft - ITEM_LIFETIME_TICKS;
            while (!itemSpawnLog.isEmpty() && itemSpawnLog.peekFirst()[0] < horizon) {
                itemSpawnLog.removeFirst();
            }
            long aliveEst = 0;
            for (long[] rec : itemSpawnLog) {
                aliveEst += rec[1];
            }

            int deficitItems = Math.max(0, planItems - aliveItems);
            int deficitHostiles = Math.max(0, planHostiles - aliveHostiles);
            int deficitPassives = Math.max(0, planPassives - alivePassives);
            pendingItems = deficitItems;
            pendingHostiles = deficitHostiles;
            pendingPassives = deficitPassives;

            getLogger().info(MARK + " POPULATION TOPUP-SCAN tick=" + ft
                    + " aliveReal(items=" + aliveItems + ",hostiles=" + aliveHostiles
                    + ",passives=" + alivePassives + ")"
                    + " deficit(items=" + deficitItems + ",hostiles=" + deficitHostiles
                    + ",passives=" + deficitPassives + ")"
                    + " aliveEst(items-model)=" + aliveEst
                    + " topupSpawnedTotal=" + topupSpawnedTotal);

            startTopupDrainTask();
        }, TOPUP_PERIOD_TICKS, TOPUP_PERIOD_TICKS);
    }

    /**
     * S7-148: deficit-driven drain budget — не отстаёт от ванильного распада
     * при повышенных ticks-per-wall-second. Урок leg #2'' (35318755582):
     * фиксированные 20/тик < ~42/тик валового распада (item-merge герды,
     * горение, cramming при 900+ тиках) ⇒ сцена дренировала 148k→79k,
     * A/B несопоставимы. Бюджет = deficit/TOPUP_DRAIN_HORIZON (догнать за
     * 50 тиков), зажатый в [TOPUP_PER_TICK, TOPUP_PER_TICK_MAX]; при
     * нулевом дефиците задача выходит рано и в базе дренаж дремлет
     * (240 тиков < старого 600-периода — bit-for-bit совместимо).
     */
    private static int drainBudget(int deficitTotal) {
        if (deficitTotal <= 0) {
            return TOPUP_PER_TICK; // early-exit не сработает только если pending>0; безопасный минимум
        }
        int budget = deficitTotal / TOPUP_DRAIN_HORIZON;
        if (budget < TOPUP_PER_TICK) {
            budget = TOPUP_PER_TICK;
        }
        if (budget > TOPUP_PER_TICK_MAX) {
            budget = TOPUP_PER_TICK_MAX;
        }
        return budget;
    }

    /** S7-147: continuous deficit drain — at most TOPUP_PER_TICK spawns per tick. */
    private void startTopupDrainTask() {
        if (topupDrainTaskRunning) {
            return;
        }
        topupDrainTaskRunning = true;
        Bukkit.getScheduler().runTaskTimer(this, () -> {
            if (pendingItems <= 0 && pendingHostiles <= 0 && pendingPassives <= 0) {
                return;
            }
            World w = Bukkit.getWorlds().get(0);
            long ft = w.getFullTime();
            Random rng = new Random(seed ^ (ft * 1_000_003L) ^ topupSpawnedTotal);
            int budget = drainBudget(pendingItems + pendingHostiles + pendingPassives);
            int missStreak = 0;
            while (budget > 0) {
                // drain the largest pending lane first (deterministic tie-break:
                // items -> hostiles -> passives)
                int lane;
                if (pendingItems >= pendingHostiles && pendingItems >= pendingPassives && pendingItems > 0) {
                    lane = 0;
                } else if (pendingHostiles >= pendingPassives && pendingHostiles > 0) {
                    lane = 1;
                } else if (pendingPassives > 0) {
                    lane = 2;
                } else {
                    break;
                }
                Chunk ch = chunkOrder.isEmpty() ? null : chunkOrder.get(rng.nextInt(chunkOrder.size()));
                if (ch == null) {
                    break;
                }
                Location base = centerOf(ch);
                if (base == null) {
                    if (++missStreak > 64) {
                        break; // chunk set degraded mid-run — retry next tick
                    }
                    continue;
                }
                missStreak = 0;
                if (consecutiveFails >= STALL_FAIL_ABORT) {
                    break; // C61: corruption-guard — drain task yields, inject timer decides
                }
                boolean ok;
                if (lane == 0) {
                    ok = spawnItem(w, jitter(base, rng), ITEM_POOL[rng.nextInt(ITEM_POOL.length)]);
                    if (ok) {
                        pendingItems--;
                    }
                } else if (lane == 1) {
                    ok = spawnMob(w, jitter(base, rng), HOSTILE_POOL[rng.nextInt(HOSTILE_POOL.length)]);
                    if (ok) {
                        pendingHostiles--;
                    }
                } else {
                    ok = spawnMob(w, jitter(base, rng), PASSIVE_POOL[rng.nextInt(PASSIVE_POOL.length)]);
                    if (ok) {
                        pendingPassives--;
                    }
                }
                if (ok) {
                    topupSpawnedTotal++;
                    budget--;
                }
            }
        }, 1L, 1L);
    }
}
