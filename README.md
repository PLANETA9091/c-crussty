# c-crussty — Crussty CE native surface как плагин

Модуль платформы [CRUSSTY](https://github.com/PLANETA9091/CRUSSTY): инжектит
полный JNI-мост Crussty CE (98 bridge-классов, 283 натива) в ядро, плюс
hot-patch `ImprovedNoise` через ASM-вейвинг.

## Структура

- `src/` — Rust (jni_table, bridge_class, classfile, improved_noise, area_map)
- `area-map/` — `ca/` — Java-исходники bridge-классов Moonrise, `build/` — скомпилированные (вшиты через `include_bytes!`)
- `noise/` — `net/` — Java-исходники noise-бриджа, `build/` — скомпилированные
- `tests/fixtures/` — фикстуры для тестов

## Сборка и установка

Клонируй в `<crussty>/modules/crussty` и собери:

```bash
cargo build
cp target/debug/libcrussty.so libcrussty.so
```

Включение noise-патча: `CRUSSTY_NATIVE_IMPROVED_NOISE=1` в env при старте.

Примечание: runtime-зависимости `native/libpaper_native_jni.so` берутся из
оригинального Crussty CE (в репозиторий не входят).
