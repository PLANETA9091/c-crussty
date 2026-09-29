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
# ДЕРЕЙФЛЫ (честно, fail-open по построению):
#   world_sha256 / chunk_checksums / chunk_set / scoreboard_sha256 — DEFERRED
#   (след. тик: байт-в-байт выравнивание с scan_region_dir арбитра
#   scripts/world_diff_parity_v2.py, C04 §5.2). В fp.json пишутся null+status —
#   offline-гейт увидит deferred, а НЕ ложный дайджест.
#
# РЕЖИМЫ:
#   parity_phase75.sh <world_dir> <work_dir>   # скан (fail-open: всегда exit 0)
#   parity_phase75.sh --selftest [tmpdir]      # самотест (fail-closed: exit 1
#                                              #  при любом FAIL-кейсе)
# Самотест: 10 кейсов на фикстурах (T-канон, клон/удаление/пере-порядок =
# D6-класс; hostile-locale; END-TO-END на реальных сгенерированных mca-байтах
# entities/r.0.0.mca + level.dat; fail-open-доказательство S10).
# Все ожидаемые sha256 — прeregistered константы, вычислены НЕЗАВИСИМОЙ
# python3-hashlib реализацией канона (кросс-чек bash-реализации).
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

# ==============================================================================
# NBT/MCA ЭКСТРАКТОР (python3 stdlib-only, embed; C04 §5 шаги 1+3)
# entities/*.mca (chunk NBT: Entities[]) -> TSV uuid\ttype\tx\ty\tz (repr-float
# round-trip) + counts.tsv + meta KV. level.dat -> Time/DataVersion.
# region-чанк-канон (C04 §5 шаг 2) — DEFERRED: выравнивание с арбитром след. тик.
# ==============================================================================
run_extractor() { # <world_dir> <tmpdir>
python3 - "$1" "$2" <<'PYEOF'
import gzip, os, struct, sys, zlib
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
    for fn in ent_files:
        try: scan_entities_mca(os.path.join(ent_dir,fn),tsv,st)
        except Exception: st["corrupt_chunks"]+=1
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
PYEOF
}

# ==============================================================================
# fp.json — поля по C04 §1 (deferred-поля честно null+status)
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
  local tmp="$work/.p75.fp.json.tmp"
  cat > "$tmp" <<EOF
{"schema":"dp-parity-fp@1","fp_schema_version":1,
 "world_sha256":null,
 "world_sha256_status":"deferred: chunk canon byte-align vs scan_region_dir(world_diff_parity_v2.py) next tick (C04 §5.2)",
 "dp_sha256":"$dp_sha",
 "chunk_set":[],"chunk_checksums":{},"chunk_checksums_status":"deferred",
 "entity_uuid_multiset_sha256":"$uh",
 "entity_pos_digest_sha256":"$ph",
 "per_type_counts":$cj,
 "entity_count":$ecount,"entity_chunks":$echunks,"entities_files":$efiles,
 "region_files":$rfiles,"corrupt_chunks":$corr,
 "scoreboard_sha256":null,"scoreboard_sha256_status":"deferred",
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
  t1=$(date +%s%3N 2>/dev/null || date +%s)
  ss="$(awk -v a="$t0" -v b="$t1" 'BEGIN{printf "%.3f",(b-a)/1000}')"
  emit_fp_json "$work" "$uh" "$ph" "$cj" "$TMPD/meta.txt" "$st" "$ss" || return 1
  log "DP-PARITY-FP: OK uuid_multiset_sha256=$uh pos_digest_sha256=$ph entities=$(awk -F= '$1=="entity_count"{print $2}' "$TMPD/meta.txt") level_time=$(awk -F= '$1=="level_time"{print $2}' "$TMPD/meta.txt") scan_s=$ss selftest=$st"
  log "DP-PARITY-FP: deferred-fields world_sha256/scoreboard (next-tick arbiter align)"
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

  t1=$(date +%s%3N 2>/dev/null || date +%s)
  local ms=$(( t1 - t0 ))
  P75_ST_FAILS=$fails; P75_ST_MS=$ms
  return $([ "$fails" -eq 0 ] && echo 0 || echo 1)
}

selftest_main() { # fail-closed режим
  SELFTEST_TMPD="${1:-$(mktemp -d "${TMPDIR:-/tmp}/p75selftest.XXXXXXXX")}"
  echo "P75-SELFTEST: start tmp=$SELFTEST_TMPD (11 cases, preregistered constants, python-hashlib authority)"
  selftest_core "$SELFTEST_TMPD"; local rc=$?
  if [ "$rc" -eq 0 ]; then
    echo "P75-SELFTEST: PASS cases=11/11 fails=0 wall=${P75_ST_MS}ms"
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
