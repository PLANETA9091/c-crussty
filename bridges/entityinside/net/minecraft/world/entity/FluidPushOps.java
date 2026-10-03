package net.minecraft.world.entity;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicLong;
import net.minecraft.core.BlockPos;
import net.minecraft.tags.FluidTags;
import net.minecraft.tags.TagKey;
import net.minecraft.util.Mth;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.chunk.ChunkAccess;
import net.minecraft.world.level.chunk.LevelChunkSection;
import net.minecraft.world.level.chunk.status.ChunkStatus;
import net.minecraft.world.level.material.Fluid;
import net.minecraft.world.level.material.FluidState;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * ARCH-ATTACK lever S7-151 / TASK-290 — FLUID-DIRTY: per-entity memoization
 * of the Entity.updateFluidHeightAndDoFluidPushing SCAN portion
 * (event-driven dirty-stamp ledger instead of per-tick polling).
 *
 * CENSUS (S7-150 RECON, CUMULATIVE 35330129145 — валидная зелёная база):
 * fluid-push family ≈ 10% total CPU (36.4% лейна ItemEntity.tick 23.43% CPU
 * при 100k item-сущностей + 9.8% лейна Zombie). Скан = ЧИСТАЯ ФУНКЦИЯ
 * (span, fluid-состояния блоков span, isPushedByFluid) — javap-контракт
 * research/fluid-dirty-2026-09-18/step0_*.txt. Постобработка (put/пуш)
 * dm-зависима — исполняется всегда, мемоизируется ТОЛЬКО скан.
 *
 * ТОЧКИ ВМЕШАТЕЛЬСТВА (оба 3B→3B receiver-first, длина сохранена):
 *  1) ОБОИХ invokevirtual Entity.updateFluidHeightAndDoFluidPushing(TagKey;D)Z
 *     (единственные вызовы во всём kernel — census S7151_CENSUS.md):
 *     вода (updateInWaterStateAndDoWaterCurrentPushing:39) и лава
 *     (updateInWaterStateAndDoFluidPushing:41) → invokestatic
 *     FluidPushOps.scan(Entity,TagKey,double)Z. Тело ванильного метода НЕ
 *     трогается — miss-путь зовёт его обычным вызовом (рекурсии нет).
 *  2) Единственный сайт LevelChunkSection.setBlockState(IIILBlockState)BlockState
 *     в LevelChunk.setBlockState(BlockPos,BlockState,I)BlockState (offset 73)
 *     → invokestatic FluidPushOps.secWrite(...) — делегат: возвращает секции
 *     старое состояние (континуация LevelChunk не отличается), ref-compare
 *     FluidState (синглтоны) при реальном изменении жидкости инкрементирует
 *     штамп секции (event-driven инвалидация; fluid-tick/explosion/waterlog
 *     покрыты — всё идёт через setBlockState; обычные блоки штамп не дёргают).
 *
 * HIT-условие (median-exact parity): span бит-в-бит (клэмп-границыяют как
 * ваниль), isPushedByFluid равен, ссылки секций span те же И штампы секций
 * не изменились ⇒ входы скана идентичны ⇒ ванильный скан вернул бы бит-в-бит
 * тот же результат ⇒ пропуск = ванилла. Движущиеся сущности (span меняется)
 * сканируются каждый тик как ванилла; покоящиеся (масса 100k items) пропускают
 * скан. Статичная вода не мутирует ⇒ штампы не растут.
 *
 * КЭШ: плоские примитивные массивы (InsideBlockOps-дисциплина: ноль записей
 * на HIT), ДВА набора слотов (WATER/LAVA — иначе вода/лава вымывали бы друг
 * друга каждый тик), слот = entityId & (N-1), штамп = entityId (0 = пусто).
 * Модифицированные fluid-теги (не WATER/LAVA) обслуживаются чистой ваниллой.
 * Capture в свой слот (слот чужого eid не трогаем — ping-pong hardening
 * S7-136).
 *
 * FAIL-DOMINANT: любое исключение моста → ванильный вызов (никогда не
 * false-hit: ложный miss всегда безопасен). INJECTS-ONLY: класс определяется
 * в kernel loader рантайм-wiring'ом (src/fluid_dirty.rs).
 */
public final class FluidPushOps {

    private FluidPushOps() {}

    // ---------- SLOT-ХРАНИЛИЩЕ (два набора: вода / лава) ----------

    static final int NSLOTS = 1 << 17; // 131072 слотов на тег
    static final int MAXSEC = 8;       // максимум секций span в кэше (сверх — не кэшируем)

    static final long[] W_EID = new long[NSLOTS];
    static final long[] L_EID = new long[NSLOTS];

    static final int[] W_X0 = new int[NSLOTS], W_X1 = new int[NSLOTS],
            W_Y0 = new int[NSLOTS], W_Y1 = new int[NSLOTS],
            W_Z0 = new int[NSLOTS], W_Z1 = new int[NSLOTS];
    static final int[] L_X0 = new int[NSLOTS], L_X1 = new int[NSLOTS],
            L_Y0 = new int[NSLOTS], L_Y1 = new int[NSLOTS],
            L_Z0 = new int[NSLOTS], L_Z1 = new int[NSLOTS];
    static final boolean[] W_PUSH = new boolean[NSLOTS];
    static final boolean[] L_PUSH = new boolean[NSLOTS];
    static final double[] W_HACC = new double[NSLOTS], L_HACC = new double[NSLOTS];
    static final boolean[] W_HIT = new boolean[NSLOTS], L_HIT = new boolean[NSLOTS];
    static final boolean[] W_TOUCH = new boolean[NSLOTS], L_TOUCH = new boolean[NSLOTS];
    static final double[] W_FX = new double[NSLOTS], W_FY = new double[NSLOTS], W_FZ = new double[NSLOTS];
    static final double[] L_FX = new double[NSLOTS], L_FY = new double[NSLOTS], L_FZ = new double[NSLOTS];
    static final double[] W_FN = new double[NSLOTS], L_FN = new double[NSLOTS];
    static final BlockPos[] W_LAVA = new BlockPos[NSLOTS];
    static final BlockPos[] L_LAVA = new BlockPos[NSLOTS];
    static final LevelChunkSection[] W_SEC = new LevelChunkSection[NSLOTS * MAXSEC];
    static final LevelChunkSection[] L_SEC = new LevelChunkSection[NSLOTS * MAXSEC];
    static final long[] W_STAMP = new long[NSLOTS * MAXSEC];
    static final long[] L_STAMP = new long[NSLOTS * MAXSEC];
    static final int[] W_NSEC = new int[NSLOTS], L_NSEC = new int[NSLOTS];

    // ---------- LEDGER (event-driven dirty stamps по секциям) ----------

    static final ConcurrentHashMap<LevelChunkSection, AtomicLong> LEDGER =
            new ConcurrentHashMap<>();

    public static void bump(LevelChunkSection s) {
        LEDGER.computeIfAbsent(s, k -> new AtomicLong()).incrementAndGet();
    }

    /** Диагностика харнесса/бенча. */
    public static long stampOf(LevelChunkSection s) {
        AtomicLong a = LEDGER.get(s);
        return a == null ? 0L : a.get();
    }

    // ---------- СТАТИСТИКА ----------

    static final AtomicLong STAT_HIT = new AtomicLong();
    static final AtomicLong STAT_MISS = new AtomicLong();
    static final AtomicLong STAT_VANILLA = new AtomicLong();

    public static long hits() { return STAT_HIT.get(); }
    public static long misses() { return STAT_MISS.get(); }
    public static long vanillas() { return STAT_VANILLA.get(); }

    /** Резолв поверхности (харнесс): без Unsafe — мост полностью в пакете. */
    private static volatile boolean ARMED = false;

    public static void arm() {
        ARMED = true;
    }

    public static boolean armed() {
        return ARMED;
    }

    // ---------- РЕТАРГЕТ-ТОЧКА СКАНА ----------

    /**
     * Оба wrapper-сайта указывают сюда. Descriptor:
     * (LEntity;LTagKey;D)Z — receiver-first, 3B invokestatic.
     */
    public static boolean scan(Entity e, TagKey<Fluid> tag, double motionScale) {
        if (!ARMED) {
            return vanilla(e, tag, motionScale);
        }
        try {
            return scanArmed(e, tag, motionScale);
        } catch (Throwable t) {
            return vanilla(e, tag, motionScale); // fail-dominant
        }
    }

    private static boolean vanilla(Entity e, TagKey<Fluid> tag, double motionScale) {
        STAT_VANILLA.incrementAndGet();
        return e.updateFluidHeightAndDoFluidPushing(tag, motionScale);
    }

    private static boolean scanArmed(Entity e, TagKey<Fluid> tag, double motionScale) {
        final boolean water = tag == FluidTags.WATER;
        final boolean lava = tag == FluidTags.LAVA;
        if (!water && !lava) {
            return vanilla(e, tag, motionScale); // модифицированные теги — чистая ванилла
        }
        final long[] EID = water ? W_EID : L_EID;

        // Ванильный guard ДО всего (ваниль не пишет fluidHeight при unloaded)
        if (e.touchingUnloadedChunk()) {
            return false;
        }

        final long eid = e.getId();
        final int slot = (int) (eid & (NSLOTS - 1));
        final boolean pushedByFluid = e.isPushedByFluid();

        // span = клэмп-границыяют ванильного скана (бит-в-бит та же математика)
        AABB box = e.getBoundingBox().deflate(0.001);
        Level level = e.level();
        final int minSec = level.getMinSectionY();
        final int maxSec = minSec + level.getSectionsCount() - 1;
        final int x0 = Mth.floor(box.minX);
        final int y0 = Math.max(minSec << 4, Mth.floor(box.minY));
        final int z0 = Mth.floor(box.minZ);
        final int x1 = Mth.ceil(box.maxX) - 1;
        final int y1 = Math.min((maxSec << 4) | 15, Mth.ceil(box.maxY) - 1);
        final int z1 = Mth.ceil(box.maxZ) - 1;

        // Секции span + текущие штампы (детерминированный порядок cz→cx→sy)
        final int cx0 = x0 >> 4, cx1 = x1 >> 4, cz0 = z0 >> 4, cz1 = z1 >> 4;
        final int syLo = Math.max(y0 >> 4, minSec);
        final int syHi = Math.min(y1 >> 4, maxSec);
        int nsec = 0;
        LevelChunkSection[] curSec = SEC_TLS.get();
        long[] curStamp = STAMP_TLS.get();
        for (int cz = cz0; cz <= cz1; cz++) {
            for (int cx = cx0; cx <= cx1; cx++) {
                ChunkAccess ch = level.getChunk(cx, cz, ChunkStatus.FULL, false);
                if (ch == null) {
                    return false; // guard прошёл, чанк пропал — теоретический рейс; без put как ванилла-guard
                }
                LevelChunkSection[] secs = ch.getSections();
                for (int sy = syLo; sy <= syHi; sy++) {
                    int idx = sy - minSec;
                    if (idx < 0 || idx >= secs.length) {
                        continue;
                    }
                    LevelChunkSection s = secs[idx];
                    if (s == null) {
                        continue;
                    }
                    if (nsec < MAXSEC) {
                        curSec[nsec] = s;
                        AtomicLong a = LEDGER.get(s);
                        curStamp[nsec] = a == null ? 0L : a.get();
                    }
                    nsec++;
                }
            }
        }
        final boolean cacheable = nsec <= MAXSEC;

        // ---- HIT-проверка ----
        if (cacheable && EID[slot] == eid) {
            final int[] X0 = water ? W_X0 : L_X0, X1 = water ? W_X1 : L_X1;
            final int[] Y0 = water ? W_Y0 : L_Y0, Y1 = water ? W_Y1 : L_Y1;
            final int[] Z0 = water ? W_Z0 : L_Z0, Z1 = water ? W_Z1 : L_Z1;
            final boolean[] PUSH = water ? W_PUSH : L_PUSH;
            final LevelChunkSection[] SEC = water ? W_SEC : L_SEC;
            final long[] ST = water ? W_STAMP : L_STAMP;
            final int[] NSEC = water ? W_NSEC : L_NSEC;
            if (X0[slot] == x0 && X1[slot] == x1 && Y0[slot] == y0 && Y1[slot] == y1
                    && Z0[slot] == z0 && Z1[slot] == z1 && PUSH[slot] == pushedByFluid
                    && NSEC[slot] == nsec) {
                boolean stampsOk = true;
                final int base = slot * MAXSEC;
                for (int i = 0; i < nsec; i++) {
                    if (SEC[base + i] != curSec[i] || ST[base + i] != curStamp[i]) {
                        stampsOk = false;
                        break;
                    }
                }
                if (stampsOk) {
                    STAT_HIT.incrementAndGet();
                    return postprocess(e, tag, motionScale, slot, water);
                }
            }
        }

        // ---- MISS: реимплементация ванильного скана + capture ----
        STAT_MISS.incrementAndGet();
        final ScanOut out = OUT_TLS.get();
        out.lavaPos = null;
        boolean hitFlag = mirrorScan(e, level, box, x0, y0, z0, x1, y1, z1,
                tag, pushedByFluid, out);

        // capture (только в свой слот; чужой не трогаем — ping-pong hardening)
        if (cacheable && (EID[slot] == 0 || EID[slot] == eid)) {
            commit(water, slot, eid, x0, x1, y0, y1, z0, z1, pushedByFluid,
                    nsec, curSec, curStamp, out);
        }
        return postprocess2(e, tag, motionScale, out);
    }

    private static final ThreadLocal<LevelChunkSection[]> SEC_TLS =
            ThreadLocal.withInitial(() -> new LevelChunkSection[MAXSEC]);
    private static final ThreadLocal<long[]> STAMP_TLS =
            ThreadLocal.withInitial(() -> new long[MAXSEC]);
    private static final ThreadLocal<ScanOut> OUT_TLS =
            ThreadLocal.withInitial(ScanOut::new);
    private static final ThreadLocal<BlockPos.MutableBlockPos> MP_TLS =
            ThreadLocal.withInitial(BlockPos.MutableBlockPos::new);

    /** Capture-структура скана (плоские поля, ThreadLocal — ноль аллокаций). */
    private static final class ScanOut {
        double hAcc;
        boolean hitFlag;
        boolean flowTouched;
        double fx, fy, fz, fn;
        BlockPos lavaPos;
    }

    private static void commit(boolean water, int slot, long eid,
            int x0, int x1, int y0, int y1, int z0, int z1, boolean push,
            int nsec, LevelChunkSection[] secs, long[] stamps, ScanOut out) {
        final long[] EID = water ? W_EID : L_EID;
        final int[] X0 = water ? W_X0 : L_X0, X1 = water ? W_X1 : L_X1;
        final int[] Y0 = water ? W_Y0 : L_Y0, Y1 = water ? W_Y1 : L_Y1;
        final int[] Z0 = water ? W_Z0 : L_Z0, Z1 = water ? W_Z1 : L_Z1;
        final boolean[] PUSH = water ? W_PUSH : L_PUSH;
        final double[] HACC = water ? W_HACC : L_HACC;
        final boolean[] HIT = water ? W_HIT : L_HIT;
        final boolean[] TOUCH = water ? W_TOUCH : L_TOUCH;
        final double[] FX = water ? W_FX : L_FX, FY = water ? W_FY : L_FY, FZ = water ? W_FZ : L_FZ;
        final double[] FN = water ? W_FN : L_FN;
        final BlockPos[] LAVA = water ? W_LAVA : L_LAVA;
        final LevelChunkSection[] SEC = water ? W_SEC : L_SEC;
        final long[] ST = water ? W_STAMP : L_STAMP;
        final int[] NSEC = water ? W_NSEC : L_NSEC;
        final int base = slot * MAXSEC;
        // порядок: данные, потом eid (publish как InsideBlockOps)
        X0[slot] = x0; X1[slot] = x1; Y0[slot] = y0; Y1[slot] = y1; Z0[slot] = z0; Z1[slot] = z1;
        PUSH[slot] = push;
        HACC[slot] = out.hAcc;
        HIT[slot] = out.hitFlag;
        TOUCH[slot] = out.flowTouched;
        FX[slot] = out.fx; FY[slot] = out.fy; FZ[slot] = out.fz;
        FN[slot] = out.fn;
        LAVA[slot] = out.lavaPos;
        for (int i = 0; i < nsec; i++) {
            SEC[base + i] = secs[i];
            ST[base + i] = stamps[i];
        }
        NSEC[slot] = nsec;
        EID[slot] = eid;
    }

    /**
     * Реимплементация ванильного скана (javap-контракт step0, шаги 2-7):
     * порядок обхода x→y→z и все float/double-преобразования бит-в-бит;
     * пред-фетч матрицы секций по чанкам — как ваниль.
     * Побочный эффект скана: lastLavaContact — пишется один раз ПОСЛЕ скана
     * (финальное значение = последний хит — совпадает с ванилью).
     */
    private static boolean mirrorScan(Entity e, Level level, AABB aabb,
            int x0, int y0, int z0, int x1, int y1, int z1,
            TagKey<Fluid> tag, boolean pushedByFluid, ScanOut out) {
        final int minSec = level.getMinSectionY();
        final int maxSec = minSec + level.getSectionsCount() - 1;
        final int cx0 = x0 >> 4, cx1 = x1 >> 4, cz0 = z0 >> 4, cz1 = z1 >> 4;
        final int nCx = cx1 - cx0 + 1;
        final int nCz = cz1 - cz0 + 1;
        // ванильный пред-фетч: rows[(czIdx)*nCx + cxIdx] = chunk.getSections()
        final LevelChunkSection[][] rows = new LevelChunkSection[nCx * nCz][];
        for (int cz = cz0; cz <= cz1; cz++) {
            for (int cx = cx0; cx <= cx1; cx++) {
                ChunkAccess ch = level.getChunk(cx, cz, ChunkStatus.FULL, false);
                rows[(cz - cz0) * nCx + (cx - cx0)] = ch.getSections();
            }
        }
        BlockPos.MutableBlockPos mpos = MP_TLS.get();
        Vec3 flowAcc = Vec3.ZERO;
        double hAcc = 0.0;
        double flowN = 0.0;
        boolean hitFlag = false;
        final boolean isLava = tag == FluidTags.LAVA;
        for (int x = x0; x <= x1; x++) {
            for (int y = y0; y <= y1; y++) {
                final int sy = (y >> 4) - minSec;
                for (int z = z0; z <= z1; z++) {
                    LevelChunkSection[] row = rows[((z >> 4) - cz0) * nCx + ((x >> 4) - cx0)];
                    if (sy < 0 || sy >= row.length) {
                        continue; // недостижимо при клэмпе — защитный skip
                    }
                    LevelChunkSection sec = row[sy];
                    if (sec == null) {
                        continue;
                    }
                    BlockState st = sec.states.get(
                            (x & 15) | ((z & 15) << 4) | ((y & 15) << 8));
                    FluidState fs = st.getFluidState();
                    if (fs.isEmpty() || !fs.is(tag)) {
                        continue;
                    }
                    mpos.set(x, y, z);
                    if (isLava) {
                        out.lavaPos = mpos.immutable();
                    }
                    double d = (double) ((float) y + fs.getHeight(level, mpos));
                    double rel = d - aabb.minY;
                    if (!(rel >= 0.0)) {
                        continue; // dcmpg/ifge — NaN-семантика как ваниль
                    }
                    hitFlag = true;
                    hAcc = Math.max(hAcc, rel);
                    if (!pushedByFluid) {
                        continue;
                    }
                    flowN += 1.0;
                    Vec3 flow = fs.getFlow(level, mpos);
                    double sc = (hAcc >= 0.4) ? 1.0 : hAcc; // dcmpg/ifge — NaN-семантика как ваниль
                    flowAcc = flowAcc.add(flow.scale(sc));
                }
            }
        }
        out.hAcc = hAcc;
        out.hitFlag = hitFlag;
        out.flowTouched = flowAcc != Vec3.ZERO; // REF-identity как ваниль
        out.fx = flowAcc.x;
        out.fy = flowAcc.y;
        out.fz = flowAcc.z;
        out.fn = flowN;
        return hitFlag;
    }

    // ---------- ПОСТОБРАБОТКА (шаги 8-10 ванили, общая семантика miss/hit) ----------

    private static boolean postprocess2(Entity e, TagKey<Fluid> tag, double motionScale, ScanOut out) {
        e.fluidHeight.put(tag, out.hAcc);
        if (out.lavaPos != null && tag == FluidTags.LAVA) {
            e.lastLavaContact = out.lavaPos;
        }
        if (!out.flowTouched) {
            return out.hitFlag;
        }
        return pushTail(e, motionScale, out.fx, out.fy, out.fz, out.fn);
    }

    private static boolean postprocess(Entity e, TagKey<Fluid> tag, double motionScale, int slot, boolean water) {
        final double[] HACC = water ? W_HACC : L_HACC;
        final boolean[] HIT = water ? W_HIT : L_HIT;
        final boolean[] TOUCH = water ? W_TOUCH : L_TOUCH;
        final double[] FX = water ? W_FX : L_FX, FY = water ? W_FY : L_FY, FZ = water ? W_FZ : L_FZ;
        final double[] FN = water ? W_FN : L_FN;
        final BlockPos[] LAVA = water ? W_LAVA : L_LAVA;
        e.fluidHeight.put(tag, HACC[slot]);
        if (!water && LAVA[slot] != null) {
            e.lastLavaContact = LAVA[slot];
        }
        if (!TOUCH[slot]) {
            return HIT[slot];
        }
        return pushTail(e, motionScale, FX[slot], FY[slot], FZ[slot], FN[slot]);
    }

    /** Хвост пуша (ваниль 572-677): бит-в-бит та же последовательность операций. */
    private static boolean pushTail(Entity e, double motionScale,
            double fx, double fy, double fz, double fn) {
        Vec3 vec = new Vec3(fx, fy, fz); // бит-в-бит те же биты аккумулятора
        vec = vec.scale(1.0 / fn);
        if (!(e instanceof net.minecraft.world.entity.player.Player)) {
            vec = vec.normalize();
        }
        vec = vec.scale(motionScale);
        Vec3 dm = e.getDeltaMovement();
        if (Math.abs(dm.x) < 0.003 && Math.abs(dm.z) < 0.003 && vec.length() < 0.0045) {
            vec = vec.normalize().scale(0.0045);
        }
        e.setDeltaMovement(dm.add(vec));
        return true;
    }

    // ---------- РЕТАРГЕТ-ТОЧКА ЗАПИСИ СЕКЦИИ ----------

    /**
     * Делегат единственного сайта LevelChunk.setBlockState →
     * LevelChunkSection.setBlockState(IIILBlockState)BlockState.
     * Возвращает СТАРОЕ состояние (контракт ванили) — континуация LevelChunk
     * не отличается. Реальное изменение fluid-состояния (ref-compare —
     * FluidState-синглтоны) инкрементирует штамп секции.
     */
    public static BlockState secWrite(LevelChunkSection section, int x, int y, int z, BlockState newState) {
        BlockState old = section.setBlockState(x, y, z, newState);
        if (old.getFluidState() != newState.getFluidState()) {
            bump(section);
        }
        return old;
    }
}
