//! Minimal JSON parser (NCF Phase 1, P1.1/P1.2).
//!
//! Why hand-rolled: the crate policy is zero external dependencies (std only)
//! so the factory stays buildable in isolation and never drags a dependency
//! tree into the sealed .so boundary (NCF invariant I7). Worldgen JSON is
//! machine-generated and well-formed, but the parser is still strict and
//! fully tested: numbers, strings with escapes, booleans, null, arrays,
//! objects, and Unicode surrogate pairs.
//!
//! Number handling matters for bit-exactness: every JSON number is stored as
//! its ORIGINAL TEXT plus the parsed f64. The f64 conversion uses Rust's
//! `str::parse::<f64>()` which is correctly rounded (IEEE 754 nearest-even),
//! matching Java's `Double.parseDouble` used by DFU codecs. The original text
//! is kept for the world-spec hash (P1.7) so the hash sees exactly what the
//! datapack author wrote.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    /// Parsed double + the original literal text (for the spec hash).
    Num(f64, String),
    Str(String),
    Arr(Vec<Json>),
    /// BTreeMap keeps key order deterministic for hashing/iteration.
    Obj(BTreeMap<String, Json>),
}

impl Json {
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(m) => m.get(key),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Json::Num(v, _) => Some(*v),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Json::Num(v, _) => Some(*v as i64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_arr(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(a) => Some(a),
            _ => None,
        }
    }

    pub fn is_obj(&self) -> bool {
        matches!(self, Json::Obj(_))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub pos: usize,
    pub msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "json parse error at byte {}: {}", self.pos, self.msg)
    }
}

pub fn parse(input: &str) -> Result<Json, ParseError> {
    let bytes = input.as_bytes();
    let mut p = Parser { b: bytes, i: 0 };
    p.skip_ws();
    let v = p.value()?;
    p.skip_ws();
    if p.i != bytes.len() {
        return Err(p.err("trailing characters after JSON value"));
    }
    Ok(v)
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn err(&self, msg: &str) -> ParseError {
        ParseError { pos: self.i, msg: msg.to_string() }
    }

    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                b' ' | b'\t' | b'\n' | b'\r' => self.i += 1,
                _ => break,
            }
        }
    }

    fn expect(&mut self, c: u8) -> Result<(), ParseError> {
        if self.peek() == Some(c) {
            self.i += 1;
            Ok(())
        } else {
            Err(self.err(&format!("expected '{}'", c as char)))
        }
    }

    fn value(&mut self) -> Result<Json, ParseError> {
        match self.peek() {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b't') => self.lit("true", Json::Bool(true)),
            Some(b'f') => self.lit("false", Json::Bool(false)),
            Some(b'n') => self.lit("null", Json::Null),
            Some(c) if c == b'-' || c.is_ascii_digit() => self.number(),
            _ => Err(self.err("expected a JSON value")),
        }
    }

    fn lit(&mut self, word: &str, v: Json) -> Result<Json, ParseError> {
        if self.b[self.i..].starts_with(word.as_bytes()) {
            self.i += word.len();
            Ok(v)
        } else {
            Err(self.err(&format!("expected '{}'", word)))
        }
    }

    fn object(&mut self) -> Result<Json, ParseError> {
        self.expect(b'{')?;
        let mut map = BTreeMap::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.i += 1;
            return Ok(Json::Obj(map));
        }
        loop {
            self.skip_ws();
            let key = self.string()?;
            self.skip_ws();
            self.expect(b':')?;
            self.skip_ws();
            let val = self.value()?;
            map.insert(key, val);
            self.skip_ws();
            match self.peek() {
                Some(b',') => {
                    self.i += 1;
                }
                Some(b'}') => {
                    self.i += 1;
                    return Ok(Json::Obj(map));
                }
                _ => return Err(self.err("expected ',' or '}' in object")),
            }
        }
    }

    fn array(&mut self) -> Result<Json, ParseError> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.i += 1;
            return Ok(Json::Arr(items));
        }
        loop {
            self.skip_ws();
            items.push(self.value()?);
            self.skip_ws();
            match self.peek() {
                Some(b',') => {
                    self.i += 1;
                }
                Some(b']') => {
                    self.i += 1;
                    return Ok(Json::Arr(items));
                }
                _ => return Err(self.err("expected ',' or ']' in array")),
            }
        }
    }

    fn string(&mut self) -> Result<String, ParseError> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            let c = self.peek().ok_or_else(|| self.err("unterminated string"))?;
            self.i += 1;
            match c {
                b'"' => return Ok(out),
                b'\\' => {
                    let e = self.peek().ok_or_else(|| self.err("bad escape"))?;
                    self.i += 1;
                    match e {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{0008}'),
                        b'f' => out.push('\u{000C}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let hi = self.hex4()?;
                            let ch = if (0xD800..0xDC00).contains(&hi) {
                                // surrogate pair
                                if self.peek() == Some(b'\\') {
                                    self.i += 1;
                                    self.expect(b'u')?;
                                    let lo = self.hex4()?;
                                    if !(0xDC00..0xE000).contains(&lo) {
                                        return Err(self.err("invalid low surrogate"));
                                    }
                                    let cp = 0x10000
                                        + (((hi as u32) - 0xD800) << 10)
                                        + ((lo as u32) - 0xDC00);
                                    char::from_u32(cp)
                                        .ok_or_else(|| self.err("invalid surrogate pair"))?
                                } else {
                                    return Err(self.err("lone high surrogate"));
                                }
                            } else {
                                char::from_u32(hi as u32)
                                    .ok_or_else(|| self.err("invalid unicode escape"))?
                            };
                            out.push(ch);
                        }
                        _ => return Err(self.err("unknown escape")),
                    }
                }
                _ => {
                    // Re-assemble UTF-8 multibyte sequences byte-wise.
                    if c < 0x80 {
                        out.push(c as char);
                    } else {
                        let start = self.i - 1;
                        let len = utf8_len(c);
                        let end = start + len;
                        if end > self.b.len() {
                            return Err(self.err("truncated UTF-8"));
                        }
                        let s = std::str::from_utf8(&self.b[start..end])
                            .map_err(|_| self.err("invalid UTF-8"))?;
                        out.push_str(s);
                        self.i = end;
                    }
                }
            }
        }
    }

    fn hex4(&mut self) -> Result<u16, ParseError> {
        if self.i + 4 > self.b.len() {
            return Err(self.err("short \\u escape"));
        }
        let s = std::str::from_utf8(&self.b[self.i..self.i + 4])
            .map_err(|_| self.err("bad \\u escape"))?;
        let v = u16::from_str_radix(s, 16).map_err(|_| self.err("bad \\u escape"))?;
        self.i += 4;
        Ok(v)
    }

    fn number(&mut self) -> Result<Json, ParseError> {
        let start = self.i;
        if self.peek() == Some(b'-') {
            self.i += 1;
        }
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.i += 1;
        }
        if self.peek() == Some(b'.') {
            self.i += 1;
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.i += 1;
            }
        }
        if matches!(self.peek(), Some(b'e') | Some(b'E')) {
            self.i += 1;
            if matches!(self.peek(), Some(b'+') | Some(b'-')) {
                self.i += 1;
            }
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.i += 1;
            }
        }
        let text = std::str::from_utf8(&self.b[start..self.i]).unwrap();
        if text.is_empty() || text == "-" {
            return Err(self.err("empty number"));
        }
        let v: f64 = text
            .parse()
            .map_err(|_| self.err(&format!("bad number literal {text}")))?;
        Ok(Json::Num(v, text.to_string()))
    }
}

fn utf8_len(first: u8) -> usize {
    if first >= 0xF0 {
        4
    } else if first >= 0xE0 {
        3
    } else {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_worldgen_shapes() {
        let src = r#"{
          "type": "minecraft:shifted_noise",
          "noise": "minecraft:temperature",
          "shift_x": "minecraft:overworld/shift_x",
          "shift_y": 0,
          "xz_scale": 0.25,
          "y_scale": 0.0,
          "list": [-1.5, 2e3, -0.07999999821186066],
          "flag": true,
          "none": null
        }"#;
        let v = parse(src).unwrap();
        let obj = match &v {
            Json::Obj(m) => m,
            _ => panic!("expected object"),
        };
        assert_eq!(obj.len(), 9);
        assert_eq!(v.get("noise").unwrap().as_str().unwrap(), "minecraft:temperature");
        assert_eq!(v.get("shift_y").unwrap().as_f64().unwrap(), 0.0);
        assert_eq!(
            v.get("xz_scale").unwrap().as_f64().unwrap(),
            0.25
        );
        let arr = v.get("list").unwrap().as_arr().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[2].as_f64().unwrap(), -0.07999999821186066);
        // original text preserved (spec hash input)
        match &arr[2] {
            Json::Num(_, txt) => assert_eq!(txt, "-0.07999999821186066"),
            _ => panic!("expected number"),
        }
    }

    #[test]
    fn escapes_and_surrogates() {
        let v = parse(r#""aA\n\"\\\/è😀""#).unwrap();
        match v {
            Json::Str(s) => {
                assert_eq!(s, "aA\n\"\\/è😀");
            }
            _ => panic!("expected string"),
        }
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse("{").is_err());
        assert!(parse("[1,]").is_err());
        assert!(parse("{\"a\":1,}").is_err());
        assert!(parse("1 2").is_err());
        assert!(parse("nul").is_err());
        assert!(parse("-").is_err());
    }

    #[test]
    fn exponent_and_big_numbers() {
        let v = parse("[1E3, -7e-2, 1.7976931348623157E308]").unwrap();
        let a = v.as_arr().unwrap();
        assert_eq!(a[0].as_f64().unwrap(), 1000.0);
        assert_eq!(a[1].as_f64().unwrap(), -0.07);
        assert_eq!(a[2].as_f64().unwrap(), f64::MAX);
    }
}
