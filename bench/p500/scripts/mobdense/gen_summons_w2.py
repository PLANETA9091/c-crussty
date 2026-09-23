#!/usr/bin/env python3
"""TASK-81 mob-dense census — WINDOW 2 summon generator (stable/zoned profile).

W1 lesson (11:17Z window): unzoned mix = combat chaos — husks hunt villagers
(150->70), crowding triggers maxEntityCramming=24 damage deaths (husks
250->196). W2 isolates species into zones beyond targeting range:

  Zone A (villagers): 150, grid 15x10 sp3, z -364..-337 (north)
  Zone C (items):     100, grid 10x10 sp4, z -330..-321 (middle)
  Zone B (husks):     250, grid 25x10 sp1.5x2, z -240..-223 (south, ~100
                      blocks from villagers = beyond husk target range 35)

Harness controls (measurement infra, not product gameplay; same class as
TASK-78 Age:-32768):
  - PersistenceRequired:1b on all mobs (kills the W1/TASK-78 despawn confound)
  - /gamerule maxEntityCramming 0 — no cramming DAMAGE, pushing/collision
    resolution still fully active (issue killed ~134 entities in W1)
  - /gamerule doWeatherCycle false is NOT touched; no other rules changed
"""
X0 = -570
cmds = []

# Zone A: 150 villagers
for row in range(10):
    for col in range(15):
        cmds.append(f"summon minecraft:villager {X0 + col * 3} 75 {-364 + row * 3} {{PersistenceRequired:1b}}")

# Zone C: 100 items
for row in range(10):
    for col in range(10):
        cmds.append(
            f"summon minecraft:item {X0 + col * 4 + 6} 75 {-330 + row} "
            f"{{Item:{{id:\"minecraft:stone\",count:1}},Age:-32768s,PickupDelay:-1s}}"
        )

# Zone B: 250 husks far south
for row in range(10):
    for col in range(25):
        x = X0 + col * 1.5
        x = int(x) if x == int(x) else x
        cmds.append(f"summon minecraft:husk {x} 75 {-240 + row * 2} {{PersistenceRequired:1b}}")

assert len(cmds) == 500
for c in cmds:
    print(c)
