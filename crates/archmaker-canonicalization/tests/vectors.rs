use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

const DEFAULT_DOMAIN: &str = "archmaker:manifest:v1";
const EXPECTED_VECTORS: usize = 34;

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../contracts/test-vectors/canonicalization")
}

fn check_vector(v: &Value) {
    let id = v["id"].as_str().expect("vector id");
    let domain = v
        .get("domain")
        .and_then(|d| d.as_str())
        .unwrap_or(DEFAULT_DOMAIN);
    if v.get("expect").and_then(|e| e.as_str()) == Some("reject") {
        let raw = b64decode(v["inputBytesB64"].as_str().expect("inputBytesB64"))
            .unwrap_or_else(|e| panic!("{id}: bad base64: {e}"));
        assert!(
            strict_parse(&raw).is_err(),
            "{id}: expected reject, input was accepted"
        );
        return;
    }
    let mut input = v["input"].clone();
    if let Some(specs) = v.get("sortArrays") {
        apply_sort_arrays(&mut input, specs);
    }
    let canonical = archmaker_canonicalization::canonicalize(&input);
    assert_eq!(
        canonical,
        v["canonical"].as_str().expect("canonical").as_bytes(),
        "{id}: canonical bytes"
    );
    assert_eq!(
        archmaker_canonicalization::digest(&canonical, domain),
        v["digest"].as_str().expect("digest"),
        "{id}: digest"
    );
}

fn apply_sort_arrays(root: &mut Value, specs: &Value) {
    for spec in specs.as_array().expect("sortArrays array") {
        let pointer = spec["pointer"].as_str().expect("pointer");
        let by = spec["by"].as_str().expect("by");
        let arr = pointer_mut(root, pointer)
            .expect("pointer hits value")
            .as_array_mut()
            .expect("pointer hits array");
        arr.sort_by(|a, b| {
            let ka = a.get(by).and_then(|x| x.as_str()).expect("sort key");
            let kb = b.get(by).and_then(|x| x.as_str()).expect("sort key");
            ka.encode_utf16().cmp(kb.encode_utf16())
        });
    }
}

fn pointer_mut<'a>(root: &'a mut Value, pointer: &str) -> Option<&'a mut Value> {
    let mut cur = root;
    for part in pointer.split('/') {
        if part.is_empty() {
            continue;
        }
        let key = part.replace("~1", "/").replace("~0", "~");
        cur = cur.get_mut(&key)?;
    }
    Some(cur)
}

fn strict_parse(raw: &[u8]) -> Result<Value, String> {
    if raw.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Err("leading-bom".to_string());
    }
    let text = std::str::from_utf8(raw).map_err(|e| e.to_string())?;
    scan_no_duplicates(text)?;
    serde_json::from_str(text).map_err(|e| e.to_string())
}

struct Cursor<'a> {
    b: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn ws(&mut self) {
        while self.pos < self.b.len() && matches!(self.b[self.pos], b' ' | b'\t' | b'\n' | b'\r') {
            self.pos += 1;
        }
    }
    fn peek(&self) -> Option<u8> {
        self.b.get(self.pos).copied()
    }
    fn eat(&mut self, c: u8) -> bool {
        if self.peek() == Some(c) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn value(&mut self, stack: &mut Vec<HashSet<String>>) -> Result<(), String> {
        self.ws();
        match self.peek() {
            Some(b'{') => self.object(stack),
            Some(b'[') => self.array(stack),
            Some(b'"') => {
                self.string()?;
                Ok(())
            }
            Some(b't') => self.word("true"),
            Some(b'f') => self.word("false"),
            Some(b'n') => self.word("null"),
            Some(_) => self.scalar(),
            None => Err("unexpected-end".to_string()),
        }
    }
    fn object(&mut self, stack: &mut Vec<HashSet<String>>) -> Result<(), String> {
        self.pos += 1;
        stack.push(HashSet::new());
        self.ws();
        if self.eat(b'}') {
            stack.pop();
            return Ok(());
        }
        loop {
            self.ws();
            if self.peek() != Some(b'"') {
                return Err("expected-key".to_string());
            }
            let key = self.string()?;
            self.ws();
            if !self.eat(b':') {
                return Err("expected-colon".to_string());
            }
            if !stack.last_mut().expect("object frame").insert(key) {
                return Err("duplicate-key".to_string());
            }
            self.value(stack)?;
            self.ws();
            if self.eat(b',') {
                continue;
            }
            if self.eat(b'}') {
                stack.pop();
                return Ok(());
            }
            return Err("expected-comma-or-end".to_string());
        }
    }
    fn array(&mut self, stack: &mut Vec<HashSet<String>>) -> Result<(), String> {
        self.pos += 1;
        self.ws();
        if self.eat(b']') {
            return Ok(());
        }
        loop {
            self.value(stack)?;
            self.ws();
            if self.eat(b',') {
                continue;
            }
            if self.eat(b']') {
                return Ok(());
            }
            return Err("expected-comma-or-end".to_string());
        }
    }
    fn string(&mut self) -> Result<String, String> {
        let start = self.pos;
        self.pos += 1;
        while let Some(c) = self.peek() {
            match c {
                b'"' => {
                    self.pos += 1;
                    let raw =
                        std::str::from_utf8(&self.b[start..self.pos]).map_err(|e| e.to_string())?;
                    return serde_json::from_str(raw).map_err(|e| e.to_string());
                }
                b'\\' => {
                    self.pos = self.pos.saturating_add(2).min(self.b.len());
                }
                _ => {
                    self.pos += 1;
                }
            }
        }
        Err("unterminated-string".to_string())
    }
    fn word(&mut self, word: &str) -> Result<(), String> {
        if self.b[self.pos..].starts_with(word.as_bytes()) {
            self.pos += word.len();
            Ok(())
        } else {
            Err("unexpected-literal".to_string())
        }
    }
    fn scalar(&mut self) -> Result<(), String> {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || matches!(c, b'+' | b'-' | b'.') {
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.pos == start {
            return Err("unexpected-char".to_string());
        }
        Ok(())
    }
}

fn scan_no_duplicates(text: &str) -> Result<(), String> {
    let mut cur = Cursor {
        b: text.as_bytes(),
        pos: 0,
    };
    let mut stack = Vec::new();
    cur.value(&mut stack)?;
    cur.ws();
    if cur.pos != cur.b.len() {
        return Err("trailing-data".to_string());
    }
    Ok(())
}

fn b64val(c: u8) -> Option<u8> {
    match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn b64decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(4) {
        return Err("bad-b64-len".to_string());
    }
    let pad = s.bytes().rev().take_while(|&c| c == b'=').count();
    if pad > 2 {
        return Err("bad-b64-pad".to_string());
    }
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(s.len() / 4 * 3);
    let mut i = 0;
    while i < bytes.len() {
        let mut q = [0u8; 4];
        for (j, qj) in q.iter_mut().enumerate() {
            let c = bytes[i + j];
            *qj = if c == b'=' {
                0
            } else {
                b64val(c).ok_or("bad-b64")?
            };
        }
        let n = ((q[0] as u32) << 18) | ((q[1] as u32) << 12) | ((q[2] as u32) << 6) | q[3] as u32;
        out.push((n >> 16) as u8);
        out.push((n >> 8) as u8);
        out.push(n as u8);
        i += 4;
    }
    out.truncate(out.len() - pad);
    Ok(out)
}

#[test]
fn golden_corpus_byte_identity_and_digest() {
    let dir = corpus_dir();
    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(dir.join("manifest.json")).expect("manifest"))
            .expect("manifest json");
    let mut count = 0;
    for entry in manifest["files"].as_array().expect("files") {
        let file = entry["file"].as_str().expect("file");
        let data: Value =
            serde_json::from_str(&fs::read_to_string(dir.join(file)).expect("vectors file"))
                .expect("vectors json");
        for v in data["vectors"].as_array().expect("vectors") {
            count += 1;
            check_vector(v);
        }
    }
    assert_eq!(count, EXPECTED_VECTORS, "corpus vector count");
}

#[test]
fn m_key_order_byte_identity() {
    let value = serde_json::json!({"b": 1, "a": 2, "c": 3, "za": 0});
    let canonical = archmaker_canonicalization::canonicalize(&value);
    assert_eq!(canonical, b"{\"a\":2,\"b\":1,\"c\":3,\"za\":0}");
    assert_eq!(
        archmaker_canonicalization::digest(&canonical, DEFAULT_DOMAIN),
        "61b081617b80016523dbd29447d9ba50736076c81c34283f0c2b3ee4bdbf4fa2"
    );
}

#[test]
fn n_negative_zero_byte_identity() {
    let value = serde_json::json!({"a": -0.0, "b": 0.0});
    let canonical = archmaker_canonicalization::canonicalize(&value);
    assert_eq!(canonical, b"{\"a\":0,\"b\":0}");
    assert_eq!(
        archmaker_canonicalization::digest(&canonical, DEFAULT_DOMAIN),
        "2772fdf7f7eef0231679bea18ffa41fa76cca4faf932b47a7e217c4ae8bf5a5d"
    );
}

#[test]
fn t_ephemeral_byte_identity() {
    let value = serde_json::json!({
        "id": "x",
        "createdAt": "2026-01-01T00:00:00Z",
        "loadedAt": "2026-01-01T00:00:01Z",
        "localPath": "/tmp/a"
    });
    let canonical = archmaker_canonicalization::canonicalize(&value);
    assert_eq!(canonical, b"{\"id\":\"x\"}");
    assert_eq!(
        archmaker_canonicalization::digest(&canonical, DEFAULT_DOMAIN),
        "b7596a5c13d95bd3661c91e8946326dd7ba718f57482c99afadc926e6658a37a"
    );
}
