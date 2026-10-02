package harness;

import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.Random;

import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.ZeroCursorIter;
import net.minecraft.core.ZeroCursorOps;

/**
 * CursorLockstepHarness (TASK-330, CRUSSTY_ZERO_CURSOR v1): randomized bit-exact
 * lockstep of the pooled ZeroCursorIter replica vs the REAL vanilla
 * lambda$betweenCornersInDirection$8 factory (the exact code the redirect
 * replaces), driven over randomized direction permutations and integer bounds.
 *
 * PASS criterion: for every scenario, the emitted (x,y,z) position sequences
 * are identical element-by-element, including length and the hasNext()
 * contract after exhaustion.
 *
 * Usage: java -cp kernel.jar:zerocursor-classes.jar harness.CursorLockstepHarness [scenarios]
 */
public final class CursorLockstepHarness {

    private static final Direction[] DIRS = Direction.values();

    public static void main(String[] args) throws Exception {
        int scenarios = args.length > 0 ? Integer.parseInt(args[0]) : 100_000;
        Method vanillaLambda = BlockPos.class.getDeclaredMethod(
            "lambda$betweenCornersInDirection$8",
            Direction.class, Direction.class, Direction.class,
            int.class, int.class, int.class, int.class, int.class, int.class);
        vanillaLambda.setAccessible(true);

        Random rnd = new Random(0xC0FFE42L);
        long checkedPositions = 0;
        for (int s = 0; s < scenarios; s++) {
            // three DISTINCT directions (the vanilla caller passes distinct
            // axis directions in dominant-travel order)
            Direction d1 = DIRS[rnd.nextInt(DIRS.length)];
            Direction d2 = DIRS[rnd.nextInt(DIRS.length)];
            Direction d3 = DIRS[rnd.nextInt(DIRS.length)];
            int x0 = rnd.nextInt(33) - 16;
            int y0 = rnd.nextInt(33) - 16;
            int z0 = rnd.nextInt(33) - 16;
            int extThird = rnd.nextInt(8);      // 0..7 positions per axis
            int extSecond = rnd.nextInt(8);
            int extFirst = rnd.nextInt(8);

            List<long[]> expected = drain((Iterator<?>) vanillaLambda.invoke(null,
                d1, d2, d3, x0, y0, z0, extThird, extSecond, extFirst));
            List<long[]> actual = drain(ZeroCursorOps.lambda8(
                d1, d2, d3, x0, y0, z0, extThird, extSecond, extFirst));

            if (expected.size() != actual.size()) {
                fail(s, d1, d2, d3, x0, y0, z0, extThird, extSecond, extFirst,
                    "size " + expected.size() + " vs " + actual.size());
            }
            for (int i = 0; i < expected.size(); i++) {
                long[] e = expected.get(i);
                long[] a = actual.get(i);
                if (e[0] != a[0] || e[1] != a[1] || e[2] != a[2]) {
                    fail(s, d1, d2, d3, x0, y0, z0, extThird, extSecond, extFirst,
                        "pos#" + i + " (" + e[0] + "," + e[1] + "," + e[2] + ") vs ("
                        + a[0] + "," + a[1] + "," + a[2] + ")");
                }
            }
            // hasNext must stay false after exhaustion (contract parity)
            Iterator<?> mine = ZeroCursorOps.lambda8(
                d1, d2, d3, x0, y0, z0, extThird, extSecond, extFirst);
            drainFully(mine);
            if (mine.hasNext()) {
                fail(s, d1, d2, d3, x0, y0, z0, extThird, extSecond, extFirst,
                    "hasNext()==true after exhaustion");
            }
            checkedPositions += expected.size();
        }
        System.out.println("CURSOR-LOCKSTEP PASS scenarios=" + scenarios
            + " positions=" + checkedPositions
            + " pooledCalls=" + ZeroCursorOps.pooledCallsSnapshot());
    }

    private static List<long[]> drain(Iterator<?> it) {
        List<long[]> out = new ArrayList<>();
        while (it.hasNext()) {
            Object o = it.next();
            BlockPos p = (BlockPos) o;
            out.add(new long[] {p.getX(), p.getY(), p.getZ()});
        }
        return out;
    }

    private static void drainFully(Iterator<?> it) {
        while (it.hasNext()) {
            it.next();
        }
    }

    private static void fail(int scenario, Direction d1, Direction d2, Direction d3,
                             int x0, int y0, int z0, int e3, int e2, int e1, String why) {
        throw new AssertionError("CURSOR-LOCKSTEP FAIL scenario=" + scenario
            + " dirs=(" + d1 + "," + d2 + "," + d3 + ")"
            + " base=(" + x0 + "," + y0 + "," + z0 + ")"
            + " ext=(" + e3 + "," + e2 + "," + e1 + ") — " + why);
    }

    private CursorLockstepHarness() {
    }
}
