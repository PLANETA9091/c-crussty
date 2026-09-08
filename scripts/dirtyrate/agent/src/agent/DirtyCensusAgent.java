package agent;

import java.io.*;
import java.lang.instrument.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.*;
import java.security.ProtectionDomain;
import java.util.concurrent.atomic.AtomicLongArray;

/**
 * TASK-90 hopper-inventory dirty-rate census agent (phase-2 impl of TASK-84 spec,
 * docs/DIRTY_RATE_CENSUS_TOOLING.md — agent-7625532f, 2026-09-09).
 *
 * Dormant byte-identity: unless env CRUSSTY_DIRTY_CENSUS is set to "1"/"on",
 * NO transformer is registered and the agent does nothing (same discipline as
 * FluidPushGuardHook / CRUSSTY_DIRTY_CENSUS default-OFF in the TASK-84 spec).
 *
 * Instrument: ClassFileTransformer (ASM 9.7) inserting ONE static counter call
 * at method ENTRY of each pre-registered probe target — plain long increment
 * into a static AtomicLongArray, no allocation, no boxing, no behavior change
 * (no branching on counters, no returns altered, no args read).
 *
 * Probes (verified via javap against the running purpur-1.21.10.jar):
 *  C0 MUTATION  net/minecraft/world/level/block/entity/BlockEntity
 *                 setChanged ()V    (mod-count signal; ALL BE types — documented
 *                                    upper bound for hopper dirty%)
 *  C1 QUERY     .../HopperBlockEntity tryMoveItems (Lnet/minecraft/world/level/Level;
 *                 Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;
 *                 Lnet/minecraft/world/level/block/entity/HopperBlockEntity;
 *                 Ljava/util/function/BooleanSupplier;)Z
 *  C2 QUERY     .../HopperBlockEntity suckInItems (Lnet/minecraft/world/level/Level;
 *                 Lnet/minecraft/world/level/block/entity/Hopper;)Z
 *  C3 QUERY*    .../HopperBlockEntity pushItemsTick (Lnet/minecraft/world/level/Level;
 *                 Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;
 *                 Lnet/minecraft/world/level/block/entity/HopperBlockEntity;)V  (*spec-plus)
 *
 * TSV dump: daemon thread, every 30s, cumulative lines "epoch_s\tsurface\tcounter\tvalue"
 * to ${CRUSSTY_DIRTY_OUT:-/tmp/dirty_census.tsv} + shutdown-hook final flush (spec §1).
 */
public final class DirtyCensusAgent {

    static final String[][] PROBES = {
        {"net/minecraft/world/level/block/entity/BlockEntity",
         "setChanged", "()V", "0"},
        {"net/minecraft/world/level/block/entity/HopperBlockEntity",
         "tryMoveItems",
         "(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;"
         + "Lnet/minecraft/world/level/block/state/BlockState;"
         + "Lnet/minecraft/world/level/block/entity/HopperBlockEntity;"
         + "Ljava/util/function/BooleanSupplier;)Z",
         "1"},
        {"net/minecraft/world/level/block/entity/HopperBlockEntity",
         "suckInItems",
         "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/entity/Hopper;)Z",
         "2"},
        {"net/minecraft/world/level/block/entity/HopperBlockEntity",
         "pushItemsTick",
         "(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;"
         + "Lnet/minecraft/world/level/block/state/BlockState;"
         + "Lnet/minecraft/world/level/block/entity/HopperBlockEntity;)V",
         "3"},
    };
    static final String[] LABELS = {"M_besetchanged_all", "Q_hopper_trymove",
                                    "Q_hopper_suckin", "Q_hopper_pushtick"};

    public static void premain(String args, Instrumentation inst) {
        String gate = System.getenv("CRUSSTY_DIRTY_CENSUS");
        if (!"1".equals(gate) && !"on".equalsIgnoreCase(gate)) {
            System.out.println("[dirty-census] dormant (CRUSSTY_DIRTY_CENSUS unset) — no transformer, byte-identity");
            return;
        }
        inst.addTransformer(new ProbeTransformer(), false);
        StaticCounter.start();
        System.out.println("[dirty-census] armed — 4 entry probes, TSV "
            + System.getenv().getOrDefault("CRUSSTY_DIRTY_OUT", "/tmp/dirty_census.tsv"));
    }

    static final class ProbeTransformer implements ClassFileTransformer {
        static final java.util.Set<String> SEEN = java.util.Collections.synchronizedSet(new java.util.HashSet<>());
        @Override
        public byte[] transform(ClassLoader loader, String cn, Class<?> being,
                                ProtectionDomain pd, byte[] buf) {
            for (String[] p : PROBES) {
                if (p[0].equals(cn)) {
                    if (SEEN.add(cn)) System.out.println("[dirty-census] SAW target class: " + cn
                        + " loader=" + loader);
                    try {
                        return weave(buf, cn);
                    } catch (Throwable t) {
                        System.err.println("[dirty-census] weave FAILED for " + cn + ": " + t);
                        return null; // fail-open: class loads unmodified, census loses one probe
                    }
                }
            }
            return null;
        }
    }

    static byte[] weave(byte[] buf, final String cn) throws IOException {
        org.objectweb.asm.ClassReader cr = new org.objectweb.asm.ClassReader(buf);
        org.objectweb.asm.ClassWriter cw =
            new org.objectweb.asm.ClassWriter(cr, org.objectweb.asm.ClassWriter.COMPUTE_MAXS);
        cr.accept(new org.objectweb.asm.ClassVisitor(org.objectweb.asm.Opcodes.ASM9, cw) {
            @Override
            public org.objectweb.asm.MethodVisitor visitMethod(int acc, String name, String desc,
                                                               String sig, String[] exc) {
                org.objectweb.asm.MethodVisitor mv = super.visitMethod(acc, name, desc, sig, exc);
                for (String[] p : PROBES) {
                    if (p[0].equals(cn) && p[1].equals(name) && p[2].equals(desc)) {
                        final int ord = Integer.parseInt(p[3]);
                        final org.objectweb.asm.MethodVisitor base = mv;
                        return new org.objectweb.asm.MethodVisitor(org.objectweb.asm.Opcodes.ASM9, base) {
                            @Override public void visitCode() {
                                super.visitCode();
                                super.visitLdcInsn((Integer) ord);
                                super.visitMethodInsn(org.objectweb.asm.Opcodes.INVOKESTATIC,
                                    "agent/DirtyCensusAgent$StaticCounter", "N", "(I)V", false);
                            }
                        };
                    }
                }
                return mv;
            }
        }, 0);
        return cw.toByteArray();
    }

    /** Static counter bank + 30s TSV dumper. Pure Java, no JNI. */
    public static final class StaticCounter {
        static final AtomicLongArray COUNTERS = new AtomicLongArray(8);
        static volatile Path out;

        public static void N(int ordinal) { COUNTERS.incrementAndGet(ordinal); }

        static void start() {
            String p = System.getenv().getOrDefault("CRUSSTY_DIRTY_OUT", "/tmp/dirty_census.tsv");
            out = Paths.get(p);
            flush("init");
            Thread t = new Thread(() -> {
                while (true) {
                    try { Thread.sleep(30_000); } catch (InterruptedException e) { return; }
                    flush("tick");
                }
            }, "dirty-census-dumper");
            t.setDaemon(true);
            t.start();
            Runtime.getRuntime().addShutdownHook(new Thread(() -> flush("shutdown"), "dirty-census-flush"));
        }

        static void flush(String phase) {
            if (out == null) return;
            try {
                long ep = System.currentTimeMillis() / 1000L;
                // TASK-84 analyzer contract (scripts/dirtyrate/analyze_dirtyrate.py):
                // per-SURFACE rows with counter literally "query"/"mutation",
                // APPEND mode (analyzer needs multi-timestamp history in one file).
                long c0 = COUNTERS.get(0); // BlockEntity.setChanged (all BEs)
                long c1 = COUNTERS.get(1); // tryMoveItems
                long c2 = COUNTERS.get(2); // suckInItems
                long c3 = COUNTERS.get(3); // pushItemsTick (spec-plus)
                StringBuilder sb = new StringBuilder();
                // spec-exact surface pair (TASK-84 §2 hopper/inventory):
                //   MUTATION = setChanged (all-BE, documented upper bound)
                //   QUERY    = tryMoveItems + suckInItems
                sb.append(ep).append("\thopper-inventory\tmutation\t").append(c0).append('\n');
                sb.append(ep).append("\thopper-inventory\tquery\t").append(c1 + c2).append('\n');
                // spec-plus extension surface (push-side scan attempts, separate row)
                sb.append(ep).append("\thopper-push-tick\tquery\t").append(c3).append('\n');
                sb.append(ep).append("\tMETA\tphase\t").append(phase).append('\n');
                Files.write(out, sb.toString().getBytes(StandardCharsets.UTF_8),
                    StandardOpenOption.CREATE, StandardOpenOption.WRITE, StandardOpenOption.APPEND);
            } catch (Throwable t) { /* census must never crash the server */ }
        }
    }
}
