# Noise-bridge A/B — Cleaner lifecycle + per-thread direct-mapped fast path

Date: 2026-09-08 (session 003). Environment: 2-vCPU Debian 13 sandbox,
Temurin JDK 21 (`/home/z/jdk21`), G1GC, closed Crussty CE kernels from
`native/libpaper_native_jni.so`.

Question: does the hot-path rework of `ImprovedNoiseNativeOps`
(Finalizer→Cleaner lifecycle, done in session 002; synchronizedMap taken OFF
the per-call path by a per-thread direct-mapped identity cache + epoch
invalidation, session 003) change measured sampling cost?

Method: `run_noise_ab.sh` regenerates the legacy baseline FROM GIT HEAD
(finalize + `Collections.synchronizedMap` get on every call), so both bridges
live in one JVM, call the SAME closed native kernels, and the delta is pure
Java-bridge overhead. Parity check before timing: both bridges must produce
bit-identical sample sums or the run aborts (they do).

## Results (min-of-medians across forward/reverse orders)

| profile | legacy (HEAD) | current | verdict |
|---|---:|---:|---|
| single (1 noise, 1 thread) | 40.8 | 40.6 | parity |
| inter8 (8 noises, 1 thread) | 41.2 | 41.5 | parity |
| inter64 (64 noises, 1 thread) | 42.8 | 41.4 | parity |
| churn 3x50k build+drop | 877-1283 ms | 871-998 ms | no crash, no double-free |

All numbers hover at the JNI-transition floor (~40 ns/op) measured
independently by `bench/p500/jni_floor/` (pure JNI ~80-112 ns for object-heavy
signatures, java-only baseline 8-13 ns). Both bridges hide completely inside
that floor single-threaded: the map/monitor path AND the direct-mapped cache
path are effectively free against one closed-kernel call.

## Multi-thread profiles are NOT measurable on this box

Three consecutive `mt2x8` runs on the same build produced:

| run | legacy | current |
|---|---:|---:|
| 1 | 80.2 | 114.8 |
| 2 | 84.9 | 85.4 |
| 3 (JFR) | 108.2 | 52.7 |

An 80→108 swing for one variant and a 114→52 swing for the other on identical
binaries means the 2-vCPU scheduler dominates; no bridge conclusion is
possible from these numbers. The `rawMt2` probe (nativeNoise called directly
with prebuilt handles, no bridge) DID establish one hard fact:

```
nativeNoise 1 thread: 43.1 ns/op; 2 threads: 28.0 ns/op; ratio 0.65x
```

The closed kernel SCALES across threads (per-op cost DROPS with a second
worker on the second vCPU), so there is no hidden native-side serialization
that would cap worldgen worker parallelism. Which bridge wins under real
4+ core contention stays an open question for a real runner — see the
`noise-ab` CI job added alongside (GitHub runners give 4 vCPU; `mt2x8` +
`rawMt2` run there on demand).

## Why the fast path is kept anyway (design argument, not a bench claim)

* Single-threaded: parity, zero regression, reproduced every run.
* The legacy design serializes EVERY sample of EVERY worldgen worker on ONE
  monitor (`Collections.synchronizedMap`). Uncontended monitors are cheap;
  that does not change the scalability ceiling of a design where the common
  case enters a shared critical section. The current design's common case is
  a few identity compares with NO shared write (epoch is read-only in the hot
  path; slots are thread-local), so its ceiling is strictly higher.
* `releaseHandle()` correctness requires epoch invalidation regardless of
  which lookup wins; that guard is in place (volatile epoch, checked on every
  cache hit).

## Artifacts

* `run_noise_ab.sh` — regenerates baseline from git HEAD, compiles, runs.
* `src/net/.../synth/NoiseAbBench.java` — profiles: single/inter8/inter64/
  mt2x8/rawMt2/mt4x8/churn, parity-gated, order-bias-cancelled.
* Transcript: see git history of this file's companion `RESULTS.md` updates.
