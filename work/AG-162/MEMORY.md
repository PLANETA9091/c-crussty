# AG-162 MEMORY (≤15 строк уроков)
1. Л141-вилки-(2+3) — мои; вилка-1 = AG-196. Читай clm соседа ПЕРЕД работой по чужому FAIL.
2. lineunion_harness крашился TypeError при javac/rustc=None; фикс = graceful-skip + PASS-PARTIAL-вердикт, exit 0.
3. canonline (C2b-гейт) чисто-python: работает на голой платформе — только краш харнесса его глушил.
4. javac живёт в /tmp/jdk-21.0.12.1+1/bin/javac (AG-153); харнессу его даёт env LINEUNION_JAVAC.
5. Вериф-паттерн харнесса: 3 мода (no-TC full / javac full / selftest) — no-TC: 35/51+16skip, javac: 45/53+8skip, 0 missed/0 FP.
6. sparse-worktree: --no-checkout + sparse-checkout set + reset --hard → index полный (ls-tree 3564), материал только /scripts /tests/lineunion /claims /clm /work.
7. CAS-аппенд доски: /tmp/board_append.py (GET sha → PUT, retry 409) — живой, переиспользуемый.
8. Локальный git commit своей ветки легален; push — только API POST /git/refs FULL 40-sha.
9. Ретро-ценз: сигнатуры swallowed-пайпов = 'command not found'/'No such file' в шаге с conclusion success.
10. Очередь w527: диск 4.4G/9.9G, очередь ~400q/38ip — диспатчи дольше 25-мин бюджета, код-фиксы быстрее.
