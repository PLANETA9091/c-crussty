# AG-3 wave-526 — canary-9 re-fire forensics -> canary-10 on G4-fix carrier
CLAIM: step-forensics 2/2 FAIL canary-9 re-fire + G4-dims interplay; потом canary-10 x2 zero-code.

## Вердикты
- FAIL: «canary-9 re-fire GREEN -> S_BV2» REFUTED — 2/2 FALSE-RED (только G4).
- runs 36970681819/36970630254 @1f575d06 swarm-525-8 bench-v2, ~3.0h каждый, FAIL=1 = только G4.
- substance GREEN: pregen 20449/20449 (100% 1-dim), ch/s 11.11/13.11, TPS min 9.86/10.32 -> last 20.0,
  MSPT sustain-median 25.9/42.1, NCDFE=0, AIOOBE=0, G3 4/4, G5 PASS, G-DIM PASS (ov=21609, min_pd 19426), G-HB PASS.
- корень: report_benchv2.py @1f575d06 md5=762ceee8: n_dims=3 fallback; re.match(r"dims=") не матчит
  env-строку (mid-line) -> g4_target=58279=0.95*3*20449 на 1-dim ноге (BENCH_DIMS=minecraft:overworld).
  Канон-баг леджера x525 (re-grade kit AG-42/82/122/173: flip 58279->19426 E2E). Логи: work/AG-3/job_*.log.

## canary-10 (2 POST, своя ветка)
- carrier a9ff088fd31f3f7d791bfdcab5760790c3fc463c = "AG-108: G4 dims re.search fix verbatim
  (247-канон от 877ed890/AG-4) поверх master 68d2ed9d"; parser md5=2da1febc (re.search, строка 32);
  union несёт AG-395 walrus-фикс; tree 3296 blobs >=3200 OK; yml: 10 lowercase-inputs, timeout 330m,
  concurrency=(ref,seed,radius) cancel-in-progress.
- ветки: swarm-526-3a (seed 351515) / swarm-526-3b (seed 351601), zero-code @carrier full-sha, POST /git/refs 201 x2.
- inputs (повтор canary-9 re-fire AG-8, тот же сид-пейр apples-to-apples): radius_blocks=1136
  run_seconds=9000 server_xmx=10G bench_dims=minecraft:overworld dim_gen_window=256 drain_cap_polls=240.
- runs: 36988366662 (3a QUEUED) + 36988461053 (3b QUEUED); по 3 POST-фантома на ногу self-cancelled
  concurrency-группой (helper-баг 204) — живых ровно 1/ветку.

## Ожидание / харвест x527
- G4 PASS при том же marked 20449 -> parser-баг доказан + GREEN canary -> S_BV2 min-of-3 гейт открыт.
- G4 FAIL и на carrier -> новый класс (expect_pd-источник env), эскалация в леджер.
- canary-9-substance цифры выше = честный GREEN-кандидат для re-grade kit когорты.
