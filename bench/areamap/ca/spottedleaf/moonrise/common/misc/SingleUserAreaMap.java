package ca.spottedleaf.moonrise.common.misc;

/** Smoke-local kernel supertype (same shape as the compile-time stub). */
abstract class SingleUserAreaMap<T> {
    protected abstract void addCallback(T param, int chunkX, int chunkZ);
    protected abstract void removeCallback(T param, int chunkX, int chunkZ);
}
