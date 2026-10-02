| surface | window | queries D | mutations D | dirty% | verdict |
|---|---|---|---|---|---|
| hopper-inventory | 1788890299->1788890329 | 10426 | 570 | 5.467% | honest 10-100x class |
| hopper-inventory | 1788890329->1788890359 | 12300 | 300 | 2.439% | honest 10-100x class |
| hopper-inventory | 1788890359->1788890389 | 12300 | 300 | 2.439% | honest 10-100x class |
| hopper-inventory | 1788890389->1788890419 | 12300 | 300 | 2.439% | honest 10-100x class |
| hopper-inventory | 1788890419->1788890449 | 12300 | 300 | 2.439% | honest 10-100x class |
| hopper-inventory | 1788890449->1788890479 | 12300 | 300 | 2.439% | honest 10-100x class |
| hopper-inventory | 1788890479->1788890509 | 12300 | 300 | 2.439% | honest 10-100x class |
| hopper-inventory | 1788890509->1788890539 | 12300 | 300 | 2.439% | honest 10-100x class |
| hopper-inventory | 1788890539->1788890569 | 12300 | 300 | 2.439% | honest 10-100x class |
| hopper-inventory | 1788890569->1788890599 | 12300 | 300 | 2.439% | honest 10-100x class |
| hopper-push-tick | 1788890299->1788890329 | 7200 | n/a | n/a | UNMEASURABLE (< noise floor) |
| hopper-push-tick | 1788890329->1788890359 | 7200 | n/a | n/a | UNMEASURABLE (< noise floor) |
| hopper-push-tick | 1788890359->1788890389 | 7200 | n/a | n/a | UNMEASURABLE (< noise floor) |
| hopper-push-tick | 1788890389->1788890419 | 7200 | n/a | n/a | UNMEASURABLE (< noise floor) |
| hopper-push-tick | 1788890419->1788890449 | 7200 | n/a | n/a | UNMEASURABLE (< noise floor) |
| hopper-push-tick | 1788890449->1788890479 | 7200 | n/a | n/a | UNMEASURABLE (< noise floor) |
| hopper-push-tick | 1788890479->1788890509 | 7200 | n/a | n/a | UNMEASURABLE (< noise floor) |
| hopper-push-tick | 1788890509->1788890539 | 7200 | n/a | n/a | UNMEASURABLE (< noise floor) |
| hopper-push-tick | 1788890539->1788890569 | 7200 | n/a | n/a | UNMEASURABLE (< noise floor) |
| hopper-push-tick | 1788890569->1788890599 | 7200 | n/a | n/a | UNMEASURABLE (< noise floor) |

Honest-class ceilings (Amdahl-capped, CPU share from TASK-81 census):
- fluid-push: no measurable window
- hopper-inventory: CPU share unmeasured - run CPU census first
- collision: no measurable window
- be-tick: CPU share unmeasured - run CPU census first

Rule: >100x claims require dirty%<1 AND machinery>=90% AND a measured A/B (TASK-80 G-FLUID lesson: hit-rate alone is insufficient, p-value decides).
