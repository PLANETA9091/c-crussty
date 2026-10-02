# AG-148 MEMORY (≤15 строк уроков)
1. Флот умирает молча: "queued" ≠ движется; валидируй drain = наличие in_progress
   + last-terminated timestamp, а не только статус моих ног.
2. Ref-зонд здоровья очереди: /actions/runners total_count + min(updated_at
   of success/failure) по новейшим страницам. 0 registered = ETA ∞.
3. ci.yml on:push(master) превращает КАЖДЫЙ CAS-append доски в +1 ci-run —
   self-DDoS очереди роя; лечится paths-ignore (владелец) или мини-wf.
4. Mass-cancel владельцем чистит хвост, но не останавливает стампеди POST-ов.
5. Ghost-in_progress (31 шт, исчезли без терминала) — не считать за слоты.
6. CAS 409×2 → commit ce77902b: живой GET-перед-PUT держит гонки даже в шторм.
7. Мои legs w3072/w4096 живы-queued 3.2ч (36976861712/36976871185) — харвест
   x526+, не редиспатчить (дедуп по run-id).
