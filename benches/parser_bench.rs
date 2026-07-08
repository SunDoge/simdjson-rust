use criterion::{Criterion, black_box, criterion_group, criterion_main};
use serde::{Deserialize, Serialize};
use simdjson_rust::{dom::Parser, serde::from_tape};

// ---------------------------------------------------------------------------
// Struct Definitions for Benchmarking
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct SmallData {
    x: f64,
    y: f64,
    label: String,
    valid: bool,
}

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

// ---------------------------------------------------------------------------
// Helper generators
// ---------------------------------------------------------------------------

fn make_small_data() -> (SmallData, String) {
    let data = SmallData {
        x: 12.34,
        y: -56.78,
        label: "Benchmark Coordinates".to_string(),
        valid: true,
    };
    let json = serde_json::to_string(&data).unwrap();
    (data, json)
}

fn make_medium_data() -> (UserProfile, String) {
    let data = UserProfile {
        id: 998877,
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        active: true,
        friends: (0..50).map(|i| 1000 + i).collect(),
        score: 98.76,
    };
    let json = serde_json::to_string(&data).unwrap();
    (data, json)
}

fn make_large_data() -> (LargeData, String) {
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
    let json = serde_json::to_string(&data).unwrap();
    (data, json)
}

// ---------------------------------------------------------------------------
// Benchmarks
// ---------------------------------------------------------------------------

fn bench_small(c: &mut Criterion) {
    let (_, json) = make_small_data();
    let mut parser = Parser::default();

    let mut group = c.benchmark_group("Small JSON (~100B)");

    group.bench_function("serde_json_deserialize_struct", |b| {
        b.iter(|| {
            let res: SmallData = serde_json::from_str(black_box(&json)).unwrap();
            black_box(res);
        })
    });

    group.bench_function("serde_json_parse_to_value", |b| {
        b.iter(|| {
            let res: serde_json::Value = serde_json::from_str(black_box(&json)).unwrap();
            black_box(res);
        })
    });

    group.bench_function("simdjson_rust_deserialize_struct_reuse", |b| {
        b.iter(|| {
            let tape = parser.parse_str(black_box(&json)).unwrap();
            let res: SmallData = from_tape(tape).unwrap();
            black_box(res);
        })
    });

    group.bench_function("simdjson_rust_parse_to_value_reuse", |b| {
        b.iter(|| {
            let res = parser.parse_to_value(black_box(&json)).unwrap();
            black_box(res);
        })
    });

    group.finish();
}

fn bench_medium(c: &mut Criterion) {
    let (_, json) = make_medium_data();
    let mut parser = Parser::default();

    let mut group = c.benchmark_group("Medium JSON (~2KB)");

    group.bench_function("serde_json_deserialize_struct", |b| {
        b.iter(|| {
            let res: UserProfile = serde_json::from_str(black_box(&json)).unwrap();
            black_box(res);
        })
    });

    group.bench_function("serde_json_parse_to_value", |b| {
        b.iter(|| {
            let res: serde_json::Value = serde_json::from_str(black_box(&json)).unwrap();
            black_box(res);
        })
    });

    group.bench_function("simdjson_rust_deserialize_struct_reuse", |b| {
        b.iter(|| {
            let tape = parser.parse_str(black_box(&json)).unwrap();
            let res: UserProfile = from_tape(tape).unwrap();
            black_box(res);
        })
    });

    group.bench_function("simdjson_rust_parse_to_value_reuse", |b| {
        b.iter(|| {
            let res = parser.parse_to_value(black_box(&json)).unwrap();
            black_box(res);
        })
    });

    group.finish();
}

fn bench_large(c: &mut Criterion) {
    let (_, json) = make_large_data();
    let mut parser = Parser::default();

    let mut group = c.benchmark_group("Large JSON (~100KB)");

    group.bench_function("serde_json_deserialize_struct", |b| {
        b.iter(|| {
            let res: LargeData = serde_json::from_str(black_box(&json)).unwrap();
            black_box(res);
        })
    });

    group.bench_function("serde_json_parse_to_value", |b| {
        b.iter(|| {
            let res: serde_json::Value = serde_json::from_str(black_box(&json)).unwrap();
            black_box(res);
        })
    });

    group.bench_function("simdjson_rust_deserialize_struct_reuse", |b| {
        b.iter(|| {
            let tape = parser.parse_str(black_box(&json)).unwrap();
            let res: LargeData = from_tape(tape).unwrap();
            black_box(res);
        })
    });

    group.bench_function("simdjson_rust_parse_to_value_reuse", |b| {
        b.iter(|| {
            let res = parser.parse_to_value(black_box(&json)).unwrap();
            black_box(res);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_small, bench_medium, bench_large);
criterion_main!(benches);
