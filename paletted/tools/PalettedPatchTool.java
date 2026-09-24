import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.Label;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.Map;

/**
 * PALETTED-DEMUX build-time patcher (S7-131; write-path guard v3 = TASK-437-B
 * cmp436_pdemux) — ASM COMPUTE_FRAMES rewrite of the REAL kernel
 * PalettedContainer.class. Offline JVM tool (ECJ + asm jars); NOT a boot, NOT
 * a server process (INJECTS-ONLY discipline).
 *
 * Four deterministic edits (mirror of the PALETTED-DEMUX spec in
 * src/classfile.rs — the rust patcher encodes the SAME protocol v3):
 *   1. inject 5 public instance fields (crusstySnap/snapGen/gen/miss/epoch);
 *   2. get(int): full body replacement — fast demux path
 *      (snapGen!=0 && snapGen==gen+1 && snap!=null -> vals[demux[index]])
 *      with a fallback into PalettedContainerOps.get; COMPUTE_FRAMES
 *      recomputes the StackMapTable (the hand-rolled frame was rejected by
 *      the verifier). UNCHANGED by v3 — the read path is the proven lane;
 *   3. getAndSet(I,T) + set(I,T): WRITE-PATH GUARD v3. The mutation
 *      prologue/epilogue used to pay 2 volatile gen++ stores (plus an
 *      unconditional onMutateStart call) on EVERY mutation. v3 moves the
 *      gate into two tiny synthetic private methods so the mutator body
 *      itself stays branch-free (no new StackMapTable needed):
 *        keep = crusstyWriteGate();   // inline gate + blacklist skip
 *        <ORIGINAL body verbatim>
 *        crusstyEpilogue(keep);       // gen++ only when the gate bumped
 *      crusstyWriteGate(): (a) live snapshot (snapGen!=0) -> onMutateStart
 *      release + gen++ (even->odd), keep=1 — in-flight readers are
 *      invalidated exactly like v2; (b) snapGen==0 && crusstyEpoch<2 ->
 *      machinery still materializes, keep the v2 protocol, gen++, keep=1;
 *      (c) snapGen==0 && crusstyEpoch>=2 (release-blacklisted, write-heavy)
 *      -> no snapshot can be live and none can publish again
 *      (PalettedContainerOps.tryMaterialize aborts when crusstyEpoch moved
 *      since entry), so the volatile gen++ PAIR is observably dead — skip
 *      it. Steady-state write-hot cost drops from 2 volatile stores +
 *      static call to 1 volatile load + 1 plain load.
 *      crusstyEpilogue(keep): gen++ (odd->even) only when keep==1 — the
 *      parity invariant (prologue bumped => epilogue bumps) is what keeps
 *      gen even outside mutation epochs.
 *   4. the two synthetic private methods carry their own StackMapTables
 *      (COMPUTE_FRAMES builds them; the rust mirror hand-builds the same
 *      full_frames).
 *
 * Run with the kernel jar on the classpath (COMPUTE_FRAMES resolves the
 * class hierarchy for frame merging).
 *
 * Usage: java -cp <asm jar>:<kernel jar>:. PalettedPatchTool <in.class> <out.class>
 */
public final class PalettedPatchTool implements Opcodes {

    static final String PC = "net/minecraft/world/level/chunk/PalettedContainer";
    static final String OPS = "net/minecraft/world/level/chunk/PalettedContainerOps";
    static final String GATE_METHOD = "crusstyWriteGate";
    static final String GATE_DESC = "()I";
    static final String EPI_METHOD = "crusstyEpilogue";
    static final String EPI_DESC = "(I)V";
    /** Release-blacklist threshold — MUST equal
     * PalettedContainerOps.MAX_BUILDS_PER_CONTAINER (2). Hardcoded here
     * because the gate runs per-mutation; a cross-reference assertion lives
     * in the javap gate below (check both sources when touching either). */
    static final int MAX_BUILDS = 2;

    public static void main(String[] args) throws Exception {
        byte[] orig = Files.readAllBytes(Path.of(args[0]));

        // Pre-pass: record the ORIGINAL maxLocals of both guarded mutators —
        // the keep-flag local must land one slot ABOVE every original local.
        Map<String, Integer> maxLocalsOf = new HashMap<>();
        ClassReader probe = new ClassReader(orig);
        probe.accept(new ClassVisitor(ASM9, null) {
            @Override
            public MethodVisitor visitMethod(int access, String name, String desc, String sig, String[] exceptions) {
                boolean isGas = name.equals("getAndSet") && desc.equals("(ILjava/lang/Object;)Ljava/lang/Object;");
                boolean isSet = name.equals("set") && desc.equals("(ILjava/lang/Object;)V");
                if (!isGas && !isSet) return null;
                String key = name + desc;
                return new MethodVisitor(ASM9, null) {
                    @Override
                    public void visitMaxs(int maxStack, int maxLocals) {
                        maxLocalsOf.put(key, maxLocals);
                    }
                };
            }
        }, 0);
        Integer gasLocals = maxLocalsOf.get("getAndSet(ILjava/lang/Object;)Ljava/lang/Object;");
        Integer setLocals = maxLocalsOf.get("set(ILjava/lang/Object;)V");
        if (gasLocals == null || setLocals == null) {
            throw new IllegalStateException("kernel shape drift: guarded mutators missing (pre-pass)");
        }

        ClassReader cr = new ClassReader(orig);
        ClassWriter cw = new ClassWriter(cr, ClassWriter.COMPUTE_FRAMES | ClassWriter.COMPUTE_MAXS);

        ClassVisitor cv = new ClassVisitor(ASM9, cw) {
            @Override
            public void visitEnd() {
                // append the 5 injected public fields (zero-init by the JVM)
                super.visitField(ACC_PUBLIC | ACC_VOLATILE | ACC_TRANSIENT, "crusstySnap", "[Ljava/lang/Object;", null, null);
                super.visitField(ACC_PUBLIC | ACC_VOLATILE, "crusstySnapGen", "I", null, null);
                super.visitField(ACC_PUBLIC | ACC_VOLATILE, "crusstyGen", "I", null, null);
                super.visitField(ACC_PUBLIC, "crusstyMiss", "I", null, null);
                // crusstyEpoch is protocol-volatile (v3): the writer gate's
                // skip condition (snapGen==0 && epoch>=2) and the
                // materializer's epoch-transition validation both depend on
                // fresh epoch reads — a stale plain read could publish a
                // snapshot into the skip state. Volatile loads are free on
                // x86, and the field is written once per build (rare).
                super.visitField(ACC_PUBLIC | ACC_VOLATILE, "crusstyEpoch", "I", null, null);
                emitWriteGate();
                emitEpilogue();
                super.visitEnd();
            }

            /** private synthetic int crusstyWriteGate() — the v3 mutation
             * gate: release+invalidate when a snapshot is live, keep the
             * protocol while materialization is alive, skip the volatile
             * pair on release-blacklisted (write-heavy) containers. */
            private void emitWriteGate() {
                MethodVisitor mv = super.visitMethod(ACC_PRIVATE | ACC_SYNTHETIC, GATE_METHOD, GATE_DESC, null, null);
                mv.visitCode();
                Label alive = new Label();
                Label skip = new Label();
                mv.visitVarInsn(ALOAD, 0);
                mv.visitFieldInsn(GETFIELD, PC, "crusstySnapGen", "I");
                mv.visitJumpInsn(IFEQ, alive);
                // live snapshot: release + gen++ (even -> odd), keep=1
                mv.visitVarInsn(ALOAD, 0);
                mv.visitMethodInsn(INVOKESTATIC, OPS, "onMutateStart", "(L" + PC + ";)V", false);
                emitGenInc(mv);
                mv.visitInsn(ICONST_1);
                mv.visitInsn(IRETURN);
                mv.visitLabel(alive);
                // machinery alive (epoch < MAX_BUILDS): keep the v2 protocol
                mv.visitVarInsn(ALOAD, 0);
                mv.visitFieldInsn(GETFIELD, PC, "crusstyEpoch", "I");
                mv.visitInsn(ICONST_2); // == PalettedContainerOps.MAX_BUILDS_PER_CONTAINER
                mv.visitJumpInsn(IF_ICMPGE, skip);
                emitGenInc(mv);
                mv.visitInsn(ICONST_1);
                mv.visitInsn(IRETURN);
                mv.visitLabel(skip);
                // dead machinery: snapshot pair observably absent -> keep=0
                mv.visitInsn(ICONST_0);
                mv.visitInsn(IRETURN);
                mv.visitMaxs(0, 0); // COMPUTE_FRAMES/COMPUTE_MAXS
                mv.visitEnd();
            }

            /** private synthetic void crusstyEpilogue(int keep) — gen++
             * (odd -> even) only when the gate bumped in the prologue. */
            private void emitEpilogue() {
                MethodVisitor mv = super.visitMethod(ACC_PRIVATE | ACC_SYNTHETIC, EPI_METHOD, EPI_DESC, null, null);
                mv.visitCode();
                Label done = new Label();
                mv.visitVarInsn(ILOAD, 1);
                mv.visitJumpInsn(IFEQ, done);
                emitGenInc(mv);
                mv.visitLabel(done);
                mv.visitInsn(RETURN);
                mv.visitMaxs(0, 0); // COMPUTE_FRAMES/COMPUTE_MAXS
                mv.visitEnd();
            }

            @Override
            public MethodVisitor visitMethod(int access, String name, String desc, String sig, String[] exceptions) {
                MethodVisitor mv = super.visitMethod(access, name, desc, sig, exceptions);
                boolean isGet = name.equals("get") && desc.equals("(I)Ljava/lang/Object;");
                boolean isGas = name.equals("getAndSet") && desc.equals("(ILjava/lang/Object;)Ljava/lang/Object;");
                boolean isSet = name.equals("set") && desc.equals("(ILjava/lang/Object;)V");
                if (isGas) return new GuardedMethod(mv, false, true, maxLocalsOf.get("getAndSet(ILjava/lang/Object;)Ljava/lang/Object;"));
                if (isSet) return new GuardedMethod(mv, false, false, maxLocalsOf.get("set(ILjava/lang/Object;)V"));
                if (!isGet) return mv;
                return new GuardedMethod(mv, true, false, 0);
            }
        };
        cr.accept(cv, 0);
        byte[] out = cw.toByteArray();
        Files.write(Path.of(args[1]), out);
        System.out.println("PalettedPatchTool: " + orig.length + " -> " + out.length + " bytes (v3 write-path guard)");
    }

    /** aload_0; dup; getfield crusstyGen; iconst_1; iadd; putfield crusstyGen. */
    static void emitGenInc(MethodVisitor mv) {
        mv.visitVarInsn(ALOAD, 0);
        mv.visitInsn(DUP);
        mv.visitFieldInsn(GETFIELD, PC, "crusstyGen", "I");
        mv.visitInsn(ICONST_1);
        mv.visitInsn(IADD);
        mv.visitFieldInsn(PUTFIELD, PC, "crusstyGen", "I");
    }

    /** Body replacement (get) / gate-wrapper (mutators). */
    static final class GuardedMethod extends MethodVisitor {
        final boolean isGet, isGas;
        final int keepSlot; // local slot above every original local (mutators)
        boolean swallow; // true while emitting the replaced get(int) body

        GuardedMethod(MethodVisitor mv, boolean isGet, boolean isGas, int origMaxLocals) {
            super(ASM9, mv);
            this.isGet = isGet;
            this.isGas = isGas;
            this.keepSlot = origMaxLocals; // one past the last original slot
            if (!isGet && origMaxLocals <= 0) {
                throw new IllegalStateException("keep-flag slot unresolved for mutator");
            }
        }

        @Override
        public void visitCode() {
            mv.visitCode();
            if (isGas || !isGet) { // mutator: keep = crusstyWriteGate();
                mv.visitVarInsn(ALOAD, 0);
                mv.visitMethodInsn(INVOKEVIRTUAL, PC, GATE_METHOD, GATE_DESC, false);
                mv.visitVarInsn(ISTORE, keepSlot);
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

        @Override
        public void visitInsn(int opcode) {
            if (swallow) return;
            if (isGas && opcode == ARETURN) {
                // crusstyEpilogue(keep); return old;
                mv.visitVarInsn(ALOAD, 0);
                mv.visitVarInsn(ILOAD, keepSlot);
                mv.visitMethodInsn(INVOKEVIRTUAL, PC, EPI_METHOD, EPI_DESC, false);
                mv.visitInsn(ARETURN);
            } else if (!isGet && !isGas && opcode == RETURN) {
                mv.visitVarInsn(ALOAD, 0);
                mv.visitVarInsn(ILOAD, keepSlot);
                mv.visitMethodInsn(INVOKEVIRTUAL, PC, EPI_METHOD, EPI_DESC, false);
                mv.visitInsn(RETURN);
            } else {
                mv.visitInsn(opcode);
            }
        }

        // ---- swallow overrides: the replaced get(int) body must not leak
        // the original instructions behind the emitted replacement ----
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
        public void visitIincInsn(int var, int increment) {
            if (swallow) return;
            mv.visitIincInsn(var, increment);
        }

        @Override
        public void visitTypeInsn(int opcode, String type) {
            if (swallow) return;
            mv.visitTypeInsn(opcode, type);
        }

        @Override
        public void visitIntInsn(int opcode, int operand) {
            if (swallow) return;
            mv.visitIntInsn(opcode, operand);
        }

        @Override
        public void visitVarInsn(int opcode, int var) {
            if (swallow) return;
            mv.visitVarInsn(opcode, var);
        }

        @Override
        public void visitJumpInsn(int opcode, Label label) {
            if (swallow) return;
            mv.visitJumpInsn(opcode, label);
        }

        @Override
        public void visitLdcInsn(Object cst) {
            if (swallow) return;
            mv.visitLdcInsn(cst);
        }

        @Override
        public void visitTableSwitchInsn(int min, int max, Label dflt, Label... labels) {
            if (swallow) return;
            mv.visitTableSwitchInsn(min, max, dflt, labels);
        }

        @Override
        public void visitLookupSwitchInsn(Label dflt, int[] keys, Label[] labels) {
            if (swallow) return;
            mv.visitLookupSwitchInsn(dflt, keys, labels);
        }

        @Override
        public void visitMultiANewArrayInsn(String desc, int dims) {
            if (swallow) return;
            mv.visitMultiANewArrayInsn(desc, dims);
        }

        @Override
        public void visitMaxs(int maxStack, int maxLocals) {
            mv.visitMaxs(maxStack, maxLocals); // COMPUTE_MAXS/FRAMES recomputes
        }
    }
}
