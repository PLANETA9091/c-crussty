#!/usr/bin/env python3
"""verdict_482_c14_w8.py — C14 W8@r480 артефакт-разбор + norm + 19a-числа (канон absorb_482_main)."""
import json, os, re, shutil, subprocess, sys, zipfile

sys.path.insert(0, "/home/z/c-crussty/scripts")
import absorb_482_main as A

RUN = 36419896337
ART = 10969618118
D = "/home/z/rounds/ROUND-482/c14"
os.makedirs(D, exist_ok=True)

t = A.tok()
zpath = f"{D}/w8_art.zip"
url = f"https://api.github.com/repos/{A.REPO}/actions/artifacts/{ART}/zip"
p = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {t}", url, "-o", zpath],
                   capture_output=True, timeout=600)
assert open(zpath, "rb").read(2) == b"PK", "artifact zip bad"
names = zipfile.ZipFile(zpath).namelist()
keep = ("run-env.txt", "server-stdout.log", "gc.log", "BOTTLENECKS_3.md")
with zipfile.ZipFile(zpath) as z:
    for m in keep:
        cand = [n for n in z.namelist() if n.endswith(m)]
        if cand:
            with z.open(cand[0]) as src, open(os.path.join(D, m), "wb") as dst:
                shutil.copyfileobj(src, dst)

v = A.norm_of(D)
env = open(f"{D}/run-env.txt", errors="replace").read()
log = open(f"{D}/server-stdout.log", errors="replace").read()

echo = {}
for k in ("REGION_THREADS", "FORCELOAD_RADIUS", "GC_TUNE", "LEVER_FLAG", "LEVER_ARG",
          "POPULATION_TARGET", "POPULATION_SEED", "SERVER_XMX", "SERVER_XMS"):
    m = re.search(rf"^{k}:\s*(\S+)", env, re.M) or re.search(rf"^{k}=(\S+)", env, re.M)
    if m:
        echo[k] = m.group(1)
band_echo = re.search(r"runner_cpu_index=(\d+) band=\[([\d.,]+)\]", env)
m_force = re.findall(r"Forceloaded (\d+) chunks in (\d+(?:\.\d+)?)s", log)
m_done = "DONE" in log and bool(re.search(r"forceload", log, re.I))
gen_lines = [l for l in log.splitlines() if re.search(r"forceload|Forceload", l)][:6]
pop_ok = "POPULATION INJECT DONE" in log
pop_valid = "POPULATION FIXTURE-VALIDITY: VALID" in log
fixt = "FIXTURE-VALIDITY: VALID" in log

out = {"run": RUN, "norm_v5": v, "echo": echo,
       "band_echo": band_echo.groups() if band_echo else None,
       "forceload_matches": m_force[:4], "gen_lines": gen_lines,
       "pop_inject_done": pop_ok, "pop_valid": pop_valid, "fixture_valid": fixt}
json.dump(out, open(f"{D}/c14_verdict.json", "w"), indent=1, ensure_ascii=False)
print(json.dumps(out, indent=1, ensure_ascii=False)[:3000])
