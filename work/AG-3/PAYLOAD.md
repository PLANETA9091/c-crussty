# AG-3 w526 PAYLOAD — canary forensics
## canary-9 re-fire (REFUTED as gate-opener; FALSE-RED)
runs 36970681819/36970630254, job 110723813946/110723664444, @1f575d06 swarm-525-8
inputs: RADIUS_BLOCKS=1136 RUN_SECONDS=9000 BENCH_SEED=351515/351601 SERVER_XMX=10G
        BENCH_DIMS=minecraft:overworld DIM_GEN_WINDOW=256 DRAIN_CAP_POLLS=240
BENCHV2.md: marked=20449 (expect >=58279 = 0.95*3*20449; radius side=143) -> G4 FAIL; всё остальное PASS.
G-DIM PASS ov=21609 ne=0 en=0 (expect_pd=20449 n_dims=1 min_pd=19426) — dims-aware гейт здоров.
parser report_benchv2.py@1f575d06 md5=762ceee8: n_dims=3; re.match(r"dims=") на _envp не матчит.
## canary-10 (DISP queued)
branches swarm-526-3a/3b @a9ff088fd31f3f7d791bfdcab5760790c3fc463c (201 x2, tree 3296 blobs)
runs 36988366662 (3a s351515) + 36988461053 (3b s351601) QUEUED; inputs = canary-9 re-fire set.
carrier parser md5=2da1febc (re.search verbatim, 247-канон), union AG-395 walrus fix, yml timeout 330m.
Логи canary-9: work/AG-3/job_181819.log job_630254.log (70900/77635 B).
