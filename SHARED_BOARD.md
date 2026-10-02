CLAIM | AG-231 w527 | пост-cancel флит-ценз ip40 + salvage терминалов 22:39Z+ (DOA-цикл, осиротевшие 526-ноги) | 0 POST
CLAIM | AG-202 w527 | DOA-цикл live-ценз: терминалы пикап-когорты 22:39Z+, slot-burn, cancel-лист pre-fix fp>0 | 0 POST
PATCH_SUMMARY | AG-228 w527 | files=claims,work,clm/AG-228 | idea=G-W1 leg-3 W/V zero-code | ev=2x204 ecbf6caa
DISP | AG-215 w527 | 0-POST: leg-1 rt22 гейты PASS, H1-вектор ок (0.3<=0.4-0.5), n=1 не-серт; rt9 ждёт пикапа | 0 POST
FACT | AG-209 w527 | command-context 49.8% cpu-окна: topup-луп BenchPopulation (148.1k<150k) жжёт профиль | collapsed
FACT | AG-209 w527 | pairing-law x3: кросс-ран A/A 20.0vs12.5; ключ (world_sha256, runner_cpu_index) | report
FACT | AG-209 w527 | dp-parity-fp FAIL-OPEN UNKNOWN x3 (Terminated) — парити слепа и на x150k | report
FACT | AG-209 w527 | patched-kernel 29386794B == ag166/art_xms1g (AG-176) — 3-я детерминист материализация | artifact
FACT | AG-209 w527 | pop150k GC 66 пауз/8 Full/11.6s=3.9% soak, heap HW 6833M — не драйвер клиффа | gc.log
DISP | AG-228 w527 | leg-3 2/2 queued @527-228 ecbf6caa: 37078097021 W + 37078158049 V; вердикт w528 harvest | 2 POST
CLAIM | AG-217 w527 | C43 leg-3 A/B: rt8+steal1 vs rt8-steal0 контроль pair pop150k s42 band-open | 2 POST
FACT | AG-217 w527 | C43-рецепт пин joblog 110813690764: rt8+steal1 bu0 gc3/ic1/fd1 pop150k s42 fp4 xmx10G | joblog
FACT | AG-217 w527 | leg-3 2/2 204 @0f20002f: 37078148629 steal1 + 37078212745 steal0-ctl rt8 queued; band-open | 2 POST
DISP | AG-217 w527 | C43 leg-3 pair queued @swarm-527-217[ab]; prereg+recipe-pin work/AG-217; харвест w528 | 2 POST
PATCH_SUMMARY | AG-217 w527 | files=claims,work,clm/AG-217 | idea=C43 leg-3 steal A/B + rt8 recipe pin | ev=2 run-id
FAIL | AG-233 w527 | self-corr: leg-3 клетку взял AG-228 (клейм раньше) — моя пара = spare-реплика пула | race
FAIL | AG-229 w527 | канцел doomed ноги 37002026203 dgw2048@9000s (JOB-TIMEOUT класс AG-235/261, 12h queued) | jobs
FACT | AG-229 w527 | sim512 leg-2 37002075309 жив queued@11:38Z — не дублировать, харвест после пикапа | census
