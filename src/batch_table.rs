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
//! | 12 | A'    | PaperNativeDensityAp2MinMaxFill                    | oldSummary              | (III[J)I   |
//! | 13 | A'    | PaperNativeDensityAp2MinMaxFill                    | newSummary              | (III[J)I   |
//! | 14 | C     | PaperNativeStaticCacheGet                          | newBatchSummary         | (IIIII[I[J)I |
//! | 15 | D     | PaperNativeRangeChoice                             | optimizedFillArraySummary | ([D[I[I[II[J)I |
//! | 16 | E     | PaperNativeSpigotLoadOrderDependency               | newLoadAfterBuildSummary | (I[Ljava/lang/Object;[J)I |
//! | 17 | F     | PaperNativeSpigotLoadOrderDependency               | newRemovedCountSummary  | (I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I |
//!
//! All 18 are real symbols verified present in `libpaper_native_jni.so`
//! (`JNI_EXPORTS.manifest` / live proof: id 0 is the same kernel the plugin's
//! `live_proof` drives through the bridge). Ids 0-9 cover the dominant
//! `(I[J)I` P500 shape ("scalar + long[] dst, returns count written"); ids
//! 10-11 cover the second real shape `([J[J)J` ("long[] src + long[] dst,
//! returns a jlong result"); ids 12-13 (TASK-48 Phase 1, runbook §8 G3) cover
//! the wave-1 three-scalar shape `(III[J)I` — the P500 g9 density
//! min/max-fill pair (direct 119.8 ns, PROVEN_WINS WIN-grade, not in
//! `DO_NOT_WIRE`). Id 14 is the G3 wave-1 spike completion: the g42
//! `StaticCacheGet` floor anchor (P500 global minimum, 34.6 ns —
//! `P500_REPORT_v2.md` §42) in shape `C` `(IIIII[I[J)I` — five jint scalars,
//! an `int[]` key slice in, `long[]` dst out, return-carried jint result.
//! Ids 15-17 (wire v3, S7-14) complete the wave-1 structural path: the
//! P500 g35/g39/g40 PARITY pairs (direct 81.4-89.0 ns, `baseline.tsv`
//! lines 55/58/59) in the NEW ref-plane shapes `D`/`E`/`F` — their inputs
//! are heterogeneous arrays (`[D`/`[I`/`Object[]`) that cannot ride the
//! long arena, so the batch bridge gained a 7th `refArgs` plane and the
//! descriptor-parser port (`crate::batch_desc`, BATCH_API_PROPOSAL §4/§5)
//! precomputes the per-shape widths.
//! No `(I[J)Z` symbols exist in the table today —
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
/// - [`Shape::APrime`] (A′) — `(III[J)I`:
///   `fn(JNIEnv*, jclass, jint, jint, jint, jlongArray) -> jint`. Three scalar
///   arguments packed into the `args0` scalar plane (see [`Shape::scalar_width`]
///   / `batch_api` packing rule); `argCounts[i]` stays the OUTPUT capacity of
///   the `long[]` dst and the kernel returns the count written — same
///   output contract as [`Shape::A`].
/// - [`Shape::C`] — `(IIIII[I[J)I` (wave-1 g42, G3 spike):
///   `fn(JNIEnv*, jclass, jint, jint, jint, jint, jint, jintArray, jlongArray)
///   -> jint`. Five packed jint scalars from the `args0` scalar plane (width
///   [`Shape::scalar_width`] = 5, one long slot per scalar, narrowed to the
///   low 32 bits — the BATCH_API_PROPOSAL §5 scalar encoding), `argCounts[i]`
///   int slots from the SAME packed `args1` prefix-sum stream (one int per
///   long slot, narrowed) copied into a per-thread `int[]` scratch, `long[]`
///   dst = the shared 64-long scratch. The jint return value is stored as the
///   single result long for the op (return-carried, like shape B; the closed
///   g42 kernel carries its result in the return and leaves dst untouched —
///   probe-verified 2026-09-09 — so dst contents are NOT propagated).
/// - [`Shape::D`] — `([D[I[I[II[J)I` (wire v3, P500 g35):
///   `fn(JNIEnv*, jclass, jdoubleArray, jintArray, jintArray, jintArray, jint,
///   jlongArray) -> jint`. Four INPUT refs (one `double[]` + three `int[]`)
///   ride the NEW `refArgs` plane (heterogeneous arrays cannot ride the long
///   arena); the single packed jint scalar narrows from the `args0` scalar
///   plane; `argCounts[i]` stays the OUTPUT capacity and the kernel returns
///   the count written — same output contract as [`Shape::A`].
/// - [`Shape::E`] — `(I[Ljava/lang/Object;[J)I` (wire v3, P500 g39):
///   `fn(JNIEnv*, jclass, jint, jobjectArray, jlongArray) -> jint`. One jint
///   scalar + one `Object[]` input ref on the ref plane; A-style output
///   contract (count written).
/// - [`Shape::F`] — `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I`
///   (wire v3, P500 g40):
///   `fn(JNIEnv*, jclass, jint, jobjectArray, jobjectArray, jobjectArray,
///   jint, jlongArray) -> jint`. Two jint scalars + three `Object[]` input
///   refs on the ref plane; A-style output contract (count written).
/// - [`Shape::Z`] — reserved: `(I[J)Z` (`-> jboolean`). No such symbol exists
///   in `MAIN_JNI_TABLE` yet; listed so the dispatcher contract is complete.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    /// `(I[J)I` — scalar in, `long[]` dst out, returns count written.
    A,
    /// `([J[J)J` — `long[]` src in, `long[]` dst out, returns jlong result.
    B,
    /// `(III[J)I` — three scalars in, `long[]` dst out, returns count written
    /// (wave-1 shape, TASK-48: P500 g9 `DensityAp2MinMaxFill` pair).
    APrime,
    /// `(IIIII[I[J)I` — five packed scalars + `int[]` in, `long[]` dst out,
    /// return-carried jint result (g42 wave-1 kernel, G3 spike).
    C,
    /// `([D[I[I[II[J)I` — four ref-plane inputs (double[] + int[]×3) + one
    /// jint scalar, `long[]` dst out, returns count written (wire v3: P500
    /// g35 `RangeChoice.optimizedFillArraySummary`, 81.6 ns direct).
    D,
    /// `(I[Ljava/lang/Object;[J)I` — one jint scalar + one Object[] ref-plane
    /// input, `long[]` dst out, returns count written (wire v3: P500 g39
    /// `SpigotLoadOrderDependency.newLoadAfterBuildSummary`, 88.0 ns direct).
    E,
    /// `(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I` —
    /// two jint scalars + three Object[] ref-plane inputs, `long[]` dst out,
    /// returns count written (wire v3: P500 g40
    /// `SpigotLoadOrderDependency.newRemovedCountSummary`, 88.6 ns direct).
    F,
    /// `(I[J)Z` — reserved (no real symbols yet); boolean result stored as 0/1.
    /// Kept so kernels of this shape can be added without an ABI break.
    #[allow(dead_code)]
    Z,
}

impl Shape {
    /// JNI method descriptor this shape implies (used for cross-checking
    /// against the copied `sig` strings at init).
    pub const fn sig(self) -> &'static str {
        match self {
            Shape::A => "(I[J)I",
            Shape::B => "([J[J)J",
            Shape::APrime => "(III[J)I",
            Shape::C => "(IIIII[I[J)I",
            Shape::D => "([D[I[I[II[J)I",
            Shape::E => "(I[Ljava/lang/Object;[J)I",
            Shape::F => "(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I",
            Shape::Z => "(I[J)Z",
        }
    }

    /// Number of `args0` scalar-plane longs this shape consumes per op
    /// (TASK-48 wire rule: the scalar plane is shape-packed by prefix sums —
    /// op *i* owns `scalar_width(shape_i)` consecutive longs starting at
    /// `scalar_starts[i]`; shape B consumes none). The Java caller derives
    /// the identical layout from `kernelIds` before the call, so the packing
    /// is deterministic and needs no extra control plane.
    pub const fn scalar_width(self) -> usize {
        match self {
            Shape::A | Shape::Z => 1,
            Shape::APrime => 3,
            Shape::C => 5,
            Shape::D | Shape::E => 1,
            Shape::F => 2,
            Shape::B => 0,
        }
    }

    /// INPUT ref-plane slots this shape consumes per op, EXCLUDING the dst
    /// (wire v3 rule): op *i* owns `refs(shape_i)` consecutive `refArgs`
    /// slots starting at the prefix sum `ref_starts[i]` — the same
    /// shape-packed arithmetic as the scalar plane, derived identically on
    /// the Java side from `kernelIds`.
    ///
    /// ONLY the new D/E/F shapes carry input refs on the ref plane (D = 4:
    /// one `double[]` + three `int[]`; E = 1: one `Object[]`; F = 3: three
    /// `Object[]`) — their inputs are heterogeneous arrays that CANNOT ride
    /// the packed long arena. Shapes A/Z/A′ take no input refs (their only
    /// ref parameter is the trailing `[J` dst = the dispatcher's shared
    /// scratch), so they own 0 ref slots. Shapes B/C keep their packed
    /// inputs on the `args1` arena (v2 back-compat of their layout — B's
    /// `long[]` src and C's `int[]` keys ride `args1` exactly as wire v2;
    /// `crate::batch_desc::input_refs` reports 1 each, matching the
    /// descriptor's input-ref count minus the trailing dst) and therefore
    /// own 0 ref-plane slots. Elements are handed to kernels ZERO-COPY
    /// (BATCH_API_PROPOSAL §5) via `GetObjectArrayElement` local refs,
    /// deleted after the call.
    pub const fn refs(self) -> usize {
        match self {
            Shape::D => 4,
            Shape::E => 1,
            Shape::F => 3,
            Shape::A | Shape::Z | Shape::APrime | Shape::B | Shape::C => 0,
        }
    }

    /// The descriptor-truth INPUT-ref count the fail-closed cross-check
    /// (`batch_api::resolve_fns` + the table tests) expects
    /// `crate::batch_desc::input_refs(parse_sig(sig))` to produce. This is
    /// [`Shape::refs`] for the ref-plane-native shapes; for the args1/dst
    /// shapes it is the descriptor-derived count (B/C: 1 packed input ref;
    /// A/Z/A′: 0 — trailing `[J` dst only). Split from [`Shape::refs`]
    /// because the WIRE ref-plane width (0 for B/C) and the DESCRIPTOR
    /// input-ref count (1 for B/C) intentionally differ — the wire rule is
    /// documented on [`Shape::refs`].
    pub const fn expected_input_refs(self) -> usize {
        match self {
            Shape::D | Shape::E | Shape::F => self.refs(),
            Shape::B | Shape::C => 1,
            Shape::A | Shape::Z | Shape::APrime => 0,
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
/// v2 (TASK-48): `args0` became the shape-packed scalar plane
/// ([`Shape::scalar_width`]; v1 = one long per op, shape-A only). Ids 0-11
/// wire behavior is byte-identical to v1 — the bump exists so a v1 caller
/// that is unaware of the packing rule cannot accidentally feed a batch
/// containing A′ ids (it would fail the id range check on v1 anyway), and so
/// the version alone signals "read the packing docs". Id 14 (G3 shape C)
/// rides the same v2 packing.
/// v3 (S7-14, wave-1 descriptor-parser port — BATCH_API_PROPOSAL §4/§5):
/// the batch bridge gains a 7th `refArgs` argument (the reference plane,
/// `Shape::refs` slots per D/E/F op) and 3 new kernels enter the table
/// (ids 15/16/17, shapes D/E/F). History: v1 = per-op long scalar,
/// v2 = shape-packed scalar plane, v3 = ref plane + 18 kernels.
/// v4 (TASK-61, old-member wiring): the OLD members of the three wave-1
/// parity pairs enter the table (ids 18/19/20, same shapes D/E/F, same
/// descriptors) so the dispatcher can express BOTH legs of each pair —
/// parity-through-dispatcher (OldMemberParityProbe) closes the evidence
/// loop against the P500 anchors. Calling convention unchanged; the bump
/// signals the kernel-count change to embeds (EXPECTED_ABI gate).
pub const TABLE_VERSION: u32 = 4;

/// The compile-time kernel table (21 real symbols, `jni_table.rs` line noted).
pub const KERNELS: [BatchKernel; 21] = [
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
    // jni_table.rs:173 — TASK-48 wave-1 shape A′ (P500 g9, direct 119.8 ns).
    BatchKernel {
        id: 12,
        shape: Shape::APrime,
        class: "PaperNativeDensityAp2MinMaxFill",
        method: "oldSummary",
        sig: "(III[J)I",
        symbol: "Java_PaperNativeDensityAp2MinMaxFill_oldSummary",
    },
    // jni_table.rs:174 — the proven-WIN partner kernel (313.6x is g21; g9
    // pair is WIN-grade at 119.8 ns — see PROVEN_WINS_SYNC / TASK-31 sync).
    BatchKernel {
        id: 13,
        shape: Shape::APrime,
        class: "PaperNativeDensityAp2MinMaxFill",
        method: "newSummary",
        sig: "(III[J)I",
        symbol: "Java_PaperNativeDensityAp2MinMaxFill_newSummary",
    },
    // jni_table.rs:246 — G3 wave-1 spike completion: g42 StaticCacheGet floor
    // anchor (P500 global minimum 34.6 ns, P500_REPORT_v2 §42), shape C
    // (IIIII[I[J)I. newBatchSummary (the alt/optimized member of the parity
    // pair, 0.997 vs old); oldBatchSummary stays unwired until a wave needs it.
    BatchKernel {
        id: 14,
        shape: Shape::C,
        class: "PaperNativeStaticCacheGet",
        method: "newBatchSummary",
        sig: "(IIIII[I[J)I",
        symbol: "Java_PaperNativeStaticCacheGet_newBatchSummary",
    },
    // jni_table.rs:79 — wire v3 wave-1 (S7-14): g35 optimized/alt member of
    // the parity pair (old/optimized ratio 1.0025, baseline.tsv:55 — PARITY
    // grade; direct 81.4-81.6 ns, p500_expected_summary.tsv §35), shape D.
    // Symbol verified in the .so exports manifest + nm.
    BatchKernel {
        id: 15,
        shape: Shape::D,
        class: "PaperNativeRangeChoice",
        method: "optimizedFillArraySummary",
        sig: "([D[I[I[II[J)I",
        symbol: "Java_PaperNativeRangeChoice_optimizedFillArraySummary",
    },
    // jni_table.rs:210 — wire v3 wave-1 (S7-14): g39 alt member of the parity
    // pair (old/new ratio 1.0034, baseline.tsv:58 — PARITY grade; direct
    // 87.7-88.0 ns, p500_expected_summary.tsv §39), shape E.
    BatchKernel {
        id: 16,
        shape: Shape::E,
        class: "PaperNativeSpigotLoadOrderDependency",
        method: "newLoadAfterBuildSummary",
        sig: "(I[Ljava/lang/Object;[J)I",
        symbol: "Java_PaperNativeSpigotLoadOrderDependency_newLoadAfterBuildSummary",
    },
    // jni_table.rs:212 — wire v3 wave-1 (S7-14): g40 alt member of the parity
    // pair (old/new ratio 0.9955, baseline.tsv:59 — PARITY grade; direct
    // 88.6-89.0 ns, p500_expected_summary.tsv §40), shape F.
    BatchKernel {
        id: 17,
        shape: Shape::F,
        class: "PaperNativeSpigotLoadOrderDependency",
        method: "newRemovedCountSummary",
        sig: "(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I",
        symbol: "Java_PaperNativeSpigotLoadOrderDependency_newRemovedCountSummary",
    },
    // jni_table.rs:79 — TASK-61 old-member wiring: g35 OLD member (P500
    // anchor 81.4 ns, baseline.json:22; pair old/optimized PARITY ratio
    // 1.0025, baseline.tsv:55), shape D, same descriptor as id 15. Wired so
    // the dispatcher can express BOTH legs of the pair (parity-through-
    // dispatcher); NOT a promotion candidate (batch loses on D/E/F floors,
    // S7-14 floor numbers).
    BatchKernel {
        id: 18,
        shape: Shape::D,
        class: "PaperNativeRangeChoice",
        method: "oldFillArraySummary",
        sig: "([D[I[I[II[J)I",
        symbol: "Java_PaperNativeRangeChoice_oldFillArraySummary",
    },
    // jni_table.rs:211 — TASK-61 old-member wiring: g39 OLD member (pair
    // old/new PARITY ratio 1.0034, baseline.tsv:58), shape E, same
    // descriptor as id 16.
    BatchKernel {
        id: 19,
        shape: Shape::E,
        class: "PaperNativeSpigotLoadOrderDependency",
        method: "oldLoadAfterBuildSummary",
        sig: "(I[Ljava/lang/Object;[J)I",
        symbol: "Java_PaperNativeSpigotLoadOrderDependency_oldLoadAfterBuildSummary",
    },
    // jni_table.rs:213 — TASK-61 old-member wiring: g40 OLD member (pair
    // old/new PARITY ratio 0.9955, baseline.tsv:59), shape F, same
    // descriptor as id 17.
    BatchKernel {
        id: 20,
        shape: Shape::F,
        class: "PaperNativeSpigotLoadOrderDependency",
        method: "oldRemovedCountSummary",
        sig: "(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I",
        symbol: "Java_PaperNativeSpigotLoadOrderDependency_oldRemovedCountSummary",
    },
];

/// Slice view of the compile-time table (same shape as `MAIN_JNI_TABLE`).
pub static BATCH_KERNELS: &[BatchKernel] = &KERNELS;

/// Number of batch kernels (const — used for the fixed-size fn-pointer table).
pub const KERNEL_COUNT: usize = KERNELS.len();

/// Compile-time lookup: `kernel_by_id(i) == Some(&KERNELS[i])` iff `i` is in
/// range. Const so callers can embed id validity in `const` assertions.
#[allow(dead_code)] // public table API; dispatcher indexes KERNELS directly
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

#[cfg(test)]
mod tests {
    use super::*;

    /// G3 spike completion: the g42 wave-1 kernel rides the new shape C,
    /// declared exactly like every other entry (copied `JniEntry` tuple +
    /// shape) at table id 14 (after TASK-48's A′ pair 12/13).
    #[test]
    fn g42_wave1_kernel_is_declared_as_shape_c() {
        let k = kernel_by_id(14).expect("g42 kernel id 14 must exist");
        assert_eq!(k.shape, Shape::C);
        assert_eq!(k.class, "PaperNativeStaticCacheGet");
        assert_eq!(k.method, "newBatchSummary");
        assert_eq!(k.sig, "(IIIII[I[J)I");
        assert_eq!(k.symbol, "Java_PaperNativeStaticCacheGet_newBatchSummary");
        // Descriptor cross-check (the init-time resolve_fns rule, mirrored).
        assert_eq!(k.sig, k.shape.sig());
    }

    /// Per-shape scalar-plane widths (the TASK-48 v2 packing rule): A/Z keep
    /// the historical 1 long/op, B consumes none, A′ packs three, C packs the
    /// five jint scalars of `(IIIII[I[J)I`; wire v3 adds D/E (1 jint each)
    /// and F (2 jints).
    #[test]
    fn shape_scalar_widths() {
        assert_eq!(Shape::A.scalar_width(), 1);
        assert_eq!(Shape::B.scalar_width(), 0);
        assert_eq!(Shape::APrime.scalar_width(), 3);
        assert_eq!(Shape::C.scalar_width(), 5);
        assert_eq!(Shape::D.scalar_width(), 1);
        assert_eq!(Shape::E.scalar_width(), 1);
        assert_eq!(Shape::F.scalar_width(), 2);
        assert_eq!(Shape::Z.scalar_width(), 1);
    }

    /// Runtime mirror of the compile-time table asserts: ids stay dense and
    /// every descriptor matches its declared shape (21 entries after the
    /// TASK-61 old-member wiring).
    #[test]
    fn table_is_dense_with_matching_descriptors() {
        assert_eq!(KERNEL_COUNT, 21);
        for (i, k) in BATCH_KERNELS.iter().enumerate() {
            assert_eq!(k.id as usize, i);
            assert_eq!(k.sig, k.shape.sig());
        }
    }

    /// Wire-v3 ref-plane rule ([`Shape::refs`]): only the new D/E/F shapes
    /// own input-ref slots (D = 4, E = 1, F = 3); every args1/dst shape owns
    /// zero (B's `[J` src and C's `int[]` keys stay on the packed args1
    /// arena for v2 back-compat of their layout).
    #[test]
    fn shape_ref_plane_widths() {
        assert_eq!(Shape::A.refs(), 0);
        assert_eq!(Shape::B.refs(), 0);
        assert_eq!(Shape::APrime.refs(), 0);
        assert_eq!(Shape::C.refs(), 0);
        assert_eq!(Shape::D.refs(), 4);
        assert_eq!(Shape::E.refs(), 1);
        assert_eq!(Shape::F.refs(), 3);
        assert_eq!(Shape::Z.refs(), 0);
    }

    /// Wire-v3 descriptor-parser cross-check (BATCH_API_PROPOSAL §4: the
    /// widths are "precomputed by porting the descriptor parser ... unit-
    /// tested against all 49 group signatures"): every table sig must parse
    /// to exactly its shape's declared widths. Mirrors the fail-closed
    /// check wired into `batch_api::resolve_fns` — the parser is
    /// load-bearing, not decorative. The 49-sig parser coverage lives in
    /// `batch_desc::tests`.
    #[test]
    fn every_kernel_sig_cross_checks_against_batch_desc() {
        for k in BATCH_KERNELS {
            let d = crate::batch_desc::parse_sig(k.sig)
                .unwrap_or_else(|e| panic!("kernel {} sig {:?}: {}", k.id, k.sig, e));
            let s = crate::batch_desc::slots(&d);
            assert_eq!(
                s.scalars,
                k.shape.scalar_width(),
                "scalar width drift for kernel {} ({})",
                k.id,
                k.sig
            );
            assert_eq!(
                crate::batch_desc::input_refs(&d),
                k.shape.expected_input_refs(),
                "input-ref drift for kernel {} ({})",
                k.id,
                k.sig
            );
        }
    }

    /// Wire-v3 wave-1 pins: ids 15/16/17 carry the g35/g39/g40 OPTIMIZED
    /// members (g42 precedent: only the alt/optimized member is wired),
    /// declared exactly like every other entry (copied `JniEntry` tuple +
    /// shape).
    #[test]
    fn wave1_kernels_are_declared_as_shapes_d_e_f() {
        let g35 = kernel_by_id(15).expect("g35 kernel id 15 must exist");
        assert_eq!(g35.shape, Shape::D);
        assert_eq!(g35.class, "PaperNativeRangeChoice");
        assert_eq!(g35.method, "optimizedFillArraySummary");
        assert_eq!(g35.sig, "([D[I[I[II[J)I");
        assert_eq!(g35.symbol, "Java_PaperNativeRangeChoice_optimizedFillArraySummary");
        assert_eq!(g35.sig, g35.shape.sig());

        let g39 = kernel_by_id(16).expect("g39 kernel id 16 must exist");
        assert_eq!(g39.shape, Shape::E);
        assert_eq!(g39.class, "PaperNativeSpigotLoadOrderDependency");
        assert_eq!(g39.method, "newLoadAfterBuildSummary");
        assert_eq!(g39.sig, "(I[Ljava/lang/Object;[J)I");
        assert_eq!(
            g39.symbol,
            "Java_PaperNativeSpigotLoadOrderDependency_newLoadAfterBuildSummary"
        );
        assert_eq!(g39.sig, g39.shape.sig());

        let g40 = kernel_by_id(17).expect("g40 kernel id 17 must exist");
        assert_eq!(g40.shape, Shape::F);
        assert_eq!(g40.class, "PaperNativeSpigotLoadOrderDependency");
        assert_eq!(g40.method, "newRemovedCountSummary");
        assert_eq!(
            g40.sig,
            "(I[Ljava/lang/Object;[Ljava/lang/Object;[Ljava/lang/Object;I[J)I"
        );
        assert_eq!(
            g40.symbol,
            "Java_PaperNativeSpigotLoadOrderDependency_newRemovedCountSummary"
        );
        assert_eq!(g40.sig, g40.shape.sig());
    }
}
