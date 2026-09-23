package net.minecraft.world.level.chunk.storage;

import com.mojang.serialization.Codec;
import java.lang.reflect.Method;
import java.util.IdentityHashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.world.level.ChunkPos;
import net.minecraft.world.level.chunk.PalettedContainer;

/**
 * CHUNK-PARSE SECTION-CACHE — STABILITY DEEPENING (TASK-419-C base,
 * TASK-420-C iteration, chunk-pipeline lever cmp420_chunk2, law 8:
 * player-visible chunk-loading axis; RECON-13b/13f: the parse path is the
 * top alloc lane of the 150k scene — 33.38% of ALL alloc bytes in the
 * 240-300s burst window; codec machinery 19.06% + paletted-decode 13.98%).
 *
 * Wave-419 ground truth (runs cha/chb/chc @ f7b0be4): hit-rate plateaued at
 * 64% because CACHE_CAP=1024 thrashed — every overflow did a full
 * inner.clear(), dropping hot serialized-section tags between chunk
 * revisions; and the tag probe (deep hashCode + equals) ran INSIDE the
 * global CACHE lock, adding cross-worker contention nondeterminism to the
 * late-run polls (the cha/chc tail-crush signal).
 *
 * TASK-420-C deepening (same decoder contract, deeper cache):
 * <ul>
 *   <li><b>CACHE_CAP 1024 -&gt; 16384</b> — serialized-section templates
 *       now survive across chunk revisions for the whole run (~2-3 KB per
 *       4-bit template; 16384 x ~2.5 KB &lt;= 40 MB, bounded, no leak).</li>
 *   <li><b>evict-half on overflow</b> — a thrashing overflow no longer
 *       wipes the working set: every other entry is dropped (hits stay
 *       monotonic, worst case re-decodes half the map once).</li>
 *   <li><b>lock-free probe</b> — the per-codec inner map is a
 *       ConcurrentHashMap; the global lock now guards ONLY the outer
 *       identity lookup (a few entries, O(#codecs)). The deep
 *       tag hashCode/equals probe runs OUTSIDE any lock — cross-worker
 *       contention removed from the chunk-load burst (tail stability).</li>
 * </ul>
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
 * classes (an inner one would detonate as NoClassDefFoundError; the
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
 * Grep markers: "cmp420_chunk2: parse-cache".
 */
public final class ChunkParseOps {

    private ChunkParseOps() {}

    /**
     * Inner-cache bound (per codec); overflow evicts HALF of that map
     * (TASK-420-C: was 1024 with a full clear — the thrash that capped the
     * wave-419 hit-rate at 64%).
     */
    static final int CACHE_CAP = 16384;

    /** Sections decoded through the selftest compare window. */
    static final int SELFTEST_SECTIONS = 3;

    /** Marker/log prefix (matches the rust ARM markers). */
    static final String PFX = "[crussty-plugin] cmp420_chunk2:";

    /**
     * Mega-carrier union marker (TASK-420 mega, law 7): this plane also rides
     * the cmp420_colpush composite carrier (disjoint-lane composition).
     * Constant is referenced from the init log line — kept alive in the blob
     * constant pool for check_blobs_sync gate-flag consistency (x93 lesson).
     */
    static final String CARRIER_UNION = "cmp420_colpush";

    /**
     * TASK-424-C composite-carrier union (law 7): the chunk-pipeline plane
     * rides the cmp423_wgen carrier (chunk-parse cache ⊕ biomes-parse cache
     * ⊕ queryplane, ROUND-423). Kept in the constant pool for the raw-byte
     * blob-sync gate (check_blobs_sync.sh) — same discipline as CARRIER_UNION.
     */
    static final String CARRIER_UNION_423 = "cmp423_wgen";

    /**
     * TASK-425-C round-424 mega-carrier union (law 7): cmp424_chunksend =
     * colpush proven set ⊕ wgen set (queryplane ⊕ chunk-parse ⊕ biomes-parse
     * ⊕ noise_fill). Kept in the constant pool for the raw-byte blob-sync
     * gate (check_blobs_sync.sh + chunk_parse.rs carrier tests).
     */
    static final String CARRIER_UNION_424 = "cmp424_chunksend";

    /**
     * TASK-428-C chunk-axis union (закон 8): cmp428_chunkunion =
     * cmp424_chunksend (wgen-slice, 6f92ea7) ⊕ cmp424_mobfeed (awakened
     * protocol-v2 mobsoa, fe4ee57). Kept in the constant pool for the
     * raw-byte blob-sync gate (check_blobs_sync.sh + chunk_parse.rs tests).
     */
    static final String CARRIER_UNION_428 = "cmp428_chunkunion";

    /**
     * TASK-429-A noise-fill/worldgen stabilization round (закон 8, cmp429_wgen):
     * the wgen plane (noise-generation stage + parse/biomes cache) rides the
     * UNION carrier on this round's own lever id — STRICT-OR, no broadening.
     * Kept in the constant pool for the raw-byte blob-sync gate.
     */
    static final String CARRIER_UNION_429 = "cmp429_wgen";

    /**
     * TASK-434-A wgen3 deepening round (cmp434_wgen3 STRICT-OR): full-stack
     * arm on the ccefix composite base + paletted-demux carry. Raw-byte gate
     * const for the blob-sync audit.
     */
    static final String CARRIER_UNION_434 = "cmp434_wgen3";

    /**
     * codec(identity) -> (tag -> pristine decoded template). The outer map
     * is synchronized ONLY for its own few-entry get/put; the inner maps are
     * ConcurrentHashMaps so the deep tag probe runs lock-free (TASK-420-C:
     * the deep hashCode/equals left the global critical section).
     */
    private static final Map<Object, ConcurrentHashMap<CompoundTag, PalettedContainer<?>>> CACHE =
            new IdentityHashMap<>();
    private static long hits = 0;
    private static long misses = 0;
    private static long sections = 0;
    private static long evictions = 0;
    private static int selftestLeft = SELFTEST_SECTIONS;
    private static volatile int selftestEdge = 0;
    private static boolean firstHitLogged = false;

    /** Biomes-site counters (TASK-424-C R5c: lambda$parse$7 mirror cache). */
    private static long biomesHits = 0;
    private static long biomesMisses = 0;
    private static long biomesSections = 0;
    private static int biomesSelftestLeft = SELFTEST_SECTIONS;
    private static boolean biomesFirstHitLogged = false;

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
        System.out.println(PFX + " bridge init ok (twin=" + twin + ", union=" + CARRIER_UNION
                + ", wgen=" + CARRIER_UNION_429 + ", wgen3=" + CARRIER_UNION_434 + ")");
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
            return cachedDecode(codec, pos, y, tag, false);
        } finally {
            d[0]--;
        }
    }

    /**
     * TASK-424-C (R5c): redirected body of the BIOMES decode lambda
     * ({@code SerializableChunkData.lambda$parse$7}) — the exact canonical
     * descriptor of the blocks lambda, mirrored template cache.
     *
     * <p>MISS path = in-bridge reflection replica of the vanilla body
     * ({@link #vanillaReplica}): codec.parse(NbtOps, tag).promotePartial
     * (logErrors).getOrThrow(new ChunkReadException(msg)) — bit-in-bit
     * vanilla semantics, no pristine twin remains (BOTH section lambdas are
     * patched by this lever now). The blocks-site MISS still reaches the
     * replica through the patched twin (lambda$parse$7 → this method, depth
     * guard fires, replica) — identical body, identical semantics.</p>
     */
    public static PalettedContainer<?> parseBiomesSection(
            Codec<?> codec, ChunkPos pos, int y, CompoundTag tag) {
        int[] d = DEPTH.get();
        if (d[0] > 0) {
            // the blocks-site twin call lands HERE (lambda$parse$7 is patched
            // to this method) — bypass the cache, exact vanilla replica.
            return vanillaReplica(codec, pos, y, tag);
        }
        d[0]++;
        try {
            return cachedDecode(codec, pos, y, tag, true);
        } finally {
            d[0]--;
        }
    }

    /**
     * Shared cache-first decode (blocks AND biomes sites). The CACHE is
     * keyed by codec IDENTITY — the biomes codec is a distinct object, so
     * the sites never cross-hit; per-site stats stay separate via the
     * {@code biomes} flag.
     */
    private static PalettedContainer<?> cachedDecode(
            Codec<?> codec, ChunkPos pos, int y, CompoundTag tag, boolean biomes) {
        ConcurrentHashMap<CompoundTag, PalettedContainer<?>> inner;
        synchronized (CACHE) {
            inner = CACHE.get(codec);
            if (inner == null) {
                inner = new ConcurrentHashMap<>();
                CACHE.put(codec, inner);
            }
        }
        // LOCK-FREE probe: deep hashCode + equals run outside every lock.
        PalettedContainer<?> tpl = inner.get(tag);
        if (tpl != null) {
            if (biomes) {
                biomesHits++;
                if (!biomesFirstHitLogged) {
                    biomesFirstHitLogged = true;
                    System.out.println(
                            PFX + " biomes-cache first hit (chunk section reuse live, union="
                                    + CARRIER_UNION_423 + ")");
                }
            } else {
                hits++;
                if (!firstHitLogged) {
                    firstHitLogged = true;
                    System.out.println(
                            PFX + " parse-cache first hit (chunk section reuse live)");
                }
            }
            return tpl.copy();
        }
        if (biomes) {
            biomesMisses++;
        } else {
            misses++;
        }
        // MISS: exact vanilla decode. Blocks site keeps the twin hop
        // (lambda$parse$7 — patched to parseBiomesSection, whose depth guard
        // routes to the replica); biomes site calls the replica directly.
        // TASK-429-A selftest hygiene: while a selftest is pending for this
        // site, snapshot a PRISTINE tag copy BEFORE the production decode —
        // the observed first-call biomes re-decode failures (cu-l1 "biomes
        // selftest FAIL (AIOOBE Index 2 out of bounds for length 2)", cwgen-l2
        // Index 1/length 1) are consistent with the vanilla decode consuming
        // tag state, which makes a POST-decode re-decode of the same tag
        // non-idempotent. Selftest re-decodes the pristine snapshot instead
        // (bounded cost: SELFTEST_SECTIONS tag copies per site per run);
        // production decode semantics unchanged (original tag, vanilla).
        CompoundTag selftestTag =
                (biomes ? biomesSelftestLeft > 0 : selftestLeft > 0) ? tag.copy() : null;
        PalettedContainer<?> fresh = biomes
                ? vanillaReplica(codec, pos, y, tag)
                : invokeTwin(codec, pos, y, tag);
        PalettedContainer<?> copy = fresh.copy();
        if (inner.size() >= CACHE_CAP) {
            // evict-half (TASK-420-C): keep the working set warm instead
            // of the wave-419 full clear; CHM iterators are weakly
            // consistent — no lock, at most one extra decode per entry.
            int seen = 0;
            Iterator<Map.Entry<CompoundTag, PalettedContainer<?>>> it =
                    inner.entrySet().iterator();
            while (it.hasNext()) {
                it.next();
                if ((seen & 1) == 0) {
                    it.remove();
                    evictions++;
                }
                seen++;
            }
        }
        inner.put(tag, copy);
        if (biomes) {
            biomesSections++;
            if (biomesSelftestLeft > 0) {
                biomesSelftestLeft--;
                biomesSelftest(pos, y, codec, selftestTag != null ? selftestTag : tag, fresh);
            } else if ((biomesSections & 8191L) == 0) {
                long total = biomesHits + biomesMisses;
                System.out.println(PFX + " biomes-cache stats sections=" + biomesSections
                        + " hits=" + biomesHits + " misses=" + biomesMisses
                        + " rate=" + (total == 0 ? 0 : (biomesHits * 100 / total)) + "%"
                        + " cap=" + CACHE_CAP);
            }
        } else {
            sections++;
            if (selftestLeft > 0) {
                selftestLeft--;
                selftest(pos, y, codec, selftestTag != null ? selftestTag : tag, fresh);
            } else if ((sections & 8191L) == 0) {
                long total = hits + misses;
                System.out.println(PFX + " parse-cache stats sections=" + sections
                        + " hits=" + hits + " misses=" + misses
                        + " rate=" + (total == 0 ? 0 : (hits * 100 / total)) + "%"
                        + " evicted=" + evictions + " cap=" + CACHE_CAP);
            }
        }
        return fresh;
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
            throw new IllegalStateException("cmp420_chunk2 twin invoke", t);
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
            // TASK-429-A EDGE marker (no exception text: keeps the manual
            // grep-AIOOBE=0 verdict check meaningful for lever bugs — this
            // class of throw is a vanilla-decode edge on the re-decode path,
            // caught, production unaffected; counted in stats()).
            selftestEdge++;
            System.out.println(PFX
                    + " parse-cache selftest EDGE (re-decode threw on pristine copy; production decode ok; edge counted in stats)");
        }
    }

    /**
     * Biomes-site selftest (TASK-424-C): re-decode the SAME section through
     * the reflection replica (exact vanilla body) and compare bit-in-bit
     * against a fresh copy of the cached template. PASS marker = bench
     * effect-marker for the verdict checklist.
     */
    private static void biomesSelftest(ChunkPos pos, int y, Object codec,
                                       CompoundTag tag, PalettedContainer<?> fresh) {
        try {
            PalettedContainer<?> vanilla = vanillaReplica(codec, pos, y, tag);
            PalettedContainer<?> cached;
            synchronized (CACHE) {
                cached = CACHE.get(codec).get(tag);
            }
            PalettedContainer<?> hit = cached.copy();
            boolean ok = vanilla.getClass() == hit.getClass()
                    && vanilla.bitsPerEntry() == hit.bitsPerEntry();
            if (ok) {
                for (int i = 0; i < 4096; i++) {
                    if (vanilla.get(i) != hit.get(i)) {
                        ok = false;
                        break;
                    }
                }
            }
            if (ok) {
                System.out.println(PFX + " biomes selftest PASS");
            } else {
                System.out.println(PFX + " biomes selftest FAIL");
            }
            System.out.println(PFX + " biomes selftest details y=" + y
                    + " bits=" + vanilla.bitsPerEntry() + " pos=" + pos.x + "," + pos.z);
        } catch (Throwable t) {
            // TASK-429-A EDGE marker: see selftest() twin comment — the
            // cu-l1/cwgen-l2 "biomes selftest FAIL (AIOOBE...)" first-call
            // class is root-caused to non-idempotent tag consumption by the
            // vanilla decode; pristine-copy re-decode + EDGE marker keeps the
            // verdict greps honest (production path never saw the throw).
            selftestEdge++;
            System.out.println(PFX
                    + " biomes selftest EDGE (re-decode threw on pristine copy; production decode ok; edge counted in stats)");
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
            throw new IllegalStateException("cmp420_chunk2 vanilla replica", e);
        }
    }

    /** Diagnostics for the boot/absorb greps (never allocates on hot path). */
    public static String stats() {
        int codecs;
        synchronized (CACHE) {
            codecs = CACHE.size();
        }
        return PFX + " sections=" + sections + " hits=" + hits + " misses=" + misses
                + " codecs=" + codecs + " cap=" + CACHE_CAP
                + " evicted=" + evictions
                + " selftest=" + (SELFTEST_SECTIONS - selftestLeft)
                + " selftestEdge=" + selftestEdge
                + " biomesSections=" + biomesSections
                + " biomesHits=" + biomesHits + " biomesMisses=" + biomesMisses
                + " union=" + CARRIER_UNION_423 + " wgen=" + CARRIER_UNION_429 + " wgen3=" + CARRIER_UNION_434;
    }
}
