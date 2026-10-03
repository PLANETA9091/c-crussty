package harness;

import java.lang.reflect.Method;
import net.minecraft.core.Vec3i;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.phys.Vec3;

/** Trace8: reflect vanilla getFurthestCorner. */
public final class TraverseTrace8 {
    public static void main(String[] args) throws Exception {
        Method m = BlockGetter.class.getDeclaredMethod("getFurthestCorner", Vec3.class);
        m.setAccessible(true);
        Vec3[] deltas = {
            new Vec3(0, 0, 7.75), new Vec3(0, 0, -7.75), new Vec3(7.75, 0, 0),
            new Vec3(-7.75, 0, 0), new Vec3(0, 7.75, 0), new Vec3(0, -7.75, 0),
            new Vec3(1, 2, 3), new Vec3(1, 2, -3), new Vec3(-1, -2, 3),
            new Vec3(0, 0, 0.001), new Vec3(0.001, 0, 0),
        };
        for (Vec3 d : deltas) {
            Vec3i r = (Vec3i) m.invoke(null, d);
            System.out.println("delta=(" + d.x + "," + d.y + "," + d.z + ") -> furthest=("
                    + r.getX() + "," + r.getY() + "," + r.getZ() + ")");
        }
    }
}
