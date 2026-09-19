import jdk.jfr.consumer.*;
import java.nio.file.*;
import java.util.*;

/** RECON-13c: временная гистограмма jdk.ObjectAllocationSample по 30s-бакетам. */
public class JfrAlloc {
    public static void main(String[] args) throws Exception {
        var file = Path.of(args[0]);
        Map<String, Long> types = new HashMap<>();
        Map<Long, Map<String, Long>> buckets = new TreeMap<>();
        Map<String, long[]> totals = new HashMap<>(); // class -> [bytes, count]
        long t0 = Long.MIN_VALUE, t1 = Long.MIN_VALUE, ev = 0;
        try (var rec = new RecordingFile(file)) {
            while (rec.hasMoreEvents()) {
                RecordedEvent e = rec.readEvent();
                String name = e.getEventType().getName();
                types.merge(name, 1L, Long::sum);
                if (!name.equals("jdk.ObjectAllocationSample")) continue;
                ev++;
                RecordedObject vo = e.getValue("objectClass");
                String cls = vo != null ? vo.getString("name") : "?";
                long w = e.getLong("weight");
                long tms = e.getStartTime().toEpochMilli();
                if (t0 == Long.MIN_VALUE) t0 = tms;
                t1 = tms;
                long b = (tms - t0) / 30000;
                buckets.computeIfAbsent(b, k -> new TreeMap<>()).merge(cls, w, Long::sum);
                totals.computeIfAbsent(cls, k -> new long[2]);
                totals.get(cls)[0] += w; totals.get(cls)[1]++;
            }
        }
        System.out.println("events=" + ev + " span_s=" + (t0 == Long.MIN_VALUE ? 0 : (t1 - t0) / 1000));
        System.out.println("-- top types --");
        types.entrySet().stream().sorted(Map.Entry.<String,Long>comparingByValue().reversed())
             .limit(15).forEach(x -> System.out.println(x.getValue() + " " + x.getKey()));
        // топ-классы по весу
        System.out.println("-- top classes by total weight --");
        List<Map.Entry<String, long[]>> top = totals.entrySet().stream()
            .sorted((a,b) -> Long.compare(b.getValue()[0], a.getValue()[0])).limit(12).toList();
        for (var x : top) System.out.printf("%s %d bytes n=%d%n", x.getKey(), x.getValue()[0], x.getValue()[1]);
        // гистограмма по бакетам для топ-классов
        Set<String> focus = new HashSet<>();
        for (var x : top) focus.add(x.getKey());
        System.out.println("-- buckets 30s: total | class bytes (top classes only) --");
        for (var be : buckets.entrySet()) {
            long tot = be.getValue().values().stream().mapToLong(Long::longValue).sum();
            StringBuilder sb = new StringBuilder(String.format("t=%ds tot=%d", be.getKey()*30, tot));
            for (var x : top) {
                long w = be.getValue().getOrDefault(x.getKey(), 0L);
                if (w > 0) sb.append(String.format(" | %s=%d", shortName(x.getKey()), w));
            }
            System.out.println(sb);
        }
    }
    static String shortName(String fqn) {
        int p = fqn.lastIndexOf('.');
        String s = p >= 0 ? fqn.substring(p+1) : fqn;
        return s.length() > 22 ? s.substring(0, 22) : s;
    }
}
