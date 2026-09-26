package net.minecraft.world.level.chunk;

import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.state.BlockState;

/**
 * WILD-3D census bridge (round-466 C97, lever cmp466_c97 — NOT-A-BENCH leg).
 *
 * Whole-body census target: {@code LevelChunk.getBlockStateFinal(int,int,int)}
 * — the TRUNK site of the tick-thread block-state READ lane (M15 vanilla
 * anchor 36222911641, 116,506 cpu-samples: PalettedContainer.get 3.58% +
 * SimpleBitStorage.get 1.33% + readPalette 0.90% + getFluidState 0.87% +
 * getBlockStateFinal 0.84% = 7.52% all-CPU; 96.9% of PalettedContainer.get on
 * RegionTick tick-threads; caller families fluid-push 49.3% / misc
 * getBlockState 19.8% / collision 16.1% / nav 4.7% — the SAME section is read
 * by 4-5 independent consumers per tick).
 *
 * The bridge replicates the vanilla body BIT-FOR-BIT (javap ground truth
 * patched-kernel e2992d63: getSectionIndex(y) bounds guard -> sections[i]
 * nonEmptyBlockCount==0 AIR guard -> states.get((y&15)<<8 | (z&15)<<4 | x&15))
 * and adds a census probe: per 30s window, CALLS vs DISTINCT
 * (chunkIdentity,sectionIndex) pairs = the REUSE-RATIO R that decides the
 * WILD-3D "fiber" vector (per-tick per-worker decoded flat integer section
 * planes — integer-компо + tick-computation reuse). R >= 4 -> decode-once
 * wins; R ~ 1 -> REFUTED.
 *
 * PARITY: every control path is the vanilla one (same guards, same AIR
 * constant, same packed index arithmetic, same PalettedContainer.get
 * invocation, same erased checkcast). The census adds only plain-static
 * counter work + 2 array probes — NO allocation on the hot path, NO JNI,
 * NO natives. Census counters are deliberately NON-atomic plain statics
 * (racy visibility lag / lost increments acceptable for census-grade
 * numbers; never used for game semantics).
 *
 * FLUSH: every ~30s one stderr line
 * "[c97-wild-cens] lever=cmp466_c97 win_s=30.0 calls=N distinct=M reuse=R calls_per_s=.. distinct_per_s=.."
 * (window check is debounced to every 64th call; ~10 lines / thread / 300s).
 *
 * Table: open-address single/2-step probe, 2^20 slots (long key, long
 * window-seq) = 16MB static — working set ~1e5 (chunk,section) pairs across
 * ~9.2k forceloaded chunks -> load ~0.1, collision undercount <~10%
 * (census-grade, biases reuse UP not down... rather: undercounts DISTINCT ->
 * biases reuse UP; documented).
 */
public final class WildOps {

    private WildOps() {}

    // ------------------------------------------------------------------
    // Census state (plain statics — census-grade, NOT game semantics)
    // ------------------------------------------------------------------
    static long CALLS = 0L;
    static long DISTINCT = 0L;
    static long WIN_SEQ = 0L;
    static long WIN_START = System.nanoTime();
    static final long WIN_NS = 30L * 1000L * 1000L * 1000L; // 30s

    static final int TAB_BITS = 20;
    static final int TAB_SIZE = 1 << TAB_BITS;
    static final int TAB_MASK = TAB_SIZE - 1;
    static final long[] TAB_KEY = new long[TAB_SIZE];
    static final long[] TAB_WIN = new long[TAB_SIZE];
    static final Object FLUSH_LOCK = new Object();
    static final long GOLDEN = 0x9E3779B97F4A7C15L;

    public static boolean CENSUS = true;

    private static void maybeFlush() {
        long now = System.nanoTime();
        if (now - WIN_START < WIN_NS) {
            return;
        }
        synchronized (FLUSH_LOCK) {
            if (System.nanoTime() - WIN_START < WIN_NS) {
                return;
            }
            double winS = WIN_NS / 1.0e9;
            double reuse = DISTINCT == 0L ? 0.0d : (double) CALLS / (double) DISTINCT;
            System.err.println(
                "[c97-wild-cens] lever=cmp466_c97 win_s=" + winS
                    + " calls=" + CALLS
                    + " distinct=" + DISTINCT
                    + " reuse=" + String.format("%.2f", reuse)
                    + " calls_per_s=" + String.format("%.1f", CALLS / winS)
                    + " distinct_per_s=" + String.format("%.1f", DISTINCT / winS));
            CALLS = 0L;
            DISTINCT = 0L;
            WIN_SEQ++; // lazy table invalidation (probe misses on stale seq)
            WIN_START = System.nanoTime();
        }
    }

    private static void count(Object chunk, int sectionIndex) {
        long c0 = ++CALLS;
        if ((c0 & 0x3FL) == 0L) {
            maybeFlush();
        }
        long key = ((long) System.identityHashCode(chunk) << 8) | (sectionIndex & 0xFFL);
        if (key == 0L) {
            key = 1L; // keep 0 as the empty-slot sentinel
        }
        int slot = (int) ((key * GOLDEN) >>> (64 - TAB_BITS)) & TAB_MASK;
        long ws = WIN_SEQ;
        // Linear probe (insert-only within a window; stale-window slots are
        // reclaimable lazily — no clear pass, probe depth <= 8, load ~0.2).
        for (int step = 0; step < 8; step++) {
            int s = (slot + step) & TAB_MASK;
            long k = TAB_KEY[s];
            if (k == key && TAB_WIN[s] == ws) {
                return; // seen this (chunk,section) within the window
            }
            if (k == 0L || TAB_WIN[s] != ws) {
                TAB_KEY[s] = key;
                TAB_WIN[s] = ws;
                DISTINCT++;
                return;
            }
        }
        DISTINCT++; // probe overflow (rare): conservatively distinct
    }

    // ------------------------------------------------------------------
    // The whole-body census redirect target — VANILLA BODY, verbatim
    // (javap LevelChunk.getBlockStateFinal(III) patched-kernel e2992d63).
    // ------------------------------------------------------------------
    public static BlockState gbsf(LevelChunk c, int x, int y, int z) {
        int i = c.getSectionIndex(y);
        if (i < 0 || i >= c.sections.length) {
            return Blocks.AIR.defaultBlockState();
        }
        LevelChunkSection section = c.sections[i];
        if (section.nonEmptyBlockCount == 0) {
            return Blocks.AIR.defaultBlockState();
        }
        if (CENSUS) {
            count(c, i);
        }
        return section.states.get((y & 15) << 8 | (z & 15) << 4 | (x & 15));
    }
}
