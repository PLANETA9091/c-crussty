package net.minecraft.server.level;

/**
 * ITEMS-MONO harness shadow stub (TASK-396-F): a minimal ServerLevel
 * stand-in defined by the shadow loader INSTEAD of the kernel class. The
 * compiled bridge's S7-170 static block only reflectively needs a class
 * named net.minecraft.server.level.ServerLevel carrying the
 * `navigatingMobs` field (Unsafe.objectFieldOffset) — shadowing it here
 * keeps the harness off the kernel server classpath (no boot, no libs).
 */
public class ServerLevel {
    public final Object navigatingMobs = new Object();
}
