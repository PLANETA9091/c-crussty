# BOOT W1 NATIVE ATTRIBUTION — S7-35 (TASK-93)

Data: S033 1ms-census artifacts re-analyzed (`/tmp/boot_deep_s733/deep_live.jfr`,
live dump at boot of 14.435s; 5362 ExecutionSample + 5239 NativeMethodSample).
New tool: `scripts/boot/analyze_boot_windows.py` (multi-window, family taxonomy,
R1 gate). Fixed tosec bug (hours*3600 must be added, not multiplied into running
sum — self-consistent in original windows so prior slice results stand).

## 1. The ExecutionSample blind spot — native layer measured for the first time

All prior boot censuses (S030 10ms, S032, S033 1ms) used jdk.ExecutionSample
only. It sees RUNNABLE-thread Java frames exclusively. jdk.NativeMethodSample
was present in the recording all along (5,239 events) but never parsed.

Registry window W1 = log [16:13:23 → 16:13:28) ("Initialized 0 plugins" →
"Environment:"), 753 Java samples (92% ServerMain) + 241 ServerMain native
samples + 15 GC pauses (466ms) + ~0 parks.

ServerMain native in W1 (top):
- java.util.zip.Inflater.inflateBytesBytes 152 (58% of native)
- ClassLoader.defineClass1 34, Class.forName0 15
- RandomAccessFile.readBytes0/seek0 21
- joml.MemUtil.createInstance 5

**Reading: the inflate+define stream is the ~1.3k library-jar classes
(configurate/jline/etc.) that CDS v2 does NOT cover** (S032 classload census:
15,184/27,826 mapped; remainder = library jars + lambda bootstrap agents).
They are decompressed and defined at boot cost, per class, inside W1.

## 2. W1 composition (R1 gate verdict)

- no-I/O in Java stacks: 718/753 = **95%** → window IS construction-dominated
  (I/O only 4%) — R1 census-first precondition HOLDS.
- BUT composition is NOT a single registry-island amenable to
  registry-persistence; it is 4+ distinct construction islands:
  - VoxelShape join machinery (Moonrise CollisionUtil.merge/joinUnoptimized/
    isJoinNonEmpty/forAllBoxes/BitSet.expandTo): ~136 Java samples ≈ 18% —
    collision-shape joins during block/shape registry construction.
  - DFU schema joins: ~85 ≈ 11% (R3 channel, confirmed at 1ms).
  - invoke-bootstrap (LambdaForm/DMH): ~83 ≈ 11% (JDK21-inherent, known).
  - classload-native (Inflater+defineClass+forName): ~241 native ≈ 25% of
    native+Java mixed basis — CDS v3 channel, quantified natively.
  - jvm_misc (hash/equals/copyOf/BitSet/String.split): ~20% supporting cost.
- gc: 466ms pauses inside W1 (2-core parallel phases).

Verdict for R1 (registry-persistence design line): the 5s window is 95%
construction (no I/O), but NO single island reaches the ≥80% "one mechanism"
gate. Registry-persistence would address the registry/DFU subset only.
**CDS v3 (library jars into dump classpath) is the cheapest verified channel**
against the native classload island; collision-join is a second engine-level
candidate (boot ~0.13s sampled share + runtime collision surface, TASK-84
collision probe will price it).

## 3. Sampler-starvation caveat (honesty)

On this 2-core box JFR samplers are CPU-starved: saturated Worker-Main-1
shows 600-700 samples/s instead of ~1000 at 1ms period. Absolute CPU totals
are underestimates (unknown factor ~1.4-1.5x); relative composition per
window remains proportional. Do not convert sampled seconds into wall claims
without this factor.

## 4. Serve-at-load design doc (NEXT-1 of S034)

Problem: boot-window native noise kernels cannot serve — arming chain is
deliberately deferred post-Done (activate() waits find_class scan cadence →
wait_for_boot() → define bridges → retransform → serve), the S7-25 guard
against defineClass1 SIGSEGV during the boot classloading storm. S034 ABBA:
armed-boot A/B = NULL, mechanism = deferral (0 patches in-window).

Variants:
- V1 status quo (post-Done arm): zero boot effect by design. Current default.
- V2 OnLoad-time bridge define: define bridge classes into an isolated
  ClassLoader at Agent_OnLoad, BEFORE app classloading storm, then enable
  ClassFileLoadHook after bridges exist; hook-time retransform of noise
  classes at FIRST LOAD (serve-at-load). Risk: S7-25 SIGSEGV was in
  defineClass1 during hook storm; OnLoad-time define precedes storm —
  hypothesis to test; engine src/ change (crussty runtime), P500 duty,
  hs_err watch mandatory.
- V3 retransform-to-native without bridge classes: impossible — making an
  existing Java method native requires ACC_NATIVE flag change, which
  retransform forbids (structural class change). Closed.
- Honest ceiling: in-boot noise share (S033: 11.6% of window CPU) sits mostly
  in W2/Worker spawn-gen, not W1; boot gain ceiling ≈ 0.2s. Real value of V2
  = post-Done lazy-gen window (3.2s) + earlier runtime kernel coverage.
  **Priority verdict: V2 is a design line only — CDS v3 outranks it for the
  boot directive; revisit when TASK-84 prices the collision/runtime surface.**

## 5. Next-session queue (re-ranked)

1. CDS v3 spike: extend dump classpath with library jars (cds_rebuild.sh),
   re-dump, A/B ×3 vs 13.351s default. Targets Inflater+defineClass native
   island (~0.3-0.6s est, sampled-basis). Cheap, no engine risk.
2. Collision-join census (TASK-84 collision probe prices runtime share; boot
   share 18% of W1 known).
3. R3 DFU-schema cache design (11% of W1, confirmed twice).
4. V2 serve-at-load (engine src/, P500 duty) — only after 1-3.
