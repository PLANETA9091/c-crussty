# CRAC AFTERRESTORE RE-BIND DESIGN — netty eventloop resurrection path (FRONT-C, S7-77)

Target: upgrade post-restore state from liveness (measured: double-alive, 0.27s first_output, probes 4/4 DEAD) to SERVING, within measured constraints (P6B-17: warp refuses listening sockets at checkpoint; netty epoll/fd anon_inodes must be closed pre-checkpoint; layer-A ignore != native-exempt).

## Constraint ledger (measured, S7-68..77)

- C1: pre-checkpoint we MUST close netty channel object + its JNI fds (P6B-17 + layer-B sweep) — no state survives at kernel level for the listener.
- C2: restore RESUMES the process; netty objects exist but their channel/epoll are dead; MinecraftServer will never re-run bind (startup path skipped).
- C3: jdk.crac/org.crac afterRestore hooks FIRE in the restored process (AR-ORG/AR-RAW markers ×2 per boot, S7-77) — the sanctioned post-restore execution point.
- C4: kernel port 25565 is FREE post-restore (PORT-CLEAR listening=false, probes connect-refused) — a fresh bind is possible.
- C5: engine restores regular-file fds natively (P6B-16 fix) but not sockets (P6B-17).

## Design: R1 — in-process re-bind at afterRestore (chosen next step)

Execution point: our registered org-context Resource.afterRestore (AR-ORG site — it fires; instrument via listenState() added v9.1).

Steps (agent v10):
1. afterRestore: `listenState()` verdict first (AR-LISTEN) — ground truth of kernel state.
2. Reflect MinecraftServer `connection` (ServerConnectionListener) — same reflection chain as pre-checkpoint netty-close (P6B-10 proven: getAllLoadedClasses discovery).
3. Build a NEW EpollServerSocketChannel via the restored process's existing EventLoopGroup (ServerConnectionListener holds `pis`/channel futures list — locate `channels` list field, proven readable in surgery code).
4. `new ServerBootstrap().group(...).channel(EpollServerSocketChannel.class).childHandler(<copy pipeline factory from existing dead channel's config>)` — simplest honest path: copy the channel's `pipeline` factory by re-invoking the SAME lambda MinecraftServer used (chp init is in MinecraftServer#initServer; alternative: reflect ServerBootstrap from existing channel and re-`bind(port)`).
5. `bind(new InetSocketAddress(port)).sync()` — kernel listener returns (C4).
6. Re-register the future into `channels` list (reflection write) so Paper's connection acceptance path picks it up.
7. Markers: AR-REBIND rc, channel class, localSocketAddress; then external SLP probe must flip DEAD->SERVING (rig already probes).

Risks: netty allocator/native state post-restore (epoll eventloop group's JNI fds were swept — the loop must re-create its epoll fd lazily; netty EpollEventLoop re-creates epoll fd on next select? if not, force `newEpollEventLoopGroup`); pipeline handler instances carrying pre-checkpoint state (they are just objects — checkpoint-safe).

## Design: R2 — rcon parity (after R1 works)

Same pattern for rcon ServerSocket (new ServerSocket bind 25575 in afterRestore) — restores the accept-loop thread by restarting RconThread via server rcon registry reflection.

## Non-goals / honesty

- No server config touch, no gameplay values, pure agent-side runtime surgery.
- The checkpoint-time refusal for LISTENING sockets (P6B-17) means the re-bind must happen AFTER restore, not through checkpointed socket state — this design respects that.
- Serving claims only after SLP probe returns SERVING in the rig journal.

## Pre-registered for attempt-16 (S7-78)

1. Run v9.1 (AR-LISTEN at AR-ORG site) => measure kernel-listener verdicts (expect absent ×4 — confirms C1/C4).
2. If AR-LISTEN plumbing verified: implement R1 step 1-6 as agent v10 behind `crussty.rebind=1` flag, one boot, SLP verdict = the acceptance.

## a22 ADDENDUM (S7-83): EVENTLOOP FD RESURRECTION — javap-verified facts (netty 4.1.118.Final)
`javap -p io.netty.channel.epoll.EpollEventLoop` (from netty-transport-native-epoll + classes-epoll):
- `private unix.FileDescriptor epollFd;` `private unix.FileDescriptor eventFd;` `private unix.FileDescriptor timerFd;`
- `private final EpollEventArray events;` (native malloc'd buffer — fd-independent, survives)
Fds are WRAPPER OBJECTS (not raw ints) => resurrection = reflectively read each wrapper's
internal int (unix.FileDescriptor holds `int fd` + native methods), create fresh fds via
Rust JNI lib (epoll_create1 / eventfd / timerfd_create — add externs to fd_surgery.rs),
then SET the wrapper's int field to the new fd (setAccessible; cleaner than dup2-to-number,
no fd-table lottery). Repair order: epollFd -> eventFd -> timerFd per loop, loops discovered
via `Netty Epoll Server IO #N` thread -> or register a probe channel to force group.next().
After repair => rebind (startTcpServerListener + acceptConnections) — the PendingRegistrationPromise
from LAW P6B-23 will then be serviced. Risk register: EpollEventLoop may cache native state
beyond the three fds (IovArray/datagram arrays are buffers, fd-free — verified above); wakeup()
writes eventFd directly (works after swap); timer for scheduled tasks uses timerFd (same).
