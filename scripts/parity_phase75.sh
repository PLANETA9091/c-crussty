#!/usr/bin/env bash
# ==============================================================================
# parity_phase75.sh — DP-PARITY ФАЗА-7.5 СКЕЛЕТ (C78), MEGA-SWARM v19.0, ROUND-489
# Воплощение entity-линии спеки CLM-C04.md (SPEC-READY, D1-D9 uuid_multiset_sha256).
# СТАТУС: лабораторный скелет (закон 5: 0 правок repo/master/CI; диспатч-точка
# вклинивания задокументирована ниже, в repo НЕ внесена).
#
# ТОЧКА ВКЛИНИВАНИЯ (verified read 2026-09-29, bench/world3/run_world3.sh, 930 строк):
#   L911: kill -9 "$SERVER_PID" 2>/dev/null || true   <- сервер полностью остановлен
#   L915: rm -f "$WORK/console.pipe"
#   L916: # --- 8. bottleneck report -------------------------------------------
# ФАЗА-7.5 вставляется МЕЖДУ L911 и L916 (после rm console.pipe, перед фазой-8):
#   # --- 7.5 dp-parity fingerprint (off-clock, post-stop; C04 §2) --------------
#   PARITY_T0=$(date +%s)
#   timeout 90 bash "$SCRIPT_DIR/parity_phase75.sh" "$SERVER/world" "$WORK" \
#     >> "$WORK/dp-parity-fp.log" 2>&1 \
#     || log "WARN: dp-parity phase7.5 failed (non-fatal, fail-open; gate увидит
#             отсутствие/ошибку fp.json = UNKNOWN, НЕ парити-FAIL)"
#   log "dp-parity phase7.5: $(( $(date +%s) - PARITY_T0 ))s"
# АРТЕФАКТ (+2 файла ≤2MB, в upload world3-bench после world3-run/run-env.txt):
#   world3-run/dp-parity-fp.json
#   world3-run/dp-parity-fp.log
#
# ША-КАНОН (прeregistered, CLM-C78 = CLM-C04 §3 entity-линия):
#   entity_uuid_multiset_sha256:
#     UUID-строки java UUID.toString() lowercase (32 hex, дефисы);
#     сортировка ЛЕКСИКОГРАФИЧЕСКАЯ byteorder (LC_ALL=C sort, БЕЗ -u —
#     multiset, дубликаты НЕ дедуплицируются: сдвиг RNG=клонирование ловится);
#     join "\n" + завершающий "\n"; UTF-8 байты -> sha256 hex lowercase.
#     Пустой multiset -> sha256("\n") = 01ba4719c80b6fe911b091a7c05124b6
#     4eeece964e09c058ef8f9805daca546b (пустой != null).
#     ПОПРАВКА К C04 §3: C04 цитировала префикс «526b3cd2…» — НЕ совпадает ни с
#     sha256("\n")=01ba4719…, ни с sha1("\n")=adc83b19… (проверено 2026-09-29);
#     скелет пинит 01ba4719… как авторитетную empty-константу D6.
#   entity_pos_digest_sha256:
#     записи "<uuid>,%.3f,%.3f,%.3f" (x,y,z; LC_ALL=C/LC_NUMERIC=C — точка,
#     libm round-half-even = python %.3f байт-в-байт); сорт по ПОЛНОЙ строке
#     записи (первичный ключ uuid; тай-брейк dup-uuid — по pos-суффиксу,
#     детерминирован); join "\n"+"\n" -> sha256.
#     Инвариантен к порядку NBT-записей; %.3f = 1мм-гранулярность (дрейф
#     AI-пути <1мм не фейлит бит-в-бит цель).
#   ВЕСЬ скрипт выполняется под LC_ALL=C LC_NUMERIC=C (анти-хостильная-локаль).
#
# v2 (round-491-c70-parv2, C70 ИМПЛЕМЕНТ поверх C76/C78): DEFERRED сняты для D4/D7
#   (C04 ×491 стадия-1; C27 P1/P2):
#   D4 world_sha256 + chunk_checksums — БАЙТ-В-БАЙТ канон арбитра
#   scripts/world_diff_parity_v2.py: scan_region_dir/iter_region_chunks/section_blocks/
#   palette_str/_canon_digest; per-chunk = sha256("".join(f"{y}:{dig}" для y sorted));
#   world = _side_digest: "cx,cz,status;" + "y:dig;" по sorted((cx,cz)) ЧИСЛОВО, LAST-WINS
#   на dup (cx,cz) (== арбитр dict-семантика). per-chunk карта — в ОТДЕЛЬНОМ артефакте
#   dp-parity-fp-region.json (C27 P1: 9216 чанков ≈ 590KB ≪ 2MB; main fp.json не пухнет).
#   region-ошибки (ctype 4+/усечение) -> region_errors>0 -> world_sha256=null (fail-open,
#   анти-полу-дайджест: никакой digest по частичному множеству).
#   D7 scoreboard_sha256 — канон C04 ×489 §3: sorted (objective,holder,score) ->
#   LC_ALL=C sort -> join "\n"+"\n" -> sha256 (инвариант к HashMap-порядку NBT-списков;
#   absent==absent -> SKIP-subfield). РАЗРЕШЕНИЕ КОНФЛИКТА C27-P2 «raw-file sha256» vs
#   C04-канон: берётся PREREGISTERED C04-канон — raw-sha фейлит на пере-сериализации
#   (gzip-заголовок/порядок списков) = ложный D7-FAIL = ложный REFUTED (анти-цель канона).
#   Objectives-без-скоров D7 не видит (документированное ограничение; скоры = геймплей-
#   видимая поверхность, определения целей покрывает D1-пин стенда/dp).
#
# РЕЖИМЫ:
#   parity_phase75.sh <world_dir> <work_dir>   # скан (fail-open: всегда exit 0)
#   parity_phase75.sh --selftest [tmpdir]      # самотест (fail-closed: exit 1
#                                              #  при любом FAIL-кейсе)
# Самотест: 13 кейсов на фикстурах (T-канон, клон/удаление/пере-порядок =
# D6-класс; hostile-locale; END-TO-END на реальных сгенерированных mca-байтах
# entities/r.0.0.mca + region/r.0.0.mca + scoreboard.dat + level.dat;
# fail-open-доказательство S10; v2: T4 D4-region-мутант, T5 D7-scoreboard).
# D4-константы — вычислены САМИМ арбитром world_diff_parity_v2.py на тех же
# фикстурах (byte-align по построению); D7 — независимой python3-hashlib.
# Зависимости: bash4+, coreutils (sort/sha256sum/mktemp), awk, python3 (stdlib:
# gzip/zlib/struct) — уже есть на runner'ах (report_world3.py).
# ==============================================================================

# --- локаle-пин (анти-хостильная-локаль; C04 «LC_NUMERIC=C» суперсет) ---------
export LC_ALL=C LC_NUMERIC=C LANG=C
umask 022

# --- прeregistered selftest-константы (python3 hashlib, независимая реализация)
U1="00000000-0000-0000-0000-000000000001"
U2="0f0e0d0c-0b0a-4321-8765-ba0987654321"
U3="7f3a2b1c-4d5e-4f60-a1b2-c3d4e5f60718"
U4="ffffffff-ffff-4fff-bfff-ffffffffffff"
H_BASE="22e68ac46b33ccb55397080440b4252a92b016a68fe2ea2b547a2b2e6a925762"   # {U1..U4} сорт+join
H_CLONE="097ab16c1f1b4a7eda7577293ba1c9ca0013f0d285be686a3c60ed4a426501f4"  # {U1,U2,U2,U3,U4}
H_REMOVE="902f75448737b4bd533ece6798cd8b6b204caa08043e04fcd2d3cd35911cd97e" # {U1,U2,U3}
H_EMPTY="01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b"  # sha256("\n")
H_POS_BASE="09135502fc5d2750714bcfc515135f56b36394d664c9a53fbac19129d98d6bae"
H_POS_CLONE="ad9228bafb57e2d48aa9c2a2cf514c6c228b55c77f3a16f9eeb9af7c0252e31c"
H2="33ef09792dca659a30ba0fc05cf8ded8c28983e7b299606e98c4d437204830b1"       # mca-фикстура {U1,U2}
H2C="f24c1af331a23fcd54b22be136b54bcf89bd707113de271696d53b68cd7f83e1"      # mca-мутант {U1,U2,U2}
P2="6fe33a9757578c50e3a2266828eb74b126b2d446f4cf0d26d42f66cb4fb3fa1c"       # mca pos-дайджест
P2C="e7fa219f5e25e928c0e00a8c7c65b2edb16916d7dc15e5b64a450a17a9678805"      # mca pos-мутант
POS_U1="0.0005,64.0,-0.0004"; POS_U2="100.1235,65.5,2000.9995"
# POS_%.3f ожидания (libm round-half-even): 0.0005->"0.001"; -0.0004->"-0.000";
# 100.1235->"100.124"; 2000.9995->"2000.999"
FIX_LEVEL_TIME=133700; FIX_DATAVERSION=4440
# --- v2 D4/D7 preregistered константы. AUTHORITY: D4-константы вычислены САМИМ
#     арбитром world_diff_parity_v2.py (scan_region_dir/_side_digest/_canon_digest)
#     на фикстурах ниже = byte-align по построению; D7 — независимый python-hashlib.
H_RBASE="1d7043335a473695bcea1ea0e891ca474c26283bca9c5aafabba614020515150"  # worldR world_sha256 (arbiter)
H_RMUT="772f3110743fce3e7083a3795dd3a22a1d11fb59a796cac839f52bdfb9d623cc"  # worldR-mut (coarse_dirt) != H_RBASE
H_CK00="12297eb904ccd670491a7d660940fea268e4ec25cacafd883ecd9c71544dafbb"  # per-chunk 0,0 (stone|air+2 longs, air-секция Y=-1)
H_CK53="e027d24052e08465b2bcee63ee0e1f3823f5f2916d63dece73306adf6ca42cef"  # per-chunk 5,3 (dirt, без data)
H_CK53M="e61fb075ef577ca0591a68bcfed64a045681c8ce88bc9c91331384c81fa4151a" # per-chunk 5,3 мутант
H_SB="61c080e881efc00c4446b6569062f69fa38583d613bacd587b7e2ce9dac2a8fd"    # scoreboard base (Alice5/Bob3/Carol7, вставка НЕсортированная)
H_SBMUT="66370ba404e9a83dff9302d800214741e8f95cb10bf2414dfcfaafad0e21acb9" # scoreboard мутант (Bob 3->4)

LAST_TMPD=""; SELFTEST_TMPD=""
cleanup_all() { [ -n "$LAST_TMPD" ] && rm -rf "$LAST_TMPD" 2>/dev/null; \
                [ -n "$SELFTEST_TMPD" ] && rm -rf "$SELFTEST_TMPD" 2>/dev/null; return 0; }
trap cleanup_all EXIT INT TERM

log() { printf '%s [p75] %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$*"; }

# ==============================================================================
# КАНОН (bash/coreutils, байт-в-байт против python-эталона)
# ==============================================================================

# canon_join_sha256 <file>
#   join "\n" + завершающий "\n" -> sha256 hex lowercase.
#   Пустой вход (0 строк) -> ровно один байт "\n" (пустой != null).
canon_join_sha256() {
  awk 'END{if(NR==0)printf "\n"} {sub(/\r$/,""); printf "%s\n",$0}' "$1" \
    | sha256sum | cut -d' ' -f1
}

# canon_uuid_multiset_sha256 <file-uuid-lines | tsv (uuid = колонка 1)>
#   extract col1 (uuid) -> LC_ALL=C sort БЕЗ -u (multiset, без дедупа) -> канон -> sha256.
canon_uuid_multiset_sha256() {
  LC_ALL=C cut -f1 "$1" | LC_ALL=C sort | canon_join_sha256 /dev/stdin
}

# canon_pos_digest_sha256 <tsv: uuid \t type \t x \t y \t z>
#   записи "<uuid>,%.3f,%.3f,%.3f" (LC_ALL=C: точка+libm), сорт полной строки,
#   канон-join -> sha256. Колонка type в дайджест НЕ входит (C04 §3).
canon_pos_digest_sha256() {
  LC_ALL=C awk -F'\t' '{sub(/\r$/,""); printf "%s,%.3f,%.3f,%.3f\n",$1,$3,$4,$5}' "$1" \
    | LC_ALL=C sort \
    | canon_join_sha256 /dev/stdin
}

# canon_counts_json <counts.tsv: type \t count>
#   -> JSON-объект {"type":N,...}, ключи LC_ALL=C sorted; пусто -> {}
canon_counts_json() {
  local body
  body="$(LC_ALL=C sort -t"$(printf '\t')" -k1,1 "$1" \
    | awk -F'\t' '{if(NR>1)printf ","; printf "\"%s\":%d",$1,$2}')"
  printf '{%s}' "$body"
}

# canon_sb_sha256 <tsv: objective \t holder \t score>
#   D7-канон (C04 ×489 §3): LC_ALL=C sort строк (канонизирует порядок NBT-списков,
#   инвариант к HashMap-итерации) -> canon-join -> sha256. present-но-пусто -> $H_EMPTY.
canon_sb_sha256() { LC_ALL=C sort "$1" | canon_join_sha256 /dev/stdin; }

# ==============================================================================
# NBT/MCA ЭКСТРАКТОР (python3 stdlib-only, embed; C04 §5 шаги 1+3)
# entities/*.mca (chunk NBT: Entities[]) -> TSV uuid\ttype\tx\ty\tz (repr-float
# round-trip) + counts.tsv + meta KV. level.dat -> Time/DataVersion.
# region-чанк-канон (C04 §5 шаг 2) — v2: МАТЕРИАЛИЗОВАН (ниже, байт-в-байт == арбитр).
# ==============================================================================
run_extractor() { # <world_dir> <tmpdir>
python3 - "$1" "$2" <<'PYEOF'
import gzip, hashlib, os, struct, sys, zlib
world, tmpd = sys.argv[1], sys.argv[2]

class R:
    __slots__=("b","i")
    def __init__(s,b): s.b=b; s.i=0
    def u8(s): v=s.b[s.i]; s.i+=1; return v
    def u16(s): v=struct.unpack_from(">H",s.b,s.i)[0]; s.i+=2; return v
    def i32(s): v=struct.unpack_from(">i",s.b,s.i)[0]; s.i+=4; return v
    def i64(s): v=struct.unpack_from(">q",s.b,s.i)[0]; s.i+=8; return v
    def f64(s): v=struct.unpack_from(">d",s.b,s.i)[0]; s.i+=8; return v
    def barr(s,n): v=s.b[s.i:s.i+n]; s.i+=n; return v
    def s(s): return s.barr(s.u16()).decode("utf-8","replace")

def pay(r,t):
    if t==1: return r.u8()
    if t==2: return struct.unpack(">h",r.barr(2))[0]
    if t==3: return r.i32()
    if t==4: return r.i64()
    if t==5: return struct.unpack(">f",r.barr(4))[0]
    if t==6: return r.f64()
    if t==7: return r.barr(r.i32())
    if t==8: return r.s()
    if t==9:
        et=r.u8(); n=r.i32(); return [pay(r,et) for _ in range(max(n,0))]
    if t==10:
        d={}
        while True:
            tt=r.u8()
            if tt==0: return d
            nm=r.s()          # имя ПЕРЕД payload (python d[k]=v вычисляет RHS первым!)
            d[nm]=pay(r,tt)
    if t==11: return [r.i32() for _ in range(max(r.i32(),0))]
    if t==12: return [r.i64() for _ in range(max(r.i32(),0))]
    raise ValueError("nbt tag %d" % t)

def nbt_root(buf):
    r=R(buf); t=r.u8()
    if t!=10: raise ValueError("root not compound")
    r.s(); return pay(r,10)

def java_uuid(a,b,c,d):
    hi=((a&0xFFFFFFFF)<<32)|(b&0xFFFFFFFF); lo=((c&0xFFFFFFFF)<<32)|(d&0xFFFFFFFF)
    return "%08x-%04x-%04x-%04x-%012x"%(hi>>32,(hi>>16)&0xFFFF,hi&0xFFFF,(lo>>48)&0xFFFF,lo&0xFFFFFFFFFFFF)

def scan_entities_mca(path):
    # AG-59 w527: рефактор в pure-function (returns lines,counters,cnt) — байт-в-байт
    # семантика прежнего serial-скана (частичные записи сохраняются, per-chunk try).
    lines=[]; d={"entities":0,"entity_chunks":0,"corrupt_chunks":0,"no_uuid":0,"no_pos":0}; c={}
    try:
        with open(path,"rb") as f: data=f.read()
        if len(data)<8192:
            d["corrupt_chunks"]+=1; return lines,d,c
        for idx in range(1024):
            b=data[idx*4:idx*4+3]
            off=(b[0]<<16)|(b[1]<<8)|b[2]
            if off==0: continue
            try:
                p=off*4096; ln=struct.unpack_from(">I",data,p)[0]
                ct=data[p+4]; blob=data[p+5:p+4+ln]
                raw=zlib.decompress(blob) if ct==2 else gzip.decompress(blob) if ct==1 else None
                if raw is None: raise ValueError("ctype %d"%ct)
                root=nbt_root(raw)
                ents=root.get("Entities")
                if not isinstance(ents,list): continue
                d["entity_chunks"]+=1
                for e in ents:
                    if not isinstance(e,dict): continue
                    et=e.get("id","minecraft:unknown")
                    u=e.get("UUID")
                    if isinstance(u,list) and len(u)==4:
                        us=java_uuid(*u)
                    elif "UUIDMost" in e and "UUIDLeast" in e:
                        us=java_uuid(0,0,e["UUIDMost"],e["UUIDLeast"])
                    else:
                        d["no_uuid"]+=1; continue
                    p3=e.get("Pos")
                    if not (isinstance(p3,list) and len(p3)==3): d["no_pos"]+=1; continue
                    lines.append("%s\t%s\t%r\t%r\t%r\n"%(us,et,p3[0],p3[1],p3[2]))
                    d["entities"]+=1; c[et]=c.get(et,0)+1
            except Exception:
                d["corrupt_chunks"]+=1
    except Exception:
        d["corrupt_chunks"]+=1
    return lines,d,c

def _p75_workers():
    # AG-59 w527: P75_JOBS=1 => serial; 0/absent => auto min(cpu,8); fail-open => 1
    try:
        import multiprocessing as _mp
        n=int(os.environ.get("P75_JOBS","0"))
        if n<=0: n=min(_mp.cpu_count() or 1,8)
        return max(1,n)
    except Exception:
        return 1

def _p75_map(fn, items):
    # fork-пул по файлам; порядок результатов == порядок items (byte-aligned merge).
    if len(items)>1 and _p75_workers()>1:
        try:
            from multiprocessing import get_context
            with get_context("fork").Pool(_p75_workers()) as pool:
                return pool.map(fn, items, chunksize=1)
        except Exception:
            pass
    return [fn(x) for x in items]

def _ent_worker(path):
    return scan_entities_mca(path)

tsv=open(os.path.join(tmpd,"entities.tsv"),"w")
cntf=open(os.path.join(tmpd,"counts.tsv"),"w")
cnt={}; st={"entities":0,"entity_chunks":0,"corrupt_chunks":0,"no_uuid":0,"no_pos":0}
ent_dir=os.path.join(world,"entities"); ent_files=[]
if os.path.isdir(ent_dir):
    ent_files=sorted(fn for fn in os.listdir(ent_dir) if fn.endswith(".mca"))
    for lines,d,c in _p75_map(_ent_worker,[os.path.join(ent_dir,fn) for fn in ent_files]):
        for ln in lines: tsv.write(ln)
        for k2,v2 in d.items(): st[k2]=st.get(k2,0)+v2
        for k2,v2 in c.items(): cnt[k2]=cnt.get(k2,0)+v2
for k in sorted(cnt): cntf.write("%s\t%d\n"%(k,cnt[k]))
tsv.close(); cntf.close()

lt="null"; dv="null"
ldat=os.path.join(world,"level.dat")
try:
    root=nbt_root(gzip.open(ldat,"rb").read())
    d=root.get("Data",{})
    lt=str(int(d.get("Time",0))); dv=str(int(d.get("DataVersion",0)))
except Exception:
    lt="null"; dv="null"
reg_dir=os.path.join(world,"region")
reg_n=sum(1 for _ in os.listdir(reg_dir)) if os.path.isdir(reg_dir) else 0
meta=[("level_time",lt),("data_version",dv),("entity_count",str(st["entities"])),
      ("entity_chunks",str(st["entity_chunks"])),("entities_files",str(len(ent_files))),
      ("region_files",str(reg_n)),("corrupt_chunks",str(st["corrupt_chunks"])),
      ("no_uuid",str(st["no_uuid"])),("no_pos",str(st["no_pos"])),
      ("entities_dir","1" if os.path.isdir(ent_dir) else "0")]
with open(os.path.join(tmpd,"meta.txt"),"w") as m:
    for k,v in meta: m.write("%s=%s\n"%(k,v))

# ---- v2 D4: region-канон, байт-в-байт == world_diff_parity_v2.py -------------
def palette_str(entry):
    if isinstance(entry, str): return entry
    if not isinstance(entry, dict): return str(entry)
    name = entry.get("Name", "?")
    props = entry.get("Properties")
    if isinstance(props, dict) and props:
        return name + "[" + ",".join(k + "=" + props[k] for k in sorted(props)) + "]"
    return name

def section_blocks(sec):
    y = int(sec.get("Y", 0))
    bs = sec.get("block_states")
    if not isinstance(bs, dict) or "palette" not in bs:
        return y, [AIR], None
    pal = [palette_str(e) for e in (bs.get("palette") or [])]
    data = bs.get("data")
    if isinstance(data, (bytes, bytearray)):
        data = list(struct.unpack(">%dq" % len(data), data))
    return y, pal, data

def canon_digest(pal, data):
    h = hashlib.sha256()
    h.update(("|".join(pal)).encode())
    for l in (data or []):
        h.update(struct.pack(">q", l))
    return h.hexdigest()

def region_iter(path, rx, rz):
    # == iter_region_chunks арбитра (yield (cx,cz,root)); raise на unsupported ctype/усечение
    with open(path, "rb") as f: raw = f.read()
    if len(raw) < 8192: return
    for i in range(1024):
        o = i * 4
        off_sectors = (raw[o] << 16) | (raw[o + 1] << 8) | raw[o + 2]
        cnt = raw[o + 3]
        if off_sectors == 0 or cnt == 0: continue
        start = off_sectors * 4096
        if start + 5 > len(raw): raise ValueError("chunk %d вне файла" % i)
        plen = struct.unpack_from(">I", raw, start)[0]
        comp = raw[start + 4]
        payload = raw[start + 5:start + 4 + plen]
        if comp == 1: payload = gzip.decompress(payload)
        elif comp == 2: payload = zlib.decompress(payload)
        elif comp != 3: raise ValueError("ctype %d (LZ4/Zstd)" % comp)
        root = nbt_root(payload)
        cx = int(root.get("xPos", rx * 32 + (i % 32)))
        cz = int(root.get("zPos", rz * 32 + (i // 32)))
        yield cx, cz, root

AIR = "minecraft:air"
world_map = {}  # (cx,cz) -> (status, blob "y:dig;"); LAST-WINS == арбитр dict
def scan_region_file(path, rx, rz):
    # AG-59 w527: per-file pure-function; partial-чанки до исключения СОХРАНЯЮТСЯ
    # (== serial try-вокруг-цикла), err=1 == serial reg_errors+=1
    out=[]; err=0
    try:
        for cx, cz, root in region_iter(path, rx, rz):
            status = str(root.get("Status", root.get("status", "?")))
            secs = {}
            for sec in (root.get("sections") or []):
                if isinstance(sec, dict):
                    y, pal, data = section_blocks(sec)
                    secs[y] = (pal, data)
            blob = "".join("%s:%s;" % (y, canon_digest(*secs[y])) for y in sorted(secs))
            out.append((cx, cz, status, blob))
    except Exception:
        err = 1
    return out, err

def _reg_worker(arg):
    fn, rx, rz = arg
    return scan_region_file(fn, rx, rz)

reg_dir = os.path.join(world, "region")
reg_files = len([f for f in os.listdir(reg_dir) if f.endswith(".mca")]) if os.path.isdir(reg_dir) else 0
reg_errors = 0
if reg_files:
    _reg_args=[]
    for fn in sorted(f for f in os.listdir(reg_dir) if f.endswith(".mca")):
        parts = fn.split(".")
        try: rx, rz = int(parts[1]), int(parts[2])
        except (IndexError, ValueError): rx = rz = 0
        _reg_args.append((os.path.join(reg_dir, fn), rx, rz))
    for _out, _err in _p75_map(_reg_worker, _reg_args):
        for cx, cz, status, blob in _out:
            world_map[(cx, cz)] = (status, blob)
        reg_errors += _err
reg_chunks = len(world_map)
with open(os.path.join(tmpd,"chunk_set.tsv"),"w") as fset, \
     open(os.path.join(tmpd,"chunk_sums.tsv"),"w") as fsum, \
     open(os.path.join(tmpd,"world.sha"),"w") as fw:
    if reg_errors == 0 and reg_files > 0:
        h = hashlib.sha256()
        for (cx, cz) in sorted(world_map):   # числовой tuple-сорт == арбитр sorted(S)
            status, blob = world_map[(cx, cz)]
            h.update(("%s,%s,%s;" % (cx, cz, status)).encode())
            h.update(blob.encode())
            fset.write("%s,%s,%s\n" % (cx, cz, status))
            fsum.write("%s,%s\t%s\n" % (cx, cz,
                hashlib.sha256(blob.replace(";", "").encode()).hexdigest()))
        fw.write(h.hexdigest())
reg_status = "ok" if (reg_errors == 0 and reg_files > 0) else \
             ("absent" if reg_errors == 0 else "error")

# ---- v2 D7: scoreboard.dat -> scoreboard.tsv (objective\tholder\tscore) ------
sb_status = "absent"
sbf = os.path.join(world, "data", "scoreboard.dat")
try:
    if os.path.isfile(sbf):
        sroot = nbt_root(gzip.open(sbf, "rb").read())
        sdata = sroot.get("Data", {})
        if not isinstance(sdata, dict): sdata = {}
        slines = []
        for sc in (sdata.get("PlayerScores") or []):
            if isinstance(sc, dict):
                slines.append("%s\t%s\t%s" % (str(sc.get("Objective", "?")),
                                              str(sc.get("Name", "?")), int(sc.get("Score", 0))))
        with open(os.path.join(tmpd,"scoreboard.tsv"),"w") as fsb:
            for ln in slines: fsb.write(ln + "\n")
        sb_status = "present"
except Exception:
    sb_status = "error"

with open(os.path.join(tmpd,"meta2.txt"),"w") as m2:
    m2.write("region_status=%s\nregion_files=%s\nregion_chunks=%s\nregion_errors=%s\nsb_status=%s\n" %
             (reg_status, reg_files, reg_chunks, reg_errors, sb_status))
PYEOF
}

# ==============================================================================
# fp.json — поля по C04 §1; v2: D4/D7 материализованы, per-chunk карта — в
# dp-parity-fp-region.json (C27 P1), main fp.json держит указатель+статусы.
# ==============================================================================
emit_fp_json() { # <work_dir> <uh> <ph> <counts_json> <meta> <selftest> <scan_s>
  local work="$1" uh="$2" ph="$3" cj="$4" metaf="$5" st="$6" ss="$7"
  local m k v level_time=null data_version=null ecount=0 echunks=0 efiles=0 rfiles=0 corr=0
  while IFS='=' read -r k v; do
    case "$k" in
      level_time) level_time="$v";; data_version) data_version="$v";;
      entity_count) ecount="$v";; entity_chunks) echunks="$v";;
      entities_files) efiles="$v";; region_files) rfiles="$v";;
      corrupt_chunks) corr="$v";;
    esac
  done < "$metaf"
  local dp_sha="none"
  if [ -f "$work/run-env.txt" ]; then
    dp_sha="$(grep -m1 'DP-INSTALLED sha256=' "$work/run-env.txt" 2>/dev/null \
              | sed 's/.*sha256=\([0-9a-f]\{4,\}\).*/\1/')"
  fi
  printf '%s' "$dp_sha" | grep -Eq '^[0-9a-f]{64}$' || dp_sha="none"
  # v2: аргументы 8-13 — D4/D7 поверхность
  local wsha="${8:-null}" rstat="${9:-absent}" rchunks="${10:-0}" rerr="${11:-0}"
  local sbsha="${12:-null}" sbstat="${13:-absent}"
  [ "$wsha" = "null" ] || printf '%s' "$wsha" | grep -Eq '^[0-9a-f]{64}$' || wsha="null"
  [ "$sbsha" = "null" ] || printf '%s' "$sbsha" | grep -Eq '^[0-9a-f]{64}$' || { sbsha="null"; sbstat="error"; }
  [ "$wsha" = "null" ] || wsha="\"$wsha\""    # JSON-строка вместо баre-хекса
  [ "$sbsha" = "null" ] || sbsha="\"$sbsha\""
  local tmp="$work/.p75.fp.json.tmp"
  cat > "$tmp" <<EOF
{"schema":"dp-parity-fp@1","fp_schema_version":1,
 "world_sha256":$wsha,
 "world_sha256_status":"$rstat",
 "dp_sha256":"$dp_sha",
 "chunk_set":[],"chunk_checksums":{},"chunk_checksums_status":"in:dp-parity-fp-region.json (D4 canon byte-aligned with world_diff_parity_v2.py; region_errors=$rerr)",
 "entity_uuid_multiset_sha256":"$uh",
 "entity_pos_digest_sha256":"$ph",
 "per_type_counts":$cj,
 "entity_count":$ecount,"entity_chunks":$echunks,"entities_files":$efiles,
 "region_files":$rfiles,"corrupt_chunks":$corr,"region_chunks":$rchunks,"region_errors":$rerr,
 "scoreboard_sha256":$sbsha,"scoreboard_sha256_status":"$sbstat",
 "level_time":"$level_time","data_version":"$data_version",
 "selftest":"$st","scan_seconds":$ss}
EOF
  # артефакт-бюджет ≤2MB (жёсткий пин)
  if [ "$(wc -c < "$tmp")" -gt 2097152 ]; then
    log "ERROR fp.json exceeds 2MB budget — replacing with error stub"
    printf '{"schema":"dp-parity-fp@1","fp_schema_version":1,"error":"fp.json>2MB"}\n' > "$tmp"
  fi
  mv -f "$tmp" "$work/dp-parity-fp.json"
}

# emit_region_json <work_dir> <tmpdir> — v2 D4 per-chunk артефакт (C27 P1), ≤2MB пин
emit_region_json() {
  local work="$1" t="$2" wsha rstat rchunks rerr body cset tmp
  wsha="$(cat "$t/world.sha" 2>/dev/null)"; [ -n "$wsha" ] || wsha=""
  local wj="null"; [ -n "$wsha" ] && wj="\"$wsha\""
  rstat="$(awk -F= '$1=="region_status"{print $2}' "$t/meta2.txt" 2>/dev/null)"
  rchunks="$(awk -F= '$1=="region_chunks"{print $2}' "$t/meta2.txt" 2>/dev/null)"
  rerr="$(awk -F= '$1=="region_errors"{print $2}' "$t/meta2.txt" 2>/dev/null)"
  body="$(awk -F'\t' '{printf "%s\"%s\":\"%s\"",(NR>1?",":""),$1,$2}' "$t/chunk_sums.tsv" 2>/dev/null)"
  cset="$(awk '{printf "%s\"%s\"",(NR>1?",":""),$0}' "$t/chunk_set.tsv" 2>/dev/null)"
  tmp="$work/.p75.fp-region.json.tmp"
  printf '{"schema":"dp-parity-fp-region@1","fp_schema_version":1,"world_sha256":%s,"region_status":"%s","region_chunks":%s,"region_errors":%s,"chunk_set":[%s],"chunk_checksums":{%s}}\n' \
    "$wj" "${rstat:-absent}" "${rchunks:-0}" "${rerr:-0}" "$cset" "$body" > "$tmp" 2>/dev/null || return 0
  if [ "$(wc -c < "$tmp")" -gt 2097152 ]; then
    log "ERROR dp-parity-fp-region.json exceeds 2MB budget — replacing with error stub"
    printf '{"schema":"dp-parity-fp-region@1","fp_schema_version":1,"error":"region.json>2MB"}\n' > "$tmp"
  fi
  mv -f "$tmp" "$work/dp-parity-fp-region.json" 2>/dev/null
  return 0
}

emit_error_json() { # <work_dir> <reason-alnum>
  local work="$1" reason="$2"
  mkdir -p "$work" 2>/dev/null || return 0
  local clean; clean="$(printf '%s' "$reason" | tr -c 'A-Za-z0-9_.=-' '_')"
  printf '{"schema":"dp-parity-fp@1","fp_schema_version":1,"error":"%s","selftest":"NOT-RUN"}\n' \
    "$clean" > "$work/.p75.fp.json.tmp" 2>/dev/null \
    && mv -f "$work/.p75.fp.json.tmp" "$work/dp-parity-fp.json" 2>/dev/null
  return 0
}

# ==============================================================================
# СКАН (fail-open: вызывающая обёртка scan_entry ВСЕГДА возвращает 0)
# ==============================================================================
main_scan() { # <world_dir> <work_dir>
  local world="$1" work="$2" t0 t1 ss TMPD   # TMPD локальный: вложенный main_scan (selftest pre-flight) не клобберает
  [ -d "$world" ] || { log "ERROR world_dir missing: $world"; return 1; }
  mkdir -p "$work" || { log "ERROR work_dir not creatable: $work"; return 1; }
  TMPD="$(mktemp -d "${TMPDIR:-/tmp}/p75scan.XXXXXXXX" 2>/dev/null)" \
    || TMPD="$(mktemp -d "$work/.p75scan.XXXXXXXX" 2>/dev/null)" || { log "ERROR mktemp failed"; return 1; }
  LAST_TMPD="$TMPD"   # снапшот для abort-safety через EXIT-trap
  t0=$(date +%s%3N 2>/dev/null || date +%s)

  # selftest pre-flight (fail-open: результат пишется в fp.json, скан продолжается)
  local st="PASS"
  if [ "${P75_SKIP_SELFTEST:-0}" != "1" ]; then
    if ! selftest_core "$TMPD/selftest" >/dev/null 2>&1; then
      st="FAIL"; log "WARN: selftest pre-flight FAIL — digests marked untrusted in fp.json"
    fi
  else st="SKIPPED"; fi

  run_extractor "$world" "$TMPD" || { log "ERROR extractor failed"; return 1; }
  local uh ph cj
  uh="$(canon_uuid_multiset_sha256 "$TMPD/entities.tsv")" || return 1
  ph="$(canon_pos_digest_sha256 "$TMPD/entities.tsv")" || return 1
  cj="$(canon_counts_json "$TMPD/counts.tsv")" || return 1
  # v2: D4/D7 поверхность (регион+scoreboard уже извлечены run_extractor'ом)
  local wsha rstat rchunks rerr sbsha sbstat k v
  wsha="null"; rstat="absent"; rchunks=0; rerr=0; sbsha="null"; sbstat="absent"
  while IFS='=' read -r k v; do
    case "$k" in
      region_status) rstat="$v";; region_chunks) rchunks="$v";;
      region_errors) rerr="$v";; sb_status) sbstat="$v";;
    esac
  done < "$TMPD/meta2.txt"
  [ -s "$TMPD/world.sha" ] && wsha="$(cat "$TMPD/world.sha")"
  [ "$wsha" = "null" ] || printf '%s' "$wsha" | grep -Eq '^[0-9a-f]{64}$' || wsha="null"
  [ "$rstat" = "ok" ] || [ "$wsha" = "null" ] || wsha="null"  # digest легален только при status=ok
  if [ "$sbstat" = "present" ]; then
    sbsha="$(canon_sb_sha256 "$TMPD/scoreboard.tsv")"
    printf '%s' "$sbsha" | grep -Eq '^[0-9a-f]{64}$' || { sbsha="null"; sbstat="error"; }
  fi
  t1=$(date +%s%3N 2>/dev/null || date +%s)
  ss="$(awk -v a="$t0" -v b="$t1" 'BEGIN{printf "%.3f",(b-a)/1000}')"
  emit_fp_json "$work" "$uh" "$ph" "$cj" "$TMPD/meta.txt" "$st" "$ss" \
    "$wsha" "$rstat" "$rchunks" "$rerr" "$sbsha" "$sbstat" || return 1
  emit_region_json "$work" "$TMPD" || true
  log "DP-PARITY-FP: OK uuid_multiset_sha256=$uh pos_digest_sha256=$ph entities=$(awk -F= '$1=="entity_count"{print $2}' "$TMPD/meta.txt") level_time=$(awk -F= '$1=="level_time"{print $2}' "$TMPD/meta.txt") scan_s=$ss selftest=$st"
  log "DP-PARITY-FP: v2 D4/D7 world_sha256=$wsha ($rstat) chunks=$rchunks region_errors=$rerr scoreboard=$sbsha ($sbstat)"
  rm -rf "$TMPD" 2>/dev/null; LAST_TMPD=""
  return 0
}

scan_entry() { # fail-open обёртка
  local world="$1" work="$2" rc
  if [ -z "$world" ] || [ -z "$work" ]; then
    log "P75 usage: parity_phase75.sh <world_dir> <work_dir> | --selftest [tmpdir]"
    [ -n "$work" ] && emit_error_json "$work" "usage"
    return 0
  fi
  main_scan "$world" "$work"; rc=$?
  if [ "$rc" -ne 0 ]; then
    log "DP-PARITY-FP: FAIL-OPEN reason=main_scan_rc=$rc (anti-false-REFUTED: error-fp.json => UNKNOWN, не парити-FAIL)"
    emit_error_json "$work" "main_scan_rc=$rc"
  fi
  return 0
}

# ==============================================================================
# САМОТЕСТ (10 кейсов; фикстуры генерируются in-situ: T-канон на строках +
# END-TO-END на реальных mca-байтах). Fail-closed: возвращает 1 при любом FAIL.
# ==============================================================================
build_fixture_worlds() { # <tmpdir> — python-энкодер мини-миров (base + clone-мутант)
python3 - "$1" <<'PYEOF'
import gzip, os, struct, sys, zlib
T=sys.argv[1]
U1="00000000-0000-0000-0000-000000000001"; U2="0f0e0d0c-0b0a-4321-8765-ba0987654321"
def ints(u):
    h=u.replace('-','')
    return [ (int(h[i:i+8],16)-2**32 if int(h[i:i+8],16)>=2**31 else int(h[i:i+8],16)) for i in (0,8,16,24) ]
def tg(i,name,pl):
    nb=name.encode(); return bytes([i])+struct.pack(">H",len(nb))+nb+pl
def ctag(name,pl):
    # compound = tag10 + name + payload + End(0x00) — декодер читает до End
    nb=name.encode(); return bytes([10])+struct.pack(">H",len(nb))+nb+pl+b"\x00"
def tstr(v):
    b=v.encode(); return struct.pack(">H",len(b))+b
def tint(v): return struct.pack(">i",v)
def tlong(v): return struct.pack(">q",v)
def tdbl(v): return struct.pack(">d",v)
def tintarr(xs): return struct.pack(">i",len(xs))+b"".join(struct.pack(">i",x) for x in xs)
def tlist(et,payloads): return bytes([et])+struct.pack(">i",len(payloads))+b"".join(payloads)
def ent(uid,typ,pos):
    # NBT-спека: элемент списка-компаундов = ГОЛЫЙ payload (дети+End) БЕЗ tag-байта и имени
    return (tg(8,"id",tstr(typ))+tg(11,"UUID",tintarr(ints(uid)))+
            tg(9,"Pos",tlist(6,[tdbl(pos[0]),tdbl(pos[1]),tdbl(pos[2])]))+b"\x00")
def chunk_root(ents):
    return ctag("", tg(3,"DataVersion",tint(4440))+
              tg(9,"Position",tlist(3,[tint(0),tint(0)]))+
              tg(9,"Entities",tlist(10,ents)))
def write_mca(path, ents):
    payload=zlib.compress(chunk_root(ents))
    # спецификация region: length = байты ПОСЛЕ 4B-поля = 1(ctype)+payload
    rec=struct.pack(">I",len(payload)+1)+bytes([2])+payload
    sectors=(len(rec)+4095)//4096
    hdr=bytearray(8192); hdr[0:3]=b"\x00\x00\x02"; hdr[3]=sectors
    hdr[4096:4100]=struct.pack(">I",1)
    with open(path,"wb") as f:
        f.write(hdr); f.write(rec); f.write(b"\x00"*(sectors*4096-len(rec)))
def write_ldat(path):
    root=ctag("", ctag("Data", tg(4,"Time",tlong(133700))+tg(3,"DataVersion",tint(4440))))
    open(path,"wb").write(gzip.compress(root))
POS_U1=(0.0005,64.0,-0.0004); POS_U2=(100.1235,65.5,2000.9995)
for wname,ents in (("world",[ent(U1,"minecraft:item",POS_U1), ent(U2,"minecraft:zombie",POS_U2)]),
                   ("worldc",[ent(U1,"minecraft:item",POS_U1), ent(U2,"minecraft:zombie",POS_U2),
                              ent(U2,"minecraft:zombie",POS_U2)])):
    wd=os.path.join(T,wname); os.makedirs(os.path.join(wd,"entities"),exist_ok=True)
    write_mca(os.path.join(wd,"entities","r.0.0.mca"),ents); write_ldat(os.path.join(wd,"level.dat"))

# ---- v2 fixture-расширение: region/*.mca (D4) + data/scoreboard.dat (D7) -----
def tlongarr(xs): return struct.pack(">i",len(xs))+b"".join(struct.pack(">q",x) for x in xs)
def sect(y,pal,data):
    pl=b""
    if pal is not None:
        pl+=tg(9,"palette",tlist(8,[tstr(p) for p in pal]))
        if data: pl+=tg(12,"data",tlongarr(data))
    return tg(3,"Y",tint(y))+ctag("block_states",pl)+b"\x00"
def rchunk(x,z,status,secs):
    return ctag("", tg(3,"DataVersion",tint(4440))+tg(3,"xPos",tint(x))+tg(3,"zPos",tint(z))+
              tg(8,"Status",tstr(status))+tg(9,"sections",tlist(10,secs)))
def write_region_mca(path, items):
    recs=[]
    for slot,payload in items:
        comp=zlib.compress(payload); rec=struct.pack(">I",len(comp)+1)+bytes([2])+comp
        recs.append((slot,rec))
    hdr=bytearray(8192); off=2
    for slot,rec in recs:
        sectors=(len(rec)+4095)//4096
        hdr[slot*4:slot*4+3]=off.to_bytes(3,"big"); hdr[slot*4+3]=sectors
        off+=sectors
    with open(path,"wb") as f:
        f.write(hdr)
        for slot,rec in recs:
            sectors=(len(rec)+4095)//4096
            f.write(rec); f.write(b"\x00"*(sectors*4096-len(rec)))
def sbdat(scores):
    objs=[tg(8,"Name",tstr("obj_a"))+tg(8,"Criteria",tstr("dummy"))+b"\x00",
          tg(8,"Name",tstr("obj_b"))+tg(8,"Criteria",tstr("dummy"))+b"\x00"]
    pls=[tg(8,"Name",tstr(h))+tg(8,"Objective",tstr(o))+tg(3,"Score",tint(v))+b"\x00"
         for (o,h,v) in scores]
    return gzip.compress(ctag("", ctag("Data", tg(9,"Objectives",tlist(10,objs))+
                                       tg(9,"PlayerScores",tlist(10,pls)))))
CHUNK_A=(0,0,"minecraft:full",[sect(-1,None,None),
         sect(0,["minecraft:stone","minecraft:air"],[305419896,-305419897])])
CHUNK_B =(5,3,"minecraft:full",[sect(0,["minecraft:dirt"],None)])
CHUNK_BM=(5,3,"minecraft:full",[sect(0,["minecraft:coarse_dirt"],None)])
SB_BASE=[("obj_b","Carol",7),("obj_a","Alice",5),("obj_a","Bob",3)]  # вставка НЕсортированная
SB_MUT =[("obj_b","Carol",7),("obj_a","Alice",5),("obj_a","Bob",4)]
for wname,cb,sb,with_sb in (("worldR",CHUNK_B,SB_BASE,True),
                            ("worldR-mut",CHUNK_BM,SB_BASE,True),
                            ("worldR-sbmut",CHUNK_B,SB_MUT,True),
                            ("worldR-nosb",CHUNK_B,SB_BASE,False)):
    wd=os.path.join(T,wname); os.makedirs(os.path.join(wd,"region"),exist_ok=True)
    write_region_mca(os.path.join(wd,"region","r.0.0.mca"),[(0,rchunk(*CHUNK_A)),(101,rchunk(*cb))])
    write_ldat(os.path.join(wd,"level.dat"))
    if with_sb:
        os.makedirs(os.path.join(wd,"data"),exist_ok=True)
        open(os.path.join(wd,"data","scoreboard.dat"),"wb").write(sbdat(sb))
PYEOF
}

selftest_core() { # <tmpdir>; печатает per-case строки; rc=0 iff 0 FAIL
  local T="$1" fails=0 t0 t1
  t0=$(date +%s%3N 2>/dev/null || date +%s)
  mkdir -p "$T" || return 1
  # T-фикстура-строки (порядок вставки ≠ ascii-сорт — канон нормализует)
  printf '%s\n%s\n%s\n%s\n' "$U3" "$U1" "$U4" "$U2" > "$T/uuid.base"
  printf '%s\n%s\n%s\n%s\n%s\n' "$U3" "$U1" "$U2" "$U4" "$U2" > "$T/uuid.clone"   # клон U2
  printf '%s\n%s\n%s\n%s\n' "$U4" "$U2" "$U3" "$U1" > "$T/uuid.reorder"           # пере-порядок
  printf '%s\n%s\n%s\n' "$U3" "$U1" "$U2" > "$T/uuid.remove"                      # удаление U4
  { printf '%s\tminecraft:item\t%s\t%s\t%s\n' "$U1" 0.0005 64.0 -0.0004
    printf '%s\tminecraft:zombie\t%s\t%s\t%s\n' "$U2" 100.1235 65.5 2000.9995
    printf '%s\tminecraft:villager\t%s\t%s\t%s\n' "$U3" -12345.6789 0.0 32000.0
    printf '%s\tminecraft:husk\t%s\t%s\t%s\n' "$U4" -0.0005 29999999.0 1.05; } > "$T/pos.base"
  cp "$T/pos.base" "$T/pos.clone"
  printf '%s\tminecraft:zombie\t%s\t%s\t%s\n' "$U2" 100.1235 65.5 2000.9995 >> "$T/pos.clone"  # clone->pos (5 записей)

  local h
  h="$(canon_uuid_multiset_sha256 "$T/uuid.base")"
  [ "$h" = "$H_BASE" ] && echo "P75-SELFTEST S1 PASS uuid-identity=$h" || { echo "P75-SELFTEST S1 FAIL uuid-identity got=$h want=$H_BASE"; fails=$((fails+1)); }
  h="$(canon_uuid_multiset_sha256 "$T/uuid.clone")"
  [ "$h" = "$H_CLONE" ] && [ "$h" != "$H_BASE" ] && echo "P75-SELFTEST S2 PASS clone-caught=$h" || { echo "P75-SELFTEST S2 FAIL clone got=$h want=$H_CLONE"; fails=$((fails+1)); }
  h="$(canon_uuid_multiset_sha256 "$T/uuid.reorder")"
  [ "$h" = "$H_BASE" ] && echo "P75-SELFTEST S3 PASS order-invariant" || { echo "P75-SELFTEST S3 FAIL reorder got=$h want=$H_BASE"; fails=$((fails+1)); }
  h="$(canon_uuid_multiset_sha256 "$T/uuid.remove")"
  [ "$h" = "$H_REMOVE" ] && [ "$h" != "$H_BASE" ] && echo "P75-SELFTEST S4 PASS removal-caught=$h" || { echo "P75-SELFTEST S4 FAIL removal got=$h want=$H_REMOVE"; fails=$((fails+1)); }
  printf '' > "$T/uuid.empty"
  h="$(canon_uuid_multiset_sha256 "$T/uuid.empty")"
  [ "$h" = "$H_EMPTY" ] && echo "P75-SELFTEST S5 PASS empty=sha256(0x0A)=$h (C04-const correction)" || { echo "P75-SELFTEST S5 FAIL empty got=$h want=$H_EMPTY"; fails=$((fails+1)); }
  h="$(canon_pos_digest_sha256 "$T/pos.base")"
  [ "$h" = "$H_POS_BASE" ] && echo "P75-SELFTEST S6 PASS pos-identity=$h" || { echo "P75-SELFTEST S6 FAIL pos-identity got=$h want=$H_POS_BASE"; fails=$((fails+1)); }
  h="$(LC_ALL=ru_RU.UTF-8 LC_NUMERIC=ru_RU.UTF-8 LANG=ru_RU.UTF-8 canon_pos_digest_sha256 "$T/pos.base" 2>/dev/null)"
  [ "$h" = "$H_POS_BASE" ] && echo "P75-SELFTEST S7 PASS hostile-locale pinned (ru_RU == C)" || { echo "P75-SELFTEST S7 FAIL hostile-locale got=$h want=$H_POS_BASE"; fails=$((fails+1)); }
  h="$(canon_pos_digest_sha256 "$T/pos.clone")"  # 5 записей (U2 dup) — pos-клон
  [ "$h" = "$H_POS_CLONE" ] && echo "P75-SELFTEST S7b PASS pos-clone-caught=$h" || { echo "P75-SELFTEST S7b FAIL pos-clone got=$h want=$H_POS_CLONE"; fails=$((fails+1)); }

  # E2E: реальные mca-байты (энкодер) -> extractor -> canon
  build_fixture_worlds "$T" || { echo "P75-SELFTEST S8 FAIL fixture-build"; return 1; }
  local wa="$T/wA" wb="$T/wB" wfail="$T/wFAIL"
  P75_SKIP_SELFTEST=1 main_scan "$T/world" "$wa" >/dev/null 2>&1 \
    && P75_SKIP_SELFTEST=1 main_scan "$T/worldc" "$wb" >/dev/null 2>&1 \
    || { echo "P75-SELFTEST S8 FAIL e2e-scan"; fails=$((fails+1)); }
  if python3 - "$wa" "$wb" <<'PYCHK'
import json,sys,os
wa,wb=sys.argv[1],sys.argv[2]
a=json.load(open(os.path.join(wa,"dp-parity-fp.json")))
b=json.load(open(os.path.join(wb,"dp-parity-fp.json")))
H2="33ef09792dca659a30ba0fc05cf8ded8c28983e7b299606e98c4d437204830b1"
H2C="f24c1af331a23fcd54b22be136b54bcf89bd707113de271696d53b68cd7f83e1"
P2="6fe33a9757578c50e3a2266828eb74b126b2d446f4cf0d26d42f66cb4fb3fa1c"
P2C="e7fa219f5e25e928c0e00a8c7c65b2edb16916d7dc15e5b64a450a17a9678805"
assert a["entity_uuid_multiset_sha256"]==H2, a
assert a["entity_pos_digest_sha256"]==P2, a
assert a["level_time"]=="133700" and a["data_version"]=="4440", a
assert a["per_type_counts"]=={"minecraft:item":1,"minecraft:zombie":1}, a
assert a["entity_count"]==2 and a["selftest"]=="SKIPPED", a
assert b["entity_uuid_multiset_sha256"]==H2C!=H2, b   # клон пойман на реальных байтах
assert b["entity_pos_digest_sha256"]==P2C!=P2, b
assert b["per_type_counts"]=={"minecraft:item":1,"minecraft:zombie":2}, b
print("P75-SELFTEST S8S9-JSON-OK")
PYCHK
  then echo "P75-SELFTEST S8 PASS e2e-mca-base (uuid==H2 pos==P2 counts/level_time OK)"; \
       echo "P75-SELFTEST S9 PASS e2e-mca-clone-mutant (H2C!=H2, P2C!=P2 — RNG-клон ловится на mca-байтах)"
  else echo "P75-SELFTEST S8/S9 FAIL e2e-mca"; fails=$((fails+1)); fi
  # S10 fail-open: несуществующий мир -> exit 0 + error-fp.json
  P75_SKIP_SELFTEST=1 scan_entry "$T/does-not-exist" "$wfail" >/dev/null 2>&1
  local s10rc=$?
  if [ "$s10rc" -eq 0 ] && python3 -c 'import json,sys; d=json.load(open(sys.argv[1]+"/dp-parity-fp.json")); assert "error" in d' "$wfail" 2>/dev/null; then
    echo "P75-SELFTEST S10 PASS fail-open (rc=0 + error-fp.json)"
  else echo "P75-SELFTEST S10 FAIL fail-open rc=$s10rc"; fails=$((fails+1)); fi

  # T4: D4 e2e на реальных region-байтах; константы вычислены САМИМ арбитром
  # (world_diff_parity_v2.py scan_region_dir/_side_digest/_canon_digest) — byte-align proof
  local w4="$T/wR4" w4m="$T/wR4m"
  P75_SKIP_SELFTEST=1 main_scan "$T/worldR" "$w4" >/dev/null 2>&1 \
    && P75_SKIP_SELFTEST=1 main_scan "$T/worldR-mut" "$w4m" >/dev/null 2>&1 \
    || { echo "P75-SELFTEST T4 FAIL e2e-region-scan"; fails=$((fails+1)); }
  if python3 - "$w4" "$w4m" <<'PYCHK4'
import json, os, sys
w4, w4m = sys.argv[1], sys.argv[2]
R  = "1d7043335a473695bcea1ea0e891ca474c26283bca9c5aafabba614020515150"
RM = "772f3110743fce3e7083a3795dd3a22a1d11fb59a796cac839f52bdfb9d623cc"
CK53  = "e027d24052e08465b2bcee63ee0e1f3823f5f2916d63dece73306adf6ca42cef"
CK53M = "e61fb075ef577ca0591a68bcfed64a045681c8ce88bc9c91331384c81fa4151a"
a = json.load(open(os.path.join(w4,  "dp-parity-fp-region.json")))
b = json.load(open(os.path.join(w4m, "dp-parity-fp-region.json")))
fa = json.load(open(os.path.join(w4, "dp-parity-fp.json")))
assert a["world_sha256"] == R and a["region_status"] == "ok" and a["region_chunks"] == 2, a
assert a["chunk_checksums"]["5,3"] == CK53 and len(a["chunk_checksums"]["0,0"]) == 64, a
assert a["chunk_set"] == ["0,0,minecraft:full", "5,3,minecraft:full"], a
assert b["world_sha256"] == RM != R, b
assert b["chunk_checksums"]["5,3"] == CK53M != CK53, b   # чанк-мутант ловится
assert fa["world_sha256"] == R and fa["region_errors"] == 0, fa
print("P75-SELFTEST-T4-JSON-OK")
PYCHK4
  then echo "P75-SELFTEST T4 PASS d4-region-arbiter-aligned (world==H_RBASE(=арбитр), чанк-мутант ловится)"; \
  else echo "P75-SELFTEST T4 FAIL d4-region"; fails=$((fails+1)); fi

  # T5: D7 scoreboard present/absent/mutant (канон C04 ×489 §3: sorted objective,holder,score)
  local w5="$T/wR5" w5n="$T/wR5n" w5m="$T/wR5m"
  P75_SKIP_SELFTEST=1 main_scan "$T/worldR" "$w5" >/dev/null 2>&1 \
    && P75_SKIP_SELFTEST=1 main_scan "$T/worldR-nosb" "$w5n" >/dev/null 2>&1 \
    && P75_SKIP_SELFTEST=1 main_scan "$T/worldR-sbmut" "$w5m" >/dev/null 2>&1 \
    || { echo "P75-SELFTEST T5 FAIL e2e-sb-scan"; fails=$((fails+1)); }
  if python3 - "$w5" "$w5n" "$w5m" <<'PYCHK5'
import json, os, sys
SB  = "61c080e881efc00c4446b6569062f69fa38583d613bacd587b7e2ce9dac2a8fd"
SBM = "66370ba404e9a83dff9302d800214741e8f95cb10bf2414dfcfaafad0e21acb9"
a = json.load(open(os.path.join(sys.argv[1], "dp-parity-fp.json")))
n = json.load(open(os.path.join(sys.argv[2], "dp-parity-fp.json")))
m = json.load(open(os.path.join(sys.argv[3], "dp-parity-fp.json")))
assert a["scoreboard_sha256"] == SB and a["scoreboard_sha256_status"] == "present", a
assert n["scoreboard_sha256"] is None and n["scoreboard_sha256_status"] == "absent", n
assert m["scoreboard_sha256"] == SBM != SB and m["scoreboard_sha256_status"] == "present", m
print("P75-SELFTEST-T5-JSON-OK")
PYCHK5
  then echo "P75-SELFTEST T5 PASS d7-scoreboard (present==H_SB, absent==absent->SKIP, мутант ловится)"; \
  else echo "P75-SELFTEST T5 FAIL d7-scoreboard"; fails=$((fails+1)); fi

  t1=$(date +%s%3N 2>/dev/null || date +%s)
  local ms=$(( t1 - t0 ))
  P75_ST_FAILS=$fails; P75_ST_MS=$ms
  return $([ "$fails" -eq 0 ] && echo 0 || echo 1)
}

selftest_main() { # fail-closed режим
  SELFTEST_TMPD="${1:-$(mktemp -d "${TMPDIR:-/tmp}/p75selftest.XXXXXXXX")}"
  echo "P75-SELFTEST: start tmp=$SELFTEST_TMPD (13 cases, preregistered constants, python-hashlib+arbiter authority)"
  selftest_core "$SELFTEST_TMPD"; local rc=$?
  if [ "$rc" -eq 0 ]; then
    echo "P75-SELFTEST: PASS cases=13/13 fails=0 wall=${P75_ST_MS}ms"
  else
    echo "P75-SELFTEST: FAIL fails=$P75_ST_FAILS wall=${P75_ST_MS}ms"
  fi
  return $rc
}

# ==============================================================================
# ВХОД
# ==============================================================================
case "${1:-}" in
  --selftest) shift; selftest_main "$@"; exit $? ;;
esac
scan_entry "${1:-}" "${2:-}"
exit 0
