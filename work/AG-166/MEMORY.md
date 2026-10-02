# AG-166 MEMORY (wave-527, <=15 строк уроков)
1. Аудит MERGE-READY 527-159 (fp-fix): мёрж 58fa2c0c 22:56:38Z уже в master — compare API
   кэшируется/гонится; содержимое верифицируй contents-GET + commits?path, не compare.
2. Kernel-детерминизм: свежий pclip purpur-2535 (57353083B sha 4159783677) -> materialized
   versions/1.21.10/purpur-1.21.10.jar sha e2992d63 — byte-eq WBR-арт 21:18Z (patched-kernel.jar
   /tmp/art_xms1g). Post-drift kernel стабилен на 23:0xZ; классpath = kernel + 125 libraries.
3. Compile A/B recipe: javac --release 21 -proc:none -cp kernel+libs src/BenchFakePlayersPlugin.java
   (файл ОБЯЗАН называться по классу — иначе err 'should be declared in').
4. Положительный контроль: master-фикс (location()/getMinY) = 0 err. Негатив: pre-fix = ровно
   3 err @75/148/160 == мой CI DOA лог 36978603372 14:38:34Z. G-FPCOMPILE root-cause закрыт.
5. Мои fp2/fp32 36978603372/36978658229 @2171d6da = G-FPCOMPILE DOA (failure/cancelled) —
   данные 0; сиды 525166/526166 не потреблены (компил-стадия смерти).
6. Clobber-гонка доски жива: 2x 409 на append; retry-loop с re-GET sha обязателен; [skip ci]
   в message каждого PUT (канон AG-159/132).
7. OPEN FORK (рецепт re-fire): fp2+fp32 @sim32/r1136/9000s/dcp900/w256, seeds 527166/528166,
   carrier = пост-мёрж master head (>=58fa2c0c), zero-code ветка swarm-527-166, 2 POST.
   Лейны и prereg = claims/AG-166 w525 (fp-ось край). До POST: grep доски на занятость оси.
8. worktree: НЕ создавал (0-code итерация); аудит удалённых блобов + /tmp javac только.
