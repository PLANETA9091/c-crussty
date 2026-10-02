package harness;

import java.util.Optional;
import net.minecraft.core.BlockPos;
import net.minecraft.util.Mth;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/** Debug: replicate flat formulas step by step with prints. */
public final class TraverseTrace {

    public static void main(String[] args) {
        Vec3 from = new Vec3(1.37, 64.62, -3.11);
        Vec3 to = new Vec3(1.37, 64.62, 4.640000000000001);
        AABB box = new AABB(1.07, 64.62, -3.41, 1.67, 66.42, -2.81);
        Vec3 delta = to.subtract(from);
        double sizeX = box.getXsize(), sizeY = box.getYsize(), sizeZ = box.getZsize();
        int signX = delta.x >= 0 ? 1 : -1, signY = delta.y >= 0 ? 1 : -1, signZ = delta.z >= 0 ? 1 : -1;
        // furthest
        double ax = Math.abs(delta.x), ay = Math.abs(delta.y), az = Math.abs(delta.z);
        int fX, fY, fZ;
        if (ax <= ay && ax <= az) { fX = -signX; fY = -signZ; fZ = -signY; }
        else if (ay <= az)        { fX = signZ;  fY = -signY; fZ = -signX; }
        else                      { fX = -signY; fY = signX;  fZ = -signZ; }
        System.out.println("delta=" + delta.x + "," + delta.y + "," + delta.z);
        System.out.println("furthest=" + fX + "," + fY + "," + fZ);
        double cX = Mth.lerp(0.5, box.minX, box.maxX);
        double cY = Mth.lerp(0.5, box.minY, box.maxY);
        double cZ = Mth.lerp(0.5, box.minZ, box.maxZ);
        System.out.println("center=" + cX + "," + cY + "," + cZ);
        Vec3 outer = new Vec3(cX + sizeX * 0.5 * (double) fX, cY + sizeY * 0.5 * (double) fY, cZ + sizeZ * 0.5 * (double) fZ);
        Vec3 travel = outer.subtract(delta);
        System.out.println("outer=" + outer.x + "," + outer.y + "," + outer.z);
        System.out.println("travel=" + travel.x + "," + travel.y + "," + travel.z);
        int bx = Mth.floor(travel.x), by = Mth.floor(travel.y), bz = Mth.floor(travel.z);
        int sX = Mth.sign(delta.x), sY = Mth.sign(delta.y), sZ = Mth.sign(delta.z);
        double stX = sX == 0 ? Double.MAX_VALUE : (double) sX / delta.x;
        double stY = sY == 0 ? Double.MAX_VALUE : (double) sY / delta.y;
        double stZ = sZ == 0 ? Double.MAX_VALUE : (double) sZ / delta.z;
        double tmX = stX * (sX > 0 ? 1.0 - Mth.frac(travel.x) : Mth.frac(travel.x));
        double tmY = stY * (sY > 0 ? 1.0 - Mth.frac(travel.y) : Mth.frac(travel.y));
        double tmZ = stZ * (sZ > 0 ? 1.0 - Mth.frac(travel.z) : Mth.frac(travel.z));
        System.out.println("cell0=" + bx + "," + by + "," + bz);
        System.out.println("tMax=" + tmX + "," + tmY + "," + tmZ);
        int steps = 0;
        while (tmX <= 1.0 || tmY <= 1.0 || tmZ <= 1.0) {
            if (tmX < tmY) {
                if (tmX < tmZ) { bx += sX; tmX += stX; } else { bz += sZ; tmZ += stZ; }
            } else {
                if (tmY < tmZ) { by += sY; tmY += stY; } else { bz += sZ; tmZ += stZ; }
            }
            Optional<Vec3> hit = AABB.clip((double) bx, (double) by, (double) bz,
                    (double) (bx + 1), (double) (by + 1), (double) (bz + 1), travel, outer);
            System.out.println("iter cell=" + bx + "," + by + "," + bz
                    + " hit=" + (hit.isPresent() ? hit.get().toString() : "EMPTY"));
            if (hit.isEmpty()) { continue; }
            steps++;
            Vec3 h = hit.get();
            double clX = Mth.clamp(h.x, (double) ((float) bx + 1.0E-5f), (double) bx + 1.0 - 9.999999747378752E-6);
            double clY = Mth.clamp(h.y, (double) ((float) by + 1.0E-5f), (double) by + 1.0 - 9.999999747378752E-6);
            double clZ = Mth.clamp(h.z, (double) ((float) bz + 1.0E-5f), (double) bz + 1.0 - 9.999999747378752E-6);
            int oX = Mth.floor(clX - sizeX * (double) fX);
            int oY = Mth.floor(clY - sizeY * (double) fY);
            int oZ = Mth.floor(clZ - sizeZ * (double) fZ);
            System.out.println("  steps=" + steps + " clamp=" + clX + "," + clY + "," + clZ
                    + " off=" + oX + "," + oY + "," + oZ);
            if (steps > 10) break;
        }
        System.out.println("total steps=" + steps);
    }
}
