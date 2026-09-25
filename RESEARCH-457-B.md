# RESEARCH-457-B — монстр-охота носителя cmp456_poi (POI-подсистема целиком в Rust, закон 6)

Агент: TASK-457-B · worktree `/home/z/rounds/ROUND-457/agent-b` · ветка `round-457b-poi` (от origin/master 1838ae1d).
Мандат: ≥+20% pair-stable (мандат владельца 2026-09-23); цикл закона 3 до ≥+20% или честного исчерпания бюджета тика.

## 0. Носитель и пины (не меняются)
- Фикс-коммит POI-носителя = **5ecd841a128aa97c633e8b005479c5dcf09781aa** (NCDFE-фикс: cmp456_poi игла в EntityGoalQueryOps DEFINE-гейт, канон fa9054d9 arm-AFTER-define). Валидирован ×2 ре-роллами NCDFE=0.
- Ветка `round-456b-poi` head = 0fa13d72 (содержит фикс + пи-эфай-скрипт); **ноги диспатчатся на ветках, созданных РОВНО на 5ecd841a**.
- Код носителя НЕ трогаю (sha-pin ЖЁСТКО), новых леверов нет, пустой lever не диспатчу.

## 1. Банк носителя (наследство ×456)
| leg | norm | idx | вердикт |
|---|---|---|---|
| poi456-1 | +16.9 | 6765332 | чист (до рейса); max pair +17.6 (↔a8 −0.7@6731200 Δ34k) — суб-бар |
| poi456-2 | — | — | NCDFE DELIVERY-FAIL (не вердикт) |
| poi456-3 | −0.5 | 7181470 | банк |
| poi456-4 | +15.4 | 8957260 | 3.10 TPS, unpaired — зона 8.9-9.0M редкая |

Лейн-гейт сертный: items 0.00, nav ~3.2, broad ~9.8-10.9, selfTest==true, AIOOBE=0.

## 2. Якорный пул
×456 банк (14 перечислено в брифе): a1 +4.4@8935474, a7 +7.4@7264766, a8 −0.7@6731200, a9 +5.2@6590823, a10 −2.0@6865200, a12 +8.1@6966170, a15 +8.0@6756575, a19 +7.5@6803823, a22 +4.4@6899724, a23 +6.0@6958213, a27 +10.1@6125089, a29 +2.9@7051107, a31 +3.7@6740240, a34 +5.9@6520174.
×457 новые: main диспатчит round-457-anchor-N — список в /home/z/rounds/ROUND-457/ANCHORS.md (появится у main; проверять каждый цикл).

## 3. Цикл закона 3 (до 6 ног за тик)
1. ПОЛЛ раз в ~10 мин: runs?head_branch=round-456b-poi-N.
2. АБСОРБ СРАЗУ: `absorb_round.py <run_id> poi457-<N>`; маркеры (ARM cmp456_poi, ЭФФЕКТ, selfTest, NCDFE, AIOOBE, лейны) ИЗ stdout ДО пурджа; пурдж zips/collapsed/flamegraph/spark сразу (диск 83%).
3. Вердикт-гейты: NCDFE=0 (T1), ARM+ЭФФЕКТ, AIOOBE=0, selfTest==true (FAIL ×1-2 = fixture-шум), лейны, band 6.0-9.5M (вне = BAND-DISCARD ре-ролл ≤2), threw=1 = REFUTED, block-entity ×10 = шум, dep-гейт norm ≥ −2.
4. Пары: leg_norm − anchor_norm, Δ≤50k, pair-fresh, min-of-3. Особое внимание зоне 8.9-9.0M.
5. Следующая нога: ветка round-456b-poi-<N+1> РОВНО на 5ecd841a, диспатч dispatch_457b.py (argv-guard --dry-run/--no-batch/--leg, канон-инпуты ×456/×457).
6. КАЖДЫЙ ШАГ commit+push + RESULT.json write-through.

## 4. Логи тика ×457 (append-only)
- PHASE 0: worktree создан на 1838ae1d, dispatch_457b.py написан (пин exact), RESULT.json write-through.
- **poi457-5 (run 36131787478, ветка round-456b-poi-5): completed failure на шаге «Runner calibration band gate (pair-hunter fast-fail, S7-96d pairing law)» = BAND fast-fail инфра → НЕ ВЕРДИКТ, ре-ролл ≤2 (канон).**
- poi457-6 (36131797497) / poi457-7 (36131807435) в полёте.

## 5. Вердикт (финал тика)
- ≥3 ноги ≥+18 norm с парами ≥+20 (min-of-3) → RESULT.json verdict=CERT_READY (мержит main, сам НЕ мержу).
- Иначе после 6 ног — честный банк verdict=BANK.
