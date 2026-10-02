# AG-318 legs (wave 526)
| run | cell | seed | inputs | status |
|---|---|---|---|---|
| 37008833663 | xmx96G heap-front | 527318 | r1136/1d/w256/9000s/dcp900/xmx96G @a9ff088f | QUEUED 204 12:47:31Z |
| 37008881197 | s6000 sustain-mid | 528318 | r1136/1d/w256/s6000/dcp900/xmx10G @a9ff088f | QUEUED 204 12:47:59Z |
Cap-math: pregen 20449ch @w256 ~9.9-11 ch/s = 1860-2070s; +9000s bench = ~11.1ks < JOB cap 330m (19800s) OK;
s6000 leg: ~8ks OK. xmx96G: 80G прошёл queue (AG-272), 96G при нехватке RAM хоста = fast-fail известный класс.
Harvest w527: арты benchv2 -> ch/s, TPS-med, run-env, cpu_index из job-LOG (канон AG-271).
