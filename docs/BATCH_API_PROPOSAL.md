# Proposal: Native Batch Dispatch API (`PaperNativeBatchDispatch`)

> **Status 2026-09-08**: v1 is WIRED and policy-gated — see
> [`BATCH_WIRING_PLAN.md`](BATCH_WIRING_PLAN.md) (as-built: modules in
> `lib.rs`, `batch_api::init` call site, `kernel_policy` gate,
> `ERR_KERNEL_REFUSED`, drift-guard tests). Floor numbers errata: this
> document predates `P500_REPORT_v2.md` — the canon JNI transition floor is
> **35–90 ns** (13 floor groups < 200 ns), not ~115 ns; batching math and
> structure below are unchanged.

Design for amortizing the JNI transition floor across many native
kernel invocations in c-crussty. Companion context:
[`OPTIMIZATION_ROADMAP.md`](OPTIMIZATION_ROADMAP.md) (§2 floor insight, §3
Phase 2.2). Target repo state: `src/` (Rust plugin), `native/JNI_EXPORTS.manifest`
(283 exports / 98 bridge classes), `bench/p500/` (measurement rig).

---

## 1. Motivation — 49 groups × 115 ns floor

P500 (`bench/p500/results/P500_REPORT.md`) shows ~40 of the 49 groups pinned
in a 112–220 ns band where old and optimized kernels are within the ±15%
parity band:

| kernel pair (same group) | old ns/op | alt ns/op |
|---|---:|---:|
| `PaperNativeRangeChoice.oldFillArraySummary` / `optimizedFillArraySummary` | 111.9 | 114.6 |
| `PaperNativePluginNameLog.oldTreesetSummary` / `newArrayListSortSummary` | 117.8 | 112.7 |
| `PaperNativeSpigotLoadOrderDependency.oldLoadAfterBuildSummary` / `newLoadAfterBuildSummary` | 114.8 | 114.5 |
| `PaperNativeStaticCacheGet.oldBatchSummary` / `newBatchSummary` | 138.2 | 139.5 |
| `PaperNativePluginClassLoaderGroup.oldLookupSummary` / `skipRequesterSummary` | 146.7 | 153.2 |

Interpretation: the measured time is dominated by the **Java→native
transition** (JNI call barrier: `CallStatic...MethodA`, argument marshalling,
`NativeMethod` entry/exit, safepoint poll, frame setup), not by kernel body
work. For those groups a kernel "body" is often < 30 ns — micro-optimizing
bodies cannot yield more than ~2x in the best case and ~0% in most; **removing
transitions** can yield 5–25x. The counter-example groups (e.g. `MarkerCache`
119 µs, `NoiseInterpolatorSlice` 1.9–6.2 ms) have bodies ≫ floor and do not
benefit — batching is scoped to the floor-bound class only.

Secondary motivation: every individual JNI call pays per-call `HandleScope`
push/pop, local-ref bookkeeping, and an exception check. A batch pays each of
those once.

## 2. Design principle — the dispatcher *is* the transition

c-crussty already resolves every export symbol to a function pointer during
injection (`src/lib.rs::define_and_register` → `loader::NativeLib::symbol(sym)`
→ `JNINativeMethod.fnPtr`). A JNI-exported function is a plain C function
`(JNIEnv*, jclass, args...)`. Therefore a batch dispatcher can:

1. be the **single** registered native for the batch bridge (one transition),
2. decode packed arguments on the Rust side,
3. call the resolved kernel function pointers **directly** — no
   `env->CallStatic...`, no second transition, no `GetMethodID` lookups.

Per-op cost collapses to argument decode + an indirect C call.

## 3. API surface

One plugin-owned bridge class, injected in the same bootstrap pass as the
other 98 (`src/lib.rs`), **not** added to `native/JNI_EXPORTS.manifest` (the
manifest is the closed-surface contract; the dispatcher is c-crussty
infrastructure):

```java
package net.minecraft.world.level.levelgen;   // or a crussty package

public final class PaperNativeBatchDispatch {
    /** ABI/capability version handshake (bitmask: v1 features). */
    public static native int capability();
    /** One batch of the SAME kernel repeated `count` times (80% case). */
    public static native int dispatchRepeat(int kernelId, int count,
            long[] scalarArgs, Object[] refArgs,
            long[] outs, int[] rets);
    /** Heterogeneous batch: one op per kernelIds[i] (v1.1, see §10.2). */
    public static native int dispatch(int[] kernelIds,
            long[] scalarArgs, Object[] refArgs,
            long[] outs, int[] rets);
}
```

* `kernelId` — stable per-boot index into the dispatch table, assigned in
  `jni_table` iteration order (MAIN table then CHUNK table, `src/jni_table.rs`;
  deterministic because `scripts/gen_crussty_table.py render` is a pure
  function of the manifest, and `render --check` + `verify` are CI-enforced,
  `native/MANIFEST.md`). Java side obtains ids once at startup via
  `capability()`-time registration (see §9).
* `scalarArgs` — packed primitive arguments, fixed per-kernel stride (§5).
* `refArgs` — companion array for reference-typed arguments
  (`[J/[I/[D/...` arrays, `String`, `Object[]`); stride fixed per kernel.
  Without it, the 9+ floor-bound `Object[]`-bearing plugin/loading groups
  (`PluginLoadingAllocation`, `PluginNameLog`, `PluginMetaDependency`,
  `PluginStartupRollup`, `RemapperIndexCleanup`, `SpigotLoadOrderDependency`,
  `PluginClassLoaderGroup`, `ObfHelperMaps`, `RemapperHashThreshold`) would be
  unreachable — the exact groups this API exists for.
* `outs` — result longs (the `*Summary` kernels write counts/sums into their
  trailing `[J`; the dispatcher redirects kernel output into `outs` slices, or
  kernels keep writing their own arrays passed via `refArgs` — v1 uses the
  latter, zero-copy; `outs` carries per-op returned longs for `(I)J`-style
  kernels such as `PaperNativeProtoChunkHeightmap`).
* `rets` — one jint per op as returned by the kernel (counts), or the failure
  sentinel `0x80000000` (§7).

## 4. Rust-side dispatch table

New module `src/batch_dispatch.rs`:

```rust
pub struct KernelDesc {
    class: &'static str,
    method: &'static str,
    sig: &'static str,
    scalars: u16,     // packed long slots per op
    refs: u16,        // reference slots per op
    ret: RetKind,     // I / J / D / V
    fptr: *mut c_void // resolved via loader::NativeLib::symbol
}
static TABLE: OnceLock<Vec<KernelDesc>> = OnceLock::new();
```

* Built **once** inside `inject_surface` while the loop in
  `src/lib.rs::define_and_register` already resolves every symbol
  (`lib.symbol(sym)`), then frozen via `OnceLock`. Zero symbol lookups and
  zero allocations at dispatch time.
* `scalars`/`refs` are precomputed by porting the descriptor parser from
  `bench/p500/gen_p500_bench.py::parse_params` (same grammar: dims, `L...;`,
  primitives) into `src/batch_dispatch.rs`, unit-tested against all 49 group
  signatures in `bench/p500/java/p500/groups.tsv`.
* The JNI impls are registered like any other native via
  `env.register_natives` (helper in `src/lib.rs::define_and_register`), giving
  the dispatcher the same `JNIEnv*` the kernels expect. Kernels never receive
  the dispatcher's `jclass` in a meaningful way — verified: all 283 exports
  are static bridges taking only their declared parameters.

## 5. Encoding rules per signature shape

Signature inventory over the 49 bench groups (`groups.tsv`): 20 distinct
shapes; the hot ones are `(I[J)I` ×9, `(II[J)I` ×4, `(III[J)I` ×3, plus
`[J/[I/[D/[B/[F/[Z` array mixes, `String`, and `[Ljava/lang/Object;` variants.

| param shape | encoding | slots |
|---|---|---:|
| `I/Z/B/S/C` | sign-extended into low 32 bits of one `long` slot (high half 0/−1 per sign) | 1 |
| `J` | raw value | 1 |
| `F` | `Float.floatToRawIntBits` sign-extended to long | 1 |
| `D` | `Double.doubleToRawLongBits` | 1 |
| `[X` primitive array | reference stored in `refArgs`, consumed by the kernel via JNI array access — **no copy** | 1 ref |
| `Ljava/lang/String;`, `[Ljava/lang/Object;` | reference in `refArgs` | 1 ref |
| return `I/J/D` | kernel return value → `rets[i]` (int) or `outs[i]` (long/double bits) | — |

* Stride: `strideS(desc)`, `strideR(desc)` precomputed per `kernelId`;
  op `i` reads `scalarArgs[i*strideS ..]`, `refArgs[i*strideR ..]`.
* Trailing-`[J` "summary out" convention (P500 generator rule:
  "the last `[J` of a descriptor is the summary/dst array",
  `gen_p500_bench.py` header) is preserved: callers place the destination
  array in `refArgs`; batches writing the same dst array **append** — callers
  must partition `outs`/dst slices per op unless the kernel is
  offset-parameterized (v1: document per-kernel; do not auto-rewrite).
* Alignment rule: `scalarArgs.length == count * strideS`,
  `refArgs.length == count * strideR` (when the kernel takes refs), else
  structural error (§7).
* Booleans/bytes/chars/shorts are rare (one `[Z` group:
  `PaperNativeJigsawCanAttach`); they ride the scalar encoding, and the kernel
  receives them as `jvalue` unions exactly as a normal JNI call would.

## 6. Memory management & ThreadLocal scratch

* **Java side**: per-thread reusable buffers to keep the steady-state
  allocation-free:
  ```java
  private static final ThreadLocal<long[]>   SCALARS; // grows to count*strideS, cap 256 ops
  private static final ThreadLocal<Object[]> REFS;    // grows to count*strideR
  private static final ThreadLocal<long[]>   OUTS;    // per-op long results
  private static final ThreadLocal<int[]>    RETS;
  ```
  Callers either fill the ThreadLocal buffers directly (fast path, e.g. a
  hook's op loop) or use the `record(...)` builder API that grows them.
* **Rust side**: no per-op allocation. `scalarArgs` is pinned once per batch
  via `GetPrimitiveArrayCritical` (single `jlong*` view), decoded into a
  stack-allocated `jvalue[16]` (max scalar stride across all 283 sigs is ≤ 16;
  `PaperNativeOreFeatureLoop` has 16 params — assert at table build),
  `ReleasePrimitiveArrayCritical` after the batch. Critical sections are
  bounded: ≤ 256 ops × ~100 ns body ≈ tens of µs — within JNI critical-region
  guidance; batches above the cap fall back to
  `GetLongArrayElements`-copy for that batch (§10.4).
* `refArgs` is passed through as a `jobjectArray`; kernel-side array accesses
  behave exactly as in individual calls (same pinned/copy policy of the JVM).

## 7. Error model

* **Structural errors** (batch-level, before any op runs): null arrays,
  stride mismatch, `kernelId` out of range, `count` > 256, unknown ABI
  version, **a kernel refused by `kernel_policy`** (as-built:
  `ERR_KERNEL_REFUSED = -10`, see [`BATCH_WIRING_PLAN.md`](BATCH_WIRING_PLAN.md))
  → return negative code, throw `IllegalArgumentException` (one
  exception per batch, never per op), **no partial execution**.
* **Per-op kernel exceptions** (closed-source kernels may throw): after each
  direct call, `ExceptionCheck`; on pending exception:
  `ExceptionClear`, `rets[i] = 0x80000000`, continue with the next op.
  Callers inspect `rets` after the batch. Rationale: one JVM exception throw
  per batch-entry cost model requires clearing inside the batch; silently
  propagating op N's exception would mask ops 0..N−1 results.
* **Kernel aborts** (`panic → SIGABRT`, `SIGSEGV` on hostile input): not
  catchable. Mitigations mirror P500 practice (`bench/p500/README.md`): bench
  runs keep the one-JVM-per-group isolation + retry ladder; production wires
  only kernels already proven across the P500 matrix (129 kernels, 0 crashes),
  and batch sizes stay bounded so a lost batch is small.
* **Id/ABI drift**: `capability()` returns a version bitmask + table length;
  ids are re-derived every boot from the shipped binary (SHA-256 pinned in
  `native/MANIFEST.md`), so a swapped `.so` invalidates nothing persisted.

## 8. Expected win math

Model per individual call: `T = F + B` where `F ≈ 115 ns` (measured floor) and
`B` = kernel body. Batch of `N` ops, one transition:

```
T_batch(N) ≈ F + N·B + N·d + c
d ≈ 2–4 ns   (scalar decode + jvalue assemble + indirect C call)
c ≈ 50–100 ns (pin, stride checks, rets pass)
per-op overhead = F/N + d + c/N
```

| N | F/N | per-op overhead |
|---:|---:|---:|
| 16 | 7.2 ns | ~9–12 ns |
| **64** | **1.8 ns** | **~3–8 ns** |
| 256 | 0.45 ns | ~2–5 ns |

Concrete projections (floor-bound groups, using measured totals as `F+B`):

* `PluginNameLog.newArrayListSortSummary` (112.7 ns, body ≈ 0 ns): individual
  112.7 ns → batched ≈ 3–8 ns/op ⇒ **~15–35x** for pure-floor work.
* `PluginLoadingAllocation.newLazyValidateSummary` (140.6 ns, body ≈ 25 ns):
  → ≈ 29–33 ns/op ⇒ **~4.5x**.
* `SpigotLoadOrderDependency.newLoadAfterBuildSummary` (114.5 ns) ⇒ ~15–30x.
* NOT worth batching: `MarkerCache` (119 µs body), `NoiseInterpolatorSlice`
  (1.9 ms) — overhead reduction is < 0.01%; those groups need body-level work
  only (and 4 groups are confirmed regressions, `P500_SCALING.md` — never
  wired).

A batch-64 run over a pure-floor kernel saves
`64×115 − (115 + 64×~4 + ~80) ≈ 6.4 µs` per batch, i.e. ~100 ns/op — the
entire floor minus a few ns of decode.

## 9. Integration plan

1. **Table + descriptors** — `src/batch_dispatch.rs`: port `parse_params`,
   build `KernelDesc` during the existing injection loop (`src/lib.rs`),
   assert scalar stride ≤ 16 and freeze `OnceLock<Vec<KernelDesc>>`.
2. **Bridge class** — synthesize `PaperNativeBatchDispatch` through the same
   `bridge_class_bytes` path (`src/bridge_class.rs`) and `register_natives`
   the three JNI impls (`capability`, `dispatchRepeat`, `dispatch`). This is
   the "jni_table entry": add the class as a plugin-owned 99th bridge in
   `src/lib.rs` (the generated `src/jni_table.rs` stays a pure closed-surface
   artifact).
3. **Java helper** — `BatchDispatch.java` static facade with the ThreadLocal
   scratch (§6) + id lookup done once per kernel class at hook-arming time
   (ids are not constants across .so updates — never hardcode).
4. **BenchFloor validation** — extend `bench/p500/gen_p500_bench.py` with a
   synthetic group that, per floor-bound kernel: (a) N=64 individual calls
   (existing path), (b) one `dispatchRepeat(64)`. Report per-op overhead in
   `P500_REPORT.md` ("Batch" column). Acceptance gates (from roadmap §3.2.2):
   ≤ 8 ns/op at N=64; SINK parity vs individual calls on all primitive-only
   groups; N=1 dispatch ≤ 1.5x individual call; unknown-id → negative return.
5. **Wire call sites** — first consumer is the `area_map` op loop
   (`PaperNativeAreaMap.nativeUpdateOpsBatch` already batches; next: floor-bound
   plugin/loading kernels behind hot kernel call sites identified in roadmap
   Phase 3.1), each behind an env gate until live-server metrics pass.
6. **Docs/CI** — this proposal + `native/MANIFEST.md` note that the dispatcher
   adds a 99th plugin-owned class (does not break `render --check` /
   `verify`, which cover only the closed 283).

## 10. Open questions

1. **`refArgs` in v1 or primitive-only v1?** Primitive-only covers ~30 of 49
   groups but *excludes most floor-bound plugin/loading groups* (they carry
   `[Ljava/lang/Object;`/`String`). Recommendation: ship with `refArgs`
   immediately; the extra decode is one `GetObjectArrayElement` per ref slot.
2. **Heterogeneous `dispatch(int[] kernelIds, ...)`** — defer to v1.1;
   `dispatchRepeat` is simpler to validate and covers per-call-site usage.
3. **Mutation semantics**: ops in a batch share arrays, so op *i* observes
   writes of op *i−1* (P500's fresh-args protocol proves some kernels mutate
   inputs — `bench/p500/README.md`). Spec: strict sequential execution,
   caller-visible shared-array semantics is a documented feature (zero
   copies); never parallelize a batch in v1.
4. **Critical-section policy**: `GetPrimitiveArrayCritical` (no copy, blocks
   some GC work) vs `GetLongArrayElements` (copy, ~stride×8 ns). V1: Critical
   for N ≤ 256, copy fallback above. Measure GC pause impact in the BenchFloor
   run.
5. **`outs` ownership**: v1 keeps kernel-written summary arrays as caller
   arguments (zero-copy, matches the trailing-`[J` convention). A
   dispatcher-owned `outs` slicing scheme needs per-kernel output-length
   knowledge the manifest does not carry — only worth it if BenchFloor shows
   array-access overhead matters.
6. **kernelId stability across CE binary updates**: re-derived per boot;
   expose `capability()`/`tableLength` and fail fast on mismatch. SHA-256 of
   shipped `.so`s is the real pin (`native/MANIFEST.md`).
7. **In-batch SIGSEGV/SIGABRT**: accepted risk for proven kernels; consider a
   crash-only "quarantine" flag flipping the dispatcher off for the offending
   kernelId until restart.
8. **Upstream to CRUSSTY runtime (Phase 3)**: single shared dispatcher + id
   space for all modules (c-cells/c-collisions/c-dist) with an ABI version
   handshake; c-crussty keeps the plugin-local implementation as fallback.
9. **Interaction with hot-patches**: the `area_map`/`improved_noise` hooks
   call the bridge directly; a batched variant of the area-map op stream
   (`nativeUpdateOpsBatch` already exists as precedent) should be measured
   before replacing the single-op path — hook-local batching may beat
   dispatcher batching there because the op encoding is already flat.
