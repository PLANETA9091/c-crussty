package net.minecraft.world.level;

import java.util.Optional;
import net.minecraft.core.BlockPos;
import net.minecraft.util.Mth;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * TraverseOps (S7-163, ARCH-ATTACK lever #9 FLAT-TRAVERSAL) — flat
 * re-implementation of {@link BlockGetter#forEachBlockIntersectedBetween}
 * with BIT-EXACT visit order/steps and no iterator orchestration.
 *
 * Contract source: research/inside-pipeline-2026-09-19/
 * CONTRACT_traversal_javap.txt + CONTRACT_EXTRA/CLIP/MTH_S7163.txt
 * (javap of kernel 1.21.10, bit-in-bit). The vanilla implementation
 * allocates per call: 3+ guava AbstractIterators (BlockPos$6 /
 * BlockPos$4), Vec3/AABB temporaries (move/scale/center) and a
 * LongOpenHashSet — RECON-4 measured the orchestration family at
 * ~3050 of ~3400 tail samples (guava 1443, betweenCornersInDirection
 * bodies 765, betweenClosed lambdas 473, LongOpenHashSet 369) on top of
 * the 7.27% CPU traversal lane. This port removes exactly that layer:
 *
 *   - flat (f,s,t) loops replace BlockPos$6 (corner iterator): the
 *     vanilla order is outer=axis1, middle=axis2, inner=axis3 with
 *     axisStepOrder = (|dx|<|dz|) ? YZX : YXZ (Y ALWAYS outer), start
 *     corner per component sign, unit ±1 steps, extents = max-min
 *     (each dimension iterated extents+1 times inclusive);
 *   - the stationary path replicates BlockPos$4 (betweenClosed):
 *     index/%-iteration with X innermost, Y middle, Z outermost,
 *     count = extX*extY*extZ (int math, overflow parity preserved);
 *   - dedupe uses an open-addressing long table (linear probing,
 *     exact membership — hash choice cannot change behavior) with
 *     LongSet.add was-new semantics;
 *   - AABB.clip static is CALLED VERBATIM (public API) — the ray-box
 *     hit point stays bit-identical by construction; only the
 *     iteration layer is replaced. Inline-clip is a later squeeze
 *     within this lever if the absorb tail shows it is worth the
 *     parity risk.
 *
 * Verbatim-formula traps honored (javap census):
 *   - AABB.getCenter = Mth.lerp(0.5, min, max) = max + 0.5*(min-max)
 *     (REVERSED lerp — not (min+max)/2);
 *   - DDA loop continues while ANY tMax <= 1.0 (exit when ALL > 1);
 *   - axis choice: (tMaxX < tMaxY) ? (tMaxX < tMaxZ ? X : Z)
 *                                   : (tMaxY < tMaxZ ? Y : Z);
 *   - clamp low = (double)((float)cell + 1.0E-5f) (FLOAT add then
 *     widen), high = (cell+1.0) - 9.999999747378752E-6 (doubles);
 *   - off = floor(clamp - size * furthest), furthest from
 *     getFurthestCorner (-sx,-sz,-sy)/(sz,-sy,-sx)/(-sy,sx,-sz);
 *   - stepX = sign==0 ? Double.MAX_VALUE : sign/delta.x;
 *   - tMax = step * (sign>0 ? 1.0 - frac(travel) : frac(travel));
 *   - outer.x = center.x + size.x*0.5*furthest.x, evaluated as
 *     (size*0.5)*furthest (Java left-to-right, matches bytecode).
 *
 * Vanilla-parity surface: the visitor object is the SAME vanilla
 * lambda (allocated at the call site, untouched); positions are
 * delivered through ONE BlockPos.MutableBlockPos cursor per call
 * (vanilla also reuses a cursor per iterator — the visitor never
 * retains it). visit(pos, step) sequence, short-circuit points and
 * the boolean return are bit-in-bit identical; verified by
 * TraverseLockstepHarness (random + degenerate scenarios).
 */
public final class TraverseOps {

    private TraverseOps() {}

    /**
     * Exact replacement for
     * BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB,
     * BlockStepVisitor) — the single call site retargeted by the
     * traversal stage of the entity_compose chain
     * (Entity.checkInsideBlocks). Signature MUST stay identical to
     * the vanilla declaration (descriptor parity for the retarget).
     */
    public static boolean forEachFlat(Vec3 from, Vec3 to, AABB box,
                                      BlockGetter.BlockStepVisitor visitor) {
        Vec3 delta = to.subtract(from);
        if (delta.lengthSqr() < Mth.square(1.0E-5f)) {
            return forEachStationary(box, visitor);
        }
        BlockPos.MutableBlockPos cursor = new BlockPos.MutableBlockPos();
        LongTable visited = new LongTable();

        // PHASE A: start box — betweenCornersInDirection(
        // box.move(delta.scale(-1.0)), delta): visit(pos, 0) for EVERY
        // corner (false -> false), then visited.add(pos).
        // move adds the negated delta; a + (-b) == a - b bit-exact.
        int aMinX = Mth.floor(box.minX - delta.x);
        int aMinY = Mth.floor(box.minY - delta.y);
        int aMinZ = Mth.floor(box.minZ - delta.z);
        int aMaxX = Mth.floor(box.maxX - delta.x);
        int aMaxY = Mth.floor(box.maxY - delta.y);
        int aMaxZ = Mth.floor(box.maxZ - delta.z);
        if (!forEachCorners(aMinX, aMinY, aMinZ, aMaxX, aMaxY, aMaxZ,
                delta, cursor, visited, visitor, 0, MODE_PHASE_A)) {
            return false;
        }

        // PHASE B: DDA march.
        int steps = addCollisionsAlongTravel(visited, delta, box, visitor, cursor);
        if (steps < 0) {
            return false;
        }

        // PHASE C: end box — betweenCornersInDirection(box, delta):
        // visited.add -> visit(pos, steps+1) (false -> false).
        int cMinX = Mth.floor(box.minX);
        int cMinY = Mth.floor(box.minY);
        int cMinZ = Mth.floor(box.minZ);
        int cMaxX = Mth.floor(box.maxX);
        int cMaxY = Mth.floor(box.maxY);
        int cMaxZ = Mth.floor(box.maxZ);
        return forEachCorners(cMinX, cMinY, cMinZ, cMaxX, cMaxY, cMaxZ,
                delta, cursor, visited, visitor, steps + 1, MODE_PHASE_C);
    }

    private static final int MODE_PHASE_A = 0;
    private static final int MODE_PHASE_B = 1;
    private static final int MODE_PHASE_C = 2;

    /**
     * Stationary path: BlockPos.betweenClosed(box) — containing(floor)
     * min/max, then BlockPos$4 index iteration (X innermost, Y middle,
     * Z outermost), visit(pos, 0), false -> false.
     */
    private static boolean forEachStationary(AABB box,
                                             BlockGetter.BlockStepVisitor visitor) {
        BlockPos.MutableBlockPos cursor = new BlockPos.MutableBlockPos();
        int ax = Mth.floor(box.minX);
        int ay = Mth.floor(box.minY);
        int az = Mth.floor(box.minZ);
        int bx = Mth.floor(box.maxX);
        int by = Mth.floor(box.maxY);
        int bz = Mth.floor(box.maxZ);
        int minX = Math.min(ax, bx);
        int minY = Math.min(ay, by);
        int minZ = Math.min(az, bz);
        int maxX = Math.max(ax, bx);
        int maxY = Math.max(ay, by);
        int maxZ = Math.max(az, bz);
        int extX = maxX - minX + 1;
        int extY = maxY - minY + 1;
        int extZ = maxZ - minZ + 1;
        int count = extX * extY * extZ;
        for (int index = 0; index < count; index++) {
            int offX = index % extX;
            int offY = (index / extX) % extY;
            int offZ = (index / extX) / extY;
            cursor.set(minX + offX, minY + offY, minZ + offZ);
            if (!visitor.visit(cursor, 0)) {
                return false;
            }
        }
        return true;
    }

    /**
     * Corner iteration (BlockPos$6 order, bit-exact): outer axis = Y,
     * middle/inner per axisStepOrder (|dx|<|dz| ? Z,X : X,Z), start
     * corner per component sign, inclusive extents+1 counts, first
     * element at all-zero offsets.
     *
     * mode PHASE_A: visit(pos, step) first (false -> false), then
     *              visited.add unconditionally.
     * mode PHASE_B/C: visited.add -> visit(pos, step) (false -> false,
     *              reported by the caller as -1 via the return value).
     */
    private static boolean forEachCorners(int x0, int y0, int z0,
                                          int x1, int y1, int z1,
                                          Vec3 delta,
                                          BlockPos.MutableBlockPos cursor,
                                          LongTable visited,
                                          BlockGetter.BlockStepVisitor visitor,
                                          int step, int mode) {
        int minX = Math.min(x0, x1);
        int minY = Math.min(y0, y1);
        int minZ = Math.min(z0, z1);
        int maxX = Math.max(x0, x1);
        int maxY = Math.max(y0, y1);
        int maxZ = Math.max(z0, z1);
        int extX = maxX - minX;
        int extY = maxY - minY;
        int extZ = maxZ - minZ;
        int startX = delta.x >= 0 ? minX : maxX;
        int startY = delta.y >= 0 ? minY : maxY;
        int startZ = delta.z >= 0 ? minZ : maxZ;
        boolean zx = Math.abs(delta.x) < Math.abs(delta.z);
        int stepY = delta.y >= 0 ? 1 : -1;
        int stepMid;
        int stepInner;
        int startMid;
        int startInner;
        int extMid;
        int extInner;
        if (zx) { // YZX: middle = Z, inner = X
            stepMid = delta.z >= 0 ? 1 : -1;
            startMid = delta.z >= 0 ? minZ : maxZ;
            extMid = extZ;
            stepInner = delta.x >= 0 ? 1 : -1;
            startInner = delta.x >= 0 ? minX : maxX;
            extInner = extX;
        } else { // YXZ: middle = X, inner = Z
            stepMid = delta.x >= 0 ? 1 : -1;
            startMid = delta.x >= 0 ? minX : maxX;
            extMid = extX;
            stepInner = delta.z >= 0 ? 1 : -1;
            startInner = delta.z >= 0 ? minZ : maxZ;
            extInner = extZ;
        }
        for (int f = 0; f <= extY; f++) {
            int cy = startY + stepY * f;
            for (int s = 0; s <= extMid; s++) {
                int cMid = startMid + stepMid * s;
                for (int t = 0; t <= extInner; t++) {
                    int cInner = startInner + stepInner * t;
                    int px;
                    int pz;
                    if (zx) {
                        px = cInner;
                        pz = cMid;
                    } else {
                        px = cMid;
                        pz = cInner;
                    }
                    cursor.set(px, cy, pz);
                    long key = BlockPos.asLong(px, cy, pz);
                    if (mode == MODE_PHASE_A) {
                        if (!visitor.visit(cursor, step)) {
                            return false;
                        }
                        visited.add(key);
                    } else {
                        if (visited.add(key) && !visitor.visit(cursor, step)) {
                            return false;
                        }
                    }
                }
            }
        }
        return true;
    }

    /**
     * DDA march (addCollisionsAlongTravel, bit-exact): cell walk from
     * floor(travel) with min-tMax axis stepping, AABB.clip per cell
     * (vanilla static, called verbatim), clamped hit -> corner box
     * visit with dedupe. Returns the step count, or -1 when the
     * visitor short-circuits inside phase B.
     */
    private static int addCollisionsAlongTravel(LongTable visited, Vec3 delta,
                                                AABB box,
                                                BlockGetter.BlockStepVisitor visitor,
                                                BlockPos.MutableBlockPos cursor) {
        double sizeX = box.getXsize();
        double sizeY = box.getYsize();
        double sizeZ = box.getZsize();
        // getFurthestCorner(delta) — permutation, LAST component NOT inverted
        // (javap verbatim: (−sx,−sz,sy)/(sz,−sy,−sx)/(−sy,sx,−sz); verified by
        // reflection: (0,0,+7.75) -> (−1,−1,+1)).
        double absX = Math.abs(delta.x);
        double absY = Math.abs(delta.y);
        double absZ = Math.abs(delta.z);
        int signX = delta.x >= 0 ? 1 : -1;
        int signY = delta.y >= 0 ? 1 : -1;
        int signZ = delta.z >= 0 ? 1 : -1;
        int furthX;
        int furthY;
        int furthZ;
        if (absX <= absY && absX <= absZ) {
            furthX = -signX;
            furthY = -signZ;
            furthZ = signY;
        } else if (absY <= absZ) {
            furthX = signZ;
            furthY = -signY;
            furthZ = -signX;
        } else {
            furthX = -signY;
            furthY = signX;
            furthZ = -signZ;
        }
        // AABB.getCenter = Mth.lerp(0.5, min, max) = max + 0.5*(min - max).
        double centerX = Mth.lerp(0.5, box.minX, box.maxX);
        double centerY = Mth.lerp(0.5, box.minY, box.maxY);
        double centerZ = Mth.lerp(0.5, box.minZ, box.maxZ);
        // outer = center + (size*0.5)*furthest — Java left-to-right.
        Vec3 outer = new Vec3(centerX + sizeX * 0.5 * (double) furthX,
                centerY + sizeY * 0.5 * (double) furthY,
                centerZ + sizeZ * 0.5 * (double) furthZ);
        Vec3 travel = outer.subtract(delta);
        int bx = Mth.floor(travel.x);
        int by = Mth.floor(travel.y);
        int bz = Mth.floor(travel.z);
        int signStepX = Mth.sign(delta.x);
        int signStepY = Mth.sign(delta.y);
        int signStepZ = Mth.sign(delta.z);
        double stepX = signStepX == 0 ? Double.MAX_VALUE : (double) signStepX / delta.x;
        double stepY = signStepY == 0 ? Double.MAX_VALUE : (double) signStepY / delta.y;
        double stepZ = signStepZ == 0 ? Double.MAX_VALUE : (double) signStepZ / delta.z;
        double tMaxX = stepX * (signStepX > 0 ? 1.0 - Mth.frac(travel.x) : Mth.frac(travel.x));
        double tMaxY = stepY * (signStepY > 0 ? 1.0 - Mth.frac(travel.y) : Mth.frac(travel.y));
        double tMaxZ = stepZ * (signStepZ > 0 ? 1.0 - Mth.frac(travel.z) : Mth.frac(travel.z));
        int steps = 0;
        // Continue while ANY tMax <= 1.0 (exit when ALL exceed 1).
        while (tMaxX <= 1.0 || tMaxY <= 1.0 || tMaxZ <= 1.0) {
            if (tMaxX < tMaxY) {
                if (tMaxX < tMaxZ) {
                    bx += signStepX;
                    tMaxX += stepX;
                } else {
                    bz += signStepZ;
                    tMaxZ += stepZ;
                }
            } else {
                if (tMaxY < tMaxZ) {
                    by += signStepY;
                    tMaxY += stepY;
                } else {
                    bz += signStepZ;
                    tMaxZ += stepZ;
                }
            }
            Optional<Vec3> hit = AABB.clip((double) bx, (double) by, (double) bz,
                    (double) (bx + 1), (double) (by + 1), (double) (bz + 1),
                    travel, outer);
            if (hit.isEmpty()) {
                continue;
            }
            steps++;
            Vec3 hitV = hit.get();
            double clampedX = Mth.clamp(hitV.x, (double) ((float) bx + 1.0E-5f),
                    (double) bx + 1.0 - 9.999999747378752E-6);
            double clampedY = Mth.clamp(hitV.y, (double) ((float) by + 1.0E-5f),
                    (double) by + 1.0 - 9.999999747378752E-6);
            double clampedZ = Mth.clamp(hitV.z, (double) ((float) bz + 1.0E-5f),
                    (double) bz + 1.0 - 9.999999747378752E-6);
            int offX = Mth.floor(clampedX - sizeX * (double) furthX);
            int offY = Mth.floor(clampedY - sizeY * (double) furthY);
            int offZ = Mth.floor(clampedZ - sizeZ * (double) furthZ);
            int visitStep = steps;
            if (!forEachCorners(bx, by, bz, offX, offY, offZ, delta, cursor,
                    visited, visitor, visitStep, MODE_PHASE_B)) {
                return -1;
            }
        }
        return steps;
    }

    /**
     * Open-addressing long set (linear probing) with LongSet.add
     * was-new semantics. Membership is EXACT for any hash — the hash
     * choice cannot change behavior, only probe order. Key 0 is
     * carried by a dedicated flag (asLong(0,0,0) = 0 is reachable).
     */
    private static final class LongTable {
        private static final long MIX = 0x9E3779B97F4A7C15L;
        private long[] keys;
        private boolean hasZero;
        private int size;
        private int limit;

        LongTable() {
            this.keys = new long[64];
            this.limit = 38; // 64 * 0.6
        }

        boolean add(long key) {
            if (key == 0L) {
                boolean wasNew = !this.hasZero;
                this.hasZero = true;
                return wasNew;
            }
            long[] tab = this.keys;
            int mask = tab.length - 1;
            int idx = mix(key) & mask;
            while (true) {
                long cur = tab[idx];
                if (cur == 0L) {
                    tab[idx] = key;
                    this.size++;
                    if (this.size > this.limit) {
                        this.grow();
                    }
                    return true;
                }
                if (cur == key) {
                    return false;
                }
                idx = (idx + 1) & mask;
            }
        }

        private void grow() {
            long[] old = this.keys;
            long[] tab = new long[old.length << 1];
            this.limit = (int) (tab.length * 0.6);
            for (long k : old) {
                if (k != 0L) {
                    int idx = mix(k) & (tab.length - 1);
                    while (tab[idx] != 0L) {
                        idx = (idx + 1) & (tab.length - 1);
                    }
                    tab[idx] = k;
                }
            }
            this.keys = tab;
        }

        private static int mix(long key) {
            long h = key * MIX;
            h ^= h >>> 32;
            return (int) h;
        }
    }
}
