package net.minecraft.world.entity.item;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import net.minecraft.util.Mth;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.MoverType;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * ITEMS-STAGGER bridge (ROUND-396 vector B, TASK-396-B).
 *
 * Architectural replacement of the item-entity per-tick heavy-check SCHEDULE:
 * instead of running every heavy check every tick, each check runs on a
 * phase-shifted cadence keyed by entity id:
 *
 *   due(e, unit) = ((e.tickCount / unit) + e.getId()) % N == 0
 *
 * where `unit` is the caller's vanilla grid size (40 merge / 4 move / 1
 * noCollision), so the phase is always computed on exact divisions.
 *
 * Gates (retargeted 1:1 call sites inside ItemEntity.tick — identical stack
 * shapes, zero bytecode-length change, no StackMapTable edits):
 *
 *   1. mergeWithNeighbours()V      -> mergeWithNeighbours(ItemEntity)V
 *      stationary case: cadence 40 -> 40*N (vanilla already throttles to 40);
 *      vanilla moved-case (block changed this tick) is passed through 1:1;
 *      first scheduled tick (spawn, tickCount<=1) is always due.
 *   2. Entity.move(MoverType,Vec3)V -> move(Entity,MoverType,Vec3)V
 *      only the vanilla settled-slow branch (recomputed exactly:
 *      onGround && horizontalDistanceSqr < 1e-5) gets 4 -> 4*N;
 *      moving/falling items are NEVER extra-skipped.
 *   3. Level.noCollision(Entity,AABB)Z -> noCollision(Level,Entity,AABB)Z
 *      freshness N: non-due ticks answer the safe default `false`
 *      ("not squeezed" => normal collision physics); squeezed items
 *      re-evaluate within <= N ticks (documented deviation).
 *
 * Fail-dominance: the static initializer throws if the private
 * mergeWithNeighbours MethodHandle cannot be built — the wiring worker then
 * aborts BEFORE retransforming ItemEntity, so the class stays vanilla.
 * The class is stateless after init (config read-only) — safe under
 * region-thread parallel ticking; no collections, no memoization.
 *
 * Flag gate lives on the rust side (CRUSSTY_LEVER_FLAG == "items_stagger",
 * CRUSSTY_LEVER_ARG = N, default 4). Dormant lever = vanilla by construction
 * (no retarget ever installed).
 */
public final class ItemStaggerOps {

    /** Phase multiplier (CRUSSTY_LEVER_ARG, default 4). */
    public static final int N;

    private static final MethodHandle MERGE_VANILLA;

    static {
        int n = 4;
        try {
            String s = System.getenv("CRUSSTY_LEVER_ARG");
            if (s != null && !s.trim().isEmpty()) {
                n = Math.max(1, Integer.parseInt(s.trim()));
            }
        } catch (Throwable t) {
            // default stays 4
        }
        N = n;
        MethodHandle mh = null;
        try {
            Class<?> k = Class.forName("net.minecraft.world.entity.item.ItemEntity");
            mh = MethodHandles.privateLookupIn(k, MethodHandles.lookup())
                    .findVirtual(k, "mergeWithNeighbours", MethodType.methodType(void.class));
        } catch (Throwable t) {
            t.printStackTrace();
            mh = null;
        }
        if (mh == null) {
            // Fail-dominant: without the vanilla-body handle the retarget would
            // REMOVE merges entirely. Throwing fails the static init; the
            // wiring worker observes the failed signature() probe and aborts
            // before any retransform — ItemEntity stays 100% vanilla.
            throw new IllegalStateException("items_stagger: cannot build private mergeWithNeighbours handle — lever must not arm");
        }
        MERGE_VANILLA = mh;
        System.out.println("[crussty-items-stagger] ops initialized N=" + N + " mergeHandle=ok");
    }

    private ItemStaggerOps() {
    }

    /** Init probe: forces the static initializer; rust aborts the lever on any failure. */
    public static String signature() {
        return "items_stagger N=" + N + " mergeHandle=ok";
    }

    private static boolean movedBlockNow(ItemEntity e) {
        // EXACT vanilla predicate (ItemEntity.tick): block position changed this tick.
        return Mth.floor(e.xo) != Mth.floor(e.getX())
                || Mth.floor(e.yo) != Mth.floor(e.getY())
                || Mth.floor(e.zo) != Mth.floor(e.getZ());
    }

    // ------------------------------------------------------------------
    // Gate 1: merge (the vector's letter). Vanilla calls this static ONLY
    // on its own due ticks (tickCount % (moved?2:40) == 0 && !client &&
    // isMergable()). We pass movers straight through and additionally
    // schedule the stationary case by id-phase: 40 -> 40*N.
    // ------------------------------------------------------------------
    public static void mergeWithNeighbours(ItemEntity e) {
        if (e.tickCount > 1 && !movedBlockNow(e) && ((e.tickCount / 40) + e.getId()) % N != 0) {
            return;
        }
        try {
            MERGE_VANILLA.invokeExact(e);
        } catch (Throwable t) {
            throw new RuntimeException("items_stagger: vanilla merge dispatch failed", t);
        }
    }

    // ------------------------------------------------------------------
    // Gate 2: settled-item physics move extension. This static is reached
    // only through the single retargeted call site in ItemEntity.tick;
    // vanilla's own gate (skip 3 of 4 ticks when settled-slow) still runs
    // in front of it. We recompute the settled predicate exactly and only
    // then extend the cadence 4 -> 4*N by id-phase.
    // ------------------------------------------------------------------
    public static void move(Entity self, MoverType type, Vec3 delta) {
        if (self instanceof ItemEntity
                && self.tickCount > 1
                && self.onGround()
                && self.getDeltaMovement().horizontalDistanceSqr() < 1.0E-5
                && (((self.tickCount + self.getId()) / 4) + self.getId()) % N != 0) {
            return;
        }
        self.move(type, delta);
    }

    // ------------------------------------------------------------------
    // Gate 3: noCollision freshness. Non-due ticks answer the safe default
    // `false` (not squeezed => vanilla collision physics); due ticks run the
    // real query. Exact task formula: (tickCount + id) % N.
    // ------------------------------------------------------------------
    public static boolean noCollision(Level recv, Entity self, AABB box) {
        if (self instanceof ItemEntity
                && self.tickCount > 1
                && (self.tickCount + self.getId()) % N != 0) {
            return false;
        }
        return recv.noCollision(self, box);
    }
}
