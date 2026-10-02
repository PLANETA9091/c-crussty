import jdk.jfr.consumer.*;
import java.nio.file.*;
import java.util.*;

/** RECON-13c: ObjectAllocationSample — по потокам для класса-фильтра в окне бакетов. */
public class JfrThread {
    public static void main(String[] args) throws Exception {
        var file = Path.of(args[0]);
        String needle = args.length > 1 ? args[1] : "BlockPos";
        long t0 = Long.MIN_VALUE;
        Map<String, long[]> threads = new HashMap<>(); // threadName -> [weight, count]
        Map<String, long[]> byBucket = new TreeMap<>(); // bucket:thread -> [w, c]
        long ev = 0, evNeedle = 0, wNeedle = 0;
        try (var rec = new RecordingFile(file)) {
            while (rec.hasMoreEvents()) {
                RecordedEvent e = rec.readEvent();
                if (!e.getEventType().getName().equals("jdk.ObjectAllocationSample")) continue;
                ev++;
                RecordedObject vo = e.getValue("objectClass");
                String cls = vo != null ? vo.getString("name") : "?";
                if (!cls.contains(needle)) continue;
                evNeedle++;
                long w = e.getLong("weight");
                wNeedle += w;
                long tms = e.getStartTime().toEpochMilli();
                if (t0 == Long.MIN_VALUE) t0 = tms;
                long b = (tms - t0) / 30000;
                var th = e.getThread();
                String tn = th == null ? "?" : (th.getJavaName() == null ? "?" : th.getJavaName());
                threads.computeIfAbsent(tn, k -> new long[2]);
                threads.get(tn)[0] += w; threads.get(tn)[1]++;
                byBucket.computeIfAbsent(b + ":" + tn, k -> new long[2]);
                byBucket.get(b + ":" + tn)[0] += w; byBucket.get(b + ":" + tn)[1]++;
            }
        }
        System.out.println("events=" + ev + " needle(" + needle + ")=" + evNeedle + " weight=" + wNeedle);
        System.out.println("-- threads total --");
        for (var x : threads.entrySet().stream().sorted((a,b)->Long.compare(b.getValue()[0],a.getValue()[0])).toList())
            System.out.printf("  %15d n=%6d  %s%n", x.getValue()[0], x.getValue()[1], x.getKey());
        System.out.println("-- bucket:thread (weight>10M) --");
        for (var x : byBucket.entrySet())
            if (x.getValue()[0] > 10_000_000)
                System.out.printf("  t=%-4s %15d n=%6d  %s%n", x.getKey().split(":")[0] + "s", x.getValue()[0], x.getValue()[1], x.getKey().split(":")[1]);
    }
}
