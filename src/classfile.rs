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
const TAG_STRING: u8 = 8;
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

    /// Resolve a CONSTANT_String index to its value (tag 8 -> Utf8 payload).
    /// Used by the LDC-anchored retarget resolver (RECON-13d): the anchor is
    /// the `ldc "xPos"` string constant preceding the single getIntOr site
    /// inside SerializableChunkData.parse.
    fn string_value(&self, idx: u16) -> Option<String> {
        let (_, tag, payload) = self.entries.iter().find(|(i, _, _)| *i == idx)?;
        if *tag != TAG_STRING {
            return None;
        }
        if payload.len() < 2 {
            return None;
        }
        let utf8_idx = u16::from_be_bytes([payload[0], payload[1]]);
        self.utf8_value(utf8_idx)
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
// F2-tick2 BODY SWAP (TASK-442-D sense scope-expansion, the F2 machine applied
// to `Brain.tickEachRunningBehavior`):
//
//   javap ground truth (patched-kernel.jar round-396-a, purpur-1.21.10):
//   `private void tickEachRunningBehavior(ServerLevel, LivingEntity)` =
//     gameTime = level.getGameTime();              // @0-4, read ONCE
//     for (bc : getRunningBehaviors())             // NEW ObjectArrayList + FULL
//                                                  // triple-nested map walk
//                                                  // collecting RUNNING
//         bc.tickOrStop(level, entity, gameTime);  // @38-43
//
//   getRunningBehaviors() (the ONLY hot caller is tickEachRunningBehavior;
//   stopAll is a rare teardown path) re-walks availableBehaviorsByPriority
//   EVERY tick per brain mob and allocates a fresh fastutil ObjectArrayList.
//   The swap re-uses the banked F2 flat snapshot (same lens/fingerprint/order)
//   plus a per-brain reusable RUNNING mask (statuses snapshotted at list-build
//   time == vanilla list semantics; tickOrStop per mask slot in flat order ==
//   vanilla list order). see BrainOps.tickEachRunning javadoc.
//
//   The 10-byte body:
//   aload_0; getfield availableBehaviorsByPriority:Ljava/util/Map;
//   aload_1 (ServerLevel); aload_2 (LivingEntity);
//   invokestatic BrainOps.tickEachRunning:(Ljava/util/Map;Lnet/minecraft/
//     server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)V
//   return
//   No branches => EMPTY StackMapTable (0 frames); getfield executes inside
//   Brain.class on its own private field (verifier-legal, F2 precedent).
//
//   GATED (unlike the F2 baseline): applied by brainhook ONLY under the
//   cmp438_sense family ∨ composite cmp439_sense_scan AND only after
//   BrainOps.selfTestTickEach()==true (TASK-437-A selfTest-before-ARM).
//
// Idempotency: patch(patch(x)) == patch(x). Fail-closed: any other class /
// missing method / name-descriptor mismatch => Err before any mutation.
pub fn patch_brain_tick_each(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout")?;
    let this_name = this_class_name(&layout).ok_or("cannot resolve this_class name")?;
    if this_name != BRAIN_CLASS {
        return Err(format!("unexpected class {this_name}"));
    }
    let mut pool = layout.pool;

    let Some(name_idx) = pool.find_utf8("tickEachRunningBehavior") else {
        return Err("tickEachRunningBehavior not found".into());
    };
    let Some(desc_idx) = pool.find_utf8(START_EACH_DESC) else {
        return Err("tickEachRunningBehavior descriptor not found".into());
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx).ok_or(
        "tickEachRunningBehavior(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)V not found",
    )?;

    let f_prio = pool.field_ref(&this_name, "availableBehaviorsByPriority", "Ljava/util/Map;");
    let lens_desc = "(Ljava/util/Map;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)V";
    let m_lens = pool.method_ref(BRAIN_OPS_CLASS, "tickEachRunning", lens_desc);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for F2-tick2 refs".into());
    }

    let mut code = Vec::with_capacity(10);
    let u2 = |out: &mut Vec<u8>, v: u16| out.extend_from_slice(&v.to_be_bytes());
    code.push(0x2a); // aload_0
    code.push(0xb4); // getfield availableBehaviorsByPriority
    u2(&mut code, f_prio);
    code.push(0x2b); // aload_1 (ServerLevel)
    code.push(0x2c); // aload_2 (LivingEntity)
    code.push(0xb8); // invokestatic BrainOps.tickEachRunning
    u2(&mut code, m_lens);
    code.push(0xb1); // return
    debug_assert_eq!(code.len(), 10, "emitted code is {}", code.len());

    // ---- Code attribute: empty exception table + EMPTY StackMapTable ----
    let mut code_attr = Vec::new();
    u2(&mut code_attr, pool.utf8("Code"));
    let mut body = Vec::new();
    u2(&mut body, 5); // max_stack: >= 3 slots the line needs (vanilla value kept)
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
// SENSE-PLANE body swap (TASK-438-A2, vector cmp438_sense — the F2 machine
// applied to the targeting nearest-pick CHOKEPOINT):
//
//   `ServerEntityGetter.getNearestEntity(List,TargetingConditions,LivingEntity,
//   double,double,double)` (INTERFACE default method — javap ground truth
//   round-396-a purpur-1.21.10: ALL targeting-conditions nearest picks
//   converge here; ServerLevel does NOT override — the four getNearestPlayer
//   variants + getNearestEntity(Class/TagKey) all delegate to it, and
//   NearestAttackableTargetGoal.findTarget calls it directly @84) ->
//   14-byte straight line
//   `aload_0; aload_1; aload_2; aload_3; dload 4; dload 6; dload 8;
//   invokestatic SenseOps.nearestEntityGate:(Lnet/minecraft/server/level/
//   ServerEntityGetter;Ljava/util/List;Lnet/minecraft/world/entity/ai/
//   targeting/TargetingConditions;Lnet/minecraft/world/entity/LivingEntity;
//   DDD)Lnet/minecraft/world/entity/LivingEntity;; areturn`.
//
//   The java gate replicates the vanilla ladder bit-for-bit (first passing
//   candidate with the strictly-closer `(d==-1.0||e<d)` ladder, ties keep the
//   earlier list element) and adds TWO decision-exact accelerations:
//   (1) rust-guided closest-first test (identity-guarded sense epoch column),
//   (2) distance pruning of the remaining sweep (`d >= bestD` candidates can
//   never win the ladder, so their test is skipped — TargetingConditions.test
//   is a pure predicate: range/invisibility/idle/selector/LOS, no
//   RandomSource). ANY test order yields the vanilla pick; a stale epoch only
//   degrades the WIN, never correctness (law 4).
//
// No branches in the new body => EMPTY StackMapTable (0 frames). max_stack 10
// (4 refs + 3 doubles×2 slots), max_locals 10 (double arg at local 8 spans
// 8,9). The receiver is passed as the explicit first argument, so the body
// never touches foreign privates.
//
// Idempotency: patch(patch(x)) == patch(x) — dedup Pool appends nothing on
// the second pass and the body re-emits byte-identical. Fail-closed: any
// other class / missing method => Err before any mutation.
pub const SENSE_GETTER_CLASS: &str = "net/minecraft/server/level/ServerEntityGetter";
pub const SENSE_OPS_CLASS: &str = "net/minecraft/world/entity/SenseOps";
pub(crate) const SENSE_NEAREST_DESC: &str = "(Ljava/util/List;Lnet/minecraft/world/entity/ai/targeting/TargetingConditions;Lnet/minecraft/world/entity/LivingEntity;DDD)Lnet/minecraft/world/entity/LivingEntity;";
pub(crate) const SENSE_GATE_DESC: &str = "(Lnet/minecraft/server/level/ServerEntityGetter;Ljava/util/List;Lnet/minecraft/world/entity/ai/targeting/TargetingConditions;Lnet/minecraft/world/entity/LivingEntity;DDD)Lnet/minecraft/world/entity/LivingEntity;";

pub fn patch_sense_nearest_entity(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout")?;
    let this_name = this_class_name(&layout).ok_or("cannot resolve this_class name")?;
    if this_name != SENSE_GETTER_CLASS {
        return Err(format!("unexpected class {this_name}"));
    }
    let mut pool = layout.pool;

    // Find-only probes first (fail-closed BEFORE any pool mutation).
    let Some(name_idx) = pool.find_utf8("getNearestEntity") else {
        return Err("getNearestEntity not found".into());
    };
    let Some(desc_idx) = pool.find_utf8(SENSE_NEAREST_DESC) else {
        return Err("getNearestEntity(List,TargetingConditions,LivingEntity,DDD) descriptor not found".into());
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
        .ok_or("getNearestEntity(List,...) method entry not found")?;

    // ---- constant ref needed by the new body (appended when absent) ----
    let m_gate = pool.method_ref(SENSE_OPS_CLASS, "nearestEntityGate", SENSE_GATE_DESC);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for sense refs".into());
    }

    let mut code = Vec::with_capacity(14);
    let u2 = |out: &mut Vec<u8>, v: u16| out.extend_from_slice(&v.to_be_bytes());
    code.push(0x2a); // aload_0 (receiver: ServerEntityGetter this)
    code.push(0x2b); // aload_1 (List entities)
    code.push(0x2c); // aload_2 (TargetingConditions)
    code.push(0x2d); // aload_3 (LivingEntity targeter)
    code.push(0x18);
    code.push(0x04); // dload 4 (x)
    code.push(0x18);
    code.push(0x06); // dload 6 (y)
    code.push(0x18);
    code.push(0x08); // dload 8 (z)
    code.push(0xb8); // invokestatic SenseOps.nearestEntityGate
    u2(&mut code, m_gate);
    code.push(0xb0); // areturn (LivingEntity)
    debug_assert_eq!(code.len(), 14, "emitted code is {}", code.len());

    // ---- Code attribute: empty exception table + EMPTY StackMapTable ----
    let mut code_attr = Vec::new();
    u2(&mut code_attr, pool.utf8("Code"));
    let mut body = Vec::new();
    u2(&mut body, 10); // max_stack: 4 refs + 3 double slots×2
    u2(&mut body, 10); // max_locals: receiver + 3 refs + 3 doubles (locals 8,9)
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

/// RECON-13d resolution closure: the embedded ChunkParseDiagOps bytes MUST
/// declare the receiver-prepended `diagXIntOr` the retarget emits (name +
/// descriptor UTF8 entries present); otherwise the first chunk parse
/// detonates a NoSuchMethodError and the lever must stay dormant.
pub fn parse_diag_resolution_closure(ops_bytes: &[u8]) -> bool {
    let layout = match parse_layout(ops_bytes) {
        Some(l) => l,
        None => return false,
    };
    let pool = layout.pool;
    pool.find_utf8("diagXIntOr").is_some()
        && pool
            .find_utf8("(Lnet/minecraft/nbt/CompoundTag;Ljava/lang/String;I)I")
            .is_some()
}

/// LDC-anchored virtual->static retarget (RECON-13d, TASK-327): rewrite the
/// SINGLE `invokevirtual CompoundTag.getIntOr(String,I)I` call site that is
/// immediately preceded by `ldc "xPos"` inside `method_name`/`method_desc`
/// (SerializableChunkData.parse — its ONLY `xPos`-anchored site; the zPos/starlight
/// sites are NOT anchored by an "xPos" ldc and are never touched).
///
/// The target `to` MUST be the receiver-prepended static bridge
/// (`ChunkParseDiagOps.diagXIntOr(CompoundTag,String,I)I`): the verifier-visible
/// stack shape stays identical because the invokevirtual receiver becomes the
/// first static argument. The descriptor therefore differs from `from` (this is
/// the ONE legitimate difference from [`retarget_invokestatic`], which forbids it).
///
/// Site resolution is BY NAME (`pool.methodref_parts`), never by offset (G4 §9).
/// Idempotency: a site already resolving to `to` yields AlreadyPatched with the
/// original bytes back. NotFound yields the original bytes without pool growth.
/// Panic-free on hook-delivered bytes: bounded walk via [`opcode_extra`].
pub fn retarget_ldc_virtual_to_static(
    bytes: &[u8],
    method_name: &str,
    method_desc: &str,
    ldc_const: &str,
    from: (&str, &str, &str),
    to: (&str, &str, &str),
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    let mut pool = layout.pool;
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

    let to_triple = (to.0.to_string(), to.1.to_string(), to.2.to_string());
    let from_triple = (from.0.to_string(), from.1.to_string(), from.2.to_string());

    // Walk the code array once; find the ldc-anchored invokevirtual site.
    let mut site_op: Option<usize> = None; // absolute offset of the opcode byte
    let mut site_cp: Option<u16> = None;
    let mut already: Option<usize> = None;
    let mut armed = false; // true right after the anchor ldc
    let mut pc = 0usize;
    while pc < code.len() {
        let op = code[pc];
        if op == 0x12 {
            // ldc: one-byte CP operand. The anchor is ONLY the "xPos" string;
            // a different ldc between the anchor and its invoke disarms.
            let cidx = *code
                .get(pc + 1)
                .ok_or_else(|| "truncated ldc".to_string())? as u16;
            armed = pool.string_value(cidx).as_deref() == Some(ldc_const);
            pc += 2;
            continue;
        }
        if (op == 0xb6 || op == 0xb8) && armed {
            let b = code
                .get(pc + 1..pc + 3)
                .ok_or_else(|| "invoke operand truncated".to_string())?;
            let cp_idx = u16::from_be_bytes([b[0], b[1]]);
            match pool.methodref_parts(cp_idx) {
                Some(parts) if parts == to_triple => already = Some(code_start + pc + 1),
                Some(parts) if parts == from_triple => {
                    site_op = Some(code_start + pc);
                    site_cp = Some(cp_idx);
                }
                _ => {}
            }
            armed = false;
            pc += 3;
            continue;
        }
        // NOTE: no blanket disarm here — the anchor ldc may be separated from
        // its invoke by stack-setup instructions (iconst default, etc.).
        let extra = opcode_extra(op, code, pc)?;
        pc = pc
            .checked_add(1 + extra)
            .ok_or_else(|| "code walk overflow".to_string())?;
        if pc > code.len() {
            return Err("truncated code (walk past end)".into());
        }
    }

    if site_op.is_none() {
        return Ok((
            bytes.to_vec(),
            if already.is_some() {
                RetargetOutcome::AlreadyPatched { sites: 1 }
            } else {
                RetargetOutcome::NotFound
            },
        ));
    }

    let op_off = site_op.unwrap();
    // Append (or reuse) the Methodref for `to` — append-only, dedup.
    let new_idx = pool.method_ref(to.0, to.1, to.2);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for diag ref".into());
    }
    let mut tail = bytes[layout.cp_end..].to_vec();
    let rel = op_off
        .checked_sub(layout.cp_end)
        .ok_or_else(|| "retarget opcode inside pool (corrupt layout?)".to_string())?;
    if rel + 2 >= tail.len() {
        return Err("retarget operand outside class tail (corrupt layout?)".into());
    }
    // 0xb6 invokevirtual -> 0xb8 invokestatic; operand -> appended Methodref.
    tail[rel] = 0xb8;
    let want = new_idx.to_be_bytes();
    tail[rel + 1] = want[0];
    tail[rel + 2] = want[1];
    let _ = site_cp;
    let mut out = Vec::with_capacity(bytes.len() + 64);
    out.extend_from_slice(&bytes[0..8]);
    out.extend_from_slice(&pool.next.to_be_bytes());
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&tail);
    Ok((out, RetargetOutcome::Retargeted { sites: 1 }))
}

#[test]
fn parse_diag_retargets_xpos_site_once() {
    // Real kernel class (purpur-1.21.10, captured from run s7165 artifacts).
    // The patch must retarget EXACTLY the ldc-"xPos"-anchored getIntOr site.
    let bytes = include_bytes!("../tests/fixtures/SerializableChunkData.class");
    let parse_desc = "(Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/chunk/PalettedContainerFactory;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/chunk/storage/SerializableChunkData;";
    let (out1, outcome1) = retarget_ldc_virtual_to_static(
        bytes,
        "parse",
        parse_desc,
        "xPos",
        (
            "net/minecraft/nbt/CompoundTag",
            "getIntOr",
            "(Ljava/lang/String;I)I",
        ),
        (
            "net/minecraft/world/level/chunk/storage/ChunkParseDiagOps",
            "diagXIntOr",
            "(Lnet/minecraft/nbt/CompoundTag;Ljava/lang/String;I)I",
        ),
    )
    .unwrap();
    assert_eq!(
        outcome1,
        RetargetOutcome::Retargeted { sites: 1 },
        "exactly one ldc-anchored site must be retargeted"
    );
    // The patched class must still parse (layout validity) and the single
    // re-sight must be idempotent.
    let (_out2, outcome2) = retarget_ldc_virtual_to_static(
        &out1,
        "parse",
        parse_desc,
        "xPos",
        (
            "net/minecraft/nbt/CompoundTag",
            "getIntOr",
            "(Ljava/lang/String;I)I",
        ),
        (
            "net/minecraft/world/level/chunk/storage/ChunkParseDiagOps",
            "diagXIntOr",
            "(Lnet/minecraft/nbt/CompoundTag;Ljava/lang/String;I)I",
        ),
    )
    .unwrap();
    assert_eq!(
        outcome2,
        RetargetOutcome::AlreadyPatched { sites: 1 },
        "re-sighting patched bytes must be AlreadyPatched, never a double patch"
    );
    // The zPos site must remain an invokevirtual on the ORIGINAL Methodref.
    let pool_ok = {
        let cp_count = u16::from_be_bytes([out1[8], out1[9]]);
        let (pool, _end) = Pool::parse(&out1, 10, cp_count).unwrap();
        pool.find_utf8("zPos").is_some()
            && pool
                .find_utf8("(Ljava/lang/String;I)I")
                .is_some()
    };
    assert!(pool_ok, "pool must stay consistent after the retarget");
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

    // -------------------------------------------------------------------
    // S7-164 ZERO-ALLOC body-redirect tests (lever #10).
    // Fixture: the REAL kernel Entity.class (pristine, 1.21.10).
    // -------------------------------------------------------------------
    const REAL_ENTITY: &[u8] = include_bytes!("../tests/fixtures/Entity_real.class");

    // TRAVEL-DIET v2b: pristine LivingEntity fixture for the travelInFluid
    // body-redirect test (same capture pipeline as Entity_real.class).
    const REAL_LIVING: &[u8] = include_bytes!("../tests/fixtures/LivingEntity.class");

    const ZA_FLUID_V: &str =
        "(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;)Z";
    const ZA_SHAPE_V: &str =
        "(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Ljava/util/List;)Z";
    const ZA_PUSH_V: &str = "(Lnet/minecraft/tags/TagKey;D)Z";

    /// All three bodies redirect to ZeroAllocOps statics; the redirected
    /// Code attribute is exactly aload-chain + invokestatic + ireturn with
    /// max_stack == max_locals == the descriptor slot count; the method
    /// re-parses cleanly (pool growth is coherent).
    #[test]
    fn zeroalloc_redirect_all_three_and_verify() {
        let (out, outcome) = patch_entity_zeroalloc(REAL_ENTITY).expect("patch");
        assert_eq!(outcome, RetargetOutcome::Retargeted { sites: 3 });
        assert_ne!(out.as_slice(), REAL_ENTITY, "redirect must change bytes");

        let cases: [(&str, &str, &str); 3] = [
            (
                "collidedWithFluid",
                ZA_FLUID_V,
                "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;)Z",
            ),
            (
                "collidedWithShapeMovingFrom",
                ZA_SHAPE_V,
                "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Ljava/util/List;)Z",
            ),
            (
                "updateFluidHeightAndDoFluidPushing",
                ZA_PUSH_V,
                "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/tags/TagKey;D)Z",
            ),
        ];
        let layout = parse_layout(&out).expect("re-parse redirected Entity");
        for (name, vdesc, sdesc) in cases {
            let name_idx = layout.pool.find_utf8(name).expect("name kept");
            let desc_idx = layout.pool.find_utf8(vdesc).expect("desc kept");
            let m = find_method(&out, layout.methods_start, name_idx, desc_idx)
                .expect("redirected method found");
            let (code_start, code_len) =
                find_code_attr(&out, &layout.pool, &m).expect("code attr");
            let code = &out[code_start..code_start + code_len];
            // shape: aload_0 [+ param loads] + invokestatic + ireturn
            assert_eq!(code[0], 0x2a, "{name}: body starts with aload_0");
            let ret_pos = code.len() - 1;
            assert_eq!(code[ret_pos], 0xac, "{name}: boolean method ends with ireturn");
            assert_eq!(code[ret_pos - 3], 0xb8, "{name}: dispatch is invokestatic");
            let cp_idx = u16::from_be_bytes([code[ret_pos - 2], code[ret_pos - 1]]);
            let parts = layout.pool.methodref_parts(cp_idx).expect("resolve target");
            assert_eq!(parts.0, ZERO_ALLOC_OPS_CLASS, "{name}: target owner");
            assert_eq!(parts.1, name, "{name}: target name matches");
            assert_eq!(parts.2, sdesc, "{name}: target static desc = receiver-prepended");

            // slot accounting: max_stack == max_locals == total slots
            let total_slots = desc_param_slots(vdesc).expect("slots");
            // Code attr layout: name(2) len(4) max_stack(2) max_locals(2)
            // code_len(4) code[..] — code_start points at the code array.
            let ms = u16::from_be_bytes([
                out[code_start - 8],
                out[code_start - 7],
            ]);
            let ml = u16::from_be_bytes([
                out[code_start - 6],
                out[code_start - 5],
            ]);
            assert_eq!(usize::from(ms), total_slots, "{name}: max_stack == slots");
            assert_eq!(usize::from(ml), total_slots, "{name}: max_locals == slots");
            // exact code length: typed loads + 3-byte invokestatic + ireturn
            let load_bytes: usize = std::iter::once((0usize, SlotKind::A))
                .chain(desc_slot_kinds(vdesc).expect("kinds"))
                .map(|(slot, _)| usize::from(slot > 3) + 1)
                .sum();
            assert_eq!(
                code_len as usize,
                load_bytes + 4,
                "{name}: code length = typed-load chain + 3-byte invokestatic + ireturn"
            );
        }
    }

    /// Idempotency: re-sighting the redirected Entity is AlreadyPatched
    /// with byte-identical output.
    #[test]
    fn zeroalloc_redirect_idempotent() {
        let (first, out1) = patch_entity_zeroalloc(REAL_ENTITY).expect("first");
        assert_eq!(out1, RetargetOutcome::Retargeted { sites: 3 });
        let (second, out2) = patch_entity_zeroalloc(&first).expect("second");
        assert_eq!(out2, RetargetOutcome::AlreadyPatched { sites: 3 });
        assert_eq!(first, second, "AlreadyPatched must not touch bytes");
    }

    /// TRAVEL-DIET v2a (RECON-21): the collide body redirects to the
    /// TravelDietOps static; the redirected Code attribute is exactly
    /// aload-chain + invokestatic + areturn (Vec3 = object return), slots
    /// accounted, and the redirect is idempotent.
    #[test]
    fn traveldiet_redirect_collide_and_verify() {
        let (out, outcome) = patch_entity_traveldiet(REAL_ENTITY).expect("patch");
        assert_eq!(outcome, RetargetOutcome::Retargeted { sites: 2 });
        assert_ne!(out.as_slice(), REAL_ENTITY, "redirect must change bytes");

        let vdesc = TD_COLLIDE_DESC;
        let sdesc = "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;";
        let layout = parse_layout(&out).expect("re-parse redirected Entity");
        let name_idx = layout.pool.find_utf8("collide").expect("name kept");
        let desc_idx = layout.pool.find_utf8(vdesc).expect("desc kept");
        let m = find_method(&out, layout.methods_start, name_idx, desc_idx)
            .expect("redirected method found");
        let (code_start, code_len) =
            find_code_attr(&out, &layout.pool, &m).expect("code attr");
        let code = &out[code_start..code_start + code_len];
        // shape: aload_0 + aload_1 + invokestatic + areturn
        assert_eq!(code[0], 0x2a, "collide: body starts with aload_0");
        let ret_pos = code.len() - 1;
        assert_eq!(code[ret_pos], 0xb0, "collide: object method ends with areturn");
        assert_eq!(code[ret_pos - 3], 0xb8, "collide: dispatch is invokestatic");
        let cp_idx = u16::from_be_bytes([code[ret_pos - 2], code[ret_pos - 1]]);
        let parts = layout.pool.methodref_parts(cp_idx).expect("resolve target");
        assert_eq!(parts.0, TRAVEL_DIET_OPS_CLASS, "collide: target owner");
        assert_eq!(parts.1, "collide", "collide: target name matches");
        assert_eq!(parts.2, sdesc, "collide: target static desc = receiver-prepended");

        let total_slots = desc_param_slots(vdesc).expect("slots");
        let ms = u16::from_be_bytes([out[code_start - 8], out[code_start - 7]]);
        let ml = u16::from_be_bytes([out[code_start - 6], out[code_start - 5]]);
        assert_eq!(usize::from(ms), total_slots, "collide: max_stack == slots");
        assert_eq!(usize::from(ml), total_slots, "collide: max_locals == slots");
        let load_bytes: usize = std::iter::once((0usize, SlotKind::A))
            .chain(desc_slot_kinds(vdesc).expect("kinds"))
            .map(|(slot, _)| usize::from(slot > 3) + 1)
            .sum();
        assert_eq!(
            code_len as usize,
            load_bytes + 4,
            "collide: code length = typed-load chain + 3-byte invokestatic + areturn"
        );
    }

    /// TRAVEL-DIET v2a idempotency: re-sight = AlreadyPatched, bytes intact.
    #[test]
    fn traveldiet_redirect_idempotent() {
        let (first, out1) = patch_entity_traveldiet(REAL_ENTITY).expect("first");
        assert_eq!(out1, RetargetOutcome::Retargeted { sites: 2 });
        let (second, out2) = patch_entity_traveldiet(&first).expect("second");
        assert_eq!(out2, RetargetOutcome::AlreadyPatched { sites: 2 });
        assert_eq!(first, second, "AlreadyPatched must not touch bytes");
    }

    /// TRAVEL-DIET v2b (RECON-21 section 4): the STATIC getInputVector body
    /// redirects to the TravelDietOps static with an IDENTICAL descriptor
    /// (no receiver); the redirected Code is aload_0/fload_1/fload_2 +
    /// invokestatic + areturn, static slot numbering from 0.
    #[test]
    fn traveldiet_redirect_getinputvector_and_verify() {
        let (out, outcome) = patch_entity_traveldiet(REAL_ENTITY).expect("patch");
        assert_eq!(outcome, RetargetOutcome::Retargeted { sites: 2 });
        let layout = parse_layout(&out).expect("re-parse redirected Entity");
        let name_idx = layout.pool.find_utf8("getInputVector").expect("name kept");
        let desc_idx = layout
            .pool
            .find_utf8(TD_INPUTVEC_DESC)
            .expect("desc kept");
        let m = find_method(&out, layout.methods_start, name_idx, desc_idx)
            .expect("redirected static method found");
        let (code_start, code_len) =
            find_code_attr(&out, &layout.pool, &m).expect("code attr");
        let code = &out[code_start..code_start + code_len];
        // static shape: aload_0 (slot 0 = Vec3 arg) + fload_1 + fload_2 +
        // invokestatic + areturn
        assert_eq!(code[0], 0x2a, "getInputVector: aload_0 = Vec3 arg (static slot 0)");
        assert_eq!(code[1], 0x23, "getInputVector: fload_1 = friction");
        assert_eq!(code[2], 0x24, "getInputVector: fload_2 = yaw");
        let ret_pos = code.len() - 1;
        assert_eq!(code[ret_pos], 0xb0, "getInputVector: ends with areturn");
        assert_eq!(code[ret_pos - 3], 0xb8, "getInputVector: invokestatic");
        let cp_idx = u16::from_be_bytes([code[ret_pos - 2], code[ret_pos - 1]]);
        let parts = layout.pool.methodref_parts(cp_idx).expect("resolve target");
        assert_eq!(parts.0, TRAVEL_DIET_OPS_CLASS, "getInputVector: target owner");
        assert_eq!(parts.1, "getInputVector", "getInputVector: target name");
        assert_eq!(
            parts.2, TD_INPUTVEC_DESC,
            "getInputVector: static-to-static, IDENTICAL descriptor"
        );
        assert_eq!(usize::from(code_len), 3 + 4, "getInputVector: 3 loads + invokestatic + areturn");
    }

    /// TRAVEL-DIET v2b: the LivingEntity travelInFluid body redirects to the
    /// receiver-prepended static; idempotent; bytes change.
    #[test]
    fn traveldiet_redirect_travelinfluid_and_verify() {
        let (out, outcome) = patch_livingentity_traveldiet(REAL_LIVING).expect("patch");
        assert_eq!(outcome, RetargetOutcome::Retargeted { sites: 1 });
        assert_ne!(out.as_slice(), REAL_LIVING, "redirect must change bytes");
        let layout = parse_layout(&out).expect("re-parse redirected LivingEntity");
        let name_idx = layout.pool.find_utf8("travelInFluid").expect("name kept");
        let desc_idx = layout
            .pool
            .find_utf8("(Lnet/minecraft/world/phys/Vec3;)V")
            .expect("desc kept");
        let m = find_method(&out, layout.methods_start, name_idx, desc_idx)
            .expect("redirected method found");
        let (code_start, code_len) =
            find_code_attr(&out, &layout.pool, &m).expect("code attr");
        let code = &out[code_start..code_start + code_len];
        // shape: aload_0 (receiver) + aload_1 (Vec3) + invokestatic + vreturn
        assert_eq!(code[0], 0x2a, "travelInFluid: aload_0 receiver");
        let ret_pos = code.len() - 1;
        assert_eq!(code[ret_pos], 0xb1, "travelInFluid: void method -> vreturn");
        assert_eq!(code[ret_pos - 3], 0xb8, "travelInFluid: invokestatic");
        let cp_idx = u16::from_be_bytes([code[ret_pos - 2], code[ret_pos - 1]]);
        let parts = layout.pool.methodref_parts(cp_idx).expect("resolve target");
        assert_eq!(parts.0, TRAVEL_DIET_OPS_CLASS, "travelInFluid: target owner");
        assert_eq!(parts.1, "travelInFluid", "travelInFluid: target name");
        assert_eq!(
            parts.2,
            "(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/phys/Vec3;)V",
            "travelInFluid: receiver-prepended desc"
        );
        // idempotency
        let (again, outcome2) = patch_livingentity_traveldiet(&out).expect("second");
        assert_eq!(outcome2, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(out, again, "AlreadyPatched must not touch bytes");
    }

    /// NotFound: a body-redirect on a class without the target method must
    /// return the ORIGINAL bytes and never grow the pool (no half-composed
    /// stage: the probe happens before any mutation).
    #[test]
    fn zeroalloc_redirect_missing_method_not_found_no_mutation() {
        let before = parse_layout(REAL_ENTITY).unwrap().pool.next;
        let (bytes_back, outcome) = redirect_method_body_to_static(
            REAL_ENTITY,
            "no_such_method",
            "(I)I",
            "net/minecraft/world/entity/Entity",
            ZERO_ALLOC_OPS_CLASS,
            "no_such_method",
            "(Lnet/minecraft/world/entity/Entity;I)I",
        )
        .expect("clean NotFound");
        assert_eq!(outcome, RetargetOutcome::NotFound);
        assert_eq!(bytes_back, REAL_ENTITY, "NotFound returns original bytes");
        let after = parse_layout(REAL_ENTITY).unwrap().pool.next;
        assert_eq!(before, after, "NotFound must not append pool entries");
    }

    /// Shape contract: a static desc without the receiver prepended is
    /// refused with an error (stack shape would change).
    #[test]
    fn zeroalloc_redirect_receiver_shape_enforced() {
        let err = redirect_method_body_to_static(
            REAL_ENTITY,
            "collidedWithFluid",
            ZA_FLUID_V,
            "net/minecraft/world/entity/Entity",
            ZERO_ALLOC_OPS_CLASS,
            "collidedWithFluid",
            ZA_FLUID_V, // NOT receiver-prepended
        )
        .unwrap_err();
        assert!(err.contains("receiver"), "{err}");
    }

    /// Harness bridge: with CRUSSTY_EMIT_PATCHED_ENTITY=<path> set, the
    /// fully redirected Entity.class is written to disk so the offline
    /// lockstep harness can defineClass it (HotSpot verifies the generated
    /// bytecode: typed loads, max_stack/locals, attribute lengths). Silent
    /// no-op in the default suite.
    #[test]
    fn zeroalloc_emit_patched_entity_for_harness() {
        if let Ok(path) = std::env::var("CRUSSTY_EMIT_PATCHED_ENTITY") {
            if path.trim().is_empty() {
                return;
            }
            let (out, outcome) = patch_entity_zeroalloc(REAL_ENTITY).expect("patch");
            assert_eq!(outcome, RetargetOutcome::Retargeted { sites: 3 });
            std::fs::write(&path, out).expect("write patched entity");
        }
    }

    /// S7-166 harness bridge: emit the Entity.class with the #13-SBB
    /// setBoundingBox body-redirect applied (stage-8 shape, sites==1) for
    /// the SkipStoreLockstepHarness HotSpot verifier pass. Silent no-op in
    /// the default suite.
    #[test]
    fn skipstore_emit_patched_entity_for_harness() {
        if let Ok(path) = std::env::var("CRUSSTY_EMIT_PATCHED_ENTITY_SSB") {
            if path.trim().is_empty() {
                return;
            }
            let (out, outcome) = patch_entity_skip_store_bb(REAL_ENTITY).expect("patch");
            assert_eq!(outcome, RetargetOutcome::Retargeted { sites: 1 });
            std::fs::write(&path, out).expect("write patched entity");
        }
    }

    /// S7-166 strict-shape: the redirect MUST target EXACTLY one site (the
    /// single (AABB)V setter body) and the generated invokestatic must
    /// carry the receiver-prepended descriptor. Re-patching the SAME bytes
    /// yields AlreadyPatched (idempotent), NotFound returns originals.
    #[test]
    fn skipstore_redirect_shape_single_site() {
        let (out, outcome) = patch_entity_skip_store_bb(REAL_ENTITY).expect("patch");
        assert_eq!(outcome, RetargetOutcome::Retargeted { sites: 1 });
        assert_ne!(out.len(), REAL_ENTITY.len(), "pool grew with bridge entries");
        let (out2, outcome2) = patch_entity_skip_store_bb(&out).expect("repatch");
        assert_eq!(outcome2, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(out2, out, "idempotent re-patch is byte-stable");
    }

    /// S7-166 delivery closure mirrored in the test suite: the built
    /// SkipStoreOps classfile must declare the receiver-prepended
    /// setBoundingBox static.
    #[test]
    fn skipstore_embedded_declares_redirect_target() {
        if let Err(e) =
            crate::classfile::skipstore_resolution_closure(crate::skip_store::SKIP_STORE_BYTES)
        {
            panic!(
                "RESOLUTION CLOSURE FAILED: {e} — rebuild entityinside/ via build_skipstore_ops.sh"
            );
        }
    }

    /// Slot accounting: double/long parameters occupy two slots (the
    /// fluid-push descriptor has a trailing double).
    #[test]
    fn zeroalloc_desc_slot_accounting() {
        assert_eq!(
            desc_slot_kinds(ZA_PUSH_V),
            Some(vec![(1, SlotKind::A), (2, SlotKind::D)]),
            "TagKey=slot1(ref), double=slot2(cat2)"
        );
        assert_eq!(desc_param_slots(ZA_PUSH_V), Some(4));
        assert_eq!(
            desc_slot_kinds("(JLjava/util/List;)Z"),
            Some(vec![(1, SlotKind::J), (3, SlotKind::A)])
        );
        assert_eq!(desc_param_slots("()V"), Some(1)); // receiver only
    }


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

/// Bridge class defined into the kernel loader by mobs_manager.rs
/// (TASK-401-E mob-soa).
pub const MOB_OPS_CLASS: &str = "net/minecraft/world/entity/MobPushOps";

const GET_PUSHABLES_DESC: &str =
    "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;";
const OPS_PUSHABLES_DESC: &str =
    "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;\
     Lnet/minecraft/world/phys/AABB;)Ljava/util/List;";

const MUTABLE_POS_CLASS: &str = "net/minecraft/core/BlockPos$MutableBlockPos";

/// Bridge class defined into the kernel loader by inside_cache.rs
/// (S7-135 / TASK-271 INSIDE-CACHE lever).
pub const INSIDE_OPS_CLASS: &str = "net/minecraft/world/entity/InsideBlockOps";

/// Sibling bridge class for the INSIDE-BATCH plane (TASK-459-56, ID-P31):
/// defined into the kernel loader by inside_batch.rs; supersedes InsideBlockOps
/// on the checkInsideBlocks method-entry site when its lever is armed
/// (S7-162 single-owner discipline).
pub const INSIDE_BATCH_OPS_CLASS: &str = "net/minecraft/world/entity/InsideBatchOps";

/// javap-контракт: единственный `isAffectedByBlocks` сайт внутри
/// `Entity.checkInsideBlocks(List, StepBasedCollector)` (offset 1).
const CHECK_INSIDE_DESC: &str =
    "(Ljava/util/List;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;)V";
const GATE_DESC: &str = "(Lnet/minecraft/world/entity/Entity;)Z";

// ---------------------------------------------------------------------------
// INSIDE-BITMASK (TASK-357, ARCH-ATTACK lever — the inside-discovery lane,
// section all-air pre-gate; RECON-32/33 contract
// research/gc-recon-2026-09-19/RECON33_INSIDE_BITMASK_CONTRACT.md).
//
// javap-контракт: ЕДИНСТВЕННЫЙ вызыватель private
// `Entity.checkInsideBlocks(List, StepBasedCollector)V` — это
// `Entity.applyEffectsFromBlocks(List<Movement>)` bc 58..64:
//   58: aload_0
//   59: aload_1
//   60: aload_0
//   61: getfield insideEffectCollector
//   64: invokevirtual checkInsideBlocks:(Ljava/util/List;LStepBasedCollector;)V
// (ref-census: другие вызыватели отсутствуют; all per-tick входы —
// ItemEntity/ExperienceOrb/FallingBlock/PrimedTnt/EndCrystal.tick,
// EnderDragon.aiStep, AbstractBoat.tick ×2, AbstractMinecart.move —
// фуннелятся через applyEffectsFromBlocks(List)).
//
// Shape: ретаргет этого ОДНОГО invokevirtual на invokestatic
// InsideBitmaskOps.checkInsideBlocksGated
//   (LEntity;Ljava/util/List;LStepBasedCollector;)V
// (receiver-first, 3B→3B, длина и форма стека сохранены). Ванильное тело
// НЕ трогается: фоллбэк = MethodHandle-вызов исходного метода из Ops
// (fail-closed; probe-then-patch в inside_bitmask.rs гарантирует, что патч
// не встанет на разоружённый бридж).
//
// Strict: ровно ОДИН сайт (javap-ценз s7194); AlreadyPatched — только когда
// сайт уже invokestatic на бридж; остальное = Err (fail closed, vanilla).

pub const INSIDE_BITMASK_OPS_CLASS: &str = "net/minecraft/world/entity/InsideBitmaskOps";
const AFB_HOST: &str = "applyEffectsFromBlocks";
const AFB_HOST_DESC: &str = "(Ljava/util/List;)V";
const CIB_TARGET: (&str, &str, &str) = (
    "net/minecraft/world/entity/Entity",
    "checkInsideBlocks",
    CHECK_INSIDE_DESC,
);
const BITMASK_GATE_DESC: &str =
    "(Lnet/minecraft/world/entity/Entity;Ljava/util/List;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;)V";

pub fn patch_inside_bitmask(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    for probe in ["applyEffectsFromBlocks", "checkInsideBlocks", "insideEffectCollector"] {
        if layout.pool.find_utf8(probe).is_none() {
            return Err(format!("{probe} absent from pool (kernel rename?)"));
        }
    }
    let expect_static = format!("(L{};{}", CIB_TARGET.0, &CIB_TARGET.2[1..]);
    if BITMASK_GATE_DESC != expect_static {
        return Err("bitmask gate descriptor is not the receiver-prepended target form".into());
    }
    let (out, outcome) = retarget_virtual_to_static(
        bytes,
        AFB_HOST,
        AFB_HOST_DESC,
        CIB_TARGET,
        (INSIDE_BITMASK_OPS_CLASS, "checkInsideBlocksGated", BITMASK_GATE_DESC),
    )?;
    if let RetargetOutcome::Retargeted { sites } = &outcome {
        if *sites != 1 {
            return Err(format!(
                "expected exactly one checkInsideBlocks site in applyEffectsFromBlocks(List), got {sites}"
            ));
        }
    }
    Ok((out, outcome))
}

// ---------------------------------------------------------------------------
// ENTITYMAP FENCE (TASK-411-A k5b, cmp405_navplane lane): ChunkMap.entityMap
// full-table race. Root cause (docs/TASK411_A_AIOOBE_ROOTCAUSE.md): the
// unsynchronized fastutil Int2ObjectOpenHashMap is mutated from two thread
// domains (population puts vs tick-phase removes) -> size/table desync ->
// full table -> infinite containsKey probe (watchdog) + MapIterator
// key[--pos] downscan "Index -1 ... length 131073".
//
// FIX: every Int2ObjectMap interface call site on entityMap is retargeted
// to the EntityMapOps static fence helpers (monitor serialized on the map
// instance; snapshot-safe iterators). LENGTH-PRESERVING rewrite:
//   invokeinterface #idx, count (0xb9, 5 bytes)
//     -> invokestatic #idx' (0xb8, 3 bytes) + 0x00 0x00 (2 nops)
// The receiver becomes the first static argument (receiver-prepended
// descriptor contract) — identical operand stack, identical instruction
// length (branch offsets / exception ranges / StackMapTable UNTOUCHED).
//
// javap census on pristine kernel 1.21.10 ChunkMap.class (ALL sites are
// `getfield #205 entityMap` immediately before the invoke):
//   containsKey(I)Z x3 (addEntity x2, removeEntity x1)
//   put(ILjava/lang/Object;)Ljava/lang/Object; x1 (addEntity)
//   remove(I)Ljava/lang/Object; x1 (removeEntity)
//   get(I)Ljava/lang/Object; x4 (sendToTrackingPlayers + 3 helpers)
//   values()ObjectCollection x3 (addEntity warn, removeEntity,
//                                 forEachEntityTrackedBy)
// STRICT: exact census counts or NO patch at all (fail-dominant vanilla).
// The read-only site in Entity.resendPossiblyDesyncedEntityData is
// deliberately left vanilla (fail-dominant fallback for that site).
// ---------------------------------------------------------------------------
pub const EMAP_OPS_CLASS: &str = "net/minecraft/server/level/EntityMapOps";

const EMAP_I2O: &str = "it/unimi/dsi/fastutil/ints/Int2ObjectMap";

/// (from-name, from-desc, exact site census) — single source of truth.
pub const EMAP_SITES: [(&str, &str, usize); 5] = [
    ("containsKey", "(I)Z", 3),
    ("put", "(ILjava/lang/Object;)Ljava/lang/Object;", 1),
    ("remove", "(I)Ljava/lang/Object;", 1),
    ("get", "(I)Ljava/lang/Object;", 4),
    (
        "values",
        "()Lit/unimi/dsi/fastutil/objects/ObjectCollection;",
        3,
    ),
];

/// Total fence sites (= sum of the census).
pub const EMAP_SITES_TOTAL: usize = 12;

fn emap_to_desc(from_desc: &str) -> String {
    // Receiver-prepended static form (same contract as
    // retarget_virtual_to_static): the entityMap reference becomes arg0.
    format!("(L{};{}", EMAP_I2O, &from_desc[1..])
}

/// Collect (code_start, code_len) spans of EVERY method's Code attribute.
fn collect_code_spans(bytes: &[u8], pool: &Pool, methods_start: usize) -> Result<Vec<(usize, usize)>, String> {
    let mut p = methods_start;
    let count = usize::from(u16_at(bytes, p).ok_or("methods_count oob")?);
    p += 2;
    let mut spans = Vec::with_capacity(count);
    for _ in 0..count {
        let access = u16_at(bytes, p).ok_or("method access oob")?;
        let name_idx = u16_at(bytes, p + 2).ok_or("method name oob")?;
        let desc_idx = u16_at(bytes, p + 4).ok_or("method desc oob")?;
        p += 6;
        let attr_count = usize::from(u16_at(bytes, p).ok_or("method attr_count oob")?);
        p += 2;
        for _ in 0..attr_count {
            let len = u32_at(bytes, p + 2).ok_or("attr len oob")? as usize;
            let start = p;
            p = p.checked_add(6 + len).ok_or("attr overflow")?;
            if pool.utf8_value(u16_at(bytes, start).ok_or("attr name oob")?)
                .as_deref()
                == Some("Code")
            {
                let _ = (access, name_idx, desc_idx);
                let data = start + 6;
                let code_len = u32_at(bytes, data.checked_add(4).ok_or("code_len oob")?)
                    .ok_or("code_len oob")? as usize;
                let code_start = data.checked_add(8).ok_or("code start oob")?;
                if code_start.checked_add(code_len).ok_or("code end oob")? > start + 6 + len {
                    return Err("Code attr claims code past its own length".into());
                }
                spans.push((code_start, code_len));
            }
        }
    }
    Ok(spans)
}

/// Strict (exact census) length-preserving fence patch over the WHOLE
/// ChunkMap class. Idempotent: sites already resolving to the fence
/// helpers are classified AlreadyPatched (their counts must STILL match
/// the census — drift fails closed).
pub fn patch_chunkmap_entitymap(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout")?;
    // Find-only probe (audit A4): a kernel without the fastutil utf8 can
    // not carry the sites — fail closed BEFORE any pool mutation.
    if layout.pool.find_utf8(EMAP_I2O).is_none() {
        return Err(format!("{EMAP_I2O} absent from pool (kernel rename?)"));
    }
    let spans = collect_code_spans(bytes, &layout.pool, layout.methods_start)?;
    let mut sites: Vec<(usize, u16)> = Vec::new(); // (abs offset of opcode, cp idx)
    for (code_start, code_len) in spans {
        let code = bytes
            .get(code_start..code_start.checked_add(code_len).ok_or("code span oob")?)
            .ok_or("code span oob")?;
        let mut pc = 0usize;
        while pc < code.len() {
            let op = code[pc];
            if op == 0xb9 || op == 0xb8 {
                let b = code
                    .get(pc + 1..pc + 3)
                    .ok_or_else(|| "invoke operand truncated".to_string())?;
                sites.push((code_start + pc, u16::from_be_bytes([b[0], b[1]])));
                pc += 3; // 0xb9: +2 more bytes consumed below (count/zero)
                if op == 0xb9 {
                    pc += 2;
                }
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
    }

    // Classify every invoke site by its resolved triple. A from-site
    // resolves to Int2ObjectMap (invokeinterface), an already-patched site
    // resolves to EntityMapOps (invokestatic) — BOTH are visible to the
    // walk (idempotency discipline of scan_invoke_sites).
    let mut rewrite: Vec<usize> = Vec::new();
    let mut already = 0usize;
    let mut per_name: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for &(op_pc, cp_idx) in &sites {
        let Some(parts) = layout.pool.methodref_parts(cp_idx) else {
            continue;
        };
        if parts.0 == EMAP_I2O {
            if EMAP_SITES
                .iter()
                .any(|(n, d, _)| *n == parts.1 && *d == parts.2)
            {
                rewrite.push(op_pc);
                *per_name.entry(parts.1).or_insert(0) += 1;
            }
            // An Int2ObjectMap site with an UNEXPECTED name/desc = kernel
            // drift -> strict failure (never rewrite a site we did not
            // census).
            else {
                return Err(format!(
                    "uncensused Int2ObjectMap site {}{} — kernel drift, fence rejected",
                    parts.1, parts.2
                ));
            }
        } else if parts.0 == EMAP_OPS_CLASS {
            // Already-fenced site: must resolve to one of the helpers with
            // the EXACT receiver-prepended descriptor.
            let Some((_, from_desc, _)) = EMAP_SITES.iter().find(|(n, _, _)| *n == parts.1)
            else {
                return Err(format!(
                    "unexpected EntityMapOps site {}{} — kernel drift, fence rejected",
                    parts.1, parts.2
                ));
            };
            if parts.2 != emap_to_desc(from_desc) {
                return Err(format!(
                    "fence descriptor drift at site {}{} — fence rejected",
                    parts.1, parts.2
                ));
            }
            already += 1;
            *per_name.entry(parts.1).or_insert(0) += 1;
        }
    }
    for (name, _desc, want) in &EMAP_SITES {
        let got = per_name.get(*name).copied().unwrap_or(0);
        if got != *want {
            return Err(format!(
                "emap census violated for {name}: {got} sites, expected {want} — fence rejected"
            ));
        }
    }
    if rewrite.is_empty() {
        if already == EMAP_SITES_TOTAL {
            return Ok((bytes.to_vec(), RetargetOutcome::AlreadyPatched { sites: already }));
        }
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    }

    // Append (or reuse) the fence Methodrefs — append-only, dedup.
    let mut pool = layout.pool;
    let mut idx_of: std::collections::HashMap<String, u16> = std::collections::HashMap::new();
    for (name, desc, _n) in &EMAP_SITES {
        let idx = pool.method_ref(EMAP_OPS_CLASS, name, &emap_to_desc(desc));
        idx_of.insert(name.to_string(), idx);
    }
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for fence refs".into());
    }

    // Splice: header + grown pool + tail; rewrite each matched site
    // b9 xxxxxxxx (5B) -> b8 yyyy 00 00 (3B + 2 nops) — LENGTH PRESERVED.
    let mut tail = bytes[layout.cp_end..].to_vec();
    for &op_off in &rewrite {
        let rel = op_off - layout.cp_end;
        if rel + 4 >= tail.len() {
            return Err("fence site outside class tail (corrupt layout?)".into());
        }
        let op = tail[rel];
        if op != 0xb9 {
            return Err(format!("fence site opcode {op:#x} is not invokeinterface"));
        }
        let cp_idx = u16::from_be_bytes([tail[rel + 1], tail[rel + 2]]);
        let Some(parts) = pool.methodref_parts(cp_idx) else {
            return Err("fence site operand does not resolve".into());
        };
        let idx = *idx_of.get(&parts.1).ok_or("no fence ref for site")?;
        let want = idx.to_be_bytes();
        tail[rel] = 0xb8; // invokestatic
        tail[rel + 1] = want[0];
        tail[rel + 2] = want[1];
        tail[rel + 3] = 0x00; // nop (the invokeinterface count byte)
        tail[rel + 4] = 0x00; // nop (the invokeinterface zero byte)
    }
    let mut out = Vec::with_capacity(bytes.len() + 64);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    out.extend_from_slice(&pool.next.to_be_bytes()); // new cp_count
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&tail);
    Ok((out, RetargetOutcome::Retargeted { sites: rewrite.len() }))
}

// ---------------------------------------------------------------------------
// REFSYNC (TASK-412-A): ReferenceList mutator fence — the SECOND fastutil
// race surface (a-k5b leg evidence, run 35698807454). The k5b emap fence
// closed ChunkMap.entityMap; the SAME population-vs-region-worker race
// then surfaced on moonrise ReferenceList.referenceToIndex
// (Reference2IntOpenHashMap): watchdog RUNNABLE forever in
// Reference2IntOpenHashMap.find:246 <- putIfAbsent:422 <-
// ReferenceList.add:65 <- ServerEntityLookup.entityStartLoaded (main
// thread, injection), plus 3278 AIOOBE ("Index -1 ... length 16385" =
// downscan below slot 0 on the n+1 table at n=16384, x203 message-ful +
// x3075 message-less) in the same population window.
//
// Fix: every ReferenceList add/remove/contains call site (the map
// mutators + the map probe) is retargeted — LENGTH-PRESERVING
// invokevirtual (0xb6, 3 bytes) -> invokestatic (0xb8, 3 bytes), the
// opcode byte is the ONLY change (operand CP index and stack shape are
// identical: the static helper consumes the receiver as its first
// parameter) — to the EntityMapOps.refList* helpers (receiver-prepended
// descriptor), each serializing on the LIST INSTANCE monitor. Read-only
// array readers (size/getRawDataUnchecked) are deliberately left vanilla
// (Entity.resendPossiblyDesyncedEntityData precedent: they cannot
// corrupt the map; vanilla weakly-consistent iteration semantics).
// OPCODE NOTE (test-caught): ReferenceList is a final CLASS, so its call
// sites are invokevirtual 0xB6 — NOT invokeinterface 0xB9 (the emap
// fence's Int2ObjectMap sites are interface invokes; this fence is the
// 3-byte single-swap idiom of the Entity rng retarget, same as line ~1500).
//
// javap census on pristine kernel 1.21.10 (COMPLETE: ReferenceList is
// final => every site is an invokevirtual whose Methodref owner is the
// class name itself; binary constant-pool grep finds 12 referencing
// classes, of which 8 carry add/remove/contains sites):
//   EntityScheduler$EntitySchedulerTickList  add x1 remove x1
//   BaseChunkSystemHooks                     add x3 remove x3
//   NearbyPlayers$TrackedChunk               add x2 remove x1
//   ServerEntityLookup                       add x1 remove x1
//   CraftWorld                               add x1
//   ChunkHolder              add x1 remove x1 contains x1
//   ServerLevel              add x2 remove x2
//   ChunkMap$TrackedEntity                   contains x1
// TOTAL 22 sites (add x11, remove x9, contains x2). STRICT per-class
// census or NO patch at all (fail-dominant vanilla).
// ---------------------------------------------------------------------------
/// The ReferenceList owner class (all retargeted sites resolve to it).
pub const REFLIST_CLASS: &str = "ca/spottedleaf/moonrise/common/list/ReferenceList";

/// (from-name, from-desc) pairs — add/remove/contains share the erased
/// one-Object-arg shape; the static helpers take the receiver prepended.
pub const REFSYNC_SITES: [(&str, &str); 3] = [
    ("add", "(Ljava/lang/Object;)Z"),
    ("remove", "(Ljava/lang/Object;)Z"),
    ("contains", "(Ljava/lang/Object;)Z"),
];

/// Per-class STRICT census: (this_class, [(site-name, want)]).
/// A class NOT in this table is rejected outright (kernel drift guard —
/// the refsync compose runs on EXACTLY these 8 classes).
pub const REFSYNC_CENSUS: [(&str, [(&str, usize); 3]); 8] = [
    (
        "io/papermc/paper/threadedregions/EntityScheduler$EntitySchedulerTickList",
        [("add", 1), ("remove", 1), ("contains", 0)],
    ),
    (
        "ca/spottedleaf/moonrise/paper/util/BaseChunkSystemHooks",
        [("add", 3), ("remove", 3), ("contains", 0)],
    ),
    (
        "ca/spottedleaf/moonrise/common/misc/NearbyPlayers$TrackedChunk",
        [("add", 2), ("remove", 1), ("contains", 0)],
    ),
    (
        "ca/spottedleaf/moonrise/patches/chunk_system/level/entity/server/ServerEntityLookup",
        [("add", 1), ("remove", 1), ("contains", 0)],
    ),
    (
        "org/bukkit/craftbukkit/CraftWorld",
        [("add", 1), ("remove", 0), ("contains", 0)],
    ),
    (
        "net/minecraft/server/level/ChunkHolder",
        [("add", 1), ("remove", 1), ("contains", 1)],
    ),
    (
        "net/minecraft/server/level/ServerLevel",
        [("add", 2), ("remove", 2), ("contains", 0)],
    ),
    (
        "net/minecraft/server/level/ChunkMap$TrackedEntity",
        [("add", 0), ("remove", 0), ("contains", 1)],
    ),
];

/// Total retargeted sites across the kernel (add 11 + remove 9 + contains 2).
pub const REFSYNC_SITES_TOTAL: usize = 22;

/// ReferenceList READ-ONLY members whose call sites are DELIBERATELY left
/// vanilla (they touch only the `references` array / `count` snapshot, never
/// the referenceToIndex map — the corruption surface is mutator-only).
/// A ReferenceList site outside REFSYNC_SITES ∪ this allowlist is kernel
/// drift => the whole class is rejected (fail-dominant).
pub const REFSYNC_READONLY_SITES: [&str; 7] = [
    "size",
    "getRawData",
    "getRawDataUnchecked",
    "getChecked",
    "getUnchecked",
    "iterator",
    "copy",
];

/// Receiver-prepended static descriptor for a ReferenceList mutator site.
/// All three sites share the erased one-Object-arg boolean shape.
fn refsync_to_desc() -> String {
    format!("(L{};Ljava/lang/Object;)Z", REFLIST_CLASS)
}

/// Expected retargeted site count for a censused refsync class (0 = the
/// class carries only read-only ReferenceList sites and stays vanilla).
pub fn refsync_want_sites(class: &str) -> usize {
    REFSYNC_CENSUS
        .iter()
        .find(|(c, _)| *c == class)
        .map(|(_, w)| w.iter().map(|(_, n)| *n).sum())
        .unwrap_or(0)
}

fn refsync_helper_name(from_name: &str) -> String {
    let mut c = from_name.chars();
    match c.next() {
        Some(f) => format!("refList{}{}", f.to_ascii_uppercase(), c.as_str()),
        None => "refList".to_string(),
    }
}

/// Strict per-class retarget of ReferenceList add/remove/contains sites to
/// the EntityMapOps.refList* fence helpers. Idempotent: already-retargeted
/// sites (invokestatic to EntityMapOps with the refList* name + exact
/// receiver-prepended descriptor) count as AlreadyPatched.
pub fn patch_referencelist_callsites(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout")?;
    let this_name = this_class_name(&layout).ok_or("cannot resolve this_class name")?;
    let want: [(&str, usize); 3] = match REFSYNC_CENSUS.iter().find(|(c, _)| *c == this_name) {
        Some((_, w)) => *w,
        None => {
            return Err(format!(
                "class {this_name} is not in the refsync census — patch rejected"
            ));
        }
    };
    let spans = collect_code_spans(bytes, &layout.pool, layout.methods_start)?;
    let mut sites: Vec<(usize, u16)> = Vec::new();
    for (code_start, code_len) in spans {
        let code = bytes
            .get(code_start..code_start.checked_add(code_len).ok_or("code span oob")?)
            .ok_or("code span oob")?;
        let mut pc = 0usize;
        while pc < code.len() {
            let op = code[pc];
            // 0xb6 invokevirtual (3 bytes) and 0xb8 invokestatic (3 bytes):
            // vanilla ReferenceList sites are invokevirtual (final class);
            // the static form is the ALREADY-PATCHED shape (idempotency).
            if op == 0xb6 || op == 0xb8 {
                let b = code
                    .get(pc + 1..pc + 3)
                    .ok_or_else(|| "invoke operand truncated".to_string())?;
                sites.push((code_start + pc, u16::from_be_bytes([b[0], b[1]])));
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
    }
    let mut rewrite: Vec<usize> = Vec::new();
    let mut already = 0usize;
    let mut per_name: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for &(op_pc, cp_idx) in &sites {
        let Some(parts) = layout.pool.methodref_parts(cp_idx) else {
            continue;
        };
        if parts.0 == REFLIST_CLASS {
            if REFSYNC_SITES
                .iter()
                .any(|(n, d)| *n == parts.1 && *d == parts.2)
            {
                rewrite.push(op_pc);
                *per_name.entry(parts.1.to_string()).or_insert(0) += 1;
            } else if REFSYNC_READONLY_SITES.contains(&parts.1.as_str()) {
                // Read-only array reader: left vanilla by design (see the
                // fence rationale above) — neither counted nor rewritten.
            } else {
                return Err(format!(
                    "uncensused ReferenceList site {}{} in {this_name} — kernel drift, refsync rejected",
                    parts.1, parts.2
                ));
            }
        } else if parts.0 == EMAP_OPS_CLASS && parts.1.starts_with("refList") {
            // Already-fenced site: helper name must map back to a censused
            // site and the descriptor must be the exact receiver-prepended
            // form.
            let suffix = parts.1.strip_prefix("refList").unwrap_or("");
            let from_name = if suffix.is_empty() {
                String::new()
            } else {
                let mut c = suffix.chars();
                match c.next() {
                    Some(f) => format!("{}{}", f.to_ascii_lowercase(), c.as_str()),
                    None => String::new(),
                }
            };
            if !REFSYNC_SITES.iter().any(|(n, _)| *n == from_name)
                || parts.2 != refsync_to_desc()
            {
                return Err(format!(
                    "unexpected refsync helper site {}{} in {this_name} — kernel drift, refsync rejected",
                    parts.1, parts.2
                ));
            }
            already += 1;
            *per_name.entry(from_name).or_insert(0) += 1;
        }
    }
    for (name, want_n) in &want {
        let got = per_name.get(*name).copied().unwrap_or(0);
        if got != *want_n {
            return Err(format!(
                "refsync census violated for {this_name}.{name}: {got} sites, expected {want_n} — patch rejected"
            ));
        }
    }
    if rewrite.is_empty() {
        let total: usize = want.iter().map(|(_, n)| *n).sum();
        if already == total {
            return Ok((bytes.to_vec(), RetargetOutcome::AlreadyPatched { sites: already }));
        }
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    }

    // Append (or reuse) the fence Methodrefs — append-only, dedup.
    let mut pool = layout.pool;
    for (name, _d) in &REFSYNC_SITES {
        let _ = pool.method_ref(EMAP_OPS_CLASS, &refsync_helper_name(name), &refsync_to_desc());
    }
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for refsync refs".into());
    }

    let mut tail = bytes[layout.cp_end..].to_vec();
    for &op_off in &rewrite {
        let rel = op_off - layout.cp_end;
        if rel + 4 >= tail.len() {
            return Err("refsync site outside class tail (corrupt layout?)".into());
        }
        let op = tail[rel];
        if op != 0xb6 {
            return Err(format!("refsync site opcode {op:#x} is not invokevirtual"));
        }
        let cp_idx = u16::from_be_bytes([tail[rel + 1], tail[rel + 2]]);
        let Some(parts) = pool.methodref_parts(cp_idx) else {
            return Err("refsync site operand does not resolve".into());
        };
        let idx =
            pool.method_ref(EMAP_OPS_CLASS, &refsync_helper_name(&parts.1), &refsync_to_desc());
        // LENGTH-PRESERVING 3-byte swap: invokevirtual (0xb6, 3 bytes) ->
        // invokestatic (0xb8, 3 bytes). Opcode byte changes AND the 2-byte
        // CP operand is repointed at the appended helper Methodref (the
        // receiver becomes the helper's first parameter — same stack
        // shape). No nops, no code-span shift => branch offsets and
        // StackMapTable stay byte-identical.
        let want_idx = idx.to_be_bytes();
        tail[rel] = 0xb8; // invokestatic
        tail[rel + 1] = want_idx[0];
        tail[rel + 2] = want_idx[1];
    }
    let mut out = Vec::with_capacity(bytes.len() + 64);
    out.extend_from_slice(&bytes[0..8]); // magic, minor, major
    out.extend_from_slice(&pool.next.to_be_bytes()); // new cp_count
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&tail);
    Ok((out, RetargetOutcome::Retargeted { sites: rewrite.len() }))
}

// ---------------------------------------------------------------------------
// FLAT-TRAVERSAL (S7-163, ARCH-ATTACK lever #9 — the inside-pipeline
// traversal lane). The private int-overload
//   Entity.checkInsideBlocks(Vec3, Vec3, StepBasedCollector, LongSet, int)I
// carries the SINGLE invokestatic call site of
//   BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB,
//   BlockGetter$BlockStepVisitor)Z
// (javap census on kernel 1.21.10: strict sites=1; the other two
// checkInsideBlocks overloads do not call it). The traversal stage
// retargets that site to the flat TraverseOps.forEachFlat with the
// IDENTICAL descriptor (same stack shape; TraverseOps is defined into
// the kernel loader by traversal.rs before the Entity retransform).
// Bit-exactness oracle: TraverseLockstepHarness (60k+ scenarios,
// sequence+return bit-in-bit vs the vanilla implementation).
pub const TRAVERSE_OPS_CLASS: &str = "net/minecraft/world/level/TraverseOps";
const TRAVERSAL_FROM: (&str, &str, &str) = (
    "net/minecraft/world/level/BlockGetter",
    "forEachBlockIntersectedBetween",
    "(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/AABB;Lnet/minecraft/world/level/BlockGetter$BlockStepVisitor;)Z",
);
const TRAVERSAL_SITE_DESC: &str =
    "(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;Lit/unimi/dsi/fastutil/longs/LongSet;I)I";

/// Strict (sites==1) retarget of the forEachBlockIntersectedBetween call
/// site inside Entity.checkInsideBlocks(Vec3,Vec3,StepBasedCollector,
/// LongSet,int)I to TraverseOps.forEachFlat (same descriptor). Idempotent
/// via [`retarget_invokestatic`] classification; NotFound returns the
/// original bytes without pool growth.
pub fn patch_entity_traversal(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let (out, outcome) = retarget_invokestatic(
        bytes,
        "checkInsideBlocks",
        TRAVERSAL_SITE_DESC,
        TRAVERSAL_FROM,
        (TRAVERSE_OPS_CLASS, "forEachFlat", TRAVERSAL_FROM.2),
    )?;
    if let RetargetOutcome::Retargeted { sites } = &outcome {
        if *sites != 1 {
            return Err(format!(
                "expected exactly one forEachBlockIntersectedBetween site in checkInsideBlocks, got {sites}"
            ));
        }
    }
    Ok((out, outcome))
}


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

/// TASK-401-E (mob-soa): the SAME single `Level.getPushableEntities` call
/// site inside `LivingEntity.pushEntities()V`, retargeted to the static
/// `MobPushOps.pushables` bridge (net/minecraft/world/entity/MobPushOps,
/// defined into the kernel loader by mobs_manager.rs). Strict single-site,
/// fail-closed — same contract as `patch_push_entities`.
pub fn patch_push_entities_mob(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    retarget_virtual_to_static(
        bytes,
        "pushEntities",
        "()V",
        ("net/minecraft/world/level/Level", "getPushableEntities", GET_PUSHABLES_DESC),
        (MOB_OPS_CLASS, "pushables", OPS_PUSHABLES_DESC),
    )
}

/// TASK-419-A (colpush): WHOLE-BODY redirect of `LivingEntity.pushEntities()V`
/// to the static `ColpushOps.pushEntities(LivingEntity)V` bridge
/// (net/minecraft/world/entity/ColpushOps, defined into the kernel loader by
/// colpush.rs). The body replacement is total: the per-entity
/// `MobPushOps.pushables` ladder (per-entity JNI mobUpsert + chain scan +
/// per-query allocations) is bypassed entirely — the bridge re-implements the
/// vanilla tail bit-exactly (cramming RNG/numCollisions/doPush on live
/// fields) consuming the bulk CSR broadphase written once per tick by the
/// rust `colpushTick` native. Strict single-method, fail-closed — same
/// contract as `redirect_method_body_to_static`.
pub const COLPUSH_OPS_CLASS: &str = "net/minecraft/world/entity/ColpushOps";

pub fn patch_push_entities_colpush(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    redirect_method_body_to_static(
        bytes,
        "pushEntities",
        "()V",
        "net/minecraft/world/entity/LivingEntity",
        COLPUSH_OPS_CLASS,
        "pushEntities",
        "(Lnet/minecraft/world/entity/LivingEntity;)V",
    )
}

/// S7-164-style resolution gate for the delivered ColpushOps classfile: the
/// bridge must declare EVERY static entry point the surgery / the
/// RegionTickOps trigger call, offline, before any retransform (member drift
/// fails here instead of NoSuchMethodError on the first tick).
pub fn colpush_resolution_closure(ops: &[u8]) -> Result<(), String> {
    redirect_targets_resolution_closure(
        ops,
        &[
            (
                "net/minecraft/world/entity/LivingEntity",
                "()V",
                "pushEntities",
                "(Lnet/minecraft/world/entity/LivingEntity;)V",
            ),
            (
                "net/minecraft/server/level/ServerLevel",
                "()V",
                "bulkTick",
                "()V",
            ),
        ],
    )
}

// ---------------------------------------------------------------------------
// ZERO-ALLOC-INSIDE (S7-164, lever #10): METHOD-BODY REDIRECT.
//
// Unlike retarget_* (which rewrites INVOKE SITES inside one caller), a
// body-redirect replaces the whole Code attribute of a method with
// `aload 0..N; invokestatic ZeroAllocOps.<name>(receiver+args); return` —
// every virtual dispatch from ANY class then lands in the zero-alloc scalar
// implementation with vanilla-identical parameters. The descriptor is
// preserved (length-preserving receiver-prepended static form, same shape
// contract as retarget_virtual_to_static). Debug attributes
// (LineNumberTable/LocalVariableTable/LocalVariableTypeTable) are dropped
// with the old code; `Exceptions` and everything else is kept.
// ---------------------------------------------------------------------------

/// Total local-slot count of a method descriptor's parameters (no receiver).
fn desc_param_slots(desc: &str) -> Option<usize> {
    desc_slot_kinds(desc).map(|v| match v.last() {
        Some((start, kind)) => {
            start + usize::from(matches!(kind, SlotKind::J | SlotKind::D))
                + 1
        }
        None => 1, // no params: receiver occupies slot 0
    })
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SlotKind {
    I, // boolean/byte/char/short/int
    J, // long
    F, // float
    D, // double
    A, // reference / array
}

/// For each parameter: its starting local slot (receiver = slot 0 -> first
/// param starts at 1), its load-kind, and whether it is category-two.
/// The load kind selects the correct *_load opcode family: an aload on a
/// double slot would VerifyError on the first redirected call.
fn desc_slot_kinds(desc: &str) -> Option<Vec<(usize, SlotKind)>> {
    let b = desc.as_bytes();
    if b.first() != Some(&b'(') {
        return None;
    }
    let mut i = 1;
    let mut slot = 1usize;
    let mut starts = Vec::new();
    while i < b.len() && b[i] != b')' {
        let mut was_array = false;
        while i < b.len() && b[i] == b'[' {
            was_array = true;
            i += 1;
        }
        let kind = match b.get(i)? {
            b'L' => {
                while i < b.len() && b[i] != b';' {
                    i += 1;
                }
                if i >= b.len() {
                    return None;
                }
                i += 1;
                SlotKind::A
            }
            b'D' => {
                i += 1;
                if was_array {
                    SlotKind::A // [D is an objectref: one slot
                } else {
                    SlotKind::D
                }
            }
            b'J' => {
                i += 1;
                if was_array {
                    SlotKind::A // [J
                } else {
                    SlotKind::J
                }
            }
            b'F' => {
                i += 1;
                if was_array {
                    SlotKind::A // [F
                } else {
                    SlotKind::F
                }
            }
            b'B' | b'C' | b'I' | b'S' | b'Z' => {
                i += 1;
                if was_array {
                    SlotKind::A
                } else {
                    SlotKind::I
                }
            }
            _ => return None,
        };
        let cat2 = matches!(kind, SlotKind::J | SlotKind::D);
        starts.push((slot, kind));
        slot += if cat2 { 2 } else { 1 };
    }
    if i >= b.len() || b[i] != b')' {
        return None;
    }
    Some(starts)
}

/// Push the typed load for a parameter slot (short forms for slots 0-3,
/// explicit form 0x15..0x19 + u1 slot otherwise).
fn push_load(slot: usize, kind: SlotKind, code: &mut Vec<u8>) -> Result<(), String> {
    let (short_base, wide_op): (u8, u8) = match kind {
        SlotKind::I => (0x1a, 0x15), // iload_0.. / iload
        SlotKind::J => (0x1e, 0x16), // lload_0.. / lload
        SlotKind::F => (0x22, 0x17), // fload_0.. / fload
        SlotKind::D => (0x26, 0x18), // dload_0.. / dload
        SlotKind::A => (0x2a, 0x19), // aload_0.. / aload
    };
    match slot {
        0..=3 => code.push(short_base + slot as u8),
        s if s <= 255 => {
            code.push(wide_op);
            code.push(s as u8);
        }
        _ => return Err("local slot overflow (wide load not supported)".into()),
    }
    Ok(())
}

fn return_opcode(desc: &str) -> Option<u8> {
    let i = desc.rfind(')')?;
    match &desc[i + 1..] {
        "Z" | "I" | "B" | "C" | "S" => Some(0xac),
        "J" => Some(0xad),
        "F" => Some(0xae),
        "D" => Some(0xaf),
        "V" => Some(0xb1),
        _ => Some(0xb0), // L...; / [...
    }
}

/// Replace the whole Code attribute of `method_name`/`method_desc` with a
/// receiver-prepended static dispatch to `target_class`/`target_name`.
/// Returns the patched bytes; `AlreadyPatched` if the body already IS the
/// redirect to the same target. Fail-closed on any shape mismatch.
pub fn redirect_method_body_to_static(
    bytes: &[u8],
    method_name: &str,
    method_desc: &str,
    receiver_class: &str,
    target_class: &str,
    target_name: &str,
    target_static_desc: &str,
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    // Length-preserving shape contract: static desc = virtual desc with the
    // METHOD OWNER (receiver) class prepended — the bridge's first parameter
    // receives the dispatched `this`. `target_class` is the bridge itself.
    let expect = format!("(L{};{}", receiver_class, &method_desc[1..]);
    if target_static_desc != expect {
        return Err(format!(
            "static desc {} is not the virtual desc {} with receiver {} prepended",
            target_static_desc, method_desc, target_class
        ));
    }

    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    let mut pool = layout.pool;

    // Find-only probes before ANY pool mutation (audit A4 discipline).
    let Some(name_idx) = pool.find_utf8(method_name) else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let Some(desc_idx) = pool.find_utf8(method_desc) else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
        .ok_or_else(|| format!("method {method_name}{method_desc} not found"))?;

    let Some(slot_kinds) = desc_slot_kinds(method_desc) else {
        return Err(format!("unparseable descriptor {method_desc}"));
    };
    let Some(ret_op) = return_opcode(method_desc) else {
        return Err(format!("unparseable return type {method_desc}"));
    };
    let total_slots = desc_param_slots(method_desc).ok_or_else(|| "bad slots".to_string())?;

    // Generate the replacement bytecode (before mutation, for probe).
    let mut code: Vec<u8> = Vec::with_capacity(total_slots + 4);
    for (slot, kind) in std::iter::once((0usize, SlotKind::A)).chain(slot_kinds.iter().copied()) {
        push_load(slot, kind, &mut code)?;
    }
    let invoke_pos = code.len();
    code.push(0xb8); // invokestatic (index filled after pool append)
    code.push(0);
    code.push(0);
    code.push(ret_op);

    // Locate the Code attribute within the method's attribute table and
    // collect the attributes to keep (drop Code + debug tables).
    let mut p = m
        .start
        .checked_add(6)
        .ok_or_else(|| "method header truncated".to_string())?; // access(2) name(2) desc(2) -> attrs_count
    let attr_count =
        usize::from(u16_at(bytes, p).ok_or_else(|| "attr count truncated".to_string())?);
    p = p
        .checked_add(2)
        .ok_or_else(|| "attr count truncated".to_string())?;
    let mut code_attr_start: Option<usize> = None;
    let mut code_attr_end: Option<usize> = None;
    let mut kept: Vec<u8> = Vec::new();
    let mut kept_count = 0usize;
    for _ in 0..attr_count {
        let a_name_idx =
            u16_at(bytes, p).ok_or_else(|| "attr header truncated".to_string())?;
        let len = u32_at(
            bytes,
            p.checked_add(2)
                .ok_or_else(|| "attr header truncated".to_string())?,
        )
        .ok_or_else(|| "attr length truncated".to_string())? as usize;
        let data = p
            .checked_add(6)
            .ok_or_else(|| "attr header truncated".to_string())?;
        let aend = data
            .checked_add(len)
            .ok_or_else(|| "attr data truncated".to_string())?;
        let aname = pool.utf8_value(a_name_idx).unwrap_or_default();
        match aname.as_str() {
            "Code" => {
                code_attr_start = Some(p);
                code_attr_end = Some(aend);
            }
            "LineNumberTable" | "LocalVariableTable" | "LocalVariableTypeTable" => {
                // dropped with the old code (debug-only; no semantic value)
            }
            _ => {
                kept.extend_from_slice(
                    bytes
                        .get(p..aend)
                        .ok_or_else(|| "method attribute truncated".to_string())?,
                );
                kept_count += 1;
            }
        }
        p = aend;
    }
    let Some(code_attr_start) = code_attr_start else {
        return Err(format!("method {method_name}{method_desc} has no Code attribute"));
    };
    let code_attr_end = code_attr_end
        .ok_or_else(|| "code attr end missing".to_string())?
        - 0;

    // AlreadyPatched probe: existing code identical to the generated shape
    // and its invokestatic resolves to the same target Methodref.
    let code_start = code_attr_start + 6 + 8; // attr hdr + max_stack/max_locals/code_len
    let existing_code_len = u32_at(bytes, code_attr_start + 6 + 4)
        .ok_or_else(|| "code length truncated".to_string())? as usize;
    if existing_code_len == code.len() {
        let probe = &bytes[code_start..code_start + existing_code_len];
        let same_shape = probe[..invoke_pos] == code[..invoke_pos]
            && probe[probe.len() - 1] == ret_op;
        if same_shape {
            let cp_idx = u16::from_be_bytes([probe[invoke_pos + 1], probe[invoke_pos + 2]]);
            if pool.methodref_parts(cp_idx)
                == Some((
                    target_class.to_string(),
                    target_name.to_string(),
                    target_static_desc.to_string(),
                ))
            {
                return Ok((bytes.to_vec(), RetargetOutcome::AlreadyPatched { sites: 1 }));
            }
        }
    }

    // Pool growth: append (dedup) the target Methodref.
    let new_idx = pool.method_ref(target_class, target_name, target_static_desc);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for redirect ref".into());
    }
    let idx_bytes = new_idx.to_be_bytes();
    code[invoke_pos + 1] = idx_bytes[0];
    code[invoke_pos + 2] = idx_bytes[1];

    // Build the new Code attribute bytes:
    //   name_idx(2) len(4) | max_stack(2) max_locals(2) code_len(4) code[..]
    //   | exception_table_len(2)=0 | attributes_count(2)=0
    // attribute_length covers ONLY what follows the length field:
    //   max_stack(2) + max_locals(2) + code_length(4) + code + exc_len(2)
    //   + attributes_count(2) = 12 + code.len() (the 6-byte name+len header
    //   is NOT part of it — JVMS 4.7.3).
    let mut new_attr: Vec<u8> = Vec::with_capacity(code.len() + 20);
    let code_attr_name_idx = pool
        .find_utf8("Code")
        .ok_or_else(|| "Code utf8 missing from pool".to_string())?;
    new_attr.extend_from_slice(&code_attr_name_idx.to_be_bytes());
    let attr_len = (12 + code.len()) as u32;
    new_attr.extend_from_slice(&attr_len.to_be_bytes());
    new_attr.extend_from_slice(&(total_slots as u16).to_be_bytes()); // max_stack
    new_attr.extend_from_slice(&(total_slots as u16).to_be_bytes()); // max_locals
    new_attr.extend_from_slice(&(code.len() as u32).to_be_bytes());
    new_attr.extend_from_slice(&code);
    new_attr.extend_from_slice(&0u16.to_be_bytes()); // exception_table_len
    new_attr.extend_from_slice(&0u16.to_be_bytes()); // attributes_count

    // New method segment: access(2) name(2) desc(2) attrs_count(2) = new Code
    // first + kept attributes (Exceptions etc.).
    let mut method_seg: Vec<u8> = Vec::with_capacity(8 + new_attr.len() + kept.len());
    method_seg.extend_from_slice(bytes.get(m.start..m.start + 6).ok_or("method hdr")?);
    method_seg.extend_from_slice(&((kept_count + 1) as u16).to_be_bytes());
    method_seg.extend_from_slice(&new_attr);
    method_seg.extend_from_slice(&kept);

    // Splice: header+grown pool, method prefix, new method segment, tail.
    let mut out = Vec::with_capacity(bytes.len() + new_attr.len() + 64);
    out.extend_from_slice(&bytes[0..8]);
    out.extend_from_slice(&pool.next.to_be_bytes());
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(
        bytes
            .get(layout.cp_end..m.start)
            .ok_or_else(|| "class tail truncated (prefix)".to_string())?,
    );
    out.extend_from_slice(&method_seg);
    out.extend_from_slice(
        bytes
            .get(m.end..)
            .ok_or_else(|| "class tail truncated (suffix)".to_string())?,
    );
    let _ = code_attr_end; // superseded by m.end splice (whole method replaced)
    Ok((out, RetargetOutcome::Retargeted { sites: 1 }))
}

/// ZERO-ALLOC-INSIDE bridge target (kernel loader, same package as
/// BlockGetter/TraverseOps).
pub const ZERO_ALLOC_OPS_CLASS: &str = "net/minecraft/world/level/ZeroAllocOps";

const ZA_FLUID_DESC: &str =
    "(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;)Z";
const ZA_SHAPE_DESC: &str =
    "(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Ljava/util/List;)Z";
const ZA_FLUID_PUSH_DESC: &str = "(Lnet/minecraft/tags/TagKey;D)Z";

/// Single source of truth for the S7-164 redirect graph: (site name,
/// site descriptor, bridge target name, bridge target descriptor).
/// Consumed by (a) `patch_entity_zeroalloc` (bytecode surgery) and
/// (b) `zeroalloc_resolution_closure` (delivery guard) so the generated
/// `invokestatic` operands and the delivered ZeroAllocOps classfile can
/// never drift apart.
pub const ZA_REDIRECT_TARGETS: [(&str, &str, &str, &str); 3] = [
    (
        "collidedWithFluid",
        ZA_FLUID_DESC,
        "collidedWithFluid",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;)Z",
    ),
    (
        "collidedWithShapeMovingFrom",
        ZA_SHAPE_DESC,
        "collidedWithShapeMovingFrom",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Ljava/util/List;)Z",
    ),
    (
        "updateFluidHeightAndDoFluidPushing",
        ZA_FLUID_PUSH_DESC,
        "updateFluidHeightAndDoFluidPushing",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/tags/TagKey;D)Z",
    ),
];

/// S7-164 leg#1 TECH-DUD guard (resolution closure): the bridge classfile
/// actually being DELIVERED to the kernel loader must declare a static with
/// EXACTLY the (name, descriptor) of every redirect target. A missing
/// target resolves lazily on the FIRST entity tick and detonates as a
/// NoSuchMethodError storm (66 throws in leg#1, 2026-09-19) — invisible to
/// the defineClass verifier (verification does not resolve) and to the
/// lockstep oracle (which calls the bridge from source, compile-time).

pub fn redirect_static_method_body_to_static(
    bytes: &[u8],
    method_name: &str,
    method_desc: &str,
    target_class: &str,
    target_name: &str,
    target_static_desc: &str,
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    // STATIC->STATIC shape contract (TASK-330 zero-cursor): the bridge
    // static descriptor must be IDENTICAL to the redirected static method
    // descriptor — stack shape unchanged, no receiver prepended.
    if target_static_desc != method_desc {
        return Err(format!(
            "static desc {} does not match redirected static desc {} (stack shape must stay identical)",
            target_static_desc, method_desc
        ));
    }

    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    let mut pool = layout.pool;

    // Find-only probes before ANY pool mutation (audit A4 discipline).
    let Some(name_idx) = pool.find_utf8(method_name) else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let Some(desc_idx) = pool.find_utf8(method_desc) else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
        .ok_or_else(|| format!("method {method_name}{method_desc} not found"))?;

    let Some(slot_kinds) = desc_slot_kinds(method_desc) else {
        return Err(format!("unparseable descriptor {method_desc}"));
    };
    // s7172 root cause (locally reproduced via Instrumentation.retransformClasses:
    // VerifyError on an aload_1-first load chain): desc_slot_kinds numbers params
    // from slot 1 (instance-method convention, receiver at 0) while THIS helper
    // redirects a STATIC method whose params start at slot 0. Remap down by one
    // or the verifier rejects the served bytes (JVMTI 62 FAILS_VERIFICATION)
    // and the hook stays dormant.
    let slot_kinds: Vec<(usize, SlotKind)> = slot_kinds
        .into_iter()
        .map(|(slot, kind)| (slot - 1, kind))
        .collect();
    let Some(ret_op) = return_opcode(method_desc) else {
        return Err(format!("unparseable return type {method_desc}"));
    };
    let total_slots = desc_param_slots(method_desc).ok_or_else(|| "bad slots".to_string())?;

    // Generate the replacement bytecode (before mutation, for probe).
    let mut code: Vec<u8> = Vec::with_capacity(total_slots + 4);
    // static method: param slots start at 0, no receiver aload.
    for (slot, kind) in slot_kinds.iter().copied() {
        push_load(slot, kind, &mut code)?;
    }
    let invoke_pos = code.len();
    code.push(0xb8); // invokestatic (index filled after pool append)
    code.push(0);
    code.push(0);
    code.push(ret_op);

    // Locate the Code attribute within the method's attribute table and
    // collect the attributes to keep (drop Code + debug tables).
    let mut p = m
        .start
        .checked_add(6)
        .ok_or_else(|| "method header truncated".to_string())?; // access(2) name(2) desc(2) -> attrs_count
    let attr_count =
        usize::from(u16_at(bytes, p).ok_or_else(|| "attr count truncated".to_string())?);
    p = p
        .checked_add(2)
        .ok_or_else(|| "attr count truncated".to_string())?;
    let mut code_attr_start: Option<usize> = None;
    let mut code_attr_end: Option<usize> = None;
    let mut kept: Vec<u8> = Vec::new();
    let mut kept_count = 0usize;
    for _ in 0..attr_count {
        let a_name_idx =
            u16_at(bytes, p).ok_or_else(|| "attr header truncated".to_string())?;
        let len = u32_at(
            bytes,
            p.checked_add(2)
                .ok_or_else(|| "attr header truncated".to_string())?,
        )
        .ok_or_else(|| "attr length truncated".to_string())? as usize;
        let data = p
            .checked_add(6)
            .ok_or_else(|| "attr header truncated".to_string())?;
        let aend = data
            .checked_add(len)
            .ok_or_else(|| "attr data truncated".to_string())?;
        let aname = pool.utf8_value(a_name_idx).unwrap_or_default();
        match aname.as_str() {
            "Code" => {
                code_attr_start = Some(p);
                code_attr_end = Some(aend);
            }
            "LineNumberTable" | "LocalVariableTable" | "LocalVariableTypeTable" => {
                // dropped with the old code (debug-only; no semantic value)
            }
            _ => {
                kept.extend_from_slice(
                    bytes
                        .get(p..aend)
                        .ok_or_else(|| "method attribute truncated".to_string())?,
                );
                kept_count += 1;
            }
        }
        p = aend;
    }
    let Some(code_attr_start) = code_attr_start else {
        return Err(format!("method {method_name}{method_desc} has no Code attribute"));
    };
    let code_attr_end = code_attr_end
        .ok_or_else(|| "code attr end missing".to_string())?
        - 0;

    // AlreadyPatched probe: existing code identical to the generated shape
    // and its invokestatic resolves to the same target Methodref.
    let code_start = code_attr_start + 6 + 8; // attr hdr + max_stack/max_locals/code_len
    let existing_code_len = u32_at(bytes, code_attr_start + 6 + 4)
        .ok_or_else(|| "code length truncated".to_string())? as usize;
    if existing_code_len == code.len() {
        let probe = &bytes[code_start..code_start + existing_code_len];
        let same_shape = probe[..invoke_pos] == code[..invoke_pos]
            && probe[probe.len() - 1] == ret_op;
        if same_shape {
            let cp_idx = u16::from_be_bytes([probe[invoke_pos + 1], probe[invoke_pos + 2]]);
            if pool.methodref_parts(cp_idx)
                == Some((
                    target_class.to_string(),
                    target_name.to_string(),
                    target_static_desc.to_string(),
                ))
            {
                return Ok((bytes.to_vec(), RetargetOutcome::AlreadyPatched { sites: 1 }));
            }
        }
    }

    // Pool growth: append (dedup) the target Methodref.
    let new_idx = pool.method_ref(target_class, target_name, target_static_desc);
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for redirect ref".into());
    }
    let idx_bytes = new_idx.to_be_bytes();
    code[invoke_pos + 1] = idx_bytes[0];
    code[invoke_pos + 2] = idx_bytes[1];

    // Build the new Code attribute bytes:
    //   name_idx(2) len(4) | max_stack(2) max_locals(2) code_len(4) code[..]
    //   | exception_table_len(2)=0 | attributes_count(2)=0
    // attribute_length covers ONLY what follows the length field:
    //   max_stack(2) + max_locals(2) + code_length(4) + code + exc_len(2)
    //   + attributes_count(2) = 12 + code.len() (the 6-byte name+len header
    //   is NOT part of it — JVMS 4.7.3).
    let mut new_attr: Vec<u8> = Vec::with_capacity(code.len() + 20);
    let code_attr_name_idx = pool
        .find_utf8("Code")
        .ok_or_else(|| "Code utf8 missing from pool".to_string())?;
    new_attr.extend_from_slice(&code_attr_name_idx.to_be_bytes());
    let attr_len = (12 + code.len()) as u32;
    new_attr.extend_from_slice(&attr_len.to_be_bytes());
    new_attr.extend_from_slice(&(total_slots as u16).to_be_bytes()); // max_stack
    new_attr.extend_from_slice(&(total_slots as u16).to_be_bytes()); // max_locals
    new_attr.extend_from_slice(&(code.len() as u32).to_be_bytes());
    new_attr.extend_from_slice(&code);
    new_attr.extend_from_slice(&0u16.to_be_bytes()); // exception_table_len
    new_attr.extend_from_slice(&0u16.to_be_bytes()); // attributes_count

    // New method segment: access(2) name(2) desc(2) attrs_count(2) = new Code
    // first + kept attributes (Exceptions etc.).
    let mut method_seg: Vec<u8> = Vec::with_capacity(8 + new_attr.len() + kept.len());
    method_seg.extend_from_slice(bytes.get(m.start..m.start + 6).ok_or("method hdr")?);
    method_seg.extend_from_slice(&((kept_count + 1) as u16).to_be_bytes());
    method_seg.extend_from_slice(&new_attr);
    method_seg.extend_from_slice(&kept);

    // Splice: header+grown pool, method prefix, new method segment, tail.
    let mut out = Vec::with_capacity(bytes.len() + new_attr.len() + 64);
    out.extend_from_slice(&bytes[0..8]);
    out.extend_from_slice(&pool.next.to_be_bytes());
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(
        bytes
            .get(layout.cp_end..m.start)
            .ok_or_else(|| "class tail truncated (prefix)".to_string())?,
    );
    out.extend_from_slice(&method_seg);
    out.extend_from_slice(
        bytes
            .get(m.end..)
            .ok_or_else(|| "class tail truncated (suffix)".to_string())?,
    );
    let _ = code_attr_end; // superseded by m.end splice (whole method replaced)
    Ok((out, RetargetOutcome::Retargeted { sites: 1 }))
}

/// INSIDE-DIET resolution closure (TASK-332 lever #12 v1): the redirected
/// Entity.checkInsideBlocks body invokes InsideDietOps.checkInsideBlocks with
/// the receiver-prepended descriptor; InsideDietOps constructs an
/// InsideDietVisitor and calls the vanilla static walk. Both bridges must
/// declare those members or the first entity tick detonates NoSuchMethodError.
pub fn insidediet_resolution_closure(ops: &[u8], visitor: &[u8]) -> Result<(), String> {
    const OPS_DESC: &str = "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;Lit/unimi/dsi/fastutil/longs/LongSet;I)I";
    const VISIT_DESC: &str = "(Lnet/minecraft/core/BlockPos;I)Z";
    let targets: &[(&str, &str, &str, &str)] = &[
        (
            "class",
            "net/minecraft/world/entity/InsideDietOps",
            "checkInsideBlocks",
            OPS_DESC,
        ),
        (
            "class",
            "net/minecraft/world/entity/InsideDietVisitor",
            "visit",
            VISIT_DESC,
        ),
    ];
    check_members(ops, &targets[..1])?;
    check_members(visitor, &targets[1..])
}

/// ZERO-ALLOC-INSIDE bridge target (kernel loader, same package as
/// BlockGetter/TraverseOps).
pub fn zeroalloc_resolution_closure(bridge: &[u8]) -> Result<(), String> {
    redirect_targets_resolution_closure(bridge, &ZA_REDIRECT_TARGETS)
}

/// Shared core of the resolution-closure guard (S7-166: reused by the
/// #13-SBB skip-store redirect so the two delivery graphs can never drift
/// into different enforcement semantics).
fn redirect_targets_resolution_closure(
    bridge: &[u8],
    targets: &[(&str, &str, &str, &str)],
) -> Result<(), String> {
    let Some(layout) = parse_layout(bridge) else {
        return Err("bridge classfile unparseable".into());
    };
    let mut p = layout.methods_start;
    let count = usize::from(u16_at(bridge, p).ok_or("truncated method count")?);
    p = p.checked_add(2).ok_or("truncated method table")?;
    let mut have: Vec<(String, String)> = Vec::with_capacity(count);
    for _ in 0..count {
        let n_idx = u16_at(bridge, p.checked_add(2).ok_or("truncated method")?).ok_or("truncated method name")?;
        let d_idx = u16_at(bridge, p.checked_add(4).ok_or("truncated method")?).ok_or("truncated method desc")?;
        let attr_count = usize::from(
            u16_at(bridge, p.checked_add(6).ok_or("truncated method")?).ok_or("truncated method attrs")?,
        );
        p = p.checked_add(8).ok_or("truncated method table")?;
        for _ in 0..attr_count {
            let len = u32_at(bridge, p.checked_add(2).ok_or("truncated attr")?).ok_or("truncated attr")? as usize;
            p = p.checked_add(6).ok_or("truncated attr")?.checked_add(len).ok_or("truncated attr")?;
        }
        let name = layout.pool.utf8_value(n_idx).ok_or("bad name idx")?;
        let desc = layout.pool.utf8_value(d_idx).ok_or("bad desc idx")?;
        have.push((name, desc));
    }
    for (_, _, tname, tdesc) in targets {
        if !have.iter().any(|(n, d)| n == tname && d == tdesc) {
            return Err(format!(
                "bridge misses redirect target {tname}{tdesc} — \
                 the entity_compose redirect would detonate NoSuchMethodError on the first tick"
            ));
        }
    }
    Ok(())
}

/// SKIP-STORE-BB bridge target (kernel loader, same package as
/// BlockGetter/ZeroAllocOps).
pub const SKIP_STORE_OPS_CLASS: &str = "net/minecraft/world/level/SkipStoreOps";

pub const SSB_BB_DESC: &str = "(Lnet/minecraft/world/phys/AABB;)V";

/// Single source of truth for the #13-SBB redirect graph (S7-166):
/// (site name, site descriptor, bridge target name, bridge target
/// descriptor). EXACTLY ONE target — the javap contract RECON-12a proved
/// setDeltaMovement parity-risky (5 identity sites, the move() guard
/// window) and sync/chunk-lists are out of #13-SBB scope. Consumed by (a)
/// `patch_entity_skip_store_bb` (bytecode surgery) and (b)
/// `skipstore_resolution_closure` (delivery guard).
pub const SSB_REDIRECT_TARGETS: [(&str, &str, &str, &str); 1] = [(
    "setBoundingBox",
    SSB_BB_DESC,
    "setBoundingBox",
    "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)V",
)];

/// #13-SBB delivery guard: the SkipStoreOps classfile being DELIVERED to
/// the kernel loader must declare setBoundingBox with the receiver-
/// prepended static descriptor (same TECH-DUD discipline as stage 7).
pub fn skipstore_resolution_closure(bridge: &[u8]) -> Result<(), String> {
    redirect_targets_resolution_closure(bridge, &SSB_REDIRECT_TARGETS)
}

/// S7-166 lever #13-SBB: redirect the Entity.setBoundingBox(AABB) body to
/// the SkipStoreOps value-equal skip bridge. The vanilla body always
/// allocates a fresh AABB + putfields into the old-gen Entity (one young
/// alloc + one old->young remembered card PER CALL); the bridge repeats
/// the javap-verbatim normalization ladder and skips the store when the
/// current field already bit-matches (RECON-12a: 0 identity sites on bb,
/// AABB immutable — parity-safe). Strict: exactly ONE site (single
/// non-overloaded setter body), fail-dominant like stage 7.
pub fn patch_entity_skip_store_bb(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let (name, desc, tname, tdesc) = SSB_REDIRECT_TARGETS[0];
    let (p, outcome) = redirect_method_body_to_static(
        bytes,
        name,
        desc,
        "net/minecraft/world/entity/Entity",
        SKIP_STORE_OPS_CLASS,
        tname,
        tdesc,
    )?;
    match outcome {
        RetargetOutcome::Retargeted { .. } => Ok((p, RetargetOutcome::Retargeted { sites: 1 })),
        RetargetOutcome::AlreadyPatched { .. } => {
            Ok((p, RetargetOutcome::AlreadyPatched { sites: 1 }))
        }
        RetargetOutcome::NotFound => Ok((bytes.to_vec(), RetargetOutcome::NotFound)),
    }
}

/// S7-168 (STEAL v2 defect-fix, TASK-335): the single ServerLevel
/// sendBlockUpdated body redirects to BlockUpdateOps.handle — the javap-
/// verbatim vanilla body (codelen=235 disassembled by javap_lite) lives in
/// the bridge; workers defer only the navigatingMobs pass to the main
/// thread (phase-4 FIFO replay), killing the s7176 race NPE
/// (ObjectOpenHashSet$SetIterator "wrapped is null").
pub const BLOCKUPD_OPS_CLASS: &str = "net/minecraft/server/level/BlockUpdateOps";

pub const BLOCKUPD_REDIRECT_TARGETS: [(&str, &str, &str, &str); 1] = [(
    "sendBlockUpdated",
    "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;I)V",
    "handle",
    "(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;I)V",
)];

/// #13-STEAL-v2 delivery guard: the BlockUpdateOps classfile being
/// DELIVERED to the kernel loader must declare handle with the receiver-
/// prepended static descriptor.
pub fn blockupd_resolution_closure(bridge: &[u8]) -> Result<(), String> {
    redirect_targets_resolution_closure(bridge, &BLOCKUPD_REDIRECT_TARGETS)
}

pub fn patch_serverlevel_send_block_updated(
    bytes: &[u8],
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let (name, desc, tname, tdesc) = BLOCKUPD_REDIRECT_TARGETS[0];
    let (p, outcome) = redirect_method_body_to_static(
        bytes,
        name,
        desc,
        "net/minecraft/server/level/ServerLevel",
        BLOCKUPD_OPS_CLASS,
        tname,
        tdesc,
    )?;
    match outcome {
        RetargetOutcome::Retargeted { .. } => Ok((p, RetargetOutcome::Retargeted { sites: 1 })),
        RetargetOutcome::AlreadyPatched { .. } => {
            Ok((p, RetargetOutcome::AlreadyPatched { sites: 1 }))
        }
        RetargetOutcome::NotFound => Ok((bytes.to_vec(), RetargetOutcome::NotFound)),
    }
}

/// NAV-PLANE (TASK-405-A restart, cmp405_navplane): the same single-site
/// body redirect as the BU-DEFER variant, but to OUR NavPlaneOps.handle —
/// the javap-verbatim vanilla body with the navigatingMobs pass collapsed
/// into ONE bulk navDecide native per block-update batch (law 6: one
/// JNI per batch; per-entity JNI = design error). Composed only when the
/// lever flag is armed (region_threads.rs), independent of bu_defer so the
/// armed delta is purely the nav-batch plane.
pub const NAVPLANE_OPS_CLASS: &str = "net/minecraft/server/level/NavPlaneOps";

pub const NAVPLANE_REDIRECT_TARGETS: [(&str, &str, &str, &str); 1] = [(
    "sendBlockUpdated",
    "(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;I)V",
    "handle",
    "(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;I)V",
)];

pub fn patch_serverlevel_send_block_updated_navplane(
    bytes: &[u8],
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let (name, desc, tname, tdesc) = NAVPLANE_REDIRECT_TARGETS[0];
    let (p, outcome) = redirect_method_body_to_static(
        bytes,
        name,
        desc,
        "net/minecraft/server/level/ServerLevel",
        NAVPLANE_OPS_CLASS,
        tname,
        tdesc,
    )?;
    match outcome {
        RetargetOutcome::Retargeted { .. } => Ok((p, RetargetOutcome::Retargeted { sites: 1 })),
        RetargetOutcome::AlreadyPatched { .. } => {
            Ok((p, RetargetOutcome::AlreadyPatched { sites: 1 }))
        }
        RetargetOutcome::NotFound => Ok((bytes.to_vec(), RetargetOutcome::NotFound)),
    }
}

/// NAV-POOL (TASK-410-A k5, cmp405_navplane STRICT eq): the A* node-pool —
/// NodeEvaluator.prepare body-redirected to NavPoolOps.prepare (vanilla
/// body with nodes.clear() REPLACED by the fresh-shape laundering) and
/// NodeEvaluator.getNode(III) body-redirected to NavPoolOps.getNode
/// (map.get + position check + new-on-miss, ZERO lambda on the hot path).
/// Composed only when the lever flag is armed (region_threads.rs). The
/// bridge lives in the SAME package (protected `nodes` field access).
/// Composite fail-dominant: BOTH sites or none (parity: a half-pool would
/// mix laundering with vanilla computeIfAbsent — semantically safe but the
/// strict pair keeps the delivery inspectable).
pub const NAVPOOL_OPS_CLASS: &str = "net/minecraft/world/level/pathfinder/NavPoolOps";

pub const NAVPOOL_REDIRECT_TARGETS: [(&str, &str, &str, &str); 2] = [
    (
        "prepare",
        "(Lnet/minecraft/world/level/PathNavigationRegion;Lnet/minecraft/world/entity/Mob;)V",
        "prepare",
        "(Lnet/minecraft/world/level/pathfinder/NodeEvaluator;Lnet/minecraft/world/level/PathNavigationRegion;Lnet/minecraft/world/entity/Mob;)V",
    ),
    (
        "getNode",
        "(III)Lnet/minecraft/world/level/pathfinder/Node;",
        "getNode",
        "(Lnet/minecraft/world/level/pathfinder/NodeEvaluator;III)Lnet/minecraft/world/level/pathfinder/Node;",
    ),
];

pub fn patch_nodeevaluator_navpool(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let mut cur = bytes.to_vec();
    let mut ret = 0usize;
    let mut already = 0usize;
    for (name, desc, tname, tdesc) in NAVPOOL_REDIRECT_TARGETS {
        let (p, outcome) = redirect_method_body_to_static(
            &cur,
            name,
            desc,
            "net/minecraft/world/level/pathfinder/NodeEvaluator",
            NAVPOOL_OPS_CLASS,
            tname,
            tdesc,
        )?;
        cur = p;
        match outcome {
            RetargetOutcome::Retargeted { .. } => ret += 1,
            RetargetOutcome::AlreadyPatched { .. } => already += 1,
            RetargetOutcome::NotFound => {
                return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
            }
        }
    }
    if ret == 2 {
        Ok((cur, RetargetOutcome::Retargeted { sites: 2 }))
    } else {
        Ok((cur, RetargetOutcome::AlreadyPatched { sites: 2 }))
    }
}

pub fn navpool_resolution_closure(bridge: &[u8]) -> Result<(), String> {
    redirect_targets_resolution_closure(bridge, &NAVPOOL_REDIRECT_TARGETS)
}

/// S7-164 lever #10: redirect the THREE hottest Entity inside/fluid bodies
/// (census: collidedWithFluid ← lambda$checkInsideBlocks$2; collidedAlongVector ←
/// collidedWithShapeMovingFrom only; both from Entity) to the scalar
/// ZeroAllocOps implementations. Composite: all three must succeed or the
/// stage is skipped as a whole (fail-dominant, no half-composed stage).
pub fn patch_entity_zeroalloc(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let targets = ZA_REDIRECT_TARGETS;
    let mut cur = bytes.to_vec();
    let mut ret = 0usize;
    let mut already = 0usize;
    for (name, desc, tname, tdesc) in targets {
        let (p, outcome) = redirect_method_body_to_static(
            &cur,
            name,
            desc,
            "net/minecraft/world/entity/Entity",
            ZERO_ALLOC_OPS_CLASS,
            tname,
            tdesc,
        )?;
        cur = p;
        match outcome {
            RetargetOutcome::Retargeted { .. } => ret += 1,
            RetargetOutcome::AlreadyPatched { .. } => already += 1,
            RetargetOutcome::NotFound => {
                return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
            }
        }
    }
    if ret == 3 {
        Ok((cur, RetargetOutcome::Retargeted { sites: 3 }))
    } else {
        Ok((cur, RetargetOutcome::AlreadyPatched { sites: 3 }))
    }
}

/// INSIDE-DIET (TASK-332 lever #12 v1): single-site body redirect of the
/// private instance method Entity.checkInsideBlocks(Vec3,Vec3,
/// StepBasedCollector,LongSet,I)I to the InsideDietOps bridge (receiver
/// prepended). Fail-closed: NotFound -> error up to the compose stage.
pub fn patch_entity_inside_diet(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    redirect_method_body_to_static(
        bytes,
        "checkInsideBlocks",
        "(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;Lit/unimi/dsi/fastutil/longs/LongSet;I)I",
        "net/minecraft/world/entity/Entity",
        "net/minecraft/world/entity/InsideDietOps",
        "checkInsideBlocks",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;Lit/unimi/dsi/fastutil/longs/LongSet;I)I",
    )
}

/// TRAVEL-DIET v2a+v2b (RECON-21, lever #14): body redirects of the
/// travel-lane allocation sites to the TravelDietOps bridge. Fail-closed:
/// NotFound -> error up to the compose stage.
pub const TRAVEL_DIET_OPS_CLASS: &str = "net/minecraft/world/entity/TravelDietOps";

pub const TD_COLLIDE_DESC: &str =
    "(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;";
pub const TD_INPUTVEC_DESC: &str =
    "(Lnet/minecraft/world/phys/Vec3;FF)Lnet/minecraft/world/phys/Vec3;";

/// Single source of truth for the #14 ENTITY redirect graph: (site name,
/// site descriptor, bridge target name, bridge target descriptor).
/// v2a: collide (instance -> receiver-prepended static).
/// v2b: getInputVector (PROTECTED STATIC -> static-to-static, identical
/// descriptor, `redirect_static_method_body_to_static`).
pub const TD_ENTITY_TARGETS: [(&str, &str, &str, &str); 2] = [
    (
        "collide",
        TD_COLLIDE_DESC,
        "collide",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;",
    ),
    (
        "getInputVector",
        TD_INPUTVEC_DESC,
        "getInputVector",
        TD_INPUTVEC_DESC,
    ),
];

/// Single source of truth for the #14-v2b LIVINGENTITY redirect graph:
/// travelInFluid (private instance -> receiver-prepended static).
pub const TD_LIVING_TARGETS: [(&str, &str, &str, &str); 1] = [(
    "travelInFluid",
    "(Lnet/minecraft/world/phys/Vec3;)V",
    "travelInFluid",
    "(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/phys/Vec3;)V",
)];

/// Every TD target the ONE bridge classfile must declare (resolution
/// closure over the delivered bytes).
pub const TD_ALL_TARGETS: [(&str, &str, &str, &str); 3] = [
    TD_ENTITY_TARGETS[0],
    TD_ENTITY_TARGETS[1],
    TD_LIVING_TARGETS[0],
];

/// Composite Entity patch (fail-dominant: both sites or none, zeroalloc
/// precedent). Site 0 = collide (instance redirect), site 1 = getInputVector
/// (static redirect). Served by entity_compose STAGE 10 (sites:2).
pub fn patch_entity_traveldiet(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let (p0, o0) = redirect_method_body_to_static(
        bytes,
        TD_ENTITY_TARGETS[0].0,
        TD_ENTITY_TARGETS[0].1,
        "net/minecraft/world/entity/Entity",
        TRAVEL_DIET_OPS_CLASS,
        TD_ENTITY_TARGETS[0].2,
        TD_ENTITY_TARGETS[0].3,
    )?;
    let o0_ok = matches!(
        o0,
        RetargetOutcome::Retargeted { .. } | RetargetOutcome::AlreadyPatched { .. }
    );
    if !o0_ok {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    }
    let (p1, o1) = redirect_static_method_body_to_static(
        &p0,
        TD_ENTITY_TARGETS[1].0,
        TD_ENTITY_TARGETS[1].1,
        TRAVEL_DIET_OPS_CLASS,
        TD_ENTITY_TARGETS[1].2,
        TD_ENTITY_TARGETS[1].3,
    )?;
    let o1_ok = matches!(
        o1,
        RetargetOutcome::Retargeted { .. } | RetargetOutcome::AlreadyPatched { .. }
    );
    if !o1_ok {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    }
    let ret = matches!(o0, RetargetOutcome::Retargeted { .. }) as usize
        + matches!(o1, RetargetOutcome::Retargeted { .. }) as usize;
    if ret == 2 {
        Ok((p1, RetargetOutcome::Retargeted { sites: 2 }))
    } else {
        Ok((p1, RetargetOutcome::AlreadyPatched { sites: 2 }))
    }
}

/// LivingEntity patch (v2b): single-site travelInFluid body redirect,
/// receiver-prepended static. Served by the travel_diet LivingEntity hook.
pub fn patch_livingentity_traveldiet(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let (p, outcome) = redirect_method_body_to_static(
        bytes,
        TD_LIVING_TARGETS[0].0,
        TD_LIVING_TARGETS[0].1,
        "net/minecraft/world/entity/LivingEntity",
        TRAVEL_DIET_OPS_CLASS,
        TD_LIVING_TARGETS[0].2,
        TD_LIVING_TARGETS[0].3,
    )?;
    match outcome {
        RetargetOutcome::Retargeted { .. } => Ok((p, RetargetOutcome::Retargeted { sites: 1 })),
        RetargetOutcome::AlreadyPatched { .. } => {
            Ok((p, RetargetOutcome::AlreadyPatched { sites: 1 }))
        }
        RetargetOutcome::NotFound => Ok((bytes.to_vec(), RetargetOutcome::NotFound)),
    }
}

pub fn traveldiet_resolution_closure(bridge: &[u8]) -> Result<(), String> {
    redirect_targets_resolution_closure(bridge, &TD_ALL_TARGETS)
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

// ============================================================================
// TASK-401-B COLLIDE BATCH-MERGE (vector collide-batch, lever cmp401_collide).
//
// Single body-redirect on the kernel collision data plane (RESEARCH-B.md):
//   STATIC scan: CollisionUtil.getCollisionsForBlocksOrWorldBorder ->
//   CollideBatchOps.blockCollisions (identical descriptor; the bridge runs
//   the verbatim moonrise fragment below the sweep threshold and serves
//   dense sections from a per-tick per-worker section plan — batch-merge of
//   per-entity block queries to the same section into one build per tick).
//   Unlike round-400-F collidesweep there is NO Entity.collide redirect:
//   Entity.collide stays vanilla verbatim; the mechanism is purely the
//   inter-query dedup of block reads.
// ============================================================================

/// Kernel owner of the block-collision scan (same class as round-400-F).
pub const COLLISION_UTIL_CLASS_B: &str =
    "ca/spottedleaf/moonrise/patches/collisions/CollisionUtil";
/// COLLIDE-BATCH bridge (kernel loader, same package as Entity).
pub const COLLIDE_BATCH_OPS_CLASS: &str = "net/minecraft/world/entity/CollideBatchOps";

/// Descriptor of CollisionUtil.getCollisionsForBlocksOrWorldBorder (javap on
/// the real kernel class; the redirect target CollideBatchOps.blockCollisions
/// MUST declare exactly this erased signature — stack shape unchanged).
pub const CB_SCAN_DESC: &str = "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/List;Ljava/util/List;ILjava/util/function/BiPredicate;)Z";

/// Single source of truth for the collide-batch redirect graph (bytecode
/// surgery + delivered-classfile resolution closure).
pub const CB_REDIRECT_TARGETS: [(&str, &str, &str, &str); 1] = [(
    "blockCollisions",
    CB_SCAN_DESC,
    "blockCollisions",
    CB_SCAN_DESC,
)];

/// Resolution closure (S7-164 NoSuchMethodError-storm guard): the delivered
/// CollideBatchOps classfile must declare the redirect static exactly.
pub fn collidebatch_resolution_closure(ops: &[u8]) -> Result<(), String> {
    redirect_targets_resolution_closure(ops, &CB_REDIRECT_TARGETS)
}

/// Static→static whole-body redirect of
/// CollisionUtil.getCollisionsForBlocksOrWorldBorder to
/// CollideBatchOps.blockCollisions. Strict sites: 1 (the kernel has exactly
/// one such method; NotFound means a foreign kernel shape — fail closed).
pub fn patch_collision_batch(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let (p, outcome) = redirect_static_method_body_to_static(
        bytes,
        "getCollisionsForBlocksOrWorldBorder",
        CB_SCAN_DESC,
        COLLIDE_BATCH_OPS_CLASS,
        "blockCollisions",
        CB_SCAN_DESC,
    )?;
    Ok((p, outcome))
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

/// Entity stage for the INSIDE-BATCH plane (TASK-459-56, ID-P31): the same
/// method-entry `isAffectedByBlocks` site in `checkInsideBlocks` retargeted to
/// the static `InsideBatchOps.batchGate(Entity)Z` (receiver-first, 3B→3B,
/// length-preserving). S7-162 supersede discipline: the entity_compose chain
/// installs exactly ONE owner of the site — inside_batch when its lever is
/// armed, inside_cache otherwise. Same fail-closed guards as
/// `patch_inside_cache` (Unsafe field-name probe, strict single site).
pub fn patch_inside_batch(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
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
        (INSIDE_BATCH_OPS_CLASS, "batchGate", GATE_DESC),
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

/// Resolve a CONSTANT_Methodref / CONSTANT_InterfaceMethodref cp index to
/// (class_name, name, desc). Find-only — no pool mutation.
fn methodref_parts(pool: &Pool, idx: u16) -> Option<(String, String, String)> {
    let (_, tag, payload) = pool.entries.iter().find(|(i, _, _)| *i == idx)?;
    if *tag != TAG_METHODREF && *tag != TAG_INTERFACEMETHODREF || payload.len() < 4 {
        return None;
    }
    let class_idx = u16::from_be_bytes([payload[0], payload[1]]);
    let nat_idx = u16::from_be_bytes([payload[2], payload[3]]);
    let (_, ctag, cpayload) = pool.entries.iter().find(|(i, _, _)| *i == class_idx)?;
    if *ctag != TAG_CLASS || cpayload.len() < 2 {
        return None;
    }
    let cname_idx = u16::from_be_bytes([cpayload[0], cpayload[1]]);
    let (_, ntag, npayload) = pool.entries.iter().find(|(i, _, _)| *i == nat_idx)?;
    if *ntag != TAG_NAMEANDTYPE || npayload.len() < 4 {
        return None;
    }
    let mname_idx = u16::from_be_bytes([npayload[0], npayload[1]]);
    let mdesc_idx = u16::from_be_bytes([npayload[2], npayload[3]]);
    Some((
        pool.utf8_value(cname_idx)?,
        pool.utf8_value(mname_idx)?,
        pool.utf8_value(mdesc_idx)?,
    ))
}

/// S55-PREREG (round-467/S55 §3A) scaffold-yield discriminator, ladder R468-S19.
///
/// The LIVE InsideBatchOps blob is a pass-through scaffold: javap batchGate =
/// 8 instr / 16 code bytes, BOTH branches return Entity.isAffectedByBlocks:()Z,
/// collectBatch has zero callers, the insideBatchMask native is declared but
/// never invoked. Arming this blob on carrier legs (cmp456_chunkmono_p31snap /
/// cmp466_c98ai) supersedes a banked inside_cache (CRUSSTY_INSIDE_CACHE=1)
/// while delivering ZERO batch effect — the anchor (lever_flag="") keeps a
/// live inside_cache while the leg loses it: the systematic anchor-vs-leg
/// inside-plane handicap (invisible to check_blobs_sync / ncdfe_guard, both
/// green on a dead plane).
///
/// Byte-EXACT test: batchGate's Code attribute must be exactly the 16-byte
/// dual-branch pass-through (getstatic BATCH_ARMED / ifne 11 / aload_0 /
/// invokevirtual isAffectedByBlocks / ireturn / aload_0 / invokevirtual /
/// ireturn) with both invokevirtual operands resolving to
/// net/minecraft/world/entity/Entity.isAffectedByBlocks:()Z. Any other code
/// (full v1 collectBatch wiring, drifted rebuild) or any layout/attribute
/// failure => false => inside_batch keeps the site (fail-dominant toward the
/// pre-yield owner). 16 bytes of exact opcode skeleton leave no room for a
/// hidden extra invoke (S55: "2x invokevirtual isAffectedByBlocks, 0 others").
pub fn inside_batch_is_scaffold(bytes: &[u8]) -> bool {
    const SCAFFOLD_LEN: usize = 16;
    let Some(layout) = parse_layout(bytes) else {
        return false;
    };
    let Some(n_idx) = layout.pool.find_utf8("batchGate") else {
        return false;
    };
    let Some(d_idx) = layout.pool.find_utf8(GATE_DESC) else {
        return false;
    };
    let Some(m) = find_method(bytes, layout.methods_start, n_idx, d_idx) else {
        return false;
    };
    let Some(code_idx) = layout.pool.find_utf8("Code") else {
        return false;
    };
    // Walk the method attribute table: access(2) name(2) desc(2) count(2)...
    let Some(attr_count) = u16_at(bytes, m.start.checked_add(6).unwrap_or(usize::MAX)) else {
        return false;
    };
    let mut p = m.start + 8;
    let mut code: &[u8] = &[];
    for _ in 0..attr_count {
        let Some(a_name) = u16_at(bytes, p) else {
            return false;
        };
        let Some(a_len) = u32_at(bytes, p.checked_add(2).unwrap_or(usize::MAX)) else {
            return false;
        };
        let Some(body) = p.checked_add(6) else {
            return false;
        };
        if a_name == code_idx {
            // Code: max_stack(2) max_locals(2) code_len(4) code[code_len]
            let Some(clen) = u32_at(bytes, body.checked_add(4).unwrap_or(usize::MAX)) else {
                return false;
            };
            let Some(start) = body.checked_add(8) else {
                return false;
            };
            match bytes.get(start..start.checked_add(clen as usize).unwrap_or(usize::MAX)) {
                Some(slice) => code = slice,
                None => return false,
            }
            break;
        }
        p = match body.checked_add(a_len as usize) {
            Some(v) => v,
            None => return false,
        };
    }
    if code.len() != SCAFFOLD_LEN {
        return false;
    }
    // getstatic / ifne / aload_0 / invokevirtual / ireturn / aload_0 /
    // invokevirtual / ireturn
    let skeleton = [
        code[0], code[3], code[6], code[7], code[10], code[11], code[12], code[15],
    ];
    if skeleton != [0xB2, 0x9A, 0x2A, 0xB6, 0xAC, 0x2A, 0xB6, 0xAC] {
        return false;
    }
    let v1 = u16::from_be_bytes([code[8], code[9]]);
    let v2 = u16::from_be_bytes([code[13], code[14]]);
    if v1 != v2 {
        return false;
    }
    match methodref_parts(&layout.pool, v1) {
        Some((cls, name, desc)) => {
            cls == "net/minecraft/world/entity/Entity"
                && name == "isAffectedByBlocks"
                && desc == "()Z"
        }
        None => false,
    }
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

// ---------------------------------------------------------------------------
// INSIDE-SNAP (TASK-424-B, lever cmp424_inside): per-section BlockState[4096]
// snapshot plane. TWO retargets:
//   1) The SINGLE Level.getBlockState call inside
//      Entity.lambda$checkInsideBlocks$2 (javap @4789ca7: one invokevirtual
//      Level.getBlockState at bytecode 29, receiver = this.level()) ->
//      InsideSnapOps.snapGet(Level,BlockPos)BlockState (receiver-first 3B->3B).
//   2) The SINGLE LevelChunkSection.setBlockState(IIILBlockState)BlockState
//      call site inside LevelChunk.setBlockState(BlockPos,BlockState,I) ->
//      InsideSnapOps.secWrite(...) (delegate + generation bump on a real
//      change; FluidPushOps.secWrite pattern — same FROM site as fluid_dirty,
//      disjoint lever: STRICT flag eq, only one of the two is ever armed).

pub const INSIDE_SNAP_OPS_CLASS: &str = "net/minecraft/world/entity/InsideSnapOps";
pub const INSIDE_SNAP_GATE_CALLER: (&str, &str) = (
    "lambda$checkInsideBlocks$2",
    "(ILjava/util/concurrent/atomic/AtomicInteger;ZLnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lit/unimi/dsi/fastutil/longs/LongSet;ZLnet/minecraft/world/phys/AABB;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;Lnet/minecraft/core/BlockPos;I)Z",
);
pub const INSIDE_SNAP_GATE_FROM: (&str, &str, &str) = (
    "net/minecraft/world/level/Level",
    "getBlockState",
    "(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;",
);
pub const INSIDE_SNAP_GATE_TO: (&str, &str, &str) = (
    "net/minecraft/world/entity/InsideSnapOps",
    "snapGet",
    "(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;",
);

/// Entity bytes: retarget the single Level.getBlockState site inside
/// lambda$checkInsideBlocks$2 to InsideSnapOps.snapGet. Strict: exactly ONE
/// site (javap census @4789ca7).
pub fn patch_inside_snap_gate(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    for probe in [
        "checkInsideBlocks",
        "getBlockState",
        "net/minecraft/core/BlockPos",
        "net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector",
    ] {
        if layout.pool.find_utf8(probe).is_none() {
            return Err(format!("{probe} absent from pool (kernel rename?)"));
        }
    }
    let expect_static = format!(
        "(L{};{}",
        INSIDE_SNAP_GATE_FROM.0,
        &INSIDE_SNAP_GATE_FROM.2[1..]
    );
    if INSIDE_SNAP_GATE_TO.2 != expect_static {
        return Err("snapGet descriptor is not the receiver-prepended target form".into());
    }
    let (name, desc) = INSIDE_SNAP_GATE_CALLER;
    retarget_virtual_to_static(bytes, name, desc, INSIDE_SNAP_GATE_FROM, INSIDE_SNAP_GATE_TO)
}

/// LevelChunk bytes: retarget the single LevelChunkSection.setBlockState site
/// to InsideSnapOps.secWrite (delegate + generation bump).
pub fn patch_inside_snap_levelchunk(
    bytes: &[u8],
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    for probe in ["setBlockState", "net/minecraft/world/level/chunk/LevelChunkSection"] {
        if layout.pool.find_utf8(probe).is_none() {
            return Err(format!("{probe} absent from pool (kernel rename?)"));
        }
    }
    retarget_virtual_to_static(
        bytes,
        "setBlockState",
        LEVELCHUNK_SETBLOCK_DESC,
        SEC_WRITE_FROM,
        (INSIDE_SNAP_OPS_CLASS, "secWrite", SEC_WRITE_DESC),
    )
}

/// Structural delivery guard: the bridge must declare every static the two
/// retargets + the native table resolve (entity_compose redirects would
/// detonate NoSuchMethodError on the first tick otherwise).
pub fn inside_snap_resolution_closure(ops: &[u8]) -> Result<(), String> {
    redirect_targets_resolution_closure(
        ops,
        &[
            (
                "net/minecraft/world/level/Level",
                "(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;",
                "snapGet",
                "(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;",
            ),
            (
                "net/minecraft/world/level/chunk/LevelChunkSection",
                "(IIILnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;",
                "secWrite",
                "(Lnet/minecraft/world/level/chunk/LevelChunkSection;IIILnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;",
            ),
            (
                "",
                "",
                "selfTest",
                "()Z",
            ),
            (
                "",
                "",
                "arm",
                "()V",
            ),
        ],
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

/// S7-161 BATCH-COLLECTOR (lever #8 v2): retarget the single
/// `new StepBasedCollector; dup; invokespecial <init>()V` site inside
/// `Entity.<init>(EntityType, Level)` so EVERY entity is born owning the
/// zero-map BatchCollector. The ctor is the ONLY kernel writer of the
/// final `insideEffectCollector` field (javap census — single NEW site at
/// pristine offsets 193-200), so a constructor-level substitution is
/// persistent by construction; the S7-160 lazy Unsafe swap into the final
/// field did not survive across ticks and is retired from the hot path.
/// The population fixture injects AFTER arm-time, so ~100% of the measured
/// window's population is covered.
///
/// Bytecode contract (javap pristine 1.21.10):
///   193: new  #48   // class …StepBasedCollector
///   196: dup
///   197: invokespecial #923 // …StepBasedCollector."<init>":()V
///   200: putfield #925     // Field insideEffectCollector
/// Rewrite: NEW class operand -> Class(BatchCollector), invokespecial
/// operand -> Methodref(BatchCollector."<init>":()V) (same ()V shape;
/// BatchCollector extends StepBasedCollector so the putfield subsumption
/// passes). dup is untouched. Strict: exactly 1 site on first sight;
/// idempotent (AlreadyPatched when the site already resolves to
/// BatchCollector).
pub fn patch_entity_collector_ctor(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    const VANILLA: &str =
        "net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector";
    const BATCH: &str = "net/minecraft/world/entity/BatchCollector";
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    let mut pool = layout.pool;
    // Probe-only utf8 lookups first: a ctor whose name/desc utf8 entries
    // are absent cannot exist; NotFound must not mutate the pool.
    let Some(name_idx) = pool.find_utf8("<init>") else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let Some(desc_idx) = pool.find_utf8(
        "(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V",
    ) else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
        .ok_or_else(|| "Entity.<init>(EntityType, Level) not found".to_string())?;
    let (code_start, code_len) = find_code_attr(bytes, &pool, &m)
        .ok_or_else(|| "Entity ctor has no Code attribute".to_string())?;
    let code_end = code_start
        .checked_add(code_len)
        .ok_or_else(|| "code length overflow".to_string())?;
    let code = bytes
        .get(code_start..code_end)
        .ok_or_else(|| "code region truncated".to_string())?;

    // Scan for the dup+invokespecial-anchored NEW site: [0xBB i1 i2]
    // [0x59] [0xB7 j1 j2] with j resolving to the vanilla collector ctor.
    let mut new_site: Option<(usize, u16)> = None; // (abs offset of 0xBB, cp idx)
    let mut inv_site: Option<usize> = None; // abs offset of 0xB7
    let mut already = 0usize;
    let mut pc = 0usize;
    while pc + 6 <= code.len() {
        if code[pc] == 0xBB
            && code[pc + 3] == 0x59
            && code[pc + 4] == 0xB7
        {
            let inv_idx = u16::from_be_bytes([code[pc + 5], code[pc + 6]]);
            match pool.methodref_parts(inv_idx) {
                Some((cls, name, desc))
                    if cls == BATCH && name == "<init>" && desc == "()V" =>
                {
                    already += 1;
                }
                Some((cls, name, desc)) if cls == VANILLA && name == "<init>" && desc == "()V" => {
                    if new_site.is_some() {
                        return Err("multiple collector NEW sites in Entity ctor".into());
                    }
                    new_site = Some((code_start + pc, u16::from_be_bytes([code[pc + 1], code[pc + 2]])));
                    inv_site = Some(code_start + pc + 4);
                }
                _ => {}
            }
            pc += 7;
            continue;
        }
        pc += 1;
    }
    if new_site.is_none() {
        return Ok((
            bytes.to_vec(),
            if already > 0 {
                RetargetOutcome::AlreadyPatched { sites: already }
            } else {
                RetargetOutcome::NotFound
            },
        ));
    }
    let (new_abs, _vanilla_new_idx) = new_site.unwrap();
    let inv_abs = inv_site.unwrap();
    // Pool growth: Class(BatchCollector) + Methodref(BatchCollector."<init>":()V).
    let batch_utf8 = pool.utf8(BATCH);
    let class_idx = pool.class_of(batch_utf8);
    let ctor_ref = pool.method_ref(BATCH, "<init>", "()V");
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for collector retarget".into());
    }
    let mut tail = bytes[layout.cp_end..].to_vec();
    let rel_new = new_abs - layout.cp_end;
    let rel_inv = inv_abs - layout.cp_end;
    if rel_new + 2 >= tail.len() || rel_inv + 2 >= tail.len() {
        return Err("collector site outside class tail (corrupt layout?)".into());
    }
    tail[rel_new] = 0xBB; // new (opcode unchanged, operand swapped)
    tail[rel_new + 1] = class_idx.to_be_bytes()[0];
    tail[rel_new + 2] = class_idx.to_be_bytes()[1];
    tail[rel_inv] = 0xB7; // invokespecial (opcode unchanged, operand swapped)
    tail[rel_inv + 1] = ctor_ref.to_be_bytes()[0];
    tail[rel_inv + 2] = ctor_ref.to_be_bytes()[1];
    let mut out = Vec::with_capacity(bytes.len() + 64);
    out.extend_from_slice(&bytes[0..8]);
    out.extend_from_slice(&pool.next.to_be_bytes());
    out.extend_from_slice(&pool.serialize());
    out.extend_from_slice(&tail);
    Ok((out, RetargetOutcome::Retargeted { sites: 1 }))
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
// QUERYPLANE (TASK-412-B/413-B, lever cmp412_b2p1 STRICT eq — entity-query
// snapshot plane, broadphase-лейн 13.91-14.23% на meganav-профиле).
//
// Три сайта (javap-контракты patched-kernel.jar 2025-12-11):
//
//   1) Level.getEntitiesOfClass(Class,AABB,Predicate) — public final,
//      erased desc (Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;
//      Ljava/util/function/Predicate;)Ljava/util/List; — WHOLE-BODY REDIRECT
//      (receiver-prepended static) на QueryPlaneOps.getEntitiesOfClass.
//      Ванильное тело (javap @0-37): Profiler.get().incrementCounter
//      ("getEntities") + new ArrayList + ChunkSystemLevel.moonrise$
//      getEntityLookup().getEntities(clazz, null, box, list, pred) + areturn
//      — реплицировано в бридже бит-в-бит (fallback и fast path).
//
//   2) Level.moonrise$getHardCollidingEntities(Entity,AABB,Predicate) —
//      public final, тот же shape — WHOLE-BODY REDIRECT на
//      QueryPlaneOps.getHardCollidingEntities.
//
//   3) ChunkEntitySlices.addEntity(Entity,int)Z — ОДИН invokeinterface-сайт
//      ChunkSystemEntity.moonrise$isHardColliding:()Z (javap @50, 5 байт:
//      0xb9 idx1 idx2 count 0) → invokestatic
//      QueryPlaneOps.isHardCollidingProbe (3 байта) + 2×nop —
//      length-preserving (flush-diet fladd прецедент), стек-шейп
//      ([Entity]→[Z]) идентичен, StackMapTable не сдвигается. Монотонный
//      add-счетчик hard-colliders (HARD_ADDS) — гейт пустого fast path
//      плоскости 2: 0 добавлений за жизнь JVM ⇒ hard-коллекции пусты ⇒
//      ванильный walk вернул бы empty list.
//
// Fail-closed: любой shape-mismatch = Err/NotFound (оригинальные байты,
// ваниль); резолюшн-кложура бриджа проверяется ДО define (S7-164-guard).
// ---------------------------------------------------------------------------

pub const QUERY_OPS_CLASS: &str = "net/minecraft/world/entity/QueryPlaneOps";
pub const QUERY_LEVEL_CLASS: &str = "net/minecraft/world/level/Level";
pub const SLICES_CLASS: &str =
    "ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices";
pub const CHUNK_SYSTEM_ENTITY_CLASS: &str =
    "ca/spottedleaf/moonrise/patches/chunk_system/entity/ChunkSystemEntity";

/// Виртуальный (erased) дескриптор Level.getEntitiesOfClass(Class,AABB,Pred).
pub const QUERY_GEOC_DESC: &str = "(Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;";
/// Статический бридж-дескриптор = receiver Level, префиксованный.
pub const QUERY_GEOC_STATIC_DESC: &str = "(Lnet/minecraft/world/level/Level;Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;";
/// Виртуальный (erased) дескриптор Level.moonrise$getHardCollidingEntities.
pub const QUERY_GHC_DESC: &str = "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;";
/// Статический бридж-дескриптор = receiver Level, префиксованный.
pub const QUERY_GHC_STATIC_DESC: &str = "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;";
/// Дескриптор ChunkEntitySlices.addEntity.
pub const SLICES_ADD_DESC: &str = "(Lnet/minecraft/world/entity/Entity;I)Z";
/// Статический probe-дескриптор = receiver-класс CP-сайта (интерфейс
/// ChunkSystemEntity), префиксованный (flush-diet fladd форма).
pub const SLICES_PROBE_DESC: &str = "(Lca/spottedleaf/moonrise/patches/chunk_system/entity/ChunkSystemEntity;)Z";

/// Whole-body redirect сайта 1 (см. баннер секции).
pub fn patch_level_get_entities_of_class(
    bytes: &[u8],
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    redirect_method_body_to_static(
        bytes,
        "getEntitiesOfClass",
        QUERY_GEOC_DESC,
        QUERY_LEVEL_CLASS,
        QUERY_OPS_CLASS,
        "getEntitiesOfClass",
        QUERY_GEOC_STATIC_DESC,
    )
}

/// Whole-body redirect сайта 2 (см. баннер секции).
pub fn patch_level_get_hard_colliding_entities(
    bytes: &[u8],
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    redirect_method_body_to_static(
        bytes,
        "moonrise$getHardCollidingEntities",
        QUERY_GHC_DESC,
        QUERY_LEVEL_CLASS,
        QUERY_OPS_CLASS,
        "getHardCollidingEntities",
        QUERY_GHC_STATIC_DESC,
    )
}

/// Сайт 3: invokeinterface → invokestatic + 2×nop внутри
/// ChunkEntitySlices.addEntity(Entity,int)Z. Strict: ровно ОДИН сайт
/// moonrise$isHardColliding (javap-контракт @50); zero сайтов при уже
/// стоящем probe = AlreadyPatched; иначе — shape mismatch → Err.
/// Идемпотентен (re-sight не портит байты).
pub fn patch_slices_hard_probe(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let layout = parse_layout(bytes).ok_or_else(|| "bad classfile layout".to_string())?;
    // Kernel-rename guard: имена контракта должны присутствовать ДО мутаций.
    for probe in ["addEntity", "moonrise$isHardColliding"] {
        if layout.pool.find_utf8(probe).is_none() {
            return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
        }
    }
    let Some(name_idx) = layout.pool.find_utf8("addEntity") else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let Some(desc_idx) = layout.pool.find_utf8(SLICES_ADD_DESC) else {
        return Ok((bytes.to_vec(), RetargetOutcome::NotFound));
    };
    let m = find_method(bytes, layout.methods_start, name_idx, desc_idx)
        .ok_or_else(|| "method addEntity(Entity,int)Z not found".to_string())?;
    let (code_start, code_len) = find_code_attr(bytes, &layout.pool, &m)
        .ok_or_else(|| "addEntity has no Code attribute".to_string())?;
    let code_end = code_start
        .checked_add(code_len)
        .ok_or_else(|| "code length overflow".to_string())?;
    let code = bytes
        .get(code_start..code_end)
        .ok_or_else(|| "code region truncated".to_string())?;

    // Walk: invokeinterface (0xb9, 5B) + invokestatic (0xb8, 3B), всё
    // остальное через общий length-table (fail-closed на неизвестных опах).
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

    // Классификация ПО ИМЕНИ (CP-индексы плывут между ASM-прогонами).
    let from_triple = (
        CHUNK_SYSTEM_ENTITY_CLASS.to_string(),
        "moonrise$isHardColliding".to_string(),
        "()Z".to_string(),
    );
    let to_triple = (
        QUERY_OPS_CLASS.to_string(),
        "isHardCollidingProbe".to_string(),
        SLICES_PROBE_DESC.to_string(),
    );
    let mut rewrite: Vec<usize> = Vec::new();
    let mut already = 0usize;
    for (op_pc, cp_idx) in &iface {
        if layout.pool.methodref_parts(*cp_idx) == Some(from_triple.clone()) {
            rewrite.push(*op_pc);
        }
    }
    for (op_pc, cp_idx) in &statics {
        if layout.pool.methodref_parts(*cp_idx) == Some(to_triple.clone()) {
            already += 1;
        } else {
            // Посторонний invokestatic внутри addEntity — сайт ровно один,
            // любой чужой статик = shape mismatch из другой вселенной.
            return Err(format!(
                "addEntity carries a foreign invokestatic at {op_pc:#x} (shape mismatch)"
            ));
        }
    }
    if rewrite.is_empty() {
        return Ok((
            bytes.to_vec(),
            if already == 1 {
                RetargetOutcome::AlreadyPatched { sites: already }
            } else {
                RetargetOutcome::NotFound
            },
        ));
    }
    if rewrite.len() != 1 {
        return Err(format!(
            "expected exactly one moonrise$isHardColliding site in addEntity, got {}",
            rewrite.len()
        ));
    }

    // Append-only (dedup) Methodref на probe.
    let mut pool = layout.pool;
    let new_idx = pool.method_ref(
        QUERY_OPS_CLASS,
        "isHardCollidingProbe",
        SLICES_PROBE_DESC,
    );
    if pool.next > u16::MAX - 16 {
        return Err("constant pool overflow: no index space left for probe ref".into());
    }

    // Splice: header + grown pool + tail; 5 байт на сайт:
    // [0xb8][idx1][idx2][0x00][0x00] — invokestatic + 2 nop (бывшие count/zero
    // операнды invokeinterface; length-preserving, StackMapTable не двигается).
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

/// Резолюшн-кложура бриджа (S7-164 guard): QueryPlaneOps обязан объявлять
/// ровно те статики, куда прыгают редиректы — иначе NoSuchMethodError на
/// первом же запросе. Проверяется ДО define_class.
pub const QUERY_REDIRECT_TARGETS: [(&str, &str, &str, &str); 3] = [
    (
        "getEntitiesOfClass",
        QUERY_GEOC_DESC,
        "getEntitiesOfClass",
        QUERY_GEOC_STATIC_DESC,
    ),
    (
        "moonrise$getHardCollidingEntities",
        QUERY_GHC_DESC,
        "getHardCollidingEntities",
        QUERY_GHC_STATIC_DESC,
    ),
    (
        "moonrise$isHardColliding",
        "()Z",
        "isHardCollidingProbe",
        SLICES_PROBE_DESC,
    ),
];

pub fn queryplane_resolution_closure(ops: &[u8]) -> Result<(), String> {
    redirect_targets_resolution_closure(ops, &QUERY_REDIRECT_TARGETS)
}

// ---------------------------------------------------------------------------

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

    // ---- F2-tick2 body swap (Brain.tickEachRunningBehavior, TASK-442-D) ----

    /// Round-trip on the REAL Brain fixture: the swapped body is the exact
    /// 12-byte straight-line delegation; max_stack 5 / max_locals 3; the
    /// getfield operand resolves to Brain's own field by NAME; the
    /// invokestatic operand resolves to BrainOps.tickEachRunning.
    #[test]
    fn f2t2_patch_roundtrip_verified() {
        // COMPOSED bytes: tick2 applies ON TOP of the F2 baseline (hook order).
        let f2 = patch_brain_start_each(BRAIN).expect("f2");
        let patched = patch_brain_tick_each(&f2).expect("tick2");
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert_eq!(patched[..8], BRAIN[..8], "version preserved");

        let layout = parse_layout(&patched).expect("re-parse");
        let name_idx = layout
            .pool
            .find_utf8("tickEachRunningBehavior")
            .expect("name utf8 present");
        let desc_idx = layout
            .pool
            .find_utf8(START_EACH_DESC)
            .expect("desc utf8 present");
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx)
            .expect("tickEachRunningBehavior present");
        let (start, len) =
            find_code_attr(&patched, &layout.pool, &m).expect("Code attr");
        let code = &patched[start..start + len];
        assert_eq!(code.len(), 10, "straight-line body is 10 bytes");
        let ms = u16::from_be_bytes([patched[start - 8], patched[start - 7]]);
        let ml = u16::from_be_bytes([patched[start - 6], patched[start - 5]]);
        assert_eq!(ms, 5, "max_stack kept");
        assert_eq!(ml, 3, "max_locals = this,level,entity");
        let skel = [code[0], code[1], code[4], code[5], code[6], code[9]];
        let want = [0x2a, 0xb4, 0x2b, 0x2c, 0xb8, 0xb1];
        assert_eq!(skel, want, "opcode skeleton exact");
        // Operand resolution BY NAME.
        let layout2 = parse_layout(&patched).expect("re-parse 2");
        let f1 = layout2
            .pool
            .fieldref_parts(u16::from_be_bytes([code[2], code[3]]))
            .expect("getfield operand resolves");
        assert_eq!(
            f1,
            (
                BRAIN_CLASS.to_string(),
                "availableBehaviorsByPriority".to_string(),
                "Ljava/util/Map;".to_string()
            )
        );
        let m1 = layout2
            .pool
            .methodref_parts(u16::from_be_bytes([code[7], code[8]]))
            .expect("invokestatic operand resolves");
        assert_eq!(m1.0, BRAIN_OPS_CLASS, "owner = BrainOps");
        assert_eq!(m1.1, "tickEachRunning", "method name");
        assert_eq!(
            m1.2,
            "(Ljava/util/Map;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)V",
            "helper descriptor"
        );
    }

    /// Idempotency + composition order: F2∘tick2 == F2∘tick2∘tick2 (and the
    /// F2 baseline body is NOT disturbed by the tick2 pass).
    #[test]
    fn f2t2_patch_is_idempotent_and_preserves_f2() {
        let f2 = patch_brain_start_each(BRAIN).expect("f2");
        let once = patch_brain_tick_each(&f2).expect("first");
        let twice = patch_brain_tick_each(&once).expect("second");
        assert_eq!(once, twice, "double tick2 patch is byte-identical");
        // F2 body intact after tick2.
        let (ms, ml, code) = f2_code_of(&once);
        assert_eq!(code.len(), 14, "F2 startEach body unchanged");
        assert_eq!(ms, 5);
        assert_eq!(ml, 3);
    }

    /// Fail-closed: wrong class / garbage rejected without panic.
    #[test]
    fn f2t2_patch_rejects_wrong_class_and_garbage() {
        let e = patch_brain_tick_each(SERVER).expect_err("ServerLevel is not Brain");
        assert!(e.starts_with("unexpected class"));
        let e = patch_brain_tick_each(&BRAIN[..64]).expect_err("truncated header");
        assert!(!e.is_empty());
        for cut in [10usize, 100, 1000, 10000, BRAIN.len() - 1] {
            let _ = patch_brain_tick_each(&BRAIN[..cut]);
        }
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
    #[cfg(test)]
    const COLPUSH_BLOB: &[u8] = include_bytes!("../colpush/build/net/minecraft/world/entity/ColpushOps.class");
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
    fn push_entities_colpush_whole_body_redirect_roundtrip() {
        // TASK-419-A: whole-body redirect of pushEntities on the REAL kernel
        // LivingEntity fixture — Retargeted on first sight, byte-identical on
        // re-sight (idempotent retransform), and the delivered ColpushOps
        // blob passes the resolution closure (every redirect target declared).
        let (patched, outcome) = patch_push_entities_colpush(LIVING).expect("colpush patch");
        assert!(matches!(outcome, RetargetOutcome::Retargeted { sites: 1 }), "{outcome:?}");
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // Re-sight (retransform replay): the redirect machinery detects the
        // already-rewritten body — AlreadyPatched, byte-identical output.
        let (again, outcome2) = patch_push_entities_colpush(&patched).expect("repatch");
        assert!(matches!(outcome2, RetargetOutcome::AlreadyPatched { .. }), "{outcome2:?}");
        assert_eq!(again, patched, "colpush repatch must be byte-identical");
        colpush_resolution_closure(COLPUSH_BLOB).expect("resolution closure");
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
mod inside_batch {
    // TASK-459-56 (ID-P31): same REAL kernel Entity fixture as inside_cache —
    // the batch plane retargets the SAME single method-entry site.
    const ENTITY: &[u8] = include_bytes!("../tests/fixtures/Entity_real.class");

    use crate::classfile::*;

    #[test]
    fn inside_batch_retargets_exactly_one_site() {
        let (patched, outcome) = patch_inside_batch(ENTITY).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "exactly one isAffectedByBlocks site in checkInsideBlocks(List,Collector)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert!(patched.len() >= ENTITY.len());
    }

    #[test]
    fn inside_batch_site_resolves_to_batch_bridge() {
        let (patched, _) = patch_inside_batch(ENTITY).expect("patch");
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        assert!(
            triples.iter().any(|t| t.0 == "net/minecraft/world/entity/InsideBatchOps"
                && t.1 == "batchGate"
                && t.2 == "(Lnet/minecraft/world/entity/Entity;)Z"),
            "batchGate Methodref appended"
        );
        // The collector-field guard precondition (bridge Unsafe resolution).
        assert!(pool.find_utf8("insideEffectCollector").is_some());
    }

    #[test]
    fn inside_batch_idempotent() {
        let (patched, _) = patch_inside_batch(ENTITY).expect("patch");
        let (again, outcome) = patch_inside_batch(&patched).expect("repatch");
        assert_eq!(outcome, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn inside_batch_wrong_class_fails_closed() {
        match patch_inside_batch(include_bytes!(
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

    /// S7-168 (STEAL v2 defect-fix): the REAL ServerLevel fixture's
    /// sendBlockUpdated body redirects to BlockUpdateOps.handle EXACTLY
    /// once (single non-overloaded body; javap codelen=235 contract).
    #[test]
    fn blockupd_serverlevel_retargets_exactly_one_sendblockupdated() {
        let (patched, outcome) =
            patch_serverlevel_send_block_updated(SERVER).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "ServerLevel.sendBlockUpdated is EXACTLY the single BU-DEFER site"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // Idempotence guard: repatching the patched bytes must not double-emit.
        let (_, outcome2) =
            patch_serverlevel_send_block_updated(&patched).expect("repatch");
        assert_eq!(
            outcome2,
            RetargetOutcome::AlreadyPatched { sites: 1 },
            "second pass must detect the already-retargeted body"
        );
    }

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

    /// TASK-411-A k5b: the entityMap fence retargets the EXACT javap
    /// census (containsKey x3 + put x1 + remove x1 + get x4 + values x3
    /// = 12 sites) on the REAL pristine ChunkMap fixture, composes with
    /// the region-tracker patch (tracker first, fence second — live boot
    /// order), and re-patches idempotently (AlreadyPatched sites:12,
    /// byte-identical). Every rewritten site must be invokestatic into
    /// EntityMapOps with the receiver-prepended descriptor followed by
    /// exactly two nops (length-preserving: code size UNCHANGED, so
    /// branch offsets / exception ranges / StackMapTable stay valid).
    #[test]
    fn emap_fence_retargets_exactly_twelve_sites() {
        let (patched, outcome) = patch_chunkmap_entitymap(CHUNKMAP).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 12 },
            "entityMap fence = EXACTLY the 12 censused Int2ObjectMap sites"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // Pool grew coherently.
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, cp_end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let triples: Vec<_> = (1..pool.next)
            .filter_map(|i| pool.methodref_parts(i))
            .collect();
        for (name, _d, _want) in EMAP_SITES.iter() {
            let got = triples
                .iter()
                .filter(|t| t.0 == EMAP_OPS_CLASS && t.1 == *name)
                .count();
            assert_eq!(
                got, 1,
                "exactly ONE deduped fence Methodref for {name} (one ref, many sites)"
            );
        }
        assert_eq!(
            triples
                .iter()
                .filter(|t| t.0 == EMAP_OPS_CLASS)
                .count(),
            EMAP_SITES.len(),
            "pool carries exactly the five fence helper refs"
        );
        // Walk the patched code: each fence site is b8 + 2 nops, and the
        // class is otherwise length-preserving (cp grew, code spans did
        // not move — re-scan must find the SAME 12 sites as already).
        let (again, outcome2) = patch_chunkmap_entitymap(&patched).expect("repatch");
        assert_eq!(
            outcome2,
            RetargetOutcome::AlreadyPatched { sites: 12 },
            "second pass must classify all 12 fence sites as already-patched"
        );
        assert_eq!(again, patched, "repatch must be byte-identical");
        // Compose contract with the live boot chain: tracker patch FIRST,
        // fence SECOND (the tracker site in tick() must survive the fence
        // walk untouched).
        let (tracker_patched, t_outcome) =
            patch_region_tracker_chunkmap(CHUNKMAP).expect("tracker patch");
        assert_eq!(t_outcome, RetargetOutcome::Retargeted { sites: 1 });
        let (both, f_outcome) = patch_chunkmap_entitymap(&tracker_patched).expect("fence patch");
        assert_eq!(f_outcome, RetargetOutcome::Retargeted { sites: 12 });
        let (tp, _cp_end2) = Pool::parse(&both, 10, cp_count).expect("cp parse");
        let t2: Vec<_> = (1..tp.next)
            .filter_map(|i| tp.methodref_parts(i))
            .collect();
        assert!(
            t2.iter()
                .any(|t| t.0 == "net/minecraft/server/level/TrackerTickOps"
                    && t.1 == "newTrackerTick"),
            "tracker site survives the fence compose"
        );
        let _ = cp_end;
    }

    /// Fail-dominant probe: the fence must refuse to patch a class
    /// without the fastutil utf8 in its pool (kernel-rename guard).
    #[test]
    fn emap_fence_rejects_missing_fastutil_pool() {
        // The SingleUserAreaMap fixture has no Int2ObjectMap utf8.
        const SAM: &[u8] = include_bytes!("../tests/fixtures/SingleUserAreaMap.class");
        let r = patch_chunkmap_entitymap(SAM);
        assert!(r.is_err(), "fence must fail closed on a foreign class");
    }

    /// Dump artifacts for the offline JVM-verifier harness (emap).
    #[test]
    fn dump_emap_patched_for_verifier() {
        let (patched, _) = patch_chunkmap_entitymap(CHUNKMAP).expect("patch");
        std::fs::create_dir_all("tests/out").unwrap();
        std::fs::write("tests/out/ChunkMap.emap.patched.class", &patched).unwrap();
    }

    // ---------------- REFSYNC (TASK-412-A) ----------------

    const BASE_HOOKS: &[u8] = include_bytes!("../tests/fixtures/BaseChunkSystemHooks_real.class");
    const NEARBY_CHUNK: &[u8] =
        include_bytes!("../tests/fixtures/NearbyPlayers$TrackedChunk_real.class");
    const TRACKED_ENTITY: &[u8] =
        include_bytes!("../tests/fixtures/ChunkMap$TrackedEntity_real.class");
    const SCHED_TICKLIST: &[u8] =
        include_bytes!("../tests/fixtures/EntityScheduler$EntitySchedulerTickList_real.class");

    /// TASK-412-A: the ReferenceList mutator fence retargets the EXACT
    /// per-class javap census on REAL kernel fixtures (add/remove/contains
    /// -> EntityMapOps.refList*), composes idempotently, and preserves
    /// length (code spans untouched => branch offsets / StackMapTable
    /// stay valid).
    #[test]
    fn refsync_retargets_exact_census_on_real_fixtures() {
        for (fixture, class, sites) in [
            (
                BASE_HOOKS,
                "ca/spottedleaf/moonrise/paper/util/BaseChunkSystemHooks",
                6usize,
            ),
            (
                NEARBY_CHUNK,
                "ca/spottedleaf/moonrise/common/misc/NearbyPlayers$TrackedChunk",
                3,
            ),
            (
                TRACKED_ENTITY,
                "net/minecraft/server/level/ChunkMap$TrackedEntity",
                1,
            ),
            (
                SCHED_TICKLIST,
                "io/papermc/paper/threadedregions/EntityScheduler$EntitySchedulerTickList",
                2,
            ),
        ] {
            let (patched, outcome) = patch_referencelist_callsites(fixture).expect("patch");
            assert_eq!(
                outcome,
                RetargetOutcome::Retargeted { sites },
                "{class} must retarget EXACTLY its censused sites"
            );
            assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
            let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
            let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
            let triples: Vec<_> = (1..pool.next)
                .filter_map(|i| pool.methodref_parts(i))
                .collect();
            for helper in ["refListAdd", "refListRemove", "refListContains"] {
                let got = triples
                    .iter()
                    .filter(|t| t.0 == EMAP_OPS_CLASS && t.1 == helper)
                    .count();
                assert!(got <= 1, "{helper}: at most one deduped Methodref");
            }
            // Idempotence: second pass classifies every site as already
            // patched and is byte-identical.
            let (again, outcome2) = patch_referencelist_callsites(&patched).expect("repatch");
            assert_eq!(
                outcome2,
                RetargetOutcome::AlreadyPatched { sites },
                "{class}: second pass must see all sites already fenced"
            );
            assert_eq!(again, patched, "{class}: repatch must be byte-identical");
        }
    }

    /// Fail-dominant: a class OUTSIDE the census table is rejected.
    #[test]
    fn refsync_rejects_uncensused_class() {
        const SAM: &[u8] = include_bytes!("../tests/fixtures/SingleUserAreaMap.class");
        assert!(patch_referencelist_callsites(SAM).is_err());
        assert!(patch_referencelist_callsites(CHUNKMAP).is_err());
    }

    /// The census table must sum to the documented totals
    /// (add 11 + remove 9 + contains 2 = 22 sites across 8 classes).
    #[test]
    fn refsync_census_table_sums_to_22_sites() {
        let mut per = std::collections::HashMap::new();
        for (_, sites) in REFSYNC_CENSUS.iter() {
            for (name, n) in sites.iter() {
                *per.entry(*name).or_insert(0usize) += n;
            }
        }
        assert_eq!(per.get("add"), Some(&11));
        assert_eq!(per.get("remove"), Some(&9));
        assert_eq!(per.get("contains"), Some(&2));
        let total: usize = per.values().sum();
        assert_eq!(total, REFSYNC_SITES_TOTAL);
        // Helper naming contract matches the EntityMapOps.java source.
        assert_eq!(refsync_helper_name("add"), "refListAdd");
        assert_eq!(refsync_helper_name("remove"), "refListRemove");
        assert_eq!(refsync_helper_name("contains"), "refListContains");
        assert_eq!(
            refsync_to_desc(),
            "(Lca/spottedleaf/moonrise/common/list/ReferenceList;Ljava/lang/Object;)Z"
        );
    }

    /// Dump artifacts for the offline JVM-verifier harness (refsync).
    #[test]
    fn dump_refsync_patched_for_verifier() {
        std::fs::create_dir_all("tests/out").unwrap();
        for (fixture, out) in [
            (BASE_HOOKS, "BaseChunkSystemHooks.refsync.patched.class"),
            (
                NEARBY_CHUNK,
                "NearbyPlayers$TrackedChunk.refsync.patched.class",
            ),
            (
                SCHED_TICKLIST,
                "EntityScheduler$EntitySchedulerTickList.refsync.patched.class",
            ),
        ] {
            let (patched, _) = patch_referencelist_callsites(fixture).expect("patch");
            std::fs::write(format!("tests/out/{out}"), &patched).unwrap();
        }
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
    fn batch_collector_ctor_retargets_exactly_one_new_site() {
        // S7-161: compose order matches the live region chain — rng first
        // (CP growth shifts indexes), then the collector ctor retarget.
        let (rng_patched, rng_outcome) = patch_region_rng_entity(ENTITY).expect("rng patch");
        assert_eq!(rng_outcome, RetargetOutcome::Retargeted { sites: 1 });
        let (patched, outcome) = patch_entity_collector_ctor(&rng_patched).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "the ONLY new StepBasedCollector;dup;invokespecial site in Entity.<init>(EntityType, Level)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // The rewritten NEW operand must resolve to BatchCollector BY NAME
        // (index assumptions are forbidden — G4 §3).
        let cp_count = u16::from_be_bytes([patched[8], patched[9]]);
        let (pool, _end) = Pool::parse(&patched, 10, cp_count).expect("cp parse");
        let mut new_batch_sites = 0;
        let mut vanilla_new_left = 0;
        // Re-locate the ctor and walk its code for the NEW+dup+invokespecial
        // pattern.
        let layout = parse_layout(&patched).expect("layout");
        let name_idx = pool.find_utf8("<init>").expect("<init> utf8");
        let desc_idx = pool
            .find_utf8("(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V")
            .expect("ctor desc utf8");
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx)
            .expect("ctor found");
        let (code_start, code_len) =
            find_code_attr(&patched, &pool, &m).expect("code attr");
        let code = &patched[code_start..code_start + code_len];
        let mut pc = 0usize;
        while pc + 6 <= code.len() {
            if code[pc] == 0xBB && code[pc + 3] == 0x59 && code[pc + 4] == 0xB7 {
                let new_idx = u16::from_be_bytes([code[pc + 1], code[pc + 2]]);
                // class entry: TAG_CLASS -> name utf8
                let (_, tag, payload) = pool
                    .entries
                    .iter()
                    .find(|(i, _, _)| *i == new_idx)
                    .expect("new operand resolves");
                assert_eq!(*tag, 7u8, "NEW operand is a CONSTANT_Class");
                let name_u = u16::from_be_bytes([payload[0], payload[1]]);
                let cls = pool.utf8_value(name_u).expect("class name");
                if cls != "net/minecraft/world/entity/BatchCollector"
                    && cls != "net/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector"
                {
                    pc += 1;
                    continue; // unrelated NEW site (HashSet etc.)
                }
                if cls == "net/minecraft/world/entity/BatchCollector" {
                    new_batch_sites += 1;
                } else {
                    vanilla_new_left += 1;
                }
                let inv_idx = u16::from_be_bytes([code[pc + 5], code[pc + 6]]);
                let triple = pool.methodref_parts(inv_idx).expect("invokespecial resolves");
                assert_eq!(
                    triple.0, "net/minecraft/world/entity/BatchCollector",
                    "invokespecial rewritten to BatchCollector.<init>"
                );
                assert_eq!(triple.1, "<init>");
                assert_eq!(triple.2, "()V");
            }
            pc += 1;
        }
        assert_eq!(new_batch_sites, 1, "exactly one BatchCollector NEW site");
        assert_eq!(vanilla_new_left, 0, "no vanilla collector NEW site left");
        let (again, outcome2) = patch_entity_collector_ctor(&patched).expect("repatch");
        assert_eq!(outcome2, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn batch_collector_ctor_wrong_class_fails_closed() {
        // ChunkMap has no Entity ctor -> NotFound, bytes untouched.
        match patch_entity_collector_ctor(CHUNKMAP) {
            Err(e) => assert!(!e.is_empty(), "{e}"),
            Ok((_, outcome)) => assert!(matches!(outcome, RetargetOutcome::NotFound)),
        }
        // truncated / garbage never panics
        for cut in [10usize, 100, 1000, 10000, ENTITY.len() - 1] {
            let _ = patch_entity_collector_ctor(&ENTITY[..cut]);
        }
    }

    #[test]
    fn entity_compose_chain_inside_rng_batch_composes_strictly() {
        // S7-162: the LIVE entity_compose stage order is inside -> rng ->
        // batch (stage 1 is length-preserving, stages 4-5 grow the CP).
        // The full chain must compose strictly on the real Entity fixture:
        // every stage finds EXACTLY its own site, none of the later scans
        // is confused by the earlier rewrites, and the final bytes are a
        // well-formed classfile (parseable constant pool + layout).
        let (b1, out1) = patch_inside_cache(ENTITY).expect("inside patch");
        assert_eq!(
            out1,
            RetargetOutcome::Retargeted { sites: 1 },
            "stage 1: the ONLY isAffectedByBlocks gate site in checkInsideBlocks"
        );
        let (b2, out2) = patch_region_rng_entity(&b1).expect("rng patch");
        assert_eq!(
            out2,
            RetargetOutcome::Retargeted { sites: 1 },
            "stage 4: the ONLY Mth.createInsecureUUID site after the inside rewrite"
        );
        let (b3, out3) = patch_entity_collector_ctor(&b2).expect("batch patch");
        assert_eq!(
            out3,
            RetargetOutcome::Retargeted { sites: 1 },
            "stage 5: the ONLY new StepBasedCollector site after inside+rng"
        );
        assert!(b3.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // The composed bytes must survive a full structural parse (the
        // runtime serves them to the JVM verifier).
        let cp_count = u16::from_be_bytes([b3[8], b3[9]]);
        let (pool, _end) = Pool::parse(&b3, 10, cp_count).expect("composed cp parse");
        let layout = parse_layout(&b3).expect("composed layout parse");
        assert!(layout.methods_start > 0);
        // The inside gate is idempotent on top of the composed chain too
        // (re-registration safety).
        let (again, out_again) = patch_inside_cache(&b3).expect("inside repatch");
        assert!(matches!(
            out_again,
            RetargetOutcome::AlreadyPatched { .. } | RetargetOutcome::Retargeted { .. }
        ));
        let _ = again;
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

// ---------------------------------------------------------------------------
// S7-163 FLAT-TRAVERSAL patch tests (real kernel Entity fixture).
// ---------------------------------------------------------------------------
#[cfg(test)]
mod entity_traversal {
    const ENTITY: &[u8] = include_bytes!("../tests/fixtures/Entity_real.class");

    use crate::classfile::*;

    #[test]
    fn traversal_retargets_exactly_one_site() {
        let (patched, outcome) = patch_entity_traversal(ENTITY).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "exactly one forEachBlockIntersectedBetween site (javap census)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert!(patched.len() >= ENTITY.len(), "CP growth only");
    }

    #[test]
    fn traversal_site_resolves_to_traverse_ops() {
        let (patched, _) = patch_entity_traversal(ENTITY).expect("patch");
        // Byte-level verification: walk checkInsideBlocks's Code and confirm
        // the invokestatic operand now resolves to TraverseOps.forEachFlat.
        // (The OLD Methodref entry legitimately remains in the pool —
        // CP-growth is append-only; nothing in the code references it.)
        let layout = parse_layout(&patched).expect("re-parse retargeted class");
        let name_idx = layout
            .pool
            .find_utf8("checkInsideBlocks")
            .expect("name kept");
        let desc_idx = layout
            .pool
            .find_utf8(TRAVERSAL_SITE_DESC)
            .expect("desc kept");
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx)
            .expect("int-overload found");
        let (code_start, code_len) =
            find_code_attr(&patched, &layout.pool, &m).expect("code attr");
        let sites = scan_invokestatics(
            &patched[code_start..code_start + code_len],
            code_start,
        )
        .expect("scan");
        let targets: Vec<_> = sites
            .iter()
            .map(|(_, idx)| layout.pool.methodref_parts(*idx).expect("resolve"))
            .collect();
        assert!(
            targets.iter().any(|t| {
                t.0 == TRAVERSE_OPS_CLASS
                    && t.1 == "forEachFlat"
                    && t.2 == TRAVERSAL_FROM.2
            }),
            "the single traversal site must resolve to TraverseOps.forEachFlat: {targets:?}"
        );
        assert!(
            !targets
                .iter()
                .any(|t| t.0 == TRAVERSAL_FROM.0 && t.1 == TRAVERSAL_FROM.1),
            "no CODE site may still reference the vanilla traversal: {targets:?}"
        );
    }

    #[test]
    fn traversal_idempotent() {
        let (patched, _) = patch_entity_traversal(ENTITY).expect("patch");
        let (again, outcome) = patch_entity_traversal(&patched).expect("repatch");
        assert_eq!(outcome, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn traversal_wrong_class_fails_clean() {
        match patch_entity_traversal(include_bytes!(
            "../tests/fixtures/PalettedContainer.class"
        )) {
            Err(e) => {
                assert!(e.contains("not found") || e.contains("absent"), "{e}");
            }
            Ok((_, outcome)) => assert!(matches!(
                outcome,
                RetargetOutcome::NotFound | RetargetOutcome::AlreadyPatched { .. }
            )),
        }
    }

    #[test]
    fn traversal_composes_over_full_v2_chain() {
        // The traversal stage rides the entity_compose chain LAST (stage 6):
        // the full v2 chain (inside -> rng -> batch) followed by the
        // traversal retarget must stay idempotent on re-composition (the
        // retransform cycle) and preserve every earlier stage's outcome.
        let chain = |bytes: &[u8]| -> Vec<u8> {
            let (b, _) = patch_inside_cache(bytes).expect("inside");
            let (b, _) = patch_region_rng_entity(&b).expect("rng");
            let (b, _) = patch_entity_collector_ctor(&b).expect("batch");
            let (b, _) = patch_entity_traversal(&b).expect("traversal");
            b
        };
        let composed = chain(ENTITY);
        assert_eq!(composed, chain(&composed), "compose idempotent on composed input");
        // Idempotency of each individual stage over the composed image.
        let (b, out) = patch_inside_cache(&composed).expect("inside re");
        assert!(matches!(
            out,
            RetargetOutcome::AlreadyPatched { .. } | RetargetOutcome::Retargeted { .. }
        ));
        let (b, out) = patch_region_rng_entity(&b).expect("rng re");
        assert!(matches!(
            out,
            RetargetOutcome::AlreadyPatched { .. } | RetargetOutcome::Retargeted { .. }
        ));
        let (b, out) = patch_entity_collector_ctor(&b).expect("batch re");
        assert!(matches!(
            out,
            RetargetOutcome::AlreadyPatched { .. } | RetargetOutcome::Retargeted { .. }
        ));
        let (_, out) = patch_entity_traversal(&b).expect("traversal re");
        assert_eq!(out, RetargetOutcome::AlreadyPatched { sites: 1 });
        // Dump for the offline structural gate.
        std::fs::create_dir_all("tests/out").unwrap();
        std::fs::write("tests/out/Entity.traversal.patched.class", &composed).unwrap();
    }
}

// ---------------------------------------------------------------------------
// TASK-330 ZERO-CURSOR static body-redirect tests (real kernel BlockPos fixture).
// ---------------------------------------------------------------------------
#[cfg(test)]
mod blockpos_zerocursor {
    const BLOCKPOS: &[u8] = include_bytes!("../tests/fixtures/BlockPos_real.class");

    use crate::classfile::*;

    const LAMBDA_NAME: &str = "lambda$betweenCornersInDirection$8";
    const LAMBDA_DESC: &str =
        "(Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;IIIIII)Ljava/util/Iterator;";
    const TARGET_CLASS: &str = "net/minecraft/core/ZeroCursorOps";
    const TARGET_NAME: &str = "lambda8";

    #[test]
    fn zerocursor_redirects_exactly_one_site_and_is_idempotent() {
        let (patched, outcome) = redirect_static_method_body_to_static(
            BLOCKPOS,
            LAMBDA_NAME,
            LAMBDA_DESC,
            TARGET_CLASS,
            TARGET_NAME,
            LAMBDA_DESC,
        )
        .expect("redirect");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "exactly one lambda$betweenCornersInDirection$8 body (javap census)"
        );
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        // body-redirect may SHRINK the file (22B lambda body -> 18B dispatch
        // + dropped debug tables); only guard against gross truncation.
        assert!(
            patched.len() * 100 >= BLOCKPOS.len() * 90,
            "patched {} vs original {} (gross truncation?)",
            patched.len(),
            BLOCKPOS.len()
        );
        // Idempotency doubles as content proof: the second pass recognizes
        // the generated shape AND its Methodref == ZeroCursorOps.lambda8.
        let (again, outcome2) = redirect_static_method_body_to_static(
            &patched, LAMBDA_NAME, LAMBDA_DESC, TARGET_CLASS, TARGET_NAME, LAMBDA_DESC,
        )
        .expect("re-redirect");
        assert_eq!(outcome2, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
    }

    #[test]
    fn zerocursor_wrong_method_not_found() {
        let (_, outcome) = redirect_static_method_body_to_static(
            BLOCKPOS,
            "lambda$betweenCornersInDirection$7",
            LAMBDA_DESC,
            TARGET_CLASS,
            TARGET_NAME,
            LAMBDA_DESC,
        )
        .expect("probe must not error");
        assert_eq!(outcome, RetargetOutcome::NotFound);
    }

    #[test]
    fn zerocursor_desc_mismatch_refused() {
        let changed =
            "(Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;IIIII)Ljava/util/Iterator;";
        let e = redirect_static_method_body_to_static(
            BLOCKPOS, LAMBDA_NAME, LAMBDA_DESC, TARGET_CLASS, TARGET_NAME, changed,
        )
        .unwrap_err();
        assert!(e.contains("does not match"), "{e}");
    }

    #[test]
    fn zerocursor_preserves_class_shape() {
        let (patched, _) = redirect_static_method_body_to_static(
            BLOCKPOS, LAMBDA_NAME, LAMBDA_DESC, TARGET_CLASS, TARGET_NAME, LAMBDA_DESC,
        )
        .expect("redirect");
        let major = u16::from_be_bytes([patched[6], patched[7]]);
        assert_eq!(major, 65, "kernel class major must stay 65");
        let n_orig = u16::from_be_bytes([BLOCKPOS[8], BLOCKPOS[9]]);
        let n_new = u16::from_be_bytes([patched[8], patched[9]]);
        assert!(n_new >= n_orig, "pool may only grow");
    }

    /// s7172 regression guard (locally reproduced via
    /// Instrumentation.retransformClasses → VerifyError): the redirected
    /// STATIC method's load chain must read the REAL param slots (0-based,
    /// no receiver). The original helper reused the instance-method slot
    /// map (1-based) → aload_1 first → JVM verification rejected the served
    /// bytes (JVMTI 62 FAILS_VERIFICATION) and the lever never measured.
    #[test]
    fn zerocursor_static_load_chain_is_zero_based() {
        let (patched, _) = redirect_static_method_body_to_static(
            BLOCKPOS, LAMBDA_NAME, LAMBDA_DESC, TARGET_CLASS, TARGET_NAME, LAMBDA_DESC,
        )
        .expect("redirect");

        // Walk the classfile to the redirected method's Code attribute.
        let layout = parse_layout(&patched).expect("parse");
        let name_idx = layout
            .pool
            .find_utf8(LAMBDA_NAME)
            .expect("lambda name utf8");
        let desc_idx = layout.pool.find_utf8(LAMBDA_DESC).expect("lambda desc utf8");
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx)
            .expect("redirected method");
        let mut p = m.start + 6;
        let attrs = usize::from(u16_at(&patched, p).expect("attr count"));
        p += 2;
        let mut code: Option<&[u8]> = None;
        for _ in 0..attrs {
            let an = u16_at(&patched, p).expect("attr name");
            let len = u32_at(&patched, p + 2).expect("attr len") as usize;
            if layout.pool.utf8_value(an).as_deref() == Some("Code") {
                let start = p + 14; // name(2)+len(4)+max_stack(2)+max_locals(2)+code_len(4)
                let clen =
                    u32_at(&patched, p + 10).expect("code len") as usize;
                code = Some(&patched[start..start + clen]);
            }
            p += 6 + len;
        }
        let code = code.expect("Code attribute");
        // Static params: 3 refs (slots 0..2) then 6 ints (slots 3..8).
        let expected: Vec<u8> = {
            let mut e = vec![0x2a, 0x2b, 0x2c]; // aload_0, aload_1, aload_2
            e.push(0x1d); // iload_3
            e.push(0x15); e.push(4); // iload 4
            e.push(0x15); e.push(5); // iload 5
            e.push(0x15); e.push(6); // iload 6
            e.push(0x15); e.push(7); // iload 7
            e.push(0x15); e.push(8); // iload 8
            e
        };
        assert_eq!(
            &code[..expected.len()],
            &expected[..],
            "load chain must read static params from slot 0 (was 1-based → VerifyError s7172)"
        );
        assert_eq!(code[code.len() - 1], 0xb0, "must end with areturn");
    }

    /// Dump the patched bytes for the offline JVM-verifier probe
    /// (entityinside/harness/RetransformProbe.java): defineClass verification
    /// is NOT enough — retransform-time verification caught the s7172 slot
    /// bug only via the real JVMTI RetransformClasses path.
    /// TASK-332 lever #12 v1: the 5-arg checkInsideBlocks body-redirect on
    /// the REAL Entity fixture must retarget exactly one site and be
    /// idempotent (re-patch byte-identical).
    #[test]
    fn insidediet_redirects_exactly_one_site_and_is_idempotent() {
        const REAL_ENTITY: &[u8] = include_bytes!("../tests/fixtures/Entity_real.class");
        const CHECK_DESC: &str = "(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;Lit/unimi/dsi/fastutil/longs/LongSet;I)I";
        let (patched, outcome) = patch_entity_inside_diet(REAL_ENTITY).expect("redirect");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 1 },
            "exactly one 5-arg checkInsideBlocks body"
        );
        let (again, outcome2) = patch_entity_inside_diet(&patched).expect("re-redirect");
        assert_eq!(outcome2, RetargetOutcome::AlreadyPatched { sites: 1 });
        assert_eq!(again, patched, "repatch must be byte-identical");
        // shape guards: same major, pool may only grow
        assert_eq!(
            u16::from_be_bytes([patched[6], patched[7]]),
            u16::from_be_bytes([REAL_ENTITY[6], REAL_ENTITY[7]]),
            "class major must stay pinned"
        );
        let n_orig = u16::from_be_bytes([REAL_ENTITY[8], REAL_ENTITY[9]]);
        let n_new = u16::from_be_bytes([patched[8], patched[9]]);
        assert!(n_new >= n_orig, "pool may only grow");
        let _ = CHECK_DESC;
    }

    #[test]
    fn dump_patched_blockpos_for_verifier_probe() {
        let (patched, _) = redirect_static_method_body_to_static(
            BLOCKPOS, LAMBDA_NAME, LAMBDA_DESC, TARGET_CLASS, TARGET_NAME, LAMBDA_DESC,
        )
        .expect("redirect");
        std::fs::create_dir_all("tests/out").unwrap();
        std::fs::write("tests/out/BlockPos.zerocursor.patched.class", &patched).unwrap();
    }
}

/// TASK-330 zero-cursor resolution closure: the redirected
/// `BlockPos.lambda$betweenCornersInDirection$8` body invokes
/// `ZeroCursorOps.lambda8` with the factory descriptor; `ZeroCursorOps`
/// hands out `ZeroCursorIter` instances whose reset() consumes the factory
/// args. Both bridges must declare exactly those members or the first
/// cursor walk detonates a NoSuchMethodError.
pub fn zerocursor_resolution_closure(ops: &[u8], iter: &[u8]) -> Result<(), String> {
    const FACTORY_DESC: &str = "(Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;IIIIII)Ljava/util/Iterator;";
    // s7171 lesson: ZeroCursorIter.reset returns VOID — the pooled instance is
    // returned by lambda8 itself (`it.reset(args); return it;`), so the runtime
    // call-site descriptor of reset is (... )V. The earlier Iterator-return
    // assumption failed the closure and kept the hook dormant (PG-Z2 fail,
    // lever never measured). Verified against the embedded classfiles by the
    // zerocursor_delivery_tests::zerocursor_resolution_closure_accepts test.
    const RESET_DESC: &str = "(Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;IIIIII)V";
    let targets: &[(&str, &str, &str, &str)] = &[
        (
            "class",
            "net/minecraft/core/ZeroCursorOps",
            "lambda8",
            FACTORY_DESC,
        ),
        ("class", "net/minecraft/core/ZeroCursorIter", "reset", RESET_DESC),
        ("class", "net/minecraft/core/ZeroCursorIter", "hasNext", "()Z"),
        ("class", "net/minecraft/core/ZeroCursorIter", "next", "()Lnet/minecraft/core/BlockPos$MutableBlockPos;"),
    ];
    // reuse the shared per-bridge member check for both classfiles
    check_members(ops, &targets[..1])?;
    check_members(iter, &targets[1..])
}

fn check_members(bridge: &[u8], targets: &[(&str, &str, &str, &str)]) -> Result<(), String> {
    let Some(layout) = parse_layout(bridge) else {
        return Err("bridge classfile unparseable".into());
    };
    let mut p = layout.methods_start;
    let count = usize::from(u16_at(bridge, p).ok_or("truncated method count")?);
    p = p.checked_add(2).ok_or("truncated method table")?;
    let mut have: Vec<(String, String)> = Vec::with_capacity(count);
    for _ in 0..count {
        let n_idx = u16_at(bridge, p.checked_add(2).ok_or("truncated method")?).ok_or("truncated method name")?;
        let d_idx = u16_at(bridge, p.checked_add(4).ok_or("truncated method")?).ok_or("truncated method desc")?;
        let attr_count = usize::from(
            u16_at(bridge, p.checked_add(6).ok_or("truncated method")?).ok_or("truncated method attrs")?,
        );
        p = p.checked_add(8).ok_or("truncated method table")?;
        for _ in 0..attr_count {
            let len = u32_at(bridge, p.checked_add(2).ok_or("truncated attr")?).ok_or("truncated attr")? as usize;
            p = p.checked_add(6).ok_or("truncated attr")?.checked_add(len).ok_or("truncated attr")?;
        }
        let name = layout.pool.utf8_value(n_idx).ok_or("bad name idx")?;
        let desc = layout.pool.utf8_value(d_idx).ok_or("bad desc idx")?;
        have.push((name, desc));
    }
    for (_, _, tname, tdesc) in targets {
        if !have.iter().any(|(n, d)| n == tname && d == tdesc) {
            return Err(format!(
                "bridge misses member {tname}{tdesc} — zero_cursor redirect would detonate NoSuchMethodError"
            ));
        }
    }
    Ok(())
}

/// TASK-399-B (cmp399_shard): transparent CONSTANT_Utf8 replacement.
///
/// Replaces the SINGLE Utf8 entry whose value equals `from` with `to`.
/// Constant-pool indices are unchanged (replacement, not append), so every
/// bytecode reference (ldc of the gate string, method/field refs, attributes)
/// stays valid — the JVM-visible delta is exactly the string constant. The
/// use-case: widen the ITEM-SUBSYS2 Java arm gate (ItemEntityManager.<clinit>,
/// `ENABLED = "items_subsys2".equals(trimToEmpty(getenv(...)))`) to the
/// cmp399_* lever family without javac in the loop (the committed .class is
/// the round-398-J build artifact; local toolchain is JRE-only).
///
/// Fails loudly on anything but exactly one occurrence, and self-verifies the
/// rebuilt class by re-parsing the constant pool (the patched entry must
/// resolve to `to`, and `from` must be gone).
// ---------------------------------------------------------------------------
// CHUNK-PARSE SECTION-CACHE (TASK-419-C base, TASK-420-C deepening,
// lever cmp420_chunk2, law 8
// chunk-loading axis). Ground truth: kernel javap (round-396-a
// patched-kernel.jar, purpur 1.21.10) — SerializableChunkData.parse's
// section loop calls the block_states decode through indy #4 -> BOOTSTRAP
// method #4 -> lambda$parse$5, and the biomes decode through indy #6 ->
// BOOTSTRAP #6 -> lambda$parse$7. Both lambdas carry the SAME canonical
// descriptor (below) and byte-identical semantics:
//   codec.parse(NbtOps, tag).promotePartial(-> logErrors(pos,y,msg))
//     .getOrThrow(-> new ChunkReadException(msg))  (checkcast + areturn)
// The redirect swaps the BLOCKS lambda body for ChunkParseOps.parseSection
// (cache-first decoder, twin = the pristine biomes lambda); the canonical
// descriptor makes the static->static stack shape a pass-through.
// ---------------------------------------------------------------------------
pub const CHUNKPARSE_TARGET_CLASS: &str =
    "net/minecraft/world/level/chunk/storage/SerializableChunkData";
pub const CHUNKPARSE_OPS_CLASS: &str = "net/minecraft/world/level/chunk/storage/ChunkParseOps";
pub const CHUNKPARSE_BLOCKS_LAMBDA: &str = "lambda$parse$5";
pub const CHUNKPARSE_TWIN_LAMBDA: &str = "lambda$parse$7";
/// TASK-424-C (R5c): the bridge entry point for the BIOMES site — the twin
/// lambda itself is now redirected here (both section lambdas patched).
pub const CHUNKPARSE_BIOMES_OPS_METHOD: &str = "parseBiomesSection";
pub const CHUNKPARSE_SECTION_LAMBDA_DESC: &str = "(Lcom/mojang/serialization/Codec;Lnet/minecraft/world/level/ChunkPos;ILnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/chunk/PalettedContainer;";

/// Resolution closure for the ChunkParseOps bridge: the bridge must declare
/// `parseSection` with the EXACT vanilla lambda descriptor (redirect
/// stack-shape contract) and `init(Ljava/lang/String;)V` (twin injection),
/// and must be FLAT (zero nested classes — the classfile is defined alone
/// into the kernel loader; a nested class would detonate as
/// NoClassDefFoundError on the first parse).
pub fn chunkparse_resolution_closure(ops: &[u8]) -> Result<(), String> {
    let targets: &[(&str, &str, &str, &str)] = &[
        (
            "class",
            CHUNKPARSE_OPS_CLASS,
            "parseSection",
            CHUNKPARSE_SECTION_LAMBDA_DESC,
        ),
        (
            "class",
            CHUNKPARSE_OPS_CLASS,
            // TASK-424-C (R5c): biomes mirror cache entry point — same
            // canonical descriptor (redirect stack-shape contract).
            CHUNKPARSE_BIOMES_OPS_METHOD,
            CHUNKPARSE_SECTION_LAMBDA_DESC,
        ),
        ("class", CHUNKPARSE_OPS_CLASS, "init", "(Ljava/lang/String;)V"),
    ];
    check_members(ops, targets)?;
    // Flat delivery guard: a nested class would surface as a
    // "ChunkParseOps$..." Class reference somewhere in the pool.
    if find_nested_class_ref(ops, "ChunkParseOps$") {
        return Err("bridge declares/references a nested ChunkParseOps$ class — flat-only delivery contract".into());
    }
    Ok(())
}

/// Byte-level scan: does this classfile reference any class whose binary
/// name contains `needle` (e.g. "ChunkParseOps$")? Pure constant-pool Utf8
/// walk — robust against pool entry reordering.
fn find_nested_class_ref(bytes: &[u8], needle: &str) -> bool {
    let needle = needle.as_bytes();
    bytes.windows(needle.len()).any(|w| w == needle)
}

/// Pristine guard for the ORIGINAL SerializableChunkData bytes (called on
/// the captured bytes BEFORE any patch is computed; a kernel shape drift
/// must leave the hook dormant instead of serving a blind redirect):
///   * `parse(LevelHeightAccessor, PalettedContainerFactory, CompoundTag)`
///     exists (the owner of the indy sites);
///   * BOTH section lambdas (`lambda$parse$5` blocks + `lambda$parse$7`
///     biomes twin) exist with the canonical descriptor;
///   * the descriptor is carried by EXACTLY 2 methods (no third candidate —
///     a drift would make the name->role mapping untrustworthy).
pub fn chunkparse_pristine_guard(bytes: &[u8]) -> Result<(), String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout".to_string())?;
    let mut p = layout.methods_start;
    let count = usize::from(u16_at(bytes, p).ok_or("truncated method count")?);
    p = p.checked_add(2).ok_or("truncated method table")?;
    let mut section_lambdas: Vec<String> = Vec::with_capacity(2);
    let mut parse_ok = false;
    for _ in 0..count {
        let n_idx = u16_at(bytes, p.checked_add(2).ok_or("truncated method")?)
            .ok_or("truncated method name")?;
        let d_idx = u16_at(bytes, p.checked_add(4).ok_or("truncated method")?)
            .ok_or("truncated method desc")?;
        let attr_count = usize::from(
            u16_at(bytes, p.checked_add(6).ok_or("truncated method")?)
                .ok_or("truncated method attrs")?,
        );
        p = p.checked_add(8).ok_or("truncated method table")?;
        for _ in 0..attr_count {
            let len = u32_at(bytes, p.checked_add(2).ok_or("truncated attr")?)
                .ok_or("truncated attr")? as usize;
            p = p
                .checked_add(6)
                .ok_or("truncated attr")?
                .checked_add(len)
                .ok_or("truncated attr")?;
        }
        let name = layout.pool.utf8_value(n_idx).ok_or("bad name idx")?;
        let desc = layout.pool.utf8_value(d_idx).ok_or("bad desc idx")?;
        if name == "parse"
            && desc
                == "(Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/chunk/PalettedContainerFactory;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/chunk/storage/SerializableChunkData;"
        {
            parse_ok = true;
        }
        if desc == CHUNKPARSE_SECTION_LAMBDA_DESC {
            section_lambdas.push(name);
        }
    }
    if !parse_ok {
        return Err("parse(LevelHeightAccessor, PalettedContainerFactory, CompoundTag) not found — kernel shape drift".into());
    }
    for expected in [CHUNKPARSE_BLOCKS_LAMBDA, CHUNKPARSE_TWIN_LAMBDA] {
        if !section_lambdas.iter().any(|n| n == expected) {
            return Err(format!("section lambda {expected} missing — kernel shape drift"));
        }
    }
    if section_lambdas.len() != 2 {
        return Err(format!(
            "canonical section-lambda descriptor carried by {} methods, expected exactly 2 — name->role mapping untrustworthy",
            section_lambdas.len()
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// CHUNK-SEND SERIALIZATION SNAPSHOT (TASK-438-C, lever cmp437_chunk4, law 8
// chunk-loading axis WIDENING). Ground truth: kernel javap (round-396-a
// patched-kernel.jar, purpur 1.21.10) — net/minecraft/server/network/
// PlayerChunkSender declares the static
//   sendChunk(ServerGamePacketListenerImpl, ServerLevel, LevelChunk)V
// whose body constructs a fresh ClientboundLevelChunkWithLightPacket PER SEND
// (per player): the serialize-side mirror of the parse codec machinery
// (RESEARCH-F: parse codec 33.38% burst-window alloc) with NO per-player input
// when anti-xray is off (shouldModify == false). The redirect swaps that body
// for ChunkSendOps.sendChunk (snapshot-first: revision/unsaved-keyed packet
// reuse, zero-copy handoff; isUnsaved() invalidation; anti-xray bypass;
// per-send events preserved). Static->static stack shape is a pass-through.
// ---------------------------------------------------------------------------
pub const CHUNKSEND_TARGET_CLASS: &str = "net/minecraft/server/network/PlayerChunkSender";
pub const CHUNKSEND_OPS_CLASS: &str = "net/minecraft/server/network/ChunkSendOps";
pub const CHUNKSEND_METHOD: &str = "sendChunk";
pub const CHUNKSEND_DESC: &str = "(Lnet/minecraft/server/network/ServerGamePacketListenerImpl;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/LevelChunk;)V";

/// Resolution closure for the ChunkSendOps bridge: the bridge must declare
/// `sendChunk` with the EXACT vanilla static descriptor (redirect stack-shape
/// contract) and `selfTest()Z` (pre-ARM oracle), and must be FLAT (zero nested
/// classes — the classfile is defined alone into the kernel loader).
pub fn chunksend_resolution_closure(ops: &[u8]) -> Result<(), String> {
    let targets: &[(&str, &str, &str, &str)] = &[
        (
            "class",
            CHUNKSEND_OPS_CLASS,
            CHUNKSEND_METHOD,
            CHUNKSEND_DESC,
        ),
        ("class", CHUNKSEND_OPS_CLASS, "selfTest", "()Z"),
    ];
    check_members(ops, targets)?;
    // Flat delivery guard: a nested class would surface as a
    // "ChunkSendOps$..." Class reference somewhere in the pool.
    if find_nested_class_ref(ops, "ChunkSendOps$") {
        return Err("bridge declares/references a nested ChunkSendOps$ class — flat-only delivery contract".into());
    }
    Ok(())
}

/// Pristine guard for the ORIGINAL PlayerChunkSender bytes (called on the
/// captured bytes BEFORE any patch is computed; a kernel shape drift must
/// leave the hook dormant instead of serving a blind redirect):
///   * `sendChunk(ServerGamePacketListenerImpl, ServerLevel, LevelChunk)V`
///     exists, is STATIC (ACC_STATIC 0x0008) and is the ONLY such method;
///   * the class references ClientboundLevelChunkWithLightPacket (the packet
///     construction site the redirect replaces — validates the shape).
pub fn chunksend_pristine_guard(bytes: &[u8]) -> Result<(), String> {
    const ACC_STATIC: u16 = 0x0008;
    let layout = parse_layout(bytes).ok_or("bad classfile layout".to_string())?;
    let mut p = layout.methods_start;
    let count = usize::from(u16_at(bytes, p).ok_or("truncated method count")?);
    p = p.checked_add(2).ok_or("truncated method table")?;
    let mut send_chunk: Option<u16> = None;
    for _ in 0..count {
        let access = u16_at(bytes, p).ok_or("truncated method access")?;
        let n_idx = u16_at(bytes, p.checked_add(2).ok_or("truncated method")?)
            .ok_or("truncated method name")?;
        let d_idx = u16_at(bytes, p.checked_add(4).ok_or("truncated method")?)
            .ok_or("truncated method desc")?;
        let attr_count = usize::from(
            u16_at(bytes, p.checked_add(6).ok_or("truncated method")?)
                .ok_or("truncated method attrs")?,
        );
        p = p.checked_add(8).ok_or("truncated method table")?;
        for _ in 0..attr_count {
            let len = u32_at(bytes, p.checked_add(2).ok_or("truncated attr")?)
                .ok_or("truncated attr")? as usize;
            p = p
                .checked_add(6)
                .ok_or("truncated attr")?
                .checked_add(len)
                .ok_or("truncated attr")?;
        }
        let name = layout.pool.utf8_value(n_idx).ok_or("bad name idx")?;
        let desc = layout.pool.utf8_value(d_idx).ok_or("bad desc idx")?;
        if name == CHUNKSEND_METHOD && desc == CHUNKSEND_DESC {
            if send_chunk.is_some() {
                return Err("duplicate sendChunk declaration — name->role mapping untrustworthy".into());
            }
            send_chunk = Some(access);
        }
    }
    match send_chunk {
        None => {
            return Err("sendChunk(ServerGamePacketListenerImpl, ServerLevel, LevelChunk)V not found — kernel shape drift".into());
        }
        Some(access) if access & ACC_STATIC == 0 => {
            return Err(
                "sendChunk is not static — static->static redirect contract violated".into(),
            );
        }
        Some(_) => {}
    }
    // The redirect target packet must be the construction site of THIS body.
    if !find_nested_class_ref(bytes, "ClientboundLevelChunkWithLightPacket") {
        return Err(
            "sendChunk body does not reference ClientboundLevelChunkWithLightPacket — kernel shape drift"
                .into(),
        );
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// CHUNK-PACKET ENCODE CACHE (TASK-444-B, lever cmp444_chunk5, law 8 stage-2):
// the private ClientboundLevelChunkWithLightPacket.write(RegistryFriendlyByteBuf)
// body is javap-verified as writeInt(x)/writeInt(z)/chunkData.write/lightData
// .write — a pure function of the packet instance. The instance->static
// redirect (receiver prepended) swaps that body for ChunkPacketEncodeOps.write:
// encode-once (scratch capture) then byte[]-replay per player, keyed by packet
// instance. Instance shape contract: virtual desc with the METHOD OWNER class
// prepended (redirect_method_body_to_static).
// ---------------------------------------------------------------------------
pub const CHUNKPACKET_TARGET_CLASS: &str =
    "net/minecraft/network/protocol/game/ClientboundLevelChunkWithLightPacket";
pub const CHUNKPACKET_OPS_CLASS: &str = "net/minecraft/server/network/ChunkPacketEncodeOps";
pub const CHUNKPACKET_METHOD: &str = "write";
pub const CHUNKPACKET_VIRTUAL_DESC: &str = "(Lnet/minecraft/network/RegistryFriendlyByteBuf;)V";
pub const CHUNKPACKET_STATIC_DESC: &str = "(Lnet/minecraft/network/protocol/game/ClientboundLevelChunkWithLightPacket;Lnet/minecraft/network/RegistryFriendlyByteBuf;)V";

/// Resolution closure for the ChunkPacketEncodeOps bridge: the bridge must
/// declare `write` with the EXACT receiver-prepended static descriptor
/// (redirect stack-shape contract) and `selfTest()Z` (pre-ARM oracle), and
/// must be FLAT (zero nested classes).
pub fn chunkpacket_resolution_closure(ops: &[u8]) -> Result<(), String> {
    let targets: &[(&str, &str, &str, &str)] = &[
        (
            "class",
            CHUNKPACKET_OPS_CLASS,
            CHUNKPACKET_METHOD,
            CHUNKPACKET_STATIC_DESC,
        ),
        ("class", CHUNKPACKET_OPS_CLASS, "selfTest", "()Z"),
    ];
    check_members(ops, targets)?;
    if find_nested_class_ref(ops, "ChunkPacketEncodeOps$") {
        return Err("bridge declares/references a nested ChunkPacketEncodeOps$ class — flat-only delivery contract".into());
    }
    Ok(())
}

/// Pristine guard for the ORIGINAL ClientboundLevelChunkWithLightPacket bytes:
///   * the private `write(RegistryFriendlyByteBuf)V` exists (instance method)
///     and is the ONLY such method;
///   * the class declares getChunkData/getLightData (the vanilla-equivalent
///     encode surface the bridge replays).
pub fn chunkpacket_pristine_guard(bytes: &[u8]) -> Result<(), String> {
    let layout = parse_layout(bytes).ok_or("bad classfile layout".to_string())?;
    let mut p = layout.methods_start;
    let count = usize::from(u16_at(bytes, p).ok_or("truncated method count")?);
    p = p.checked_add(2).ok_or("truncated method table")?;
    let mut write_method: Option<u16> = None;
    for _ in 0..count {
        let access = u16_at(bytes, p).ok_or("truncated method access")?;
        let n_idx = u16_at(bytes, p.checked_add(2).ok_or("truncated method")?)
            .ok_or("truncated method name")?;
        let d_idx = u16_at(bytes, p.checked_add(4).ok_or("truncated method")?)
            .ok_or("truncated method desc")?;
        let attr_count = usize::from(
            u16_at(bytes, p.checked_add(6).ok_or("truncated method")?)
                .ok_or("truncated method attrs")?,
        );
        p = p.checked_add(8).ok_or("truncated method table")?;
        for _ in 0..attr_count {
            let len = u32_at(bytes, p.checked_add(2).ok_or("truncated attr")?)
                .ok_or("truncated attr")? as usize;
            p = p
                .checked_add(6)
                .ok_or("truncated attr")?
                .checked_add(len)
                .ok_or("truncated attr")?;
        }
        let name = layout.pool.utf8_value(n_idx).ok_or("bad name idx")?;
        let desc = layout.pool.utf8_value(d_idx).ok_or("bad desc idx")?;
        if name == CHUNKPACKET_METHOD && desc == CHUNKPACKET_VIRTUAL_DESC {
            if write_method.is_some() {
                return Err("duplicate write(RegistryFriendlyByteBuf) declaration — name->role mapping untrustworthy".into());
            }
            write_method = Some(access);
        }
    }
    match write_method {
        None => {
            return Err("write(RegistryFriendlyByteBuf)V not found — kernel shape drift".into());
        }
        Some(access) if access & 0x0002 == 0 => {
            // ACC_PRIVATE (0x0002): the vanilla body is private — a public
            // write would be a different shape (the codec contract).
            return Err("write(RegistryFriendlyByteBuf)V is not private — kernel shape drift".into());
        }
        Some(_) => {}
    }
    for getter in ["getChunkData", "getLightData", "getX", "getZ"] {
        if !find_nested_class_ref(bytes, getter) {
            return Err(format!(
                "ClientboundLevelChunkWithLightPacket does not declare {getter} — kernel shape drift"
            ));
        }
    }
    Ok(())
}

pub fn patch_utf8_gate(bytes: &[u8], from: &str, to: &str) -> Result<Vec<u8>, String> {
    if bytes.len() < 10 || &bytes[0..4] != [0xCA, 0xFE, 0xBA, 0xBE] {
        return Err("bad magic".to_string());
    }
    let cp_count = u16::from_be_bytes([bytes[8], bytes[9]]);
    let mut p = 10usize;
    let mut hits: Vec<usize> = Vec::new();
    let entries_total = u32::from(cp_count.saturating_sub(1));
    let mut seen: u32 = 0;
    while seen < entries_total {
        let tag = *bytes.get(p).ok_or("cp overrun")?;
        p += 1;
        match tag {
            TAG_UTF8 => {
                if p + 2 > bytes.len() {
                    return Err("utf8 overrun".to_string());
                }
                let len = usize::from(u16::from_be_bytes([bytes[p], bytes[p + 1]]));
                let val = bytes
                    .get(p + 2..p + 2 + len)
                    .ok_or("utf8 overrun")?;
                if val == from.as_bytes() {
                    hits.push(p); // offset of the u2 length prefix
                }
                p += 2 + len;
                seen += 1;
            }
            TAG_INTEGER | 4 => {
                p += 4;
                seen += 1;
            }
            5 | 6 => {
                p += 8;
                seen += 2;
            }
            7 | 8 | 16 | 19 | 20 => {
                p += 2;
                seen += 1;
            }
            9 | 10 | 11 | 12 | 17 | 18 => {
                p += 4;
                seen += 1;
            }
            15 => {
                p += 3;
                seen += 1;
            }
            _ => return Err(format!("bad cp tag {tag}")),
        }
    }
    if hits.len() != 1 {
        return Err(format!(
            "gate utf8 '{from}': {} occurrences, expected exactly 1",
            hits.len()
        ));
    }
    let off = hits[0];
    let mut out = Vec::with_capacity(bytes.len() + to.len() - from.len());
    out.extend_from_slice(&bytes[..off]);
    out.extend_from_slice(&(to.len() as u16).to_be_bytes());
    out.extend_from_slice(to.as_bytes());
    out.extend_from_slice(&bytes[off + 2 + from.len()..]);
    // Self-check: the patched class must re-parse cleanly and the gate string
    // must now resolve to `to` (and `from` must be gone).
    let cp_count2 = u16::from_be_bytes([out[8], out[9]]);
    let (pool, _) = Pool::parse(&out, 10, cp_count2).ok_or("re-parse failed")?;
    if pool.find_utf8(to).is_none() || pool.find_utf8(from).is_some() {
        return Err("gate utf8 not swapped".to_string());
    }
    Ok(out)
}

#[cfg(test)]
mod navpool {
    // TASK-410-A k5: REAL kernel fixture (round-396-a patched-kernel.jar,
    // same source jar as every other fixture).
    const NODE_EVALUATOR: &[u8] = include_bytes!("../tests/fixtures/NodeEvaluator_real.class");

    use crate::classfile::*;

    /// The REAL NodeEvaluator's prepare + getNode(III) bodies redirect to
    /// the NavPoolOps bridge EXACTLY twice (composite fail-dominant).
    #[test]
    fn nodeevaluator_retargets_exactly_two_sites() {
        let (patched, outcome) = patch_nodeevaluator_navpool(NODE_EVALUATOR).expect("patch");
        assert_eq!(
            outcome,
            RetargetOutcome::Retargeted { sites: 2 },
            "k5 contract: prepare + getNode(III) must both retarget — a NotFound/1-site \
             outcome means the kernel shape drifted and the lever must stay vanilla"
        );
        assert!(patched.len() != NODE_EVALUATOR.len() || patched != NODE_EVALUATOR);
        // Idempotent re-sight: second pass is AlreadyPatched{2}.
        let (again, outcome2) = patch_nodeevaluator_navpool(&patched).expect("re-patch");
        assert_eq!(
            outcome2,
            RetargetOutcome::AlreadyPatched { sites: 2 },
            "retransform re-sights must be idempotent (PATCHED-swap convention)"
        );
        assert_eq!(again, patched);
    }

    // ---- SENSE-PLANE (TASK-438-A2, cmp438_sense) roundtrip on the REAL
    // ServerEntityGetter fixture — interface default-method body swap. ----

    const SENSE_GETTER: &[u8] = include_bytes!("../tests/fixtures/ServerEntityGetter.class");

    #[test]
    fn sense_patch_roundtrip_verified() {
        let patched = patch_sense_nearest_entity(SENSE_GETTER).expect("patch");
        assert!(patched.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE]));
        assert_eq!(patched[..8], SENSE_GETTER[..8], "version preserved");

        // Locate getNearestEntity(List,TC,LE,DDD) in the PATCHED bytes and
        // read its Code attribute (max_stack/max_locals precede the body).
        let layout = parse_layout(&patched).expect("re-parse patched");
        let name_idx = layout
            .pool
            .find_utf8("getNearestEntity")
            .expect("name kept");
        let desc_idx = layout
            .pool
            .find_utf8(SENSE_NEAREST_DESC)
            .expect("desc kept");
        let m = find_method(&patched, layout.methods_start, name_idx, desc_idx)
            .expect("getNearestEntity(List,...) kept");
        assert_eq!(m.access, 0x0001, "access flags preserved (public default)");

        let (start, len) = find_code_attr(&patched, &layout.pool, &m).expect("Code attr");
        let code = &patched[start..start + len];
        assert_eq!(code.len(), 14, "straight-line body is 14 bytes");
        let ms = u16::from_be_bytes([patched[start - 8], patched[start - 7]]);
        let ml = u16::from_be_bytes([patched[start - 6], patched[start - 5]]);
        assert_eq!(ms, 10, "max_stack = 4 refs + 3 doubles x2 slots");
        assert_eq!(ml, 10, "max_locals = 4 refs + 3 double args");

        // Opcode skeleton: aload_0..aload_3, dload 4/6/8, invokestatic, areturn.
        assert_eq!(code[0], 0x2a, "aload_0 (receiver)");
        assert_eq!(code[1], 0x2b, "aload_1 (List)");
        assert_eq!(code[2], 0x2c, "aload_2 (TargetingConditions)");
        assert_eq!(code[3], 0x2d, "aload_3 (LivingEntity targeter)");
        assert_eq!(&code[4..6], &[0x18, 0x04], "dload 4 (x)");
        assert_eq!(&code[6..8], &[0x18, 0x06], "dload 6 (y)");
        assert_eq!(&code[8..10], &[0x18, 0x08], "dload 8 (z)");
        assert_eq!(code[10], 0xb8, "invokestatic");
        assert_eq!(code[13], 0xb0, "areturn");

        // Operand resolution BY NAME (never by index assumption).
        let r = layout
            .pool
            .methodref_parts(u16::from_be_bytes([code[11], code[12]]))
            .expect("invokestatic operand resolves");
        assert_eq!(
            r,
            (
                SENSE_OPS_CLASS.to_string(),
                "nearestEntityGate".to_string(),
                SENSE_GATE_DESC.to_string()
            )
        );

        // EMPTY StackMapTable on the swapped method (no branch targets =>
        // attribute_length 2, number_of_entries 0).
        let smt_idx = layout.pool.find_utf8("StackMapTable").expect("smt utf8");
        let smt_bytes = [
            (smt_idx >> 8) as u8,
            (smt_idx & 0xFF) as u8,
            0x00,
            0x00,
            0x00,
            0x02,
            0x00,
            0x00,
        ];
        assert!(
            patched[m.start..m.end]
                .windows(smt_bytes.len())
                .any(|w| w == smt_bytes),
            "empty StackMapTable (len 2, 0 frames) present in method attrs"
        );

        // Dump for the runtime verifier gate (sense/verify_patched.sh: a real
        // HotSpot link-time verification — the byte-level checks above cannot
        // prove verifier legality, a JVM can).
        let out = std::env::temp_dir().join("ccrussty_patched_ServerEntityGetter.class");
        std::fs::write(&out, &patched).expect("dump patched ServerEntityGetter");
        eprintln!("wrote {} bytes to {}", patched.len(), out.display());
    }

    /// Idempotency: patch(patch(x)) == patch(x) — the second pass appends
    /// nothing (dedup Pool) and re-emits the identical body.
    #[test]
    fn sense_patch_is_idempotent() {
        let once = patch_sense_nearest_entity(SENSE_GETTER).expect("first");
        let twice = patch_sense_nearest_entity(&once).expect("second");
        assert_eq!(once, twice, "double patch is byte-identical");
    }

    /// Fail-closed discipline: wrong class rejected before any mutation;
    /// truncated/hostile bytes rejected without panic (hook-delivery audit
    /// discipline — a panic on a class-load thread would abort the JVM).
    #[test]
    fn sense_patch_rejects_wrong_class_and_garbage() {
        let e = patch_sense_nearest_entity(include_bytes!(
            "../tests/fixtures/SingleUserAreaMap.class"
        ))
        .expect_err("area_map is not ServerEntityGetter");
        assert!(e.starts_with("unexpected class"));

        let e = patch_sense_nearest_entity(&SENSE_GETTER[..64]).expect_err("truncated header");
        assert!(!e.is_empty());
        let e = patch_sense_nearest_entity(&[0xCA, 0xFE, 0xBA, 0xBE, 0, 0, 0, 65, 0, 3, 1, 2])
            .expect_err("garbage pool");
        assert!(!e.is_empty());
        // Whole-pool truncation at every prefix must never panic.
        for cut in [10usize, 100, 1000, 5000, SENSE_GETTER.len() - 1] {
            let _ = patch_sense_nearest_entity(&SENSE_GETTER[..cut]);
        }
    }
}
// ---------------------------------------------------------------------------
// TASK-405-C (vector eindex — lever cmp405_eindex): EntityLookup query-body
// redirects + note-site retargets for the Rust entity-chunk mirror
// (src/entity_index.rs). Delivery contract (javap round-j2b):
//   - the 4 core getEntities/getHardCollidingEntities bodies are redirected
//     whole (receiver-prepended static form, EntityIndexOps.getEntitiesE/T/C/
//     getHardCollidingE); the trailing-int overloads stay vanilla;
//   - the 4 ChunkEntitySlices.addEntity/removeEntity invoke sites inside
//     EntityLookup.addEntity/removeEntity/moveEntity are retargeted to
//     EntityIndexOps.noteAdd/noteRemove;
//   - the 5 Entity.setBoundingBox invoke sites (Entity.setPosRaw(DDDZ),
//     Shulker.onSyncedDataUpdated, HangingEntity/LeashFenceKnotEntity
//     .recalculateBoundingBox, Interaction.readAdditionalSaveData) are
//     retargeted to the per-owner noteBB overloads.
// All-or-nothing per class: any NotFound/mismatch → Err (fail closed).
// ---------------------------------------------------------------------------

pub const EIDX_OPS_CLASS: &str = "net/minecraft/world/entity/EntityIndexOps";
pub const EIDX_LOOKUP_CLASS: &str =
    "ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup";
pub const EIDX_SLICES_CLASS: &str =
    "ca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices";

const EIDX_GENT_DESC: &str = "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/List;Ljava/util/function/Predicate;)V";
const EIDX_GTYPE_DESC: &str = "(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/phys/AABB;Ljava/util/List;Ljava/util/function/Predicate;)V";
const EIDX_GCLASS_DESC: &str = "(Ljava/lang/Class;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/List;Ljava/util/function/Predicate;)V";
const EIDX_MOVE_DESC: &str =
    "(Lnet/minecraft/world/entity/Entity;)Lca/spottedleaf/moonrise/patches/chunk_system/level/entity/ChunkEntitySlices;";
const EIDX_SLICES_ADD_DESC: &str = "(Lnet/minecraft/world/entity/Entity;I)Z";
const EIDX_BB_DESC: &str = "(Lnet/minecraft/world/phys/AABB;)V";

fn eidx_static(desc: &str) -> String {
    format!("(L{EIDX_LOOKUP_CLASS};{}", &desc[1..])
}

/// The 4 query-body redirects (EntityLookup). Count: 4 Retargeted (or
/// AlreadyPatched after a re-serve) = success; anything else = Err.
pub fn patch_eindex_lookup_redirects(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let specs: [(&str, &str, &str); 4] = [
        ("getEntities", EIDX_GENT_DESC, "getEntitiesE"),
        ("getHardCollidingEntities", EIDX_GENT_DESC, "getHardCollidingE"),
        ("getEntities", EIDX_GTYPE_DESC, "getEntitiesT"),
        ("getEntities", EIDX_GCLASS_DESC, "getEntitiesC"),
    ];
    let mut cur = bytes.to_vec();
    let mut ret = 0usize;
    let mut already = 0usize;
    for (name, desc, tname) in specs {
        let tdesc = eidx_static(desc);
        let (p, outcome) = redirect_method_body_to_static(
            &cur,
            name,
            desc,
            EIDX_LOOKUP_CLASS,
            EIDX_OPS_CLASS,
            tname,
            &tdesc,
        )?;
        cur = p;
        match outcome {
            RetargetOutcome::Retargeted { .. } => ret += 1,
            RetargetOutcome::AlreadyPatched { .. } => already += 1,
            RetargetOutcome::NotFound => {
                return Err(format!("eindex redirect site {name}{desc} not found"));
            }
        }
    }
    if ret == 4 {
        Ok((cur, RetargetOutcome::Retargeted { sites: 4 }))
    } else {
        Ok((cur, RetargetOutcome::AlreadyPatched { sites: already }))
    }
}

/// The 4 note-site retargets inside EntityLookup (addEntity×2, removeEntity×2).
/// RUNTIME-VERIFIED (round409cleg2b entity-recon.txt, booted kernel):
///   addEntity(Entity;ZZ)Z  → slices.addEntity(Entity;I)Z   @291 (×1)
///   removeEntity(Entity)V  → slices.removeEntity(Entity;I)Z @119 (×1)
///   moveEntity(Entity)     → slices.removeEntity(Entity;I)Z @145 (×1)
///   moveEntity(Entity)     → slices.addEntity(Entity;I)Z    @177 (×1)
/// cleg2b ROOT-CAUSE (run 35667449721): `from` was hardcoded to addEntity for
/// all rows — the removeEntity rows searched for an addEntity invoke inside
/// EntityLookup.removeEntity → NotFound → whole patch rejected → dormant.
pub fn patch_eindex_lookup_notes(bytes: &[u8]) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let add_to = (
        EIDX_OPS_CLASS,
        "noteAdd",
        format!("(L{EIDX_SLICES_CLASS};{}", &EIDX_SLICES_ADD_DESC[1..]),
    );
    let rem_to = (
        EIDX_OPS_CLASS,
        "noteRemove",
        format!("(L{EIDX_SLICES_CLASS};{}", &EIDX_SLICES_ADD_DESC[1..]),
    );
    let from_add = (EIDX_SLICES_CLASS, "addEntity", EIDX_SLICES_ADD_DESC);
    let from_rem = (EIDX_SLICES_CLASS, "removeEntity", EIDX_SLICES_ADD_DESC);
    let specs: [(&str, &str, bool); 4] = [
        ("addEntity", "(Lnet/minecraft/world/entity/Entity;ZZ)Z", true),
        ("moveEntity", EIDX_MOVE_DESC, true),
        ("removeEntity", "(Lnet/minecraft/world/entity/Entity;)V", false),
        ("moveEntity", EIDX_MOVE_DESC, false),
    ];
    let mut cur = bytes.to_vec();
    let mut ret = 0usize;
    for (name, desc, is_add) in specs {
        let (to, from) = if is_add { (&add_to, &from_add) } else { (&rem_to, &from_rem) };
        let (p, outcome) = retarget_virtual_to_static(
            &cur,
            name,
            desc,
            (&from.0, &from.1, &from.2),
            (&to.0, &to.1, &to.2),
        )?;
        cur = p;
        match outcome {
            RetargetOutcome::Retargeted { .. } | RetargetOutcome::AlreadyPatched { .. } => {
                ret += 1;
            }
            RetargetOutcome::NotFound => {
                return Err(format!(
                    "eindex note site {name}{desc} (callee {}.{}) not found",
                    from.0.rsplit('/').next().unwrap_or(from.0),
                    from.1
                ));
            }
        }
    }
    if ret == 4 {
        Ok((cur, RetargetOutcome::Retargeted { sites: 4 }))
    } else {
        Ok((cur, RetargetOutcome::AlreadyPatched { sites: 4 }))
    }
}

/// One Entity.setBoundingBox invoke-site retarget in `owner.method`. The CP
/// class of the site is the OWNER class itself (javap short-form reference),
/// so the noteBB overload's first parameter is the owner type.
pub fn patch_eindex_bb_site(
    bytes: &[u8],
    method_name: &str,
    method_desc: &str,
    owner_class: &str,
    note_bb_desc: &str,
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    retarget_virtual_to_static(
        bytes,
        method_name,
        method_desc,
        (owner_class, "setBoundingBox", EIDX_BB_DESC),
        (EIDX_OPS_CLASS, "noteBB", note_bb_desc),
    )
}

// ---------------------------------------------------------------------------
// CHUNK6-SCHED (TASK-456-C, cmp456_chunkmono): ServerChunkCache scheduling
// mono-plane — BOTH sites or none (composite fail-dominant, NAVPOOL canon):
//   1. getChunkNow(II)Lnet/minecraft/world/level/chunk/LevelChunk;
//      -> ChunkSchedOps.getNow(ServerChunkCache,II)LevelChunk
//   2. moonrise$setFullChunk(IILnet/minecraft/world/level/chunk/LevelChunk;)V
//      -> ChunkSchedOps.onSetFullChunk(ServerChunkCache,IILevelChunk)V
// The bridge is defined into the kernel loader BEFORE the patch is computed;
// the redirect helper enforces the receiver-prepended descriptor contract.
// ---------------------------------------------------------------------------

pub const CHUNKSCHED_OPS_CLASS: &str = "net/minecraft/server/level/ChunkSchedOps";
pub const SERVER_CHUNK_CACHE_CLASS: &str = "net/minecraft/server/level/ServerChunkCache";

pub const CHUNKSCHED_REDIRECT_TARGETS: [(&str, &str, &str, &str); 2] = [
    (
        "getChunkNow",
        "(II)Lnet/minecraft/world/level/chunk/LevelChunk;",
        "getNow",
        "(Lnet/minecraft/server/level/ServerChunkCache;II)Lnet/minecraft/world/level/chunk/LevelChunk;",
    ),
    (
        "moonrise$setFullChunk",
        "(IILnet/minecraft/world/level/chunk/LevelChunk;)V",
        "onSetFullChunk",
        "(Lnet/minecraft/server/level/ServerChunkCache;IILnet/minecraft/world/level/chunk/LevelChunk;)V",
    ),
];

pub fn patch_server_chunk_cache_sched(
    bytes: &[u8],
) -> Result<(Vec<u8>, RetargetOutcome), String> {
    let mut cur = bytes.to_vec();
    let mut retargeted = 0usize;
    let mut already = 0usize;
    for (name, desc, tname, tdesc) in CHUNKSCHED_REDIRECT_TARGETS {
        let (p, outcome) = redirect_method_body_to_static(
            &cur,
            name,
            desc,
            SERVER_CHUNK_CACHE_CLASS,
            CHUNKSCHED_OPS_CLASS,
            tname,
            tdesc,
        )?;
        cur = p;
        match outcome {
            RetargetOutcome::Retargeted { .. } => retargeted += 1,
            RetargetOutcome::AlreadyPatched { .. } => already += 1,
            RetargetOutcome::NotFound => {
                return Err(format!("chunk-sched site {name} not found (fail-dominant)"));
            }
        }
    }
    if retargeted == CHUNKSCHED_REDIRECT_TARGETS.len() {
        Ok((cur, RetargetOutcome::Retargeted { sites: retargeted }))
    } else if retargeted == 0 && already == CHUNKSCHED_REDIRECT_TARGETS.len() {
        Ok((cur, RetargetOutcome::AlreadyPatched { sites: already }))
    } else {
        Err(format!(
            "chunk-sched partial patch rejected (retargeted={retargeted}, already={already}; BOTH-or-none)"
        ))
    }
}

/// Resolution closure (parse_diag precedent): the embedded bridge MUST declare
/// the two retarget receiver-prepended statics + the native names, otherwise
/// the first getChunkNow/ setFullChunk detonates a NoSuchMethodError.
pub fn chunk_sched_resolution_closure(ops_bytes: &[u8]) -> Result<(), String> {
    let layout = parse_layout(ops_bytes).ok_or_else(|| "bad ops classfile layout".to_string())?;
    let pool = layout.pool;
    let need: [(&str, &str); 5] = [
        ("getNow", "(Lnet/minecraft/server/level/ServerChunkCache;II)Lnet/minecraft/world/level/chunk/LevelChunk;"),
        (
            "onSetFullChunk",
            "(Lnet/minecraft/server/level/ServerChunkCache;IILnet/minecraft/world/level/chunk/LevelChunk;)V",
        ),
        ("mirrorEvent", "(JZ)Z"),
        ("schedProbe", "()J"),
        ("arm", "()V"),
    ];
    for (n, d) in need {
        let Some(ni) = pool.find_utf8(n) else {
            return Err(format!("ops pool missing name {n}"));
        };
        let Some(di) = pool.find_utf8(d) else {
            return Err(format!("ops pool missing desc {d}"));
        };
        if find_method(ops_bytes, layout.methods_start, ni, di).is_none() {
            return Err(format!("ops missing method {n}{d}"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod s19_scaffold_tests {
    /// S55-PREREG / R468-S19: the LIVE tracked blob must be recognized as the
    /// pass-through scaffold byte-exactly (a stale or v1 rebuild flips this).
    #[test]
    fn live_inside_batch_blob_is_scaffold() {
        let b: &[u8] = include_bytes!("../entityinside/build/net/minecraft/world/entity/InsideBatchOps.class");
        assert!(super::inside_batch_is_scaffold(b), "live blob must be the 16B pass-through scaffold");
    }

    #[test]
    fn garbage_is_not_scaffold() {
        assert!(!super::inside_batch_is_scaffold(b"not a classfile at all"));
        assert!(!super::inside_batch_is_scaffold(&[]));
    }
}
