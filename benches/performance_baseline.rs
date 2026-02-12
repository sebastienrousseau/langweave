//! Performance Baseline Benchmarks for LangWeave
//!
//! This benchmark establishes baseline metrics for core performance paths
//! and tests with scaling factors as requested.

#![allow(unused_results)]
#![allow(missing_docs)]

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use langweave::{
    is_language_supported, language_detector::LanguageDetector,
    language_detector_trait::LanguageDetectorTrait,
    supported_languages, translate, translator::Translator,
};
use std::hint::black_box;

/// Benchmark the hot path: language detection with different text sizes
fn bench_language_detection_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("language_detection_scaling");

    let base_text = "The quick brown fox jumps over the lazy dog. ";
    let detector = LanguageDetector::new();

    // Test with 1x, 10x, 100x scaling
    for scale in [1, 10, 100] {
        let text = base_text.repeat(scale);
        let text_len = text.len();

        group.throughput(Throughput::Bytes(text_len as u64));
        group.bench_with_input(
            BenchmarkId::new("sync", format!("{}x_scale", scale)),
            &text,
            |b, text| {
                b.iter(|| detector.detect(black_box(text)));
            },
        );
    }

    group.finish();
}

/// Benchmark translation performance with different text sizes
fn bench_translation_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("translation_scaling");

    let base_text = "Hello world. ";

    for scale in [1, 10, 100] {
        let text = base_text.repeat(scale);
        let text_len = text.len();

        group.throughput(Throughput::Bytes(text_len as u64));
        group.bench_with_input(
            BenchmarkId::new("translate", format!("{}x_scale", scale)),
            &text,
            |b, text| {
                b.iter(|| translate(black_box("fr"), black_box(text)));
            },
        );
    }

    group.finish();
}

/// Benchmark the critical allocation bottlenecks identified in analysis
fn bench_allocation_hotspots(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_hotspots");

    // HOTSPOT 1: supported_languages() creates Vec<String> every call
    group.bench_function("supported_languages_vec_alloc", |b| {
        b.iter(|| black_box(supported_languages()));
    });

    // HOTSPOT 2: is_language_supported calls supported_languages
    group.bench_function("is_language_supported_double_alloc", |b| {
        b.iter(|| is_language_supported(black_box("en")));
    });

    // HOTSPOT 3: LanguageDetector::new clones patterns Arc
    group.bench_function("language_detector_new_clone", |b| {
        b.iter(|| black_box(LanguageDetector::new()));
    });

    // HOTSPOT 4: Translator creation + validation
    group.bench_function("translator_creation_cost", |b| {
        b.iter(|| Translator::new(black_box("fr")).ok());
    });

    group.finish();
}

/// Benchmark async vs sync performance
fn bench_async_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_overhead");

    let detector = LanguageDetector::new();
    let text = "Hello world";

    // Sync baseline
    group.bench_function("sync_baseline", |b| {
        b.iter(|| detector.detect(black_box(text)));
    });

    // Async with runtime overhead
    group.bench_function("async_with_runtime", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| rt.block_on(detector.detect_async(black_box(text))));
    });

    group.finish();
}

/// Test memory usage patterns under sustained load
fn bench_sustained_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("sustained_load");

    let detector = LanguageDetector::new();
    let texts = [
        "The quick brown fox",
        "Le chat noir",
        "Der braune Fuchs",
        "El gato rápido",
    ];

    // Simulate sustained load: many sequential operations
    group.throughput(Throughput::Elements(100));
    group.bench_function("sequential_100_detections", |b| {
        b.iter(|| {
            for i in 0..100 {
                let text = &texts[i % texts.len()];
                let _ = detector.detect(black_box(text));
            }
        });
    });

    group.finish();
}

/// Benchmark regex compilation cost (static vs dynamic)
fn bench_regex_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex_patterns");

    // Cost of regex compilation (shows benefit of static compilation)
    group.bench_function("regex_compilation_cost", |b| {
        b.iter(|| {
            regex::Regex::new(black_box(
                r"(?i)\b(hello|hi|hey|goodbye|bye|thank you|thanks|please|the|a|an|in|on|at|for|to|of)\b"
            )).unwrap()
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_language_detection_scaling,
    bench_translation_scaling,
    bench_allocation_hotspots,
    bench_async_overhead,
    bench_sustained_load,
    bench_regex_patterns
);
criterion_main!(benches);
