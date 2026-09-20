package net.minecraft.world.entity;

import it.unimi.dsi.fastutil.ints.Int2ObjectOpenHashMap;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.util.Objects;

import io.papermc.paper.entity.activation.ActivationRange;

import net.minecraft.util.Mth;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.entity.item.ItemEntity;

import org.bukkit.craftbukkit.event.CraftEventFactory;
import org.bukkit.event.entity.EntityRemoveEvent;

/**
 * ItemsSweepOps (ROUND-396, TASK-396-C — ARCH-REPLACE lever items_sweep:
 * sweep-line batch-merge, the TOP-1 bottleneck vector).
 *
 * Vanilla cost model (javap contract, patched-kernel 1.21.10):
 *   - ItemEntity.tick fires mergeWithNeighbours at tickCount % (moved?2:40)
 *     and EACH firing does level.getEntitiesOfClass(ItemEntity, bb.inflate(
 *     itemMerge, yR, itemMerge)) — an O(section-scan) broadphase query plus
 *     a fresh List, per eligible item, per firing. With ~70% of the bench
 *     population being item entities this dominates the items lane (31.17%)
 *     and feeds the broadphase (15.66%), fastutil (8.54%) and java_util
 *     (7.01%) lanes.
 *
 * Architectural replacement: ONE sweep per region-bucket per tick, run by
 * RegionTickOps.tickBucket BEFORE the vanilla consumer loop:
 *   1. collect the bucket's ItemEntities directly (no getEntities),
 *      source-eligible = isMergable && ActivationRange.checkIfActive &&
 *      stride mirror ((tickCount+1) % 40 == 0, or % 2 == 0 for movers by
 *      last movement's block-boundary crossing);
 *   2. hash into an XZ grid with cell = itemMerge + 0.25 (the exact contact
 *      reach: bb half-width 0.125 + inflate R + candidate half 0.125) via an
 *      Int2ObjectOpenHashMap (ThreadLocal, reused);
 *   3. for each source (snapshot order = vanilla intra-bucket tick order),
 *      scan the 3x3 cell neighborhood, exact AABB interval test equal to the
 *      vanilla query box, vanilla isMergable predicate + in-loop recheck;
 *   4. apply merges through the VANILLA decision/application surface,
 *      transcribed bit-for-bit from javap: target-UUID equality, public
 *      ItemEntity.areMergable, public ItemStack.merge(stackA, stackB, 64),
 *      public CraftEventFactory.callItemMergeEvent (same receiver/source
 *      argument order), pickupDelay max, age min, public discard(MERGE);
 *      vanilla break semantics on isRemoved(src).
 * The per-entity query is SUPPRESSED during the parallel phase by
 * retargeting the single ItemEntity.tick call site to tickMerge (receiver-
 * prepended static, strict sites==1): in-phase calls return (the sweep owns
 * this tick's merges), out-of-phase calls fall back to the pristine private
 * mergeWithNeighbours via a MethodHandle — the fallback path is byte-for-
 * byte vanilla semantics.
 *
 * Parity (preregistered, RESEARCH-C.md §3): same rules, same events, same
 * application order per pair; documented deviations are (i) candidate order
 * within one source's loop is grid-order instead of storage-order (vanilla's
 * own order is implementation-defined by EntitySectionStorage), (ii) the
 * stride mirror reads LAST movement's block crossing (vanilla computes its
 * crossing after move() mid-tick) — a ±1 tick shift on rest<->motion
 * transitions, (iii) the sweep sees pre-tick positions (vanilla merges after
 * the source's move) — contacts created by this tick's movement are caught
 * by the next sweep.
 *
 * Delivery: defined into the KERNEL loader by the region_threads bridge
 * (RegionTickOps.tickBucket references sweepBucket unconditionally, so the
 * class MUST be co-defined with the region bridge even when this lever is
 * dormant: SWEEP=false makes the call a static-read no-op). The ItemEntity
 * retarget is applied by src/items_sweep.rs ONLY when CRUSSTY_LEVER_FLAG ==
 * "items_sweep" AND region_threads is enabled, strictly after the bridge is
 * ready — so tickMerge can never be resolved before its class exists and
 * the dormant path never touches ItemEntity bytes at all.
 *
 * MUST compile to exactly ONE classfile (no nested classes, non-capturing
 * lambdas only) — travel_diet one-classfile law.
 */
public final class ItemsSweepOps {

    private ItemsSweepOps() {
    }

    /**
     * Armed from the owner directive lever pipe (run_world3.sh exports
     * CRUSSTY_LEVER_FLAG/CRUSSTY_LEVER_ARG for the server JVM). Non-final so
     * a mid-flight sweep failure can self-quarantine the lever (fail-open to
     * the vanilla query path on the NEXT tick — loud, one-shot log).
     */
    public static boolean SWEEP = "items_sweep"
            .equals(System.getenv("CRUSSTY_LEVER_FLAG"));

    private static final MethodHandle MH_IS_MERGABLE;
    private static final MethodHandle MH_MERGE_WITH_NEIGHBOURS;

    static {
        MH_IS_MERGABLE = privateVirtual("isMergable", MethodType.methodType(boolean.class));
        MH_MERGE_WITH_NEIGHBOURS = privateVirtual("mergeWithNeighbours", MethodType.methodType(void.class));
    }

    private static MethodHandle privateVirtual(String name, MethodType type) {
        try {
            MethodHandles.Lookup lookup = MethodHandles.privateLookupIn(ItemEntity.class, MethodHandles.lookup());
            return lookup.findVirtual(ItemEntity.class, name, type);
        } catch (Throwable t) {
            throw new ExceptionInInitializerError(t);
        }
    }

    // ==================================================================
    // Retargeted call site (the ONLY one): ItemEntity.tick bc470
    // invokevirtual mergeWithNeighbours -> invokestatic tickMerge.
    // ==================================================================

    /**
     * Receiver-prepended static replacement of the tick-time merge call.
     * In the region-parallel phase the batch sweep of this tick has already
     * applied every merge the vanilla query would have found, so the
     * per-item broadphase query is suppressed (architectural replacement,
     * not a diet: the query is REMOVED from the hot path, its work is done
     * by the O(contacts) grid sweep). Outside the phase the pristine
     * vanilla body runs via the MethodHandle.
     */
    public static void tickMerge(ItemEntity self) {
        if (SWEEP && RegionTickOps.sweepPhase()) {
            return;
        }
        try {
            MH_MERGE_WITH_NEIGHBOURS.invokeExact(self);
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    // ==================================================================
    // Batch sweep driver — called from RegionTickOps.tickBucket before the
    // vanilla consumer loop, once per slot per tick, in parallel by the
    // region workers (buckets are spatially disjoint 8x8-chunk regions and
    // the contact reach is 0.75 blocks, so merge pairs never cross slots).
    // ==================================================================

    private static final ThreadLocal<ItemEntity[]> TL_ITEMS = ThreadLocal.withInitial(() -> new ItemEntity[4096]);
    private static final ThreadLocal<Int2ObjectOpenHashMap<int[]>> TL_GRID = ThreadLocal.withInitial(Int2ObjectOpenHashMap::new);

    /**
     * @param bucket the slot's entity slice (RegionTickOps.bucketArr[slot])
     * @param len    used length (RegionTickOps.bucketLen[slot])
     */
    public static void sweepBucket(Entity[] bucket, int len) {
        if (!SWEEP || len < 2) {
            return;
        }
        try {
            sweepBucketInner(bucket, len);
        } catch (Throwable t) {
            SWEEP = false; // self-quarantine: vanilla queries resume next tick
            System.err.println("[crussty-items-sweep] sweep failed once — lever quarantined to vanilla: " + t);
        }
    }

    private static void sweepBucketInner(Entity[] bucket, int len) {
        ItemEntity[] items = TL_ITEMS.get();
        if (items.length < len) {
            items = new ItemEntity[Math.max(len, items.length * 2)];
            TL_ITEMS.set(items);
        }
        int n = 0;
        for (int i = 0; i < len; i++) {
            if (bucket[i] instanceof ItemEntity ie && sourceEligible(ie)) {
                items[n++] = ie;
            }
        }
        if (n < 2) {
            return;
        }

        // Per-bucket constants read live from config (same values the
        // vanilla query reads): horizontal radius R and the Y inflation
        // (paper onlyMergeItemsHorizontally collapses it to 0).
        Level level = items[0].level();
        double r = level.spigotConfig.itemMerge;
        boolean onlyHorizontal = level.paperConfig().entities.behavior.onlyMergeItemsHorizontally;
        double yr = onlyHorizontal ? 0.0D : r - 0.5D;
        // Grid cell == contact reach: any mergeable pair differs by less in
        // each axis, any pair >= 2 cells apart is >= reach away.
        final double cell = Math.max(0.25D, r + 0.25D);
        final double inv = 1.0D / cell;

        Int2ObjectOpenHashMap<int[]> grid = TL_GRID.get();
        grid.clear();
        for (int i = 0; i < n; i++) {
            ItemEntity e = items[i];
            int key = pack(cellOf(e.getX(), inv), cellOf(e.getZ(), inv));
            int[] a = grid.get(key);
            if (a == null) {
                a = new int[4];
                a[1] = i;
                a[0] = 1;
                grid.put(key, a);
            } else {
                if (a[0] == a.length - 1) {
                    a = java.util.Arrays.copyOf(a, a.length * 2);
                    grid.put(key, a);
                }
                a[++a[0]] = i;
            }
        }

        // Sources in snapshot order (= vanilla intra-bucket tick order).
        for (int si = 0; si < n; si++) {
            ItemEntity src = items[si];
            if (src.isRemoved()) {
                continue; // absorbed by an earlier source this sweep
            }
            int scx = cellOf(src.getX(), inv);
            int scz = cellOf(src.getZ(), inv);
            boolean stop = false;
            for (int cz = scz - 1; cz <= scz + 1 && !stop; cz++) {
                for (int cx = scx - 1; cx <= scx + 1 && !stop; cx++) {
                    int[] cellList = grid.get(pack(cx, cz));
                    if (cellList == null) {
                        continue;
                    }
                    for (int k = 1, m = cellList[0]; k <= m; k++) {
                        int ci = cellList[k];
                        if (ci == si) {
                            continue;
                        }
                        ItemEntity cand = items[ci];
                        // vanilla: query predicate + in-loop recheck
                        if (cand.isRemoved() || !isMergable(cand)) {
                            continue;
                        }
                        // vanilla: the query's AABB filter (exact intervals)
                        if (!contact(src, cand, r, yr)) {
                            continue;
                        }
                        // vanilla: fixItemsMergingThroughWalls clip gate
                        if (level.paperConfig().fixes.fixItemsMergingThroughWalls
                                && level.clipDirect(src.position(), cand.position(),
                                        net.minecraft.world.phys.shapes.CollisionContext.of(src))
                                == net.minecraft.world.phys.HitResult.Type.BLOCK) {
                            continue;
                        }
                        tryMerge(src, cand);
                        if (src.isRemoved()) { // vanilla break semantics
                            stop = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    // ==================================================================
    // Vanilla-mirror eligibility (sources only). Candidates are NOT gated
    // by activation — the vanilla query does not check it either.
    // ==================================================================

    private static boolean sourceEligible(ItemEntity e) {
        if (!isMergable(e)) {
            return false;
        }
        if (!ActivationRange.checkIfActive(e)) {
            return false; // inactive items never reach ItemEntity.tick
        }
        int t = e.tickCount + 1; // baseTick increments before the merge site
        if (t % 40 == 0) {
            return true;
        }
        if (t % 2 != 0) {
            return false;
        }
        // stride-2 movers: mirror the block-boundary crossing with LAST
        // movement's pre/post positions (this tick's move has not happened
        // yet at sweep time — documented ±1 tick transition deviation).
        return Mth.floor(e.xo) != Mth.floor(e.getX())
                || Mth.floor(e.yo) != Mth.floor(e.getY())
                || Mth.floor(e.zo) != Mth.floor(e.getZ());
    }

    // ==================================================================
    // Exact vanilla contact test: query box bb.inflate(R, yR, R) vs the
    // candidate's own box, as AABB.intersects does (strict comparisons).
    // ==================================================================

    private static boolean contact(ItemEntity src, ItemEntity cand, double r, double yr) {
        AABB s = src.getBoundingBox();
        AABB c = cand.getBoundingBox();
        return c.maxX > s.minX - r && c.minX < s.maxX + r
                && c.maxY > s.minY - yr && c.minY < s.maxY + yr
                && c.maxZ > s.minZ - r && c.minZ < s.maxZ + r;
    }

    // ==================================================================
    // Vanilla merge decision + application, transcribed bit-for-bit from
    // javap (tryToMerge + static merge(ItemEntity,ItemStack,ItemEntity,
    // ItemStack)): same direction rule, same event, same call order.
    // ==================================================================

    private static void tryMerge(ItemEntity src, ItemEntity other) {
        ItemStack is1 = src.getItem();
        ItemStack is2 = other.getItem();
        if (!Objects.equals(src.target, other.target)) {
            return;
        }
        if (!ItemEntity.areMergable(is1, is2)) {
            return;
        }
        if (is2.getCount() < is1.getCount()) {
            applyMerge(src, is1, other, is2);
        } else {
            applyMerge(other, is2, src, is1);
        }
    }

    private static void applyMerge(ItemEntity recv, ItemStack recvStack, ItemEntity srcEnt, ItemStack srcStack) {
        if (!CraftEventFactory.callItemMergeEvent(recv, srcEnt)) {
            return;
        }
        recv.setItem(ItemEntity.merge(recvStack, srcStack, 64));
        recv.pickupDelay = Math.max(recv.pickupDelay, srcEnt.pickupDelay);
        recv.age = Math.min(recv.age, srcEnt.age);
        if (srcStack.isEmpty()) {
            srcEnt.discard(EntityRemoveEvent.Cause.MERGE);
        }
    }

    /** Private isMergable via a stable MethodHandle (despawnRate is private). */
    private static boolean isMergable(ItemEntity e) {
        try {
            return (boolean) MH_IS_MERGABLE.invokeExact(e);
        } catch (Throwable t) {
            throw new RuntimeException(t);
        }
    }

    // ==================================================================
    // Grid helpers (int hash: two 16-bit signed-wrapped cell coords).
    // Collisions require two cells 65536 apart (~49k blocks) inside one
    // 8x8-chunk bucket slot — impossible.
    // ==================================================================

    private static int cellOf(double v, double inv) {
        return (int) Math.floor(v * inv);
    }

    private static int pack(int cx, int cz) {
        return ((cx & 0xFFFF) << 16) | (cz & 0xFFFF);
    }
}
