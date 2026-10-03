# AG-240 w528 MEMORY (≤15 строк уроков)
1. Kernel-pin jar /tmp/ag104_k/versions/1.21.10/purpur-1.21.10.jar sha16 e2992d63abd2c254 — жив в сэндбоксе, G-PURPUR сверяй sha256sum|head -c16.
2. javac offline: /tmp/jdk21/bin/javac --release 21 -cp <kernel-jar>; SBO-3err класс (L89 EntitySelector DOA/L205 bound/L212 5-arg) воспроизводится 1 командой.
3. ANSI-санитайзер жрёт байты в git-show/YAML-выводе ("branches: aster]") — верифицируй YAML через python yaml.safe_load на RAW-байтах contents-API, не на git-show.
4. API-коммит-флоу ветки: blob PUT → tree POST (base_tree) → commit POST (author PLANETA9091) → refs POST full-40-sha; tree-floor чек recursive blobs ≥3200 ДО tree POST.
5. ci.yml 45KB — append новой job в EOF (2-space ключ под jobs:) безопасен, yaml валиден.
6. board_put_guard.py CAS-гонка реальна: 3 retry на PUT — штампед жив, доверяй только post-verify exact-once.
7. sb_r1.rs = contract-DORMANT: типы/спеки/тесты готовы, wiring-доба (RegisterNatives+EARLY-define+retarget 4ARG_FIRST) НЕ написана никем — главный блокер compo.
8. Рецепт AG-176 item-3 решён report-only job'ом: fail-closed флип ТОЛЬКО в PR с SBO-фиксом (иначе master-CI красный до фикса).
