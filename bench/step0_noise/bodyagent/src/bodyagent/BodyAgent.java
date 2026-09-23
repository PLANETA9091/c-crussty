package bodyagent;

import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.Instrumentation;
import java.security.ProtectionDomain;

import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

/**
 * G-BODY (TASK-71) — standalone whole-body dispatch prototype of the
 * batching-layer patch shape (design doc §3.2 / §8 Session 2), validated in
 * a CPU-only JVM BEFORE any module .so / server work.
 *
 * Lifecycle (trigger-based so the same JVM can produce a pristine baseline):
 *   1. premain registers a canRetransform transformer. At PerlinNoise
 *      class-load it captures the pristine bytes and returns null (the JVM
 *      defines the class untouched — dormant-invisible).
 *   2. The driver computes the pristine baseline outputs, then calls
 *      retransformNow(PerlinNoise.class) — the transform pass with a
 *      non-null `cd` now returns the patched bytes.
 *   3. Patched body: the ENTIRE getValue(DDDDDDZ)D method (the octave loop)
 *      is replaced by one straight-line invokestatic of
 *      BodyDispatch.getValue(PerlinNoise,DDDDDDZ)D — the G9 whole-method
 *      shape with a trivially validable body (no branches, no frames,
 *      COMPUTE_MAXS only — no ASM class resolution pitfalls).
 *
 * This rig is diagnostics-only: it never touches the module .so, src/, or
 * the server. Production retransform discipline (quiet activation worker,
 * exactly one retransform, pristine-bytes fallback, B.2.2 ladder) is the
 * module's own machinery and is unchanged.
 */
public final class BodyAgent {
    private BodyAgent() {}

    private static volatile Instrumentation INST;
    private static volatile boolean PATCH_REQUESTED;
    private static volatile boolean PATCH_APPLIED;
    private static volatile byte[] PRISTINE;

    public static void premain(String args, Instrumentation inst) {
        INST = inst;
        System.setProperty("crussty.bodyagent.armed", "1");
        inst.addTransformer(new ClassFileTransformer() {
            @Override
            public byte[] transform(ClassLoader cl, String cn, Class<?> cd,
                                    ProtectionDomain pd, byte[] b) {
                if (!"net/minecraft/world/level/levelgen/synth/PerlinNoise".equals(cn)) {
                    return null;
                }
                if (PRISTINE == null) PRISTINE = b.clone();
                if (!PATCH_REQUESTED || cd == null) return null; // load pass: capture only
                byte[] out = patch(b);
                if (out != null) PATCH_APPLIED = true;
                return out;
            }
        }, true);
        System.out.println("[bodyagent] armed: capture-on-load, patch-on-retransform (G-BODY rig)");
    }

    public static void retransformNow(Class<?> k) throws Exception {
        PATCH_REQUESTED = true;
        INST.retransformClasses(k);
    }

    public static boolean applied() { return PATCH_APPLIED; }

    public static int pristineLen() { byte[] p = PRISTINE; return p == null ? -1 : p.length; }

    /** Whole-body replacement of getValue(DDDDDDZ)D — straight-line dispatch. */
    static byte[] patch(byte[] src) {
        try {
            ClassReader cr = new ClassReader(src);
            ClassWriter cw = new ClassWriter(cr, ClassWriter.COMPUTE_MAXS);
            cr.accept(new ClassVisitor(Opcodes.ASM9, cw) {
                @Override
                public MethodVisitor visitMethod(int acc, String name, String desc,
                                                 String sig, String[] ex) {
                    MethodVisitor mv = super.visitMethod(acc, name, desc, sig, ex);
                    if (!"getValue".equals(name) || !"(DDDDDZ)D".equals(desc)) return mv; // 5 doubles + boolean
                    // Write the replacement body at visitCode; every later
                    // visitor callback of the ORIGINAL body is a no-op
                    // (null delegate — ASM default), so the whole loop
                    // region and its frames are dropped.
                    // NOTE: capture the writer in a differently-named final —
                    // inside the anonymous MethodVisitor the inherited field
                    // `mv` (null delegate) shadows the outer local.
                    final MethodVisitor body = mv;
                    return new MethodVisitor(Opcodes.ASM9, null) {
                        @Override
                        public void visitCode() {
                            body.visitCode();
                            body.visitVarInsn(Opcodes.ALOAD, 0);
                            body.visitVarInsn(Opcodes.DLOAD, 1);
                            body.visitVarInsn(Opcodes.DLOAD, 3);
                            body.visitVarInsn(Opcodes.DLOAD, 5);
                            body.visitVarInsn(Opcodes.DLOAD, 7);
                            body.visitVarInsn(Opcodes.DLOAD, 9);
                            body.visitVarInsn(Opcodes.ILOAD, 11);
                            body.visitMethodInsn(Opcodes.INVOKESTATIC,
                                "net/minecraft/world/level/levelgen/synth/BodyDispatch",
                                "getValue",
                                "(Lnet/minecraft/world/level/levelgen/synth/PerlinNoise;DDDDDZ)D",
                                false);
                            body.visitInsn(Opcodes.DRETURN);
                            body.visitMaxs(0, 0); // recomputed by COMPUTE_MAXS
                            body.visitEnd();
                        }
                    };
                }
            }, 0);
            return cw.toByteArray();
        } catch (Throwable t) {
            System.out.println("[bodyagent] patch FAILED: " + t);
            return null;
        }
    }
}
