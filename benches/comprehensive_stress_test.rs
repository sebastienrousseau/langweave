// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Comprehensive stress test and performance baseline establishment for LangWeave
//!
//! This benchmark establishes performance budgets for all hot paths and tests
//! with large inputs (10x, 100x typical workload) as required.

#![allow(unused_results)]
#![allow(missing_docs)]

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput
};
use langweave::{
    detect_language, detect_language_async, translate, supported_languages,
    is_language_supported, language_detector::LanguageDetector,
    language_detector_trait::LanguageDetectorTrait,
    optimized::{
        supported_languages_optimized, is_language_supported_optimized,
        is_language_supported_zero_alloc
    }
};
use std::hint::black_box;
use std::time::Duration;

/// Establish baseline performance metrics for language detection
fn benchmark_language_detection_baselines(c: &mut Criterion) {
    let mut group = c.benchmark_group("language_detection_baselines");
    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(10));

    let detector = LanguageDetector::new();

    // Pre-allocate repeated strings to avoid lifetime issues
    let medium_en = "Hello world this is a test sentence. ".repeat(10);
    let medium_fr = "Bonjour le monde ceci est une phrase de test. ".repeat(10);
    let medium_de = "Hallo Welt das ist ein Testsatz. ".repeat(10);
    let large_en = "Hello world this is a test sentence with many words. ".repeat(100);
    let large_fr = "Bonjour le monde ceci est une phrase de test avec beaucoup de mots. ".repeat(100);
    let large_de = "Hallo Welt das ist ein Testsatz mit vielen Wörtern. ".repeat(100);
    let mixed_large = "Hello world bonjour le monde hallo Welt. ".repeat(50);

    // Test cases with varying complexity and input sizes
    let test_cases: Vec<(&str, &str)> = vec![
        // Small inputs (typical case)
        ("tiny_en", "Hello"),
        ("tiny_fr", "Bonjour"),
        ("tiny_de", "Hallo"),

        // Medium inputs (10x typical)
        ("medium_en", &medium_en),
        ("medium_fr", &medium_fr),
        ("medium_de", &medium_de),

        // Large inputs (100x typical)
        ("large_en", &large_en),
        ("large_fr", &large_fr),
        ("large_de", &large_de),

        // Mixed language inputs (stress case)
        ("mixed_small", "Hello bonjour hallo"),
        ("mixed_large", &mixed_large),
    ];

    for (name, text) in test_cases {
        let text_size = text.len();
        group.throughput(Throughput::Bytes(text_size as u64));

        group.bench_with_input(
            BenchmarkId::new("sync_detect", name),
            &text,
            |b, text| {
                b.iter(|| {
                    black_box(detect_language(black_box(text)))
                })
            }
        );

        group.bench_with_input(
            BenchmarkId::new("direct_detect", name),
            &text,
            |b, text| {
                b.iter(|| {
                    black_box(detector.detect(black_box(text)))
                })
            }
        );
    }

    group.finish();
}

/// Benchmark async language detection performance
fn benchmark_async_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_detection_performance");
    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(10));

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Pre-allocate repeated strings to avoid lifetime issues
    let medium_async = "Hello world test sentence. ".repeat(50);
    let large_async = "Hello world test sentence. ".repeat(500);

    let test_cases: Vec<(&str, &str)> = vec![
        ("small_async", "Hello world"),
        ("medium_async", &medium_async),
        ("large_async", &large_async),
    ];

    for (name, text) in test_cases {
        group.throughput(Throughput::Bytes(text.len() as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &text,
            |b, text| {
                b.to_async(&rt).iter(|| async {
                    black_box(detect_language_async(black_box(*text)).await)
                })
            }
        );
    }

    group.finish();
}

/// Benchmark translation performance with various input sizes
fn benchmark_translation_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("translation_baselines");
    group.warm_up_time(Duration::from_secs(2));
    group.measurement_time(Duration::from_secs(8));

    let languages = ["en", "fr", "de", "es"];
    let keys = ["Hello", "Goodbye", "Thank you", "Please"];

    // Test translation performance for each language/key combination
    for lang in &languages {
        for key in &keys {
            group.bench_with_input(
                BenchmarkId::new("translate", format!("{}_{}", lang, key)),
                &(lang, key),
                |b, (lang, key)| {
                    b.iter(|| {
                        black_box(translate(black_box(lang), black_box(key)))
                    })
                }
            );
        }
    }

    group.finish();
}

/// Benchmark language support checking with different optimization levels
fn benchmark_language_support_optimizations(c: &mut Criterion) {
    let mut group = c.benchmark_group("language_support_optimizations");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));

    let test_languages = ["en", "fr", "de", "es", "invalid", "zz", "EN", "FR"];

    for lang in &test_languages {
        // Original implementation
        group.bench_with_input(
            BenchmarkId::new("original", lang),
            lang,
            |b, lang| {
                b.iter(|| black_box(is_language_supported(black_box(lang))))
            }
        );

        // Optimized implementation
        group.bench_with_input(
            BenchmarkId::new("optimized", lang),
            lang,
            |b, lang| {
                b.iter(|| black_box(is_language_supported_optimized(black_box(lang))))
            }
        );

        // Zero-allocation implementation
        group.bench_with_input(
            BenchmarkId::new("zero_alloc", lang),
            lang,
            |b, lang| {
                b.iter(|| black_box(is_language_supported_zero_alloc(black_box(lang))))
            }
        );
    }

    group.finish();
}

/// Benchmark memory allocation patterns in hot paths
fn benchmark_memory_hotspots(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation_hotspots");
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));

    // Benchmark supported_languages() allocation pattern
    group.bench_function("supported_languages_vec", |b| {
        b.iter(|| {
            black_box(supported_languages())
        })
    });

    group.bench_function("supported_languages_optimized", |b| {
        b.iter(|| {
            black_box(supported_languages_optimized())
        })
    });

    // Benchmark repeated allocations (worst case scenario)
    group.bench_function("repeated_allocations_1000x", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(supported_languages());
            }
        })
    });

    group.bench_function("repeated_optimized_1000x", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(supported_languages_optimized());
            }
        })
    });

    group.finish();
}

/// Stress test with extreme workload (1000x typical)
fn benchmark_extreme_stress_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("extreme_stress_test");
    group.warm_up_time(Duration::from_secs(5));
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(10); // Smaller sample size for very long operations

    // Extremely large text (1MB+)
    let huge_text = "The quick brown fox jumps over the lazy dog. ".repeat(10000);

    group.throughput(Throughput::Bytes(huge_text.len() as u64));
    group.bench_function("detect_1mb_text", |b| {
        b.iter(|| {
            black_box(detect_language(black_box(&huge_text)))
        })
    });

    // Sustained high-frequency operations
    group.bench_function("sustained_detection_burst", |b| {
        let texts = vec![
            "Hello world",
            "Bonjour monde",
            "Hallo Welt",
            "Hola mundo",
            "Ciao mondo"
        ];

        b.iter(|| {
            for _ in 0..100 {
                for text in &texts {
                    let _ = black_box(detect_language(black_box(text)));
                }
            }
        })
    });

    group.finish();
}

/// Concurrent access performance test
fn benchmark_concurrent_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_performance");
    group.warm_up_time(Duration::from_secs(2));
    group.measurement_time(Duration::from_secs(8));

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Test with varying numbers of concurrent tasks
    for &concurrency in &[1, 4, 8, 16, 32] {
        group.bench_with_input(
            BenchmarkId::new("concurrent_detection", concurrency),
            &concurrency,
            |b, &concurrency| {
                b.to_async(&rt).iter(|| async {
                    let mut handles = Vec::with_capacity(concurrency);

                    for i in 0..concurrency {
                        let text = match i % 4 {
                            0 => "Hello world",
                            1 => "Bonjour monde",
                            2 => "Hallo Welt",
                            _ => "Hola mundo"
                        };

                        handles.push(tokio::spawn(async move {
                            detect_language_async(text).await
                        }));
                    }

                    for handle in handles {
                        let _ = black_box(handle.await);
                    }
                })
            }
        );
    }

    group.finish();
}

/// Regression detector pattern benchmarks
fn benchmark_regex_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex_performance");
    group.warm_up_time(Duration::from_secs(2));
    group.measurement_time(Duration::from_secs(8));

    let detector = LanguageDetector::new();

    // Test different types of input that exercise regex patterns
    let test_cases = vec![
        ("english_patterns", "Hello world thank you please"),
        ("french_patterns", "Bonjour monde merci s'il vous plaît"),
        ("german_patterns", "Hallo Welt danke bitte schön"),
        ("mixed_scripts", "Hello مرحبا 你好 こんにちは"),
        ("no_match_fallback", "xyz123 nonexistent patterns abc"),
        ("unicode_heavy", "مرحبا أهلاً وسهلاً كيف الحال؟"),
        ("cjk_patterns", "你好世界 こんにちは世界 안녕하세요 세계"),
    ];

    for (name, text) in test_cases {
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &text,
            |b, text| {
                b.iter(|| {
                    black_box(detector.detect(black_box(text)))
                })
            }
        );
    }

    group.finish();
}

criterion_group!(
    stress_benches,
    benchmark_language_detection_baselines,
    benchmark_async_detection,
    benchmark_translation_performance,
    benchmark_language_support_optimizations,
    benchmark_memory_hotspots,
    benchmark_extreme_stress_test,
    benchmark_concurrent_performance,
    benchmark_regex_performance
);

criterion_main!(stress_benches);