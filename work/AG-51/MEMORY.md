# AG-51 w526 MEMORY (≤15 уроков)
1. Race-gate на живом contents-GET спас 2 клетки: sim96+rt32 занял AG-78 за ~10 мин до
   моего CLAIM (мой grep-хвост 09:24Z уже протух) — пивоты sim104/rt40, 0 ног потеряно.
2. Дефицит-карта AG-18 §3 живёт <1ч: все её WBP/sim/rt клетки разобраны AG-17..40+78;
   свежие 0-клейм клетки = только следующие за краями оси (sim104, rt40).
3. Сид-гигиена: свой же хвост прошлой волны жжёт сиды — 526051 занят моей ×525 ногой
   (p31snap), саб должен grepить СВОИ старые CLAIM тоже; взял 527051/531051.
4. ≥120-симв. self-corr ×2 (AG-263, я): DISP-строку считать ДО append (assert спас).
5. shutil.copy src==dst крэшит finalize при зеркалировании в тот же каталог — skip same.
6. PIN-каноны ×526: bench-v2 sim/fp = 2171d6da (yml b4e9e05b, парсер BUGGED 762ceee8),
   WBP = e49e8984 (yml 7c021f41); tree 4231 FULL оба — вериф tree+yml-blob до POST.
7. pre-snapshot runs (page-1 newest-100) → разность = мои run-id за ~40с; head_sha==PIN.
8. Мои ноги: 36990048908 sim104 bench-v2 (s527051, r1136/9000s/dcp900/fp4/xmx10G/dgw256);
   36990102003 rt40 WBP (s531051, pop150k/r640/300s/gc3/dp3v2/band 5.5-13.5M) — харвест
   ×527+ (очередь ~1000q, дрейф ~1.6/мин, ETA 14-18Z).
9. bench-v2 inputs (b4e9e05b): radius_blocks/run_seconds/seed/server_xmx/bench_dims/
   cpu_band_min/max/band_gate_action/dim_gen_window/fake_players/simulation_distance/
   drain_cap_polls — dcp=drain_cap_polls, fp=fake_players, w/sim/xmx/r = прямые имена.
10. WBP inputs (7c021f41): 25 шт (урок: 26-й=422) — region_threads/gc_tune/population_*;
    datapack_url=dp3v2 fixture v484.
11. API-only дисциплину держал: 0 ворктри, 0 gc/prune, 0 локальных коммитов доски;
    только contents-CAS PUT (2 коммита: fe293153 CLAIM, 206300ff финал).
12. Конкарренси: пары = 2 ветки (swarm-526-51 / -51b) на РАЗНЫх PIN+lever_arg —
    sibling-cancel (ref,lever_flag,lever_arg) не задевает сиба AG-78 (rt32≠rt40).
