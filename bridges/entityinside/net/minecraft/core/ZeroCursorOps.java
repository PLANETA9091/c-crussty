package net.minecraft.core;

import java.util.Iterator;

/**
 * CRUSSTY_ZERO_CURSOR lever #11 v1 (TASK-330) — ops bridge.
 *
 * Redirect target for the synthetic lambda behind
 * BlockPos.betweenCornersInDirection (javac: lambda$betweenCornersInDirection$8,
 * the (Direction,Direction,Direction,IIIIII) -> Iterator factory that news a
 * BlockPos$6 per call). ZeroCursorOps.lambda8 receives the IDENTICAL arguments
 * and returns a pooled, reset ZeroCursorIter — the emitted position sequence is
 * bit-exact (CursorLockstepHarness enforces vs the real vanilla iterator).
 *
 * MUST stay a single classfile with ZERO nested classes (kernel-loader define
 * discipline, same as ChunkParseDiagOps/ZeroAllocOps).
 *
 * Pool design: thread-local ring of RING_DEPTH iterators; each lambda8 call
 * takes the next slot and resets it. Live iterators per thread never exceed
 * 1-2 (the consumer drains before the next call; block reads inside the
 * visitor cannot re-enter betweenCornersInDirection), RING_DEPTH=4 gives a
 * wide safety margin. Abandoned (short-circuited) iterators are simply
 * overwritten on later turns — nothing holds them. Diag counters are plain
 * volatile longs (reset-by-ring needs no CAS).
 */
public final class ZeroCursorOps {

    private static final int RING_DEPTH = 4;
    private static final ThreadLocal<ZeroCursorIter[]> RING =
        ThreadLocal.withInitial(() -> new ZeroCursorIter[RING_DEPTH]);
    private static final ThreadLocal<Integer> TURN = ThreadLocal.withInitial(() -> 0);

    public static volatile long pooledCalls;
    public static volatile long stepsServed;

    private ZeroCursorOps() {
    }

    /// Redirect target: same descriptor as lambda$betweenCornersInDirection$8.
    public static Iterator<BlockPos.MutableBlockPos> lambda8(
        Direction direction1, Direction direction2, Direction direction3,
        int x0, int y0, int z0, int extThird, int extSecond, int extFirst) {
        int turn = (TURN.get() + 1) % RING_DEPTH;
        TURN.set(turn);
        ZeroCursorIter[] ring = RING.get();
        ZeroCursorIter it = ring[turn];
        if (it == null) {
            it = new ZeroCursorIter();
            ring[turn] = it;
        }
        it.reset(direction1, direction2, direction3, x0, y0, z0, extThird, extSecond, extFirst);
        pooledCalls = pooledCalls + 1L;
        return it;
    }

    /// Optional diagnostics (read by the harness; never armed in production).
    public static long pooledCallsSnapshot() {
        return pooledCalls;
    }
}
