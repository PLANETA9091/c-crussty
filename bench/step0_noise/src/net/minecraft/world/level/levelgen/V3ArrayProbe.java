package net.minecraft.world.level.levelgen;

import java.lang.reflect.Field;
import net.minecraft.core.Holder;
import net.minecraft.util.RandomSource;
import net.minecraft.world.level.levelgen.synth.NormalNoise;

/**
 * TASK-108 v3 phase-1 probe (PROGRESS-6, agent-7625532f): standalone
 * bit-exactness check of the array-form density tree interpreter
 * ({@link DensityArrayInterpreter#eval}) against the vanilla per-point
 * oracle, on REAL kernel noise instances (same closed-lib natives as the
 * server). CPU-only, no deploy, no boot.
 *
 * Oracle: for each point i, root.compute(provider.forIndex(i)) — the exact
 * vanilla evaluation. Treatment: one DensityArrayInterpreter.eval(root, out,
 * provider) call per size. Verdict: raw-bits equality at every size.
 *
 * Also: node census dump (design §7.5 — the fallback share must be visible),
 * native-arming assertion (liveHandles >= 3 — the batch kernels MUST be
 * armed, a fallback-only pass would be vacuous), and a NEGATIVE control
 * (perturbed tree must mismatch) to prove the comparator has teeth.
 * Not covered here (in-server smoke, next tick): interpFillArray branch
 * (fillingCell pass-through) and the NoiseChunk$1 provider protocol.
 */
public final class V3ArrayProbe {

    private static Object F(final Class<?> k, final String name, final Object o) throws Exception {
        final Field f = k.getDeclaredField(name);
        f.setAccessible(true);
        return f.get(o);
    }

    private static boolean bitEqual(final double[] a, final double[] b) {
        for (int i = 0; i < a.length; i++) {
            if (Double.doubleToRawLongBits(a[i]) != Double.doubleToRawLongBits(b[i])) {
                return false;
            }
        }
        return true;
    }

    private static String firstMismatch(final double[] a, final double[] b) {
        for (int i = 0; i < a.length; i++) {
            if (Double.doubleToRawLongBits(a[i]) != Double.doubleToRawLongBits(b[i])) {
                return " first@" + i + " want=" + Double.toHexString(b[i])
                        + " got=" + Double.toHexString(a[i]);
            }
        }
        return "";
    }

    /** Builds the mixed-tier probe tree. Every tier of design §7.5 present. */
    private static DensityFunction buildTree(final long seedA, final long seedB) throws Exception {
        final NormalNoise nn1 = NormalNoise.create(RandomSource.create(seedA), -3, 1.0, 1.0, 1.0, 1.0, 1.0);
        final NormalNoise nn2 = NormalNoise.create(RandomSource.create(seedB), -7, 1.0, 1.0);
        final NormalNoise nn3 = NormalNoise.create(RandomSource.create(seedA + 555L), -4, 1.0, 1.0, 1.0);

        final NormalNoise.NoiseParameters p1 = (NormalNoise.NoiseParameters) F(NormalNoise.class, "parameters", nn1);
        final NormalNoise.NoiseParameters p2 = (NormalNoise.NoiseParameters) F(NormalNoise.class, "parameters", nn2);
        final NormalNoise.NoiseParameters p3 = (NormalNoise.NoiseParameters) F(NormalNoise.class, "parameters", nn3);

        final DensityFunction.NoiseHolder h1 = new DensityFunction.NoiseHolder(Holder.direct(p1), nn1);
        final DensityFunction.NoiseHolder h2 = new DensityFunction.NoiseHolder(Holder.direct(p2), nn2);
        final DensityFunction.NoiseHolder h3 = new DensityFunction.NoiseHolder(Holder.direct(p3), nn3);

        // tier 1 leaves
        final DensityFunctions.Noise noiseLeaf = new DensityFunctions.Noise(h1, 0.5, 1.25);
        final DensityFunctions.ShiftA shiftA = new DensityFunctions.ShiftA(h2);
        final DensityFunctions.ShiftB shiftB = new DensityFunctions.ShiftB(h2);
        final DensityFunction gradient =
                DensityFunctions.yClampedGradient(-64, 320, -1.0, 1.0);

        // tier 2 combinators (kernel-own transform / array loops)
        final DensityFunctions.Mapped square = DensityFunctions.Mapped.create(
                DensityFunctions.Mapped.Type.SQUARE, noiseLeaf);
        final DensityFunctions.Clamp clamp = new DensityFunctions.Clamp(shiftA, -2.0, 2.0);
        final DensityFunctions.HolderHolder held = new DensityFunctions.HolderHolder(
                Holder.direct((DensityFunction) shiftB));

        // tier 3 fallback exercises (real kernel classes the interpreter
        // deliberately does not batch: ShiftedNoise + RangeChoice)
        final DensityFunctions.ShiftedNoise shifted = new DensityFunctions.ShiftedNoise(
                DensityFunctions.constant(11.0), DensityFunctions.constant(-7.0),
                DensityFunctions.constant(3.0), 0.25, 0.5, h3);
        final DensityFunction ranged = DensityFunctions.rangeChoice(
                gradient, -0.5, 0.5, shifted, clamp);

        // root: MAX( ADD( MUL(square, gradient), clamp(held)), MIN(shiftB, ranged) )
        return DensityFunctions.max(
                DensityFunctions.add(
                        DensityFunctions.mul(square, gradient),
                        clamp),
                DensityFunctions.min(
                        held,
                        ranged));
    }

    public static void main(final String[] args) throws Exception {
        System.load(args[0]);
        net.minecraft.SharedConstants.tryDetectVersion(); // ServerBuildInfo needs a game version
        net.minecraft.server.Bootstrap.bootStrap(); // DensityFunctions codecs need registries
        int failures = 0;

        final NormalNoiseBatchOps.TestProvider tp = new NormalNoiseBatchOps.TestProvider();

        final DensityFunction root = buildTree(1234L, 4321L);
        final DensityFunction mutated = buildTree(1235L, 4320L); // negative control

        final int[] sizes = {1, 17, 256, 1024};
        boolean anyControlMismatch = false;

        for (final int n : sizes) {
            // vanilla oracle (per-point, exact vanilla evaluation order)
            final double[] want = new double[n];
            for (int i = 0; i < n; i++) {
                want[i] = root.compute(tp.forIndex(i));
            }
            // array-form interpreter
            DensityArrayInterpreter.censusReset();
            final double[] got = new double[n];
            DensityArrayInterpreter.eval(root, got, tp);

            final boolean ok = bitEqual(want, got);
            if (!ok) {
                failures++;
            }
            System.out.println("V3 size=" + n + (ok ? " BIT-EXACT" : " MISMATCH" + firstMismatch(want, got)));
            System.out.println("  " + DensityArrayInterpreter.censusDump());

            // negative control: a different tree MUST produce different bits
            // on at least one point (comparator discriminating power)
            final double[] wrong = new double[n];
            for (int i = 0; i < n; i++) {
                wrong[i] = mutated.compute(tp.forIndex(i));
            }
            if (!bitEqual(want, wrong)) {
                anyControlMismatch = true;
            }
        }

        final int handles = NormalNoiseBatchOps.liveHandles();
        // Arming ledger: nn1 (Noise leaf) + nn2 (ShiftA/ShiftB share h2) = 2
        // handles MUST exist — a null handle inside fillNoise/fillShift means
        // the batch path silently degraded to the vanilla fallback. nn3
        // (ShiftedNoise) lives inside the tier-3 RangeChoice and is evaluated
        // by the node's own vanilla compute — no handle BY DESIGN.
        final boolean armed = handles >= 2;
        System.out.println("V3 native-arming liveHandles=" + handles + (armed ? " ARMED" : " NOT-ARMED (vacuous pass!)"));
        if (!armed) {
            failures++;
        }
        System.out.println("V3 negative-control " + (anyControlMismatch ? "MISMATCHES-AS-EXPECTED" : "VACUOUS (comparator has no teeth!)"));
        if (!anyControlMismatch) {
            failures++;
        }

        System.out.println("V3 PROBE " + (failures == 0 ? "PASS-ALL" : "FAIL(" + failures + ")"));
    }
}
