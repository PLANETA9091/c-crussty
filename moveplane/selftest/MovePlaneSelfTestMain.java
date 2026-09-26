import net.minecraft.world.entity.ai.control.MovePlaneOps;

/**
 * TASK-463-69a selftest driver: runs MovePlaneOps.selfTest() against the
 * REAL kernel-jar Mth (the in-JVM oracle) and exits non-zero on divergence.
 */
public class MovePlaneSelfTestMain {
    public static void main(String[] args) {
        boolean ok = MovePlaneOps.selfTest();
        System.out.println(ok ? "move_plane selfTest PASS" : "move_plane selfTest FAIL");
        if (!ok) {
            System.exit(1);
        }
    }
}
