package harness;

import java.util.Optional;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/** Trace6: probe clip returns for the tail cells. */
public final class TraverseTrace6 {
    public static void main(String[] args) {
        Vec3 outer = new Vec3(1.0700000000000003, 64.62, -3.41);
        Vec3 travel = new Vec3(1.0700000000000003, 64.62, -11.16);
        int[][] cells = {{1,64,-11},{1,64,-10},{1,64,-4},{1,64,-3},{1,64,-2}};
        for (int[] c : cells) {
            Optional<Vec3> a = AABB.clip(c[0], c[1], c[2], c[0]+1.0, c[1]+1.0, c[2]+1.0, travel, outer);
            Optional<Vec3> b = AABB.clip(c[0], c[1], c[2], c[0]+1.0, c[1]+1.0, c[2]+1.0, outer, travel);
            System.out.println("cell " + c[2] + ": (travel,outer)=" + (a.isPresent() ? a.get().z : "EMPTY")
                    + "  (outer,travel)=" + (b.isPresent() ? b.get().z : "EMPTY"));
        }
    }
}
