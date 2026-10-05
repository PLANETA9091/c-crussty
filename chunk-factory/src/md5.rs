//! MD5 (RFC 1321) — in-crate, zero-dep (NCF I7: std only).
//!
//! Needed for `RandomSupport.seedFromHashOf(String)`: Guava's
//! `Hashing.md5().hashString(s, UTF_8).asBytes()`, then the 16 bytes are
//! packed big-endian into two i64 halves:
//!
//! ```java
//! long lo = Longs.fromBytes(b0, b1, b2, b3, b4, b5, b6, b7);
//! long hi = Longs.fromBytes(b8, b9, b10, b11, b12, b13, b14, b15);
//! ```
//!
//! Verified against RFC 1321 test vectors and against JVM-captured
//! `fromHashOf` seeds (bench/golden vector gate).

pub const DIGEST_LEN: usize = 16;

pub fn md5(msg: &[u8]) -> [u8; DIGEST_LEN] {
    // Per-round shift amounts and constants (standard MD5).
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, //
        5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, //
        4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, //
        6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    // K[i] = floor(2^32 * abs(sin(i+1))) — precomputed constants.
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, //
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501, //
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, //
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821, //
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, //
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8, //
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, //
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, //
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, //
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, //
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, //
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, //
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, //
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1, //
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, //
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;

    // Padding: msg || 0x80 || zeros || bitlen(u64 LE)
    let bitlen = (msg.len() as u64).wrapping_mul(8);
    let mut data = msg.to_vec();
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bitlen.to_le_bytes());

    for chunk in data.as_chunks::<64>().0 {
        let mut m = [0u32; 16];
        for (i, w) in m.iter_mut().enumerate() {
            *w = u32::from_le_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
        for i in 0..64 {
            let (f, g) = match i / 16 {
                0 => ((b & c) | (!b & d), i),
                1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                2 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let f2 = a
                .wrapping_add(f)
                .wrapping_add(K[i])
                .wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f2.rotate_left(S[i]));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let mut out = [0u8; DIGEST_LEN];
    out[0..4].copy_from_slice(&a0.to_le_bytes());
    out[4..8].copy_from_slice(&b0.to_le_bytes());
    out[8..12].copy_from_slice(&c0.to_le_bytes());
    out[12..16].copy_from_slice(&d0.to_le_bytes());
    out
}

/// RandomSupport.seedFromHashOf: MD5 of the UTF-8 string, big-endian packing
/// per Guava Longs.fromBytes(b0..b7).
pub fn seed_from_hash_of(name: &str) -> (i64, i64) {
    let digest = md5(name.as_bytes());
    let lo = i64::from_be_bytes(digest[0..8].try_into().unwrap());
    let hi = i64::from_be_bytes(digest[8..16].try_into().unwrap());
    (lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(d: &[u8]) -> String {
        d.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[test]
    fn rfc1321_vectors() {
        assert_eq!(hex(&md5(b"")), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(hex(&md5(b"a")), "0cc175b9c0f1b6a831c399e269772661");
        assert_eq!(hex(&md5(b"abc")), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(
            hex(&md5(b"message digest")),
            "f96b697d7cb7938d525a2f31aaf161d0"
        );
        assert_eq!(
            hex(&md5(b"abcdefghijklmnopqrstuvwxyz")),
            "c3fcd3d76192e4007dfb496cca67e13b"
        );
    }

    #[test]
    fn pack_is_big_endian_per_longs_from_bytes() {
        // digest[0] is the MOST significant byte of `lo`.
        let (lo, hi) = seed_from_hash_of("minecraft:terrain");
        let d = md5(b"minecraft:terrain");
        assert_eq!(lo as u64, u64::from_be_bytes(d[0..8].try_into().unwrap()));
        assert_eq!(hi as u64, u64::from_be_bytes(d[8..16].try_into().unwrap()));
    }
}
