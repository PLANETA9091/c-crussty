package net.minecraft.world.entity.item;

import net.minecraft.world.entity.Entity;

/**
 * ITEMS-MONO harness shadow stub (TASK-396-F): the ItemEntity stand-in.
 * Extends the shadow Entity; its tick()V body marks the MONOMORPHIC item
 * lane (the branch the compiled entityTick type-test split must take for
 * every ItemEntity receiver).
 */
public class ItemEntity extends Entity {
    @Override public void tick() {
        lastBody = "ItemEntity.tick";
        ticks++;
    }
}
