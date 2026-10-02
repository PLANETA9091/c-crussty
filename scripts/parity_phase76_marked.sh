#!/usr/bin/env bash
# =============================================================================
# parity_phase76_marked.sh — CANARY-4 SCOPE-SPEC (Л-492-C73, ×493; ×495 derived-fix)
# D4 region-плоскость скоупится к marked-набору:
#   (a) world/data/forcedload.dat (ForcedChunksSavedData) когда флашен; ЛИБО
#   (b) DERIVED-набор (×495 Л-494-CANARY4): kill-9-стоп не флашит SavedData →
#       marked = геометрия forceload-свипа харнесса (run_world3.sh §5):
#       TILES=ceil(R/256), chunk-квадрат [-TILES*16 .. TILES*16-1]²,
#       R=BENCH_FORCELOAD_RADIUS (дефолт 640 → 96×96 = 9216 чанков).
#       fl_status="derived-r<R>", scope="marked-derived-<n>". Детерминировано и
#       пар-сравнимо (обе ноги одной конфигурации → один и тот же набор).
# Мотивация: v2 O(all-chunks) = 62,604×~9.4ms ≈ 588s → timeout 90/600 + 2MB-лов
#   = UNKNOWN ×3 (canary-2v2/v3); marked-9216-only ≈ 87s / 0.94MB — единственный
#   вариант со всеми 4 дайджестами внутри 600s-бюджета.
# FAIL-OPEN: derived-набор не полностью покрыт region-сканом → mismatch →
#   world_sha256=null → fp.json = UNKNOWN (НЕ parity-FAIL). Ложный PASS невозможен.
# SKIP-ОПТИМИЗАЦИЯ (×495): chunk-координаты из slot-индекса (rx*32+i%32, rz*32+i//32)
#   ДО декомпрессии — декомпрессия только marked-слотов (9216 из ~595k ≈ 10-15s).
#   Слот вне marked не декомпрессируется (коррапт вне скоупа D4-плоскости —
#   детерминизм пары сохранён); slot↔xPos-расхождение → covered-гейт ловит →
#   mismatch (fail-open, 0 ложных PASS).
# Сравнимость: обе ноги canary-4 сканируют один и тот же marked-набор (ok ИЛИ
#   derived, ОДИН класс scope) → world.sha бит-сравним; scope_mismatch/класс-разница
#   на ОДНОЙ ноге = UNKNOWN пары.
# Самотест: bash parity_phase76_marked.sh --selftest <tmpdir> (fail-closed, exit 1)
#   Константы preregistered независимо (python3 hashlib, НЕ код сканера):
#   WORLD_SHA_MARKED 65355c14… (фикстура CHUNK_A@(0,0), mark {(0,0)}),
#   H2/P2 = v2-канон entity-плоскости (33ef0979…/6fe33a97…),
#   WORLD_SHA_DERIVED (×495 M12, worldD: 1024 чанков [-16..15]² stone/air-секта).
# =============================================================================
set -u
H_EMPTY="01ba4719c80b6fe911b091a7c058ef8f9805daca546b0"  # placeholder, переопределяется ниже
H_EMPTY="01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b"  # sha256("\n")
# preregistered selftest-константы (независимая реализация, см. шапку)
ST_H_ENT="33ef09792dca659a30ba0fc05cf8ded8c28983e7b299606e98c4d437204830b1"
ST_P_ENT="6fe33a9757578c50e3a2266828eb74b126b2d446f4cf0d26d42f66cb4fb3fa1c"
ST_COUNTS='{"minecraft:item":1,"minecraft:zombie":1}'
ST_WORLD_SHA="65355c14f5ea499c7c54d0f73579ce5a618df3b57365bed640df1b642658f187"
ST_PER_SUM="12297eb904ccd670491a7d660940fea268e4ec25cacafd883ecd9c71544dafbb"
ST_SET_SHA="34cd703e5cf56217444e7e5956c66b833cdafb45b6dc766d9ac5a54a5242e819"
# ×495 derived-fix константы (M12: R=256 → [-16..15]², 1024 чанков, все full/stone-air)
ST_D_R="256"
ST_D_MARKED_N="1024"
ST_D_WORLD_SHA="4cd5c59e901b77313ad0590fd6f4256e79d36911284f215375da03f6c1e64bfa"

log() { echo "[p76] $*"; }

# --- канон-дайджесты (байт-в-байт v2 C04/×491-арбитр) ------------------------
canon_join_sha256() {
  awk 'END{if(NR==0)printf "\n"} {sub(/\r$/,""); printf "%s\n",$0}' "$1" \
    | sha256sum | cut -d' ' -f1
}
canon_uuid_multiset_sha256() {
  LC_ALL=C cut -f1 "$1" | LC_ALL=C sort | canon_join_sha256 /dev/stdin
}
canon_pos_digest_sha256() {
  LC_ALL=C awk -F'\t' '{sub(/\r$/,""); printf "%s,%.3f,%.3f,%.3f\n",$1,$3,$4,$5}' "$1" \
    | LC_ALL=C sort | canon_join_sha256 /dev/stdin
}
canon_counts_json() {
  local body
  body="$(LC_ALL=C sort -t"$(printf '\t')" -k1,1 "$1" \
    | awk -F'\t' '{if(NR>1)printf ","; printf "\"%s\":%d",$1,$2}')"
  printf '{%s}' "$body"
}

# --- extract/scan (python3 stdlib-only, embed) -------------------------------
run_scan() { # <world_dir> <tmpdir>
  mkdir -p "$2" || return 2
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
            nm=r.s(); d[nm]=pay(r,tt)
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

# --- entities (плоскость full-scope как v2; дешёвая, ~15s на 150k) -----------
def scan_entities_mca(path, out, st):
    with open(path,"rb") as f: data=f.read()
    if len(data)<8192: st["corrupt_chunks"]+=1; return
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
            st["entity_chunks"]+=1
            for e in ents:
                if not isinstance(e,dict): continue
                et=e.get("id","minecraft:unknown")
                u=e.get("UUID")
                if isinstance(u,list) and len(u)==4:
                    us=java_uuid(*u)
                elif "UUIDMost" in e and "UUIDLeast" in e:
                    us=java_uuid(0,0,e["UUIDMost"],e["UUIDLeast"])
                else:
                    st["no_uuid"]+=1; continue
                p3=e.get("Pos")
                if not (isinstance(p3,list) and len(p3)==3): st["no_pos"]+=1; continue
                out.write("%s\t%s\t%r\t%r\t%r\n"%(us,et,p3[0],p3[1],p3[2]))
                st["entities"]+=1; cnt[et]=cnt.get(et,0)+1
        except Exception:
            st["corrupt_chunks"]+=1

tsv=open(os.path.join(tmpd,"entities.tsv"),"w")
cntf=open(os.path.join(tmpd,"counts.tsv"),"w")
cnt={}; st={"entities":0,"entity_chunks":0,"corrupt_chunks":0,"no_uuid":0,"no_pos":0}
ent_dir=os.path.join(world,"entities"); ent_files=[]
if os.path.isdir(ent_dir):
    ent_files=sorted(fn for fn in os.listdir(ent_dir) if fn.endswith(".mca"))
    for fn in ent_files: scan_entities_mca(os.path.join(ent_dir,fn), tsv, st)
tsv.close()
for k in sorted(cnt): cntf.write("%s\t%d\n"%(k,cnt[k]))
cntf.close()

lt="null"; dv="null"
ldat=os.path.join(world,"level.dat")
try:
    root=nbt_root(gzip.open(ldat,"rb").read())
    d=root.get("Data",{})
    lt=str(int(d.get("Time",0))); dv=str(int(d.get("DataVersion",0)))
except Exception:
    lt="null"; dv="null"

# --- marked-набор: forcedload.dat (ForcedChunksSavedData) --------------------
# root{data{ForcedChunks{<dim>: TAG_Long_Array packed}}}
# ChunkPos.asLong: l = (z & 0xFFFFFFFF) << 32 | (x & 0xFFFFFFFF);
# unpack: cz = l >> 32 (sign), cx = l & 0xFFFFFFFF (sign-ify).
# Любой дефект структуры → mismatch.
def parse_marked(path):
    root=nbt_root(gzip.open(path,"rb").read())
    data=root.get("data")
    if not isinstance(data,dict): raise ValueError("no data compound")
    fc=data.get("ForcedChunks")
    if not isinstance(fc,dict): raise ValueError("no ForcedChunks compound")
    dims=0; marked=set()
    for dim,arr in fc.items():
        if not isinstance(arr,list): raise ValueError("dim not long-array: %r"%dim)
        if len(arr)==0: continue
        dims+=1
        for l in arr:
            if not isinstance(l,int): raise ValueError("element not long")
            cz=l>>32; cx=l&0xFFFFFFFF
            if cx>=2**31: cx-=2**32
            marked.add((cx,cz))
    if dims==0: raise ValueError("ForcedChunks empty")
    return marked

fl_path=os.path.join(world,"data","forcedload.dat")
marked=None; fl_status="absent"
if os.path.isfile(fl_path):
    try:
        marked=parse_marked(fl_path); fl_status="ok"
    except Exception:
        marked=None; fl_status="error"
else:
    fl_status="absent"

# --- ×495 derived-set fallback (Л-494-CANARY4) -------------------------------
# kill-9-стоп харнесса не флашит SavedData → forcedload.dat отсутствует/пуст.
# marked = геометрия forceload-свипа run_world3.sh §5 (детерминизм конфига):
#   STEP=256, TILES=ceil(R/256), forceload add tx tz tx+255 tz+255 для
#   tx,tz ∈ {-TILES*256 .. (TILES-1)*256 step 256} → chunk-квадрат
#   [-TILES*16 .. TILES*16-1]². Обе ноги одной конфигурации → один набор.
if marked is None:
    # ВНИМАНИЕ: имена _flr/_flt — НЕ R (R = класс NBT-ридера выше; shadowing =
    # TypeError 'int' object is not callable — пойман selftest-ом M12 ×495)
    try:
        _flr=int(os.environ.get("BENCH_FORCELOAD_RADIUS","") or 640)
    except Exception:
        _flr=640
    if _flr<=0: _flr=640
    _flt=(_flr+255)//256
    _lo=-_flt*16; _hi=_flt*16
    marked=set((cx,cz) for cx in range(_lo,_hi) for cz in range(_lo,_hi))
    fl_status="derived-r%d"%_flr

# --- region marked-scope scan (D4) ------------------------------------------
AIR="minecraft:air"
def palette_str(entry):
    if isinstance(entry,str): return entry
    if not isinstance(entry,dict): return str(entry)
    name=entry.get("Name","?")
    props=entry.get("Properties")
    if isinstance(props,dict) and props:
        return name+"["+",".join(k+"="+props[k] for k in sorted(props))+"]"
    return name

def section_blocks(sec):
    y=int(sec.get("Y",0))
    bs=sec.get("block_states")
    if not isinstance(bs,dict) or "palette" not in bs:
        return y,[AIR],None
    pal=[palette_str(e) for e in (bs.get("palette") or [])]
    data=bs.get("data")
    if isinstance(data,(bytes,bytearray)):
        data=list(struct.unpack(">%dq"%len(data),data))
    return y,pal,data

def canon_digest(pal,data):
    h=hashlib.sha256()
    h.update(("|".join(pal)).encode())
    for l in (data or []): h.update(struct.pack(">q",l))
    return h.hexdigest()

def region_iter(path,rx,rz):
    with open(path,"rb") as f: raw=f.read()
    if len(raw)<8192: return
    for i in range(1024):
        o=i*4
        off_sectors=(raw[o]<<16)|(raw[o+1]<<8)|raw[o+2]
        cnt=raw[o+3]
        if off_sectors==0 or cnt==0: continue
        start=off_sectors*4096
        if start+5>len(raw): raise ValueError("chunk %d вне файла"%i)
        plen=struct.unpack_from(">I",raw,start)[0]
        comp=raw[start+4]
        payload=raw[start+5:start+4+plen]
        if comp==1: payload=gzip.decompress(payload)
        elif comp==2: payload=zlib.decompress(payload)
        elif comp!=3: raise ValueError("ctype %d (LZ4/Zstd)"%comp)
        root=nbt_root(payload)
        cx=int(root.get("xPos",rx*32+(i%32)))
        cz=int(root.get("zPos",rz*32+(i//32)))
        yield cx,cz,root

def region_iter_marked(path,rx,rz,marked):
    # ×495 SKIP-ОПТИМИЗАЦИЯ: slot-координаты ДО декомпрессии; декомпрессия
    # только слотов-кандидатов marked. Слот вне marked не декомпрессируется.
    # slot↔xPos-расхождение у marked-кандидата: chunk учитывается по xPos —
    # если xPos-чанк не marked, он просто не попадёт в world_map → covered-гейт
    # ловит недостачу → mismatch (fail-open, 0 ложных PASS).
    with open(path,"rb") as f: raw=f.read()
    if len(raw)<8192: return
    for i in range(1024):
        o=i*4
        off_sectors=(raw[o]<<16)|(raw[o+1]<<8)|raw[o+2]
        cnt=raw[o+3]
        if off_sectors==0 or cnt==0: continue
        ccx=rx*32+(i%32); ccz=rz*32+(i//32)
        if (ccx,ccz) not in marked:
            yield ccx,ccz,None,False
            continue
        start=off_sectors*4096
        if start+5>len(raw): raise ValueError("chunk %d вне файла"%i)
        plen=struct.unpack_from(">I",raw,start)[0]
        comp=raw[start+4]
        payload=raw[start+5:start+4+plen]
        if comp==1: payload=gzip.decompress(payload)
        elif comp==2: payload=zlib.decompress(payload)
        elif comp!=3: raise ValueError("ctype %d (LZ4/Zstd)"%comp)
        root=nbt_root(payload)
        cx=int(root.get("xPos",ccx))
        cz=int(root.get("zPos",ccz))
        yield cx,cz,root,True

world_map={}
reg_dir=os.path.join(world,"region")
reg_files=len([f for f in os.listdir(reg_dir) if f.endswith(".mca")]) if os.path.isdir(reg_dir) else 0
reg_errors=0; reg_seen=0; reg_skipped=0
if marked is not None and reg_files:
    for fn in sorted(f for f in os.listdir(reg_dir) if f.endswith(".mca")):
        parts=fn.split(".")
        try: rx,rz=int(parts[1]),int(parts[2])
        except (IndexError,ValueError): rx=rz=0
        try:
            for cx,cz,root,slot_hit in region_iter_marked(os.path.join(reg_dir,fn),rx,rz,marked):
                if not slot_hit:
                    reg_skipped+=1; continue
                reg_seen+=1
                if (cx,cz) not in marked:
                    reg_skipped+=1; continue
                status=str(root.get("Status",root.get("status","?")))
                secs={}
                for sec in (root.get("sections") or []):
                    if isinstance(sec,dict):
                        y,pal,data=section_blocks(sec)
                        secs[y]=(pal,data)
                blob="".join("%s:%s;"%(y,canon_digest(*secs[y])) for y in sorted(secs))
                world_map[(cx,cz)]=(status,blob)
        except Exception:
            reg_errors+=1
reg_chunks=len(world_map)
with open(os.path.join(tmpd,"chunk_set.tsv"),"w") as fset, \
     open(os.path.join(tmpd,"chunk_sums.tsv"),"w") as fsum, \
     open(os.path.join(tmpd,"world.sha"),"w") as fw:
    if marked is not None and reg_errors==0 and reg_files>0 and reg_chunks>0:
        h=hashlib.sha256()
        for (cx,cz) in sorted(world_map):
            status,blob=world_map[(cx,cz)]
            h.update(("%s,%s,%s;"%(cx,cz,status)).encode())
            h.update(blob.encode())
            fset.write("%s,%s,%s\n"%(cx,cz,status))
            fsum.write("%s,%s\t%s\n"%(cx,cz,
                hashlib.sha256(blob.replace(";","").encode()).hexdigest()))
        fw.write(h.hexdigest())

# marked-набор, заявленный forcedload, должен быть ПОЛНОСТЬЮ покрыт region-сканом
covered=(marked is not None) and (reg_errors==0) and \
        (marked.issubset(set(world_map.keys())))
if marked is None or reg_errors>0 or not covered:
    scope="mismatch"
elif reg_chunks==0:
    scope="mismatch"   # mark-набор есть, но ни одного чанка не найдено
elif fl_status=="ok":
    scope="marked-%d"%reg_chunks
else:
    scope="marked-derived-%d"%reg_chunks

with open(os.path.join(tmpd,"meta2.txt"),"w") as m2:
    m2.write("region_status=%s\nregion_files=%s\nregion_chunks=%s\nregion_errors=%s\nregion_scope=%s\nfl_status=%s\nmarked_n=%s\nregion_seen=%s\nregion_skipped=%s\nentity_count=%s\nentity_chunks=%s\nentities_files=%s\ncorrupt_chunks=%s\n" %
        ("ok" if (reg_errors==0 and reg_files>0) else ("absent" if reg_errors==0 else "error"),
         reg_files,reg_chunks,reg_errors,scope,fl_status,
         len(marked) if marked is not None else "0",
         reg_seen,reg_skipped,str(st["entities"]),str(st["entity_chunks"]),
         str(len(ent_files)),str(st["corrupt_chunks"])))
PYEOF
}

# --- emit fp.json (schema dp-parity-fp@1 + region_scope canary-4) ------------
emit_fp_json() { # <work> <uh> <ph> <counts_json> <meta2> <selftest> <scan_s> <wsha|use>
  local work="$1" uh="$2" ph="$3" cj="$4" metaf="$5" st="$6" ss="$7"
  local m k v level_time=null data_version=null ecount=0 echunks=0 efiles=0 rfiles=0 corr=0
  while IFS='=' read -r k v; do
    case "$k" in
      level_time) level_time="$v";; data_version) data_version="$v";;
      entity_count) ecount="$v";; entity_chunks) echunks="$v";;
      entities_files) efiles="$v";; region_files) rfiles="$v";;
      corrupt_chunks) corr="$v";; region_scope) rscope="$v";;
      region_errors) rerr="$v";; region_chunks) rch="$v";;
    esac
  done < "$metaf"
  local dp_sha="none"
  if [ -f "$work/run-env.txt" ]; then
    dp_sha="$(grep -m1 'DP-INSTALLED sha256=' "$work/run-env.txt" 2>/dev/null \
              | sed 's/.*sha256=\([0-9a-f]\{4,\}\).*/\1/')"
  fi
  printf '%s' "$dp_sha" | grep -Eq '^[0-9a-f]{64}$' || dp_sha="none"
  local wsha="null"
  if [ "$rscope" != "mismatch" ] && [ -s "${work}/.p76.world.sha" ]; then
    wsha="$(cat "${work}/.p76.world.sha")"
    printf '%s' "$wsha" | grep -Eq '^[0-9a-f]{64}$' || wsha="null"
  fi
  [ "$wsha" = "null" ] || wsha="\"$wsha\""
  local tmp="$work/.p76.fp.json.tmp"
  cat > "$tmp" <<EOF
{"schema":"dp-parity-fp@1","fp_schema_version":1,
 "scanner":"parity_phase76_marked",
 "region_scope":"$rscope",
 "world_sha256":$wsha,
 "world_sha256_status":"marked-scope",
 "dp_sha256":"$dp_sha",
 "chunk_set":[],"chunk_checksums":{},"chunk_checksums_status":"in:dp-parity-fp-region.json (marked-9216 scope; region_errors=$rerr)",
 "entity_uuid_multiset_sha256":"$uh",
 "entity_pos_digest_sha256":"$ph",
 "per_type_counts":$cj,
 "entity_count":$ecount,"entity_chunks":$echunks,"entities_files":$efiles,
 "region_files":$rfiles,"corrupt_chunks":$corr,"region_chunks":$rch,"region_errors":$rerr,
 "scoreboard_sha256":null,"scoreboard_sha256_status":"absent",
 "level_time":"$level_time","data_version":"$data_version",
 "selftest":"$st","scan_seconds":$ss}
EOF
  if [ "$(wc -c < "$tmp")" -gt 2097152 ]; then
    log "ERROR fp.json exceeds 2MB budget — replacing with error stub"
    printf '{"schema":"dp-parity-fp@1","fp_schema_version":1,"error":"fp.json>2MB"}\n' > "$tmp"
  fi
  mv -f "$tmp" "$work/dp-parity-fp.json"
  cp -f "${work}/.p76.chunk_set.tsv" "$work/dp-parity-fp-region.json" 2>/dev/null || true
}

main() { # <world_dir> <work_dir>
  local world="$1" work="$2"
  [ -d "$world" ] || { log "ERROR world dir missing: $world"; return 2; }
  mkdir -p "$work" || return 2
  local t0=$(date +%s)
  if ! run_scan "$world" "$work" > "$work/.p76.scan.log" 2>&1; then
    log "WARN: scan python failed (fail-open):"
    tail -5 "$work/.p76.scan.log"
    printf '{"schema":"dp-parity-fp@1","fp_schema_version":1,"error":"p76 scan-crash","selftest":"NOT-RUN"}\n' \
      > "$work/dp-parity-fp.json"
    return 0
  fi
  cp -f "$work/chunk_set.tsv" "$work/.p76.chunk_set.tsv" 2>/dev/null || true
  cp -f "$work/world.sha" "$work/.p76.world.sha" 2>/dev/null || true
  local uh ph cj st
  uh="$(canon_uuid_multiset_sha256 "$work/entities.tsv")"
  ph="$(canon_pos_digest_sha256 "$work/entities.tsv")"
  cj="$(canon_counts_json "$work/counts.tsv")"
  st="true"
  local scan_s=$(( $(date +%s) - t0 ))
  emit_fp_json "$work" "$uh" "$ph" "$cj" "$work/meta2.txt" "$st" "$scan_s"
  log "scan done: scope=$(grep -oP 'region_scope=\K.*' "$work/meta2.txt" | head -1) ${scan_s}s"
}

# --- selftest ----------------------------------------------------------------
build_fixtures() { # <tmpdir>
python3 - "$1" <<'PYEOF'
import gzip, os, struct, sys, zlib
T=sys.argv[1]
U1="00000000-0000-0000-0000-000000000001"; U2="0f0e0d0c-0b0a-4321-8765-ba0987654321"
def ints(u):
    h=u.replace('-','')
    return [(int(h[i:i+8],16)-2**32 if int(h[i:i+8],16)>=2**31 else int(h[i:i+8],16)) for i in (0,8,16,24)]
def tg(i,name,pl):
    nb=name.encode(); return bytes([i])+struct.pack(">H",len(nb))+nb+pl
def ctag(name,pl):
    nb=name.encode(); return bytes([10])+struct.pack(">H",len(nb))+nb+pl+b"\x00"
def tstr(v):
    b=v.encode(); return struct.pack(">H",len(b))+b
def tint(v): return struct.pack(">i",v)
def tlong(v): return struct.pack(">q",v)
def tdbl(v): return struct.pack(">d",v)
def tintarr(xs): return struct.pack(">i",len(xs))+b"".join(struct.pack(">i",x) for x in xs)
def tlist(et,payloads): return bytes([et])+struct.pack(">i",len(payloads))+b"".join(payloads)
def tlongarr(xs): return struct.pack(">i",len(xs))+b"".join(struct.pack(">q",x) for x in xs)
def ent(uid,typ,pos):
    return (tg(8,"id",tstr(typ))+tg(11,"UUID",tintarr(ints(uid)))+
            tg(9,"Pos",tlist(6,[tdbl(pos[0]),tdbl(pos[1]),tdbl(pos[2])]))+b"\x00")
def write_mca(path, ents):
    payload=zlib.compress(ctag("", tg(3,"DataVersion",tint(4440))+
        tg(9,"Position",tlist(3,[tint(0),tint(0)]))+tg(9,"Entities",tlist(10,ents))))
    rec=struct.pack(">I",len(payload)+1)+bytes([2])+payload
    sectors=(len(rec)+4095)//4096
    hdr=bytearray(8192); hdr[0:3]=b"\x00\x00\x02"; hdr[3]=sectors
    hdr[4096:4100]=struct.pack(">I",1)
    with open(path,"wb") as f:
        f.write(hdr); f.write(rec); f.write(b"\x00"*(sectors*4096-len(rec)))
def write_ldat(path):
    root=ctag("", ctag("Data", tg(4,"Time",tlong(133700))+tg(3,"DataVersion",tint(4440))))
    open(path,"wb").write(gzip.compress(root))
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
def write_forcedload(path, chunks):
    # vanilla ChunkPos.asLong: z в старших 32, x в младших 32 (signed-64)
    packed=[]
    for (cx,cz) in chunks:
        v=((cz & 0xFFFFFFFF)<<32)|(cx & 0xFFFFFFFF)
        if v>=2**63: v-=2**64
        packed.append(v)
    dims=struct.pack(">i",len(packed))+b"".join(struct.pack(">q",v) for v in packed)
    # vanilla shape: ForcedChunks{ "minecraft:overworld": TAG_Long_Array }
    fl=tg(12,"minecraft:overworld", dims)
    open(path,"wb").write(gzip.compress(ctag("", ctag("data",
        ctag("ForcedChunks", fl)+tg(3,"DataVersion",tint(4440))))))
POS_U1=(0.0005,64.0,-0.0004); POS_U2=(100.1235,65.5,2000.9995)
ENTS=[ent(U1,"minecraft:item",POS_U1), ent(U2,"minecraft:zombie",POS_U2)]
CHUNK_A=(0,0,"minecraft:full",[sect(-1,None,None),
         sect(0,["minecraft:stone","minecraft:air"],[305419896,-305419897])])
CHUNK_B =(5,3,"minecraft:full",[sect(0,["minecraft:dirt"],None)])
CHUNK_BM=(5,3,"minecraft:full",[sect(0,["minecraft:coarse_dirt"],None)])
def mkworld(name, chunk_b, marks, with_fl=True, bad_fl=False):
    wd=os.path.join(T,name)
    os.makedirs(os.path.join(wd,"entities"),exist_ok=True)
    os.makedirs(os.path.join(wd,"region"),exist_ok=True)
    write_mca(os.path.join(wd,"entities","r.0.0.mca"),ENTS)
    write_region_mca(os.path.join(wd,"region","r.0.0.mca"),
                     [(0,rchunk(*CHUNK_A)),(101,rchunk(*chunk_b))])
    write_ldat(os.path.join(wd,"level.dat"))
    if with_fl:
        os.makedirs(os.path.join(wd,"data"),exist_ok=True)
        if bad_fl:
            open(os.path.join(wd,"data","forcedload.dat"),"wb").write(b"GARBAGE-NOT-GZIP")
        else:
            write_forcedload(os.path.join(wd,"data","forcedload.dat"),marks)
mkworld("worldM", CHUNK_B, [(0,0)])                    # mark только (0,0)
mkworld("worldM-mut", CHUNK_BM, [(0,0)])               # мутант НЕ-marked чанка
mkworld("worldM-fl2", CHUNK_B, [(0,0),(5,3)])          # mark обоих чанков
mkworld("worldM-nofl", CHUNK_B, [])                    # без forcedload.dat
mkworld("worldM-badfl", CHUNK_B, [], bad_fl=True)      # битый forcedload.dat

def write_worldD():
    # ×495 M12: derived-OK фикстура. НЕТ data/forcedload.dat → fallback;
    # R=256 (env) → TILES=1 → marked=[-16..15]² = 1024 чанков, все в 4 region-файлах.
    wd=os.path.join(T,"worldD")
    os.makedirs(os.path.join(wd,"entities"),exist_ok=True)
    os.makedirs(os.path.join(wd,"region"),exist_ok=True)
    write_mca(os.path.join(wd,"entities","r.0.0.mca"),ENTS)
    write_ldat(os.path.join(wd,"level.dat"))
    for rx in (-1,0):
        for rz in (-1,0):
            items=[]
            for cx in range(rx*32,rx*32+32):
                for cz in range(rz*32,rz*32+32):
                    if -16<=cx<16 and -16<=cz<16:
                        slot=(cz&31)*32+(cx&31)
                        items.append((slot,rchunk(cx,cz,"minecraft:full",
                            [sect(0,["minecraft:stone","minecraft:air"],[305419896,-305419897])])))
            write_region_mca(os.path.join(wd,"region","r.%d.%d.mca"%(rx,rz)),items)
write_worldD()
PYEOF
}

selftest() { # <tmpdir>
  local T="$1" fails=0 h ws
  mkdir -p "$T" || return 1
  build_fixtures "$T" || { echo "P76-SELFTEST FIXTURE-BUILD FAIL"; return 1; }
  local worldM="$T/worldM"
  if run_scan "$worldM" "$T/outM" >/dev/null 2>&1; then
    uh="$(canon_uuid_multiset_sha256 "$T/outM/entities.tsv")"
    [ "$uh" = "$ST_H_ENT" ] && echo "P76-SELFTEST M1 PASS uuid-multiset=$uh" \
      || { echo "P76-SELFTEST M1 FAIL uuid got=$uh want=$ST_H_ENT"; fails=$((fails+1)); }
    ph="$(canon_pos_digest_sha256 "$T/outM/entities.tsv")"
    [ "$ph" = "$ST_P_ENT" ] && echo "P76-SELFTEST M2 PASS pos-digest=$ph" \
      || { echo "P76-SELFTEST M2 FAIL pos got=$ph want=$ST_P_ENT"; fails=$((fails+1)); }
    ws="$(cat "$T/outM/world.sha" 2>/dev/null)"
    [ "$ws" = "$ST_WORLD_SHA" ] && echo "P76-SELFTEST M3 PASS marked-world-sha=$ws" \
      || { echo "P76-SELFTEST M3 FAIL sha got=$ws want=$ST_WORLD_SHA"; fails=$((fails+1)); }
    ps="$(head -1 "$T/outM/chunk_sums.tsv" 2>/dev/null | cut -f2)"
    [ "$ps" = "$ST_PER_SUM" ] && echo "P76-SELFTEST M4 PASS per-chunk-sum" \
      || { echo "P76-SELFTEST M4 FAIL sum got=$ps want=$ST_PER_SUM"; fails=$((fails+1)); }
    n="$(wc -l < "$T/outM/chunk_set.tsv" 2>/dev/null)"
    [ "$n" = "1" ] && echo "P76-SELFTEST M5 PASS marked-filter n=$n" \
      || { echo "P76-SELFTEST M5 FAIL marked-filter n=$n want=1"; fails=$((fails+1)); }
  else
    echo "P76-SELFTEST SCAN-CRASH worldM"; fails=$((fails+1))
  fi
  run_scan "$T/worldM-mut" "$T/outMut" >/dev/null 2>&1
  ws2="$(cat "$T/outMut/world.sha" 2>/dev/null)"
  [ "$ws2" = "$ST_WORLD_SHA" ] && echo "P76-SELFTEST M6 PASS unmarked-mutant-invariant" \
    || { echo "P76-SELFTEST M6 FAIL unmarked mutation changed digest got=$ws2"; fails=$((fails+1)); }
  run_scan "$T/worldM-fl2" "$T/outFl2" >/dev/null 2>&1
  ws3="$(cat "$T/outFl2/world.sha" 2>/dev/null)"
  [ -n "$ws3" ] && [ "$ws3" != "$ST_WORLD_SHA" ] && echo "P76-SELFTEST M7 PASS mark2-changes-digest=$ws3" \
    || { echo "P76-SELFTEST M7 FAIL mark2 digest=$ws3"; fails=$((fails+1)); }
  run_scan "$T/worldM-nofl" "$T/outNofl" >/dev/null 2>&1
  grep -q "region_scope=mismatch" "$T/outNofl/meta2.txt" && echo "P76-SELFTEST M8 PASS no-forcedload-failopen" \
    || { echo "P76-SELFTEST M8 FAIL no-fl scope=$(grep region_scope "$T/outNofl/meta2.txt" 2>/dev/null)"; fails=$((fails+1)); }
  run_scan "$T/worldM-badfl" "$T/outBadfl" >/dev/null 2>&1
  grep -q "region_scope=mismatch" "$T/outBadfl/meta2.txt" && echo "P76-SELFTEST M9 PASS bad-forcedload-failopen" \
    || { echo "P76-SELFTEST M9 FAIL bad-fl"; fails=$((fails+1)); }
  # end-to-end fp.json на worldM (mismatch-free path)
  rm -rf "$T/wMfp"; cp -r "$worldM" "$T/wMfp"
  main "$T/wMfp" "$T/outFp" >/dev/null 2>&1
  python3 -c "
import json,sys
fp=json.load(open('$T/outFp/dp-parity-fp.json'))
ok = fp.get('region_scope','').startswith('marked-') and fp.get('world_sha256','').startswith('65355c14') and fp.get('selftest')=='true' and fp.get('entity_uuid_multiset_sha256','').startswith('33ef0979')
sys.exit(0 if ok else 1)" \
    && echo "P76-SELFTEST M10 PASS fp.json end-to-end" \
    || { echo "P76-SELFTEST M10 FAIL fp.json"; fails=$((fails+1)); }
  # ×495 derived-fix (Л-494-CANARY4): M11 fail-open при неполном покрытии,
  # M12 derived-OK end-to-end (R=256 → 1024 чанков, sha preregistered независимо)
  run_scan "$T/worldM-nofl" "$T/outD11" >/dev/null 2>&1
  grep -q "fl_status=derived-r640" "$T/outD11/meta2.txt" && \
  grep -q "region_scope=mismatch" "$T/outD11/meta2.txt" \
    && echo "P76-SELFTEST M11 PASS derived-fallback-fires-failopen" \
    || { echo "P76-SELFTEST M11 FAIL fl=\$(grep fl_status "$T/outD11/meta2.txt" 2>/dev/null) sc=\$(grep region_scope "$T/outD11/meta2.txt" 2>/dev/null)"; fails=$((fails+1)); }
  rm -rf "$T/outD12"
  BENCH_FORCELOAD_RADIUS="$ST_D_R" run_scan "$T/worldD" "$T/outD12" >/dev/null 2>&1
  sc12="$(grep -oP 'region_scope=\K.*' "$T/outD12/meta2.txt" 2>/dev/null | head -1)"
  ws12="$(cat "$T/outD12/world.sha" 2>/dev/null)"
  [ "$sc12" = "marked-derived-$ST_D_MARKED_N" ] && [ "$ws12" = "$ST_D_WORLD_SHA" ] \
    && echo "P76-SELFTEST M12 PASS derived-1024 sha=$ws12" \
    || { echo "P76-SELFTEST M12 FAIL scope=$sc12 sha=$ws12 want=$ST_D_WORLD_SHA"; fails=$((fails+1)); }
  if [ "$fails" -eq 0 ]; then echo "P76-SELFTEST ALL PASS (12/12)"; return 0
  else echo "P76-SELFTEST FAILURES=$fails"; return 1; fi
}

case "${1:-}" in
  --selftest)
    selftest "${2:-/tmp/p76-selftest.$$}"
    ;;
  *)
    [ $# -ge 2 ] || { echo "usage: $0 <world_dir> <work_dir> | --selftest <tmpdir>"; exit 2; }
    main "$1" "$2"
    ;;
esac
