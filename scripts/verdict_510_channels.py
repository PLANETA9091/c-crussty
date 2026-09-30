#!/usr/bin/env python3
"""verdict_510_channels.py — ×510 вердикт-каналы из registry_510.json:
Г3 min-of-3 (дерево C16), RPIN min-of-3, dp G-B2/G-S20/реплики, пробы C28, банк-фид §3."""
import json, re, statistics

REG = json.load(open("/home/z/rounds/ROUND-510/absorb/registry_510.json"))
res = REG["results"]

print("=== КЛАССЫ ===")
from collections import Counter
cls = Counter(r.get("verdict", {}).get("class", ("NO-ART/" + str(r.get("err"))[:40]) if r.get("ok") else "ERR:" + str(r.get("err"))[:40]) for r in res)
for k, v in cls.most_common():
    print(f"  {k}: {v}")

print("\n=== Г3 КАНАЛ (fen ad174a10 / unf 110e431a) ===")
g3 = {}
for r in res:
    b = r.get("branch") or ""
    if b.startswith("round-510-g3"):
        v = r.get("verdict", {})
        s = v.get("stz59_last", {})
        g3[b] = {"ok": r["ok"], "rows": v.get("stz59_rows", 0), "sd_max": v.get("stz59_sd_max"),
                 "ops": v.get("stz59_ops_last"), "arm": s.get("arm"), "ncdfe": v.get("ncdfe"),
                 "aioobe": v.get("aioobe"), "class": v.get("class"), "norm": v.get("norm"), "cpu": v.get("cpu")}
        print(f"  {b}: rows={v.get('stz59_rows')} sd_max={v.get('stz59_sd_max')} ops={v.get('stz59_ops_last')} arm={s.get('arm')} ncdfe={v.get('ncdfe')} class={v.get('class')}")
fen_ok = [g3[b] for b in g3 if "g3fen" in b and g3[b]["ok"] and g3[b].get("rows")]
unf_ok = [g3[b] for b in g3 if "g3unf" in b and g3[b]["ok"] and g3[b].get("rows")]
if fen_ok and unf_ok:
    fen_arm = {f.get("arm") for f in fen_ok}; unf_arm = {u.get("arm") for u in unf_ok}
    ops_ok = all((f.get("ops") or 0) >= 1e8 for f in fen_ok + unf_ok)
    sd_fen = [f["sd_max"] or 0 for f in fen_ok]
    sd_unf = [u["sd_max"] or 0 for u in unf_ok]
    print(f"  GATE armature: fen arms={fen_arm} sd={sd_fen} | unf arms={unf_arm} sd={sd_unf} | ops_ok(≥1e8)={ops_ok}")
    med_fen, med_unf = statistics.median(sd_fen), statistics.median(sd_unf)
    print(f"  fen_med={med_fen} unf_med={med_unf} r_med={med_fen/med_unf if med_unf else None}")
    print(f"  ДЕРЕВО: {'A1-suppression' if med_unf and med_fen/med_unf <= 0.05 else ('A3-VACUOUS-0' if med_fen == 0 else 'D/REOPEN')}")

print("\n=== RPIN min-of-3 ===")
for r in res:
    b = r.get("branch") or ""
    if b.startswith("round-510-rpin"):
        v = r.get("verdict", {})
        print(f"  {b}: start={v.get('rpin_start')} end={v.get('rpin_end')} drift={v.get('rpin_drift_pct')}% → {v.get('rpin_verdict')} | class={v.get('class')} norm={v.get('norm')}")

print("\n=== dp-КАНАЛ ===")
dp = {}
for r in res:
    b = r.get("branch") or ""
    if b.startswith("round-510-dp0"):
        v = r.get("verdict", {})
        dp[b] = v
        print(f"  {b}: con={r['conclusion']} notps={v.get('dp_notps')} med={v.get('dp_med')} norm={v.get('dp_norm')} pop={v.get('pop')} n_polls={v.get('n_polls')} class={v.get('class')}")
# G-B2: dp01=seed1330, dp02=seed1331
g_b2_fires = 8  # k=8 n=17 до волны
for leg in ("round-510-dp01", "round-510-dp02"):
    v = dp.get(leg)
    if v and v.get("dp_notps") and v.get("pop") == 50000:
        g_b2_fires += 1
n_tot = 17 + sum(1 for leg in ("round-510-dp01", "round-510-dp02") if dp.get(leg) and dp[leg].get("pop") == 50000)
print(f"  G-B2: k={g_b2_fires} n={n_tot}")

print("\n=== ПРОБЫ C28 ===")
for r in res:
    b = r.get("branch") or ""
    if b in ("round-507-ax10", "round-507-ax26", "round-508-ax03"):
        v = r.get("verdict", {})
        print(f"  {b}: con={r['conclusion']} PASS-A={'YES' if r['conclusion']=='success' else ('N/A--cancelled' if r['conclusion']=='cancelled' else 'NO')} norm={v.get('norm') if v else None} class={v.get('class') if v else '-'}")

print("\n=== БАНК-ФИД §3 (vanilla pop=150k CLEAN LEGAL) ===")
feed = []
for r in res:
    v = r.get("verdict")
    if not v or not (r.get("branch") or "").startswith("round-510-"):
        continue
    if v.get("vanilla_valid") and v.get("m1") and v.get("band") and v.get("pop") == 150000 and (v.get("norm") or 0) <= 15.0:
        feed.append({"branch": r["branch"], "run_id": r["run_id"], "cpu": v["cpu"], "norm": v["norm"], "strict": v["strict"]})
for f in sorted(feed, key=lambda x: x["cpu"]):
    print(f"  {f['branch']} | run {f['run_id']} | cpu {f['cpu']} | norm {f['norm']}{' [STRICT]' if f['strict'] else ''}")
print(f"ИТОГО §3-фид: {len(feed)}")
print("\n=== НЕ-§3 succ классы (для ценза) ===")
for r in res:
    v = r.get("verdict")
    if r["conclusion"] == "success" and v and not (v.get("vanilla_valid") and v.get("m1") and v.get("band") and v.get("pop") == 150000 and (v.get("norm") or 0) <= 15.0):
        print(f"  {r['branch']} | cpu {v.get('cpu')} | norm {v.get('norm')} | {v.get('class')} | armed {v.get('armed') or '∅'} | ncdfe {v.get('ncdfe')} | world {v.get('world')} | pop {v.get('pop')}")

print("\n=== НЕ-§3 succ классы (для ценза) ===")
for r in res:
    v = r.get("verdict")
    if r["conclusion"] == "success" and v and v.get("class") not in ("VANILLA-VALID-§3(банк-фид)", "STRICT-IN-POINT(банк-фид)"):
        print(f"  {r['branch']} | cpu {v.get('cpu')} | norm {v.get('norm')} | {v.get('class')} | armed {v.get('armed') or '∅'} | ncdfe {v.get('ncdfe')} | world {v.get('world')} | pop {v.get('pop')}")
