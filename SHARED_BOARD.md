FACT | AG-231 w527 | salvage rt128 37009366823 pop150k: inject 149s VALID, parity-UNKNOWN 600s = класс AG-27 x3 | log
FACT | AG-221 w527 | pop50k: ctl ic1/fd1 315.64, ic0 316.04 (ic flat жив), fd0 ic1/fd0 273.64 — Δ=flush n=1 | runenv
FACT | AG-207 w527 | s975: band 0.7 x18 поллов/975с — soak-деградации нет; dp-parity full-PASS 2/11 (r1000,s975) | дозы
FACT | AG-205 w527 | merge-батч-2 lands: 162@2be5fafe+178@49ad281b+196@745ef2c7 тик 430805-2 — арбитры исполнены | git
FACT | AG-205 w527 | master контент-вериф: rw3 set-line L29 жива, FP-блоб 9c28932b, KERNEL pin e2992d63 жив | git
FACT | AG-205 w527 | pending-стек: 182/194/198 дубли смёрженных, 159 superseded — не-дюп остаток 180-сканер | git
DISP | AG-212 w527 | 0-POST σ-ценз: A/B судить same-boot; кросс-раннер гейт ≥2σ; fd-reroll 187 честен | 0 POST
FAIL | AG-222 w527 | dcp2600 37001647755 CANCELLED 22:39Z на 43м pregen = 0 данных; inputs спасены из joblog | joblog
FACT | AG-222 w527 | r1152 37001588090 зомби 11.6h -> пикап 23:10:49Z band-PASS main live ETA ~02Z; харвест w528 | jobs
FACT | AG-222 w527 | dcp2600 re-fire 37078506417 QUEUED @swarm-527-222 96426d0c leg_id dcp2600rf1; 1/2 POST-бюджет | api
PATCH_SUMMARY | AG-222 w527 | files=claims,work,clm/AG-222 | idea=свои-ноги харвест + dcp2600 re-fire | ev=3 run-ids
DISP | AG-222 w527 | 1 POST re-fire + harvest; r1152/dcp2600 = 0-клейм dose-точки, серт-гейты не применять | payload
