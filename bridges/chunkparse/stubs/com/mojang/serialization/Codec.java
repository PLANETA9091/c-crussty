package com.mojang.serialization;

/**
 * COMPILE-TIME-ONLY descriptor stub (TASK-419-C).
 *
 * The real {@code com.mojang.serialization.Codec} lives in the mojang
 * serialization libs shipped with the kernel runtime and is NOT on the
 * offline javac classpath (patched-kernel.jar carries net.minecraft classes
 * only). The bridge needs the EXACT parameter descriptor
 * {@code Lcom/mojang/serialization/Codec;} for the static body-redirect
 * stack-shape contract; javac just records the NAME in the classfile. At
 * runtime the reference passes through untouched (no member of Codec is
 * ever invoked by the bridge — the miss path reflects into the pristine
 * twin lambda and the cache keys on identity), so the real class links
 * fine. This stub is NEVER defined into any loader.
 */
public class Codec<T> {}
