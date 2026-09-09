# RESEARCH: CRaC UNCLAIMED-FD & SOCKET LIFECYCLE LAWS (P6B-20..29 SYNTHESIS, S7-90 FRONT-C, 2026-09-09)

Companion to RESEARCH_CRAC_SERVING_2026-09-09.md (S7-79 web fragments). This doc synthesizes the
a22..a27 measured law chain (results CRAC_P6B_ATTEMPT22..26, ledger §79..§88) against the public
CRaC/CRIU ecosystem knowledge. Every law below is EVIDENCE-BANKED in this repo; ecosystem gaps are
stated where public docs are silent.

## The measured law chain (Minecraft/Purpur 1.21.10 + Zulu CRaC JDK, inject-only constraint)

1. **P6B-20 — per-tick cgroup fd re-open**: instrumentation agents (spark) re-open cgroup quota
   fds EVERY TICK -> bounded checkpoint retries are the only pre-a24b lever; unbounded retry =
   lottery (a23 att3-luck, a24 0/3).
2. **P6B-25/P6B-26 — static supplier fd discovery**: mojmap stores netty event groups in
   guava `Suppliers.memoize` (package-private runtime class); reflective `getMethod("get")`
   fails (IllegalAccessException) while a cast to the public `j.u.f.Supplier` interface works.
   Any post-restore group-walking code must cast, not reflect-method.
3. **P6B-28 — two-layer fd policy**: the CRaC policies file (`jdk.crac.resource-policies`) is
   consulted by JAVA-registered resources only (JDKFileResource.findPolicy; libjvm has NO
   OpenResourcePolicies strings). Unclaimed fds are decided natively by FdsInfo scan against
   `-XX:CRaCAllowedOpenFilePrefixes` (ccstrlist). Deterministic checkpoint needs BOTH layers
   tuned (v12.4 Java + v12.5 native => att1-clear 3 runs in a row).
4. **P6B-29 — unclaimed java.net sockets are closed JAVA-LEVEL at checkpoint**: jdk.crac core
   logs "Socket ... was not closed by the application" and closes the socket through Java
   (NioSocketImpl state -> CLOSED) — the closed-state travels INSIDE the image. Post-restore,
   `accept()` throws "Socket closed" from `ensureOpen` BEFORE any syscall => fd-number
   resurrection (dup2) is structurally inapplicable for java.net sockets; the SERVER-LEVEL
   isClosed() is blind to the impl-layer state (dual-state evidence banked a25).
5. **P6B-29 corollary — acceptor-thread storm**: vanilla acceptor loops with
   catch(IOException)->continue (RconThread.run bytecode: no exit path, field re-read per
   iteration) spin at exception rate forever post-restore => log/disk DoS (~2400 events per
   restore window pre-fix) unless the fd is healed or the loop exits. Bounded trimmer = hygiene,
   not remedy.
6. **P6B-24 remedy class — object field-swap**: for pure-Java listener objects (java.net
   ServerSocket), reflective field-swap of a FRESH bound socket heals storm + serving in one
   step (a25: RCON-SERVING type=2 rid=-1, soak 3/3+3/3). Bytecode guarantees pickup (field
   re-read per iteration). Works because rcon acceptor is a plain thread (not parked in native
   epoll_wait).
7. **P6B-23/P6B-30 — netty eventloop wakeup-gap**: eventloop fds (eventfd/timerfd/epollfd)
   closed by sweeps leave loops parked in-flight epoll_wait. dup2 branch repairs in-place;
   INT-SWAP branch (fd number recycled) MUST re-arm the fresh epoll with the wakeup eventfd
   (epoll_ctl ADD) — a26 measured: without re-arm the loop is dead-but-blocked (SLP timeout
   with TCP-accept-ok); with re-arm serving recovers same-boot (a26 soak 3/3+3/3).

## Ecosystem mapping / gaps (vs S7-79 fragments)

- Spring/CRaC close-restart pattern (Fragment 2) covers REGISTERED resources; it does NOT cover
  unregistered third-party sockets — P6B-29 shows the runtime's default is a SILENT java-level
  close + a warning line, and the app then storms. No public framework doc documents the storm
  class or the impl-vs-object dual-state. **Novel, banked.**
- CRIU TCP_REPAIR (Fragment 1) restores CONNECTION state kernel-side; warp/CRaC here has no
  TCP repair — but our results show the LISTENER layer is fully user-space reconstructible
  (25565 rebind + 25575 field-swap), and the wakeup-fd layer is user-space reconstructible with
  one epoll_ctl rule (P6B-30). Live player connections through checkpoint remain a non-goal
  (inject-only scope).
- The static-supplier discovery trap (P6B-25) is invisible in framework docs because frameworks
  own their registries; for reflective surgery on OBSFUCATED/mojmap game servers, the
  interface-cast rule is the difference between 0 loops and 10 loops (a23 vs a24b measured).

## Puzzle-assembly conclusions (what we can now build that no single fragment provided)

1. Deterministic checkpoint = policy file (Java layer) + prefix whitelist (native layer) + fd
   inventory evidence at BCP (rconPreCapture pattern). All three shipped in rig v12.6+.
2. Full serving restoration = async image-gated rebind (P6B-19) + per-loop fd repair with
   branch verdicts + ctlAdd re-arm in swap branch (P6B-30) + object field-swap for java.net
   listeners + bounded storm accounting. All shipped, both ports SERVING sustained (a26/a27).
3. Remaining open: N-minute durability (a27 LONG-SOAK, running), NioEventLoop native path
   (second group is Nio — repaired via object/int swap only), production-agent integration
   (phase-6d: move rig machinery into the c-crussty plugin lifecycle as CRaC Resource hooks).
