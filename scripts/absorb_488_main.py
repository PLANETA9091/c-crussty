#!/usr/bin/env python3
"""absorb_488_main.py — абсорб IN-FLIGHT ×487 (44 бенч-ноги, тик ×488 11:08+08).
Скачивает артефакты completed-ранов из IN-FLIGHT-таблицы ×488,
распаковывает в /tmp/abs488/<run_id>/, читает TPS-ключи (collapsed-сводки),
выдаёт таблицу чисел для вердиктов. Пары min-of-3 / банды — дальше вручную по спеке C06.
"""
import json, os, subprocess, sys, zipfile, re, io, statistics

TOK = open('/tmp/gh_token').read().strip()
REPO = "PLANETA9091/c-crussty"
OUT = "/tmp/abs488"
os.makedirs(OUT, exist_ok=True)

# run_id -> (label, branch, what)
RUNS = {
    36506102482: ("C01-baserep", "dp-база 0.5-аномалии АРБИТР"),
    36506087795: ("C13-p207", "207k клиф"),
    36506006916: ("C13-p207b", "207k клиф rep"),
    36506233693: ("C15-m1", "215k v6-veto #3"),
    36509438915: ("C58-towr4", "towers rt4 rep"),
    36506572719: ("C17-terr4", "terralith rt4"),
    36511603266: ("C73-terr2", "terralith rt2"),
    36506121364: ("C16-tow8", "towers rt8"),
    36506460713: ("C19-st7", "STRICT st7 roll-3"),
    36505985827: ("C20-st8", "STRICT st8"),
    36507529511: ("C30-limbo900", "LIMBO 900s tect"),
    36507467885: ("C36-sensn16", "sensn16 canon"),
    36507190933: ("C37-p212", "212k"),
    36511623039: ("C70-p218", "218k"),
    36511638238: ("C71-p208", "208k"),
    36507485999: ("C38-gc1", "gc_tune=1"),
    36507520906: ("C38-gc2", "gc_tune=2"),
    36507557499: ("C40-rt6", "region_threads=6"),
    36507391053: ("C40-rt8", "region_threads=8"),
    36509535833: ("C51-dp03", "dp-0.3 якорь rep"),
    36509360193: ("C52-dp03", "dp-0.3 якорь rep-2"),
    36509237987: ("C53-dp100k", "dp@100k"),
    36509262326: ("C54-dp50k2", "dp@50k-2"),
    36511750518: ("C77-dp200k", "dp@200k"),
    36509405245: ("C55-p205", "205k rep"),
    36509515495: ("C56-p210r2", "210k CENS rep"),
    36509334849: ("C57-mna", "мед-норма a"),
    36509340686: ("C57-mnb", "мед-норма b"),
    36509307265: ("C59-p165", "165k-gc6 rep"),
    36509250451: ("C60-tectr", "tect-мир rep"),
    36511767593: ("C74-r480", "r480 rep"),
    36511780876: ("C75-r480g6", "r480×gc6"),
    36511686234: ("C76-fp0", "fp0-фактор"),
    36511617786: ("C78-dps43", "dp seed43"),
    36511755006: ("C79-cnr9", "canary cnr9"),
    36511721327: ("C80-cnr10", "canary cnr10"),
    36513549586: ("C98-dps600", "dp-600s"),
    36514478659: ("XM1", "xm1 STRICT"),
    36514506554: ("XS1", "xs1 st7"),
    36514515367: ("XS2", "xs2 st7"),
    36514523985: ("XS3", "xs3 st7"),
}

API = f"https://api.github.com/repos/{REPO}/actions/runs"
HDR = ["-H", f"Authorization: token {TOK}"]

def curl_json(url):
    r = subprocess.run(["curl", "-s"] + HDR + [url], capture_output=True, text=True)
    return json.loads(r.stdout)

def sh(cmd):
    return subprocess.run(cmd, shell=True, capture_output=True, text=True)

rows = []
for rid, (label, what) in RUNS.items():
    d = os.path.join(OUT, str(rid))
    meta_f = os.path.join(d, "meta.json")
    if os.path.exists(meta_f):
        meta = json.load(open(meta_f))
    else:
        meta = curl_json(f"{API}/{rid}/artifacts")
        os.makedirs(d, exist_ok=True)
        json.dump(meta, open(meta_f, "w"))
    tps_result = None
    parsed = {}
    if meta.get("total_count", 0) > 0:
        for art in meta["artifacts"]:
            af = os.path.join(d, art["name"] + ".zip")
            if not os.path.exists(af):
                sh(f"curl -sL -H 'Authorization: token {TOK}' '{art['archive_download_url']}' -o '{af}'")
            try:
                with zipfile.ZipFile(af) as z:
                    for n in z.namelist():
                        if n == "BOTTLENECKS_3.md":
                            txt = z.read(n).decode("utf-8", "replace")
                            m = re.search(r"TPS polls captured:\s*(\d+),\s*first-of-window values:\s*\[([^\]]*)\]", txt)
                            if m:
                                ncaps = int(m.group(1))
                                vals = [float(x.strip()) for x in m.group(2).split(",") if x.strip()]
                                parsed["polls"] = vals
                                parsed["ncaps"] = ncaps
                            mc = re.search(r"runner_cpu_index:\s*(\d+)", txt)
                            if mc:
                                parsed["cpu"] = int(mc.group(1))
                            mp = re.search(r"population_target:\s*(\d+)", txt)
                            if mp:
                                parsed["pop"] = int(mp.group(1))
                            mg = re.search(r"gc_tune:\s*(\d+)", txt)
                            if mg:
                                parsed["gct"] = int(mg.group(1))
                            mr = re.search(r"region_threads:\s*(\d+)", txt)
                            if mr:
                                parsed["rt"] = int(mr.group(1))
            except Exception as e:
                parsed["err"] = str(e)[:120]
    polls = parsed.get("polls", [])
    cpu = parsed.get("cpu")
    # медиана оконных значений (без boot-окна >5 TPS)
    win = [v for v in polls if v <= 5.0]
    med = statistics.median(win) if win else None
    band = "OK" if (cpu and 6.0e6 <= cpu <= 9.5e6) else ("OUT" if cpu else "?")
    c = f"{cpu/1e6:.2f}M" if cpu else "?"
    print(f"{label} | {rid} | med={med if med is not None else '-'} | polls={polls} | cpu={c} band={band} | pop={parsed.get('pop','?')} gc={parsed.get('gct','?')} rt={parsed.get('rt','?')}")
