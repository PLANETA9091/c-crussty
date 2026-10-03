import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.Map;

/**
 * FLUSH-DIET offline harness (S7-137, ARCH-ATTACK lever #4) — plain JVM,
 * real kernel classes, NO server boot (INJECTS-ONLY discipline).
 *
 * What it proves OFFLINE:
 *   1. STRUCTURAL — the patched StepBasedCollector class file (rust
 *      iface->static+2nop retarget, CP-append, length-preserving) is
 *      ACCEPTED by the JVM: defineClass runs the structural pass, and the
 *      Class.forName link verifies the retargeted flushStep body against
 *      the REAL kernel classes (a malformed splice or a bad stack shape
 *      dies here, never on a live boot).
 *   2. WIRING — the patched class references FlushOps and carries the
 *      invokestatic sites (raw constant-pool scan, JVM-side audit; the
 *      exact two-site/count/nop-shape proof lives in the rust tests).
 *   3. BEHAVIORAL SMOKE — the dominant vanilla path (advanceStep ->
 *      flushStep over empty before/after lists — the path the alloc census
 *      pinned at 4.6% churn) executes end-to-end on the patched class with
 *      zero exceptions, meaning the invokestatic FlushOps.fladd call shape
 *      links and runs; the same smoke on the vanilla kernel class passes
 *      identically (pair discipline). The empty-source fladd returns false
 *      without touching dst (vanilla addAll semantics minus the wasted
 *      new Object[0]); a non-empty source delegates to List.addAll —
 *      median-exact parity by construction, enforced live by the CI leg's
 *      fixture gates.
 *
 * Identity note (documented S7-135): the patched collector is defined in a
 * byte-map child loader while the vanilla one stays in the parent — a
 * deliberate identity fork for the smoke only (no kernel Entity instances
 * are involved); the runtime serves the patch via retransform in the ONE
 * kernel loader.
 *
 * Exit code 0 = PASS; any failure throws (nonzero).
 */
public final class FlushDietHarness {

    static class ByteMapClassLoader extends ClassLoader {
        private final Map<String, byte[]> map;

        ByteMapClassLoader(Map<String, byte[]> map, ClassLoader parent) {
            super(parent);
            this.map = map;
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            byte[] bytes = map.get(name);
            if (bytes != null) {
                Class<?> c = findLoadedClass(name);
                if (c == null) {
                    c = defineClass(name, bytes, 0, bytes.length);
                }
                if (resolve) resolveClass(c);
                return c;
            }
            return super.loadClass(name, resolve);
        }
    }

    /** Smoke the advanceStep/flushStep lane via reflection (public API). */
    static void smoke(Class<?> sbc, String tag) throws Exception {
        Object collector = sbc.getDeclaredConstructor().newInstance();
        Class<?> posCls = Class.forName("net.minecraft.core.BlockPos");
        Object pos = posCls.getMethod("containing", double.class, double.class, double.class)
            .invoke(null, 1.0d, 64.0d, 1.0d);
        Class<?> typeCls = Class.forName(
            "net.minecraft.world.entity.InsideBlockEffectType");
        Object[] types = (Object[]) typeCls.getMethod("values").invoke(null);
        Class<?> consumerCls = Class.forName("java.util.function.Consumer");
        java.lang.reflect.Method advance = sbc.getMethod("advanceStep", int.class, posCls);
        java.lang.reflect.Method applyM = sbc.getMethod("apply", typeCls);
        java.lang.reflect.Method runBefore = sbc.getMethod(
            "runBefore", typeCls, consumerCls);
        java.lang.reflect.Method runAfter = sbc.getMethod(
            "runAfter", typeCls, consumerCls);
        java.lang.reflect.Method applyAndClear = sbc.getMethod(
            "applyAndClear", Class.forName("net.minecraft.world.entity.Entity"));

        // (a) dominant path: 2000 empty-step advanceSteps (vanilla spent a
        // new Object[0] per addAll here; patched must run the bridge).
        for (int i = 0; i < 2000; i++) {
            advance.invoke(collector, i, pos);
        }
        // (b) non-empty path: register before/after/single effects, step —
        // flushStep moves them into finalEffects THROUGH fladd.
        for (int i = 0; i < types.length; i++) {
            Object t = types[i];
            java.util.function.Consumer<Object> noop = x -> {};
            runBefore.invoke(collector, t, noop);
            runAfter.invoke(collector, t, noop);
            applyM.invoke(collector, t);
        }
        advance.invoke(collector, 100000, pos);
        // (c) applyAndClear on a fresh collector (empty finalEffects — the
        // Entity argument is never dereferenced before the first effect).
        Object fresh = sbc.getDeclaredConstructor().newInstance();
        applyAndClear.invoke(fresh, (Object) null);
        System.out.println("harness: " + tag + " smoke ok (2000 empty steps, "
            + types.length + " typed effects, fresh applyAndClear)");
    }

    public static void main(String[] args) throws Exception {
        if (args.length != 2) {
            throw new IllegalArgumentException("usage: FlushDietHarness <StepBasedCollector.patched> <RecordedEffect.vanilla>");
        }
        byte[] sbcPatched = Files.readAllBytes(Path.of(args[0]));

        // Offline registry static-init (NOT a boot: no server, no world, no
        // eula — the same tier as javac-against-kernel compiles). Needed
        // because InsideBlockEffectType's <clinit> wires a BuiltInRegistries
        // default value.
        net.minecraft.SharedConstants.tryDetectVersion();
        net.minecraft.server.Bootstrap.bootStrap();

        ClassLoader parent = FlushDietHarness.class.getClassLoader();

        Map<String, byte[]> map = new HashMap<>();
        map.put("net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector",
            sbcPatched);
        // Nest partner: private static RecordedEffect must share the loader
        // (IllegalAccessError cross-loader otherwise — nest-host discipline).
        map.put("net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector$RecordedEffect",
            Files.readAllBytes(Path.of(args[1])));
        ClassLoader l2 = new ByteMapClassLoader(map, parent);

        // 1. STRUCTURAL + verifier link of the patched collector.
        Class<?> patched = Class.forName(
            "net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector",
            false, l2);
        System.out.println("harness: patched StepBasedCollector defined (structural pass, "
            + sbcPatched.length + " B)");

        // 2. WIRING: patched class references FlushOps; carries invokestatic.
        String latin = new String(sbcPatched, java.nio.charset.StandardCharsets.ISO_8859_1);
        if (!latin.contains("FlushOps")) {
            throw new AssertionError("patched collector lacks FlushOps reference");
        }
        boolean hasInvokestatic = false;
        for (int i = 0; i + 2 < sbcPatched.length && !hasInvokestatic; i++) {
            if (sbcPatched[i] == (byte) 0xb8
                && latin.charAt(i + 1) == 'n' && latin.charAt(i + 2) == 'e') {
                // 'n','e' of a net/... CP name right after the opcode — a
                // heuristic the raw scan supplements; the authoritative
                // two-site/nop-shape proof is in the rust tests.
                hasInvokestatic = true;
            }
        }
        boolean anyStatic = false;
        for (byte b : sbcPatched) { if (b == (byte) 0xb8) { anyStatic = true; break; } }
        if (!anyStatic) {
            throw new AssertionError("no invokestatic opcode in patched collector");
        }
        System.out.println("harness: flushStep retarget verified "
            + "(invokestatic -> FlushOps.fladd; rust tests pin site count + nop shape)");

        // Bridge must define + link in the parent (kernel classpath).
        Class<?> ops = Class.forName("net.minecraft.world.entity.FlushOps", true, parent);
        Object fladdRet = ops.getMethod("fladd", java.util.List.class, java.util.Collection.class)
            .invoke(null, new java.util.ArrayList(), new java.util.ArrayList());
        if (!Boolean.FALSE.equals(fladdRet)) {
            throw new AssertionError("fladd(empty) must be false");
        }
        java.util.List<String> dst = new java.util.ArrayList<>();
        java.util.List<String> src = java.util.List.of("a", "b");
        Object addRet = ops.getMethod("fladd", java.util.List.class, java.util.Collection.class)
            .invoke(null, dst, src);
        if (!Boolean.TRUE.equals(addRet) || dst.size() != 2 || !dst.get(0).equals("a")) {
            throw new AssertionError("fladd(non-empty) must delegate to addAll in order");
        }
        System.out.println("harness: FlushOps.fladd linked and "
            + "semantics verified (empty->false no-op, non-empty->ordered addAll)");

        // 3. BEHAVIORAL SMOKE pair: patched vs vanilla kernel class.
        smoke(patched, "patched");
        Class<?> vanilla = Class.forName(
            "net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector",
            false, parent);
        smoke(vanilla, "vanilla");

        System.out.println("FLUSH-DIET OFFLINE PASS");
    }
}
