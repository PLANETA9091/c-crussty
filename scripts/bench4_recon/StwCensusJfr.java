import jdk.jfr.consumer.RecordingFile;
import jdk.jfr.consumer.RecordedEvent;

import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/**
 * StwCensusJfr — G3-STW census straight off a recon.jfr (JEP 344 / JFR stream
 * replacement for `jfr summary`, dry-run TASK-463-49; integration point
 * ROUND-462/LEDGER-53: step BETWEEN "Run Benchmark 3.0" and "Bottleneck
 * report gate", fail-closed exit 42).
 *
 * Canon G3 (LAB_LEDGER L05/L09, ROUND-463/STW-CENSUS.md):
 *   INVALID-STW-HOST  iff  stw_total_s > 23.0  OR  avg_ms > 200
 *                          OR  Full outside 9+-1 (5xCodeCache+4xMetadata)
 *   Full = 9 = 5xCodeCache + 4xMetadata (jitter 9+-1 boot/teardown)
 *
 * Usage:  java StwCensusJfr.java <recon.jfr> [--types]
 *   prints one JSON line with census fields:
 *   stw_total_s / pause_count / avg_ms / max_ms / full / full_kinds
 *   / full_total_ms / young_count / scavAvg_ms / gc_total_ms
 *   plus gcPhase_sumOfPauses_s (cross-check lane) and event-type census.
 * Exit codes: 0 = parsed OK (verdict inside JSON), 42 = gate INVALID,
 *   3 = parse failure (fail-closed).
 */
public class StwCensusJfr {
    static final double CAP_TOTAL_S = 23.0;
    static final double CAP_AVG_MS = 200.0;

    public static void main(String[] args) throws Exception {
        if (args.length < 1) {
            System.err.println("usage: java StwCensusJfr.java <recon.jfr> [--types]");
            System.exit(3);
        }
        Path file = Path.of(args[0]);
        boolean dumpTypes = List.of(args).contains("--types");
        boolean dumpPhases = List.of(args).contains("--phases");

        // ---- per-pause collection from GCPhasePauseLevel1..4 (disjoint STW phases)
        long stwTotalNanos = 0;
        long pauseCount = 0;
        long maxNanos = 0;
        long youngTotalNanos = 0;
        long youngCount = 0;
        long fullPhaseNanos = 0;
        long fullPhaseCount = 0;
        long otherTotalNanos = 0;
        long otherCount = 0;
        Map<String, Long> otherNames = new LinkedHashMap<>();
        // ---- per-GC-cycle lane (jdk.GarbageCollection: sumOfPauses/longestPause/cause)
        long gcSumOfPausesNanos = 0;
        long gcCount = 0;
        long gcYoungCount = 0;
        long gcYoungSumNanos = 0;
        long fullCount = 0;
        long fullSumNanos = 0;
        long oldTypeEvents = 0;   // jdk.OldGarbageCollection + jdk.ParallelOldGarbageCollection
        long youngTypeEvents = 0; // jdk.YoungGarbageCollection
        Map<String, Integer> fullKinds = new LinkedHashMap<>();
        Map<String, Long> typeCounts = new LinkedHashMap<>();
        long unknownNamePauses = 0;

        try (RecordingFile rec = new RecordingFile(file)) {
            while (rec.hasMoreEvents()) {
                RecordedEvent ev = rec.readEvent();
                String tn = ev.getEventType().getName();
                typeCounts.merge(tn, 1L, Long::sum);
                if (tn.equals("jdk.GCPhasePause")) {
                    // TOP-LEVEL STW pause. GCPhasePauseLevel1..4 are NESTED
                    // sub-phases inside it (dry-run fact on G1 specimen:
                    // summing them double/triple-counts the same wall time).
                    Duration d = ev.getDuration();
                    long nanos = d.toNanos();
                    stwTotalNanos += nanos;
                    pauseCount++;
                    if (nanos > maxNanos) maxNanos = nanos;
                    String name = strOf(ev, "name");
                    if (dumpPhases) {
                        System.out.println("PHASE " + tn + " | " + name + " | " + d.toNanos() + "ns");
                    }
                    boolean isFull = name != null && name.contains("Full");
                    boolean isYoung = name != null && name.contains("Young");
                    if (isFull) {
                        fullPhaseNanos += nanos;
                        fullPhaseCount++;
                    } else if (isYoung) {
                        youngTotalNanos += nanos;
                        youngCount++;
                    } else {
                        otherTotalNanos += nanos;
                        otherCount++;
                        otherNames.merge(nz(name), 1L, Long::sum);
                    }
                    if (name == null) unknownNamePauses++;
                    continue;
                }
                if (!tn.startsWith("jdk.GCPhasePause")) {
                    if (tn.equals("jdk.OldGarbageCollection") || tn.equals("jdk.ParallelOldGarbageCollection")) {
                        oldTypeEvents++;
                    } else if (tn.equals("jdk.YoungGarbageCollection")) {
                        youngTypeEvents++;
                    }
                    if (tn.equals("jdk.GarbageCollection")) {
                        // instant per-GC-cycle summary event
                        gcCount++;
                        if (dumpPhases) {
                            StringBuilder fs = new StringBuilder();
                            ev.getEventType().getFields().forEach(f -> fs.append(f.getName()).append(' '));
                            Object sumRaw = null;
                            try { sumRaw = ev.getValue("sumOfPauses"); } catch (RuntimeException ignored) { }
                            System.out.println("GCEV fields=[" + fs.toString().trim() + "] sumRaw=" + sumRaw
                                    + " | " + strOf(ev, "name") + " | " + strOf(ev, "cause"));
                        }
                        Duration sumP = durOf(ev, "sumOfPauses");
                        String name = strOf(ev, "name");
                        String cause = strOf(ev, "cause");
                        if (dumpPhases && sumP == null) {
                            System.out.println("GCEV | " + name + " | " + cause + " | sumOfPauses=NULL");
                        }
                        if (sumP != null) {
                            long s = sumP.toNanos();
                            gcSumOfPausesNanos += s;
                            // cycle names on this JVM: ParallelScavenge (young) /
                            // ParallelOld (full) / G1New/G1Old / *Full* — generic
                            // "Young|Full" matching does NOT cover ParallelGC.
                            boolean isFull = name != null && (name.contains("Full") || name.contains("Old"));
                            boolean isYoung = name != null && (name.contains("Young") || name.contains("Scavenge") || name.contains("New"));
                            if (isFull) {
                                fullCount++;
                                fullSumNanos += s;
                                String kind = kindOf(name, cause);
                                fullKinds.merge(kind, 1, Integer::sum);
                            } else if (isYoung) {
                                gcYoungCount++;
                                gcYoungSumNanos += s;
                            }
                        }
                    }
                    continue;
                }
                // nested GCPhasePauseLevel* sub-phases: tracked separately (not counted in STW total)
            }
        }

        double totalS = stwTotalNanos / 1e9;
        double avgMs = pauseCount > 0 ? (stwTotalNanos / 1e6) / pauseCount : 0.0;
        double maxMs = maxNanos / 1e6;
        double scavAvgMs = youngCount > 0 ? (youngTotalNanos / 1e6) / youngCount
                : (gcYoungCount > 0 ? (gcYoungSumNanos / 1e6) / gcYoungCount : 0.0);
        // G3 gate ×462-53: total>23.0s OR avg>200ms OR Full outside 9±1.
        // Full-count source of truth = old-generation cycle events (Old ∨
        // ParallelOld), fallback name-based cycles; on this Debian OpenJDK
        // 21.0.12.1 the ParallelGC pause name is generic "GC Pause" and the
        // cycle names are ParallelScavenge/ParallelOld — word-matching alone
        // returns 0 Fulls (dry-run fact, fixture ×463).
        long fullEff = fullCount > 0 ? fullCount : oldTypeEvents;
        boolean fullWindowOk = fullEff >= 8 && fullEff <= 10;
        boolean invalid = totalS > CAP_TOTAL_S || avgMs > CAP_AVG_MS || !fullWindowOk;
        String verdict = pauseCount == 0 ? "NO-GC-EVENTS"
                : (invalid ? "INVALID-STW-HOST" : "STW-CLEAN");

        StringBuilder json = new StringBuilder(1024);
        json.append('{');
        boolean first = true;
        first = kv(json, first, "tool", "StwCensusJfr");
        first = kv(json, first, "source", file.toString());
        first = kv(json, first, "format", "jfr-stream");
        first = kv(json, first, "stw_total_s", r2(totalS));
        first = kv(json, first, "pause_count", pauseCount);
        first = kv(json, first, "avg_ms", r1(avgMs));
        first = kv(json, first, "max_ms", r1(maxMs));
        first = kv(json, first, "young_count", youngCount);
        first = kv(json, first, "scavAvg_ms", r1(scavAvgMs));
        first = kv(json, first, "full", fullPhaseCount);
        first = kv(json, first, "full_cycles_eff", fullCount > 0 ? fullCount : oldTypeEvents);
        first = kv(json, first, "young_cycles", Math.max(gcYoungCount, youngTypeEvents));
        first = kv(json, first, "full_total_ms", r1(fullPhaseNanos > 0 ? fullPhaseNanos / 1e6 : fullSumNanos / 1e6));
        first = kv(json, first, "other_stw_count", otherCount);
        first = kv(json, first, "other_stw_ms", r1(otherTotalNanos / 1e6));
        first = kvRaw(json, first, "other_stw_names", mapJson(otherNames));
        first = kv(json, first, "gc_cycles", gcCount);
        first = kv(json, first, "gc_full_cycles", fullCount);
        first = kvRaw(json, first, "gc_full_kinds", mapJson(fullKinds));
        first = kv(json, first, "gc_sumOfPauses_s", r2(gcSumOfPausesNanos / 1e9));
        first = kv(json, first, "full_window", fullWindowOk ? "OK(9±1)" : "OUT");
        first = kv(json, first, "gc_young_scavAvg_ms", gcYoungCount > 0 ? (Object) r1((gcYoungSumNanos / 1e6) / gcYoungCount) : "null");
        first = kv(json, first, "unknown_name_pauses", unknownNamePauses);
        first = kv(json, first, "cap", "total<=" + CAP_TOTAL_S + "s & avg<=" + CAP_AVG_MS + "ms");
        first = kv(json, first, "verdict", verdict);
        json.append('}');
        System.out.println(json);
        if (dumpTypes) {
            for (Map.Entry<String, Long> e : typeCounts.entrySet()) {
                System.out.println("TYPE " + e.getKey() + " " + e.getValue());
            }
        }
        System.exit(invalid ? 42 : 0);
    }

    private static String kindOf(String name, String cause) {
        // Full GC name for HotSpot: "Pause Full (cause)..." — split out cause;
        // prefer the explicit jdk.GCCause string.
        if (cause != null && !cause.isBlank()) {
            if (cause.contains("CodeCache")) return "CodeCache GC";
            if (cause.contains("Metadata")) return "Metadata GC";
            if (cause.contains("System.gc")) return "System.gc()";
            if (cause.contains("Allocation")) return "Allocation Failure";
            if (cause.contains("Ergonomics")) return "Ergonomics";
            return cause;
        }
        int open = name.indexOf('(');
        int close = name.lastIndexOf(')');
        return (open >= 0 && close > open) ? name.substring(open + 1, close) : name;
    }

    private static Duration durOf(RecordedEvent ev, String field) {
        try {
            Object v = ev.getValue(field);
            if (v instanceof Duration d) return d;
            if (v instanceof Number n) return Duration.ofNanos(n.longValue()); // raw-nanos shape
            return null;
        } catch (RuntimeException e) {
            return null;
        }
    }

    private static String strOf(RecordedEvent ev, String field) {
        try {
            Object v = ev.getValue(field);
            return v != null ? v.toString() : null;
        } catch (RuntimeException e) {
            return null;
        }
    }

    private static String nz(String s) {
        return s != null ? s : "";
    }

    private static boolean kv(StringBuilder sb, boolean first, String k, Object v) {
        if (!first) sb.append(',');
        sb.append('"').append(k).append("\":").append(v instanceof String ? "\"" + v + "\"" : String.valueOf(v));
        return false;
    }

    /** pre-rendered JSON fragment (maps) — emitted without quoting. */
    private static boolean kvRaw(StringBuilder sb, boolean first, String k, String jsonFragment) {
        if (!first) sb.append(',');
        sb.append('"').append(k).append("\":").append(jsonFragment);
        return false;
    }

    /** Java Map.toString() is NOT JSON — emit a proper JSON object. */
    private static String mapJson(Map<String, ? extends Number> m) {
        StringBuilder sb = new StringBuilder(64);
        sb.append('{');
        boolean f = true;
        for (Map.Entry<String, ? extends Number> e : m.entrySet()) {
            if (!f) sb.append(',');
            f = false;
            sb.append('"').append(e.getKey().replace("\\", "\\\\").replace("\"", "\\\"")).append("\":").append(e.getValue());
        }
        return sb.append('}').toString();
    }

    private static double r2(double v) { return Math.round(v * 100.0) / 100.0; }
    private static double r1(double v) { return Math.round(v * 10.0) / 10.0; }
}
