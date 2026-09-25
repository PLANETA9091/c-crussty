package net.minecraft.world.level.chunk.storage;

/**
 * CHUNK-SERIAL LRU DECOMPRESS-ARENA (TASK-459-82, идея C-X3 — wild-нога
 * закона 11, тик-459; STRICT dormant, lever cmp459_cx3 STRICT eq).
 *
 * Java-носитель C-X3: per-thread LRU-арена разжатых секций в Rust
 * (src/chunk_serial_arena.rs). При СЕРИЙНОМ чтении чанков (topup-волны,
 * соседние чанки из того же region-файла, повторные promote-попытки)
 * ванильный путь делает re-decompress одного и того же payload'а.
 * C-X3: post-inflate байты кладутся в арену (put), повторное чтение того
 * же (cx, cz, sy) при том же epoch обслуживается из арены (get) с
 * CRC32IEEE-контролем байт — любой mismatch → отравление слота → MISS →
 * ваниль.
 *
 * Паритет (закон 4, бит-в-байт):
 *  - MISS = точный ванильный decompress-путь (арена только НАБЛЮДАЕТ put);
 *  - HIT отдаёт байты, бит-в-байт равные ванильному decompress (CRC-гейт
 *    + epoch-гейт: несовпадение версии данных → MISS);
 *  - любая ошибка натива → one-shot armed=false → ваниль навсегда
 *    (fail-closed, ERR-лестница mobs_soa);
 *  - lever ≠ cmp459_cx3: класс НИКОГДА не дефайнится в кернел, гвард
 *    ARMED статически false → ни один read-сайт не активирован →
 *    ваниль бит-в-байт по построению.
 *
 * NCDFE-канон: класс дефайнится ДО RegisterNatives (rust: register_arena
 * на только что определённом классе); ARMED ставится ТОЛЬКО
 * rust-регистрацией (arenaArmed) — см. ROOTCAUSE-NCDFE.
 *
 * WIRING-ПЛАН (следующая нога, javap-декомпиляция обязательна):
 *  - site-1 (get-hook): RegionFile/IOWorker read-путь после разбора
 *    loc-entry и ДО Inflater.inflate — chunkSerialArenaGet(cx, cz, sy,
 *    epoch); rc&gt;0 → serve bytes из буфера, rc==0 → ванильный inflate;
 *  - site-2 (put-hook): сразу после успешного inflate —
 *    chunkSerialArenaPut(cx, cz, sy, epoch, inflated);
 *  - epoch: RegionFile version stamp (mtime/счётчик перезагрузок файла),
 *    инвариант: файл изменился → все слоты MISS.
 *
 * СТАТУС (12e scaffold): нативы объявлены, гвард one-shot, selftest —
 * реальный read-path retarget НЕ активирован (включается только
 * rust-регистрацией при lever cmp459_cx3 + отдельной ARM-ногой).
 */
public final class ChunkSerialArenaOps {

    /// One-shot гвард: ставится ТОЛЬКО rust-регистрацией (arenaArmed).
    private static volatile boolean ARMED = false;

    private ChunkSerialArenaOps() {}

    /// Rust RegisterNatives → arenaArmed(): единственный путь включения.
    static void arenaArmed() {
        ARMED = true;
    }

    /// Гвард read-сайтов (следующая нога): без ARMED — всегда ваниль.
    public static boolean isArmed() {
        return ARMED;
    }

    /**
     * GET: len&gt;0 = HIT (serve bytes), 0 = MISS (ванильный inflate),
     * &lt;0 = ERR (fail-closed → armed сброс → ваниль навсегда).
     * Scaffold-фаза: натив зарегистрирован только при lever cmp459_cx3;
     * без ARMED сюда заходить нельзя (гвард на read-сайте).
     */
    public static native int chunkSerialArenaGet(int cx, int cz, int sy, long epoch);

    /**
     * PUT: кладёт только что разжатые ванилью байты секции (post-inflate).
     * Возврат: 1 = stored, 0 = skip (dormant/cap/oversized). Арена никогда
     * не бросает: ошибки проглатываются (паритет важнее кэша).
     */
    public static native int chunkSerialArenaPut(int cx, int cz, int sy, long epoch, byte[] data);

    /// Selftest scaffold: класс грузится, гвард спит, нативы не линкуются
    /// без rust-регистрации → java-сторона честно dormant.
    public static void main(String[] args) {
        System.out.println("cx3-arena java stub: armed=" + ARMED
                + " (dormant scaffold; natives register only under lever cmp459_cx3)");
        if (ARMED) {
            throw new IllegalStateException("ARMED without rust registration — NCDFE-canon breach");
        }
        System.out.println("cx3-arena selftest OK (dormant)");
    }
}
