//! Minimal classfile surgery for the `area_map` byte hook.
//!
//! Replaces the body of `SingleUserAreaMap.update(int,int,int)` with a
//! branch-minimal body that delegates the whole difference-apply to the
//! injected `SingleUserAreaMapOps` helper (native enumeration + Java apply
//! loop). The original contract is preserved exactly:
//!   - negative newDistance -> IllegalArgumentException + athrow
//!   - lastChunkX == NOT_SET (i32::MIN) -> return false, fields untouched
//!   - otherwise: write fields, run native ops, return true.
//!
//! All new constant-pool entries are APPENDED (existing indices stay valid).
//! The new method body has branch targets, so its Code attribute carries a
//! StackMapTable (mandatory for class-file major >= 51). One frame per basic
//! block leader (offset 0, the ifge fall-through at 4, the ifge target at 16,
//! the if_icmpne fall-through at 42, the if_icmpne target at 44):
//!   0, 4, 16, 42        -> this, toX, toZ, newD
//!   44                  -> this, toX, toZ, newD, fromX, fromZ, oldD
//!
//! Layout of the new body (locals: 0=this, 1=toX, 2=toZ, 3=newD, 4=fromX,
//! 5=fromZ, 6=oldD, 7=param; max_stack 8 = the static run call). JVM "_n"
//! load/store forms exist ONLY for slots 0-3 (iload=0x15, istore=0x36,
//! aload=0x19, astore=0x3a are the two-byte generic forms), and branch
//! offsets are measured from the address of the opcode, not the next
//! instruction:
//! ```text
//!   0: iload_3
//!   1: ifge +15            -> 16
//!   4: new 0xIllegalArgumentException
//!   7: dup
//!   8: iload_3
//!   9: invokestatic Integer.toString(I)Ljava/lang/String;
//!  12: invokespecial 0xIllegalArgumentException.<init>(Ljava/lang/String;)V
//!  15: athrow
//!  16: aload_0; getfield lastChunkX; istore 4
//!  22: aload_0; getfield lastChunkZ; istore 5
//!  28: aload_0; getfield distance;  istore 6
//!  34: iload 4; ldc_w MIN_VALUE; if_icmpne +5 -> 44
//!  42: iconst_0; ireturn
//!  44: aload_0; iload_1; putfield lastChunkX
//!  49: aload_0; iload_2; putfield lastChunkZ
//!  54: aload_0; iload_3; putfield distance
//!  59: aload_0; getfield parameter; astore 7
//!  65: aload_0; iload 4; iload 5; iload 6; iload_1; iload_2; iload_3; aload 7
//!  77: invokestatic SingleUserAreaMapOps.run(LSingleUserAreaMap;IIIIIILjava/lang/Object;)V
//!  80: iconst_1; ireturn
//! ```

const TAG_UTF8: u8 = 1;
const TAG_INTEGER: u8 = 3;
const TAG_CLASS: u8 = 7;
const TAG_FIELDREF: u8 = 9;
const TAG_METHODREF: u8 = 10;
/// CONSTANT_InterfaceMethodref (JVMS 4.4.2) — `invokestatic`/`invokeinterface`
/// may legally reference it on class major >= 52; the retarget resolver must
/// accept both ref tags or a hostile/edge case returns NotFound instead of
/// patching.
const TAG_INTERFACEMETHODREF: u8 = 11;
const TAG_NAMEANDTYPE: u8 = 12;

pub const OPS_CLASS: &str = "ca/spottedleaf/moonrise/common/misc/SingleUserAreaMapOps";
pub const MAP_CLASS: &str = "ca/spottedleaf/moonrise/common/misc/SingleUserAreaMap";

const NIE: &str = "java/lang/IllegalArgumentException";
const INTEGER: &str = "java/lang/Integer";

/// One parsed constant-pool entry: (apparent index, tag, payload).
type Entry = (u16, u8, Vec<u8>);

pub struct Pool {
    entries: Vec<Entry>,
    /// 1-based index the next appended entry will get; after a parse this
    /// equals cp_count (index space = 1 + sum of apparent slots).
    next: u16,
}

impl Pool {
    /// Parse the constant pool of `bytes` starting at `cp_start`; returns the
    /// pool and the offset just past it (where access_flags begins).
    pub fn parse(bytes: &[u8], cp_start: usize, cp_count: u16) -> Option<(Pool, usize)> {
        let mut pool = Pool {
            entries: Vec::with_capacity(cp_count as usize),
            next: 1,
        };
        let mut p = cp_start;
        let mut seen = 0u32;
        // JVMS: constant_pool_count = number of ENTRIES + 1 (index 0 is
        // reserved), where a long/double takes TWO table slots. The loop
        // therefore terminates on SLOT count — `seen` must advance by the
        // entry's slot count, not by 1 (G4 S7-12 root cause: pools carrying
        // long/double constants overran the pool into tag 0 and parse
        // failed — the area_map fixture has no longs, so the bug stayed
        // invisible until the real ImprovedNoise class hit this path).
        let entries_total = u32::from(cp_count.saturating_sub(1));
        while seen < entries_total {
            let tag = *bytes.get(p)?;
            p += 1;
            let (payload, slots): (Vec<u8>, u16) = match tag {
                TAG_UTF8 => {
                    if p + 2 > bytes.len() {
                        return None;
                    }
                    let len = usize::from(u16::from_be_bytes([bytes[p], bytes[p + 1]]));
                    // store the whole entry payload (length prefix + bytes) so
                    // utf8()/utf8_value() are symmetric with the file layout
                    let d = bytes.get(p..p + 2 + len)?.to_vec();
                    p += 2 + len;
                    (d, 1)
                }
                TAG_INTEGER | 4 => {
                    let d = bytes.get(p..p + 4)?.to_vec();
                    p += 4;
                    (d, 1)
                }
                5 | 6 => {
                    let d = bytes.get(p..p + 8)?.to_vec();
                    p += 8;
                    (d, 2)
                }
                7 | 8 | 16 | 19 | 20 => {
                    let d = bytes.get(p..p + 2)?.to_vec();
                    p += 2;
                    (d, 1)
                }
                9 | 10 | 11 | 12 | 17 | 18 => {
                    let d = bytes.get(p..p + 4)?.to_vec();
                    p += 4;
                    (d, 1)
                }
                15 => {
                    let d = bytes.get(p..p + 3)?.to_vec();
                    p += 3;
                    (d, 1)
                }
                _ => return None,
            };
            pool.entries.push((pool.next, tag, payload));
            pool.next = pool.next.checked_add(slots)?;
            seen += u32::from(slots);
        }
        Some((pool, p))
    }

    fn find(&self, tag: u8, payload: &[u8]) -> Option<u16> {
        self.entries
            .iter()
            .find(|(_, t, d)| *t == tag && d == payload)
            .map(|(i, _, _)| *i)
    }

    fn push(&mut self, tag: u8, payload: Vec<u8>, slots: u16) -> u16 {
        let idx = self.next;
        self.entries.push((idx, tag, payload));
        // A4-F3: saturate instead of silently wrapping CP indices near 64K.
        // patch_update guards on the saturated value and errors out loudly.
        self.next = self.next.saturating_add(slots);
        idx
    }

    fn utf8(&mut self, s: &str) -> u16 {
        let mut payload = Vec::with_capacity(2 + s.len());
        payload.extend_from_slice(&(s.len() as u16).to_be_bytes());
        payload.extend_from_slice(s.as_bytes());
        self.find(TAG_UTF8, &payload)
            .unwrap_or_else(|| self.push(TAG_UTF8, payload, 1))
    }

    fn int_const(&mut self, v: i32) -> u16 {
        let payload = v.to_be_bytes().to_vec();
        self.find(TAG_INTEGER, &payload)
            .unwrap_or_else(|| self.push(TAG_INTEGER, payload, 1))
    }

    fn class_of(&mut self, utf8_idx: u16) -> u16 {
        let payload = utf8_idx.to_be_bytes().to_vec();
        self.find(TAG_CLASS, &payload)
            .unwrap_or_else(|| self.push(TAG_CLASS, payload, 1))
    }

    fn name_and_type(&mut self, name: &str, desc: &str) -> u16 {
        let n = self.utf8(name);
        let d = self.utf8(desc);
        let mut payload = n.to_be_bytes().to_vec();
        payload.extend_from_slice(&d.to_be_bytes());
        self.find(TAG_NAMEANDTYPE, &payload)
            .unwrap_or_else(|| self.push(TAG_NAMEANDTYPE, payload, 1))
    }

    fn field_ref(&mut self, owner: &str, name: &str, desc: &str) -> u16 {
        let owner_utf8 = self.utf8(owner);
        let c = self.class_of(owner_utf8);
        let nat = self.name_and_type(name, desc);
        let mut payload = c.to_be_bytes().to_vec();
        payload.extend_from_slice(&nat.to_be_bytes());
        self.find(TAG_FIELDREF, &payload)
            .unwrap_or_else(|| self.push(TAG_FIELDREF, payload, 1))
    }

    fn method_ref(&mut self, owner: &str, name: &str, desc: &str) -> u16 {
        let owner_utf8 = self.utf8(owner);
        let c = self.class_of(owner_utf8);
        let nat = self.name_and_type(name, desc);
        let mut payload = c.to_be_bytes().to_vec();
        payload.extend_from_slice(&nat.to_be_bytes());
        self.find(TAG_METHODREF, &payload)
            .unwrap_or_else(|| self.push(TAG_METHODREF, payload, 1))
    }

    fn utf8_value(&self, idx: u16) -> Option<String> {
        let (_, tag, payload) = self.entries.iter().find(|(i, _, _)| *i == idx)?;
        if *tag != TAG_UTF8 {
            return None;
        }
        let len = usize::from(u16::from_be_bytes([payload[0], payload[1]]));
        String::from_utf8(payload[2..2 + len].to_vec()).ok()
    }

    /// Find-only UTF8 lookup (NO append). The retarget resolver uses this to
    /// probe for the target method's name/descriptor entries before anything
    /// is mutated: a method whose name/desc utf8 entries are absent cannot
    /// exist, and NotFound must not grow the pool as a side effect.
    pub fn find_utf8(&self, s: &str) -> Option<u16> {
        let mut payload = Vec::with_capacity(2 + s.len());
        payload.extend_from_slice(&(s.len() as u16).to_be_bytes());
        payload.extend_from_slice(s.as_bytes());
        self.find(TAG_UTF8, &payload)
    }

    /// Resolve a Methodref/InterfaceMethodref index to its
    /// `(class_internal_name, method_name, descriptor)` triple (G4 §3:
    /// call-site resolution is BY NAME, never by fixed bytecode offset —
    /// ASM COMPUTE_FRAMES runs shift CP indexes between boots).
    fn methodref_parts(&self, idx: u16) -> Option<(String, String, String)> {
        let (_, tag, payload) = self.entries.iter().find(|(i, _, _)| *i == idx)?;
        if *tag != TAG_METHODREF && *tag != TAG_INTERFACEMETHODREF {
            return None;
        }
        // Payload layout: class_index(2) name_and_type_index(2). The parser
        // guarantees 4 bytes, but this runs on hook-delivered bytes — stay
        // bounds-checked end to end (A4 audit style).
        if payload.len() < 4 {
            return None;
        }
        let class_idx = u16::from_be_bytes([payload[0], payload[1]]);
        let nat_idx = u16::from_be_bytes([payload[2], payload[3]]);
        let (_, ctag, cpayload) = self.entries.iter().find(|(i, _, _)| *i == class_idx)?;
        if *ctag != TAG_CLASS || cpayload.len() < 2 {
            return None;
        }
        let class_utf8 = u16::from_be_bytes([cpayload[0], cpayload[1]]);
        let (_, ntag, npayload) = self.entries.iter().find(|(i, _, _)| *i == nat_idx)?;
        if *ntag != TAG_NAMEANDTYPE || npayload.len() < 4 {
            return None;
        }
        let name_utf8 = u16::from_be_bytes([npayload[0], npayload[1]]);
        let desc_utf8 = u16::from_be_bytes([npayload[2], npayload[3]]);
        Some((
            self.utf8_value(class_utf8)?,
            self.utf8_value(name_utf8)?,
            self.utf8_value(desc_utf8)?,
        ))
    }

    /// Resolve a Fieldref index to its `(class_internal_name, field_name,
    /// descriptor)` triple — the F1 roundtrip test uses this to verify the
    /// patched body's getfield operand resolves to ServerLevel.simpleRandom
    /// by NAME (never by index assumption). Layout mirrors methodref_parts.
    fn fieldref_parts(&self, idx: u16) -> Option<(String, String, String)> {
        let (_, tag, payload) = self.entries.iter().find(|(i, _, _)| *i == idx)?;
        if *tag != TAG_FIELDREF {
            return None;
        }
        if payload.len() < 4 {
            return None;
        }
        let class_idx = u16::from_be_bytes([payload[0], payload[1]]);
        let nat_idx = u16::from_be_bytes([payload[2], payload[3]]);
        let (_, ctag, cpayload) = self.entries.iter().find(|(i, _, _)| *i == class_idx)?;
        if *ctag != TAG_CLASS || cpayload.len() < 2 {
            return None;
        }
        let class_utf8 = u16::from_be_bytes([cpayload[0], cpayload[1]]);
        let (_, ntag, npayload) = self.entries.iter().find(|(i, _, _)| *i == nat_idx)?;
        if *ntag != TAG_NAMEANDTYPE || npayload.len() < 4 {
            return None;
        }
        let name_utf8 = u16::from_be_bytes([npayload[0], npayload[1]]);
        let desc_utf8 = u16::from_be_bytes([npayload[2], npayload[3]]);
        Some((
            self.utf8_value(class_utf8)?,
            self.utf8_value(name_utf8)?,
            self.utf8_value(desc_utf8)?,
        ))
    }

    fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (_, tag, payload) in &self.entries {
            out.push(*tag);
            out.extend_from_slice(payload);
        }
        out
    }
}

struct Method {
    /// Offset of the method's access_flags within the whole classfile.
    start: usize,
    /// Offset just past the method's last attribute.
    end: usize,
    name_idx: u16,
    desc_idx: u16,
    access: u16,
}

/// Bounds-checked big-endian readers (audit A4-F1 / A11-H-02: find_method and
/// parse_layout run on bytes delivered by ClassFileLoadHook — a truncated or
/// hostile class must yield None, never an index panic, because a panic on a
/// class-load thread aborts the whole JVM).
fn u16_at(bytes: &[u8], p: usize) -> Option<u16> {
    let b = bytes.get(p..p.checked_add(2)?)?;
    Some(u16::from_be_bytes([b[0], b[1]]))
}

fn u32_at(bytes: &[u8], p: usize) -> Option<u32> {
    let b = bytes.get(p..p.checked_add(4)?)?;
    Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

/// Locate the method with the given (name_idx, desc_idx) in the method table
/// beginning at `methods_start` (offset of the methods_count field).
fn find_method(
    bytes: &[u8],
    methods_start: usize,
    name_idx: u16,
    desc_idx: u16,
) -> Option<Method> {
    let mut p = methods_start;
    let count = usize::from(u16_at(bytes, p)?);
    p = p.checked_add(2)?;
    for _ in 0..count {
        let start = p;
        let access = u16_at(bytes, p)?;
        let n = u16_at(bytes, p.checked_add(2)?)?;
        let d = u16_at(bytes, p.checked_add(4)?)?;
        p = p.checked_add(6)?;
        let attr_count = usize::from(u16_at(bytes, p)?);
        p = p.checked_add(2)?;
        for _ in 0..attr_count {
            let len = u32_at(bytes, p.checked_add(2)?)?;
            p = p.checked_add(6)?.checked_add(len as usize)?;
        }
        if n == name_idx && d == desc_idx {
            return Some(Method {
                start,
                end: p,
                name_idx: n,
                desc_idx: d,
                access,
            });
        }
    }
    None
}

pub struct ClassLayout {
    pool: Pool,
    /// Offset of access_flags (end of the constant pool region).
    cp_end: usize,
    /// cpool index of this_class (a CONSTANT_Class entry).
    this_class_idx: u16,
    /// Offset of the methods_count field (start of the method table).
    methods_start: usize,
    /// Offset of the fields_count field (start of the field table).
    fields_start: usize,
}

fn parse_layout(bytes: &[u8]) -> Option<ClassLayout> {
    // A4-F2: magic + version gate. StackMapTable requires major >= 51; a
    // non-classfile blob (garbage hook input) must fail closed HERE.
    if bytes.len() < 10 {
        return None;
    }
    if u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) != 0xCA_FE_BA_BE {
        return None;
    }
    if u16::from_be_bytes([bytes[6], bytes[7]]) < 51 {
        return None;
    }
    let cp_count = u16::from_be_bytes([bytes[8], bytes[9]]);
    let (pool, cp_end) = Pool::parse(bytes, 10, cp_count)?;
    let this_class_idx = u16_at(bytes, cp_end.checked_add(2)?)?;
    let mut p = cp_end.checked_add(6)?; // access_flags(2) this_class(2) super_class(2)
    let iface_count = usize::from(u16_at(bytes, p)?);
    p = p.checked_add(2)?.checked_add(2 * iface_count)?;
    let fields_start = p;
    let fields_count = usize::from(u16_at(bytes, p)?);
    p = p.checked_add(2)?;
    for _ in 0..fields_count {
        p = p.checked_add(6)?;
        let attr_count = usize::from(u16_at(bytes, p)?);
        p = p.checked_add(2)?;
        for _ in 0..attr_count {
            let len = u32_at(bytes, p.checked_add(2)?)?;
            p = p.checked_add(6)?.checked_add(len as usize)?;
        }
    }
    Some(ClassLayout {
        pool,
        cp_end,
        this_class_idx,
        fields_start,
        methods_start: p,
    })
}

/// Resolve the internal name of this_class (e.g. "a/b/C") from the pool.
fn this_class_name(layout: &ClassLayout) -> Option<String> {
    let (_, tag, payload) = layout
        .pool
        .entries
        .iter()
        .find(|(i, _, _)| *i == layout.this_class_idx)?;
    if *tag != TAG_CLASS {
        return None;
    }
    let utf8_idx = u16::from_be_bytes([payload[0], payload[1]]);
    layout.pool.utf8_value(utf8_idx)
}

/// Build the patched class bytes for `SingleUserAreaMap`: the constant pool
/// keeps every original entry (append-only), the `update` method's Code is
/// replaced wholesale.
pub fn patch_update(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout")?;
    let this_name = this_class_name(&layout).ok_or("cannot resolve this_class name")?;
    if this_name != MAP_CLASS {
        return Err(format!("unexpected class {this_name}"));
    }
    let mut pool = layout.pool;

    let update_name = pool.utf8("update");
    let update_desc = pool.utf8("(III)Z");
    let m = find_method(bytes, layout.methods_start, update_name, update_desc)
        .ok_or("update(III)Z not found")?;

    // ---- constant refs needed by the new body (appended when absent) ----
    let f_last_x = pool.field_ref(&this_name, "lastChunkX", "I");
    let f_last_z = pool.field_ref(&this_name, "lastChunkZ", "I");
    let f_dist = pool.field_ref(&this_name, "distance", "I");
    let f_param = pool.field_ref(&this_name, "parameter", "Ljava/lang/Object;");
    let nie_utf8 = pool.utf8(NIE);
    let cls_nie = pool.class_of(nie_utf8);
    let m_nie_init = pool.method_ref(NIE, "<init>", "(Ljava/lang/String;)V");
    let m_to_str = pool.method_ref(INTEGER, "toString", "(I)Ljava/lang/String;");
    let min_int = pool.int_const(i32::MIN);
    let run_desc = format!("(L{this_name};IIIIIILjava/lang/Object;)V");
    let m_run = pool.method_ref(OPS_CLASS, "run", &run_desc);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for patched refs".into());
    }

    let mut code = Vec::with_capacity(82);
    let u2 = |out: &mut Vec<u8>, v: u16| out.extend_from_slice(&v.to_be_bytes());
    // A: throw guard (16 bytes; ifge +15 -> 0x10, offset measured from the
    // opcode address per JVMS, not from the next instruction)
    code.push(0x1d); // iload_3
    code.extend_from_slice(&[0x9c, 0x00, 0x0f]); // ifge +15 -> 0x10
    code.push(0xbb); // new
    u2(&mut code, cls_nie);
    code.push(0x59); // dup
    code.push(0x1d); // iload_3
    code.push(0xb8); // invokestatic
    u2(&mut code, m_to_str);
    code.push(0xb7); // invokespecial
    u2(&mut code, m_nie_init);
    code.push(0xbf); // athrow
    debug_assert_eq!(code.len(), 16);
    // B: snapshot old state (18 bytes). Slots 4-6 need the generic forms:
    // there are NO istore_4.. istore_7 short opcodes (istore<i> = 0x36 + u1).
    code.push(0x2a); // aload_0
    code.push(0xb4); // getfield
    u2(&mut code, f_last_x);
    code.extend_from_slice(&[0x36, 0x04]); // istore 4
    code.push(0x2a);
    code.push(0xb4);
    u2(&mut code, f_last_z);
    code.extend_from_slice(&[0x36, 0x05]); // istore 5
    code.push(0x2a);
    code.push(0xb4);
    u2(&mut code, f_dist);
    code.extend_from_slice(&[0x36, 0x06]); // istore 6
    // C: NOT_SET check. 0xa0 = if_icmpne (0xa4 is if_icmple); there is no
    // iload_4.._6 either, so use generic iload = 0x15 + u1.
    code.extend_from_slice(&[0x15, 0x04]); // iload 4
    code.push(0x13); // ldc_w
    u2(&mut code, min_int);
    code.extend_from_slice(&[0xa0, 0x00, 0x05]); // if_icmpne +5 -> 0x2c
    code.push(0x03); // iconst_0
    code.push(0xac); // ireturn
    debug_assert_eq!(code.len(), 44);
    // D: write fields, run ops
    code.push(0x2a); // aload_0
    code.push(0x1b); // iload_1
    code.push(0xb5); // putfield
    u2(&mut code, f_last_x);
    code.push(0x2a);
    code.push(0x1c); // iload_2
    code.push(0xb5);
    u2(&mut code, f_last_z);
    code.push(0x2a);
    code.push(0x1d); // iload_3
    code.push(0xb5);
    u2(&mut code, f_dist);
    code.push(0x2a);
    code.push(0xb4); // getfield parameter
    u2(&mut code, f_param);
    code.extend_from_slice(&[0x3a, 0x07]); // astore 7
    code.push(0x2a); // aload_0
    code.extend_from_slice(&[0x15, 0x04]); // iload 4
    code.extend_from_slice(&[0x15, 0x05]); // iload 5
    code.extend_from_slice(&[0x15, 0x06]); // iload 6
    code.push(0x1b); // iload_1
    code.push(0x1c); // iload_2
    code.push(0x1d); // iload_3
    code.extend_from_slice(&[0x19, 0x07]); // aload 7
    code.push(0xb8); // invokestatic
    u2(&mut code, m_run);
    code.push(0x04); // iconst_1
    code.push(0xac); // ireturn
    debug_assert_eq!(code.len(), 82, "emitted code is {}", code.len());

    // ---- StackMapTable: one frame per branch target, mirroring javac ----
    // Only the two branch targets (16 and 44) are basic-block leaders that
    // need stack-map entries; full_frame entries at non-leader offsets are
    // rejected by the HotSpot verifier. Frame @16 has the method's initial
    // locals (this + toX + toZ + newD) -> same_frame (16). Frame @44 has
    // those plus the three snapshot int locals (fromX/fromZ/oldD) appended
    // via append_frame (254). Per JVMS 4.10.1.2 each offset_delta after the
    // first frame is offset_of(previous) + offset_delta + 1, so the second
    // entry needs offset_delta 44-16-1 = 27 (0x1b), not 28.
    let mut stackmap = (2u16).to_be_bytes().to_vec();
    stackmap.push(0x10); // same_frame @16 -> absolute frame @16 (initial 4 locals)
    stackmap.extend_from_slice(&[0xfe, 0x00, 0x1b, 0x01, 0x01, 0x01]); // append -> absolute @44

    // ---- Code attribute ----
    let mut code_attr = Vec::new();
    u2(&mut code_attr, pool.utf8("Code"));
    // attribute_length comes after we know the body size
    let mut body = Vec::new();
    u2(&mut body, 8); // max_stack
    u2(&mut body, 8); // max_locals
    body.extend_from_slice(&(code.len() as u32).to_be_bytes());
    body.extend_from_slice(&code);
    body.extend_from_slice(&[0, 0]); // exception_table_length
    body.extend_from_slice(&(1u16).to_be_bytes()); // attributes_count
    u2(&mut body, pool.utf8("StackMapTable"));
    body.extend_from_slice(&(stackmap.len() as u32).to_be_bytes());
    body.extend_from_slice(&stackmap);
    code_attr.extend_from_slice(&(body.len() as u32).to_be_bytes());
    code_attr.extend_from_slice(&body);

    // ---- replacement method entry ----
    let mut method = Vec::new();
    u2(&mut method, m.access);
    u2(&mut method, m.name_idx);
    u2(&mut method, m.desc_idx);
    u2(&mut method, 1); // attributes_count
    method.extend_from_slice(&code_attr);

    // ---- splice: header + new cp + tail with the update method replaced ----
    let mut out = Vec::with_capacity(bytes.len() + 256);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    u2(&mut out, pool.next); // new cp_count
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&bytes[layout.cp_end..m.start]);
    out.extend_from_slice(&method);
    out.extend_from_slice(&bytes[m.end..]);
    Ok(out)
}

// ---------------------------------------------------------------------------
// F1 BATCH-RNG body swap (family-agg pack member F1, TASK-247/S7-111 ->
// S7-112): replaces the body of `ServerLevel.optimiseRandomTick(LevelChunk;I)V`
// with a straight-line `invokestatic RandomTickOps.run(...)` delegation. The
// helper (randomtick/src/RandomTickOps.java, ECJ-compiled, include_bytes! in
// src/randomtick.rs) carries the inlined bit-exact LCG pick loop; parity bank:
// research/f1-batchrng-2026-09-17/parity_output.txt (320K attempts, 8 seeds,
// 160K hit interleaves incl. nextGaussian — ALL PASS).
//
// Contract (cfdump-verified on the run21 patched-kernel bytes, build
// 2025-12-11, fixture = tests/fixtures/ServerLevel.class):
//   this_class   net/minecraft/server/level/ServerLevel
//   simpleRandom 0x0012 (private final)
//                Lca/spottedleaf/moonrise/common/util/SimpleThreadUnsafeRandom;
//   optimiseRandomTick 0x0002 (private instance) (Lnet/minecraft/world/level/
//                chunk/LevelChunk;I)V — sole call site is an invokevirtual in
//                the same class, untouched by a body swap.
//
// New body — NO branches, so the only verifier frame is the implicit initial
// one: an EMPTY StackMapTable (0 entries) is emitted explicitly (javac
// convention; HotSpot's type-checking verifier requires frames only at branch
// targets). Locals: 0=this, 1=chunk, 2=ticks. max_stack=4 (this, chunk, ticks,
// random at the invokestatic).
// ```text
//    0: aload_0
//    1: aload_1
//    2: iload_2
//    3: aload_0
//    4: getfield simpleRandom
//    7: invokestatic RandomTickOps.run
//       (Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/
//        chunk/LevelChunk;ILca/spottedleaf/moonrise/common/util/
//        SimpleThreadUnsafeRandom;)V
//   10: return
// ```
// CP growth is append-only (dedup via the shared Pool); existing indices stay
// valid; the 64K saturation guard errors out loudly. Panic-free on
// hook-delivered bytes (same audit discipline as patch_update): every read is
// bounds-checked through parse_layout/find_method; malformed input yields Err.
//
// Idempotency: patch(patch(x)) == patch(x) — the second pass re-finds the
// method, re-emits the identical 11-byte body, and the dedup Pool appends
// nothing, so the output is byte-identical. The hook layer ALSO guards with
// the PATCHED-swap convention (src/randomtick.rs), making re-sights free.
pub const SERVER_LEVEL_CLASS: &str = "net/minecraft/server/level/ServerLevel";
pub const RANDOMTICK_OPS_CLASS: &str = "net/minecraft/server/level/RandomTickOps";
const SIMPLE_RANDOM_DESC: &str =
    "Lca/spottedleaf/moonrise/common/util/SimpleThreadUnsafeRandom;";

pub fn patch_optimise_random_tick(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout")?;
    let this_name = this_class_name(&layout).ok_or("cannot resolve this_class name")?;
    if this_name != SERVER_LEVEL_CLASS {
        return Err(format!("unexpected class {this_name}"));
    }
    let mut pool = layout.pool;

    // Find-only probes first: a ServerLevel without the target method's
    // name/descriptor entries is not the kernel build we verified — fail
    // closed BEFORE any pool mutation.
    let opt_desc = "(Lnet/minecraft/world/level/chunk/LevelChunk;I)V";
    let Some(name_idx) = pool.find_utf8("optimiseRandomTick") else {
        return Err("optimiseRandomTick not found".into());
    };
    let Some(desc_idx) = pool.find_utf8(opt_desc) else {
        return Err("optimiseRandomTick descriptor not found".into());
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
        .ok_or("optimiseRandomTick(Lnet/minecraft/world/level/chunk/LevelChunk;I)V not found")?;

    // ---- constant refs needed by the new body (appended when absent) ----
    let f_rand = pool.field_ref(&this_name, "simpleRandom", SIMPLE_RANDOM_DESC);
    let run_desc = format!(
        "(L{this_name};Lnet/minecraft/world/level/chunk/LevelChunk;I{SIMPLE_RANDOM_DESC})V"
    );
    let m_run = pool.method_ref(RANDOMTICK_OPS_CLASS, "run", &run_desc);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for F1 refs".into());
    }

    let mut code = Vec::with_capacity(11);
    let u2 = |out: &mut Vec<u8>, v: u16| out.extend_from_slice(&v.to_be_bytes());
    code.push(0x2a); // aload_0
    code.push(0x2b); // aload_1
    code.push(0x1c); // iload_2
    code.push(0x2a); // aload_0
    code.push(0xb4); // getfield simpleRandom
    u2(&mut code, f_rand);
    code.push(0xb8); // invokestatic RandomTickOps.run
    u2(&mut code, m_run);
    code.push(0xb1); // return
    debug_assert_eq!(code.len(), 11, "emitted code is {}", code.len());

    // ---- Code attribute: empty exception table + EMPTY StackMapTable ----
    let mut code_attr = Vec::new();
    u2(&mut code_attr, pool.utf8("Code"));
    let mut body = Vec::new();
    u2(&mut body, 4); // max_stack: this, chunk, ticks, random
    u2(&mut body, 3); // max_locals: this, chunk, ticks
    body.extend_from_slice(&(code.len() as u32).to_be_bytes());
    body.extend_from_slice(&code);
    body.extend_from_slice(&[0, 0]); // exception_table_length
    body.extend_from_slice(&(1u16).to_be_bytes()); // attributes_count
    u2(&mut body, pool.utf8("StackMapTable"));
    body.extend_from_slice(&2u32.to_be_bytes()); // attribute_length
    body.extend_from_slice(&0u16.to_be_bytes()); // number_of_entries = 0
    code_attr.extend_from_slice(&(body.len() as u32).to_be_bytes());
    code_attr.extend_from_slice(&body);

    // ---- replacement method entry ----
    let mut method = Vec::new();
    u2(&mut method, m.access);
    u2(&mut method, m.name_idx);
    u2(&mut method, m.desc_idx);
    u2(&mut method, 1); // attributes_count
    method.extend_from_slice(&code_attr);

    // ---- splice: header + new cp + tail with the method replaced ----
    let mut out = Vec::with_capacity(bytes.len() + 128);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    u2(&mut out, pool.next); // new cp_count
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&bytes[layout.cp_end..m.start]);
    out.extend_from_slice(&method);
    out.extend_from_slice(&bytes[m.end..]);
    Ok(out)
}

// ---------------------------------------------------------------------------
// F2 BRAIN-ITERATORS body swap (family-agg pack member F2, TASK-249/S7-113 ->
// S7-114; protocol: docs/FAMILY_AGG_PREREGISTRATION.md §5). Same machine as
// [`patch_optimise_random_tick`]: the whole body of
// `Brain.startEachNonRunningBehavior` becomes a straight-line delegation to
// the banked lens helper `BrainOps.startEachNonRunning` (parity PASS: 4828
// production-entry calls / 3083 order-exact events / 1740 mutations,
// research/f2-brainiter-2026-09-17/parity_output.txt).
//
// The 14-byte body (offset-by-offset contract in BrainOps.java header):
//   aload_0; getfield availableBehaviorsByPriority:Ljava/util/Map;
//   aload_0; getfield activeActivities:Ljava/util/Set;
//   aload_1 (ServerLevel); aload_2 (LivingEntity);
//   invokestatic BrainOps.startEachNonRunning:(Ljava/util/Map;Ljava/util/Set;
//     Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/
//     LivingEntity;)V
//   return
// No branches => EMPTY StackMapTable (0 frames). The two getfields execute
// inside Brain.class itself on its own private fields — verifier-legal per
// JVMS access control (class-identity, not caller-shape), so the helper needs
// NO Unsafe/reflection at all. max_stack 5 (vanilla value, >= the 4 slots the
// straight line needs), max_locals 3 (this, level, entity).
//
// Idempotency: patch(patch(x)) == patch(x) — dedup Pool appends nothing on
// the second pass and the body re-emits byte-identical. Fail-closed: any
// other class / missing method => Err before any mutation.
pub const BRAIN_CLASS: &str = "net/minecraft/world/entity/ai/Brain";
pub const BRAIN_OPS_CLASS: &str = "net/minecraft/world/entity/ai/BrainOps";
const START_EACH_DESC: &str =
    "(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)V";

pub fn patch_brain_start_each(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout")?;
    let this_name = this_class_name(&layout).ok_or("cannot resolve this_class name")?;
    if this_name != BRAIN_CLASS {
        return Err(format!("unexpected class {this_name}"));
    }
    let mut pool = layout.pool;

    // Find-only probes first: a Brain without the target method's name/
    // descriptor entries is not the kernel build we verified — fail closed
    // BEFORE any pool mutation.
    let Some(name_idx) = pool.find_utf8("startEachNonRunningBehavior") else {
        return Err("startEachNonRunningBehavior not found".into());
    };
    let Some(desc_idx) = pool.find_utf8(START_EACH_DESC) else {
        return Err("startEachNonRunningBehavior descriptor not found".into());
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx).ok_or(
        "startEachNonRunningBehavior(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)V not found",
    )?;

    // ---- constant refs needed by the new body (appended when absent) ----
    let f_prio = pool.field_ref(&this_name, "availableBehaviorsByPriority", "Ljava/util/Map;");
    let f_active = pool.field_ref(&this_name, "activeActivities", "Ljava/util/Set;");
    let lens_desc = "(Ljava/util/Map;Ljava/util/Set;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)V";
    let m_lens = pool.method_ref(BRAIN_OPS_CLASS, "startEachNonRunning", lens_desc);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for F2 refs".into());
    }

    let mut code = Vec::with_capacity(14);
    let u2 = |out: &mut Vec<u8>, v: u16| out.extend_from_slice(&v.to_be_bytes());
    code.push(0x2a); // aload_0
    code.push(0xb4); // getfield availableBehaviorsByPriority
    u2(&mut code, f_prio);
    code.push(0x2a); // aload_0
    code.push(0xb4); // getfield activeActivities
    u2(&mut code, f_active);
    code.push(0x2b); // aload_1 (ServerLevel)
    code.push(0x2c); // aload_2 (LivingEntity)
    code.push(0xb8); // invokestatic BrainOps.startEachNonRunning
    u2(&mut code, m_lens);
    code.push(0xb1); // return
    debug_assert_eq!(code.len(), 14, "emitted code is {}", code.len());

    // ---- Code attribute: empty exception table + EMPTY StackMapTable ----
    let mut code_attr = Vec::new();
    u2(&mut code_attr, pool.utf8("Code"));
    let mut body = Vec::new();
    u2(&mut body, 5); // max_stack: vanilla value, >= 4 the line needs
    u2(&mut body, 3); // max_locals: this, level, entity
    body.extend_from_slice(&(code.len() as u32).to_be_bytes());
    body.extend_from_slice(&code);
    body.extend_from_slice(&[0, 0]); // exception_table_length
    body.extend_from_slice(&(1u16).to_be_bytes()); // attributes_count
    u2(&mut body, pool.utf8("StackMapTable"));
    body.extend_from_slice(&2u32.to_be_bytes()); // attribute_length
    body.extend_from_slice(&0u16.to_be_bytes()); // number_of_entries = 0
    code_attr.extend_from_slice(&(body.len() as u32).to_be_bytes());
    code_attr.extend_from_slice(&body);

    // ---- replacement method entry ----
    let mut method = Vec::new();
    u2(&mut method, m.access);
    u2(&mut method, m.name_idx);
    u2(&mut method, m.desc_idx);
    u2(&mut method, 1); // attributes_count
    method.extend_from_slice(&code_attr);

    // ---- splice: header + new cp + tail with the method replaced ----
    let mut out = Vec::with_capacity(bytes.len() + 128);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    u2(&mut out, pool.next); // new cp_count
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&bytes[layout.cp_end..m.start]);
    out.extend_from_slice(&method);
    out.extend_from_slice(&bytes[m.end..]);
    Ok(out)
}

// ---------------------------------------------------------------------------
// F3 LEVELTICKS-READS body swaps (family-agg pack member F3, TASK-251/S7-115
// -> S7-116; protocol: docs/FAMILY_AGG_PREREGISTRATION.md §5). The F2 machine
// applied twice on the scheduled-tick drain pair:
//
//   1. `LevelTicks.runCollectedTicks(BiConsumer)` (private, vanilla body
//      @0-76) -> 6-byte straight line
//      `aload_0; aload_1; invokestatic TickBlockOps.runCollectedTicks:
//      (Lnet/minecraft/world/ticks/LevelTicks;Ljava/util/function/BiConsumer;)V;
//      return` — the helper replicates the vanilla drain bytecode-exactly
//      (incl. the set.remove-under-isEmpty-guard QUIRK) and opens the per-drain
//      section-cache window (parity PASS: research/f3-levelticks-2026-09-17/
//      parity_output.txt, S1-S9). max_stack 2, max_locals 2.
//
//   2. `ServerLevel.tickBlock(BlockPos, Block)` (private, vanilla body @0-53)
//      -> 7-byte straight line
//      `aload_0; aload_1; aload_2; invokestatic TickBlockOps.tickBlock:
//      (Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;
//      Lnet/minecraft/world/level/block/Block;)V; return` — the helper
//      replicates is->tick->counter&7->mid-tick with the getChunk hop routed
//      through the section cache. (The S7-115 GOAL note said "11 bytes":
//      arithmetic slip — 3 loads + 3B invokestatic + return = 7 bytes.
//      max_stack 3, max_locals 3.)
//
// No branches in either body => EMPTY StackMapTable (0 frames). Both helpers
// take the receiver as an explicit argument (public static, defined into the
// kernel loader by tickhook.rs — no nested classes), so NO getfields are
// needed and the bodies never touch foreign privates.
//
// COHABITATION WITH F1 (both hooks target ServerLevel): JVMTI retransformation
// re-runs the hook chain over the ORIGINAL class bytes, so a tickBlock-only
// patch returned during the F3 retransform would silently UNDO the F1
// optimiseRandomTick swap. tickhook.rs therefore composes: its ServerLevel
// callback re-applies [`patch_optimise_random_tick`] (idempotent — dedup CP
// appends make patch(patch(x)) == patch(x)) before [`patch_tick_block`].
// Both functions only touch their OWN method entry, so any application order
// converges.
//
// Idempotency per function: patch(patch(x)) == patch(x). Fail-closed: any
// other class / missing method => Err before any mutation.
pub const LEVELTICKS_CLASS: &str = "net/minecraft/world/ticks/LevelTicks";
pub const TICKBLOCK_OPS_CLASS: &str = "net/minecraft/server/level/TickBlockOps";
const RUN_COLLECTED_DESC: &str = "(Ljava/util/function/BiConsumer;)V";
const RUN_COLLECTED_OPS_DESC: &str =
    "(Lnet/minecraft/world/ticks/LevelTicks;Ljava/util/function/BiConsumer;)V";
const TICK_BLOCK_DESC: &str =
    "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;)V";
const TICK_BLOCK_OPS_DESC: &str = "(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;)V";

pub fn patch_run_collected_ticks(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout")?;
    let this_name = this_class_name(&layout).ok_or("cannot resolve this_class name")?;
    if this_name != LEVELTICKS_CLASS {
        return Err(format!("unexpected class {this_name}"));
    }
    let mut pool = layout.pool;

    // Find-only probes first (fail closed BEFORE any pool mutation).
    let Some(name_idx) = pool.find_utf8("runCollectedTicks") else {
        return Err("runCollectedTicks not found".into());
    };
    let Some(desc_idx) = pool.find_utf8(RUN_COLLECTED_DESC) else {
        return Err("runCollectedTicks descriptor not found".into());
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
        .ok_or("runCollectedTicks(Ljava/util/function/BiConsumer;)V not found")?;

    // Constant ref needed by the new body (appended when absent).
    let m_ops = pool.method_ref(TICKBLOCK_OPS_CLASS, "runCollectedTicks", RUN_COLLECTED_OPS_DESC);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for F3 refs".into());
    }

    let mut code = Vec::with_capacity(6);
    let u2 = |out: &mut Vec<u8>, v: u16| out.extend_from_slice(&v.to_be_bytes());
    code.push(0x2a); // aload_0 (LevelTicks receiver)
    code.push(0x2b); // aload_1 (BiConsumer)
    code.push(0xb8); // invokestatic TickBlockOps.runCollectedTicks
    u2(&mut code, m_ops);
    code.push(0xb1); // return
    debug_assert_eq!(code.len(), 6, "emitted code is {}", code.len());

    // Code attribute: empty exception table + EMPTY StackMapTable.
    let mut code_attr = Vec::new();
    u2(&mut code_attr, pool.utf8("Code"));
    let mut body = Vec::new();
    u2(&mut body, 2); // max_stack: receiver + consumer
    u2(&mut body, 2); // max_locals: this, BiConsumer
    body.extend_from_slice(&(code.len() as u32).to_be_bytes());
    body.extend_from_slice(&code);
    body.extend_from_slice(&[0, 0]); // exception_table_length
    body.extend_from_slice(&(1u16).to_be_bytes()); // attributes_count
    u2(&mut body, pool.utf8("StackMapTable"));
    body.extend_from_slice(&2u32.to_be_bytes()); // attribute_length
    body.extend_from_slice(&0u16.to_be_bytes()); // number_of_entries = 0
    code_attr.extend_from_slice(&(body.len() as u32).to_be_bytes());
    code_attr.extend_from_slice(&body);

    // Replacement method entry + splice (F2 machine verbatim).
    let mut method = Vec::new();
    u2(&mut method, m.access);
    u2(&mut method, m.name_idx);
    u2(&mut method, m.desc_idx);
    u2(&mut method, 1); // attributes_count
    method.extend_from_slice(&code_attr);

    let mut out = Vec::with_capacity(bytes.len() + 64);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    u2(&mut out, pool.next); // new cp_count
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&bytes[layout.cp_end..m.start]);
    out.extend_from_slice(&method);
    out.extend_from_slice(&bytes[m.end..]);
    Ok(out)
}

/// F3-queue hook (S7-117): `LevelTicks.collectTicks(JILProfilerFiller;)V`
/// (private, vanilla body @0-34 — the fused sort+counter+drain+reschedule
/// pipeline) -> 9-byte straight line
/// `aload_0; lload_1; iload_3; aload 4; invokestatic
/// TickBlockOps.collectTicks:(Lnet/minecraft/world/ticks/LevelTicks;
/// JILnet/minecraft/util/profiling/ProfilerFiller;)V; return`.
/// No branches => EMPTY StackMapTable. max_stack 5 (1+2+1+1 operand slots),
/// max_locals 5 (this, long, int, filler). Parity: REAL-vanilla REF bank PASS
/// (research/f3-levelticks-2026-09-17/parity_output.txt SQ1-SQ5).
pub fn patch_collect_ticks(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout")?;
    let this_name = this_class_name(&layout).ok_or("cannot resolve this_class name")?;
    if this_name != LEVELTICKS_CLASS {
        return Err(format!("unexpected class {this_name}"));
    }
    let mut pool = layout.pool;

    // Find-only probes first (fail closed BEFORE any pool mutation).
    const COLLECT_DESC: &str = "(JILnet/minecraft/util/profiling/ProfilerFiller;)V";
    const COLLECT_OPS_DESC: &str =
        "(Lnet/minecraft/world/ticks/LevelTicks;JILnet/minecraft/util/profiling/ProfilerFiller;)V";
    let Some(name_idx) = pool.find_utf8("collectTicks") else {
        return Err("collectTicks not found".into());
    };
    let Some(desc_idx) = pool.find_utf8(COLLECT_DESC) else {
        return Err("collectTicks descriptor not found".into());
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
        .ok_or("collectTicks(JILnet/minecraft/util/profiling/ProfilerFiller;)V not found")?;

    // Constant ref needed by the new body (appended when absent).
    let m_ops = pool.method_ref(TICKBLOCK_OPS_CLASS, "collectTicks", COLLECT_OPS_DESC);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for F3 refs".into());
    }

    let mut code = Vec::with_capacity(9);
    let u2 = |out: &mut Vec<u8>, v: u16| out.extend_from_slice(&v.to_be_bytes());
    code.push(0x2a); // aload_0 (LevelTicks receiver)
    code.push(0x1f); // lload_1 (gameTime)
    code.push(0x1d); // iload_3 (maxTicks)
    code.push(0x19); // aload 4 (ProfilerFiller; wide local index)
    code.push(0x04);
    code.push(0xb8); // invokestatic TickBlockOps.collectTicks
    u2(&mut code, m_ops);
    code.push(0xb1); // return
    debug_assert_eq!(code.len(), 9, "emitted code is {}", code.len());

    // Code attribute: empty exception table + EMPTY StackMapTable.
    let mut code_attr = Vec::new();
    u2(&mut code_attr, pool.utf8("Code"));
    let mut body = Vec::new();
    u2(&mut body, 5); // max_stack: receiver(1)+long(2)+int(1)+filler(1)
    u2(&mut body, 5); // max_locals: this, long(2 slots), int, filler
    body.extend_from_slice(&(code.len() as u32).to_be_bytes());
    body.extend_from_slice(&code);
    body.extend_from_slice(&[0, 0]); // exception_table_length
    body.extend_from_slice(&(1u16).to_be_bytes()); // attributes_count
    u2(&mut body, pool.utf8("StackMapTable"));
    body.extend_from_slice(&2u32.to_be_bytes()); // attribute_length
    body.extend_from_slice(&0u16.to_be_bytes()); // number_of_entries = 0
    code_attr.extend_from_slice(&(body.len() as u32).to_be_bytes());
    code_attr.extend_from_slice(&body);

    // Replacement method entry + splice (F2 machine verbatim).
    let mut method = Vec::new();
    u2(&mut method, m.access);
    u2(&mut method, m.name_idx);
    u2(&mut method, m.desc_idx);
    u2(&mut method, 1); // attributes_count
    method.extend_from_slice(&code_attr);

    let mut out = Vec::with_capacity(bytes.len() + 64);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    u2(&mut out, pool.next); // new cp_count
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&bytes[layout.cp_end..m.start]);
    out.extend_from_slice(&method);
    out.extend_from_slice(&bytes[m.end..]);
    Ok(out)
}

pub fn patch_tick_block(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout")?;
    let this_name = this_class_name(&layout).ok_or("cannot resolve this_class name")?;
    if this_name != SERVER_LEVEL_CLASS {
        return Err(format!("unexpected class {this_name}"));
    }
    let mut pool = layout.pool;

    // Find-only probes first (fail closed BEFORE any pool mutation).
    let Some(name_idx) = pool.find_utf8("tickBlock") else {
        return Err("tickBlock not found".into());
    };
    let Some(desc_idx) = pool.find_utf8(TICK_BLOCK_DESC) else {
        return Err("tickBlock descriptor not found".into());
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx).ok_or(
        "tickBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;)V not found",
    )?;

    // Constant ref needed by the new body (appended when absent).
    let m_ops = pool.method_ref(TICKBLOCK_OPS_CLASS, "tickBlock", TICK_BLOCK_OPS_DESC);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for F3 refs".into());
    }

    let mut code = Vec::with_capacity(7);
    let u2 = |out: &mut Vec<u8>, v: u16| out.extend_from_slice(&v.to_be_bytes());
    code.push(0x2a); // aload_0 (ServerLevel receiver)
    code.push(0x2b); // aload_1 (BlockPos)
    code.push(0x2c); // aload_2 (Block)
    code.push(0xb8); // invokestatic TickBlockOps.tickBlock
    u2(&mut code, m_ops);
    code.push(0xb1); // return
    debug_assert_eq!(code.len(), 7, "emitted code is {}", code.len());

    // Code attribute: empty exception table + EMPTY StackMapTable.
    let mut code_attr = Vec::new();
    u2(&mut code_attr, pool.utf8("Code"));
    let mut body = Vec::new();
    u2(&mut body, 3); // max_stack: level, pos, block
    u2(&mut body, 3); // max_locals: this, BlockPos, Block
    body.extend_from_slice(&(code.len() as u32).to_be_bytes());
    body.extend_from_slice(&code);
    body.extend_from_slice(&[0, 0]); // exception_table_length
    body.extend_from_slice(&(1u16).to_be_bytes()); // attributes_count
    u2(&mut body, pool.utf8("StackMapTable"));
    body.extend_from_slice(&2u32.to_be_bytes()); // attribute_length
    body.extend_from_slice(&0u16.to_be_bytes()); // number_of_entries = 0
    code_attr.extend_from_slice(&(body.len() as u32).to_be_bytes());
    code_attr.extend_from_slice(&body);

    // Replacement method entry + splice (F2 machine verbatim).
    let mut method = Vec::new();
    u2(&mut method, m.access);
    u2(&mut method, m.name_idx);
    u2(&mut method, m.desc_idx);
    u2(&mut method, 1); // attributes_count
    method.extend_from_slice(&code_attr);

    let mut out = Vec::with_capacity(bytes.len() + 64);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    u2(&mut out, pool.next); // new cp_count
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&bytes[layout.cp_end..m.start]);
    out.extend_from_slice(&method);
    out.extend_from_slice(&bytes[m.end..]);
    Ok(out)
}

// ---------------------------------------------------------------------------
// G4: invokestatic call-site retarget (docs/G4_SITE_PATCH_DESIGN.md §3,
// Variant R). Same-length CP-operand rewrite: scan a method's Code attribute
// for `invokestatic` (0xb8) instructions, resolve each 2-byte CP operand to
// its Methodref, and on a name match rewrite ONLY the operand to a new
// (append-only) Methodref. The replacement keeps the ORIGINAL descriptor, so
// the verifier-visible stack shape is unchanged: no branch fixups, no
// exception-table edits, no StackMapTable deltas, no method-size growth.
// ---------------------------------------------------------------------------

/// What [`retarget_invokestatic`] found/did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetargetOutcome {
    /// `sites` call sites were rewritten from `from` to the (appended) `to`
    /// Methodref.
    Retargeted { sites: usize },
    /// Every call site matching `to` already points there — the idempotent
    /// re-sight path (engine `is_invokestatic_to` pattern; module PATCHED-swap
    /// convention). Nothing was modified.
    AlreadyPatched { sites: usize },
    /// The method carries no invokestatic matching either spec (nothing to do;
    /// for the demonstrator this means the ASM patch shape changed and the
    /// caller must keep the unretargeted bytes).
    NotFound,
}

/// Locate the target method's Code attribute; returns `(code_start,
/// code_len)` — absolute offset/length of the bytecode array within `bytes`.
fn find_code_attr(bytes: &[u8], pool: &Pool, m: &Method) -> Option<(usize, usize)> {
    let mut p = m.start.checked_add(6)?; // access(2) name(2) desc(2) -> attrs_count
    let count = usize::from(u16_at(bytes, p)?);
    p = p.checked_add(2)?;
    for _ in 0..count {
        let name_idx = u16_at(bytes, p)?;
        let len = u32_at(bytes, p.checked_add(2)?)? as usize;
        let data = p.checked_add(6)?;
        if pool.utf8_value(name_idx).as_deref() == Some("Code") {
            // Code body: max_stack(2) max_locals(2) code_length(4) code[..]
            let code_len = u32_at(bytes, data.checked_add(4)?)? as usize;
            let code_start = data.checked_add(8)?;
            return Some((code_start, code_len));
        }
        p = data.checked_add(len)?;
    }
    None
}

/// Operand-byte count AFTER the opcode byte for `op`. `code`/`pc` are needed
/// only by the variable-length ops (tableswitch/lookupswitch/wide). Unknown
/// opcodes are an ERROR (fail closed) — a walk that cannot be proven exact
/// must never rewrite operands (a mis-stepped scan would corrupt instructions).
fn opcode_extra(op: u8, code: &[u8], pc: usize) -> Result<usize, String> {
    // `need(from, n)`: code[from..from+n] must exist.
    let need = |from: usize, n: usize| -> Result<(), String> {
        if from.checked_add(n).map(|e| e <= code.len()).unwrap_or(false) {
            Ok(())
        } else {
            Err("truncated code (operand past end)".into())
        }
    };
    let fixed: usize = match op {
        // No-operand opcodes: const/load_n/store_n/array ops, pop..swap,
        // arithmetic 0x60..0x83, conversions/comparisons, returns, misc.
        0x00..=0x0f
        | 0x1a..=0x35 // _n loads (1a..2d) + array loads (2e..35)
        | 0x3b..=0x56 // _n stores (3b..4e) + array stores (4f..56)
        | 0x57..=0x5f // pop, pop2, dup*, swap
        | 0x60..=0x83 // iadd..lxor
        | 0x85..=0x93 // i2l..i2s
        | 0x94..=0x98 // lcmp..dcmpg
        | 0xac..=0xb1 // ireturn..return
        | 0xbe | 0xbf // arraylength, athrow
        | 0xc2 | 0xc3 => 0, // monitorenter/exit
        // One operand byte: bipush, ldc, generic loads/stores (0x15..0x19,
        // 0x36..0x3a), ret, newarray.
        0x10 | 0x12 | 0x15..=0x19 | 0x36..=0x3a | 0xa9 | 0xbc => 1,
        // Two operand bytes: sipush, ldc_w/ldc2_w, branches + goto/jsr
        // (0x99..0xa8), iinc, field ops, invoke* (non-w/interface), new,
        // anewarray, checkcast, instanceof, ifnull/nonnull.
        0x11 | 0x13 | 0x14 | 0x84 | 0x99..=0xa8 | 0xb2..=0xb8 | 0xbb | 0xbd | 0xc0 | 0xc1
        | 0xc6 | 0xc7 => 2,
        // Four operand bytes: invokeinterface/invokedynamic (count bytes
        // included), goto_w/jsr_w.
        0xb9 | 0xba | 0xc8 | 0xc9 => 4,
        // multianewarray: index + dimensions.
        0xc5 => 3,
        0xaa => {
            // tableswitch: pad to 4-alignment RELATIVE TO THE CODE START
            // (JVMS: defaultbyte1 begins at an address that is a multiple of
            // four bytes from the start of the code array), then default(4)
            // low(4) high(4) + 4*(high-low+1).
            let pad = (4 - ((pc + 1) & 3)) & 3;
            let d = pc + 1 + pad;
            need(d, 12)?;
            let low = i32::from_be_bytes([code[d + 4], code[d + 5], code[d + 6], code[d + 7]]) as i64;
            let high = i32::from_be_bytes([code[d + 8], code[d + 9], code[d + 10], code[d + 11]]) as i64;
            if high < low {
                return Err("tableswitch high < low".into());
            }
            let n = (high - low + 1) as usize;
            // 4 * n with overflow discipline (a hostile code array must err,
            // not wrap).
            let table = n.checked_mul(4).ok_or("tableswitch table overflow")?;
            let extra = (pad + 12).checked_add(table).ok_or("tableswitch overflow")?;
            need(pc + 1, extra)?;
            return Ok(extra);
        }
        0xab => {
            // lookupswitch: pad, then default(4) npairs(4) + 8*npairs.
            let pad = (4 - ((pc + 1) & 3)) & 3;
            let d = pc + 1 + pad;
            need(d, 8)?;
            let npairs =
                u32::from_be_bytes([code[d + 4], code[d + 5], code[d + 6], code[d + 7]]) as usize;
            let pairs = npairs.checked_mul(8).ok_or("lookupswitch overflow")?;
            let extra = (pad + 8).checked_add(pairs).ok_or("lookupswitch overflow")?;
            need(pc + 1, extra)?;
            return Ok(extra);
        }
        0xc4 => {
            // wide: modded opcode next; iinc carries an extra const(2).
            need(pc + 1, 1)?;
            let modded = code[pc + 1];
            let extra = if modded == 0x84 { 5 } else { 3 };
            need(pc + 1, extra)?;
            return Ok(extra);
        }
        _ => return Err(format!("unknown opcode 0x{op:02x} in code walk")),
    };
    need(pc + 1, fixed)?;
    Ok(fixed)
}

/// Scan a bytecode array for `invokestatic` instructions. Returns
/// `(absolute_offset_of_opcode, cp_operand)` pairs. Bounded and panic-free:
/// every step is width-checked against the code array (audit A4 style — this
/// runs on ClassFileLoadHook-delivered bytes).
fn scan_invokestatics(code: &[u8], code_start: usize) -> Result<Vec<(usize, u16)>, String> {
    let mut out = Vec::new();
    let mut pc = 0usize;
    while pc < code.len() {
        let op = code[pc];
        if op == 0xb8 {
            let b = code
                .get(pc + 1..pc + 3)
                .ok_or_else(|| "invokestatic operand truncated".to_string())?;
            out.push((code_start + pc, u16::from_be_bytes([b[0], b[1]])));
            pc += 3;
            continue;
        }
        let extra = opcode_extra(op, code, pc)?;
        pc = pc
            .checked_add(1 + extra)
            .ok_or_else(|| "code walk overflow".to_string())?;
        if pc > code.len() {
            return Err("truncated code (walk past end)".into());
        }
    }
    Ok(out)
}

/// Variant R retarget (G4 §3): rewrite every `invokestatic` call site in
/// `method_name`/`method_desc` that currently resolves to `from`
/// (class, method, descriptor) so it resolves to `to` (same descriptor —
/// the caller's contract; a mismatching `to` descriptor would change the
/// verifier-visible stack shape and is a caller bug).
///
/// Returns the (possibly new) class bytes plus the outcome. Idempotency
/// (G4 §3): a site already resolving to `to` is counted and LEFT ALONE —
/// re-sighting patched bytes yields [`RetargetOutcome::AlreadyPatched`] and
/// the ORIGINAL bytes back, never a double patch. `NotFound` also returns
/// the original bytes and must not grow the constant pool.
///
/// CP growth is append-only via [`Pool::method_ref`] (dedup); existing
/// indices stay valid. The 64K saturation guard is honored loudly.
///
/// # Panic-free contract
/// Runs on the quiet activation worker over hook-delivered bytes: every
/// read is bounds-checked; a truncated/hostile class yields `Err`, never a
/// panic (the panic-across-JNI discipline, G4 §6).
pub fn retarget_invokestatic(
    bytes: &[u8],
    method_name: &str,
    method_desc: &str,
    from: (&str, &str, &str),
    to: (&str, &str, &str),
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    if from.2 != to.2 {
        return Err(format!(
            "retarget would change descriptor {} -> {} (verifier-visible stack shape must stay identical)",
            from.2, to.2
        ));
    }
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    let mut pool = layout.pool;
    // Find-only probes: a method whose name/desc utf8 entries are absent
    // cannot exist; NotFound must not mutate the pool.
    let Some(name_idx) = pool.find_utf8(method_name) else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let Some(desc_idx) = pool.find_utf8(method_desc) else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
        .ok_or_else(|| format!("method {method_name}{method_desc} not found"))?;
    let (code_start, code_len) = find_code_attr(bytes, &pool, &m)
        .ok_or_else(|| format!("method {method_name}{method_desc} has no Code attribute"))?;
    let code_end = code_start
        .checked_add(code_len)
        .ok_or_else(|| "code length overflow".to_string())?;
    let code = bytes
        .get(code_start..code_end)
        .ok_or_else(|| "code region truncated".to_string())?;
    let sites = scan_invokestatics(code, code_start)?;

    // Classify every call site BY NAME (never by offset — CP indexes shift
    // between ASM runs, G4 §9).
    let to_triple = (to.0.to_string(), to.1.to_string(), to.2.to_string());
    let from_triple = (from.0.to_string(), from.1.to_string(), from.2.to_string());
    let mut rewrite: Vec<usize> = Vec::new(); // absolute offsets of the 2-byte operands
    let mut already = 0usize;
    for (op_pc, cp_idx) in sites {
        match pool.methodref_parts(cp_idx) {
            Some(parts) if parts == to_triple => already += 1,
            Some(parts) if parts == from_triple => rewrite.push(op_pc + 1),
            _ => {}
        }
    }
    if rewrite.is_empty() {
        return Ok((bytes.to_vec(), if already > 0 {
            RetargetOutcome::AlreadyPatched { sites: already }
        } else {
            RetargetOutcome::NotFound
        }));
    }

    // Append (or reuse) the Methodref for `to` — append-only, dedup.
    let new_idx = pool.method_ref(to.0, to.1, to.2);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for retarget ref".into());
    }

    // Splice: header + grown pool + tail (everything from cp_end on) with
    // the matched operands rewritten. All operand offsets are >= cp_end
    // (the Code attribute lives in the method table, after the pool), so
    // the rewrite applies to the tail copy at (offset - cp_end).
    let mut tail = bytes[layout.cp_end..].to_vec();
    let want = new_idx.to_be_bytes();
    for &op_off in &rewrite {
        let rel = op_off - layout.cp_end;
        if rel + 1 >= tail.len() {
            return Err("retarget operand outside class tail (corrupt layout?)".into());
        }
        tail[rel] = want[0];
        tail[rel + 1] = want[1];
    }
    let mut out = Vec::with_capacity(bytes.len() + 64);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    out.extend_from_slice(&pool.next.to_be_bytes()); // new cp_count
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&tail);
    Ok((out, RetargetOutcome::Retargeted { sites: rewrite.len() }))
}

#[test]
fn dbg_parse() {
    let bytes = include_bytes!("../tests/fixtures/SingleUserAreaMap.class");
    let cp_count = u16::from_be_bytes([bytes[8], bytes[9]]);
    eprintln!("file len {} cp_count {cp_count}", bytes.len());
    let (pool, end) = crate::classfile::Pool::parse(bytes, 10, cp_count).unwrap();
    eprintln!("parsed {} entries, next={}, cp_end={end}", pool.entries.len(), pool.next);
    let mut p = end + 6;
    let iface = u16::from_be_bytes([bytes[p], bytes[p+1]]);
    eprintln!("p={p} iface={iface}");
    p += 2 + 2 * iface as usize;
    eprintln!("after iface p={p}");
    let fields = u16::from_be_bytes([bytes[p], bytes[p+1]]);
    eprintln!("fields={fields}");
}

// ---------------------------------------------------------------------------
// PALETTED-DEMUX (S7-131, ARCH-ATTACK lever #1 — the owner's top-1 function).
//
// Target: `net/minecraft/world/level/chunk/PalettedContainer.get(int)` — the
// top-1 kernel JVM-Java leaf across all X150K profiles (4.20% @10k smoke,
// 3.62% @fp4, 3.3% @150k prime; + readPalette 0.8% + SimpleBitStorage.get
// 0.8% = the ~4.9% "palette lane"). Operationalization of the owner's
// x150000 bar: the function must DISAPPEAR from the profile.
//
// Shape: three bytecode edits applied at FIRST CLASS LOAD (field injection
// changes the class shape, so the retransform path is unavailable — the byte
// hook serves the patched bytes at define time; if the hook is not READY
// before the class loads, the module fails closed to vanilla):
//
//   1. FIELD INJECTION — 4 public instance fields appended to the field
//      table (zero-init by the JVM; no <init> changes needed):
//        crusstySnap    : [Ljava/lang/Object;  PUBLIC VOLATILE TRANSIENT
//        crusstySnapGen : I                    PUBLIC VOLATILE
//        crusstyGen     : I                    PUBLIC VOLATILE
//        crusstyMiss    : I                    PUBLIC (plain; benign races)
//      crusstySnap holds { int[] demux, Object[] vals } (a 2-slot holder so
//      the pair is consistent); crusstyGen is the write-epoch counter
//      (odd = mutation in flight); crusstySnapGen is the published
//      snapshot's build epoch; crusstyMiss is the per-container slow-read
//      heat counter driving lazy materialization.
//
//   2. get(int) BODY SWAP — straight-line fast path (gen/snap checks +
//      vals[demux[index]]) with a fallback invokestatic into
//      PalettedContainerOps.get(self, index) which reproduces the vanilla
//      body EXACTLY (public API: data/storage()/palette()/moonrise
//      fast-palette) and lazily materializes the snapshot. ONE hand-built
//      full_frame StackMapTable entry at the branch target (locals: this,
//      int, int, Object[]).
//
//   3. MUTATOR GUARDS — getAndSet(int,T) and set(int,T) (the two private
//      bit-level write entry points; every public write path funnels into
//      them) gain a PROLOGUE (snap = null; gen++ → odd) and an EPILOGUE
//      (gen++ → even; Ops.onWrite(self) → refcount release). Both bodies
//      are branch-free with no exception table (verified fail-closed in the
//      patcher), so the original code is copied VERBATIM (cp indices and
//      relative offsets unchanged) and the epilogue lands before the single
//      return opcode. onResize/updateData/read stay UNPATCHED: they either
//      run inside the guarded odd-epoch window (resize during idFor) or are
//      server-never (read(FriendlyByteBuf) is the client chunk-receive
//      path; a dedicated server never executes it).
//
// RACE PROTOCOL (single-writer-per-container is guaranteed by the kernel's
// own moonrise region-lock discipline — the same invariant that licenses
// the unsynchronized getAndSetUnchecked):
//   - writer: snap=null, gen++ (odd), ...mutate..., gen++ (even) — a reader
//     or materializer seeing odd gen or a snapGen mismatch takes the
//     vanilla path (never a torn snapshot).
//   - materializer probes only at stable EVEN gen, re-checks gen+data
//     after the probe, publishes snap BEFORE snapGen (snapGen last, both
//     volatile) — readers validate snapGen == gen at read time, so a
//     snapshot is used iff no write epoch started since its build.
//   - a fast read passing the gen check while a write is in flight would
//     have raced in vanilla too (unsynchronized get on in-place bit
//     writes) — parity of observable windows, not stronger.
// Refcount: LIVE counts published snapshots; prologue nulls snap on first
// write, the epilogue's Ops.onWrite releases the count (snapGen = -1 marks
// the uncounted state). gen wraps at 2^31 writes per container —
// unreachable in any bench window (documented).
//
// Fail-closed matrix: hook not READY at class load → vanilla; kernel shape
// mismatch (any expected method/field absent) → Err → vanilla; Ops class
// missing at first get execution → impossible (READY requires Ops defined).
// patch(patch(x)) == patch(x) via the crusstySnapGen pool probe.
//
// RUNTIME PATH NOTE (S7-131): the JVM verifier rejected the hand-built
// first-frame offset convention (StackMapTable bad offset), so the RUNTIME
// serves the ASM COMPUTE_FRAMES image produced by
// paletted/tools/PalettedPatchTool.java at BUILD time
// (paletted/build/PalettedContainer.patched.class, embedded in src/paletted.rs
// behind a fingerprint gate). THIS rust patcher remains as the offline
// diagnostic: it re-validates the cp/field-splice mechanics and the
// mutator-body contracts on the real fixture (paletted_patch_roundtrip).
// Its get/mutator bodies encode protocol v1 (snap=null prologue + snapGen=gen
// gate); the ASM runtime image is protocol v2 (onMutateStart release +
// snapGen==gen+1 gate, initial-state-safe refcount).
// ---------------------------------------------------------------------------

pub const PALETTED_CLASS: &str = "net/minecraft/world/level/chunk/PalettedContainer";
pub const PALETTED_OPS_CLASS: &str = "net/minecraft/world/level/chunk/PalettedContainerOps";

const F_SNAP: &str = "crusstySnap";
const F_SNAPGEN: &str = "crusstySnapGen";
const F_GEN: &str = "crusstyGen";
const F_MISS: &str = "crusstyMiss";
const DESC_OBJ_ARRAY: &str = "[Ljava/lang/Object;";
const DESC_INT_ARRAY: &str = "[I";

const ACC_PUBLIC: u16 = 0x0001;
const ACC_VOLATILE: u16 = 0x0040;
const ACC_TRANSIENT: u16 = 0x0080;

/// The four injected (name, descriptor, access) tuples.
const INJECTED_FIELDS: [(&str, &str, u16); 4] = [
    (F_SNAP, DESC_OBJ_ARRAY, ACC_PUBLIC | ACC_VOLATILE | ACC_TRANSIENT),
    (F_SNAPGEN, "I", ACC_PUBLIC | ACC_VOLATILE),
    (F_GEN, "I", ACC_PUBLIC | ACC_VOLATILE),
    (F_MISS, "I", ACC_PUBLIC),
];

/// Whole-container patch: inject fields + swap get(int) + guard both
/// mutators, in a single append-only pass. Idempotent.
pub fn patch_paletted_container(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout")?;
    let this_name = this_class_name(&layout).ok_or("cannot resolve this_class name")?;
    if this_name != PALETTED_CLASS {
        return Err(format!("unexpected class {this_name}"));
    }
    let mut pool = layout.pool;

    // Idempotency probe: a pool that already carries the injected-field
    // names is already patched — return the bytes unchanged.
    if pool.find_utf8(F_SNAPGEN).is_some() {
        return Ok(bytes.to_vec());
    }

    // Fail-closed probes BEFORE any mutation: every expected method must
    // exist with the exact kernel descriptor.
    let get_name = pool.find_utf8("get").ok_or("get name utf8 not found")?;
    let get_desc = pool
        .find_utf8("(I)Ljava/lang/Object;")
        .ok_or("get(I) desc utf8 not found")?;
    let gas_name = pool.find_utf8("getAndSet").ok_or("getAndSet name utf8 not found")?;
    let gas_desc = pool
        .find_utf8("(ILjava/lang/Object;)Ljava/lang/Object;")
        .ok_or("getAndSet(I,T) desc utf8 not found")?;
    let set_name = pool.find_utf8("set").ok_or("set name utf8 not found")?;
    let set_desc = pool
        .find_utf8("(ILjava/lang/Object;)V")
        .ok_or("set(I,T) desc utf8 not found")?;
    let m_get = find_method(bytes, layout.methods_start, get_name, get_desc)
        .ok_or("get(I)Ljava/lang/Object; not found")?;
    let m_gas = find_method(bytes, layout.methods_start, gas_name, gas_desc)
        .ok_or("getAndSet(ILjava/lang/Object;)Ljava/lang/Object; not found")?;
    let m_set = find_method(bytes, layout.methods_start, set_name, set_desc)
        .ok_or("set(ILjava/lang/Object;)V not found")?;

    // ---- constant pool additions (append-only) ----
    let mut f_refs = [0u16; 4];
    for (i, (name, desc, _)) in INJECTED_FIELDS.iter().enumerate() {
        f_refs[i] = pool.field_ref(PALETTED_CLASS, name, desc);
    }
    let ops_get = pool.method_ref(
        PALETTED_OPS_CLASS,
        "get",
        &format!("(L{PALETTED_CLASS};I)Ljava/lang/Object;"),
    );
    let ops_onwrite = pool.method_ref(
        PALETTED_OPS_CLASS,
        "onWrite",
        &format!("(L{PALETTED_CLASS};)V"),
    );
    let u8_int_array = pool.utf8(DESC_INT_ARRAY);
    let cls_int_array = pool.class_of(u8_int_array);
    let u8_obj_array = pool.utf8(DESC_OBJ_ARRAY);
    let cls_obj_array = pool.class_of(u8_obj_array);
    if pool.next > u16::MAX - 32 {
        return Err("constant pool overflow: no index space left for PALETTED refs".into());
    }

    // ---- get(int): fast path + Ops fallback ----
    let this_class_utf8 = pool.utf8(PALETTED_CLASS);
    let this_class_cp = pool.class_of(this_class_utf8);
    let attr_code = pool.utf8("Code");
    let attr_smt = pool.utf8("StackMapTable");
    let get_code = build_paletted_get_body(&pool, f_refs[1], f_refs[2], f_refs[0], ops_get, cls_int_array, cls_obj_array, this_class_cp, attr_code, attr_smt);
    let get_method = build_method_entry(&m_get, &get_code);

    // ---- mutators: prologue + verbatim + epilogue ----
    // getAndSet returns T (areturn 0xb0); set returns void (return 0xb1).
    let gas_code = guard_mutator_body(bytes, &pool, attr_code, &m_gas, f_refs[0], f_refs[2], ops_onwrite, 0xb0)?;
    let gas_method = build_method_entry(&m_gas, &gas_code);
    let set_code = guard_mutator_body(bytes, &pool, attr_code, &m_set, f_refs[0], f_refs[2], ops_onwrite, 0xb1)?;
    let set_method = build_method_entry(&m_set, &set_code);

    // ---- field table: original entries + 4 injected field_info blocks ----
    let mut fields_out = Vec::with_capacity(64);
    let orig_fields_count = u16_at(bytes, layout.fields_start).ok_or("fields_count oob")?;
    fields_out.extend_from_slice(&(orig_fields_count.saturating_add(4)).to_be_bytes());
    fields_out.extend_from_slice(&bytes[layout.fields_start + 2..layout.methods_start]);
    for (i, (name, desc, access)) in INJECTED_FIELDS.iter().enumerate() {
        let _ = i;
        let n = pool.find_utf8(name).ok_or("field name utf8 missing")?;
        let d = pool.find_utf8(desc).ok_or("field desc utf8 missing")?;
        fields_out.extend_from_slice(&access.to_be_bytes());
        fields_out.extend_from_slice(&n.to_be_bytes());
        fields_out.extend_from_slice(&d.to_be_bytes());
        fields_out.extend_from_slice(&0u16.to_be_bytes()); // attributes_count
    }

    // ---- assemble: header + new cp + tail with fields & methods spliced ----
    let mut out = Vec::with_capacity(bytes.len() + 512);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    out.extend_from_slice(&pool.next.to_be_bytes()); // new cp_count
    out.extend_from_slice(&pool.serialize());
    // this_class/super_class/interfaces + field table (rewritten count)
    out.extend_from_slice(&bytes[layout.cp_end..layout.fields_start]);
    out.extend_from_slice(&fields_out);
    // method table with the three bodies replaced (order-agnostic splice)
    let mut edits: [(usize, usize, &[u8]); 3] = [
        (m_get.start, m_get.end, &get_method),
        (m_gas.start, m_gas.end, &gas_method),
        (m_set.start, m_set.end, &set_method),
    ];
    edits.sort_by_key(|e| e.0);
    if edits[0].1 > edits[1].0 || edits[1].1 > edits[2].0 {
        return Err("method ranges overlap".into());
    }
    let mut cursor = layout.methods_start;
    for (s, e, repl) in edits.iter() {
        out.extend_from_slice(&bytes[cursor..*s]);
        out.extend_from_slice(repl);
        cursor = *e;
    }
    out.extend_from_slice(&bytes[cursor..]);
    Ok(out)
}

/// Rebuild a method_info block: original access/name/desc + a fresh Code
/// attribute (the only attribute kept — these methods carry no others that
/// the verifier needs; the patcher fails closed when they do).
fn build_method_entry(m: &Method, full_code_attr: &[u8]) -> Vec<u8> {
    let mut method = Vec::with_capacity(full_code_attr.len() + 8);
    method.extend_from_slice(&m.access.to_be_bytes());
    method.extend_from_slice(&m.name_idx.to_be_bytes());
    method.extend_from_slice(&m.desc_idx.to_be_bytes());
    method.extend_from_slice(&1u16.to_be_bytes()); // attributes_count = Code only
    method.extend_from_slice(full_code_attr);
    method
}

/// JVM instruction-length walk: true if the body contains any control-flow
/// opcode (branches, switches, athrow, wide, jsr). Lengths per JVMS 6.5 —
/// operand bytes are never misread as opcodes.
fn code_has_branches(code: &[u8]) -> bool {
    let mut p = 0usize;
    while p < code.len() {
        let op = code[p];
        let len = match op {
            0x10 | 0x12 => 2,                       // bipush, ldc
            0x11 => 3,                              // sipush
            0x13..=0x14 => 3,                       // ldc_w, ldc2_w
            0x84 => 3,                              // iinc
            0x99..=0xc7 => 3,                       // if*, goto, jsr, null-branch
            0xb2..=0xb8 => 3,                       // get/putstatic, get/putfield, invokes
            0xba => 5,                              // invokedynamic
            0xb9 => 5,                              // invokeinterface
            0xbb..=0xc1 => 3,                       // new, anewarray, checkcast, instanceof
            0xc5 => 4,                              // multianewarray
            0xc8..=0xc9 => 5,                       // goto_w, jsr_w
            0xaa | 0xab => return true,             // tableswitch, lookupswitch
            0xc4 => return true,                    // wide
            0xbf => return true,                    // athrow
            _ => 1,                                 // everything else: 1 byte
        };
        p += len;
    }
    false
}

/// Build the guarded mutator Code attribute: prologue + verbatim original
/// code (minus its final return) + epilogue + return. Fail-closed: the
/// original body must be branch-free, exception-table-free and end with the
/// expected single return opcode.
fn guard_mutator_body(
    bytes: &[u8],
    pool: &Pool,
    attr_code: u16,
    m: &Method,
    f_snap: u16,
    f_gen: u16,
    ops_onwrite: u16,
    ret_opcode: u8,
) -> Result<Vec<u8>, String> {
    let (orig_max_stack, orig_max_locals, orig_code) = parse_code_attr(bytes, pool, m.start, m.end)?;
    if code_has_exception_table(bytes, pool, m.start, m.end)? {
        return Err("mutator has exception table".into());
    }
    // NOTE: debug attributes (LineNumberTable etc.) inside Code are DROPPED
    // (the replacement body carries none) — the verifier does not need them;
    // their line offsets would be stale under the prologue anyway.
    // Branch scan: walk the INSTRUCTION stream (opcode-true lengths — a raw
    // byte scan would false-positive on cp operand bytes) and fail closed on
    // any jump/switch/athrow/wide (branch-free bodies need no stackmap and
    // no offset fixups; the prologue/epilogue insert stays trivial).
    if code_has_branches(orig_code) {
        return Err("mutator body has branch/throw opcodes".into());
    }
    let last = *orig_code.last().ok_or("mutator body empty")?;
    if last != ret_opcode {
        return Err(format!(
            "mutator body does not end with expected return 0x{ret_opcode:02x} (got 0x{last:02x})"
        ));
    }
    let body = &orig_code[..orig_code.len() - 1];

    let mut code = Vec::with_capacity(orig_code.len() + 40);
    // PROLOGUE: snap = null; gen++ (even -> odd).
    code.push(0x2a); // aload_0
    code.push(0x01); // aconst_null
    code.push(0xb5); // putfield crusstySnap
    code.extend_from_slice(&f_snap.to_be_bytes());
    code.push(0x2a); // aload_0
    code.push(0x59); // dup
    code.push(0xb4); // getfield crusstyGen
    code.extend_from_slice(&f_gen.to_be_bytes());
    code.push(0x04); // iconst_1
    code.push(0x60); // iadd
    code.push(0xb5); // putfield crusstyGen
    code.extend_from_slice(&f_gen.to_be_bytes());
    // ORIGINAL body (verbatim, minus the final return).
    code.extend_from_slice(body);
    // EPILOGUE: gen++ (odd -> even); Ops.onWrite(self).
    code.push(0x2a); // aload_0
    code.push(0x59); // dup
    code.push(0xb4); // getfield crusstyGen
    code.extend_from_slice(&f_gen.to_be_bytes());
    code.push(0x04); // iconst_1
    code.push(0x60); // iadd
    code.push(0xb5); // putfield crusstyGen
    code.extend_from_slice(&f_gen.to_be_bytes());
    code.push(0x2a); // aload_0
    code.push(0xb8); // invokestatic PalettedContainerOps.onWrite
    code.extend_from_slice(&ops_onwrite.to_be_bytes());
    code.push(ret_opcode);

    Ok(assemble_code_attr(attr_code, &code, orig_max_stack.saturating_add(2), orig_max_locals, None))
}

/// Build the get(int) full Code attribute: fast demux path + Ops fallback
/// with a single full_frame stackmap entry at the fallback target.
fn build_paletted_get_body(
    pool: &Pool,
    f_snapgen: u16,
    f_gen: u16,
    f_snap: u16,
    ops_get: u16,
    cls_int_array: u16,
    cls_obj_array: u16,
    this_class_cp: u16,
    attr_code: u16,
    attr_smt: u16,
) -> Vec<u8> {
    const OPS_TARGET: i32 = 39; // byte offset of the fallback inside the new body
    let u2 = |out: &mut Vec<u8>, v: u16| out.extend_from_slice(&v.to_be_bytes());
    let mut code = Vec::with_capacity(48);
    // locals: 0=this, 1=index, 2=snapGen, 3=snap
    code.push(0x2a); // aload_0
    code.push(0xb4); // getfield crusstySnapGen
    u2(&mut code, f_snapgen);
    code.push(0x3d); // istore_2
    code.push(0x2a); // aload_0
    code.push(0xb4); // getfield crusstySnap
    u2(&mut code, f_snap);
    code.push(0x4e); // astore_3
    code.push(0x1c); // iload_2
    code.push(0x2a); // aload_0
    code.push(0xb4); // getfield crusstyGen
    u2(&mut code, f_gen);
    code.push(0xa0); // if_icmpne -> OPS (3B at 15..17)
    u2(&mut code, (OPS_TARGET - 15) as u16);
    code.push(0x2d); // aload_3
    code.push(0xc6); // ifnull -> OPS (3B at 19..21)
    u2(&mut code, (OPS_TARGET - 19) as u16);
    // fast: vals[demux[index]]
    code.push(0x2d); // aload_3
    code.push(0x03); // iconst_0
    code.push(0x32); // aaload
    code.push(0xc0); // checkcast [I
    u2(&mut code, cls_int_array);
    code.push(0x1b); // iload_1
    code.push(0x2e); // iaload
    code.push(0x2d); // aload_3
    code.push(0x04); // iconst_1
    code.push(0x32); // aaload
    code.push(0xc0); // checkcast [Ljava/lang/Object;
    u2(&mut code, cls_obj_array);
    code.push(0x5f); // swap
    code.push(0x32); // aaload
    code.push(0xb0); // areturn
    debug_assert_eq!(code.len(), OPS_TARGET as usize, "fast path must end at the fallback target");
    // OPS: fallback -> PalettedContainerOps.get(self, index)
    code.push(0x2a); // aload_0
    code.push(0x1b); // iload_1
    code.push(0xb8); // invokestatic PalettedContainerOps.get
    u2(&mut code, ops_get);
    code.push(0xb0); // areturn

    // StackMapTable: one full_frame at offset 39 (offset_delta = 39 - (-1) = 40):
    //   locals: [PalettedContainer, int, int, Object[]], stack: []
    let this_class = this_class_cp;
    let mut frames = Vec::with_capacity(24);
    frames.push(0xff); // full_frame
    // First-frame offset convention (empirical, matches the kernel's own
    // updateData frame: branch target 26 <-> same_frame type 26):
    // offset(0) = offset_delta(0), i.e. the virtual initial frame sits at
    // offset 0, not -1.
    u2(&mut frames, OPS_TARGET as u16);
    u2(&mut frames, 4); // number_of_locals
    frames.push(0x07); // OBJECT
    u2(&mut frames, this_class);
    frames.push(0x01); // INTEGER (index)
    frames.push(0x01); // INTEGER (snapGen local)
    frames.push(0x07); // OBJECT
    u2(&mut frames, cls_obj_array);
    u2(&mut frames, 0); // number_of_stack_items

    assemble_code_attr(attr_code, &code, 3, 4, Some((attr_smt, &frames)))
}

/// Emit a FULL Code attribute: u2("Code" name idx) + u32 len + body
/// [max_stack, max_locals, code_len, code, exception_table_length=0,
///  attributes_count (0 or 1 = StackMapTable)].
fn assemble_code_attr(
    attr_code: u16,
    code: &[u8],
    max_stack: u16,
    max_locals: u16,
    stackmap: Option<(u16, &[u8])>,
) -> Vec<u8> {
    let smt_payload_len = stackmap.as_ref().map(|(_, f)| 2 + 4 + f.len()).unwrap_or(0);
    let body_len = 2 + 2 + 4 + code.len() + 2 + 2 + smt_payload_len;
    let mut out = Vec::with_capacity(6 + body_len);
    out.extend_from_slice(&attr_code.to_be_bytes());
    out.extend_from_slice(&(body_len as u32).to_be_bytes());
    out.extend_from_slice(&max_stack.to_be_bytes());
    out.extend_from_slice(&max_locals.to_be_bytes());
    out.extend_from_slice(&(code.len() as u32).to_be_bytes());
    out.extend_from_slice(code);
    out.extend_from_slice(&0u16.to_be_bytes()); // exception_table_length = 0
    match stackmap {
        None => out.extend_from_slice(&0u16.to_be_bytes()),
        Some((name_idx, frames)) => {
            out.extend_from_slice(&1u16.to_be_bytes()); // attributes_count
            out.extend_from_slice(&name_idx.to_be_bytes());
            out.extend_from_slice(&(frames.len() as u32).to_be_bytes());
            out.extend_from_slice(frames);
        }
    }
    out
}

/// Walk a method_info block and extract its Code attribute body:
/// (max_stack, max_locals, code bytes). Resolves the attribute by NAME via
/// the pool (strict — no positional guessing).
fn parse_code_attr<'a>(
    bytes: &'a [u8],
    pool: &Pool,
    start: usize,
    _end: usize,
) -> Result<(u16, u16, &'a [u8]), String> {
    let mut p = start + 6; // access(2) name(2) desc(2)
    let attr_count = usize::from(u16_at(bytes, p).ok_or("method attr_count oob")?);
    p += 2;
    for _ in 0..attr_count {
        let name_idx = u16_at(bytes, p).ok_or("attr name oob")?;
        let len = u32_at(bytes, p + 2).ok_or("attr len oob")? as usize;
        let body = bytes.get(p + 6..p + 6 + len).ok_or("attr body oob")?;
        if pool.utf8_value(name_idx).as_deref() == Some("Code") {
            if body.len() < 8 {
                return Err("Code attribute truncated".into());
            }
            let max_stack = u16::from_be_bytes([body[0], body[1]]);
            let max_locals = u16::from_be_bytes([body[2], body[3]]);
            let code_len = u32::from_be_bytes([body[4], body[5], body[6], body[7]]) as usize;
            let code = body.get(8..8 + code_len).ok_or("Code code_len oob")?;
            return Ok((max_stack, max_locals, code));
        }
        p += 6 + len;
    }
    Err("no Code attribute found".into())
}

/// Strict mutator precondition: exception_table_length == 0 (debug attrs
/// inside Code are dropped by the rebuilt attribute).
fn code_has_exception_table(bytes: &[u8], pool: &Pool, start: usize, _end: usize) -> Result<bool, String> {
    let mut p = start + 6;
    let attr_count = usize::from(u16_at(bytes, p).ok_or("method attr_count oob")?);
    p += 2;
    for _ in 0..attr_count {
        let name_idx = u16_at(bytes, p).ok_or("attr name oob")?;
        let len = u32_at(bytes, p + 2).ok_or("attr len oob")? as usize;
        let body = bytes.get(p + 6..p + 6 + len).ok_or("attr body oob")?;
        if pool.utf8_value(name_idx).as_deref() == Some("Code") {
            if body.len() < 8 {
                return Err("Code attribute truncated".into());
            }
            let code_len = u32::from_be_bytes([body[4], body[5], body[6], body[7]]) as usize;
            let et = u16::from_be_bytes([body[8 + code_len], body[9 + code_len]]);
            let attrs_pos = 10 + code_len + 2 * et as usize;
            let extra = if attrs_pos + 2 <= body.len() {
                u16::from_be_bytes([body[attrs_pos], body[attrs_pos + 1]])
            } else {
                0
            };
            let _ = extra; // inner debug attrs (LNT/LVTT) are legal — dropped by the rebuilt body
            return Ok(et != 0);
        }
        p += 6 + len;
    }
    Err("no Code attribute found".into())
}

#[cfg(test)]
mod tests {
    use crate::classfile::*;

    const REAL: &[u8] = include_bytes!("../tests/fixtures/SingleUserAreaMap.class");

    #[test]
    fn patch_roundtrip() {
        let patched = patch_update(REAL).expect("patch");
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // header preserved from the source class (kernel is Java 21 = major 65)
        assert_eq!(patched[..8], REAL[..8]);

        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, cp_end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        assert!(cp_count > 100, "pool grew with appended entries");

        let this_idx = u16::from_be_bytes([patched[cp_end + 2], patched[cp_end + 3]]);
        let (_, tag, payload) = pool
            .entries
            .iter()
            .find(|(i, _, _)| *i == this_idx)
            .expect("this_class entry");
        assert_eq!(*tag, TAG_CLASS);
        let name_utf8 = u16::from_be_bytes([payload[0], payload[1]]);
        assert_eq!(pool.utf8_value(name_utf8).as_deref(), Some(MAP_CLASS));

        // walk members to the update method and check its Code attribute
        let mut p = cp_end + 6;
        let iface_count = usize::from(u16::from_be_bytes([patched[p], patched[p + 1]]));
        p += 2 + 2 * iface_count;
        let fields_count = usize::from(u16::from_be_bytes([patched[p], patched[p + 1]]));
        p += 2;
        for _ in 0..fields_count {
            p += 6;
            let attr_count = usize::from(u16::from_be_bytes([patched[p], patched[p + 1]]));
            p += 2;
            for _ in 0..attr_count {
                let len = u32::from_be_bytes([patched[p + 2], patched[p + 3], patched[p + 4], patched[p + 5]]);
                p += 6 + len as usize;
            }
        }
        let methods_count = usize::from(u16::from_be_bytes([patched[p], patched[p + 1]]));
        p += 2;
        let mut found_update = false;
        let mut found_code = false;
        for _ in 0..methods_count {
            let name_idx = u16::from_be_bytes([patched[p + 2], patched[p + 3]]);
            let desc_idx = u16::from_be_bytes([patched[p + 4], patched[p + 5]]);
            let attr_count = usize::from(u16::from_be_bytes([patched[p + 6], patched[p + 7]]));
            p += 8;
            let mut code_len = 0usize;
            let mut stackmap = false;
            for _ in 0..attr_count {
                let aname_idx = u16::from_be_bytes([patched[p], patched[p + 1]]);
                let alen = u32::from_be_bytes([patched[p + 2], patched[p + 3], patched[p + 4], patched[p + 5]]) as usize;
                if pool.utf8_value(aname_idx).as_deref() == Some("Code") {
                    code_len = u32::from_be_bytes([patched[p + 10], patched[p + 11], patched[p + 12], patched[p + 13]]) as usize;
                    // StackMapTable lives inside the Code attribute
                    let mut q = p + 14 + code_len; // after code bytes
                    let exc = usize::from(u16::from_be_bytes([patched[q], patched[q + 1]]));
                    q += 2 + 8 * exc;
                    let inner_attrs = usize::from(u16::from_be_bytes([patched[q], patched[q + 1]]));
                    q += 2;
                    for _ in 0..inner_attrs {
                        let ia_name = u16::from_be_bytes([patched[q], patched[q + 1]]);
                        let ia_len = u32::from_be_bytes([patched[q + 2], patched[q + 3], patched[q + 4], patched[q + 5]]) as usize;
                        if pool.utf8_value(ia_name).as_deref() == Some("StackMapTable") {
                            stackmap = true;
                        }
                        q += 6 + ia_len;
                    }
                }
                p += 6 + alen;
            }
            if pool.utf8_value(name_idx).as_deref() == Some("update")
                && pool.utf8_value(desc_idx).as_deref() == Some("(III)Z")
            {
                found_update = true;
                assert_eq!(code_len, 82, "patched update body is 82 bytes");
                assert!(stackmap, "update carries StackMapTable");
                found_code = true;
            }
        }
        assert!(found_update && found_code);

        let out = std::env::temp_dir().join("ccrussty_patched_SingleUserAreaMap.class");
        std::fs::write(&out, &patched).expect("dump patched class");
        eprintln!("wrote {} bytes to {}", patched.len(), out.display());
    }

    #[test]
    fn rejects_garbage() {
        let result = patch_update(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 52, 0, 5, 1, 2, 3, 4]);
        assert!(result.is_err());
    }

    // ---- G4: invokestatic retarget (Variant R, docs/G4_SITE_PATCH_DESIGN.md) ----

    /// The run descriptor `patch_update` emits for `SingleUserAreaMapOps.run`
    /// (computed in patch_update as `(L{MAP_CLASS};IIIIIILjava/lang/Object;)V`).
    const RUN_DESC: &str =
        "(Lca/spottedleaf/moonrise/common/misc/SingleUserAreaMap;IIIIIILjava/lang/Object;)V";
    const HELPER: &str = "crussty/test/BatchOpsHelper";

    /// Round-trip on the REAL fixture: patch_update's emitted body carries
    /// exactly one `invokestatic SingleUserAreaMapOps.run`, the retarget
    /// rewrites it to the helper, a second pass is a no-op (AlreadyPatched,
    /// original bytes back), and the patched bytes re-verify: pool grew by
    /// the appended Methodref, the Code attribute is unchanged in length
    /// (same-descriptor operand rewrite — zero StackMapTable/exception-table
    /// deltas by construction), and the operand now resolves to the helper.
    #[test]
    fn retarget_roundtrip_idempotent_and_verified() {
        let patched = patch_update(REAL).expect("patch");
        let to = (HELPER, "run", RUN_DESC);

        let (out1_bytes, out1) =
            retarget_invokestatic(&patched, "update", "(III)Z", (OPS_CLASS, "run", RUN_DESC), to)
                .expect("retarget");
        assert_eq!(out1, RetargetOutcome::Retargeted { sites: 1 });
        assert_ne!(out1_bytes, patched, "a real retarget must change the bytes");

        // Idempotency: re-sighting the retargeted bytes is a no-op and hands
        // the input bytes back unchanged.
        let (out2_bytes, out2) =
            retarget_invokestatic(&out1_bytes, "update", "(III)Z", (OPS_CLASS, "run", RUN_DESC), to)
                .expect("second pass");
        assert_eq!(out2, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(out2_bytes, out1_bytes, "AlreadyPatched must not touch the bytes");

        // Byte-level verification of the retargeted class: parse it, walk to
        // update's Code, and confirm the single invokestatic now resolves to
        // the helper while the code array length is unchanged (82 bytes).
        let layout = parse_layout(&out1_bytes).expect("re-parse retargeted class");
        let name_idx = layout.pool.find_utf8("update").expect("update name kept");
        let desc_idx = layout.pool.find_utf8("(III)Z").expect("desc kept");
        let m = find_method(&out1_bytes, layout.methods_start, name_idx, desc_idx).expect("update found");
        let (code_start, code_len) = find_code_attr(&out1_bytes, &layout.pool, &m).expect("code attr");
        assert_eq!(code_len, 82, "same-descriptor retarget must not change code length");
        let sites = scan_invokestatics(
            &out1_bytes[code_start..code_start + code_len],
            code_start,
        )
        .expect("scan");
        assert_eq!(sites.len(), 2, "body carries toString + run call sites");
        let targets: Vec<_> = sites
            .iter()
            .map(|(_, idx)| layout.pool.methodref_parts(*idx).expect("resolve"))
            .collect();
        assert!(targets.contains(&(
            HELPER.to_string(),
            "run".to_string(),
            RUN_DESC.to_string()
        )));
        assert!(
            !targets.iter().any(|t| t.0 == OPS_CLASS),
            "no call site may still reference the original after a full retarget"
        );
        // The other invokestatic (Integer.toString) must be untouched.
        assert!(targets
            .iter()
            .any(|t| t.0 == "java/lang/Integer" && t.1 == "toString"));
    }

    #[test]
    fn retarget_wrong_site_not_found() {
        let patched = patch_update(REAL).expect("patch");
        // A from-triple that appears nowhere in the body -> NotFound (and the
        // pool must NOT have grown: NotFound never mutates).
        let before = parse_layout(&patched).unwrap().pool.next;
        let (bytes_back, out) = retarget_invokestatic(
            &patched,
            "update",
            "(III)Z",
            ("java/lang/Math", "max", "(II)I"),
            (HELPER, "max", "(II)I"),
        )
        .expect("clean NotFound");
        assert_eq!(out, RetargetOutcome::NotFound);
        assert_eq!(bytes_back, patched, "NotFound returns the original bytes");
        let after = parse_layout(&patched).unwrap().pool.next;
        assert_eq!(before, after, "NotFound must not append pool entries");
    }

    #[test]
    fn retarget_descriptor_mismatch_refused() {
        let patched = patch_update(REAL).expect("patch");
        let err = retarget_invokestatic(
            &patched,
            "update",
            "(III)Z",
            (OPS_CLASS, "run", RUN_DESC),
            (HELPER, "run", "(LI;IIIIIILjava/lang/Object;)V"),
        )
        .unwrap_err();
        assert!(err.contains("descriptor"), "{err}");
    }

    #[test]
    fn retarget_missing_method_errors_cleanly() {
        let patched = patch_update(REAL).expect("patch");
        // Name utf8 that exists NOWHERE -> NotFound (no mutation, no error).
        let (bytes_back, out) = retarget_invokestatic(
            &patched,
            "nonexistent",
            "(III)Z",
            (OPS_CLASS, "run", RUN_DESC),
            (HELPER, "run", RUN_DESC),
        )
        .expect("clean NotFound");
        assert_eq!(out, RetargetOutcome::NotFound);
        assert_eq!(bytes_back, patched);
        // Name/desc utf8 entries that exist ("Code" attribute name, the
        // update descriptor) but name NO method -> the loud Err path.
        let err = retarget_invokestatic(
            &patched,
            "Code",
            "(III)Z",
            (OPS_CLASS, "run", RUN_DESC),
            (HELPER, "run", RUN_DESC),
        )
        .unwrap_err();
        assert!(err.contains("not found"), "{err}");
    }

    /// A Code attribute whose code_length lies (claims 0xFFFF, file is ~1KB)
    /// must fail CLOSED with Err — never panic, never rewrite (panic-free
    /// contract over hook-delivered bytes, G4 §6).
    #[test]
    fn retarget_corrupt_code_length_fails_closed() {
        let patched = patch_update(REAL).expect("patch");
        // Locate update's Code attribute in the patched bytes and inflate its
        // code_length field.
        let layout = parse_layout(&patched).unwrap();
        let name_idx = layout.pool.find_utf8("update").unwrap();
        let desc_idx = layout.pool.find_utf8("(III)Z").unwrap();
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx).unwrap();
        let mut p = m.start + 6;
        let count = usize::from(u16::from_be_bytes([patched[p], patched[p + 1]]));
        p += 2;
        for _ in 0..count {
            let name_idx = u16::from_be_bytes([patched[p], patched[p + 1]]);
            let len = u32::from_be_bytes([
                patched[p + 2],
                patched[p + 3],
                patched[p + 4],
                patched[p + 5],
            ]);
            let data = p + 6;
            if layout.pool.utf8_value(name_idx).as_deref() == Some("Code") {
                let mut corrupt = patched.to_vec();
                // code_length sits at data+4; claim the maximum.
                corrupt[data + 4] = 0xFF;
                corrupt[data + 5] = 0xFF;
                corrupt[data + 6] = 0xFF;
                corrupt[data + 7] = 0xFF;
                let res = retarget_invokestatic(
                    &corrupt,
                    "update",
                    "(III)Z",
                    (OPS_CLASS, "run", RUN_DESC),
                    (HELPER, "run", RUN_DESC),
                )
                .map(|_| ());
                assert!(res.is_err(), "claimed 4GB code must fail closed");
                return;
            }
            p = data + len as usize;
        }
        panic!("no Code attribute found in fixture");
    }

    /// The opcode-width table must step exactly: a tableswitch / lookupswitch
    /// / wide sequence followed by an invokestatic must place the found call
    /// site at the right pc (a mis-step would corrupt operands on rewrite).
    #[test]
    fn scan_steps_variable_length_ops_exactly() {
        // tableswitch at 0 with default -> +20, low=-1, high=1 (3 targets).
        // Layout: op(1) pad(3) default(4) low(4) high(4) offsets(3*4=12) = 29
        // bytes, then invokestatic with operand 0x1234.
        let mut code = vec![0xaa];
        let pad = (4 - ((0 + 1) & 3)) & 3; // = 3
        code.extend(std::iter::repeat(0u8).take(pad));
        code.extend_from_slice(&20i32.to_be_bytes()); // default
        code.extend_from_slice(&(-1i32).to_be_bytes());
        code.extend_from_slice(&1i32.to_be_bytes());
        for _ in 0..3 {
            code.extend_from_slice(&4i32.to_be_bytes()); // targets
        }
        assert_eq!(code.len(), 1 + pad + 12 + 12);
        code.extend_from_slice(&[0xb8, 0x12, 0x34]); // the invokestatic
        let sites = scan_invokestatics(&code, 0).expect("tableswitch walk");
        assert_eq!(sites, vec!((code.len() - 3, 0x1234)));

        // lookupswitch: op(1) pad(3) default(4) npairs(4) pairs(2*8=16)
        // then invokestatic.
        let mut code2 = vec![0xab];
        code2.extend(std::iter::repeat(0u8).take(pad));
        code2.extend_from_slice(&24i32.to_be_bytes());
        code2.extend_from_slice(&2i32.to_be_bytes()); // npairs
        for _ in 0..2 {
            code2.extend_from_slice(&1i32.to_be_bytes());
            code2.extend_from_slice(&4i32.to_be_bytes());
        }
        code2.extend_from_slice(&[0xb8, 0x56, 0x78]);
        let sites2 = scan_invokestatics(&code2, 0).expect("lookupswitch walk");
        assert_eq!(sites2, vec!((code2.len() - 3, 0x5678)));

        // wide iload (4 bytes total) + wide iinc (6 bytes total) + one
        // 0-byte op + invokestatic at offset 11.
        let code3 = [
            0xc4, 0x15, 0x01, 0x02, // wide iload 0x0102 (indices 0-3)
            0xc4, 0x84, 0x00, 0x05, 0x00, 0x64, // wide iinc 5 by 100 (4-9)
            0x1b, // iload_1 (no operands, offset 10)
            0xb8, 0xaa, 0xbb, // invokestatic at 11
        ];
        let sites3 = scan_invokestatics(&code3, 0).expect("wide walk");
        assert_eq!(sites3, vec!((11, 0xaabb)));

        // Unknown/reserved opcodes fail closed.
        assert!(opcode_extra(0xfe, &[0xfe], 0).is_err());
        assert!(opcode_extra(0xca, &[0xca], 0).is_err());
        // Truncation: an invokestatic whose operand runs past the end.
        assert!(scan_invokestatics(&[0xb8, 0x00], 0).is_err());
    }
}

#[cfg(test)]
mod dbg2 {
    use super::*;

    #[test]
    fn dbg_parse2() {
        let bytes = include_bytes!("../tests/fixtures/SingleUserAreaMap.class");
        let cp_count = u16::from_be_bytes([bytes[8], bytes[9]]);
        let (pool, end) = Pool::parse(bytes, 10, cp_count).unwrap_or_else(|| panic!("parse fail"));
        eprintln!("entries={} next={} cp_end={end} cp_count={cp_count}", pool.entries.len(), pool.next);
        let this_idx = u16::from_be_bytes([bytes[end + 2], bytes[end + 3]]);
        eprintln!("this_class_idx={this_idx}");
        let found = pool.entries.iter().find(|(i, _, _)| *i == this_idx).cloned();
        let (_, t, p) = found.expect("this_class entry");
        eprintln!("this class entry tag={t} payload={p:02x?}");
        let name_utf8 = u16::from_be_bytes([p[0], p[1]]);
        eprintln!("this_name = {:?}", pool.utf8_value(name_utf8));
        let mut p2 = end + 6;
        let iface = u16::from_be_bytes([bytes[p2], bytes[p2 + 1]]);
        p2 += 2 + 2 * usize::from(iface);
        let fields = u16::from_be_bytes([bytes[p2], bytes[p2 + 1]]);
        eprintln!("iface={iface} fields={fields}");
    }
}

#[cfg(test)]
mod dbg3 {
    use super::*;

    #[test]
    fn dbg_methods() {
        let bytes = include_bytes!("../tests/fixtures/SingleUserAreaMap.class");
        let layout = parse_layout(bytes).unwrap();
        let mut p = layout.methods_start;
        let count = usize::from(u16::from_be_bytes([bytes[p], bytes[p + 1]]));
        eprintln!("methods_count={count} at offset {p}");
        p += 2;
        for mi in 0..count {
            let access = u16::from_be_bytes([bytes[p], bytes[p + 1]]);
            let n = u16::from_be_bytes([bytes[p + 2], bytes[p + 3]]);
            let d = u16::from_be_bytes([bytes[p + 4], bytes[p + 5]]);
            let attr_count = usize::from(u16::from_be_bytes([bytes[p + 6], bytes[p + 7]]));
            eprintln!(
                "m{mi}: access=0x{access:04x} name={} desc={} attrs={attr_count}",
                layout.pool.utf8_value(n).unwrap_or_default(),
                layout.pool.utf8_value(d).unwrap_or_default()
            );
            p += 8;
            for _ in 0..attr_count {
                let aname = u16::from_be_bytes([bytes[p], bytes[p + 1]]);
                let alen = u32::from_be_bytes([bytes[p + 2], bytes[p + 3], bytes[p + 4], bytes[p + 5]]);
                eprintln!("   attr {} len={alen}", layout.pool.utf8_value(aname).unwrap_or_default());
                p += 6 + alen as usize;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ALLOC-DIET (S7-133, TASK-269 — ARCH-ATTACK lever #2, the allocation lane).
//
// Target: G1 GC + oop write-barriers ≈ 27% of Server-thread CPU on the
// X150K prime scene — an ADDRESSABLE DERIVATIVE of entity-lane allocation
// churn. javap contracts (STEP-0 census, materialized kernel
// purpur-1.21.10.jar) identified two per-tick allocation nodes:
//
//   1. LivingEntity.pushEntities()V — one site
//      `invokevirtual Level.getPushableEntities(Entity,AABB)List`
//      whose vanilla wrapper body allocates TWO ArrayLists per call (one
//      dead guava list whose result is never read + the fill list) and
//      then fills via the fill-into deep method
//      EntityLookup.getEntities(Entity,AABB,List,Predicate). ~45k+ living
//      entities tick per tick on the prime scene.
//   2. CollisionUtil.getCollisionsForBlocksOrWorldBorder — ONE
//      unconditional `new BlockPos.MutableBlockPos; dup; <init>:()V`
//      before any branch (~250k+ collision queries per tick: 148k moves +
//      100k item noPhysics noCollision checks). LazyEntityCollisionContext
//      pooling is deferred to wave-2 (private final base-class fields —
//      safe re-init requires Unsafe/MH; rejected for hot-path cost).
//
// Fix: both call sites are retargeted to `EntityQueryOps`
// (net/minecraft/world/entity/EntityQueryOps, defined into the kernel
// loader by alloc_diet.rs), which fills a ROTATING grow-only pool (8
// slots/thread — nested queries from event callbacks up to depth 7 are
// safe; beyond that, a loud CME, never silent corruption) and returns it.
// The deep fill method, its argument order, PlatformHooks.addToGetEntities
// and the Profiler counter are the SAME as vanilla → the returned entity
// SEQUENCE is bit-identical (median-exact parity). mutablePos() re-inits a
// pooled MutableBlockPos via set(0,0,0) — the exact state the vanilla
// no-arg constructor produces.
//
// Both edits are LENGTH-PRESERVING (no branch-offset or StackMapTable
// churn): the virtual→static retarget swaps the opcode byte + CP operand
// (3B→3B; stack shape identical because the receiver becomes the first
// static argument); the ctor splice swaps
// [new;dup;invokespecial] (7B) for [invokestatic;nop×4] (7B).
//
// Audit discipline (same as retarget_invokestatic / patch_update): call
// sites resolved BY NAME (never by offset, G4 §9), CP growth append-only
// with the 64K saturation guard, every read bounds-checked, fail-closed
// Err on any shape mismatch (alloc_diet.rs then stays vanilla).
// ---------------------------------------------------------------------------

/// Bridge class defined into the kernel loader by alloc_diet.rs.
pub const ALLOC_OPS_CLASS: &str = "net/minecraft/world/entity/EntityQueryOps";

const GET_PUSHABLES_DESC: &str =
    "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;";
const OPS_PUSHABLES_DESC: &str =
    "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;\
     Lnet/minecraft/world/phys/AABB;)Ljava/util/List;";

const MUTABLE_POS_CLASS: &str = "net/minecraft/core/BlockPos$MutableBlockPos";

/// Bridge class defined into the kernel loader by inside_cache.rs
/// (S7-135 / TASK-271 INSIDE-CACHE lever).
pub const INSIDE_OPS_CLASS: &str = "net/minecraft/world/entity/InsideBlockOps";

/// javap-контракт: единственный `isAffectedByBlocks` сайт внутри
/// `Entity.checkInsideBlocks(List, StepBasedCollector)` (offset 1).
const CHECK_INSIDE_DESC: &str =
    "(Ljava/util/List;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;)V";
const GATE_DESC: &str = "(Lnet/minecraft/world/entity/Entity;)Z";

/// Scan a bytecode array for `invokevirtual` (0xb6) AND `invokestatic`
/// (0xb8) instructions. The static opcode is required for IDEMPOTENCY: a
/// site already retargeted by this patcher is now 0xb8 and must still be
/// visible so it can be classified as `AlreadyPatched` (classification is
/// by the resolved triple — the virtual `from` triple cannot collide with
/// the static `to` triple because their descriptors differ). Same
/// bounded-walk contract as [`scan_invokestatics`].
fn scan_invoke_sites(code: &[u8], code_start: usize) -> Result<Vec<(usize, u16)>, String> {
    let mut out = Vec::new();
    let mut pc = 0usize;
    while pc < code.len() {
        let op = code[pc];
        if op == 0xb6 || op == 0xb8 {
            let b = code
                .get(pc + 1..pc + 3)
                .ok_or_else(|| "invoke operand truncated".to_string())?;
            out.push((code_start + pc, u16::from_be_bytes([b[0], b[1]])));
            pc += 3;
            continue;
        }
        let extra = opcode_extra(op, code, pc)?;
        pc = pc
            .checked_add(1 + extra)
            .ok_or_else(|| "code walk overflow".to_string())?;
        if pc > code.len() {
            return Err("truncated code (walk past end)".into());
        }
    }
    Ok(out)
}

/// Variant S retarget (alloc-diet): rewrite every `invokevirtual` call site
/// in `method_name`/`method_desc` that resolves to `from` (class, method,
/// descriptor) so it invokes `to` STATICALLY. The receiver of the virtual
/// call becomes the first static argument, so the verifier-visible stack
/// shape is unchanged; the `to` descriptor must therefore be the `from`
/// descriptor with the receiver class PREPENDED (asserted — a mismatch is
/// a caller bug that would corrupt the operand stack).
///
/// Idempotency: sites already resolving to `to` are counted and left alone
/// ([`RetargetOutcome::AlreadyPatched`]). NotFound returns the original
/// bytes without pool growth.
pub fn retarget_virtual_to_static(
    bytes: &[u8],
    method_name: &str,
    method_desc: &str,
    from: (&str, &str, &str),
    to: (&str, &str, &str),
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    // Receiver-prepended form: "(LReceiver;" + from.2 without its "(".
    let expect_static = format!("(L{};{}", from.0, &from.2[1..]);
    if to.2 != expect_static {
        return Err(format!(
            "static desc {} is not the virtual desc {} with receiver {} prepended \
             (stack shape would change)",
            to.2, from.2, from.0
        ));
    }
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    let mut pool = layout.pool;
    // Find-only probes (audit A4): a method whose name/desc utf8 entries
    // are absent cannot exist; NotFound must not mutate the pool.
    let Some(name_idx) = pool.find_utf8(method_name) else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let Some(desc_idx) = pool.find_utf8(method_desc) else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
        .ok_or_else(|| format!("method {method_name}{method_desc} not found"))?;
    let (code_start, code_len) = find_code_attr(bytes, &pool, &m)
        .ok_or_else(|| format!("method {method_name}{method_desc} has no Code attribute"))?;
    let code_end = code_start
        .checked_add(code_len)
        .ok_or_else(|| "code length overflow".to_string())?;
    let code = bytes
        .get(code_start..code_end)
        .ok_or_else(|| "code region truncated".to_string())?;
    let sites = scan_invoke_sites(code, code_start)?;

    let to_triple = (to.0.to_string(), to.1.to_string(), to.2.to_string());
    let from_triple = (from.0.to_string(), from.1.to_string(), from.2.to_string());
    let mut rewrite: Vec<usize> = Vec::new(); // absolute offsets of the opcode byte
    let mut already = 0usize;
    for (op_pc, cp_idx) in sites {
        match pool.methodref_parts(cp_idx) {
            Some(parts) if parts == to_triple => already += 1,
            Some(parts) if parts == from_triple => rewrite.push(op_pc),
            _ => {}
        }
    }
    if rewrite.is_empty() {
        return Ok((bytes.to_vec(), if already > 0 {
            RetargetOutcome::AlreadyPatched { sites: already }
        } else {
            RetargetOutcome::NotFound
        }));
    }

    // Append (or reuse) the Methodref for `to` — append-only, dedup.
    let new_idx = pool.method_ref(to.0, to.1, to.2);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for retarget ref".into());
    }

    // Splice: header + grown pool + tail; rewrite opcode byte AND operands
    // for each matched site. All sites are >= cp_end (method table follows
    // the pool), so the rewrite applies to the tail copy.
    let mut tail = bytes[layout.cp_end..].to_vec();
    let want = new_idx.to_be_bytes();
    for &op_off in &rewrite {
        let rel = op_off - layout.cp_end;
        if rel + 2 >= tail.len() {
            return Err("retarget opcode outside class tail (corrupt layout?)".into());
        }
        tail[rel] = 0xb8; // invokestatic
        tail[rel + 1] = want[0];
        tail[rel + 2] = want[1];
    }
    let mut out = Vec::with_capacity(bytes.len() + 64);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    out.extend_from_slice(&pool.next.to_be_bytes()); // new cp_count
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&tail);
    Ok((out, RetargetOutcome::Retargeted { sites: rewrite.len() }))
}

/// Whole-bridge patch for `LivingEntity.pushEntities()V`: retarget the
/// single `Level.getPushableEntities` call site to the static
/// `EntityQueryOps.pushables` bridge. Strict: anything other than exactly
/// one retargeted site on first sight is a shape mismatch → Err (fail
/// closed; alloc_diet stays vanilla).
pub fn patch_push_entities(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    retarget_virtual_to_static(
        bytes,
        "pushEntities",
        "()V",
        ("net/minecraft/world/level/Level", "getPushableEntities", GET_PUSHABLES_DESC),
        (ALLOC_OPS_CLASS, "pushables", OPS_PUSHABLES_DESC),
    )
}

/// Find a method by NAME only (desc resolved by the caller from the found
/// entry). Returns all matches — the collision patcher requires exactly one
/// (fail-closed on overloads).
fn find_methods_by_name(
    bytes: &[u8],
    methods_start: usize,
    name_idx: u16,
) -> Option<Vec<Method>> {
    let mut p = methods_start;
    let count = usize::from(u16_at(bytes, p)?);
    p = p.checked_add(2)?;
    let mut out = Vec::new();
    for _ in 0..count {
        let start = p;
        let access = u16_at(bytes, p)?;
        let n = u16_at(bytes, p.checked_add(2)?)?;
        let d = u16_at(bytes, p.checked_add(4)?)?;
        p = p.checked_add(6)?;
        let attr_count = usize::from(u16_at(bytes, p)?);
        p = p.checked_add(2)?;
        for _ in 0..attr_count {
            let len = u32_at(bytes, p.checked_add(2)?)?;
            p = p.checked_add(6)?.checked_add(len as usize)?;
        }
        if n == name_idx {
            out.push(Method {
                start,
                end: p,
                name_idx: n,
                desc_idx: d,
                access,
            });
        }
    }
    Some(out)
}

/// Resolve a CONSTANT_Class index to its internal name.
fn class_name_of(pool: &Pool, idx: u16) -> Option<String> {
    let (_, tag, payload) = pool.entries.iter().find(|(i, _, _)| *i == idx)?;
    if *tag != TAG_CLASS || payload.len() < 2 {
        return None;
    }
    let utf8_idx = u16::from_be_bytes([payload[0], payload[1]]);
    pool.utf8_value(utf8_idx)
}

/// Whole-bridge patch for
/// `CollisionUtil.getCollisionsForBlocksOrWorldBorder` — replace the
/// unconditional `new BlockPos.MutableBlockPos; dup; <init>:()V` sequence
/// (7 bytes: [0xbb #X][0x59][0xb7 #Y]) with
/// [0xb8 #M][0x00 0x00 0x00 0x00] = `invokestatic
/// EntityQueryOps.mutablePos ()Lnet/minecraft/core/BlockPos$MutableBlockPos;`
/// + 4×nop. Length-preserving: no branch offsets or StackMapTable frames
/// move; the operand stack after the sequence is [ref] in both forms.
///
/// Strict: exactly ONE matching site expected (javap census on the real
/// kernel: a single unconditional MutableBlockPos allocation in the
/// method); zero sites = already patched (idempotent no-op), >1 = shape
/// mismatch → Err (fail closed).
pub fn patch_collision_temps(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    let mut pool = layout.pool;
    let Some(name_idx) = pool.find_utf8("getCollisionsForBlocksOrWorldBorder") else {
        return Err("collision method name absent from pool".into());
    };
    let matches = find_methods_by_name(bytes, layout.methods_start, name_idx)
        .ok_or_else(|| "method table walk failed".to_string())?;
    if matches.len() != 1 {
        return Err(format!(
            "expected exactly one getCollisionsForBlocksOrWorldBorder, found {}",
            matches.len()
        ));
    }
    let (code_start, code_len) = find_code_attr(bytes, &pool, &matches[0])
        .ok_or_else(|| "collision method has no Code attribute".to_string())?;
    let code_end = code_start
        .checked_add(code_len)
        .ok_or_else(|| "code length overflow".to_string())?;
    let code = bytes
        .get(code_start..code_end)
        .ok_or_else(|| "code region truncated".to_string())?;

    // Walk the code, pattern-match [new #X][dup][invokespecial #Y] with
    // X → MutableBlockPos class, Y → (MutableBlockPos, <init>, ()V).
    let mut sites: Vec<usize> = Vec::new(); // absolute offsets of the `new` opcode
    let mut pc = 0usize;
    while pc < code.len() {
        let op = code[pc];
        if op == 0xbb && pc + 7 <= code.len() {
            let class_idx = u16::from_be_bytes([code[pc + 1], code[pc + 2]]);
            let class_ok = class_name_of(&pool, class_idx)
                .map(|n| n == MUTABLE_POS_CLASS)
                .unwrap_or(false);
            if class_ok
                && code[pc + 3] == 0x59 // dup
                && code[pc + 4] == 0xb7
            {
                let init_idx = u16::from_be_bytes([code[pc + 5], code[pc + 6]]);
                let init_ok = pool
                    .methodref_parts(init_idx)
                    .map(|(c, n, d)| c == MUTABLE_POS_CLASS && n == "<init>" && d == "()V")
                    .unwrap_or(false);
                if init_ok {
                    sites.push(code_start + pc);
                    pc += 7;
                    continue;
                }
            }
        }
        let extra = opcode_extra(op, code, pc)?;
        pc = pc
            .checked_add(1 + extra)
            .ok_or_else(|| "code walk overflow".to_string())?;
        if pc > code.len() {
            return Err("truncated code (walk past end)".into());
        }
    }

    if sites.is_empty() {
        // Already patched (idempotent re-sight) — return original bytes.
        return Ok(bytes.to_vec());
    }
    if sites.len() > 1 {
        return Err(format!(
            "expected exactly one MutableBlockPos ctor site, found {}",
            sites.len()
        ));
    }

    // Append (or reuse) the Methodref for the static factory.
    let new_idx = pool.method_ref(
        ALLOC_OPS_CLASS,
        "mutablePos",
        "()Lnet/minecraft/core/BlockPos$MutableBlockPos;",
    );
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for ops ref".into());
    }

    let mut tail = bytes[layout.cp_end..].to_vec();
    let want = new_idx.to_be_bytes();
    for &op_off in &sites {
        let rel = op_off - layout.cp_end;
        if rel + 6 >= tail.len() {
            return Err("ctor splice outside class tail (corrupt layout?)".into());
        }
        tail[rel] = 0xb8; // invokestatic
        tail[rel + 1] = want[0];
        tail[rel + 2] = want[1];
        tail[rel + 3] = 0x00; // nop (former dup)
        tail[rel + 4] = 0x00; // nop (former invokespecial)
        tail[rel + 5] = 0x00; // nop
        tail[rel + 6] = 0x00; // nop
    }
    let mut out = Vec::with_capacity(bytes.len() + 64);
    out.extend_from_slice(&bytes[0..8]);
    out.extend_from_slice(&pool.next.to_be_bytes());
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&tail);
    Ok(out)
}

/// Whole-bridge patch for S7-135/TASK-271 INSIDE-CACHE: retarget the single
/// `Entity.isAffectedByBlocks` site INSIDE
/// `Entity.checkInsideBlocks(List, StepBasedCollector)` (javap offset 1 —
/// the method-entry gate) to the static `InsideBlockOps.gate(Entity)Z`.
/// Receiver-first, 3B→3B, stack shape [this]→[boolean] preserved.
///
/// Extra fail-closed guard: the bridge resolves the private
/// `insideEffectCollector` field via Unsafe by NAME — if the kernel renamed
/// it, the lever must not arm (pool utf8 probe here, before any rewrite).
/// Strict: exactly one retargeted site expected on first sight.
pub fn patch_inside_cache(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    if layout.pool.find_utf8("insideEffectCollector").is_none() {
        return Err("insideEffectCollector field absent from pool (kernel rename?)".into());
    }
    let (out, outcome) = retarget_virtual_to_static(
        bytes,
        "checkInsideBlocks",
        CHECK_INSIDE_DESC,
        (
            "net/minecraft/world/entity/Entity",
            "isAffectedByBlocks",
            "()Z",
        ),
        (INSIDE_OPS_CLASS, "gate", GATE_DESC),
    )?;
    if let RetargetOutcome::Retargeted { sites } = &outcome {
        if *sites != 1 {
            return Err(format!(
                "expected exactly one isAffectedByBlocks site in checkInsideBlocks, got {sites}"
            ));
        }
    }
    Ok((out, outcome))
}

// ---------------------------------------------------------------------------
// FLUSH-DIET (S7-137, ARCH-ATTACK lever #4 — the StepBasedCollector.flushStep
// allocation lane).
//
// STEP-0 javap contract (purpur-1.21.10 kernel, byte-identical to the CI
// booted kernel): vanilla flushStep() transfers the per-type before/after
// effect lists into finalEffects via List.addAll. ArrayList.addAll resolves
// its argument through c.toArray() BEFORE the emptiness check, so every
// empty-list call still allocates a throwaway new Object[0] through
// Arrays.copyOf. The alloc census (run 35275967738, X150K, 25.6GB/60s) pinned
// 336 samples (4.6% of the true churn) to exactly
//   advanceStep -> flushStep -> ArrayList.addAll -> ArrayList.toArray
//     -> Arrays.copyOf -> Object[].
//
// Shape: BOTH addAll call sites inside flushStep()V are invokeinterface
// (5 bytes: 0xb9 idx1 idx2 count 0) to
//   java/util/List.addAll:(Ljava/util/Collection;)Z.
// Retarget form: invokestatic FlushOps.fladd
//   (Ljava/util/List;Ljava/util/Collection;)Z (3 bytes) + 2 nop (0x00 0x00)
// filling the former count/zero operand slots — length-preserving, no branch
// or StackMapTable offsets move; the operand stack ([List, Collection] ->
// [int]) is identical for both forms. Receiver-first descriptor equals the
// virtual descriptor with the receiver class prepended (asserted below).
//
// Strict: exactly TWO matching sites (javap census on the real kernel —
// offsets 41 and 114); zero matching invokeinterface sites while BOTH
// existing invokestatics resolve to the bridge = AlreadyPatched (idempotent
// no-op); anything else = shape mismatch -> Err (fail closed, vanilla stays).

const FLUSH_SBC_CLASS: &str = "net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector";
const FLUSH_OPS_CLASS: &str = "net/minecraft/world/entity/FlushOps";
const FLUSH_FLADD_DESC: &str = "(Ljava/util/List;Ljava/util/Collection;)Z";

// ---------------------------------------------------------------------------
// FLUID-FREE (S7-138, ARCH-ATTACK lever #5 — the fluid-scan lane).
//
// STEP-0 javap contract (DESIGN.md, research/fluid-free-2026-09-18): the
// moonrise-shaped updateFluidHeightAndDoFluidPushing(TagKey,double) runs a
// triple AABB loop with a DIRECT PalettedContainer.get per cell for EVERY
// entity per tick (2 call sites: WATER 0.014 + LAVA ultraWarm — baseTick
// lane); the dominant cell verdict is isEmpty (land entities scan in
// vain). Fluid reads are 56% of the top-1 CPU function's clients.
//
// Shape: retarget BOTH invokevirtual call sites of
// updateFluidHeightAndDoFluidPushing (inside the two wrappers:
// updateInWaterStateAndDoWaterCurrentPushing offset 39 (WATER) and
// updateInWaterStateAndDoFluidPushing offset 41 (LAVA)) to the static
// FluidOps.fgate(Entity,TagKey,double)Z (receiver-first, 3B->3B,
// length-preserving). The vanilla body itself stays UNTOUCHED: a bridge
// miss delegates back via a plain (un-retargeted) invokevirtual — no
// recursion. HIT (all sections fluid-free, event-driven gen validation)
// reproduces the exact vanilla empty outcome: fluidHeight.put(tag, 0.0)
// + return false.
//
// Strict: exactly ONE matching site per wrapper (javap census), total TWO;
// zero both = AlreadyPatched only when both already static; anything else
// = fail closed.

const FLUID_OPS_CLASS: &str = "net/minecraft/world/entity/FluidOps";
const FLUID_FGATE_DESC: &str = "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/tags/TagKey;D)Z";
const FLUID_TARGET: (&str, &str, &str) = (
    "net/minecraft/world/entity/Entity",
    "updateFluidHeightAndDoFluidPushing",
    "(Lnet/minecraft/tags/TagKey;D)Z",
);

pub fn patch_fluid_gate(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    // Kernel-rename guards: both wrappers and the fluidHeight field must be
    // present before anything mutates.
    for probe in ["updateInWaterStateAndDoWaterCurrentPushing", "fluidHeight"] {
        if layout.pool.find_utf8(probe).is_none() {
            return Err(format!("{probe} absent from pool (kernel rename?)"));
        }
    }
    let expect_static = format!("(L{};{}", FLUID_TARGET.0, &FLUID_TARGET.2[1..]);
    if FLUID_FGATE_DESC != expect_static {
        return Err("fgate descriptor is not the receiver-prepended target form".into());
    }

    // Retarget site #1: the WATER wrapper's call.
    let (out1, outcome1) = retarget_virtual_to_static(
        bytes,
        "updateInWaterStateAndDoWaterCurrentPushing",
        "()V",
        FLUID_TARGET,
        (FLUID_OPS_CLASS, "fgate", FLUID_FGATE_DESC),
    )?;
    // Retarget site #2: the LAVA wrapper's call (on top of #1's output).
    let (out2, outcome2) = retarget_virtual_to_static(
        &out1,
        "updateInWaterStateAndDoFluidPushing",
        "()Z",
        FLUID_TARGET,
        (FLUID_OPS_CLASS, "fgate", FLUID_FGATE_DESC),
    )?;

    // Classify the combined outcome (strict: 1+1 on first sight).
    let sites1 = match &outcome1 {
        RetargetOutcome::Retargeted { sites } => *sites,
        RetargetOutcome::AlreadyPatched { sites } => *sites,
        RetargetOutcome::NotFound => 0,
    };
    let sites2 = match &outcome2 {
        RetargetOutcome::Retargeted { sites } => *sites,
        RetargetOutcome::AlreadyPatched { sites } => *sites,
        RetargetOutcome::NotFound => 0,
    };
    match (sites1, sites2) {
        (1, 1) => match (&outcome1, &outcome2) {
            (RetargetOutcome::AlreadyPatched { .. }, RetargetOutcome::AlreadyPatched { .. }) => {
                Ok((out2, RetargetOutcome::AlreadyPatched { sites: 2 }))
            }
            _ => Ok((out2, RetargetOutcome::Retargeted { sites: 2 })),
        },
        _ => Err(format!(
            "expected exactly one fgate site per wrapper (water={sites1}, lava={sites2})"
        )),
    }
}

// ---------------------------------------------------------------------------
// FLUID-DIRTY (S7-151 / TASK-290, ARCH-ATTACK lever #6): per-entity
// memoization of the fluid-scan portion. Two retargets:
//   1) BOTH Entity wrapper call-sites of updateFluidHeightAndDoFluidPushing
//      (the ONLY two call sites in the whole kernel — census
//      research/fluid-dirty-2026-09-18/S7151_CENSUS.md) →
//      FluidPushOps.scan(Entity,TagKey,double)Z (receiver-first, 3B→3B).
//   2) The single LevelChunkSection.setBlockState(IIILBlockState)BlockState
//      call site inside LevelChunk.setBlockState(BlockPos,BlockState,I) →
//      FluidPushOps.secWrite(...) (delegate returning the old state; bumps
//      the section dirty-stamp on a real fluid-state change).

const FLUID_PUSH_OPS_CLASS: &str = "net/minecraft/world/entity/FluidPushOps";
const FLUID_SCAN_DESC: &str = "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/tags/TagKey;D)Z";
const SEC_WRITE_DESC: &str = "(Lnet/minecraft/world/level/chunk/LevelChunkSection;IIILnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;";
const SEC_WRITE_FROM: (&str, &str, &str) = (
    "net/minecraft/world/level/chunk/LevelChunkSection",
    "setBlockState",
    "(IIILnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;",
);
const LEVELCHUNK_SETBLOCK_DESC: &str = "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Lnet/minecraft/world/level/block/state/BlockState;";

/// Entity bytes: retarget both fluid-scan wrapper sites to FluidPushOps.scan.
/// Strict: exactly ONE site per wrapper (water + lava), like patch_fluid_gate.
pub fn patch_fluid_dirty_entity(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    for probe in [
        "updateInWaterStateAndDoWaterCurrentPushing",
        "updateInWaterStateAndDoFluidPushing",
        "fluidHeight",
        "touchingUnloadedChunk",
    ] {
        if layout.pool.find_utf8(probe).is_none() {
            return Err(format!("{probe} absent from pool (kernel rename?)"));
        }
    }
    let expect_static = format!("(L{};{}", FLUID_TARGET.0, &FLUID_TARGET.2[1..]);
    if FLUID_SCAN_DESC != expect_static {
        return Err("scan descriptor is not the receiver-prepended target form".into());
    }

    let (out1, outcome1) = retarget_virtual_to_static(
        bytes,
        "updateInWaterStateAndDoWaterCurrentPushing",
        "()V",
        FLUID_TARGET,
        (FLUID_PUSH_OPS_CLASS, "scan", FLUID_SCAN_DESC),
    )?;
    let (out2, outcome2) = retarget_virtual_to_static(
        &out1,
        "updateInWaterStateAndDoFluidPushing",
        "()Z",
        FLUID_TARGET,
        (FLUID_PUSH_OPS_CLASS, "scan", FLUID_SCAN_DESC),
    )?;

    let sites1 = match &outcome1 {
        RetargetOutcome::Retargeted { sites } => *sites,
        RetargetOutcome::AlreadyPatched { sites } => *sites,
        RetargetOutcome::NotFound => 0,
    };
    let sites2 = match &outcome2 {
        RetargetOutcome::Retargeted { sites } => *sites,
        RetargetOutcome::AlreadyPatched { sites } => *sites,
        RetargetOutcome::NotFound => 0,
    };
    match (sites1, sites2) {
        (1, 1) => match (&outcome1, &outcome2) {
            (RetargetOutcome::AlreadyPatched { .. }, RetargetOutcome::AlreadyPatched { .. }) => {
                Ok((out2, RetargetOutcome::AlreadyPatched { sites: 2 }))
            }
            _ => Ok((out2, RetargetOutcome::Retargeted { sites: 2 })),
        },
        _ => Err(format!(
            "expected exactly one scan site per wrapper (water={sites1}, lava={sites2})"
        )),
    }
}

/// LevelChunk bytes: retarget the single LevelChunkSection.setBlockState
/// site to FluidPushOps.secWrite (delegate + dirty-stamp bump).
pub fn patch_fluid_dirty_levelchunk(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    for probe in ["setBlockState", "net/minecraft/world/level/chunk/LevelChunkSection"] {
        if layout.pool.find_utf8(probe).is_none() {
            return Err(format!("{probe} absent from pool (kernel rename?)"));
        }
    }
    let expect_static = format!("(L{};{}", SEC_WRITE_FROM.0, &SEC_WRITE_FROM.2[1..]);
    if SEC_WRITE_DESC != expect_static {
        return Err("secWrite descriptor is not the receiver-prepended target form".into());
    }
    retarget_virtual_to_static(
        bytes,
        "setBlockState",
        LEVELCHUNK_SETBLOCK_DESC,
        SEC_WRITE_FROM,
        (FLUID_PUSH_OPS_CLASS, "secWrite", SEC_WRITE_DESC),
    )
}

// --- REGION-THREADS (S7-156, TASK-295) -------------------------------------

const REGION_TICK_OPS_CLASS: &str = "net/minecraft/world/entity/RegionTickOps";
const SL_TICK_DESC: &str = "(Ljava/util/function/BooleanSupplier;)V";
const SL_FOREACH_FROM: (&str, &str, &str) = (
    "net/minecraft/world/level/entity/EntityTickList",
    "forEach",
    "(Ljava/util/function/Consumer;)V",
);
const SL_FOREACH_TO_DESC: &str =
    "(Lnet/minecraft/world/level/entity/EntityTickList;Ljava/util/function/Consumer;)V";
const ETL_ADD_FROM: (&str, &str, &str) = (
    "net/minecraft/world/level/entity/EntityTickList",
    "add",
    "(Lnet/minecraft/world/entity/Entity;)V",
);
const ETL_REMOVE_FROM: (&str, &str, &str) = (
    "net/minecraft/world/level/entity/EntityTickList",
    "remove",
    "(Lnet/minecraft/world/entity/Entity;)V",
);
const ETL_GUARD_TO_DESC: &str =
    "(Lnet/minecraft/world/level/entity/EntityTickList;Lnet/minecraft/world/entity/Entity;)V";

// S7-158b/d (TASK-298) hardening targets: removal-safe tracker sweep +
// serialized UUID seeding. Both races are live-leg-2 evidence (35363758352):
// the fatal newTrackerTick NPE and the UUID-duplicate spawn WARN.
const TRACKER_TICK_OPS_CLASS: &str = "net/minecraft/server/level/TrackerTickOps";
const RNG_OPS_CLASS: &str = "net/minecraft/util/RngOps";
const CHUNKMAP_TICK_DESC: &str = "()V";
const CHUNKMAP_TRACKER_FROM: (&str, &str, &str) = (
    "net/minecraft/server/level/ChunkMap",
    "newTrackerTick",
    "()V",
);
const ENTITY_CTOR_DESC: &str =
    "(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V";
const MTH_INSECURE_UUID_FROM: (&str, &str, &str) = (
    "net/minecraft/util/Mth",
    "createInsecureUUID",
    "(Lnet/minecraft/util/RandomSource;)Ljava/util/UUID;",
);
const RNG_INSECURE_UUID_TO: (&str, &str, &str) = (
    "net/minecraft/util/RngOps",
    "createInsecureUUID",
    "(Lnet/minecraft/util/RandomSource;)Ljava/util/UUID;",
);

/// S7-158b: the ONLY `newTrackerTick` call site in the kernel —
/// `ChunkMap.tick()V`, whose whole body is exactly that single call (javap:
/// bytecode 0-4) — retargeted to `TrackerTickOps.newTrackerTick(ChunkMap)`:
/// the vanilla sweep BYTE-FOR-BYTE plus the removal-safe null guard (live
/// crash 35363758352: the unchecked `trackerEntities.getRawDataUnchecked()`
/// walk NPE'd on a slot nulled by a parallel worker's entity removal).
/// Strict: caller asserts Retargeted{1}.
pub fn patch_region_tracker_chunkmap(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    for probe in [
        "newTrackerTick",
        "net/minecraft/server/level/ChunkMap",
        "ca/spottedleaf/moonrise/patches/entity_tracker/EntityTrackerEntity",
    ] {
        if layout.pool.find_utf8(probe).is_none() {
            return Err(format!("{probe} absent from pool (kernel rename?)"));
        }
    }
    retarget_virtual_to_static(
        bytes,
        "tick",
        CHUNKMAP_TICK_DESC,
        CHUNKMAP_TRACKER_FROM,
        (
            TRACKER_TICK_OPS_CLASS,
            "newTrackerTick",
            "(Lnet/minecraft/server/level/ChunkMap;)V",
        ),
    )
}

/// S7-158d: the ONLY `Mth.createInsecureUUID(RandomSource)` call site in the
/// kernel (Entity ctor; census: ServerBossEvent uses the no-arg Mth.RANDOM
/// variant which is already thread-safe) -> `RngOps.createInsecureUUID`
/// — static->static, SAME descriptor (retarget_invokestatic enforces the
/// stack shape), body = vanilla two-draw UUIDv4 serialized per-source
/// (purpur entity-shared-random=true routes EVERY entity random through one
/// ThreadUnsafeRandom; two parallel workers constructing spawned entities
/// drew identical state -> duplicate UUID -> EntityLookup "can't add" WARN
/// and the spawned entity lost). Strict: caller asserts Retargeted{1}.
pub fn patch_region_rng_entity(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    for probe in [
        "createInsecureUUID",
        "net/minecraft/util/Mth",
        "net/minecraft/util/RandomSource",
    ] {
        if layout.pool.find_utf8(probe).is_none() {
            return Err(format!("{probe} absent from pool (kernel rename?)"));
        }
    }
    retarget_invokestatic(
        bytes,
        "<init>",
        ENTITY_CTOR_DESC,
        MTH_INSECURE_UUID_FROM,
        RNG_INSECURE_UUID_TO,
    )
}


/// S7-156: the single `EntityTickList.forEach(Consumer)` call site inside
/// `ServerLevel.tick(BooleanSupplier)` -> `RegionTickOps.forEach` (1:1
/// receiver-prepended stack shape). Strict: caller asserts Retargeted{1}.
pub fn patch_region_tick_serverlevel(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    for probe in [
        "tick",
        SL_TICK_DESC,
        "net/minecraft/world/level/entity/EntityTickList",
        "java/util/function/Consumer",
    ] {
        if layout.pool.find_utf8(probe).is_none() {
            return Err(format!("{probe} absent from pool (kernel rename?)"));
        }
    }
    retarget_virtual_to_static(
        bytes,
        "tick",
        SL_TICK_DESC,
        SL_FOREACH_FROM,
        (REGION_TICK_OPS_CLASS, "forEach", SL_FOREACH_TO_DESC),
    )
}

/// S7-156: the ONLY `EntityTickList.add/remove` call sites in the whole
/// kernel (`ServerLevel$EntityCallbacks.onTickingStart/onTickingEnd`) ->
/// `RegionTickOps.onTickingStart/onTickingEnd` (deferred FIFO during a
/// parallel phase). Strict: caller asserts Retargeted{1} on BOTH.
pub fn patch_region_tick_callbacks(
    bytes: &[u8],
) -> Result<(Vec<u8>, (RetargetOutcome, RetargetOutcome)), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    for probe in [
        "onTickingStart",
        "onTickingEnd",
        "net/minecraft/world/entity/Entity",
        "net/minecraft/world/level/entity/EntityTickList",
    ] {
        if layout.pool.find_utf8(probe).is_none() {
            return Err(format!("{probe} absent from pool (kernel rename?)"));
        }
    }
    let (b1, o_add) = retarget_virtual_to_static(
        bytes,
        "onTickingStart",
        "(Lnet/minecraft/world/entity/Entity;)V",
        ETL_ADD_FROM,
        (REGION_TICK_OPS_CLASS, "onTickingStart", ETL_GUARD_TO_DESC),
    )?;
    let (b2, o_rem) = retarget_virtual_to_static(
        &b1,
        "onTickingEnd",
        "(Lnet/minecraft/world/entity/Entity;)V",
        ETL_REMOVE_FROM,
        (REGION_TICK_OPS_CLASS, "onTickingEnd", ETL_GUARD_TO_DESC),
    )?;
    Ok((b2, (o_add, o_rem)))
}

/// S7-157b: the ONLY worker-reachable mid-tick pump site —
/// `Level.guardEntityTick`'s trailing `invokevirtual moonrise$midTickTasks()V`
/// (census S7-157b: 7 `executeMidTickTasks` callers kernel-wide, but exactly
/// one is reachable from entity-tick worker threads; tickBlockEntities /
/// tickFluid / tickBlock / runAllTasksAtTickStart / pollTaskInternal /
/// iterateTickingChunksFaster are all main-loop). Retargeted to
/// `RegionTickOps.midTickTasks(Level)` which suppresses the pump on region
/// workers (live crash 35353820223: concurrent main-thread-queue poll from a
/// worker -> NoSuchElementException) and reproduces the exact vanilla
/// virtual dispatch on the main thread. Strict: caller asserts Retargeted{1}.
pub fn patch_region_tick_guardentity(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    for probe in [
        "guardEntityTick",
        "moonrise$midTickTasks",
        "net/minecraft/world/entity/Entity",
        "java/util/function/Consumer",
    ] {
        if layout.pool.find_utf8(probe).is_none() {
            return Err(format!("{probe} absent from pool (kernel rename?)"));
        }
    }
    retarget_virtual_to_static(
        bytes,
        "guardEntityTick",
        "(Ljava/util/function/Consumer;Lnet/minecraft/world/entity/Entity;)V",
        (
            "net/minecraft/world/level/Level",
            "moonrise$midTickTasks",
            "()V",
        ),
        (
            REGION_TICK_OPS_CLASS,
            "midTickTasks",
            "(Lnet/minecraft/world/level/Level;)V",
        ),
    )
}


pub fn patch_flush_step(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    // Kernel-rename guard: the collector's own fields must be present.
    if layout.pool.find_utf8("beforeEffectsInStep").is_none() {
        return Err("beforeEffectsInStep field absent from pool (kernel rename?)".into());
    }
    let expect_static = "(Ljava/util/List;Ljava/util/Collection;)Z";
    if FLUSH_FLADD_DESC != expect_static {
        return Err("fladd descriptor is not the receiver-prepended List.addAll form".into());
    }
    let Some(name_idx) = layout.pool.find_utf8("flushStep") else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let Some(desc_idx) = layout.pool.find_utf8("()V") else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
        .ok_or_else(|| "method flushStep()V not found".to_string())?;
    let (code_start, code_len) = find_code_attr(bytes, &layout.pool, &m)
        .ok_or_else(|| "flushStep has no Code attribute".to_string())?;
    let code_end = code_start
        .checked_add(code_len)
        .ok_or_else(|| "code length overflow".to_string())?;
    let code = bytes
        .get(code_start..code_end)
        .ok_or_else(|| "code region truncated".to_string())?;

    // Walk the bytecode, collecting BOTH invokeinterface (0xb9, 5 bytes) and
    // invokestatic (0xb8, 3 bytes) sites; every other opcode through the
    // shared length table.
    let mut iface: Vec<(usize, u16)> = Vec::new();
    let mut statics: Vec<(usize, u16)> = Vec::new();
    let mut pc = 0usize;
    while pc < code.len() {
        let op = code[pc];
        if op == 0xb9 {
            let b = code
                .get(pc + 1..pc + 3)
                .ok_or_else(|| "invokeinterface operand truncated".to_string())?;
            iface.push((code_start + pc, u16::from_be_bytes([b[0], b[1]])));
            pc += 5;
            continue;
        }
        if op == 0xb8 {
            let b = code
                .get(pc + 1..pc + 3)
                .ok_or_else(|| "invokestatic operand truncated".to_string())?;
            statics.push((code_start + pc, u16::from_be_bytes([b[0], b[1]])));
            pc += 3;
            continue;
        }
        let extra = opcode_extra(op, code, pc)?;
        pc = pc
            .checked_add(1 + extra)
            .ok_or_else(|| "code walk overflow".to_string())?;
        if pc > code.len() {
            return Err("truncated code (walk past end)".into());
        }
    }

    // Classify BY NAME (never by offset — CP indexes shift between ASM runs).
    let list_addall = (
        "java/util/List".to_string(),
        "addAll".to_string(),
        "(Ljava/util/Collection;)Z".to_string(),
    );
    let to_triple = (
        FLUSH_OPS_CLASS.to_string(),
        "fladd".to_string(),
        FLUSH_FLADD_DESC.to_string(),
    );
    let mut rewrite: Vec<usize> = Vec::new(); // absolute offsets of opcode bytes
    let mut already = 0usize;
    let mut addall_iface_left = 0usize;
    for (op_pc, cp_idx) in &iface {
        match layout.pool.methodref_parts(*cp_idx) {
            Some(parts) if parts == list_addall => {
                rewrite.push(*op_pc);
                addall_iface_left += 1;
            }
            _ => {}
        }
    }
    for (op_pc, cp_idx) in &statics {
        match layout.pool.methodref_parts(*cp_idx) {
            Some(parts) if parts == to_triple => already += 1,
            _ => {}
        }
    }
    if rewrite.is_empty() {
        // Idempotent no-op only when BOTH sites already resolve to the
        // bridge (2 static fladd, no List.addAll invokeinterface left).
        if already == statics.len() && already == 2 && addall_iface_left == 0 {
            return Ok((
                bytes.to_vec(),
                RetargetOutcome::AlreadyPatched { sites: already },
            ));
        }
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    }
    if rewrite.len() != 2 {
        return Err(format!(
            "expected exactly two List.addAll sites in flushStep, got {}",
            rewrite.len()
        ));
    }

    // Append (or reuse) the Methodref for the bridge — append-only, dedup.
    let mut pool = layout.pool;
    let new_idx = pool.method_ref(FLUSH_OPS_CLASS, "fladd", FLUSH_FLADD_DESC);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for fladd ref".into());
    }

    // Splice: header + grown pool + tail; rewrite 5 bytes per site:
    // [0xb8][idx1][idx2][0x00][0x00] — invokestatic + 2 nop replacing the
    // former count/zero operand slots of invokeinterface.
    let mut tail = bytes[layout.cp_end..].to_vec();
    let want = new_idx.to_be_bytes();
    for &op_off in &rewrite {
        let rel = op_off - layout.cp_end;
        if rel + 4 >= tail.len() {
            return Err("retarget opcode outside class tail (corrupt layout?)".into());
        }
        tail[rel] = 0xb8; // invokestatic
        tail[rel + 1] = want[0];
        tail[rel + 2] = want[1];
        tail[rel + 3] = 0x00; // nop (former count operand)
        tail[rel + 4] = 0x00; // nop (former zero operand)
    }
    let mut out = Vec::with_capacity(bytes.len() + 64);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    out.extend_from_slice(&pool.next.to_be_bytes()); // new cp_count
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&tail);
    Ok((
        out,
        RetargetOutcome::Retargeted { sites: rewrite.len() },
    ))
}

// ---------------------------------------------------------------------------
// SECTION-FF FIELD INJECTION (S7-143, FLUID-FREE-SECTION lever) — append-only
// 2-field splice into LevelChunkSection:
//   crusstyFf    : B  PUBLIC VOLATILE — verdict byte, 0=unknown / 1=fluid-free
//                       / 2=has-fluids. Zero-init is NEVER served as a verdict
//                       (unknown forces the lazy scan; fail-dominant = false
//                       miss, never a false HIT).
//   crusstyFfGen : I  PUBLIC VOLATILE — the demux MUTATION-epoch (crusstyGen,
//                       ±2 per mutation: prologue+epilogue) the verdict was
//                       computed against. NOT crusstySnapGen.
// Verdict protocol lives in FluidOps: ff==1 && ffGen==states.crusstyGen ->
// free-HIT; ff==2 -> miss; else lazy 4096-cell scan publishing (ffGen, ff)
// in that write order. This patcher is pure field-splice: bodies untouched,
// no Code edits, idempotent, fail-closed on any shape mismatch.
// ---------------------------------------------------------------------------
pub const SECTION_CLASS: &str = "net/minecraft/world/level/chunk/LevelChunkSection";
pub const F_FF: &str = "crusstyFf";
pub const F_FFGEN: &str = "crusstyFfGen";

pub fn patch_section_ff(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout".to_string())?;
    let this_name = this_class_name(&layout)
        .ok_or_else(|| "cannot resolve this_class name".to_string())?;
    if this_name != SECTION_CLASS {
        return Err(format!("unexpected class {this_name}"));
    }
    let mut pool = layout.pool;

    // Idempotency: a pool carrying the injected names is already patched.
    if pool.find_utf8(F_FFGEN).is_some() {
        return Ok(bytes.to_vec());
    }

    // Fail-closed probes before any mutation: the states container field
    // must exist (kernel-rename guard; FluidOps pairs ff with states.gen).
    pool.find_utf8("states")
        .ok_or("states absent from pool (kernel rename?)".to_string())?;

    // ---- constant pool additions (append-only; utf8 ONLY — field_info
    // references name_idx/desc_idx, and no code in this class touches the
    // new fields (FluidOps reaches them via Unsafe offsets), so no
    // Fieldref/NameAndType entries are needed) ----
    pool.utf8(F_FF);
    pool.utf8("B");
    pool.utf8(F_FFGEN);
    pool.utf8("I");
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for SECTION-FF refs".into());
    }
    let n_ff = pool
        .find_utf8(F_FF)
        .ok_or("ff name utf8 missing".to_string())?;
    let d_ff = pool
        .find_utf8("B")
        .ok_or("B descriptor utf8 missing".to_string())?;
    let n_gen = pool
        .find_utf8(F_FFGEN)
        .ok_or("ffGen name utf8 missing".to_string())?;
    let d_gen = pool
        .find_utf8("I")
        .ok_or("I descriptor utf8 missing".to_string())?;

    // ---- field table: original entries + 2 injected field_info blocks ----
    let acc_ff = ACC_PUBLIC | ACC_VOLATILE;
    let mut fields_out = Vec::with_capacity(64);
    let orig_fields_count = u16_at(bytes, layout.fields_start).ok_or("fields_count oob")?;
    fields_out.extend_from_slice(&(orig_fields_count.saturating_add(2)).to_be_bytes());
    fields_out.extend_from_slice(&bytes[layout.fields_start + 2..layout.methods_start]);
    for (access, n, d) in [(acc_ff, n_ff, d_ff), (acc_ff, n_gen, d_gen)] {
        fields_out.extend_from_slice(&access.to_be_bytes());
        fields_out.extend_from_slice(&n.to_be_bytes());
        fields_out.extend_from_slice(&d.to_be_bytes());
        fields_out.extend_from_slice(&0u16.to_be_bytes()); // attributes_count
    }

    // ---- assemble: header + new cp + [cp_end..fields_start] + fields + tail ----
    let mut out = Vec::with_capacity(bytes.len() + 128);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    out.extend_from_slice(&pool.next.to_be_bytes()); // new cp_count
    out.extend_from_slice(&pool.serialize());
    // this_class/super_class/interfaces stay verbatim
    out.extend_from_slice(&bytes[layout.cp_end..layout.fields_start]);
    out.extend_from_slice(&fields_out);
    // methods + class attributes: NO method edits, verbatim tail
    out.extend_from_slice(&bytes[layout.methods_start..]);
    Ok(out)
}


#[cfg(test)]
mod real_noise {
    // G4 S7-12: the REAL ImprovedNoise class (extracted from the live
    // purpur-1.21.10 server jar, byte-identical to the hook-captured
    // original — 5691 bytes, major 65). This is the demonstrator's actual
    // target, and it REGRESSED the pool parser: it carries double constants
    // (d11..d21 fields), whose 2-slot JVMS entries overran the
    // seen-per-entry walk into tag 0 (fixed in Pool::parse). Kept as the
    // long/double-pool regression fixture.

    use super::*;
    use crate::classfile::*;

    const REAL_NOISE: &[u8] = include_bytes!("../tests/fixtures/ImprovedNoise_real.class");

    /// The real class must parse end-to-end (slot-counting pool walk) and
    /// expose its noise(DDDDD)D method to the retarget machinery.
    #[test]
    fn real_improvednoise_parses_and_finds_noise() {
        let layout = parse_layout(REAL_NOISE).expect("real ImprovedNoise must parse");
        assert_eq!(
            this_class_name(&layout).as_deref(),
            Some("net/minecraft/world/level/levelgen/synth/ImprovedNoise")
        );
        let name_idx = layout.pool.find_utf8("noise").expect("noise name in pool");
        let desc_idx = layout.pool.find_utf8("(DDDDD)D").expect("noise desc in pool");
        let m = find_method(REAL_NOISE, layout.methods_start, name_idx, desc_idx)
            .expect("noise(DDDDD)D present");
        let (code_start, code_len) =
            find_code_attr(REAL_NOISE, &layout.pool, &m).expect("noise has Code");
        let code = &REAL_NOISE[code_start..code_start + code_len];
        let sites = scan_invokestatics(code, code_start).expect("walk the real noise body");
        assert!(
            !sites.is_empty(),
            "the vanilla noise body carries invokestatic call sites (NoiseUtils.parity...)"
        );
        // The vanilla body references NoiseUtils — resolve at least one
        // site to a name triple (the name-based resolution path).
        let resolved: Vec<Option<(String, String, String)>> = sites
            .iter()
            .map(|(_, idx)| layout.pool.methodref_parts(*idx))
            .collect();
        assert!(resolved.iter().any(|r| r.is_some()), "vanilla sites resolve by name");
    }

    /// Retarget ON the real class: the vanilla body's NoiseUtils call site
    /// must survive an unrelated retarget (no-op NotFound) and a targeted
    /// retarget of a synthetic spec must rewrite exactly the matching sites
    /// while leaving the rest byte-identical.
    #[test]
    fn real_improvednoise_retarget_is_selective() {
        let out = retarget_invokestatic(
            REAL_NOISE,
            "noise",
            "(DDDDD)D",
            ("java/lang/Math", "sqrt", "(D)D"),
            ("crussty/test/X", "sqrt", "(D)D"),
        )
        .expect("clean run");
        assert_eq!(out.1, RetargetOutcome::NotFound, "unrelated spec is a no-op");
        assert_eq!(out.0, REAL_NOISE.to_vec(), "NotFound must not touch the bytes");
    }

    // ---- F1 BATCH-RNG body swap (ServerLevel.optimiseRandomTick) ----

    const SERVER: &[u8] = include_bytes!("../tests/fixtures/ServerLevel.class");

    /// The `run` descriptor `patch_optimise_random_tick` emits, computed the
    /// same way as in the patch fn (from this_name + the constant desc).
    const F1_RUN_DESC: &str = "(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/LevelChunk;ILca/spottedleaf/moonrise/common/util/SimpleThreadUnsafeRandom;)V";

    /// Locate optimiseRandomTick in a classfile and return its max_stack,
    /// max_locals, and bytecode — the shared verification walker for the F1
    /// tests below. NOTE: find_code_attr returns the BYTECODE start/length
    /// (data+8 per its contract); max_stack/max_locals live in the 4 bytes
    /// immediately before it.
    fn f1_code_of(bytes: &[u8]) -> (u16, u16, Vec<u8>) {
        let layout = parse_layout(bytes).expect("parse");
        let name_idx = layout
            .pool
            .find_utf8("optimiseRandomTick")
            .expect("name utf8 present");
        let desc_idx = layout
            .pool
            .find_utf8("(Lnet/minecraft/world/level/chunk/LevelChunk;I)V")
            .expect("desc utf8 present");
        let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
            .expect("optimiseRandomTick present");
        let (start, len) = find_code_attr(bytes, &layout.pool, &m).expect("Code attr");
        let code = bytes[start..start + len].to_vec();
        let ms = u16::from_be_bytes([bytes[start - 8], bytes[start - 7]]);
        let ml = u16::from_be_bytes([bytes[start - 6], bytes[start - 5]]);
        (ms, ml, code)
    }

    /// Round-trip on the REAL ServerLevel fixture: the swapped body is the
    /// exact 11-byte straight-line delegation; max_stack 4 / max_locals 3;
    /// the getfield operand resolves to ServerLevel.simpleRandom with the
    /// cfdump-verified descriptor; the invokestatic operand resolves to
    /// RandomTickOps.run with the helper's ECJ-compiled descriptor.
    #[test]
    fn f1_patch_roundtrip_verified() {
        let patched = patch_optimise_random_tick(SERVER).expect("patch");
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert_eq!(patched[..8], SERVER[..8], "version preserved");

        let (ms, ml, code) = f1_code_of(&patched);
        assert_eq!(code.len(), 11, "straight-line body is 11 bytes");
        assert_eq!(ms, 4, "max_stack = this,chunk,ticks,random");
        assert_eq!(ml, 3, "max_locals = this,chunk,ticks");
        // Opcode skeleton (operands at 5-6 and 8-9 are CP indices — resolved
        // by name below, never hard-coded).
        let skel = [
            code[0], code[1], code[2], code[3], code[4], code[7], code[10],
        ];
        let want = [0x2a, 0x2b, 0x1c, 0x2a, 0xb4, 0xb8, 0xb1];
        assert_eq!(skel, want, "opcode skeleton exact");

        // Operand resolution by NAME (never by index assumption).
        let layout = parse_layout(&patched).expect("re-parse patched");
        let f = layout
            .pool
            .fieldref_parts(u16::from_be_bytes([code[5], code[6]]))
            .expect("getfield operand resolves");
        assert_eq!(
            f,
            (
                SERVER_LEVEL_CLASS.to_string(),
                "simpleRandom".to_string(),
                "Lca/spottedleaf/moonrise/common/util/SimpleThreadUnsafeRandom;".to_string()
            )
        );
        let r = layout
            .pool
            .methodref_parts(u16::from_be_bytes([code[8], code[9]]))
            .expect("invokestatic operand resolves");
        assert_eq!(
            r,
            (
                RANDOMTICK_OPS_CLASS.to_string(),
                "run".to_string(),
                F1_RUN_DESC.to_string()
            )
        );

        // The patched class carries an EMPTY StackMapTable on the method (no
        // branch targets -> 0 frames) and keeps a private-instance access.
        let name_idx = layout
            .pool
            .find_utf8("optimiseRandomTick")
            .expect("name kept");
        let desc_idx = layout
            .pool
            .find_utf8("(Lnet/minecraft/world/level/chunk/LevelChunk;I)V")
            .expect("desc kept");
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx).expect("method");
        assert_eq!(m.access, 0x0002, "access flags preserved (private instance)");

        // Dump for the runtime verifier gate (randomtick/verify_patched.sh:
        // a real HotSpot resolveClass() pass over these bytes — the byte-level
        // checks above cannot prove verifier legality, a JVM can).
        let out = std::env::temp_dir().join("ccrussty_patched_ServerLevel.class");
        std::fs::write(&out, &patched).expect("dump patched ServerLevel");
        eprintln!("wrote {} bytes to {}", patched.len(), out.display());
    }

    /// Idempotency: patch(patch(x)) == patch(x) — the second pass appends
    /// nothing (dedup Pool) and re-emits the identical body.
    #[test]
    fn f1_patch_is_idempotent() {
        let once = patch_optimise_random_tick(SERVER).expect("first");
        let twice = patch_optimise_random_tick(&once).expect("second");
        assert_eq!(once, twice, "double patch is byte-identical");
    }

    /// Fail-closed discipline: wrong class rejected before any mutation;
    /// truncated/hostile bytes rejected without panic (hook-delivery audit
    /// discipline — a panic on a class-load thread would abort the JVM).
    #[test]
    fn f1_patch_rejects_wrong_class_and_garbage() {
        let e = patch_optimise_random_tick(include_bytes!(
            "../tests/fixtures/SingleUserAreaMap.class"
        ))
        .expect_err("area_map is not ServerLevel");
        assert!(e.starts_with("unexpected class"));

        let e = patch_optimise_random_tick(&SERVER[..64]).expect_err("truncated header");
        assert!(!e.is_empty());
        let e = patch_optimise_random_tick(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 65, 0, 3, 1, 2])
            .expect_err("garbage pool");
        assert!(!e.is_empty());
        // Whole-pool truncation at every prefix must never panic.
        for cut in [10usize, 100, 1000, 10000, SERVER.len() - 1] {
            let _ = patch_optimise_random_tick(&SERVER[..cut]);
        }
    }

    // ---- F2 BRAIN-ITERATORS body swap (Brain.startEachNonRunningBehavior) ----

    const BRAIN: &[u8] = include_bytes!("../tests/fixtures/Brain.class");

    /// The lens descriptor `patch_brain_start_each` emits, computed the same
    /// way as in the patch fn (fixed string — the helper's ECJ-compiled
    /// signature, see randomtick/src/BrainOps.java).
    const F2_LENS_DESC: &str = "(Ljava/util/Map;Ljava/util/Set;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)V";

    /// Locate startEachNonRunningBehavior in a classfile and return its
    /// max_stack, max_locals, and bytecode — shared walker for the F2 tests
    /// (mirror of f1_code_of).
    fn f2_code_of(bytes: &[u8]) -> (u16, u16, Vec<u8>) {
        let layout = parse_layout(bytes).expect("parse");
        let name_idx = layout
            .pool
            .find_utf8("startEachNonRunningBehavior")
            .expect("name utf8 present");
        let desc_idx = layout
            .pool
            .find_utf8(START_EACH_DESC)
            .expect("desc utf8 present");
        let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
            .expect("startEachNonRunningBehavior present");
        let (start, len) = find_code_attr(bytes, &layout.pool, &m).expect("Code attr");
        let code = bytes[start..start + len].to_vec();
        let ms = u16::from_be_bytes([bytes[start - 8], bytes[start - 7]]);
        let ml = u16::from_be_bytes([bytes[start - 6], bytes[start - 5]]);
        (ms, ml, code)
    }

    /// Round-trip on the REAL Brain fixture (run21 cfdump source, sha
    /// c08105a9…): the swapped body is the exact 14-byte straight-line
    /// delegation; max_stack 5 / max_locals 3; both getfield operands resolve
    /// to Brain's own fields with the CP-verified descriptors; the
    /// invokestatic operand resolves to BrainOps.startEachNonRunning.
    #[test]
    fn f2_patch_roundtrip_verified() {
        let patched = patch_brain_start_each(BRAIN).expect("patch");
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert_eq!(patched[..8], BRAIN[..8], "version preserved");

        let (ms, ml, code) = f2_code_of(&patched);
        assert_eq!(code.len(), 14, "straight-line body is 14 bytes");
        assert_eq!(ms, 5, "max_stack = vanilla value");
        assert_eq!(ml, 3, "max_locals = this,level,entity");
        // Opcode skeleton (operands at 2-3, 6-7, 11-12 are CP indices).
        let skel = [
            code[0], code[1], code[4], code[5], code[8], code[9], code[10], code[13],
        ];
        let want = [0x2a, 0xb4, 0x2a, 0xb4, 0x2b, 0x2c, 0xb8, 0xb1];
        assert_eq!(skel, want, "opcode skeleton exact");

        // Operand resolution by NAME (never by index assumption).
        let layout = parse_layout(&patched).expect("re-parse patched");
        let f1 = layout
            .pool
            .fieldref_parts(u16::from_be_bytes([code[2], code[3]]))
            .expect("getfield #1 operand resolves");
        assert_eq!(
            f1,
            (
                BRAIN_CLASS.to_string(),
                "availableBehaviorsByPriority".to_string(),
                "Ljava/util/Map;".to_string()
            )
        );
        let f2 = layout
            .pool
            .fieldref_parts(u16::from_be_bytes([code[6], code[7]]))
            .expect("getfield #2 operand resolves");
        assert_eq!(
            f2,
            (
                BRAIN_CLASS.to_string(),
                "activeActivities".to_string(),
                "Ljava/util/Set;".to_string()
            )
        );
        let r = layout
            .pool
            .methodref_parts(u16::from_be_bytes([code[11], code[12]]))
            .expect("invokestatic operand resolves");
        assert_eq!(
            r,
            (
                BRAIN_OPS_CLASS.to_string(),
                "startEachNonRunning".to_string(),
                F2_LENS_DESC.to_string()
            )
        );

        // The patched class carries an EMPTY StackMapTable on the method (no
        // branch targets -> 0 frames) and keeps the private-instance access.
        let name_idx = layout
            .pool
            .find_utf8("startEachNonRunningBehavior")
            .expect("name kept");
        let desc_idx = layout.pool.find_utf8(START_EACH_DESC).expect("desc kept");
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx).expect("method");
        assert_eq!(m.access, 0x0002, "access flags preserved (private instance)");

        // Dump for the runtime verifier gate (randomtick/verify_brain_patched.sh:
        // a real HotSpot resolveClass() pass over these bytes).
        let out = std::env::temp_dir().join("ccrussty_patched_Brain.class");
        std::fs::write(&out, &patched).expect("dump patched Brain");
        eprintln!("wrote {} bytes to {}", patched.len(), out.display());
    }

    /// Idempotency: patch(patch(x)) == patch(x).
    #[test]
    fn f2_patch_is_idempotent() {
        let once = patch_brain_start_each(BRAIN).expect("first");
        let twice = patch_brain_start_each(&once).expect("second");
        assert_eq!(once, twice, "double patch is byte-identical");
    }

    /// Fail-closed discipline: wrong class rejected before any mutation;
    /// truncated/hostile bytes rejected without panic (hook-delivery audit
    /// discipline — a panic on a class-load thread would abort the JVM).
    #[test]
    fn f2_patch_rejects_wrong_class_and_garbage() {
        let e = patch_brain_start_each(SERVER).expect_err("ServerLevel is not Brain");
        assert!(e.starts_with("unexpected class"));
        let e = patch_brain_start_each(include_bytes!(
            "../tests/fixtures/SingleUserAreaMap.class"
        ))
        .expect_err("area_map is not Brain");
        assert!(e.starts_with("unexpected class"));

        let e = patch_brain_start_each(&BRAIN[..64]).expect_err("truncated header");
        assert!(!e.is_empty());
        let e = patch_brain_start_each(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 65, 0, 3, 1, 2])
            .expect_err("garbage pool");
        assert!(!e.is_empty());
        // Whole-pool truncation at every prefix must never panic.
        for cut in [10usize, 100, 1000, 10000, BRAIN.len() - 1] {
            let _ = patch_brain_start_each(&BRAIN[..cut]);
        }
    }

    // ---- F3 LEVELTICKS-READS body swaps (runCollectedTicks + tickBlock) ----

    const LEVELTICKS: &[u8] = include_bytes!("../tests/fixtures/LevelTicks.class");

    /// Shared verification walker for the F3 tests: locate `name`/`desc` in
    /// the classfile and return (max_stack, max_locals, bytecode). Same
    /// contract as [`f1_code_of`] — find_code_attr points at the bytecode,
    /// max_stack/max_locals sit 8/6 bytes before it.
    fn f3_code_of(bytes: &[u8], name: &str, desc: &str) -> (u16, u16, Vec<u8>) {
        let layout = parse_layout(bytes).expect("parse");
        let name_idx = layout.pool.find_utf8(name).expect("name utf8 present");
        let desc_idx = layout.pool.find_utf8(desc).expect("desc utf8 present");
        let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
            .expect("target method present");
        let (start, len) = find_code_attr(bytes, &layout.pool, &m).expect("Code attr");
        let code = bytes[start..start + len].to_vec();
        let ms = u16::from_be_bytes([bytes[start - 8], bytes[start - 7]]);
        let ml = u16::from_be_bytes([bytes[start - 6], bytes[start - 5]]);
        (ms, ml, code)
    }

    /// Round-trip on the REAL fixtures: runCollectedTicks -> exact 6-byte
    /// straight line (max_stack 2 / max_locals 2), tickBlock -> exact 7-byte
    /// straight line (max_stack 3 / max_locals 3); operands resolve BY NAME
    /// to TickBlockOps with the ECJ-compiled descriptors; private access
    /// preserved. Dumps both for the HotSpot verifier gate
    /// (randomtick/verify_f3_patched.sh).
    #[test]
    fn f3_patch_roundtrip_verified() {
        // --- LevelTicks.runCollectedTicks (6 bytes) ---
        let patched = patch_run_collected_ticks(LEVELTICKS).expect("patch");
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert_eq!(patched[..8], LEVELTICKS[..8], "version preserved");

        let (ms, ml, code) = f3_code_of(&patched, "runCollectedTicks", RUN_COLLECTED_DESC);
        assert_eq!(code.len(), 6, "straight-line body is 6 bytes");
        assert_eq!(ms, 2, "max_stack = receiver,consumer");
        assert_eq!(ml, 2, "max_locals = this,BiConsumer");
        let skel = [code[0], code[1], code[2], code[5]];
        assert_eq!(skel, [0x2a, 0x2b, 0xb8, 0xb1], "opcode skeleton exact");

        let layout = parse_layout(&patched).expect("re-parse patched");
        let r = layout
            .pool
            .methodref_parts(u16::from_be_bytes([code[3], code[4]]))
            .expect("invokestatic operand resolves");
        assert_eq!(
            r,
            (
                TICKBLOCK_OPS_CLASS.to_string(),
                "runCollectedTicks".to_string(),
                RUN_COLLECTED_OPS_DESC.to_string()
            )
        );
        let name_idx = layout.pool.find_utf8("runCollectedTicks").expect("name kept");
        let desc_idx = layout.pool.find_utf8(RUN_COLLECTED_DESC).expect("desc kept");
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx).expect("method");
        assert_eq!(m.access, 0x0002, "access flags preserved (private instance)");

        let out = std::env::temp_dir().join("ccrussty_patched_LevelTicks.class");
        std::fs::write(&out, &patched).expect("dump patched LevelTicks");
        eprintln!("wrote {} bytes to {}", patched.len(), out.display());

        // --- ServerLevel.tickBlock (7 bytes) ---
        let patched = patch_tick_block(SERVER).expect("patch");
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert_eq!(patched[..8], SERVER[..8], "version preserved");

        let (ms, ml, code) = f3_code_of(&patched, "tickBlock", TICK_BLOCK_DESC);
        assert_eq!(code.len(), 7, "straight-line body is 7 bytes (3 loads + 3B invokestatic + return)");
        assert_eq!(ms, 3, "max_stack = level,pos,block");
        assert_eq!(ml, 3, "max_locals = this,BlockPos,Block");
        let skel = [code[0], code[1], code[2], code[3], code[6]];
        assert_eq!(skel, [0x2a, 0x2b, 0x2c, 0xb8, 0xb1], "opcode skeleton exact");

        let layout = parse_layout(&patched).expect("re-parse patched");
        let r = layout
            .pool
            .methodref_parts(u16::from_be_bytes([code[4], code[5]]))
            .expect("invokestatic operand resolves");
        assert_eq!(
            r,
            (
                TICKBLOCK_OPS_CLASS.to_string(),
                "tickBlock".to_string(),
                TICK_BLOCK_OPS_DESC.to_string()
            )
        );
        let name_idx = layout.pool.find_utf8("tickBlock").expect("name kept");
        let desc_idx = layout.pool.find_utf8(TICK_BLOCK_DESC).expect("desc kept");
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx).expect("method");
        assert_eq!(m.access, 0x0002, "access flags preserved (private instance)");

        let out = std::env::temp_dir().join("ccrussty_patched_ServerLevel_F3.class");
        std::fs::write(&out, &patched).expect("dump patched ServerLevel (F3-only)");
        eprintln!("wrote {} bytes to {}", patched.len(), out.display());
    }

    /// Idempotency: patch(patch(x)) == patch(x), both F3 functions.
    #[test]
    fn f3_patch_is_idempotent() {
        let once = patch_run_collected_ticks(LEVELTICKS).expect("first");
        let twice = patch_run_collected_ticks(&once).expect("second");
        assert_eq!(once, twice, "double runCollectedTicks patch is byte-identical");

        let once = patch_tick_block(SERVER).expect("first");
        let twice = patch_tick_block(&once).expect("second");
        assert_eq!(once, twice, "double tickBlock patch is byte-identical");

        let once = patch_collect_ticks(LEVELTICKS).expect("first");
        let twice = patch_collect_ticks(&once).expect("second");
        assert_eq!(once, twice, "double collectTicks patch is byte-identical");
    }

    /// F3-queue roundtrip (S7-117): collectTicks -> exact 9-byte straight line
    /// (max_stack 5 / max_locals 5), operand resolves BY NAME to
    /// TickBlockOps.collectTicks with the wide aload 4; private access kept.
    #[test]
    fn f3_collect_roundtrip_verified() {
        let patched = patch_collect_ticks(LEVELTICKS).expect("patch");
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert_eq!(patched[..8], LEVELTICKS[..8], "version preserved");

        let (ms, ml, code) = f3_code_of(
            &patched,
            "collectTicks",
            "(JILnet/minecraft/util/profiling/ProfilerFiller;)V",
        );
        assert_eq!(code.len(), 9, "straight-line body is 9 bytes");
        assert_eq!(ms, 5, "max_stack = receiver+long(2)+int+filler");
        assert_eq!(ml, 5, "max_locals = this,long(2),int,filler");
        let skel = [code[0], code[1], code[2], code[3], code[4], code[5], code[8]];
        assert_eq!(
            skel,
            [0x2a, 0x1f, 0x1d, 0x19, 0x04, 0xb8, 0xb1],
            "opcode skeleton exact (wide aload 4)"
        );

        let layout = parse_layout(&patched).expect("re-parse patched");
        let r = layout
            .pool
            .methodref_parts(u16::from_be_bytes([code[6], code[7]]))
            .expect("invokestatic operand resolves");
        assert_eq!(
            r,
            (
                TICKBLOCK_OPS_CLASS.to_string(),
                "collectTicks".to_string(),
                "(Lnet/minecraft/world/ticks/LevelTicks;JILnet/minecraft/util/profiling/ProfilerFiller;)V"
                    .to_string()
            )
        );
        let name_idx = layout.pool.find_utf8("collectTicks").expect("name kept");
        let desc_idx = layout
            .pool
            .find_utf8("(JILnet/minecraft/util/profiling/ProfilerFiller;)V")
            .expect("desc kept");
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx).expect("method");
        assert_eq!(m.access, 0x0002, "access flags preserved (private instance)");
    }

    /// LevelTicks composition (S7-117): BOTH F3 LevelTicks bodies alive after
    /// the tickhook compose chain (runCollectedTicks then collectTicks);
    /// deterministic + idempotent — same property f3_serverlevel_composes_with_f1
    /// pins for ServerLevel.
    #[test]
    fn f3_levelticks_composes_drain_and_queue() {
        let compose = |bytes: &[u8]| -> Vec<u8> {
            let b = patch_run_collected_ticks(bytes).expect("drain compose");
            patch_collect_ticks(&b).expect("queue compose")
        };
        let composed = compose(LEVELTICKS);

        let (_, _, drain) = f3_code_of(
            &composed,
            "runCollectedTicks",
            "(Ljava/util/function/BiConsumer;)V",
        );
        assert_eq!(drain.len(), 6, "F3 drain body alive in composition");
        let (_, _, queue) = f3_code_of(
            &composed,
            "collectTicks",
            "(JILnet/minecraft/util/profiling/ProfilerFiller;)V",
        );
        assert_eq!(queue.len(), 9, "F3 queue body alive in composition");

        assert_eq!(composed, compose(LEVELTICKS), "compose deterministic");
        assert_eq!(composed, compose(&composed), "compose idempotent on composed input");

        // Dump the COMPOSED image for the runtime verifier gate
        // (randomtick/verify_f3_patched.sh prefers this over the drain-only dump).
        let out = std::env::temp_dir().join("ccrussty_patched_LevelTicks_F3full.class");
        std::fs::write(&out, &composed).expect("dump composed LevelTicks");
        eprintln!("wrote {} bytes to {}", composed.len(), out.display());
    }

    /// Fail-closed discipline: wrong class rejected before any mutation;
    /// truncated/hostile bytes rejected without panic.
    #[test]
    fn f3_patch_rejects_wrong_class_and_garbage() {
        // Cross-class: runCollectedTicks patch refuses everything but LevelTicks.
        let e = patch_run_collected_ticks(SERVER).expect_err("ServerLevel is not LevelTicks");
        assert!(e.starts_with("unexpected class"));
        let e = patch_run_collected_ticks(BRAIN).expect_err("Brain is not LevelTicks");
        assert!(e.starts_with("unexpected class"));
        // Cross-class: tickBlock patch refuses everything but ServerLevel.
        let e = patch_tick_block(LEVELTICKS).expect_err("LevelTicks is not ServerLevel");
        assert!(e.starts_with("unexpected class"));
        let e = patch_tick_block(include_bytes!(
            "../tests/fixtures/SingleUserAreaMap.class"
        ))
        .expect_err("area_map is not ServerLevel");
        assert!(e.starts_with("unexpected class"));

        // Missing method: LevelTicks without runCollectedTicks is not the
        // kernel build we verified — probe fails closed (garbage-pool path).
        let e = patch_run_collected_ticks(&LEVELTICKS[..64]).expect_err("truncated header");
        assert!(!e.is_empty());
        let e = patch_tick_block(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 65, 0, 3, 1, 2])
            .expect_err("garbage pool");
        assert!(!e.is_empty());
        // Whole-pool truncation at every prefix must never panic.
        for cut in [10usize, 100, 1000, 10000, LEVELTICKS.len() - 1] {
            let _ = patch_run_collected_ticks(&LEVELTICKS[..cut]);
        }
        for cut in [10usize, 100, 1000, 10000, SERVER.len() - 1] {
            let _ = patch_tick_block(&SERVER[..cut]);
        }
    }

    /// F1+F3 COHABITATION (the ServerLevel seam): both hooks target
    /// ServerLevel, and the F3 retransform re-runs the chain over the
    /// ORIGINAL bytes — so tickhook's ServerLevel callback re-applies the F1
    /// patch (idempotent) before the F3 patch. These assertions pin the
    /// property the runtime relies on:
    ///   1. composing F1 then F3 leaves BOTH bodies swapped correctly;
    ///   2. the composed result re-applied through the same chain (the
    ///      retransform cycle) is byte-identical — no patch is lost or
    ///      duplicated by repeated dispatch.
    #[test]
    fn f3_serverlevel_composes_with_f1() {
        // The composed chain tickhook.rs runs during the F3 retransform:
        // optimise (re-apply, idempotent) then tick_block, over the ORIGINAL
        // fixture bytes.
        let compose = |bytes: &[u8]| -> Vec<u8> {
            let b = patch_optimise_random_tick(bytes).expect("F1 compose");
            patch_tick_block(&b).expect("F3 compose")
        };
        let composed = compose(SERVER);

        // Both bodies present and exact.
        let (ms, ml, code) = f3_code_of(&composed, "tickBlock", TICK_BLOCK_DESC);
        assert_eq!(code.len(), 7, "F3 tickBlock body present in composition");
        assert_eq!((ms, ml), (3, 3));
        let (_, _, code) = f3_code_of(
            &composed,
            "optimiseRandomTick",
            "(Lnet/minecraft/world/level/chunk/LevelChunk;I)V",
        );
        assert_eq!(code.len(), 11, "F1 optimiseRandomTick body ALIVE in composition");

        // Retransform cycle: re-running the composed chain over the ORIGINAL
        // bytes again yields the identical image (dedup CP + idempotent
        // bodies) — repeated dispatch neither loses nor duplicates patches.
        let recomposed = compose(SERVER);
        assert_eq!(composed, recomposed, "compose chain is deterministic");
        let c2 = compose(&composed); // input = already-composed bytes (model where retransform passes current bytes)
        assert_eq!(composed, c2, "compose is idempotent on composed input");

        // Dump the COMPOSED image for the runtime verifier gate
        // (randomtick/verify_f3_patched.sh: a real HotSpot resolveClass()
        // pass over exactly the bytes the runtime will carry).
        let out = std::env::temp_dir().join("ccrussty_patched_ServerLevel_F1F3.class");
        std::fs::write(&out, &composed).expect("dump composed ServerLevel");
        eprintln!("wrote {} bytes to {}", composed.len(), out.display());
    }

    // ---- PALETTED-DEMUX (S7-131) ----
    const PALETTED_REAL: &[u8] = include_bytes!("../tests/fixtures/PalettedContainer.class");

    #[test]
    fn paletted_patch_roundtrip() {
        let patched = patch_paletted_container(PALETTED_REAL).expect("patch");
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert_eq!(patched[..8], PALETTED_REAL[..8]);

        // structural re-parse: valid classfile, grew fields and cp
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, cp_end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let mut p = cp_end + 6;
        let iface_count = usize::from(u16::from_be_bytes([patched[p], patched[p + 1]]));
        p += 2 + 2 * iface_count;
        let fields_count = usize::from(u16::from_be_bytes([patched[p], patched[p + 1]]));
        // original fields count from the fixture layout
        let (opool, ocp_end) = Pool::parse(PALETTED_REAL, 10, u16::from_be_bytes([PALETTED_REAL[8], PALETTED_REAL[9]])).unwrap();
        let mut q = ocp_end + 6;
        let oif = usize::from(u16::from_be_bytes([PALETTED_REAL[q], PALETTED_REAL[q + 1]]));
        q += 2 + 2 * oif;
        let ofields = usize::from(u16::from_be_bytes([PALETTED_REAL[q], PALETTED_REAL[q + 1]]));
        assert_eq!(fields_count, ofields + 4, "4 fields injected");

        // injected field names present in the new pool
        for n in ["crusstySnap", "crusstySnapGen", "crusstyGen", "crusstyMiss"] {
            assert!(pool.find_utf8(n).is_some(), "{n} utf8 present");
        }

        // idempotency: second pass is a no-op
        let twice = patch_paletted_container(&patched).expect("patch twice");
        assert_eq!(twice, patched, "patch(patch(x)) == patch(x)");

        // artifact for the Ops build (stub jar) + JVM harness
        if std::env::var("CRUSSTY_PALETTE_ARTIFACTS").is_ok() {
            std::fs::create_dir_all("tests/out").unwrap();
            std::fs::write("tests/out/PalettedContainer.patched.class", &patched).unwrap();
        }
    }
}

#[cfg(test)]
mod alloc_diet {
    // S7-133 / TASK-269: REAL kernel classes (purpur-1.21.10.jar, hook-byte
    // identical), extracted 2026-09-18 for the alloc-diet patchers.
    const LIVING: &[u8] = include_bytes!("../tests/fixtures/LivingEntity.class");
    const COLLISION: &[u8] = include_bytes!("../tests/fixtures/CollisionUtil.class");

    use crate::classfile::*;

    #[test]
    fn push_entities_retargets_exactly_one_site() {
        let (patched, outcome) = patch_push_entities(LIVING).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "exactly one getPushableEntities site in pushEntities"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // The CODE attribute length is preserved (no branch/StackMapTable
        // churn); the FILE grows by the appended CP entries (Methodref +
        // utf8s) — both expected.
        assert!(patched.len() >= LIVING.len());
    }

    #[test]
    fn push_entities_site_resolves_to_ops_bridge() {
        let (patched, _) = patch_push_entities(LIVING).expect("patch");
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        // The bridge Methodref must exist and resolve by name.
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/world/entity/EntityQueryOps"
                && t.1 == "pushables"
                && t.2 == "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;"),
            "bridge Methodref appended"
        );
    }

    #[test]
    fn push_entities_idempotent() {
        let (patched, _) = patch_push_entities(LIVING).expect("patch");
        let (again, outcome) = patch_push_entities(&patched).expect("repatch");
        assert_eq!(outcome, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn push_entities_wrong_class_fails_closed() {
        // A class without pushEntities must NOT be patched.
        // A class without pushEntities: NotFound outcome, ORIGINAL bytes,
        // no pool growth (fail-closed to vanilla).
        let (out, outcome) = patch_push_entities(include_bytes!(
            "../tests/fixtures/PalettedContainer.class"
        ))
        .expect("notfound path must not error");
        assert_eq!(outcome, RetargetOutcome::NotFound);
        assert_eq!(out, include_bytes!("../tests/fixtures/PalettedContainer.class").to_vec());
    }

    #[test]
    fn collision_temps_splices_exactly_one_site() {
        let patched = patch_collision_temps(COLLISION).expect("patch");
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // CODE length preserved; file grows by appended CP entries.
        assert!(patched.len() >= COLLISION.len());
    }

    #[test]
    fn collision_temps_site_resolves_to_ops_bridge() {
        let patched = patch_collision_temps(COLLISION).expect("patch");
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/world/entity/EntityQueryOps"
                && t.1 == "mutablePos"
                && t.2 == "()Lnet/minecraft/core/BlockPos$MutableBlockPos;"),
            "mutablePos Methodref appended"
        );
    }

    #[test]
    fn collision_temps_idempotent() {
        let patched = patch_collision_temps(COLLISION).expect("patch");
        let again = patch_collision_temps(&patched).expect("repatch");
        assert_eq!(again, patched, "repatch must be byte-identical (no sites left)");
    }

    #[test]
    fn collision_temps_wrong_class_fails_closed() {
        let e = patch_collision_temps(include_bytes!(
            "../tests/fixtures/Brain.class"
        ))
        .err()
        .expect("no method/no site => Err (fail closed)");
        assert!(
            e.contains("absent from pool")
                || e.contains("expected exactly one"),
            "{e}"
        );
    }

    /// Dump artifacts for the offline JVM-verifier harness (define-only
    /// smoke runs on the patched bytes; see scripts/verify_alloc_diet.sh).
    #[test]
    fn dump_patched_for_verifier() {
        let (living, _) = patch_push_entities(LIVING).expect("patch");
        let collision = patch_collision_temps(COLLISION).expect("patch");
        std::fs::create_dir_all("tests/out").unwrap();
        std::fs::write("tests/out/LivingEntity.patched.class", &living).unwrap();
        std::fs::write("tests/out/CollisionUtil.patched.class", &collision).unwrap();
    }
}


#[cfg(test)]
mod inside_cache {
    // S7-135 / TASK-271: REAL kernel Entity.class (purpur-1.21.10.jar,
    // hook-byte identical), extracted 2026-09-18 for the inside-cache patcher.
    const ENTITY: &[u8] = include_bytes!("../tests/fixtures/Entity_real.class");

    use crate::classfile::*;

    #[test]
    fn inside_cache_retargets_exactly_one_site() {
        let (patched, outcome) = patch_inside_cache(ENTITY).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "exactly one isAffectedByBlocks site in checkInsideBlocks(List,Collector)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // CODE length preserved; the file grows by the appended CP entries.
        assert!(patched.len() >= ENTITY.len());
    }

    #[test]
    fn inside_cache_site_resolves_to_ops_bridge() {
        let (patched, _) = patch_inside_cache(ENTITY).expect("patch");
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/world/entity/InsideBlockOps"
                && t.1 == "gate"
                && t.2 == "(Lnet/minecraft/world/entity/Entity;)Z"),
            "gate Methodref appended"
        );
        // The collector-field guard precondition (bridge Unsafe resolution).
        assert!(pool.find_utf8("insideEffectCollector").is_some());
    }

    #[test]
    fn inside_cache_idempotent() {
        let (patched, _) = patch_inside_cache(ENTITY).expect("patch");
        let (again, outcome) = patch_inside_cache(&patched).expect("repatch");
        assert_eq!(outcome, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn inside_cache_wrong_class_fails_closed() {
        // A class without the collector field fails the pool guard (Err);
        // a class with the field but no checkInsideBlocks(List,Collector)
        // hits the NotFound path (original bytes, no pool growth).
        match patch_inside_cache(include_bytes!(
            "../tests/fixtures/PalettedContainer.class"
        )) {
            Err(e) => assert!(e.contains("insideEffectCollector"), "{e}"),
            Ok((out, outcome)) => {
                assert_eq!(outcome, RetargetOutcome::NotFound);
                assert_eq!(
                    out,
                    include_bytes!("../tests/fixtures/PalettedContainer.class").to_vec()
                );
            }
        }
    }

    /// Dump artifacts for the offline JVM-verifier harness.
    #[test]
    fn dump_patched_for_verifier() {
        let (entity, _) = patch_inside_cache(ENTITY).expect("patch");
        std::fs::create_dir_all("tests/out").unwrap();
        std::fs::write("tests/out/Entity.patched.class", &entity).unwrap();
    }
}

#[cfg(test)]
mod flush_diet {
    // S7-137 / ARCH-ATTACK lever #4: REAL kernel
    // InsideBlockEffectApplier$StepBasedCollector.class (purpur-1.21.10.jar,
    // byte-identical to the hook-captured original — 5695 bytes), extracted
    // 2026-09-18 for the flush-diet patcher.
    const SBC: &[u8] = include_bytes!(
        "../tests/fixtures/InsideBlockEffectApplier$StepBasedCollector.class"
    );

    use crate::classfile::*;

    #[test]
    fn flush_diet_retargets_exactly_two_sites() {
        let (patched, outcome) = patch_flush_step(SBC).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 2 },
            "exactly two List.addAll sites in flushStep (javap offsets 41 and 114)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // CODE length preserved; the file grows by the appended CP entries.
        assert!(patched.len() >= SBC.len());
    }

    #[test]
    fn flush_diet_sites_resolve_to_ops_bridge() {
        let (patched, _) = patch_flush_step(SBC).expect("patch");
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/world/entity/FlushOps"
                && t.1 == "fladd"
                && t.2 == "(Ljava/util/List;Ljava/util/Collection;)Z"),
            "fladd Methodref appended"
        );
        // Kernel-rename guard precondition.
        assert!(pool.find_utf8("beforeEffectsInStep").is_some());
    }

    /// Bytecode shape of the patched flushStep: exactly two invokestatic
    /// sites resolving to the bridge, each immediately followed by 2 nop
    /// (former count/zero operand slots); no List.addAll invokeinterface
    /// remains. All other interface sites (Map.get/remove, List.add/clear)
    /// stay untouched.
    #[test]
    fn flush_diet_bytecode_shape() {
        let (patched, _) = patch_flush_step(SBC).expect("patch");
        let layout = parse_layout(&patched).expect("parse");
        let name_idx = layout.pool.find_utf8("flushStep").expect("name");
        let desc_idx = layout.pool.find_utf8("()V").expect("desc");
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx)
            .expect("flushStep present");
        let (code_start, code_len) =
            find_code_attr(&patched, &layout.pool, &m).expect("Code attr");
        let code = &patched[code_start..code_start + code_len];

        let mut fladd_sites = 0usize;
        let mut list_addall_left = 0usize;
        let mut pc = 0usize;
        while pc < code.len() {
            let op = code[pc];
            if op == 0xb8 {
                let idx = u16::from_be_bytes([code[pc + 1], code[pc + 2]]);
                if let Some(t) = layout.pool.methodref_parts(idx) {
                    if t.0 == "net/minecraft/world/entity/FlushOps" && t.1 == "fladd" {
                        fladd_sites += 1;
                        assert_eq!(
                            (&code[pc + 3], &code[pc + 4]),
                            (&0x00, &0x00),
                            "former count/zero slots must be nop"
                        );
                    }
                }
                pc += 3;
                continue;
            }
            if op == 0xb9 {
                let idx = u16::from_be_bytes([code[pc + 1], code[pc + 2]]);
                if let Some(t) = layout.pool.methodref_parts(idx) {
                    if t.0 == "java/util/List" && t.1 == "addAll" {
                        list_addall_left += 1;
                    }
                }
                pc += 5;
                continue;
            }
            let extra = opcode_extra(op, code, pc).expect("walk");
            pc += 1 + extra;
        }
        assert_eq!(fladd_sites, 2, "both addAll sites retargeted");
        assert_eq!(list_addall_left, 0, "no vanilla List.addAll site remains");
    }

    #[test]
    fn flush_diet_idempotent() {
        let (patched, _) = patch_flush_step(SBC).expect("patch");
        let (again, outcome) = patch_flush_step(&patched).expect("repatch");
        assert_eq!(outcome, RetargetOutcome::AlreadyPatched { sites: 2 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn flush_diet_wrong_class_fails_closed() {
        // A class without the collector fields fails the pool guard (Err);
        // a class with the field name but no flushStep()V hits NotFound
        // (original bytes, no pool growth).
        match patch_flush_step(include_bytes!(
            "../tests/fixtures/PalettedContainer.class"
        )) {
            Err(e) => assert!(e.contains("beforeEffectsInStep"), "{e}"),
            Ok((out, outcome)) => {
                assert_eq!(outcome, RetargetOutcome::NotFound);
                assert_eq!(
                    out,
                    include_bytes!("../tests/fixtures/PalettedContainer.class").to_vec()
                );
            }
        }
    }

    /// Dump artifacts for the offline JVM-verifier harness.
    #[test]
    fn dump_patched_for_verifier() {
        let (sbc, _) = patch_flush_step(SBC).expect("patch");
        std::fs::create_dir_all("tests/out").unwrap();
        std::fs::write("tests/out/StepBasedCollector.patched.class", &sbc).unwrap();
    }
}

#[cfg(test)]
mod fluid_gate {
    // S7-138 / ARCH-ATTACK lever #5: the REAL kernel Entity.class fixture
    // (same bytes as the inside_cache fixture — one entity, two levers).
    const ENTITY: &[u8] = include_bytes!("../tests/fixtures/Entity_real.class");

    use crate::classfile::*;

    #[test]
    fn fluid_gate_retargets_exactly_two_wrappers() {
        let (patched, outcome) = patch_fluid_gate(ENTITY).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 2 },
            "exactly one fgate site per wrapper (water + lava)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert!(patched.len() >= ENTITY.len());
    }

    #[test]
    fn fluid_gate_sites_resolve_to_ops_bridge() {
        let (patched, _) = patch_fluid_gate(ENTITY).expect("patch");
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/world/entity/FluidOps"
                && t.1 == "fgate"
                && t.2 == "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/tags/TagKey;D)Z"),
            "fgate Methodref appended"
        );
        assert!(pool.find_utf8("fluidHeight").is_some());
    }

    /// Idempotency: on already-patched bytes the wrapper bodies contain
    /// invokestatic fgate (NotFound per retarget_virtual_to_static) — the
    /// combined classifier must yield AlreadyPatched{2} byte-identically.
    #[test]
    fn fluid_gate_idempotent() {
        let (patched, _) = patch_fluid_gate(ENTITY).expect("patch");
        let (again, outcome) = patch_fluid_gate(&patched).expect("repatch");
        assert_eq!(outcome, RetargetOutcome::AlreadyPatched { sites: 2 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn fluid_gate_wrong_class_fails_closed() {
        match patch_fluid_gate(include_bytes!(
            "../tests/fixtures/PalettedContainer.class"
        )) {
            Err(e) => {
                assert!(
                    e.contains("absent from pool")
                        || e.contains("not found")
                        || e.contains("per wrapper"),
                    "{e}"
                );
            }
            Ok((_, outcome)) => assert!(matches!(
                outcome,
                RetargetOutcome::NotFound | RetargetOutcome::AlreadyPatched { .. }
            )),
        }
    }

    /// Dump artifacts for the offline JVM-verifier harness.
    #[test]
    fn dump_patched_for_verifier() {
        let (entity, _) = patch_fluid_gate(ENTITY).expect("patch");
        std::fs::create_dir_all("tests/out").unwrap();
        std::fs::write("tests/out/Entity.fluidpatched.class", &entity).unwrap();
    }
}

#[cfg(test)]
mod section_ff {
    use super::*;

    const SECTION: &[u8] = include_bytes!("../tests/fixtures/LevelChunkSection_real.class");

    /// The patch applies to the real kernel section, grows the file, and
    /// carries both injected names in the appended pool.
    #[test]
    fn section_ff_applies_and_grows() {
        let out = patch_section_ff(SECTION).expect("patch");
        assert!(out.len() > SECTION.len(), "append-only must grow");
        let latin = String::from_utf8_lossy(&out).into_owned();
        assert!(latin.contains("crusstyFfGen"), "gen name must be appended");
        assert!(latin.contains("crusstyFf"), "ff name must be appended");
        // Recorded S7-143 fixture identity: 15041 -> 15088 bytes.
        assert_eq!(SECTION.len(), 15041, "pristine fixture identity");
        assert_eq!(out.len(), 15088, "patched size must match 91fcd70 record");
    }

    /// patch(patch(x)) == patch(x) — the pool-probe idempotency contract.
    #[test]
    fn section_ff_idempotent() {
        let patched = patch_section_ff(SECTION).expect("patch");
        let again = patch_section_ff(&patched).expect("repatch");
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    /// Non-section class => hard Err (no partial mutation).
    #[test]
    fn section_ff_wrong_class_fails_closed() {
        match patch_section_ff(include_bytes!("../tests/fixtures/PalettedContainer.class")) {
            Err(e) => assert!(e.contains("unexpected class"), "{e}"),
            Ok(_) => panic!("must fail closed on foreign class"),
        }
    }

    /// Byte-shape audit: both injected field_info blocks carry
    /// PUBLIC|VOLATILE (0x0041) access, the right descriptors, and no
    /// attributes; original field entries are copied verbatim before them.
    #[test]
    fn section_ff_field_shape() {
        let out = patch_section_ff(SECTION).expect("patch");
        let layout = parse_layout(&out).expect("reparse patched bytes");
        let pool = &layout.pool;
        let n_ff = pool.find_utf8("crusstyFf").expect("ff name idx");
        let d_ff = pool.find_utf8("B").expect("B desc idx");
        let n_gen = pool.find_utf8("crusstyFfGen").expect("gen name idx");
        let d_gen = pool.find_utf8("I").expect("I desc idx");
        let count = u16_at(&out, layout.fields_start).expect("fields count");
        let mut p = layout.fields_start + 2;
        let mut found = 0;
        for _ in 0..count {
            let access = u16_at(&out, p).expect("access");
            let name_idx = u16_at(&out, p + 2).expect("name idx");
            let desc_idx = u16_at(&out, p + 4).expect("desc idx");
            let attrs = u16_at(&out, p + 6).expect("attrs");
            if name_idx == n_ff || name_idx == n_gen {
                assert_eq!(access, 0x0041, "injected field must be PUBLIC|VOLATILE");
                assert_eq!(attrs, 0, "injected field must carry no attributes");
                if name_idx == n_ff {
                    assert_eq!(desc_idx, d_ff, "ff must be B");
                } else {
                    assert_eq!(desc_idx, d_gen, "ffGen must be I");
                }
                found += 1;
            }
            // skip this field's attribute blocks
            p += 8;
            for _ in 0..attrs {
                let alen =
                    u32::from_be_bytes([out[p + 2], out[p + 3], out[p + 4], out[p + 5]]) as usize;
                p += 6 + alen;
            }
        }
        assert_eq!(found, 2, "exactly two injected fields expected");
    }

    /// Dump artifacts for the offline FluidFreeHarness (S7-143 protocol).
    #[test]
    fn dump_section_ff_for_verifier() {
        let out = patch_section_ff(SECTION).expect("patch");
        std::fs::create_dir_all("tests/out").unwrap();
        std::fs::write("tests/out/LevelChunkSection.patched.class", &out).unwrap();
    }
}

#[cfg(test)]
mod fluid_dirty {
    // S7-151 / ARCH-ATTACK lever #6: REAL kernel fixtures (same source jar
    // as every other fixture — patched-kernel e2992d63).
    const ENTITY: &[u8] = include_bytes!("../tests/fixtures/Entity_real.class");
    const LEVEL_CHUNK: &[u8] = include_bytes!("../tests/fixtures/LevelChunk_real.class");

    use crate::classfile::*;

    #[test]
    fn fluid_dirty_entity_retargets_exactly_two_wrappers() {
        let (patched, outcome) = patch_fluid_dirty_entity(ENTITY).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 2 },
            "exactly one scan site per wrapper (water + lava)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert!(patched.len() >= ENTITY.len());
    }

    #[test]
    fn fluid_dirty_entity_sites_resolve_to_bridge() {
        let (patched, _) = patch_fluid_dirty_entity(ENTITY).expect("patch");
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples
                .iter()
                .any(|t| t.0 == "net/minecraft/world/entity/FluidPushOps"
                    && t.1 == "scan"
                    && t.2
                        == "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/tags/TagKey;D)Z"),
            "scan Methodref appended"
        );
        assert!(pool.find_utf8("touchingUnloadedChunk").is_some());
    }

    #[test]
    fn fluid_dirty_entity_idempotent() {
        let (patched, _) = patch_fluid_dirty_entity(ENTITY).expect("patch");
        let (again, outcome) = patch_fluid_dirty_entity(&patched).expect("repatch");
        assert_eq!(outcome, RetargetOutcome::AlreadyPatched { sites: 2 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn fluid_dirty_entity_wrong_class_fails_closed() {
        match patch_fluid_dirty_entity(include_bytes!(
            "../tests/fixtures/PalettedContainer.class"
        )) {
            Err(e) => {
                assert!(
                    e.contains("absent from pool")
                        || e.contains("not found")
                        || e.contains("per wrapper"),
                    "{e}"
                );
            }
            Ok((_, outcome)) => assert!(matches!(
                outcome,
                RetargetOutcome::NotFound | RetargetOutcome::AlreadyPatched { .. }
            )),
        }
    }

    #[test]
    fn fluid_dirty_levelchunk_retargets_exactly_one_site() {
        let (patched, outcome) = patch_fluid_dirty_levelchunk(LEVEL_CHUNK).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "exactly one LevelChunkSection.setBlockState site in LevelChunk.setBlockState"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert!(patched.len() >= LEVEL_CHUNK.len());
    }

    #[test]
    fn fluid_dirty_levelchunk_sites_resolve_to_bridge() {
        let (patched, _) = patch_fluid_dirty_levelchunk(LEVEL_CHUNK).expect("patch");
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/world/entity/FluidPushOps"
                && t.1 == "secWrite"
                && t.2 == "(Lnet/minecraft/world/level/chunk/LevelChunkSection;IIILnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;"),
            "secWrite Methodref appended"
        );
    }

    #[test]
    fn fluid_dirty_levelchunk_idempotent() {
        let (patched, _) = patch_fluid_dirty_levelchunk(LEVEL_CHUNK).expect("patch");
        let (again, outcome) = patch_fluid_dirty_levelchunk(&patched).expect("repatch");
        assert_eq!(outcome, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn fluid_dirty_levelchunk_wrong_class_fails_closed() {
        match patch_fluid_dirty_levelchunk(include_bytes!(
            "../tests/fixtures/PalettedContainer.class"
        )) {
            Err(e) => {
                assert!(
                    e.contains("absent from pool")
                        || e.contains("not found")
                        || e.contains("bad classfile layout"),
                    "{e}"
                );
            }
            Ok((_, outcome)) => assert!(matches!(
                outcome,
                RetargetOutcome::NotFound | RetargetOutcome::AlreadyPatched { .. }
            )),
        }
    }

    /// Compose sanity: inside + fluid_dirty retargets coexist on ONE Entity
    /// classfile (non-overlapping sites — the runtime chain serves all three
    /// levers from the same bytes: inside + fluid_free + fluid_dirty).
    #[test]
    fn fluid_dirty_composes_with_inside() {
        let (inside, _) = patch_inside_cache(ENTITY).expect("inside patch");
        let (both, outcome) = patch_fluid_dirty_entity(&inside).expect("fluid_dirty compose");
        assert_eq!(outcome, RetargetOutcome::Retargeted { sites: 2 });
        let cp_count = u16::from_be_bytes([both[8], both[9]]);
        let (pool, _end) = Pool::parse(&both, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(triples
            .iter()
            .any(|t| t.0 == "net/minecraft/world/entity/InsideBlockOps" && t.1 == "gate"));
        assert!(triples
            .iter()
            .any(|t| t.0 == "net/minecraft/world/entity/FluidPushOps" && t.1 == "scan"));
    }

    /// Dump artifacts for the offline JVM-verifier harness.
    #[test]
    fn dump_patched_for_verifier() {
        let (entity, _) = patch_fluid_dirty_entity(ENTITY).expect("patch");
        let (lc, _) = patch_fluid_dirty_levelchunk(LEVEL_CHUNK).expect("patch");
        std::fs::create_dir_all("tests/out").unwrap();
        std::fs::write("tests/out/Entity.fluiddirty.patched.class", &entity).unwrap();
        std::fs::write("tests/out/LevelChunk.fluiddirty.patched.class", &lc).unwrap();
    }
}

#[cfg(test)]
mod region_threads {
    // S7-156 / ARCH-ATTACK lever #7: REAL kernel fixtures (same source jar
    // as every other fixture — patched-kernel e2992d63).
    const SERVER: &[u8] = include_bytes!("../tests/fixtures/ServerLevel.class");
    const CALLBACKS: &[u8] =
        include_bytes!("../tests/fixtures/ServerLevel$EntityCallbacks_real.class");
    const LEVEL: &[u8] = include_bytes!("../tests/fixtures/Level_real.class");
    // S7-158b/d: same source jar (patched-kernel e2992d63).
    const CHUNKMAP: &[u8] = include_bytes!("../tests/fixtures/ChunkMap_real.class");
    const ENTITY: &[u8] = include_bytes!("../tests/fixtures/Entity_real.class");

    use crate::classfile::*;

    #[test]
    fn region_tracker_chunkmap_retargets_exactly_one_sweep_call() {
        let (patched, outcome) = patch_region_tracker_chunkmap(CHUNKMAP).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "ChunkMap.tick()V body is EXACTLY the single newTrackerTick call (javap 0-4)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/server/level/TrackerTickOps"
                && t.1 == "newTrackerTick"),
            "TrackerTickOps.newTrackerTick Methodref appended"
        );
        let (again, outcome2) = patch_region_tracker_chunkmap(&patched).expect("repatch");
        assert_eq!(outcome2, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn region_rng_entity_retargets_exactly_one_uuid_call() {
        let (patched, outcome) = patch_region_rng_entity(ENTITY).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "the ONLY Mth.createInsecureUUID(RandomSource) call site in the kernel (Entity ctor)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/util/RngOps"
                && t.1 == "createInsecureUUID"),
            "RngOps.createInsecureUUID Methodref appended"
        );
        let (again, outcome2) = patch_region_rng_entity(&patched).expect("repatch");
        assert_eq!(outcome2, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn region_hardening_wrong_class_fails_closed() {
        // tracker patcher on Entity: no newTrackerTick -> Err/NotFound
        match patch_region_tracker_chunkmap(ENTITY) {
            Err(e) => assert!(!e.is_empty(), "{e}"),
            Ok((_, outcome)) => assert!(matches!(outcome, RetargetOutcome::NotFound)),
        }
        // rng patcher on ChunkMap: createInsecureUUID absent -> Err/NotFound
        match patch_region_rng_entity(CHUNKMAP) {
            Err(e) => assert!(!e.is_empty(), "{e}"),
            Ok((_, outcome)) => assert!(matches!(outcome, RetargetOutcome::NotFound)),
        }
        // truncated / garbage never panics
        for cut in [10usize, 100, 1000, 10000, CHUNKMAP.len() - 1] {
            let _ = patch_region_tracker_chunkmap(&CHUNKMAP[..cut]);
        }
        for cut in [10usize, 100, 1000, 10000, ENTITY.len() - 1] {
            let _ = patch_region_rng_entity(&ENTITY[..cut]);
        }
    }

    #[test]
    fn region_tick_guardentity_retargets_exactly_one_pump() {
        let (patched, outcome) = patch_region_tick_guardentity(LEVEL).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "exactly one moonrise$midTickTasks site in Level.guardEntityTick \
             (the ONLY worker-reachable mid-tick pump — census S7-157b)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert!(patched.len() >= LEVEL.len());
    }

    #[test]
    fn region_tick_guardentity_site_resolves_to_bridge() {
        let (patched, _) = patch_region_tick_guardentity(LEVEL).expect("patch");
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/world/entity/RegionTickOps"
                && t.1 == "midTickTasks"
                && t.2 == "(Lnet/minecraft/world/level/Level;)V"),
            "midTickTasks Methodref appended with the receiver-prepended desc"
        );
    }

    #[test]
    fn region_tick_guardentity_idempotent() {
        let (patched, _) = patch_region_tick_guardentity(LEVEL).expect("patch");
        let (again, outcome) = patch_region_tick_guardentity(&patched).expect("repatch");
        assert_eq!(outcome, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn region_tick_guardentity_wrong_class_fails_closed() {
        // guardEntityTick absent from EntityCallbacks -> Err or NotFound
        match patch_region_tick_guardentity(CALLBACKS) {
            Err(e) => assert!(!e.is_empty(), "{e}"),
            Ok((_, outcome)) => assert_eq!(
                outcome,
                RetargetOutcome::NotFound,
                "no midTickTasks call site in EntityCallbacks"
            ),
        }
    }

    #[test]
    fn region_tick_serverlevel_retargets_exactly_one_foreach() {
        let (patched, outcome) = patch_region_tick_serverlevel(SERVER).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "exactly one EntityTickList.forEach site in ServerLevel.tick(BooleanSupplier)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert!(patched.len() >= SERVER.len());
    }

    #[test]
    fn region_tick_serverlevel_site_resolves_to_bridge() {
        let (patched, _) = patch_region_tick_serverlevel(SERVER).expect("patch");
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples
                .iter()
                .any(|t| t.0 == "net/minecraft/world/entity/RegionTickOps"
                    && t.1 == "forEach"
                    && t.2
                        == "(Lnet/minecraft/world/level/entity/EntityTickList;Ljava/util/function/Consumer;)V"),
            "forEach Methodref appended with the receiver-prepended desc"
        );
    }

    #[test]
    fn region_tick_serverlevel_idempotent() {
        let (patched, _) = patch_region_tick_serverlevel(SERVER).expect("patch");
        let (again, outcome) = patch_region_tick_serverlevel(&patched).expect("repatch");
        assert_eq!(outcome, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn region_tick_callbacks_retargets_exactly_one_add_and_one_remove() {
        let (patched, (o_add, o_rem)) = patch_region_tick_callbacks(CALLBACKS).expect("patch");
        assert_eq!(
            o_add,
            RetargetOutcome::Retargeted { sites: 1 },
            "the ONLY EntityTickList.add site in the kernel (onTickingStart)"
        );
        assert_eq!(
            o_rem,
            RetargetOutcome::Retargeted { sites: 1 },
            "the ONLY EntityTickList.remove site in the kernel (onTickingEnd)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // both retargets appended distinct pool entries
        assert!(patched.len() >= CALLBACKS.len());
    }

    #[test]
    fn region_tick_callbacks_sites_resolve_to_bridge() {
        let (patched, _) = patch_region_tick_callbacks(CALLBACKS).expect("patch");
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/world/entity/RegionTickOps"
                && t.1 == "onTickingStart"),
            "onTickingStart Methodref appended"
        );
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/world/entity/RegionTickOps"
                && t.1 == "onTickingEnd"),
            "onTickingEnd Methodref appended"
        );
    }

    #[test]
    fn region_tick_callbacks_idempotent() {
        let (patched, _) = patch_region_tick_callbacks(CALLBACKS).expect("patch");
        let (again, (o_add, o_rem)) = patch_region_tick_callbacks(&patched).expect("repatch");
        assert_eq!(
            o_add,
            RetargetOutcome::AlreadyPatched { sites: 1 },
            "second pass sees only the bridge onTickingStart"
        );
        assert_eq!(
            o_rem,
            RetargetOutcome::AlreadyPatched { sites: 1 },
            "second pass sees only the bridge onTickingEnd"
        );
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn region_tick_wrong_class_fails_closed() {
        // ServerLevel patcher on EntityCallbacks: tick absent -> Err or NotFound
        match patch_region_tick_serverlevel(CALLBACKS) {
            Err(e) => assert!(!e.is_empty(), "{e}"),
            Ok((_, outcome)) => {
                assert!(matches!(outcome, RetargetOutcome::NotFound))
            }
        }
        // Callbacks patcher on ServerLevel: onTickingStart absent -> Err/NotFound
        match patch_region_tick_callbacks(SERVER) {
            Err(e) => assert!(!e.is_empty(), "{e}"),
            Ok(_) => unreachable!("ServerLevel must not carry onTickingStart"),
        }
        // truncated / garbage never panics
        for cut in [10usize, 100, 1000, 10000, SERVER.len() - 1] {
            let _ = patch_region_tick_serverlevel(&SERVER[..cut]);
        }
        for cut in [10usize, 100, 1000, 10000, CALLBACKS.len() - 1] {
            let _ = patch_region_tick_callbacks(&CALLBACKS[..cut]);
        }
    }

    /// Cohabitation pin (the ServerLevel seam): F1 (optimiseRandomTick) + F3
    /// (tickBlock) then region-threads — the tick splice composes on top of
    /// the F1/F3 bytes; re-running the whole chain over the composed image
    /// (the retransform cycle) must be byte-identical.
    #[test]
    fn region_tick_composes_with_f1_f3() {
        let compose = |bytes: &[u8]| -> Vec<u8> {
            let b = patch_optimise_random_tick(bytes).expect("F1");
            let b = patch_tick_block(&b).expect("F3");
            // Retargeted on first sight, AlreadyPatched on re-runs — both
            // outcomes preserve the bytes (idempotent chain, like F1/F3).
            let (b, _) = patch_region_tick_serverlevel(&b).expect("region-threads");
            b
        };
        let composed = compose(SERVER);
        assert_eq!(composed, compose(&composed), "compose idempotent on composed input");

        // Dump for the offline RegionThreadsHarness structural gate.
        std::fs::create_dir_all("tests/out").unwrap();
        std::fs::write("tests/out/ServerLevel.regionthreads.patched.class", &composed).unwrap();
        let (cb, _) = patch_region_tick_callbacks(CALLBACKS).expect("callbacks");
        std::fs::write("tests/out/EntityCallbacks.regionthreads.patched.class", &cb).unwrap();
        // S7-157b: the mid-tick gate dump (Level.guardEntityTick retarget).
        let (lv, _) = patch_region_tick_guardentity(LEVEL).expect("guard");
        std::fs::write("tests/out/Level.regionthreads.patched.class", &lv).unwrap();
        // S7-158b/d: removal-safe tracker sweep + serialized UUID seeding.
        let (cm, _) = patch_region_tracker_chunkmap(CHUNKMAP).expect("tracker");
        std::fs::write("tests/out/ChunkMap.regionthreads.patched.class", &cm).unwrap();
        let (en, _) = patch_region_rng_entity(ENTITY).expect("rng");
        std::fs::write("tests/out/Entity.regionthreads.patched.class", &en).unwrap();
    }
}
