package harness;

import java.util.List;
import net.minecraft.core.BlockPos;
import net.minecraft.world.level.BlockGetter;
import net.minecraft.world.level.TraverseOps;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/** Debug: dump both visit sequences for one scenario. */
public final class TraverseDebug {

    private static final class Dump implements BlockGetter.BlockStepVisitor {
        final String tag;
        Dump(String tag) { this.tag = tag; }
        @Override
        public boolean visit(BlockPos p, int s) {
            System.out.println("  [" + tag + "] (" + p.getX() + "," + p.getY()
                    + "," + p.getZ() + ") step=" + s);
            return true;
        }
    }

    public static void main(String[] args) {
        Vec3 from = new Vec3(1.37, 64.62, -3.11);
        Vec3 to = new Vec3(1.37, 64.62, 4.640000000000001);
        AABB box = new AABB(1.07, 64.62, -3.41, 1.67, 66.42, -2.81);
        System.out.println("vanilla:");
        BlockGetter.forEachBlockIntersectedBetween(from, to, box, new Dump("van"));
        System.out.println("flat:");
        TraverseOps.forEachFlat(from, to, box, new Dump("flat"));
    }
}
