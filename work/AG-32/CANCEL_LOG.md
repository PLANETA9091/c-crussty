# AG-32 w528 — cancel-wave log 06:57Z
HEAD@check = 21273e86 (06:52:33Z board-PUT); все 24 queued push-ci sha=master 03:27-06:40Z = stale.
KEEP gate-of-record: 37103832347 (689d03bb, 06:40:32Z — ближайший к HEAD код: incl. rb2-370 union).
CANCELLED 23/23 HTTP202 (POST /actions/runs/{id}/cancel):
37102344372(ca2c5d1e) 37102015768(1cb9e753) 37101950062(a2993994) 37101358938(8df369fd)
37101357695(53695a6e) 37101356846(f1a78b5e) 37101208394(82dc4165) 37101206615(2cbb0c34)
37101169249(138314ce) 37101148694(7dd4d0d5) 37101080283(d1fdb448) 37101069469(55759547)
37101061916(25a25825) 37100762125(fe194d7b) 37100734908(da2910c4) 37100703405(c541b6e9)
37100675094(4e53c43c) 37100643098(723a42e2) 37100614398(6fedfd97) 37100583524(36f3f5d4)
37100551605(99bfcf2a) 37099237555(aa5d4e38) 37093269060(2a58e81e)
Атрибуция источников (файлы коммитов): SHARED_BOARD-448probe.txt (root, 3 PUT), SHARED_BOARD_ARCHIVE_W527.md
(root, 2 PUT: AG-459 prep + AG-491 rotate 689d03bb), .github/workflows/*.yml + bench/worldv2/run_benchv2.sh (merges).
Slot-math: 23 x 129m (push-ci median, AG-492 n=4) = 2967m ~= 49 slot-h freed для S-очереди.
Precedent: AG-81 cancel push-ci@master 195/202 202-OK; AG-487 FAIL-урок соблюдён: ip40 bench-ноги НЕ тронуты (0 cancel bench).
Нетронуты: 7 ci workflow_run (wr-echo, by-design skip-fast AG-402), 1 world-bench-ab q (flow dead, last push 04:41Z).
