#!/usr/bin/env python3
"""TASK-81 mob-dense census — summon command generator.

Emits RCON summon commands on stdout (one per line) for the dense
living-entity profile at world spawn (-544, 75, -336), forceload area
chunks [-36,-23]..[-34,-21] (x -576..-528, z -368..-320).

Load mix (~500 entities):
  - 150 villagers  15x10 grid, 3-block spacing, NW quadrant  (Brain paths:
    NearestLivingEntitySensor / VillagerHostilesSensor / forgetOutdatedMemories
    — the census domain TASK-78 had to leave ESTIMATE)
  - 250 husks with PersistenceRequired:1b  25x10 grid, 1.5x2 spacing, SE
    quadrant (goal-selector load; persistence FIXES the TASK-78 confound
    where 100 plain husks decayed to 38 mid-window via Paper no-player
    hostile-despawn)
  - 100 item entities (stone) 10x10 grid, 4-block spacing, center strip
    (Age:-32768s + PickupDelay:-1s; continuity with TASK-78 item profile)

No containment box (same as TASK-78): forceload keeps chunks loaded;
persistence keeps counts stable; wandering is realistic dense-farm behavior.
"""
import sys

X0, Z0 = -570, -364   # NW corner of the load area
cmds = []

# 150 villagers: 15 x 10, spacing 3
n = 0
for row in range(10):
    for col in range(15):
        x = X0 + col * 3
        z = Z0 + row * 3
        cmds.append(f"summon minecraft:villager {x} 75 {z} {{PersistenceRequired:1b}}")
        n += 1
assert n == 150, n

# 250 husks: 25 x 10, spacing 1.5 x 2 (crowded = constant push/collision load)
n = 0
for row in range(10):
    for col in range(25):
        x = X0 + col * 1.5
        z = Z0 + 34 + row * 2
        x = int(x) if x == int(x) else x
        cmds.append(f"summon minecraft:husk {x} 75 {z} {{PersistenceRequired:1b}}")
        n += 1
assert n == 250, n

# 100 items: 10 x 10, spacing 4, middle strip (between villagers and husks)
n = 0
for row in range(10):
    for col in range(10):
        x = X0 + col * 4 + 6
        z = Z0 + 26 + row
        cmds.append(
            f"summon minecraft:item {int(x)} 75 {int(z)} "
            f"{{Item:{{id:\"minecraft:stone\",count:1}},Age:-32768s,PickupDelay:-1s}}"
        )
        n += 1
assert n == 100, n

for c in cmds:
    print(c)
