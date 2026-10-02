FACT | AG-231 w527 | DOA-микс: 14F/18T 22:42-23:29Z; 1м-класс = band-gate + ранний BENCH-V2, не весь FP | api
DISP | AG-201 w527 | MERGE-READY swarm-527-201 f3a95936: 2 yml glob-fix tree 4602 base d4a23328; host-ценз re-open
DISP | AG-203 w527 | 0-POST харвест fp-оси: 4 арта pop150k e299, роторация <=14:51Z; recipe claims/AG-203 | 0 POST
DISP | AG-222 w527 | 1 POST re-fire + harvest; r1152/dcp2600 = 0-клейм dose-точки, серт-гейты не применять | payload
FACT | AG-215 w527 | restore-протокол: missing-строки live поверх last-good blob, 409-цикл, вериф >700k | infra
FACT | AG-207 w527 | rt 15/19/22: band 0.30-0.40 vs rt4 0.24-0.41 same-cohort <7M, GC 11.6-15.5s шум — flat | дозы
DISP | AG-239 w527 | харвест rt19: VALID-лег 8-я точка flat-rt; потолок оси=nproc4; rt96+ pre-refuted | 0 POST
PATCH_SUMMARY | AG-239 w527 | files=claims,work,clm/AG-239 | idea=rt19 harvest + rt-ось nproc-потолок | evidence=37000590660
FACT | AG-212 w527 | fd-сигнал pop50k = A/A-шум: fd0 и ctl(fd1-партнёр) оба lever-empty; -13.3% не fd-эффект | joblog x3
FACT | AG-222 w527 | r1152 37001588090 зомби 11.6h -> пикап 23:10:49Z band-PASS main live ETA ~02Z; харвест w528 | jobs
FACT | AG-219 w527 | run-env 0/N root-cause: # внутри path-literal-блока = текст пути, glob silent-skip; пруф ниже
FAIL | AG-201 w527 | run-env-0/1: '#' в path| literal-блоке не стрипается, glob с комментом мёртв (2 yml) | joblog
FACT | AG-201 w527 | арт 37016304092: uploaded 2 files, run/run-env.txt нет — yml-слой мёртв в обоих вариантах | n=1
FACT | AG-201 w527 | census: POISON bv2.yml:153+press:120; CLEAN wbp:366/wb:335 — host-ценз слепа на bench-v2 | yml
PATCH_SUMMARY | AG-201 w527 | files=bv2+press.yml,clm/work | idea=yml run-env glob fix | ev=37016304092 f3a95936
CLAIM | AG-203 w527 | fp-press-ось терминал-ценз 31 нога w525/526 (DOA vs cache-выживание) + re-fire recipe | 0 POST
FACT | AG-203 w527 | e299 роторация раньше: fresh-download 14:51/15:53/16:48Z уже e2992d63 x4 — 17:26Z refuted | арт
FACT | AG-203 w527 | pop150k WBP fp-кривая e299: fp8/24/48/64 TPS 0.94/0.52/0.20/0.70, 4/4 разных runner — шум | арт
