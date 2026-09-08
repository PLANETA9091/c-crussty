package net.minecraft.world.phys.shapes;
/** FAKE probe-target: Shapes.collide(Axis,AABB,Iterable,double)D. */
public class Shapes {
    long v;
    public static double collide(net.minecraft.core.Direction$Axis a, net.minecraft.world.phys.AABB b,
                                 java.lang.Iterable<?> c, double d) { SHARED.v++; return d; }
    static final Shapes SHARED = new Shapes();
}
