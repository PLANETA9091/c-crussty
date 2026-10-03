import java.lang.reflect.Field;
import java.util.Iterator;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.ZeroCursorOps;
import net.minecraft.core.ZeroCursorIter;

public class CursorDebug1 {
    public static void main(String[] a) throws Exception {
        Direction d1 = Direction.UP, d2 = Direction.DOWN, d3 = Direction.DOWN;
        int x0 = -4, y0 = -9, z0 = 0, e3 = 0, e2 = 7, e1 = 1;
        Iterator<?> mi = ZeroCursorOps.lambda8(d1, d2, d3, x0, y0, z0, e3, e2, e1);
        ZeroCursorIter it = (ZeroCursorIter) mi;
        Class<?> c = ZeroCursorIter.class;
        Field fi = c.getDeclaredField("firstIndex"); fi.setAccessible(true);
        Field fs = c.getDeclaredField("secondIndex"); fs.setAccessible(true);
        Field ft = c.getDeclaredField("thirdIndex"); ft.setAccessible(true);
        Field fe = c.getDeclaredField("end"); fe.setAccessible(true);
        Field fst = c.getDeclaredField("started"); fst.setAccessible(true);
        int n = 0;
        while (it.hasNext()) {
            BlockPos p = it.next();
            System.out.println((n++) + " (" + p.getX() + "," + p.getY() + "," + p.getZ() + ")"
                + " idx(f,s,t)=(" + fi.get(it) + "," + fs.get(it) + "," + ft.get(it) + ")"
                + " end=" + fe.get(it) + " started=" + fst.get(it));
        }
        System.out.println("final: end=" + fe.get(it) + " started=" + fst.get(it)
            + " idx=(" + fi.get(it) + "," + fs.get(it) + "," + ft.get(it) + ")");
    }
}
