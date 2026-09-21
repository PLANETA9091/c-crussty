package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup;
import com.google.common.base.Predicates;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.function.Predicate;
import net.minecraft.util.profiling.Profiler;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;

/**
 * TASK-401-F (vector mob-ossport): port of the Lithium <i>unpushable_cramming</i>
 * push-lane mechanism (CaffeineMC/lithium, mixin/entity/collisions/unpushable_cramming,
 * develop branch) onto the crussty runtime byte-patch bridge for Purpur 1.21.10.
 *
 * Upstream URLs (see /home/z/rounds/ROUND-401/RESEARCH-F.md):
 *  - .../mixin/entity/collisions/unpushable_cramming/LevelMixin.java
 *    (@Redirect of Level.getEntities INSIDE Level.getPushableEntities; first act:
 *     {@code if (predicate == Predicates.alwaysFalse()) return Collections.emptyList();})
 *  - .../mixin/entity/collisions/unpushable_cramming/EntitySelectorMixin.java
 *    (@Redirect of Predicate.and inside EntitySelector.pushableBy — the upstream
 *     move to a first-class composed pushable predicate instead of chained lambdas)
 *  - .../common/world/WorldHelper.java#getPushableEntities(Level, EntitySectionStorage, ...)
 *    (collect-into-provided-list section walk instead of the vanilla
 *     fresh-ArrayList + LevelEntityGetter indirection)
 *  - .../mixin/entity/collisions/unpushable_cramming/EntitySectionMixin.java
 *    (per-section pushable filtering; upstream still keeps the same per-candidate
 *     intersects/isSpectator/except/predicate contract)
 *
 * Vanilla body (javap, purpur-1.21.10) that this bridge replaces:
 * <pre>
 *   public List&lt;Entity&gt; getPushableEntities(Entity entity, AABB box) {
 *       return this.getEntities(entity, box, EntitySelector.pushableBy(entity));
 *   }
 *   // Level.getEntities(Entity, AABB, Predicate) (vanilla):
 *   //   profiler.incrementCounter("getEntities");
 *   //   List list = Lists.newArrayList();
 *   //   this.getEntities().get(entity, area, list, predicate);  // Moonrise EntityLookup
 *   //   return list;
 * </pre>
 *
 * Lithium adaptation: the per-candidate contract (predicate — Bukkit/Paper
 * pushability semantics inside EntitySelector.pushableBy, order, duplicates,
 * exceptions) is byte-exact vanilla: the predicate instance itself is produced
 * by the vanilla static EntitySelector.pushableBy(entity). What changes is ONLY
 * the container/indirection around the walk, exactly like upstream:
 *  1. alwaysFalse fast-fail (upstream LevelMixin) — team collision rule NEVER
 *     yields the shared alwaysFalse singleton; vanilla would still walk the box
 *     testing every candidate against a predicate that returns false.
 *  2. collect into a REUSED per-thread scratch list (upstream WorldHelper shape)
 *     instead of a fresh Lists.newArrayList per entity per tick — the result is
 *     consumed synchronously by the ONLY kernel caller
 *     (LivingEntity.pushEntities; verified: no other call site in the kernel,
 *     and the vanilla doPush/Entity.push chain cannot re-enter the query), so
 *     the reuse is invisible.
 *  3. direct EntityLookup walk (the very same method the vanilla LevelEntityGetter
 *     indirection ends up in) with the same "getEntities" profiler increment.
 *
 * Parity fallbacks (fail-dominant, items_oss precedent): non-ServerLevel, missing
 * lookup, or any self-test failure → exact vanilla Level.getEntities query.
 *
 * Gate: armed only by mob_oss.rs when CRUSSTY_LEVER_FLAG=cmp401_ossport; with
 * the flag unset nothing is patched and vanilla runs untouched.
 */
public final class MobOssOps {

    private static volatile MethodHandle lookupMH;
    private static volatile boolean mhFailed;
    private static volatile boolean mhChecked;

    /** Reused per-thread result list (regionized ticking safe; see class doc). */
    private static final ThreadLocal<ArrayList<Entity>> SCRATCH =
            ThreadLocal.withInitial(ArrayList::new);

    private MobOssOps() {
    }

    /** Resolve the Moonrise lookup accessor once; must succeed before arming. */
    public static boolean selfTest() {
        if (mhChecked) {
            return !mhFailed;
        }
        synchronized (MobOssOps.class) {
            if (mhChecked) {
                return !mhFailed;
            }
            try {
                MethodHandles.Lookup lookup =
                        MethodHandles.privateLookupIn(Level.class, MethodHandles.lookup());
                // Level.moonrise$getEntityLookup() is injected by the Moonrise
                // patch (public interface method on the patched Level).
                lookupMH = lookup.findVirtual(Level.class, "moonrise$getEntityLookup",
                        MethodType.methodType(EntityLookup.class));
                mhFailed = false;
            } catch (Throwable t) {
                mhFailed = true;
                System.err.println("[crussty-plugin] MobOssOps: MethodHandle init failed: " + t);
            }
            mhChecked = true;
            return !mhFailed;
        }
    }

    private static EntityLookup entityLookup(Level level) {
        try {
            return (EntityLookup) lookupMH.invokeExact(level);
        } catch (Throwable t) {
            throw new AssertionError("moonrise$getEntityLookup MH", t);
        }
    }

    /**
     * Whole-body replacement of {@code Level.getPushableEntities(Entity, AABB)}.
     * Body shape is the vanilla transcript; only the container/indirection of
     * the walk is the Lithium mechanism (with vanilla fallback).
     */
    public static List<Entity> getPushableEntities(Level level, Entity entity, AABB box) {
        // vanilla predicate construction, untouched (Bukkit pushability semantics)
        Predicate<Entity> predicate = EntitySelector.pushableBy(entity);

        // upstream LevelMixin: never walk when the predicate is alwaysFalse
        // (team collision rule NEVER) — vanilla would scan the box for nothing.
        @SuppressWarnings("unchecked")
        Predicate<Entity> alwaysFalse = (Predicate<Entity>) (Predicate<?>) Predicates.alwaysFalse();
        if (predicate == alwaysFalse) {
            return Collections.emptyList();
        }

        // upstream WorldHelper#getPushableEntities shape: direct storage walk
        // into a provided (reused) list; vanilla-equivalent profiler counter.
        if (!mhFailed && level instanceof net.minecraft.server.level.ServerLevel) {
            EntityLookup lookup = entityLookup(level);
            if (lookup != null) {
                Profiler.get().incrementCounter("getEntities");
                ArrayList<Entity> out = SCRATCH.get();
                out.clear();
                lookup.getEntities(entity, box, out, predicate);
                // vanilla Level.getEntities tail: platform hook may add extra
                // entities (e.g. dragon parts) to the result — must run too.
                ca.spottedleaf.moonrise.common.PlatformHooks.get()
                        .addToGetEntities(level, entity, box, predicate, out);
                return out;
            }
        }

        // exact vanilla query (Level.getEntities)
        return level.getEntities(entity, box, predicate);
    }
}
