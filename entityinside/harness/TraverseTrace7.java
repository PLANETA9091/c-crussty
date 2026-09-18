package harness;

import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;
import net.minecraft.core.BlockPos;
import net.minecraft.util.Mth;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/** Trace7: wide brute-force over structural variants. */
public final class TraverseTrace7 {

    private static final class Rec implements BlockGetter.BlockStepVisitor {
        final List<String> rows = new ArrayList<>();
        public boolean visit(BlockPos p, int s) {
            rows.add("(" + p.getX() + "," + p.getY() + "," + p.getZ() + ")#" + s);
            return true;
        }
    }

    // sel: 0=min, 1=max; dirMult: +1/-1; offF: 0=floor(clamp - size*f), 1=floor(clamp + size*f), 2=floor(clamp), 3=floor(clamp - size)
    static List<String> sim(Vec3 delta, AABB box, int clipOrder, boolean stepFirst, int sel, int dirMult, int offF) {
        List<String> rows = new ArrayList<>();
        java.util.Set<Long> visited = new java.util.HashSet<>();
        double sizeX = box.getXsize(), sizeY = box.getYsize(), sizeZ = box.getZsize();
        int sx = delta.x >= 0 ? 1 : -1, sy = delta.y >= 0 ? 1 : -1, sz = delta.z >= 0 ? 1 : -1;
        double ax = Math.abs(delta.x), ay = Math.abs(delta.y), az = Math.abs(delta.z);
        int fX, fY, fZ;
        if (ax <= ay && ax <= az) { fX = -sx; fY = -sz; fZ = -sy; }
        else if (ay <= az)        { fX = sz;  fY = -sy; fZ = -sx; }
        else                      { fX = -sy; fY = sx;  fZ = -sz; }
        double cX = Mth.lerp(0.5, box.minX, box.maxX);
        double cY = Mth.lerp(0.5, box.minY, box.maxY);
        double cZ = Mth.lerp(0.5, box.minZ, box.maxZ);
        Vec3 outer = new Vec3(cX + sizeX * 0.5 * fX, cY + sizeY * 0.5 * fY, cZ + sizeZ * 0.5 * fZ);
        Vec3 travel = outer.subtract(delta);
        int bx = Mth.floor(travel.x), by = Mth.floor(travel.y), bz = Mth.floor(travel.z);
        int sX = Mth.sign(delta.x) * dirMult, sY = Mth.sign(delta.y) * dirMult, sZ = Mth.sign(delta.z) * dirMult;
        double stX = sX == 0 ? Double.MAX_VALUE : (double) sX / delta.x;
        double stY = sY == 0 ? Double.MAX_VALUE : (double) sY / delta.y;
        double stZ = sZ == 0 ? Double.MAX_VALUE : (double) sZ / delta.z;
        double tmX = stX * (sX > 0 ? 1.0 - Mth.frac(travel.x) : Mth.frac(travel.x));
        double tmY = stY * (sY > 0 ? 1.0 - Mth.frac(travel.y) : Mth.frac(travel.y));
        double tmZ = stZ * (sZ > 0 ? 1.0 - Mth.frac(travel.z) : Mth.frac(travel.z));
        int steps = 0, guard = 0;
        while (tmX <= 1.0 || tmY <= 1.0 || tmZ <= 1.0) {
            if (stepFirst) {
                step(bx0 -> {}, 0);
                if (sel == 0) {
                    if (tmX < tmY) { if (tmX < tmZ) { bx += sX; tmX += stX; } else { bz += sZ; tmZ += stZ; } }
                    else { if (tmY < tmZ) { by += sY; tmY += stY; } else { bz += sZ; tmZ += stZ; } }
                } else {
                    if (tmX > tmY) { if (tmX > tmZ) { bx += sX; tmX += stX; } else { bz += sZ; tmZ += stZ; } }
                    else { if (tmY > tmZ) { by += sY; tmY += stY; } else { bz += sZ; tmZ += stZ; } }
                }
            }
            java.util.Optional<Vec3> hit = clipOrder == 0
                    ? AABB.clip(bx, by, bz, bx + 1.0, by + 1.0, bz + 1.0, travel, outer)
                    : AABB.clip(bx, by, bz, bx + 1.0, by + 1.0, bz + 1.0, outer, travel);
            if (!hit.isEmpty()) {
                steps++;
                Vec3 h = hit.get();
                double clX = Mth.clamp(h.x, (double) ((float) bx + 1.0E-5f), bx + 1.0 - 9.999999747378752E-6);
                double clY = Mth.clamp(h.y, (double) ((float) by + 1.0E-5f), by + 1.0 - 9.999999747378752E-6);
                double clZ = Mth.clamp(h.z, (double) ((float) bz + 1.0E-5f), bz + 1.0 - 9.999999747378752E-6);
                int oX, oY, oZ;
                switch (offF) {
                    case 0: oX = Mth.floor(clX - sizeX * fX); oY = Mth.floor(clY - sizeY * fY); oZ = Mth.floor(clZ - sizeZ * fZ); break;
                    case 1: oX = Mth.floor(clX + sizeX * fX); oY = Mth.floor(clY + sizeY * fY); oZ = Mth.floor(clZ + sizeZ * fZ); break;
                    case 2: oX = Mth.floor(clX); oY = Mth.floor(clY); oZ = Mth.floor(clZ); break;
                    default: oX = Mth.floor(clX - sizeX); oY = Mth.floor(clY - sizeY); oZ = Mth.floor(clZ - sizeZ); break;
                }
                corners(bx, by, bz, oX, oY, oZ, delta, visited, steps, rows);
            }
            if (!stepFirst) {
                if (sel == 0) {
                    if (tmX < tmY) { if (tmX < tmZ) { bx += sX; tmX += stX; } else { bz += sZ; tmZ += stZ; } }
                    else { if (tmY < tmZ) { by += sY; tmY += stY; } else { bz += sZ; tmZ += stZ; } }
                } else {
                    if (tmX > tmY) { if (tmX > tmZ) { bx += sX; tmX += stX; } else { bz += sZ; tmZ += stZ; } }
                    else { if (tmY > tmZ) { by += sY; tmY += stY; } else { bz += sZ; tmZ += stZ; } }
                }
            }
            if (++guard > 300) break;
        }
        return rows;
    }

    static void step(java.util.function.Consumer<Object> c, int x) {}

    static void corners(int x0, int y0, int z0, int x1, int y1, int z1,
                        Vec3 delta, java.util.Set<Long> visited, int step, List<String> rows) {
        int minX = Math.min(x0, x1), minY = Math.min(y0, y1), minZ = Math.min(z0, z1);
        int maxX = Math.max(x0, x1), maxY = Math.max(y0, y1), maxZ = Math.max(z0, z1);
        int extX = maxX - minX, extY = maxY - minY, extZ = maxZ - minZ;
        int stX = delta.x >= 0 ? 1 : -1, stY = delta.y >= 0 ? 1 : -1, stZ = delta.z >= 0 ? 1 : -1;
        int startX = delta.x >= 0 ? minX : maxX;
        int startY = delta.y >= 0 ? minY : maxY;
        int startZ = delta.z >= 0 ? minZ : maxZ;
        boolean zx = Math.abs(delta.x) < Math.abs(delta.z);
        int stepMid, stepInner, startMid, startInner, extMid, extInner;
        if (zx) {
            stepMid = delta.z >= 0 ? 1 : -1; startMid = delta.z >= 0 ? minZ : maxZ; extMid = extZ;
            stepInner = delta.x >= 0 ? 1 : -1; startInner = delta.x >= 0 ? minX : maxX; extInner = extX;
        } else {
            stepMid = delta.x >= 0 ? 1 : -1; startMid = delta.x >= 0 ? minX : maxX; extMid = extX;
            stepInner = delta.z >= 0 ? 1 : -1; startInner = delta.z >= 0 ? minZ : maxZ; extInner = extZ;
        }
        for (int f = 0; f <= extY; f++) {
            int cy = startY + stY * f;
            for (int s = 0; s <= extMid; s++) {
                int cMid = startMid + stepMid * s;
                for (int t = 0; t <= extInner; t++) {
                    int cInner = startInner + stepInner * t;
                    int px, pz;
                    if (zx) { px = cInner; pz = cMid; } else { px = cMid; pz = cInner; }
                    long key = BlockPos.asLong(px, cy, pz);
                    if (visited.add(key)) rows.add("(" + px + "," + cy + "," + pz + ")#" + step);
                }
            }
        }
    }

    public static void main(String[] args) throws Exception {
        Method m = BlockGetter.class.getDeclaredMethod("addCollisionsAlongTravel",
                Class.forName("it.unimi.dsi.fastutil.longs.LongSet"),
                Vec3.class, AABB.class, BlockGetter.BlockStepVisitor.class);
        m.setAccessible(true);
        Vec3[][] cases = {
            { new Vec3(1.37, 64.62, -3.11), new Vec3(1.37, 64.62, 4.640000000000001) },
            { new Vec3(1.37, 64.62, -3.11), new Vec3(1.37, 61.370000000000005, 4.640000000000001) },
            { new Vec3(-8.767898777796914, 25.71881668767324, 20.29093740945693),
              new Vec3(-9.525732887266937, 24.880154467256343, 22.21714906331043) },
        };
        AABB[] boxes = {
            new AABB(1.07, 64.62, -3.41, 1.67, 66.42, -2.81),
            new AABB(1.07, 64.62, -3.41, 1.67, 66.42, -2.81),
            new AABB(-10.616284241735793, 24.880154467256343, 21.126597708841572,
                     -8.435181532798081, 27.81180088181024, 23.307700417779287),
        };
        List<String>[] vanilla = new List[3];
        for (int c = 0; c < 3; c++) {
            Rec rec = new Rec();
            m.invoke(null, new it.unimi.dsi.fastutil.longs.LongOpenHashSet(),
                    cases[c][1].subtract(cases[c][0]), boxes[c], rec);
            vanilla[c] = rec.rows;
        }
        for (int co = 0; co <= 1; co++)
        for (int sf = 0; sf <= 1; sf++)
        for (int sel = 0; sel <= 1; sel++)
        for (int dm = 0; dm <= 1; dm++)
        for (int offF = 0; offF <= 3; offF++) {
            int score = 0;
            for (int c = 0; c < 3; c++) {
                Vec3 delta = cases[c][1].subtract(cases[c][0]);
                List<String> s = sim(delta, boxes[c], co, sf == 1, sel, dm == 0 ? 1 : -1, offF);
                if (s.equals(vanilla[c])) score++;
            }
            if (score > 0) System.out.println("MATCH co=" + co + " sf=" + sf + " sel=" + sel
                    + " dm=" + dm + " offF=" + offF + " score=" + score);
        }
        System.out.println("search done");
    }
}
