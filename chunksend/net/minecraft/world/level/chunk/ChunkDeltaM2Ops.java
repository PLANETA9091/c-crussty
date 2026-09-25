package net.minecraft.world.level.chunk;

import java.lang.reflect.Method;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.protocol.game.ClientboundLevelChunkPacketData;

/**
 * PER-SECTION CHUNK DELTA PLANE — java side of the RUST bulk-JNI delta
 * engine, DORMANT SCAFFOLD (TASK-459-79, lever {@code cmp459_m2chdelta};
 * law-11 revisit of RESEARCH-458-M ID-M2; card RESEARCH-459-M2.md).
 *
 * <p>Flat-class canon (chunk5/chunk_send discipline): ZERO nested classes,
 * ZERO lambdas, major 65 ({@code --release 21}); the class is defined from
 * rust into the kernel loader as a single flat classfile.</p>
 *
 * <h2>NCDFE-canon — define BEFORE first touch</h2>
 * The rust activator MUST {@code define_class("net/minecraft/world/level/chunk/
 * ChunkDeltaM2Ops")} into the kernel loader BEFORE any kernel code can resolve
 * the ops name and BEFORE the body redirect is served (arm-after-define;
 * selfTest()==true is a precondition of ARM). A define defect leaves the
 * plane dormant forever — the vanilla 2-arg delegate stays byte-identical
 * (fail-closed, lesson-408 discipline: no ARM marker without a passing
 * selfTest oracle).
 *
 * <h2>Law-4 parity by construction</h2>
 * The rust side of the delta engine NEVER re-encodes a section: the only
 * writer of payload bytes is the vanilla
 * {@code ClientboundLevelChunkPacketData.extractChunkData(FriendlyByteBuf,
 * LevelChunk, ChunkPacketInfo)} section loop
 * ({@code LevelChunkSection.write} — javap round-396-a, public). HIT replay
 * writes back exactly the bytes the vanilla encoder wrote (readback capture);
 * the serve-gate is the chunk4-certified {@code LevelChunk.isUnsaved()}
 * oracle: any block/block-entity change marks the chunk BEFORE the next send
 * probes the delta cache, so a changed section can never be replayed stale.
 * Online selftest (step-2 gate G2): the first 2 captured constructions are
 * re-encoded by vanilla a SECOND time into a scratch buffer and compared
 * section-by-section bit-in-bit; any mismatch flips a DISABLED latch and the
 * redirect body becomes a pure vanilla passthrough forever.
 *
 * <h2>Bulk-JNI discipline (law 6)</h2>
 * Exactly ONE rust→java bulk crossing per tick carries the batch of changed
 * section payloads (delta), never per-chunk/per-send crossings.
 *
 * <p>DORMANT in this scaffold: {@link #extractChunkData(FriendlyByteBuf,
 * LevelChunk)} below is the byte-exact vanilla passthrough — it is the
 * future static-&gt;static redirect TARGET (sites:1, exact descriptor
 * {@code (Lnet/minecraft/network/FriendlyByteBuf;Lnet/minecraft/world/level/
 * chunk/LevelChunk;)V}), but no redirect is installed while the lever is
 * dormant, so kernel behavior is vanilla bit-in-bit.</p>
 */
public final class ChunkDeltaM2Ops {

    private ChunkDeltaM2Ops() {
    }

    /**
     * Structural selftest oracle — MUST return {@code true} before the rust
     * side arms anything (selfTest до ARM). Pure reflection census of the
     * javap ground truth; no JVM state is touched, no exceptions escape.
     *
     * @return {@code true} iff all pinned kernel shapes are present:
     *         (1) the 2-arg {@code extractChunkData} static delegate,
     *         (2) the 3-arg {@code extractChunkData} static body,
     *         (3) public {@code LevelChunkSection.write(FriendlyByteBuf,
     *         ChunkPacketInfo, int)},
     *         (4) public {@code LevelChunk.isUnsaved()}.
     */
    public static boolean selfTest() {
        try {
            Class<?> packetData = Class.forName(
                "net.minecraft.network.protocol.game.ClientboundLevelChunkPacketData");
            Class<?> section = Class.forName(
                "net.minecraft.world.level.chunk.LevelChunkSection");
            Class<?> chunk = LevelChunk.class;
            Class<?> buf = FriendlyByteBuf.class;
            Class<?> packetInfo = Class.forName(
                "io.papermc.paper.antixray.ChunkPacketInfo");

            Method twoArg = packetData.getDeclaredMethod("extractChunkData",
                buf, chunk);
            if (!twoArg.getReturnType().equals(void.class)) {
                return false;
            }
            // 3-arg body: (FriendlyByteBuf, LevelChunk, ChunkPacketInfo)void
            packetData.getDeclaredMethod("extractChunkData", buf, chunk, packetInfo);
            // per-section vanilla writer: (FriendlyByteBuf, ChunkPacketInfo, int)void
            Method sectionWrite = section.getDeclaredMethod("write",
                buf, packetInfo, int.class);
            if (!sectionWrite.getReturnType().equals(void.class)) {
                return false;
            }
            // chunk4-certified dirty oracle: ()boolean
            Method isUnsaved = chunk.getDeclaredMethod("isUnsaved");
            if (!isUnsaved.getReturnType().equals(boolean.class)) {
                return false;
            }
            return true;
        } catch (Throwable t) {
            // fail-closed: any defect = oracle false = the plane never arms
            return false;
        }
    }

    /**
     * Byte-exact vanilla passthrough — the future redirect TARGET body for
     * the vanilla 2-arg delegate (sites:1). DORMANT: the body IS the vanilla
     * delegation, so even after the step-2 redirect installs, an un-armed or
     * DISABLED-latch state is indistinguishable from vanilla (the redirect
     * only ever swaps this body for the delta-replay body AFTER selfTest
     * passes; latch failure restores this body).
     *
     * <p>Exact descriptor contract (javap round-396-a, 2026-09-26):
     * {@code (Lnet/minecraft/network/FriendlyByteBuf;Lnet/minecraft/world/
     * level/chunk/LevelChunk;)V}.</p>
     *
     * @param buffer the pre-calibrated packet buffer (vanilla ctor invariant:
     *               {@code writerIndex() == capacity()} at end of extract)
     * @param chunk  the chunk being serialized
     */
    public static void extractChunkData(FriendlyByteBuf buffer, LevelChunk chunk) {
        ClientboundLevelChunkPacketData.extractChunkData(buffer, chunk);
    }

    /**
     * Lever id marker — single source of the ARM/EFFECT grep token
     * ("cmp459_m2chdelta") shared with the rust side; keeps the dormant
     * class greppable in blob audits.
     */
    public static String leverId() {
        return "cmp459_m2chdelta";
    }
}
