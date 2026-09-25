package net.minecraft.world.level.chunk.storage;

import com.mojang.serialization.Codec;
import java.lang.reflect.Constructor;
import java.lang.reflect.Method;
import java.util.Arrays;
import java.util.IdentityHashMap;
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.world.level.ChunkPos;
import net.minecraft.world.level.chunk.DataLayer;
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
 * TASK-459-60 P21 WIDEN (ID-P21, law 11 card): the plane extends to the
 * LIGHT decode sites of SerializableChunkData.parse — the vanilla bootstrap#8
 * indy (Function byte[] -> new DataLayer(bytes), applied to BOTH "BlockLight"
 * and "SkyLight" of every section) is retargeted to {@link #parseLight}:
 * probe key (len, contentHash) + full Arrays.equals cert on every hit (a hash
 * collision can never serve a foreign layer), template = defensive clone,
 * hits hand out fresh clones, miss = EXACTLY the vanilla newInvokeSpecial
 * constructor (twin-fallback by construction). Online selftest re-decode 1/100
 * pins the constructor-invariance precondition. The vanillaReplica twin
 * fallback additionally caches its reflective handles (fast MISS tail).
 *
 * Grep markers: "cmp420_chunk2: parse-cache" + "cmp459_p21".
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
     * TASK-434-C chunk-pipeline R5 carrier union (law 7): the chunk-pipeline
     * plane rides the cmp434_chunkpl carrier (chunk-parse deep cache ⊕
     * biomes-parse cache ⊕ full composite union, ROUND-434-C; lineage:
     * cmp423_wgen ROUND-423 → never merged until this port). Kept in the
     * constant pool for the raw-byte blob-sync gate (check_blobs_sync.sh) —
     * same discipline as CARRIER_UNION.
     */
    static final String CARRIER_UNION_423 = "cmp434_chunkpl";

    /**
     * TASK-435-C chunk-pipeline R6 carrier (law 7/8): STRICT-OR successor
     * id ON TOP of cmp434_chunkpl (same planes: block_states deep cache ⊕
     * biomes-parse cache ⊕ full composite union; no new lever — round-id
     * hygiene for ROUND-435 certification). Kept in the constant pool for
     * the raw-byte blob-sync gate (check_blobs_sync.sh) — x93 lesson.
     */
    static final String CARRIER_UNION_435 = "cmp435_chunk3";

    /**
     * TASK-438-C chunk-pipeline R7 carrier (law 7/8): STRICT-OR successor id
     * ON TOP of cmp435_chunk3 — the composite now also carries the chunk-send
     * serialization snapshot plane (PlayerChunkSender.sendChunk body redirect,
     * unsaved-keyed packet reuse; round-id hygiene for ROUND-438-C). Kept in
     * the constant pool for the raw-byte blob-sync gate (check_blobs_sync.sh)
     * — x93 lesson.
     */
    static final String CARRIER_UNION_437 = "cmp437_chunk4";
    /** TASK-444-B: R8 stage-2 carrier (STRICT-OR; raw-cp marker for the
     * check_blobs_sync gate). */
    static final String CARRIER_UNION_444 = "cmp444_chunk5";
    /** TASK-450-C union carrier (STRICT-OR; raw-cp marker for the
     * check_blobs_sync gate). */
    static final String CARRIER_UNION_450 = "cmp450_chunk";
    /** TASK-452-C mega-composite (senseins ⊕ chunk union; STRICT-OR; raw-cp
     * marker for the check_blobs_sync gate). */
    static final String CARRIER_UNION_452 = "cmp452_mega";
    /** TASK-453-C diet composite (section-codec kept: NOT overlapped by chunk4
     * [parse=disk-load path vs send=player path]; raw-cp marker). */
    static final String CARRIER_UNION_453 = "cmp453_diet";

    /**
     * TASK-459-60 P21 parse-cache WIDEN carrier (law 7/8, STRICT-OR successor
     * on top of the master union): the plane now also covers the LIGHT decode
     * sites (bootstrap#8 retarget -> parseLight, light template cache with the
     * (len,hash) probe + full Arrays.equals cert) and the fast twin-fallback
     * (cached reflective replica handles). Raw-cp marker for the
     * check_blobs_sync gate — x93 lesson.
     */
    static final String CARRIER_UNION_459 = "cmp459_p21";

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
    private static boolean firstHitLogged = false;

    /** Biomes-site counters (TASK-424-C R5c: lambda$parse$7 mirror cache). */
    private static long biomesHits = 0;
    private static long biomesMisses = 0;
    private static long biomesSections = 0;
    private static int biomesSelftestLeft = SELFTEST_SECTIONS;
    private static boolean biomesFirstHitLogged = false;

    /**
     * TASK-459-60 P21: LIGHT cache — the third decode plane of
     * SerializableChunkData.parse. Vanilla applies the bootstrap#8 indy
     * (Function byte[] -> new DataLayer(bytes)) to BOTH light layers of every
     * section; homogeneous pregen light repeats identical arrays heavily
     * (full-empty nibble sections).
     *
     * <p>P21 key (card ID-P21 risk row "tag-hash collisions -> key
     * (codec-identity, tagHash, len)"): a byte[] cannot be a CHM key
     * (identity equals), so the probe key packs (len, contentHash) into one
     * long and EVERY hit re-verifies with a full {@code Arrays.equals} — a
     * hash collision can never serve a foreign layer (probe != cert).</p>
     */
    static final int LIGHT_CACHE_CAP = 8192;
    private static final ConcurrentHashMap<Long, byte[]> LIGHT_CACHE =
            new ConcurrentHashMap<>();
    private static long lightHits = 0;
    private static long lightMisses = 0;
    private static long lightEvictions = 0;
    /** Online selftest sampling (card: "online selftest re-decode 1/100"):
     * every 100th light MISS re-decodes via the vanilla constructor and
     * verifies the byte-in-byte invariant. */
    static final long LIGHT_SELFTEST_EVERY = 100;

    /**
     * Fast twin-fallback (P21): the vanilla replica previously resolved its
     * reflective handles on EVERY miss call (getMethod walk inside the
     * chunk-load burst). The handles are cached in volatile fields — same
     * pattern as {@code twinMethod} below — identical call semantics, minus
     * the per-call lookup cost.
     */
    private static volatile Method replicaParse = null;
    private static volatile Method replicaPromote = null;
    private static volatile Method replicaGetOrThrow = null;
    private static volatile Constructor<?> replicaChunkReadException = null;

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
        System.out.println(PFX + " bridge init ok (twin=" + twin + ", union=" + CARRIER_UNION + ")");
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
                                    + CARRIER_UNION_423 + "/" + CARRIER_UNION_435 + ")");
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
                biomesSelftest(pos, y, codec, tag, fresh);
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
                selftest(pos, y, codec, tag, fresh);
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
            System.out.println(PFX + " parse-cache selftest FAIL (throwable " + t + ")");
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
            System.out.println(PFX + " biomes selftest FAIL (throwable " + t + ")");
        }
    }

    /**
     * NEVER-PATH reflection replica of the vanilla lambda body:
     * codec.parse(NbtOps, tag).promotePartial(-> log).getOrThrow(msg -> new
     * ChunkReadException(msg)). Used only if the depth guard ever fires.
     *
     * <p>P21 fast twin-fallback: the reflective handles are resolved ONCE into
     * the volatile fields above (the twinMethod pattern) instead of on every
     * call — identical call semantics, minus the per-call getMethod walk that
     * used to run inside the chunk-load burst.</p>
     */
    private static PalettedContainer<?> vanillaReplica(
            Object codec, ChunkPos pos, int y, CompoundTag tag) {
        try {
            Method parse = replicaParse;
            if (parse == null || parse.getDeclaringClass() != codec.getClass()) {
                Class<?> ops = Class.forName("com.mojang.serialization.DynamicOps");
                parse = codec.getClass().getMethod("parse", ops, Object.class);
                replicaParse = parse;
            }
            Method promote = replicaPromote;
            if (promote == null) {
                Class<?> dr = Class.forName("com.mojang.serialization.DataResult");
                promote = dr.getMethod("promotePartial", java.util.function.Consumer.class);
                replicaPromote = promote;
            }
            Object res = parse.invoke(codec, net.minecraft.nbt.NbtOps.INSTANCE, tag);
            res = promote.invoke(res, (java.util.function.Consumer<Object>) msg ->
                    System.out.println("Recoverable errors when loading section ["
                            + pos.x + ", " + y + ", " + pos.z + "]: " + msg));
            Constructor<?> ctor = replicaChunkReadException;
            if (ctor == null) {
                Class<?> cre = Class.forName(
                        "net.minecraft.world.level.chunk.storage.SerializableChunkData$ChunkReadException");
                ctor = cre.getDeclaredConstructor(String.class);
                ctor.setAccessible(true);
                replicaChunkReadException = ctor;
            }
            Method getOrThrow = replicaGetOrThrow;
            if (getOrThrow == null) {
                Class<?> dr = Class.forName("com.mojang.serialization.DataResult");
                getOrThrow = dr.getMethod("getOrThrow", java.util.function.Function.class);
                replicaGetOrThrow = getOrThrow;
            }
            final Constructor<?> ctorF = ctor;
            Object out = getOrThrow.invoke(res, (java.util.function.Function<Object, Object>) msg -> {
                try {
                    return ctorF.newInstance(msg);
                } catch (ReflectiveOperationException e) {
                    throw new IllegalStateException("ChunkReadException", e);
                }
            });
            return (PalettedContainer<?>) out;
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException("cmp420_chunk2 vanilla replica", e);
        }
    }

    /**
     * TASK-459-60 P21: redirected impl of the LIGHT decode bootstrap (the
     * vanilla indy was {@code REF_newInvokeSpecial DataLayer.<init>([B)V} —
     * the retarget swaps ONLY the impl handle, the call-site descriptor and
     * the instantiated type {@code ([B)Lnet/minecraft/world/level/chunk/
     * DataLayer;} are unchanged).
     *
     * <ul>
     *   <li><b>HIT</b> (probe (len,hash) + full Arrays.equals cert): return
     *       {@code new DataLayer(template.clone())} — byte-in-byte identical
     *       content, NEVER aliasing the template or the caller array.</li>
     *   <li><b>MISS</b>: {@code new DataLayer(bytes)} — EXACTLY the vanilla
     *       newInvokeSpecial handle (twin-fallback by construction); a
     *       defensive template clone is stored for future hits.</li>
     * </ul>
     *
     * <p>Online selftest 1/100 (card): every 100th MISS re-decodes via the
     * vanilla constructor and verifies
     * {@code Arrays.equals(new DataLayer(bytes).getData(), bytes)} — the
     * precondition of template-copy correctness (the constructor must not
     * transform its input). PASS marker goes to stdout for the verdict
     * checklist.</p>
     */
    public static DataLayer parseLight(byte[] bytes) {
        long key = lightKey(bytes);
        byte[] tpl = LIGHT_CACHE.get(key);
        if (tpl != null && Arrays.equals(tpl, bytes)) {
            lightHits++;
            return new DataLayer(tpl.clone());
        }
        lightMisses++;
        DataLayer fresh = new DataLayer(bytes);
        if (LIGHT_CACHE.size() >= LIGHT_CACHE_CAP) {
            // evict-half (TASK-420-C discipline): keep the working set warm.
            int seen = 0;
            Iterator<Map.Entry<Long, byte[]>> it = LIGHT_CACHE.entrySet().iterator();
            while (it.hasNext()) {
                it.next();
                if ((seen & 1) == 0) {
                    it.remove();
                    lightEvictions++;
                }
                seen++;
            }
        }
        LIGHT_CACHE.put(key, bytes.clone());
        if (lightMisses % LIGHT_SELFTEST_EVERY == 1L) {
            try {
                boolean ok = Arrays.equals(new DataLayer(bytes).getData(), bytes);
                System.out.println(ok
                        ? PFX + " light selftest PASS"
                        : PFX + " light selftest FAIL");
            } catch (Throwable t) {
                System.out.println(PFX + " light selftest FAIL (throwable " + t + ")");
            }
        } else if ((lightMisses & 8191L) == 0) {
            long total = lightHits + lightMisses;
            System.out.println(PFX + " light-cache stats misses=" + lightMisses
                    + " hits=" + lightHits
                    + " rate=" + (total == 0 ? 0 : (lightHits * 100 / total)) + "%"
                    + " evicted=" + lightEvictions + " cap=" + LIGHT_CACHE_CAP);
        }
        return fresh;
    }

    /** P21 light probe key: (len, contentHash) packed into one long. */
    private static long lightKey(byte[] bytes) {
        return ((long) bytes.length << 32) | (Arrays.hashCode(bytes) & 0xFFFFFFFFL);
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
                + " biomesSections=" + biomesSections
                + " biomesHits=" + biomesHits + " biomesMisses=" + biomesMisses
                + " lightMisses=" + lightMisses + " lightHits=" + lightHits
                + " lightCached=" + LIGHT_CACHE.size()
                + " union=" + CARRIER_UNION_423 + "/" + CARRIER_UNION_435
                + "/" + CARRIER_UNION_437 + "/" + CARRIER_UNION_459;
    }
}
