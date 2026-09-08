#!/usr/bin/env python3
"""Minimal RCON client (vanilla protocol) — S7-14 g9 JFR probe load driver.

Usage: rcon.py <host> <port> <password> <command> [command...]
Login type 3, command type 2; prints each response payload.
"""
import socket
import struct
import sys


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


def main() -> None:
    host, port, pw = sys.argv[1], int(sys.argv[2]), sys.argv[3]
    cmds = sys.argv[4:]
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
