import java.lang.reflect.Method;
import java.util.Iterator;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.ZeroCursorOps;

public class CursorDebug0 {
    public static void main(String[] a) throws Exception {
        Method v = BlockPos.class.getDeclaredMethod("lambda$betweenCornersInDirection$8",
            Direction.class, Direction.class, Direction.class,
            int.class, int.class, int.class, int.class, int.class, int.class);
        v.setAccessible(true);
        Direction d1 = Direction.UP, d2 = Direction.DOWN, d3 = Direction.DOWN;
        int x0 = -4, y0 = -9, z0 = 0, e3 = 0, e2 = 7, e1 = 1;
        System.out.println("vanilla:");
        Iterator<?> vi = (Iterator<?>) v.invoke(null, d1, d2, d3, x0, y0, z0, e3, e2, e1);
        int n = 0;
        while (vi.hasNext()) {
            BlockPos p = (BlockPos) vi.next();
            System.out.println("  " + (n++) + " (" + p.getX() + "," + p.getY() + "," + p.getZ() + ")");
        }
        System.out.println("mine:");
        Iterator<?> mi = ZeroCursorOps.lambda8(d1, d2, d3, x0, y0, z0, e3, e2, e1);
        n = 0;
        while (mi.hasNext()) {
            BlockPos p = (BlockPos) mi.next();
            System.out.println("  " + (n++) + " (" + p.getX() + "," + p.getY() + "," + p.getZ() + ")");
        }
    }
}
