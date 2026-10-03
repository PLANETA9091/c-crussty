# AG-263 w527 PAYLOAD — merge-arb 526-180 execution log

T03:24Z: CLAIM CAS (2x409, landed 72353dbc). Board len 779-781k.
T03:2xZ: fetch origin/swarm-526-180 (2171d6da) + master. merge-base e2dd984d (w525).
Реальный патч = 2 файла, +16/-1 (diff vs master врал 394 файла/128k del — merge-base метод).
T03:2xZ: master moved 3a781abd->63d764bd (board CAS-коммиты) — re-verif на свежем:
merge-tree --write-tree CLEAN → 7996aab3133b; bash -n OK; YAML OK (wd.inputs 13);
mode run.sh 100644 — безвредно (обе ноги вызова: bench-v2.yml:133, press.yml:103 = bash-канон).
T03:3xZ: POST /merges → 201, commit 2a58e81e5c. Post-fetch verif:
tree 3676≥3200; blob-shas 1ab8f4a6/2f715bdc == предсказание; SIM_DISTANCE x3 live; diff vs предсказанное дерево = SHARED_BOARD.md only.
Вердикты:
- defaults byte-eq канон → 0 риска для queued-ног (bv2 102 + WBP 27 в очереди).
- unlocks: AG-224 sim53/sim64 re-fire, AG-238 simx4 — base=master легален.
- pending-стек w526 закрыт (162/178/196 lands AG-205; 182/194/198 дубли/superseded; 180 = этот мёрж).
Open: canary этого мёржа не требуется (additive plumbing, bench-поведение byte-eq);
но первый же sim-диспатч с master = де-факто canary (G4/G5/BAND гейты prereg AG-224/238).
