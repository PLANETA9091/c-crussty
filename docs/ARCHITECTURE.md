# c-crussty — Architecture

This document is the deep dive into how the `crussty` c-plugin works: the
injection pipeline, the classfile machinery, the byte-hook contract, both
kernel hot-patches end-to-end, the native surface inventory, and the failure
modes the code guards against. Every claim here is derived from the source
(`src/`, `cplug-sdk/`, `cplug-abi/`) and verified live on Purpur 1.21.10 +
CRUSSTY launcher (see the project `worklog.md` for the session history).

Companion docs: [`../README.md`](../README.md) (quickstart, status),
[`../bench/p500/README.md`](../bench/p500/README.md) (benchmark),
[`../native/MANIFEST.md`](../native/MANIFEST.md) (binary provenance).

---

## 1. Component map

```
CRUSSTY runtime (JVMTI agent, started by launcher.jar)
  │  recursive scan of modules/ → dlopen(modules/crussty/libcrussty.so)
  │  reads module.json = {"id":"crussty","version":"0.1.0"}
  ▼
cplugin_init(api, vm, options)                 [src/lib.rs]  (cdylib export)
  │  cplug_sdk::init(api, vm)                  stash CPluginApi + JavaVM*
  │  area_map::register()                      byte hook #1   (no JVM work)
  │  improved_noise::register()                byte hook #2   (env-gated)
  └─ spawn ──────────► inject_surface()        background thread
                          │ sleep 3 s (VM finishes init)
                          │ plugin_dir() via dladdr(cplugin_init)
                          │ dlopen native/libpaper_native_jni.so        (required)
                          │ dlopen native/libpaper_native_chunk_encode_jni.so (optional)
                          │ with_attached(JNI env)
                          │ define_and_register() × 98 bridge classes
                          │      ├ bridge_class::bridge_class_bytes()  [src/bridge_class.rs]
                          │      ├ env.define_class(name, NULL loader) (bootstrap)
                          │      └ dlsym + env.register_natives()
                          │ log: "native surface live: 98 classes, 283 natives"
                          │ live_proof(): 2 real native calls through the bridge
                          ├─► area_map::activate()        worker thread
                          └─► improved_noise::activate()  worker thread (env-gated)

Byte-hook path (JVMTI ClassFileLoadHook, wrapped by cplug-abi + cplug-sdk):
  JVM class load / retransform
    └► cplug_sdk::hooks::dispatch_bytes(name, bytes)         [cplug-sdk/src/hooks.rs]
         └► registered pattern closures, chained in registration order
              ├─ area_map hook     → classfile::patch_update(bytes) when READY
              └─ improved_noise    → capture pristine / serve cached patch
```

**Timing rule.** `cplugin_init` runs on the JVMTI `OnLoad` thread *before the
VM is ready* — any JNI/JVMTI work there (even `GetLoadedClasses`) can crash
the boot. The module therefore does zero JVM work in `cplugin_init`: hooks are
only registered, and all real work happens on background threads, the first
one after a 3 s sleep (the VM is up by then; `define_class` with a null loader
and `RegisterNatives` need no kernel classes).

---

## 2. The classfile patcher (`src/classfile.rs`)

Two different classfile problems are solved in this repo, and they should not
be confused:

### 2.1 Synthesis — `src/bridge_class.rs` (classes that do not exist yet)

Every bridge class is structurally identical to:

```java
public class <name> {                       // default package or net.minecraft.* etc.
    public static native <ret> <method>(<params>);   // ×N
}
```

`bridge_class_bytes()` writes this by hand — no `javac`, no ASM, no
dependencies:

* constant pool contains only `CONSTANT_Utf8` + `CONSTANT_Class` entries
  (deduplicated through an index map), the class header, and one method entry
  per native method;
* access flags: class `0x0021` (public | super), methods `0x0109`
  (public | static | native);
* class-file **major 52** (Java 8) — runs on any kernel JVM we support;
* native methods carry **no `Code` attribute**, so the verifier needs no
  `StackMapTable` — the file is literally a constant pool plus headers and
  method descriptors.

### 2.2 Surgery — `patch_update()` (rewriting an existing kernel class)

For the area_map hot-patch the module rewrites the body of the kernel's
`SingleUserAreaMap.update(int,int,int)` inside its already-compiled classfile:

* the constant pool is **parsed** (JVMS rules: `cp_count`-1 real entries,
  long/double take two slots) and new entries are **appended only** — existing
  indices stay valid;
* the new body is hand-assembled bytecode (documented instruction-by-instruction
  in the module doc comment) that preserves the original contract exactly:
  * negative `newDistance` → `IllegalArgumentException` + `athrow`;
  * `lastChunkX == NOT_SET` (i32::MIN) → return `false`, fields untouched;
  * otherwise: write the three fields, then one
    `invokestatic SingleUserAreaMapOps.run(this, fromX, fromZ, oldD, toX, toZ, newD, param)`
    which delegates the whole difference-apply to the native-backed helper;
* the body has branches, so its `Code` attribute carries a hand-computed
  `StackMapTable` (mandatory for class-file major ≥ 51) — one frame per basic
  block leader;
* **field access flags are never touched** — the JVM rejects any
  field-modifier change in a retransformed class with
  `JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED`.

Verified live: `update()` patched 5075 → 3320 bytes, retransform rc=0.

---

## 3. The byte-hook contract (`cplug-abi` → `cplug-sdk`)

The runtime exposes a single class-file hook to every module (CPAPI ≥ 2):

```rust
// cplug-abi: JVMTI CLASS_FILE_LOAD_HOOK trampoline
ClassHookFn(ctx, name, class_data, class_data_len, out_data, out_len) -> i32
//   return 0        → JVM uses *out_data/*out_len as the new class bytes
//   return non-zero → keep the original bytes
// the replacement buffer MUST come from api.jvmti_allocate (freed by the runtime)
```

`cplug-sdk::hooks` wraps this into a friendly registry:

* `register(pattern, cb)` — name-only notification, glob syntax
  (`*` matches any run including `/`, `?` one char);
* `register_bytes(pattern, cb)` — the real contract: `cb(name, bytes) ->
  Option<Vec<u8>>`; hooks **chain in registration order**, each receiving the
  previous hook's output;
* `dispatch_bytes()` is called by the runtime for every class load *and* every
  retransform — the same callback serves both paths;
* `on_kernel_ready(class, cb)` — background polling for a loaded class
  (200 ms interval, 120 s cap), safe to call from `cplugin_init`.

**The golden rule: callbacks run on the class-loading thread while the JVM
holds loader locks — keep them cheap and free of JVM side effects.** The two
hard-earned reasons (both found as live deadlocks/bugs):

1. ASM's `ClassReader` with `COMPUTE_FRAMES` resolves `StackMapTable` frame
   types through `Class.forName` — a class definition racing live server
   class loads *inside* the redefinition callback deadlocks the JVM. So the
   ASM pipeline never runs in the callback (see §6).
2. A `define_class` performed from inside a class-file hook mid-retransform is
   equally forbidden — the ASM helper class is therefore pre-defined before
   any retransform is armed.

---

## 4. Bridge-class generation: `JNI_EXPORTS.manifest` → `jni_table.rs` → runtime

```
native/JNI_EXPORTS.manifest        (single source of truth, 283 rows)
  class|method|JNIsig|Java_symbol  — one line per export
        │
        ▼  generator (see native/MANIFEST.md) — never edit by hand
src/jni_table.rs
  MAIN_JNI_TABLE   280 entries / 97 classes  (libpaper_native_jni.so)
  CHUNK_JNI_TABLE    3 entries /  1 class    (libpaper_native_chunk_encode_jni.so)
  MAIN_BRIDGE_CLASSES / CHUNK_BRIDGE_CLASSES — definition order
        │
        ▼  define_and_register() per class  [src/lib.rs]
  1. group the table rows by class name
  2. bridge_class::bridge_class_bytes(class, [(method, sig), ...])   §2.1
  3. env.define_class(class, loader = NULL)   → bootstrap loader
  4. for each row: lib.symbol(sym) → JNINativeMethod{name, sig, fnPtr}
  5. env.register_natives(cls, natives)
  6. unresolved symbols are counted, never fatal — reported in the boot line
```

Why this works: the `Java_*` export name encodes the target package
(`Java_net_minecraft_world_level_levelgen_synth_PaperNativeNormalNoise_*`
implies `net/minecraft/world/level/levelgen/synth/PaperNativeNormalNoise`), so
the generated classes land exactly where the closed-source Crussty CE Java
side expects them — callable from **any** Paper-family kernel without a fork.

---

## 5. Loader (`src/loader.rs`)

* `NativeLib::new(path)` — `libloading` `dlopen` of the bundled `.so`.
* `symbol(name)` — exact-name resolution of each `Java_*` symbol.
* **Keep-alive:** every opened handle is `Box::leak`ed into a process-lifetime
  `KEEP_ALIVE` registry. `register_natives` stashes raw function pointers into
  JVM method entries; dropping the handle (→ `dlclose`) would leave them
  dangling. Leaking is deliberate and required.
* Library search order: `<plugin>/native/<lib>` first, then `<plugin>/<lib>`.
  `native/` deliberately contains no `module.json`, so the runtime's plugin
  scan does not treat the bundled libraries as modules.
* The chunk-encode library is **optional**: if it is missing or fails to
  `dlopen`, the module continues without its 3 exports (logged).

---

## 6. Hot-patch #1 — `SingleUserAreaMap.update()` (always on)

The kernel's (Moonrise) `SingleUserAreaMap` is loaded lazily by *its own*
loader, so the patch cannot simply define things into the bootstrap. Sequence
(`src/area_map.rs`):

```
cplugin_init        activate worker (background)          JVM / byte hook
────────────        ────────────────────────────          ────────────────
register():             │
  install byte hook  ──►│ poll cplug_sdk::classes::find_class(MAP_CLASS)
  (READY = false)       │   (JVMTI GetLoadedClasses — sees kernel-loader
                        │    classes, unlike JNI FindClass = system loader)
                        │   60 s deadline; at deadline−50 s:
                        │     force_load_kernel_class():
                        │       Class.forName(name, true, kernelLoader)
                        │       (Bukkit class seeds getClassLoader; running
                        │        <clinit> doubles as a link canary)
                        │
                     class found:
                        │ with_attached:
                        │   loader = mapClass.getClassLoader()
                        │   define_class into THAT loader:
                        │     SingleUserAreaMapOps, $Scratch, $1
                        │     (NOT bootstrap: helpers reference the kernel
                        │      class directly; a bootstrap copy would fail
                        │      to resolve it and shadow it parent-first)
                        │
                        │ READY = true ──► retransform_class(MAP_CLASS)
                        │                     │
                        │                     ▼
                        │              hook fires with CURRENT bytes
                        │              PATCHED=true gate (exactly once)
                        │              classfile::patch_update(bytes)
                        │              §2.2 → new bytes via jvmti_allocate
                        │                     │
                     self-test ◄──────────────┘
                       64 deterministic LCG rects:
                       nativeUpdateOpsBatch(...) vs naive_set_difference
                       → "self-test OK (64 rects, native == naive set difference)"
```

Live evidence: `patched ... (5075 -> 3320 bytes)`, `retransform rc=0`,
self-test OK on Purpur 1.21.10.

---

## 7. Hot-patch #2 — `ImprovedNoise.noise(DDDDD)D` (env-gated, off by default)

The noise patch replaces the body of the kernel's perlin sampling method with
a call into `ImprovedNoiseNativeOps` (compiled bridge, embedded via
`include_bytes!` from `noise/build/`), which samples through the native handle
(`PaperNativeImprovedNoise.nativeBuildHandle` / `nativeNoise`). The rewritten
body reads `this`'s private `p`/`xo`/`yo`/`zo` itself — legal, because it is
`ImprovedNoise`'s own method — so **field access flags stay untouched** (§2.2).

Gate: `CRUSSTY_NATIVE_IMPROVED_NOISE` ∈ {1, true, on, yes} enables the patch;
anything else (default) leaves it fully dormant.

```
cplugin_init        activate worker (background)            JVM / byte hook
────────────        ────────────────────────────            ────────────────
enabled()? ── no ─► log "dormant", NO hook registered, NO worker   (guard G2)
     │ yes
register():             │
  install byte hook  ──►│ hook (READY = false): capture PRISTINE bytes
     │                 │  on the class's own load — never rewrite here
     │                 │
     │                 │ poll for net/.../synth/ImprovedNoise
     │                 │   60 s deadline; force load at deadline−50 s
     │                 │
     │                 │ wait_for_boot(): org.bukkit.Bukkit.getServer()
     │                 │   non-null + 10 s settle (120 s cap)
     │                 │   (defining into the kernel loader during the
     │                 │    boot class-loading storm deadlocks defineClass1)
     │                 │
     │                 │ GUARD G1 — class-version check:
     │                 │   JVM major = System.getProperty("java.class.version")
     │                 │   embedded bridge major ≤ JVM major, else
     │                 │   refuse with actionable message, stay dormant
     │                 │
     │                 │ with_attached:
     │                 │   loader = ImprovedNoise.getClassLoader() (global ref)
     │                 │   define ImprovedNoiseNativeOps + $Handle  (kernel loader)
     │                 │   define ASM helper (cplug_sdk::asm::ensure_defined)
     │                 │     → the byte hook will never define a class
     │                 │       mid-retransform (§3 golden rule)
     │                 │
     │                 │ orig bytes missing?  (class loaded before the hook
     │                 │   registered — common: early worldgen load)
     │                 │   → no-op retransform: JVM delivers CURRENT bytes,
     │                 │     hook (still READY=false) stores them = baseline
     │                 │
     │                 │ PATCH COMPUTE (quiet thread, no JVMTI locks):
     │                 │   asm::replace_body(ReplaceBody {
     │                 │     method: "noise(DDDDD)D",
     │                 │     bridge:  ImprovedNoiseNativeOps.noise(
     │                 │               ImprovedNoise,[B, D×9)D,
     │                 │     args: this · this.p/xo/yo/zo · 5 coord locals })
     │                 │   → PATCH_CACHE
     │                 │
     │                 │ READY = true ──► retransform_class(NOISE_CLASS)
     │                 │                     │
     │                 │                     ▼
     │                 │              hook fires → serves cached patch
     │                 │              (zero Java work on this thread)
     │                 │
     │                 │ bridge_selftest(): 256-byte permutation
     │                 │   nativeBuildHandle → nonzero, nativeNoise samples
     │                 │   finite + deterministic, nativeFreeHandle clean
     │                 │   → "self-test passed (native handle round-trip)"
```

Live evidence: `pristine sighting ... (major 65)` → `patch 5691→5403 bytes` →
`retransform rc=0` → `SELF-TEST PASSED` (Purpur 1.21.10, JDK 21).

---

## 8. Native surface inventory (283 exports / 98 bridge classes)

Computed from `src/jni_table.rs` (regenerable from
`native/JNI_EXPORTS.manifest`). Subsystem grouping is by class-name analysis —
useful for orientation, not a formal taxonomy:

| Subsystem | Bridge classes | Natives | Notes |
|---|---:|---:|---|
| worldgen / noise (ImprovedNoise, Perlin, Blended, NoiseChunk, density, aquifer, surface rules, carvers, ore, climate, splines…) | 45 | 138 | the largest block; includes the P500 noise kernels |
| tickets & waypoints (TicketSet, TicketPack, TicketCompare, Waypoint*) | 10 | 35 | O(N) hot-path kernels, P500 WaypointHotPath group |
| chunks, heightmaps & palettes (PalettedReencode*, Remapper*, *Heightmap, Chunk*) | 10 | 23 | includes the 4 documented regressions |
| plugin loading (Plugin*, MarkerCache) | 8 | 22 | 13 groups sit on the 35–90 ns JNI-transition floor (TASK-33 errata: the old "~40 groups / ~115 ns" was a pre-audit v1 artifact — see BATCH_ADOPTION_MATRIX.md) |
| io / compression / nbt (LZ4 stream, Deflater, NBT, VarInt, Hash, Gzip) | 10 | 18 | |
| entities & lookups (EntityBoundingBox, EntityLookupStatus, CraftPlayerCanSee, Position, ReferenceList, ServerEntityDeltaIdentity) | 6 | 25 | |
| misc (biome lookup, static caches, obf-helper maps, alias removal, load order, NearbyPlayerMap) | 6 | 15 | |
| area map (Moonrise) — `PaperNativeAreaMap` (default pkg, summary kernels) + `ca/.../misc/PaperNativeAreaMap` (native ops) | 2 | 4 | backs hot-patch #1 (`nativeUpdateOpsBatch`) |
| chunk packet encode — `net/.../game/PaperNativeChunkPacketEncode` | 1 | 3 | separate lib `libpaper_native_chunk_encode_jni.so` |
| **Total** | **98** | **283** | 280 exports in the main lib, 3 in the chunk-encode lib |

Naming conventions inside the surface (what P500 exploits):

* `old*` vs `optimized*` / `guarded*` / `direct*` / `new*` — A/B kernel pairs;
* `*Summary` — measurement kernels that write `SUMMARY_FIELDS` longs into an
  array (return value = count), used both by P500 and by the injection
  live-proof.

---

## 9. Failure modes and guards

| # | Failure mode | Guard in code |
|---|---|---|
| G1 | **Class-version mismatch** — bridge `.class` compiled by a newer javac (major 69 from javac 25) into a Java 21 kernel dies with a bare `UnsupportedClassVersionError` that names no source | bridges pinned to **major 52** (`bridge_class.rs`, `scripts/build_noise.sh --release 8`); at activation the JVM's real max is read from `java.class.version` and the patch refuses early with actionable numbers (both for embedded bytes and per-define) |
| G2 | **Dormant-gate leak** — `improved_noise::activate()` used to define bridges even when the env gate was off; dead classes nobody calls, and it masked the v69 bug | `activate()` returns immediately when `enabled()` is false; the byte hook is then *not registered at all* — dormancy is total |
| G3 | **Deadlock in the byte-hook callback** — `COMPUTE_FRAMES` → `Class.forName` racing boot-time loads, or `define_class` inside a hook | callbacks capture/serve bytes only; ASM runs on the quiet activation worker; ASM helper pre-defined before any retransform (§3) |
| G4 | **Boot-storm deadlock** — `define_class` into the kernel loader while worldgen codecs load classes on the main thread | `wait_for_boot()`: `Bukkit.getServer()` non-null + 10 s settle before any define |
| G5 | **Kernel class never loads** (lazy Moonrise load on an idle world) | 60 s poll via JVMTI `GetLoadedClasses`; half-way force load via `Class.forName(name, true, kernelLoader)` whose `<clinit>` run doubles as a link canary |
| G6 | **Class predates the hook** — `ImprovedNoise` loads during early worldgen before the plugin's hook registers | no-op retransform delivers the class's *current* bytes through the (still passive) hook → baseline capture without JVMTI locks |
| G7 | **Dangling native function pointers** — `dlclose` after `register_natives` | dlopen handles deliberately leaked for the process lifetime (`KEEP_ALIVE`) |
| G8 | **Retransform schema change** — JVM rejects field-modifier changes | patchers never touch access flags; private fields are read from the class's own rewritten method |
| G9 | **VM not ready at `cplugin_init`** | no JNI/JVMTI work in init; +3 s sleep, background threads only |
| G10 | **Noisy expected failures** — transient lookups (`find_class` before the kernel is up) throw routinely | `clear_exception()` everywhere a miss is expected; `describe_exception()` only on real faults |
| G11 | **Missing/optional payload** | main `.so` missing → explicit install hint and abort; chunk-encode `.so` missing → continue with 3 exports lost |
| G12 | **Kernel classloader scoping** — bridge classes referencing kernel classes must not live in the bootstrap | helper/bridge classes are defined into the *kernel class's own loader* (parent-first shading trap documented in both hot-patches) |

---

## 10. Where new hooks plug in

A new hot-patch follows the exact template of `area_map` (simple) or
`improved_noise` (env-gated, bridge-in-kernel-loader):

1. **Identify the kernel class + method** and the optimization. If it needs
   native support, check whether a suitable export already exists in
   `native/JNI_EXPORTS.manifest`; if not, the export must come from the
   closed-source Crussty CE crates (this repo cannot add native kernels).
2. **Build the Java bridge** against compile-time stubs of the kernel shapes
   (pattern: `area-map/RuntimeStubs.java`, `noise/RuntimeStubs.java` — stubs
   are discarded, only real bridge classes ship), compile with a pinned
   `--release` and embed via `include_bytes!` (pattern:
   `scripts/build_noise.sh`).
3. **`register()`** — install a `cplug_sdk::hooks::register_bytes(...)` hook:
   capture pristine bytes while `READY=false`; serve the cached patch when
   `READY=true`. Zero JVM work in the callback (§3).
4. **`activate()`** — spawn a worker: poll `classes::find_class` (with
   force-load fallback) → `wait_for_boot()` → class-version guard → define
   helper/bridge classes into the **kernel class's loader** → compute the
   patch (ASM `replace_body` or `classfile.rs` surgery) on the quiet thread →
   flip `READY` → **exactly one** `retransform_class` → run a semantic
   self-test through the real bridge and log PASS/FAIL.
5. **Claim unique names** — use `cplug_sdk::claim("class:a/b/C")` /
   `claim("native:...")` so co-resident modules cannot collide on bridge class
   or native bindings.
6. **Prove it live** — a patch without a self-test or a worklog entry with the
   boot-log evidence is not done (see README "Verification story").

For pure native surface (no bytecode patching), a new export only needs a row
in `JNI_EXPORTS.manifest` — the bridge class and binding are generated
(§4); regenerate the table, never hand-edit `jni_table.rs`.
