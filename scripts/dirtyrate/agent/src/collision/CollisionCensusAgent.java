package collision;

import java.io.*;
import java.lang.instrument.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.*;
import java.security.ProtectionDomain;
import java.util.concurrent.atomic.AtomicLongArray;

/**
 * TASK-104 collision dirty-rate census agent (phase-2 impl of TASK-84 phase-1 third
 * surface, docs/COLLISION_CENSUS_DESIGN.md — S7-46 main, 2026-09-09). Structurally a
 * copy of the TASK-90 hopper agent (agent.DirtyCensusAgent) with the §1 probe table;
 * kept in a separate package so the proven hopper artifact stays byte-identical.
 *
 * Dormant byte-identity: unless env CRUSSTY_DIRTY_CENSUS is set to "1"/"on",
 * NO transformer is registered and the agent does nothing.
 *
 * Probes (javap-verified on the mojang-mapped runtime jar
 * /home/z/server/versions/1.21.10/purpur-1.21.10.jar, raw excerpts
 * /tmp/collision_census_findings.md):
 *  0 Q1 QUERY     net/minecraft/world/entity/Entity collide
 *                   (Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
 *                 -- THE swept-collision query, per move() resolution
 *  1 Q2 QUERY     net/minecraft/world/level/BlockCollisions computeNext ()Ljava/lang/Object;
 *                 -- one call per candidate block/shape visited (density-sensitive);
 *                    NOTE: CollisionSpliterator does NOT exist in 1.21.10 (stale-doc trap)
 *  2 Q3 QUERY     net/minecraft/world/phys/shapes/Shapes collide
 *                   (Lnet/minecraft/core/Direction$Axis;Lnet/minecraft/world/phys/AABB;
 *                    Ljava/lang/Iterable;D)D
 *                 -- per-axis resolution, 3x per collide()
 *  3 Q4 QUERY     net/minecraft/world/entity/Entity move
 *                   (Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V
 *                 -- caller-context mechanism sanity (LivingEntity does NOT override)
 *  4 M1 MUTATION  net/minecraft/world/level/Level setBlock
 *                   (Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z
 *                 -- ALL-block-update upper bound (analog of TASK-90 BlockEntity.setChanged)
 *  5 M2 MUTATION  net/minecraft/world/entity/Entity setPos (DDD)V
 *                 -- entity's own position/AABB change; the OTHER invalidator (position
 *                    is part of collide()'s state input)
 *
 * TSV dump: daemon thread, every 30s, cumulative lines "epoch_s\tsurface\tcounter\tvalue"
 * to ${CRUSSTY_DIRTY_OUT:-/tmp/collision_census.tsv} + shutdown-hook final flush.
 * Surfaces (TASK-84 analyzer contract, counter literally "query"/"mutation"):
 *   collision              mutation = M1+M2   query = Q1     -> primary dirty%
 *   collision-blockiter    query = Q2                        -> amplification (per Q1)
 *   collision-shapes-axis  query = Q3                        -> amplification (per Q1)
 *   collision-move         query = Q4                        -> mechanism sanity
 */
public final class CollisionCensusAgent {

    static final String[][] PROBES = {
        {"net/minecraft/world/entity/Entity",
         "collide",
         "(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;",
         "0"},
        {"net/minecraft/world/level/BlockCollisions",
         "computeNext",
         "()Ljava/lang/Object;",
         "1"},
        {"net/minecraft/world/phys/shapes/Shapes",
         "collide",
         "(Lnet/minecraft/core/Direction$Axis;Lnet/minecraft/world/phys/AABB;"
         + "Ljava/lang/Iterable;D)D",
         "2"},
        {"net/minecraft/world/entity/Entity",
         "move",
         "(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V",
         "3"},
        {"net/minecraft/world/level/Level",
         "setBlock",
         "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z",
         "4"},
        {"net/minecraft/world/entity/Entity",
         "setPos",
         "(DDD)V",
         "5"},
    };

    public static void premain(String args, Instrumentation inst) {
        String gate = System.getenv("CRUSSTY_DIRTY_CENSUS");
        if (!"1".equals(gate) && !"on".equalsIgnoreCase(gate)) {
            System.out.println("[collision-census] dormant (CRUSSTY_DIRTY_CENSUS unset) — no transformer, byte-identity");
            return;
        }
        inst.addTransformer(new ProbeTransformer(), false);
        StaticCounter.start();
        System.out.println("[collision-census] armed — 6 entry probes (Q1-Q4/M1-M2), TSV "
            + System.getenv().getOrDefault("CRUSSTY_DIRTY_OUT", "/tmp/collision_census.tsv"));
    }

    static final class ProbeTransformer implements ClassFileTransformer {
        static final java.util.Set<String> SEEN = java.util.Collections.synchronizedSet(new java.util.HashSet<>());
        @Override
        public byte[] transform(ClassLoader loader, String cn, Class<?> being,
                                ProtectionDomain pd, byte[] buf) {
            for (String[] p : PROBES) {
                if (p[0].equals(cn)) {
                    if (SEEN.add(cn)) System.out.println("[collision-census] SAW target class: " + cn
                        + " loader=" + loader);
                    try {
                        return weave(buf, cn);
                    } catch (Throwable t) {
                        System.err.println("[collision-census] weave FAILED for " + cn + ": " + t);
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
                                    "collision/CollisionCensusAgent$StaticCounter", "N", "(I)V", false);
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
            String p = System.getenv().getOrDefault("CRUSSTY_DIRTY_OUT", "/tmp/collision_census.tsv");
            out = Paths.get(p);
            flush("init");
            Thread t = new Thread(() -> {
                while (true) {
                    try { Thread.sleep(30_000); } catch (InterruptedException e) { return; }
                    flush("tick");
                }
            }, "collision-census-dumper");
            t.setDaemon(true);
            t.start();
            Runtime.getRuntime().addShutdownHook(new Thread(() -> flush("shutdown"), "collision-census-flush"));
        }

        static void flush(String phase) {
            if (out == null) return;
            try {
                long ep = System.currentTimeMillis() / 1000L;
                long q1 = COUNTERS.get(0); // Entity.collide
                long q2 = COUNTERS.get(1); // BlockCollisions.computeNext
                long q3 = COUNTERS.get(2); // Shapes.collide(axis,...)
                long q4 = COUNTERS.get(3); // Entity.move
                long m1 = COUNTERS.get(4); // Level.setBlock
                long m2 = COUNTERS.get(5); // Entity.setPos
                StringBuilder sb = new StringBuilder();
                // primary surface: dirty% = (M1+M2) / ((M1+M2) + Q1) per docs/COLLISION_CENSUS_DESIGN.md §6
                sb.append(ep).append("\tcollision\tmutation\t").append(m1 + m2).append('\n');
                sb.append(ep).append("\tcollision\tquery\t").append(q1).append('\n');
                // amplification / mechanism rows (query-only, reported as ratios by the reader)
                sb.append(ep).append("\tcollision-blockiter\tquery\t").append(q2).append('\n');
                sb.append(ep).append("\tcollision-shapes-axis\tquery\t").append(q3).append('\n');
                sb.append(ep).append("\tcollision-move\tquery\t").append(q4).append('\n');
                sb.append(ep).append("\tMETA\tphase\t").append(phase).append('\n');
                Files.write(out, sb.toString().getBytes(StandardCharsets.UTF_8),
                    StandardOpenOption.CREATE, StandardOpenOption.WRITE, StandardOpenOption.APPEND);
            } catch (Throwable t) { /* census must never crash the server */ }
        }
    }
}
