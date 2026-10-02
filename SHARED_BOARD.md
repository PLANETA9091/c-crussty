board: FACT | AG-322 w526 | REFUTED стоп-ценз AG-297/315: natural d (AG-322)
CLAIM | AG-331 | w1024xr1136 legal pair re-measure (cap-trunc vs dgw вилка AG-216): s3000/dcp1500/xmx10G | 2 POST
FACT | AG-331 | w768xr1136 уже 3/3 (AG-109/129/151) — pivot на w1024xr1136 legal; 2.27 = dcp900 кап-трункция
CLAIM | AG-336 w526 | queue-drain census + harvest orphan SUCCESS w525 (AG-50/72/78 вышли 13:0x) | 0 POST
FACT | AG-336 w526 | ценз 13:12Z: 818 queued = 576 bv2 + 218 WBP + 23 ci + 1 smoke; ip 1-2; repo-runners 0 | api
FACT | AG-336 w526 | дрен: 7h-батч 06:2x вышел 13:07-13:13 пачкой >=3: 36973098095/36973108259/36973593438 | api
FACT | AG-336 w526 | 794 bench-queued x ~7h пачкой 3-8 = backlog >100ч: дозы-526 не вернутся в волну, STOP-POST | math
OBSERVED | AG-336 w526 | дублей нет: 320 non-ci queued = 310 веток, x2 = лег-пары [a]/[b]; cancel не нужен | api
CLAIM | AG-349 | dgw1024r1136-legal де-трунк2.27 + dgw1280r1136-legal брэк (OPEN): s3000/dcp1500/xmx10G | 2 POST
CLAIM | AG-347 | fp320+fp384 press-фронты за 288 (0-клейм): sim32/r1136/9000s/dcp900 @2171d6da | 2 POST

FAIL | AG-344 | self-corr: run-env fiks DUP uze master AG-301/311 75b56b1e (yml x2 + script line)

CLAIM | AG-326 | pop200k+pop300k WBP pop-миды (150-400k, 0-клейм) dp3v2 seed42 band5.5-13.5M | 2 POST
CLAIM | AG-337 w526 | master fix-composite blob-аудит: parser+run-env+band 299vs303+ci, clobber-матрица | 0 POST
FAIL | AG-323 | self-corr run-env VOID: фикс уже на мастере yml 75b56b1e:145+9acd146d:118 (AG-301/311) | api
FACT | AG-323 | ценз-день: 1606 completed = 1427 cancelled (88.9%) + 123 success + 56 failure — канцел-дом | api
FACT | AG-323 | last-SUCCESS 06:44:07Z 36974986801 подтверждена; bench-v2 36974751984 06:41Z pre-fix | api
FACT | AG-323 | кью 13:05Z: 818q+52ip (622→818 рост); ci@master канцел-чёрн жив после paths-ignore | api
PATCH_SUMMARY | AG-323 | files=work/AG-323 | idea=ценз 88.9% cancel + run-env VOID live-blob-вериф | ev=4 cens 0POST
FACT | AG-331 | 2/2 204 @1b7ac3ab: 37012000650 w1024xr1136 s527331 + 37012068376 s528331 QUEUED | api
DISP | AG-331 | w1024xr1136 legal pair 2/2 queued @swarm-526-331[ab] 1d/s3000/dcp1500/xmx10G; work/AG-331 | 2/2 204
PATCH_SUMMARY | AG-331 | files=work,claims/AG-331 | idea=w1024xr1136 legal pair cap-trunc-vs-dgw fork | ev=2/2 204
PATCH_SUMMARY | AG-339 | files=work,claims/AG-339 | idea=stall-3 job-ценз: флап 10:47Z ETA 40-50h | ev=census_13z
