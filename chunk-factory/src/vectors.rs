//! Golden-vector CSV loading + Java hex-float parsing (NCF P0.3/P1 gate).
//!
//! Java's `Double.toHexString` / `Float.toHexString` are LOSSLESS: the exact
//! bit pattern survives a parse round-trip. The Java side of the vector
//! harness writes hex floats exactly so the Rust side can compare bits, not
//! decimal approximations.
//!
//! CSV conventions (fields never contain commas; '|' is the inner separator):
//!   random.csv : family,src,seed_lo,seed_hi,op,arg,index,value
//!   noise.csv  : impl,seed,params,x,y,z,v0,v1,v2,v3
//!   density.csv: field,x,y,z,value
//! '#' lines and blank lines are ignored. value encoding: signed decimal for
//! ints/longs, Java hex-float for float/double, "NaN"/"Infinity"/
//! "-Infinity" for specials, "true"/"false"/"0"/"1" for booleans.

/// Parse a Java hex-float string ("0x1.8p1", "-0x1.fffffffffffffp1023",
/// "0x0.0p0") to the EXACT f64 bit pattern. Also accepts plain decimal
/// literals and NaN/Infinity spellings.
pub fn parse_hex_f64(s: &str) -> Result<f64, String> {
    let t = s.trim();
    match t {
        "NaN" => return Ok(f64::NAN),
        "Infinity" => return Ok(f64::INFINITY),
        "-Infinity" => return Ok(f64::NEG_INFINITY),
        _ => {}
    }
    if !t.contains("0x") && !t.contains("0X") {
        // decimal fallback (single correctly-rounded parse)
        return t.parse::<f64>().map_err(|_| format!("bad double '{s}'"));
    }
    let (neg, rest) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t),
    };
    let rest = rest
        .strip_prefix("0x")
        .or_else(|| rest.strip_prefix("0X"))
        .ok_or_else(|| format!("bad hex float '{s}'"))?;
    // split at p/P
    let (mant, exp) = match rest.split_once(['p', 'P']) {
        Some((m, e)) => (m, e.parse::<i32>().map_err(|_| format!("bad exponent in '{s}'"))?),
        None => (rest, 0i32),
    };
    let (int_part, frac_part) = match mant.split_once('.') {
        Some((a, b)) => (a, b),
        None => (mant, ""),
    };
    let mut mantissa: u64 = 0;
    let mut bits_used = 0u32;
    let mut any_digit = false;
    for c in int_part.chars().chain(frac_part.chars()) {
        let d = c.to_digit(16).ok_or_else(|| format!("bad hex digit in '{s}'"))?;
        any_digit = true;
        if mantissa == 0 && d == 0 {
            continue; // skip leading zeros (frac part only)
        }
        if bits_used + 4 <= 64 {
            mantissa = (mantissa << 4) | d as u64;
            bits_used += 4;
        } else {
            // more than 64 bits — cannot happen for Java hexfloat of a double
            // (max 15 significant hex digits); record sticky and shift.
            mantissa = mantissa << 1 | (mantissa >> 63) & 1;
        }
    }
    if !any_digit {
        return Err(format!("empty mantissa in '{s}'"));
    }
    // binary exponent correction: consumed frac digits are 4 bits each
    let frac_digits = frac_part.len() as i32;
    let _ = bits_used;
    let e = exp - 4 * frac_digits;
    if mantissa == 0 {
        let z = 0.0f64;
        return Ok(if neg { -z } else { z });
    }
    let m = mantissa as f64; // exact (<= 53 significant bits for doubles)
    let scaled = scale_pow2(m, e);
    Ok(if neg { -scaled } else { scaled })
}

/// Parse a Java Float.toHexString string to the exact f32 bits.
pub fn parse_hex_f32(s: &str) -> Result<f32, String> {
    let t = s.trim();
    match t {
        "NaN" => return Ok(f32::NAN),
        "Infinity" => return Ok(f32::INFINITY),
        "-Infinity" => return Ok(f32::NEG_INFINITY),
        _ => {}
    }
    if !t.contains("0x") && !t.contains("0X") {
        return t.parse::<f32>().map_err(|_| format!("bad float '{s}'"));
    }
    let (neg, rest) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t),
    };
    let rest = rest
        .strip_prefix("0x")
        .or_else(|| rest.strip_prefix("0X"))
        .ok_or_else(|| format!("bad hex float '{s}'"))?;
    let (mant, exp) = match rest.split_once(['p', 'P']) {
        Some((m, e)) => (m, e.parse::<i32>().map_err(|_| format!("bad exponent in '{s}'"))?),
        None => (rest, 0i32),
    };
    let (int_part, frac_part) = match mant.split_once('.') {
        Some((a, b)) => (a, b),
        None => (mant, ""),
    };
    let mut mantissa: u64 = 0;
    for c in int_part.chars().chain(frac_part.chars()) {
        let d = c.to_digit(16).ok_or_else(|| format!("bad hex digit in '{s}'"))?;
        if mantissa == 0 && d == 0 {
            continue;
        }
        mantissa = (mantissa << 4) | d as u64;
    }
    let e = exp - 4 * frac_part.len() as i32;
    if mantissa == 0 {
        let z = 0.0f32;
        return Ok(if neg { -z } else { z });
    }
    let m = mantissa as f64;
    let scaled = scale_pow2(m, e);
    // single rounding to f32 (Java parses FLOAT hexfloats directly, but the
    // vectors only ever carry values that came FROM an f32, so the mantissa
    // fits exactly and this rounding is exact)
    let f = scaled as f32;
    Ok(if neg { -f } else { f })
}

/// m * 2^e with exact power-of-two scaling (single rounding on under/
/// overflow only).
fn scale_pow2(m: f64, e: i32) -> f64 {
    // split e into exact f64-scaleable chunks
    let mut v = m;
    let mut remaining = e;
    while remaining > 1023 {
        v *= f64::from_bits((1023u64) << 52); // 2^1023
        remaining -= 1023;
        if v.is_infinite() {
            return v;
        }
    }
    while remaining < -1022 {
        v *= f64::from_bits((1u64) << 52); // 2^-1022 (min normal)
        remaining += 1022;
        if v == 0.0 {
            return v;
        }
    }
    if remaining != 0 {
        v *= f64::from_bits((((remaining + 1023) as u64) & 0x7FF) << 52);
    }
    v
}

/// One parsed CSV row as raw string columns.
pub struct Row {
    pub cols: Vec<String>,
}

impl Row {
    pub fn c(&self, i: usize) -> &str {
        self.cols.get(i).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn f64(&self, i: usize) -> Result<f64, String> {
        parse_hex_f64(self.c(i))
    }

    pub fn f32(&self, i: usize) -> Result<f32, String> {
        parse_hex_f32(self.c(i))
    }

    pub fn i64(&self, i: usize) -> Result<i64, String> {
        self.c(i).parse::<i64>().map_err(|_| format!("bad int '{}'", self.c(i)))
    }

    pub fn i32(&self, i: usize) -> Result<i32, String> {
        self.c(i).parse::<i32>().map_err(|_| format!("bad int '{}'", self.c(i)))
    }
}

/// Load a CSV file into rows (skipping '#'/blank lines).
pub fn load_csv(path: &Path) -> Result<Vec<Row>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut rows = Vec::new();
    for line in text.lines() {
        let t = line.trim_end_matches('\r');
        let tt = t.trim();
        if tt.is_empty() || tt.starts_with('#') {
            continue;
        }
        rows.push(Row { cols: t.split(',').map(|s| s.to_string()).collect() });
    }
    Ok(rows)
}

use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_f64_roundtrip_known_values() {
        assert_eq!(parse_hex_f64("0x0.0p0").unwrap().to_bits(), 0);
        assert_eq!(parse_hex_f64("0x1.0p0").unwrap(), 1.0);
        assert_eq!(parse_hex_f64("0x1.8p1").unwrap(), 3.0);
        assert_eq!(parse_hex_f64("-0x1.9p4").unwrap(), -25.0);
        assert_eq!(parse_hex_f64("0x1.0p-53").unwrap().to_bits(), (f64::MIN_POSITIVE * 0.25f64 * 0.0f64 + 1.1102230246251565e-16).to_bits());
        // Double.toHexString(2^-53) == 0x1.0p-53
        assert_eq!(parse_hex_f64("0x1.0p-53").unwrap(), 2f64.powi(-53));
        assert_eq!(parse_hex_f64("0x1.fffffffffffffp1023").unwrap(), f64::MAX);
        assert!(parse_hex_f64("NaN").unwrap().is_nan());
        assert_eq!(parse_hex_f64("Infinity").unwrap(), f64::INFINITY);
        assert_eq!(parse_hex_f64("-Infinity").unwrap(), f64::NEG_INFINITY);
        // decimal passthrough
        assert_eq!(parse_hex_f64("3.5").unwrap(), 3.5);
    }

    #[test]
    fn hex_f32_roundtrip() {
        assert_eq!(parse_hex_f32("0x1.0p0").unwrap(), 1.0f32);
        // Float.toHexString(2^-24) == 0x1.0p-24
        assert_eq!(parse_hex_f32("0x1.0p-24").unwrap(), 2f32.powi(-24));
        assert_eq!(parse_hex_f32("0x1.999998p-4").unwrap().to_bits(), (0.099999994f32).to_bits());
    }

    #[test]
    fn csv_rows() {
        let dir = std::env::temp_dir().join("ncf_vec_test");
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("t.csv");
        std::fs::write(&p, "# comment\na,b,0x1.8p1\n\nx,y,-0x1.0p2\n").unwrap();
        let rows = load_csv(&p).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].c(0), "a");
        assert_eq!(rows[0].f64(2).unwrap(), 3.0);
        assert_eq!(rows[1].f64(2).unwrap(), -4.0);
    }
}
