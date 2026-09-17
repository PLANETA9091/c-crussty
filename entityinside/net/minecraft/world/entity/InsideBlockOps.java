package net.minecraft.world.entity;

import net.minecraft.core.BlockPos;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.minecraft.world.phys.shapes.Shapes;

/**
 * ARCH-ATTACK lever S7-135 / TASK-271 — INSIDE-CACHE: per-entity
 * memoization of the checkInsideBlocks discovery для статичных сущностей.
 *
 * CENSUS (первый истинный alloc-ценз X150K, run 35275967738, §155):
 * 66% young-gen чёрна (21.4MB/тик) = movement-геометрия (Vec3.add 9.5%
 * через collidedWithFluid/collidedAlongVector, AABB makeBoundingBox 5.8%,
 * Fluid.getAABB 3.0%) + inside-blocks (LongOpenHashSet 6.1% — dedup НА
 * СУЩНОСТЬ НА ТИК в forEachBlockIntersectedBetween; BlockPos$6
 * corner-lambda 5.2%; flushStep Arrays.copyOf 4.6%). Топ-1 kernel
 * CPU-функция PalettedContainer.get (3.1% + readPalette 0.8%) питается
 * ТЕМИ ЖЕ путями (getBlockState визитора checkInsideBlocks). Ванила на
 * статичной сущности исполняет ДВЕ полных traversal за тик (main
 * from-to + финальный to-to с бюджетом 1) — вторая даёт только
 * visitedBlocks-дюпы (чистый оверхед: set/lambda/visitor/getBlockState/
 * shape/fluid-геометрия на каждую позицию).
 *
 * ТОЧКА ВМЕШАТЕЛЬСТВА (javap-контракт, единственный 3B→3B ретаргет):
 *   private void Entity.checkInsideBlocks(List&lt;Movement&gt;, StepBasedCollector)
 *     0: aload_0
 *     1: invokevirtual Entity.isAffectedByBlocks:()Z   ← единственный сайт
 *        methodref'а (Entity,isAffectedByBlocks,()Z) В ЭТОМ методе
 *     4: ifeq 242
 *   ретаргетится на invokestatic
 *   InsideBlockOps.gate:(Lnet/minecraft/world/entity/Entity;)Z
 *   (receiver-first, форма стека [this]→[boolean] сохранена, длина 3B).
 *   Сайт в applyEffectsFromBlocks (offset 1) НЕ трогается — stepOn/
 *   fire-логика applyEffectsFromBlocks остаётся ванильной.
 *
 * ПОЧЕМУ КОРРЕКТНО (median-exact parity, дисциплина TASK-77):
 *  - Replay переигрывает ВЫЗОВЫ ванильной логики (advanceStep +
 *    BlockState.entityInside + onInsideBlock + FluidState.entityInside)
 *    с верифицированными входами: Block.getId каноничен (== identity),
 *    позиция бит-в-бит; свежее исполнение сохраняет зависимость от
 *    эволюции состояния сущности (canFreeze и т.п.) — кэшируются
 *    ВЫЗОВЫ, не результаты. applyAndClear делает ванильный вызыватель.
 *  - Ветвление визитора (javap 140-345): effectful = hitShape || inFluid;
 *    block-ветка (hitShape): advanceStep + entityInside(.., intersected)
 *    + onInsideBlock; fluid-ветка (inFluid, ПОСЛЕ block): advanceStep +
 *    FluidState.entityInside. intersected = hasMoved || box.intersects,
 *    для статики hasMoved=false (javap 170-190: distanceToSqr(from,to)>
 *    square(0.9999900000002526)).
 *  - Mirror-traversal использует ТЕ ЖЕ ванильные примитивы:
 *    forEachBlockIntersectedBetween(pos, pos, makeBoundingBox(pos).
 *    deflate(9.999999747378752E-6), recorder) — то же множество
 *    позиций/шагов, что у обеих ванильных traversal статичной сущности
 *    (from==to ⇒ (from,to) и (to,to) с тем же боксом дают одно
 *    множество; visitedBlocks-дедуп делает вторую traversal net-нулём).
 *    hitShape/inFluid пересчитываются теми же вызовами, что в визиторе
 *    (getEntityInsideCollisionShape == Shapes.block() ||
 *    collidedWithShapeMovingFrom; collidedWithFluid). isAlive — на
 *    каждом visit (зеркало гейта 0-8), budget 16 — зеркало 9-16.
 *  - СТАТИЧНОСТЬ: gate обслуживает только deltaMovement==(0,0,0) и
 *    позицию, бит-в-бит равную закэшированной. Путь move() передаёт
 *    from=позиция-начала-тика == закэшированной и to==from при нуле
 *    дельты ⇒ swept-множество совпадает. Экзотический вызов
 *    applyEffectsFromBlocks(vecFrom!=vecTo, vecTo==кэш) остаётся
 *    ванильным по построению кэша (позиция изменилась ⇒ слот сброшен),
 *    кроме телепорта-в-ту-же-точку (частота ноль, документировано).
 *  - visitedBlocks остаётся пустым: единственные потребители —
 *    семейство checkInsideBlocks (javap-census поля #922); ванильные
 *    (нестатичные) тики чистят его в конце тела как обычно.
 *
 * COLLECTOR: поле insideEffectCollector приватное (javap #925) — читается
 * Unsafe.objectFieldOffset (раз/тик/сущность, ~1ns); резолв в static-init
 * fail-closed (ARMED=false ⇒ gate всегда ваниллен); rust-патчер
 * дополнительно проверяет имя поля в байтах ядра ДО армирования.
 *
 * КЭШ: плоские примитивные массивы (никаких oop-массивов — ноль
 * card-mark давления на старое поколение, урок §153/§155), слот =
 * entityId & (NSLOTS-1), штамп = entityId (0 = пусто). Ноль аллокаций
 * и ноль записей на HIT (записи только на capture). Overflow (>MAXVIS
 * позиций) => слот не коммитится, эффекты всё равно применены зеркалом
 * (parity не зависит от кэша). Коллизия слотов => перезапись (мисс).
 */
public final class InsideBlockOps {

    private InsideBlockOps() {}

    static final int NSLOTS = 1 << 17; // 131072 слотов
    static final int MAXVIS = 12;
    static final int MAXSTEPS = 16; // javap-контракт бюджета визитора

    static final long[] SLOT_EID = new long[NSLOTS];   // штамп (0 = пусто)
    static final long[] SLOT_FX = new long[NSLOTS];    // raw bits позиции
    static final long[] SLOT_FY = new long[NSLOTS];
    static final long[] SLOT_FZ = new long[NSLOTS];
    static final int[] SLOT_NVIS = new int[NSLOTS];
    static final int[] SLOT_NEFF = new int[NSLOTS];
    static final long[] VIS_POS = new long[NSLOTS * MAXVIS];
    static final int[] VIS_STATE = new int[NSLOTS * MAXVIS];
    static final long[] EFF_POS = new long[NSLOTS * MAXVIS];
    static final int[] EFF_STATE = new int[NSLOTS * MAXVIS];
    static final int[] EFF_STEP = new int[NSLOTS * MAXVIS];
    static final int[] EFF_FLAG = new int[NSLOTS * MAXVIS]; // bit0 block, bit1 fluid, bit2 intersected

    static final double DEFLATE = 9.999999747378752E-6d; // javap #2462 inner checkInsideBlocks
    private static final int FLAG_BLOCK = 1;
    private static final int FLAG_FLUID = 2;
    private static final int FLAG_INTERSECTED = 4;

    private static final sun.misc.Unsafe UNSAFE;
    private static final long COL_OFFSET;
    private static final boolean ARMED;

    static {
        sun.misc.Unsafe u = null;
        long off = 0L;
        try {
            java.lang.reflect.Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
            uf.setAccessible(true);
            u = (sun.misc.Unsafe) uf.get(null);
            java.lang.reflect.Field cf = Entity.class.getDeclaredField("insideEffectCollector");
            off = u.objectFieldOffset(cf);
        } catch (Throwable t) {
            u = null;
        }
        UNSAFE = u;
        COL_OFFSET = off;
        ARMED = UNSAFE != null;
    }

    @SuppressWarnings("unchecked")
    private static InsideBlockEffectApplier.StepBasedCollector col(Entity e) {
        Object o = UNSAFE.getObject(e, COL_OFFSET);
        return o instanceof InsideBlockEffectApplier.StepBasedCollector c ? c : null;
    }

    /**
     * Ретаргет-точка первого invokevirtual checkInsideBlocks(List,Collector).
     * return false => ванильное тело пропущено (мост уже обслужил тик);
     * return e.isAffectedByBlocks() => ванильное тело как обычно.
     *
     * СТАТИК-ДЕТЕКТОР (leg #2, урок leg #1 35282003292): xo==x,yo==y,zo==z
     * (позиция начала тика == текущей ⇒ movement from==to==pos; deltaMovement
     * НЕ годится — покоящиеся предметы несут ненулевой гравитационный
     * остаток). Пустой слот + статик ⇒ MIRROR (bootstrap capture) — в leg #1
     * capture был недостижим (mirror только на инвалидации).
     */
    public static boolean gate(Entity e) {
        if (!ARMED) {
            return e.isAffectedByBlocks();
        }
        long eid = e.getId();
        int slot = (int) (eid & (NSLOTS - 1));
        // статик-детектор: движение тика отсутствует (from==to==текущая позиция)
        boolean staticTick = e.xo == e.getX() && e.yo == e.getY() && e.zo == e.getZ();
        double px = e.getX(), py = e.getY(), pz = e.getZ();
        InsideBlockEffectApplier.StepBasedCollector col = ARMED ? col(e) : null;
        if (col == null) {
            return e.isAffectedByBlocks(); // неожиданный тип collector'а — ваниль
        }
        if (SLOT_EID[slot] != eid) {
            // ПИНГ-ПОНГ ХАРДЕНИНГ (S7-136, урок leg #2 35284069355): capture
            // ТОЛЬКО из ПУСТОГО слота. При 150k живых > 2^17 слотов ~19k пар
            // eid делят слот; прежний перехват занятого слота mirror'ом делал
            // вечный ping-pong пары (A capture ⇒ B перехват ⇒ A перехват ...:
            // полная traversal + Recorder + 6 arraycopy card-marks КАЖДЫЙ ТИК
            // у обеих, ноль HIT) — главный подозреваемый коллапса leg2.
            // Слот, занятый чужим eid (в т.ч. мёртвым), НЕ трогаем: чистая
            // ваниль без инвалидации; ~13% сущностей остаются ванильными
            // (150000>131072) — допустимо по preregistered плану S7-135b.
            if (SLOT_EID[slot] == 0 && staticTick && e.isAlive() && e.isAffectedByBlocks()) {
                mirror(e, e.level(), col, px, py, pz, slot, eid);
                return false;
            }
            return e.isAffectedByBlocks();
        }
        if (!staticTick
                || SLOT_FX[slot] != Double.doubleToRawLongBits(px)
                || SLOT_FY[slot] != Double.doubleToRawLongBits(py)
                || SLOT_FZ[slot] != Double.doubleToRawLongBits(pz)) {
            SLOT_EID[slot] = 0; // движение/позиция изменились — сброс, ваниль
            return e.isAffectedByBlocks();
        }
        if (!e.isAlive() || !e.isAffectedByBlocks()) {
            return e.isAffectedByBlocks();
        }
        Level level = e.level();
        // HIT-верификация всех visited-позиций
        int nv = SLOT_NVIS[slot];
        int base = slot * MAXVIS;
        for (int i = 0; i < nv; i++) {
            BlockPos.MutableBlockPos mp = EntityQueryOps.mutablePos();
            mp.set(BlockPos.getX(VIS_POS[base + i]), BlockPos.getY(VIS_POS[base + i]), BlockPos.getZ(VIS_POS[base + i]));
            BlockState st = level.getBlockState(mp);
            if (Block.getId(st) != VIS_STATE[base + i]) {
                SLOT_EID[slot] = 0; // инвалидация
                mirror(e, level, col, px, py, pz, slot, eid); // re-discover + apply + capture
                return false;
            }
        }
        // REPLAY: свежие вызовы ванильной логики эффектов в visit-порядке
        int ne = SLOT_NEFF[slot];
        int ebase = slot * MAXVIS;
        for (int i = 0; i < ne; i++) {
            BlockPos bp = BlockPos.of(EFF_POS[ebase + i]);
            BlockState st = level.getBlockState(bp);
            int flag = EFF_FLAG[ebase + i];
            if ((flag & FLAG_BLOCK) != 0) {
                col.advanceStep(EFF_STEP[ebase + i], bp);
                st.entityInside(level, bp, e, col, (flag & FLAG_INTERSECTED) != 0);
                e.onInsideBlock(st);
            }
            if ((flag & FLAG_FLUID) != 0) {
                col.advanceStep(EFF_STEP[ebase + i], bp);
                st.getFluidState().entityInside(level, bp, e, col);
            }
        }
        return false; // discovery пропущен; applyAndClear делает ванильный вызыватель
    }

    /**
     * Mirror: однопроходная traversal + эффекты + capture одним кодом.
     * Эффекты применяются для ВСЕХ позиций независимо от кэш-лимитов
     * (parity не зависит от кэша); запись — пока влезает в слот.
     */
    static void mirror(Entity e, Level level, InsideBlockEffectApplier.StepBasedCollector col,
                       double px, double py, double pz, int slot, long eid) {
        SLOT_EID[slot] = 0; // до успеха — пусто (fail-closed)
        if (!e.isAlive() || !e.isAffectedByBlocks()) {
            return;
        }
        Vec3 pos = new Vec3(px, py, pz);
        AABB box = e.makeBoundingBox(pos).deflate(DEFLATE);
        Recorder rec = new Recorder(e, level, col, pos, box);
        BlockGetter.forEachBlockIntersectedBetween(pos, pos, box, rec);
        int nv = rec.n;
        if (nv <= 0 || nv > MAXVIS || rec.ne > MAXVIS) {
            return; // не кэшируем (эффекты уже применены — parity цело)
        }
        // коммит слота
        int base = slot * MAXVIS;
        System.arraycopy(rec.vpos, 0, VIS_POS, base, nv);
        System.arraycopy(rec.vstate, 0, VIS_STATE, base, nv);
        System.arraycopy(rec.epos, 0, EFF_POS, base, nv);
        System.arraycopy(rec.estate, 0, EFF_STATE, base, nv);
        System.arraycopy(rec.estep, 0, EFF_STEP, base, nv);
        System.arraycopy(rec.eflag, 0, EFF_FLAG, base, nv);
        SLOT_FX[slot] = Double.doubleToRawLongBits(px);
        SLOT_FY[slot] = Double.doubleToRawLongBits(py);
        SLOT_FZ[slot] = Double.doubleToRawLongBits(pz);
        SLOT_NVIS[slot] = nv;
        SLOT_NEFF[slot] = rec.ne;
        SLOT_EID[slot] = eid;
    }

    /** Visit-рекордер + исполнитель эффектов (зеркало на capture-тике). */
    private static final class Recorder implements BlockGetter.BlockStepVisitor {
        final Entity e;
        final Level level;
        final InsideBlockEffectApplier.StepBasedCollector col;
        final Vec3 pos;
        final AABB box;
        final long[] vpos = new long[MAXVIS];
        final int[] vstate = new int[MAXVIS];
        final long[] epos = new long[MAXVIS];
        final int[] estate = new int[MAXVIS];
        final int[] estep = new int[MAXVIS];
        final int[] eflag = new int[MAXVIS];
        int n;
        int ne;

        Recorder(Entity e, Level level, InsideBlockEffectApplier.StepBasedCollector col,
                 Vec3 pos, AABB box) {
            this.e = e;
            this.level = level;
            this.col = col;
            this.pos = pos;
            this.box = box;
        }

        @Override
        public boolean visit(BlockPos bp, int step) {
            if (step >= MAXSTEPS) {
                return false; // зеркало бюджета визитора (9-16)
            }
            if (!e.isAlive()) {
                return false; // зеркало гейта визитора (0-8)
            }
            BlockState st = level.getBlockState(bp);
            long p = bp.asLong();
            if (n < MAXVIS) {
                vpos[n] = p;
                vstate[n] = Block.getId(st);
            }
            n++;
            if (st.isAir()) {
                return true; // визитор: air — без эффектов (debug-ветка пуста на сцене)
            }
            // hitShape (контракт визитора 80-121)
            var shape = st.getEntityInsideCollisionShape(level, bp, e);
            boolean hs = shape == Shapes.block()
                    || e.collidedWithShapeMovingFrom(pos, pos, shape.move(new Vec3(bp)).toAabbs());
            // inFluid (контракт визитора 123-135 — вычисляется всегда)
            boolean infl = e.collidedWithFluid(st.getFluidState(), bp, pos, pos);
            if (!hs && !infl) {
                return true; // не-effectful (контракт 140-147)
            }
            int flag = 0;
            if (hs) {
                flag |= FLAG_BLOCK;
                // intersected = hasMoved || box.intersects(pos); hasMoved=false (статика)
                if (box.intersects(bp)) {
                    flag |= FLAG_INTERSECTED;
                }
            }
            if (infl) {
                flag |= FLAG_FLUID;
            }
            // применяем немедленно — порядок = visit-порядок (контракт 192-315)
            if (hs) {
                col.advanceStep(step, bp);
                st.entityInside(level, bp, e, col, (flag & FLAG_INTERSECTED) != 0);
                e.onInsideBlock(st);
            }
            if (infl) {
                col.advanceStep(step, bp);
                st.getFluidState().entityInside(level, bp, e, col);
            }
            if (ne < MAXVIS) {
                epos[ne] = p;
                estate[ne] = Block.getId(st);
                estep[ne] = step;
                eflag[ne] = flag;
                ne++;
            }
            return true;
        }
    }
}
