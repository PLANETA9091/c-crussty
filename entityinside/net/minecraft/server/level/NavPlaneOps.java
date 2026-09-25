package net.minecraft.server.level;

import it.unimi.dsi.fastutil.objects.ObjectArrayList;
import net.minecraft.Util;
import net.minecraft.core.BlockPos;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.ai.navigation.PathNavigation;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.pathfinder.Node;
import net.minecraft.world.level.pathfinder.Path;
import net.minecraft.world.phys.shapes.BooleanOp;
import net.minecraft.world.phys.shapes.Shapes;

/**
 * NAV-PLANE (TASK-405-A restart, lever cmp405_navplane — STRICT eq).
 *
 * The nav_ai lane's single hottest cross-mob site is
 * ServerLevel.sendBlockUpdated: per collision-shape-delta block update it
 * iterates the whole navigatingMobs set and calls
 * PathNavigation.shouldRecomputePath(BlockPos) per mob. Per RECON-8 the
 * shouldRecomputePath body (Paper) allocates one Vec3 per mob per
 * block-update and re-derives (node+mob)/2 midpoint math; with region
 * workers emitting block updates continuously this is the nav-lane top
 * sample. The lever collapses the per-mob decision into ONE bulk JNI call
 * per block-update batch (law 6: buffer inputs -> one native call -> ready
 * outputs; per-entity JNI is a design error).
 *
 * Contract (javap-verbatim, ServerLevel.sendBlockUpdated codelen=235):
 *  - prefix: isUpdatingNavigations guard log, chunkSource.blockChanged,
 *    pathTypesByPosCache.invalidate (private -> Unsafe, BlockUpdateOps
 *    pattern), paperConfig misc.updatePathfindingOnBlockUpdate gate,
 *    Shapes.joinIsNotEmpty(old, new, NOT_SAME) gate;
 *  - collect pass: iterate navigatingMobs in SET ORDER, collect nav + the
 *    exact shouldRecomputePath inputs (delayed flag, path null/isDone/
 *    nodeCount==0 tri-state, end node x/y/z, nodeCount-nextNodeIndex,
 *    mob x/y/z) into flat primitive arrays;
 *  - ONE native navDecide(n, bx, by, bz, meta, nodes, mobxyz, out) computes
 *    every decision bit-in-bit:
 *      vecX = ((double)node.x + mobX) / 2.0        (i2d, dadd, ddiv 2.0)
 *      g    = (double)bx + 0.5 - vecX              (Vec3i.distToCenterSqr)
 *      res  = (g*g + h*h) + i2*i2                  (left-to-right dadd)
 *      out  = res < (double)remaining * remaining  (closerToCenterThan,
 *                                                   Mth.square = d*d)
 *    Errors -> negative rc -> batchOk latch disarms -> java-side identical
 *    math (decideJava) applies for the call (fail-closed, mobs_soa ERR
 *    ladder discipline; behavior identical, only the executor differs);
 *  - apply pass: recomputePath() for decided mobs IN COLLECT ORDER (same
 *    order as the vanilla ObjectArrayList pass);
 *  - CME retry: the original exception table covers 95..157 -> target 160
 *    (catch ConcurrentModificationException -> self re-dispatch + return);
 *    replicated exactly; the finally(latch) covers only the apply pass.
 *
 * Empty lever flag: this class is never defined and the sendBlockUpdated
 * body is never retargeted (gate in src/region_threads.rs composes the
 * redirect only when CRUSSTY_LEVER_FLAG eq cmp405_navplane) -> vanilla
 * bit-in-byte by construction. Different flag values are rejected too.
 *
 * P45 PRE-GATE (TASK-459-70, ID-P45 scaffold): O(1) chunk-keyed
 * navigatingMobs occupancy gate BEFORE the collect pass —
 * src/nav_chunk_pregate.rs (lever cmp459_p45 STRICT eq). The pregate native
 * is reachable ONLY after rust RegisterNatives + pregateArmed() (one-shot
 * guard, define-before-arm NCDFE canon); empty/other lever => pregateOk
 * stays false => bit-in-byte vanilla by construction.
 * SCAFFOLD NOTE: this source is AHEAD of the committed
 * entityinside/build/.../NavPlaneOps.class — javac rebuild + javap verify
 * (major 65, zero nested) is the FIRST step of the next leg before any arm.
 */
public final class NavPlaneOps {
    private NavPlaneOps() {}

    private static final sun.misc.Unsafe UNSAFE;
    private static final long PATH_TYPES_OFFSET;
    private static final long NAV_PATH_OFFSET;
    private static final long NAV_DELAYED_OFFSET;
    private static final long NAV_MOB_OFFSET;

    static {
        try {
            java.lang.reflect.Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            UNSAFE = (sun.misc.Unsafe) uf.get(null);
            java.lang.reflect.Field f =
                    ServerLevel.class.getDeclaredField("pathTypesByPosCache");
            PATH_TYPES_OFFSET = UNSAFE.objectFieldOffset(f);
            java.lang.reflect.Field pf =
                    PathNavigation.class.getDeclaredField("path");
            NAV_PATH_OFFSET = UNSAFE.objectFieldOffset(pf);
            java.lang.reflect.Field df =
                    PathNavigation.class.getDeclaredField("hasDelayedRecomputation");
            NAV_DELAYED_OFFSET = UNSAFE.objectFieldOffset(df);
            java.lang.reflect.Field mf =
                    PathNavigation.class.getDeclaredField("mob");
            NAV_MOB_OFFSET = UNSAFE.objectFieldOffset(mf);
        } catch (Throwable t) {
            throw new ExceptionInInitializerError(t);
        }
    }

    /** Bulk decision kernel (Rust). Returns 0 on success, negative on ERR. */
    public static native int navDecide(int n, int bx, int by, int bz,
                                       int[] meta, int[] nodes, double[] mobxyz,
                                       byte[] out);

    /** One-shot disarm latch: any ERR/throwable falls back to java math. */
    private static volatile boolean batchOk = true;

    // ------------------------------------------------------------------
    // P45 pre-gate (TASK-459-70, ID-P45): O(1) chunk-keyed navigatingMobs
    // occupancy gate. Contract: 0 = EMPTY (no live navigating mob decision
    // sphere covers this chunk -> the vanilla full pass is provably a no-op
    // -> skip), 1 = MAYBE (run the vanilla collect + navDecide as today),
    // negative = ERR (one-shot disarm -> full vanilla pass forever).
    // Superset gate: EMPTY => shouldRecomputePath(pos) == false for EVERY
    // mob, so skipping the collect pass is observationally identical to
    // vanilla (parity contract of src/entity_index.rs count==0).
    // ------------------------------------------------------------------
    /** O(1) pregate kernel (Rust). 0 EMPTY / 1 MAYBE / negative ERR. */
    public static native int navPregate(int cx, int cz);

    /** One-shot arm latch: set ONLY by rust registration (pregateArmed). */
    private static volatile boolean pregateOk = false;

    /** Called by src/nav_chunk_pregate.rs AFTER RegisterNatives succeeds. */
    public static void pregateArmed() {
        pregateOk = true;
    }

    /** Receiver-prepended entry the classfile.rs retarget emits. */
    public static void handle(ServerLevel level, BlockPos pos, BlockState oldS,
                              BlockState newS, int flags) {
        if (level.isUpdatingNavigations) {
            Util.logAndPauseIfInIde(
                    "Detected use of sendBlockUpdated while updating navigations",
                    new IllegalStateException(
                            "Thread is already updating navigations"));
        }
        level.getChunkSource().blockChanged(pos);
        pathTypes(level).invalidate(pos);
        io.papermc.paper.configuration.WorldConfiguration.Misc misc =
                level.paperConfig().misc;
        if (!misc.updatePathfindingOnBlockUpdate) {
            return;
        }
        if (!Shapes.joinIsNotEmpty(
                oldS.getCollisionShape(level, pos),
                newS.getCollisionShape(level, pos),
                BooleanOp.NOT_SAME)) {
            return;
        }

        // P45 pre-gate: zero-work when no navigating-mob decision sphere
        // covers this chunk (superset gate, see class doc). Any ERR or
        // throwable disarms the gate permanently (fail-closed -> the full
        // vanilla pass below, never a behavior change).
        if (pregateOk) {
            int rc;
            try {
                rc = navPregate(pos.getX() >> 4, pos.getZ() >> 4);
            } catch (Throwable t) {
                pregateOk = false;
                rc = 1;
            }
            if (rc == 0) {
                return;
            }
            if (rc < 0) {
                pregateOk = false;
            }
        }

        // Collect pass: SET ORDER, exact shouldRecomputePath inputs.
        int cap = level.navigatingMobs.size();
        int[] meta = new int[Math.max(2, cap * 2)];
        int[] nodes = new int[Math.max(3, cap * 3)];
        double[] mobxyz = new double[Math.max(3, cap * 3)];
        PathNavigation[] navs = new PathNavigation[Math.max(1, cap)];
        int n = 0;
        try {
            for (Object o : level.navigatingMobs) {
                Mob mob = (Mob) o;
                PathNavigation nav = mob.getNavigation();
                Path p = (Path) UNSAFE.getObject(nav, NAV_PATH_OFFSET);
                boolean delayed = UNSAFE.getBoolean(nav, NAV_DELAYED_OFFSET);
                int fl;
                int remaining = 0;
                int nx = 0;
                int ny = 0;
                int nz = 0;
                if (delayed) {
                    fl = 1;
                } else if (p == null || p.isDone() || p.getNodeCount() == 0) {
                    fl = 2;
                } else {
                    fl = 0;
                    Node nd = p.getEndNode();
                    nx = nd.x;
                    ny = nd.y;
                    nz = nd.z;
                    remaining = p.getNodeCount() - p.getNextNodeIndex();
                }
                if ((n + 1) * 2 > meta.length || navs.length == n) {
                    meta = growInt(meta, n * 4);
                    nodes = growInt(nodes, n * 6);
                    mobxyz = growDbl(mobxyz, n * 6);
                    navs = growNav(navs, n * 2);
                }
                meta[n * 2] = fl;
                meta[n * 2 + 1] = remaining;
                nodes[n * 3] = nx;
                nodes[n * 3 + 1] = ny;
                nodes[n * 3 + 2] = nz;
                mobxyz[n * 3] = mob.getX();
                mobxyz[n * 3 + 1] = mob.getY();
                mobxyz[n * 3 + 2] = mob.getZ();
                navs[n] = nav;
                n++;
            }
        } catch (java.util.ConcurrentModificationException cme) {
            // Vanilla exception table 95..157 -> 160: re-dispatch self + return.
            handle(level, pos, oldS, newS, flags);
            return;
        }

        if (n == 0) {
            return;
        }

        level.isUpdatingNavigations = true;
        try {
            byte[] out = null;
            if (batchOk) {
                try {
                    byte[] buf = new byte[n];
                    int rc = navDecide(n, pos.getX(), pos.getY(), pos.getZ(),
                            meta, nodes, mobxyz, buf);
                    if (rc == 0) {
                        out = buf;
                    } else {
                        batchOk = false;
                    }
                } catch (Throwable t) {
                    batchOk = false;
                }
            }
            if (out != null) {
                for (int i = 0; i < n; i++) {
                    if (out[i] != 0) {
                        navs[i].recomputePath();
                    }
                }
            } else {
                for (int i = 0; i < n; i++) {
                    if (decideJava(meta[i * 2], meta[i * 2 + 1],
                            nodes[i * 3], nodes[i * 3 + 1], nodes[i * 3 + 2],
                            mobxyz[i * 3], mobxyz[i * 3 + 1], mobxyz[i * 3 + 2],
                            pos.getX(), pos.getY(), pos.getZ())) {
                        navs[i].recomputePath();
                    }
                }
            }
        } finally {
            level.isUpdatingNavigations = false;
        }
    }

    /** Bit-exact java replica of the native decision (fail-closed path). */
    private static boolean decideJava(int fl, int remaining, int nx, int ny,
                                      int nz, double mx, double my, double mz,
                                      int bx, int by, int bz) {
        if (fl == 1) {
            return false;
        }
        if (fl == 2) {
            return false;
        }
        double vx = ((double) nx + mx) / 2.0;
        double vy = ((double) ny + my) / 2.0;
        double vz = ((double) nz + mz) / 2.0;
        double g = (double) bx + 0.5 - vx;
        double h = (double) by + 0.5 - vy;
        double i2 = (double) bz + 0.5 - vz;
        double sq = (double) remaining * (double) remaining;
        return g * g + h * h + i2 * i2 < sq;
    }

    private static int[] growInt(int[] src, int need) {
        int[] dst = new int[Math.max(need, src.length * 2)];
        System.arraycopy(src, 0, dst, 0, src.length);
        return dst;
    }

    private static double[] growDbl(double[] src, int need) {
        double[] dst = new double[Math.max(need, src.length * 2)];
        System.arraycopy(src, 0, dst, 0, src.length);
        return dst;
    }

    private static PathNavigation[] growNav(PathNavigation[] src, int need) {
        PathNavigation[] dst = new PathNavigation[Math.max(need, src.length * 2)];
        System.arraycopy(src, 0, dst, 0, src.length);
        return dst;
    }

    private static net.minecraft.world.level.pathfinder.PathTypeCache pathTypes(
            ServerLevel level) {
        return (net.minecraft.world.level.pathfinder.PathTypeCache)
                UNSAFE.getObject(level, PATH_TYPES_OFFSET);
    }
}
