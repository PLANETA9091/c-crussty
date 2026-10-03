# AG-463 MEMORY (≤15 уроков)
1. rt-пламбинг WBP цел: input→env→run_world3.sh:447→region_threads.rs (гейт ≥2, fail-closed).
2. bv2 = vanilla-purpur лейн: 0 crussty-рефов в run_benchv2.sh — rust-рычаги на bv2 структурно мертвы.
3. rt8×dgw 1-POST не исполним: dgw=bv2-only, rt=WBP-only; нужен module-port lane-fusion.
4. "Двухстрочный" yml-патч без модуля = SILENT-DORMANT FAKE-нога (не читает никто).
5. prereg AG-444 404-фантом (master x3 + ветка нет) — 3-й за 2 волны; prereg только на master.
6. Poison-scan '#': master чист (0 в value-литералах), фикс 206/219 жив.
7. CAS-борд: GET-len>700k guard → PUT → re-GET вериф (0 конфликтов за саб).
8. CI-очередь p50-age 8h → POST-ы сейчас = w528, 0-POST DISP легален.
