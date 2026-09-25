package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.common.util.WorldUtil;
import ca.spottedleaf.moonrise.patches.chunk_system.level.ChunkSystemLevel;
import ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices;
import ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup;
import net.minecraft.server.MinecraftServer;
import net.minecraft.server.level.FullChunkStatus;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.Mth;
import net.minecraft.world.phys.AABB;

import java.util.List;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Predicate;

/**
 * TASK-405-C (vector eindex — levers cmp405_eindex | cmp458_roar): bulk-synced
 * Rust mirror of the moonrise chunk-system entity storage, used as the
 * candidate plane for EntityLookup.getEntities*.
 *
 * TASK-458-K (ID-H04 + ID-H06, lever cmp458_roar): the rust plane gains
 * PER-SECTION (16³) occupancy — the note stream already carries the vanilla
 * sectionY (addEntity/removeEntity/moveEntity sites pass the exact value
 * vanilla uses: javap-verified clamp(blockY>>4, minSection, maxSection), the
 * same formula seedAll applies — so the section gate cannot false-negative by
 * construction) and a 4KB insert-only bloom of ever-occupied chunks pre-gates
 * each rect-chunk count inside the SAME single fused bulk JNI (no java
 * allocations, one bulk call per tick — the counts semantics of this class
 * are UNCHANGED: java still only skips chunks whose count is 0).
 *
 * CONTRACT (javap, patched-kernel.jar round-j2b, verified 2026-09-21):
 *  - EntityLookup.getEntities(Entity,AABB,List,Predicate) walks
 *    region-Z → region-X → chunk-z → chunk-x over the rect
 *    ((floor(minX)-2)>>4 .. (floor(maxX)+2)>>4) × (same for z), calling
 *    ChunkEntitySlices.getEntities for every slices with
 *    status.isOrAfter(FULL). The other three redirected bodies
 *    (getHardCollidingEntities, EntityType/Class variants) share the shape
 *    (the trailing-int overloads keep their own bodies — vanilla).
 *  - ChunkEntitySlices.addEntity(Entity,int)Z / removeEntity(Entity,int)Z
 *    have exactly one call site each inside EntityLookup.addEntity(ZZ) /
 *    removeEntity(Entity), and one of each inside moveEntity (the re-home;
 *    fires on EVERY section change — including a y-only one in one chunk).
 *  - Entity bb writes funnel through exactly 5 invokevirtual setBoundingBox
 *    sites (Entity.setPosRaw(DDDZ), Shulker.onSyncedDataUpdated,
 *    HangingEntity/LeashFenceKnotEntity.recalculateBoundingBox,
 *    Interaction.readAdditionalSaveData(ValueInput)).
 *
 * DELIVERY (counts-skip v1): this bridge NEVER changes the result list —
 * candidate chunks are still scanned by the VANILLA ChunkEntitySlices.getEntities
 * (bit-for-bit order/filters/dedup/predicate). The Rust plane
 * (src/entity_index.rs) only answers "which rect chunks hold ≥1 candidate":
 *  - note sites (retargeted invokevirtuals) record id/op/bb/cell rows into a
 *    per-thread buffer — zero JNI per note;
 *  - every query first drains ALL published buffers (its own + foreign
 *    region threads' — the busy/n publish protocol guarantees no note is
 *    ever lost or skipped: a foreign note at worst drains twice, and every
 *    op is idempotent) and gets the per-chunk candidate counts in ONE fused
 *    JNI (eidxFlushQuery); chunks whose count is 0 skip the region/status/
 *    section/storage walk entirely. The rust count is a SUPERSET: same
 *    pos-section home membership (cells come from the vanilla call sites
 *    themselves) + relaxed (non-strict) AABB span test — bit-exact strict
 *    AABB.intersects is a strict subset, so count==0 implies vanilla's scan
 *    would have contributed nothing;
 *  - chunks with count>0 are handled by vanilla itself.
 *
 * FAIL-CLOSED: ENABLED (env == "cmp405_eindex", STRICT eq, baked at define
 * time) && ARMED (rust manager flips it after seed+patches) && !broken.
 * Any native ERR_STRUCT → broken=true → every query takes vanillaReplica
 * (exact public-API replication of the vanilla body) — bit-for-bit vanilla,
 * permanently. ERR_RANGE (absurd rect / seqlock retry exhaustion) → per-call
 * replica only.
 *
 * Vanilla with an empty/foreign lever flag: this class is never defined and
 * no bytecode patch is served at all (byte-indistinguishable from vanilla).
 */
public final class EntityIndexOps {

    private EntityIndexOps() {}

    // ------------------------------------------------------------------
    // Gate (STRICT eq, baked into the classfile at <clinit>).
    // ------------------------------------------------------------------
    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        String v = f == null ? "" : f.trim();
        // TASK-458-K: cmp458_roar arms the SAME subsystem (roaring section
        // occupancy + bloom live inside the rust mirror; law 6).
        return "cmp405_eindex".equals(v) || "cmp458_roar".equals(v);
    }

    private static final boolean ENABLED = leverEnabled();

    /** Flipped by the rust manager AFTER seed + all patches are served. */
    public static volatile boolean ARMED = false;

    /** Sticky disarm after ERR_STRUCT (mirror corrupted — vanilla forever). */
    public static volatile boolean broken = false;

    /** Called by the rust manager after seed + patch service. */
    public static void armNow() {
        ARMED = true;
    }

    /** Fail-closed disarm (rare-funnel shape mismatch / structural defect). */
    public static void breakNow() {
        broken = true;
    }

    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    // ------------------------------------------------------------------
    // Per-thread dirty-op buffer (record-only hot path, no JNI per note).
    //
    // Publish protocol (no lost notes across region threads):
    //   owner note : spin on busy → read n (acquire) → write row (plain)
    //                → publish n (release);
    //   drainer    : CAS busy → read n (acquire) → copy rows → n=0
    //                → busy=false (release).
    // A note published after a drain is re-published on top by its owner
    // (n only grows past any reset), so worst case an op drains twice;
    // every op is idempotent (ADD=upsert, REMOVE=noop-if-absent, BB=set).
    // On x86 the note path is plain MOVs (no locked op) — ~ns per note.
    // ------------------------------------------------------------------
    private static final int CAP = 8192;
    private static final int REG_CAP = 256;

    static final byte OP_BB = 0;
    static final byte OP_ADD = 1;
    static final byte OP_REMOVE = 2;

    static final class Buf {
        final AtomicBoolean busy = new AtomicBoolean(false);
        final AtomicInteger n = new AtomicInteger(0);
        final int[] id = new int[CAP];
        final byte[] op = new byte[CAP];
        /** 6 doubles per op: minX,minY,minZ,maxX,maxY,maxZ. */
        final double[] bb = new double[6 * CAP];
        /** 3 ints per op: chunkX, chunkZ, sectionY (ADD only; else 0). */
        final int[] cell = new int[3 * CAP];
        /** Per-chunk candidate counts (rect row-major, grow-on-demand). */
        int[] counts = new int[256];
    }

    private static final Buf[] REGISTRY = new Buf[REG_CAP];
    private static final AtomicInteger REG_SIZE = new AtomicInteger(0);

    private static final ThreadLocal<Buf> T_BUF = ThreadLocal.withInitial(() -> {
        Buf b = new Buf();
        int i = REG_SIZE.getAndIncrement();
        if (i >= 0 && i < REG_CAP) {
            REGISTRY[i] = b;
        }
        return b;
    });

    private static void record(Buf b, int op, Entity e, int cx, int cz, int secY, AABB box) {
        while (b.busy.getAcquire()) {
            Thread.onSpinWait();
        }
        int i = b.n.getAcquire();
        if (i >= CAP) {
            flushOps(b);
            i = b.n.getAcquire();
        }
        b.id[i] = e.getId();
        b.op[i] = (byte) op;
        int o6 = 6 * i;
        if (box != null) {
            b.bb[o6] = box.minX;
            b.bb[o6 + 1] = box.minY;
            b.bb[o6 + 2] = box.minZ;
            b.bb[o6 + 3] = box.maxX;
            b.bb[o6 + 4] = box.maxY;
            b.bb[o6 + 5] = box.maxZ;
        }
        int o3 = 3 * i;
        b.cell[o3] = cx;
        b.cell[o3 + 1] = cz;
        b.cell[o3 + 2] = secY;
        b.n.setRelease(i + 1);
    }

    private static void flushOps(Buf b) {
        int n = b.n.getAcquire();
        if (n == 0) {
            return;
        }
        int rc = eidxFlush(n, b.id, b.op, b.bb, b.cell);
        b.n.set(0);
        if (rc < 0 && rc == ERR_STRUCT) {
            // Mirror maintenance failed: the plane is no longer trustworthy.
            // Queries keep serving exact vanilla; counts become irrelevant.
            broken = true;
        }
    }

    /** Drain every OTHER published buffer into b (one JNI batch stays one). */
    private static void drainOthers(Buf b) {
        int reg = REG_SIZE.getAcquire();
        int lim = reg < REG_CAP ? reg : REG_CAP;
        for (int j = 0; j < lim; j++) {
            Buf o = REGISTRY[j];
            if (o == null || o == b) {
                continue;
            }
            if (o.n.getAcquire() == 0) {
                continue;
            }
            if (!o.busy.compareAndSet(false, true)) {
                continue; // owner mid-flush or another drainer — next query gets it
            }
            int on = o.n.getAcquire();
            if (on > 0) {
                if (b.n.getAcquire() + on > CAP) {
                    flushOps(b);
                }
                int dst = b.n.getAcquire();
                System.arraycopy(o.id, 0, b.id, dst, on);
                System.arraycopy(o.op, 0, b.op, dst, on);
                System.arraycopy(o.bb, 6 * dst, b.bb, 6 * dst, 6 * on);
                System.arraycopy(o.cell, 3 * dst, b.cell, 3 * dst, 3 * on);
                b.n.setRelease(dst + on);
                o.n.set(0);
            }
            o.busy.set(false);
        }
    }

    // ------------------------------------------------------------------
    // Note sites — retargeted invokevirtuals (record + delegate, the
    // delegated methods are NOT patched: no recursion).
    // ------------------------------------------------------------------

    /** ChunkEntitySlices.addEntity(Entity,int)Z call sites inside EntityLookup. */
    public static boolean noteAdd(ChunkEntitySlices slices, Entity e, int sectionY) {
        if (ENABLED) {
            record(T_BUF.get(), OP_ADD, e, slices.chunkX, slices.chunkZ, sectionY, e.getBoundingBox());
        }
        return slices.addEntity(e, sectionY);
    }

    /** ChunkEntitySlices.removeEntity(Entity,int)Z call sites inside EntityLookup. */
    public static boolean noteRemove(ChunkEntitySlices slices, Entity e, int sectionY) {
        if (ENABLED) {
            // TASK-458-K: pass the vanilla sectionY through (informational —
            // the rust mirror keeps its own authoritative s_sec per slot).
            record(T_BUF.get(), OP_REMOVE, e, slices.chunkX, slices.chunkZ, sectionY, null);
        }
        return slices.removeEntity(e, sectionY);
    }

    /** Entity.setBoundingBox(AABB)V invoke sites (5 classes, see class doc).
     * The CP class of each retargeted site is the OWNER class (javap
     * short-form reference), so each owner gets its own overload — the
     * delegating bodies keep the note path shared. */
    public static void noteBB(Entity e, AABB bb) {
        if (ENABLED) {
            record(T_BUF.get(), OP_BB, e, 0, 0, 0, bb);
        }
        e.setBoundingBox(bb);
    }

    public static void noteBB(net.minecraft.world.entity.monster.Shulker e, AABB bb) {
        noteBB((Entity) e, bb);
    }

    public static void noteBB(net.minecraft.world.entity.decoration.HangingEntity e, AABB bb) {
        noteBB((Entity) e, bb);
    }

    public static void noteBB(net.minecraft.world.entity.decoration.LeashFenceKnotEntity e, AABB bb) {
        noteBB((Entity) e, bb);
    }

    public static void noteBB(net.minecraft.world.entity.Interaction e, AABB bb) {
        noteBB((Entity) e, bb);
    }

    // ------------------------------------------------------------------
    // Seed (called once by the rust manager BEFORE ARMED flips): upsert
    // every live entity of every ServerLevel from an atomic snapshot
    // (getAllCopy — concurrent table copy, safe off-thread). Idempotent by
    // id — buffered notes racing the seed converge on the vanilla state.
    // Returns false → the manager must NOT arm (fail-closed vanilla).
    // ------------------------------------------------------------------
    public static int seedAll() {
        if (!ENABLED) {
            return 0;
        }
        try {
            for (ServerLevel level : MinecraftServer.getServer().getAllLevels()) {
                EntityLookup lookup = ((ChunkSystemLevel) (Object) level).moonrise$getEntityLookup();
                int minSec = WorldUtil.getMinSection(level);
                int maxSec = WorldUtil.getMaxSection(level);
                Entity[] all = lookup.getAllCopy();
                int[] ids = new int[4096];
                byte[] op = new byte[4096];
                double[] bb = new double[6 * 4096];
                int[] cell = new int[3 * 4096];
                int n = 0;
                for (Entity e : all) {
                    if (e == null) {
                        continue;
                    }
                    ids[n] = e.getId();
                    op[n] = OP_ADD;
                    net.minecraft.core.BlockPos pos = e.blockPosition();
                    int cx = pos.getX() >> 4;
                    int cz = pos.getZ() >> 4;
                    int secY = Mth.clamp(pos.getY() >> 4, minSec, maxSec);
                    AABB box = e.getBoundingBox();
                    int o6 = 6 * n;
                    bb[o6] = box.minX;
                    bb[o6 + 1] = box.minY;
                    bb[o6 + 2] = box.minZ;
                    bb[o6 + 3] = box.maxX;
                    bb[o6 + 4] = box.maxY;
                    bb[o6 + 5] = box.maxZ;
                    int o3 = 3 * n;
                    cell[o3] = cx;
                    cell[o3 + 1] = cz;
                    cell[o3 + 2] = secY;
                    n++;
                    if (n == 4096) {
                        if (eidxFlush(n, ids, op, bb, cell) < 0) {
                            return 0;
                        }
                        n = 0;
                    }
                }
                if (n > 0 && eidxFlush(n, ids, op, bb, cell) < 0) {
                    return 0;
                }
            }
            return 1;
        } catch (Throwable t) {
            return 0;
        }
    }

    // ------------------------------------------------------------------
    // Query bodies (the 4 redirected EntityLookup methods).
    // ------------------------------------------------------------------

    public static void getEntitiesE(EntityLookup lookup, Entity except, AABB box,
                                    List<Entity> out, Predicate<? super Entity> pred) {
        query(lookup, except, box, out, pred, 0, null, null);
    }

    public static void getHardCollidingE(EntityLookup lookup, Entity except, AABB box,
                                         List<Entity> out, Predicate<? super Entity> pred) {
        query(lookup, except, box, out, pred, 1, null, null);
    }

    public static void getEntitiesT(EntityLookup lookup, EntityType<?> type, AABB box,
                                    List<? super Entity> out, Predicate<? super Entity> pred) {
        query(lookup, null, box, out, pred, 2, type, null);
    }

    public static void getEntitiesC(EntityLookup lookup, Class<? extends Entity> cls, Entity except,
                                    AABB box, List<? super Entity> out, Predicate<? super Entity> pred) {
        query(lookup, except, box, out, pred, 3, null, cls);
    }

    private static void query(EntityLookup lookup, Entity except, AABB box, List<?> out,
                              Predicate<? super Entity> pred, int mode, EntityType<?> type,
                              Class<? extends Entity> cls) {
        if (!ENABLED || !ARMED || broken) {
            vanillaReplica(lookup, except, box, out, pred, mode, type, cls);
            return;
        }
        Buf b = T_BUF.get();
        int minCX = (Mth.floor(box.minX) - 2) >> 4;
        int minCZ = (Mth.floor(box.minZ) - 2) >> 4;
        int maxCX = (Mth.floor(box.maxX) + 2) >> 4;
        int maxCZ = (Mth.floor(box.maxZ) + 2) >> 4;
        int w = maxCX - minCX + 1;
        int h = maxCZ - minCZ + 1;
        if (w <= 0 || h <= 0 || w > 64 || h > 64 || (long) w * (long) h > 4096L) {
            // Absurd rect (far-land/portal abuse): per-call replica.
            vanillaReplica(lookup, except, box, out, pred, mode, type, cls);
            return;
        }
        if (b.counts.length < w * h) {
            b.counts = new int[Math.max(w * h, Integer.highestOneBit(w * h - 1) << 1)];
        }
        // One fused JNI: flush own + foreign published ops, then per-chunk
        // candidate counts for the rect.
        drainOthers(b);
        int minSec = WorldUtil.getMinSection(lookup.world);
        int maxSec = WorldUtil.getMaxSection(lookup.world);
        int rc = eidxFlushQuery(b.n.getAcquire(), b.id, b.op, b.bb, b.cell,
                box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ,
                minCX, minCZ, maxCX, maxCZ, minSec, maxSec, b.counts);
        b.n.set(0);
        if (rc < 0) {
            if (rc == ERR_STRUCT) {
                broken = true;
            }
            vanillaReplica(lookup, except, box, out, pred, mode, type, cls);
            return;
        }
        // Vanilla-order rect walk with counts-skip. Chunk visit order is the
        // vanilla body verbatim: region-Z → region-X → chunk-z → chunk-x;
        // candidate chunks are scanned by VANILLA slices.getEntities, so the
        // result list (order/filters/dedup/predicate) is bit-for-bit vanilla.
        int[] counts = b.counts;
        int minRX = minCX >> 5, minRZ = minCZ >> 5, maxRX = maxCX >> 5, maxRZ = maxCZ >> 5;
        for (int rz = minRZ; rz <= maxRZ; rz++) {
            int zStart = rz == minRZ ? (minCZ & 31) : 0;
            int zEnd = rz == maxRZ ? (maxCZ & 31) : 31;
            for (int rx = minRX; rx <= maxRX; rx++) {
                EntityLookup.ChunkSlicesRegion region = lookup.getRegion(rx, rz);
                if (region == null) {
                    continue;
                }
                int xStart = rx == minRX ? (minCX & 31) : 0;
                int xEnd = rx == maxRX ? (maxCX & 31) : 31;
                for (int z = zStart; z <= zEnd; z++) {
                    int rowBase = (z - minCZ) * w - minCX;
                    for (int x = xStart; x <= xEnd; x++) {
                        int cx = (rx << 5) | x;
                        if (counts[rowBase + cx] == 0) {
                            continue; // rust superset says: nothing here for sure
                        }
                        ChunkEntitySlices slices = region.get(x | (z << 5));
                        if (slices == null) {
                            continue;
                        }
                        if (!slices.status.isOrAfter(FullChunkStatus.FULL)) {
                            continue;
                        }
                        scanSlices(slices, except, box, out, pred, mode, type, cls);
                    }
                }
            }
        }
    }

    /** The per-chunk tail of the vanilla body (mode dispatch, public API). */
    private static void scanSlices(ChunkEntitySlices slices, Entity except, AABB box, List<?> out,
                                   Predicate<? super Entity> pred, int mode, EntityType<?> type,
                                   Class<? extends Entity> cls) {
        switch (mode) {
            case 0 -> slices.getEntities(except, box, (List<Entity>) out, pred);
            case 1 -> slices.getHardCollidingEntities(except, box, (List<Entity>) out, pred);
            case 2 -> slices.<Entity>getEntities(type, box, (List<? super Entity>) out, pred);
            default -> slices.<Entity>getEntities(cls, except, box, (List<? super Entity>) out, pred);
        }
    }

    /**
     * Exact public-API replication of the vanilla EntityLookup body (fallback
     * path): same rect, same region-Z → region-X → z → x order, same
     * FULL-status gate, vanilla per-chunk scans. Bit-for-bit vanilla results.
     */
    private static void vanillaReplica(EntityLookup lookup, Entity except, AABB box, List<?> out,
                                       Predicate<? super Entity> pred, int mode, EntityType<?> type,
                                       Class<? extends Entity> cls) {
        int minCX = (Mth.floor(box.minX) - 2) >> 4;
        int minCZ = (Mth.floor(box.minZ) - 2) >> 4;
        int maxCX = (Mth.floor(box.maxX) + 2) >> 4;
        int maxCZ = (Mth.floor(box.maxZ) + 2) >> 4;
        int minRX = minCX >> 5, minRZ = minCZ >> 5, maxRX = maxCX >> 5, maxRZ = maxCZ >> 5;
        for (int rz = minRZ; rz <= maxRZ; rz++) {
            int zStart = rz == minRZ ? (minCZ & 31) : 0;
            int zEnd = rz == maxRZ ? (maxCZ & 31) : 31;
            for (int rx = minRX; rx <= maxRX; rx++) {
                EntityLookup.ChunkSlicesRegion region = lookup.getRegion(rx, rz);
                if (region == null) {
                    continue;
                }
                int xStart = rx == minRX ? (minCX & 31) : 0;
                int xEnd = rx == maxRX ? (maxCX & 31) : 31;
                for (int z = zStart; z <= zEnd; z++) {
                    for (int x = xStart; x <= xEnd; x++) {
                        ChunkEntitySlices slices = region.get(x | (z << 5));
                        if (slices == null) {
                            continue;
                        }
                        if (!slices.status.isOrAfter(FullChunkStatus.FULL)) {
                            continue;
                        }
                        scanSlices(slices, except, box, out, pred, mode, type, cls);
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Natives (impl — src/entity_index.rs; registered by the rust manager).
    // ------------------------------------------------------------------
    private static native int eidxFlush(int nOps, int[] ids, byte[] ops, double[] bb, int[] cells);

    private static native int eidxFlushQuery(int nOps, int[] ids, byte[] ops, double[] bb, int[] cells,
                                             double minX, double minY, double minZ,
                                             double maxX, double maxY, double maxZ,
                                             int minCX, int minCZ, int maxCX, int maxCZ,
                                             int minSec, int maxSec, int[] outCounts);

    /** Probe (manager RegisterNatives sanity; magic "EIDX"). */
    private static native int eidxProbe();
}
