package dev.crussty.ncf;

import java.io.ByteArrayInputStream;
import java.io.DataInputStream;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.nbt.NbtIo;

/**
 * NCF P3.3/P3.4 — Moonrise RegionDataController.readData hook (session 6
 * CFR record): ca.spottedleaf.moonrise.patches.chunk_system.io
 * .MoonriseRegionFileIO$RegionDataController.readData(int chunkX, int
 * chunkZ) -> ReadData(ReadResult result, DataInputStream input,
 * CompoundTag syncRead, int recalculateCount).
 *
 * Step B: on a native hit, return ReadData SYNC_READ with syncRead =
 * NbtIo.read(...) over the byte[] from ONE NativeChunks.generateChunk call.
 * Any throw / null payload -> NO_DATA (vanilla Java generation, I8 lanes).
 * Byte-identity of the read-back path is pinned by the ncf-staged corpus
 * gates; enablement stays behind the kernel policy registry (I9).
 */
public final class NativeChunksIO {
    private NativeChunksIO() {}

    public static Object readData(int chunkX, int chunkZ) {
        byte[] payload;
        try {
            payload = NativeChunks.generateChunk(chunkX, chunkZ);
        } catch (Throwable t) {
            return null; // NO_DATA -> Java generation (I8)
        }
        if (payload == null) return null;
        try {
            CompoundTag tag = NbtIo.read(
                new DataInputStream(new ByteArrayInputStream(payload)));
            // build Moonrise ReadData{SYNC_READ, null input, tag, 0} via the
            // recorded constructor — reflection keeps this source free of a
            // compile-time Moonrise dep (the agent injects the call site).
            Class<?> rd = Class.forName(
                "ca.spottedleaf.moonrise.patches.chunk_system.io.MoonriseRegionFileIO$RegionDataController$ReadData");
            for (java.lang.reflect.Constructor<?> c : rd.getDeclaredConstructors()) {
                Class<?>[] p = c.getParameterTypes();
                if (p.length == 4 && p[0].isEnum()) {
                    c.setAccessible(true);
                    Object[] args = new Object[4];
                    args[0] = Enum.valueOf((Class<? extends Enum>) p[0].asSubclass(Enum.class), "SYNC_READ");
                    args[2] = tag;
                    return c.newInstance(args);
                }
            }
            return null;
        } catch (Throwable t) {
            return null; // NO_DATA on any parse/reflect failure (I8)
        }
    }
}
