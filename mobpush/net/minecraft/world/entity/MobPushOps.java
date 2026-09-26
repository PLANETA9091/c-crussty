package net.minecraft.world.entity;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.function.Predicate;

import ca.spottedleaf.moonrise.common.PlatformHooks;
import ca.spottedleaf.moonrise.patches.chunk_system.level.ChunkSystemLevel;
import ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup;
import net.minecraft.util.profiling.Profiler;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.AABB;

/**
 * MOB-SOA (TASK-401-E, vector soa — lever cmp401_soa).
 *
 * STRUCTURE-OF-ARRAYS flat mirror broadphase для мобового push-лейна (DOD
 * реврайт data-плоскости round-400-J mobs_grid: вместо per-slot seqlock
 * записей — плоские параллельные векторы x/y/z/hw/hh/flags по плотному id).
 * Порт семантики апстримного Paper «optimize entity pushes»
 * (paper-server/patches/sources/net/minecraft/world/entity/LivingEntity.java.patch:
 * «don't run getEntities if we're not going to use its result» + «Cap entity
 * collisions»): ванильное тело LivingEntity.pushEntities потребляет результат
 * Level.getPushableEntities(this, this.getBoundingBox()) ТОЛЬКО циклом
 * doPush(entity) + cramming-счётчиками. Этот бридж заменяет ТОЛЬКО
 * перечисление кандидатов:
 *
 *   - ГОРЯЧИЕ ПОЛЯ пушатся в rust SoA-плоскость per-tick (mobUpsert: id +
 *     lid + центр AABB + hw/hh радиусы; bounding-радиус-гейт r_eff =
 *     max(halfW, halfH) <= 1.0 java-стороной — в бенч-популяции все мобы
 *     <= 0.7/2.0), без объектов — только плотный id->Entity массив
 *     java-стороны; rust держит ПЛОСКИЕ массивы (structure-of-arrays,
 *     фиксированная ёмкость, без реаллокаций и указателей);
 *   - push-кандидаты отдаются java ОДНИМ запросом mobQuery(box, lid, out):
 *     SoA-скан = одна глобальная seqlock-версия на запрос (не per-cell
 *     v1->данные->v2), обход цепочек 1.0-ячеек с ПЛОСКИМ чтением полей и
 *     грубым center±радиусы AABB-пруном прямо в rust (суперсет
 *     AABB.intersects: hw >= истинных x/z полуэкстентов — доказ. в
 *     mobs_soa.rs); java фильтрует только истинных соседей;
 *   - точные ванильные фильтры на каждом кандидате: other.level() == level,
 *     AABB.intersects(box), EntitySelector.pushableBy(entity) (предикат
 *     ванильный — Scoreboard-лейн сохраняется), other != entity;
 *   - хвост LivingEntity.pushEntities (cramming + numCollisions + doPush) —
 *     НЕ ТРОНУТ ванильный байткод; смещения одного пуша идентичны ванильным
 *     по построению, порядок кандидатов = порядок цепочек ячеек (документир.
 *     дельта класса items_subsys2).
 *
 * СВЕЖЕСТЬ (без epsilon): pushEntities вызывается в aiStep ПОСЛЕ travel/move
 * (javap offsets 620..850), поэтому само-апсерт в момент запроса кладёт в
 * плоскость end-of-move позицию self; кандидаты, тикающие позже в этом же
 * тике, ещё не двигались (их SoA-позиция = их прошлый end-of-tick =
 * текущая). Взаимный push(Entity) меняет только deltaMovement — позиция
 * интегрируется собственным move() пушуемого (Paper Entity.java.patch
 * push(Entity): additive deltas) ⇒ living<->living свежесть точная в
 * пределах однопроходного тика.
 *
 * FAIL-CLOSED: ENABLED (env == "cmp401_soa", STRICT eq) && nativeOk (mobProbe
 * magic) && !broken (структурный отказ плоскости — дизарм навсегда) &&
 * !oversized (в популяции замечен r_eff > RADIUS_GATE — весь рычаг в ванильный
 * режим) — иначе 100% ванильный fill (точная реплика fill-последовательности
 * Level.getEntities: Profiler-счётчик + EntityLookup.getEntities +
 * PlatformHooks.addToGetEntities, EntityQueryOps-контракт). Любой ненулевой
 * rc mobUpsert → ваниль на ЭТОТ вызов (self не опубликован — грид не
 * используется), ERR_STRUCT — дизарм.
 * Пустой/чужой CRUSSTY_LEVER_FLAG — сайт вообще не ретаргетится (rust-сторона
 * не ставит патч), путь ванильный по построению.
 *
 * TASK-411-C (k4soa): K4-нога — флаг "cmp411_k4soa" (STRICT eq; прежние
 * флаги сохраняют ТОЧНОЕ прежнее поведение).
 * (1) РАДИУС-РЕМОНТ: RADIUS_GATE 1.0 → 2.0 — хроника oversized-disarm
 *     (camel 1.1875 round-406d..410ck3l, iron_golem 1.35 round-409eleg2b,
 *     warden 1.45 round-409multi1) глобально дизармила ВСЮ SoA-плоскость
 *     с тика ~13 в КАЖДОЙ SoA-armed ноге — 4 PARITY/RED ноги серии меряли
 *     чистую ваниль. Пад в rust mob_query поднят до ±2 ячеек (soundness
 *     floor(q0−hw) ≥ floor(q0)−2 при hw ≤ 2.0) — пара (RADIUS_GATE, PAD)
 *     = контракт суперсета.
 * (2) PUSH ИЗ СНАПШОТА (0 per-query JNI): под k4soa pushables СНАЧАЛА
 *     пробует EntityGoalQueryOps.pushCandidates — тот же eqEpoch
 *     chain-снапшот, что entitiesOfClassGate, но предикат pushableBy,
 *     фильтр other != entity, без cls-гейта (универс = SoA-популяция
 *     LivingEntity — ТОТ ЖЕ контракт round-401, что и mobQuery: не-living
 *     pushables (boats) не в универсе — унаследованная документированная
 *     дельта, не новая). ДЕДУП ячеек прямоугольника ДО прохода цепей:
 *     двойной проход одной цепи дал бы дубликат-кандидата = двойной
 *     doPush (для nearest-пика дубликаты безвредны — для push tail НЕТ).
 *     Лестница: снапшот не готов → легаси mobQuery (JNI per-call,
 *     без изменений) → vanillaFill.
 *
 * TASK-411-C (eqsnap, v2 — пост-мортем run 35691270899 RED 0.5 TPS):
 * флаг "cmp411_eqsnap" (STRICT eq; прежние флаги — бит-в-байт). cl1-профиль
 * (84140 samples): mob_upsert = 24.9% CPU — КАЖДЫЙ per-entity upsert (48k/
 * тик) шёл через JNI под ГЛОБАЛЬНЫМ WLOCK + seqlock + 1-блочный cell-хэш;
 * сам eq chain build = 0.025% — rebuild НЕ дорог, дорога per-entity мутация
 * плоскости. V2: (а) upsertSelf ВЫЗОВЫ НЕ ИЗМЕНИЛИСЬ java-стороне — native
 * mobUpsert под eqsnap аппендит dirty-строку (id,alive,x,y,z,hw,hh) в
 * ПЕР-ПОТОКОВЫЙ delta-шард руста (0 локов/seqlock/хэша); eq_epoch (ОДИН
 * bulk JNI/тик) СНАЧАЛА сливает шарды в плоские колонки (O(dirty), один
 * потребитель), ПОТОМ строит цепи (cost per-tick = O(dirty)); (б) cell-
 * цепи плоскости под eqsnap НЕ поддерживаются → легаси mobQuery/grid-ноги
 * НЕВалидны и ПРОПУСКАЮТСЯ: лестница eqsnap = снапшот → vanillaFill
 * (точная ваниль = безупречный фоллбек); (в) RADIUS_GATE 2.0 (ремонт
 * населения k4soa переносится). Дельта свежести: позиция в снапшоте отстаёт
 * ≤1 тик (drain-каденция) — тот же документированный ghost-контракт,
 * что и у самой цепи-снапшота (замороженные колонки per-tick).
 */
public final class MobPushOps {

    private static boolean leverEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        // TASK-402-B: the round-402 composite arms the SoA plane (primary)
        // together with the mobs_grid sharded mirror (per-call fallback read
        // plane — mobGridQuery below). Legacy cmp401_soa keeps its exact
        // prior behavior: grid natives are never called under it.
        return f != null
                && (f.trim().equals("cmp401_soa") || f.trim().equals("cmp402_comp")
                    || f.trim().equals("cmp402_stagcomp")
                    || f.trim().equals("cmp403_tickplane")
                    || f.trim().equals("cmp405_stagtick")
                    // TASK-406-D: композит раунда-406 (stagtick ⊕ ai-window).
                    || f.trim().equals("cmp406_aibatch")
                    // TASK-406-E: композит раунда-406 (stagtick ⊕ sscan).
                    || f.trim().equals("cmp406_sscan")
                    // TASK-409: мультикомпозит comp⊕aibatch⊕sscan.
                    || f.trim().equals("cmp409_multi")
                    // TASK-412-F meganav: multi ⊕ navplane+navpool.
                    || f.trim().equals("cmp412_meganav")
                    // TASK-410-C (eindexq): K3-пивот R2 — SoA-плоскость =
                    // источник популяции для goal-query CSR-снапшота
                    // (EntityQueryOps.eqEpoch; sscan-прецедент TASK-406-E:
                    // составная нога SoA-plane + query-мост, маржинал меряется
                    // против cmp401_soa контроль-ноги).
                    || f.trim().equals("cmp410_eindexq")
                    // TASK-411-C (k4soa): K4 — радиус-ремонт населения
                    // (gate 2.0 / rust pad 2) + push-лейн из chain-снапшота.
                    || f.trim().equals("cmp411_k4soa")
                    // TASK-411-C (eqsnap, v2): dirty-дельты — upserts в
                    // пер-потоковые шарды, drain O(dirty) за тик.
                    || f.trim().equals("cmp411_eqsnap")
                    // TASK-412-C (eqsnap-v3): meganav ⊕ eqsnap — STRICT OR
                    // (плоскости cmp412_meganav || eqsnap-плоскость).
                    // cmp412_meganav-сайты остаются нетронутыми.
                    || f.trim().equals("cmp412_eqsnapv3")
                    // TASK-414-B: leg flag cmp414_cvs.
                    || f.trim().equals("cmp414_cvs")
                    // TASK-417-C: cvs-носитель ⊕ queryplane.
                    || f.trim().equals("cmp417_bq")
                    // TASK-419-A (colpush): колпаш-носитель — SoA-плоскость
                    // жива для planeReady()/byArr()/idCount() (eqsnap/sscan/ai);
                    // сам per-entity upsert спит (whole-body redirect).
                    || f.trim().equals("cmp420_colpush")
                    // TASK-421-A: brain-носитель (STRICT OR).
                    || f.trim().equals("cmp421_brain")
                    // TASK-422-B: brain iter-2 вектор-флаг (STRICT OR).
                    || f.trim().equals("cmp422_brain2")
                    // TASK-424-A: GC-ревизия brain3 (STRICT OR).
                    || f.trim().equals("cmp423_brain3")
                    // TASK-426-A: SoA-feed carrier (STRICT OR).
                    || f.trim().equals("cmp424_mobfeed") || f.trim().equals("cmp430_inside") || f.trim().equals("cmp432_inside2") || f.trim().equals("cmp436_ins4") || f.trim().equals("cmp451_senseins") || f.trim().equals("cmp458_swar") || f.trim().equals("cmp457_paldelta") || f.trim().equals("cmp457_eqsnap2") || f.trim().equals("cmp466_poiun") || f.trim().equals("cmp452_mega") || f.trim().equals("cmp424_mobfeed") || f.trim().equals("cmp430_inside") || f.trim().equals("cmp434_chunkpl") || f.trim().equals("cmp435_chunk3") || f.trim().equals("cmp437_chunk4") || f.trim().equals("cmp444_chunk5") || f.trim().equals("cmp450_chunk") || f.trim().equals("cmp456_chunkmono") || f.trim().equals("cmp456_chunkmono_p31snap") || f.trim().equals("cmp452_mega") || f.trim().equals("cmp453_diet"));
    }

    private static final boolean ENABLED = leverEnabled();

    /** TASK-402-B: composite mode (mirror-grid fallback active). */
    private static boolean compositeEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && (f.trim().equals("cmp402_comp")
                || f.trim().equals("cmp402_stagcomp"));
    }

    private static final boolean COMPOSITE = compositeEnabled();

    /** TASK-411-C (k4soa): K4 push-from-snapshot mode (STRICT eq). */
    private static boolean k4Enabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && f.trim().equals("cmp411_k4soa");
    }

    private static final boolean K4 = k4Enabled();

    /** TASK-411-C (eqsnap, v2): delta-shard mode (STRICT eq). */
    private static boolean eqsnapEnabled() {
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        return f != null && (f.trim().equals("cmp411_eqsnap")
                // TASK-412-C (eqsnap-v3): меганав-композит несёт eqsnap-плоскость.
                || f.trim().equals("cmp412_eqsnapv3")
                    // TASK-414-B: leg flag cmp414_cvs.
                    || f.trim().equals("cmp414_cvs")
                    // TASK-417-C: cvs-носитель ⊕ queryplane.
                    || f.trim().equals("cmp417_bq")
                    // TASK-419-A (colpush): shard-drain no-op (шарды пусты),
                    // eq_epoch chain-build жив над colpush-колонками.
                    || f.trim().equals("cmp420_colpush")
                    // TASK-421-A: brain-носитель (STRICT OR).
                    || f.trim().equals("cmp421_brain")
                    // TASK-422-B: brain iter-2 вектор-флаг (STRICT OR).
                    || f.trim().equals("cmp422_brain2")
                    // TASK-424-A: GC-ревизия brain3 (STRICT OR).
                    || f.trim().equals("cmp423_brain3")
                    // TASK-426-A: SoA-feed carrier (STRICT OR).
                    || f.trim().equals("cmp424_mobfeed") || f.trim().equals("cmp430_inside") || f.trim().equals("cmp432_inside2") || f.trim().equals("cmp436_ins4") || f.trim().equals("cmp451_senseins") || f.trim().equals("cmp458_swar") || f.trim().equals("cmp457_paldelta") || f.trim().equals("cmp457_eqsnap2") || f.trim().equals("cmp466_poiun") || f.trim().equals("cmp452_mega") || f.trim().equals("cmp424_mobfeed") || f.trim().equals("cmp430_inside") || f.trim().equals("cmp434_chunkpl") || f.trim().equals("cmp435_chunk3") || f.trim().equals("cmp437_chunk4") || f.trim().equals("cmp444_chunk5") || f.trim().equals("cmp450_chunk") || f.trim().equals("cmp456_chunkmono") || f.trim().equals("cmp456_chunkmono_p31snap") || f.trim().equals("cmp452_mega") || f.trim().equals("cmp453_diet"));
    }

    private static final boolean EQSNAP = eqsnapEnabled();

    /** LABEL для EFFECT-маркеров (server-stdout greps). */
    private static final String K4_LABEL = eqsnapFlagLabel();

    /** TASK-412-C (eqsnap-v3): EFFECT-метка = точный активный флаг. */
    private static String eqsnapFlagLabel() {
        if (!EQSNAP) return "cmp411_k4soa";
        String f = System.getenv("CRUSSTY_LEVER_FLAG");
        // TASK-419-B (sense-plane composite): свой id в EFFECT-маркерах.
        if (f != null && f.trim().equals("cmp422_brain2")) return "cmp422_brain2";
        if (f != null && f.trim().equals("cmp423_brain3")) return "cmp423_brain3";
        if (f != null && f.trim().equals("cmp424_mobfeed")) return "cmp424_mobfeed"; // TASK-426-A
        if (f != null && f.trim().equals("cmp430_inside")) return "cmp430_inside"; // TASK-430-B
        if (f != null && f.trim().equals("cmp432_inside2")) return "cmp432_inside2"; // TASK-432-B
        if (f != null && f.trim().equals("cmp436_ins4") || f.trim().equals("cmp458_swar") || f.trim().equals("cmp457_paldelta") || f.trim().equals("cmp457_eqsnap2") || f.trim().equals("cmp466_poiun")) return "cmp436_ins4"; // TASK-436-B
        if (f != null && (f.trim().equals("cmp437_chunk4") || f.trim().equals("cmp444_chunk5") || f.trim().equals("cmp450_chunk") || f.trim().equals("cmp456_chunkmono") || f.trim().equals("cmp456_chunkmono_p31snap"))) return "cmp437_chunk4"; // TASK-438-C R7 marker id (452c: precedence fixed vs 451c)
        if (f != null && (f.trim().equals("cmp434_chunkpl") || f.trim().equals("cmp435_chunk3"))) return "cmp434_chunkpl"; // TASK-434-C
        if (f != null && f.trim().equals("cmp452_mega")) return "cmp452_mega"; // TASK-452-C mega-composite marker id
        if (f != null && f.trim().equals("cmp453_diet") || f.trim().equals("cmp450_chunk") || f.trim().equals("cmp456_chunkmono") || f.trim().equals("cmp456_chunkmono_p31snap")) return "cmp453_diet"; // TASK-453-C diet composite marker id
        if (f != null && (f.trim().equals("cmp437_chunk4") || f.trim().equals("cmp444_chunk5") || f.trim().equals("cmp450_chunk") || f.trim().equals("cmp456_poi") || f.trim().equals("cmp466_poiun"))) return "cmp437_chunk4"; // TASK-438-C R7 marker id (452c: precedence fixed vs 451c)
        if (f != null && f.trim().equals("cmp453_diet") || f.trim().equals("cmp456_poi") || f.trim().equals("cmp466_poiun") || f.trim().equals("cmp450_chunk")) return "cmp453_diet"; // TASK-453-C diet composite marker id
        if (f != null && f.trim().equals("cmp421_brain")) return "cmp421_brain";
        return f != null && f.trim().equals("cmp412_eqsnapv3")
                ? "cmp412_eqsnapv3" : "cmp411_eqsnap";
    }

    /**
     * TASK-411-C (k4soa): радиус-гейт населения. 1.0 глобально дизармился на
     * camel 1.1875 / iron_golem 1.35 / warden 1.45 (хроника round-406d..410);
     * 2.0 покрывает r_eff всех ванильных мобов бенча. МЕНЯЕТСЯ ТОЛЬКО В ПАРЕ
     * с rust PAD (src/mobs_soa.rs mob_query окно ±PAD ячеек, PAD ≥ ceil(gate)).
     * STRICT-eq изоляция ног: константа фолдится компилятором — под прежними
     * флагами (cmp401_soa, cmp402_comp/stagcomp, cmp410_eindexq) гейт
     * ОСТАЁТСЯ 1.0 (бит-в-байт прежнее поведение, включая oversized-дизарм),
     * 2.0 — только под cmp411_k4soa / cmp411_eqsnap / cmp412_eqsnapv3
     * (TASK-412-C: v3-композит несёт eqsnap-плоскость → тот же гейт).
     */
    static final double RADIUS_GATE = (K4 || EQSNAP) ? 2.0D : 1.0D;

    /** One-shot EFFECT-пруф k4soa/eqsnap снапшот-пути (server-stdout.log). */
    private static volatile boolean K4_SNAP_LOGGED = false;

    private static void k4SnapMarker(int n) {
        if (!K4_SNAP_LOGGED) {
            K4_SNAP_LOGGED = true;
            LOG.info("[crussty-plugin] " + K4_LABEL + ": push-snapshot EFFECT armed"
                    + " (first chain-snapshot serve at tick "
                    + net.minecraft.server.MinecraftServer.getServer().getTickCount()
                    + ", candidates=" + n + ", zero per-query JNI)");
        }
    }

    private static final int PROBE_MAGIC = 0x5053; // "SOA"
    private static final int GRID_PROBE_MAGIC = 0x4D50; // "MP" (mobs_grid)

    /** Result codes natives: >=0 ok/count; -1 ERR_STRUCT (дизарм); -2 ERR_RANGE (per-call vanilla); -(cap) overflow. */
    private static final int ERR_STRUCT = -1;
    private static final int ERR_RANGE = -2;

    /** Overflow-порог: -(cap) при стартовом scratch 256 -> n <= -256; -2 не пересекается. */
    private static final int OVERFLOW_MAX = -256;

    static final java.util.logging.Logger LOG =
            java.util.logging.Logger.getLogger("crussty-plugin");

    // ---- natives (impl: src/mobs_soa.rs, RegisterNatives после define) ----
    private static native int mobProbe();
    private static native int mobUpsert(int id, int lid, double x, double y, double z,
            double hw, double hh);
    private static native int mobRemove(int id);
    private static native int mobQuery(double qx0, double qy0, double qz0,
            double qx1, double qy1, double qz1, int lid, int[] out);

    // ---- TASK-402-B composite: sharded mirror grid (impl: src/mobs_grid.rs;
    // кандидаты — те же плотные id SoA-плоскости, резолвятся тем же byId) ----
    private static native int mobGridProbe();
    private static native int mobGridQuery(double qx0, double qy0, double qz0,
            double qx1, double qy1, double qz1, int lid, int[] out);

    private static volatile boolean nativeOk;
    private static volatile boolean gridNativeOk;
    private static volatile boolean broken;
    private static volatile boolean oversized;

    /** id -> entity (плотный массив, grow x2; ids реиспользуются через freeIds). */
    private static Entity[] byId = new Entity[1024];
    private static int idTop = 0;

    // ---- TASK-410-C (eindexq): package-private accessors для goal-query
    // моста EntityQueryOps (тот же пакет; dense-id = SoA-ряды mobs_soa). ----
    static Entity[] byIdArr() {
        return byId;
    }

    static int idCount() {
        return idTop;
    }

    static int idCapacity() {
        return byId.length;
    }

    /** Готовность SoA-плоскости как источника популяции (fail-closed гейт). */
    static boolean planeReady() {
        return !broken && !oversized && probeOnce();
    }

    // ------------------------------------------------------------------
    // TASK-406-E (sscan despawn plane; мержено из meganav-линии TASK-412-F):
    // package-private read accessors for MobScanOps (same package) — the
    // despawn-scan bridge reads the plane's dense id space so its per-tick
    // bulk sscanEpoch pass and the O(1) per-mob nearest-player lookup share
    // the SAME id universe as the push lane. Read-only: the scan plane never
    // mutates the id/SoA state.
    // ------------------------------------------------------------------

    /** Плотный id моба в SoA-плоскости или null (не апсертнут). */
    static int[] idBoxOf(Entity e) {
        return idMap.get(e);
    }

    /**
     * TASK-419-A (colpush): java-side id-регистрация БЕЗ per-entity JNI
     * (0 JNI: колпаш-плоскость кормится ОДНИМ bulk colpushTick/тик через
     * colpush_plane_refresh). Тот же idMap/byId/freeIds юниверс, что у
     * upsertSelf (eqsnap/sscan/ai-плоскости его читают). @return id-box
     * или null (broken).
     */
    static int[] boxFor(Entity e) {
        if (broken) {
            return null;
        }
        int[] box = idMap.get(e);
        if (box != null) {
            return box;
        }
        synchronized (ID_LOCK) {
            box = idMap.get(e);
            if (box != null) {
                return box;
            }
            if (broken) {
                return null;
            }
            int id;
            if (freeTop > 0) {
                id = freeIds[--freeTop];
            } else {
                if (idTop == byId.length) {
                    byId = java.util.Arrays.copyOf(byId, byId.length * 2);
                }
                id = idTop++;
            }
            box = new int[] {id};
            byId[id] = e;
            idMap.put(e, box);
            return box;
        }
    }

    /** TASK-419-A (colpush): graveyard sweep из bulkTick (каденция внутри). */
    static void colpushSweep() {
        maybeSweep();
    }

    private static int[] freeIds = new int[256];
    private static int freeTop = 0;
    /** entity -> id-box. Пишется под ID_LOCK; читается воркерами. */
    private static final ConcurrentHashMap<Entity, int[]> idMap = new ConcurrentHashMap<>();
    private static final Object ID_LOCK = new Object();

    /** Query scratch: per-thread, grow-only, ноль аллокаций в steady-state. */
    private static final ThreadLocal<int[]> SCRATCH =
            ThreadLocal.withInitial(() -> new int[256]);

    /** Кольцо результатов (EntityQueryOps-паттерн: 8 слотов, вложенные запросы до глубины 7). */
    private static final int RING_SLOTS = 8;
    private static final ThreadLocal<ArrayList<Entity>[]> RING =
            ThreadLocal.withInitial(MobPushOps::newRing);
    private static final ThreadLocal<int[]> RING_CURSOR =
            ThreadLocal.withInitial(() -> new int[1]);

    @SuppressWarnings("unchecked")
    private static ArrayList<Entity>[] newRing() {
        ArrayList<Entity>[] ring = new ArrayList[RING_SLOTS];
        for (int i = 0; i < RING_SLOTS; i++) {
            ring[i] = new ArrayList<>(16);
        }
        return ring;
    }

    private MobPushOps() {}

    /** Ленивая проверка нативов (первый armed(); до регистрации — Throwable -> false, ретрай). */
    private static boolean probeOnce() {
        if (nativeOk) {
            return true;
        }
        synchronized (MobPushOps.class) {
            if (nativeOk) {
                return true;
            }
            try {
                nativeOk = mobProbe() == PROBE_MAGIC;
            } catch (Throwable t) {
                nativeOk = false;
            }
            return nativeOk;
        }
    }

    /** Ленивая проверка зеркального грида (только под композитом). */
    private static boolean gridProbeOnce() {
        if (!COMPOSITE || broken) {
            return false;
        }
        if (gridNativeOk) {
            return true;
        }
        synchronized (MobPushOps.class) {
            if (gridNativeOk) {
                return true;
            }
            try {
                gridNativeOk = mobGridProbe() == GRID_PROBE_MAGIC;
            } catch (Throwable t) {
                gridNativeOk = false;
            }
            return gridNativeOk;
        }
    }

    /** Gate для rust-стороны/диагностики: армирован ли бридж. */
    public static boolean armed() {
        return ENABLED && !broken && !oversized && probeOnce();
    }

    /**
     * Замена сайта Level.getPushableEntities в LivingEntity.pushEntities
     * (ванильный хвост тела — cramming/numCollisions/doPush — работает по
     * возвращённому списку без изменений).
     */
    public static List<Entity> pushables(Level level, Entity entity, AABB box) {
        Profiler.get().incrementCounter("getEntities");
        if (upsertSelf(entity) || broken) {
            return vanillaFill(level, entity, box);
        }
        // TASK-411-C (k4soa/eqsnap): снапшот-путь ПЕРВЫМ (0 per-query JNI):
        // тот же eqEpoch chain-снапшот, что и entitiesOfClassGate. null =
        // снапшот не готов/структурный дрейф/абсурдный rect — дальше легаси
        // mobQuery (без изменений), затем vanillaFill. Контракт универса —
        // ТОТ ЖЕ, что у mobQuery (SoA-популяция LivingEntity) — см. class doc.
        // TASK-411-C (eqsnap): под delta-шардами cell-цепи плоскости НЕ
        // поддерживаются (upserts в шардах, drain обновляет ТОЛЬКО плоские
        // колонки) — легаси mobQuery/grid-ноги невалидны и пропускаются:
        // лестница eqsnap = снапшот → vanillaFill (точная ваниль).
        if (K4 || EQSNAP) {
            ArrayList<Entity>[] ring = RING.get();
            int[] cursor = RING_CURSOR.get();
            int slot = cursor[0];
            cursor[0] = (slot + 1) % RING_SLOTS;
            ArrayList<Entity> list = ring[slot];
            list.clear();
            if (EntityGoalQueryOps.pushCandidates(level, entity, box, list)) {
                k4SnapMarker(list.size());
                maybeSweep();
                return list;
            }
            list.clear(); // не обслужено — слот кольца чист для legacy-пути
            if (EQSNAP) {
                return vanillaFill(level, entity, box);
            }
        }
        int lid = System.identityHashCode(level);
        int[] out = SCRATCH.get();
        int n = mobQuery(box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ, lid, out);
        if (n < 0) {
            if (n <= OVERFLOW_MAX) {
                out = new int[(-n) * 4];
                SCRATCH.set(out);
                n = mobQuery(box.minX, box.minY, box.minZ, box.maxX, box.maxY, box.maxZ, lid, out);
            }
            if (n < 0) {
                if (n == ERR_STRUCT) {
                    broken = true; // структурный отказ — весь рычаг дизармится
                    return vanillaFill(level, entity, box);
                }
                // ERR_RANGE / overflow-retry-fail: под композитом — retry
                // через зеркальный sharded grid (те же плотные id, тот же
                // byId, те же точные ванильные фильтры) ДО ванильного fill;
                // сама плоскость SoA не дизармится (per-call деградация).
                if (gridProbeOnce()) {
                    int n2 = mobGridQuery(box.minX, box.minY, box.minZ,
                            box.maxX, box.maxY, box.maxZ, lid, out);
                    if (n2 < 0 && n2 <= OVERFLOW_MAX) {
                        out = new int[(-n2) * 4];
                        SCRATCH.set(out);
                        n2 = mobGridQuery(box.minX, box.minY, box.minZ,
                                box.maxX, box.maxY, box.maxZ, lid, out);
                    }
                    if (n2 >= 0) {
                        return collect(level, entity, box, out, n2);
                    }
                }
                return vanillaFill(level, entity, box); // ERR_RANGE/overflow-retry-fail — per-call vanilla
            }
        }
        return collect(level, entity, box, out, n);
    }

    /**
     * Точная ванильная фильтрация кандидатов (level/AABB/pushableBy/other !=
     * entity) — общий хвост SoA-пути и зеркального grid-пути композита.
     */
    private static List<Entity> collect(Level level, Entity entity, AABB box, int[] out, int n) {
        ArrayList<Entity>[] ring = RING.get();
        int[] cursor = RING_CURSOR.get();
        int slot = cursor[0];
        cursor[0] = (slot + 1) % RING_SLOTS;
        ArrayList<Entity> list = ring[slot];
        list.clear();
        Predicate<Entity> predicate = EntitySelector.pushableBy(entity);
        for (int i = 0; i < n; i++) {
            int cid = out[i];
            if (cid < 0 || cid >= byId.length) {
                continue; // grow-гонка idMap/byId — bounds-guard
            }
            Entity other = byId[cid];
            if (other == null || other == entity || other.level() != level) {
                continue;
            }
            if (!other.getBoundingBox().intersects(box)) {
                continue; // точный ванильный box-тест
            }
            if (!predicate.test(other)) {
                continue; // ванильный предикат (pushableBy: isPushable/canCollideWith/team)
            }
            list.add(other);
        }
        maybeSweep();
        return list;
    }

    /**
     * Само-апсерт self в грид (id лениво). @return true — уйти в ваниль
     * (oversized / broken / структурный отказ).
     */
    private static boolean upsertSelf(Entity e) {
        if (oversized) {
            return true;
        }
        AABB bb = e.getBoundingBox();
        double hw = Math.max((bb.maxX - bb.minX) * 0.5D, (bb.maxZ - bb.minZ) * 0.5D);
        double hh = (bb.maxY - bb.minY) * 0.5D;
        if (Math.max(hw, hh) > RADIUS_GATE) {
            oversized = true; // не-грид-юниверс: весь рычаг в ваниль (fail-closed)
            LOG.warning("[crussty-plugin] cmp401_soa/k4soa: oversized bounding radius "
                    + Math.max(hw, hh) + " on " + e.getType() + " — lever reverted to vanilla");
            return true;
        }
        double cx = (bb.minX + bb.maxX) * 0.5D;
        double cy = (bb.minY + bb.maxY) * 0.5D;
        double cz = (bb.minZ + bb.maxZ) * 0.5D;
        int lid = System.identityHashCode(e.level());
        int[] box = idMap.get(e);
        if (box != null) {
            int rc = mobUpsert(box[0], lid, cx, cy, cz, hw, hh);
            if (rc != 0) {
                if (rc == ERR_STRUCT) {
                    broken = true;
                }
                return true; // ненулевой rc — ваниль на этот вызов
            }
            return false;
        }
        synchronized (ID_LOCK) {
            box = idMap.get(e);
            if (box == null) {
                if (broken) {
                    return true;
                }
                int id;
                if (freeTop > 0) {
                    id = freeIds[--freeTop];
                } else {
                    if (idTop == byId.length) {
                        byId = java.util.Arrays.copyOf(byId, byId.length * 2);
                    }
                    id = idTop++;
                }
                int rc = mobUpsert(id, lid, cx, cy, cz, hw, hh);
                if (rc != 0) {
                    if (rc == ERR_STRUCT) {
                        broken = true; // id не опубликован — ванильный путь
                    }
                    return true;   // ненулевой rc — id НЕ публикуем, ваниль на этот вызов
                }
                box = new int[] {id};
                byId[id] = e;
                idMap.put(e, box);
                return false;
            }
        }
        int rc = mobUpsert(box[0], lid, cx, cy, cz, hw, hh);
        if (rc != 0) {
            if (rc == ERR_STRUCT) {
                broken = true;
            }
            return true; // ненулевой rc — self не опубликован: ваниль на этот вызов
        }
        return false;
    }

    /**
     * Точная реплика fill-последовательности Level.getEntities(Entity,AABB,Predicate)
     * (javap: Profiler-счётчик уже инкрементирован выше; deep-fill
     * EntityLookup.getEntities + PlatformHooks.addToGetEntities; свежий
     * ArrayList вместо мёртвого guava-списка — EntityQueryOps-контракт).
     */
    private static List<Entity> vanillaFill(Level level, Entity entity, AABB box) {
        Predicate<Entity> predicate = EntitySelector.pushableBy(entity);
        ArrayList<Entity> list = new ArrayList<>();
        EntityLookup lookup = ((ChunkSystemLevel) (Object) level).moonrise$getEntityLookup();
        lookup.getEntities(entity, box, list, predicate);
        PlatformHooks.get().addToGetEntities(level, entity, box, predicate, list);
        return list;
    }

    // ------------------------------------------------------------------
    // Graveyard sweep: isRemoved()-энтити -> mobRemove + freeId (каденция
    // ~1/1024 вызова; атомарный single-sweeper; id reuse только под ID_LOCK).
    // ------------------------------------------------------------------
    private static final AtomicBoolean SWEEPING = new AtomicBoolean();
    private static long sweepGate = 0;

    private static void maybeSweep() {
        if (broken || (++sweepGate & 0x3FF) != 0L) {
            return; // sweepGate — каденция (racy инкремент с воркеров допустим)
        }
        if (!SWEEPING.compareAndSet(false, true)) {
            return;
        }
        try {
            int top = idTop;
            for (int id = 0; id < top; id++) {
                Entity e = byId[id];
                if (e == null || !e.isRemoved()) {
                    continue;
                }
                synchronized (ID_LOCK) {
                    Entity e2 = byId[id];
                    if (e2 == null || !e2.isRemoved()) {
                        continue;
                    }
                    byId[id] = null;
                    idMap.remove(e2);
                    int rc = mobRemove(id);
                    if (rc == ERR_STRUCT) {
                        broken = true;
                        return;
                    }
                    if (freeTop == freeIds.length) {
                        freeIds = java.util.Arrays.copyOf(freeIds, Math.max(16, freeTop * 2));
                    }
                    freeIds[freeTop++] = id;
                }
            }
        } catch (Throwable t) {
            LOG.warning("[crussty-plugin] cmp401_soa: sweep failed: " + t);
        } finally {
            SWEEPING.set(false);
        }
    }
}
