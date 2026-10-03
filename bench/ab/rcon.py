#!/usr/bin/env python3
"""Minimal RCON client (stdlib only) — command channel for the paper A/B bench.

The sandbox harness runs the whole server lifecycle inside ONE harness
invocation, but the server console itself is driven over RCON (stdin-fifo
relays are fragile under detached processes). Both arms use the identical
server.properties (RCON on localhost), so the measurement channel is
symmetric and touches no gameplay surface.

Usage: rcon.py <port> <password> <command> [<command> ...]
"""
import socket
import struct
import sys


def pkt(rid: int, ptype: int, payload: str) -> bytes:
    body = struct.pack('<ii', rid, ptype) + payload.encode('utf-8') + b'\x00\x00'
    return struct.pack('<i', len(body)) + body


def recv_pkt(s: socket.socket):
    raw = b''
    while len(raw) < 4:
        c = s.recv(4 - len(raw))
        if not c:
            raise ConnectionError('rcon connection closed')
        raw += c
    (ln,) = struct.unpack('<i', raw)
    data = b''
    while len(data) < ln:
        c = s.recv(ln - len(data))
        if not c:
            raise ConnectionError('rcon connection closed mid-packet')
        data += c
    rid, ptype = struct.unpack('<ii', data[:8])
    return rid, ptype, data[8:-2].decode('utf-8', 'replace')


def main() -> None:
    host, port, pw = '127.0.0.1', int(sys.argv[1]), sys.argv[2]
    s = socket.create_connection((host, port), timeout=10)
    s.sendall(pkt(1, 3, pw))
    rid, _, _ = recv_pkt(s)
    if rid == -1:
        print('RCON AUTH FAIL', file=sys.stderr)
        sys.exit(2)
    for cmd in sys.argv[3:]:
        s.sendall(pkt(2, 2, cmd))
        try:
            rid, _, body = recv_pkt(s)
            print(body)
        except (ConnectionError, socket.timeout):
            # 'stop' tears the connection down without an ack — that is fine.
            print(f'(no ack for {cmd!r} — connection closed)')
            break
    s.close()


if __name__ == '__main__':
    main()
