// SPDX-License-Identifier: MIT or Apache-2.0
#![allow(
    clippy::unreadable_literal,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::should_panic_without_expect,
    clippy::items_after_statements,
    clippy::match_same_arms
)]
//! Performance benchmarks for multi-codec

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use multi_codec::Codec;
use multi_trait::TryDecodeFrom;

/// Serialize a value to CBOR bytes using `ciborium`.
fn cbor_to_vec<T: serde::Serialize>(value: &T) -> Vec<u8> {
    let mut buf = Vec::new();
    ciborium::into_writer(value, &mut buf).expect("CBOR serialize");
    buf
}

/// Benchmark conversions from u64
fn bench_from_u64(c: &mut Criterion) {
    c.bench_function("codec_from_u64_valid", |b| {
        b.iter(|| Codec::try_from(black_box(0xEDu64)));
    });

    c.bench_function("codec_from_u64_invalid", |b| {
        b.iter(|| Codec::try_from(black_box(0xDEADBEEFu64)));
    });
}

/// Benchmark conversions to u64
fn bench_to_u64(c: &mut Criterion) {
    let codec = Codec::Ed25519Pub;

    c.bench_function("codec_to_u64", |b| {
        b.iter(|| {
            let _code: u64 = black_box(codec).into();
        });
    });

    c.bench_function("codec_code_method", |b| b.iter(|| black_box(codec).code()));
}

/// Benchmark conversions from string
fn bench_from_str(c: &mut Criterion) {
    c.bench_function("codec_from_str_valid", |b| {
        b.iter(|| Codec::try_from(black_box("ed25519-pub")));
    });

    c.bench_function("codec_from_str_invalid", |b| {
        b.iter(|| Codec::try_from(black_box("unknown-codec")));
    });
}

/// Benchmark conversions to string
fn bench_to_str(c: &mut Criterion) {
    c.bench_function("codec_as_str", |b| {
        let codec = Codec::Ed25519Pub;
        b.iter(|| {
            let s = codec.as_str();
            black_box(s.len())
        });
    });
}

/// Benchmark encoding
fn bench_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoding");

    let codecs = vec![
        ("Identity", Codec::Identity),
        ("Sha2256", Codec::Sha2256),
        ("Ed25519Pub", Codec::Ed25519Pub),
    ];

    for (name, codec) in codecs {
        group.bench_with_input(BenchmarkId::new("into_vec", name), &codec, |b, &codec| {
            b.iter(|| {
                let _v: Vec<u8> = black_box(codec).into();
            });
        });
    }

    group.finish();
}

/// Benchmark decoding
fn bench_decoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("decoding");

    let test_cases = vec![
        ("Identity", vec![0x00]),
        ("Sha2256", vec![0x12]),
        ("Ed25519Pub", vec![0xED, 0x01]),
    ];

    for (name, bytes) in test_cases {
        group.bench_with_input(
            BenchmarkId::new("try_decode_from", name),
            &bytes,
            |b, bytes| b.iter(|| Codec::try_decode_from(black_box(bytes.as_slice()))),
        );
    }

    group.finish();
}

/// Benchmark hash operations
fn bench_hash(c: &mut Criterion) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let codec = Codec::Ed25519Pub;

    c.bench_function("codec_hash", |b| {
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            black_box(codec).hash(&mut hasher);
            hasher.finish()
        });
    });
}

/// Benchmark roundtrip operations
fn bench_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");

    group.bench_function("encode_decode", |b| {
        let codec = Codec::Ed25519Pub;
        b.iter(|| {
            let encoded: Vec<u8> = black_box(codec).into();
            let (decoded, _) = Codec::try_decode_from(black_box(&encoded)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("str_codec_str", |b| {
        b.iter(|| {
            let codec = Codec::try_from(black_box("ed25519-pub")).unwrap();
            let len = codec.as_str().len();
            black_box(len);
        });
    });

    group.bench_function("u64_codec_u64", |b| {
        b.iter(|| {
            let codec = Codec::try_from(black_box(0xEDu64)).unwrap();
            let _code: u64 = codec.into();
        });
    });

    group.finish();
}

/// Benchmark serde operations
#[cfg(feature = "serde")]
fn bench_serde(c: &mut Criterion) {
    use serde_json;

    let mut group = c.benchmark_group("serde");

    let codec = Codec::Ed25519Pub;

    group.bench_function("json_serialize", |b| {
        b.iter(|| serde_json::to_string(&black_box(codec)));
    });

    let json = serde_json::to_string(&codec).unwrap();
    group.bench_function("json_deserialize", |b| {
        b.iter(|| serde_json::from_str::<Codec>(black_box(&json)));
    });

    group.bench_function("cbor_serialize", |b| {
        b.iter(|| cbor_to_vec(&black_box(codec)));
    });

    let cbor = cbor_to_vec(&codec);
    group.bench_function("cbor_deserialize", |b| {
        b.iter(|| ciborium::from_reader::<Codec, _>(black_box(cbor.as_slice())));
    });

    group.finish();
}

#[cfg(feature = "serde")]
criterion_group!(
    benches,
    bench_from_u64,
    bench_to_u64,
    bench_from_str,
    bench_to_str,
    bench_encoding,
    bench_decoding,
    bench_hash,
    bench_roundtrip,
    bench_serde
);

#[cfg(not(feature = "serde"))]
criterion_group!(
    benches,
    bench_from_u64,
    bench_to_u64,
    bench_from_str,
    bench_to_str,
    bench_encoding,
    bench_decoding,
    bench_hash,
    bench_roundtrip
);

criterion_main!(benches);
