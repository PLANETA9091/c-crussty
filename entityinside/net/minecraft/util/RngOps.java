package net.minecraft.util;

import java.util.UUID;

/**
 * RngOps (S7-158d, TASK-298 — ARCH-ATTACK lever #7 hardening: region-threaded
 * entity ticking, UUID-seeding race fix).
 *
 * Live incident 35363758352 (leg #2): ONE "UUID duplicate" WARN (base: 0) —
 * a worker-spawned Arrow (skeleton shot) aliased its UUID with a Rotten Flesh
 * item drop ~900 blocks away (different region), the second add was rejected
 * by EntityLookup ("can't add") and the spawned entity was LOST — a real
 * behavioral deviation vs the single-threaded base.
 *
 * Root cause (javap, kernel e2992d63): Purpur's PurpurWorldConfig
 * {@code entitySharedRandom} DEFAULTS TO TRUE (iconst_1 in the config
 * defaults) — so {@code Entity.ctor} routes {@code this.random} to the
 * shared static {@code Entity.SHARED_RANDOM}, which is a moonrise
 * {@code ThreadUnsafeRandom}. Entity construction then draws the UUID via
 * {@code Mth.createInsecureUUID(this.random)} — two consecutive nextLong()
 * calls with NO synchronization. Under REGION-THREADS two workers can
 * construct entities concurrently: both read the same generator state, both
 * advance it identically, both derive the SAME UUID pair.
 *
 * Fix: retarget of the ONLY {@code Mth.createInsecureUUID(RandomSource)}
 * call site in the kernel (the Entity ctor; census: ServerBossEvent uses the
 * no-arg variant over Mth.RANDOM which is RandomSource.createThreadSafe() and
 * needs no fix) to this bridge — the vanilla two-draw UUIDv4 computation
 * BIT-FOR-BIT (javap: most = nextLong() & -61441 | 16384, least =
 * nextLong() & 4611686018427387903 | Long.MIN_VALUE) serialized per-source
 * via monitor on the RandomSource itself. Cost: one uncontended monitor
 * enter/exit per entity CONSTRUCTION (spawns are rare vs 150k ticking
 * entities; per-tick AI draws on the shared source keep their pre-existing
 * benign lost-update race — no behavioral change vs leg #2 there).
 *
 * Strict Retargeted{1}, fail-closed (rust region_threads v3). Defined into
 * the KERNEL loader.
 */
public final class RngOps {

    private RngOps() {}

    /** Vanilla Mth.createInsecureUUID(RandomSource) body, serialized. */
    public static UUID createInsecureUUID(RandomSource random) {
        synchronized (random) {
            long l = random.nextLong() & -61441L | 16384L;
            long m = random.nextLong() & 4611686018427387903L | Long.MIN_VALUE;
            return new UUID(l, m);
        }
    }
}
