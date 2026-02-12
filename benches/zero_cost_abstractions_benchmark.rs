// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(missing_docs, unused_results)]

//! Zero-cost abstractions performance benchmarks

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use langweave::{detect_language, detect_language_async, translate, is_language_supported, supported_languages};
use langweave::optimized::{is_language_supported_zero_alloc, supported_languages_optimized};
use std::sync::Arc;
use tokio::runtime::Runtime;

#[cfg(feature = "batch")]
use langweave::batch::{BatchConfig, detect_batch_async, translate_batch_async};

#[cfg(feature = "stream")]
use langweave::streaming::{StreamConfig, detect_language_stream, chunk_text};

const SAMPLE_TEXTS: &[&str] = &[
    "Hello world",
    "Bonjour le monde",
    "Hola mundo",
    "Hallo Welt",
    "Ciao mondo",
    "Olá mundo",
    "Привет мир",
    "مرحبا بالعالم",
    "你好世界",
    "안녕하세요",
    "こんにちは",
    "नमस्ते दुनिया",
    "שלום עולם",
    "Halo dunia",
    "Hallo wereld",
];

#[cfg(feature = "stream")]
const LONG_TEXT: &str = "This is a much longer text that contains multiple sentences and paragraphs. We want to test how the system performs when dealing with larger amounts of text data. This text should be long enough to trigger any performance issues related to string processing, memory allocation, and text chunking algorithms.";

fn bench_string_clones(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_clones");

    // Current implementation with clones
    group.bench_function("detect_language_async_with_clones", |b| {
        let rt = Runtime::new().unwrap();
        b.to_async(&rt).iter(|| async {
            detect_language_async(black_box("Hello world")).await.unwrap()
        });
    });

    // Measure synchronous version for comparison
    group.bench_function("detect_language_sync", |b| {
        b.iter(|| {
            detect_language(black_box("Hello world")).unwrap()
        });
    });

    group.finish();
}

fn bench_language_support_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("language_support");
    group.throughput(Throughput::Elements(15)); // Number of supported languages

    // Original implementation
    group.bench_function("is_language_supported_original", |b| {
        b.iter(|| {
            for &lang in ["en", "fr", "de", "es", "pt", "it", "nl", "ru", "ar", "he", "hi", "ja", "ko", "zh", "id"].iter() {
                black_box(is_language_supported(black_box(lang)));
            }
        });
    });

    // Optimized zero-allocation version
    group.bench_function("is_language_supported_zero_alloc", |b| {
        b.iter(|| {
            for &lang in ["en", "fr", "de", "es", "pt", "it", "nl", "ru", "ar", "he", "hi", "ja", "ko", "zh", "id"].iter() {
                black_box(is_language_supported_zero_alloc(black_box(lang)));
            }
        });
    });

    // Case-insensitive lookups
    group.bench_function("case_insensitive_lookup", |b| {
        b.iter(|| {
            for &lang in ["EN", "Fr", "DE", "Es", "PT"].iter() {
                black_box(is_language_supported_zero_alloc(black_box(lang)));
            }
        });
    });

    group.finish();
}

fn bench_supported_languages(c: &mut Criterion) {
    let mut group = c.benchmark_group("supported_languages");

    // Original heap-allocating version
    group.bench_function("supported_languages_heap", |b| {
        b.iter(|| {
            let langs = black_box(supported_languages());
            black_box(langs.len());
        });
    });

    // Optimized static version
    group.bench_function("supported_languages_static", |b| {
        b.iter(|| {
            let langs = black_box(supported_languages_optimized());
            black_box(langs.len());
        });
    });

    group.finish();
}

fn bench_translation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("translation");

    for text in SAMPLE_TEXTS.iter() {
        group.bench_with_input(
            BenchmarkId::new("translate_sync", text.len()),
            text,
            |b, text| {
                b.iter(|| {
                    let _ = translate(black_box("fr"), black_box(text));
                });
            }
        );
    }

    group.finish();
}

fn bench_memory_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocations");

    // Measure heap allocations in string operations
    group.bench_function("string_to_string_overhead", |b| {
        let text = "Hello world";
        b.iter(|| {
            // This simulates the unnecessary clones in async functions
            let _owned1 = black_box(text.to_string());
            let _owned2 = black_box(text.to_string());
        });
    });

    // Measure zero-allocation alternative
    group.bench_function("string_slice_operations", |b| {
        let text = "Hello world";
        b.iter(|| {
            // Zero allocation operations on string slices
            black_box(text.len());
            black_box(text.chars().count());
        });
    });

    group.finish();
}

#[cfg(feature = "batch")]
fn bench_batch_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_operations");
    let rt = Runtime::new().unwrap();

    // Batch detection with various concurrency levels
    for concurrency in [1, 5, 10, 20].iter() {
        let config = BatchConfig { max_concurrency: *concurrency };
        group.bench_with_input(
            BenchmarkId::new("detect_batch", concurrency),
            &config,
            |b, config| {
                b.to_async(&rt).iter(|| async {
                    detect_batch_async(black_box(SAMPLE_TEXTS), black_box(config)).await
                });
            }
        );
    }

    // Batch translation
    let config = BatchConfig::default();
    group.bench_function("translate_batch", |b| {
        b.to_async(&rt).iter(|| async {
            translate_batch_async(black_box("fr"), black_box(SAMPLE_TEXTS), black_box(&config)).await
        });
    });

    group.finish();
}

#[cfg(feature = "stream")]
fn bench_streaming_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming");
    let rt = Runtime::new().unwrap();

    // Text chunking performance
    group.bench_function("chunk_text", |b| {
        b.iter(|| {
            chunk_text(black_box(LONG_TEXT), black_box(100))
        });
    });

    // Streaming vs batch comparison
    let config = StreamConfig::default();
    group.bench_function("detect_language_stream", |b| {
        use tokio_stream::StreamExt;
        b.to_async(&rt).iter(|| async {
            let mut stream = detect_language_stream(black_box(LONG_TEXT), black_box(&config));
            let mut count = 0;
            while let Some(_result) = stream.next().await {
                count += 1;
            }
            count
        });
    });

    group.finish();
}

fn bench_async_runtime_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_overhead");
    let rt = Runtime::new().unwrap();

    // Measure spawn_blocking overhead
    group.bench_function("spawn_blocking_simple", |b| {
        b.to_async(&rt).iter(|| async {
            tokio::task::spawn_blocking(|| {
                black_box(42)
            }).await.unwrap()
        });
    });

    // Compare with direct computation
    group.bench_function("direct_computation", |b| {
        b.iter(|| {
            black_box(42)
        });
    });

    // Measure async function call overhead vs sync
    group.bench_function("detect_async_overhead", |b| {
        b.to_async(&rt).iter(|| async {
            detect_language_async(black_box("Hello world")).await.unwrap()
        });
    });

    group.bench_function("detect_sync_baseline", |b| {
        b.iter(|| {
            detect_language(black_box("Hello world")).unwrap()
        });
    });

    group.finish();
}

fn bench_arc_mutex_alternatives(c: &mut Criterion) {
    let mut group = c.benchmark_group("arc_mutex_alternatives");

    // Arc overhead measurement
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    group.bench_function("arc_clone_overhead", |b| {
        b.iter(|| {
            let _cloned = Arc::clone(&data);
            black_box(_cloned.len())
        });
    });

    // Direct reference baseline
    let data_ref = [1, 2, 3, 4, 5];
    group.bench_function("direct_reference", |b| {
        b.iter(|| {
            black_box(data_ref.len())
        });
    });

    group.finish();
}

fn bench_regex_vs_pattern_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_matching");

    let inputs = ["en", "EN", "fr", "FR", "invalid", "zz"];

    // Current regex-based approach (simulated)
    group.bench_function("case_insensitive_comparison", |b| {
        b.iter(|| {
            for &input in inputs.iter() {
                // Simulate the current eq_ignore_ascii_case approach
                black_box("en".eq_ignore_ascii_case(input) ||
                          "fr".eq_ignore_ascii_case(input) ||
                          "de".eq_ignore_ascii_case(input));
            }
        });
    });

    // Pattern matching approach
    group.bench_function("match_with_fallback", |b| {
        b.iter(|| {
            for &input in inputs.iter() {
                match input {
                    "en" | "fr" | "de" => black_box(true),
                    _ => black_box("en".eq_ignore_ascii_case(input) ||
                                  "fr".eq_ignore_ascii_case(input) ||
                                  "de".eq_ignore_ascii_case(input)),
                };
            }
        });
    });

    group.finish();
}

fn bench_compile_time_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_time");

    // Simulate const vs lazy static performance
    const STATIC_ARRAY: &[&str] = &["en", "fr", "de", "es", "pt"];

    group.bench_function("const_array_access", |b| {
        b.iter(|| {
            black_box(STATIC_ARRAY.len());
            black_box(STATIC_ARRAY.contains(&"en"));
        });
    });

    // Lazy static simulation (using Arc to simulate the overhead)
    let lazy_vec = Arc::new(vec!["en", "fr", "de", "es", "pt"]);
    group.bench_function("arc_vec_access", |b| {
        b.iter(|| {
            black_box(lazy_vec.len());
            black_box(lazy_vec.contains(&"en"));
        });
    });

    group.finish();
}

// Comprehensive criterion groups
criterion_group!(
    benches,
    bench_string_clones,
    bench_language_support_lookup,
    bench_supported_languages,
    bench_translation_patterns,
    bench_memory_allocations,
    bench_async_runtime_overhead,
    bench_arc_mutex_alternatives,
    bench_regex_vs_pattern_matching,
    bench_compile_time_patterns
);

// Conditional groups for feature-gated benchmarks
#[cfg(feature = "batch")]
criterion_group!(batch_benches, bench_batch_operations);

#[cfg(feature = "stream")]
criterion_group!(stream_benches, bench_streaming_operations);

// Main benchmark entry point
#[cfg(all(feature = "batch", feature = "stream"))]
criterion_main!(benches, batch_benches, stream_benches);

#[cfg(all(feature = "batch", not(feature = "stream")))]
criterion_main!(benches, batch_benches);

#[cfg(all(not(feature = "batch"), feature = "stream"))]
criterion_main!(benches, stream_benches);

#[cfg(all(not(feature = "batch"), not(feature = "stream")))]
criterion_main!(benches);