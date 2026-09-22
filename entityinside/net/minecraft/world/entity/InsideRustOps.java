package net.minecraft.world.entity;

import it.unimi.dsi.fastutil.longs.LongSet;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.Mth;
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
 * TASK-411-B — INSIDE-BATCH (R3): negative fast-plane + per-tick bulk verdict
 * flush for the private per-movement
 * {@code Entity.checkInsideBlocks(Vec3,Vec3,StepBasedCollector,LongSet,int)I}.
 *
 * RETARGET (javap-контракт RESEARCH-B-k3 §4b, байткод round-396-a): ровно 3
 * сайта invokevirtual перегрузки внутри checkInsideBlocks(List,Collector)V
 * (bc 171 / 204 / 229 — единственные вызовы перегрузки в кернеле) →
 * invokestatic {@link #checkInside} (receiver-prepended, 3B→3B,
 * length-preserving). Патчер: classfile::patch_inside_batch (STRICT sites==3).
 *
 * СЕМАНТИКА still-запроса (бит-в-бит): FBIB still-ветка посещает
 * BlockPos.betweenClosed(box) с step≡0 ⇒ steps.set(0) на каждой попытке ⇒
 * ванильный возврат = steps.get()+1 = 1 ВСЕГДА (независимо от бюджета и
 * air/non-air); air-visit возвращает true ДО любых наблюдаемых действий
 * (эффектов нет, visitedBlocks не пишется). box = makeBoundingBox(to)
 * .deflate(9.999999747378752E-6). Вердикт-хулл = секции
 * [floor(min)>>4..floor(max)>>4]×3 оси (между min..max секций не бывает
 * чужих клеток): все секции hasOnlyAir ⟺ все клетки air (двусторонняя
 * точность); секции вне [minSecY,maxSecY) = void-air (vanilla
 * getBlockState). verdict==0 ⟺ ваниль эквивалентна return 1 без эффектов —
 * NEGATIVE FAST-PLANE (verdict!=0 → всегда ваниль).
 *
 * БАТЧ-ПРОТОКОЛ (один bulk-JNI на тик на поток, per-entity JNI отсутствует):
 *  - FAST-PLANE: thread-confined таблица записей {fp=id|from×3|to×3|bb×6,
 *    box, verdict, lastPhase}; HIT (полный фингерпринт, verdict==0,
 *    lastPhase==фаза) → return 1, нулевой обход/alloc (цена ~25ns).
 *  - GATHER (zero JNI): miss → ваниль-реплика тела (MethodHandle ORIG —
 *    точное ванильное тело) + append записи {fp, box=makeBoundingBox(to)
 *    .deflate(DEFLATE)} в thread-очередь (valve CAP; hull-продукт >64 секций
 *    — ваниль навсегда, без записи). Движущиеся запросы не собираются (урок
 *    bl2). bb-биты в фингерпринте делают запись точной по dims+pos: любое
 *    изменение → miss → пере-сбор (stale-вер дикт невозможен).
 *  - FLUSH: первый вызов с новым gameTime → java резолвит ДИСТИHKT секции
 *    хуллов всех записей (getChunk(FULL,false)→sections→hasOnlyAir,
 *    unknown=2) в плоский буфер ids/полы/AABB и зовёт РОВНО ОДИН native
 *    {@link #insideBatchTick}: Rust независимо перечитывает геометрию
 *    (строгая структурная проверка floors/counts, ERR_STRUCT → перманентный
 *    disarm) и комбинирует флаги секций бит-в-байт в out[] (out[i]=0 ⟺ все
 *    секции хулла i пустые). Верdict-стамп = gameTime, single-phase TTL
 *    (lastPhase==фаза). Очередь пуста / SEC_CAP переполнен → flush
 *    пропускается, верdict'ы остаются stale → все miss → ваниль (fail-closed).
 *
 * FAIL-CLOSED: гейт CRUSSTY_LEVER_FLAG STRICT eq "cmp411_insidebat" (пустой =
 * ваниль, реакция только на точное совпадение); probe-then-patch (rust-стадия
 * публикует BRIDGE_READY только при armState()=="ARMED"); ЛЮБОЙ Throwable /
 * отрицательный native rc → DISARMED навсегда → точная ваниль-реплика (ORIG).
 * Флюиды: non-air BlockState → водные секции честно дают verdict=1 → ваниль
 * → fluid-эффекты не теряются по построению. Debug-подписчики: путь ИХ
 * запросов не скипается (debugActive → ORIG).
 *
 * SUPERSEDE-аудит (урок eb7a870): сайты ретаргета — только 3 внутри
 * checkInsideBlocks(List); byte-сайты fluid_guard (updateFluidHeightAndDoFluidPushing),
 * inside_cache (isAffectedByBlocks bc 1-4 того же метода), flush_diet,
 * batch_collector не пересекаются; entity_compose регистрируется ПОЗЖЕ
 * fluid_guard в byte-hook цепи → compose serve из pristine-кэша чисто
 * замещает whole-class serve (пара чистая и на якорях, и на ноге). Реплика
 * зовёт публичные collidedWithFluid/collidedWithShapeMovingFrom — те же
 * точки, что ваниль.
 */
public final class InsideRustOps {

    private static final String LEVER_FLAG = "cmp411_insidebat";
    /** Vanilla deflate constant of the int-overload (javap bc 4414-4415). */
    private static final double DEFLATE = 9.999999747378752E-6;
    /** Record valve (cap of the thread-confined verdict table). */
    private static final int CAP = 65536;
    /** Hash table size (power of two, 2x CAP). */
    private static final int TABLE = 131072;
    /** Distinct-section cap per flush (wire valve); overflow → skip flush. */
    private static final int SEC_CAP = 65536;
    /** Per-record section-hull product cap; larger hulls stay vanilla. */
    private static final int HULL_SEC_MAX = 64;
    /** Fingerprint longs: id, from x3, to x3, bb x3+3. */
    private static final int FP_LEN = 13;

    private static final boolean ARMED;
    private static volatile boolean DISARMED = false;
    private static final MethodHandle ORIG;
    private static final Unsafe UNSAFE;
    private static final long SECTIONS_OFFSET;

    private InsideRustOps() {
    }

    /** Probe target of the rust stage (probe-then-patch discipline). */
    public static String armState() {
        return (ARMED && !DISARMED) ? "ARMED" : "DISARMED";
    }

    static {
        Unsafe u = null;
        MethodHandle mh = null;
        long off = -1L;
        boolean ok = false;
        boolean flagOff = false;
        try {
            String flag = System.getenv("CRUSSTY_LEVER_FLAG");
            if (!LEVER_FLAG.equals(flag)) {
                flagOff = true; // STRICT eq; empty/other = vanilla bit-in-bit
            } else {
                Method m = Entity.class.getDeclaredMethod("checkInsideBlocks",
                        Vec3.class, Vec3.class,
                        InsideBlockEffectApplier.StepBasedCollector.class,
                        LongSet.class, int.class);
                m.setAccessible(true);
                mh = MethodHandles.lookup().unreflect(m);
                if (mh.type().changeReturnType(int.class).parameterCount() != 5) {
                    throw new IllegalStateException("unexpected checkInsideBlocks handle type");
                }
                Field uf = Unsafe.class.getDeclaredField("theUnsafe");
                uf.setAccessible(true);
                u = (Unsafe) uf.get(null);
                Field sf = ChunkAccess.class.getDeclaredField("sections");
                off = u.objectFieldOffset(sf);
                Level.class.getMethod("getGameTime");
                Level.class.getMethod("getMinSectionY");
                Level.class.getMethod("getMaxSectionY");
                Level.class.getMethod("getChunk", int.class, int.class, ChunkStatus.class, boolean.class);
                LevelChunkSection.class.getMethod("hasOnlyAir");
                Entity.class.getMethod("getId");
                Entity.class.getMethod("getBoundingBox");
                ok = true;
            }
        } catch (Throwable t) {
            System.out.println("[crussty-insidebat] not armed: " + t);
            ok = false;
        }
        if (ok) {
            System.out.println("[crussty-insidebat] ARMED flag=" + LEVER_FLAG
                    + " cap=" + CAP + " hull=" + HULL_SEC_MAX);
        } else if (flagOff) {
            // dormant-invisible: no marker for the empty/other-flag case
        }
        ARMED = ok;
        ORIG = mh;
        UNSAFE = u;
        SECTIONS_OFFSET = off;
    }

    // ------------------------------------------------------------------
    // Retarget destination: (Entity,Vec3,Vec3,StepBasedCollector,LongSet,int)I
    // (receiver-prepended vanilla form; rust patcher asserts the descriptor).
    // ------------------------------------------------------------------
    public static int checkInside(Entity e, Vec3 from, Vec3 to,
            InsideBlockEffectApplier.StepBasedCollector col, LongSet visited, int budget) {
        if (!ARMED || DISARMED) {
            return orig(e, from, to, col, visited, budget);
        }
        TState ts = TS.get();
        try {
            long phase = e.level().getGameTime();
            if (ts.phase != phase) {
                ts.phase = phase;
                flushPending(ts, e.level());
                if (DISARMED) {
                    return orig(e, from, to, col, visited, budget);
                }
            }
            if (debugActive(e)) {
                return orig(e, from, to, col, visited, budget);
            }
            // Exact vanilla FBIB still predicate: delta.lengthSqr()
            // < (double)Mth.square(1.0E-5f) (javap BlockGetter bc 0..20).
            double dx = to.x - from.x;
            double dy = to.y - from.y;
            double dz = to.z - from.z;
            if (dx * dx + dy * dy + dz * dz >= (double) Mth.square(1.0E-5f)) {
                return orig(e, from, to, col, visited, budget); // moving: never gathered (bl2 lesson)
            }
            AABB bb = e.getBoundingBox();
            long[] fp = ts.fpScratch;
            fp[0] = e.getId();
            fp[1] = Double.doubleToLongBits(from.x);
            fp[2] = Double.doubleToLongBits(from.y);
            fp[3] = Double.doubleToLongBits(from.z);
            fp[4] = Double.doubleToLongBits(to.x);
            fp[5] = Double.doubleToLongBits(to.y);
            fp[6] = Double.doubleToLongBits(to.z);
            fp[7] = Double.doubleToLongBits(bb.minX);
            fp[8] = Double.doubleToLongBits(bb.minY);
            fp[9] = Double.doubleToLongBits(bb.minZ);
            fp[10] = Double.doubleToLongBits(bb.maxX);
            fp[11] = Double.doubleToLongBits(bb.maxY);
            fp[12] = Double.doubleToLongBits(bb.maxZ);
            long h = fpHash(fp);
            int slot = tableLookup(ts, fp, h);
            if (slot >= 0 && ts.lastPhase[slot] == ts.phase && ts.verdict[slot] == 0) {
                ts.fastHits++;
                if (!ts.effectMarked) {
                    ts.effectMarked = true;
                    System.out.println("[crussty-insidebat] EFFECT fast-hit (negative fast-plane, verdict=0 -> return 1)");
                }
                return 1; // all-air hull: vanilla return is always 1, no effects, visited untouched
            }
            int ret = orig(e, from, to, col, visited, budget);
            if (slot >= 0) {
                ts.phaseSeen[slot] = (int) Math.min(Integer.MAX_VALUE, phase);
            } else {
                ensureRecord(ts, e, to, fp, h, phase);
            }
            return ret;
        } catch (Throwable t) {
            disarm("checkInside throwable: " + t);
            return orig(e, from, to, col, visited, budget);
        }
    }

    /** Exact vanilla body via MethodHandle (fail-closed vanilla replica). */
    private static int orig(Entity e, Vec3 from, Vec3 to,
            InsideBlockEffectApplier.StepBasedCollector col, LongSet visited, int budget) {
        try {
            return (int) ORIG.invokeExact(e, from, to, col, visited, budget);
        } catch (RuntimeException | Error rt) {
            throw rt;
        } catch (Throwable t) {
            throw new RuntimeException("checkInsideBlocks rethrow", t);
        }
    }

    /** Ванильный bc 41..78 int-overload: подписчики debug-потока → ваниль. */
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

    private static void disarm(String why) {
        if (!DISARMED) {
            DISARMED = true;
            System.out.println("[crussty-insidebat] DISARMED (permanent vanilla replica): " + why);
        }
    }

    // ------------------------------------------------------------------
    // Thread-confined verdict table + wire scratch
    // ------------------------------------------------------------------

    private static final class TState {
        long phase = -1L;
        final long[] fp = new long[FP_LEN * CAP];
        final long[] fpScratch = new long[FP_LEN];
        final double[] box = new double[6 * CAP];
        final byte[] verdict = new byte[CAP];
        final int[] lastPhase = new int[CAP];
        final int[] phaseSeen = new int[CAP];
        final long[] hkey = new long[TABLE];
        final int[] hslot = new int[TABLE];
        int count = 0;
        boolean flushMarked = false;
        boolean effectMarked = false;
        long flushCount = 0L;
        long fastHits = 0L;
        long gathers = 0L;
        // wire buffers (allocated on first flush)
        int[] metaI;
        double[] metaD;
        int[] secIdx;
        int[] flags;
        byte[] out;
        int[] recMap;
        long[] secKey;
        int[] secVal;
        int secCount;
    }

    private static final ThreadLocal<TState> TS = ThreadLocal.withInitial(TState::new);

    private static long fpHash(long[] fp) {
        long h = 0x9E3779B97F4A7C15L;
        for (int i = 0; i < FP_LEN; i++) {
            long k = fp[i] * 0xC2B2AE3D27D4EB4FL;
            k ^= k >>> 31;
            h ^= k;
            h *= 0x9E3779B97F4A7C15L;
        }
        h ^= h >>> 32;
        h *= 0xD6E8FEB86659FD93L;
        h ^= h >>> 32;
        return h == 0L ? 1L : h;
    }

    private static int tableLookup(TState ts, long[] fp, long h) {
        long[] hkey = ts.hkey;
        int mask = TABLE - 1;
        int i = (int) h & mask;
        for (int probes = 0; probes < TABLE; probes++) {
            long k = hkey[i];
            if (k == 0L) {
                return -1;
            }
            if (k == h) {
                int slot = ts.hslot[i];
                long[] base = ts.fp;
                int o = FP_LEN * slot;
                boolean eq = true;
                for (int j = 0; j < FP_LEN; j++) {
                    if (base[o + j] != fp[j]) {
                        eq = false;
                        break;
                    }
                }
                if (eq) {
                    return slot;
                }
            }
            i = (i + 1) & mask;
        }
        return -1;
    }

    private static void tableInsert(TState ts, long[] fp, long h, int slot) {
        long[] hkey = ts.hkey;
        int mask = TABLE - 1;
        int i = (int) h & mask;
        for (int probes = 0; probes < TABLE; probes++) {
            if (hkey[i] == 0L) {
                hkey[i] = h;
                ts.hslot[i] = slot;
                return;
            }
            i = (i + 1) & mask;
        }
        // table full (impossible: TABLE = 2*CAP) — record stays orphaned
    }

    /** GATHER: append the miss record (box = makeBoundingBox(to).deflate). */
    private static void ensureRecord(TState ts, Entity e, Vec3 to, long[] fp, long h, long phase) {
        if (ts.count >= CAP) {
            return; // valve: new fingerprints stay vanilla
        }
        AABB box;
        try {
            box = e.makeBoundingBox(to).deflate(DEFLATE);
        } catch (Throwable t) {
            return; // fail-closed: no record, vanilla every call
        }
        int x0 = floori(box.minX) >> 4;
        int y0 = floori(box.minY) >> 4;
        int z0 = floori(box.minZ) >> 4;
        int x1 = floori(box.maxX) >> 4;
        int y1 = floori(box.maxY) >> 4;
        int z1 = floori(box.maxZ) >> 4;
        long prod = (long) (x1 - x0 + 1) * (long) (y1 - y0 + 1) * (long) (z1 - z0 + 1);
        if (prod > HULL_SEC_MAX) {
            return; // huge hull: vanilla forever, no record
        }
        int slot = ts.count++;
        long[] base = ts.fp;
        int o = FP_LEN * slot;
        for (int j = 0; j < FP_LEN; j++) {
            base[o + j] = fp[j];
        }
        ts.box[6 * slot] = box.minX;
        ts.box[6 * slot + 1] = box.minY;
        ts.box[6 * slot + 2] = box.minZ;
        ts.box[6 * slot + 3] = box.maxX;
        ts.box[6 * slot + 4] = box.maxY;
        ts.box[6 * slot + 5] = box.maxZ;
        ts.verdict[slot] = 1;    // conservative until the first flush confirms
        ts.lastPhase[slot] = 0;  // stale → miss → vanilla until flushed
        ts.phaseSeen[slot] = (int) Math.max(0L, Math.min(Integer.MAX_VALUE, phase));
        ts.gathers++;
        tableInsert(ts, fp, h, slot);
    }

    private static int floori(double v) {
        int i = (int) v;
        return v < i ? i - 1 : i;
    }

    // ------------------------------------------------------------------
    // FLUSH: one bulk JNI per phase per thread
    // ------------------------------------------------------------------

    private static void flushPending(TState ts, Level lvl) {
        int n = ts.count;
        if (n == 0) {
            return;
        }
        int minSecY = lvl.getMinSectionY();
        int maxSecY = lvl.getMaxSectionY(); // exclusive
        if (ts.metaI == null) {
            ts.metaI = new int[7 * CAP];
            ts.metaD = new double[6 * CAP];
            ts.secIdx = new int[3 * SEC_CAP];
            ts.flags = new int[SEC_CAP];
            ts.out = new byte[CAP];
            ts.recMap = new int[CAP];
            ts.secKey = new long[2 * SEC_CAP];
            ts.secVal = new int[2 * SEC_CAP];
        }
        java.util.Arrays.fill(ts.secKey, 0L);
        ts.secCount = 0;
        int[] metaI = ts.metaI;
        double[] metaD = ts.metaD;
        int[] secIdx = ts.secIdx;
        int[] flags = ts.flags;
        int[] recMap = ts.recMap;
        int w = 0;
        int s = 0;
        boolean skip = false;
        for (int i = 0; i < n && !skip; i++) {
            double b0 = ts.box[6 * i];
            double b1 = ts.box[6 * i + 1];
            double b2 = ts.box[6 * i + 2];
            double b3 = ts.box[6 * i + 3];
            double b4 = ts.box[6 * i + 4];
            double b5 = ts.box[6 * i + 5];
            int x0 = floori(b0) >> 4;
            int y0 = floori(b1) >> 4;
            int z0 = floori(b2) >> 4;
            int x1 = floori(b3) >> 4;
            int y1 = floori(b4) >> 4;
            int z1 = floori(b5) >> 4;
            if (y0 < minSecY) {
                y0 = minSecY;
            }
            if (y1 > maxSecY - 1) {
                y1 = maxSecY - 1;
            }
            if (y0 > y1) {
                // hull entirely outside world height: every cell is void-air
                ts.verdict[i] = 0;
                ts.lastPhase[i] = (int) Math.min(Integer.MAX_VALUE, ts.phase);
                continue;
            }
            long prod = (long) (x1 - x0 + 1) * (long) (y1 - y0 + 1) * (long) (z1 - z0 + 1);
            if (prod > HULL_SEC_MAX) {
                ts.verdict[i] = 1;
                ts.lastPhase[i] = (int) Math.min(Integer.MAX_VALUE, ts.phase);
                continue; // never wired; verdict stays vanilla-side
            }
            metaI[7 * w] = (int) ts.fp[FP_LEN * i]; // entity id
            metaI[7 * w + 1] = x0;
            metaI[7 * w + 2] = y0;
            metaI[7 * w + 3] = z0;
            metaI[7 * w + 4] = x1;
            metaI[7 * w + 5] = y1;
            metaI[7 * w + 6] = z1;
            metaD[6 * w] = b0;
            metaD[6 * w + 1] = b1;
            metaD[6 * w + 2] = b2;
            metaD[6 * w + 3] = b3;
            metaD[6 * w + 4] = b4;
            metaD[6 * w + 5] = b5;
            boolean ovf = false;
            for (int cx = x0; cx <= x1 && !ovf; cx++) {
                for (int cy = y0; cy <= y1 && !ovf; cy++) {
                    for (int cz = z0; cz <= z1; cz++) {
                        long key = secPack(cx, cy, cz);
                        int j = secDedupGet(ts, key);
                        if (j == 0) {
                            if (s >= SEC_CAP) {
                                ovf = true;
                                break;
                            }
                            secIdx[3 * s] = cx;
                            secIdx[3 * s + 1] = cy;
                            secIdx[3 * s + 2] = cz;
                            flags[s] = resolveSection(lvl, cx, cy, cz, minSecY, maxSecY);
                            s++;
                            secDedupPut(ts, key, s); // store idx+1
                        }
                    }
                }
            }
            if (ovf) {
                skip = true; // fail-closed: stale verdicts → all miss → vanilla
                break;
            }
            recMap[w] = i;
            w++;
        }
        if (skip || w == 0) {
            return;
        }
        byte[] out = ts.out;
        int rc = insideBatchTick(w, minSecY, maxSecY, metaI, metaD, secIdx, flags, out);
        if (rc != 0) {
            disarm("flush rc=" + rc + " (ERR_STRUCT: structural floors/counts mismatch)");
            return;
        }
        int zero = 0;
        for (int j = 0; j < w; j++) {
            int i = recMap[j];
            ts.verdict[i] = out[j];
            ts.lastPhase[i] = (int) Math.min(Integer.MAX_VALUE, ts.phase);
            if (out[j] == 0) {
                zero++;
            }
        }
        ts.flushCount++;
        if (!ts.flushMarked) {
            ts.flushMarked = true;
            System.out.println("[crussty-insidebat] FLUSH n=" + w + " sec=" + s
                    + " verdict0=" + zero + " phase=" + ts.phase);
        }
    }

    private static long secPack(int cx, int cy, int cz) {
        // non-negative 63-bit pack; 0 impossible for sane coords (sentinel-safe)
        return ((cx + 1048576L) << 42) | ((cz + 1048576L) << 21) | (cy + 1048576L);
    }

    /** 0 = absent (sentinel), else distinct-index + 1. */
    private static int secDedupGet(TState ts, long key) {
        long[] keys = ts.secKey;
        int mask = keys.length - 1;
        int i = (int) (key ^ (key >>> 21)) & mask;
        for (int probes = 0; probes < keys.length; probes++) {
            long k = keys[i];
            if (k == 0L || k == key) {
                return ts.secVal[i];
            }
            i = (i + 1) & mask;
        }
        return 0;
    }

    private static void secDedupPut(TState ts, long key, int val) {
        long[] keys = ts.secKey;
        int mask = keys.length - 1;
        int i = (int) (key ^ (key >>> 21)) & mask;
        for (int probes = 0; probes < keys.length; probes++) {
            if (keys[i] == 0L) {
                keys[i] = key;
                ts.secVal[i] = val;
                ts.secCount++;
                return;
            }
            i = (i + 1) & mask;
        }
    }

    /** Java-side section verdict: 0=hasOnlyAir, 1=non-air (blocks OR fluids
     *  — nonEmptyBlockCount counts both), 2=unknown (unloaded chunk etc.). */
    private static int resolveSection(Level lvl, int cx, int cy, int cz,
            int minSecY, int maxSecY) {
        if (cy < minSecY || cy >= maxSecY) {
            return 0; // void-air (defensive; hulls are pre-clamped)
        }
        try {
            ChunkAccess ca = lvl.getChunk(cx, cz, ChunkStatus.FULL, false);
            if (!(ca instanceof LevelChunk)) {
                return 2;
            }
            Object o = UNSAFE.getObject(ca, SECTIONS_OFFSET);
            if (!(o instanceof LevelChunkSection[])) {
                return 2;
            }
            LevelChunkSection[] secs = (LevelChunkSection[]) o;
            int idx = cy - minSecY;
            if (idx < 0 || idx >= secs.length) {
                return 2;
            }
            LevelChunkSection sec = secs[idx];
            if (sec == null) {
                return 0; // missing section = air (vanilla getBlockState)
            }
            return sec.hasOnlyAir() ? 0 : 1;
        } catch (Throwable t) {
            return 2; // unknown → vanilla (fail-closed)
        }
    }

    /**
     * THE one bulk JNI per phase per thread. Rust re-reads the metaD geometry
     * (strict structural check of floors/counts — any mismatch is a negative
     * ERR code and the java side disarms permanently), then combines the
     * per-section flags bit-in-byte into out[]: out[i]=0 iff ALL sections of
     * record i's hull carry flag 0 (all-air). Unknown sections (2) and
     * non-air (1) both make the verdict 1 = vanilla.
     *
     * @param metaI  int[7n]: per record [id, x0,y0,z0, x1,y1,z1] (clamped floors)
     * @param metaD  double[6n]: per record box minX..maxZ (gather-time)
     * @param secIdx int[3s]: DISTINCT sections (cx,cy,cz) in FIRST-OCCURRENCE
     *               order of the per-record nested x/y/z enumeration (a
     *               section shared by several records is wired once, where
     *               the earliest record reached it); the native replays the
     *               same enumeration from metaI/metaD and cross-validates
     *               the distinct sequence + flags bit-in-byte
     * @param flags  int[s]: per distinct section {0,1,2}
     * @param out    byte[n]: verdict per record {0,1}
     * @return 0 ok; negative ERR_* → permanent vanilla on the java side
     */
    private static native int insideBatchTick(int n, int minSecY, int maxSecY,
            int[] metaI, double[] metaD, int[] secIdx, int[] flags, byte[] out);
}
