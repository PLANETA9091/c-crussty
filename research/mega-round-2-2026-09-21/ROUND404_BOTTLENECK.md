# ROUND-404 BOTTLENECK (TASK-404, 2026-09-21 23:08→, cron 402447)

## База
- master e9bd2dc (= код 348510f, банк v4). ТОП ступень эры: **stagcomp (cmp402_stagcomp @60fe902) = +25pp pair-медиана ×5 ARMED-ног** (2.35/2.7/2.9/2.9/3.15).
- Профиль stagcomp-ноги (lane_map stc5, 109k сэмплов): items 0% (rust), fluid 17.59% (топ java-лейн), broadphase 13.77%, nav_ai 8.60%, inside 11.46%, fastutil 7.52%, java_util 7.16%, paletted 6.74%.
- JNI-граница 12-17% wall (скрытая цена rust-путей) — jnibulk leg1 +4.3pp pair (открыт, не добит).
- Якорная линия 403: 2.1-2.4 (5 ног); 404-якоря диспатчены (@e9bd2dc).

## ТОП-3 лейна поверх stagcomp (путь к 80%)
1. **nav+push+collide плоскость ~30-35% wall** (nav_ai 8.6% + broadphase/collide 13.8% + push в перемещении) — единственный честный путь к бару: единый rust-плейс тела моб-тика (класс «замена подсистемы», прецедент J-subsys2).
2. **fluid 17.59%** — точечные рычаги refuted ×4 (fluid_dirty/bitmask/guard), остаётся замена дата-плейна fluid-скана подсистемой (разрешено законом 5).
3. **JNI-граница 12-17% wall** — jnibulk leg1 +4.3pp, min-of-3 + углубление механики.

## План тика
- Якоря ×3 + stagcomp дрейф-нога (6-я в серии).
- Волна 1: A navplane (MEGA) / B jnibulk-min-of-3 / C fluidplane — все на базе 60fe902 (композиция поверх топ-ступени), STRICT флаги cmp404_*.
- Базовая линия для их вердиктов: **stagcomp-пул** (2.35@6765263 / 2.7@6811639 / 2.9@8785618 / 2.9@7120479 / 3.15@8537609) — дельта X = (stagcomp⊕X) vs stagcomp; кумулятив vs master-якоря.
- Root-caused и исправлен absorb-тул: push-CI раны (5 джоб без world-bench) = NOT-A-BENCH, не INFRA (гвард в main()).

## Законы вердикта (неизменны)
Pair-by-runner + min-of-3; линия TPS_exp не для мержа; BAND 6.0-9.5M; ARM-маркер обязателен; parity PASS; RAM/CPU без регресса; ванильность.

## ИТОГ TASK-404 (interim)
- Якоря: 2.1@6781844 / 2.6@9297453 валидны; 2.1@5795289 BAND-DISCARD.
- stagcomp дрейф-лега: 2.2@8794534 ARMED ×10 → pair −15.4pp; серия 6 ног медиана +20.8pp (шум высок — день-4 репликация NEXT-405).
- jnibulk FUSED (bfeafe8): jb2 +10.9pp / jb3 0.0pp pair (медиана +5.5pp, ARMED ×9 оба; leg3 NEXT-405).
- navplane (agent A, гейт-чеклист правки) / fluidplane (agent C, имплементация) — в работе, ноги = тик 405.
- БАР 80% НЕ ВЗЯТ — МЕРЖ НЕТ.
