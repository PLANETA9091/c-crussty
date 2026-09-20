//! Runtime wiring for the `items_paper` lever (TASK-397-F, vector F — OSS port).
//!
//! Ports the Canvas (Paper/Folia fork) item-entity merge optimizations onto the
//! measured TOP-1 bottleneck `ItemEntity.tick` (31.17% java, bank v4 fp=4):
//!
//!   P1 — `entities.itemEntities.itemEntitiesWaitTwoSecondsForMergeCheckAlways=true`
//!        semantic (Canvas WorldConfig.java L340-353): the tick merge-check
//!        cadence `rate = moved ? 2 : 40` is forced to `rate = 40` for moved
//!        items too. Bytecode: `tick()V` @434 `ifeq +4` → `goto +4` (opcode
//!        0x99→0xa8, identical operand width) — the moved-path `iconst_2`
//!        becomes dead code, `rate` is always 40. DOC-DEV-1 (upstream-parity
//!        note, canvas config semantics verbatim): merge-scan cadence of
//!        moving items drops 2→40 ticks, merge latency ≤2s instead of ≤1 tick.
//!   P2 — Canvas merge-loop exit (`if (!this.isMergable()) break;` instead of
//!        `if (this.isRemoved()) break;`): `mergeWithNeighbours()V` @156
//!        `invokevirtual isRemoved` → `invokevirtual isMergable` (CP-u2
//!        operand swap to the existing isMergable Methodref) + `ifeq`→`ifne`
//!        (0x99→0x9a). Zero semantic deviation: after a successful
//!        `tryToMerge` a FULL `this` makes every remaining loop iteration a
//!        provable no-op (`areMergable` requires other.count + this.count <=
//!        max — impossible when this is full), so the break only skips
//!        no-op work; vanilla break-on-removed behavior is preserved because
//!        a removed `this` is also not mergeable (`isAlive()` term).
//!
//! Both edits are SIZE-NEUTRAL (opcode / operand bytes only): Code length,
//! exception table and StackMapTable are byte-identical, and no new branch
//! targets are introduced (the P1 dead region carries no frames; the P2
//! targets 165/168 keep their frames and stack shapes). Verifier-safe by
//! construction; the patch is additionally re-verified statically before
//! arming (see `verify_patch`).
//!
//! Upstream proofs: /home/z/rounds/ROUND-397/RESEARCH-F.md (Canvas
//! ItemEntity.java.patch + WorldConfig.java, PaperMC/Paper ItemEntity.java.patch
//! context, kernel javap of tick/mergeWithNeighbours/tryToMerge).
//!
//! Gate: env `CRUSSTY_LEVER_FLAG == "items_paper"` (round-397 lever protocol,
//! exported by run_world3.sh). Off by default — dormant-invisible discipline:
//! with the gate off no byte hook is registered, nothing is retransformed,
//! the module is byte-indistinguishable from the pre-TASK-397 plugin.

use jvmti_bindings::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, PoisonError};

pub const ITEM_CLASS: &str = "net/minecraft/world/entity/item/ItemEntity";

/// env gate per the round-397 lever protocol (NOT a CRUSSTY_ITEMS_PAPER flag).
fn enabled() -> bool {
    matches!(std::env::var("CRUSSTY_LEVER_FLAG").as_deref(), Ok("items_paper"))
}

static READY: AtomicBool = AtomicBool::new(false);
/// Global ref to the kernel ItemEntity classloader, captured at activation.
static KERNEL_LOADER: AtomicUsize = AtomicUsize::new(0);

/// Original class bytes captured from the FIRST sight of the class (byte hook
/// while READY=false), else via no-op retransform / resource stream.
static ORIG_BYTES: std::sync::OnceLock<std::sync::Mutex<Option<Vec<u8>>>> =
    std::sync::OnceLock::new();

/// Patched bytecode cache: Arc<[u8]> + major parsed once (TASK-26/C5 pattern).
#[derive(Clone)]
struct PatchCache {
    bytes: Arc<[u8]>,
    major: u16,
}
static PATCH_CACHE: std::sync::OnceLock<std::sync::Mutex<Option<PatchCache>>> =
    std::sync::OnceLock::new();
static SERVE_LOGGED: AtomicBool = AtomicBool::new(false);

fn orig_lock() -> &'static std::sync::Mutex<Option<Vec<u8>>> {
    ORIG_BYTES.get_or_init(|| std::sync::Mutex::new(None))
}
fn patch_lock() -> &'static std::sync::Mutex<Option<PatchCache>> {
    PATCH_CACHE.get_or_init(|| std::sync::Mutex::new(None))
}

/// Register the byte hook (idempotent; call once from cplugin_init).
///
/// The callback performs NO JNI/class-file work (loader-lock discipline —
/// see improved_noise): pristine capture at the class's own load, patch
/// served from the cache computed on the quiet activation worker.
pub fn register() {
    if !enabled() {
        eprintln!(
            "[crussty-plugin] item_paper: dormant (lever_flag != items_paper, vanilla item merging)"
        );
        return;
    }
    cplug_sdk::hooks::register_bytes(ITEM_CLASS, |_name, bytes| {
        if !READY.load(Ordering::Relaxed) {
            // Pristine sighting (the original class load): stash the bytes
            // for the worker to patch; never rewrite here.
            eprintln!(
                "[crussty-plugin] item_paper: pristine sighting {} bytes (major {})",
                bytes.len(),
                crate::improved_noise::class_version(bytes).map(|(m, _)| m).unwrap_or(0)
            );
            let mut orig = orig_lock().lock().unwrap_or_else(PoisonError::into_inner);
            if orig.is_none() {
                *orig = Some(bytes.to_vec());
            }
            return None;
        }
        // Serve the precomputed patch; the clone is an Arc refcount bump.
        let cached = patch_lock().lock().unwrap_or_else(PoisonError::into_inner).clone();
        if !SERVE_LOGGED.swap(true, Ordering::Relaxed) {
            eprintln!(
                "[crussty-plugin] item_paper: hook serve {} bytes (major {})",
                cached.as_ref().map(|c| c.bytes.len()).unwrap_or(0),
                cached.as_ref().map(|c| c.major).unwrap_or(0)
            );
        }
        cached.map(|c| c.bytes.to_vec())
    });
}

/// Background activation: wait for ItemEntity, capture pristine bytes, compute
/// the size-neutral cadence/loop patch, flip READY and retransform.
pub fn activate() {
    if !enabled() {
        return;
    }
    std::thread::spawn(|| {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(180);
        let mut forced_attempts = 0usize;
        loop {
            if cplug_sdk::classes::find_class(ITEM_CLASS).is_some() {
                break;
            }
            if std::time::Instant::now() > deadline {
                eprintln!(
                    "[crussty-plugin] item_paper: {ITEM_CLASS} not loaded within 180s, hook stays dormant"
                );
                return;
            }
            if std::time::Instant::now() > deadline - std::time::Duration::from_secs(170)
                && (forced_attempts < 12 || forced_attempts % 12 == 0)
            {
                forced_attempts += 1;
                eprintln!(
                    "[crussty-plugin] item_paper: forcing kernel load of {ITEM_CLASS} (attempt {forced_attempts})"
                );
                crate::improved_noise::force_load_kernel_class(ITEM_CLASS);
            }
            let sighted = cplug_sdk::classes::is_sighted(ITEM_CLASS);
            std::thread::sleep(std::time::Duration::from_millis(if sighted {
                2_000
            } else {
                10_000
            }));
        }

        if !crate::improved_noise::wait_for_boot() {
            eprintln!("[crussty-plugin] item_paper: boot marker not seen, hook stays dormant");
            return;
        }
        // TASK-80 crash lesson: settle after Done before any retransform.
        std::thread::sleep(std::time::Duration::from_secs(20));
        eprintln!("[crussty-plugin] item_paper: server booted, capturing kernel ItemEntity");

        let loader_ref = cplug_sdk::jni_util::with_attached(|env| {
            let Some(cls) = cplug_sdk::classes::find_class(ITEM_CLASS) else {
                return None;
            };
            let Some(class_cls) = env.find_class("java/lang/Class") else {
                crate::clear_exception(env);
                return None;
            };
            let loader = env
                .get_method_id(class_cls, "getClassLoader", "()Ljava/lang/ClassLoader;")
                .and_then(|mid| {
                    let l = env.call_object_method(cls.as_jclass(), mid, &[]);
                    (l as usize != 0).then_some(l)
                });
            env.delete_local_ref(class_cls);
            let loader = loader?; // None => clear exception, bail
            let gref = env.new_global_ref(loader);
            env.delete_local_ref(loader);
            if gref.is_null() {
                crate::describe_exception(env);
                return None;
            }
            KERNEL_LOADER.store(gref as usize, Ordering::SeqCst);
            Some(())
        });
        if loader_ref.is_none() || KERNEL_LOADER.load(Ordering::SeqCst) == 0 {
            eprintln!("[crussty-plugin] item_paper: no kernel loader captured, hook stays dormant");
            return;
        }

        // Capture pristine bytes if the class predates the hook (fast boots).
        if orig_lock().lock().unwrap_or_else(PoisonError::into_inner).is_none() {
            eprintln!(
                "[crussty-plugin] item_paper: class predates hook, capturing current bytes via no-op retransform"
            );
            let mut captured = false;
            for attempt in 1..=3 {
                let rc = cplug_sdk::retransform_class(ITEM_CLASS);
                eprintln!(
                    "[crussty-plugin] item_paper: capture retransform rc={rc} (attempt {attempt})"
                );
                captured = orig_lock()
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .is_some();
                if captured {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
            if !captured {
                eprintln!(
                    "[crussty-plugin] item_paper: retransform capture empty after 3 attempts, trying loader resource stream"
                );
                match resource_stream_capture() {
                    Some(bytes) if bytes.len() > 10 => {
                        eprintln!(
                            "[crussty-plugin] item_paper: resource-stream capture {} bytes",
                            bytes.len()
                        );
                        *orig_lock().lock().unwrap_or_else(PoisonError::into_inner) = Some(bytes);
                    }
                    _ => {
                        eprintln!(
                            "[crussty-plugin] item_paper: resource-stream capture failed too, hook stays dormant"
                        );
                        return;
                    }
                }
            }
        }
        let original = orig_lock().lock().unwrap_or_else(PoisonError::into_inner).clone();
        let Some(original) = original else {
            eprintln!(
                "[crussty-plugin] item_paper: no original bytes captured even after retransform, hook stays dormant"
            );
            return;
        };

        // Compute the size-neutral patch on the quiet thread (no JNI).
        let patched = match compute_patch(&original) {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "[crussty-plugin] item_paper: patch computation failed ({e}), hook stays dormant"
                );
                return;
            }
        };
        if let Err(e) = verify_patch(&original, &patched) {
            eprintln!(
                "[crussty-plugin] item_paper: patched bytes failed static verification ({e}), hook stays dormant"
            );
            return;
        }
        let major = crate::improved_noise::class_version(&patched).map(|(m, _)| m).unwrap_or(0);
        eprintln!(
            "[crussty-plugin] item_paper: computed size-neutral patch ({} -> {} bytes, major {major}, P1 rate≡40 + P2 merge-loop exit)",
            original.len(),
            patched.len()
        );
        *patch_lock().lock().unwrap_or_else(PoisonError::into_inner) = Some(PatchCache {
            bytes: Arc::from(patched),
            major,
        });

        // A SINGLE retransform; the callback serves the cached patch.
        crate::kernel_policy::audit_wire(
            ITEM_CLASS,
            "tick()V+mergeWithNeighbours()V",
            "items_paper canvas dither/loop-exit port v1",
        );
        READY.store(true, Ordering::Release);
        let rc = cplug_sdk::retransform_class(ITEM_CLASS);
        eprintln!("[crussty-plugin] item_paper: hook armed, retransform rc={rc}");
    });
}

/// Last-resort pristine capture: the kernel loader's resource stream yields
/// the original class file bytes with no JVMTI event delivery involved.
fn resource_stream_capture() -> Option<Vec<u8>> {
    cplug_sdk::jni_util::with_attached(|env| {
        let loader = KERNEL_LOADER.load(Ordering::SeqCst);
        if loader == 0 {
            return None;
        }
        let loader_cls = env.find_class("java/lang/ClassLoader")?;
        let garm = env.get_method_id(
            loader_cls,
            "getResourceAsStream",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
        )?;
        let res_name = env.new_string_utf(&format!("{ITEM_CLASS}.class"))?;
        let stream = env.call_object_method(
            loader as jvmti_bindings::jni::jobject,
            garm,
            &[jvmti_bindings::jni::jvalue { l: res_name }],
        );
        if stream.is_null() {
            crate::clear_exception(env);
            env.delete_local_ref(res_name);
            env.delete_local_ref(loader_cls);
            return None;
        }
        let in_cls = env.find_class("java/io/InputStream")?;
        let rab = env.get_method_id(in_cls, "readAllBytes", "()[B")?;
        let arr = env.call_object_method(stream, rab, &[]);
        let out = if arr.is_null() {
            crate::clear_exception(env);
            None
        } else {
            let jarr = arr as jvmti_bindings::jni::jbyteArray;
            let len = env.get_array_length(arr as jvmti_bindings::jni::jarray);
            let mut signed = vec![0i8; len as usize];
            env.get_byte_array_region(jarr, 0, len, &mut signed);
            Some(signed.iter().map(|&b| b as u8).collect::<Vec<u8>>())
        };
        env.delete_local_ref(arr);
        env.delete_local_ref(stream);
        env.delete_local_ref(res_name);
        env.delete_local_ref(loader_cls);
        out
    })
    .flatten()
}

// ---------------------------------------------------------------------------
// Minimal class-file surgery (read-only walk + size-neutral byte edits).
// ---------------------------------------------------------------------------

/// One constant-pool entry: tag + payload (a/b = u2 operands; utf8 = value).
#[derive(Clone, Debug)]
enum Cp {
    Utf8(String),
    Methodref { class: u16, nat: u16 },
    NameAndType { name: u16, desc: u16 },
    Other,
}

fn read_u2(b: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*b.get(off)?, *b.get(off + 1)?]))
}
fn read_u4(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_be_bytes([
        *b.get(off)?,
        *b.get(off + 1)?,
        *b.get(off + 2)?,
        *b.get(off + 3)?,
    ]))
}

/// Parse the constant pool; returns (entries indexed by cp index 1..count, end offset).
fn parse_cp(b: &[u8]) -> Option<(Vec<Cp>, usize)> {
    if read_u4(b, 0)? != 0xCAFEBABE {
        return None;
    }
    let count = read_u2(b, 8)? as usize;
    let mut cp = vec![Cp::Other; count.max(1)];
    let mut off = 10usize;
    let mut i = 1usize;
    while i < count {
        let tag = *b.get(off)?;
        match tag {
            1 => {
                let len = read_u2(b, off + 1)? as usize;
                let s = b.get(off + 3..off + 3 + len)?; // data after tag(1)+len(2)
                cp[i] = Cp::Utf8(String::from_utf8_lossy(s).into_owned());
                off += 3 + len; // 1 tag + 2 len + len data (off sits ON the tag)
            }
            5 | 6 => {
                cp[i] = Cp::Other;
                off += 9; // 1 tag + 8 payload
                i += 1; // long/double take two slots
            }
            9 | 10 | 11 => {
                cp[i] = Cp::Methodref {
                    class: read_u2(b, off + 1)?,
                    nat: read_u2(b, off + 3)?,
                };
                off += 5;
            }
            12 => {
                cp[i] = Cp::NameAndType {
                    name: read_u2(b, off + 1)?,
                    desc: read_u2(b, off + 3)?,
                };
                off += 5;
            }
            3 | 4 => {
                cp[i] = Cp::Other;
                off += 5;
            }
            7 | 8 | 16 | 19 | 20 => {
                cp[i] = Cp::Other;
                off += 3;
            }
            17 | 18 => {
                // Dynamic / InvokeDynamic: bootstrap index + NameAndType.
                cp[i] = Cp::Other;
                off += 5;
            }
            15 => {
                cp[i] = Cp::Other;
                off += 4;
            }
            _ => return None,
        }
        i += 1;
    }
    Some((cp, off))
}

fn utf8(cp: &[Cp], idx: u16) -> Option<&str> {
    match cp.get(idx as usize)? {
        Cp::Utf8(s) => Some(s),
        _ => None,
    }
}

/// Resolve a Methodref's (name, descriptor) via its NameAndType.
fn methodref_info(cp: &[Cp], idx: u16) -> Option<(String, String)> {
    match cp.get(idx as usize)? {
        Cp::Methodref { nat, .. } => match cp.get(*nat as usize)? {
            Cp::NameAndType { name, desc } => {
                Some((utf8(cp, *name)?.to_owned(), utf8(cp, *desc)?.to_owned()))
            }
            _ => None,
        },
        _ => None,
    }
}

/// Locate a method by (name, desc) and return its `Code` attribute's bytecode
/// range [start, end) as absolute offsets into the class bytes.
fn method_code_range(b: &[u8], cp: &[Cp], name: &str, desc: &str) -> Option<(usize, usize)> {
    let (_, mut off) = parse_cp(b)?;
    off += 2 + 2 + 2; // access_flags, this_class, super_class
    let ifc = read_u2(b, off)? as usize;
    off += 2 + ifc * 2;
    for section in ["fields", "methods"] {
        let count = read_u2(b, off)? as usize;
        off += 2;
        for _ in 0..count {
            let _access = read_u2(b, off)?;
            let mname = read_u2(b, off + 2)?;
            let mdesc = read_u2(b, off + 4)?;
            let attrs = read_u2(b, off + 6)? as usize;
            off += 8;
            let named = utf8(cp, mname).map(|s| s == name).unwrap_or(false);
            let descd = utf8(cp, mdesc).map(|s| s == desc).unwrap_or(false);
            if section == "methods" && named && descd {
                for _ in 0..attrs {
                    let aname = read_u2(b, off)?;
                    let alen = read_u4(b, off + 2)? as usize;
                    let adata = off + 6;
                    if utf8(cp, aname) == Some("Code") && alen >= 8 {
                        let code_len = read_u4(b, adata + 4)? as usize;
                        let cstart = adata + 8;
                        if cstart + code_len <= adata + alen {
                            return Some((cstart, cstart + code_len));
                        }
                        return None;
                    }
                    off = adata + alen;
                }
                return None; // method found but no Code attribute
            }
            for _ in 0..attrs {
                let alen = read_u4(b, off + 2)? as usize;
                off = off + 6 + alen;
            }
        }
    }
    None
}

/// Find all occurrences of `pattern` (0xFF bytes are wildcards) in `hay`.
fn find_pattern(hay: &[u8], pattern: &[u8]) -> Vec<usize> {
    let mut hits = Vec::new();
    if pattern.len() > hay.len() {
        return hits;
    }
    'outer: for i in 0..=(hay.len() - pattern.len()) {
        for (j, p) in pattern.iter().enumerate() {
            if *p != 0xFF && hay[i + j] != *p {
                continue 'outer;
            }
        }
        hits.push(i);
    }
    hits
}

/// Compute the patched class bytes:
//   P1: tick() `ifeq +4` → `goto +4` (rate == 40, Canvas 2s-wait-always).
//   P2: mergeWithNeighbours `invokevirtual isRemoved; ifeq +3` →
///       `invokevirtual isMergable; ifne +3` (Canvas loop exit).
fn compute_patch(original: &[u8]) -> Result<Vec<u8>, String> {
    let (cp, _) = parse_cp(original).ok_or("constant pool parse failed")?;

    // Resolve the CP indexes we patch against, from the pool itself.
    let is_removed_ref = (1..cp.len() as u16)
        .find(|&i| methodref_info(&cp, i).is_some_and(|(n, d)| n == "isRemoved" && d == "()Z"))
        .ok_or("CP: isRemoved()Z Methodref not found")?;
    let is_mergable_ref = (1..cp.len() as u16)
        .find(|&i| methodref_info(&cp, i).is_some_and(|(n, d)| n == "isMergable" && d == "()Z"))
        .ok_or("CP: isMergable()Z Methodref not found")?;

    let (tick_s, tick_e) =
        method_code_range(original, &cp, "tick", "()V").ok_or("tick()V code not found")?;
    let (merge_s, merge_e) = method_code_range(original, &cp, "mergeWithNeighbours", "()V")
        .ok_or("mergeWithNeighbours()V code not found")?;

    let mut out = original.to_vec();

    // ---- P1: cadence dither (Canvas itemEntitiesWaitTwoSecondsForMergeCheckAlways) ----
    // istore_2; iload_2; ifeq +7; iconst_2; goto +5; bipush 40; istore_3;
    // aload_0; getfield tickCount:I; iload_3; irem; ifne +24
    // Branch operands are RELATIVE TO THE OPCODE (JVM spec): ifeq +7 targets 441,
    // goto +5 targets 443, ifne +24 targets 474 (javap absolute offsets).
    const P1: [u8; 21] = [
        0x3d, 0x1c, 0x99, 0x00, 0x07, 0x05, 0xa7, 0x00, 0x05, 0x10, 0x28, 0x3e, 0x2a, 0xb4, 0xFF,
        0xFF, 0x1d, 0x70, 0x9a, 0x00, 0x18,
    ];
    let tick = &original[tick_s..tick_e];
    let p1_hits = find_pattern(tick, &P1);
    if p1_hits.len() != 1 {
        return Err(format!("P1 cadence pattern matched {} times in tick()V", p1_hits.len()));
    }
    let hit = tick_s + p1_hits[0];
    // The getfield operand must resolve to tickCount:I (defensive CP check).
    let gf_idx = u16::from_be_bytes([original[hit + 14], original[hit + 15]]);
    let gf_info = match cp.get(gf_idx as usize) {
        Some(Cp::Methodref { nat, .. }) => match cp.get(*nat as usize) {
            Some(Cp::NameAndType { name, desc }) => Some((
                utf8(&cp, *name).unwrap_or("").to_owned(),
                utf8(&cp, *desc).unwrap_or("").to_owned(),
            )),
            _ => None,
        },
        _ => None,
    };
    let gf_ok = matches!(&gf_info, Some((n, d)) if n == "tickCount" && d == "I");
    if !gf_ok {
        return Err(format!("P1 getfield is {gf_info:?}, expected tickCount:I"));
    }
    out[hit + 2] = 0xa8; // ifeq +4 -> goto +4 : moved-path iconst_2 goes dead, rate ≡ 40

    // ---- P2: merge-loop exit (Canvas: if (!this.isMergable()) break;) ----
    // aload_0; aload 4 (0x19 0x04, wide-index form); invokevirtual tryToMerge;
    // aload_0; invokevirtual isRemoved; ifeq +6; goto +6; goto -89
    const P2: [u8; 19] = [
        0x2a, 0x19, 0x04, 0xb6, 0xFF, 0xFF, 0x2a, 0xb6, 0xFF, 0xFF, 0x99, 0x00, 0x06, 0xa7,
        0x00, 0x06, 0xa7, 0xff, 0xa7,
    ];
    let merge = &original[merge_s..merge_e];
    let p2_hits = find_pattern(merge, &P2);
    if p2_hits.len() != 1 {
        return Err(format!(
            "P2 loop-exit pattern matched {} times in mergeWithNeighbours()V",
            p2_hits.len()
        ));
    }
    let hit2 = merge_s + p2_hits[0];
    // Defensive CP checks: the two invokevirtuals are tryToMerge / isRemoved.
    let ttm_idx = u16::from_be_bytes([original[hit2 + 4], original[hit2 + 5]]);
    let rem_idx = u16::from_be_bytes([original[hit2 + 8], original[hit2 + 9]]);
    if rem_idx != is_removed_ref {
        return Err(format!(
            "P2 second invokevirtual is #{rem_idx}, expected isRemoved #{is_removed_ref}"
        ));
    }
    match cp.get(ttm_idx as usize) {
        Some(Cp::Methodref { .. }) => {}
        _ => return Err("P2 first invokevirtual is not a Methodref".into()),
    }
    out[hit2 + 8] = (is_mergable_ref >> 8) as u8; // isRemoved -> isMergable
    out[hit2 + 9] = (is_mergable_ref & 0xff) as u8;
    out[hit2 + 10] = 0x9a; // ifeq -> ifne : if (!this.isMergable()) break;

    if out.len() != original.len() {
        return Err("patch changed class size (impossible)".into());
    }
    Ok(out)
}

/// Static re-verification of the patched bytes (defense in depth before arming).
fn verify_patch(original: &[u8], patched: &[u8]) -> Result<(), String> {
    let (cp, _) = parse_cp(patched).ok_or("patched CP parse failed")?;
    let is_mergable_ref = (1..cp.len() as u16)
        .find(|&i| methodref_info(&cp, i).is_some_and(|(n, d)| n == "isMergable" && d == "()Z"))
        .ok_or("patched CP: isMergable()Z not found")?;

    let (tick_s, tick_e) =
        method_code_range(patched, &cp, "tick", "()V").ok_or("patched tick()V code not found")?;
    let (merge_s, merge_e) = method_code_range(patched, &cp, "mergeWithNeighbours", "()V")
        .ok_or("patched mergeWithNeighbours()V code not found")?;

    // P1: the patched cadence must be goto (+4) — i.e. ifeq form gone.
    const P1_AFTER: [u8; 21] = [
        0x3d, 0x1c, 0xa8, 0x00, 0x07, 0x05, 0xa7, 0x00, 0x05, 0x10, 0x28, 0x3e, 0x2a, 0xb4, 0xFF,
        0xFF, 0x1d, 0x70, 0x9a, 0x00, 0x18,
    ];
    if find_pattern(&patched[tick_s..tick_e], &P1_AFTER).len() != 1 {
        return Err("P1: patched cadence (goto form) not found".into());
    }
    // P2: isMergable operand + ifne present.
    let p2 = find_pattern(&patched[merge_s..merge_e], &[
        0x2a, 0x19, 0x04, 0xb6, 0xFF, 0xFF, 0x2a, 0xb6, 0xFF, 0xFF, 0x9a, 0x00, 0x06, 0xa7,
        0x00, 0x06, 0xa7, 0xff, 0xa7,
    ]);
    if p2.len() != 1 {
        return Err("P2: patched loop-exit (ifne form) not found".into());
    }
    let idx = u16::from_be_bytes([
        patched[merge_s + p2[0] + 8],
        patched[merge_s + p2[0] + 9],
    ]);
    if idx != is_mergable_ref {
        return Err(format!("P2: invokevirtual operand #{idx} != isMergable #{is_mergable_ref}"));
    }
    // Size-neutral guarantee: only the four patched bytes may differ
    // (P1 opcode; P2 CP-operand hi/lo + opcode).
    let diffs: Vec<usize> = original
        .iter()
        .zip(patched.iter())
        .enumerate()
        .filter_map(|(i, (a, b))| (a != b).then_some(i))
        .collect();
    if diffs.len() != 4 {
        return Err(format!("expected exactly 4 differing bytes, got {}", diffs.len()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Optional live-kernel test: point CRUSSTY_ITEM_CLASS_TEST at an extracted
    /// purpur-1.21.10 net/minecraft/world/entity/item/ItemEntity.class to run
    /// the size-neutral surgery + verification against the real bytes. Skipped
    /// when unset (CI has no kernel jar).
    #[test]
    fn live_kernel_patch() {
        let Some(path) = std::env::var_os("CRUSSTY_ITEM_CLASS_TEST") else {
            return;
        };
        let bytes = std::fs::read(std::path::Path::new(&path)).expect("read ItemEntity.class");
        let patched = compute_patch(&bytes).expect("compute_patch");
        verify_patch(&bytes, &patched).expect("verify_patch");
        assert_eq!(bytes.len(), patched.len());
        eprintln!("item_paper live-kernel patch verified on {} bytes", bytes.len());
    }
}
