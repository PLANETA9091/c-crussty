# RESEARCH — BOAT STEP-0 kill-gate verdict (task164 / S7-95)

**Lever:** BOAT whole-body hot-patch (preregistered by TASK-230/S7-93 as the round winner, STEP-0 kill-gate: replaceable base >= 4% of TOTAL tick CPU, else REFUTED pre-implementation, fallback = BATCH-RNG same round).
**Method:** exact-frame anatomy of `AbstractBoat.tick` over the byte-same run#10 collapsed artifact (224,660 samples) — the STEP-0 protocol the lever itself preregistered. Script: `bench/world3/task164_boat_anatomy.py`.

## 1. Presence vs replaceable — the mirage quantified

- Boat lane stack presence: **21,703 samples = 9.66% of total CPU** (matches TASK-230's number exactly — the presence was real).
- But a whole-body patch only OWNS what the boat itself computes. Ownership model (leaf classification of every sample inside boat stacks):

| leaf class | samples | share of TOTAL CPU | disposition |
|---|---|---|---|
| MISC (long tail: Entity.push 0.45, getRootVehicle 0.21, Vec3, maps…) | 7,464 | 3.32% | mostly kernel-owned machinery |
| JVM-BOUND (SynchedEntityData, VarHandle, maps, String.equals, ArrayList…) | 5,715 | 2.54% | stays JVM even after patch |
| MOONRISE-COLLISION (CollisionUtil.*, collideX/Y, findFloor, getEntityHardCollisions) | 2,184 | 0.97% | **do-not-duplicate** (task161 law) |
| KERNEL-WORLD-READS (PalettedContainer.get, getBlockStateFinal, BitStorage.get, BlockBehaviour…) | 2,161 | 0.96% | move-not-save: reads still happen (snapshot moves them, does not remove them) |
| MIRROR-ENTBP (ChunkEntitySlices.getEntities + AABB.intersects) | 1,978 | 0.88% | already-owned by task161's entity_mirror — boats are its TOP consumer (61% of the broadphase lane) |
| **BOAT-OWNED** (leaf == AbstractBoat.tick 0.16% + vehicle/* leaves: setPaddleState 0.44, push 0.11…) | 1,707 | **0.76%** | the ONLY part a boat-native actually replaces |
| PREDICATE-PUSH-MEMO (Predicate lambdas, EntitySelector) | 494 | 0.22% | PUSH-MEMO lever (task161 follow-up) |

**VERDICT: REFUTED — 0.76% boat-owned vs 4% kill-gate.** Even the maximally generous reading (BOAT-OWNED + PREDICATE + half of MISC) stays under 2%. The preregistered ceiling "4.5–6.2%" was derived from PRESENCE, and STEP-0 did exactly its job: presence 9.66% ≠ replaceable 0.76%. One implementation round saved by the kill-gate — the discipline works.

## 2. What the anatomy GAVE us (positive yield)

1. **Boats are the #1 consumer of the broadphase lane** (2.31% of the 3.79% presence) — `entity_mirror.rs` (task161, built, 79/79) serves mostly VEHICLES. The ENT-BP production value is vehicle-concentrated; if wiring ever proceeds, A/B should run in a vehicle-dense world.
2. **F3 per-class tick split (new CI metric, live in report v2):** run#11 top entity tickers by presence — AbstractBoat 9.70%, **Brain.tick 6.85%** (mob AI: villagers/hostsiles), Villager 3.47%, Skeleton 2.80%, minecarts ~5.3% combined, ItemEntity 1.10%. The Brain lane (AI) is a new, larger-than-expected cluster for the follow-up queue (note: Brain.tick stacks are counted inside entity tick, distinct mechanism from movement).
3. **run#11 full v2 coverage first time:** wall-collapsed (85MB) + alloc-collapsed (109MB) + GC stats (216 pauses / 4.77s STW / 0 Full) + F1 module share 0.00% (§8 PASS). Alloc profile mirrors CPU ranking — no hidden allocation monster; GC lane stays diffuse "allocation-shape" (F2 wired, MB/s estimate next).
4. **Recon leg fixed (root-caused):** the standalone entity-recon job unzipped the PAPERCLIP jar — classes live inside the PATCHED kernel that paperclip materializes at boot (`cache/`/`versions/`). entity-recon is now a post-boot step inside world-bench (javap from the real booted jar + patched-kernel.jar uploaded as artifact). This unblocks the BATCH-RNG bit-exact contract: `SimpleThreadUnsafeRandom.advanceSeed` + `ServerLevel.optimiseRandomTick` bytecode comes from the artifact, not from memory.

## 3. Next lever (per preregistered fallback, same round)

**BATCH-RNG (random-tick lane)** — targets `ServerLevel.optimiseRandomTick` 2.2% self + `SimpleThreadUnsafeRandom.advanceSeed` 1.7% (alloc profile #2/#4 sites) ≈ 3.9% cluster. Mechanism: native owns the seed pipeline + position selection per chunk-tick (ONE JNI crossing per chunk), kernel dispatches randomTick as before — selection overhead moves to Rust, dispatch count unchanged. **Bit-exact contract requires the advanceSeed/optimiseRandomTick bytecode** → core lands NEXT tick from the entity-recon artifact (no guessing; "from the booted jar — not from memory" law).

## 4. Honesty ledger

- All numbers: run#10/#11 CI artifacts (diagnostic semantics — they rank, not prove parity).
- The BOAT refutation does NOT demote entity_mirror (its lane is measured separately: 3.79% presence, replaceable 2.27% — parked per TASK-230 as before).
- run#11 MSPT avg 84.47ms (run#10: 80.86) — world variance ~4% between runs; A/B gates must be same-session paired (as preregistered).
- INJECTS-ONLY: 0 sandbox boots; CI boots sanctioned.
