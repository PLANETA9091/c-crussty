#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
parity_validator.py — ваниль-паритет fixture-валидатор (ROUND-466 / C85, Swarm v19.0).

КАНОН:
  • закон 4  (ВАНИЛЬНОСТЬ): поведение мира после оптимизации неотличимо от ванили —
    мерджи/деспавн/пикап/ИИ/коллизии/поведение мобов/сети не ломаем.
  • закон 20d (ПАРИТЕТ-ГЕЙТ): поведение мира до/после оптимизации бит-в-байт
    (или seed-идентичные чек-суммы чанков) — ванильность проверяется на ТЯЖЁЛОМ стенде.
  • парная дисциплина S7-96d: парить по (world_sha256, runner_cpu_index); разные
    world_sha256 = разные стенды = паритет-вердикт невозможен (канон T1).

ВХОД: два бенч-лога (vanilla vs leg). Сторона = один или несколько файлов:
  - server-stdout.log : сырой лог (POPULATION INJECT DONE / TOPUP / mobcaps per-type /
                        Marked…force loaded / "Can't keep up")
  - BOTTLENECKS_3.md  : срез-отчёт (FIXTURE-VALIDITY, forceload N (M chunks), entity
                        totals, top entity types, tick-behind warnings)
  - run-env.txt       : конфиг (world_sha256, runner_cpu_index, population_seed/target,
                        fake_players)
Формат автодетектится по содержимому; приоритет полей raw-лог > md-отчёт > run-env.

ГЕЙТЫ (статус OK / FAIL / SKIP, SKIP = поле есть только на одной стороне):
  FIXTURE-VALIDITY : VALID==VALID (+ гейты 1a/1b/1c из отчёта PASS==PASS)
  WORLD            : world_sha256 равен (S7-96d; --ignore-world чтобы отключить)
  FAKE-PLAYERS     : N BenchFake игроков равен
  POP-TOTALS       : target/injected/items/hostiles/passives — точное равенство
  SEED             : population_seed (INJECT START seed= / run-env) — точное равенство
  FORCELOAD        : chunks (и cmds) — точное равенство
  ENTITY-TYPES     : per-type (mobcaps / TOPUP aliveReal / md top-types) в банде
                     |a-b| <= max(floor, tol*max); --strict-entities = бит-в-байт
  TICK-BEHIND      : канон tick-behind=0 на обеих сторонах
ВЫХОД: PARITY-OK / PARITY-FAIL + дифф; exit 0/1 (2 = ошибка входа).
  Односторонние per-type из md top-N — артефакт усечения списка (не FAIL при
  не-strict; полный перечь даёт сырой лог).

САМОТЕСТ (--self-test [DIR], default /home/z/rounds/ROUND-466): реальная пара логов
  vanilla = c58-artifacts/run-36228910649 (raw-лог + md + env),
  leg     = A2_BOTTLENECKS_3.md + A2_run-env.txt (C42-якорь, run 36230386556);
  + негатив-контроль: мутации seed/forceload/fixture/entities/tick-behind/pop-totals
  обязаны давать PARITY-FAIL в своём гейте; + кросс-чек «сырой лог == md-срез».

Пример:
  python3 scripts/parity_validator.py vanilla.log md_slice.md run-env.txt -- leg.log leg-env.txt
  python3 scripts/parity_validator.py --self-test
"""

from __future__ import annotations

import json
import os
import re
import sys
import tempfile
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple

# ----------------------------------------------------------------------------
# регексы сырого лога (Purpur 1.21.10 + BenchPopulation / BenchFakePlayers)
# ----------------------------------------------------------------------------

RE_LOG_TS = re.compile(r"^\[\d{2}:\d{2}:\d{2}\s+(INFO|WARN|ERROR)\]", re.M)
RE_INJECT_START = re.compile(
    r"POPULATION INJECT START target=(?P<target>\d+) seed=(?P<seed>\d+)"
    r"(?: loadedChunks=(?P<loaded>\d+))?"
    r"(?: plan\(items=(?P<items>\d+), hostiles=(?P<hostiles>\d+), passives=(?P<passives>\d+)\))?"
)
RE_INJECT_DONE = re.compile(
    r"POPULATION INJECT DONE target=(?P<target>\d+) injected=(?P<injected>\d+)"
    r" items=(?P<items>\d+) hostiles=(?P<hostiles>\d+) passives=(?P<passives>\d+)"
)
RE_FIXTURE = re.compile(r"FIXTURE-VALIDITY\s*[:=]\s*\**\s*(?P<v>VALID|INVALID)")
RE_TOPUP = re.compile(
    r"POPULATION TOPUP-SCAN tick=(?P<tick>\d+)"
    r" aliveReal\(items=(?P<ai>\d+),hostiles=(?P<ah>\d+),passives=(?P<ap>\d+)\)"
    r" deficit\(items=(?P<di>\d+),hostiles=(?P<dh>\d+),passives=(?P<dp>\d+)\)"
    r" aliveEst\(items-model\)=(?P<est>\d+) topupSpawnedTotal=(?P<topup>\d+)"
)
RE_ALIVE = re.compile(r"alive-check:\s*level\.players\(\)=(?P<real>\d+)\s+injected=(?P<inj>\d+)")
RE_MOBCAPS_HDR = re.compile(r"Mobcaps for world:.*?\((?P<chunks>\d+) spawnable chunks\)")
RE_TYPE_LINE = re.compile(
    r"^(?:\[[\d:]+\s+\w+\]:\s*)?\s*(?P<cnt>\d+) \(\d+\) : (?P<type>[\w:.\-]+)\s*$", re.M)
RE_FORCELOAD_MARK = re.compile(
    r"Marked (?P<chunks>\d+) chunks in (?P<dim>[\w:.\-]+) from \[[^\]]*\] to \[[^\]]*\] to be force loaded"
)
RE_TICK_BEHIND = re.compile(r"Can't keep up!|Running \d+ms or \d+ ticks behind|running behind")
RE_ENTITY_COUNT = re.compile(r"entity-count\b(?P<body>.*)$", re.M)

# ----------------------------------------------------------------------------
# регексы md-среза BOTTLENECKS_3.md
# ----------------------------------------------------------------------------

RE_FL_MD = re.compile(
    r"forceload commands issued:\s*(?P<cmds>\d+)\s*\((?P<chunks>\d+) chunks force-loaded\)"
)
RE_TOTALS_MD = re.compile(r"entity totals seen:\s*\[(?P<vals>[^\]]*)\]")
RE_TYPES_MD = re.compile(r"top entity types \(max seen\):\s*(?P<body>[^\n]+)")
RE_TYPES_MD_PAIR = re.compile(r"(?P<type>[\w:.\-]+)×(?P<cnt>\d+)")
RE_TB_MD = re.compile(r"tick-behind warnings in log:\s*(?P<n>\d+)")
RE_GATE_MD = re.compile(r"gate (?P<id>1[abc]) [^:]*: (?P<res>PASS|FAIL)")
RE_SPAWN_POLLS_MD = re.compile(r"spawnable-chunk polls[^:]*:\s*\[(?P<vals>[^\]]*)\]")
RE_HEARTBEAT_MD = re.compile(r"alive-check heartbeat series:\s*\[(?P<vals>[^\]]*)\]")
RE_BOOT_MD = re.compile(r"boot reached Done:\s*\**(?P<done>\d+)\**")
RE_WORLD_SHA = re.compile(r"^world_sha256:\s*(?P<v>[0-9a-fA-F]{8,64})", re.M)
RE_CPU_INDEX = re.compile(r"^runner_cpu_index:\s*(?P<v>\d+)", re.M)
RE_ENV_KV = re.compile(r"^(?:-\s*)?(?P<key>[a-z_0-9]+):\s*(?P<val>[^\n(]*?)(?:\s*\(|\s*$)", re.M)


# ----------------------------------------------------------------------------
# модель фактов одной стороны
# ----------------------------------------------------------------------------

TIER = {"raw": 0, "report": 1, "env": 2}  # приоритет источника (меньше = сильнее)


@dataclass
class RunFacts:
    source: str = ""
    # fixture
    fixture_validity: Optional[str] = None                        # VALID / INVALID
    fixture_gates: Dict[str, str] = field(default_factory=dict)   # 1a/1b/1c -> PASS/FAIL
    fake_players: Optional[int] = None
    alive_players: List[int] = field(default_factory=list)        # polls level.players()
    spawnable_chunks: List[int] = field(default_factory=list)     # polls mobcaps header
    # population
    pop_seed: Optional[int] = None
    pop_target: Optional[int] = None
    pop_injected: Optional[int] = None
    pop_items: Optional[int] = None
    pop_hostiles: Optional[int] = None
    pop_passives: Optional[int] = None
    pop_plan: Optional[Tuple[int, int, int]] = None               # (items, hostiles, passives)
    topup_spawned_total: List[int] = field(default_factory=list)
    # forceload
    forceload_cmds: Optional[int] = None
    forceload_chunks: Optional[int] = None
    # entities
    entity_totals: List[int] = field(default_factory=list)        # polls/итоги total
    entity_types: Dict[str, int] = field(default_factory=dict)    # type -> max seen
    # дисциплина
    tick_behind: Optional[int] = None
    world_sha256: Optional[str] = None
    runner_cpu_index: Optional[str] = None
    boot_done: Optional[int] = None
    # метаданные
    provenance: Dict[str, str] = field(default_factory=dict)      # поле -> tier:файл
    notes: List[str] = field(default_factory=list)

    def set_field(self, name: str, value, tier: str, fname: str) -> bool:
        """Запись с приоритетом источника; True если записано."""
        if tier not in TIER:
            return False
        cur = self.provenance.get(name)
        cur_tier = TIER[cur.split(":", 1)[0]] if cur else 99
        if TIER[tier] >= cur_tier:
            return False  # уже записано более приоритетным источником
        setattr(self, name, value)
        self.provenance[name] = f"{tier}:{os.path.basename(fname)}"
        return True


# ----------------------------------------------------------------------------
# снифферы формата (чтобы парсеры не перекрёстно пачкали друг друга)
# ----------------------------------------------------------------------------

def is_server_log(t: str) -> bool:
    return ("POPULATION INJECT" in t or "Mobcaps for world" in t
            or "alive-check:" in t or bool(RE_LOG_TS.search(t)))


def is_md_report(t: str) -> bool:
    return ("entity totals seen" in t or "tick-behind warnings" in t
            or "BOTTLENECKS" in t and "FIXTURE-VALIDITY" in t)


def is_run_env(t: str) -> bool:
    return ("world_sha256" in t or "runner_cpu_index" in t
            or ("population_seed" in t and "population_target" in t))


# ----------------------------------------------------------------------------
# парсеры
# ----------------------------------------------------------------------------

def _ints(csv: str) -> List[int]:
    return [int(x) for x in re.findall(r"-?\d+", csv)]


def parse_server_log(text: str, facts: RunFacts, fname: str) -> None:
    """Сырой server-stdout.log (tier=raw)."""
    m = RE_INJECT_START.search(text)
    if m:
        facts.set_field("pop_seed", int(m.group("seed")), "raw", fname)
        facts.set_field("pop_target", int(m.group("target")), "raw", fname)
        if m.group("items"):
            facts.set_field("pop_plan", (int(m.group("items")), int(m.group("hostiles")),
                                         int(m.group("passives"))), "raw", fname)
    m = RE_INJECT_DONE.search(text)
    if m:
        facts.set_field("pop_target", int(m.group("target")), "raw", fname)
        facts.set_field("pop_injected", int(m.group("injected")), "raw", fname)
        facts.set_field("pop_items", int(m.group("items")), "raw", fname)
        facts.set_field("pop_hostiles", int(m.group("hostiles")), "raw", fname)
        facts.set_field("pop_passives", int(m.group("passives")), "raw", fname)
    for m in RE_TOPUP.finditer(text):
        facts.topup_spawned_total.append(int(m.group("topup")))
    for m in RE_ALIVE.finditer(text):
        facts.alive_players.append(int(m.group("real")))
    for m in RE_MOBCAPS_HDR.finditer(text):
        facts.spawnable_chunks.append(int(m.group("chunks")))
        # per-type строки идут сразу после заголовка mobcaps-блока (до следующего блока)
        tail = text[m.end():]
        nxt = tail.find("Mobcaps for world")
        if nxt != -1:
            tail = tail[:nxt]
        for lm in RE_TYPE_LINE.finditer(tail):
            t, cnt = lm.group("type"), int(lm.group("cnt"))
            if t not in facts.entity_types or cnt > facts.entity_types[t]:
                facts.entity_types[t] = cnt
        if facts.entity_types:
            facts.provenance.setdefault("entity_types", f"raw:{os.path.basename(fname)}")
    fl_cmds = fl_chunks = 0
    for m in RE_FORCELOAD_MARK.finditer(text):
        fl_cmds += 1
        fl_chunks += int(m.group("chunks"))
    if fl_cmds:
        facts.set_field("forceload_cmds", fl_cmds, "raw", fname)
        facts.set_field("forceload_chunks", fl_chunks, "raw", fname)
    # tick-behind: построчный скан (vanilla-строка содержит два маркера — считать СТРОКИ);
    # 0 «Can't keep up» = честный 0 (канон tick-behind=0)
    facts.set_field("tick_behind",
                    sum(1 for ln in text.splitlines() if RE_TICK_BEHIND.search(ln)),
                    "raw", fname)
    for m in RE_ENTITY_COUNT.finditer(text):
        # защитный парсер будущего формата: entity-count total=N minecraft:xxx=N ...
        mt = re.search(r"total=(\d+)", m.group("body"))
        if mt:
            facts.entity_totals.append(int(mt.group(1)))
            facts.provenance.setdefault("entity_totals", f"raw:{os.path.basename(fname)}")
    m = RE_FIXTURE.search(text)
    if m:
        facts.set_field("fixture_validity", m.group("v"), "raw", fname)


def parse_bottlenecks_md(text: str, facts: RunFacts, fname: str) -> None:
    """Срез-отчёт BOTTLENECKS_3.md (tier=report/env для встроенного run-env блока)."""
    m = RE_FIXTURE.search(text)
    if m:
        facts.set_field("fixture_validity", m.group("v"), "report", fname)
    for m in RE_GATE_MD.finditer(text):
        facts.fixture_gates[m.group("id")] = m.group("res")
        facts.provenance.setdefault("fixture_gates", f"report:{os.path.basename(fname)}")
    m = RE_FL_MD.search(text)
    if m:
        facts.set_field("forceload_cmds", int(m.group("cmds")), "report", fname)
        facts.set_field("forceload_chunks", int(m.group("chunks")), "report", fname)
    m = RE_TOTALS_MD.search(text)
    if m:
        vals = _ints(m.group("vals"))
        if vals:
            facts.set_field("entity_totals", vals, "report", fname)
    m = RE_TYPES_MD.search(text)
    if m:
        md_types = {p.group("type"): int(p.group("cnt"))
                    for p in RE_TYPES_MD_PAIR.finditer(m.group("body"))}
        if md_types:
            # report-список усечён до top-N: мержим max() к уже взятому raw
            merged = dict(md_types)
            for t, c in facts.entity_types.items():
                merged[t] = max(c, md_types.get(t, 0))
            if facts.entity_types:
                merged_src = facts.provenance.get("entity_types")
            else:
                merged_src = None
            facts.entity_types = merged
            if merged_src is None:
                facts.provenance["entity_types"] = f"report:{os.path.basename(fname)}"
    m = RE_TB_MD.search(text)
    if m:
        facts.set_field("tick_behind", int(m.group("n")), "report", fname)
    m = RE_SPAWN_POLLS_MD.search(text)
    if m and _ints(m.group("vals")):
        facts.spawnable_chunks = _ints(m.group("vals"))
        facts.provenance.setdefault("spawnable_chunks", f"report:{os.path.basename(fname)}")
    m = RE_HEARTBEAT_MD.search(text)
    if m:
        for tok in re.findall(r"'?(\d+)/\d+'?", m.group("vals")):
            facts.alive_players.append(int(tok))
        facts.provenance.setdefault("alive_players", f"report:{os.path.basename(fname)}")
    m = RE_BOOT_MD.search(text)
    if m:
        facts.set_field("boot_done", int(m.group("done")), "report", fname)
    m = RE_WORLD_SHA.search(text)
    if m:
        facts.set_field("world_sha256", m.group("v").lower(), "report", fname)
    m = RE_CPU_INDEX.search(text)
    if m:
        facts.set_field("runner_cpu_index", m.group("v"), "report", fname)


def parse_run_env(text: str, facts: RunFacts, fname: str) -> None:
    """run-env.txt / встроенный Run environment блок (tier=env)."""
    m = RE_WORLD_SHA.search(text)
    if m:
        facts.set_field("world_sha256", m.group("v").lower(), "env", fname)
    m = RE_CPU_INDEX.search(text)
    if m:
        facts.set_field("runner_cpu_index", m.group("v"), "env", fname)
    for m in RE_ENV_KV.finditer(text):
        key, val = m.group("key"), m.group("val").strip()
        if key == "population_target":
            facts.set_field("pop_target", int(val), "env", fname)
        elif key == "population_seed":
            facts.set_field("pop_seed", int(val), "env", fname)
        elif key == "fake_players":
            facts.set_field("fake_players", int(val), "env", fname)


def load_facts(paths: List[str]) -> RunFacts:
    facts = RunFacts(source=",".join(os.path.basename(p) for p in paths))
    for p in paths:
        with open(p, "r", encoding="utf-8", errors="replace") as f:
            text = f.read()
        if is_server_log(text):
            parse_server_log(text, facts, p)
        if is_md_report(text):
            parse_bottlenecks_md(text, facts, p)
        if is_run_env(text):
            parse_run_env(text, facts, p)
        if not (is_server_log(text) or is_md_report(text) or is_run_env(text)):
            facts.notes.append(f"формат не опознан: {p}")
    return facts


# ----------------------------------------------------------------------------
# сравнение
# ----------------------------------------------------------------------------

OK, FAIL, SKIP = "OK", "FAIL", "SKIP"


@dataclass
class GateResult:
    gate: str
    status: str
    detail: str


def _entity_band(a: int, b: int, tol: float, floor: int) -> Tuple[bool, float]:
    drift = abs(a - b)
    limit = max(floor, tol * max(a, b))
    return drift <= limit, (drift / max(a, b) * 100.0 if max(a, b) else 0.0)


def compare(van: RunFacts, leg: RunFacts, tol: float, floor: int,
            strict_entities: bool = False, ignore_world: bool = False) -> List[GateResult]:
    out: List[GateResult] = []

    def both(getter):
        a, b = getter(van), getter(leg)
        return (None, a, b) if a is None or b is None else ((a, b), a, b)

    # 1. FIXTURE-VALIDITY (+гейты 1a/1b/1c)
    pair, a, b = both(lambda f: f.fixture_validity)
    if pair is None:
        out.append(GateResult("FIXTURE-VALIDITY", FAIL,
                              f"нет данных (vanilla={a}, leg={b}) — паритет не верифицируем"))
    elif a == b == "VALID":
        bad = []
        sub = []
        for g in sorted(set(van.fixture_gates) | set(leg.fixture_gates)):
            ga, gb = van.fixture_gates.get(g), leg.fixture_gates.get(g)
            if ga and gb:
                sub.append(f"{g}:{ga}=={gb}")
                if ga != gb or ga != "PASS":
                    bad.append(f"{g}: {ga} vs {gb}")
            elif (ga or gb) and (ga or gb) != "PASS":
                bad.append(f"{g}: {ga or '—'} vs {gb or '—'}")
        if bad:
            out.append(GateResult("FIXTURE-VALIDITY", FAIL, f"VALID==VALID, но гейты: {'; '.join(bad)}"))
        else:
            d = " ".join(sub) if sub else "гейты 1a/1b/1c не публиковались"
            out.append(GateResult("FIXTURE-VALIDITY", OK, f"VALID==VALID; {d}"))
    else:
        out.append(GateResult("FIXTURE-VALIDITY", FAIL, f"vanilla={a} leg={b} (ожидается VALID==VALID)"))

    # 2. WORLD (парная дисциплина S7-96d)
    if ignore_world:
        out.append(GateResult("WORLD", SKIP, "--ignore-world"))
    else:
        pair, a, b = both(lambda f: f.world_sha256)
        if pair is None:
            out.append(GateResult("WORLD", SKIP, f"world_sha256 не найден (vanilla={a}, leg={b})"))
        elif a == b:
            out.append(GateResult("WORLD", OK, f"sha256={a[:12]}…"))
        else:
            out.append(GateResult("WORLD", FAIL,
                                  f"разные стенды: {a[:12]}… vs {b[:12]}… (парить по world_sha256, S7-96d)"))

    # 3. FAKE-PLAYERS
    pair, a, b = both(lambda f: f.fake_players)
    if pair is not None:
        out.append(GateResult("FAKE-PLAYERS", OK if a == b else FAIL,
                              f"{a} == {b}" if a == b else f"{a} != {b}"))
    else:
        out.append(GateResult("FAKE-PLAYERS", SKIP, "fake_players не найден с обеих сторон (не критично)"))

    # 4. POP-TOTALS (POPULATION INJECT DONE: target/injected/items/hostiles/passives)
    #    сравниваем ПЕРЕСЕЧЕНИЕ полей, имеющихся на обеих сторонах (md-срез без raw-лога
    #    несёт только population_target/seed из run-env-блока — честная частичная проверка)
    fields = [("target", "pop_target"), ("injected", "pop_injected"),
              ("items", "pop_items"), ("hostiles", "pop_hostiles"), ("passives", "pop_passives")]
    common, one_side, mism = [], [], []
    for label, attr in fields:
        a, b = getattr(van, attr), getattr(leg, attr)
        if a is not None and b is not None:
            common.append((label, a, b))
            if a != b:
                mism.append(f"{label}: {a} != {b}")
        elif a is not None or b is not None:
            one_side.append(f"{label}(van={a},leg={b})")
    _, pa, pb = both(lambda f: f.pop_plan)
    plan_note = None
    if pa is not None and pb is not None:
        if pa != pb:
            mism.append(f"plan(INJECT START): {pa} != {pb}")
    elif pa is not None or pb is not None:
        one_side.append(f"plan(INJECT START)(van={pa},leg={pb})")
    if mism:
        out.append(GateResult("POP-TOTALS", FAIL, "; ".join(mism)))
    elif common:
        vals = " ".join(f"{l}={b}" for l, _a, b in common)
        note = f" [полусторона без raw-лога: {', '.join(one_side)}]" if one_side else ""
        out.append(GateResult("POP-TOTALS", OK, vals + note))
    elif one_side:
        out.append(GateResult("POP-TOTALS", SKIP,
                              f"pop-totals только на одной стороне: {', '.join(one_side)}"))
    else:
        out.append(GateResult("POP-TOTALS", FAIL,
                              "POPULATION INJECT DONE не найден ни на одной стороне — pop-totals не верифицируем"))

    # 5. SEED
    pair, a, b = both(lambda f: f.pop_seed)
    if pair is None:
        out.append(GateResult("SEED", FAIL, f"seed не найден (vanilla={a}, leg={b})"))
    else:
        out.append(GateResult("SEED", OK if a == b else FAIL, f"seed={a}" if a == b else f"{a} != {b}"))

    # 6. FORCELOAD
    pair, a, b = both(lambda f: f.forceload_chunks)
    if pair is None:
        out.append(GateResult("FORCELOAD", FAIL, f"forceload не найден (vanilla={a}, leg={b})"))
    else:
        pc, ca, cb = both(lambda f: f.forceload_cmds)
        if a != b:
            extra = f", cmds {ca} vs {cb}" if pc else ""
            out.append(GateResult("FORCELOAD", FAIL, f"chunks {a} != {b}{extra}"))
        elif pc and ca != cb:
            out.append(GateResult("FORCELOAD", FAIL, f"chunks {a}=={b}, но cmds {ca} != {cb}"))
        else:
            out.append(GateResult("FORCELOAD", OK,
                                  f"chunks={a}" + (f", cmds={ca}" if pc else "")))

    # 7. ENTITY-TYPES (per-type band; strict = бит-в-байт)
    if not van.entity_types or not leg.entity_types:
        out.append(GateResult("ENTITY-TYPES", FAIL,
                              f"per-type данные не найдены (vanilla={len(van.entity_types)} типов,"
                              f" leg={len(leg.entity_types)} типов)"))
    else:
        shared = sorted(set(van.entity_types) & set(leg.entity_types))
        one_side = sorted(set(van.entity_types) ^ set(leg.entity_types))
        bad, worst = [], (0.0, "—")
        for t in shared:
            a, b = van.entity_types[t], leg.entity_types[t]
            in_band, pct = _entity_band(a, b, 0.0 if strict_entities else tol,
                                        0 if strict_entities else floor)
            if pct > worst[0]:
                worst = (pct, t)
            if not in_band:
                bad.append(f"{t}: {a} vs {b} (Δ{pct:.1f}%)")
        if strict_entities and one_side:
            bad.append(f"односторонние типы (strict): {one_side}")
        if bad:
            out.append(GateResult("ENTITY-TYPES", FAIL,
                                  f"{len(bad)} нарушений из {len(shared)} общих типов: " + "; ".join(bad[:8])))
        else:
            note = (f"; односторонние (top-N усечение/сырой перечь): "
                    f"{len(one_side)} типов [{', '.join(one_side[:5])}…]") if one_side else ""
            out.append(GateResult("ENTITY-TYPES", OK,
                                  f"{len(shared)} общих типов в банде (tol={tol:.2%}, floor={floor}),"
                                  f" макс-дрейф {worst[0]:.2f}% ({worst[1]}){note}"))

    # 8. TICK-BEHIND == 0 (канон)
    pair, a, b = both(lambda f: f.tick_behind)
    if pair is None:
        out.append(GateResult("TICK-BEHIND", FAIL,
                              f"нет данных (vanilla={a}, leg={b}) — канон требует tick-behind=0"))
    elif a == b == 0:
        out.append(GateResult("TICK-BEHIND", OK, "0 == 0"))
    else:
        out.append(GateResult("TICK-BEHIND", FAIL, f"vanilla={a}, leg={b} (канон: 0)"))

    return out


def build_diff(van: RunFacts, leg: RunFacts, gates: List[GateResult],
               tol: float, floor: int) -> List[str]:
    lines = [f"  {g.status:<4} | {g.gate:<17} | {g.detail}" for g in gates]
    if van.topup_spawned_total or leg.topup_spawned_total:
        lines.append(f"  info | TOPUP             | topupSpawnedTotal vanilla={van.topup_spawned_total}"
                     f" leg={leg.topup_spawned_total} (естественный чурн — не гейт)")
    if van.spawnable_chunks and leg.spawnable_chunks:
        lines.append(f"  info | SPAWNABLE-CHUNKS  | vanilla={van.spawnable_chunks} leg={leg.spawnable_chunks}")
    vt = max(van.entity_totals) if van.entity_totals else None
    lt = max(leg.entity_totals) if leg.entity_totals else None
    if vt and lt:
        in_band, pct = _entity_band(vt, lt, tol, floor)
        lines.append(f"  info | ENTITY-TOTALS     | max vanilla={vt} leg={lt} (Δ{pct:.2f}%,"
                     f" band {'in' if in_band else 'OUT'})")
    for side, f in (("vanilla", van), ("leg", leg)):
        if f.runner_cpu_index:
            idx = int(f.runner_cpu_index)
            band = "in-band" if 6_000_000 <= idx <= 9_500_000 else "OUT-OF-BAND 6.0-9.5M"
            lines.append(f"  info | RUNNER-CPU-INDEX  | {side}={f.runner_cpu_index} ({band})")
    return lines


def evaluate(van_paths: List[str], leg_paths: List[str], tol: float, floor: int,
             strict_entities: bool = False, ignore_world: bool = False):
    """Полный конвейер: load → compare → verdict. Возвращает (verdict, reasons, gates, diff, van, leg)."""
    van = load_facts(van_paths)
    leg = load_facts(leg_paths)
    gates = compare(van, leg, tol, floor, strict_entities, ignore_world)
    failed = [g for g in gates if g.status == FAIL]
    evaluated = [g for g in gates if g.status != SKIP]
    if not evaluated:
        v, reasons = "PARITY-FAIL", ["ни один гейт не вычислен — вход пуст/неопознан"]
    elif failed:
        v, reasons = "PARITY-FAIL", [f"{g.gate}: {g.detail}" for g in failed]
    else:
        v, reasons = "PARITY-OK", []
    diff = build_diff(van, leg, gates, tol, floor)
    return v, reasons, gates, diff, van, leg


# ----------------------------------------------------------------------------
# самотест: реальная пара ROUND-466 + негатив-контроль
# ----------------------------------------------------------------------------

SELFTEST_PAIR_NOTE = ("пара ROUND-466: vanilla=c58-artifacts/run-36228910649 (raw-лог+md+env), "
                      "leg=A2_BOTTLENECKS_3.md+A2_run-env.txt (C42-якорь run 36230386556)")


def _find_selftest_artifacts(root: str) -> Tuple[List[str], List[str]]:
    """Канонические пути ROUND-466 + фолбэк-скан по DIR."""
    c58 = os.path.join(root, "c58-artifacts", "run-36228910649")
    van = [os.path.join(c58, n) for n in ("server-stdout.log", "BOTTLENECKS_3.md", "run-env.txt")]
    van = [p for p in van if os.path.isfile(p)]
    leg = [p for p in (os.path.join(root, "A2_BOTTLENECKS_3.md"),
                       os.path.join(root, "A2_run-env.txt")) if os.path.isfile(p)]
    if van and leg:
        return van, leg
    md = [os.path.join(root, f) for f in sorted(os.listdir(root)) if "BOTTLENECKS" in f]
    env = [os.path.join(root, f) for f in sorted(os.listdir(root)) if "run-env" in f]
    logs = []
    for dirpath, _d, files in os.walk(root):
        logs += [os.path.join(dirpath, fn) for fn in files if fn == "server-stdout.log"]
    van = ([logs[0]] if logs else []) + md[:1] + env[:1]
    leg = md[1:2] + env[1:2]
    return van, leg


def _mutate(text: str, subs: List[Tuple[str, str]]) -> str:
    t = text
    for a, b in subs:
        if a not in t:
            raise AssertionError(f"мутация не нашла образец: {a!r}")
        t = t.replace(a, b, 1)
    return t


def self_test(root: str, tol: float, floor: int) -> int:
    checks: List[Tuple[str, bool, str]] = []

    def run(van_paths, leg_paths, **kw):
        return evaluate(van_paths, leg_paths, kw.get("tol", tol), kw.get("floor", floor),
                        kw.get("strict", False), kw.get("ignore_world", False))[:4]

    van_paths, leg_paths = _find_selftest_artifacts(root)
    if not van_paths or not leg_paths:
        print(f"SELFTEST FAIL: пара логов не найдена в {root}")
        return 1
    print(f"== САМОТЕСТ ({SELFTEST_PAIR_NOTE})")
    print(f"   vanilla = {[os.path.basename(p) for p in van_paths]}")
    print(f"   leg     = {[os.path.basename(p) for p in leg_paths]}")

    # P1: идентичность (same-boot A/A) — все гейты, вкл. POP-TOTALS
    v, _r, gates, _d = run(van_paths, van_paths)
    checks.append(("P1 identity(A/A) -> PARITY-OK", v == "PARITY-OK", v))
    checks.append(("P1 POP-TOTALS вычислен",
                   any(g.gate == "POP-TOTALS" and g.status == "OK" for g in gates), ""))
    # P2: реальная пара C41-vs-C42 (leg = md-срез + run-env)
    v, _r, gates, _d = run(van_paths, leg_paths)
    checks.append(("P2 pair(C41-vs-C42) -> PARITY-OK", v == "PARITY-OK", v))
    pop2 = next((g for g in gates if g.gate == "POP-TOTALS"), None)
    checks.append(("P2 POP-TOTALS частично (общее target, injected — полусторона)",
                   pop2 is not None and pop2.status == "OK" and "полусторона" in pop2.detail,
                   pop2.detail if pop2 else "нет гейта"))

    # негатив-контроль: мутации leg-стороны через тот же конвейер
    with open(leg_paths[0], "r", encoding="utf-8", errors="replace") as f:
        leg_md_text = f.read()
    mut_cases = [
        ("N1 seed 42->43 -> FAIL(SEED)",
         [("population_seed: 42", "population_seed: 43")], "SEED"),
        ("N2 forceload 9216->4608 -> FAIL(FORCELOAD)",
         [("9216 chunks force-loaded", "4608 chunks force-loaded")], "FORCELOAD"),
        ("N3 fixture VALID->INVALID -> FAIL(FIXTURE-VALIDITY)",
         [("FIXTURE-VALIDITY: VALID", "FIXTURE-VALIDITY: INVALID")], "FIXTURE-VALIDITY"),
        ("N4 entity item +25% -> FAIL(ENTITY-TYPES)",
         [("minecraft:item×103319", "minecraft:item×130000")], "ENTITY-TYPES"),
        ("N5 tick-behind 0->3 -> FAIL(TICK-BEHIND)",
         [("tick-behind warnings in log: 0", "tick-behind warnings in log: 3")], "TICK-BEHIND"),
    ]
    for name, subs, expect_gate in mut_cases:
        try:
            mutated = _mutate(leg_md_text, subs)
            fd, tmp = tempfile.mkstemp(prefix=f".c85_mut_{expect_gate}_", suffix=".md")
            with os.fdopen(fd, "w", encoding="utf-8") as f:
                f.write(mutated)
            v, _r, gates, _d = run(van_paths, [tmp, *leg_paths[1:]])
            checks.append((name, v == "PARITY-FAIL"
                           and any(g.gate == expect_gate and g.status == "FAIL" for g in gates), v))
            os.unlink(tmp)
        except (AssertionError, OSError) as e:
            checks.append((name, False, str(e)))

    # N6: мутация raw-лога (pop-totals injected)
    try:
        with open(van_paths[0], "r", encoding="utf-8", errors="replace") as f:
            raw = f.read()
        m = re.search(r"POPULATION INJECT DONE target=(\d+) injected=(\d+)", raw)
        if m:
            raw_mut = (raw[:m.start()] + f"POPULATION INJECT DONE target={m.group(1)} injected=140000"
                       + raw[m.end():])
            fd, tmp = tempfile.mkstemp(prefix=".c85_mut_POP-TOTALS_", suffix=".log")
            with os.fdopen(fd, "w", encoding="utf-8") as f:
                f.write(raw_mut)
            v, _r, gates, _d = run(van_paths, [tmp])
            checks.append(("N6 injected 150000->140000 (raw) -> FAIL(POP-TOTALS)",
                           v == "PARITY-FAIL"
                           and any(g.gate == "POP-TOTALS" and g.status == "FAIL" for g in gates), v))
            os.unlink(tmp)
        else:
            checks.append(("N6 injected 150000->140000 (raw) -> FAIL(POP-TOTALS)", False,
                           "INJECT DONE не найден в vanilla-логе"))
    except OSError as e:
        checks.append(("N6 injected 150000->140000 (raw) -> FAIL(POP-TOTALS)", False, str(e)))

    # кросс-чек: сырой лог == md-срез (эквивалентность извлечения)
    c58_md = os.path.join(os.path.dirname(van_paths[0]), "BOTTLENECKS_3.md")
    if van_paths[0].endswith(".log") and os.path.isfile(c58_md):
        van = load_facts(van_paths)
        md_facts = RunFacts()
        with open(c58_md, "r", encoding="utf-8", errors="replace") as f:
            parse_bottlenecks_md(f.read(), md_facts, c58_md)
        agree = all(van.entity_types.get(t) == c for t, c in md_facts.entity_types.items())
        checks.append(("X1 per-type max(raw) == md-срез", agree,
                       f"raw={len(van.entity_types)} типов, md top-N={len(md_facts.entity_types)}"))
        checks.append(("X2 forceload(raw) == md-срез",
                       van.forceload_chunks == md_facts.forceload_chunks
                       and van.forceload_cmds == md_facts.forceload_cmds,
                       f"raw=({van.forceload_cmds},{van.forceload_chunks})"
                       f" md=({md_facts.forceload_cmds},{md_facts.forceload_chunks})"))

    ok = True
    print("== САМОТЕСТ РЕЗУЛЬТАТ")
    for name, passed, info in checks:
        print(f"  {'PASS' if passed else 'FAIL'} | {name}" + (f" | {info}" if info else ""))
        ok = ok and passed
    print(f"VERDICT: {'SELFTEST-PASS' if ok else 'SELFTEST-FAIL'}")
    return 0 if ok else 1


# ----------------------------------------------------------------------------
# CLI
# ----------------------------------------------------------------------------

def _facts_json(f: RunFacts) -> dict:
    return {k: getattr(f, k) for k in (
        "source", "fixture_validity", "fixture_gates", "fake_players", "pop_seed", "pop_target",
        "pop_injected", "pop_items", "pop_hostiles", "pop_passives", "pop_plan", "topup_spawned_total",
        "forceload_cmds", "forceload_chunks", "entity_totals", "entity_types", "tick_behind",
        "world_sha256", "runner_cpu_index", "boot_done")}


USAGE = """usage:
  parity_validator.py [OPTIONS] VANILLA_FILES... -- LEG_FILES...
  parity_validator.py --self-test [DIR]

VANILLA_FILES / LEG_FILES : server-stdout.log и/или BOTTLENECKS_3.md и/или run-env.txt
  (одна сторона может быть стеком из нескольких файлов; raw-лог приоритетнее среза)
OPTIONS:
  --entity-tolerance X   относительная банда per-type дрейфа (default 0.05)
  --entity-abs-floor N   абсолютный допуск малых популяций (default 64)
  --strict-entities      бит-в-байт per-type (tol=0, floor=0)
  --ignore-world         не гейтить world_sha256
  --json PATH            машинный отчёт (JSON)
  --quiet                только VERDICT-строка
  --self-test [DIR]      самотест на реальной паре ROUND-466 + негатив-контроль
"""

_VALUE_OPTS = {"--entity-tolerance": ("entity_tolerance", float),
               "--entity-abs-floor": ("entity_abs_floor", int),
               "--json": ("json_path", str)}
_FLAG_OPTS = {"--strict-entities": "strict_entities",
              "--ignore-world": "ignore_world",
              "--quiet": "quiet"}


def _parse_cli(argv: List[str]) -> dict:
    """Ручной парсер: argparse не умеет две '*'-позиции вокруг '--'."""
    o = {"entity_tolerance": 0.05, "entity_abs_floor": 64, "strict_entities": False,
         "ignore_world": False, "quiet": False, "json_path": None,
         "self_test": None, "vanilla": [], "leg": []}
    side = "vanilla"
    i = 0
    while i < len(argv):
        t = argv[i]
        if t in ("-h", "--help"):
            print(__doc__)
            raise SystemExit(0)
        if t == "--":
            side = "leg"
            i += 1
            continue
        if t == "--self-test" or t.startswith("--self-test="):
            if "=" in t:
                o["self_test"] = t.split("=", 1)[1]
            elif i + 1 < len(argv) and not argv[i + 1].startswith("-") and os.path.isdir(argv[i + 1]):
                o["self_test"] = argv[i + 1]
                i += 1
            else:
                o["self_test"] = "/home/z/rounds/ROUND-466"
            i += 1
            continue
        if t in _VALUE_OPTS:
            key, conv = _VALUE_OPTS[t]
            if i + 1 >= len(argv):
                raise ValueError(f"{t} требует значение")
            o[key] = conv(argv[i + 1])
            i += 2
            continue
        if t in _FLAG_OPTS:
            o[_FLAG_OPTS[t]] = True
            i += 1
            continue
        if t.startswith("-") and len(t) > 1:
            raise ValueError(f"неизвестная опция: {t}")
        o[side].append(t)
        i += 1
    return o


def main(argv: Optional[List[str]] = None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    try:
        args = _parse_cli(argv)
    except ValueError as e:
        print(f"ОШИБКА CLI: {e}\n{USAGE}")
        return 2
    except SystemExit:
        return 0

    if args["self_test"]:
        return self_test(args["self_test"], args["entity_tolerance"], args["entity_abs_floor"])

    van_paths, leg_paths = args["vanilla"], args["leg"]
    if not van_paths or not leg_paths:
        print("ОШИБКА CLI: нужны обе стороны через '--'\n" + USAGE)
        return 2
    try:
        v, reasons, gates, diff, van, leg = evaluate(
            van_paths, leg_paths, args["entity_tolerance"], args["entity_abs_floor"],
            args["strict_entities"], args["ignore_world"])
    except OSError as e:
        print(f"ОШИБКА входа: {e}")
        return 2

    lines = []
    if not args["quiet"]:
        lines.append("== PARITY VALIDATOR (закон 4 + 20d, Swarm v19.0) ==")
        lines.append(f"vanilla: {', '.join(van_paths)}")
        lines.append(f"leg    : {', '.join(leg_paths)}")
        lines.extend(diff)
    tail = f" [{'; '.join(reasons)}]" if reasons and v == "PARITY-FAIL" else ""
    lines.append(f"VERDICT: {v}{tail}")
    print("\n".join(lines))

    if args["json_path"]:
        report = {
            "verdict": v, "reasons": reasons,
            "gates": [g.__dict__ for g in gates],
            "vanilla": _facts_json(van), "leg": _facts_json(leg),
            "params": {"entity_tolerance": args["entity_tolerance"],
                       "entity_abs_floor": args["entity_abs_floor"],
                       "strict_entities": args["strict_entities"],
                       "ignore_world": args["ignore_world"]},
        }
        with open(args["json_path"], "w", encoding="utf-8") as f:
            json.dump(report, f, ensure_ascii=False, indent=2, default=str)
    return 0 if v == "PARITY-OK" else 1


if __name__ == "__main__":
    sys.exit(main())
