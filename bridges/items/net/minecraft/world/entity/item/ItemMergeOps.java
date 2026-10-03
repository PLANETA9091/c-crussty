package net.minecraft.world.entity.item;

import ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.util.ArrayList;
import java.util.List;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.profiling.Profiler;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.HitResult;
import net.minecraft.world.phys.shapes.CollisionContext;

/**
 * TASK-396-H (vector items_oss): port of the Lithium <i>item_entity_merging</i>
 * mechanism (CaffeineMC/lithium, experimental mixin, develop branch) onto the
 * crussty runtime byte-patch bridge for Purpur 1.21.10.
 *
 * Upstream URLs (see docs/mega-round/ROUND-396-H.md):
 *  - .../mixin/experimental/entity/item_entity_merging/ItemEntityMixin.java
 *    (@Redirect of Level.getEntitiesOfClass inside ItemEntity.mergeWithNeighbours)
 *  - .../common/entity/item/ItemEntityLazyIterationConsumer.java
 *    (dry-run merge simulation + early ABORT once the stack is full)
 *
 * Vanilla body (javap, purpur-1.21.10) that this bridge replaces:
 * <pre>
 *   private void mergeWithNeighbours() {
 *       if (this.isMergable()) {
 *           double d0 = this.level().spigotConfig.itemMerge;
 *           List&lt;ItemEntity&gt; list = this.level().getEntitiesOfClass(ItemEntity.class,
 *               this.getBoundingBox().inflate(d0,
 *                   this.level().paperConfig().entities.behavior.onlyMergeItemsHorizontally ? 0.0 : d0 - 0.5, d0),
 *               (itemEntity) -&gt; itemEntity != this &amp;&amp; itemEntity.isMergable());
 *           for (ItemEntity other : list) {
 *               if (!other.isMergable()) continue;
 *               if (paperConfig().fixes.fixItemsMergingThroughWalls
 *                       &amp;&amp; level().clipDirect(this.position(), other.position(),
 *                           CollisionContext.of(this)) == HitResult.Type.BLOCK) continue;
 *               this.tryToMerge(other);
 *               if (this.isRemoved()) return;
 *           }
 *       }
 *   }
 * </pre>
 *
 * The private vanilla {@code isMergable}/{@code tryToMerge} are invoked through
 * cached MethodHandles (privateLookupIn — same unnamed module), so the merge
 * application, events and despawn-rate semantics are byte-exact vanilla.
 *
 * Lithium adaptation for the Moonrise entity lookup (the kernel's live path):
 * EntityLookup.getEntities(Class, null, box, tmp, null, LIMIT) walks the very
 * same chunk-slice regions in the very same order as the vanilla query and
 * collects the per-class ItemEntity lists with a hard limit; the underlying
 * getEntitiesLimited returns true when list.size() >= limit which EARLY-RETURNS
 * the whole walk. Collected candidates are therefore a PREFIX of the vanilla
 * candidate list. The dry-run (predicate → wall-clip → predict transfer →
 * abort when the simulated stack is full) mirrors
 * ItemEntityLazyIterationConsumer.accept/predictReceivedItemCount 1:1, with the
 * vanilla {@code target} equality (upstream checks owner instead — ours is the
 * faithful vanilla condition).
 *
 * Parity fallbacks (fail-dominant): non-ServerLevel, missing lookup, or a
 * truncated walk that did NOT fill the simulated stack → exact vanilla
 * Level.getEntitiesOfClass query. Deviations are exactly the upstream ones
 * (abort threshold) — documented in docs/mega-round/ROUND-396-H.md.
 *
 * Gate: armed only by item_merge.rs when CRUSSTY_LEVER_FLAG=items_oss; with
 * the flag unset nothing is patched and vanilla runs untouched.
 */
public final class ItemMergeOps {

    /** Hard cap of the limited section walk (vanilla max stack transfers). */
    public static final int QUERY_LIMIT = 64;

    private static volatile MethodHandle isMergableMH;
    private static volatile MethodHandle tryToMergeMH;
    private static volatile boolean mhFailed;
    private static volatile boolean mhChecked;

    /** Per-thread scratch for the limited walk (regionized ticking safe). */
    private static final ThreadLocal<ArrayList<ItemEntity>> WALK =
            ThreadLocal.withInitial(ArrayList::new);

    private ItemMergeOps() {
    }

    /** Resolve the private vanilla handles once; must succeed before arming. */
    public static boolean selfTest() {
        if (mhChecked) {
            return !mhFailed;
        }
        synchronized (ItemMergeOps.class) {
            if (mhChecked) {
                return !mhFailed;
            }
            try {
                MethodHandles.Lookup lookup =
                        MethodHandles.privateLookupIn(ItemEntity.class, MethodHandles.lookup());
                isMergableMH = lookup.findVirtual(ItemEntity.class, "isMergable",
                        MethodType.methodType(boolean.class));
                tryToMergeMH = lookup.findVirtual(ItemEntity.class, "tryToMerge",
                        MethodType.methodType(void.class, ItemEntity.class));
                mhFailed = false;
            } catch (Throwable t) {
                mhFailed = true;
                System.err.println("[crussty-plugin] ItemMergeOps: MethodHandle init failed: " + t);
            }
            mhChecked = true;
            return !mhFailed;
        }
    }

    private static boolean isMergable(ItemEntity e) {
        try {
            return (boolean) isMergableMH.invokeExact(e);
        } catch (Throwable t) {
            throw new AssertionError("isMergable MH", t);
        }
    }

    private static void tryToMerge(ItemEntity self, ItemEntity other) {
        try {
            tryToMergeMH.invokeExact(self, other);
        } catch (Throwable t) {
            throw new AssertionError("tryToMerge MH", t);
        }
    }

    /**
     * Whole-body replacement of {@code ItemEntity.mergeWithNeighbours()V}.
     * Body shape is the vanilla transcript; only the candidate query is the
     * Lithium mechanism (with vanilla fallback).
     */
    public static void mergeWithNeighbours(ItemEntity self) {
        Level level = self.level();
        // vanilla head check (private isMergable via MH)
        if (!isMergable(self)) {
            return;
        }
        double d0 = level.spigotConfig.itemMerge;
        boolean onlyHoriz = level.paperConfig().entities.behavior.onlyMergeItemsHorizontally;
        AABB box = self.getBoundingBox().inflate(d0, onlyHoriz ? 0.0 : d0 - 0.5, d0);
        boolean fixWalls = level.paperConfig().fixes.fixItemsMergingThroughWalls;

        List<ItemEntity> candidates = query(self, level, box, fixWalls);
        if (candidates == null) {
            // exact vanilla query (also increments the "getEntities" profiler counter)
            candidates = level.getEntitiesOfClass(ItemEntity.class, box,
                    other -> other != self && isMergable(other));
        }

        // vanilla loop, transcribed from javap (order, re-checks, early return)
        for (int i = 0; i < candidates.size(); i++) {
            ItemEntity other = candidates.get(i);
            if (!isMergable(other)) {
                continue;
            }
            if (fixWalls
                    && level.clipDirect(self.position(), other.position(), CollisionContext.of(self))
                            == HitResult.Type.BLOCK) {
                continue;
            }
            tryToMerge(self, other);
            if (self.isRemoved()) {
                return;
            }
        }
    }

    /**
     * Lithium item_entity_merging query: limited per-class section walk +
     * dry-run merge simulation with early abort. Returns null to signal
     * "take the exact vanilla query" (any precondition unmet, or a truncated
     * walk whose tail could still matter).
     */
    private static List<ItemEntity> query(ItemEntity self, Level level, AABB box, boolean fixWalls) {
        if (mhFailed || !(level instanceof ServerLevel)) {
            return null;
        }
        EntityLookup lookup = level.moonrise$getEntityLookup();
        if (lookup == null) {
            return null;
        }
        Profiler.get().incrementCounter("getEntities");
        ArrayList<ItemEntity> walk = WALK.get();
        walk.clear();
        lookup.getEntities(ItemEntity.class, null, box, walk, null, QUERY_LIMIT);
        boolean truncated = walk.size() >= QUERY_LIMIT;

        ItemStack stack = self.getItem();
        int adjusted = stack.getCount();
        int max = stack.getMaxStackSize();
        ArrayList<ItemEntity> out = new ArrayList<>(Math.min(walk.size() + 1, 16));
        boolean full = false;

        // ItemEntityLazyIterationConsumer.accept 1:1 (vanilla-faithful target check)
        for (int i = 0; i < walk.size(); i++) {
            ItemEntity other = walk.get(i);
            if (other == self) {
                continue; // vanilla lambda: itemEntity != this
            }
            if (!isMergable(other)) {
                continue; // vanilla lambda: itemEntity.isMergable()
            }
            if (fixWalls
                    && level.clipDirect(self.position(), other.position(), CollisionContext.of(self))
                            == HitResult.Type.BLOCK) {
                continue; // vanilla loop would skip it — never transfers
            }
            out.add(other);
            if (adjusted >= max) {
                // lithium abort precondition already met: remaining candidates
                // cannot transfer (sum > max for vanilla stacks) — documented DOC-DEV
                full = true;
                break;
            }
            // predictReceivedItemCount (vanilla merge transfer math, dry-run)
            ItemStack os = other.getItem();
            if (java.util.Objects.equals(self.target, other.target)
                    && adjusted + os.getCount() <= os.getMaxStackSize()
                    && ItemStack.isSameItemSameComponents(stack, os)) {
                int recv;
                if (os.getCount() < adjusted) {
                    // this receives: min(min(thisMax, 64) - adjusted, otherCount)
                    recv = Math.min(Math.min(max, 64) - adjusted, os.getCount());
                } else {
                    // other receives: negative delta on this
                    recv = -Math.min(Math.min(os.getMaxStackSize(), 64) - os.getCount(), adjusted);
                }
                if (recv != 0) {
                    adjusted += recv;
                    if (adjusted <= 0 || adjusted >= max) {
                        full = true; // lithium abort condition
                        break;
                    }
                }
            }
        }

        if (truncated && !full) {
            // The walk stopped at QUERY_LIMIT while the simulated stack is not
            // full: the unscanned tail could contain merge candidates. Take the
            // exact vanilla query — parity by construction.
            return null;
        }
        return out;
    }
}
