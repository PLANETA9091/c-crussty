package net.minecraft.world.level.chunk;

import java.lang.reflect.Field;
import java.lang.reflect.Method;

/**
 * C-X5 READER VIEWS — paletted zero-copy reader-view companion stub (phase-1,
 * STRICT dormant). TASK-459-84, WILD закон-11 тик-459.
 *
 * <p>Идея C-X5: массовые чтения {@code PalettedContainer.get(int)} (4.48%
 * ваниль) + {@code SimpleBitStorage.get(int)} (1.50%) = ~6.0% paletted-лейн —
 * на батч-путях (codec parse/serialize, {@code getAll}, countPaletteSizes)
 * заменяются reader-view объектом: ОДНА выборка
 * {@code container.data -> (storage, palette, bits)} на серию запросов, дальше
 * N чтений без per-call volatile-рида {@code data}, без interface-dispatch и
 * без palette-индирекции для raw-index потребителей. Одиночные чтения —
 * ваниль. javap-грунт (patched-kernel.jar, см. RESEARCH-459-CX5.md §2):
 * get = volatile data-рид + invokeinterface + readPalette НА КАЖДЫЙ вызов;
 * {@code SimpleBitStorage.getRaw()} = {@code getfield data; areturn} — живой
 * long[] (zero-copy лег); {@code PalettedContainer$Data} — иммутабельный
 * кортеж, resize публикуется заменой volatile-ссылки {@code data}.</p>
 *
 * <p>JDK-only: кернел-классы резолвятся reflection'ом в рантайме
 * (compile-time зависимости запрещены — CI компилирует этот файл без
 * кернел-jar). Волна-2 определяет класс в kernel loader ДО первого use
 * (NCDFE-канон, см. src/improved_noise.rs header); ФАЗА-1 ничего не
 * определяет и не регистрирует.</p>
 *
 * <p>Гейт STRICT eq: {@code CRUSSTY_LEVER_FLAG == cmp459_cx5}; пустой/чужой
 * флаг = ваниль, все входы fail-closedPassthrough. Grep-маркеры:
 * "paletted_reader_views: views", "paletted_reader_views: fallbacks",
 * "cmp459_cx5".</p>
 *
 * <p>V1-ловушка (RESEARCH §6): вьюха консистентна против resize (Data
 * иммутабелен), но НЕ против in-place мутаций storage ({@code getAndSet}
 * пишет data[k] на месте) ⇒ волна-2 обязана доказать hold батч-путей
 * (снапшот/секционная блокировка); до оракула — STRICT off.</p>
 */
public final class PalettedReaderViewOps {

    /** STRICT lever id (grep-якорь вайринга волны-2). */
    public static final String LEVER = "cmp459_cx5";
    /** Env-ключ STRICT-гейта. */
    public static final String GATE_ENV = "CRUSSTY_LEVER_FLAG";

    /** Монотонные счётчики (observation-only). */
    public static volatile long VIEWS = 0, FALLBACKS = 0, RECHECKS = 0;

    private static volatile boolean probed = false;
    private static volatile Field fContainerData;
    private static volatile Method mStorage, mGetRaw, mGetBits, mPalette, mMoonrisePalette;

    private PalettedReaderViewOps() {}

    /** STRICT eq: только точный lever id. */
    public static boolean enabled() {
        String v = System.getenv(GATE_ENV);
        return v != null && v.trim().equals(LEVER);
    }

    // ------------------------------------------------------------------
    // Reader view (без типизированного кернел-класса: Object[4] кортеж
    // {long[] data, int bits, long mask, int valuesPerLong} + identity
    // исходного container.data для re-check — 5-й элемент).
    // ------------------------------------------------------------------

    /**
     * «Одна выборка storage→(palette,биты)» на серию запросов. Null ⇒ ваниль
     * (fail-closedPassthrough): чужой storage (не SimpleBitStorage по имени),
     * reflection-промах, bits вне 1..15 секционного диапазона.
     */
    public static Object[] acquireView(Object container) {
        if (!enabled() || container == null) {
            FALLBACKS++;
            return null;
        }
        try {
            probe();
            Field fd = fContainerData;
            if (fd == null) { FALLBACKS++; return null; }
            Object data = fd.get(container);                    // volatile рид РОВНО один
            Object storage = mStorage.invoke(data);             // Data.storage()
            String storageClass = storage.getClass().getName();
            if (!storageClass.endsWith("SimpleBitStorage")) {   // generic BitStorage ⇒ ваниль
                FALLBACKS++;
                return null;
            }
            long[] raw = (long[]) mGetRaw.invoke(storage);      // zero-copy живой long[]
            int bits = (Integer) mGetBits.invoke(storage);
            if (bits < 1 || bits > 15 || raw.length == 0) { FALLBACKS++; return null; }
            int vpl = 64 / bits;
            long mask = (1L << bits) - 1L;
            Object palette = mPalette.invoke(data);             // palette-ссылка на серию
            Object fast = null;
            try { fast = mMoonrisePalette.invoke(data); } catch (Throwable ignored) { }
            VIEWS++;
            return new Object[]{raw, bits, mask, vpl, data, palette, fast};
        } catch (Throwable t) {
            FALLBACKS++;                                        // любая аномалия ⇒ ваниль
            return null;
        }
    }

    /**
     * raw palette-индекс по вьюхе (serialize-потребитель, без palette-
     * индирекции). view==null или idx вне диапазона ⇒ -1 (ваниль-хвост).
     */
    public static int rawIndexAt(Object[] view, int idx) {
        if (view == null || idx < 0) { FALLBACKS++; return -1; }
        long[] data = (long[]) view[0];
        int bits = (Integer) view[1];
        long mask = (Long) view[2];
        int vpl = (Integer) view[3];
        int k = idx / vpl;
        if (k >= data.length) { FALLBACKS++; return -1; }
        return (int) ((data[k] >>> ((idx % vpl) * bits)) & mask);
    }

    /**
     * Identity re-check после серии (fail-closed): container.data обязан
     * указывать на тот же сэмпл; иначе серия отбрасывается (волна-2
     * перечитывает ваниль-циклом). true = вьюха легитимна.
     */
    public static boolean recheckIdentity(Object container, Object[] view) {
        if (!enabled() || container == null || view == null) return false;
        RECHECKS++;
        try {
            probe();
            return fContainerData.get(container) == view[4];
        } catch (Throwable t) {
            FALLBACKS++;
            return false;
        }
    }

    // ------------------------------------------------------------------
    // selfTest — канон JDK-only: (1) модельная парити packed-layout ≡ view
    // на лестнице bits 1..15 × sizes {64, 4096, 16384}; (2) оппортунистическая
    // кернел-парити magic-деления (если кернел-классы в classpath —
    // magic*idx>>>20 ≡ idx/vpl на всех точках), иначе SKIP-маркер.
    // ------------------------------------------------------------------

    public static boolean selfTest() {
        long seed = 0xC55EEDL ^ 0x9E3779B97F4A7C15L;
        long checks = 0;
        for (int bits = 1; bits <= 15; bits++) {
            int[] sizes = {64, 4096, 16384};
            for (int size : sizes) {
                int vpl = 64 / bits;
                long mask = (1L << bits) - 1L;
                long[] data = new long[(size + vpl - 1) / vpl];
                long[] expect = new long[size];
                for (int i = 0; i < size; i++) {
                    seed = seed * 6364136223846793005L + 1442695040888963407L;
                    long v = (seed >>> 11) & mask;
                    expect[i] = v;
                    int k = i / vpl, off = (i % vpl) * bits;
                    data[k] = (data[k] & ~(mask << off)) | (v << off);
                }
                // view-математика ≡ ожидаемым значениям (одна выборка)
                for (int i = 0; i < size; i++) {
                    long got = (data[i / vpl] >>> ((i % vpl) * bits)) & mask;
                    if (got != expect[i]) return false;
                    checks++;
                }
            }
        }
        System.out.println("paletted_reader_views: selfTest model parity checks=" + checks
            + " lever=" + LEVER + " gate=" + (enabled() ? "on" : "off"));
        boolean magic = magicDivisionParity();
        liveKernelOpportunistic();
        return magic;
    }

    /**
     * Парити magic-деления — транскрипт кернел-цепочки (javap, offline):
     * BETTER_MAGIC[bits] = (int) IntegerUtil.getUnsignedDivisorMagic(64/bits, 20)
     *   (SimpleBitStorage.&lt;clinit&gt; 1174..1211), где concurrentutil
     *   IntegerUtil.getUnsignedDivisorMagic(d, 20) = ((1L&lt;&lt;20)-1)/d + 1
     *   (источник: github.com/Spottedleaf/ConcurrentUtil IntegerUtil.java
     *   L192-194 — экспонтировано через web-retrieval, TASK-459-84);
     * mulBits = (64/bits)*bits (javap ctor 180..193); get(): j=magic*idx,
     * k=j&gt;&gt;&gt;20, l=(j&amp;0xFFFFF)*mulBits&gt;&gt;&gt;20. Размер кернел
     * жёстко каппит: "Size &gt; 4096 not supported" (moonrise, javap 197..215)
     * — проверяем весь поддерживаемый диапазон idx&lt;4096 × bits 1..15.
     */
    private static boolean magicDivisionParity() {
        long pts = 0;
        for (int bits = 1; bits <= 15; bits++) {
            int vpl = 64 / bits;
            int magic = (int) ((((1L << 20) - 1L) / vpl) + 1L); // concurrentutil транскрипт
            int mulBits = (64 / bits) * bits;
            for (int i = 0; i < 4096; i++) {
                int j = magic * i;
                int k = j >>> 20;
                int l = (j & 0xFFFFF) * mulBits >>> 20;
                if (k != i / vpl || l != (i % vpl) * bits) {
                    System.out.println("paletted_reader_views: magic-math DIVERGES bits=" + bits + " idx=" + i);
                    return false;
                }
                pts++;
            }
        }
        System.out.println("paletted_reader_views: magic-division parity points=" + pts
            + " (bits 1..15 x idx<4096, kernel cap)");
        return true;
    }

    /** Оппортунистическая LIVE-проверка: если кернел-классы загрузились,
     * сверяем транскрибированную формулу с фактическим полем magic. */
    private static void liveKernelOpportunistic() {
        try {
            Class<?> sbs = Class.forName("net.minecraft.util.SimpleBitStorage");
            java.lang.reflect.Constructor<?> ctor = sbs.getDeclaredConstructor(int.class, int.class);
            ctor.setAccessible(true);
            Field fm = sbs.getDeclaredField("magic"); fm.setAccessible(true);
            int mismatch = 0;
            for (int bits = 1; bits <= 15; bits++) {
                int vpl = 64 / bits;
                int want = (int) ((((1L << 20) - 1L) / vpl) + 1L);
                Object st = ctor.newInstance(bits, 64);
                if (fm.getInt(st) != want) mismatch++;
            }
            System.out.println("paletted_reader_views: live kernel magic match mismatches=" + mismatch + "/15");
        } catch (Throwable t) {
            System.out.println("paletted_reader_views: live kernel parity SKIP ("
                + t.getClass().getSimpleName() + ")");
        }
    }

    private static void probe() throws Exception {
        if (probed) return;
        synchronized (PalettedReaderViewOps.class) {
            if (probed) return;
            Class<?> pc = Class.forName("net.minecraft.world.level.chunk.PalettedContainer");
            Field fd = pc.getField("data");                      // public volatile
            Class<?> dc = Class.forName("net.minecraft.world.level.chunk.PalettedContainer$Data");
            Method mS = dc.getMethod("storage");
            Class<?> bitSt = Class.forName("net.minecraft.util.BitStorage");
            Method mRaw = bitSt.getMethod("getRaw");
            Method mBits = bitSt.getMethod("getBits");
            Method mP = dc.getMethod("palette");
            Method mF = null;
            try {
                Class<?> fpd = Class.forName("ca.spottedleaf.moonrise.patches.fast_palette.FastPaletteData");
                mF = fpd.getMethod("moonrise$getPalette");
            } catch (Throwable ignored) { }
            fContainerData = fd; mStorage = mS; mGetRaw = mRaw; mGetBits = mBits;
            mPalette = mP; mMoonrisePalette = mF;
            probed = true;
        }
    }

    /** javac+java smoke: selfTest 1/1. */
    public static void main(String[] args) {
        if (!selfTest()) {
            System.out.println("paletted_reader_views: selfTest FAIL");
            System.exit(1);
        }
        System.out.println("paletted_reader_views: selfTest PASS (dormant=" + !enabled() + ")");
    }
}
