# AG-419 w527 — dgw448 харвест (leg s528419)

## Run-ids
- s528419 = 37019318796 — SUCCESS 05:04:40Z (пикап 01:56:45Z, runner GitHub Actions), арт 11265067365 «benchv2-ag433»
- s527419 = 37019227936 — ЖИВ in_progress @r1000036173 с 03:19:19Z, ETA ~06:19Z → харвест w528 (n=2 dgw448)
- Оба: @a9ff088f pin, 1d/r1136/9000s/dcp900, gen_window=448 (CLAIM w526)

## Лог-верификация s528419
- `Starting minecraft server version 1.21.10`; `armed mode=pregen-v3 worlds=[world] radius_chunks=71 cells_per_world=20449 gen_window=448`
- GEN-START 01:58:39 → GEN-DONE 02:24:54 all_marked=20449 elapsed=1594s
- 0 stall-маркеров; NCDFE=0, AIOOBE=0; G3 4/4, G4, G5 PASS; G-FP fake_players=0
- ВНИМАНИЕ: BENCHV2.md шапка «AG-433 wave-515» — это винтаж харнеса @a9ff088f, не чужая нога

## Числа
- pregen ch/s = 20449/1594 = **12.83** (drain-def: 20449/2109 = 9.70)
- sustain mspt-медиана ≈50.8 (spark n=808), tps last=20.0, min=11.18, n=809
- entity census TOTAL median 9799.5 (3 dim по 3266.5 — не-3dim-нога, G-DIM ov=21609)

## dgw-крива (pregen ch/s, GEN-DONE def)
| dgw | ch/s | n | источник |
|-----|------|---|----------|
| 192 | 8.56 | 1 | AG-216 ghost |
| 256 | 10.37-11.08 (мед ~10.67) | 6 | AG-216 ghost, spread 6.8% |
| 384 | 8.26 | 1 | AG-216 ghost «dip» |
| **448** | **12.83** | **1** | **AG-419 s528419 (этот харвест)** |
| 512 | 12.32 | 1 | AG-216 ghost |
| 6144 | 13.29 | 1 | AG-216 ghost (s528178 leg-2) |

- 448 = +20.2пп к 256-мед (10.67) при spread 6.8% n6 → сдвиг ~3σ-подобный, но cross-cohort + n=1: НЕ серт
- 448 vs 512: +4.1% (flat в пределах σ); 448 vs 6144: -3.5% — плато 448-6144
- dip 384=8.26 аномален: между 256~10.7 и 448=12.83 провал до 8.26 малоправдоподобен механически; верификация n≥2

## Caveats
- Ghost-числа AG-216 = cancel-когорта w526, мои = live w527; ядро-пин e2992d63 стабилен 14:51-17:26Z (AG-203), pin a9ff088f re-verif 14:2xZ — сопоставимость приемлема, не идеальна
- 12.83 n=1; min-of-3 same-boot канон — см. prereg в claims/AG-419.md
