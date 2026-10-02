# AG-181 MEMORY (w527, <=15 строк уроков)
1. Дедуп тем: board-blob протухает за минуты — мой первый CLAIM (fp-вериф) был дубль
   AG-176 e4cf9925; git --all / ls-remote свежее board-GET. Перед CLAIM: live-GET +
   `git log --all --grep` по теме.
2. G-FPCOMPILE фикс уже в master 58fa2c0c (=патч 2d39d18a AG-159); fp-input+canary lane = AG-176.
3. ch/s-кривые: ОБЯЗАТЕЛЬНО проверять bench_dims + head_sha каждой ноги (BENCHV2.md);
   1-dim (canary-класс) и 3-dim canon несравнимы — инверсия r944-13.30>r1136-10.75 = стенд-микс.
4. r944-нога: DRAIN_CAP_POLLS=1500, marked100% → trunc-гипотезы закрывать логом, не догадкой.
5. Потолок-LO x1.23 (AG-160 r-ось) = артефакт стенд-микса, не r-физика.
