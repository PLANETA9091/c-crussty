//! Batch kernel table — id → symbol mapping for the batch-dispatch bridge
//! (`batch_api`).
//!
//! This file is SELF-CONTAINED by design: every entry below is a verbatim copy
//! of a `JniEntry` from `src/jni_table.rs` (source line noted per entry). The
//! main agent must NOT be required to edit `jni_table.rs` for these kernels —
//! they are already injected as regular bridge natives; this table only
//! re-names them so the batch dispatcher can call them in a loop from Rust.
//!
//! # Kernel ids
//!
//! | id | shape | bridge class (internal name)                       | method                  | signature  |
//! |----|-------|----------------------------------------------------|-------------------------|------------|
//! | 0  | A     | PaperNativeTicketSetSearch                         | binarySummary           | (I[J)I     |
//! | 1  | A     | PaperNativeTicketSetSearch                         | uncheckedBinarySummary  | (I[J)I     |
//! | 2  | A     | PaperNativeAquiferIndexStride                      | oldBatchSummary         | (I[J)I     |
//! | 3  | A     | PaperNativeAquiferIndexStride                      | newBatchSummary         | (I[J)I     |
//! | 4  | A     | PaperNativeChunkDependencies                       | oldImmutableListSummary | (I[J)I     |
//! | 5  | A     | PaperNativeChunkDependencies                       | arraySummary            | (I[J)I     |
//! | 6  | A     | PaperNativeDensitySplineContext                    | oldWrapperSummary       | (I[J)I     |
//! | 7  | A     | PaperNativeDensitySplineContext                    | newDirectSummary        | (I[J)I     |
//! | 8  | A     | PaperNativeEntityLookupStatus                      | oldStatusSummary        | (I[J)I     |
//! | 9  | A     | PaperNativeNoiseInterpolatorFractions              | divisionSummary         | (I[J)I     |
//! | 10 | B     | PaperNativeClimateRTree                            | buildTreeHandle         | ([J[J)J    |
//! | 11 | B     | net/minecraft/world/level/biome/PaperNativeClimateRTree | nativeBuildTreeHandle | ([J[J)J  |
//!
//! All 12 are real symbols verified present in `libpaper_native_jni.so`
//! (`JNI_EXPORTS.manifest` / live proof: id 0 is the same kernel the plugin's
//! `live_proof` drives through the bridge). Ids 0-9 cover the dominant
//! `(I[J)I` P500 shape ("scalar + long[] dst, returns count written"); ids
//! 10-11 cover the second real shape `([J[J)J` ("long[] src + long[] dst,
//! returns a jlong result"). No `(I[J)Z` symbols exist in the table today —
//! shape `Z` is reserved in [`Shape`] so such kernels can be added without an
//! ABI break of the batch entry point.
//!
//! # Batch bridge class (for the main agent)
//!
//! The batch entry point lives in OUR cdylib (export
//! `Java_crussty_batch_PaperNativeBatchDispatch_run`), NOT in
//! `libpaper_native_jni.so`. Therefore the generic `inject_surface` loop —
//! which resolves every `MAIN_JNI_TABLE` symbol from the closed `.so` — must
//! NOT be handed this entry as-is (it would count the symbol as unresolved).
//! `batch_api::init(env, &main)` defines the bridge class and registers the
//! export itself.
//!
//! If the main agent prefers to funnel the class through the normal table
//! anyway, this is the exact `JniEntry`:
//!
//! ```ignore
//! // --- append to MAIN_JNI_TABLE (requires the lib.rs wiring note below) ---
//! JniEntry {
//!     class: "crussty/batch/PaperNativeBatchDispatch",
//!     method: "run",
//!     sig: "([I[J[J[I[J[I)I",
//!     symbol: "Java_crussty_batch_PaperNativeBatchDispatch_run",
//! },
//! ```
//!
//! ...plus a resolution fallback in `lib.rs::inject_surface` for entries whose
//! symbol is exported by the plugin itself (e.g. try
//! `dlsym(RTLD_DEFAULT, symbol)` when `lib.symbol()` fails, or register
//! `batch_api::run_export as *mut c_void` directly). The dedicated
//! `batch_api::init` path does all of this in one call and is the recommended
//! wiring (see `reports/B3_batch_api_impl.md`).
//!
//! # Package choice
//!
//! The batch bridge class lives in the dedicated package `crussty/batch`
//! (bootstrap loader, defined via `bridge_class::bridge_class_bytes`). The
//! existing patterns are: default package for stems whose JNI symbol implies
//! it (`Java_PaperNativeTicketSetSearch_*`) and `net/minecraft/**` where the
//! export name implies it. A batch dispatcher belongs to neither stem family,
//! so a dedicated package avoids any collision with kernel classes and makes
//! the entry point obvious. `bridge_class_bytes` receives the full internal
//! name `crussty/batch/PaperNativeBatchDispatch` and emits it into the
//! CONSTANT_Class entry — packages work unchanged (same mechanism as
//! `net/minecraft/world/level/levelgen/synth/PaperNativeNormalNoise`).

/// Argument/result shape of a batch kernel.
///
/// Every shape below maps to one concrete C calling convention (the JNI
/// function signature of the closed `.so` export) and one batch marshalling
/// rule (documented in `batch_api`):
///
/// - [`Shape::A`] — `(I[J)I`: `fn(JNIEnv*, jclass, jint, jlongArray) -> jint`.
///   Scalar argument narrowed from `args0[i]`; the kernel writes up to
///   `argCounts[i]` longs and returns the count written.
/// - [`Shape::B`] — `([J[J)J`: `fn(JNIEnv*, jclass, jlongArray, jlongArray) -> jlong`.
///   Reads `argCounts[i]` longs from the packed `args1` input arena, writes
///   scratch output, and its `jlong` return value is stored as the single
///   result long for the op.
/// - [`Shape::Z`] — reserved: `(I[J)Z` (`-> jboolean`). No such symbol exists
///   in `MAIN_JNI_TABLE` yet; listed so the dispatcher contract is complete.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    /// `(I[J)I` — scalar in, `long[]` dst out, returns count written.
    A,
    /// `([J[J)J` — `long[]` src in, `long[]` dst out, returns jlong result.
    B,
    /// `(I[J)Z` — reserved (no real symbols yet); boolean result stored as 0/1.
    Z,
}

impl Shape {
    /// JNI method descriptor this shape implies (used for cross-checking
    /// against the copied `sig` strings at init).
    pub const fn sig(self) -> &'static str {
        match self {
            Shape::A => "(I[J)I",
            Shape::B => "([J[J)J",
            Shape::Z => "(I[J)Z",
        }
    }
}

/// One batch-dispatchable kernel: a copied `(class, method, sig, symbol)`
/// tuple from `jni_table.rs` plus its shape.
#[derive(Clone, Copy, Debug)]
pub struct BatchKernel {
    /// Stable kernel id (== index in [`KERNELS`]); wire format of
    /// `kernelIds[i]`.
    pub id: u32,
    /// Calling-convention shape.
    pub shape: Shape,
    /// Bridge class internal name (already injected by the normal surface).
    pub class: &'static str,
    /// Bridge method name (already registered as a native).
    pub method: &'static str,
    /// JNI descriptor (must match [`Shape::sig`] of `shape`).
    pub sig: &'static str,
    /// dlsym name in `libpaper_native_jni.so`.
    pub symbol: &'static str,
}

/// ABI version of the batch bridge (A7 §4.5 guard). Bump on any calling-
/// convention change; `abiVersion()` returns
/// `(TABLE_VERSION << 16) | KERNEL_COUNT` and stale callers fall back to
/// per-op calls on mismatch.
pub const TABLE_VERSION: u32 = 1;

/// The compile-time kernel table (12 real symbols, `jni_table.rs` line noted).
pub const KERNELS: [BatchKernel; 12] = [
    // jni_table.rs:15 — the live-proof kernel (ticketset binary search).
    BatchKernel {
        id: 0,
        shape: Shape::A,
        class: "PaperNativeTicketSetSearch",
        method: "binarySummary",
        sig: "(I[J)I",
        symbol: "Java_PaperNativeTicketSetSearch_binarySummary",
    },
    // jni_table.rs:16
    BatchKernel {
        id: 1,
        shape: Shape::A,
        class: "PaperNativeTicketSetSearch",
        method: "uncheckedBinarySummary",
        sig: "(I[J)I",
        symbol: "Java_PaperNativeTicketSetSearch_uncheckedBinarySummary",
    },
    // jni_table.rs:128 — aquifer index stride (P500: 1.15x win stem family).
    BatchKernel {
        id: 2,
        shape: Shape::A,
        class: "PaperNativeAquiferIndexStride",
        method: "oldBatchSummary",
        sig: "(I[J)I",
        symbol: "Java_PaperNativeAquiferIndexStride_oldBatchSummary",
    },
    // jni_table.rs:129
    BatchKernel {
        id: 3,
        shape: Shape::A,
        class: "PaperNativeAquiferIndexStride",
        method: "newBatchSummary",
        sig: "(I[J)I",
        symbol: "Java_PaperNativeAquiferIndexStride_newBatchSummary",
    },
    // jni_table.rs:175 — chunk dependencies.
    BatchKernel {
        id: 4,
        shape: Shape::A,
        class: "PaperNativeChunkDependencies",
        method: "oldImmutableListSummary",
        sig: "(I[J)I",
        symbol: "Java_PaperNativeChunkDependencies_oldImmutableListSummary",
    },
    // jni_table.rs:176
    BatchKernel {
        id: 5,
        shape: Shape::A,
        class: "PaperNativeChunkDependencies",
        method: "arraySummary",
        sig: "(I[J)I",
        symbol: "Java_PaperNativeChunkDependencies_arraySummary",
    },
    // jni_table.rs:30 — density spline context (P500 win: newDirect 3.29x stem).
    BatchKernel {
        id: 6,
        shape: Shape::A,
        class: "PaperNativeDensitySplineContext",
        method: "oldWrapperSummary",
        sig: "(I[J)I",
        symbol: "Java_PaperNativeDensitySplineContext_oldWrapperSummary",
    },
    // jni_table.rs:31
    BatchKernel {
        id: 7,
        shape: Shape::A,
        class: "PaperNativeDensitySplineContext",
        method: "newDirectSummary",
        sig: "(I[J)I",
        symbol: "Java_PaperNativeDensitySplineContext_newDirectSummary",
    },
    // jni_table.rs:242 — entity lookup status.
    BatchKernel {
        id: 8,
        shape: Shape::A,
        class: "PaperNativeEntityLookupStatus",
        method: "oldStatusSummary",
        sig: "(I[J)I",
        symbol: "Java_PaperNativeEntityLookupStatus_oldStatusSummary",
    },
    // jni_table.rs:155 — noise interpolator fractions.
    BatchKernel {
        id: 9,
        shape: Shape::A,
        class: "PaperNativeNoiseInterpolatorFractions",
        method: "divisionSummary",
        sig: "(I[J)I",
        symbol: "Java_PaperNativeNoiseInterpolatorFractions_divisionSummary",
    },
    // jni_table.rs:89 — climate R-tree handle builder, shape B ([J[J)J).
    BatchKernel {
        id: 10,
        shape: Shape::B,
        class: "PaperNativeClimateRTree",
        method: "buildTreeHandle",
        sig: "([J[J)J",
        symbol: "Java_PaperNativeClimateRTree_buildTreeHandle",
    },
    // jni_table.rs:92 — the net.minecraft-packaged variant of the same stem.
    BatchKernel {
        id: 11,
        shape: Shape::B,
        class: "net/minecraft/world/level/biome/PaperNativeClimateRTree",
        method: "nativeBuildTreeHandle",
        sig: "([J[J)J",
        symbol: "Java_net_minecraft_world_level_biome_PaperNativeClimateRTree_nativeBuildTreeHandle",
    },
];

/// Slice view of the compile-time table (same shape as `MAIN_JNI_TABLE`).
pub static BATCH_KERNELS: &[BatchKernel] = &KERNELS;

/// Number of batch kernels (const — used for the fixed-size fn-pointer table).
pub const KERNEL_COUNT: usize = KERNELS.len();

/// Compile-time lookup: `kernel_by_id(i) == Some(&KERNELS[i])` iff `i` is in
/// range. Const so callers can embed id validity in `const` assertions.
pub const fn kernel_by_id(id: usize) -> Option<&'static BatchKernel> {
    if id < KERNEL_COUNT {
        Some(&KERNELS[id])
    } else {
        None
    }
}

// Compile-time sanity: ids are dense (0..KERNEL_COUNT) and every entry's
// descriptor matches its declared shape. Both are needed for the dispatcher's
// direct `KERNELS[id]` indexing and for the fn-pointer cast at init.
const _: () = {
    let mut i = 0;
    while i < KERNEL_COUNT {
        assert!(KERNELS[i].id as usize == i, "kernel ids must be dense");
        assert!(
            KERNELS[i].sig.len() == KERNELS[i].shape.sig().len()
                && str_eq(KERNELS[i].sig, KERNELS[i].shape.sig()),
            "kernel sig must match its shape"
        );
        i += 1;
    }
};

/// Const byte-wise string equality (const fn `str` comparison is limited).
const fn str_eq(a: &str, b: &str) -> bool {
    let (ab, bb) = (a.as_bytes(), b.as_bytes());
    if ab.len() != bb.len() {
        return false;
    }
    let mut i = 0;
    while i < ab.len() {
        if ab[i] != bb[i] {
            return false;
        }
        i += 1;
    }
    true
}
