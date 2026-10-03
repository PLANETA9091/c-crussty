import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.FieldVisitor;
import org.objectweb.asm.Label;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

import java.nio.file.Files;
import java.nio.file.Path;

/**
 * PALETTED-DEMUX build-time patcher (S7-131) — ASM COMPUTE_FRAMES rewrite of
 * the REAL kernel PalettedContainer.class. Offline JVM tool (ECJ + asm jars);
 * NOT a boot, NOT a server process (INJECTS-ONLY discipline).
 *
 * Three deterministic edits (mirror of the PALETTED-DEMUX spec in
 * src/classfile.rs):
 *   1. inject 4 public instance fields (crusstySnap/snapGen/gen/miss);
 *   2. get(int): full body replacement — fast demux path
 *      (snapGen==gen && snap!=null -> vals[demux[index]]) with a fallback
 *      into PalettedContainerOps.get; COMPUTE_FRAMES recomputes the
 *      StackMapTable (the hand-rolled frame was rejected by the verifier);
 *   3. getAndSet(I,T) + set(I,T): prologue (snap=null, gen++ -> odd) +
 *      ORIGINAL body verbatim + epilogue (gen++ -> even, Ops.onWrite).
 *
 * Run with the kernel jar on the classpath (COMPUTE_FRAMES resolves the
 * class hierarchy for frame merging).
 *
 * Usage: java -cp <asm jars>:<kernel jar>:. PalettedPatchTool <in.class> <out.class>
 */
public final class PalettedPatchTool implements Opcodes {

    static final String PC = "net/minecraft/world/level/chunk/PalettedContainer";
    static final String OPS = "net/minecraft/world/level/chunk/PalettedContainerOps";

    public static void main(String[] args) throws Exception {
        byte[] orig = Files.readAllBytes(Path.of(args[0]));
        ClassReader cr = new ClassReader(orig);
        ClassWriter cw = new ClassWriter(cr, ClassWriter.COMPUTE_FRAMES | ClassWriter.COMPUTE_MAXS);

        ClassVisitor cv = new ClassVisitor(ASM9, cw) {
            @Override
            public void visitEnd() {
                // append the 4 injected public fields (zero-init by the JVM)
                super.visitField(ACC_PUBLIC | ACC_VOLATILE | ACC_TRANSIENT, "crusstySnap", "[Ljava/lang/Object;", null, null);
                super.visitField(ACC_PUBLIC | ACC_VOLATILE, "crusstySnapGen", "I", null, null);
                super.visitField(ACC_PUBLIC | ACC_VOLATILE, "crusstyGen", "I", null, null);
                super.visitField(ACC_PUBLIC, "crusstyMiss", "I", null, null);
                super.visitField(ACC_PUBLIC, "crusstyEpoch", "I", null, null);
                super.visitEnd();
            }

            @Override
            public MethodVisitor visitMethod(int access, String name, String desc, String sig, String[] exceptions) {
                MethodVisitor mv = super.visitMethod(access, name, desc, sig, exceptions);
                boolean isGet = name.equals("get") && desc.equals("(I)Ljava/lang/Object;");
                boolean isGas = name.equals("getAndSet") && desc.equals("(ILjava/lang/Object;)Ljava/lang/Object;");
                boolean isSet = name.equals("set") && desc.equals("(ILjava/lang/Object;)V");
                if (!isGet && !isGas && !isSet) return mv;
                return new GuardedMethod(mv, isGet, isGas, isSet);
            }
        };
        cr.accept(cv, 0);
        byte[] out = cw.toByteArray();
        Files.write(Path.of(args[1]), out);
        System.out.println("PalettedPatchTool: " + orig.length + " -> " + out.length + " bytes");
    }

    /** Body replacement / prologue-epilogue wrapping for the three targets. */
    static final class GuardedMethod extends MethodVisitor {
        final boolean isGet, isGas, isSet;
        boolean swallow; // true while emitting the replaced get(int) body

        GuardedMethod(MethodVisitor mv, boolean isGet, boolean isGas, boolean isSet) {
            super(ASM9, mv);
            this.isGet = isGet;
            this.isGas = isGas;
            this.isSet = isSet;
        }

        @Override
        public void visitCode() {
            mv.visitCode();
            if (isGas || isSet) {
                // PROLOGUE: Ops.onMutateStart (release a live count) + gen++ (even -> odd)
                mv.visitVarInsn(ALOAD, 0);
                mv.visitMethodInsn(INVOKESTATIC, OPS, "onMutateStart",
                        "(L" + PC + ";)V", false);
                mv.visitVarInsn(ALOAD, 0);
                mv.visitInsn(DUP);
                mv.visitFieldInsn(GETFIELD, PC, "crusstyGen", "I");
                mv.visitInsn(ICONST_1);
                mv.visitInsn(IADD);
                mv.visitFieldInsn(PUTFIELD, PC, "crusstyGen", "I");
            }
            if (isGet) {
                Label ops = new Label();
                // FAST PATH: locals 0=this, 1=index, 2=snapGen, 3=snap
                // gate: snapGen != 0 && snapGen == gen + 1 && snap != null
                mv.visitVarInsn(ALOAD, 0);
                mv.visitFieldInsn(GETFIELD, PC, "crusstySnapGen", "I");
                mv.visitVarInsn(ISTORE, 2);
                mv.visitVarInsn(ALOAD, 0);
                mv.visitFieldInsn(GETFIELD, PC, "crusstySnap", "[Ljava/lang/Object;");
                mv.visitVarInsn(ASTORE, 3);
                mv.visitVarInsn(ILOAD, 2);
                mv.visitJumpInsn(IFEQ, ops);
                mv.visitVarInsn(ILOAD, 2);
                mv.visitVarInsn(ALOAD, 0);
                mv.visitFieldInsn(GETFIELD, PC, "crusstyGen", "I");
                mv.visitInsn(ICONST_1);
                mv.visitInsn(IADD);
                mv.visitJumpInsn(IF_ICMPNE, ops);
                mv.visitVarInsn(ALOAD, 3);
                mv.visitJumpInsn(IFNULL, ops);
                mv.visitVarInsn(ALOAD, 3);
                mv.visitInsn(ICONST_0);
                mv.visitInsn(AALOAD);
                mv.visitTypeInsn(CHECKCAST, "[I");
                mv.visitVarInsn(ILOAD, 1);
                mv.visitInsn(IALOAD);
                mv.visitVarInsn(ALOAD, 3);
                mv.visitInsn(ICONST_1);
                mv.visitInsn(AALOAD);
                mv.visitTypeInsn(CHECKCAST, "[Ljava/lang/Object;");
                mv.visitInsn(SWAP);
                mv.visitInsn(AALOAD);
                mv.visitInsn(ARETURN);
                // FALLBACK: vanilla-equivalent read + heat counter
                mv.visitLabel(ops);
                mv.visitVarInsn(ALOAD, 0);
                mv.visitVarInsn(ILOAD, 1);
                mv.visitMethodInsn(INVOKESTATIC, OPS, "get",
                        "(L" + PC + ";I)Ljava/lang/Object;", false);
                mv.visitInsn(ARETURN);
                swallow = true; // the original body is fully replaced
            }
        }

        private void epilogue() {
            // gen++ (odd -> even); the count release already happened in the
            // prologue (onMutateStart) — the epilogue stays allocation-free
            // and Ops-free.
            mv.visitVarInsn(ALOAD, 0);
            mv.visitInsn(DUP);
            mv.visitFieldInsn(GETFIELD, PC, "crusstyGen", "I");
            mv.visitInsn(ICONST_1);
            mv.visitInsn(IADD);
            mv.visitFieldInsn(PUTFIELD, PC, "crusstyGen", "I");
        }

        @Override
        public void visitFieldInsn(int opcode, String owner, String name, String desc) {
            if (swallow) return;
            mv.visitFieldInsn(opcode, owner, name, desc);
        }

        @Override
        public void visitMethodInsn(int opcode, String owner, String name, String desc, boolean itf) {
            if (swallow) return;
            mv.visitMethodInsn(opcode, owner, name, desc, itf);
        }

        @Override
        public void visitInsn(int opcode) {
            if (swallow) return;
            if (isGas && opcode == ARETURN) {
                epilogue();
                mv.visitInsn(ARETURN);
            } else if (isSet && opcode == RETURN) {
                epilogue();
                mv.visitInsn(RETURN);
            } else {
                mv.visitInsn(opcode);
            }
        }

        @Override
        public void visitLabel(Label label) {
            if (swallow) return;
            mv.visitLabel(label);
        }

        @Override
        public void visitFrame(int type, int numLocal, Object[] local, int numStack, Object[] stack) {
            if (swallow) return;
            mv.visitFrame(type, numLocal, local, numStack, stack);
        }

        @Override
        public void visitLineNumber(int line, Label start) {
            if (swallow) return;
            mv.visitLineNumber(line, start);
        }

        @Override
        public void visitLocalVariable(String name, String desc, String sig, Label start, Label end, int index) {
            if (swallow) return;
            mv.visitLocalVariable(name, desc, sig, start, end, index);
        }

        @Override
        public void visitTryCatchBlock(Label start, Label end, Label handler, String type) {
            if (swallow) return;
            mv.visitTryCatchBlock(start, end, handler, type);
        }

        @Override
        public void visitMaxs(int maxStack, int maxLocals) {
            mv.visitMaxs(maxStack, maxLocals); // COMPUTE_MAXS/FRAMES recomputes
        }
    }
}
