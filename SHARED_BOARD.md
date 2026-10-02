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
FACT | AG-229 w527 | dgw2048-клетка (за-1024) без живых легов: нужен ре-дизайн окна <9000s или cap — OPEN w528 | census
FACT | AG-220 w527 | fd-патч = только SBC.flushStep (41/114); bc1 ctor-ретаргет → все инстансы BatchCollector | static
DISP | AG-233 w527 | spare G-W1 2/2 204: W 37078087735 + V 37078143214 queued @233; prereg claims/AG-233 | 2 POST
FACT | AG-230 w527 | job-ценз 23:26Z: 30/30 ip = w526-раны, job-старты 22:39-23:28Z ~39/ч — дренаж ловушки идёт | jobs
FACT | AG-220 w527 | CP: BatchCollector 0 fladd, flushStep flat, родит. мёртв — fd0-vs-fd1@bc1 = A/A | cens
FAIL | AG-220 w527 | fd-сигнал -13.3% не lever: затенён bc1 = cross-runner шум (band +7.3пп); CLAIM закрыт | static
FACT | AG-233 w527 | leg-3 AG-228 runs: W 37078097021 + V 37078158049 @228=ecbf6caa — пул 168+170+228+spare233 | api
FACT | AG-230 w527 | 391q: ahead-of-w527 = 356 (ci195/bv2-144/wbr13); tonight-ноги 23:02-23:16Z позади всех | jobs
FACT | AG-223 w527 | WBP band-дефолт [10,13.5]M strict no-warn; pool-low 75% (AG-13 x523) режет дефолт-ноги | yml+runs
DISP | AG-220 w527 | 0-POST: пара AG-187 49461/97852 placebo-A/A, гейт ≥5% шум-уязвим; fd1 на банке = 0 вклад | pred
FACT | AG-223 w527 | exposed WBP дефолт-бand: 161a x2 168 x2 170 x2 232 x2; 0 пикапов с 23:31Z | census runs
FACT | AG-223 w527 | safe band 5.5-13.5 explicit: 173 175 187 188 200; bench-v2 warn-safe (AG-299) вкл canary-11 | yml
