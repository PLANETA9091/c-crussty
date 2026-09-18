# S7-155 RECON-3 часть 1 — структурный census kernel e2992d63 через javap (9809 классов, 7105 распознано)

## B. RNG-census: общий Level/ServerLevel.random

| поле | классов-референсеров (все jar) | entity-классов |
|---|---|---|
| `net.minecraft.world.level.Level.random` | 132 | 47 |

| поле | классов |
|---|---|
| `net.minecraft.world.entity.Entity.random` (локальный) | 109 |

### ОБЩИЙ random: entity-классы-пользователи (порядко-зависимый хазард планировщика): 47

- `net.minecraft.world.entity.Entity`
- `net.minecraft.world.entity.EntityType<T`
- `net.minecraft.world.entity.LightningBolt`
- `net.minecraft.world.entity.LivingEntity`
- `net.minecraft.world.entity.OminousItemSpawner`
- `net.minecraft.world.entity.ai.behavior.AcquirePoi`
- `net.minecraft.world.entity.ai.behavior.CopyMemoryWithExpiry`
- `net.minecraft.world.entity.ai.behavior.GiveGiftToHero`
- `net.minecraft.world.entity.ai.behavior.GoAndGiveItemsToTarget<E`
- `net.minecraft.world.entity.ai.behavior.GoToTargetLocation`
- `net.minecraft.world.entity.ai.behavior.JumpOnBed`
- `net.minecraft.world.entity.ai.behavior.LongJumpMidJump`
- `net.minecraft.world.entity.ai.behavior.LongJumpToRandomPos<E`
- `net.minecraft.world.entity.ai.behavior.RamTarget`
- `net.minecraft.world.entity.ai.behavior.ResetRaidStatus`
- `net.minecraft.world.entity.ai.behavior.RingBell`
- `net.minecraft.world.entity.ai.behavior.SetEntityLookTargetSometimes`
- `net.minecraft.world.entity.ai.behavior.SetRaidStatus`
- `net.minecraft.world.entity.ai.behavior.UseBonemeal`
- `net.minecraft.world.entity.ai.behavior.WorkAtPoi`
- `net.minecraft.world.entity.ai.goal.GolemRandomStrollInVillageGoal`
- `net.minecraft.world.entity.ai.village.VillageSiege`
- `net.minecraft.world.entity.animal.AbstractSchoolingFish`
- `net.minecraft.world.entity.animal.Bee`
- `net.minecraft.world.entity.animal.Dolphin`
- `net.minecraft.world.entity.animal.Fox`
- `net.minecraft.world.entity.animal.Fox$FoxEatBerriesGoal`
- `net.minecraft.world.entity.animal.Parrot`
- `net.minecraft.world.entity.animal.Turtle`
- `net.minecraft.world.entity.animal.axolotl.Axolotl`
- `net.minecraft.world.entity.animal.coppergolem.CopperGolem`
- `net.minecraft.world.entity.animal.goat.Goat`
- `net.minecraft.world.entity.animal.sniffer.Sniffer`
- `net.minecraft.world.entity.boss.wither.WitherBoss`
- `net.minecraft.world.entity.item.PrimedTnt`
- `net.minecraft.world.entity.monster.Shulker`
- `net.minecraft.world.entity.monster.Zombie`
- `net.minecraft.world.entity.monster.Zombie$ZombieGroupData`
- `net.minecraft.world.entity.monster.hoglin.HoglinAi`
- `net.minecraft.world.entity.monster.piglin.AbstractPiglin`
- `net.minecraft.world.entity.monster.piglin.PiglinAi`
- `net.minecraft.world.entity.monster.piglin.PiglinBruteAi`
- `net.minecraft.world.entity.monster.piglin.RememberIfHoglinWasKilled`
- `net.minecraft.world.entity.npc.CatSpawner`
- `net.minecraft.world.entity.player.Player`
- `net.minecraft.world.entity.projectile.ThrownExperienceBottle`
- `net.minecraft.world.entity.raid.Raid`

### ЛОКАЛЬНЫЙ Entity.random: 109 классов (полный список в json)

## C. Cross-entity invokes по entity-классам (топ-25)

| класс | сайтов | семейства |
|---|---|---|
| `net.minecraft.world.entity.boss.enderdragon.EnderDragon` | 6 | getEntities=4, getEntitiesOfClass=1, getNearestPlayer=1 |
| `net.minecraft.world.entity.animal.Panda` | 6 | getEntitiesOfClass=4, getNearestPlayer=2 |
| `net.minecraft.world.entity.ExperienceOrb` | 6 | noCollision=3, getEntities=2, getNearestPlayer=1 |
| `net.minecraft.world.entity.player.Player` | 4 | noCollision=2, getEntities=1, getEntitiesOfClass=1 |
| `net.minecraft.world.entity.monster.Shulker` | 4 | getEntities=2, noCollision=2 |
| `net.minecraft.world.entity.animal.Dolphin` | 4 | getEntitiesOfClass=3, getNearestPlayer=1 |
| `net.minecraft.world.entity.Entity` | 4 | collide=2, noCollision=1, getEntityCollisions=1 |
| `net.minecraft.world.entity.vehicle.NewMinecartBehavior` | 3 | getEntities=3 |
| `net.minecraft.world.entity.animal.Fox$Variant` | 3 | getEntitiesOfClass=3 |
| `net.minecraft.world.entity.LivingEntity` | 3 | getEntities=2, noCollision=1 |
| `net.minecraft.world.entity.LightningBolt` | 3 | getEntities=2, getEntitiesOfClass=1 |
| `net.minecraft.world.entity.vehicle.OldMinecartBehavior` | 2 | getEntities=2 |
| `net.minecraft.world.entity.vehicle.AbstractBoat` | 2 | getEntities=1, noCollision=1 |
| `net.minecraft.world.entity.projectile.ProjectileUtil` | 2 | getEntities=2 |
| `net.minecraft.world.entity.npc.Villager` | 2 | getEntitiesOfClass=2 |
| `net.minecraft.world.entity.npc.CatSpawner` | 2 | getEntitiesOfClass=2 |
| `net.minecraft.world.entity.monster.Zombie` | 2 | noCollision=1, getEntitiesOfClass=1 |
| `net.minecraft.world.entity.monster.Ravager` | 2 | getEntitiesOfClass=2 |
| `net.minecraft.world.entity.monster.EnderMan` | 2 | getNearestPlayer=1, getEntities=1 |
| `net.minecraft.world.entity.item.ItemEntity` | 2 | noCollision=1, getEntitiesOfClass=1 |
| `net.minecraft.world.entity.boss.enderdragon.phases.DragonSittingScanningPhase` | 2 | getNearestPlayer=2 |
| `net.minecraft.world.entity.boss.enderdragon.EndCrystal` | 2 | getEntitiesOfClass=2 |
| `net.minecraft.world.entity.animal.Pufferfish` | 2 | getEntitiesOfClass=2 |
| `net.minecraft.world.entity.animal.Cat` | 2 | getEntitiesOfClass=2 |
| `net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal<T` | 2 | getEntitiesOfClass=1, getNearestPlayer=1 |
