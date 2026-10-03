package net.minecraft.core;

import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * CRUSSTY_ZERO_CURSOR lever #11 v1 (TASK-330): pooled bit-exact replica of the
 * vanilla anonymous iterator behind BlockPos.betweenCornersInDirection
 * (javac: BlockPos$6, a guava AbstractIterator).
 *
 * The vanilla iterator is allocated FRESH for every betweenCornersInDirection
 * call; on the X150K scene that is one iterator + one MutableBlockPos per
 * entity per tick per travel step (fresh profile s7169: cursor family =
 * 29.18% of all alloc samples, 6.42% CPU). This replica is pooled by
 * ZeroCursorOps (thread-local rotating ring) and reset per call, killing the
 * allocation churn while emitting the IDENTICAL position sequence.
 *
 * Semantics replicated 1:1 from the vanilla computeNext walk: position =
 * base + firstDir * firstIndex + secondDir * secondIndex + thirdDir * thirdIndex,
 * third index is the fastest, then second, then first; `end` after the last
 * combination. Bit-exactness is enforced EMPIRICALLY by
 * entityinside/harness/CursorLockstepHarness.java (randomized lockstep vs the
 * real vanilla iterator), not trusted from bytecode reading.
 *
 * NOT a guava AbstractIterator subclass on purpose: AbstractIterator carries a
 * private state field that cannot be reset for pooling, so the hasNext/next
 * state machine is reimplemented with the same observable contract
 * (hasNext peeks one step ahead, next consumes; NoSuchElementException after
 * exhaustion).
 */
public final class ZeroCursorIter implements Iterator<BlockPos.MutableBlockPos> {

    private final BlockPos.MutableBlockPos cursor = new BlockPos.MutableBlockPos();

    private int firstDirX, firstDirY, firstDirZ;
    private int secondDirX, secondDirY, secondDirZ;
    private int thirdDirX, thirdDirY, thirdDirZ;

    private int baseX, baseY, baseZ;
    private int boundThird, boundSecond, boundFirst;

    private int firstIndex, secondIndex, thirdIndex;
    private boolean end;
    private boolean started;
    private boolean hasCurrent;

    public void reset(Direction direction1, Direction direction2, Direction direction3,
                      int x0, int y0, int z0, int extThird, int extSecond, int extFirst) {
        this.firstDirX = direction1.getStepX();
        this.firstDirY = direction1.getStepY();
        this.firstDirZ = direction1.getStepZ();
        this.secondDirX = direction2.getStepX();
        this.secondDirY = direction2.getStepY();
        this.secondDirZ = direction2.getStepZ();
        this.thirdDirX = direction3.getStepX();
        this.thirdDirY = direction3.getStepY();
        this.thirdDirZ = direction3.getStepZ();
        this.baseX = x0;
        this.baseY = y0;
        this.baseZ = z0;
        this.boundThird = extThird;
        this.boundSecond = extSecond;
        this.boundFirst = extFirst;
        this.firstIndex = 0;
        this.secondIndex = 0;
        this.thirdIndex = 0;
        this.end = false;
        this.started = false;
        this.hasCurrent = false;
    }

    /// Vanilla BlockPos$6.computeNext walk (bit-exact replication).
    private void step() {
        this.cursor.set(
            this.baseX + this.firstDirX * this.firstIndex
                + this.secondDirX * this.secondIndex + this.thirdDirX * this.thirdIndex,
            this.baseY + this.firstDirY * this.firstIndex
                + this.secondDirY * this.secondIndex + this.thirdDirY * this.thirdIndex,
            this.baseZ + this.firstDirZ * this.firstIndex
                + this.secondDirZ * this.secondIndex + this.thirdDirZ * this.thirdIndex);
        if (this.thirdIndex < this.boundThird) {
            this.thirdIndex++;
        } else {
            this.thirdIndex = 0;
            if (this.secondIndex < this.boundSecond) {
                this.secondIndex++;
            } else {
                this.secondIndex = 0;
                if (this.firstIndex < this.boundFirst) {
                    this.firstIndex++;
                } else {
                    this.end = true;
                }
            }
        }
    }

    @Override
    public boolean hasNext() {
        if (!this.started) {
            this.started = true;
            if (this.end) {
                // vanilla contract: endOfData is returned only by the call
                // AFTER the one that set end — the position computed in that
                // same call was still emitted.
                this.hasCurrent = false;
            } else {
                this.step();
                this.hasCurrent = true;
            }
        }
        return this.hasCurrent;
    }

    @Override
    public BlockPos.MutableBlockPos next() {
        if (!this.hasNext()) {
            throw new NoSuchElementException();
        }
        this.started = false;
        return this.cursor;
    }
}
