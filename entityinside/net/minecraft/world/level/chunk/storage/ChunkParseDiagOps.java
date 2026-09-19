package net.minecraft.world.level.chunk.storage;

import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

/**
 * RECON-13d diagnostic bridge (TASK-327, CRUSSTY_PARSE_DIAG) — MUST stay a
 * single classfile with ZERO nested classes (kernel-loader define discipline).
 *
 * SerializableChunkData.parse reads the chunk coordinates through exactly one
 * `ldc "xPos"` + `invokevirtual CompoundTag.getIntOr(String,I)I` pair. The
 * classfile patch (classfile.rs retarget_ldc_virtual_to_static) retargets THAT
 * single call site to diagXIntOr — the receiver rides as the first static
 * argument, so the verifier-visible stack shape is identical, and the value is
 * delegated 1:1 (bit-exact: same getIntOr call on the same tag).
 *
 * The note path is read-only (two extra getIntOr reads on an immutable tag)
 * and wrapped in catch(Throwable) — a diagnostic must never break parsing.
 * The observed counter lane is the TOP-1 alloc lane (chunk-parse 33.38% of
 * ap-samples, RECON-13b/13e): the per-chunk load/reload census decides whether
 * a decoded-chunk cache (lever #12) can pay for itself, or whether the lane is
 * mostly first-loads (ticket churn) and needs a different lever.
 */
public final class ChunkParseDiagOps {

    /// chunkKey (x low32 | z high32) -> load count
    private static final Map<Long, Long> SEEN = new ConcurrentHashMap<>();

    private static volatile long totalLoads;
    private static volatile long firstNano;
    private static volatile long lastNano;

    private ChunkParseDiagOps() {
    }

    /// Retarget target for the `xPos` getIntOr site inside parse().
    /// Delegate is bit-exact; the note is read-only and never throws.
    public static int diagXIntOr(net.minecraft.nbt.CompoundTag tag, String key, int def) {
        try {
            if ("xPos".equals(key)) {
                long now = System.nanoTime();
                long x = tag.getIntOr("xPos", def);
                long z = tag.getIntOr("zPos", 0);
                long ck = (x & 0xFFFFFFFFL) | ((z & 0xFFFFFFFFL) << 32);
                SEEN.merge(ck, 1L, Long::sum);
                totalLoads = totalLoads + 1;
                if (firstNano == 0L) {
                    firstNano = now;
                }
                lastNano = now;
            }
        } catch (Throwable ignored) {
            // diagnostics must never break parsing (fail-open diag, fail-closed lever)
        }
        return tag.getIntOr(key, def);
    }

    /// Census snapshot line.
    private static String snapshot() {
        long uniq = SEEN.size();
        long total = totalLoads;
        long reloads = total - uniq;
        if (reloads < 0L) {
            reloads = 0L;
        }
        double share = total == 0 ? 0.0 : Math.round(1000.0 * reloads / total) / 10.0;
        return "SUMMARY total_loads=" + total
            + " unique_chunks=" + uniq
            + " repeat_loads=" + reloads
            + " repeat_share_pct=" + share
            + " span_ms=" + (firstNano == 0 ? 0 : (lastNano - firstNano) / 1_000_000L)
            + '\n';
    }

    /// Dump the census: summary + per-chunk lines sorted by load count.
    public static void dumpToFile(String path) {
        try {
            List<Map.Entry<Long, Long>> rows = new ArrayList<>(SEEN.size());
            for (Map.Entry<Long, Long> e : SEEN.entrySet()) {
                rows.add(e);
            }
            rows.sort((a, b) -> Long.compare(b.getValue(), a.getValue()));
            StringBuilder sb = new StringBuilder(rows.size() * 24 + 256);
            sb.append(snapshot());
            int limit = Math.min(rows.size(), 20000);
            for (int i = 0; i < limit; i++) {
                Map.Entry<Long, Long> e = rows.get(i);
                long ck = e.getKey();
                sb.append("chunk ").append((int) ck).append(' ').append((int) (ck >>> 32))
                    .append(' ').append(e.getValue()).append('\n');
            }
            java.nio.file.Files.writeString(
                java.nio.file.Path.of(path), sb.toString(),
                java.nio.file.StandardOpenOption.CREATE,
                java.nio.file.StandardOpenOption.TRUNCATE_EXISTING,
                java.nio.file.StandardOpenOption.WRITE);
        } catch (Throwable ignored) {
            // dump failure is not a server failure
        }
    }

    /// Periodic dump + shutdown hook (idempotent; invoked by parse_diag.rs).
    public static volatile boolean hookArmed;

    public static void armDumpTask(String path) {
        if (hookArmed) {
            return;
        }
        hookArmed = true;
        Thread t = new Thread(() -> {
            while (true) {
                try {
                    Thread.sleep(30_000L);
                } catch (InterruptedException ie) {
                    return;
                }
                dumpToFile(path);
            }
        }, "crussty-parse-diag-dump");
        t.setDaemon(true);
        t.start();
        Runtime.getRuntime().addShutdownHook(new Thread(() -> dumpToFile(path),
            "crussty-parse-diag-final"));
    }
}
