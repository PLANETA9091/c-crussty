#!/usr/bin/env python3
"""absorb_s7207.py - ЦЕЛЕВОЙ профиль-лег players-32 (TASK-392/393, ЦЕЛЬ-ДЕПЛОЙ) — absorb лега s7207
ДИАГНОСТИЧЕСКИЙ лег (не гейт-лег, без банкинга): банк v4 + fake_players=32
vs якорь fp=4 (2.6 @ 8551924 / 2.2 @ 6653417) и закрытая точка fp=16 (-0.2%,
35519072184). Карта конкарренси: где растёт цена игрока при fp=32 ->
под-лейн >=5% при игроко-нагрузке = следующий архитектурный рычаг под
МЕГА-БУСТ мандат (владелец 2026-09-21: хватит +1%-тасков).

PG-T1 delivery: банк-флаги (gc_tune=3, inside_cache=1, flush_diet=1,
  region_threads=4, batch_collector=1, fluid_guard=1) + ВСЕ refuted-флаги=0
  (fluid_bitmask=0! fluid_dirty=0, fluid_dirty_ledger=0, inside_bitmask=0,
  travel_diet=0, skip_store_bb=0, region_steal=0, bu_defer=0) + fp=32 +
  pop VALID + ParallelGC + 0 NCDFE. УРОК TASK-392: want-словарь сверяется с
  диспатч-инпутами dispatch_s7207.py (банк v4 ЧИСТЫЙ, без #15/#16 флагов).
PG-T2 crash-free + TPS-поллы. Урок s7207#1 (35530317923): 503-сгорание
  доставки = INFRA-DELIVERY-FAIL (fetch() хардинг 785b0a4) — не вердикт.
PG-T3 ИНФОРМАЦИОННЫЙ: median5 при fp=32; дельта vs TPS_exp-интерполяция
  якоря при fp=4 (2-точки); ожидание суб-линейной цены конкарренси до 16;
  32 = первый стресс-профиль. НЕ dual-bar (нет банкинга).
PG-T4 страховочные ParallelGC: young 30..250, Full <= 10, total <= 22s
  (fp=32 добавляет паркет/сет-нагрузку — гейт расширен vs fp4 20.0s).
PG-T5 ПРОФИЛЬ-КАРТА (главный выход): top-лейны cpu-collapsed при fp=32 vs
  банк-профиль RECON-42 (fluid 14.6%, paletted-reads, broadphase,
  fastutil/collections, volatile/inside) — какие лейны РАСТУТ с игроками
  (broadphase? чанк-сеты? sync/packet?) → кандидат на следующий рычаг.

INFRA-рулетка v2 (TASK-391/392): emitted-notice band regex, классы
INFRA-SCRIPT-FAIL / INFRA-DELIVERY-FAIL (curl 503/FATAL world) / CRASH-REFUTED.
"""
import os, re, statistics, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RESDIR = "/home/z/c-crussty/research/gc-recon-2026-09-19"
RUN_DIR = os.path.join(RESDIR, "run-s7207-players32")
BANK_POINTS = ((2.6, 8_551_924), (2.2, 6_653_417))
BAND_MIN, BAND_MAX = 6_000_000, 9_500_000
TOTAL_PAUSE_GATE_MS = 22_000.0
MAX_PAUSE_GATE_MS = 3000.0
FULL_GATE = 10
# БАНК v4 + fp=32; все refuted-флаги = 0 (урок TASK-392: want == dispatch inputs)
WANT = {"gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
        "region_threads": "4", "batch_collector": "1", "fluid_guard": "1",
        "fluid_bitmask": "0", "fluid_dirty_ledger": "0", "fluid_dirty": "0",
        "inside_bitmask": "0", "travel_diet": "0", "skip_store_bb": "0",
        "region_steal": "0", "bu_defer": "0", "fake_players": "32"}
ARMED_BANK = "[crussty-plugin] fluid_guard: pristine sighting"


def api(tok, url, method="GET"):
    req = urllib.request.Request(url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.loads(r.read() or b"{}")


import json  # noqa: E402


def token():
    return open("/tmp/gh_token").read().strip()


def download_artifact(tok, run_id):
    os.makedirs(RUN_DIR, exist_ok=True)
    d = api(tok, f"{API}/repos/{REPO}/actions/runs/{run_id}/artifacts")
    for a in d.get("artifacts", []):
        if a["name"] == "world3-bench":
            zpath = os.path.join(RUN_DIR, "world3-bench.zip")
            req = urllib.request.Request(a["archive_download_url"], headers={
                "Authorization": f"Bearer {tok}"})
            with urllib.request.urlopen(req, timeout=300) as r, open(zpath, "wb") as f:
                f.write(r.read())
            subprocess.run(["unzip", "-o", "-q", zpath, "-d", RUN_DIR], check=False)
            return True, os.path.getsize(zpath)
    return False, 0


def fetch_joblog(tok, run_id):
    d = api(tok, f"{API}/repos/{REPO}/actions/runs/{run_id}/jobs")
    out = ""
    for j in d.get("jobs", []):
        req = urllib.request.Request(
            f"{API}/repos/{REPO}/actions/jobs/{j['id']}/logs",
            headers={"Authorization": f"Bearer {tok}"})
        with urllib.request.urlopen(req, timeout=120) as r:
            out += r.read().decode(errors="ignore")
    return out


def env_flag(env_txt, key, want):
    m = re.search(rf"^{key}: (\S+)", env_txt, re.M)
    return bool(m) and m.group(1) == want


def main():
    run_id = int(sys.argv[1])
    tok = token()
    run = api(tok, f"{API}/repos/{REPO}/actions/runs/{run_id}")
    print(f"run {run_id}: status={run['status']} conclusion={run['conclusion']} "
          f"head={run['head_sha'][:7]}")
    rep = [f"# absorb players-32 s7207 (run {run_id}, head {run['head_sha'][:7]}) — "
           f"ДИАГНОСТИЧЕСКИЙ (карта конкарренси ЦЕЛЬ-ДЕПЛОЙ, без банкинга)"]
    verdict = None
    have_art, _ = download_artifact(tok, run_id)
    stdout_txt = ""
    lp = os.path.join(RUN_DIR, "world3-bench.log")
    if os.path.isfile(lp):
        stdout_txt = open(lp, errors="ignore").read()
    env_txt = ""
    ep = os.path.join(RUN_DIR, "run-env.txt")
    if os.path.isfile(ep):
        env_txt = open(ep, errors="ignore").read()

    med = runner = None
    gc_stats = {}
    if stdout_txt:
        t1 = [f"{k}={'OK' if env_flag(env_txt, k, v) else 'BAD'}" for k, v in WANT.items()]
        ncde = stdout_txt.count("NoClassDefFoundError")
        pop_ok = "POPULATION FIXTURE-VALIDITY: VALID" in stdout_txt
        col = "OK" if "Using Parallel" in stdout_txt else "NOT-PARALLEL"
        t1_all = all("OK" in x for x in t1)
        rep.append(f"- PG-T1: {', '.join(t1)}, NCDFE={ncde}, pop={'VALID' if pop_ok else 'INVALID'}, "
                   f"col={col}, fp-marker={'OK' if env_flag(env_txt, 'fake_players', '32') else 'BAD'} "
                   f"-> **{'PASS' if t1_all and ncde == 0 and pop_ok and col == 'OK' else 'FAIL'}**")
        m = re.search(r"runner_cpu_index: (\d+)", env_txt)
        if m:
            runner = int(m.group(1))
        meds = re.findall(r"median5[=: ]+([0-9.]+)", stdout_txt)
        if meds:
            med = float(meds[-1])
        for pat, key in ((r"total[_ ]pause[=: ]+([0-9.]+)s", "total"),
                         (r"young[=: ]+(\d+)", "young"), (r"Full[=: ]+(\d+)", "Full"),
                         (r"avg[=: ]+([0-9.]+)ms", "avg"), (r"max[=: ]+([0-9.]+)ms", "max")):
            mm = re.findall(pat, stdout_txt)
            if mm:
                gc_stats[key] = mm[-1]
        threw = stdout_txt.count("ReportedException")
        rep.append(f"- PG-T2: threw={threw} -> **{'PASS' if threw == 0 and med else 'FAIL'}**")
        if runner and med:
            (t1v, r1), (t2v, r2) = BANK_POINTS
            slope = (t1v - t2v) / (r1 - r2)
            texp = t2v + slope * (runner - r2)
            d_abs = (med - texp) / texp * 100
            d_fp16 = (med - 2.2) / 2.2 * 100 if False else None
            rep.append(f"- PG-T3 ИНФОРМАЦИОННЫЙ: runner={runner} "
                       f"(банд {BAND_MIN}..{BAND_MAX}: "
                       f"{'OK' if BAND_MIN <= runner <= BAND_MAX else 'OUT'}), median5={med}")
            rep.append(f"  vs TPS_exp@fp4-якорь {texp:.2f}: absolute {d_abs:+.1f}% "
                       f"(fp=32 цена конкарренси; fp=16 была -0.2%)")
        if gc_stats:
            tot = float(gc_stats.get("total", "0") or 0)
            ful = int(gc_stats.get("Full", "0") or 0)
            g4 = ("OK" if tot <= TOTAL_PAUSE_GATE_MS / 1000 else "OVER") if tot else "n/a"
            rep.append(f"- PG-T4: young={gc_stats.get('young','?')}, Full={ful}, "
                       f"total={gc_stats.get('total','?')}s (гейт <= {TOTAL_PAUSE_GATE_MS/1000:.0f}s), "
                       f"avg={gc_stats.get('avg','?')}ms, max={gc_stats.get('max','?')}ms -> {g4}")

    # PG-T5 профиль-карта: top-лейны при fp=32
    cp = os.path.join(RUN_DIR, "cpu-collapsed.txt")
    if os.path.isfile(cp):
        lines = open(cp, errors="ignore").read().splitlines()
        total = sum(int(l.rsplit(" ", 1)[1]) for l in lines if l.rsplit(" ", 1)[1].isdigit())
        fams = {
            "fluid(n-push/getFluidState)": r"updateFluidHeightAndDoFluidPushing|getFluidState|FluidState",
            "paletted-reads": r"PalettedContainer\.",
            "broadphase(entity-lookup)": r"EntityLookup|CollisionUtil",
            "collections/fastutil": r"it\.unimi\.dsi\.fastutil|ObjectOpenHashSet|Object2ObjectOpenHashMap|Long2ObjectOpenHashMap",
            "volatile/inside": r"checkInsideBlocks|collidedWithFluid",
            "player-packet/netty": r"netty|PlayerConnection|ServerGamePacketListener|Clientbound",
            "chunk-send/save": r"ChunkMap|sendChunk|PlayerChunkSender|serialize",
            "mob-ai/brain": r"Brain|behavior|GoalSelector",
        }
        rows = []
        for name, pat in fams.items():
            s = 0
            for l in lines:
                parts = l.rsplit(" ", 1)
                if len(parts) == 2 and parts[1].isdigit() and re.search(pat, parts[0]):
                    s += int(parts[1])
            if s:
                rows.append((s, name))
        rows.sort(reverse=True)
        rep.append("- PG-T5 ПРОФИЛЬ-КАРТА fp=32 (vs банк-базлайн RECON-42):")
        for s, name in rows[:8]:
            rep.append(f"    {name}: {s}/{total} = {s/total*100:.1f}%")
        # дельта-vs-RECON42 для fluid и fastutil
        for s, name in rows:
            if "fluid(" in name:
                rep.append(f"  fluid-family при fp=32: {s/total*100:.2f}% "
                           f"(RECON-42: 14.59%) -> "
                           f"{'РАСТЁТ' if s/total > 0.1459*1.05 else 'плоский/пал'}")
        if not rows:
            rep.append("- PG-T5: профиль пуст -> N/A")
    else:
        rep.append("- PG-T5: cpu-collapsed отсутствует -> N/A")

    if verdict is None and run.get("conclusion") != "success":
        jl = fetch_joblog(tok, run_id)
        band = bool(re.search(r"::notice::runner_cpu_index=\d+ OUTSIDE band", jl))
        unbound = bool(re.search(r"unbound variable", jl))
        delivery = bool(re.search(r"FATAL: (world|purpur|natives).*failed|curl: \(\d+\).*50[023]", jl))
        crash = sum(jl.count(x) for x in ("Encountered an unexpected exception",
                                          "ReportedException"))
        if unbound and not band:
            verdict = "INFRA-SCRIPT-FAIL"
        elif delivery:
            verdict = "INFRA-DELIVERY-FAIL (fetch-хардинг 785b0a4 стоит; внешнее хранилище)"
        elif band:
            verdict = "BAND-DISCARD"
        elif crash:
            verdict = "CRASH-REFUTED"

    if verdict is None and med and runner and BAND_MIN <= runner <= BAND_MAX:
        verdict = "ПРОФИЛЬ-КАРТА ГОТОВА — see PG-T5; следующий рычаг = растущий с игроками под-лейн (мега-буст мандат)"
    elif verdict is None:
        verdict = "INFRA-FLAKE (неполные данные)"

    rep.append(f"\n## VERDICT: **{verdict}**")
    out = os.path.join(RESDIR, "ABSORB_S7207.md")
    open(out, "w").write("\n".join(rep) + "\n")
    print("\n".join(rep))
    print(f"\nwritten: {out}")


if __name__ == "__main__":
    sys.exit(main())
