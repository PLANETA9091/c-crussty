package net.minecraft.world.level.block.state;

import ca.spottedleaf.moonrise.patches.collisions.shape.CachedShapeData;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockBehaviour.BlockStateBase;
import net.minecraft.world.phys.shapes.VoxelShape;
import sun.misc.Unsafe;

import java.lang.reflect.Field;
import java.util.concurrent.ConcurrentHashMap;

/**
 * S17 / ЛАБ-voxel (round-470): VoxelShape interning — Moonrise #195 style
 * dedupe of the kernel shape-cache segment (Л180m: "-36.8% of the shape-cache
 * segment = -44..-81MB total"), wired as lever {@code cmp470_voxelintern}.
 *
 * MECHANISM (single seam, fail-closed):
 *   {@code VoxelShape.moonrise$getCachedVoxelData()} is a PURE field read
 *   (kernel javap: aload_0/getfield/areturn, 4 bytes). The Rust side
 *   (src/voxel_intern.rs + classfile::patch_voxel_intern) rewrites that
 *   method body in-place to {@code aload_0; invokestatic
 *   VoxelShapeInternOps.cachedData(VoxelShape)CachedShapeData; areturn} —
 *   identical operand-stack shape, and because the BODY is patched (not the
 *   call sites) both the invokevirtual callers and the hot invokeinterface
 *   callers (CollisionUtil dispatches via the CollisionVoxelShape itable)
 *   land in the redirect.
 *
 *   {@link #cachedData} canonicalizes every CachedShapeData record through
 *   {@link #INTERN}: CachedShapeData is a RECORD whose equals/hashCode are
 *   content-based (kernel javap: record with final components voxelSet /
 *   full-box bounds / isEmpty / hasSingleAABB), so two records with equal
 *   components are freely interchangeable. On a dedupe hit the canonical
 *   (first-seen) record is written back into {@code VoxelShape.cachedShapeData}
 *   (private field, Unsafe offset) and returned — the duplicate becomes
 *   collectable. A content-equal substitute can never change observable
 *   behavior (immutable value object).
 *
 * TIMING-HOLE (Л187/Л198, the reason {@link #sweepNow} EXISTS): the kernel
 * computes the shape caches during boot clinit — BEFORE this plugin can arm
 * (arm = post boot+quiet, collide_batch timing canon). Without an explicit
 * post-arm sweep the whole boot-created population would stay un-interned
 * until each individual shape's first post-arm collision query: the leg
 * would measure a placebo (the x425-classes lesson). {@link #sweepNow} is
 * therefore invoked by the Rust activator IMMEDIATELY AFTER the retransform
 * (sweepNow() post-arm is MANDATORY, Л198) and force-initializes +
 * canonicalizes the blockstate plane: for every registered block state it
 * walks the state Cache collision shape plus the private
 * constantCollisionShape / occlusionShape / occlusionShapesByFace fields,
 * calls {@code moonrise$initCache()} on shapes still missing their
 * CachedShapeData (moving that lazy work OUT of the bench window, matching
 * the vanilla steady state where all caches exist by measurement time), and
 * canonicalizes each record through {@link #INTERN}.
 *
 * PARITY CONTRACT: CachedShapeData is an immutable record with value
 * semantics; replacing a reference with a content-equal record cannot alter
 * any getter result, hashCode/equals outcome, or downstream branch. The
 * redirect preserves the vanilla null path bit-exactly (a shape without a
 * CachedShapeData still yields null — vanilla never lazy-inits inside the
 * getter). Fail-closed: if the kernel renames any field this bridge touches,
 * the static initializer throws ExceptionInInitializerError, the Rust
 * define/init guard sees the failure and the lever never arms
 * (vanilla bit-for-bit).
 *
 * DELIVERY (S7-163 canon): this source MUST compile to exactly ONE classfile
 * (no nested classes — NCDFE canon, pinned by the build script count and the
 * voxelintern_source_declares_no_nested_classes Rust test). javap contract
 * checked offline: flat classfile, zero nested member classes.
 */
public final class VoxelShapeInternOps {

    private VoxelShapeInternOps() {}

    // ------------------------------------------------------------- offsets

    private static final Unsafe U;
    /** VoxelShape.cachedShapeData (private, non-final) — the intern slot. */
    private static final long OFF_CSD;
    /** BlockStateBase.cache (private) — per-state Cache holder. */
    private static final long OFF_CACHE;
    /** BlockStateBase.constantCollisionShape (private). */
    private static final long OFF_CONST;
    /** BlockStateBase.occlusionShape (private). */
    private static final long OFF_OCC;
    /** BlockStateBase.occlusionShapesByFace (private VoxelShape[]). */
    private static final long OFF_OCC_ARR;

    static {
        try {
            final Field f = Unsafe.class.getDeclaredField("theUnsafe");
            f.setAccessible(true);
            U = (Unsafe) f.get(null);
            OFF_CSD = U.objectFieldOffset(
                    VoxelShape.class.getDeclaredField("cachedShapeData"));
            final Class<?> bsb = Class.forName(
                    "net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase");
            OFF_CACHE = U.objectFieldOffset(bsb.getDeclaredField("cache"));
            OFF_CONST = U.objectFieldOffset(bsb.getDeclaredField("constantCollisionShape"));
            OFF_OCC = U.objectFieldOffset(bsb.getDeclaredField("occlusionShape"));
            OFF_OCC_ARR = U.objectFieldOffset(bsb.getDeclaredField("occlusionShapesByFace"));
        } catch (ReflectiveOperationException e) {
            // Fail-closed: lever must not arm on a renamed kernel (class javadoc).
            throw new ExceptionInInitializerError(e);
        }
    }

    // ---------------------------------------------------------- intern map

    /**
     * Content-keyed canonical registry. CachedShapeData is a record: its
     * equals/hashCode ARE the dedupe key (no extra key object allocated).
     * Cardinality is bounded by the DISTINCT shape population (blockstate
     * plane ~tens of thousands + runtime union shapes), while the raw record
     * population it dedupes is what Moonrise #195 measured at -36.8% of the
     * segment.
     */
    private static final ConcurrentHashMap<CachedShapeData, CachedShapeData> INTERN =
            new ConcurrentHashMap<>();

    /** Telemetry of the last sweepNow() (read via JNI static fields). */
    public static volatile int lastShapesSeen;
    /** Shapes force-initialized (moonrise$initCache) during the last sweep. */
    public static volatile int lastForced;
    /** Records replaced by their canonical instance during the last sweep. */
    public static volatile int lastInterned;

    /** Single-threaded (sweep-only) force-init counter. */
    private static int forcedCounter;

    // ------------------------------------------------- getter redirect (arm)

    /**
     * Whole-body replacement of VoxelShape.moonrise$getCachedVoxelData().
     * Vanilla null path preserved bit-exactly; the non-null path
     * canonicalizes through {@link #INTERN} and writes the canonical record
     * back into the shape (dedupe hit => the duplicate becomes garbage).
     */
    public static CachedShapeData cachedData(final VoxelShape holder) {
        final Object d = U.getObject(holder, OFF_CSD);
        if (d == null) {
            return null;                                   // vanilla: bare field read
        }
        final CachedShapeData cur = (CachedShapeData) d;
        final CachedShapeData canon = INTERN.putIfAbsent(cur, cur);
        if (canon == null || canon == cur) {
            return cur;                                    // first sighting / already canonical
        }
        U.putObject(holder, OFF_CSD, canon);               // dedupe hit: write-back
        return canon;
    }

    /**
     * Canonicalize ONE shape. Returns 1 when the shape's record was replaced
     * by an existing canonical instance, 0 otherwise.
     */
    private static int internOne(final VoxelShape s) {
        if (s == null) {
            return 0;
        }
        Object d = U.getObject(s, OFF_CSD);
        if (d == null) {
            // Boot left this shape un-initialized; the vanilla lazy creator
            // (CollisionDiscreteVoxelShape.moonrise$getOrCreateCachedShapeData
            // via moonrise$initCache) runs here — OUTSIDE the bench window.
            s.moonrise$initCache();
            d = U.getObject(s, OFF_CSD);
            if (d == null) {
                return 0;                                  // shape has no cacheable grid
            }
            forcedCounter++;
        }
        final CachedShapeData cur = (CachedShapeData) d;
        final CachedShapeData canon = INTERN.putIfAbsent(cur, cur);
        if (canon == null || canon == cur) {
            return 0;
        }
        U.putObject(s, OFF_CSD, canon);
        return 1;
    }

    // ------------------------------------------------------- sweepNow (post-arm)

    /**
     * THE TIMING-HOLE FIX (Л198: sweepNow() post-arm OBLIGATORY). Walks the
     * whole blockstate plane and canonicalizes every live CachedShapeData the
     * boot clinit created BEFORE the lever could arm. Called by the Rust
     * activator immediately after the retransform; the returned value is the
     * number of records replaced by canonical instances (the ARM marker
     * prints it into the leg log).
     *
     * Read-only over the registry: no block state is created or modified —
     * only VoxelShape.cachedShapeData references are swapped for
     * content-equal canonical records (parity-neutral by construction).
     */
    public static int sweepNow() {
        final int forcedBefore = forcedCounter;
        int seen = 0;
        int interned = 0;
        for (final Block block : BuiltInRegistries.BLOCK) {
            for (final BlockState state : block.getStateDefinition().getPossibleStates()) {
                final BlockStateBase bsb = (BlockStateBase) state;
                // 1) the per-state Cache collision shape (same-package read:
                //    Cache.collisionShape is protected final in THIS package)
                final Object cacheObj = U.getObject(bsb, OFF_CACHE);
                if (cacheObj instanceof BlockStateBase.Cache cache) {
                    seen++;
                    interned += internOne(cache.collisionShape);
                }
                // 2) private state-level shape fields (Unsafe offsets)
                interned += internOne((VoxelShape) U.getObject(bsb, OFF_CONST));
                interned += internOne((VoxelShape) U.getObject(bsb, OFF_OCC));
                final Object arr = U.getObject(bsb, OFF_OCC_ARR);
                if (arr instanceof VoxelShape[] shapes) {
                    for (final VoxelShape s : shapes) {
                        interned += internOne(s);
                    }
                }
            }
        }
        lastShapesSeen = seen;
        lastForced = forcedCounter - forcedBefore;
        lastInterned = interned;
        return interned;
    }
}
