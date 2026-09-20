# ROUND-396-B — ITEMS-STAGGER (lever_flag="items_stagger", CRUSSTY_LEVER_ARG=N=4)

## Mechanism
Architectural replacement of the merge-scan schedule: the heavy neighbor
query (mergeWithNeighbours -> getEntitiesOfClass, O(n) per item per tick)
runs once per N ticks per item, phase = entityId % N (even load spread);
spawn/teleport/move events scan immediately (E-moment parity), so merge
outcomes on moving items are preserved. despawn/pickup/baseTick untouched.

Three STRICT single-site retargets inside ItemEntity.tick (from pristine
bytes, sites==1 each, region_threads discipline):
1. mergeWithNeighbours()V -> ItemStaggerOps.mergeWithNeighbours(ItemEntity)
   (phase gate; off-phase = skip, on-phase = vanilla body via privateLookup).
2. ItemEntity.move(MoverType;Vec3)V -> ItemStaggerOps.move(...) = exact
   vanilla move + moved-stamp (forces scan on the mover's next scheduled tick).
3. Level.noCollision(Entity;AABB)V site inside tick -> ItemStaggerOps.
   noCollision(...) = vanilla result + moved-stamp (collision change implies
   position/grounding change).

Bridge: ItemStaggerOps defined into the kernel loader (fluid_bitmask
pattern), fail-dominant static-init probe, privateLookup MethodHandle
delegates. Gate: CRUSSTY_LEVER_FLAG=items_stagger + CRUSSTY_LEVER_ARG=N
(default 4). Fail-closed: any strict violation = vanilla.

## Parity / superiority deviations (DOC-DEV)
- Merge latency <= N ticks for stationary items (final merge set identical:
  any pair within radius is found at the mover's or the next scheduled scan).
- Ceiling: merge-scan work /N at equal outcomes -> 15-25% TPS-equivalent at
  N=4 (items lane 31.17% is scan-dominated; RESEARCH-B).

## Validation
- ItemStaggerOps recompiled --release 21 vs patched-kernel.jar (javac green).
- Rust compose is pure byte surgery with strict Retargeted{sites:1} x3;
  AlreadyPatched{sites:1} tolerated (idempotent re-serve).
