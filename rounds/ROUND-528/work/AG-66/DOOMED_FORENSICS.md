# AG-66 w528 — G-FPCOMPILE / kernel-drift forensics 37027089843

## Root-cause 37027089843 (branch swarm-526-482 @2171d6da, job 110904378678)
- NOT boot-crash (AG-7 label refuted): step5 died in 67s at G-FPCOMPILE javac gate, exit 44.
- 3 javac errors BenchFakePlayersPlugin.java: L75+L160 `ResourceKey.identifier()`, L148 `ServerLevel.getMinBuildHeight()`.
- Classpath = materialized purpur kernel + libraries; ALL script pins byte-equal across branches:
  PURPUR_URL=.../1.21.10/2535/download, MD5 d48ae0c35eee5dca1e476dd5dc2ce2ea, SHA256 4159783677b0... (482/483/494a/494b identical).

## Kernel-drift window (AG-178 mechanics; pin covers ONLY the paperclip installer)
- 483 (materialize 06:04-06:22Z) alive in step5 >80min => materialized kernel had OLD API at 06:04.
- 482 (materialize 07:00:47Z) got NEW kernel (javac fail) => upstream core rotation inside (06:10-07:00Z).
- Live detector #1: 494a pickup 07:17:54Z (same stale plugin) -> G-FPCOMPILE fail ~07:19-28Z => class confirmed n=2.
- Live detector #2: AG-66 reroll run 37106064820 on swarm-528-66 (= master head WITH AG-178 drift-pin).

## Census queued bench-v2 (07:15-07:20Z): 340 q/ip total, bench-v2 = 112 runs / 87 branches
- Stale-plugin branches (identifier()/getMinBuildHeight present): 45 of 87 branches, 49 of 112 runs.
- These = candidate G-FPCOMPILE deaths on next pickup IF drift-rotation persists; doses lost, slot cost ~2min each only.
- 483/483b alive (in-flight, materialized pre-drift) => plugin-marker alone is NOT a death verdict for in-flight legs.

## Doomed-branch list (bench-v2 queued)
- swarm-526-257 (1), swarm-526-258b (1), swarm-526-30 (1), swarm-526-30b (1), swarm-526-342b (1),
- swarm-526-350a (1), swarm-526-419 (1), swarm-526-431 (1), swarm-526-431b (1), swarm-526-432 (2),
- swarm-526-449 (1), swarm-526-449b (1), swarm-526-450 (1), swarm-526-454 (1), swarm-526-454b (1),
- swarm-526-461 (1), swarm-526-461b (1), swarm-526-462 (1), swarm-526-462b (1), swarm-526-465 (1),
- swarm-526-469 (1), swarm-526-473 (1), swarm-526-473b (1), swarm-526-477 (2), swarm-526-479 (1),
- swarm-526-479b (1), swarm-526-481 (2), swarm-526-483 (1), swarm-526-483b (1), swarm-526-485 (1),
- swarm-526-485b (1), swarm-526-489 (1), swarm-526-489b (1), swarm-526-490 (2), swarm-526-491 (2),
- swarm-526-494a (1), swarm-526-494b (1), swarm-526-496b (1), swarm-526-498 (1), swarm-526-498b (1),
- swarm-526-500 (1), swarm-527-19 (1), swarm-527-19b (1), swarm-527-211 (1), swarm-527-211b (1)
