package net.minecraft.server.level;

import it.unimi.dsi.fastutil.objects.AbstractObjectCollection;
import it.unimi.dsi.fastutil.objects.ObjectCollection;
import it.unimi.dsi.fastutil.objects.ObjectIterator;

/**
 * TASK-411-A entityMap fence: values() view (part of EntityMapOps — see
 * the race/fail-dominant contract there).
 *
 * Iterators are created INSIDE the map monitor (atomic wrt every fenced
 * mutator — the iterator never observes a mid-rehash table), then walk
 * WITHOUT the lock (weakly consistent, vanilla-identical iteration
 * semantics). EntityMapSafeItr is bound-checked: a corrupted-table AIOOBE
 * degrades to an early end, never a crash.
 */
public final class EntityMapSafeValues extends AbstractObjectCollection {

    private final ObjectCollection inner;
    private final Object lock;

    EntityMapSafeValues(ObjectCollection inner, Object lock) {
        this.inner = inner;
        this.lock = lock;
    }

    @Override
    public ObjectIterator iterator() {
        synchronized (lock) {
            return new EntityMapSafeItr(inner.iterator());
        }
    }

    @Override
    public int size() {
        synchronized (lock) {
            return inner.size();
        }
    }
}
