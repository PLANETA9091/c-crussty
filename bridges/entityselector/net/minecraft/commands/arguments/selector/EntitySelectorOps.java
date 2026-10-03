package net.minecraft.commands.arguments.selector;

import java.util.List;
import java.util.concurrent.atomic.AtomicLong;
import java.util.function.Predicate;

import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.level.entity.EntityTypeTest;
import net.minecraft.world.phys.AABB;

/**
 * ES-PT (TASK-529 per-type index — lever cmp529_espt) — CONTRACT-DORMANT leg.
 *
 * JAVAP-КОНТРАКТ (AG-110 iter-2, канон-блобы): единственные getEntities-сайты
 * класса EntitySelector = invoke #297@32 (box-ветка addEntities) и #300@48
 * (no-box-ветка). Wiring-тик ретаргетит ИМЕННО эти 2 сайта на статики ниже
 * (та же стек-форма + prepended EntitySelector-receiver), дельта jar = +1
 * ops-class, 0 иных классов. Здесь: тело-контракт + гейт + счётчики G6;
 * ДЕРЖИТ ВАНИЛЬНОЕ ПОВЕДЕНИЕ БИТ-В-БИТ пока не выполнен install() (rust
 * JNI: читает приватный EntitySelector.ANY_TYPE один раз и ставит снапшоты
 * per-type цепочек за тик — law 6, per-query JNI = 0).
 *
 * FAST-PATH ПРЕДИКАТ (AG-110 spec): type != ANY_TYPE (reference-compare с
 * JNI-снятым anyTypeRef — private-поле недоступно java-стороне, setAccessible
 * запрещён) И limit == 1 (вход в ветку возможен только при list.size()==0 —
 * цикл addEntities проверил list.size() < resultLimit). else = ванильный
 * delegate: level.getEntities(те же аргументы) — бит-в-бит.
 *
 * ПОРЯДОК-СТЕНА (Л58/Л146): per-type цепочки в E-порядке (dense-id порядок
 * eindex-зеркала); первый прошедший pred кандидат = первый ванильный match
 * (subsequence-инвариант), результат порядок-сохраняющий. Проверка живости
 * и предикат — ТОЛЬКО на живом Entity (fetchEntity-belt, ниже), superset-
 * oracle: предикат применяется ПОСЛЕ индексного фетча (та же семантика).
 *
 * NO-CACHE (RECON-39/40): никакого кэша результатов — цепочки это
 * индекс-плоскость, refresh-ей за тик; счётчики монотонные fail-dominant,
 * любой отказ = vanilla delegate + причина в FB_* (G6).
 *
 * DORMANT: armed=false по умолчанию, install() этим тиком НЕ вызывается,
 * определение класса в loader НЕ выполняется (ниже 0 define/retarget) —
 * blob = include_bytes! заготовка wiring-тика (канон chunksched/sb_r1).
 */
public final class EntitySelectorOps {

    private EntitySelectorOps() {}

    // ----- lever / arm (STRICT-eq, swarx-4 урок: один id) -------------------
    public static final String LEVER_FLAG = "cmp529_espt";
    public static final String ARM_ENV = "CRUSSTY_ES_PT";

    // ----- G6 counters: монотонные, fail-dominant, без резетов --------------
    public static final AtomicLong INDEX_HIT = new AtomicLong();
    public static final AtomicLong FB_DISARM = new AtomicLong();   // не armed / нет install
    public static final AtomicLong FB_SNAPSHOT = new AtomicLong(); // снапшот устарел (epoch!=tick)
    public static final AtomicLong FB_SLOT = new AtomicLong();     // type не в слот-таблице
    public static final AtomicLong FB_CHAIN_EMPTY = new AtomicLong(); // цепь типа пуста
    public static final AtomicLong FB_STRUCT = new AtomicLong();   // нет entity по id (уплотнение)
    public static final AtomicLong FB_ERR = new AtomicLong();      // belt Throwable
    public static final AtomicLong VANILLA_DELEGATE = new AtomicLong();

    private static volatile boolean armed = false;
    private static volatile boolean bannerPrinted = false;
    private static volatile Object anyTypeRef = null; // JNI-снятие приватного ANY_TYPE (1 раз)

    // ----- per-tick snapshot (bulk JNI 1/тик, law 6; double-buffer rust) ----
    public static final int SLOT_CAP = 1024;
    static volatile long snapEpoch = -1L;         // тик сборки снапшота
    static volatile int slotCount = 0;
    static int[] slotHash = new int[SLOT_CAP];    // identityHashCode(type)+1, 0 = пусто
    static EntityTypeTest<?, ?>[] slotRef = new EntityTypeTest[SLOT_CAP]; // reference-верификация
    static int[] slotHead = new int[SLOT_CAP];    // голова per-type цепи (denseId+1, 0=пусто)
    static int[] chainNext = new int[0];          // per-dense-id next (denseId+1, 0=конец)
    static int[] byId = new int[0];               // denseId -> net entity id (mirror-канон)
    static volatile int idTop = 0;

    /** belt: denseId -> живой Entity (wiring-тик биндит на byId-плоскость MobPushOps). */
    public interface EntityBelt { Entity byDenseId(int denseId); }
    private static volatile EntityBelt belt = null;

    // ----- install (ОДИН раз, rust wiring; до него всё = ваниль) -----------
    /** JNI-инсталл: ANY_TYPE reference + первый снапшот. Не бросает (fail-dominant). */
    public static void install(Object anyTypeInstance, EntityBelt entityBelt) {
        anyTypeRef = anyTypeInstance;
        belt = entityBelt;
        armed = true;
        if (!bannerPrinted) { // G2 ARM-страж: маркер в stdout (navmath-1 урок)
            bannerPrinted = true;
            System.out.println("es_pt: ARMED " + LEVER_FLAG);
        }
    }

    /** per-tick bulk refresh (1 JNI/тик; rust строит цепи в E-порядке denseId). */
    public static void refreshSnapshot(long epoch, int[] hash, EntityTypeTest<?, ?>[] refs,
                                       int[] heads, int[] next, int[] idMap, int nSlots, int top) {
        try {
            slotHash = hash; slotRef = refs; slotHead = heads;
            chainNext = next; byId = idMap; slotCount = nSlots; idTop = top;
            snapEpoch = epoch;
        } catch (Throwable t) {
            FB_ERR.incrementAndGet(); // снапшот не обновлён — гейт отвалится на epoch-чеке
        }
    }

    public static void disarm() { armed = false; } // fail-closed клапан (G6)

    // ----- гейт fast-path ----------------------------------------------------
    private static boolean gate(EntityTypeTest<Entity, Entity> type, int limit, long tick) {
        if (!armed || anyTypeRef == null) { FB_DISARM.incrementAndGet(); return false; }
        if (type == anyTypeRef || limit != 1) { FB_DISARM.incrementAndGet(); return false; }
        if (snapEpoch != tick) { FB_SNAPSHOT.incrementAndGet(); return false; }
        return true;
    }

    // ----- resolve slot: identityHash-проба + reference-вериф, 0 аллокаций --
    private static int resolveSlot(EntityTypeTest<?, ?> type) {
        int h = System.identityHashCode(type) + 1;
        int n = slotCount;
        int[] hs = slotHash;
        EntityTypeTest<?, ?>[] rs = slotRef;
        for (int i = 0; i < n && i < SLOT_CAP; i++) {
            if (hs[i] == h && rs[i] == type) return i; // reference-eq: коллизии хеша не ложатся
        }
        return -1;
    }

    /**
     * Fast-path: первый кандидат per-type цепи (E-порядок), pred ПОСЛЕ фетча,
     * belt- живость + живой Entity-тест. Возврат true = результат добавлен;
     * false = честный fallback (счётчик причины уже поднят).
     */
    private static boolean tryChainFirst(ServerLevel level, EntityTypeTest<Entity, Entity> type,
                                         Predicate<? super Entity> pred, List<? super Entity> list,
                                         long tick) {
        int slot = resolveSlot(type);
        if (slot < 0) { FB_SLOT.incrementAndGet(); return false; }
        int id = slotHead[slot];
        if (id == 0) { FB_CHAIN_EMPTY.incrementAndGet(); return false; }
        int[] next = chainNext;
        int[] idMap = byId;
        EntityBelt b = belt;
        if (b == null) { FB_STRUCT.incrementAndGet(); return false; }
        try {
            while (id != 0) {
                int dense = id - 1;
                Entity e = b.byDenseId(dense); // живой Entity или null (belt-fail-dominant)
                if (e != null && e.isAlive() && pred.test(e)) {
                    list.add(e);
                    INDEX_HIT.incrementAndGet();
                    return true;
                }
                id = dense < next.length ? next[dense] : 0;
            }
            FB_CHAIN_EMPTY.incrementAndGet(); // цепь живая, но pred никого не взял
            return false;
        } catch (Throwable t) {
            FB_ERR.incrementAndGet();
            return false;
        }
    }

    // ----- РЕТАРГЕТ #297@32: ServerLevel.getEntities(TypeTest,AABB,Pred,List,I)V
    // (стек-форма сайта + prepended EntitySelector receiver от wiring-хирургии;
    //  box-ветка остаётся vanilla-delegate в Fast-path: per-type цепи без AABB —
    //  box-запросы НЕ обслуживаются этим тиком, FB_DISARM честно растит счётчик).
    public static void esGetEntitiesBox(EntitySelector sel, ServerLevel level,
                                        EntityTypeTest<Entity, Entity> type, AABB box,
                                        Predicate<? super Entity> pred, List<? super Entity> list,
                                        int limit) {
        if (gate(type, limit, level.getServer().overworld().getGameTime())) {
            // box-ветка: superset-прун по AABB на живом bb возможен, но E-порядок
            // per-type цепей не обязан совпадать с ванильным box-walk порядком —
            // порядок-стена Л58: НЕ обслуживаем, честный vanilla delegate.
            FB_STRUCT.incrementAndGet();
        }
        level.getEntities(type, box, pred, list, limit);
        VANILLA_DELEGATE.incrementAndGet();
    }

    // ----- РЕТАРГЕТ #300@48: ServerLevel.getEntities(TypeTest,Pred,List,I)V --
    public static void esGetEntities(EntitySelector sel, ServerLevel level,
                                     EntityTypeTest<Entity, Entity> type,
                                     Predicate<? super Entity> pred, List<? super Entity> list,
                                     int limit) {
        if (gate(type, limit, level.getServer().overworld().getGameTime())
                && tryChainFirst(level, type, pred, list, limit)) {
            return; // fast-path: ровно 1 кандидат добавлен (limit==1 ∧ size==0)
        }
        level.getEntities(type, pred, list, limit);
        VANILLA_DELEGATE.incrementAndGet();
    }
}
