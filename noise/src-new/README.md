# noise/src-new — B1 lifecycle workspace (NOT shipped)

Scratch/test area owned by the B1 task (ImprovedNoiseNativeOps lifecycle:
finalize() -> Cleaner + explicit release). Nothing in here is embedded into
the plugin — the runtime embed contract is exactly the two class files under
`noise/build/` (see scripts/build_noise.sh and src/improved_noise.rs).

## test/ — smoke test for the Cleaner lifecycle

Pure-Java doubles of the bridge natives (`test/fake/...`) let the REAL
`ImprovedNoiseNativeOps` run without the closed `libpaper_native_jni.so`:

    # from repo root, any JDK >= 11 (/home/z/jdk21 on this box)
    javac -d /tmp/noise-smoke \
      noise/src-new/test/fake/net/minecraft/world/level/levelgen/synth/ImprovedNoise.java \
      noise/src-new/test/fake/net/minecraft/world/level/levelgen/synth/PaperNativeImprovedNoise.java \
      noise/net/minecraft/world/level/levelgen/synth/ImprovedNoiseNativeOps.java \
      noise/src-new/test/SmokeLifecycle.java
    java -cp /tmp/noise-smoke SmokeLifecycle

Scenarios: handle caching (1 handle per instance), explicit
`releaseHandle()` (free-once, no-op repeats, transparent rebuild), GC-driven
drain (drop + one map poke), and the double-free guard
(FREED_CALLS == FREED_UNIQUE at every checkpoint). Exit 0 = pass.

## lifecycle notes (B1)

- Bridge now targets `--release 11` (major 55): `java.lang.ref.Cleaner` is a
  Java 9+ API and cannot even be referenced under `--release 8`. Safe because
  the only consumer is the kernel JVM (Java 21, major 65) and
  src/improved_noise.rs compares the embedded bridge major against
  `java.class.version` at activation — older kernels keep the hook dormant.
- EMBED CONTRACT: exactly 2 class files may ship
  (ImprovedNoiseNativeOps + $Handle). The cleaning action is a capturing
  lambda (captures only `long handle` + its own `AtomicBoolean freed`, never
  `this`) so javac emits a static `lambda$` method inside $Handle instead of
  an extra `$Releaser`/`$1` class file that the runtime would never define.
- The A/B bench spec lives at bench/p500/lifecycle/LifecycleBench.java
  (runnable skeleton; DRYRUN-safe without natives).
