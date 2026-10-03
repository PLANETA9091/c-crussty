package net.minecraft.world.entity.selector;

import java.util.List;

import net.minecraft.world.entity.Entity;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;

/**
 * SELECTOR-BULK (ROUND-486 C07, lever {@code cmp486_sbulk1}) — java bridge of
 * the bulk-JNI S1-enumeration plane for the dp-stand selector lane, in the
 * EntityGoalQueryOps style (ONE bulk transition per tick, law 6).
 *
 * TARGET: {@code EntitySelector.getEntities(EntitySelector, Level, AABB)} =
 * 86.9% of the dp lane ≈ 47.7% ALL-CPU on the СТЗ-3v2 fixture (352/352
 * selectors/tick = {@code @e[type=minecraft:marker,tag=stz3v2_probe,limit=1]},
 * 1/148,044 match → full O(N) scan per query).
 *
 * S1-ENUMERATION (wiring plan: /home/z/rounds/ROUND-486/board/CLM-C07.md):
 * <ol>
 * <li>PER TICK: one bulk fill of the authoritative entity universe into the
 *     SoA plane below (sections walked in vanilla order once — order-parity
 *     canon: plane order == vanilla walk order, DP-PARITY-3 sequence-hash
 *     pins it end-to-end) + {@code structuralVersion} bump.</li>
 * <li>PER QUERY (retargeted call site, THRESH=512 gate):
 *     candidate estimate ≤ {@link #SB_THRESH} → VANILLA walk; else
 *     {@code sbEnumerate} flat filter → candidate ids → LIVE validation
 *     (type/tag/AABB.intersects on live entities — strict-superset oracle,
 *     residual predicates ALWAYS on live objects).</li>
 * </ol>
 *
 * S2-CALLER-DECIMATION: caller-side THRESH short-circuit + adaptive arm.
 * NEVER result reuse — RECON-39 (cache-class) is forbidden by construction
 * (LEDGER Л33/Л58/Л146); any structural drift between fill and query ⇒
 * fail-dominant vanilla fallback, counters monotonic (G6).
 *
 * SKELETON STATUS (this tick): source contract only — NOT compiled into the
 * kernel path, NOT defined by the rust bridge, NOT retargeted anywhere
 * (0-delta vs PIN). The wiring tick embeds the built class bytes
 * (EARLY-define + retarget_invokestatic + retransform, NCDFE T1=0 gate:
 * define-before-retransform, mirror-drift monitor) after the offline javap
 * contract pass; local javap unavailable (JRE-only sandbox) — CI gate
 * {@code scripts/cert458n/javap_flat_nested.py} verifies flat==nested.
 */
public final class SelectorBulkOps {
    /** THRESH=512 (LEDGER Л1333 preregister): candidate estimate ≤ THRESH ⇒
     * vanilla walk (plane round-trip costs more than ≤512 vanilla visits). */
    public static final int SB_THRESH = 512;

    /** STRICT-eq lever marker (yml:83-86 armer canon; single id — swarx-4). */
    public static final String LEVER_FLAG = "cmp486_sbulk1";

    // ---- per-tick SoA plane (java-owned; rust reads through critical pins) ----
    private static volatile double[] sbXs = null;
    private static volatile double[] sbZs = null;
    private static volatile int[] sbTypeOrd = null;
    private static volatile int sbPlaneRows = 0;
    private static volatile long sbStructuralVersion = 0L; // bump on EVERY fill
    private static volatile boolean sbArmed = false;        // subsystem arm

    // ---- monotonic fail-dominant capture counters (G6; never reset) ----
    private static volatile long sbQueriesBulk = 0L;
    private static volatile long sbQueriesVanilla = 0L;
    private static volatile long sbCandidatesEmitted = 0L;
    private static volatile long sbFallbackStructural = 0L;

    private SelectorBulkOps() {}

    // ---- natives (rust: src/selector_bulk.rs; registered on define) ----
    // TASK-409-E: sig must match the rust JNINativeMethod table EXACTLY.
    private static native int sbProbe();
    private static native int sbEnumerate(double[] xs, double[] zs, int[] typeOrd,
            double minX, double maxX, double minZ, double maxZ,
            int typeQ, int limit, int structuralFresh, int[] out);
    private static native int sbStats(long[] out);

    /**
     * Gated S1 path for the retargeted
     * {@code EntitySelector.getEntities} chokepoint (wiring tick).
     *
     * Contract: pure fallback semantics — ANY gate miss returns {@code null}
     * and the caller re-executes the VANILLA walk bit-in-bit (fail-dominant;
     * never throws, never changes vanilla behavior). Live validation of the
     * candidates runs the EXACT vanilla predicate on live entities.
     *
     * @return candidates (may be partially filled = fewer than limit) or
     *         {@code null} = caller MUST run the vanilla walk.
     */
    public static List<? extends Entity> getEntitiesGated(EntitySelector selector,
                                                          Level level,
                                                          AABB box) {
        if (!sbArmed || sbPlaneRows <= 0 || box == null) {
            sbQueriesVanilla++;
            return null; // vanilla bit-in-bit
        }
        // THRESH=512 gate: candidate estimate — plane rows inside the
        // (margin-expanded) box; estimate ≤ THRESH ⇒ vanilla (decimation of
        // the expensive walk callers).
        if (estimateCandidates(box) <= SB_THRESH) {
            sbQueriesVanilla++;
            return null; // vanilla bit-in-bit
        }
        double[] xs = sbXs;
        double[] zs = sbZs;
        int[] ord = sbTypeOrd;
        if (xs == null || zs == null || ord == null || xs.length != zs.length
                || xs.length != ord.length || xs.length != sbPlaneRows) {
            sbFallbackStructural++;
            return null; // structural drift — vanilla
        }
        int[] out = new int[Math.max(1, Math.min(sbPlaneRows, 1 << 16))];
        int n = sbEnumerate(xs, zs, ord, box.minX, box.maxX, box.minZ, box.maxZ,
                -1, 0, 1, out);
        if (n < 0) {
            sbFallbackStructural++;
            return null; // rust ERR_* — vanilla
        }
        sbQueriesBulk++;
        sbCandidatesEmitted += n;
        // WIRING TICK: byId resolve (dense id → Entity) in plane order +
        // live predicate (type/tag via selector, AABB.intersects) +
        // limit/sort application — superset-validated, order == vanilla walk.
        return null; // SKELETON: never reaches the fast path this tick
    }

    /** Cheap candidate estimate for the THRESH gate (wiring tick: per-fill histogram). */
    private static int estimateCandidates(AABB box) {
        // SKELETON: conservative = whole plane → THRESH gate passes only when
        // the fill pass wired its per-fill histogram (estimate in plane order).
        return sbPlaneRows;
    }

    /** Selftest: magic probe (0x5342 "SB") round-trip through the natives. */
    public static boolean selfTest() {
        if (!sbArmed) {
            return false;
        }
        return sbProbe() == 0x5342;
    }

    // ---- arm plane (wiring tick: called by the rust bridge activate) ----
    static void armPlane(double[] xs, double[] zs, int[] ord, int rows, long version) {
        sbXs = xs;
        sbZs = zs;
        sbTypeOrd = ord;
        sbPlaneRows = rows;
        sbStructuralVersion = version;
        sbArmed = rows > 0;
    }

    /** Monotonic counters readback (artifact/selftest; never reset). */
    public static long[] stats() {
        long[] out = new long[4];
        if (sbStats(out) == 4) {
            return out;
        }
        return new long[] { sbQueriesBulk, sbQueriesVanilla, sbCandidatesEmitted, sbFallbackStructural };
    }

    public static long structuralVersion() {
        return sbStructuralVersion;
    }

    // =========================================================================
    // L1-РЕПИН ×494 (Л-492-C22/C52 → javap_pins_verify_cmp493 7/7 на 83b6f9c9):
    // receiver-prepended static overloads for the R2-E/T/C trio + R1-funnel.
    // Фантом-пин (moonrise EntityLookup.getEntities(AABB)List, 0× в jar) ЗАМЕНЁН
    // реальной сигнатурной базой; desc-eq = retarget_invokestatic якоря:
    //   R2-E: (Lnet/minecraft/world/level/entity/LevelEntityGetter;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Consumer;)V
    //   R2-T: (Lnet/minecraft/world/level/entity/LevelEntityGetter;Lnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/util/AbortableIterationConsumer;)V
    //   R2-C: (Lnet/minecraft/world/level/entity/LevelEntityGetter;Lnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/world/phys/AABB;Lnet/minecraft/util/AbortableIterationConsumer;)V
    //   R1-F: (Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/entity/EntityTypeTest;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;Ljava/util/List;I)V
    // DORMANT semantic: чистая vanilla-делегация (fail-dominant по построению,
    // 0 поведения); bulk-путь — ARM-лега следующего тика (sbArmed гейт).
    // =========================================================================
    public static void getEntitiesGated(net.minecraft.world.level.entity.LevelEntityGetter receiver,
                                        net.minecraft.world.phys.AABB box,
                                        java.util.function.Consumer<Entity> consumer) {
        sbQueriesVanilla++;
        if (receiver != null && box != null && consumer != null) {
            receiver.get(box, consumer); // vanilla bit-in-bit
        }
    }

    public static <U> void getEntitiesGated(net.minecraft.world.level.entity.LevelEntityGetter receiver,
                                            net.minecraft.world.level.entity.EntityTypeTest typeTest,
                                            net.minecraft.util.AbortableIterationConsumer<U> consumer) {
        sbQueriesVanilla++;
        if (receiver != null && typeTest != null && consumer != null) {
            receiver.get(typeTest, consumer); // vanilla bit-in-bit
        }
    }

    public static <U> void getEntitiesGated(net.minecraft.world.level.entity.LevelEntityGetter receiver,
                                            net.minecraft.world.level.entity.EntityTypeTest typeTest,
                                            net.minecraft.world.phys.AABB box,
                                            net.minecraft.util.AbortableIterationConsumer<U> consumer) {
        sbQueriesVanilla++;
        if (receiver != null && typeTest != null && box != null && consumer != null) {
            receiver.get(typeTest, box, consumer); // vanilla bit-in-bit
        }
    }

    public static <T> void getEntitiesGated(Level level,
                                            net.minecraft.world.level.entity.EntityTypeTest<net.minecraft.world.entity.Entity, T> typeTest,
                                            net.minecraft.world.phys.AABB box,
                                            java.util.function.Predicate<? super T> predicate,
                                            java.util.List<? super T> out,
                                            int limit) {
        sbQueriesVanilla++;
        if (level != null && typeTest != null && box != null && predicate != null && out != null) {
            level.getEntities(typeTest, box, predicate, out, limit); // vanilla bit-in-bit
        }
    }
}
