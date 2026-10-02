# ROUND-401 VERDICT (TASK-401, тик 14:02→18:0x +08, cron 401462/402447)

## База сравнения
- master d26f524 (= код 54806ad, diff docs-only). Банк v4 ЧИСТЫЙ (lever_flag="").
- Свежая якорная тройка same-day (абсорбы round-r401anc{1,2,3}): **2.2@7125977 / 2.6@7237388 / 2.3@6797859**
  — line-дельты same-sha −4.3%..+11.9% (median 2.3) — шум якоря жив (±2пп+ класс), pair-by-runner обязателен.
- TASK-400 якоря: 2.4@7071566 / 2.3@6940803 / 2.4@8344070 / 2.35@7112096 / 2.2@6811622 (@54806ad, код тот же).

## Карта векторов (MEGA-РАУНД-5; все legs ARMED-пруф по server-stdout; вердикт = pair-by-runner)

| вектор | ветка | leg1 (ARMED) | runner | line | PAIR (ближ. якорь) | вердикт |
|---|---|---|---|---|---|---|
| A mob spatial-hash (rust 64-shard seqlock grid, push-lane getEntities 9.45%) | round-401-a-mobhash f445f06 | 2.2 | 6297987 | +3.5% | 2.2 vs 2.3@6797859 = **−4.3pp** (vs 2.2@6811622 0.0pp) | PARITY/UNPROVEN |
| B collide sweep | round-401-b-collide (бенч не дошёл) | — | — | — | — | IN FLIGHT (итерирует) |
| C nav subsystem replace | round-401-c-navsys 8904e08 | CI fail ×1 | — | — | — | IN FLIGHT (итерирует) |
| D off-thread stage (bg-prep grid 64-shard, window 5³) | round-401-d-offthread 94a5023 | 2.1 | 6637215 | −4.4% | 2.1 vs 2.3@6797859 = **−8.7pp** | RED (bg-prep дороже экономии) |
| E SoA flat arrays (rust mobs_soa x/y/z/hw/hh/flags) | round-401-e-soa 9300ad0 | 2.45 | 6584415 | +12.1% | 2.45 vs 2.3@6797859 = **+6.5pp** | **ТОП-КАНДИДАТ (1 нога, min-of-3 нужен)** |
| F OSS-порт Lithium unpushable_cramming (MobOssOps) | round-401-f-ossport d134458 | 2.3 | 8236912 | −9.2% | 2.3 vs 2.4@8344070 = **−4.2pp** | RED |
| G event-driven wakeup (9-site serverAiStep retarget, sleep/wake) | round-401-g-wakeup da9f6b5 | 2.4 | 8767633 | −9.3% | 2.4 vs 2.4@8344070 = **0.0pp** | PARITY (sleep-экономия съедена) |
| H КОМПОЗИТ bfcomp+devirtfix+mobpush (+wakefix gate), cmp401_comp | round-401-h-comp 167031d | CI fail ×1 | — | — | — | IN FLIGHT (итерирует) |
| I staggered тяжёлых проверок | round-401-i-stagger d47d944 | CI fail ×2 | — | — | — | IN FLIGHT (итерирует) |
| J девиртуализация SynchedEntityData get+set fusion | round-401-j-devirt 43e5f8c | 2.2 | 6694000 | −0.4% | 2.2 vs 2.3@6797859 = **−4.3pp** | RED/PARITY (fusion не конвертируется) |

## Mobpush min-of-3 (добор TASK-400)
- leg1 +6.8pp pair @6410427 (ARMED) — стоит. leg2 35565865219 BAND-OUT 5946251 → discard.
- Ре-роллы jmob3a/jmob3b = BAND-DISCARD ×2 (11702139 / 10089523, быстрый пул) — бюджет ≤2 исчерпан.
- jmob2 35565178476 inject-stall 900s (инфра-флейк ×1 — не ≥3 одной причины, root-fix не мандат).
- Итог: mobpush = **+6.8pp pair, UNPROVEN beyond leg1**; добор через композит H (cmp401_comp включает mobpush).

## Мерж
- БАР = +80% pair-stable (директива владельца). Лучший вектор раунда E-soa +6.5pp (1 нога) — **МЕРЖА НЕТ**.
- Композиции: сумма реплицированных (B +4.2 ⊕ J-mobpush +6.8(1) ⊕ C-devirt +4) ≈ +15пп — ступень, не бар; композит H в полёте.

## Инфра-уроки тика
- Диск 100% на старте тика → чистка target-*/collapsed-stack (−4G). Правило: PHASE 0 = + disk hygiene.
- Adapter-timeout ВСЕХ 10 Task-вызовов снова; 8/10 довели пайплайны фоном (ветки+ноги+RESEARCH). Рестарт только D/E (реальный застой 55 мин) — оба пере-диспатчнулись и дали legs за ~20 мин.
- Line TPS_exp снова завышает на быстрых раннерах (G: line −9.3% при pair 0.0; E: line +12.1% при pair +6.5).

## NEXT-402
1. **Репликация E-soa** min-of-3 (top-кандидат, ветка+гейт готовы) — при стабильных ≥+5pp входит в композит.
2. **Композит H** — добить gate-fix/CI, leg pair vs якоря; кандидаты B+J(mobpush)+E-soa композиции.
3. Итерации B/C/I до ARMED-leg; D/F/J честно в карту (RED — механики не конвертируются).
4. Якори 2-3 fresh/тик (закон pair), disk hygiene в PHASE 0.
