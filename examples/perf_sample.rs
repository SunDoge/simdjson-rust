//! Throwaway binary for perf sampling of the Large struct deserialization path.
//! Run under `perf record` to locate CPU hotspots.
use serde::{Deserialize, Serialize};
use simdjson_rust::{dom::Parser, serde::from_tape};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct UserProfile {
    id: u64,
    name: String,
    email: String,
    active: bool,
    friends: Vec<u64>,
    score: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct LargeData {
    users: Vec<UserProfile>,
    version: String,
    timestamp: u64,
}

fn make_large_data() -> String {
    let users = (0..500)
        .map(|id| UserProfile {
            id,
            name: format!("User Name {}", id),
            email: format!("user.{}@example.com", id),
            active: id % 2 == 0,
            friends: (0..(id % 20)).map(|i| 10000 + i).collect(),
            score: (id as f64) * 1.5,
        })
        .collect();
    let data = LargeData {
        users,
        version: "v1.0.0-benchmark".to_string(),
        timestamp: 1719999999,
    };
    serde_json::to_string(&data).unwrap()
}

fn main() {
    let json = make_large_data();
    let mut parser = Parser::default();
    // Warm up the parser's internal buffers.
    for _ in 0..100 {
        let tape = parser.parse_str(&json).unwrap();
        let _res: LargeData = from_tape(tape).unwrap();
    }
    // Hot loop: this is what perf samples.
    let iters = std::env::args()
        .nth(1)
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap_or(20000);
    let mut acc = 0u64;
    for _ in 0..iters {
        let tape = parser.parse_str(&json).unwrap();
        let res: LargeData = from_tape(tape).unwrap();
        acc = acc.wrapping_add(res.users.len() as u64);
    }
    // Prevent dead-code elimination.
    std::hint::black_box(acc);
}
