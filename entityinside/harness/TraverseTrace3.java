package harness;

import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;
import net.minecraft.core.BlockPos;
import net.minecraft.util.Mth;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/** Debug2: compare vanilla DDA vs two replication variants cell-by-cell. */
public final class TraverseTrace3 {

    private static final class Rec implements BlockGetter.BlockStepVisitor {
        final List<String> rows = new ArrayList<>();
        @Override
        public boolean visit(BlockPos p, int s) {
            rows.add("(" + p.getX() + "," + p.getY() + "," + p.getZ() + ")#" + s);
            return true;
        }
    }

    /** replication: order = clip(from,to) arg order: 0=(travel,outer), 1=(outer,travel) */
    private static List<String> replicate(Vec3 delta, AABB box, int order) {
        List<String> rows = new ArrayList<>();
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
        int sX = Mth.sign(delta.x), sY = Mth.sign(delta.y), sZ = Mth.sign(delta.z);
        double stX = sX == 0 ? Double.MAX_VALUE : sX / delta.x;
        double stY = sY == 0 ? Double.MAX_VALUE : sY / delta.y;
        double stZ = sZ == 0 ? Double.MAX_VALUE : sZ / delta.z;
        double tmX = stX * (sX > 0 ? 1.0 - Mth.frac(travel.x) : Mth.frac(travel.x));
        double tmY = stY * (sY > 0 ? 1.0 - Mth.frac(travel.y) : Mth.frac(travel.y));
        double tmZ = stZ * (sZ > 0 ? 1.0 - Mth.frac(travel.z) : Mth.frac(travel.z));
        int steps = 0;
        java.util.Set<Long> visited = new java.util.HashSet<>();
        while (tmX <= 1.0 || tmY <= 1.0 || tmZ <= 1.0) {
            if (tmX < tmY) {
                if (tmX < tmZ) { bx += sX; tmX += stX; } else { bz += sZ; tmZ += stZ; }
            } else {
                if (tmY < tmZ) { by += sY; tmY += stY; } else { bz += sZ; tmZ += stZ; }
            }
            java.util.Optional<Vec3> hit = order == 0
                    ? AABB.clip(bx, by, bz, bx + 1.0, by + 1.0, bz + 1.0, travel, outer)
                    : AABB.clip(bx, by, bz, bx + 1.0, by + 1.0, bz + 1.0, outer, travel);
            if (hit.isEmpty()) { continue; }
            steps++;
            Vec3 h = hit.get();
            double clX = Mth.clamp(h.x, (double) ((float) bx + 1.0E-5f), bx + 1.0 - 9.999999747378752E-6);
            double clY = Mth.clamp(h.y, (double) ((float) by + 1.0E-5f), by + 1.0 - 9.999999747378752E-6);
            double clZ = Mth.clamp(h.z, (double) ((float) bz + 1.0E-5f), bz + 1.0 - 9.999999747378752E-6);
            int oX = Mth.floor(clX - sizeX * fX);
            int oY = Mth.floor(clY - sizeY * fY);
            int oZ = Mth.floor(clZ - sizeZ * fZ);
            rows.add("cell(" + bx + "," + by + "," + bz + ") hit.z=" + h.z
                    + " off(" + oX + "," + oY + "," + oZ + ") step=" + steps);
            if (rows.size() > 12) break;
        }
        return rows;
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
        for (int c = 0; c < cases.length; c++) {
            Vec3 from = cases[c][0], to = cases[c][1];
            AABB box = boxes[c];
            Vec3 delta = to.subtract(from);
            System.out.println("=== case " + c + " delta=" + delta.x + "," + delta.y + "," + delta.z);
            Rec rec = new Rec();
            Object r = m.invoke(null, new it.unimi.dsi.fastutil.longs.LongOpenHashSet(), delta, box, rec);
            System.out.println("vanilla (returned " + r + "): " + rec.rows);
            System.out.println("rep(order0=travel,outer): " + replicate(delta, box, 0));
            System.out.println("rep(order1=outer,travel): " + replicate(delta, box, 1));
        }
    }
}
