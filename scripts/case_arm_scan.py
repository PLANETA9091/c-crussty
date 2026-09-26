#!/usr/bin/env python3
# case_arm_scan.py — x466 P0-канон (Л145): MERGE #9 line-union съел ';;'
# терминатор case-арма cmp456_chunkmono в bench/world3/run_world3.sh
# (bash -n FAIL line 532, canary-405 36226176808 failure @07:21:11Z).
#
# bash -n ловит отсутствующий ';;' МЕЖДУ армами (syntax error near ')'),
# но НЕ ловит арм без ';;' перед 'esac' (последний арм легален без
# терминатора) — а именно там line-union портит файл тихо. Этот сканер:
#   FAIL  — арм, за которым следует следующий арм без ';;' / ';&' / ';;&'
#           (дубль bash -n, но с точным file:line арма-виновника);
#   WARN  — арм без явного ';;' перед 'esac' (легальный bash, но
#           line-union-хрупкий: канон — явный терминатор везде);
#   INFO  — арм без ';;' внутри $('...') / backtick-подстановок, heredoc-тел
#           и комментариев игнорируется (не токенизируется).
#
# Полноценный мини-парсер shell (кавычки, heredoc, $(...), backtick, вложенные
# case, (extglob), арифметика). Не претендует на полный bash — это ЦЕНЗ
# терминаторов case-армов, а не общий линтер; bash -n остаётся авторитетом.
#
# Usage: python3 scripts/case_arm_scan.py FILE...   (exit 0 = нет FAIL;
#        exit 1 = есть FAIL; WARN не влияют на exit, печатаются)

import sys

IDENT = set("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_")

class Scanner:
    def __init__(self, path, text):
        self.path = path
        self.text = text
        self.n = len(text)
        self.i = 0
        self.line = 1
        self.fails = []
        self.warns = []
        self.case_stack = []   # {'state': 'await-in'|'arms', 'in_arm': bool, 'arm_line': int}
        self.depth = 0         # ( ) вне case-паттернов
        self.pending_heredocs = []
        self.in_heredoc = False
        self.word_start_ok = True  # '#' начинает комментарий только в позиции слова

    # ---------- низкоуровневые помощники ----------
    def peek(self, k=0):
        j = self.i + k
        return self.text[j] if j < self.n else ""

    def advance(self):
        if self.i >= self.n:
            return ""  # EOF-защита (незакрытые кавычки/подстановки — bash -n поймает)
        c = self.text[self.i]
        self.i += 1
        if c == "\n":
            self.line += 1
            self.word_start_ok = True
        else:
            self.word_start_ok = c in " \t;&|(" or not self.word_start_ok
            # упрощение: словная позиция определяется пробелами/разделителями
        return c

    def at_word_end(self):
        c = self.peek()
        return c == "" or c in " \t\n;&|()<>#" or c == ""

    # ---------- токен-обработка ----------
    def handle_word(self, w, wl):
        if w == "case":
            self.case_stack.append({"state": "await-in", "in_arm": False, "arm_line": 0})
        elif w == "in":
            if self.case_stack and self.case_stack[-1]["state"] == "await-in":
                self.case_stack[-1]["state"] = "arms"
        elif w == "esac":
            if self.case_stack:
                ctx = self.case_stack.pop()
                if ctx["state"] == "arms" and ctx["in_arm"]:
                    self.warns.append(
                        "%s:%d: WARN арм (открыт :%d) без явного ';;' перед 'esac' "
                        "(легальный bash, но line-union-хрупкий)" % (self.path, wl, ctx["arm_line"]))

    def open_arm(self, line):
        if not self.case_stack:
            return
        ctx = self.case_stack[-1]
        if ctx["state"] != "arms":
            return
        if ctx["in_arm"]:
            self.fails.append(
                "%s:%d: FAIL арм (открыт :%d) НЕ terminates ';;' — следующий арм/')' на :%d "
                "(x466-класс: MERGE line-union съел терминатор)" % (self.path, line, ctx["arm_line"], line))
        ctx["in_arm"] = True
        ctx["arm_line"] = line

    def close_arm(self, term_len, line):
        # терминатор валиден только в arms-состоянии внутреннего case
        if self.case_stack and self.case_stack[-1]["state"] == "arms" and self.depth == 0:
            self.case_stack[-1]["in_arm"] = False
        # вне case — ';;'/';&' syntactically invalid; bash -n поймает, тут молчим

    # ---------- heredoc ----------
    def try_heredoc_start(self):
        # вызван когда self.text[self.i] == '<' и peek(1) == '<'
        j = self.i + 2
        if j < self.n and self.text[j] == "<":  # here-string <<<
            self.i = j + 1
            return
        if j < self.n and self.text[j] == "-":
            j += 1
        q = None
        if j < self.n and self.text[j] in "'\"":
            q = self.text[j]
            j += 1
        d = []
        while j < self.n and (self.text[j] in IDENT or (q is None and self.text[j] not in " \t\n;&|()<>\\")):
            if q and self.text[j] == q:
                j += 1
                break
            d.append(self.text[j])
            j += 1
        delim = "".join(d)
        if delim:
            self.pending_heredocs.append(delim)
        self.i = j

    def consume_heredoc_bodies(self):
        # тела начинаются после новой строки; вызывается на '\n' с pending
        while self.pending_heredocs:
            delim = self.pending_heredocs[0]
            body_start = self.i + 1  # за текущим '\n'
            j = body_start
            found = False
            while j <= self.n:
                eol = self.text.find("\n", j)
                if eol == -1:
                    eol_candidate = self.n
                else:
                    eol_candidate = eol
                raw = self.text[j:eol_candidate]
                stripped = raw.strip()
                if stripped == delim or (raw == delim):
                    self.i = eol_candidate
                    # каждый '\n' тела считается ровно раз; '\n' delim-строки
                    # съест следующий advance() родительского шага
                    self.line += self.text.count("\n", body_start, eol_candidate)
                    found = True
                    break
                if eol == -1:
                    break
                j = eol + 1
            if not found:
                # незакрытый heredoc — bash -n поймает; снимаем чтобы не зависнуть
                self.pending_heredocs.pop(0)
                continue
            self.pending_heredocs.pop(0)
            # после выхода i стоит на '\n' terminating delim-строки (или EOF)

    # ---------- подстановки ----------
    def _case_awaits_pattern(self):
        # ')' в этом состоянии — закрыватель ПАТТЕРНА арма (m) / (a|b)), а не
        # конец $( ... ) — bash-канон: case-контекст старше подстановки.
        return (bool(self.case_stack) and self.case_stack[-1]["state"] == "arms"
                and not self.case_stack[-1]["in_arm"] and self.depth == 0)

    def scan_cmd_sub(self):
        # self.i стоит на '(' в '$(' — сканируем до ')' НЕ являющейся
        # закрывателем паттерна активного case (урок good1-фикстуры:
        # $(case x in m) echo ;; esac) — первый ')' это арм, не конец $()).
        save_stack = self.case_stack
        save_depth = self.depth
        self.case_stack = []
        self.depth = 0
        try:
            while self.i < self.n:
                c = self.peek()
                if c == ")" and not self._case_awaits_pattern():
                    self.advance()
                    return
                self.step(top=False)
        finally:
            self.case_stack = save_stack
            self.depth = save_depth

    def scan_backtick(self):
        save_stack = self.case_stack
        save_depth = self.depth
        self.case_stack = []
        self.depth = 0
        self.advance()  # consume `
        while self.i < self.n:
            c = self.peek()
            if c == "`":
                self.advance()
                break
            self.step(top=False)
        self.case_stack = save_stack
        self.depth = save_depth

    # ---------- главный шаг ----------
    def step(self, top=True):
        c = self.peek()

        if c == "\n" and self.pending_heredocs:
            self.advance()
            self.consume_heredoc_bodies()
            return

        if c == "\\":
            self.advance()
            if self.i < self.n:
                self.advance()
            return

        if c == "'":
            self.advance()
            while self.i < self.n and self.peek() != "'":
                self.advance()
            self.advance()
            return

        if c == '"':
            self.advance()
            while self.i < self.n:
                ch = self.peek()
                if ch == "\\":
                    self.advance()
                    self.advance()
                    continue
                if ch == '"':
                    self.advance()
                    return
                if ch == "$" and self.peek(1) == "(" and self.peek(2) == "(":
                    self.advance(); self.advance(); self.advance()
                    ad = 2
                    while self.i < self.n and ad > 0:
                        ac = self.peek()
                        if ac == "(":
                            ad += 1
                        elif ac == ")":
                            ad -= 1
                        self.advance()
                    continue
                if ch == "$" and self.peek(1) == "(":
                    self.advance()  # $
                    self.advance()  # (
                    self.scan_cmd_sub()
                    continue
                if ch == "`":
                    self.scan_backtick()
                    continue
                self.advance()
            return

        if c == "$" and self.peek(1) == "(" and self.peek(2) == "(":
            # $(( ... )) — арифметика: case/esac внутри невозможны, сканируем
            # до баланса скобок (урок crac_p6b: $((L65+1));; — вложенная
            # ( ловилась как cmd-sub и рожала фантомные FAIL на '))').
            self.advance(); self.advance(); self.advance()
            ad = 2
            while self.i < self.n and ad > 0:
                ac = self.peek()
                if ac == "(":
                    ad += 1
                elif ac == ")":
                    ad -= 1
                self.advance()
            return

        if c == "$" and self.peek(1) == "(":
            self.advance()
            self.advance()
            self.scan_cmd_sub()
            return

        if c == "`":
            self.scan_backtick()
            return

        if c == "#":
            # комментарий только в словной позиции
            if self.word_start_ok or self.prev_sep():
                while self.i < self.n and self.peek() != "\n":
                    self.advance()
                return
            # иначе literal '#' внутри слова — упадём в обычную обработку

        if c == ";":
            # ;;& / ;; / ;&
            if self.peek(1) == ";":
                if self.peek(2) == "&":
                    self.advance(); self.advance(); self.advance()
                    self.close_arm(3, self.line)
                    return
                self.advance(); self.advance()
                self.close_arm(2, self.line)
                return
            if self.peek(1) == "&":
                self.advance(); self.advance()
                self.close_arm(2, self.line)
                return
            self.advance()
            return

        if c == "<" and self.peek(1) == "<":
            self.try_heredoc_start()
            return

        if c == "(":
            # (alpha|beta) — ведущая скобка списка паттернов: не арифметика и
            # не под-оболочка; парная ')' откроет арм на уровне depth 0.
            if (self.case_stack and self.case_stack[-1]["state"] == "arms"
                    and not self.case_stack[-1]["in_arm"] and self.depth == 0):
                self.advance()
                return
            self.advance()
            self.depth += 1
            return

        if c == ")":
            self.advance()
            if self.depth == 0:
                self.open_arm(self.line)
            else:
                self.depth -= 1
            return

        if c in IDENT:
            wl = self.line
            wstart = self.i
            w = []
            while self.i < self.n and self.peek() in IDENT:
                w.append(self.advance())
            # keyword-граница: 'case' в 'hex-case' / '$case' — НЕ keyword bash
            # (граница слова слева обязательна; x466-канон против фантом-кейсов)
            sep_ok = wstart == 0 or self.text[wstart - 1] in " \t\n;&|()"
            self.handle_word("".join(w) if sep_ok else "", wl)
            return

        self.advance()
        return

    def prev_sep(self):
        j = self.i - 1
        if j < 0:
            return True
        return self.text[j] in " \t\n;&|("

    def run(self):
        # word_start_ok упрощён; для '#' используем prev_sep
        while self.i < self.n:
            self.step()
        # незакрытые case — bash -n поймает; прикладываем WARN
        if self.case_stack:
            self.warns.append("%s: WARN незакрытый 'case' (bash -n поймает отдельно)" % self.path)


def scan_file(path):
    with open(path, "r", errors="replace") as f:
        text = f.read()
    s = Scanner(path, text)
    s.run()
    return s.fails, s.warns


def main(argv):
    if not argv:
        print("usage: case_arm_scan.py FILE...", file=sys.stderr)
        return 2
    all_fails, all_warns, scanned = [], [], 0
    for p in argv:
        try:
            fails, warns = scan_file(p)
        except OSError as e:
            all_fails.append("%s: FAIL unreadable: %s" % (p, e))
            continue
        scanned += 1
        all_fails.extend(fails)
        all_warns.extend(warns)
    for w in all_warns:
        print(w)
    for f in all_fails:
        print(f)
    print("case_arm_scan: %d file(s), %d FAIL, %d WARN" % (scanned, len(all_fails), len(all_warns)))
    return 1 if all_fails else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
