package net.minecraft.server.network;

import java.lang.reflect.Constructor;
import java.lang.reflect.Method;
import java.util.Arrays;
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.game.ClientboundLevelChunkPacketData;
import net.minecraft.network.protocol.game.ClientboundLevelChunkWithLightPacket;
import net.minecraft.network.protocol.game.ClientboundLightUpdatePacketData;

/**
 * CHUNK-PACKET ENCODE CACHE — STAGE-2 of the chunk-pipeline lever
 * (TASK-444-B, lever cmp444_chunk5, law 8 player-visible chunk-loading axis;
 * rides the cmp437_chunk4 carrier ON TOP of cmp435_chunk3).
 *
 * Kernel ground truth (javap-census 2026-09-24): the vanilla private body of
 * {@code ClientboundLevelChunkWithLightPacket.write(RegistryFriendlyByteBuf)}
 * is {@code buf.writeInt(x); buf.writeInt(z); chunkData.write(buf);
 * lightData.write(buf);} — a pure function of the packet instance (all four
 * fields are final and built in the constructor; no per-player input). With
 * the cmp437_chunk4 snapshot the SAME packet OBJECT is sent to every fake
 * player (up to 4 sends of one instance), so the per-player encode
 * (heightmap codec + block-entity codec + light packing + buffer varint
 * copies) is duplicated up to 4x for identical bytes.
 *
 * This bridge is the redirected body of that private method (rust activator,
 * ONE instance->static body redirect with the receiver prepended, exact
 * stack-shape contract):
 * <ul>
 *   <li><b>HIT</b> (payload cached for this instance): single
 *       {@code buf.writeBytes(payload)} replay — bit-identical bytes, the
 *       whole per-player codec/NBT encode machinery is skipped (the stage-2
 *       half of the encode dedup: chunk4 dedups the CONSTRUCTION, chunk5
 *       dedups the ENCODE).</li>
 *   <li><b>MISS</b>: one vanilla-equivalent encode into a scratch buffer
 *       (the EXACT vanilla body calls on the public getters), one replay into
 *       the channel buffer, then the payload is stashed keyed by the packet
 *       instance.</li>
 *   <li><b>fail-closed</b>: any runtime surprise clears the cache, flips the
 *       DISABLED latch and serves the vanilla body forever; the online
 *       selftest (first 2 misses: SECOND scratch encode compared bit-in-bit
 *       against the stashed payload) does the same on a mismatch.</li>
 * </ul>
 *
 * Parity contract (law 4): the cached payload IS the vanilla body output for
 * the same instance (verified bit-in-bit online); an empty lever flag never
 * defines this class (byte-vanilla body). The cache is keyed by packet
 * INSTANCE (reference equality — the packet does not override equals), capped
 * at 2048 (mirrors the chunk4 snapshot working set 1:1) with evict-half.
 *
 * Delivery discipline: defined ALONE into the kernel loader — the source
 * declares ZERO nested classes and ZERO lambdas (plain bytecode only).
 *
 * Grep markers: "cmp444_chunk5", "chunk5 payload-cache first hit",
 * "chunk5 payload selftest", "chunk5 stats".
 */
public final class ChunkPacketEncodeOps {

    private ChunkPacketEncodeOps() {}

    /** Payload-cache bound; overflow evicts HALF (chunk_parse pattern). */
    static final int CACHE_CAP = 2048;

    /** Fresh second-encodes compared bit-in-bit (online selftest window). */
    static final int SELFTEST_PACKETS = 2;

    /** Marker/log prefix (matches the rust ARM marker). */
    static final String PFX = "[crussty-plugin] cmp444_chunk5:";

    /**
     * Chunk-pipeline R7 carrier union (law 7): this encode-side plane rides
     * the cmp444_chunk5 carrier ON TOP of cmp437_chunk4 (send snapshot) ⊕
     * cmp435_chunk3 (parse composite). Kept in the constant pool for the
     * raw-byte blob-sync gate (check_blobs_sync.sh) — x93 lesson.
     */
    static final String CARRIER_UNION_435 = "cmp435_chunk3";

    static final String CARRIER_UNION_437 = "cmp437_chunk4";
    /** TASK-450-C union carrier (STRICT-OR; raw-cp marker for the
     * check_blobs_sync gate). */
    static final String CARRIER_UNION_450 = "cmp450_chunk";
    /** TASK-452-C mega-composite (senseins ⊕ chunk union; STRICT-OR; raw-cp
     * marker for the check_blobs_sync gate). */
    static final String CARRIER_UNION_452 = "cmp452_mega";

    // ------------------------------------------------------------------
    // P27 SERIALIZATION SCRATCH-ARENA — SCAFFOLD STUB (ID-P27, TASK-459-63,
    // law-11 WILD; research: RESEARCH-459-P27.md; rust engine model:
    // src/scratch_arena.rs).
    //
    // Target: the MISS path ABOVE (encodePayload) allocates a fresh
    // Unpooled.buffer(256) (netty doubling cascade) + a fresh new byte[len]
    // per MISS encode; P27 replaces BOTH with a per-thread arena pool
    // (section buffers + heightmap-NBT scratch) under three invariants:
    //   1. FULL OVERWRITE per session — every handed-off byte in
    //      [0, write_len) was written by THIS encode (never a partial
    //      append over the previous lifetime's bytes);
    //   2. LENGTH CONTROL — write_len + capacity gate + expected-length
    //      seal from the re-encode selftest (no stale-slot tails);
    //   3. ONE PROTECTIVE COPY ON HANDOFF — the codec/channel may hold the
    //      payload reference beyond the encode, so the slot never leaks
    //      (foojay canon: reused objects must be copied before persisting).
    // Parity (law 4): the arena changes ONLY the allocator, never the
    // encoding — "байты те же"; the re-encode selftest gains the
    // vanilla-vs-arena bit-in-bit meaning on activation.
    //
    // SCAFFOLD DISCIPLINE (x93 lesson): the shipped blob
    // chunksend/build/net/minecraft/server/network/ChunkPacketEncodeOps.class
    // is deliberately NOT rebuilt in this commit — the cmp444_chunk5 plane
    // stays bit-in-bit as certified (hot methods untouched below). Any P27
    // activation cycle MUST rebuild the blob (--release 21, kernel-jar cp,
    // javap flat==nested) AND re-run check_blobs_sync.sh BEFORE arming, and
    // MUST follow the NCDFE canon (early define before first retransform,
    // pattern d73758a3/5ecd841a; NCDFE=0 до вердикта; grep AIOOBE=0).
    //
    // Grep markers: "P27", "cmp459_scratcharena".
    // ------------------------------------------------------------------

    /** P27 scratch-arena round id (raw-cp marker for the next cycle's
     * check_blobs_sync gate; NOT part of the shipped cmp444_chunk5 blob). */
    static final String CARRIER_UNION_P27 = "cmp459_scratcharena";

    /** Hard arena-slot capacity (light-chunk bound; requests beyond this are
     * served vanilla — fail-closed, matches ScratchArena::SLOT_CAP). */
    static final int P27_SLOT_CAP = 1 << 20;

    /** Bounded hot-slot pool per thread (region_threads=4 scene; matches
     * ScratchArena::SLOTS_PER_THREAD). Overflow -> vanilla path. */
    static final int P27_SLOTS_PER_THREAD = 8;

    /**
     * P27 length-control seal (SCAFFOLD — dormant: no hot-path caller).
     * Returns the sealed slot length or -1 when the request must fall back
     * to the vanilla allocator: len &lt; 0, or len beyond the hard slot cap
     * (grow protocol = a NEW slot, never an in-place resize; requests above
     * P27_SLOT_CAP never pool). The full-overwrite discipline lives in the
     * rust engine model (src/scratch_arena.rs) and lands here verbatim at
     * activation: reset before every session, seal against the fresh
     * baseline length, one protective copy on handoff.
     *
     * @param requestedLen encode payload length (fresh baseline)
     * @return sealed slot length, or -1 = vanilla fallback (fail-closed)
     */
    static int p27ArenaSlotLen(int requestedLen) {
        if (requestedLen < 0 || requestedLen > P27_SLOT_CAP) {
            return -1;
        }
        return requestedLen;
    }

    /** Packet instance -> its vanilla write() payload (reference keys). */
    private static final ConcurrentHashMap<ClientboundLevelChunkWithLightPacket, byte[]> PAYLOAD =
            new ConcurrentHashMap<>();

    private static long writes = 0;
    private static long encodes = 0;
    private static long hits = 0;
    private static long evictions = 0;
    private static int selftestLeft = SELFTEST_PACKETS;
    private static volatile boolean disabled = false;
    private static boolean firstHitLogged = false;

    // Netty surface resolved ONCE reflectively (the kernel jar does not expose
    // netty classes to javac; the runtime loader does). Any resolution defect
    // poisons NETTY_OK and selfTest() returns false -> hook stays dormant.
    private static final boolean NETTY_OK;
    private static final Constructor<?> RFB_CTOR;          // RegistryFriendlyByteBuf(ByteBuf, RegistryAccess)
    private static final Method UNPOOLED_BUFFER;           // Unpooled.buffer(int)
    private static final Method BUF_READABLE;              // ByteBuf.readableBytes()
    private static final Method BUF_READ_BYTES;            // ByteBuf.readBytes(byte[])
    private static final Method BUF_RELEASE;               // ByteBuf.release()
    private static final Method BUF_WRITE_BYTES;           // ByteBuf.writeBytes(byte[])

    static {
        boolean ok = false;
        Constructor<?> ctor = null;
        Method buffer = null;
        Method readable = null;
        Method readBytes = null;
        Method release = null;
        Method writeBytes = null;
        try {
            Class<?> byteBuf = Class.forName("io.netty.buffer.ByteBuf");
            Class<?> unpooled = Class.forName("io.netty.buffer.Unpooled");
            ctor = RegistryFriendlyByteBuf.class.getConstructor(
                    byteBuf, net.minecraft.core.RegistryAccess.class);
            buffer = unpooled.getMethod("buffer", int.class);
            readable = byteBuf.getMethod("readableBytes");
            readBytes = byteBuf.getMethod("readBytes", byte[].class);
            release = byteBuf.getMethod("release");
            writeBytes = byteBuf.getMethod("writeBytes", byte[].class);
            ok = true;
        } catch (Throwable t) {
            System.out.println(PFX + " netty resolution FAIL: " + t);
        }
        NETTY_OK = ok;
        RFB_CTOR = ctor;
        UNPOOLED_BUFFER = buffer;
        BUF_READABLE = readable;
        BUF_READ_BYTES = readBytes;
        BUF_RELEASE = release;
        BUF_WRITE_BYTES = writeBytes;
    }

    /**
     * Redirected body of the private
     * {@code ClientboundLevelChunkWithLightPacket.write(RegistryFriendlyByteBuf)}
     * — the receiver-prepended static descriptor is the EXACT canonical
     * redirect contract.
     */
    public static void write(ClientboundLevelChunkWithLightPacket packet,
                             RegistryFriendlyByteBuf buf) {
        writes++;
        if (disabled) {
            vanillaEncode(packet, buf);
            return;
        }
        byte[] payload = PAYLOAD.get(packet);
        if (payload != null) {
            hits++;
            if (!firstHitLogged) {
                firstHitLogged = true;
                System.out.println(PFX + " chunk5 payload-cache first hit"
                        + " (encode-once replay-per-player, union=" + CARRIER_UNION_435
                        + "/" + CARRIER_UNION_437 + ")");
            }
            if (!replay(buf, payload)) {
                vanillaEncode(packet, buf); // fail-closed fallback
            }
            statsLine();
            return;
        }
        byte[] fresh;
        try {
            fresh = encodePayload(packet, buf);
        } catch (Throwable t) {
            disabled = true;
            PAYLOAD.clear();
            System.out.println(PFX + " encode fail-closed: " + t);
            vanillaEncode(packet, buf);
            return;
        }
        encodes++;
        if (selftestLeft > 0) {
            selftestLeft--;
            onlineSelftest(packet, fresh, buf);
            if (disabled) {
                vanillaEncode(packet, buf);
                return;
            }
        }
        if (PAYLOAD.size() >= CACHE_CAP) {
            // evict-half: keep the working set warm instead of a full wipe.
            int seen = 0;
            Iterator<Map.Entry<ClientboundLevelChunkWithLightPacket, byte[]>> it =
                    PAYLOAD.entrySet().iterator();
            while (it.hasNext()) {
                it.next();
                if ((seen & 1) == 0) {
                    it.remove();
                    evictions++;
                }
                seen++;
            }
        }
        PAYLOAD.put(packet, fresh);
        if (!replay(buf, fresh)) {
            vanillaEncode(packet, buf); // fail-closed fallback
        }
        statsLine();
    }

    /** Raw ByteBuf.writeBytes(byte[]) replay; false = fail-closed latch set. */
    private static boolean replay(RegistryFriendlyByteBuf buf, byte[] payload) {
        try {
            BUF_WRITE_BYTES.invoke(buf, (Object) payload);
            return true;
        } catch (Throwable t) {
            disabled = true;
            PAYLOAD.clear();
            System.out.println(PFX + " replay fail-closed: " + t);
            return false;
        }
    }

    private static void statsLine() {
        if ((writes & 1023L) == 0) {
            System.out.println(PFX + " chunk5 stats writes=" + writes + " encodes=" + encodes
                    + " hits=" + hits + " cached=" + PAYLOAD.size()
                    + " cap=" + CACHE_CAP + " union=" + CARRIER_UNION_437);
        }
    }

    /**
     * The EXACT vanilla body semantics (javap-verified 2026-09-24):
     * writeInt(x); writeInt(z); chunkData.write(buf); lightData.write(buf) —
     * via the public getters so the bridge holds no field offsets.
     */
    private static void vanillaEncode(ClientboundLevelChunkWithLightPacket packet,
                                      RegistryFriendlyByteBuf buf) {
        buf.writeInt(packet.getX());
        buf.writeInt(packet.getZ());
        packet.getChunkData().write(buf);
        packet.getLightData().write(buf);
    }

    /** One vanilla-equivalent encode captured into a standalone byte[]. */
    private static byte[] encodePayload(ClientboundLevelChunkWithLightPacket packet,
                                        RegistryFriendlyByteBuf origin) throws Exception {
        Object scratchBuf = UNPOOLED_BUFFER.invoke(null, Integer.valueOf(256));
        try {
            RegistryFriendlyByteBuf scratch =
                    (RegistryFriendlyByteBuf) RFB_CTOR.newInstance(
                            scratchBuf, origin.registryAccess());
            vanillaEncode(packet, scratch);
            int len = ((Integer) BUF_READABLE.invoke(scratchBuf)).intValue();
            byte[] out = new byte[len];
            BUF_READ_BYTES.invoke(scratchBuf, (Object) out);
            return out;
        } finally {
            BUF_RELEASE.invoke(scratchBuf);
        }
    }

    /**
     * Structural oracle called by the rust activator BEFORE the hook can ever
     * serve a redirect (selfTest==true до ARM; false = hook stays dormant).
     */
    public static boolean selfTest() {
        try {
            if (!NETTY_OK || RFB_CTOR == null) {
                return false;
            }
            ClientboundLevelChunkWithLightPacket.class.getMethod("getX");
            ClientboundLevelChunkWithLightPacket.class.getMethod("getZ");
            ClientboundLevelChunkWithLightPacket.class.getMethod("getChunkData");
            ClientboundLevelChunkWithLightPacket.class.getMethod("getLightData");
            ClientboundLevelChunkWithLightPacket.class.getDeclaredMethod(
                    "write", RegistryFriendlyByteBuf.class);
            ClientboundLevelChunkPacketData.class.getMethod(
                    "write", RegistryFriendlyByteBuf.class);
            ClientboundLightUpdatePacketData.class.getMethod("write", FriendlyByteBuf.class);
            RegistryFriendlyByteBuf.class.getMethod("registryAccess");
            FriendlyByteBuf.class.getMethod("writeInt", int.class);
            return true;
        } catch (Throwable t) {
            System.out.println(PFX + " selfTest structural FAIL: " + t);
            return false;
        }
    }

    /**
     * Online selftest: re-encode the SAME instance a SECOND time into a fresh
     * scratch buffer and compare bit-in-bit against the stashed payload.
     * Same instance, same final fields -> vanilla-vs-vanilla equality.
     * PASS marker = bench effect-marker; FAIL flips DISABLED (fail-closed).
     */
    private static void onlineSelftest(ClientboundLevelChunkWithLightPacket packet,
                                       byte[] first, RegistryFriendlyByteBuf origin) {
        try {
            byte[] second = encodePayload(packet, origin);
            boolean ok = Arrays.equals(first, second);
            if (ok) {
                System.out.println(PFX + " chunk5 payload selftest PASS");
            } else {
                System.out.println(PFX + " chunk5 payload selftest FAIL");
                disabled = true;
                PAYLOAD.clear();
            }
        } catch (Throwable t) {
            System.out.println(PFX + " chunk5 payload selftest FAIL (throwable " + t + ")");
            disabled = true;
            PAYLOAD.clear();
        }
    }

    /** Diagnostics for the boot/absorb greps (never allocates on hot path). */
    public static String stats() {
        return PFX + " writes=" + writes + " encodes=" + encodes + " hits=" + hits
                + " cached=" + PAYLOAD.size() + " cap=" + CACHE_CAP
                + " evicted=" + evictions
                + " selftest=" + (SELFTEST_PACKETS - selftestLeft)
                + " disabled=" + disabled
                + " union=" + CARRIER_UNION_435 + "/" + CARRIER_UNION_437;
    }
}
