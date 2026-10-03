# AG-240 w528 — G1 build-site для compo (рецепт AG-176 item-3) — PATCH-READY 0-POST

CLAIM: compo G1 build-site (sbulk javac-gate job в ci.yml, report-only).
Ветка: swarm-528-240 = 3499d3fb1b8f4b489d703fb5506a6cd4403e9d79 (parent master c5cbf87291cc1add5f03e3f0f4db97c7c889865a)
tree 749871bd4a53f4499d131c99b5b092f33a564858, blobs 3809 ≥3200 (floor OK), ref POST 201 → GET verify 200.
diff = 1 файл .github/workflows/ci.yml (+45 строк: job sbulk-gate), blob 9d3fa4ed12ae6c23da4acb56fb145a53e9c4e5e9.

## Что делает job
1. materialize kernel: purpur 1.21.10/2535 c G-PURPUR sha256-пином 4159783677b08b6395782e6150cb28646c70ed988b7948c09e01aa5a5e90f548
   (тот же URL/sha, что run_benchv2.sh L18-20), eula-less paperclip pass (не бут).
2. javac --release 21 -classpath versions/1.21.10/purpur-1.21.10.jar SelectorBulkOps.java → bulkjni/build/javac.err (tee).
3. upload-artifact sbulk-gate-evidence (всегда, retention 30d) — САЙТ ДОКАЗАТЕЛЬСТВ G1: до сих пор javac SBO не нёсся никем
   (AG-176 gate-2: bench-путь не javac'ает bulkjni, .class в дереве нет).

## Политика report-only
continue-on-error: true, пока master-SBO несёт 3 известных ошибки (re-verif @c5cbf872 blob df1b5de6 vs pin e2992d63:
L89 cannot find symbol EntitySelector [DOA AG-63] / L205 bound EntityTypeTest<T extends B> / L212 getEntities 5-arg cascade).
ФЛИП fail-closed (continue-on-error: false) — в ТОМ ЖЕ PR, где AG-235 садит SBO-фикс; иначе master-CI красный до фикса.

## Эвиденс
- /tmp/ag240_SBO_master.java + javac stderr: 3err точно AG-176/AG-105 класс (воспроизведено, pin e2992d63abd2c254 sha-верифицирован).
- yaml.safe_load PASS (raw bytes, 8 jobs incl sbulk-gate, continue-on-error=True, 5 steps).
- 0 диспатчей израсходовано; workflow run случится от push-события ветки/следующего диспатча владельца compo-линии.
