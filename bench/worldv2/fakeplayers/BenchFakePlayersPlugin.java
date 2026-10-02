package bench.fakeplayers;

// ============================================================================
// BenchFakePlayersPlugin (v2, AG-342 wave-515) — BENCH-V2 spawn-lane fixture.
// Adapted from verified BENCH-4 fixture (bench/world3/fakeplayers, task170,
// S7-99, run 35156292165): v2 additions are (a) BENCH_FAKE_DISTRIBUTE=1 ->
// round-robin placement across overworld/nether/end so ALL THREE dims get
// real spawn/despawn lanes (owner directive: heavy stand in ALL dimensions),
// (b) periodic per-dim entity census log lines [BenchV2Census] for the
// spawn-rate metric, (c) nether/end y-anchoring (roof/void safe).
// Everything else (EmbeddedChannel stub, keepalive auto-answer, placeNewPlayer
// full registration, deterministic BenchFake-N UUIDs) is byte-identical
// to the verified v1 contract. BENCH-ONLY fixture, not for production.
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
    private int radiusBlocks = 1136;
    private boolean distribute = false;
    private final List<ServerPlayer> injected = new ArrayList<>();

    @Override
    public void onEnable() {
        try { fakeCount = Integer.parseInt(reqEnv("BENCH_FAKE_PLAYERS", "0").trim()); }
        catch (NumberFormatException e) { fakeCount = 0; }
        try { radiusBlocks = Integer.parseInt(reqEnv("BENCH_FORCELOAD_RADIUS", "1136").trim()); }
        catch (NumberFormatException e) { radiusBlocks = 1136; }
        distribute = "1".equals(reqEnv("BENCH_FAKE_DISTRIBUTE", "0").trim());
        if (fakeCount <= 0) {
            getLogger().info("BENCH_FAKE_PLAYERS=0 -> bench-v2 canon mode (no injection)");
            return;
        }
        Bukkit.getScheduler().runTask(this, this::injectPlayers);
        // 100 ticks = 5s@20TPS: alive-check + per-dim entity census (cheap logs).
        Bukkit.getScheduler().runTaskTimer(this, this::censusTick, 20L * 5, 20L * 5);
    }

    private static String reqEnv(String k, String d) {
        String v = System.getenv(k);
        return v == null ? d : v;
    }

    private void censusTick() {
        for (ServerLevel lvl : levels()) {
            int n = 0;
            for (net.minecraft.world.entity.Entity e : lvl.getAllEntities()) n++;
            getLogger().info("[BenchV2Census] dim=" + lvl.dimension().location() + " entities=" + n
                    + " players=" + lvl.players().size()); // benchv2-census
        }
        getLogger().info("[BenchFakePlayers] alive-check: injected=" + injected.size());
    }

    private List<ServerLevel> levels() {
        MinecraftServer server = ((CraftServer) Bukkit.getServer()).getServer();
        List<ServerLevel> out = new ArrayList<>();
        for (ServerLevel l : new ServerLevel[]{server.getLevel(net.minecraft.world.level.Level.OVERWORLD),
                server.getLevel(net.minecraft.world.level.Level.NETHER),
                server.getLevel(net.minecraft.world.level.Level.END)}) {
            if (l != null) out.add(l);
        }
        return out;
    }

    private void injectPlayers() {
        CraftServer craftServer = (CraftServer) Bukkit.getServer();
        MinecraftServer server = craftServer.getServer();
        ServerLevel overworld = server.getLevel(net.minecraft.world.level.Level.OVERWORLD);
        if (overworld == null) {
            getLogger().severe("overworld not found — injection aborted, fixture INVALID");
            return;
        }
        double ringR = radiusBlocks / 2.0;
        List<ServerLevel> dims = levels();
        for (int i = 0; i < fakeCount; i++) {
            String name = "BenchFake-" + i;
            UUID uuid = UUID.nameUUIDFromBytes(name.getBytes(java.nio.charset.StandardCharsets.UTF_8));
            GameProfile profile = new GameProfile(uuid, name);

            Connection connection = new Connection(PacketFlow.SERVERBOUND);
            connection.address = new InetSocketAddress(0);

            ServerLevel level = distribute ? dims.get(i % dims.size()) : overworld;
            ServerPlayer player = new ServerPlayer(server, level, profile, ClientInformation.createDefault());

            final ServerPlayer playerRef = player;
            EmbeddedChannel channel = new EmbeddedChannel(new ChannelOutboundHandlerAdapter() {
                @Override
                public void write(ChannelHandlerContext ctx, Object msg, ChannelPromise promise) throws Exception {
                    if (msg instanceof ClientboundKeepAlivePacket keepAlive) {
                        if (playerRef.connection instanceof ServerCommonPacketListenerImpl common) {
                            common.handleKeepAlive(new ServerboundKeepAlivePacket(keepAlive.getId()));
                        }
                    }
                    ReferenceCountUtil.release(msg);
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

            // Per-dim placement: ring inside the forceload zone. Nether anchors
            // at fixed y=64 (MOTION_BLOCKING hits the bedrock roof y=127 —
            // roof placement kills nearby-terrain spawn lanes). End anchors on
            // the island (ring capped at 96 blocks: island is at 0,0, ring
            // beyond it is void). Overworld = ground-snapped heightmap (v1).
            double ring = (level.dimension() == net.minecraft.world.level.Level.END) ? Math.min(ringR, 96) : ringR;
            double angle = 2.0 * Math.PI * i / fakeCount;
            int x = (int) Math.round(ring * Math.cos(angle));
            int z = (int) Math.round(ring * Math.sin(angle));
            int y;
            if (level.dimension() == net.minecraft.world.level.Level.NETHER) {
                y = 64;
            } else {
                y = Math.max(level.getHeight(Heightmap.Types.MOTION_BLOCKING, x, z) + 1, level.getMinY() + 8);
            }
            player.teleportTo(level, x + 0.5, y, z + 0.5,
                    java.util.Set.of(), 0.0F, 0.0F,
                    false, org.bukkit.event.player.PlayerTeleportEvent.TeleportCause.PLUGIN);
            player.noPhysics = true;
            player.setNoGravity(true);
            player.setInvulnerable(true);
            player.setOnGround(true);

            injected.add(player);
            getLogger().info("[BenchFakePlayers] registered " + name
                    + " uuid=" + uuid + " dim=" + level.dimension().location() + " at (" + x + "," + y + "," + z + ")");
        }
        getLogger().info("[BenchFakePlayers] DONE: injected=" + injected.size() + " distribute=" + distribute);
    }
}
