package net.minecraft.world.entity.item;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.Arrays;
import java.util.List;
import java.util.function.ObjLongConsumer;
import java.util.function.Predicate;

import net.minecraft.util.Mth;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.HitResult;
import net.minecraft.world.phys.shapes.CollisionContext;

/**
 * ItemMergeOps (MEGA-ROUND-2, TASK-397-D — lever `items_offthread`).
 *
 * OFF-THREAD STAGE OF THE ITEM MERGE SCAN. The vanilla per-item
 * mergeWithNeighbours() does a Level.getEntitiesOfClass(ItemEntity, box,
 * predicate) broadphase query on the tick path. This lever moves the CANDIDATE
 * SCAN off the critical path entirely: the RegionTickOps worker that ticked a
 * bucket sweeps its OWN bucket right after the entity sweep (post-sweep
 * positions = exactly the state each item saw at its own tick), produces
 * merge DECISIONS into a per-slot buffer, and the MAIN THREAD applies the
 * decisions after the DONE barrier (post-join, phase inactive = vanilla
 * EntityTickList mutation protocol, no worker races).
 *
 * Parity contract (vanilla semantics, deviations documented in LEVER.md):
 *  - the only kernel bytecode change is the tick()V call site
 *    mergeWithNeighbours()V -> ItemMergeOps.tickMerge(ItemEntity)V (both
 *    fail-closed: dormant lever = untouched vanilla bytes; armed lever with
 *    unwired hooks = tickMerge falls back to a public-API replica of the
 *    vanilla body below);
 *  - candidate query semantics replicated bit-exact: query box =
 *    bbox.inflate(spigotConfig.itemMerge, onlyMergeItemsHorizontally ? 0 :
 *    itemMerge - 0.5, itemMerge), predicate other != self && other.isMergable(),
 *    exact AABB.intersects post-filter, Paper per-item cadence gate
 *    (moved-this-block ? 2 : 40) and isMergable self-gate;
 *  - the merge ITSELF is the real vanilla private tryToMerge (MethodHandle,
 *    setAccessible at bind time): stack direction (smaller count absorbs...
 *    larger-count receiver), areMergable sum<=max + isSameItemSameComponents,
 *    target-UUID equality, discard/pickupDelay/age side effects are the
 *    kernel's own bytes, not a reimplementation;
 *  - decisions apply in deterministic order: per-slot scan order (= bucket
 *    snapshot order = vanilla insertion order), slots 0..W-1.
 *
 * Deviations (documented, vanilla-semantics-preserving over a window):
 *  - a decision applies at end of the item's merge-eligible tick instead of
 *    mid-sweep (<= 1 tick shift);
 *  - cross-bucket neighbours (items whose boxes straddle a region bucket
 *    boundary) are discovered on a later cadence tick;
 *  - merged-away items may complete their own physics/age tick before the
 *    end-of-tick discard.
 *
 * Zero steady-state allocation: every scan buffer is persistent per slot,
 * grown on overflow, reference-hygiene nulled after use (S7-158c pattern).
 */
public final class ItemMergeOps {

    /** Set true only after both hooks are wired into RegionTickOps. */
    public static volatile boolean ARMED = false;

    private static volatile ObjLongConsumer<Entity[]> scanHook;
    private static volatile Runnable applyHook;
    private static volatile int maxSlot = -1;

    private static MethodHandle isMergableMH;
    private static MethodHandle tryToMergeMH;

    /** Per-slot persistent scan/decision buffers (slot = region bucket id). */
    static final class SlotState {
        ItemEntity[] items = new ItemEntity[0];
        int nItems;
        long[] keys = new long[0];
        long[] cells = new long[0];
        int nCells;
        int[] cellStart = new int[0];
        int[] cursor = new int[0];
        ItemEntity[] ordered = new ItemEntity[0];
        int placedCount;
        ItemEntity[] decA = new ItemEntity[0];
        ItemEntity[] decB = new ItemEntity[0];
        int nDec;
    }

    private static final SlotState[] SLOTS = new SlotState[64];
    private static final Object SLOT_LOCK = new Object();

    /** Cell edge for the spatial bucket hash; query box span <= 2 with defaults. */
    private static final double CELL = 2.0D;
    /**
     * Coverage pad: candidate CENTER cell must fall inside the enumerated
     * range. ItemEntity boxes are 0.25 wide/high (half-extent 0.125); 0.25
     * doubles the true pad on purpose (safety against dimension drift).
     */
    private static final double PAD = 0.25D;

    private ItemMergeOps() {
    }

    // ------------------------------------------------------------------
    // Kernel retarget point: ItemEntity.tick()V calls this instead of the
    // private mergeWithNeighbours()V when the lever is armed (invokestatic,
    // receiver-prepended — identical stack shape). Vanilla fallback below.
    // ------------------------------------------------------------------
    public static void tickMerge(ItemEntity self) {
        if (ARMED) {
            return; // scan ran off-thread; decisions applied post-barrier
        }
        vanillaMergeWithNeighbours(self);
    }

    /** Public-API replica of the vanilla private body (fail-closed fallback). */
    private static void vanillaMergeWithNeighbours(ItemEntity self) {
        try {
            if (isMergableMH == null || !isMergable(self)) {
                return;
            }
            Level lvl = self.level();
            if (lvl == null) {
                return;
            }
            double d = lvl.spigotConfig.itemMerge;
            boolean onlyH = lvl.paperConfig().entities.behavior.onlyMergeItemsHorizontally;
            AABB box = self.getBoundingBox().inflate(d, onlyH ? 0.0D : d - 0.5D, d);
            boolean walls = lvl.paperConfig().fixes.fixItemsMergingThroughWalls;
            Predicate<ItemEntity> pred = o -> o != self && isMergable(o);
            List<ItemEntity> list = lvl.getEntitiesOfClass(ItemEntity.class, box, pred);
            for (int i = 0; i < list.size(); i++) {
                ItemEntity other = list.get(i);
                if (!isMergable(other)) {
                    continue;
                }
                if (walls && lvl.clipDirect(self.position(), other.position(),
                        CollisionContext.of(self)) == HitResult.Type.BLOCK) {
                    continue;
                }
                tryToMergeMH.invokeExact(self, other);
                if (self.isRemoved()) {
                    return;
                }
            }
        } catch (Throwable t) {
            logOnce("vanilla fallback path error", t);
        }
    }

    // ------------------------------------------------------------------
    // Worker side: scan ONE region bucket right after its entity sweep.
    // Hooked via RegionTickOps.postBucketHook (bucket, slot<<32|len).
    // Reads only this worker's own bucket = no cross-thread entity access.
    // ------------------------------------------------------------------
    static void scanBucket(Entity[] bucket, long slotLen) {
        if (!ARMED) {
            return;
        }
        int slot = (int) (slotLen >>> 32);
        int len = (int) slotLen;
        if (bucket == null || slot < 0 || slot >= SLOTS.length) {
            return;
        }
        SlotState s = SLOTS[slot];
        if (s == null) {
            s = ensureSlot(slot);
        }
        if (s.nDec != 0 || s.nItems != 0) {
            return; // previous frame not drained — skip rather than corrupt
        }

        double itemMerge = 0.5D;
        boolean onlyH = false;
        boolean cfg = false;

        // Pass 1: mergeable items on their vanilla merge cadence, in bucket
        // snapshot order (= vanilla insertion order).
        for (int i = 0; i < len; i++) {
            Entity e = bucket[i];
            if (!(e instanceof ItemEntity it)) {
                continue;
            }
            if (!cfg) {
                Level lvl = it.level();
                if (lvl != null) {
                    itemMerge = lvl.spigotConfig.itemMerge;
                    onlyH = lvl.paperConfig().entities.behavior.onlyMergeItemsHorizontally;
                    cfg = true;
                }
            }
            if (!isMergable(it)) {
                continue;
            }
            // Paper cadence gate, replicated from ItemEntity.tick: the rate
            // selector compares floored prev/current positions.
            boolean moved = Mth.floor(it.xo) != Mth.floor(it.getX())
                    || Mth.floor(it.yo) != Mth.floor(it.getY())
                    || Mth.floor(it.zo) != Mth.floor(it.getZ());
            int rate = moved ? 2 : 40;
            if (it.tickCount % rate != 0) {
                continue;
            }
            if (s.nItems == s.items.length) {
                s.items = Arrays.copyOf(s.items, Math.max(16, s.nItems * 2));
            }
            s.items[s.nItems++] = it;
        }
        int n = s.nItems;
        if (n == 0) {
            return;
        }

        // Pass 2: stable cell placement (counting placement by center cell;
        // within-cell order = snapshot order => deterministic).
        if (s.keys.length < n) {
            s.keys = new long[Math.max(16, n * 2)];
        }
        for (int i = 0; i < n; i++) {
            ItemEntity it = s.items[i];
            s.keys[i] = cellKey(it.getX(), it.getY(), it.getZ());
        }
        if (s.cells.length < n) {
            s.cells = new long[Math.max(16, n * 2)];
        }
        System.arraycopy(s.keys, 0, s.cells, 0, n);
        Arrays.sort(s.cells, 0, n);
        int c = 0;
        for (int i = 0; i < n; i++) {
            if (c == 0 || s.cells[c - 1] != s.cells[i]) {
                s.cells[c++] = s.cells[i];
            }
        }
        s.nCells = c;
        if (s.cellStart.length < c + 1) {
            s.cellStart = new int[Math.max(16, c + 2)];
            s.cursor = new int[s.cellStart.length];
        }
        for (int i = 0; i <= c; i++) {
            s.cellStart[i] = 0;
        }
        for (int j = 0; j < n; j++) {
            int idx = Arrays.binarySearch(s.cells, 0, c, s.keys[j]);
            s.cellStart[idx + 1]++;
        }
        for (int i = 0; i < c; i++) {
            s.cellStart[i + 1] += s.cellStart[i];
        }
        System.arraycopy(s.cellStart, 0, s.cursor, 0, c);
        if (s.ordered.length < n) {
            s.ordered = new ItemEntity[Math.max(16, n * 2)];
        }
        for (int j = 0; j < n; j++) {
            int idx = Arrays.binarySearch(s.cells, 0, c, s.keys[j]);
            s.ordered[s.cursor[idx]++] = s.items[j];
        }
        s.placedCount = n;

        // Pass 3: candidate enumeration + exact post-filters => decisions.
        double yInf = onlyH ? 0.0D : itemMerge - 0.5D;
        for (int i = 0; i < n; i++) {
            ItemEntity a = s.items[i];
            AABB qbox = a.getBoundingBox().inflate(itemMerge, yInf, itemMerge);
            long c0x = (long) Math.floor((qbox.minX - PAD) / CELL);
            long c1x = (long) Math.floor((qbox.maxX + PAD) / CELL);
            long c0y = (long) Math.floor((qbox.minY - PAD) / CELL);
            long c1y = (long) Math.floor((qbox.maxY + PAD) / CELL);
            long c0z = (long) Math.floor((qbox.minZ - PAD) / CELL);
            long c1z = (long) Math.floor((qbox.maxZ + PAD) / CELL);
            for (long cx = c0x; cx <= c1x; cx++) {
                for (long cy = c0y; cy <= c1y; cy++) {
                    for (long cz = c0z; cz <= c1z; cz++) {
                        int idx = cellIndex(s, pack(cx, cy, cz));
                        if (idx < 0) {
                            continue;
                        }
                        int from = s.cellStart[idx];
                        int to = s.cellStart[idx + 1];
                        for (int p = from; p < to; p++) {
                            ItemEntity j = s.ordered[p];
                            if (j == a) {
                                continue;
                            }
                            if (!j.getBoundingBox().intersects(qbox)) {
                                continue;
                            }
                            if (!ItemEntity.areMergable(a.getItem(), j.getItem())) {
                                continue;
                            }
                            if (s.nDec == s.decA.length) {
                                int cap = Math.max(16, s.nDec * 2);
                                s.decA = Arrays.copyOf(s.decA, cap);
                                s.decB = Arrays.copyOf(s.decB, cap);
                            }
                            s.decA[s.nDec] = a;
                            s.decB[s.nDec] = j;
                            s.nDec++;
                        }
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Main thread: apply decisions after the join barrier (phase inactive,
    // vanilla EntityTickList mutation protocol). Order: slot 0..W-1, then
    // per-slot scan order. Deterministic; guards re-validate live state so
    // every applied merge is a vanilla-legal merge at apply time.
    // ------------------------------------------------------------------
    static void applyAll() {
        if (!ARMED) {
            return;
        }
        int mx = maxSlot;
        for (int slot = 0; slot <= mx; slot++) {
            SlotState s = SLOTS[slot];
            if (s == null) {
                continue;
            }
            try {
                for (int k = 0; k < s.nDec; k++) {
                    ItemEntity a = s.decA[k];
                    ItemEntity b = s.decB[k];
                    if (a.isRemoved() || b.isRemoved()) {
                        continue;
                    }
                    if (!isMergable(a) || !isMergable(b)) {
                        continue;
                    }
                    Level lvl = a.level();
                    if (lvl != null && lvl.paperConfig().fixes.fixItemsMergingThroughWalls
                            && lvl.clipDirect(a.position(), b.position(),
                                    CollisionContext.of(a)) == HitResult.Type.BLOCK) {
                        continue;
                    }
                    try {
                        tryToMergeMH.invokeExact(a, b);
                    } catch (Throwable t) {
                        logOnce("tryToMerge invoke failed", t);
                    }
                }
            } finally {
                for (int k = 0; k < s.nDec; k++) {
                    s.decA[k] = null;
                    s.decB[k] = null;
                }
                s.nDec = 0;
                for (int k = 0; k < s.nItems; k++) {
                    s.items[k] = null;
                }
                s.nItems = 0;
                for (int k = 0; k < s.placedCount; k++) {
                    s.ordered[k] = null;
                }
                s.placedCount = 0;
                s.nCells = 0;
            }
        }
    }

    private static boolean isMergable(ItemEntity e) {
        try {
            return (boolean) isMergableMH.invokeExact(e);
        } catch (Throwable t) {
            return false;
        }
    }

    private static SlotState ensureSlot(int slot) {
        synchronized (SLOT_LOCK) {
            SlotState s = SLOTS[slot];
            if (s == null) {
                s = new SlotState();
                SLOTS[slot] = s;
            }
            if (slot > maxSlot) {
                maxSlot = slot;
            }
            return s;
        }
    }

    private static int cellIndex(SlotState s, long key) {
        return Arrays.binarySearch(s.cells, 0, s.nCells, key);
    }

    private static long cellKey(double x, double y, double z) {
        return pack(
                (long) Math.floor(x / CELL),
                (long) Math.floor(y / CELL),
                (long) Math.floor(z / CELL));
    }

    private static long pack(long cx, long cy, long cz) {
        return (cx & 0x1FFFFFL) | ((cy & 0x1FFFFFL) << 21) | ((cz & 0x1FFFFFL) << 42);
    }

    private static volatile boolean loggedOnce = false;

    private static void logOnce(String what, Throwable t) {
        if (!loggedOnce) {
            loggedOnce = true;
            System.err.println("[crussty-plugin] items_offthread: " + what + ": " + t);
        }
    }

    // ------------------------------------------------------------------
    // Bind + wire. Static init runs when Rust defines the class into the
    // kernel loader (after boot, quiet loader). RegionTickOps may not be
    // defined yet — retry on a daemon; on permanent failure ARMED stays
    // false and tickMerge serves the vanilla fallback (parity-safe).
    // ------------------------------------------------------------------
    static {
        try {
            ClassLoader cl = ItemMergeOps.class.getClassLoader();
            Class<?> iec = Class.forName("net.minecraft.world.entity.item.ItemEntity", false, cl);
            MethodHandles.Lookup lk = MethodHandles.lookup();
            Method im = iec.getDeclaredMethod("isMergable");
            im.setAccessible(true);
            isMergableMH = lk.unreflect(im);
            Method tm = iec.getDeclaredMethod("tryToMerge", ItemEntity.class);
            tm.setAccessible(true);
            tryToMergeMH = lk.unreflect(tm);
        } catch (Throwable t) {
            isMergableMH = null;
            tryToMergeMH = null;
            System.err.println("[crussty-plugin] items_offthread: MethodHandle bind failed: " + t);
        }
        Thread arm = new Thread(() -> {
            for (int i = 0; i < 30 && !ARMED; i++) {
                try {
                    wire();
                    return;
                } catch (Throwable t) {
                    try {
                        Thread.sleep(2000L);
                    } catch (InterruptedException ie) {
                        return;
                    }
                }
            }
            if (!ARMED) {
                System.err.println(
                        "[crussty-plugin] items_offthread: RegionTickOps wiring failed; vanilla fallback stays active");
            }
        }, "crussty-items-offthread-arm");
        arm.setDaemon(true);
        arm.start();
    }

    private static void wire() throws Exception {
        ClassLoader cl = ItemMergeOps.class.getClassLoader();
        Class<?> rtc = Class.forName("net.minecraft.world.entity.RegionTickOps", false, cl);
        Field f1 = rtc.getField("postBucketHook");
        Field f2 = rtc.getField("postJoinHook");
        scanHook = ItemMergeOps::scanBucket;
        applyHook = ItemMergeOps::applyAll;
        f1.set(null, scanHook);
        f2.set(null, applyHook);
        ARMED = true;
        System.err.println("[crussty-plugin] items_offthread: hooks wired into RegionTickOps, ARMED");
    }
}
