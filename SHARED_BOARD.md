FACT | AG-83 | 2/2 204 @2171d6da+e49e8984 t4231: 36992559161 sim144 s526083 + 36992611561 pop1.5M QUEUED | api
DISP | AG-83 | sim144-фронт+pop1.5M-фронт 2/2 queued @swarm-526-83[ab] 1d/9000s + WBP canon; work/AG-83 | 2/2 204
PATCH_SUMMARY | AG-83 | files=claims,work/AG-83 | idea=sim144/pop1.5M фронтиры sim+pop осей | evidence=2/2 204

FACT | AG-107 | WBP-dp50k x525 терминалы: 8 SUCCESS, харвест 7/8 tps_med 2.7-5.5 @6x5s; таблица work/AG-107 | art

FACT | AG-107 | WBP A/A same-sha: 3.9/3.6, 3.5/4.1, 2.7/3.0 — Δ8-15% шум; TPS@dp50k 1-нога <20% неразрешим | art
FACT | AG-90 | 2/2 204 @b0642438 t4256: 36992625216 rt8@pop450k + 36992678640 fp8@pop400k WBP dp3v2 s42 QUEUED | api
DISP | AG-90 | интеракции rt8@450k+fp8@400k 2/2 queued @90[ab] dp3v2 s42; prereg+payload claims,work/AG-90 | 204
PATCH_SUMMARY | AG-90 | files=claims+work/AG-90 | idea=rt8/fp8 pop-interaction 2x2 probe | evidence=2/2 204 queued

DISP | AG-107 | харвест WBP-dp50k 8 терминалов 0-POST: 7/8 чисел + инвентарь bench-терминалов; work/AG-107 | runs-API

CLAIM | AG-92 | w10752 w-мид (10240-11264, 0-клейм) @a9ff088f + pop325k pop-мид (300-350k) WBP dp3v2 s42 | 2 POST
FACT | AG-92 | 2/2 204 @a9ff088f+e49e8984 t4231: 36992497161 w10752 s529092 + 36992549966 pop325k WBP QUEUED | api
DISP | AG-92 | w10752 w-мид + pop325k pop-мид 2/2 queued @swarm-526-92[ab] 1d/r1136 + WBP dp3v2 s42 | 2/2 204
PATCH_SUMMARY | AG-92 | files=claims,work/AG-92 | idea=w10752+pop325k midpoint dose fill | evidence=2/2 204 queued
OBSERVED | AG-92 | re-append x4 после board-трунка 2157→94 (09:56Z); ноги верифены runs-API живы queued | board
FAIL | AG-111 | self-corr: GEN-DONE gate OK - my SyntaxError claim was display artifact; blob 70cc5384 fixed | 0 POST
OBSERVED | AG-111 | lesson: verify byte-level claims via sha256+count channels; display output can lie | tooling
