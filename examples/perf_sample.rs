//! CPU sampling and stage timing for JSON-to-owned-struct deserialization.
//! Usage: perf_sample [iterations=20000] [mode=reuse] [users=500]
//! Modes: reuse, fresh, padded, parse, ffi, view, serde, serde-json.
//! Each mode warms up first; timings include destruction of the result.
//! Stage timings are diagnostic and are not additive.
use std::{hint::black_box, time::Instant};

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

fn make_large_data(count: u64) -> String {
    let users = (0..count)
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

fn measure<T>(iters: usize, mut f: impl FnMut() -> T) -> f64 {
    for _ in 0..100 {
        black_box(f());
    }
    let started = Instant::now();
    for _ in 0..iters {
        black_box(f());
    }
    started.elapsed().as_secs_f64() * 1e9 / iters as f64
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let iters = args
        .get(1)
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap_or(20000);
    assert!(iters > 0);
    let mode = args.get(2).map(String::as_str).unwrap_or("reuse");
    let users = args.get(3).map(|s| s.parse().unwrap()).unwrap_or(500);
    let mut json = make_large_data(users);
    let mut parser = Parser::default();
    // Check data equivalence outside the timed region.
    let reference: LargeData = serde_json::from_str(&json).unwrap();
    let parsed: LargeData = from_tape(parser.parse_str(&json).unwrap()).unwrap();
    assert_eq!(parsed, reference);
    let ns = match mode {
        "reuse" => measure(iters, || {
            let tape = parser.parse_str(black_box(&json)).unwrap();
            from_tape::<LargeData>(tape).unwrap()
        }),
        "fresh" => measure(iters, || {
            simdjson_rust::serde::from_str::<LargeData>(black_box(&json)).unwrap()
        }),
        "padded" => {
            json.reserve(simdjson_sys::SIMDJSON_PADDING);
            measure(iters, || {
                let tape = parser.parse_padded(black_box(&mut json)).unwrap();
                from_tape::<LargeData>(tape).unwrap()
            })
        }
        "parse" => measure(iters, || {
            black_box(parser.parse_str(black_box(&json)).unwrap());
        }),
        "serde" => {
            let tape = parser.parse_str(&json).unwrap();
            measure(iters, || from_tape::<LargeData>(black_box(tape)).unwrap())
        }
        "ffi" | "view" => {
            use simdjson_sys::{SIMDJSON_MAXSIZE_BYTES, SIMDJSON_PADDING, dom_ffi};
            let mut raw = dom_ffi::parser_new(SIMDJSON_MAXSIZE_BYTES);
            let len = json.len();
            let mut padded = json.as_bytes().to_vec();
            padded.resize(len + SIMDJSON_PADDING, 0);
            // SAFETY: input has 64 initialized readable padding bytes.
            assert_eq!(
                unsafe { dom_ffi::parser_parse(raw.pin_mut(), &padded[..len], false) },
                0
            );
            if mode == "ffi" {
                measure(iters, || {
                    // SAFETY: the initialized padding allocation is unchanged.
                    assert_eq!(
                        unsafe {
                            dom_ffi::parser_parse(raw.pin_mut(), black_box(&padded[..len]), false)
                        },
                        0
                    );
                })
            } else {
                measure(iters, || {
                    // SAFETY: the parser has not changed since its successful parse.
                    unsafe { dom_ffi::parser_get_tape_view(black_box(raw.as_ref().unwrap())) }
                })
            }
        }
        "serde-json" => measure(iters, || {
            serde_json::from_str::<LargeData>(black_box(&json)).unwrap()
        }),
        _ => panic!("unknown mode: {mode}"),
    };
    println!(
        "mode={mode} users={users} bytes={} iters={iters} ns_per_iter={ns:.2}",
        json.len()
    );
}
