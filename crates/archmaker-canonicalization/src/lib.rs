use serde_json::Value;
use sha2::{Digest, Sha256};

const DENYLIST: [&str; 11] = [
    "createdAt",
    "updatedAt",
    "loadedAt",
    "lastAccessedAt",
    "uiState",
    "localPath",
    "sourcePath",
    "absolutePath",
    "workingDirectory",
    "diagnostics",
    "diagnosticLog",
];

pub fn canonicalize(value: &Value) -> Vec<u8> {
    let mut out = Vec::new();
    write_value(&mut out, value);
    out
}

pub fn digest(canonical: &[u8], domain_tag: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(domain_tag.as_bytes());
    hasher.update([0u8]);
    hasher.update(canonical);
    hex::encode(hasher.finalize())
}

fn write_value(out: &mut Vec<u8>, value: &Value) {
    match value {
        Value::Null => out.extend_from_slice(b"null"),
        Value::Bool(true) => out.extend_from_slice(b"true"),
        Value::Bool(false) => out.extend_from_slice(b"false"),
        Value::Number(n) => out.extend_from_slice(write_number(n).as_bytes()),
        Value::String(s) => write_string(out, s),
        Value::Array(items) => {
            out.push(b'[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                write_value(out, item);
            }
            out.push(b']');
        }
        Value::Object(map) => {
            let mut entries: Vec<(&String, &Value)> = map
                .iter()
                .filter(|(k, _)| !DENYLIST.contains(&k.as_str()))
                .collect();
            entries.sort_by(|a, b| a.0.encode_utf16().cmp(b.0.encode_utf16()));
            out.push(b'{');
            for (i, (k, v)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                write_string(out, k);
                out.push(b':');
                write_value(out, v);
            }
            out.push(b'}');
        }
    }
}

fn write_string(out: &mut Vec<u8>, s: &str) {
    out.push(b'"');
    for c in s.chars() {
        match c {
            '"' => out.extend_from_slice(b"\\\""),
            '\\' => out.extend_from_slice(b"\\\\"),
            '\u{08}' => out.extend_from_slice(b"\\b"),
            '\u{09}' => out.extend_from_slice(b"\\t"),
            '\u{0A}' => out.extend_from_slice(b"\\n"),
            '\u{0C}' => out.extend_from_slice(b"\\f"),
            '\u{0D}' => out.extend_from_slice(b"\\r"),
            c if (c as u32) < 0x20 => {
                out.extend_from_slice(format!("\\u{:04x}", c as u32).as_bytes())
            }
            c => {
                let mut buf = [0u8; 4];
                out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            }
        }
    }
    out.push(b'"');
}

fn write_number(n: &serde_json::Number) -> String {
    if let Some(i) = n.as_i64() {
        return format_binary64(i as f64);
    }
    if let Some(u) = n.as_u64() {
        return format_binary64(u as f64);
    }
    format_binary64(n.as_f64().expect("serde_json number is finite binary64"))
}

fn format_binary64(v: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    if !v.is_finite() {
        panic!("non-finite number is not canonicalizable");
    }
    let negative = v < 0.0;
    let magnitude = if negative { -v } else { v };
    let (digits, exp) = shortest_decimal(magnitude);
    let k = digits.len() as i64;
    let n = exp as i64 + 1;
    let body = if k <= n && n <= 21 {
        format!("{}{}", digits, "0".repeat((n - k) as usize))
    } else if 0 < n && n <= 21 {
        let n = n as usize;
        format!("{}.{}", &digits[..n], &digits[n..])
    } else if -6 < n && n <= 0 {
        format!("0.{}{}", "0".repeat((-n) as usize), digits)
    } else {
        let e = n - 1;
        let mantissa = if digits.len() == 1 {
            digits.clone()
        } else {
            format!("{}.{}", &digits[..1], &digits[1..])
        };
        format!("{}e{}{}", mantissa, if e >= 0 { "+" } else { "-" }, e.abs())
    };
    if negative {
        format!("-{body}")
    } else {
        body
    }
}

fn shortest_decimal(magnitude: f64) -> (String, i32) {
    for q in 1..=17usize {
        let s = format!("{magnitude:.p$e}", p = q - 1);
        if s.parse::<f64>().is_ok_and(|r| r == magnitude) {
            let epos = s.find('e').expect("scientific notation has exponent");
            let exp: i32 = s[epos + 1..].parse().expect("decimal exponent is i32");
            let raw: String = s[..epos].chars().filter(|&c| c != '.').collect();
            let trimmed = raw.trim_end_matches('0');
            let digits = if trimmed.is_empty() { "0" } else { trimmed };
            return (digits.to_string(), exp);
        }
    }
    unreachable!("17 significant digits always round-trip binary64");
}
