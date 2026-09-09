# CRAC P6C REGISTRY ANALYSIS — S7-69 (2026-09-09) — jdk.internal.crac static decompile

Source: `/home/z/crac-jdk/lib/modules` (Zulu CRaC JRE 21), extracted via
`/home/z/jdk21/bin/jimage extract --include 'regex:/java.base/jdk/internal/crac.*' --dir /tmp/jimage_icr`
(NOTE: jdk.internal.crac lives INSIDE java.base, not a own module — 39 classes incl. mirror/ + mirror/impl/).

## 1. Two-layer architecture (explains LAW P6B-11 fully)

LAYER A — JAVA REGISTRY (claimed fds):
- `jdk.internal.crac.ClaimedFDs` = `WeakHashMap<FileDescriptor, Descriptor>`; `claimFd(fd, owner, exceptionSupplier, args)` called by JDK resource classes when java.io/net objects are created.
- Claimed resources: `JDKFileResource` (RandomAccessFile/FileInputStream/...), `JDKSocketResource`/`JDKSocketResourceBase` (java.net sockets).
- Each resource's `beforeCheckpoint` (mirror Context traversal) resolves an OpenResourcePolicies action → outcome encoded in `boolean closed` / `boolean error` + `Supplier<Exception>` holder.

LAYER B — NATIVE SCAN (unclaimed fds):
- `mirror.Core.checkpointRestore0(int[], Object[], boolean, long)` (native) scans /proc/self/fd; fds NOT claimed by Layer A and not recognized → reported back via `translateJVMExceptions` which maps native fail-codes (JVM_CR_FAIL_FILE/SOCK/PIPE/generic) DIRECTLY to `CheckpointOpenFileException`/`CheckpointOpenSocketException`/`CheckpointOpenResourceException` — **no policy consultation on this path**.

=> LAW P6B-11 mechanism confirmed at source level: claimed fds live in the java registry (JNI close invisible to it); unclaimed fds judged by native scan (JNI close IS visible). Two different remedies per layer.

## 2. The sanctioned lever: OpenResourcePolicies (policy file)

- Activation: `-Djdk.crac.resource-policies=<file>` (PROPERTY constant verified in constant pool).
- Line format (addPolicy): comma-separated k=v; REQUIRED `type` (file|pipe|socket) + `action`; extra k=v go to params map.
- File matching: param `path` = **GLOB** via `FileSystem.getPathMatcher("glob:" + v)` (lambda$findPolicy$0 verified).
- Socket params: `localPort`, `localAddress`, `remotePort`, `remoteAddress` (JDKSocketResource constant pool).

Action semantics (JDKFileResource.beforeCheckpoint bytecode, verified):
| action | behavior |
|--------|----------|
| (none, not classpath) | default `error` → `error=true` + exception supplier → **CheckpointOpenFileException** (what we saw) |
| (none, classpath match) | `ignore` (warn only, benign) — explains why some jar entries never suppressed |
| `error` | explicit suppress (fatal) |
| `close`  | `closeBeforeCheckpoint(policy)` then `closed=true` → NO_EXCEPTION → **benign** |
| `reopen` | close + `reopenAfterRestore(policy)` at restore → benign both sides |
| `ignore` | warn only, no close → benign |

If close itself throws IOException → `CheckpointOpenResourceException("Cannot close" + path)` — so policies must NOT target fds our agent already closed independently.

## 3. Mapping to our persist-set (v7 attempt-8 evidence)

| suppressed entry | layer | remedy |
|---|---|---|
| logs/latest.log | A (claimed, log4j RAF) | policy `action=close`; REMOVE our log4j-stop/JNI-close interference (policy owns the close; closing underneath → "Cannot close") |
| world*/session.lock ×3 | A (claimed RAF locks) | policy `action=close` (lock NOT reacquired on restore — restore is ports-dead probe, documented) |
| purpur jar | A (claimed classpath) | policy `action=reopen` (must be usable after restore) |
| 25575 rcon socket | A (claimed java.net) | policy `type=socket,action=close,localPort=25575` |
| anon_inode:[eventpoll|eventfd|timerfd] ×6 | B (native, unclaimed — netty JNI) | **JNI close in our beforeCheckpoint** (runs before native scan) — visible to native scan, not blocked by java registry |

Note 25565 (netty epoll listener) never suppressed: unclaimed native-accepted class OR classpath-ignore class — empirically benign across all attempts.

## 4. Attempt-9 pre-design (agent v8 = phase-6c)

Rig changes:
1. Policies file `$W/policies.txt`:
   ```
   type=file,action=close,path=/home/z/server/logs/latest.log
   type=file,action=close,path=/home/z/server/world/session.lock
   type=file,action=close,path=/home/z/server/world_nether/session.lock
   type=file,action=close,path=/home/z/server/world_the_end/session.lock
   type=file,action=reopen,path=/home/z/server/versions/**
   type=socket,action=close,localPort=25575
   ```
2. Launch adds `-Djdk.crac.resource-policies=$W/policies.txt` (rig-local flag, zero server-config touch).
3. Hook v8 = v7 MINUS stopFileAppenders() call MINUS latest.log fdSweep, PLUS anon_inode sweep: for each /proc/self/fd link starting `anon_inode:` → closeFd() (native scan layer; ~9 fds expected incl. dups).
4. Keep netty channel close (v7-proven), keep 25565/25575 sweep (25575 sweep now redundant with policy — keep for belt-and-suspenders? NO: single-variable discipline — remove 25575 sweep, policy owns it).

Pre-registered acceptance (attempt 9):
- (a) ZERO CheckpointOpen*Exception suppressions in jcmd.out (or strictly-anon_inode-free set if close-action throws on some entry — each remaining entry named in results)
- (b) img>0 (FIRST checkpoint image on real server ever) → restore ×2 + prize metric vs 13.2s floor
- (c) if restore-alive: HOOK-AFTER-RESTORE + AR-ORG/AR-RAW markers (continuity)
- Kills: >1 boot, config-file touch (policies are rig-flag only), hs_err increment.

## 5. Laws
- LAW P6B-12 (NEW): CRaC fd policy two-layer law — claimed fds (java registry) are governed ONLY by OpenResourcePolicies (`-Djdk.crac.resource-policies`, glob path, actions close/reopen/ignore/error); unclaimed fds are governed ONLY by the native /proc scan (JNI close before checkpoint = remedy; policies do not apply). Registry-prune reflection is UNNECESSARY for phase-6c — the sanctioned lever supersedes SESSION-068 NEXT(2) candidate (i).
- LAW P6B-13: close-action interference rule — a policy-managed fd must not be closed by any other path (agent object-close, JNI sweep) or closeBeforeCheckpoint throws "Cannot close" → new suppression; policy ownership is exclusive.
