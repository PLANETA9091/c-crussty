#!/usr/bin/env python3
"""absorb_488_wiring.py — абсорб wiring-ранов c65/c66/c67 (dp-Δcpu vs якорь n=10).
Бар: base 0.3 → 58.9pp / base 0.4 → 51.8pp (спека C06 + патч C24). STRICT-флаг-пруф обязателен.
"""
import json, os, subprocess, zipfile, re, statistics

TOK = open('/tmp/gh_token').read().strip()
REPO = "PLANETA9091/c-crussty"
OUT = "/tmp/abs488"
RUNS = {
    36517282752: ("c65-sbulk1", "cmp487_sbulk1"),
    36517285326: ("c66-biroar", "cmp487_bir1"),
    36517288441: ("c67-roar1", "cmp487_roar1"),
}
# якорь-пул dp n=10 (мед 0.3, плато [6.49,8.84]M): S/T базовые константы из C06 спеки
# Δcpu = (1-(S/T)lev/(S/T)base)*100; на dp-стенде норма считается по cpu_index exp-модели normtool v5.
# Для вердикта используем TPS-эквивалент: мед-якорь 0.3 → leg-цель ≥0.73 (бар +20% pair-stable)
# и Δcpu-канон: leg-TPS ≥ 0.6 = +29.6пп… количественно ниже по квант-модели.

def curl_json(url):
    r = subprocess.run(["curl", "-s", "-H", f"Authorization: token {TOK}", url],
                       capture_output=True, text=True)
    return json.loads(r.stdout)

for rid, (label, flag) in RUNS.items():
    d = os.path.join(OUT, str(rid))
    os.makedirs(d, exist_ok=True)
    meta = curl_json(f"https://api.github.com/repos/{REPO}/actions/runs/{rid}/artifacts")
    for art in meta.get("artifacts", []):
        af = os.path.join(d, art["name"] + ".zip")
        if not os.path.exists(af):
            subprocess.run(["curl", "-sL", "-H", f"Authorization: token {TOK}",
                            art["archive_download_url"], "-o", af])
        try:
            with zipfile.ZipFile(af) as z:
                for n in z.namelist():
                    if n == "BOTTLENECKS_3.md":
                        txt = z.read(n).decode("utf-8", "replace")
                        m = re.search(r"TPS polls captured:\s*(\d+),\s*first-of-window values:\s*\[([^\]]*)\]", txt)
                        polls = [float(x) for x in m.group(2).split(",")] if m else []
                        mc = re.search(r"runner_cpu_index:\s*(\d+)", txt)
                        cpu = int(mc.group(1)) if mc else 0
                        fl = re.search(r"lever_flag:\s*(\S*)", txt)
                        dp = re.search(r"DP-INSTALLED sha256=(\w{8})", txt)
                        win = [v for v in polls if v <= 5.0]
                        med = statistics.median(win) if win else None
                        arm = "ARM" if (fl and fl.group(1) == flag) else "NO-FLAG"
                        print(f"{label} | {rid} | med={med} | polls={polls} | cpu={cpu/1e6:.2f}M | flag={fl.group(1) if fl else '?'} [{arm}] | dp={dp.group(1) if dp else '∅'}")
        except Exception as e:
            print(label, "ERR", str(e)[:100])
