#!/usr/bin/env python3
"""gen_stz110_c2_fixture.py — STZ-110 «С2 датапак-давление» fixture generator (wave-519 AG-80).

С2 (LEDGER Л-466-C94.3, приоритет 1): 5k mcfunction (200 каждотиковых) → fn-плоскость под
давлением; стенд «мир под давлением» для dp-класса (прецеденты СТЗ-3v2 A14 sha 16fa1a32,
СТЗ-82 v506-stz82). v1 = чистое fn-давление, STATELESS: листья = `data get storage` (command
storage эфемерен, 0 персистенции, 0 entity-сканов → world-diff parity 0.0000% ожидаемо).

Layout 1.21.10 (pack_format 88, dual [48,88]): data/<ns>/function/*.mcfunction (singular),
data/minecraft/tags/function/tick.json. tick.json → 200 каждотиковых корней c2:t0..t199;
каждый корень вызывает 24 детей c2:f<i>_<j>; каждый ребёнок = 1 read-only leaf.
Ops/тик = 200 roots + 4800 calls + 4800 leaves = 9800 (~15% op-cap 65k).
Прогноз (Л-482-C13.2 бэнды): 9800 × 0.2-0.5µs = 2-5ms/тик = 4-10% MSPT @50ms.
"""
import json, zipfile, hashlib, sys, os

N_ROOTS = 200
N_CHILD = 24
OUT = sys.argv[1] if len(sys.argv) > 1 else "/tmp/stz110/stz110-c2-fixture.zip"

def build():
    files = {}
    files["pack.mcmeta"] = json.dumps({"pack": {
        "pack_format": 88, "supported_formats": [48, 88],
        "description": "STZ-110 C2 datapack-pressure (stateless fn-stand)"}},
        separators=(",", ":"))
    files["data/minecraft/tags/function/tick.json"] = json.dumps(
        {"values": [f"c2:t{i}" for i in range(N_ROOTS)]}, separators=(",", ":"))
    for i in range(N_ROOTS):
        files[f"data/c2/function/t{i}.mcfunction"] = "\n".join(
            f"function c2:f{i}_{j}" for j in range(N_CHILD)) + "\n"
    for i in range(N_ROOTS):
        for j in range(N_CHILD):
            files[f"data/c2/function/f{i}_{j}.mcfunction"] = "data get storage stz_c2:buf k\n"
    return files

def main():
    files = build()
    n_fn = sum(1 for p in files if p.endswith(".mcfunction"))
    assert n_fn == N_ROOTS + N_ROOTS * N_CHILD, n_fn
    with zipfile.ZipFile(OUT, "w", zipfile.ZIP_DEFLATED) as z:
        for p in sorted(files):
            zi = zipfile.ZipInfo(p, date_time=(2026, 1, 1, 0, 0, 0))
            zi.compress_type = zipfile.ZIP_DEFLATED
            zi.external_attr = 0o644 << 16
            z.writestr(zi, files[p])
    data = open(OUT, "rb").read()
    sha = hashlib.sha256(data).hexdigest()
    print(f"{OUT} {len(data)}B sha256={sha} mcfunction={n_fn} ops_per_tick={N_ROOTS + N_ROOTS*N_CHILD*2}")

if __name__ == "__main__":
    main()
