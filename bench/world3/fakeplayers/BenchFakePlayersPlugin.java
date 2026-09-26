package bench.fakeplayers;

// ============================================================================
// BenchFakePlayersPlugin — BENCH-ONLY fixture for BENCH-4 (task170, S7-99).
//
// Owner condition (2026-09-16): mobs must spawn AND despawn "as if players
// are present". With zero players natural spawning is structurally off
// (0 spawnable chunks). This plugin injects N REAL ServerPlayers with a
// netty-free EmbeddedChannel stub so:
//   - level.players() contains them (spawn + despawn lanes activate)
//   - PlayerList.placeNewPlayer full registration path runs (chunk tracking,
//     per-player spawn proximity, join event)
//   - keepalive timeouts are answered inside the channel stub (players stay
//     registered for the whole soak window)
//
// This is a FIXTURE (scenario), not a gameplay change and not a config win:
// the server runs its own 100% unmodified spawn/despawn/AI code paths.
//
// Contract verified against materialized purpur-1.21.10.jar (mojang-mapped)
// by research/bench4-recon-2026-09-17/ANALYSIS.md (S7-99 STEP-0):
//   placeNewPlayer(Connection, ServerPlayer, CommonListenerCookie) public,
//   creates its own ServerGamePacketListenerImpl internally.
//   Connection.doSendPacket: !isConnected() -> no-op; with EmbeddedChannel
//   isConnected()==true -> writes hit our discard handler (no leak).
//   keepConnectionAlive(): protected; answered via public handleKeepAlive.
//
// Determinism / parity law: profile names + UUIDs are derived from
// "BenchFake-N" (UUID.nameUUIDFromBytes) so A and B legs register byte-identical
// players (same N, same grid, same UUIDs).
//
// NOT FOR PRODUCTION. Bench harness only (world-bench-3 CI, sanctioned boots).
// ============================================================================

import com.mojang.authlib.GameProfile;
import io.netty.channel.ChannelHandlerContext;
import io.netty.channel.ChannelOutboundHandlerAdapter;
import io.netty.channel.ChannelPromise;
import io.netty.channel.embedded.EmbeddedChannel;
import io.netty.util.ReferenceCountUtil;

import net.minecraft.network.Connection;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.network.protocol.common.ClientboundKeepAlivePacket;
import net.minecraft.network.protocol.common.ServerboundKeepAlivePacket;
import net.minecraft.server.MinecraftServer;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerCommonPacketListenerImpl;
import net.minecraft.world.level.levelgen.Heightmap;

import org.bukkit.Bukkit;
import org.bukkit.craftbukkit.CraftServer;
import org.bukkit.plugin.java.JavaPlugin;

import java.net.InetSocketAddress;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

public final class BenchFakePlayersPlugin extends JavaPlugin {

    private int fakeCount = 0;
    private int radiusBlocks = 640;
    private final List<ServerPlayer> injected = new ArrayList<>();

    @Override
    public void onEnable() {
        String nEnv = System.getenv("BENCH_FAKE_PLAYERS");
        String rEnv = System.getenv("BENCH_FORCELOAD_RADIUS");
        try {
            fakeCount = nEnv == null ? 0 : Integer.parseInt(nEnv.trim());
        } catch (NumberFormatException e) {
            fakeCount = 0;
        }
        try {
            radiusBlocks = rEnv == null ? 640 : Integer.parseInt(rEnv.trim());
        } catch (NumberFormatException e) {
            radiusBlocks = 640;
        }
        if (fakeCount <= 0) {
            getLogger().info("BENCH_FAKE_PLAYERS=0 -> bench-3 mode (no injection)");
            return;
        }
        // Defer to the first server tick: worlds fully initialized, boot
        // sequence finished, tick loop about to start.
        Bukkit.getScheduler().runTask(this, this::injectPlayers);
        // Alive-check heartbeat: lets the report prove players stayed
        // registered the whole window (keepalive/disconnect failures show up
        // as a drop in this series).
        // S7-130b: period 1200 ticks (= 60s at 20 TPS) missed the report window
        // entirely on prime-scale scenes (150k live entities => TPS 0.7-1.0 =>
        // heartbeat due at 20+ min wall time, run 35242595837 gate-1c FAIL with
        // an otherwise-healthy fixture). 100 ticks = 5s at 20 TPS, ~2 hits even
        // at TPS 0.7; the log line is trivially cheap.
        Bukkit.getScheduler().runTaskTimer(this, () -> getLogger().info(
                "[BenchFakePlayers] alive-check: level.players()=" + playersInOverworld()
                        + " injected=" + injected.size()), 20L * 5, 20L * 5);
    }

    private int playersInOverworld() {
        org.bukkit.World w = Bukkit.getWorld("world");
        return w == null ? -1 : w.getPlayers().size();
    }

    private void injectPlayers() {
        CraftServer craftServer = (CraftServer) Bukkit.getServer();
        MinecraftServer server = craftServer.getServer();
        ServerLevel level = server.getLevel(net.minecraft.world.level.Level.OVERWORLD);
        if (level == null) {
            getLogger().severe("overworld not found — injection aborted, fixture INVALID");
            return;
        }

        // N players on a ring around world origin inside the force-load zone.
        // N=4 with R=640 -> (±320, 0)/(0, ±320): one per axis quadrant, covers the zone.
        //
        // S94 WORLD-BORDER stress leg (round-468-s94-wborder): the ring radius is
        // radiusBlocks * RING_SCALE. THIS branch defaults 1.20 => ringR = 640*1.2 =
        // 768 = the forceload-square EDGE (run_world3.sh §5 sweeps 6x6 tiles of
        // 256-block forceload commands => square [-768, 767] blocks = 96x96 chunks
        // = 9216 chunks; verified "forceload 36/9216" in run 36229065940 logs,
        // Л-466-C90.1). Players stand AT the loaded/unloaded chunk boundary:
        // every 128-block natural-spawn disc and the 160-block view ring
        // (server.properties view-distance=10) hang ~half BEYOND the forced
        // square => ~920 extra loaded/ticking chunks outside the 9216-canon set
        // (+10.0% loaded set), one-time chunk emission (packet encode) for the
        // fresh ring, and natural spawn surface over fresh terrain. Canon
        // anchors (0-delta branches) keep the 0.5 center-ring fixture; env
        // BENCH_FAKE_RING_SCALE overrides (ladder 0.5 center / 1.2 edge / 1.45 void).
        double ringScale = 1.20;
        try {
            String scaleEnv = System.getenv("BENCH_FAKE_RING_SCALE");
            if (scaleEnv != null && !scaleEnv.trim().isEmpty()) {
                ringScale = Double.parseDouble(scaleEnv.trim());
            }
        } catch (NumberFormatException e) {
            getLogger().warning("BENCH_FAKE_RING_SCALE unparsable — keeping branch default " + ringScale);
        }
        double ringR = radiusBlocks * ringScale;

        for (int i = 0; i < fakeCount; i++) {
            String name = "BenchFake-" + i;
            UUID uuid = UUID.nameUUIDFromBytes(name.getBytes(java.nio.charset.StandardCharsets.UTF_8));
            GameProfile profile = new GameProfile(uuid, name);

            Connection connection = new Connection(PacketFlow.SERVERBOUND);
            connection.address = new InetSocketAddress(0); // getLoggableAddress -> safe path

            ServerPlayer player = new ServerPlayer(server, level, profile, ClientInformation.createDefault());

            final ServerPlayer playerRef = player;
            EmbeddedChannel channel = new EmbeddedChannel(new ChannelOutboundHandlerAdapter() {
                @Override
                public void write(ChannelHandlerContext ctx, Object msg, ChannelPromise promise) throws Exception {
                    // Keepalive auto-response: the canonical vanilla path —
                    // feed the client-bound keepalive back through the public
                    // handleKeepAlive so awaitingKeepAlive stays consistent
                    // and the 15s timeout can never disconnect the fixture.
                    // (playerRef.connection is the public ServerPlayer field
                    // assigned by placeNewPlayer's internal SGPL ctor.)
                    if (msg instanceof ClientboundKeepAlivePacket keepAlive) {
                        if (playerRef.connection instanceof ServerCommonPacketListenerImpl common) {
                            common.handleKeepAlive(new ServerboundKeepAlivePacket(keepAlive.getId()));
                        }
                    }
                    ReferenceCountUtil.release(msg); // packets are not refcounted -> no-op; ByteBufs released
                    promise.trySuccess();
                }
            });
            connection.channel = channel;

            try {
                server.getPlayerList().placeNewPlayer(connection, player,
                        CommonListenerCookie.createInitial(profile, false));
            } catch (Throwable t) {
                throw new IllegalStateException("fixture injection failed for " + name + " (no fallbacks)", t);
            }

            // Position on a ring point, ground-snapped via heightmap (sync
            // heightmap load of that single chunk if needed).
            double angle = 2.0 * Math.PI * i / fakeCount;
            int x = (int) Math.round(ringR * Math.cos(angle));
            int z = (int) Math.round(ringR * Math.sin(angle));
            int y = level.getHeight(Heightmap.Types.MOTION_BLOCKING, x, z) + 1;
            player.teleportTo(level, x + 0.5, y, z + 0.5,
                    java.util.Set.of(), 0.0F, 0.0F,
                    false, org.bukkit.event.player.PlayerTeleportEvent.TeleportCause.PLUGIN);
            // Fixture stability: no gravity/physics drift, immune to mob
            // damage + starvation (canonical bench compromise — the players
            // are positional anchors for spawn/despawn lanes, not combatants).
            player.noPhysics = true;
            player.setNoGravity(true);
            player.setInvulnerable(true);
            player.setOnGround(true);

            injected.add(player);
            getLogger().info("[BenchFakePlayers] registered " + name
                    + " uuid=" + uuid + " at (" + x + "," + y + "," + z + ")");
        }

        getLogger().info("[BenchFakePlayers] DONE: injected=" + injected.size()
                + " level.players()=" + level.players().size());
    }
}
