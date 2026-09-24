package io.netty.buffer;

/**
 * COMPILE-TIME-ONLY netty ByteBuf shape stub (TASK-444-B).
 *
 * The patched-kernel.jar does not expose netty classes to javac, but javac
 * needs the supertype chain of RegistryFriendlyByteBuf resolvable to compile
 * inherited-member calls (writeInt etc.). This stub provides ONLY the
 * signatures; it NEVER ships and is NEVER on a runtime classpath:
 *  - the emitted bridge bytecode references FriendlyByteBuf/minecraft
 *    declaring classes only (writeInt/write are declared there);
 *  - every netty-typed operation in the bridge goes through the reflective
 *    surface resolved at runtime inside the kernel JVM (real netty).
 * Descriptor drift into the artifact is therefore impossible by construction.
 */
public abstract class ByteBuf {
    public ByteBuf writeBytes(byte[] src) { return this; }
    public int readableBytes() { return 0; }
    public ByteBuf readBytes(byte[] dst) { return this; }
    public boolean release() { return true; }
}
