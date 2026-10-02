#!/usr/bin/env python3
"""TASK-81 mob-dense census — WINDOW 3 summon generator (DEFINITIVE profile).

W1 lesson: unzoned mix = combat chaos (husks hunt villagers, cramming dmg).
W2 lesson: fixed-y=75 summon = terrain mismatch — 7/12 probed villager-zone
grid points SOLID at y=75 (spawned inside terrain -> suffocation deaths);
survivors full HP -> spot damage, not AoE.

W3 fixes (harness controls, documented; AI paths untouched):
  - Invulnerable:1b — negates suffocation/fall/combat/cramming damage;
    tick/Brain/sensor/goal/collision machinery fully active
  - spawn at y=100 -> entities drop onto the REAL surface per column
    (no block-inside spawning anywhere in the zone)
  - PersistenceRequired:1b — no despawn (kills TASK-78 confound)
  - zoned layout kept: villagers north (z -364..-337), items middle
    (z -330..-321), husks south (z -240..-223) — beyond target range,
    so GoalSelectors run on idle/wander paths, villager Brain sensors run
    in stable-density conditions
  - /gamerule maxEntityCramming 0, doMobSpawning false (set out-of-band)
"""
X0 = -570
cmds = []

# Zone A: 150 villagers
for row in range(10):
    for col in range(15):
        cmds.append(
            f"summon minecraft:villager {X0 + col * 3} 100 {-364 + row * 3} "
            f"{{PersistenceRequired:1b,Invulnerable:1b}}"
        )

# Zone C: 100 items
for row in range(10):
    for col in range(10):
        cmds.append(
            f"summon minecraft:item {X0 + col * 4 + 6} 100 {-330 + row} "
            f"{{Item:{{id:\"minecraft:stone\",count:1}},Age:-32768s,PickupDelay:-1s}}"
        )

# Zone B: 250 husks far south
for row in range(10):
    for col in range(25):
        x = X0 + col * 1.5
        x = int(x) if x == int(x) else x
        cmds.append(
            f"summon minecraft:husk {x} 100 {-240 + row * 2} "
            f"{{PersistenceRequired:1b,Invulnerable:1b}}"
        )

assert len(cmds) == 500
for c in cmds:
    print(c)
