package net.minecraft.world.entity;

import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.Iterator;
import java.util.List;
import java.util.Objects;
import java.util.concurrent.ConcurrentHashMap;
import java.util.function.Predicate;

import net.minecraft.tags.FluidTags;
import net.minecraft.util.Mth;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.HitResult;
import net.minecraft.world.phys.Vec3;
import net.minecraft.world.phys.shapes.CollisionContext;

import org.bukkit.craftbukkit.event.CraftEventFactory;
import org.bukkit.event.entity.EntityRemoveEvent;
import org.bukkit.event.entity.ItemDespawnEvent;

/**
 * ItemStaggerOps — items_stagger lever (TASK-395 mega-round, agent B).
 *
 * Whole-body replacement of {@link net.minecraft.world.entity.item.ItemEntity#tick()}
 * (served by src/items_stagger.rs via cplug_sdk::asm::replace_body, the
 * FluidPushGuardHook / TASK-80 delivery pattern) plus a candidate-cache for
 * the merge scan. The replaced body is a FAITHFUL mirror of the kernel's
 * ItemEntity.tick + Entity.tick (despawnTime gate + baseTick) bytecode
 * (Purpur 1.21.10, javap-verified field by field), with exactly three
 * schedule deltas and no state memoization:
 *
 *   1. Rest stagger interval: vanilla (tickCount+id)%4 move skip for
 *      onGround items with horizontalDistanceSqr <= 1e-5 → lever interval 8
 *      (same gate shape, longer period, deterministic id offset — kept).
 *   2. On rest-skip ticks the lever ALSO skips two vanilla per-tick checks
 *      that upstream leaves ungated for resting items:
 *        a. noPhysics = !noCollision(bb.deflate(1e-7)) + moveTowardsClosestSpace,
 *        b. the second fluid scan hasImpulse |= updateInWaterStateAndDoFluidPushing()
 *           (baseTick already scanned the same fluid state this very tick).
 *      Deviation: a resting item reacts to block/fluid changes under it at
 *      its next move-tick — ≤ MAX_STAGGER_DELAY ticks (8) vs vanilla 0.
 *   3. Merge schedule: vanilla tickCount % (posChanged ? 2 : 40) → lever
 *      (tickCount+id) % (posChanged ? 8 : 40) + candidate cache (last valid
 *      merge partner validated without the AABB section scan).
 *      Deviation for moving items: ≤ 8 ticks (vanilla 2); for resting items
 *      the 40-tick vanilla frequency is kept, only de-synchronized by id.
 *
 * Gate: active ONLY when System.getenv("CRUSSTY_LEVER_FLAG").equals(
 * "items_stagger") — an empty/other flag makes this body byte-for-byte
 * vanilla (intervals 4/2/40, no skip, no cache) AND the Rust side never
 * installs the hook at all. CRUSSTY_LEVER_ARG "1" = preset 1 (8/8/cache on);
 * a numeric arg 2..8 selects the interval (hard-capped at 8 by the ≤8-tick
 * deviation contract), any other value falls back to preset 1.
 *
 * All static state is thread-safe: ItemEntity.tick runs on up to
 * region_threads worker buckets concurrently (RegionTickOps, bank v4).
 * Pair/pickup/despawn results are vanilla-equal; only their per-tick
 * scheduling is staggered as documented above (LEVER.md).
 */
public final class ItemStaggerOps {

    private ItemStaggerOps() {}

    // ------------------------------------------------------------------
    // Lever gates (env, read once at class init in the kernel loader)
    // ------------------------------------------------------------------
    public static final boolean LEVER_ON;
    public static final int MOVE_INTERVAL;
    public static final int MERGE_INTERVAL;
    public static final boolean CANDIDATE_CACHE;
    public static final int MAX_STAGGER_DELAY;

    static {
        boolean on = false;
        int interval = 8;
        try {
            String flag = System.getenv("CRUSSTY_LEVER_FLAG");
            if ("items_stagger".equals(flag == null ? "" : flag.trim())) {
                on = true;
                String raw = System.getenv("CRUSSTY_LEVER_ARG");
                int arg = 1;
                if (raw != null) {
                    try { arg = Integer.parseInt(raw.trim()); } catch (Throwable ignored) { arg = 1; }
                }
                // arg semantics: 1 (or unparsable) = preset 1 → 8; 2..8 → that interval.
                interval = (arg >= 2 && arg <= 8) ? arg : 8;
            }
        } catch (Throwable t) {
            on = false; // never let the lever break the tick path
        }
        LEVER_ON = on;
        MOVE_INTERVAL = on ? interval : 4;
        MERGE_INTERVAL = on ? interval : 2;
        CANDIDATE_CACHE = on;
        MAX_STAGGER_DELAY = interval;
    }

    // ------------------------------------------------------------------
    // Private kernel fields (read-only) via VarHandle — the bridge is
    // defined into the kernel loader inside net.minecraft.world.entity, so
    // privateLookupIn succeeds and the handle is JIT-inlinable (no
    // reflection alloc on the hot path).
    // ------------------------------------------------------------------
    private static final VarHandle F_DESPAWN_RATE;   // ItemEntity.despawnRate (int)
    private static final VarHandle F_DESPAWN_TIME;   // Entity.despawnTime (final int)
    static {
        VarHandle rate = null;
        VarHandle time = null;
        try {
            MethodHandles.Lookup look = MethodHandles.privateLookupIn(
                net.minecraft.world.entity.item.ItemEntity.class, MethodHandles.lookup());
            rate = look.findVarHandle(net.minecraft.world.entity.item.ItemEntity.class, "despawnRate", int.class);
            time = look.findVarHandle(Entity.class, "despawnTime", int.class);
        } catch (Throwable t) {
            rate = null;
            time = null;
        }
        F_DESPAWN_RATE = rate;
        F_DESPAWN_TIME = time;
    }

    /** Reflective self-test — the Rust activator installs the body patch
     * ONLY if this returns true (else the hook stays dormant = vanilla). */
    public static boolean selfTest() {
        return F_DESPAWN_RATE != null && F_DESPAWN_TIME != null;
    }

    // ------------------------------------------------------------------
    // Candidate cache: self entity id → last valid merge partner (weak).
    // Invalidation: partner dead/removed, level changed, left the merge
    // AABB (position/section change) — then the vanilla scan repopulates.
    // Bounded: stale sweep when size exceeds SOFT_MAX.
    // ------------------------------------------------------------------
    private static final int SOFT_MAX = 1 << 14;

    private static final class Cand extends java.lang.ref.WeakReference<net.minecraft.world.entity.item.ItemEntity> {
        Cand(net.minecraft.world.entity.item.ItemEntity referent) { super(referent); }
    }

    private static final ConcurrentHashMap<Integer, Cand> CANDIDATES = new ConcurrentHashMap<>();

    // ------------------------------------------------------------------
    // Whole-body replacement of ItemEntity.tick() — faithful mirror of the
    // Purpur 1.21.10 bytecode with the documented schedule deltas.
    // ------------------------------------------------------------------
    public static void tick(net.minecraft.world.entity.item.ItemEntity self) {
        if (self.getItem().isEmpty()) {
            self.discard(EntityRemoveEvent.Cause.DESPAWN);
            return;
        }

        // --- super.tick() == Entity.tick(): despawnTime gate + baseTick ---
        int despawnTime = (int) F_DESPAWN_TIME.get(self);
        if (despawnTime >= 0 && self.totalEntityAge >= despawnTime) {
            self.discard(EntityRemoveEvent.Cause.DESPAWN);
            return;
        }
        self.baseTick();

        if (self.pickupDelay > 0 && self.pickupDelay != 32767) {
            self.pickupDelay--;
        }

        self.xo = self.getX();
        self.yo = self.getY();
        self.zo = self.getZ();
        Vec3 originalDelta = self.getDeltaMovement();
        if (self.isInWater() && self.getFluidHeight(FluidTags.WATER) > (double) 0.1F) {
            setFluidMovement(self, (double) 0.99F);
        } else if (self.isInLava() && self.getFluidHeight(FluidTags.LAVA) > (double) 0.1F) {
            setFluidMovement(self, (double) 0.95F);
        } else {
            self.applyGravity();
        }

        // Vanilla rest predicate (offsets 226..258 of ItemEntity.tick):
        // resting = onGround && delta.horizontalDistanceSqr() <= 1e-5.
        // Vanilla skips the move-block on (tickCount+id)%4 != 0 for resting
        // items; the lever stretches the period to MOVE_INTERVAL (8).
        boolean resting = self.onGround()
            && self.getDeltaMovement().horizontalDistanceSqr() <= (double) 1.0E-5F;
        boolean skipMove = resting && (self.tickCount + self.getId()) % MOVE_INTERVAL != 0;

        if (self.level().isClientSide()) {
            self.noPhysics = false;
        } else if (LEVER_ON && skipMove) {
            // Lever schedule delta #2a: resting item — noPhysics recheck and
            // moveTowardsClosestSpace are deferred to the next move-tick
            // (position/world state unchanged since the last check).
        } else {
            self.noPhysics = !self.level().noCollision(self, self.getBoundingBox().deflate(1.0E-7));
            if (self.noPhysics) {
                AABB bb = self.getBoundingBox();
                self.moveTowardsClosestSpace(self.getX(), (bb.minY + bb.maxY) / (double) 2.0F, self.getZ());
            }
        }

        if (!skipMove) {
            self.move(MoverType.SELF, self.getDeltaMovement());
            self.applyEffectsFromBlocks();
            float friction = (float) 0.98F;
            if (self.frictionState == net.kyori.adventure.util.TriState.FALSE) {
                friction = 1.0F;
            } else if (self.onGround()) {
                friction = self.level().getBlockState(self.getBlockPosBelowThatAffectsMyMovement())
                    .getBlock().getFriction() * (float) 0.98F;
            }
            self.setDeltaMovement(self.getDeltaMovement().multiply((double) friction, 0.98, (double) friction));
            if (self.onGround()) {
                Vec3 vec3 = self.getDeltaMovement();
                if (vec3.y < 0.0) {
                    self.setDeltaMovement(vec3.multiply(1.0, (double) -0.5F, 1.0));
                }
            }
        }

        boolean posChanged = Mth.floor(self.xo) != Mth.floor(self.getX())
            || Mth.floor(self.yo) != Mth.floor(self.getY())
            || Mth.floor(self.zo) != Mth.floor(self.getZ());

        // Merge schedule: vanilla tickCount % (posChanged ? 2 : 40); lever:
        // id-offset (de-sync of the vanilla global-burst) + movers interval 8.
        // Vanilla gate shape (offsets 444..473): mod==0 && !isClientSide && isMergable.
        int mergeMod = posChanged ? MERGE_INTERVAL : 40;
        if ((LEVER_ON ? self.tickCount + self.getId() : self.tickCount) % mergeMod == 0
            && !self.level().isClientSide() && isMergable(self)) {
            mergeWithNeighbours(self);
        }

        if (self.age != -32768) {
            self.age++;
        }

        if (!(LEVER_ON && skipMove)) {
            // Lever schedule delta #2b: the second fluid scan of this tick
            // (baseTick already updated the fluid state this very tick) is
            // skipped for resting items on non-move ticks.
            self.hasImpulse |= self.updateInWaterStateAndDoFluidPushing();
        }

        if (!self.level().isClientSide()) {
            double d0 = self.getDeltaMovement().subtract(originalDelta).lengthSqr();
            if (d0 > 0.01) {
                self.hasImpulse = true;
            }
        }

        if (!self.level().isClientSide() && self.age >= (int) F_DESPAWN_RATE.get(self)) {
            ItemDespawnEvent event = CraftEventFactory.callItemDespawnEvent(self);
            if (event.isCancelled()) {
                self.age = 0;
                return;
            }
            self.discard(EntityRemoveEvent.Cause.DESPAWN);
        }
    }

    // ------------------------------------------------------------------
    // Mirrors of ItemEntity's private helpers (javap 1:1)
    // ------------------------------------------------------------------
    private static void setFluidMovement(net.minecraft.world.entity.item.ItemEntity self, double speed) {
        Vec3 vec3 = self.getDeltaMovement();
        self.setDeltaMovement(vec3.x * speed,
            vec3.y + (vec3.y < (double) 0.06F ? (double) 5.0E-4F : 0.0),
            vec3.z * speed);
    }

    private static boolean isMergable(net.minecraft.world.entity.item.ItemEntity self) {
        ItemStack itemstack = self.getItem();
        return self.isAlive()
            && self.pickupDelay != 32767
            && self.age != -32768
            && self.age < (int) F_DESPAWN_RATE.get(self)
            && itemstack.getCount() < itemstack.getMaxStackSize();
    }

    private static void mergeWithNeighbours(net.minecraft.world.entity.item.ItemEntity self) {
        if (!isMergable(self)) {
            return;
        }
        Level level = self.level();
        double radius = level.spigotConfig.itemMerge;
        AABB scanBox = self.getBoundingBox().inflate(radius,
            level.paperConfig().entities.behavior.onlyMergeItemsHorizontally ? 0.0 : radius - (double) 0.5F,
            radius);
        boolean walls = level.paperConfig().fixes.fixItemsMergingThroughWalls;

        // Lever candidate cache: validate the last partner without any
        // section scan. Exact same pair rule as the scan: alive, same
        // level, mergable, inside the inflated scan box, not wall-blocked.
        if (CANDIDATE_CACHE) {
            Cand cand = CANDIDATES.get(self.getId());
            net.minecraft.world.entity.item.ItemEntity partner = cand != null ? cand.get() : null;
            boolean valid = false;
            if (partner != null && partner != self && partner.isAlive() && !partner.isRemoved()
                && partner.level() == level && isMergable(partner)
                && scanBox.intersects(partner.getBoundingBox())) {
                valid = !walls || !wallBlocked(self, partner);
            }
            if (valid) {
                tryToMerge(self, partner);
            } else {
                CANDIDATES.remove(self.getId()); // dead/removed/level/section invalidation
            }
            if (self.isRemoved()) {
                return;
            }
        }

        List<net.minecraft.world.entity.item.ItemEntity> list =
            level.getEntitiesOfClass(net.minecraft.world.entity.item.ItemEntity.class, scanBox, new MergePredicate(self));
        for (Iterator<net.minecraft.world.entity.item.ItemEntity> it = list.iterator(); it.hasNext(); ) {
            net.minecraft.world.entity.item.ItemEntity other = it.next();
            if (isMergable(other)) {
                if (walls && wallBlocked(self, other)) {
                    continue;
                }
                tryToMerge(self, other);
                if (CANDIDATE_CACHE && !self.isRemoved()) {
                    if (CANDIDATES.size() >= SOFT_MAX) {
                        sweep();
                    }
                    CANDIDATES.put(self.getId(), new Cand(other));
                }
            }
            if (self.isRemoved()) {
                break;
            }
        }
    }

    private static boolean wallBlocked(net.minecraft.world.entity.item.ItemEntity self,
                                       net.minecraft.world.entity.item.ItemEntity other) {
        return self.level().clipDirect(self.position(), other.position(), CollisionContext.of(self))
            == HitResult.Type.BLOCK;
    }

    private static void sweep() {
        for (Iterator<java.util.Map.Entry<Integer, Cand>> it = CANDIDATES.entrySet().iterator(); it.hasNext(); ) {
            if (it.next().getValue().get() == null) {
                it.remove();
            }
        }
    }

    /** lambda$mergeWithNeighbours$0 mirror: other != self && other.isMergable(). */
    private static final class MergePredicate implements Predicate<net.minecraft.world.entity.item.ItemEntity> {
        private final net.minecraft.world.entity.item.ItemEntity self;
        MergePredicate(net.minecraft.world.entity.item.ItemEntity self) { this.self = self; }

        @Override
        public boolean test(net.minecraft.world.entity.item.ItemEntity other) {
            return other != self && isMergable(other);
        }
    }

    private static void tryToMerge(net.minecraft.world.entity.item.ItemEntity self,
                                   net.minecraft.world.entity.item.ItemEntity other) {
        ItemStack itemstack = self.getItem();
        ItemStack otherItem = other.getItem();
        if (Objects.equals(self.target, other.target) && net.minecraft.world.entity.item.ItemEntity.areMergable(itemstack, otherItem)) {
            if (otherItem.getCount() < itemstack.getCount()) {
                merge(self, itemstack, other, otherItem);
            } else {
                merge(other, otherItem, self, itemstack);
            }
        }
    }

    private static void merge(net.minecraft.world.entity.item.ItemEntity self, ItemStack stack,
                              net.minecraft.world.entity.item.ItemEntity other, ItemStack otherStack) {
        if (!CraftEventFactory.callItemMergeEvent(other, self)) {
            return;
        }
        self.setItem(net.minecraft.world.entity.item.ItemEntity.merge(stack, otherStack, 64));
        self.pickupDelay = Math.max(self.pickupDelay, other.pickupDelay);
        self.age = Math.min(self.age, other.age);
        if (otherStack.isEmpty()) {
            other.discard(EntityRemoveEvent.Cause.MERGE);
        }
    }
}
