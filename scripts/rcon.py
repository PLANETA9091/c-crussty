#!/usr/bin/env python3
"""Minimal RCON client (vanilla protocol) — S7-14 g9 JFR probe load driver.

TASK-75 (S7-21) hardening: the password is NO LONGER a positional argument
(ps-exposure + the S7-14 one-time secret ended up in dev-logs git history and
was rotated; see docs/RCON_HYGIENE_DECISION.md).

Password resolution (first match wins, fail-closed — no insecure fallback):
  1. --password-file PATH   (explicit, wins over everything)
  2. env CRUSSTY_RCON_PASSWORD
  3. default file /home/z/.rcon_password  (chmod 600, outside any repo)

Usage: rcon.py [--password-file PATH] <host> <port> <command> [command...]
Login type 3, command type 2; prints each response payload.

Exit codes: 0 ok; 2 auth failed; 3 no password source found; 4 usage error.
"""
import os
import socket
import struct
import sys

DEFAULT_PASSWORD_FILE = "/home/z/.rcon_password"


def pkt(req_id: int, ptype: int, payload: bytes) -> bytes:
    body = struct.pack("<ii", req_id, ptype) + payload + b"\x00\x00"
    return struct.pack("<i", len(body)) + body


def read_pkt(sock: socket.socket) -> tuple[int, int, bytes]:
    raw = b""
    while len(raw) < 4:
        chunk = sock.recv(4 - len(raw))
        if not chunk:
            raise EOFError("rcon closed")
        raw += chunk
    (length,) = struct.unpack("<i", raw)
    data = b""
    while len(data) < length:
        chunk = sock.recv(length - len(data))
        if not chunk:
            raise EOFError("rcon closed mid-packet")
        data += chunk
    rid, ptype = struct.unpack("<ii", data[:8])
    return rid, ptype, data[8:-2]


def resolve_password(argv: list[str]) -> tuple[str, list[str]]:
    """Pull --password-file out of argv; resolve secret fail-closed."""
    pw_file = None
    rest: list[str] = []
    i = 0
    while i < len(argv):
        a = argv[i]
        if a == "--password-file":
            if i + 1 >= len(argv):
                print("rcon.py: --password-file requires a path", file=sys.stderr)
                sys.exit(4)
            pw_file = argv[i + 1]
            i += 2
            continue
        if a.startswith("--password-file="):
            pw_file = a.split("=", 1)[1]
            i += 1
            continue
        rest.append(a)
        i += 1

    env = os.environ.get("CRUSSTY_RCON_PASSWORD")
    if env is not None and env != "":
        return env, rest

    path = pw_file if pw_file is not None else DEFAULT_PASSWORD_FILE
    try:
        with open(path, "rb") as f:
            pw = f.read().decode("utf-8").strip()
    except OSError as e:
        print(
            f"rcon.py: no password source: --password-file/env unset and default "
            f"file unreadable: {path}: {e}; see docs/RCON_HYGIENE_DECISION.md",
            file=sys.stderr,
        )
        sys.exit(3)
    if not pw:
        print(f"rcon.py: password file {path} is empty", file=sys.stderr)
        sys.exit(3)
    return pw, rest


def main() -> None:
    args = sys.argv[1:]
    if any(a in ("-h", "--help") for a in args):
        print(__doc__)
        sys.exit(0)
    pw, rest = resolve_password(args)
    if len(rest) < 3:
        print(__doc__)
        print(
            "rcon.py: expected <host> <port> <command> [command...] "
            "(password is NEVER positional)",
            file=sys.stderr,
        )
        sys.exit(4)
    host, port = rest[0], int(rest[1])
    cmds = rest[2:]
    with socket.create_connection((host, port), timeout=10) as s:
        s.sendall(pkt(1, 3, pw.encode()))
        rid, _, body = read_pkt(s)
        if rid == -1:
            print("RCON AUTH FAILED", file=sys.stderr)
            sys.exit(2)
        for i, c in enumerate(cmds):
            s.sendall(pkt(10 + i, 2, c.encode()))
            rid, _, body = read_pkt(s)
            print(f"> {c}\n{body.decode('utf-8', 'replace')}")


if __name__ == "__main__":
    main()
