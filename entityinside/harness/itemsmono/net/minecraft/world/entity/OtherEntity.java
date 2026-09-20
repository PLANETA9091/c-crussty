package net.minecraft.world.entity;

/**
 * ITEMS-MONO harness shadow stub (TASK-396-F): a NON-item entity stand-in.
 * Its tick()V body marks the VANILLA lane (the branch the compiled
 * entityTick type-test split must take for every non-ItemEntity receiver).
 */
public class OtherEntity extends Entity {
    @Override public void tick() {
        lastBody = "OtherEntity.tick";
        ticks++;
    }
}
