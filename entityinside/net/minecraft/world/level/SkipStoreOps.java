package net.minecraft.world.level;

import net.minecraft.world.entity.Entity;
import net.minecraft.world.phys.AABB;

import java.lang.reflect.Field;

/**
 * SKIP-STORE-BB (#13-SBB, S7-166, ARCH-ATTACK lever class "value-skip").
 *
 * Value-equal store-skip for Entity.setBoundingBox(AABB): the vanilla body
 * ALWAYS allocates a fresh AABB and ALWAYS putfields it into the OLD-gen
 * Entity — every call is one young allocation + one old->young remembered
 * card. On the X150K scene the boundingBox/move family is the top allocation
 * producer (36.92% of allocation samples, TASK-318 RECON-12) and the G1
 * card-set lane is the top CPU consumer (16.39%, RECON-11): when the
 * normalized argument already bit-matches the CURRENT field value, the
 * allocation and the store are observationally DEAD — the putfield is
 * skipped, no new AABB is created, no remembered card is dirtied.
 *
 * Vanilla parity BY CONSTRUCTION:
 *  - The normalization ladder below is a verbatim copy of the javap dump
 *    of the REAL kernel Entity.setBoundingBox (TASK-318 RECON-12a, 173
 *    units): 6 argument-field loads; per axis d = source.maxN - source.minN
 *    (fresh FIELD re-reads, not the stored locals); dcmpg ladder
 *    "if (d < 0.0) maxN = minN" x3; dcmpl clamp ladder
 *    "if (d > 64.0) maxN = minN + 64.0" x3 (ldc2_w 64.0); NaN passes
 *    verbatim (dcmpg(NaN,0)=1 -> no low clamp, dcmpl(NaN,64)=-1 -> no
 *    high clamp) — plain Java < and > compile to exactly dcmpg/ifge and
 *    dcmpl/ifle, so the source below is branch-identical to the dump.
 *  - The skip fires ONLY when all six doubleToLongBits of the current
 *    field value equal the normalized values. doubleToLongBits is STRICTLY
 *    stronger than the preregistered dcmp-equality: -0.0 vs +0.0 (dcmp-
 *    equal, but bit-different and observable through clip division signs)
 *    does NOT skip; NaN components canonicalize (all NaNs are value-
 *    indistinguishable downstream: every comparison false, every arithmetic
 *    result NaN). A skipped store therefore leaves a field that is
 *    bit-identical (or NaN-canonical) to what vanilla would have stored,
 *    and the AABB class is immutable — consumers are value-semantic.
 *  - Identity invariant (RECON-12a: 0 identity sites on bb across 10
 *    fixture classes): after a skip the field reference is UNCHANGED.
 *    The offline lockstep oracle (SkipStoreLockstepHarness, >= 1M
 *    scenarios) proves bit-parity AND this invariant per scenario.
 *
 * Delivered into the KERNEL loader (same as ZeroAllocOps, S7-164); the
 * Entity.setBoundingBox(AABB)V body is redirected here by entity_compose
 * stage 8 (classfile body-redirect: receiver-prepended invokestatic, same
 * descriptor shape, length-preserving).
 *
 * NO nested classes (S7-163 leg#1 lesson: kernel-loader delivery is a
 * class-graph delivery; this file compiles to exactly ONE classfile).
 */
public final class SkipStoreOps {
    private SkipStoreOps() {}

    // ------------------------------------------------------------------
    // Unsafe access to the Entity private field bb (putfield/getfield
    // semantics for an external bridge class). Offset resolved ONCE;
    // hot-path access is a plain object-field read/write.
    // ------------------------------------------------------------------
    private static final sun.misc.Unsafe UNSAFE;
    private static final long BB_OFFSET;
    static {
        try {
            Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            UNSAFE = (sun.misc.Unsafe) uf.get(null);
            Field bb = Entity.class.getDeclaredField("bb");
            bb.setAccessible(true);
            BB_OFFSET = UNSAFE.objectFieldOffset(bb);
        } catch (Throwable t) {
            throw new ExceptionInInitializerError(t);
        }
    }

    /**
     * Body of Entity.setBoundingBox(AABB) — normalization verbatim (javap
     * units 0-148) + value-equal store-skip (units 150-171 become
     * conditional). The static MUST exist with EXACTLY
     * (Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)V —
     * a missing target detonates as NoSuchMethodError on the first entity
     * tick (S7-164 leg#1 TECH-DUD lesson); the resolution-closure guard in
     * skip_store.rs fails closed before delivery.
     */
    public static void setBoundingBox(Entity e, AABB source) {
        // ---- javap 0-33: 6 argument-field loads into locals ----
        double minX = source.minX;
        double minY = source.minY;
        double minZ = source.minZ;
        double maxX = source.maxX;
        double maxY = source.maxY;
        double maxZ = source.maxZ;

        // ---- javap 35-70: X axis — d re-read from ARGUMENT fields,
        // dcmpg ladder (d < 0.0 -> max = min-LOCAL), dcmpl clamp
        // (d > 64.0 -> max = min-LOCAL + 64.0) ----
        double d = source.maxX - source.minX;
        if (d < 0.0) {
            maxX = minX;
        }
        if (d > 64.0) {
            maxX = minX + 64.0;
        }

        // ---- javap 72-109: Y axis, same ladders ----
        d = source.maxY - source.minY;
        if (d < 0.0) {
            maxY = minY;
        }
        if (d > 64.0) {
            maxY = minY + 64.0;
        }

        // ---- javap 111-148: Z axis, same ladders ----
        d = source.maxZ - source.minZ;
        if (d < 0.0) {
            maxZ = minZ;
        }
        if (d > 64.0) {
            maxZ = minZ + 64.0;
        }

        // ---- #13-SBB value-equal store-skip ----
        // Skip ONLY on bit-equality of all six components (strictly stronger
        // than dcmp-equality: -0.0/+0.0 never skips; NaN canonicalizes —
        // see class doc). Effect: no new AABB, no putfield, no old->young
        // remembered card; the field reference is preserved (identity
        // invariant, 0 identity sites on bb — RECON-12a).
        AABB cur = (AABB) UNSAFE.getObject(e, BB_OFFSET);
        if (cur != null
            && Double.doubleToLongBits(cur.minX) == Double.doubleToLongBits(minX)
            && Double.doubleToLongBits(cur.minY) == Double.doubleToLongBits(minY)
            && Double.doubleToLongBits(cur.minZ) == Double.doubleToLongBits(minZ)
            && Double.doubleToLongBits(cur.maxX) == Double.doubleToLongBits(maxX)
            && Double.doubleToLongBits(cur.maxY) == Double.doubleToLongBits(maxY)
            && Double.doubleToLongBits(cur.maxZ) == Double.doubleToLongBits(maxZ)) {
            return;
        }

        // ---- javap 150-171: new AABB + putfield (vanilla tail) ----
        UNSAFE.putObject(e, BB_OFFSET, new AABB(minX, minY, minZ, maxX, maxY, maxZ));
    }
}
