package net.minecraft.world.level.chunk.storage;

import com.mojang.serialization.Codec;
import java.lang.reflect.Method;
import java.util.HashMap;
import java.util.IdentityHashMap;
import java.util.Map;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.world.level.ChunkPos;
import net.minecraft.world.level.chunk.PalettedContainer;

/**
 * CHUNK-PARSE SECTION-CACHE (TASK-419-C, chunk-pipeline lever cmp419_chunk,
 * law 8: player-visible chunk-loading axis; RECON-13b/13f: the parse path is
 * the top alloc lane of the 150k scene — 33.38% of ALL alloc bytes in the
 * 240-300s burst window; codec machinery 19.06% + paletted-decode 13.98%).
 *
 * The redirected vanilla section-decode lambda
 * ({@code SerializableChunkData.lambda$parse$5(Codec, ChunkPos, int,
 * CompoundTag)} — bootstrap-verified block_states site) is replaced by this
 * cache-first decoder:
 *
 * <ul>
 *   <li><b>HIT</b> (same codec instance + structurally equal block_states
 *       tag): return {@code template.copy()} — a deep copy of a container
 *       that the vanilla machinery produced from an EQUAL tag (Data.copy =
 *       storage.copy + palette.copy, strategy/presetValues preserved). The
 *       entire codec/MapDecoder/DataResult/palette/SimpleBitStorage machinery
 *       is skipped.</li>
 *   <li><b>MISS</b>: the pristine twin lambda {@code lambda$parse$7}
 *       (biomes site, byte-identical body, NOT patched) is invoked
 *       reflectively with the same arguments — the exact vanilla decode,
 *       including promotePartial logErrors and the ChunkReadException error
 *       path. Parity by construction.</li>
 * </ul>
 *
 * Parity contract (law 4):
 * <ul>
 *   <li>empty lever flag -> this class is never defined, the lambda body is
 *       byte-identical vanilla (dormant-invisible);</li>
 *   <li>HIT values are value-identical to a fresh vanilla decode;</li>
 *   <li>the codec key is IDENTITY-based (IdentityHashMap) — anti-xray preset
 *       codecs (a fresh codec object per section) can never cross-hit;</li>
 *   <li>templates are never handed out — every caller receives a fresh
 *       copy(); the template itself is unreachable from game code;</li>
 *   <li>probe allocation is ZERO (the tag itself is the inner-map key; its
 *       hashCode is structural — every primitive tag is a record, CompoundTag
 *       = Map.hashCode, ListTag = List.hashCode — and equals is deep).</li>
 * </ul>
 *
 * Delivery discipline: this classfile is defined ALONE into the kernel loader
 * together with its compiled form only — the source declares ZERO nested
 * classes (an inner class would detonate as NoClassDefFoundError; the
 * rust-side guard pins this).
 *
 * Residual (documented): a section that produces a partial decode warning
 * (logErrors) logs only on the cache MISS; identical repeats skip the
 * duplicate log line. Game state is unaffected.
 *
 * The selftest (first {@link #SELFTEST_SECTIONS} distinct sections) re-decodes
 * through the twin and compares every one of the 4096 {@code get(index)}
 * results by identity plus the container bit config; the PASS marker goes to
 * stdout as the bench effect-marker for the verdict checklist.
 *
 * Grep markers: "cmp419_chunk: parse-cache".
 */
public final class ChunkParseOps {

    private ChunkParseOps() {}

    /** Inner-cache bound (per codec); overflow clears that codec's map. */
    static final int CACHE_CAP = 1024;

    /** Sections decoded through the selftest compare window. */
    static final int SELFTEST_SECTIONS = 3;

    /** Marker/log prefix (matches the rust ARM markers). */
    static final String PFX = "[crussty-plugin] cmp419_chunk:";

    /**
     * codec(identity) -> (tag -> pristine decoded template). synchronized:
     * parse runs on chunk-load worker threads; the sync also publishes the
     * templates safely to every copying reader.
     */
    private static final Map<Object, HashMap<CompoundTag, PalettedContainer<?>>> CACHE =
            new IdentityHashMap<>();
    private static long hits = 0;
    private static long misses = 0;
    private static long sections = 0;
    private static int selftestLeft = SELFTEST_SECTIONS;
    private static boolean firstHitLogged = false;

    /**
     * Re-entrancy guard: the twin must be pristine vanilla; if a misconfig
     * ever patched it too, the reflective call would re-enter parseSection.
     * depth>0 routes to the reflection replica below (never expected to
     * run; documented never-path).
     */
    private static final ThreadLocal<int[]> DEPTH =
            ThreadLocal.withInitial(() -> new int[1]);

    /** Twin lambda name, injected by the rust activator before READY. */
    private static volatile String twinName = null;
    private static volatile Method twinMethod = null;

    /**
     * Called by the rust activator (JNI, static void (String)) AFTER the
     * bridge is defined and BEFORE the lambda retransform flips READY. The
     * name was verified against the pristine classfile bytes (exactly two
     * section-lambda candidates with the canonical descriptor exist).
     */
    public static void init(String twin) {
        twinName = twin;
        System.out.println(PFX + " bridge init ok (twin=" + twin + ")");
    }

    /**
     * Redirected body of the block_states decode lambda
     * ({@code SerializableChunkData.lambda$parse$5}). The declared parameter
     * type {@code Codec} is only a pass-through reference: the descriptor
     * must match the vanilla lambda exactly for the static body redirect
     * (stack shape contract); no codec member is ever invoked directly.
     */
    public static PalettedContainer<?> parseSection(
            Codec<?> codec, ChunkPos pos, int y, CompoundTag tag) {
        int[] d = DEPTH.get();
        if (d[0] > 0) {
            // never-path: twin was somehow patched too — bypass the cache.
            return vanillaReplica(codec, pos, y, tag);
        }
        d[0]++;
        try {
            HashMap<CompoundTag, PalettedContainer<?>> inner;
            PalettedContainer<?> tpl;
            synchronized (CACHE) {
                inner = CACHE.get(codec);
                if (inner == null) {
                    inner = new HashMap<>();
                    CACHE.put(codec, inner);
                }
                tpl = inner.get(tag);
            }
            if (tpl != null) {
                hits++;
                if (!firstHitLogged) {
                    firstHitLogged = true;
                    System.out.println(
                            PFX + " parse-cache first hit (chunk section reuse live)");
                }
                return tpl.copy();
            }
            misses++;
            PalettedContainer<?> fresh = invokeTwin(codec, pos, y, tag);
            PalettedContainer<?> copy = fresh.copy();
            synchronized (CACHE) {
                if (inner.size() >= CACHE_CAP) {
                    inner.clear();
                }
                inner.put(tag, copy);
            }
            sections++;
            if (selftestLeft > 0) {
                selftestLeft--;
                selftest(pos, y, codec, tag, fresh);
            } else if ((sections & 8191L) == 0) {
                long total = hits + misses;
                System.out.println(PFX + " parse-cache stats sections=" + sections
                        + " hits=" + hits + " misses=" + misses
                        + " rate=" + (total == 0 ? 0 : (hits * 100 / total)) + "%");
            }
            return fresh;
        } finally {
            d[0]--;
        }
    }

    /** Reflective call of the pristine twin lambda (exact vanilla body). */
    private static PalettedContainer<?> invokeTwin(
            Object codec, ChunkPos pos, int y, CompoundTag tag) {
        try {
            Method m = twinMethod;
            if (m == null) {
                String name = twinName;
                if (name == null) {
                    throw new IllegalStateException("twin not injected");
                }
                for (Method cand : SerializableChunkData.class.getDeclaredMethods()) {
                    if (cand.getName().equals(name)) {
                        cand.setAccessible(true);
                        m = cand;
                        twinMethod = m;
                        break;
                    }
                }
                if (m == null) {
                    throw new NoSuchMethodException(name);
                }
            }
            return (PalettedContainer<?>) m.invoke(null, codec, pos, y, tag);
        } catch (Throwable t) {
            // The twin is guaranteed to exist (rust-side pristine-bytes
            // guard); any failure here is a delivery defect — surface it
            // loudly and rethrow as a chunk-load failure exactly like the
            // vanilla error path would.
            System.out.println(PFX + " twin invoke failed: " + t);
            if (t instanceof RuntimeException) {
                throw (RuntimeException) t;
            }
            throw new IllegalStateException("cmp419_chunk twin invoke", t);
        }
    }

    /**
     * Selftest: decode the SAME section again through the pristine twin and
     * compare bit-in-bit (identity block states + bit config) against a
     * fresh copy of the cached template. Prints the PASS marker used by the
     * bench verdict checklist.
     */
    private static void selftest(ChunkPos pos, int y, Object codec,
                                 CompoundTag tag, PalettedContainer<?> fresh) {
        try {
            PalettedContainer<?> twin = invokeTwin(codec, pos, y, tag);
            PalettedContainer<?> cached;
            synchronized (CACHE) {
                cached = CACHE.get(codec).get(tag);
            }
            PalettedContainer<?> hit = cached.copy();
            boolean ok = twin.getClass() == hit.getClass()
                    && twin.bitsPerEntry() == hit.bitsPerEntry();
            if (ok) {
                for (int i = 0; i < 4096; i++) {
                    if (twin.get(i) != hit.get(i)) {
                        ok = false;
                        break;
                    }
                }
            }
            // Verdict lines are FULL compile-time constants (PFX folds):
            // they stay plain ldc strings in the classfile constant pool, so
            // the javap blob-sync gate (check_blobs_sync.sh) can pin them.
            if (ok) {
                System.out.println(PFX + " parse-cache selftest PASS");
            } else {
                System.out.println(PFX + " parse-cache selftest FAIL");
            }
            System.out.println(PFX + " parse-cache selftest details y=" + y
                    + " bits=" + twin.bitsPerEntry() + " pos=" + pos.x + "," + pos.z);
        } catch (Throwable t) {
            System.out.println(PFX + " parse-cache selftest FAIL (throwable " + t + ")");
        }
    }

    /**
     * NEVER-PATH reflection replica of the vanilla lambda body:
     * codec.parse(NbtOps, tag).promotePartial(-> log).getOrThrow(msg -> new
     * ChunkReadException(msg)). Used only if the depth guard ever fires.
     */
    private static PalettedContainer<?> vanillaReplica(
            Object codec, ChunkPos pos, int y, CompoundTag tag) {
        try {
            Class<?> ops = Class.forName("com.mojang.serialization.DynamicOps");
            Class<?> dr = Class.forName("com.mojang.serialization.DataResult");
            Method parse = codec.getClass().getMethod("parse", ops, Object.class);
            Object res = parse.invoke(codec, net.minecraft.nbt.NbtOps.INSTANCE, tag);
            Method promote = dr.getMethod("promotePartial", java.util.function.Consumer.class);
            res = promote.invoke(res, (java.util.function.Consumer<Object>) msg ->
                    System.out.println("Recoverable errors when loading section ["
                            + pos.x + ", " + y + ", " + pos.z + "]: " + msg));
            Class<?> cre = Class.forName(
                    "net.minecraft.world.level.chunk.storage.SerializableChunkData$ChunkReadException");
            java.lang.reflect.Constructor<?> ctor = cre.getDeclaredConstructor(String.class);
            ctor.setAccessible(true);
            Method getOrThrow = dr.getMethod("getOrThrow", java.util.function.Function.class);
            Object out = getOrThrow.invoke(res, (java.util.function.Function<Object, Object>) msg -> {
                try {
                    return ctor.newInstance(msg);
                } catch (ReflectiveOperationException e) {
                    throw new IllegalStateException("ChunkReadException", e);
                }
            });
            return (PalettedContainer<?>) out;
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException("cmp419_chunk vanilla replica", e);
        }
    }

    /** Diagnostics for the boot/absorb greps (never allocates on hot path). */
    public static String stats() {
        return PFX + " sections=" + sections + " hits=" + hits + " misses=" + misses
                + " codecs=" + CACHE.size() + " cap=" + CACHE_CAP
                + " selftest=" + (SELFTEST_SECTIONS - selftestLeft);
    }
}
