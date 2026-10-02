package net.minecraft.world.entity;
/** FAKE probe-target (TASK-104 self-test only): squats Entity internal name; methods
 *  mirror the woven descriptors collide(Vec3)Vec3 / move(MoverType,Vec3)V / setPos(DDD)V. */
public class Entity {
    long v;
    public net.minecraft.world.phys.Vec3 collide(net.minecraft.world.phys.Vec3 d) { v++; return d; }
    public void move(MoverType t, net.minecraft.world.phys.Vec3 d) { v++; }
    public void setPos(double x, double y, double z) { v++; }
}
