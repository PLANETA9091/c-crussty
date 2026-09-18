package net.minecraft.world.entity;

import java.io.ByteArrayInputStream;
import java.io.DataInputStream;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.nio.ByteBuffer;
import java.nio.file.Files;
import java.nio.file.Path;
import java.security.MessageDigest;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import java.util.concurrent.atomic.AtomicLongArray;
import java.util.function.Consumer;

import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.level.entity.EntityTickList;

/**
 * REGION-THREADS PG1 lockstep harness (S7-157, lever #7) — the formal
 * OFFLINE lockstep gate preregistered in S7-155: per-entity state
 * bit-exact between vanilla-sequential (W=1, the dormant bridge path =
 * vanilla list.forEach) and region-parallel (W=2, W=4) execution of the
 * SAME deterministic scenario.
 *
 * Scenario (fully deterministic, no wall-clock, no shared RNG):
 *  - 400 base entities (ordinals 1..400) on a chunk lattice, 100 partner
 *    PAIRS co-placed inside one 8-chunk region each (mutual same-bucket
 *    state reads — the PG1 "identical neighbours" domain).
 *  - Per-entity java.util.Random(ordinal*126271+7) — the S7-155 model:
 *    per-tick RNG is per-entity, so tick order across buckets cannot
 *    influence sequences. Branching consumption: 1..3 draws/tick.
 *  - Mutation storm through the REAL retargeted guard sites
 *    (RegionTickOps.onTickingStart/onTickingEnd):
 *      * 7 entities (ordinal % 53 == 0) remove themselves mid-tick at
 *        deterministic ticks 10..29 (deferred drain under W>=2, direct
 *        vanilla remove under W=1);
 *      * 10 parents (ordinal % 37 == 0) spawn 2 children each (t=5, t=15)
 *        at the parent chunk — adds must start ticking NEXT tick in every
 *        config (vanilla never visits mid-iteration adds: BaseIterator
 *        maxIndex is pinned at construction — javap-verified S7-157).
 *  - Digest: SHA-256 over the CANONICAL per-tick state vector (entities
 *    sorted by ordinal; state/draws/tickCount/list-cardinality per tick) —
 *    insertion-order-race immune, per-entity-semantics sensitive.
 *
 * PASS = W=1 digest == W=2 digest == W=4 digest over 60 ticks
 *        (413 final entities: 400 - 7 + 20), no deadlock, drain complete.
 */
public final class RegionLockstepHarness {

    static final int BASE = 400;
    static final int TICKS = 60;
    static final int MAX_ORD = 1201;           // 400 base + <=20 spawns, padded
    static final long SEED = 0x9E3779B97F4A7C15L;

    static void check(boolean cond, String what) {
        if (!cond) throw new AssertionError("REGION-LOCKSTEP FAIL: " + what);
        System.out.println("[lockstep] ok: " + what);
    }

    /** Byte-map loader over the real kernel (the runtime loader protocol). */
    static final class PatchLoader extends ClassLoader {
        private final Map<String, byte[]> overrides;
        PatchLoader(ClassLoader parent, Map<String, byte[]> overrides) {
            super(parent);
            this.overrides = overrides;
        }
        @Override
        protected Class<?> loadClass(String name, boolean resolve)
                throws ClassNotFoundException {
            byte[] bytes = overrides.get(name);
            if (bytes != null) {
                Class<?> c = defineClass(name, bytes, 0, bytes.length);
                if (resolve) resolveClass(c);
                return c;
            }
            return super.loadClass(name, resolve);
        }
    }

    private static sun.misc.Unsafe unsafe() throws Exception {
        Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
        f.setAccessible(true);
        return (sun.misc.Unsafe) f.get(null);
    }

    private static void setIdentity(sun.misc.Unsafe unsafe, Entity e, UUID u,
                                    int cx, int cz) throws Exception {
        Field uuid = Entity.class.getDeclaredField("uuid");
        Field stringUUID = Entity.class.getDeclaredField("stringUUID");
        Field chunkPos = Entity.class.getDeclaredField("chunkPosition");
        uuid.setAccessible(true);
        stringUUID.setAccessible(true);
        chunkPos.setAccessible(true);
        unsafe.putObject(e, unsafe.objectFieldOffset(uuid), u);
        unsafe.putObject(e, unsafe.objectFieldOffset(stringUUID), u.toString());
        unsafe.putObject(e, unsafe.objectFieldOffset(chunkPos),
                new net.minecraft.world.level.ChunkPos(cx, cz));
    }

    private static final ByteBuffer LBUF = ByteBuffer.allocate(8);

    private static void putLong(MessageDigest md, long v) {
        LBUF.rewind();
        LBUF.putLong(v);
        md.update(LBUF.array());
    }

    /** Pair geometry: both members inside one 8-chunk region. */
    private static int[] pairChunk(int p) {
        int cx = ((p * 3) % 7) + 8 * ((p * 5) % 7); // x in region row, never %8==7
        int cz = (p * 11) % 63;
        return new int[]{cx, cz};
    }

    /** One deterministic scenario run at the CURRENT env worker count. */
    private static String runScenario(Method mForEach, Method mAdd, Method mRem,
                                      Method mWorkers) throws Exception {
        int w = (Integer) mWorkers.invoke(null);
        sun.misc.Unsafe unsafe = unsafe();

        final AtomicLongArray state = new AtomicLongArray(MAX_ORD);
        final AtomicLongArray draws = new AtomicLongArray(MAX_ORD);
        final AtomicLongArray ticks = new AtomicLongArray(MAX_ORD);
        final java.util.Random[] rng = new java.util.Random[MAX_ORD];
        final int[] partner = new int[MAX_ORD];
        final boolean[] removed = new boolean[MAX_ORD];
        final int[] chunkX = new int[MAX_ORD];
        final int[] chunkZ = new int[MAX_ORD];
        final Entity[] ents = new Entity[MAX_ORD];

        EntityTickList list = new EntityTickList();
        for (int i = 1; i <= BASE; i++) {
            int[] c = pairChunk((i - 1) / 2);
            chunkX[i] = c[0];
            chunkZ[i] = c[1];
            rng[i] = new java.util.Random(i * 126271L + 7L);
            state.set(i, i * SEED);
            if ((i & 1) == 1 && i + 1 <= BASE) partner[i] = i + 1;
            else if ((i & 1) == 0) partner[i] = i - 1;
            Entity e = (Entity) unsafe.allocateInstance(ItemEntity.class);
            setIdentity(unsafe, e, new UUID(0L, i), c[0], c[1]);
            ents[i] = e;
            list.add(e);
        }

        final EntityTickList flist = list;
        final int[] tickRef = new int[1];
        Consumer<Entity> consumer = e -> {
            int ord = (int) e.getUUID().getLeastSignificantBits();
            if (removed[ord]) return;             // vanilla isRemoved() gate model
            ticks.incrementAndGet(ord);
            java.util.Random r = rng[ord];
            int d = 1 + r.nextInt(3);             // branching RNG consumption
            long s = state.get(ord);
            for (int k = 0; k < d; k++) {
                s = s * SEED + r.nextLong();
            }
            int p = partner[ord];
            if (p != 0) s ^= Long.rotateLeft(state.get(p), ord & 31);
            state.set(ord, s);
            draws.set(ord, draws.get(ord) + d);

            int t = tickRef[0];
            // storm: self-removal through the REAL guard site
            if (t >= 10 && t <= 29 && ord <= BASE && ord % 53 == 0) {
                removed[ord] = true;
                try {
                    mRem.invoke(null, flist, e);
                } catch (Exception ex) {
                    throw new RuntimeException(ex);
                }
            }
            // storm: spawn through the REAL guard site (deterministic child id)
            if (ord <= BASE && ord % 37 == 0 && (t == 5 || t == 15)) {
                int child = BASE + ord * 2 + (t == 5 ? 0 : 1);
                try {
                    Entity ne = (Entity) unsafe.allocateInstance(ItemEntity.class);
                    setIdentity(unsafe, ne, new UUID(0L, child),
                            chunkX[ord], chunkZ[ord]);   // same region as parent
                    ents[child] = ne;
                    rng[child] = new java.util.Random(child * 126271L + 7L);
                    state.set(child, child * SEED);
                    partner[child] = ord;
                    mAdd.invoke(null, flist, ne);
                } catch (Exception ex) {
                    throw new RuntimeException(ex);
                }
            }
        };

        MessageDigest md = MessageDigest.getInstance("SHA-256");
        List<Entity> all = new ArrayList<>();
        for (int t = 1; t <= TICKS; t++) {
            tickRef[0] = t;
            mForEach.invoke(null, list, consumer);   // the retargeted container
            putLong(md, t);
            all.clear();
            list.forEach(all::add);                  // serial digest snapshot
            all.sort((a, b) -> Long.compare(
                    a.getUUID().getLeastSignificantBits(),
                    b.getUUID().getLeastSignificantBits()));
            for (Entity e : all) {
                int ord = (int) e.getUUID().getLeastSignificantBits();
                putLong(md, ord);
                putLong(md, state.get(ord));
                putLong(md, draws.get(ord));
                putLong(md, ticks.get(ord));
            }
            putLong(md, all.size());
        }
        StringBuilder hex = new StringBuilder();
        for (byte b : md.digest()) hex.append(String.format("%02x", b));
        int expected = BASE - 7 + 20;
        check(all.size() == expected,
                "final list cardinality " + all.size() + " == " + expected
                        + " (W=" + w + ")");
        System.out.println("LOCKSTEP DIGEST " + hex + " FINAL " + all.size()
                + " WORKERS " + w);
        return hex.toString();
    }

    private static Class<?> opsClass() throws Exception {
        byte[] ops = Files.readAllBytes(Path.of(
                "entityinside/build/net/minecraft/world/entity/RegionTickOps.class"));
        byte[] mut = Files.readAllBytes(Path.of(
                "entityinside/build/net/minecraft/world/entity/RegionTickOps$Mut.class"));
        PatchLoader loader = new PatchLoader(
                RegionLockstepHarness.class.getClassLoader(),
                Map.of("net.minecraft.world.entity.RegionTickOps", ops,
                       "net.minecraft.world.entity.RegionTickOps$Mut", mut));
        return Class.forName("net.minecraft.world.entity.RegionTickOps", false, loader);
    }

    public static void main(String[] args) throws Exception {
        net.minecraft.SharedConstants.tryDetectVersion();
        net.minecraft.server.Bootstrap.bootStrap();

        Class<?> ops = opsClass();
        Method mForEach = ops.getDeclaredMethod("forEach",
                EntityTickList.class, Consumer.class);
        Method mAdd = ops.getDeclaredMethod("onTickingStart",
                EntityTickList.class, Entity.class);
        Method mRem = ops.getDeclaredMethod("onTickingEnd",
                EntityTickList.class, Entity.class);
        Method mWorkers = ops.getDeclaredMethod("workers");

        if (args.length > 0 && args[0].equals("--child")) {
            runScenario(mForEach, mAdd, mRem, mWorkers);
            return;
        }

        // parent: W=1 (env absent = dormant vanilla passthrough inside bridge)
        check((Integer) mWorkers.invoke(null) == 1, "parent runs W=1 (dormant)");
        String d1 = runScenario(mForEach, mAdd, mRem, mWorkers);

        String d2 = runChild(2);
        String d4 = runChild(4);

        check(d2.equals(d1), "W=2 digest bit-identical to W=1 (per-entity lockstep)");
        check(d4.equals(d1), "W=4 digest bit-identical to W=1 (per-entity lockstep)");
        System.out.println("REGION-LOCKSTEP PG1 PASS (W=1 == W=2 == W=4, "
                + TICKS + " ticks, 20 deferred adds next-tick-start, "
                + "7 mid-tick removals, no deadlock)");
    }

    private static String runChild(int w) throws Exception {
        java.util.List<String> cmd = new ArrayList<>(java.util.List.of(
                System.getProperty("java.home") + java.io.File.separator + "bin"
                        + java.io.File.separator + "java",
                "-cp", System.getProperty("java.class.path"),
                "net.minecraft.world.entity.RegionLockstepHarness", "--child"));
        ProcessBuilder pb = new ProcessBuilder(cmd);
        pb.environment().put("CRUSSTY_REGION_THREADS", String.valueOf(w));
        pb.redirectErrorStream(true);
        Process p = pb.start();
        String digest = null;
        List<String> out = new ArrayList<>();
        try (java.io.BufferedReader br = new java.io.BufferedReader(
                new java.io.InputStreamReader(p.getInputStream()))) {
            String line;
            while ((line = br.readLine()) != null) {
                out.add(line);
                if (line.contains("LOCKSTEP DIGEST")) digest = line;
            }
        }
        if (!p.waitFor(240, java.util.concurrent.TimeUnit.SECONDS)) {
            p.destroyForcibly();
            throw new AssertionError("child W=" + w + " timed out (deadlock?)");
        }
        check(p.exitValue() == 0, "child W=" + w + " exited 0 ("
                + out.size() + " lines)");
        if (digest == null) {
            throw new AssertionError("child W=" + w
                    + " produced no digest; tail: " + out.subList(
                            Math.max(0, out.size() - 5), out.size()));
        }
        java.util.regex.Matcher m = java.util.regex.Pattern
                .compile("LOCKSTEP DIGEST ([0-9a-f]{64})").matcher(digest);
        check(m.find(), "child W=" + w + " digest parsed (kernel log prefix ok)");
        return m.group(1);
    }
}
