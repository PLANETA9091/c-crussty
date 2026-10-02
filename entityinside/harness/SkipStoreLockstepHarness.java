package net.minecraft.world.level;

import net.minecraft.world.phys.AABB;

import java.io.IOException;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Random;

/**
 * SKIP-STORE-BB lockstep oracle (#13-SBB, S7-166).
 *
 * OFFLINE bit-exact comparison between the REAL vanilla kernel body of
 * Entity.setBoundingBox(AABB) (kernel jar on the classpath) and the
 * SkipStoreOps bridge called directly from source.
 *
 * Twin instances are Unsafe-allocated CONCRETE EvokerFangs (a light Entity
 * subclass — Entity itself is abstract and Unsafe.allocateInstance refuses
 * it); no constructor runs and the vanilla body touches ONLY the argument
 * and the bb field, so a bare instance is a faithful twin. The bb offset is
 * taken from Entity.class (superclass layout prefix is shared by every
 * subclass instance) — exactly the offset SkipStoreOps uses on the live
 * scene. setBoundingBox is FINAL on Entity, so the subclass instance
 * dispatches straight into the vanilla body.
 *
 * Per scenario (identical pre-state bb on both twins: null / random /
 * skip-forcing / zero-sign-flipped / NaN / clamp-boundary):
 *   - the bridge MUST reproduce the vanilla normalization bit-for-bit
 *     (doubleToLongBits over all six components);
 *   - skip semantics: when the bridge skips (bit-equal pre-state) the field
 *     REFERENCE is preserved (identity invariant, RECON-12a: 0 identity
 *     sites) and its bits equal the vanilla fresh box;
 *   - skip STRICTNESS: a pre-state differing ONLY by the sign of a zero
 *     component must NOT skip (doubleToLongBits is strictly stronger than
 *     the preregistered dcmp-equality; the clip-division sign window);
 *   - non-skip MUST store a fresh reference (predicate honesty).
 *
 * Stage-8 dispatch wiring is audited offline by (1) the HotSpot verifier
 * pass on the fully patched Entity (defineClass = full bytecode
 * verification) and (2) a classfile byte-audit: the patched Entity MUST
 * embed the SkipStoreOps class reference and the receiver-prepended static
 * descriptor (the invokestatic operand materialization). The live
 * invokevirtual dispatch is proven by the CI leg PG2 markers (stage sbb
 * composed Retargeted{sites:1} + skip_store_ops defined + ARMED + 0 NCDFE)
 * — same depth split as the S7-164 precedent harness.
 *
 * >= 1,000,000 scenarios total (preregister TASK-318). Any mismatch =
 * FAIL (exit 2), printed with the exact scenario.
 *
 * INJECTS-ONLY: plain JVM, real kernel classes, NO server boot.
 */
public final class SkipStoreLockstepHarness {
    private static final sun.misc.Unsafe UNSAFE;
    private static final long BB_OFFSET;
    static {
        try {
            Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            UNSAFE = (sun.misc.Unsafe) uf.get(null);
            Field bb = net.minecraft.world.entity.Entity.class.getDeclaredField("bb");
            bb.setAccessible(true);
            BB_OFFSET = UNSAFE.objectFieldOffset(bb);
        } catch (Throwable t) {
            throw new ExceptionInInitializerError(t);
        }
    }

    public static void main(String[] args) throws Exception {
        // ---- Step 0: guarded offline bootstrap (Entity.<clinit> builds
        // serialization codecs; registries must exist before the first
        // virtual call initializes the class). ----
        try {
            net.minecraft.SharedConstants.tryDetectVersion();
        } catch (Throwable t) {
            System.out.println("note: SharedConstants.tryDetectVersion: " + t);
        }
        try {
            net.minecraft.server.Bootstrap.bootStrap();
        } catch (Throwable t) {
            System.out.println("note: Bootstrap.bootStrap: " + t);
        }

        // ---- Step 1: define the stage-8 patched Entity (HotSpot verifier
        // pass) + classfile byte-audit of the redirect wiring. ----
        Path patched = Path.of("entityinside/build/Entity_patched_ssb.class");
        if (!Files.exists(patched)) {
            System.err.println("FAIL: entityinside/build/Entity_patched_ssb.class missing — run cargo test with CRUSSTY_EMIT_PATCHED_ENTITY_SSB");
            System.exit(2);
        }
        byte[] entityBytes = Files.readAllBytes(patched);
        ClassLoader parent = SkipStoreLockstepHarness.class.getClassLoader();
        ClassLoader entityLoader = new ClassLoader(parent) {
            @Override
            protected Class<?> findClass(String name) throws ClassNotFoundException {
                if (name.equals("net.minecraft.world.entity.Entity")) {
                    return defineClass(name, entityBytes, 0, entityBytes.length);
                }
                return super.loadClass(name, true);
            }
        };
        Class<?> patchedEntity = Class.forName("net.minecraft.world.entity.Entity", false, entityLoader);
        System.out.println("lockstep: stage-8 patched Entity verified by HotSpot (major-loadable, " + entityBytes.length + " bytes)");

        boolean hasBridgeRef = indexOf(entityBytes, "SkipStoreOps".getBytes("UTF-8")) >= 0;
        boolean hasStaticDesc = indexOf(entityBytes,
            "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)V".getBytes("UTF-8")) >= 0;
        if (!hasBridgeRef || !hasStaticDesc) {
            System.err.println("FAIL: patched Entity byte-audit: SkipStoreOps ref=" + hasBridgeRef
                + " static-desc=" + hasStaticDesc + " — the stage-8 invokestatic is missing");
            System.exit(2);
        }
        System.out.println("lockstep: patched Entity byte-audit PASS (SkipStoreOps class ref + receiver-prepended static descriptor present)");

        Method sb = null;
        for (Method m : patchedEntity.getMethods()) {
            if (m.getName().equals("setBoundingBox")
                && m.getParameterCount() == 1
                && m.getParameterTypes()[0] == AABB.class) {
                sb = m;
                break;
            }
        }
        if (sb == null) {
            System.err.println("FAIL: redirected setBoundingBox(AABB) missing on patched Entity");
            System.exit(2);
        }

        // ---- Step 2: the oracle loops. Twins = concrete EvokerFangs. ----
        Class<?> twinClass = net.minecraft.world.entity.projectile.EvokerFangs.class;
        Random rng = new Random(42);
        long scenarios = 0;
        long mismatches = 0;
        long skipped = 0;
        long nonSkipped = 0;

        // Lane matrix: [0] 1,000,000 mixed random; [1] 40,000 skip-forcing
        // (pre-state = vanilla-normalized source); [2] 20,000 zero-sign
        // strictness; [3] 40,000 targeted edges (clamp boundaries,
        // inverted, NaN, subnormal, huge).
        for (int lane = 0; lane < 4 && mismatches <= 10; lane++) {
            int count = switch (lane) {
                case 0 -> 1_000_000;
                case 1 -> 40_000;
                case 2 -> 20_000;
                default -> 40_000;
            };
            for (int i = 0; i < count && mismatches <= 10; i++) {
                AABB src = switch (lane) {
                    case 0 -> randomSource(rng, i);
                    case 1 -> randomSource(rng, i);
                    case 2 -> new AABB(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
                    default -> edgeSource(rng, i);
                };
                AABB pre;
                if (lane == 1) {
                    // skip-forcing: pre-state = the exact vanilla-normalized box
                    Object scratch = UNSAFE.allocateInstance(twinClass);
                    UNSAFE.putObject(scratch, BB_OFFSET, null);
                    ((net.minecraft.world.entity.Entity) scratch).setBoundingBox(src);
                    pre = (AABB) UNSAFE.getObject(scratch, BB_OFFSET);
                    if (pre == null) {
                        continue; // cannot build a skip pre-state
                    }
                } else if (lane == 2) {
                    // pre-state = all-zero box with ONE component -0.0
                    double[] z = {0.0, 0.0, 0.0, 0.0, 0.0, 0.0};
                    z[i % 6] = -0.0;
                    pre = new AABB(z[0], z[1], z[2], z[3], z[4], z[5]);
                } else if (lane == 0 && (i % 10) == 7) {
                    pre = null; // null pre-state lane inside the random mix
                } else {
                    pre = randomSource(rng, i + 991);
                }

                // --- twin A: vanilla body ---
                net.minecraft.world.entity.Entity a =
                    (net.minecraft.world.entity.Entity) UNSAFE.allocateInstance(twinClass);
                UNSAFE.putObject(a, BB_OFFSET, pre);
                a.setBoundingBox(src);
                AABB va = (AABB) UNSAFE.getObject(a, BB_OFFSET);

                // --- twin B: bridge called directly (the body the stage-8
                // redirect invokestatics into on the live scene) ---
                net.minecraft.world.entity.Entity b =
                    (net.minecraft.world.entity.Entity) UNSAFE.allocateInstance(twinClass);
                UNSAFE.putObject(b, BB_OFFSET, pre);
                Object b0 = UNSAFE.getObject(b, BB_OFFSET);
                SkipStoreOps.setBoundingBox(b, src);
                AABB vb = (AABB) UNSAFE.getObject(b, BB_OFFSET);

                scenarios++;
                if (!sameBits(va, vb)) {
                    mismatches++;
                    System.err.printf("BITS MISMATCH lane=%d i=%d src=%s pre=%s vanilla=%s bridge=%s%n",
                        lane, i, src, pre, va, vb);
                    continue;
                }
                if (b0 != null && b0 == (Object) vb) {
                    // SKIP: reference preserved (identity invariant)
                    skipped++;
                    if (b0 != pre) {
                        mismatches++;
                        System.err.printf("IDENTITY FAIL lane=%d i=%d pre-ref=%s post=%s%n", lane, i, pre, vb);
                        continue;
                    }
                    if (lane == 2) {
                        mismatches++;
                        System.err.printf("ZERO-SIGN SKIP lane=%d i=%d — dcmp-equality leak (bit-strictness broken)%n", lane, i);
                        continue;
                    }
                } else {
                    nonSkipped++;
                    // predicate honesty: non-skip MUST store a fresh reference
                    if (vb == b0 && b0 != null) {
                        mismatches++;
                        System.err.printf("NON-SKIP KEPT REFERENCE lane=%d i=%d src=%s pre=%s%n", lane, i, src, pre);
                        continue;
                    }
                    if (lane == 1 && pre != null) {
                        mismatches++;
                        System.err.printf("SKIP-FORCING LANE DID NOT SKIP lane=%d i=%d pre=%s src=%s%n", lane, i, pre, src);
                        continue;
                    }
                }
            }
            System.out.println("lockstep: lane " + lane + " done (cumulative scenarios=" + scenarios + ")");
        }

        System.out.println("lockstep: skipped=" + skipped + " nonSkipped=" + nonSkipped);
        if (mismatches != 0) {
            System.err.println("SKIP-STORE LOCKSTEP FAIL mismatches=" + mismatches);
            System.exit(2);
        }
        if (scenarios < 1_000_000) {
            System.err.println("SKIP-STORE LOCKSTEP FAIL scenarios=" + scenarios + " < 1,000,000 (preregister TASK-318)");
            System.exit(2);
        }
        System.out.println("SKIP-STORE LOCKSTEP PASS scenarios=" + scenarios);
    }

    /** mixed random source box (lane 0/1). */
    private static AABB randomSource(Random rng, int i) {
        double minX = (rng.nextDouble() - 0.5) * 64.0;
        double minY = rng.nextDouble() * 320.0;
        double minZ = (rng.nextDouble() - 0.5) * 64.0;
        double ex, ey, ez;
        int mode = rng.nextInt(40);
        if (mode == 0) { ex = 64.0; ey = 64.0; ez = 64.0; }                  // exact clamp boundary
        else if (mode == 1) { ex = 64.0 + 1.0E-12; ey = 63.99999999999999; ez = 64.0 + 1.0E-9; }
        else if (mode == 2) { ex = -(rng.nextDouble() * 2.0); ey = 0.0; ez = 1.0; }  // inverted
        else if (mode == 3) { ex = 0.0; ey = 0.0; ez = 0.0; }                // zero extent
        else if (mode == 4 && (i % 200) == 4) {                              // NaN lane
            double nan = Double.NaN;
            return new AABB(nan, minY, minZ, nan, minY + 1.8, minZ + 0.6);
        }
        else if (mode == 5 && (i % 200) == 5) {                              // -0.0 component
            return new AABB(-0.0, minY, minZ, -0.0 + 0.6, minY + 1.8, minZ + 0.6);
        }
        else if (mode == 6 && (i % 400) == 6) {                              // huge
            double big = 1.0E300;
            return new AABB(-big, -big, -big, big, big, big);
        }
        else if (mode == 7 && (i % 400) == 7) {                              // subnormal
            double tiny = Double.MIN_VALUE;
            return new AABB(tiny, tiny, tiny, tiny + tiny, tiny + tiny, tiny + tiny);
        }
        else {
            ex = rng.nextDouble() * 2.0; ey = 0.05 + rng.nextDouble() * 2.0; ez = rng.nextDouble() * 2.0;
        }
        return new AABB(minX, minY, minZ, minX + ex, minY + ey, minZ + ez);
    }

    /** targeted edge source (lane 3): clamp-boundary neighbourhoods. */
    private static AABB edgeSource(Random rng, int i) {
        double base = (rng.nextDouble() - 0.5) * 32.0;
        double ext = switch (i % 8) {
            case 0 -> 64.0;
            case 1 -> Math.nextUp(64.0);
            case 2 -> Math.nextDown(64.0);
            case 3 -> 64.0 + 1.0E-7;
            case 4 -> -1.0E-9;                    // inverted by epsilon
            case 5 -> 0.0;
            case 6 -> 128.0;                      // clamps to 64.0
            default -> rng.nextDouble() * 100.0;
        };
        double sgn = (rng.nextBoolean() ? 1.0 : -1.0);
        return new AABB(base, base, base, base + ext, base + ext * sgn, base + ext);
    }

    private static boolean sameBits(AABB x, AABB y) {
        if (x == null || y == null) return x == y;
        return Double.doubleToLongBits(x.minX) == Double.doubleToLongBits(y.minX)
            && Double.doubleToLongBits(x.minY) == Double.doubleToLongBits(y.minY)
            && Double.doubleToLongBits(x.minZ) == Double.doubleToLongBits(y.minZ)
            && Double.doubleToLongBits(x.maxX) == Double.doubleToLongBits(y.maxX)
            && Double.doubleToLongBits(x.maxY) == Double.doubleToLongBits(y.maxY)
            && Double.doubleToLongBits(x.maxZ) == Double.doubleToLongBits(y.maxZ);
    }

    private static int indexOf(byte[] hay, byte[] needle) {
        if (needle.length == 0 || needle.length > hay.length) return -1;
        outer:
        for (int i = 0; i <= hay.length - needle.length; i++) {
            for (int j = 0; j < needle.length; j++) {
                if (hay[i + j] != needle[j]) continue outer;
            }
            return i;
        }
        return -1;
    }
}
