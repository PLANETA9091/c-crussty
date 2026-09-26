#!/usr/bin/env python3
"""TASK-464-36 marker: КЛИМБ poi456-4 нога-3, seed-вариация.

Нога-3 = точный ре-диспатч носителя cmp456_poi @5ecd841a
(population_seed=43 вместо seed=42 гипотеза-дельта: P0-parity нога
была seed-42-зависима; seed-43 проверяет робастность пар-кандидата.
НЕ слепой ре-ролл — смена семента = новая точка).

Repo: PLANETA9091/c-crussty
Branch: round-464-poi-seed43 (base 5ecd841a)
Workflow: world-bench-parallel.yml
Lever: cmp456_poi @ 5ecd841a, radius=640, seconds=300,
population_target=150000, population_seed=43
"""
