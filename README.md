# c-crussty — the Crussty CE native surface as a CRUSSTY module

**c-crussty** is a [CRUSSTY](https://github.com/PLANETA9091/CRUSSTY) c-plugin
module that injects the full **Crussty CE JNI bridge** into any Paper-family
kernel (Paper/Purpur, unmodified): **98 bridge classes** backed by **283 native
JNI exports** from two closed-source shared libraries
(`libpaper_native_jni.so`, `libpaper_native_chunk_encode_jni.so`), plus two
targeted **hot-patches** on kernel classes. It is pure infrastructure: it
changes no gameplay values and adds no content — it makes the Crussty CE
native surface *callable* from the kernel and optimizes two well-defined
kernel hot paths (area-map updates, improved-noise sampling).

The bridge classes are *not* part of Crussty CE — this module synthesizes them
at runtime: for every `Java_*` export in the JNI manifest it defines a
`public static native` class in the exact package the symbol name implies
(bootstrap loader) and binds the resolved symbol via `RegisterNatives`.

## Status — what works today

| Capability | State | Evidence |
|---|---|---|
| Native surface injection — 98 bridge classes / 283 natives | ✅ working | live boot on Purpur 1.21.10 + CRUSSTY launcher: `98 bridge classes, 283 natives registered, 0 unresolved` |
| `SingleUserAreaMap.update()` hot-patch | ✅ working, verified live | patched `5075 → 3320` bytes, retransform rc=0, 64-rect self-test OK (native == naive set difference) |
| `ImprovedNoise.noise(DDDDD)D` hot-patch | ✅ working, **off by default** (env gate) | pristine sighting major 65 → patched `5691 → 5403` bytes → retransform rc=0 → native handle round-trip self-test passed |
| Injection live-proof (self-check at boot) | ✅ working | `nativeCheck() = 1`; `binarySummary(1000)` wrote 1 long through a real kernel |
| P500 benchmark baseline | ✅ recorded | 49 groups, 129 kernels, 0 crashes — see [`bench/p500/`](bench/p500/README.md) |

## Architecture

```
                   CRUSSTY runtime (JVMTI agent, launcher.jar)
                        │  dlopen modules/crussty/libcrussty.so
                        ▼
              cplugin_init(api, vm, options)          ← the only required export
                │
                ├── area_map::register()          ── byte hooks (class-file load)
                ├── improved_noise::register()       no JVM work here
                └── spawn inject_surface thread
                        │  +3 s (VM up)
                        ▼
   dlopen native/libpaper_native_jni.so (+ chunk_encode lib)
                        │
                        ▼
   define_and_register loop  ── per bridge class (98):
        bridge_class_bytes() → define_class (bootstrap loader)
        dlsym each export   → RegisterNatives
                        │
                        ▼
   live_proof: nativeCheck()=1 · binarySummary(1000) writes 1 long
                        │
                        ▼
   area_map::activate() ────────┐   background workers: wait for kernel
   improved_noise::activate() ──┘   class → define helpers into the kernel
                                    loader → ONE retransform → self-test
```

Details, sequence diagrams and failure-mode analysis: [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).

## Layout

- `src/` — Rust module (`lib.rs` pipeline, `jni_table.rs` 283-entry bridge table, `bridge_class.rs` classfile writer, `classfile.rs` bytecode surgery, `area_map.rs`, `improved_noise.rs`, `loader.rs`)
- `cplug-abi/`, `cplug-sdk/` — vendored module ABI + SDK (byte hooks, ASM weaving, kernel-class polling)
- `area-map/` — Java sources of the Moonrise `SingleUserAreaMapOps` bridge, `build/` — compiled (embedded via `include_bytes!`)
- `noise/` — Java sources of the noise bridge (`ImprovedNoiseNativeOps`), `build/` — compiled
- `native/` — Crussty CE binaries + `JNI_EXPORTS.manifest` (single source of truth for the bridge table) + MIT license, see [`native/MANIFEST.md`](native/MANIFEST.md)
- `bench/p500/` — the P500 JNI kernel benchmark, see [`bench/p500/README.md`](bench/p500/README.md)
- `bench/lifecycle/` — P500-style A/B of the noise-handle lifecycle (old finalize+global-map vs phantom reaper+striped maps) over the real `libpaper_native_jni.so` kernels; report in `bench/lifecycle/results/LIFECYCLE_REPORT.md`
- `docs/` — architecture deep dive ([`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md))
- `scripts/build_noise.sh` — rebuilds the noise bridge classes with a pinned `--release` (class-version guard, see [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) §9)
- `tests/fixtures/` — test fixtures

## Build

```bash
# toolchain (once): Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo build --release          # → target/release/libcrussty.so  (cdylib)
```

Runtime dependencies `native/libpaper_native_jni.so` and
`native/libpaper_native_chunk_encode_jni.so` are committed here (see
`native/MANIFEST.md` for provenance and SHA-256); the module is a dead
injector without them.

## Deploy

The CRUSSTY runtime scans `modules/` recursively; every module is a directory
holding the shared library, its manifest and its native payloads:

```
modules/crussty/
├── libcrussty.so                 # cargo build --release artifact
├── module.json                   # {"id":"crussty","version":"0.1.0"}
└── native/
    ├── libpaper_native_jni.so            # 280 exports
    ├── libpaper_native_chunk_encode_jni.so  # 3 exports
    ├── JNI_EXPORTS.manifest
    ├── MANIFEST.md · LICENSE
```

(`native/` is deliberately a subdir without `module.json`, so the runtime's
plugin scan skips it; the injector falls back to the module dir itself if
`native/` is absent.)

**Deploy requirements** (mirror the `cp` conventions in
[`native/MANIFEST.md`](native/MANIFEST.md)): the CRUSSTY runtime
(`libcrussty_runtime.so`) must include the TASK-08 ClassFileLoadHook name fix
(engine commit `66ff504`, "derive class name from class bytes, not the VM
buffer") — older runtimes intermittently drop hook events whose name is read
off the VM's non-NUL-terminated buffer (`ImprovedNoisejaE`), so armed capture
becomes non-deterministic. Module builds >= capture-hardening (`625c564`)
additionally retry the retransform ×3 and fall back to the kernel-loader
resource stream, which restores deterministic arming on residual races.

## Run

Start the kernel through the CRUSSTY launcher (it loads the runtime, which
dlopens every module). The module registers itself at `cplugin_init` and does
all JVM work on background threads — no agent flags needed:

```bash
cd server/
java -jar launcher.jar nogui          # or: crussty run
```

Expected boot log:

```
[crussty-plugin] native surface live: 98 bridge classes, 283 natives registered (0 symbols unresolved)
[crussty-plugin] live proof: normalNoise.nativeCheck() = 1
[crussty-plugin] live proof: ticketset binarySummary(1000) wrote 1 long(s)
[crussty-plugin] area_map: patched ... update() (5075 -> 3320 bytes)   # once the kernel class loads
[crussty-plugin] area_map: self-test OK (64 rects, native == naive set difference)
```

**Env gate:** the `ImprovedNoise` hot-patch is off by default. Enable it by
setting the variable in the environment *at server start*:

```bash
CRUSSTY_NATIVE_IMPROVED_NOISE=1 java -jar launcher.jar nogui
```

## P500 benchmark

P500 is the revived Crussty CE kernel benchmark: it measures every kernel
group that has an `old*`/optimized kernel pair with identical synthesized
arguments, one JVM fork per group, median-of-5 time-batched, min-of-medians
over a forward/reverse pass to kill order bias. Baseline (2-CPU sandbox, JDK
21): 49 groups, 129 kernels, 0 crashes; headline win **NoiseChunkBlendCache
244×** (67 µs → 275 ns); four genuine scale-invariant regressions are
documented and flagged do-not-wire.

```bash
cd bench/p500/
python3 gen_p500_bench.py   # JNI_EXPORTS.manifest → stubs + G<gid> groups + Bench.java
./run_p500.sh               # javac + one JVM per group → results/p500_raw.tsv
python3 aggregate_p500.py   # → results/P500_REPORT.md
```

Results: [`bench/p500/results/P500_REPORT.md`](bench/p500/results/P500_REPORT.md),
scaling study: [`bench/p500/results/P500_SCALING.md`](bench/p500/results/P500_SCALING.md).

## Verification story

Everything the module claims is checked *live, on the running kernel*:

1. **Injection live-proof** — after the `define_and_register` loop the module
   calls two real natives through the injected bridge: `nativeCheck()`
   (jboolean, always 1 — define + register + symbol resolve all worked) and
   `binarySummary(1000)` (a real benchmark kernel that must write exactly one
   long into an array — array passthrough works).
2. **area_map self-test** — after patching, the real bridge
   (`nativeUpdateOpsBatch`) is driven over 64 deterministic random rectangles
   and every produced `(op, x, z)` is compared against the naive set
   difference (adds = new∖old, removes = old∖new).
3. **improved_noise self-test** — after activation, the real bridge
   (`nativeBuildHandle`/`nativeNoise`/`nativeFreeHandle`) is driven over a
   synthetic 256-byte permutation: handle builds, samples are finite and
   deterministic, handle frees cleanly.
4. **P500 harness** — reproducible old-vs-optimized measurement of the whole
   native surface, used as the regression gate when the `.so` files change.

## Docs

- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) — deep dive: component map, classfile patcher, byte-hook contract, both hot-patches end-to-end, native surface inventory, failure modes
- [`docs/KERNEL_POLICY.md`](docs/KERNEL_POLICY.md) — enforced kernel-selection policy: do-not-wire registry (4 confirmed P500 regressions), proven-win whitelist, `CRUSSTY_KERNEL_POLICY` override
- [`bench/p500/README.md`](bench/p500/README.md) — P500 methodology & fairness protocol
- [`native/MANIFEST.md`](native/MANIFEST.md) — provenance, license (`native/LICENSE`, MIT © ANDMC / P500 Project Contributors) and SHA-256 of the bundled binaries

## Contributing (including autonomous agents)

Development is agent-driven. Before working on this repo:

1. **Read `worklog.md` first** (mirrored to the private `crussty-dev-logs`
   repo) — it holds project context, session history and the template.
2. Work autonomously; never change gameplay values, never ask the user to
   test.
3. **Append a session section** after finishing (`SESSION NNN — date —
   topic`, Work Log + Stage Summary + NEXT), following the template at the
   bottom of the worklog, and push it.
4. Every nontrivial claim must be backed by a live proof or a benchmark
   artifact in the repo (see the verification story above).

Rules of thumb: `src/jni_table.rs` is generated from
`native/JNI_EXPORTS.manifest` — never edit it by hand; bridge classfiles in
`area-map/build/` and `noise/build/` are compiled artifacts embedded via
`include_bytes!` — rebuild them from `area-map/` / `noise/` sources with a
pinned `--release` (see `scripts/build_noise.sh`).
