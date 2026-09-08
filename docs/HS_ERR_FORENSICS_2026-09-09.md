# hs_err shutdown-crash family forensics (TASK-60)

Date: 2026-09-09 (cron tick 10:40+08) · Agent: agent-7625532f
Trigger: neighbor S7-14 NEXT-1 — recurring shutdown-phase JVM fatal errors on
/home/z/server. Census at analysis time: **11 hs_err_pid*.log** (Sep 7 14:53 →
Sep 8 00:58). Deliverable: characterization + root-cause chain + severity
verdict + proposals. **No engine change made** (rights: critical-bugs only —
verdict below says this family is not one). No module code change (honest
reasons in §6).

## 1. Census and classification

| pid | date | signal | thread | fault pc | module ELF vaddr* | cluster |
|---|---|---|---|---|---|---|
| 3968 | Sep 7 14:53 | SIGSEGV | pool-9-thread-1 | 0x0 | — | fast-cycles pc=0 |
| 27194 | Sep 7 15:02 | SIGSEGV | pool-9-thread-1 | 0x0 | — | fast-cycles pc=0 |
| 29706 | Sep 7 15:11 | SIGSEGV | pool-9-thread-1 | 0x0 | — | fast-cycles pc=0 |
| 16638 | Sep 7 15:16 | SIGSEGV | Server thread | in libcrussty.so | 0x8f762 | in-module |
| 19822 | Sep 7 15:20 | SIGSEGV | ServerMain | in libcrussty.so | 0x48c6b | in-module |
| 20701 | Sep 7 15:31 | SIGSEGV | (native thread) | 0x0 | — | pc=0 |
| 13224 | Sep 7 15:35 | SIGILL | pool-9-thread-1 | in libcrussty.so | 0x4a0a5 | in-module |
| 26901 | Sep 8 00:43 | SIGSEGV | **Signal Dispatcher** | in libcrussty.so | 0x64c50 | signal-path |
| 27203 | Sep 8 00:47 | SIGSEGV | **Signal Dispatcher** | in libcrussty.so | 0x61da5 | signal-path |
| 27457 | Sep 8 00:54 | SIGSEGV | **Signal Dispatcher** | in libcrussty.so | 0x64ae0 | signal-path |
| 28484 | Sep 8 00:58 | SIGSEGV | **Signal Dispatcher** | in libcrussty.so | 0x64b32 | signal-path |

\* vaddr = pc − r-x-map-start + r-x-fileoff + (p_vaddr−p_offset); per-file bias
read from each hs_err's own Dynamic-libraries section. Every non-null fault pc
falls **inside `libcrussty.so` = `/home/z/server/modules/crussty/libcrussty.so`
— the c-crussty module** (NOT the engine runtime; the agent
`libcrussty_runtime.so` appears in stacks as return addresses, e.g. 26901's
R15 and stack slots annotated `in /home/z/server/libcrussty_runtime.so`).

Build drift: r-x segment fingerprints (size/fileoff) split the census into ≥5
builds (0x61000/0x32000, 0x9f000–0xa4000/0x4c000–0x4d000, 0x72000/0x3e000,
0x73000/0x3d000 + current 0x71000-family). Only `libcrussty.so.bak_task54`
(= pre-00:35 deploy) survives on disk; the 00:35 and ~00:50 builds (4 signal-path
crashes) are overwritten. Symbol resolution is therefore **approximate** for
most files (nearest-symbol in a neighbor build), exact for none. Documented as
limit, not speculated around.

## 2. What each crash looks like

- **pc=0x0 family** (4×): `_thread_in_native` SIGSEGV at address 0 — a call
  through a NULL function pointer (JNI vtable slot / hook target zeroed or
  torn). Threads: Paper async pool workers + one native thread.
- **in-module family** (7×): fault inside module .so code; si_addr wild
  (e.g. 26901: `SEGV_MAPERR si_addr=0x0000fecff02d7a40`, RDI=0, R10 =
  0xff79747373757263 = bytes `63 72 75 73 73 74 79 ff` = ASCII **"crussty"+0xff**
  loaded for a SIMD string op; XMM lanes hold 0x16/0x22 compare patterns).
  pid13224 SIGILL = illegal instruction read/exec in torn code region.
- **Error-reporter self-crash (all 11)**: HotSpot's own stack printer dies
  while unwinding ("error occurred during error reporting", twice per file),
  reporter pc consistently `libc.so.6+0x136d5e` (one 0x136c67) — the unwind
  walks frames whose unwind data leads into torn/dead native state. That is
  why no hs_err in the family has a usable native stack — and why the family
  was previously only countable, not readable.

## 3. Resolved symbols (approximate, nearest-build)

- 26901/27203-era vaddrs resolved against `bak_task54`:
  `0x64c50 → <jvmti_bindings::jni_wrapper::JniEnv>::new_string` (JNI string
  creation) and `0x61da5 → hashbrown RawTable<(&str, String)>::reserve_rehash`
  (map rehash — the SIMD "crussty" bytes in R10 = hashing a "crussty…"-keyed
  entry; the module's class-name maps are exactly this shape,
  `cplug-sdk/src/asm.rs:79 rename_class(&HashMap<&str, String>)` and the CP
  builders in `bridge_class.rs`).
- The Rust borrow-checker rules out dangling `&str` keys in these maps
  (`rename_helper_classes` builds the map from static literals immediately
  before use) → the rehash crash is **downstream heap corruption**, not a
  lifetime bug in module-owned data. The corruption source is the earlier
  JNI-during-teardown UB (below).

## 4. Proven mechanism chain (the smoking gun)

hs_err 26901's Java-frames section survived the broken unwind far enough to
show the faulting thread's Java context:

```
j  jdk.internal.misc.Signal.dispatch(I)V+25  java.base@21.0.12.1
v  ~StubRoutines::call_stub
```

**The crash thread is executing the JVM's SIGTERM dispatch** — i.e. the
`Signal Dispatcher` thread is running the shutdown chain (Signal.dispatch →
Shutdown.exit → application shutdown hooks → DestroyJavaVM) **on itself**.
During that window classes still load (lazy loading inside shutdown hooks and
teardown paths), the engine's `ClassFileLoadHook` keeps firing, and the module
still serves its class-interest pipeline: kernel-load/arming via
`Class.forName`-equivalents (`new_string(&dot)` + find_class —
`src/area_map.rs:361`, `src/improved_noise.rs:776`), CP building
(`bridge_class.rs` push_utf8/push_class), name maps.

Live corroboration from OUR OWN TASK-59 shutdown (fifo `stop`, 02:31): the
console tail showed `area_map: forcing kernel load of SingleUserAreaMap
(attempt 2/3/4)` **during the shutdown sequence** — the arming pipeline
re-entering while the server was dying, by design retrying exactly the way the
crashing files died.

Chain: SIGTERM → Signal.dispatch on Signal-Dispatcher thread → shutdown hooks
load classes → engine CFLH → module transform/arming → JNI calls
(`NewStringUTF`, vtable calls) against a JNI environment already being torn
down → UB → wild deref (SIGSEGV) / NULL vtable slot (pc=0 family on async
workers racing the same teardown) / SIGILL. The pure-Rust rehash crash =
collateral of the earlier UB corrupting allocator state. Every observed
property of the family matches: shutdown-only, thread mix (signal dispatch /
pool workers / server thread), in-module fault sites, broken unwinds.

## 5. Severity verdict

**Not critical.** All 11 events are shutdown-phase; in every case the session's
real work (boots, benches, world saves, JFR dumps) had already completed; the
world/regional files are intact in all corresponding sessions; the crash only
changes the exit code (0 → 134) of an already-stopping JVM. No data-loss path,
no steady-state risk (TASK-57/S7-14 profiles show zero steady-state module
native work on these threads). Per the engine-rights rule ("critical bugs
only"), **no CRUSSTY engine change is made or justified**.

## 6. Proposals (none implemented here, with reasons)

1. **Engine-side (proper fix, for the engine owner)**: in the runtime's JVMTI
   callbacks: handle `VMDeath` (or check `GetPhase()` at CFLH entry) → set an
   atomic `vm_dying` flag → all downstream module callbacks short-circuit to
   "serve original bytes / skip arming". One flag kills the whole family.
   Not implemented: engine changes are gated on critical bugs; this family is
   cosmetic (§5). Proposal documented for the owner.
2. **Module-side**: no clean, verifiable mitigation exists without the engine
   event: during shutdown hooks the JVMTI phase is still LIVE (VMDeath fires
   only after hooks), so a phase check cannot distinguish "legitimate late
   class load" from "teardown race"; refusing to arm on late loads would trade
   a cosmetic crash for a functional regression (missed hooks). Any deeper
   "fix" (guarding JNI vtables, allocator poisoning checks) would be
   speculative and unverifiable with the measurement-first discipline of this
   project. Deliberately NOT done.
3. **Operational (already the norm post-TASK-59)**: prefer fifo `stop` over
   SIGTERM (Paper's console stop runs a coordinated shutdown on the Server
   thread; SIGTERM fallback is what parks work on the Signal-Dispatcher
   thread). The SIGTERM fallback remains needed for hangs — its exit-134 tail
   is acceptable and now explained.
4. **Watch policy**: count the family per session (S7-14 baseline: 2/fast-cycle
   + 1/single + 1 armed-boot; this census adds the Sep 7 fast-cycle 7). A
   change in thread mix or a NON-shutdown occurrence re-opens the task.

## 7. Reusable tooling

`/home/z/my-project/scripts/hs_err_extract.py` — per-file extraction: signal,
thread, fault pc, reporter-crash pc, and pc→library resolution from the
Dynamic-libraries maps section (handles HotSpot's maps-style lines + ELF
p_vaddr bias). Rerunnable on any future hs_err family.

## 8. Honest limits

- Per-instruction certainty limited by overwritten builds (≥5 module builds
  across the census; only `bak_task54` survives). Symbol attributions are
  nearest-build approximations, cross-checked by crash-shape (R10 SIMD string
  bytes vs map types).
- The reporter self-crash prevented full stack capture on every file; the
  mechanism chain (§4) rests on the surviving Java frame + live corroboration
  + resolved signatures, not on a complete native stack.
- No live reproduction attempt was made (would require deliberately SIGTERMing
  a live server under load — ops surface; the family's 11 occurrences already
  provide the statistics).
