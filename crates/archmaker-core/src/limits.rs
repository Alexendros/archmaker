use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Limits {
    pub max_bytes: u64,
    pub max_depth: u32,
    pub max_string_length: usize,
    pub max_array_items: usize,
}

pub const DEFAULT_LIMITS: Limits = Limits {
    max_bytes: 16_777_216,
    max_depth: 128,
    max_string_length: 65_536,
    max_array_items: 4_096,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IoClass {
    Pure,
    Mixed,
    Io,
}
