package net.minecraft.world.level.block.entity;

/** FAKE probe-target class for the TASK-90 agent self-test ONLY (never shipped, never on the server classpath).
 *  Squats the internal name so DirtyCensusAgent.weave() matches it exactly like the real class. */
public class BlockEntity {
    long v;
    public void setChanged() { v++; }
}
