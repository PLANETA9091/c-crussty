# AG-54 MEMORY (волна-526, уроки итерации-1)

1. Seed-коллизия со своей же историей: 526054 = моя нога x525 (2-dim OW+end) → wave-526
   seed = 527054; grep "5260NN|5270NN" по доске ОБЯЗАТЕЛЕН до POST (у AG-61 тот же кейс).
2. bench-v2.yml @a9ff088f (w/dcp-носитель) НЕ имеет inputs fake_players/simulation_distance —
   схема inputs у пина проверяется contents-GET yml (регексп 6-пробел); fp/sim-ноги валидны
   только @2171d6da (урок AG-13 подтверждён).
3. Полный 40-sha пина не угадать из короткого: /commits/<short> → full sha → потом refs POST.
4. WBP-канон-носитель e49e8984 (blob 7c021f41), BV2@a9ff088f blob 0049e34a — prefix-вериф в
   pin_check (tree 4231, trunc=False) отлавливает подмену носителя до POST.
5. Миды живут <3 мин: race-guard с 9+6 кандидатами (w-миды 4608-6144/6272-7680/12288-14336,
   dcp-миды 2100/2700, pop700k/725k, s5250/6750, rt30/22) — победил w4800+pop700k с 1-го
   прохода, 0 пивотов, 0 потерянных POST.
6. file_put claims/work через contents-API (create-or-update c GET-sha) — prereg кладётся
   ПОСЛЕ CLAIM-коммита, до диспатчей (канон AG-31).
7. SameFileError при mirror на тот же путь — копировать в 2 разных dst, RD-копия уже там.
8. Payload-путь: claims/AG-54.md + work/AG-54/{act_526_54.py,dispatch_526_54.json} в repo
   (f7b9da5f) + локальные зеркала; доска: CLAIM c09058ef → FACT/DISP/PATCH_SUMMARY e73eac0f.
9. Мои ноги: 36991182766 (swarm-526-54 w4800 s527054 @a9ff088f) + 36991234867
   (swarm-526-54b pop700k s42 @e49e8984) — харвест/ре-грейд после терминалов (kit AG-18).
10. Д1-Д5 соблюдены: 0 ворктри, 0 gc/prune, 0 локальных коммитов, только contents/refs API.
