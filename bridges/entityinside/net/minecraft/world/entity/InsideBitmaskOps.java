package net.minecraft.world.entity;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.List;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.debug.DebugSubscriptions;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.LevelChunk;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import sun.misc.Unsafe;

/**
 * ARCH-ATTACK lever TASK-357 — INSIDE-BITMASK: section all-air pre-gate for
 * the checkInsideBlocks discovery (RECON-33 contract,
 * research/gc-recon-2026-09-19/RECON33_INSIDE_BITMASK_CONTRACT.md).
 *
 * INSIDE-СЕМЬЯ (RECON-32: 11.28/11.65% сцены; входы per-tick = ItemEntity.tick
 * bc273 / ExperienceOrb / FallingBlockEntity / PrimedTnt / EndCrystal.tick +
 * EnderDragon.aiStep + AbstractBoat.tick x2 + AbstractMinecart.move —
 * LivingEntity/Mob пути НЕТ, bytecode-доказано) платит за МАШИНЕРИЮ обхода
 * (gate + PalettedContainer.get + visit-set + flushStep + applier +
 * guava-iterator + lambda: s7194/s7189 collapsed-листья), даже когда ВСЕ
 * посещённые блоки — air: vanilla visit-лямбда lambda$checkInsideBlocks$2
 * (javap bc 34..65) на air-ветке ВОЗВРАЩАЕТ ДО ЛЮБЫХ наблюдаемых действий —
 * эффектов нет, visitedBlocks не читается и не пишется, step-бюджет
 * (AtomicInteger) не инкрементится (проверка идёт ДО isAir).
 *
 * ТОЧКА ВМЕШАТЕЛЬСТВА (единственный вызыватель checkInsideBlocks(List,
 * StepBasedCollector) — Entity.applyEffectsFromBlocks(List<Movement>) bc 58..64):
 *   58: aload_0
 *   59: aload_1
 *   60: aload_0
 *   61: getfield insideEffectCollector
 *   64: invokevirtual checkInsideBlocks:(Ljava/util/List;LStepBasedCollector;)V
 * ретаргетится на invokestatic
 *   InsideBitmaskOps.checkInsideBlocksGated:(LEntity;Ljava/util/List;LStepBasedCollector;)V
 * (receiver-first, 3B→3B, форма стека [this,List,Collector]→[void] сохранена).
 * Один ретаргет покрывает ВСЕ классы сущностей (все входы фуннелятся через
 * AFB(List)).
 *
 * ГЕЙТ: консервативный swept-hull всех Movement-концов (BB(from) ∪ BB(to),
 * +1 блок запаса на deflate(1e-5)/dims-дрейф) разбивается на секционные
 * клетки (≤3×3×2); клетка all-air ⟺ LevelChunkSection.hasOnlyAir() =
 * nonEmptyBlockCount==0 (vanilla-поддержка: setBlockState increment/decrement,
 * recalcBlockCounts предикат !isAir, network read; ПРЕЦЕДЕНТ в кернеле:
 * LevelChunk.getFluidState уже использует этот fast-path, javap bc 13..38).
 * ВСЕ клетки air → return БЕЗ вызова vanilla-тела: collector не тронут
 * (vanilla air-visit его тоже не трогает), visitedBlocks не тронут (пин
 * RECON-33 §2.1), step-бюджет самодостаточен внутри CIB(List). Любая не-air
 * клетка / не-FULL чанк / выхлоп за границами секций / ЛЮБОЙ Throwable →
 * вызов ИСХОДНОГО vanilla-тела через MethodHandle (unreflect+setAccessible,
 * resolved в static-init; fail-closed).
 *
 * MEDIAN-EXACT (дисциплина RECON-33 §4): поток эффектов, порядок шагов,
 * visitedBlocks и бюджет бит-в-бит эквивалентны ваниле; debug-поток
 * debugBlockIntersection гейтится проверкой debugSubscribers (при активных
 * подписчиках гейт НЕ скипает — vanilla путь). Fluid-блоки — НЕ-air
 * BlockState → невключены в nonEmptyBlockCount==0 → водные секции не
 * скипаются → fluid-эффекты сохраняются по построению.
 *
 * ARM-ДИСЦИПЛИНА: static-init резолвит MethodHandle И Unsafe-offset поля
 * ChunkAccess.sections (прецедент: InsideBlockOps резолвит private-поле тем
 * же путём); ЛЮБОЙ провал → ARMED=false → rust-драйвер (inside_bitmask.rs)
 * пробит джей-эс-эн-ай armState() ДО публикации BRIDGE_READY → compose-стейдж
 * не устанавливается → ядро ванильное. ARMED=false при установленном патче
 * невозможно по построению (probe-then-patch).
 */
public final class InsideBitmaskOps {

    private static final boolean ARMED;
    private static final MethodHandle ORIG;
    private static final Unsafe UNSAFE;
    private static final long SECTIONS_OFFSET;
    private static final double SAFETY_MARGIN = 1.0D;

    private InsideBitmaskOps() {
    }

    public static String armState() {
        return ARMED ? "ARMED" : "DISARMED";
    }

    static {
        Unsafe u = null;
        MethodHandle mh = null;
        long off = -1L;
        boolean ok = false;
        try {
            Field uf = Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            u = (Unsafe) uf.get(null);
            Method m = Entity.class.getDeclaredMethod("checkInsideBlocks",
                    List.class, InsideBlockEffectApplier.StepBasedCollector.class);
            m.setAccessible(true);
            mh = MethodHandles.lookup().unreflect(m);
            if (mh.type().changeReturnType(void.class).parameterCount() != 3) {
                throw new IllegalStateException("unexpected checkInsideBlocks handle type");
            }
            Field sf = ChunkAccess.class.getDeclaredField("sections");
            off = u.objectFieldOffset(sf);
            // sanity: fast-path primitive reachable and public
            LevelChunkSection.class.getMethod("hasOnlyAir");
            Level.class.getMethod("getChunk", int.class, int.class, ChunkStatus.class, boolean.class);
            Level.class.getMethod("getMinSectionY");
            Level.class.getMethod("getMaxSectionY");
            ok = true;
        } catch (Throwable t) {
            System.out.println("[crussty-ops] InsideBitmaskOps not armed: " + t);
            ok = false;
        }
        ARMED = ok;
        ORIG = mh;
        UNSAFE = u;
        SECTIONS_OFFSET = off;
    }

    /**
     * Retarget destination of Entity.applyEffectsFromBlocks(List) bc 64.
     * Signature MUST stay the receiver-prepended vanilla form: the rust
     * patcher asserts (LEntity;Ljava/util/List;LStepBasedCollector;)V.
     */
    public static void checkInsideBlocksGated(Entity e, List<Entity.Movement> list,
            InsideBlockEffectApplier.StepBasedCollector col) {
        if (!skipCandidate(e, list)) {
            invokeOriginal(e, list, col);
        }
        // skip: vanilla air-visits contribute no effects, no visitedBlocks
        // updates, no budget changes (RECON-33 §2) — returning here is
        // bit-identical to a full vanilla traversal over all-air blocks.
    }

    private static void invokeOriginal(Entity e, List<Entity.Movement> list,
            InsideBlockEffectApplier.StepBasedCollector col) {
        try {
            ORIG.invokeExact(e, list, col);
        } catch (RuntimeException rt) {
            throw rt;
        } catch (Error err) {
            throw err;
        } catch (Throwable t) {
            throw new RuntimeException("checkInsideBlocks rethrow", t);
        }
    }

    /**
     * Thread-confined scratch for the hull bounds — zero-allocation hot path
     * (region threads each own their slot; never shared across threads).
     */
    private static final ThreadLocal<double[]> HULL_TL =
            ThreadLocal.withInitial(() -> new double[6]);

    /**
     * true = safe to skip the whole vanilla discovery for this movement list.
     * Every doubt path returns false → vanilla body runs (fail-closed).
     */
    private static boolean skipCandidate(Entity e, List<Entity.Movement> list) {
        if (!ARMED || list == null || list.isEmpty()) {
            return false;
        }
        try {
            if (!e.isAffectedByBlocks()) {
                return false; // vanilla would no-op the gate; let vanilla decide
            }
            if (!e.isAlive()) {
                return false;
            }
            if (debugActive(e)) {
                return false;
            }
            Level lvl = e.level();
            if (lvl == null) {
                return false;
            }
            double[] h = HULL_TL.get();
            sweptHullInto(h, e.getBoundingBox(), e.position(), list);
            return hullAllAir(lvl, h[0], h[1], h[2], h[3], h[4], h[5]);
        } catch (Throwable t) {
            return false;
        }
    }

    /**
     * THE conservative swept hull, verbatim (RECON-33 §3, javadoc-bound):
     * union of BB translated to BOTH endpoints of every movement (+1-block
     * safety margin over the vanilla deflate(1e-5) to-box). PACKAGE-PRIVATE
     * so the offline oracle (InsideBitmaskLockstepHarness) drives THE REAL
     * formula — the hot path calls exactly this method via HULL_TL scratch;
     * there is no second copy of the math anywhere.
     */
    static void sweptHullInto(double[] out, AABB bb, Vec3 cur, List<Entity.Movement> list) {
        double minX = bb.minX, minY = bb.minY, minZ = bb.minZ;
        double maxX = bb.maxX, maxY = bb.maxY, maxZ = bb.maxZ;
        for (int i = 0; i < list.size(); i++) {
            Entity.Movement m = list.get(i);
            Vec3 f = m.from();
            double dx = f.x - cur.x, dy = f.y - cur.y, dz = f.z - cur.z;
            if (bb.minX + dx < minX) minX = bb.minX + dx;
            if (bb.minY + dy < minY) minY = bb.minY + dy;
            if (bb.minZ + dz < minZ) minZ = bb.minZ + dz;
            if (bb.maxX + dx > maxX) maxX = bb.maxX + dx;
            if (bb.maxY + dy > maxY) maxY = bb.maxY + dy;
            if (bb.maxZ + dz > maxZ) maxZ = bb.maxZ + dz;
            Vec3 t = m.to();
            dx = t.x - cur.x; dy = t.y - cur.y; dz = t.z - cur.z;
            if (bb.minX + dx < minX) minX = bb.minX + dx;
            if (bb.minY + dy < minY) minY = bb.minY + dy;
            if (bb.minZ + dz < minZ) minZ = bb.minZ + dz;
            if (bb.maxX + dx > maxX) maxX = bb.maxX + dx;
            if (bb.maxY + dy > maxY) maxY = bb.maxY + dy;
            if (bb.maxZ + dz > maxZ) maxZ = bb.maxZ + dz;
        }
        out[0] = minX - SAFETY_MARGIN;
        out[1] = minY - SAFETY_MARGIN;
        out[2] = minZ - SAFETY_MARGIN;
        out[3] = maxX + SAFETY_MARGIN;
        out[4] = maxY + SAFETY_MARGIN;
        out[5] = maxZ + SAFETY_MARGIN;
    }

    /** true только если ВСЯ hull-область — чанки FULL и все секции all-air.
     *  Package-private: оракул проверяет индексную математику отдельно. */
    static boolean hullAllAir(Level lvl, double minX, double minY, double minZ,
            double maxX, double maxY, double maxZ) {
        int minCX = floor(minX) >> 4, maxCX = floor(maxX) >> 4;
        int minCZ = floor(minZ) >> 4, maxCZ = floor(maxZ) >> 4;
        int minSY = floor(minY) >> 4, maxSY = floor(maxY) >> 4;
        int levelMin = lvl.getMinSectionY();
        int levelMax = lvl.getMaxSectionY(); // exclusive
        if (maxSY < levelMin || minSY >= levelMax) {
            return true; // вне высоты мира: ванильные getBlockState дают void-air
        }
        int lo = Math.max(minSY, levelMin);
        int hi = Math.min(maxSY, levelMax - 1);
        for (int cx = minCX; cx <= maxCX; cx++) {
            for (int cz = minCZ; cz <= maxCZ; cz++) {
                ChunkAccess ca = lvl.getChunk(cx, cz, ChunkStatus.FULL, false);
                if (!(ca instanceof LevelChunk)) {
                    return false; // чанк не загружен — консервативный fallback
                }
                Object o = UNSAFE.getObject(ca, SECTIONS_OFFSET);
                if (!(o instanceof LevelChunkSection[])) {
                    return false;
                }
                LevelChunkSection[] secs = (LevelChunkSection[]) o;
                for (int sy = lo; sy <= hi; sy++) {
                    int idx = sy - levelMin;
                    if (idx < 0 || idx >= secs.length) {
                        return false; // defensive: не знаем карту секций — ванилла
                    }
                    LevelChunkSection s = secs[idx];
                    if (s == null) {
                        continue; // missing section = air (vanilla getBlockState)
                    }
                    if (!s.hasOnlyAir()) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    private static int floor(double v) {
        int i = (int) v;
        return v < i ? i - 1 : i;
    }

    /** Ванильный bc 41..78 CIB(Vec3,...): подписчики debug-потока → не скипаем. */
    private static boolean debugActive(Entity e) {
        try {
            Level lvl = e.level();
            if (!(lvl instanceof ServerLevel)) {
                return false;
            }
            return ((ServerLevel) lvl).getServer().debugSubscribers()
                    .hasAnySubscriberFor(DebugSubscriptions.ENTITY_BLOCK_INTERSECTIONS);
        } catch (Throwable t) {
            return true; // на сомнении — vanilla путь
        }
    }
}
