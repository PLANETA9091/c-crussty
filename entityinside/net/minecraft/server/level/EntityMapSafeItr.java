package net.minecraft.server.level;

import it.unimi.dsi.fastutil.objects.ObjectIterator;
import java.util.NoSuchElementException;

/**
 * TASK-411-A entityMap fence: bound-checked, fail-dominant iterator
 * (EntityMapOps contract).
 *
 * TASK-411-A checklist "ВСЕ индексации с bound-check": the fastutil
 * ValueIterator downscan (key[--pos]) is the ONE -1 indexer of the
 * incident (message-ful "Index -1 ... length 131073"). This wrapper
 * guards every downcall: an ArrayIndexOutOfBoundsException means the
 * underlying table was corrupted by an unfenced mutation (not possible
 * while every site is fenced — belt and suspenders) — the iterator
 * degrades to an early end (vanilla weakly-consistent semantics) instead
 * of crashing the tick. No exception is thrown when the table is healthy:
 * a try block without a throw is free on the JVM.
 */
public final class EntityMapSafeItr implements ObjectIterator {

    private final ObjectIterator inner;
    private boolean dead = false;

    EntityMapSafeItr(ObjectIterator inner) {
        this.inner = inner;
    }

    @Override
    public boolean hasNext() {
        if (dead) {
            return false;
        }
        try {
            return inner.hasNext();
        } catch (ArrayIndexOutOfBoundsException corruptedTable) {
            dead = true;
            return false;
        }
    }

    @Override
    public Object next() {
        if (dead) {
            throw new NoSuchElementException();
        }
        try {
            return inner.next();
        } catch (ArrayIndexOutOfBoundsException corruptedTable) {
            dead = true;
            throw new NoSuchElementException();
        }
    }

    @Override
    public int skip(int n) {
        int s = 0;
        while (s < n && hasNext()) {
            next();
            s++;
        }
        return s;
    }

    @Override
    public void remove() {
        // fail-dominant: vanilla NEVER calls remove() on the entityMap
        // value-iterator (javap census of all 3 values() sites: plain
        // hasNext/next sweeps only). Refuse rather than race the fence.
        throw new UnsupportedOperationException("entityMap fence: iterator.remove");
    }
}
