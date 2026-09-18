package harness;

import java.lang.reflect.Method;
import java.util.Optional;
import net.minecraft.core.BlockPos;
import net.minecraft.util.Mth;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/** Debug: isolate vanilla addCollisionsAlongTravel via reflection. */
public final class TraverseTrace2 {

    private static final class Dump implements BlockGetter.BlockStepVisitor {
        @Override
        public boolean visit(BlockPos p, int s) {
            System.out.println("  vanDDA (" + p.getX() + "," + p.getY() + ","
                    + p.getZ() + ") step=" + s);
            return true;
        }
    }

    public static void main(String[] args) throws Exception {
        Vec3 from = new Vec3(1.37, 64.62, -3.11);
        Vec3 to = new Vec3(1.37, 64.62, 4.640000000000001);
        AABB box = new AABB(1.07, 64.62, -3.41, 1.67, 66.42, -2.81);
        Vec3 delta = to.subtract(from);

        // vanilla private static addCollisionsAlongTravel(LongSet, Vec3, AABB, BlockStepVisitor)
        Method m = BlockGetter.class.getDeclaredMethod("addCollisionsAlongTravel",
                Class.forName("it.unimi.dsi.fastutil.longs.LongSet"),
                Vec3.class, AABB.class, BlockGetter.BlockStepVisitor.class);
        m.setAccessible(true);
        it.unimi.dsi.fastutil.longs.LongOpenHashSet empty = new it.unimi.dsi.fastutil.longs.LongOpenHashSet();
        Object r = m.invoke(null, empty, delta, box, new Dump());
        System.out.println("vanDDA returned " + r);

        // my clip sanity: cell (1,64,-11)
        Vec3 outer = new Vec3(1.0700000000000003, 64.62, -3.41);
        Vec3 travel = new Vec3(1.0700000000000003, 64.62, -11.16);
        Optional<Vec3> hit = AABB.clip(1.0, 64.0, -11.0, 2.0, 65.0, -10.0, travel, outer);
        System.out.println("clip(1,64,-11..2,65,-10, travel, outer) = "
                + (hit.isPresent() ? hit.get() : "EMPTY"));
        Optional<Vec3> hit2 = AABB.clip(1.0, 64.0, -11.0, 2.0, 65.0, -10.0, outer, travel);
        System.out.println("clip(..., outer, travel) = "
                + (hit2.isPresent() ? hit2.get() : "EMPTY"));
    }
}
