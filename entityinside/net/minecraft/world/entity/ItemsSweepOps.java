package net.minecraft.world.entity;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.util.Arrays;
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
 * ItemsSweepOps (ROUND-397, TASK-397-E — SWEEP-REARM lever items_sweep2:
 * sort-based sweep-line batch-merge, v2 of ROUND-396-C).
 *
 * v1 (396-C) design risks fixed here:
 *   1. NO hash grid at all — v1's Int2ObjectOpenHashMap<int[]> allocated a
 *      fresh int[4] per occupied cell per tick (plus copyOf growth) and did
 *      ~9 hash lookups per source in the 3x3 neighborhood. v2 sorts a
 *      primitive long[] (key = quantized-X << 20 | idx) in place — zero
 *      steady-state allocation, no fastutil, and a monotone X-window break
 *      (two-pointer sweep) that v1 could never take.
 *   2. Window = contact reach (r + 2*candHalfMax + quant slop), sized in the
 *      owner-mandated 1.0-2.0 band family rather than a maximal-cell 0.75
 *      grid; the exact vanilla AABB interval test still decides every pair
 *      (the window is a provable superset of the vanilla X-interval test for
 *      fixed-size item boxes).
 *   3. Each eligible pair is evaluated ONCE (earlier-in-X-order entity is
 *      the source), not twice via 3x3 symmetry — vanilla itself merges a
 *      pair at most once per tick (the post-merge isRemoved recheck).
 *   4. CANDIDATES = ALL mergeable items of the bucket (a 1-bit eligible flag
 *      rides in the sort key); only the SOURCE side is stride/activation-
 *      gated, exactly like vanilla (the vanilla query never gates its
 *      candidates by stride). v1 collected only stride-eligible items into
 *      its grid, silently delaying every resting-absorber pair by up to 40
 *      ticks and inflating live item counts — a plausible contributor to
 *      the v1 leg's load regression.
 *
 * Vanilla cost model (javap contract, patched-kernel 1.21.10) is unchanged
 * from 396-C: ItemEntity.tick fires mergeWithNeighbours at tickCount %
 * (moved?2:40) and EACH firing does level.getEntitiesOfClass(ItemEntity,
 * bb.inflate(itemMerge, yR, itemMerge)) — an O(section-scan) broadphase
 * query plus a fresh List, per eligible item, per firing.
 *
 * Suppression wiring is IDENTICAL to 396-C (proven to arm in CI): the single
 * ItemEntity.tick merge call site is retargeted to tickMerge (receiver-
 * prepended static, strict sites==1); in-phase calls return (the sweep owns
 * this tick's merges), out-of-phase calls fall back to the pristine vanilla
 * body via a privateLookup MethodHandle. The class is CO-DEFINED with
 * RegionTickOps (bridge_list); the env gate lives inside the ops class
 * (SWEEP static-init reads CRUSSTY_LEVER_FLAG): lever off = static-read
 * no-op. sweepBucket self-quarantines to vanilla on any internal failure.
 *
 * Parity (same class of deviations as 396-C, preregistered):
 *   - same candidate pairs (X/Z-window superset + exact vanilla AABB filter),
 *     same tryToMerge/merge application, same direction (smaller->bigger,
 *     equal -> candidate receives);
 *   - DOC-DEV: pair evaluation order within a tick is quantized-X order
 *     (deterministic: quant key, idx tiebreak) instead of the vanilla
 *     per-entity sequential scan — vanilla's own order is implementation-
 *     defined by EntitySectionStorage; units conserved, same terminal
 *     saturation per contact component;
 *   - DOC-DEV: stride mirror reads LAST movement's block crossing (±1 tick
 *     on rest<->motion transitions); the sweep sees pre-tick positions
 *     (contacts created by this tick's movement are caught next sweep).
 * despawn/pickup/baseTick untouched.
 *
 * MUST compile to exactly ONE classfile (no nested classes, non-capturing
 * lambdas only) — travel_diet one-classfile law.
 */
public final class ItemsSweepOps {

    private ItemsSweepOps() {
    }

    /**
     * Armed from the owner directive lever pipe (run_world3.sh exports
     * CRUSSTY_LEVER_FLAG/CRUSSTY_LEVER_ARG). Non-final so a mid-flight sweep
     * failure can self-quarantine the lever (fail-open to the vanilla query
     * path on the NEXT tick — loud, one-shot log).
     */
    public static boolean SWEEP = "items_sweep2"
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
    // Retargeted call site (the ONLY one): ItemEntity.tick merge site
    // -> invokestatic tickMerge.
    // ==================================================================

    /**
     * Receiver-prepended static replacement of the tick-time merge call.
     * In the region-parallel phase the batch sweep of this tick has already
     * applied every merge the vanilla query would have found, so the
     * per-item broadphase query is suppressed. Outside the phase the
     * pristine vanilla body runs via the MethodHandle.
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
    // the contact reach never crosses slot boundaries).
    // ==================================================================

    private static final ThreadLocal<ItemEntity[]> TL_ITEMS = ThreadLocal.withInitial(() -> new ItemEntity[4096]);
    private static final ThreadLocal<double[]> TL_XS = ThreadLocal.withInitial(() -> new double[4096]);
    private static final ThreadLocal<double[]> TL_ZS = ThreadLocal.withInitial(() -> new double[4096]);
    private static final ThreadLocal<long[]> TL_KEYS = ThreadLocal.withInitial(() -> new long[4096]);

    /** Quantization for the sort key: 1/256-block X buckets (world-safe). */
    private static final double KEY_SCALE = 256.0D;
    /** Offsets a negative quant into long space: |X|<=33.5M -> |q| < 2^33. */
    private static final long XOFF = 1L << 34;
    /** Index bits in the packed key (bucket len << 2^20 is impossible here). */
    private static final int IDX_CAP = 1 << 20;
    /** Eligible-source flag bit (bit 20 of the packed key, above the idx). */
    private static final long FLAG_BIT = 1L << 20;
    /** Covers quantization disorder (<= 1/256) + FP epsilon in the window. */
    private static final double WINDOW_SLOP = 0.01D;

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
            System.err.println("[crussty-items-sweep2] sweep failed once — lever quarantined to vanilla: " + t);
        }
    }

    private static void sweepBucketInner(Entity[] bucket, int len) {
        int cap = len > 4096 ? len : 4096;
        ItemEntity[] items = TL_ITEMS.get();
        double[] xs = TL_XS.get();
        double[] zs = TL_ZS.get();
        long[] keys = TL_KEYS.get();
        if (items.length < cap) {
            items = new ItemEntity[cap];
            TL_ITEMS.set(items);
            xs = new double[cap];
            TL_XS.set(xs);
            zs = new double[cap];
            TL_ZS.set(zs);
            keys = new long[cap];
            TL_KEYS.set(keys);
        }

        // Collect ALL ItemEntities of the bucket (every one is a candidate);
        // the stride/activation SOURCE gate is a flag bit, evaluated here
        // once per item (vanilla pays the same per-item branch in tick).
        int n = 0;
        int sources = 0;
        double candHalfMax = 0.0D;
        for (int i = 0; i < len; i++) {
            if (bucket[i] instanceof ItemEntity ie) {
                boolean eligible = sourceEligible(ie);
                items[n] = ie;
                AABB bb = ie.getBoundingBox();
                double cx = (bb.minX + bb.maxX) * 0.5D;
                double cz = (bb.minZ + bb.maxZ) * 0.5D;
                double hx = (bb.maxX - bb.minX) * 0.5D;
                if (hx > candHalfMax) {
                    candHalfMax = hx;
                }
                xs[n] = cx;
                zs[n] = cz;
                keys[n] = eligible ? FLAG_BIT : 0L; // eligibility rides here
                if (eligible) {
                    sources++;
                }
                n++;
                if (n >= IDX_CAP) {
                    // Impossible at bench populations; fail-open loudly.
                    throw new IllegalStateException("item slot exceeds key index capacity");
                }
            }
        }
        // ROUND-397-E2 (re-arm cost fix): no eligible source this tick means
        // the sweep could not merge anything even in principle (the source
        // gate mirrors vanilla's per-tick merge cadence) — skip the key pack
        // + sort + window scan entirely instead of paying them for a no-op.
        if (sources == 0) {
            return;
        }
        if (n < 2) {
            return;
        }

        Level level = items[0].level();
        double r = level.spigotConfig.itemMerge;
        boolean onlyHorizontal = level.paperConfig().entities.behavior.onlyMergeItemsHorizontally;
        double yr = onlyHorizontal ? 0.0D : r - 0.5D;
        boolean clipGate = level.paperConfig().fixes.fixItemsMergingThroughWalls;
        // Superset center-window: pair contacts iff |dx|,|dz| < r + h_src +
        // h_cand <= r + 2*candHalfMax; +slop covers quant disorder.
        final double window = r + 2.0D * candHalfMax + WINDOW_SLOP;
        final double scale = KEY_SCALE;

        // Sort by quantized X (flag+idx tiebreak => total deterministic
        // order). Primitive dual-pivot quicksort: in-place, zero allocation.
        for (int i = 0; i < n; i++) {
            keys[i] = (((long) Math.round(xs[i] * scale) + XOFF) << 21 | keys[i]) | i;
        }
        Arrays.sort(keys, 0, n);

        // Sweep-line: for each source (X order), scan forward while the
        // X gap is inside the window; break = monotone cutoff that the v1
        // hash grid could never take.
        for (int a = 0; a < n; a++) {
            long ka = keys[a];
            if ((ka & FLAG_BIT) == 0L) {
                continue; // not a source this tick (stride/activation gate)
            }
            int ia = (int) (ka & 0xFFFFF);
            ItemEntity src = items[ia];
            if (src.isRemoved()) {
                continue; // absorbed by an earlier source this sweep
            }
            double sx = xs[ia];
            double sz = zs[ia];
            for (int b = a + 1; b < n; b++) {
                int ib = (int) (keys[b] & 0xFFFFF);
                double dx = xs[ib] - sx;
                if (dx >= window) {
                    break;
                }
                ItemEntity cand = items[ib];
                // vanilla: query predicate + in-loop recheck
                if (cand.isRemoved() || !isMergable(cand)) {
                    continue;
                }
                double dz = zs[ib] - sz;
                if (dz >= window || dz <= -window) {
                    continue;
                }
                // vanilla: the query's AABB filter (exact intervals)
                if (!contact(src, cand, r, yr)) {
                    continue;
                }
                // vanilla: fixItemsMergingThroughWalls clip gate
                if (clipGate && level.clipDirect(src.position(), cand.position(),
                        net.minecraft.world.phys.shapes.CollisionContext.of(src))
                        == net.minecraft.world.phys.HitResult.Type.BLOCK) {
                    continue;
                }
                tryMerge(src, cand);
                if (src.isRemoved()) { // vanilla break semantics
                    break;
                }
            }
        }
    }

    // ==================================================================
    // Vanilla-mirror eligibility (sources only). Candidates are NOT gated
    // by activation — the vanilla query does not check it either.
    // ==================================================================

    private static boolean sourceEligible(ItemEntity e) {
        // ROUND-397-E2 (re-arm cost fix): the collect pass runs this per item
        // per tick over the whole bucket (~100k items). The tick-modulo /
        // block-crossing gates are pure arithmetic and already reject ~97%
        // of items — keep the MethodHandle (isMergable) and ActivationRange
        // calls BEHIND them. Pure predicates: order change cannot alter the
        // boolean result (parity by construction).
        int t = e.tickCount + 1; // baseTick increments before the merge site
        if (t % 40 != 0) {
            if (t % 2 != 0) {
                return false;
            }
            // stride-2 movers: mirror the block-boundary crossing with LAST
            // movement's pre/post positions (this tick's move has not
            // happened yet at sweep time — documented ±1 tick deviation).
            if (Mth.floor(e.xo) == Mth.floor(e.getX())
                    && Mth.floor(e.yo) == Mth.floor(e.getY())
                    && Mth.floor(e.zo) == Mth.floor(e.getZ())) {
                return false;
            }
        }
        if (!isMergable(e)) {
            return false;
        }
        if (!ActivationRange.checkIfActive(e)) {
            return false; // inactive items never reach ItemEntity.tick
        }
        return true;
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
}
