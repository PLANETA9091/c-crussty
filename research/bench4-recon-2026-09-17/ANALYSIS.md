# BENCH-4 STEP-0 CONTRACT VERIFICATION (task170 / TASK-236, S7-99)

Kernel: materialized `purpur-1.21.10.jar` (mojang-mapped, paperclip remap,
killed pre-main — NOT a boot), sha256-prefix e2992d63abd2c254, build 2025-12-11.
Method: javap -p -c (offline). Artifacts: signatures.txt, contract-bodies.txt,
/tmp/Connection.j, /tmp/SGPL.j, /tmp/Mob.j (bodies summarized below).

## Verified contracts

| # | Contract | Verdict |
|---|----------|---------|
| C1 | `PlayerList.placeNewPlayer(Connection, ServerPlayer, CommonListenerCookie)` public | OK — constructs its OWN `ServerGamePacketListenerImpl(server, conn, player, cookie)` internally (offset 120); sends ClientboundLoginPacket via SGPL.send; sets isRealPlayer=true; loads player data (none → fresh); adds to `players` + level |
| C2 | Connection stub safety | `sendPacket` derefs `channel.eventLoop()` UNCONDITIONALLY, but only AFTER `isConnected()`; `doSendPacket`: `!isConnected()` → packet.onPacketDispatchFinish + return (no-op). `isConnected()`: channel==null → false (null-safe). `getLoggableAddress`: address==null → "local" (null-safe). **Connection.tick() → handleDisconnection() when !isConnected() — but Connection.tick is ONLY called from ServerConnectionListener for netty-registered connections; a hand-made Connection is NOT in that list → no auto-disconnect** |
| C3 | `ServerPlayer(MinecraftServer, ServerLevel, GameProfile, ClientInformation)` 4-arg public ctor; `public ServerGamePacketListenerImpl connection` field | OK |
| C4 | SGPL ctor `(MinecraftServer, Connection, ServerPlayer, CommonListenerCookie)` | OK — created inside placeNewPlayer; ctor body has 0 channel derefs |
| C5 | `CommonListenerCookie.createInitial(GameProfile, boolean)` static | OK |
| C6 | max-players gate | `canPlayerLogin(SocketAddress, NameAndId)` NOT called in placeNewPlayer path (only getMaxPlayers() feeds login packet contents) — direct placeNewPlayer does not need max-players, bumped anyway (fixture honesty) |
| C7 | mobcaps output | `io.papermc.paper.command.subcommands.MobcapsCommand`: header "Mobcaps for world:" composed with `SpawnState.getSpawnableChunkCount()` int; per-category "cur/cap" lines follow |
| C8 | per-player spawnable-chunk source | Dec-2025 build: moonrise `PlayerMobDistanceMap` class ABSENT (renamed/replaced); equivalent = `LocalMobCapCalculator.playersNearChunk` (Long2ObjectMap<List<ServerPlayer>>) + spawnable-chunk set from player proximity — 0 players ⇒ 0 spawnable (empirically measured run#15) ⇒ same fixture requirement: real registered ServerPlayers |
| C9 | despawn side | `Mob.checkDespawn` → `Level.findNearbyPlayer(Entity, double, Predicate)` — player-list-driven; active with fake players |
| C10 | keepalive timeout trap | `ServerCommonPacketListenerImpl.keepConnectionAlive()` (protected): 15s cadence sends ClientboundKeepAlivePacket, sets awaitingKeepAlive; no response ⇒ disconnect("Timed out") ~30s after spawn → player removed → fixture dies. **MITIGATION: EmbeddedChannel discard-handler intercepts ClientboundKeepAlivePacket and calls public `handleKeepAlive(new ServerboundKeepAlivePacket(id))` (vanilla response path, internal state consistent)** |

## Stub design (chosen)

`Connection conn = new Connection(PacketFlow.SERVERBOUND)` + public field
injection `conn.channel = new EmbeddedChannel(discard-outbound-handler)`:
- `EmbeddedEventLoop.inEventLoop()` = true always → sends run inline →
  writeAndFlush hits FIRST the discard handler → `ReferenceCountUtil.release`
  + `promise.trySuccess()` → no queue buildup, no netty worker, no leak.
- Keepalive interceptor in the same handler (C10).
- Player flags after registration: teleportTo (9-arg ServerPlayer overload) to
  quadrant ground Y via `level.getHeight(MOTION_BLOCKING, x, z)+1`,
  `noPhysics=true`, `setNoGravity(true)`, `setInvulnerable(true)` (starvation/
  mob damage blocked; void path avoided by ground placement).
- Deterministic profiles: `UUID.nameUUIDFromBytes("BenchFake-N")` — same
  UUIDs/names across A/B legs (fixture parity law).

## Runtime class visibility

Paper/Purpur 1.20.5+ runtime is Mojang-mapped and PluginClassLoader delegates
NMS classes to the server classloader (paperweight-userdev contract) — plugin
compiled directly against the purpur kernel jar (javac, CI) loads at runtime.
