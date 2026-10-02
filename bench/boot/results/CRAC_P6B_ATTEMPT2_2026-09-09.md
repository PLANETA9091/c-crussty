# CRaC P6B ATTEMPT 2 (TASK-115 phase-6b, S7-61, 2026-09-09)

Rig: bench/boot/crac_p6b_realserver.sh (self-contained bundling INSIDE rig per LAW P6B-2 — 13 org.crac classes verified pre-boot).

## Run: boot PASS 17.3s — beforeCheckpoint STILL never fired
- Agent journal = PREMAIN-V2-REGISTERED ONLY. Inventory ~= attempt-1 baseline (jar + session.lock x3 + 25575 + latest.log + anon_inode/eventfd/timerfd class). Object-close UNTESTED (hooks never dispatched) — no classification value in this refusal.
- 1 boot spent (pre-registered cap) -> boots stopped, /tmp-only isolation probes follow.

## LAW P6B-3: -jar launch shape breaks org.crac -> jdk.crac dispatch (dummy-fallback, SILENT)
- Discriminator probe (/tmp/crac_bind, plain Zulu CRaC JVM, org.crac bundled in jar, registration in main, ServerSocket + beforeCheckpoint-close): -jar mini.jar -> REFUSED-SURVIVED, hook fired 0/1 (dummy context).
- Controls (historical + this session): -cp launches dispatch ALWAYS (S7-58 p5 compat probe -cp main-reg = PASS; S7-59 p6a -cp premain-reg = PASS incl. SURGERY journal writes).
- Mechanism note: org.crac silently falls back to dummy Context when reflective bind fails (README-documented fallback) — no error surfaces; only behavioral discriminator (hook-fired marker) reveals it. Attempt-1/2 journal-only-PREMAIN = same dummy path under real server (-jar purpur.jar).
- One -cp mini control run was harness-broken (jcmd never delivered; honestly excluded — historical -cp evidence stands).

## S7-62 fix (pre-registered, CLI-only 0 config)
- Paperclip manifest Main-Class = io.papermc.paperclip.Main (verified this session).
- Launch WITHOUT -jar: `java -Djava.library.path=$W -javaagent:hookv2.jar -XX:CRaCCheckpointTo=$IMG -cp hookv2.jar:purpur-1.21.10.jar io.papermc.paperclip.Main --nogui` (app-classpath + -cp shape = both proven dispatch paths; agent jar self-contained per P6B-2).
- Then: object-close (netty chain + log4j) FIRST REAL TEST; expected remaining ladder: session.lock x3 + jar (passes) + anon_inode class (native-held, classify); if img>0 -> restore x2 + prize metric vs 13.2s.
