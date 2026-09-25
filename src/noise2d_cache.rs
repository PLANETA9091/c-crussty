//! Runtime wiring for the `noise2d_cache` hook — P25 PROTOTYPE (gate OFF).
//!
//! TASK-459-65 / ID-P25 (RESEARCH-458-P.md, WILD закон-11 тик-459): flat
//! cache of 2D noise-router slices — temperature / vegetation / continents /
//! erosion / ridges (XZ → double[]) — reused across ALL 3D columns of one
//! chunk column. Vanilla `NoiseChunk.Cache2D` is a ONE-slot memo per function
//! instance (`lastSamplingColumnPos`/`lastSamplingResult`), so interleaved
//! evaluation + repeated column visits (interpolated corner columns ×8,
//! blend/density tails) thrash it: every miss = full octave recompute.
//! P25 = pure-java memo over the existing kernel wrappers, NO JNI in the hot
//! path; epoch invalidation by (seed, size) ONLY — noise is never mutated by
//! the world (card: «инвалидация только seed/размер»).
//!
//! Bit-for-bit contract (same noise call order as vanilla):
//!   * MISS = delegate.compute on the live provider — the exact vanilla
//!     first-touch evaluation order (recording discipline, cf. noise_fill.rs);
//!   * HIT  = return the stored f64 BITS of that first evaluation, zero noise
//!     calls. The consumer observes the same value sequence as vanilla.
//! Phase-1 enforcement: deterministic Rust reference-memo self-test
//! (`memo_bitexact_selftest`, f64::to_bits 1:1); Phase-2 (main agent) adds the
//! 10k-sample old-vs-new bridge harness à la proto_blend_cache.
//!
//! java-memo stub (Phase 2 — description ONLY, nothing defined in Phase 1):
//!   bridge class `net/minecraft/world/level/levelgen/Noise2DMemoOps`
//!   (+ nested `Slice`), defined EARLY into the KERNEL loader (same-loader
//!   rule as ImprovedNoiseNativeOps — a bootstrap copy would fail to resolve
//!   kernel classes under parent-first). NCDFE canon: define on the quiet
//!   activation worker AFTER the kernel NoiseChunk class is confirmed loaded,
//!   class-version guard `--release 8`; ZERO class definitions inside a
//!   retransform callback (ClassReader COMPUTE_FRAMES → Class.forName →
//!   deadlock, see src/improved_noise.rs header). Fields: `long epoch` (seed +
//!   dimension min_y/height + router identity, checked with one long-cmp),
//!   `double[] vals` per router slot (index `(localZ << 4) | localX`),
//!   `long[] filled` bitmask (256 columns). Fail-closed: any error ⇒ vanilla
//!   Cache2D path (one-slot memo), never a NaN (C2ME CACHE_MISS_NAN_BITS is a
//!   duck-interface sentinel, NOT a legal return to worldgen).
//!
//! Unlike area_map/improved_noise this module is **observation-only by
//! construction** in Phase 1: a wrong patch here changes terrain generation,
//! which is FORBIDDEN (project rule: no gameplay changes). Therefore:
//!   1. Gate: env `CRUSSTY_NATIVE_NOISE2D_CACHE` (1/true/on/yes → on), OFF by
//!      default. Gate OFF ⇒ byte hook NOT registered, activate() a no-op
//!      (dormant discipline, lesson 3a270ee).
//!   2. Even gate ON, the byte hook only CAPTURES pristine NoiseChunk bytes
//!      (`PATCH_ENABLED = false` ⇒ it can never serve a patch).
//!   3. The activation worker probes candidate patch points on the LIVE
//!      classes (existence check only) and runs the pure-Rust memo self-test;
//!      results go to the log.
//!
//! Deliberate duplication of helpers (wait_for_boot / class_version /
//! force-load pattern) from src/proto_blend_cache.rs and src/improved_noise.rs:
//! this module must compile IN ISOLATION (no `crate::` references) while the
//! main agent wires it.

#![allow(dead_code)]

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Owner of the 2D-cache wrappers (Mojang-mapped internal name; Paper 1.21.x
/// is Mojang-mapped). TODO(P25 Phase 2): confirm exact wrapper surface with
/// `javap -p -c` against the running kernel before any patch work (research
/// §5 R3; Cache2D fields per C2ME @Shadow: lastSamplingColumnPos /
/// lastSamplingResult).
pub const NOISE_CHUNK_CLASS: &str = "net/minecraft/world/level/levelgen/NoiseChunk";
/// Neighbor classes for the Phase-2 probe (existence check only, no patch):
/// NoiseRouter carries the 2D fields temperature/vegetation/continents/
/// erosion/ridges (research §1 S1/S4), Climate$Sampler is the cached climate
/// accessor (NoiseRouter.cachedClimateSampler).
pub const NOISE_ROUTER_CLASS: &str = "net/minecraft/world/level/levelgen/NoiseRouter";
pub const CLIMATE_SAMPLER_CLASS: &str = "net/minecraft/world/level/levelgen/Climate$Sampler";

/// Phase-2 java-memo bridge — description only in Phase 1 (see header stub):
/// EARLY-define into the kernel loader, NCDFE canon.
pub const MEMO_BRIDGE_CLASS: &str = "net/minecraft/world/level/levelgen/Noise2DMemoOps";

/// NEW env gate for this hook (OFF by default — do not "fix" this).
pub const GATE_ENV: &str = "CRUSSTY_NATIVE_NOISE2D_CACHE";

/// HARD OFF in this prototype. Even with the env gate set, no patched
/// bytecode is ever served until the Phase-2/3 pipeline exists. Flipping this
/// bool alone does NOT enable a patch.
const PATCH_ENABLED: bool = false;

/// Router slots served by the 2D memo (research §2; wiki field order).
pub const SLOT_TEMPERATURE: usize = 0;
pub const SLOT_VEGETATION: usize = 1;
pub const SLOT_CONTINENTS: usize = 2;
pub const SLOT_EROSION: usize = 3;
pub const SLOT_RIDGES: usize = 4;
pub const SLOT_COUNT: usize = 5;

/// 16×16 columns per chunk column; slice index = (local_z << 4) | local_x.
pub const COLUMNS_PER_CHUNK: usize = 256;

/// Candidate probe points (existence check only). Names are the Mojang-map
/// guesses — a candidate that does not resolve on the live class means
/// "redesign", never "force" (proto_blend_cache §CANDIDATE_PATCH_METHODs).
const CANDIDATE_PATCH_METHODS: &[(&str, &str)] = &[
    (
        "wrap",
        "(Lnet/minecraft/world/level/levelgen/DensityFunction;)Lnet/minecraft/world/level/levelgen/DensityFunction;",
    ),
    (
        "cache2d",
        "(Lnet/minecraft/world/level/levelgen/DensityFunction;)Lnet/minecraft/world/level/levelgen/DensityFunction;",
    ),
];

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

static READY: AtomicBool = AtomicBool::new(false);
/// Set once the Phase-1 memo self-test passed (bit-exact). A failed run is
/// FINAL for this process (no retry, hook stays dormant).
static MEMO_OK: AtomicBool = AtomicBool::new(false);
static PATCHED: AtomicBool = AtomicBool::new(false);

/// Pristine NoiseChunk bytes captured by the byte hook (observation-only in
/// this prototype; consumed by the Phase-3 patch computation).
static ORIG_BYTES: OnceLock<Mutex<Option<Vec<u8>>>> = OnceLock::new();

fn orig_lock() -> &'static Mutex<Option<Vec<u8>>> {
    ORIG_BYTES.get_or_init(|| Mutex::new(None))
}

/// Wiring diagnostics for the main agent's status logs.
#[allow(dead_code)]
pub fn memo_passed() -> bool {
    MEMO_OK.load(Ordering::Relaxed)
}

/// True once a Phase-3 patch has actually been served (always false here).
#[allow(dead_code)]
pub fn patched() -> bool {
    PATCHED.load(Ordering::Relaxed)
}

// ---------------------------------------------------------------------------
// Reference memo (Rust model of the Phase-2 java Slice) — pure std, no JNI.
// ---------------------------------------------------------------------------

/// Flat 2D slice cache for ONE chunk column: per router slot a double[256]
/// XZ→value plane + fill bitmask. Semantics pinned by memo_bitexact_selftest:
/// MISS computes through the delegate once (vanilla order), HIT replays the
/// stored bits. Epoch invalidation: seed/size change only (card contract) —
/// `invalidate(new_epoch)` clears the planes in O(1) bookkeeping (fill mask
/// reset), no per-world mutation can ever bump it.
pub struct SliceCache2D {
    epoch: u64,
    filled: [u64; SLOT_COUNT],
    vals: [[f64; COLUMNS_PER_CHUNK]; SLOT_COUNT],
}

impl SliceCache2D {
    /// 5 × 256 × 8B = 10 KiB per chunk column — GC-neutral (research §4).
    pub fn new(epoch: u64) -> Self {
        SliceCache2D {
            epoch,
            filled: [0u64; SLOT_COUNT],
            vals: [[0.0f64; COLUMNS_PER_CHUNK]; SLOT_COUNT],
        }
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    /// Epoch bump — ONLY on seed/size change (server restart / dimension
    /// switch). Clears fill masks; planes are lazily recomputed on MISS.
    pub fn invalidate(&mut self, new_epoch: u64) {
        self.epoch = new_epoch;
        self.filled = [0u64; SLOT_COUNT];
    }

    #[inline]
    fn idx(local_x: usize, local_z: usize) -> Option<(usize, u64)> {
        if local_x >= 16 || local_z >= 16 {
            return None;
        }
        let i = (local_z << 4) | local_x;
        Some((i, 1u64 << i))
    }

    /// MISS = `compute()` (delegate, vanilla first-touch order), stores bits;
    /// HIT = stored bits, `compute` never called. Returns (value, was_hit).
    pub fn get_or_insert<F: FnOnce() -> f64>(
        &mut self,
        slot: usize,
        local_x: usize,
        local_z: usize,
        compute: F,
    ) -> (f64, bool) {
        let Some((i, mask)) = Self::idx(local_x, local_z) else {
            // Out-of-chunk column: never cached, always recompute (fail-open
            // to the delegate = vanilla behavior for foreign columns).
            return (compute(), false);
        };
        if slot >= SLOT_COUNT {
            return (compute(), false);
        }
        if self.filled[slot] & mask != 0 {
            return (self.vals[slot][i], true);
        }
        let v = compute();
        self.vals[slot][i] = v;
        self.filled[slot] |= mask;
        (v, false)
    }

    /// Distinct (slot, column) planes filled — self-test bookkeeping.
    pub fn filled_columns(&self) -> usize {
        self.filled.iter().map(|m| m.count_ones() as usize).sum()
    }
}

/// Deterministic stand-in for a kernel 2D density function: pure function of
/// (slot, world x, world z) — same input ⇒ same bits, forever (the property
/// the epoch contract relies on: noise is not mutated by the world).
fn memo_delegate(slot: usize, wx: i32, wz: i32) -> f64 {
    let mut h: u64 = 0x9E37_79B9_7F4A_7C15;
    h ^= (slot as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^= (wx as i64 as u64).wrapping_mul(0x94D0_49BB_1331_11EB);
    h ^= (wz as i64 as u64).rotate_left(17);
    h ^= h >> 31;
    f64::from_bits(h)
}

/// Phase-1 self-test: direct evaluation vs memo replay must be bit-exact,
/// delegate must be called EXACTLY once per (slot, column) (zero calls on
/// HIT), and epoch invalidation must re-arm MISS. Deterministic (fixed-seed
/// xorshift64, proto_blend_cache-style). Returns (ok, misses, hits).
fn memo_bitexact_selftest() -> (bool, usize, usize) {
    let mut seed: u64 = 0x2545_F491_4F6C_DD1D;
    let mut next = move || -> u64 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };

    // One chunk column at a fixed world base (memo is per-NoiseChunk).
    let base_x = (next() % 4096) as i32 * 16;
    let base_z = (next() % 4096) as i32 * 16;
    let mut cache = SliceCache2D::new(next());

    let mut misses = 0usize;
    let mut hits = 0usize;
    let mut delegate_calls = 0usize;
    let mut failures = 0usize;

    for _ in 0..20_000 {
        let slot = (next() % SLOT_COUNT as u64) as usize;
        let lx = (next() % 16) as usize;
        let lz = (next() % 16) as usize;

        // Direct path: vanilla semantics — evaluate every time.
        let direct = memo_delegate(slot, base_x + lx as i32, base_z + lz as i32);
        // Memo path: first touch computes, the rest replay bits.
        let (memo, was_hit) = {
            let calls = &mut delegate_calls;
            cache.get_or_insert(slot, lx, lz, || {
                *calls += 1;
                memo_delegate(slot, base_x + lx as i32, base_z + lz as i32)
            })
        };
        if was_hit {
            hits += 1;
        } else {
            misses += 1;
        }
        if direct.to_bits() != memo.to_bits() {
            failures += 1;
            eprintln!(
                "[crussty-plugin] noise2d_cache: SELF-TEST FAIL slot={slot} col=({lx},{lz}): direct=0x{:016x} memo=0x{:016x}",
                direct.to_bits(),
                memo.to_bits()
            );
        }
    }

    // No noise calls on HIT: delegate ran exactly once per distinct column.
    let distinct = cache.filled_columns();
    if delegate_calls != distinct {
        failures += 1;
        eprintln!(
            "[crussty-plugin] noise2d_cache: SELF-TEST FAIL call-order: delegate_calls={delegate_calls} distinct_columns={distinct}"
        );
    }

    // Epoch invalidation: bump ⇒ everything MISSes again, same bits out.
    let before = memo_delegate(SLOT_EROSION, base_x + 3, base_z + 5);
    cache.invalidate(next());
    let (after, was_hit) = cache.get_or_insert(SLOT_EROSION, 3, 5, || {
        delegate_calls += 1;
        memo_delegate(SLOT_EROSION, base_x + 3, base_z + 5)
    });
    if was_hit || after.to_bits() != before.to_bits() {
        failures += 1;
        eprintln!(
            "[crussty-plugin] noise2d_cache: SELF-TEST FAIL epoch: was_hit={was_hit} bits_equal={}",
            after.to_bits() == before.to_bits()
        );
    }

    if failures == 0 {
        (true, misses, hits)
    } else {
        (false, misses, hits)
    }
}

// ---------------------------------------------------------------------------
// Gate
// ---------------------------------------------------------------------------

/// env-gate (off by default), read at register/activate time.
/// Same accepted spellings as proto_blend_cache / improved_noise.
pub fn enabled() -> bool {
    std::env::var(GATE_ENV)
        .map(|v| {
            let v = v.trim().to_ascii_lowercase();
            v == "1" || v == "true" || v == "on" || v == "yes"
        })
        .unwrap_or(false)
}

/// Class-file version of `b` as (major, minor), or None if not a class file.
/// (Copy of proto_blend_cache::class_version — kept local for isolation.)
fn class_version(b: &[u8]) -> Option<(u16, u16)> {
    if b.len() < 8 || u32::from_be_bytes(b[0..4].try_into().ok()?) != 0xCAFE_BABE {
        return None;
    }
    Some((
        u16::from_be_bytes([b[6], b[7]]),
        u16::from_be_bytes([b[4], b[5]]),
    ))
}

// ---------------------------------------------------------------------------
// register() — byte hook (gate OFF ⇒ nothing happens)
// ---------------------------------------------------------------------------

/// Register the byte hook (idempotent; call once from cplugin_init).
///
/// Gate OFF: log dormant and DO NOT register anything (dormant-gate-leak
/// lesson from improved_noise 3a270ee). Gate ON: observation-only — captures
/// the pristine NoiseChunk bytes, NEVER serves a patch (PATCH_ENABLED=false).
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] noise2d_cache: dormant (P25 phase-1 scaffold; set {GATE_ENV}=1 to run the read-only probe + memo self-test — no patch ships)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(NOISE_CHUNK_CLASS, |name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            // Pristine sighting: stash once, never rewrite here.
            eprintln!(
                "[crussty-plugin] noise2d_cache: pristine sighting {name} ({} bytes, major {})",
                bytes.len(),
                class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            let mut orig = orig_lock().lock().unwrap();
            if orig.is_none() {
                *orig = Some(bytes.to_vec());
            }
            return None;
        }
        // Phase 3 (not in this prototype): serve the precomputed patch.
        // TODO(P25 Phase 3): implement ONLY after the research §3 parity
        // checklist passes (10k old-vs-new bridge harness) and the §5 R1
        // noise_fill.rs layering audit is signed off. Zero class definitions
        // may happen inside this callback (COMPUTE_FRAMES deadlock).
        if PATCH_ENABLED {
            PATCHED.store(true, Ordering::SeqCst);
        }
        None
    });
}

// ---------------------------------------------------------------------------
// activate() — background worker (gate OFF ⇒ nothing happens)
// ---------------------------------------------------------------------------

/// Background activation: wait for boot, run the pure-Rust memo self-test,
/// probe candidate patch points + neighbor classes on the LIVE kernel, log
/// the verdict. Defines NO classes and retransforms NOTHING in this prototype.
pub fn activate() {
    if !enabled() {
        // register() already logged the dormant notice; mirror
        // proto_blend_cache: no background work at all when gated off.
        return;
    }
    std::thread::spawn(|| {
        if !wait_for_boot() {
            eprintln!(
                "[crussty-plugin] noise2d_cache: boot marker not seen, hook stays dormant"
            );
            return;
        }

        // 1) Memo self-test (pure Rust, deterministic, bit-exact contract).
        let (ok, misses, hits) = memo_bitexact_selftest();
        MEMO_OK.store(ok, Ordering::Release);
        if !ok {
            eprintln!(
                "[crussty-plugin] noise2d_cache: memo self-test FAILED, hook stays dormant (no patch will be served this process)"
            );
        } else {
            eprintln!(
                "[crussty-plugin] noise2d_cache: memo self-test OK (bit-exact; {misses} misses / {hits} hits, epoch re-arm verified)"
            );
        }

        // 2) Patch-point probe (existence check of research §3 candidates
        //    only) + neighbor class confirmation.
        cplug_sdk::jni_util::with_attached(|env| {
            log_candidate_sighting(env);
            Some(())
        });

        // 3) Patch step — disabled by construction in this prototype.
        if !ok {
            return;
        }
        if !PATCH_ENABLED {
            eprintln!(
                "[crussty-plugin] noise2d_cache: observation-only phase-1 complete (PATCH_ENABLED=false, nothing patched; memo bridge {} is a Phase-2 EARLY-define stub)",
                MEMO_BRIDGE_CLASS
            );
            return;
        }
        // TODO(P25 Phase 2): define Noise2DMemoOps (+nested Slice) into the
        // KERNEL loader (improved_noise define_class pattern, class-version
        // guard) AFTER kernel-class polling — NCDFE canon: EARLY-define,
        // never inside a retransform callback.
        // TODO(P25 Phase 3): compute the patch from ORIG_BYTES on this quiet
        // thread (cplug_sdk::asm::replace_body), then READY.store(true) + ONE
        // cplug_sdk::retransform_class(NOISE_CHUNK_CLASS).
        let _ = READY.load(Ordering::Relaxed);
    });
}

// ---------------------------------------------------------------------------
// Patch-point probe (existence check only — resolves method IDs and logs)
// ---------------------------------------------------------------------------

/// Probe the candidate methods against the LIVE NoiseChunk class and log
/// which resolve. Never patches anything; pending exceptions are cleared per
/// probe. Also confirms the 2D-cache neighbors are loaded (NoiseRouter /
/// Climate$Sampler).
fn log_candidate_sighting(env: &JniEnv) {
    let Some(cls) = cplug_sdk::classes::find_class(NOISE_CHUNK_CLASS) else {
        let _ = cplug_sdk::jni_util::clear_exception(env);
        eprintln!(
            "[crussty-plugin] noise2d_cache: probe: {NOISE_CHUNK_CLASS} not loaded (worldgen idle?) — candidates unverified"
        );
        return;
    };
    for (name, desc) in CANDIDATE_PATCH_METHODS {
        let hit = env.get_method_id(cls.as_jclass(), name, desc).is_some();
        let _ = cplug_sdk::jni_util::clear_exception(env);
        eprintln!(
            "[crussty-plugin] noise2d_cache: probe: NoiseChunk.{name} -> {}",
            if hit { "RESOLVED" } else { "absent (verify with javap)" }
        );
    }
    for other in [NOISE_ROUTER_CLASS, CLIMATE_SAMPLER_CLASS] {
        let hit = cplug_sdk::classes::find_class(other).is_some();
        eprintln!(
            "[crussty-plugin] noise2d_cache: probe: {other} -> {}",
            if hit { "loaded" } else { "not loaded" }
        );
    }
}

// ---------------------------------------------------------------------------
// Boot gate (copy of proto_blend_cache::wait_for_boot — isolation over DRY)
// ---------------------------------------------------------------------------

/// Wait until the server has finished booting: `org/bukkit/Bukkit`'s static
/// `getServer()` returns a non-null CraftServer, with a short settling delay
/// after that. Returns false on timeout (~120s).
fn wait_for_boot() -> bool {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    loop {
        let booted = cplug_sdk::jni_util::with_attached(|env| {
            let Some(bukkit) = cplug_sdk::classes::find_class("org/bukkit/Bukkit") else {
                return false;
            };
            let Some(get_server) = env.get_static_method_id(
                bukkit.as_jclass(),
                "getServer",
                "()Lorg/bukkit/Server;",
            ) else {
                let _ = cplug_sdk::jni_util::clear_exception(env);
                return false;
            };
            let srv = env.call_static_object_method(bukkit.as_jclass(), get_server, &[]);
            let had_exc = cplug_sdk::jni_util::clear_exception(env);
            if srv.is_null() || had_exc {
                false
            } else {
                env.delete_local_ref(srv);
                true
            }
        });
        if booted.unwrap_or(false) {
            std::thread::sleep(std::time::Duration::from_secs(10));
            return true;
        }
        if std::time::Instant::now() > deadline {
            return false;
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

// ---------------------------------------------------------------------------
// Wiring notes for the main agent:
//
//   mod noise2d_cache;                  // next to mod proto_blend_cache;
//   ...
//   noise2d_cache::register();          // in cplugin_init, after
//                                       // proto_blend_cache::register();
//   ...
//   noise2d_cache::activate();          // at the end of inject_surface, after
//                                       // proto_blend_cache::activate();
//
// Both calls are no-ops unless CRUSSTY_NATIVE_NOISE2D_CACHE is set, so wiring
// the prototype cannot change default behavior (dormant = byte-identical
// classes, hopper-jar rule).
// ---------------------------------------------------------------------------
