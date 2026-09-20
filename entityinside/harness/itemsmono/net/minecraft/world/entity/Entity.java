package net.minecraft.world.entity;

/**
 * ITEMS-MONO harness shadow stub (TASK-396-F): the base entity of the
 * child-first shadow loader. Defined INSTEAD of the kernel Entity inside
 * the shadow ClassLoader so the REAL compiled RegionTickOps.entityTick
 * bytecode can be exercised behaviorally without a live server. The
 * tick()V shape matches the kernel descriptor the compiled bridge
 * verifies against; lastBody records which body ran.
 */
public class Entity {
    public String lastBody = "none";
    public int ticks;

    public void tick() {
        lastBody = "Entity.tick";
        ticks++;
    }
}
