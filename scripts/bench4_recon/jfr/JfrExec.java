import jdk.jfr.consumer.*;
import java.nio.file.*;
import java.util.*;

/** RECON-13c: ExecutionSample по бакету 360-390s — топ фреймов и топ стэков burst-окна. */
public class JfrExec {
    public static void main(String[] args) throws Exception {
        var file = Path.of(args[0]);
        long t0 = Long.MIN_VALUE;
        // bucket -> frame -> count ; bucket -> full stack -> count
        Map<Long, Map<String, Long>> frames = new TreeMap<>();
        Map<Long, Map<String, Long>> stacks = new TreeMap<>();
        long ev = 0, evOut = 0;
        try (var rec = new RecordingFile(file)) {
            while (rec.hasMoreEvents()) {
                RecordedEvent e = rec.readEvent();
                String name = e.getEventType().getName();
                if (!name.equals("jdk.ExecutionSample")) continue;
                ev++;
                long tms = e.getStartTime().toEpochMilli();
                if (t0 == Long.MIN_VALUE) t0 = tms;
                long b = (tms - t0) / 30000;
                var trace = e.getStackTrace();
                if (trace == null || trace.getFrames().isEmpty()) continue;
                evOut++;
                List<RecordedFrame> fs = trace.getFrames();
                String top = frame(fs.get(0));
                frames.computeIfAbsent(b, k -> new HashMap<>()).merge(top, 1L, Long::sum);
                StringBuilder sb = new StringBuilder();
                for (int i = 0; i < Math.min(fs.size(), 25); i++) sb.append(frame(fs.get(i))).append(" <- ");
                stacks.computeIfAbsent(b, k -> new HashMap<>()).merge(sb.toString(), 1L, Long::sum);
            }
        }
        System.out.println("exec_samples=" + ev + " with_stack=" + evOut + " t0=" + t0);
        List<Long> sel = new ArrayList<>();
        if (args.length > 1) {
            for (String p : args[1].split(",")) sel.add(Long.parseLong(p));
        } else {
            List<Map.Entry<Long, Long>> bt = new ArrayList<>();
            for (var x : frames.entrySet()) bt.add(Map.entry(x.getKey(), x.getValue().values().stream().mapToLong(Long::longValue).sum()));
            bt.sort((a,b) -> Long.compare(b.getValue(), a.getValue()));
            System.out.println("-- bucket totals (top 6) --");
            for (var x : bt.subList(0, Math.min(6, bt.size()))) System.out.println("t=" + x.getKey()*30 + "s n=" + x.getValue());
            for (var x : bt.subList(0, Math.min(2, bt.size()))) sel.add(x.getKey());
        }
        for (long b : sel) {
            System.out.println("\n=== bucket t=" + b*30 + "-" + (b*30+30) + "s: top frames ===");
            frames.get(b).entrySet().stream().sorted((a,c)->Long.compare(c.getValue(),a.getValue())).limit(25)
                .forEach(y -> System.out.printf("  %5d  %s%n", y.getValue(), y.getKey()));
            System.out.println("=== top stacks ===");
            stacks.get(b).entrySet().stream().sorted((a,c)->Long.compare(c.getValue(),a.getValue())).limit(6)
                .forEach(y -> System.out.printf("  %5d  %s%n", y.getValue(), y.getKey().substring(0, Math.min(y.getKey().length(), 700))));
        }
    }
    static String frame(RecordedFrame f) {
        var m = f.getMethod();
        String mn = m == null ? "?" : m.getName();
        String cn = m == null ? "?" : m.getType().getName();
        int line = f.getLineNumber();
        return cn.replace('.', '/') + "." + mn + (line > 0 ? ":" + line : "");
    }
}
