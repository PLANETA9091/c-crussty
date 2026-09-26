import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

import java.nio.file.Files;
import java.nio.file.Paths;

/**
 * Mr167Patch — Tuinity/Moonrise #167 parity fix (upstream PR #188, d71d640a):
 * ChunkEntitySlices$EntityCollectionBySection.getEntities / getEntitiesLimited
 *   min section: floor(box.minY - 2.0) >> 4  ->  floor(box.minY - 4.0) >> 4
 *   max section: floor(box.maxY + 2.0) >> 4  ->  floor(box.maxY + 0.0) >> 4
 * Size-preserving LDC2_W->LDC2_W constant swap (frames/maxs reused verbatim —
 * ClassWriter(0), no COMPUTE_FRAMES/COMPUTE_MAXS).
 *
 * Core-ASM visitor with a one-instruction delay slot: an LDC2_W(Double 2.0) is
 * held; when the next simple opcode arrives, DSUB(0x67) -> emit 4.0,
 * DADD(0x63) -> emit 0.0, else flush 2.0 unchanged. Any other visit* callback
 * flushes the delay slot first, so the transform is bytecode-layout agnostic.
 *
 * Usage: java -cp asm.jar:. Mr167Patch <in.class> <out.class>
 * Exit 0 + site report; non-zero on drift.
 */
public class Mr167Patch {

    static int patched = 0;

    static class Fixer extends ClassVisitor {
        Fixer(ClassVisitor cv) { super(Opcodes.ASM9, cv); }

        @Override
        public MethodVisitor visitMethod(int access, String name, String desc,
                                         String signature, String[] exceptions) {
            MethodVisitor mv = super.visitMethod(access, name, desc, signature, exceptions);
            boolean target = name.equals("getEntities")
                    || name.equals("getEntitiesLimited");
            return target ? new DelayedLdc(mv, name, desc) : mv;
        }
    }

    /** one-instruction delay slot around LDC2_W(2.0) so the next opcode decides */
    static class DelayedLdc extends MethodVisitor {
        private final String mname, mdesc;
        private Double pending; // non-null => hold this LDC until next opcode
        private int sites = 0;

        DelayedLdc(MethodVisitor mv, String name, String desc) {
            super(Opcodes.ASM9, mv);
            this.mname = name; this.mdesc = desc;
        }

        private void flush() {
            if (pending != null) {
                emitLdc(pending);
                pending = null;
            }
        }

        private void emitLdc(double v) {
            super.visitLdcInsn(v); // ASM emits LDC2_W for doubles — 3 bytes, size-preserving
        }

        @Override
        public void visitLdcInsn(Object cst) {
            flush();
            if (cst instanceof Double && ((Double) cst).doubleValue() == 2.0) {
                pending = (Double) cst;
                return;
            }
            super.visitLdcInsn(cst);
        }

        @Override
        public void visitInsn(int opcode) {
            if (pending != null) {
                double v;
                String what;
                if (opcode == 0x67) { // DSUB (minY site)
                    v = 4.0; what = "minY -4.0";
                } else if (opcode == 0x63) { // DADD (maxY site)
                    v = 0.0; what = "maxY +0.0";
                } else {
                    v = 2.0; what = "unchanged (next=0x" + Integer.toHexString(opcode) + ")";
                }
                super.visitLdcInsn(v);
                sites++;
                System.out.printf("patched: %s%s site#%d %s%n", mname, mdesc, sites, what);
                patched++;
                pending = null;
            }
            super.visitInsn(opcode);
        }

        @Override public void visitFrame(int t, int n, Object[] l, int m, Object[] s) { flush(); super.visitFrame(t, n, l, m, s); }
        @Override public void visitVarInsn(int o, int v) { flush(); super.visitVarInsn(o, v); }
        @Override public void visitTypeInsn(int o, String t) { flush(); super.visitTypeInsn(o, t); }
        @Override public void visitFieldInsn(int o, String c, String n, String d) { flush(); super.visitFieldInsn(o, c, n, d); }
        @Override public void visitMethodInsn(int o, String c, String n, String d, boolean itf) { flush(); super.visitMethodInsn(o, c, n, d, itf); }
        @Override public void visitJumpInsn(int o, org.objectweb.asm.Label l) { flush(); super.visitJumpInsn(o, l); }
        @Override public void visitLabel(org.objectweb.asm.Label l) { flush(); super.visitLabel(l); }
        @Override public void visitIntInsn(int o, int v) { flush(); super.visitIntInsn(o, v); }
        @Override public void visitIincInsn(int v, int i) { flush(); super.visitIincInsn(v, i); }
        @Override public void visitTableSwitchInsn(int min, int max, org.objectweb.asm.Label dflt, org.objectweb.asm.Label... lbls) { flush(); super.visitTableSwitchInsn(min, max, dflt, lbls); }
        @Override public void visitLookupSwitchInsn(org.objectweb.asm.Label dflt, int[] keys, org.objectweb.asm.Label[] lbls) { flush(); super.visitLookupSwitchInsn(dflt, keys, lbls); }
        @Override public void visitMultiANewArrayInsn(String desc, int dims) { flush(); super.visitMultiANewArrayInsn(desc, dims); }
        @Override public void visitInvokeDynamicInsn(String n, String d, org.objectweb.asm.Handle b, Object... a) { flush(); super.visitInvokeDynamicInsn(n, d, b, a); }
        @Override public void visitEnd() {
            if (sites != 2)
                throw new IllegalStateException(mname + mdesc + ": expected 2 ldc2_w-2.0 sites, found " + sites + " — kernel drift, refuse");
            flush();
            super.visitEnd();
        }
    }

    public static void main(String[] args) throws Exception {
        byte[] in = Files.readAllBytes(Paths.get(args[0]));
        ClassReader cr = new ClassReader(in);
        ClassWriter cw = new ClassWriter(0);
        cr.accept(new Fixer(cw), 0);
        if (patched != 4)
            throw new IllegalStateException("total sites " + patched + " != 4 — kernel drift, refuse");
        byte[] out = cw.toByteArray();
        Files.write(Paths.get(args[1]), out);
        // NOTE: whole-file size may drift (+CP repack by ASM: new CONSTANT_Double
        // 4.0/0.0 entries). The invariant that matters is METHOD-BODY layout:
        // LDC2_W->LDC2_W keeps every instruction 3 bytes, so code_length,
        // pc offsets and StackMapTable stay valid — verified by javap pc-diff
        // in scripts/mr167_javap_gate.sh (pre pc 0..209 / 0..223 must match).
        System.out.printf("Mr167Patch: OK — %d sites; file size %d -> %d (CP repack only)%n",
                patched, in.length, out.length);
    }
}
