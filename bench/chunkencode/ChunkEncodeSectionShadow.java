import java.lang.reflect.Method;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;
import java.util.Arrays;

/**
 * TASK-149 phase-2a — IN-SERVER SectionData gate probe (attach shadow agent).
 *
 * Loaded via VirtualMachine.loadAgent into the LIVE server JVM (boot is pure-inject:
 * stock JDK + ONLY -agentpath; the attach is bench-lane instrumentation of a running
 * measurement subject, not a product dependency — pre-registered in
 * results/TASK149_PHASE2A_DESIGN_2026-09-09.md).
 *
 * What it does:
 *   1. Resolve the bridge class net/.../game/PaperNativeChunkPacketEncode. In-server
 *      the module's inject_surface() defines it on the BOOTSTRAP loader
 *      (env.define_class(name, NULL loader) + env.register_natives), so plain
 *      Class.forName resolves it from any loader. Fallback: TCCL/stack scan.
 *   2. Invoke the 3 registered natives with the phase-1 PURE-ARRAY matrix
 *      (no kernel classes — construction uses only java arrays, so every row is
 *      directly comparable with the standalone phase-1 battery rows).
 *   3. Emit one SHADOW row per case to the TSV given in agent args AND to stderr
 *      (stderr of the target = server boot.log — rows land in the boot journal too).
 *
 * Row formats (stdout TSV + stderr mirror):
 *   SHADOW_ENV\tclass=<name>\tloader=<loader toString|bootstrap>\tthread=<name>
 *   SHADOW\t<case>\trc=<int|ULINK|THROW:<cls>:<msg>>\tshape=<...>\tdst0=<hex|->
 *            \tdstHex=<up-to-16 first bytes hex|->\tms=<elapsed millis>
 *   SHADOW_SINK\tok=<nCases>\terr=<nErrors>
 *
 * Verdict tree is applied by the RIG (run_task149_phase2_inserver.sh), not here —
 * this agent only measures (anti-gate-shopping: no interpretation in-flight).
 */
public final class ChunkEncodeSectionShadow {

    private static final String BRIDGE = "net.minecraft.network.protocol.game.PaperNativeChunkPacketEncode";
    private static final int DST_CAP = 1 << 20;

    private static Path tsv;
    private static Method mLight, mSection, mSectionSized;
    private static int ok, err;

    public static void agentmain(String args, java.lang.instrument.Instrumentation inst) throws Exception {
        run(args);
    }

    public static void agentmain(String args) throws Exception { run(args); }

    private static synchronized void run(String args) throws Exception {
        String out = "/tmp/task149_shadow.tsv";
        if (args != null && !args.isBlank()) {
            for (String kv : args.split(";")) {
                int eq = kv.indexOf('=');
                if (eq > 0 && "out".equals(kv.substring(0, eq).trim())) out = kv.substring(eq + 1).trim();
            }
        }
        tsv = Paths.get(out);
        if (tsv.getParent() != null) Files.createDirectories(tsv.getParent());

        Class<?> bridge;
        String loaderStr;
        try {
            bridge = Class.forName(BRIDGE);
        } catch (ClassNotFoundException first) {
            bridge = scanForBridge();
        }
        if (bridge == null) {
            row("SHADOW_ENV", "class=RESOLVE-FAILED", "loader=-", "thread=" + Thread.currentThread().getName());
            throw new IllegalStateException("bridge class not resolvable: " + BRIDGE);
        }
        loaderStr = String.valueOf(bridge.getClassLoader());
        row("SHADOW_ENV", "class=" + bridge.getName(), "loader=" + loaderStr,
                "thread=" + Thread.currentThread().getName());

        Throwable bind = null;
        try {
            mLight = bridge.getMethod("nativeEncodeLightData",
                    long[].class, long[].class, long[].class, long[].class, byte[].class, int.class,
                    byte[].class, int.class, byte[].class);
            mSection = bridge.getMethod("nativeEncodeSectionData",
                    short[].class, byte[].class, int[].class, byte[].class, int[].class, long[].class,
                    byte[].class, int[].class, byte[].class, int[].class, long[].class, byte[].class);
            mSectionSized = bridge.getMethod("nativeEncodeSectionDataSized",
                    int.class, short[].class, byte[].class, int[].class, byte[].class, int.class,
                    int[].class, long[].class, int.class, byte[].class, int[].class, byte[].class,
                    int.class, int[].class, long[].class, int.class, byte[].class, int.class);
        } catch (Throwable t) {
            bind = t;
        }
        if (bind != null) {
            row("SHADOW_SINK", "ok=0", "err=1", "bind=THROW:" + bind.getClass().getSimpleName());
            throw new IllegalStateException("method resolution failed", bind);
        }

        // ---- case battery (phase-1 pure-array shapes, verbatim shapes where they exist) ----
        byte[] dst = new byte[DST_CAP];

        // light sanity — same-process wrapper sanity IN the live product context
        // (phase-1 standalone shape: Probe2 light_first)
        call("light_sanity", mLight, new Object[]{
                new long[]{0L}, new long[]{0L}, new long[]{0L}, new long[]{0L},
                new byte[2048], 0, new byte[2048], 0, new byte[4096]}, dst);

        // null-layer discriminators (phase-1: null_all -> -1 distinct, null_counts -> distinct)
        Arrays.fill(dst, (byte) 0);
        call("null_all", mSection, new Object[]{
                null, null, null, null, null, null, null, null, null, null, null, dst}, dst);
        Arrays.fill(dst, (byte) 0);
        call("null_counts", mSection, new Object[]{
                null, new byte[24], new int[0], new byte[24], new int[24], new long[0],
                new byte[24], new int[0], new byte[24], new int[24], new long[0], dst}, dst);

        // all-air N sweep (phase-1 probe: allair_n{0,1,24,26})
        for (int n : new int[]{0, 1, 24, 26}) {
            Arrays.fill(dst, (byte) 0);
            call("allair_n" + n, mSection, new Object[]{
                    shorts(n, 0), bytes(n, 0), new int[0], bytes(n, 0), new int[0], new long[0],
                    bytes(n, 0), new int[0], bytes(n, 0), new int[0], new long[0], dst}, dst);
        }

        // single-value sections (phase-1: single_sizes1_n24)
        {
            int n = 24;
            Arrays.fill(dst, (byte) 0);
            call("single_sizes1_n24", mSection, new Object[]{
                    shorts(n, 4096), bytes(n, 0), ints(n, 1), bytes(n, 1), ints(n, 0), new long[0],
                    bytes(n, 0), ints(n, 1), bytes(n, 1), ints(n, 0), new long[0], dst}, dst);
        }

        // linear 4-bit sections (phase-1: linear4_n24)
        {
            int n = 24;
            int[] pal = new int[n * 2];
            int[] offs = new int[n];
            long[] data = new long[n * 256];
            for (int i = 0; i < n; i++) { pal[2 * i] = 0; pal[2 * i + 1] = 1; offs[i] = 2 * i; }
            Arrays.fill(data, 0x0L);
            Arrays.fill(dst, (byte) 0);
            call("linear4_n24", mSection, new Object[]{
                    shorts(n, 4096), bytes(n, 4), pal, bytes(n, 2), offs, data,
                    bytes(n, 0), new int[0], bytes(n, 0), new int[0], new long[0], dst}, dst);
        }

        // biome-zero-length invariance check (phase-1: allair_biome0len_n24)
        {
            int n = 24;
            Arrays.fill(dst, (byte) 0);
            call("allair_biome0len_n24", mSection, new Object[]{
                    shorts(n, 0), bytes(n, 0), new int[0], bytes(n, 0), new int[0], new long[0],
                    new byte[0], new int[0], new byte[0], new int[0], new long[0], dst}, dst);
        }

        // all arrays length 15 (phase-1 probe: len15_all)
        Arrays.fill(dst, (byte) 0);
        call("len15_all", mSection, new Object[]{
                new short[15], new byte[15], new int[15], new byte[15], new int[15], new long[15],
                new byte[15], new int[15], new byte[15], new int[15], new long[15], dst}, dst);

        // 0xF header byte on every array, payload shifted (phase-1 probe: hdr0f_all)
        {
            int n = 16;
            short[] c = shorts(n, 0x0F00);
            byte[] b1 = bytes(n, 0x0F);
            int[] i15 = ints(n, 0x0F);
            long[] l15 = new long[n];
            Arrays.fill(l15, 0x0F0F0F0F0F0F0F0FL);
            Arrays.fill(dst, (byte) 0);
            call("hdr0f_all", mSection, new Object[]{
                    c, b1, i15, b1, i15, l15, b1, i15, b1, i15, l15, dst}, dst);
        }

        // global-palette shape (palSizes=0 WITH data; bits=8 direct ids) — new in 2a
        {
            int n = 24;
            long[] data = new long[n * 512]; // 4096 voxels * 8 bits / 64 = 512 longs per section
            Arrays.fill(data, 0xAAAAAAAAAAAAAAAAL);
            Arrays.fill(dst, (byte) 0);
            call("global_n24", mSection, new Object[]{
                    shorts(n, 4096), bytes(n, 8), new int[0], bytes(n, 0), new int[0], data,
                    bytes(n, 0), new int[0], bytes(n, 0), new int[0], new long[0], dst}, dst);
        }

        // sized variant (phase-1 probe shapes: sized_allair_n{1,24,26}, sized_linear4_n24)
        for (int n : new int[]{1, 24, 26}) {
            Arrays.fill(dst, (byte) 0);
            call("sized_allair_n" + n, mSectionSized, new Object[]{
                    n, shorts(n, 0), bytes(n, 0), new int[0], bytes(n, 0), 0, new int[0], new long[0], 0,
                    bytes(n, 0), new int[0], bytes(n, 0), 0, new int[0], new long[0], 0, dst, DST_CAP}, dst);
        }
        {
            int n = 24;
            int[] pal = new int[n * 2];
            int[] offs = new int[n];
            long[] data = new long[n * 256];
            for (int i = 0; i < n; i++) { pal[2 * i] = 0; pal[2 * i + 1] = 1; offs[i] = 2 * i; }
            Arrays.fill(dst, (byte) 0);
            call("sized_linear4_n24", mSectionSized, new Object[]{
                    n, shorts(n, 4096), bytes(n, 4), pal, bytes(n, 2), pal.length, offs, data, data.length,
                    bytes(n, 0), new int[0], bytes(n, 0), 0, new int[0], new long[0], 0, dst, DST_CAP}, dst);
        }

        row("SHADOW_SINK", "ok=" + ok, "err=" + err, "done=1");
    }

    // ---- helpers ------------------------------------------------------------

    private static Class<?> scanForBridge() {
        // Fallback: walk live threads' TCCL + stack-frame loaders + parents, ask each.
        for (Thread t : Thread.getAllStackTraces().keySet()) {
            ClassLoader cl = t.getContextClassLoader();
            for (int hop = 0; cl != null && hop < 8; hop++, cl = cl.getParent()) {
                try { return Class.forName(BRIDGE, true, cl); } catch (Throwable ignored) { }
            }
            for (StackTraceElement ste : t.getStackTrace()) {
                try {
                    Class<?> c = Class.forName(ste.getClassName());
                    ClassLoader fl = c.getClassLoader();
                    if (fl != null) {
                        try { return Class.forName(BRIDGE, true, fl); } catch (Throwable ignored) { }
                    }
                } catch (Throwable ignored) { }
            }
        }
        return null;
    }

    private static void call(String tag, Method m, Object[] argv, byte[] dst) {
        long t0 = System.nanoTime();
        String shape = shapeOf(argv);
        try {
            Object r = m.invoke(null, argv);
            long t1 = System.nanoTime();
            int rc = ((Number) r).intValue();
            int nz = 0;
            while (nz < 16 && nz < dst.length && dst[nz] != 0) nz++;
            StringBuilder hex = new StringBuilder();
            for (int i = 0; i < Math.min(nz, 16); i++) hex.append(String.format("%02x", dst[i] & 0xff));
            String hexs = hex.length() == 0 ? "-" : hex.toString();
            String d0 = dst.length > 0 ? String.format("%02x", dst[0] & 0xff) : "-";
            row("SHADOW", tag, "rc=" + rc, "shape=" + shape,
                    "dst0=" + (rc >= 0 ? d0 : "-"), "dstHex=" + (rc >= 0 ? hexs : "-"),
                    "ms=" + ((t1 - t0) / 1_000_000L));
            ok++;
        } catch (java.lang.reflect.InvocationTargetException ite) {
            Throwable c = ite.getCause() == null ? ite : ite.getCause();
            row("SHADOW", tag, "THROW:" + c.getClass().getSimpleName() + ":" + trim(c.getMessage()),
                    "shape=" + shape, "dst0=-", "dstHex=-", "ms=" + ((System.nanoTime() - t0) / 1_000_000L));
            err++;
        } catch (Throwable t) {
            row("SHADOW", tag, "ULINK:" + t.getClass().getSimpleName() + ":" + trim(t.getMessage()),
                    "shape=" + shape, "dst0=-", "dstHex=-", "ms=" + ((System.nanoTime() - t0) / 1_000_000L));
            err++;
        }
    }

    private static String shapeOf(Object[] argv) {
        StringBuilder sb = new StringBuilder();
        for (Object o : argv) {
            if (sb.length() > 0) sb.append(',');
            if (o == null) sb.append("null");
            else if (o instanceof byte[] a) sb.append("b").append(a.length);
            else if (o instanceof short[] a) sb.append("s").append(a.length);
            else if (o instanceof int[] a) sb.append("i").append(a.length);
            else if (o instanceof long[] a) sb.append("l").append(a.length);
            else if (o instanceof Integer i) sb.append("#").append(i);
            else sb.append(o.getClass().getSimpleName());
        }
        return sb.toString();
    }

    private static String trim(String s) {
        if (s == null) return "-";
        s = s.replace('\t', ' ').replace('\n', ' ');
        return s.length() > 80 ? s.substring(0, 80) : s;
    }

    private static synchronized void row(String... cols) {
        String line = String.join("\t", cols);
        System.err.println("[task149-shadow] " + line);
        try {
            Files.write(tsv, (line + "\n").getBytes(StandardCharsets.UTF_8),
                    StandardOpenOption.CREATE, StandardOpenOption.APPEND);
        } catch (Exception e) {
            System.err.println("[task149-shadow] TSV-WRITE-FAILED: " + e);
        }
    }

    private static short[] shorts(int n, int v) { short[] a = new short[n]; Arrays.fill(a, (short) v); return a; }
    private static byte[] bytes(int n, int v) { byte[] a = new byte[n]; Arrays.fill(a, (byte) v); return a; }
    private static int[] ints(int n, int v) { int[] a = new int[n]; Arrays.fill(a, v); return a; }
}
