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

struct Pool {
    entries: Vec<Entry>,
    /// 1-based index the next appended entry will get; after a parse this
    /// equals cp_count (index space = 1 + sum of apparent slots).
    next: u16,
}

impl Pool {
    /// Parse the constant pool of `bytes` starting at `cp_start`; returns the
    /// pool and the offset just past it (where access_flags begins).
    fn parse(bytes: &[u8], cp_start: usize, cp_count: u16) -> Option<(Pool, usize)> {
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
    fn find_utf8(&self, s: &str) -> Option<u16> {
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

struct ClassLayout {
    pool: Pool,
    /// Offset of access_flags (end of the constant pool region).
    cp_end: usize,
    /// cpool index of this_class (a CONSTANT_Class entry).
    this_class_idx: u16,
    /// Offset of the methods_count field (start of the method table).
    methods_start: usize,
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
}
