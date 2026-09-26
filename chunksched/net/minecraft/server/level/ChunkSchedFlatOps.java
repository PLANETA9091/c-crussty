package net.minecraft.server.level;

/**
 * CHUNK-TICK ELIGIBILITY FLAT bridge — scaffold stub (TASK-459-59, WILD
 * law-11, tick-459; idea ID-P22, lever cmp459_chunksched-mask — STRICT eq,
 * DORMANT by default). Native counterpart: src/chunk_sched_flat.rs.
 *
 * Plane contract (RESEARCH-459-P22.md):
 *
 *   1. COLLECT (java): per tick, the bridge gathers FLAT chunk-tick
 *      eligibility predicates into primitive arrays:
 *        - long[] chunkKeys  — CoordinateUtils.getChunkKey(x, z) of every
 *          chunk in the Moonrise ReferenceList snapshot (K2 feed: the single
 *          moonrise$setFullChunk mutation point keeps this set bit-exact);
 *        - int[]  flags      — per-chunk predicate bits (v1: B5
 *          playerTicking-membership, B6 hasAnyNearbyNarrow area-map bit);
 *        - long[] playerXZ   — packed player chunk columns for the v1
 *          distance gate (fake_players=4 on the soak bench).
 *   2. DECIDE (ONE bulk JNI): nativeEvaluateEligibility(chunkKeys, flags,
 *      playerXZ) -> long[144] eligible bitmask (9216 chunks = 144 longs =
 *      1152 bytes = ONE JNI block, law 6: <=1 crossing per tick total).
 *      Rust performs ONLY decision math; it never touches iteration order.
 *   3. TAIL (java, strict vanilla order): the vanilla consumers
 *      (ServerChunkCache.iterateTickingChunksFaster — Moonrise
 *      chunk_tick_iteration raw-array order; ChunkMap.collectSpawningChunks —
 *      playerTicking raw order) keep their exact vanilla iteration order and
 *      i&7 mid-tick density; the mask is consulted ONLY as a pre-gate:
 *      bit=1 (or unknown) re-checks the vanilla predicate; a bit=0 skip is
 *      legal only for EXACT bits (v1: B5/B6, javap K6/K7). Cross-chunk tick
 *      processing order is parity-critical (MC-310372: INTRA_TICK_DRAIN_ORDER
 *      regressions broke contraptions) — the tail NEVER reorders.
 *
 * Lockstep oracle G2 (wiring stage): flat mask == nested re-computation
 * (ReferenceList + holder status) bit-in-bit every tick; drift > 0 = fail-open
 * disarm latch (nav_plane batchOk pattern).
 *
 * NCDFE canon: this class is define_class'd into the kernel loader in the
 * EARLY arm hook (cplugin_init_impl -> chunk_sched_flat::register()), before
 * any retarget exists; a dormant lever leaves it defined + unused
 * (dormant-invisible, vanilla bit-in-bit).
 *
 * SCAFFOLD STATE: no native binding is registered yet and selfTest() returns
 * false — the plane is fail-closed DORMANT by construction (roar-2 lesson:
 * armState must be grep-able and honest). The wiring stage replaces the
 * placeholder with the real collector/hooks and flips armState to ARMED.
 */
public final class ChunkSchedFlatOps {

    private ChunkSchedFlatOps() {}

    /** Eligibility predicate bits (v1 = B5|B6 only; B0-B4/B7 reserved). */
    public static final int FLAG_PLAYER_TICKING = 1 << 5; // B5 (javap K6)
    public static final int FLAG_NEARBY_NARROW  = 1 << 6; // B6 (javap K7)

    /**
     * Fixed JNI contract (wiring stage binds it via RegisterNatives;
     * ZERO call sites in the scaffold — declaration only, never invoked):
     * given n flat chunk records and the packed player columns, return
     * long[144] with bit i of long[i >> 6] set iff chunk i is eligible under
     * the v1 distance gate (B5 && B6 superset-exact math in Rust).
     * Fallback on any native defect: all-ones mask (fail-open to the java
     * tail = exact vanilla behavior for every chunk).
     */
    private static native long[] nativeEvaluateEligibility(
            long[] chunkKeys, int[] flags, long[] playerXZ, int playerCount);

    /**
     * Structural oracle of the scaffold: true ONLY when the plane is wired
     * (blob define + selfTest + native binding). The rust activator refuses
     * to arm unless this returns true BEFORE any hook exists (probe-then-
     * patch guarantee, inside_bitmask precedent).
     */
    public static boolean selfTest() {
        return false; // scaffold: fail-closed dormant
    }

    /** Grep-able arm-state (roar-2 lesson: ARM={} silent dormancy forbidden). */
    public static String armState() {
        return "DORMANT_SCAFFOLD"; // wiring stage -> "ARMED"
    }
}
