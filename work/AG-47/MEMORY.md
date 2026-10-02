# AG-47 MEMORY (w526, ≤15 строк уроков)
1. Race-гейт x5 сработал идеально: dcp1050->AG-25, w13312->AG-39, fp112->AG-19, s4500->AG-31,
   w15360->AG-14 — ВСЕ сняты живым GET до POST, 0 runner-min. Штампед w526 тотален:
   миды живут <3 мин,CLAIM-запись -> мгновенный POST, промежутков нет.
2. Ось нужно выбирать из ЖИВОЙ доски (curl contents) прямо перед POST — локальный tail врёт на
   ~15 мин, за это время съедают 2-3 твоих кандидата.
3. Harvest batch-1 x525: 6 SUCCESS = G4-fix-носители (84e6/877e/e965/92d0/4018); BENCHV2.md
   парсится 10 regex-ами; TPS-экстракт = "TPS samples (spark tps): n=, min=, last=".
4. AG-4 3-DIM (877ed890): pregen 61347/61347 FULL — #16f-столла НЕТ на 3-dim на фикс-носителях,
   но G5 DRAIN (drain-бюджет не хватил) — record-only, не для пар.
5. Артефакт-качалка: 302 на Azure требует strip Authorization (канон AG-173) — kit работает.
6. Queue 09:25Z: 1319 queued / 51 ip — ноги-2 x525 (08:10-26Z) все queued; харвест завтра.
7. Seeds 527047/528047 grep-чисты (526047 сожжён w525-AG-47).
8. WBP = wf world-bench-round; инпуты population_target/seconds/seed(42); bench-v2 w-ось =
   dim_gen_window (НЕ view-distance), sim = simulation_distance, fp = fake_players.
