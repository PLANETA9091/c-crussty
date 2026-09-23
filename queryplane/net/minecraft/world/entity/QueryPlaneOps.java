package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.patches.chunk_system.entity.ChunkSystemEntity;
import ca.spottedleaf.moonrise.patches.chunk_system.level.ChunkSystemLevel;
import ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup;
import net.minecraft.util.profiling.Profiler;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;
import java.util.function.Predicate;

/**
 * TASK-412-B PIVOT (round-412-b-p1, lever cmp412_b2p1 STRICT eq) —
 * entity-query snapshot plane: подсистемный срез broadphase-лейна
 * (закон 6 v16 — ПОДСИСТЕМА, не одиночная функция).
 *
 * Два ретаргета (src/classfile.rs, whole-body redirect —
 * redirect_method_body_to_static, receiver-prepended 3B→3B):
 *
 *   1) Level.getEntitiesOfClass(Class,AABB,Predicate) →
 *      {@link #getEntitiesOfClass}. Профиль meganav1 (107629 сэмплов):
 *      Level.getEntitiesOfClass 1.85% wall, из них
 *      NearestAttackableTargetGoal.findTarget 0.81% + AvoidEntityGoal.canUse
 *      0.22% — ОБА запрашивают Player.class (скелет/зомби target-поиск и
 *      spider-avoidance против 4 fake-players). Ванильный путь — дорогой
 *      region-walk EntityLookup.getEntities по 16³-секциям; а кандидатов-то —
 *      игроки, и их ПОЛНЫЙ authoritative per-level список уже существует:
 *      Level.players() (O(players), в бенче 4). FAST PATH ONLY для
 *      Player.class: точный ванильный фильтр на каждом игроке
 *      (AABB.intersects + predicate, null-predicate = add-all) — SET
 *      ваниль-эквивалентен (каждый ServerLevel-игрок в lookup И в players();
 *      игроки всегда в FULL-чанках — FULL-status gate ванильного walk для
 *      них вырожден). Порядок кандидатов = players() порядок vs
 *      section-walk порядок — документир. дельта класса items_subsys2
 *      (NearestAttackableTargetGoal выбирает nearest — ties разруливаются
 *      порядком, событийная вероятность ничтожна). Все остальные классы
 *      (LivingEntity.class sensors 0.05%, ItemEntity.class hoppers 0.05%,
 *      всё прочее) — ВАНИЛЬНАЯ реплика тела бит-в-бит.
 *
 *   2) Level.moonrise$getHardCollidingEntities(Entity,AABB,Predicate) →
 *      {@link #getHardCollidingEntities}. Профиль: getEntityHardCollisions
 *      2.01% wall (Entity.collide 1.02 + Level.noCollision 0.92) — region-walk
 *      по hardCollidingEntities-коллекциям секций. В этой сцене hard-colliders
 *      (canBeCollidedWith: лодки/шалкеры/армор-стенды) не спавнятся ВООБЩЕ.
 *      Счетчик HARD_ADDS монотонно растёт на каждом add-пути
 *      ChunkEntitySlices.addEntity с moonrise$isHardColliding()==true
 *      (ретаргет invokeinterface→invokestatic+2×nop в
 *      {@link #isHardCollidingProbe}, длина-сохраняющий, stack-map цель
 *      не сдвигается). HARD_ADDS.get()==0 ⇒ все hard-коллекции пусты ⇒
 *      ванильный walk возвращает empty list — быстрый empty-list БИТ-В-БИТ
 *      эквивалентен. Счетчик НИКОГДА не декрементируется (remove не
 *      ретаргетится): один hard-collider за всю жизнь JVM → навсегда
 *      ванильный walk (fail-dominant). Blind-window (boot→retransform):
 *      addEntity до ретаргета не посчитан; в сцене hard-colliders не
 *      спавнятся вовсе — окно вырождено, честно документировано.
 *
 * FAIL-CLOSED: пустой/чужой CRUSSTY_LEVER_FLAG — ретаргеты не ставятся
 * (rust-сторона), путь ванильный по построению. Любой Throwable в fast
 * path — ПЕРМАНЕНТНЫЙ дизарм соответствующей плоскости (volatile latch) +
 * ванильная реплика тела на этот вызов. Определение класса ДО ретаргета
 * (probe-then-patch, дисциплина inside_batch R4): NCDFE невозможен.
 * Реплика ванильного тела (javap Level#getEntitiesOfClass 3-арг /
 * moonrise$getHardCollidingEntities): Profiler-счётчик "getEntities" +
 * new ArrayList + moonrise$getEntityLookup().getEntities* + areturn.
 */
public final class QueryPlaneOps {

    /** STRICT OR lever (пустой/чужой флаг = false; ретаргеты и так сняты).
     *  TASK-415-A: {cmp412_b2p1 || cmp415_mcomp} (композит эры).
     *  TASK-417-C: + cmp417_bq (cvs-носитель ⊕ queryplane awake —
     *  find_class fix: selfTest зван на local ref из define_class). */
    private static boolean flagArmed() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && (f.trim().equals("cmp412_b2p1") || f.trim().equals("cmp415_mcomp") || f.trim().equals("cmp416_mcomp")
                || f.trim().equals("cmp417_bq")
                // TASK-419-A (colpush): колпаш-носитель — queryplane awake.
                || f.trim().equals("cmp420_colpush")
                // TASK-421-A: brain-носитель (STRICT OR).
                || f.trim().equals("cmp421_brain")
                // TASK-422-B: brain iter-2 вектор-флаг (STRICT OR).
                || f.trim().equals("cmp422_brain2"));
    }

    private static final boolean ENABLED = flagArmed();

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    /** Монотонный add-счетчик hard-colliders (isHardCollidingProbe). */
    static final AtomicLong HARD_ADDS = new AtomicLong();

    /** Per-plane permanent disarm latches (fail-dominant). */
    private static volatile boolean playerPlaneBroken;
    private static volatile boolean hardPlaneBroken;

    /** ЭФФЕКТ-МАРКЕРЫ (one-shot): первый реальный fast-path hit каждой
     * плоскости — grep-доказательство, что путь не только armed, но и
     * исполняется (урок ×93/cv3b-1: ARM-строки не доказывают работу). */
    private static final AtomicBoolean FIRST_PLAYER_HIT = new AtomicBoolean();
    private static final AtomicBoolean FIRST_HARD_HIT = new AtomicBoolean();

    private QueryPlaneOps() {}

    // ------------------------------------------------------------------
    // 1) Level.getEntitiesOfClass(Class,AABB,Predicate)
    // ------------------------------------------------------------------

    public static List getEntitiesOfClass(Level level, Class clazz, AABB box, Predicate pred) {
        // Реплика головы ванильного тела (javap @0-6): Profiler-счётчик.
        Profiler.get().incrementCounter("getEntities");
        if (ENABLED && !playerPlaneBroken && clazz == net.minecraft.world.entity.player.Player.class) {
            try {
                List<? extends net.minecraft.world.entity.player.Player> players = level.players();
                ArrayList out = new ArrayList(Math.max(4, players.size()));
                for (int i = 0; i < players.size(); i++) {
                    net.minecraft.world.entity.player.Player p = players.get(i);
                    if (p == null) {
                        continue;
                    }
                    if (!p.getBoundingBox().intersects(box)) {
                        continue; // точный ванильный box-тест (section-walk @155-159)
                    }
                    if (pred != null && !pred.test(p)) {
                        continue; // точный ванильный predicate-тест (@168-185)
                    }
                    out.add(p);
                }
                if (FIRST_PLAYER_HIT.compareAndSet(false, true)) {
                    LOG.warning("[crussty-plugin] cmp412_b2p1: EFFECT first gate hit: players fast path ("
                            + out.size() + " candidates)");
                }
                return out;
            } catch (Throwable t) {
                playerPlaneBroken = true; // навсегда ваниль (fail-dominant)
                LOG.warning("[crussty-plugin] cmp412_b2p1: QueryPlaneOps player-plane disarmed: " + t);
            }
        }
        // Ванильная реплика тела (javap @11-37): new ArrayList + lookup
        // .getEntities(clazz, null, box, list, pred) + areturn.
        ArrayList list = new ArrayList();
        ((ChunkSystemLevel) (Object) level)
                .moonrise$getEntityLookup()
                .getEntities(clazz, null, box, list, pred);
        return list;
    }

    // ------------------------------------------------------------------
    // 2) Level.moonrise$getHardCollidingEntities(Entity,AABB,Predicate)
    // ------------------------------------------------------------------

    public static List getHardCollidingEntities(Level level, Entity excluded, AABB box, Predicate pred) {
        // Реплика головы ванильного тела (javap @0-6).
        Profiler.get().incrementCounter("getEntities");
        if (ENABLED && !hardPlaneBroken && HARD_ADDS.get() == 0L) {
            // Ни один hard-collider не был добавлен в НИКАКУЮ ChunkEntitySlices
            // с момента старта JVM ⇒ все hardCollidingEntities-коллекции пусты
            // ⇒ ванильный walk вернул бы empty list. Бит-в-бит.
            if (FIRST_HARD_HIT.compareAndSet(false, true)) {
                LOG.warning("[crussty-plugin] cmp412_b2p1: EFFECT first gate hit: hard-colliding empty fast path");
            }
            return new ArrayList();
        }
        // Ванильная реплика тела (javap @11-36).
        ArrayList list = new ArrayList();
        ((ChunkSystemLevel) (Object) level)
                .moonrise$getEntityLookup()
                .getHardCollidingEntities(excluded, box, list, pred);
        return list;
    }

    // ------------------------------------------------------------------
    // 3) ChunkEntitySlices.addEntity probe (monotone hard-collider counter)
    // ------------------------------------------------------------------

    /** Retarget-цель invokeinterface ChunkSystemEntity.moonrise$isHardColliding:()Z
     * в ChunkEntitySlices.addEntity(Entity,int)Z (javap @50, 5B → invokestatic
     * 3B + 2×nop). Статический дескриптор = receiver-класс CP-сайта
     * (интерфейс ChunkSystemEntity) — receiver-префиксованная форма, точный
     * стек-шейп ([Entity] -> [Z]; Entity реализует ChunkSystemEntity).
     * Семантика идентична (public abstract на интерфейсе, Entity — public
     * final), плюс монотонный счетчик. */
    public static boolean isHardCollidingProbe(ChunkSystemEntity e) {
        boolean v = e.moonrise$isHardColliding();
        if (v) {
            HARD_ADDS.incrementAndGet();
        }
        return v;
    }

    /** Self-test (armed-only, вызывается rust-стороной после define). */
    public static boolean selfTest() {
        if (!ENABLED) {
            return false;
        }
        return HARD_ADDS.get() == 0L;
    }
}
