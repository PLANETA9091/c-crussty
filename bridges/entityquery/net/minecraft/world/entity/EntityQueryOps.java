package net.minecraft.world.entity;

import ca.spottedleaf.moonrise.common.PlatformHooks;
import ca.spottedleaf.moonrise.patches.chunk_system.level.ChunkSystemLevel;
import ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup;
import net.minecraft.core.BlockPos;
import net.minecraft.util.profiling.Profiler;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

/**
 * ARCH-ATTACK lever S7-133 / TASK-269 — zero-alloc tick-thread entity queries
 * ("аллокационная диета entity-лэйна").
 *
 * Profiles (X150K, 150k живых, runs 35245032701/35264319982) показывают
 * G1 GC + oop write-barriers = ~27% CPU — производную аллокационного чёрна
 * entity-лэйна. Два подтверждённых javap-контрактом аллокационных узла:
 *
 *  1) LivingEntity.pushEntities -> Level.getPushableEntities ->
 *     Level.getEntities(Entity,AABB,Predicate): на каждый вызов
 *     new ArrayList ( DEAD guava-список, результат не читается ) +
 *     new ArrayList ( fill-список ) + барьеры заполнения. ~45k+ живых
 *     сущностей в тик => ~135k+ young-gen аллокаций/тик только на заголовки.
 *
 *  2) CollisionUtil.getCollisionsForBlocksOrWorldBorder: на каждый запрос
 *     new BlockPos.MutableBlockPos (безусловный, до веток) + new
 *     LazyEntityCollisionContext (ctx-пул отложен в wave-2: приватные
 *     final-поля базового класса, безопасная ре-инициализация невозможна
 *     без Unsafe/MH; mutable-pos сплайсится первой ногой).
 *     ~250k+ запросов/тик (148k move + 100k item noPhysics noCollision).
 *
 * Deep-машина moonrise (EntityLookup.getEntities) уже fill-into-list,
 * порядок обхода секций фиксирован — переиспользование буфера НЕ меняет
 * наблюдаемую последовательность результатов (median-exact parity):
 * заливается ТЕМ ЖЕ методом с ТЕМИ ЖЕ аргументами, включая PlatformHooks
 * addToGetEntities и Profiler-счётчик "getEntities" (полное соответствие
 * телу Level.getEntities, минус мёртвый guava-список, результат которого
 * не читается ни одной инструкцией — исчезновение этой аллокации не
 * наблюдаемо).
 *
 * БЕЗОПАСНОСТЬ ПОВТОРНОГО ВХОЖДЕНИЯ: результат не удерживается ни одним
 * байткодом вызывателя (census: pushEntities использует isEmpty/size/
 * iterator внутри метода; getCollisionsForBlocksOrWorldBorder использует
 * pos внутри метода). Тем не менее возможен вложенный запрос на том же
 * потоке ПОСЛЕ возврата, пока вызыватель ещё итерирует результат
 * (реальный сервер: колбэки Bukkit-событий в push-пути). Поэтому пул
 * РОТИРУЕТСЯ: каждый вызов берёт следующий слот из кольца из
 * ROTATION_SLOTS; вложенные запросы глубиной до ROTATION_SLOTS-1 не
 * трогают список, по которому ещё идёт итерация внешнего вызова.
 * Выход за глубину кольца = CME (громкое падение), а не тихая порча.
 *
 * Рост буферов: grow-only (ArrayList хранит backing array), clear() перед
 * заливкой. Первая итерация тика прогревает кольцо (ROTATION_SLOTS
 * аллокаций на слот один раз), далее ноль young-gen аллокаций на
 * горячем пути.
 */
public final class EntityQueryOps {

    private EntityQueryOps() {}

    /**
     * Кольцо результатов pushables. Верхняя граница живых результатов
     * одновременно: внешняя итерация + вложенные запросы из колбэков
     * событий/предикатов. 8 покрывает с запасом все наблюдаемые глубины
     * ванильного стека; вложенностей глубже CME-громко.
     */
    private static final int ROTATION_SLOTS = 8;

    private static final ThreadLocal<ArrayList<Entity>[]> PUSH_RING =
            ThreadLocal.withInitial(EntityQueryOps::newPushRing);
    private static final ThreadLocal<int[]> PUSH_CURSOR =
            ThreadLocal.withInitial(() -> new int[1]);

    @SuppressWarnings("unchecked")
    private static ArrayList<Entity>[] newPushRing() {
        ArrayList<Entity>[] ring = new ArrayList[ROTATION_SLOTS];
        for (int i = 0; i < ROTATION_SLOTS; i++) {
            ring[i] = new ArrayList<>(16);
        }
        return ring;
    }

    /**
     * Точная замена тела Level.getPushableEntities(Entity,AABB) ->
     * Level.getEntities(Entity,AABB,Predicate):
     *   Profiler-счётчик "getEntities";
     *   fill через EntityLookup.getEntities(entity, box, list, predicate);
     *   PlatformHooks.addToGetEntities(level, entity, box, predicate, list);
     *   вернуть список.
     * Отличие от ванили: (a) список переиспользуемый (кольцо), (b) мёртвый
     * guava-список (newArrayList, результат не читается) не создаётся.
     * Порядок сущностей в списке идентичен ваниле (тот же deep-метод).
     */
    public static List<Entity> pushables(Level level, Entity entity, AABB box) {
        Profiler.get().incrementCounter("getEntities");
        ArrayList<Entity>[] ring = PUSH_RING.get();
        int[] cursor = PUSH_CURSOR.get();
        int slot = cursor[0];
        cursor[0] = (slot + 1) % ROTATION_SLOTS;
        ArrayList<Entity> list = ring[slot];
        list.clear();

        Predicate<Entity> predicate = EntitySelector.pushableBy(entity);
        EntityLookup lookup = ((ChunkSystemLevel) (Object) level).moonrise$getEntityLookup();
        lookup.getEntities(entity, box, list, predicate);
        PlatformHooks.get().addToGetEntities(level, entity, box, predicate, list);
        return list;
    }

    private static final ThreadLocal<BlockPos.MutableBlockPos[]> POS_RING =
            ThreadLocal.withInitial(EntityQueryOps::newPosRing);
    private static final ThreadLocal<int[]> POS_CURSOR =
            ThreadLocal.withInitial(() -> new int[1]);

    private static BlockPos.MutableBlockPos[] newPosRing() {
        BlockPos.MutableBlockPos[] ring = new BlockPos.MutableBlockPos[ROTATION_SLOTS];
        for (int i = 0; i < ROTATION_SLOTS; i++) {
            ring[i] = new BlockPos.MutableBlockPos();
        }
        return ring;
    }

    /**
     * Точная замена последовательности
     *   new BlockPos.MutableBlockPos(); dup; invokespecial &lt;init&gt;:()V
     * в CollisionUtil.getCollisionsForBlocksOrWorldBorder.
     * Ванильный ctor (super(0,0,0)) = поля x=y=z=0; set(0,0,0) пишет те же
     * поля теми же значениями => экземпляр неотличим от свежего.
     */
    public static BlockPos.MutableBlockPos mutablePos() {
        BlockPos.MutableBlockPos[] ring = POS_RING.get();
        int[] cursor = POS_CURSOR.get();
        int slot = cursor[0];
        cursor[0] = (slot + 1) % ROTATION_SLOTS;
        BlockPos.MutableBlockPos pos = ring[slot];
        pos.set(0, 0, 0);
        return pos;
    }
}
