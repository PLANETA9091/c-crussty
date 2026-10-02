#!/usr/bin/env python3
"""[479-W3] ramp-model: extract raw polls x runs, fit poll[i]=plateau*(1-exp(-i/tau)), median-vs-plateau bias."""
import glob, json, os, re, sys, zipfile

RE_TPS   = re.compile(r"TPS from last 5s.*?: ([\d.]+),")
RE_POLLS = re.compile(r"TPS polls captured: (\d+), first-of-window values: \[([^\]]*)\]")
RE_IDX   = re.compile(r"runner_cpu_index[:=]\s*(\d+)")
TPS_MAX_VALID = 15.0  # canon C55 FROZEN

ZIPS = sorted(set(
    glob.glob("/home/z/rounds/ROUND-478/G24/art/art_*.zip") +
    glob.glob("/home/z/rounds/ROUND-479/A7/art/art_*.zip") +
    glob.glob("/home/z/rounds/ROUND-479/A8/art/art_*.zip") +
    glob.glob("/home/z/rounds/ROUND-479/W3/art/art_*.zip")))
DIRS = ["/home/z/rounds/ROUND-479/a4/s2", "/home/z/rounds/ROUND-479/a4/s3"]  # run-env has run id? use dir name mapping

out = []
for zp in ZIPS:
    try:
        with zipfile.ZipFile(zp) as z:
            names = z.namelist()
            env = z.read("run-env.txt").decode(errors="replace")
            log = z.read("server-stdout.log").decode(errors="replace")
            bot = z.read("BOTTLENECKS_3.md").decode(errors="replace") if "BOTTLENECKS_3.md" in names else ""
    except Exception as e:
        print(f"SKIP {zp}: {e}", file=sys.stderr); continue
    rid = os.path.basename(zp)[4:-4]
    idx_m = RE_IDX.search(env)
    stdout_polls = [float(x) for x in RE_TPS.findall(log)]
    mb = RE_POLLS.search(bot)
    fow = [float(x) for x in mb.group(2).split(",") if x.strip()] if mb else []
    n_cap = int(mb.group(1)) if mb else None
    valid = [x for x in (fow if fow else stdout_polls) if x < TPS_MAX_VALID]
    out.append({"run_id": rid, "src": os.path.basename(os.path.dirname(zp)),
                "cpu_index": int(idx_m.group(1)) if idx_m else None,
                "n_captured": n_cap, "raw_stdout_n": len(stdout_polls),
                "raw_stdout_polls": stdout_polls, "fow_polls": fow,
                "valid_c55": valid})

# a4 extracted dirs: s2=36362740256 s3=36362354569 (per Л-479-A4)
for d, rid in [("/home/z/rounds/ROUND-479/a4/s2", "36362740256"),
               ("/home/z/rounds/ROUND-479/a4/s3", "36362354569")]:
    if not os.path.isdir(d): continue
    env = open(os.path.join(d, "run-env.txt"), errors="replace").read()
    log = open(os.path.join(d, "server-stdout.log"), errors="replace").read()
    botp = os.path.join(d, "BOTTLENECKS_3.md")
    bot = open(botp, errors="replace").read() if os.path.exists(botp) else ""
    idx_m = RE_IDX.search(env)
    stdout_polls = [float(x) for x in RE_TPS.findall(log)]
    mb = RE_POLLS.search(bot)
    fow = [float(x) for x in mb.group(2).split(",") if x.strip()] if mb else []
    valid = [x for x in (fow if fow else stdout_polls) if x < TPS_MAX_VALID]
    out.append({"run_id": rid, "src": "a4-extracted", "cpu_index": int(idx_m.group(1)) if idx_m else None,
                "n_captured": int(mb.group(1)) if mb else None, "raw_stdout_n": len(stdout_polls),
                "raw_stdout_polls": stdout_polls, "fow_polls": fow, "valid_c55": valid})

json.dump(out, open("/home/z/rounds/ROUND-479/W3/raw_polls.json", "w"), indent=1)
print(f"runs extracted: {len(out)}")
for r in out:
    print(f"{r['run_id']} idx={r['cpu_index']} cap={r['n_captured']} stdout_n={r['raw_stdout_n']} "
          f"fow={r['fow_polls']} valid_c55={r['valid_c55']}")
