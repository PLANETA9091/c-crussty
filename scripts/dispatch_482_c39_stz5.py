#!/usr/bin/env python3
"""dispatch_482_c39_stz5.py — COMMANDER C39: СТЗ-5 Moonrise #191 structure-gen race stand (v19.0 ×482).

CLAIM (прегист закон 14a/16, ДО диспатча):
  СТЗ-5 Moonrise #191 [Bug: NeoForge & Fabric] "Nether fortresses and strongholds are corrupted
  when vanilla structure generation runs in parallel" (OPEN, 2026-08-10, MC 26.2):
  STRUCTURE_STARTS = parallel-capable в chunk scheduler Moonrise → структура-старты генерируются
  concurrently на worker-threads; NetherFortressPieces (static BRIDGE/CASTLE_PIECE_WEIGHTS,
  mutable PieceWeight.placeCount) и StrongholdPieces (static currentPieces/imposedPiece/totalWeight)
  держат piece-weight selection state в SHARED STATIC FIELDS → два одно-типных старта на разных
  потоках контаминируют счётчики: (1) premature seal-off (крепость 1/4-1/5 ваниль-размера),
  (2) abnormal extension. Expected: "identical structure layout to vanilla for the same seed".

  JAVAP-ПИН на живом носителе (patched-kernel.jar из артефакта r36412293556, JDK21):
  ChunkStatus.moonrise$isParallelCapable()/setParallelCapable(boolean) — механизм ЕСТЬ;
  NetherFortressPieces$PieceWeight.placeCount public int + static BRIDGE/CASTLE arrays — ЕСТЬ;
  StrongholdPieces static currentPieces/imposedPiece/totalWeight/resetPieces() — ЕСТЬ →
  race-поверхность 1.21.10-ядра жива.

  ×481-r960-прецедент (r36388741837 + r36412293556, оба SUCCESS): 0 structure-исключений,
  0 "Structure" строк в server-stdout — pregen MineShield-3 (afb3a0b3) = load-only, fresh-gen
  structure-стартов НЕ бывает → race НЕ воспроизводим на pregen-мире (вывод: нужен fresh-gen
  фиксстура). Найдено вместо: bit-identical ×2 block-entity desync (8 lectern + 1 trapped_chest
  + 1 chest) на ChunkFullTask.runPostLoad — детерминированный артефакт world-data, НЕ race.
  GC-подпись ×2: 12 CodeCache + 8 Metadata Pause Full (whole-run; in-window 6CC+4MD — C06 канон).

  ЭТОТ диспатч = параллельный ЛОАД-ось стенда (не gen-race): r960 (16384-чанковый grid = 2.56×
  canon 6400) + rt8 (8 region-workers) + 960s окно на мастере 3666a793. Gen-race нога = СТЗ-5
  fresh-gen фиксстура (spacing-8 structure-set + nether-forceload) — materialize след. тик,
  one-scene-per-run (закон 20a).

ПРЕГИСТ-ГЕЙТ: G-band [6.0,9.5]M fast-fail; M1 STW<=23.0 ∧ young_avg<=200 (r960/960s HOST-класс
  19a известен — 12CC+8MD каскад; срыв M1 = фича ячейки, не lever-вердикт); NCDFE=0; AIOOBE=0;
  структура-осевые гейты: 0 structure-исключений ожидаются (pregen-мир), N structure-"Marked
  chunks" батчей = 64; парити-гейт 20d = S12 STRUCT-INTEGRITY GATE v1 (docs/S12_STRUCTGATE.md):
  semantic 169/169 + structure-markers digest + G-S1 seed-identity — ОБЯЗАТЕЛЕН для любого
  structgen-оптимизационного рычага до мерджа (structures bit-identical, repeat=2, ~0.9s/вердикт).

ЗАДАЧА: 1 ваниль-диспатч world-bench-parallel @3666a793 (branch round-482-c39-stz5 = vanilla-alias
  Л188b-прецедент), canon x466-C98 ЯВНЫМ JSON (урок C66-C72) + radius=960/seconds=960/rt8,
  lever ∅. Runs >15 мин → DISPATCHED run-id (закон 18-iii).

Usage: dispatch_482_c39_stz5.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-482-c39-stz5"
PIN = "3666a7931703e24a36af4887c41a585918667b7a"

# canon x466-C98 ЯВНЫМ JSON (урок C66-C72: yml-дефолты = merge-поверхность);
# СТЗ-5 ось: radius=960 (16384-чанк grid), seconds=960, region_threads=8 — остальное ваниль.
INPUTS = {
    "radius": "960", "seconds": "960",
    "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "8",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "482-c39-stz5-dispatch"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def branch_verified(tok):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{ALIAS}")
    sha = r.get("object", {}).get("sha")
    ok = sha == PIN
    print(f"branch {ALIAS}: {'GET-verified @ ' + sha[:12] if ok else 'MISMATCH ' + str(sha)[:12]}")
    return ok


def dispatch(tok, ref):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    print(f"dispatch {ref} r960/960s/rt8 -> {'204 OK' if r == {} else r}")
    return r == {}


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"], "url": run["html_url"]}
    return None


def main():
    if "--dry-run" in sys.argv:
        print(json.dumps({"alias": ALIAS, "pin": PIN, "inputs": INPUTS}, indent=1))
        return
    tok = token()
    if not branch_verified(tok):
        sys.exit("FATAL: ветка не GET-verified — диспатч отменён")
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    if not dispatch(tok, ALIAS):
        sys.exit(2)
    run = None
    for _ in range(6):
        time.sleep(15)
        run = latest_run(tok, ALIAS, mc)
        if run:
            break
    print("C39-DISPATCH-JSON " + json.dumps(
        {"alias": ALIAS, "pin": PIN,
         "prereg": {"band": "[6.0,9.5]M fast-fail",
                    "M1": "STW<=23.0 ∧ young_avg<=200 (r960/960s HOST-класс 19a известен)",
                    "axis": "parallel-LOAD r960(16384ch)/rt8/960s — gen-race = СТЗ-5 фиксстура next-tick",
                    "struct": "0 structure-исключений ожидаются (pregen MineShield-3 afb3a0b3, load-only)",
                    "parity20d": "S12 GATE: semantic 169/169 + markers digest + seed-identity"},
         "run": run}, indent=1))


if __name__ == "__main__":
    main()
