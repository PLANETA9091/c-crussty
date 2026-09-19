package net.minecraft.world.level;

import ca.spottedleaf.moonrise.common.util.WorldUtil;
import it.unimi.dsi.fastutil.objects.Object2DoubleMap;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.tags.FluidTags;
import net.minecraft.tags.TagKey;
import net.minecraft.util.Mth;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.level.chunk.ChunkSource;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.material.Fluid;
import net.minecraft.world.level.material.FluidState;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

import java.lang.reflect.Field;
import java.util.List;

/**
 * ZERO-ALLOC-INSIDE (S7-164, ARCH-ATTACK lever #10).
 *
 * Non-caching scalar (allocation-free) rewrites of the three hottest
 * Entity inside/fluid bodies plus the private clip machinery, replacing
 * temporary AABB/Vec3/List objects with primitive doubles. Every double
 * operation, its ORDER, and every branch condition is a verbatim copy of
 * the javap contract (CONTRACT_ZEROALLOC_S7164.txt + fresh dumps of
 * EntityDimensions.makeBoundingBox / AABB.deflate / AABB.getDirection /
 * AABB.clipPoint / Fluid.getAABB / Vec3.normalize|length|scale|add taken
 * from the running kernel jar). Vanilla parity holds BY CONSTRUCTION:
 * same IEEE754 bit-exact math, same control flow, same side-effect
 * sequence (chunk reads, map puts, setDeltaMovement). No state is
 * remembered between calls: the only reused object is a thread-local
 * double[1] scratch whose content is overwritten (bounds[0]=1.0) before
 * every use — a stack-slot replacement, not a cache.
 *
 * Delivered into the KERNEL loader (same as TraverseOps, S7-163); the
 * Entity bodies are redirected here by entity_compose stage 7
 * (classfile body-redirect: receiver-prepended invokestatic, same
 * descriptors, length-preserving).
 *
 * NO nested classes (S7-163 leg#1 lesson: kernel-loader delivery is a
 * class-graph delivery; this file compiles to exactly ONE classfile).
 */
public final class ZeroAllocOps {
    private ZeroAllocOps() {}

    // ------------------------------------------------------------------
    // Unsafe accessors for Entity private fields (fluidHeight, lastLavaContact).
    // Offset resolution happens ONCE (init), hot-path access is a plain
    // object-field read/write — the only way for an external bridge class
    // to perform the vanilla putfield/getfield with identical semantics.
    // ------------------------------------------------------------------
    private static final sun.misc.Unsafe UNSAFE;
    private static final long FLUID_HEIGHT_OFFSET;
    private static final long LAST_LAVA_CONTACT_OFFSET;
    static {
        try {
            Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            UNSAFE = (sun.misc.Unsafe) uf.get(null);
            Field fh = Entity.class.getDeclaredField("fluidHeight");
            fh.setAccessible(true);
            FLUID_HEIGHT_OFFSET = UNSAFE.objectFieldOffset(fh);
            Field llc = Entity.class.getDeclaredField("lastLavaContact");
            llc.setAccessible(true);
            LAST_LAVA_CONTACT_OFFSET = UNSAFE.objectFieldOffset(llc);
        } catch (Throwable t) {
            throw new ExceptionInInitializerError(t);
        }
    }

    /** thread-local bounds scratch for the clip ladder (content, not the
     * structure, carries data; bounds[0]=1.0 before every use). */
    private static final ThreadLocal<double[]> CLIP_BOUNDS =
        ThreadLocal.withInitial(() -> new double[1]);

    // ==================================================================
    // 1. Entity.collidedWithFluid(FluidState, BlockPos, Vec3, Vec3)Z
    //    vanilla chain: fs.getAABB(level,pos) -> List.of(aabb) ->
    //    collidedWithShapeMovingFrom(from,to,boxes) ->
    //    makeBoundingBox + Vec3.subtract + AABB.collidedAlongVector.
    //    Scalars replace: Fluid.getAABB AABB, List.of, EntityDimensions
    //    AABB, subtract Vec3, and every object inside collidedAlongVector.
    // ==================================================================
    public static boolean collidedWithFluid(Entity e, FluidState fs, BlockPos pos, Vec3 from, Vec3 to) {
        // Fluid.getAABB(FluidState, BlockGetter, BlockPos) verbatim:
        if (fs.isEmpty()) {
            return false; // aabb == null -> vanilla returns false
        }
        Level level = e.level();
        float h = fs.getHeight(level, pos);
        int bx = pos.getX(), by = pos.getY(), bz = pos.getZ();
        double bminX = (double) bx, bminY = (double) by, bminZ = (double) bz;
        double bmaxX = (double) bx + 1.0;
        double bmaxY = (double) ((float) by + h); // FLOAT add, then f2d (javap 44-51)
        double bmaxZ = (double) bz + 1.0;

        // Entity.makeBoundingBox(from) -> EntityDimensions.makeBoundingBox(x,y,z) verbatim:
        float halfW = e.getBbWidth() / 2.0f;      // FLOAT division (javap 1-6)
        float bbH = e.getBbHeight();
        double xmin = from.x - (double) halfW;
        double ymin = from.y;
        double zmin = from.z - (double) halfW;
        double xmax = from.x + (double) halfW;
        double ymax = from.y + (double) bbH;
        double zmax = from.z + (double) halfW;

        // Vec3.subtract verbatim:
        double dx = to.x - from.x;
        double dy = to.y - from.y;
        double dz = to.z - from.z;

        // AABB.collidedAlongVector(delta, List.of(aabb)) verbatim (single-element list):
        return collidedAlongVectorOneBox(xmin, ymin, zmin, xmax, ymax, zmax,
            dx, dy, dz, bminX, bminY, bminZ, bmaxX, bmaxY, bmaxZ);
    }

    /** Fluid.getAABB scalar box (for the lockstep oracle): verbatim javap
     * formula — EMPTY -> null represented by Double.NaN in yMax. Returns
     * {minX, minY, minZ, maxX, maxY, maxZ}. */
    static double[] fluidBoxScalars(FluidState fs, Level level, BlockPos pos) {
        if (fs.isEmpty()) {
            return null;
        }
        float h = fs.getHeight(level, pos);
        int bx = pos.getX(), by = pos.getY(), bz = pos.getZ();
        return new double[] {
            (double) bx, (double) by, (double) bz,
            (double) bx + 1.0, (double) ((float) by + h), (double) bz + 1.0
        };
    }

    /** EntityDimensions.makeBoundingBox(x,y,z) scalar form (for the
     * lockstep oracle): FLOAT half-width division, FLOAT height. */
    static double[] entityBoxScalars(float width, float height, Vec3 from) {
        float halfW = width / 2.0f;
        double xmin = from.x - (double) halfW;
        double ymin = from.y;
        double zmin = from.z - (double) halfW;
        double xmax = from.x + (double) halfW;
        double ymax = from.y + (double) height;
        double zmax = from.z + (double) halfW;
        return new double[] {xmin, ymin, zmin, xmax, ymax, zmax};
    }

    /** Generic-list collidedAlongVector tail (for the lockstep oracle). */
    static boolean collidedAlongVectorScalars(
        double xmin, double ymin, double zmin, double xmax, double ymax, double zmax,
        double dx, double dy, double dz, List<AABB> boxes) {
        double cx = Mth.lerp(0.5, xmin, xmax);
        double cy = Mth.lerp(0.5, ymin, ymax);
        double cz = Mth.lerp(0.5, zmin, zmax);
        double fx = cx + dx;
        double fy = cy + dy;
        double fz = cz + dz;
        double sx = (xmax - xmin) * 0.5 - 1.0E-7;
        double sy = (ymax - ymin) * 0.5 - 1.0E-7;
        double sz = (zmax - zmin) * 0.5 - 1.0E-7;
        double[] bounds = CLIP_BOUNDS.get();
        for (int i = 0; i < boxes.size(); i++) {
            AABB box = boxes.get(i);
            double ixmin = box.minX - sx;
            double iymin = box.minY - sy;
            double izmin = box.minZ - sz;
            double ixmax = box.maxX + sx;
            double iymax = box.maxY + sy;
            double izmax = box.maxZ + sz;
            if (containsScalars(ixmin, iymin, izmin, ixmax, iymax, izmax, fx, fy, fz)
                || containsScalars(ixmin, iymin, izmin, ixmax, iymax, izmax, cx, cy, cz)) {
                return true;
            }
            if (clipPresent(ixmin, iymin, izmin, ixmax, iymax, izmax, cx, cy, cz, fx, fy, fz, bounds)) {
                return true;
            }
        }
        return false;
    }

    /** single-box fast path for collidedWithFluid (List.of(aabb) semantics). */
    private static boolean collidedAlongVectorOneBox(
        double xmin, double ymin, double zmin, double xmax, double ymax, double zmax,
        double dx, double dy, double dz,
        double bminX, double bminY, double bminZ, double bmaxX, double bmaxY, double bmaxZ) {
        double cx = Mth.lerp(0.5, xmin, xmax);
        double cy = Mth.lerp(0.5, ymin, ymax);
        double cz = Mth.lerp(0.5, zmin, zmax);
        double fx = cx + dx;
        double fy = cy + dy;
        double fz = cz + dz;
        double sx = (xmax - xmin) * 0.5 - 1.0E-7;
        double sy = (ymax - ymin) * 0.5 - 1.0E-7;
        double sz = (zmax - zmin) * 0.5 - 1.0E-7;
        double ixmin = bminX - sx;
        double iymin = bminY - sy;
        double izmin = bminZ - sz;
        double ixmax = bmaxX + sx;
        double iymax = bmaxY + sy;
        double izmax = bmaxZ + sz;
        if (containsScalars(ixmin, iymin, izmin, ixmax, iymax, izmax, fx, fy, fz)
            || containsScalars(ixmin, iymin, izmin, ixmax, iymax, izmax, cx, cy, cz)) {
            return true;
        }
        return clipPresent(ixmin, iymin, izmin, ixmax, iymax, izmax, cx, cy, cz, fx, fy, fz, CLIP_BOUNDS.get());
    }

    /** AABB.contains(DDD) verbatim (javap: dcmpl iflt / dcmpg ifge). */
    private static boolean containsScalars(
        double minX, double minY, double minZ, double maxX, double maxY, double maxZ,
        double px, double py, double pz) {
        return px >= minX && px < maxX && py >= minY && py < maxY && pz >= minZ && pz < maxZ;
    }

    // ==================================================================
    // 3. Entity.updateFluidHeightAndDoFluidPushing(TagKey<Fluid>, D)Z
    //    verbatim body; scalars replace the deflated AABB, the Vec3
    //    accumulate chain and their ZERO-identity compares. FlowingFluid
    //    .getFlow stays vanilla (one Vec3 per wet block — next layer).
    // ==================================================================
    public static boolean updateFluidHeightAndDoFluidPushing(Entity e, TagKey<Fluid> tag, double strength) {
        if (e.touchingUnloadedChunk()) {
            return false;
        }
        AABB full = e.getBoundingBox();
        // AABB.deflate(0.001) = AABB.inflate(-0.001): min+0.001 / max-0.001 (javap: dneg, inflate)
        double aminX = full.minX + 0.001, aminY = full.minY + 0.001, aminZ = full.minZ + 0.001;
        double amaxX = full.maxX - 0.001, amaxY = full.maxY - 0.001, amaxZ = full.maxZ - 0.001;
        Level level = e.level();
        int minSection = WorldUtil.getMinSection(level);
        int x0 = Mth.floor(aminX);
        int y0 = Math.max(minSection << 4, Mth.floor(aminY));
        int z0 = Mth.floor(aminZ);
        int x1 = Mth.ceil(amaxX) - 1;
        int y1 = Math.min((WorldUtil.getMaxSection(level) << 4) | 15, Mth.ceil(amaxY) - 1);
        int z1 = Mth.ceil(amaxZ) - 1;
        boolean push = e.isPushedByFluid();
        BlockPos.MutableBlockPos mpos = new BlockPos.MutableBlockPos();
        boolean accZero = true;   // vanilla: acc == Vec3.ZERO (reference identity)
        double ax = 0.0, ay = 0.0, az = 0.0;
        double count = 0.0;
        double maxH = 0.0;
        boolean inFluid = false;
        int cx0 = x0 >> 4, cx1 = x1 >> 4, cz0 = z0 >> 4, cz1 = z1 >> 4;
        ChunkSource src = level.getChunkSource();
        int wx = cx1 - cx0 + 1;
        int negBase = -(cx0 + wx * cz0);
        LevelChunkSection[][] sections = new LevelChunkSection[wx * (cz1 - cz0 + 1)][]; // vanilla anewarray
        for (int cz = cz0; cz <= cz1; cz++) {
            for (int cx = cx0; cx <= cx1; cx++) {
                sections[cx + wx * cz + negBase] =
                    src.getChunk(cx, cz, ChunkStatus.FULL, false).getSections();
            }
        }
        for (int x = x0; x <= x1; x++) {
            for (int y = y0; y <= y1; y++) {
                for (int z = z0; z <= z1; z++) {
                    LevelChunkSection sec =
                        sections[(x >> 4) + wx * (z >> 4) + negBase][(y >> 4) - minSection];
                    BlockState bs = sec.states.get((x & 15) | ((z & 15) << 4) | ((y & 15) << 8));
                    FluidState fs = bs.getFluidState();
                    if (fs.isEmpty() || !fs.is(tag)) {
                        continue;
                    }
                    mpos.set(x, y, z);
                    if (tag == FluidTags.LAVA) {
                        UNSAFE.putObject(e, LAST_LAVA_CONTACT_OFFSET, mpos.immutable());
                    }
                    double h = (double) ((float) y + fs.getHeight(level, mpos)); // FLOAT add (javap 422-436)
                    double dh = h - aminY;
                    if (dh < 0.0) {
                        continue;
                    }
                    inFluid = true;
                    maxH = Math.max(maxH, dh);
                    if (!push) {
                        continue;
                    }
                    count += 1.0;
                    Vec3 flow = fs.getFlow(level, mpos); // vanilla (one Vec3 per wet block)
                    if (maxH < 0.4) {
                        double sx = flow.x * maxH, sy = flow.y * maxH, sz = flow.z * maxH; // flow.scale(maxH)
                        ax = (accZero ? 0.0 : ax) + sx; // ZERO.add / acc.add verbatim
                        ay = (accZero ? 0.0 : ay) + sy;
                        az = (accZero ? 0.0 : az) + sz;
                    } else {
                        ax = (accZero ? 0.0 : ax) + flow.x;
                        ay = (accZero ? 0.0 : ay) + flow.y;
                        az = (accZero ? 0.0 : az) + flow.z;
                    }
                    accZero = false;
                }
            }
        }
        @SuppressWarnings("unchecked")
        Object2DoubleMap<TagKey<Fluid>> fhMap =
            (Object2DoubleMap<TagKey<Fluid>>) UNSAFE.getObject(e, FLUID_HEIGHT_OFFSET);
        fhMap.put(tag, maxH);
        if (accZero) {
            return inFluid;
        }
        double dInv = 1.0 / count;
        ax *= dInv; ay *= dInv; az *= dInv; // acc.scale(1.0/count)
        Vec3 dm = e.getDeltaMovement();
        if (!(e instanceof net.minecraft.world.entity.player.Player)) {
            // Vec3.normalize verbatim: len = sqrt(x^2+y^2+z^2); ZERO if len < 9.999999747378752E-6
            double len = Math.sqrt(ax * ax + ay * ay + az * az);
            if (len < 9.999999747378752E-6) {
                accZero = true; ax = 0.0; ay = 0.0; az = 0.0; // Vec3.ZERO (numerically)
            } else {
                ax /= len; ay /= len; az /= len;
            }
        }
        ax *= strength; ay *= strength; az *= strength; // acc.scale(strength)
        if (Math.abs(dm.x) < 0.003 && Math.abs(dm.z) < 0.003) {
            double accLen = Math.sqrt(ax * ax + ay * ay + az * az);
            if (accLen < 0.0045000000000000005) {
                if (accZero) {
                    ax = 0.0; ay = 0.0; az = 0.0; // normalize() -> ZERO
                } else {
                    double len2 = Math.sqrt(ax * ax + ay * ay + az * az);
                    if (len2 < 9.999999747378752E-6) {
                        ax = 0.0; ay = 0.0; az = 0.0;
                    } else {
                        ax /= len2; ay /= len2; az /= len2;
                    }
                }
                ax *= 0.0045000000000000005;
                ay *= 0.0045000000000000005;
                az *= 0.0045000000000000005;
            }
        }
        e.setDeltaMovement(new Vec3(dm.x + ax, dm.y + ay, dm.z + az)); // dm.add(acc): 1 vanilla-shape Vec3
        return true;
    }

    // ==================================================================
    // Clip ladder — verbatim scalar copy of AABB.clip(6double, Vec3, Vec3)
    // (getDirection + clipPoint). Returns whether the ray hits the box
    // (vanilla: clip(...).isPresent()). No Optional/Vec3/double[] per call;
    // the bounds scratch is overwritten before every use.
    // ==================================================================
    private static boolean clipPresent(
        double minX, double minY, double minZ, double maxX, double maxY, double maxZ,
        double fromX, double fromY, double fromZ,
        double toX, double toY, double toZ,
        double[] bounds) {
        double dx = toX - fromX;
        double dy = toY - fromY;
        double dz = toZ - fromZ;
        bounds[0] = 1.0; // vanilla: new double[]{1.0}
        Direction dir = getDirectionScalars(minX, minY, minZ, maxX, maxY, maxZ,
            fromX, fromY, fromZ, bounds, dx, dy, dz);
        return dir != null; // vanilla: Optional.of(hit) iff dir != null
    }

    /** verbatim AABB.getDirection (fresh javap dump; axes: d=axis delta,
     * e/f = the two transverse deltas in axis order). */
    private static Direction getDirectionScalars(
        double minX, double minY, double minZ, double maxX, double maxY, double maxZ,
        double fx, double fy, double fz,
        double[] bounds, double dx, double dy, double dz) {
        Direction dir = null;
        if (dx > 1.0E-7) {
            dir = clipPointScalars(bounds, dir, dx, dy, dz,
                minX, minY, maxY, minZ, maxZ, Direction.WEST, fx, fy, fz);
        } else if (dx < -1.0E-7) {
            dir = clipPointScalars(bounds, dir, dx, dy, dz,
                maxX, minY, maxY, minZ, maxZ, Direction.EAST, fx, fy, fz);
        }
        if (dy > 1.0E-7) {
            dir = clipPointScalars(bounds, dir, dy, dz, dx,
                minY, minZ, maxZ, minX, maxX, Direction.DOWN, fy, fz, fx);
        } else if (dy < -1.0E-7) {
            dir = clipPointScalars(bounds, dir, dy, dz, dx,
                maxY, minZ, maxZ, minX, maxX, Direction.UP, fy, fz, fx);
        }
        if (dz > 1.0E-7) {
            dir = clipPointScalars(bounds, dir, dz, dx, dy,
                minZ, minX, maxX, minY, maxY, Direction.NORTH, fz, fx, fy);
        } else if (dz < -1.0E-7) {
            dir = clipPointScalars(bounds, dir, dz, dx, dy,
                maxZ, minX, maxX, minY, maxY, Direction.SOUTH, fz, fx, fy);
        }
        return dir;
    }

    /** verbatim AABB.clipPoint (javap: t=(g-m)/d; u=n+t*e; v=o+t*f; the
     * dcmpg/ifge ladder: return dir iff 0.0>=t || t>=bounds[0] ||
     * (h-eps)>=u || u>=(i+eps) || (j-eps)>=v || v>=(k+eps) — i.e. hit
     * requires t>0 (intersection AHEAD of the point), t<bounds[0], and the
     * strict-open transverse window). */
    private static Direction clipPointScalars(
        double[] bounds, Direction dir,
        double d, double e, double f,
        double g, double h, double i, double j, double k,
        Direction face,
        double m, double n, double o) {
        double t = (g - m) / d;
        double u = n + t * e;
        double v = o + t * f;
        if (0.0 >= t) return dir;
        if (t >= bounds[0]) return dir;
        if (h - 1.0E-7 >= u) return dir;
        if (u >= i + 1.0E-7) return dir;
        if (j - 1.0E-7 >= v) return dir;
        if (v >= k + 1.0E-7) return dir;
        bounds[0] = t;
        return face;
    }
}
