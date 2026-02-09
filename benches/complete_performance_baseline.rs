// SPDX-License-Identifier: Apache-2.0 OR MIT
// Complete performance baseline for langweave v0.0.2 certification

#![allow(unused_results, unused_must_use)]
#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use langweave::{
    is_language_supported, supported_languages, translate, detect_language,
    language_detector::LanguageDetector,
    language_detector_trait::LanguageDetectorTrait,
    translator::Translator,
};
use std::hint::black_box;
use tokio::runtime::Runtime;

/// All 15 supported languages for comprehensive testing
const ALL_LANGUAGES: &[&str] = &[
    "en", "fr", "de", "es", "pt", "it", "nl", "ru",
    "ar", "he", "hi", "ja", "ko", "zh", "id"
];

/// Test phrases in each language for translation benchmarks
const TEST_PHRASES: &[(&str, &str)] = &[
    ("en", "Hello world"),
    ("fr", "Bonjour le monde"),
    ("de", "Hallo Welt"),
    ("es", "Hola mundo"),
    ("pt", "Olá mundo"),
    ("it", "Ciao mondo"),
    ("nl", "Hallo wereld"),
    ("ru", "Привет мир"),
    ("ar", "مرحبا بالعالم"),
    ("he", "שלום עולם"),
    ("hi", "नमस्ते संसार"),
    ("ja", "こんにちは世界"),
    ("ko", "안녕하세요 세계"),
    ("zh", "你好世界"),
    ("id", "Halo dunia"),
];

/// Benchmark all 15 language validation operations
fn bench_language_validation_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("language_validation_complete");

    for &lang in ALL_LANGUAGES {
        group.bench_with_input(
            BenchmarkId::new("is_supported", lang),
            &lang,
            |b, &lang| {
                b.iter(|| black_box(is_language_supported(lang)))
            },
        );
    }

    // Test invalid languages too
    let invalid_langs = ["xx", "zz", "invalid"];
    for &lang in &invalid_langs {
        group.bench_with_input(
            BenchmarkId::new("is_supported_invalid", lang),
            &lang,
            |b, &lang| {
                b.iter(|| black_box(is_language_supported(lang)))
            },
        );
    }

    group.finish();
}

/// Benchmark translation for all 15 languages
fn bench_translation_all_languages(c: &mut Criterion) {
    let mut group = c.benchmark_group("translation_all_languages");

    for &lang in ALL_LANGUAGES {
        // Benchmark translator creation
        group.bench_with_input(
            BenchmarkId::new("translator_creation", lang),
            &lang,
            |b, &lang| {
                b.iter(|| black_box(Translator::new(lang)))
            },
        );

        // Benchmark actual translation
        if let Ok(translator) = Translator::new(lang) {
            group.bench_with_input(
                BenchmarkId::new("translate_hello", lang),
                &translator,
                |b, translator| {
                    b.iter(|| black_box(translator.translate("Hello")))
                },
            );
        }
    }

    group.finish();
}

/// Benchmark language detection for all 15 languages
fn bench_language_detection_all(c: &mut Criterion) {
    let detector = LanguageDetector::new();
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("language_detection_all");

    for &(lang, phrase) in TEST_PHRASES {
        // Sync detection
        group.bench_with_input(
            BenchmarkId::new("detect_sync", lang),
            &phrase,
            |b, &phrase| {
                b.iter(|| black_box(detector.detect(phrase)))
            },
        );

        // Async detection (blocking in benchmark)
        group.bench_with_input(
            BenchmarkId::new("detect_async", lang),
            &phrase,
            |b, &phrase| {
                b.iter(|| {
                    rt.block_on(async {
                        black_box(detect_language(phrase).await)
                    })
                })
            },
        );
    }

    group.finish();
}

/// Stress test with 10x typical workload
fn bench_stress_test_10x(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress_test_10x");

    // Create 10x longer text
    let base_text = "The quick brown fox jumps over the lazy dog. ";
    let text_10x = base_text.repeat(10);

    let detector = LanguageDetector::new();
    let rt = Runtime::new().unwrap();

    group.throughput(Throughput::Bytes(text_10x.len() as u64));

    // Language detection stress test
    group.bench_function("language_detection_10x", |b| {
        b.iter(|| black_box(detector.detect(&text_10x)))
    });

    // Async language detection stress test
    group.bench_function("async_language_detection_10x", |b| {
        b.iter(|| {
            rt.block_on(async {
                black_box(detect_language(&text_10x).await)
            })
        })
    });

    // Translation stress test - simulate translating 10 phrases
    group.bench_function("translation_batch_10x", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for _ in 0..10 {
                if let Ok(result) = translate("fr", "Hello") {
                    results.push(black_box(result));
                }
            }
            black_box(results)
        })
    });

    group.finish();
}

/// Stress test with 100x typical workload
fn bench_stress_test_100x(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress_test_100x");

    // Create 100x longer text
    let base_text = "The quick brown fox jumps over the lazy dog. ";
    let text_100x = base_text.repeat(100);

    let detector = LanguageDetector::new();
    let rt = Runtime::new().unwrap();

    group.throughput(Throughput::Bytes(text_100x.len() as u64));

    // Language detection stress test
    group.bench_function("language_detection_100x", |b| {
        b.iter(|| black_box(detector.detect(&text_100x)))
    });

    // Async language detection stress test
    group.bench_function("async_language_detection_100x", |b| {
        b.iter(|| {
            rt.block_on(async {
                black_box(detect_language(&text_100x).await)
            })
        })
    });

    // Translation stress test - simulate translating 100 phrases
    group.bench_function("translation_batch_100x", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for _ in 0..100 {
                if let Ok(result) = translate("fr", "Hello") {
                    results.push(black_box(result));
                }
            }
            black_box(results)
        })
    });

    group.finish();
}

/// Memory allocation hot path benchmarks
fn bench_memory_hotpaths(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_hotpaths");

    // Benchmark the inefficient supported_languages() allocation
    group.bench_function("supported_languages_vec_allocation", |b| {
        b.iter(|| {
            // Creates new Vec<String> every call - major hotpath issue
            black_box(supported_languages())
        })
    });

    // Benchmark cascading allocations in language validation
    group.bench_function("validation_cascade_allocations", |b| {
        b.iter(|| {
            // This triggers: supported_languages() + to_lowercase() + contains()
            black_box(is_language_supported("FR"))
        })
    });

    // Benchmark regex compilation cost (should be cached)
    group.bench_function("detector_creation", |b| {
        b.iter(|| {
            // Creates new detector with regex compilation
            black_box(LanguageDetector::new())
        })
    });

    // Benchmark string operations in hot paths
    group.bench_function("string_operations", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for &lang in ALL_LANGUAGES {
                results.push(black_box(lang.to_lowercase()));
            }
            black_box(results)
        })
    });

    group.finish();
}

/// Performance regression detection benchmarks
fn bench_regression_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("regression_detection");

    // Core library functions that must remain fast
    group.bench_function("high_frequency_is_supported", |b| {
        b.iter(|| {
            // Simulate high-frequency validation calls
            for &lang in ALL_LANGUAGES {
                black_box(is_language_supported(lang));
            }
        })
    });

    group.bench_function("high_frequency_translation", |b| {
        b.iter(|| {
            // Simulate high-frequency translation calls
            for _ in 0..10 {
                if let Ok(result) = translate("en", "Hello") {
                    black_box(result);
                }
            }
        })
    });

    // Test edge cases that might cause performance regression
    let edge_cases = [
        "",
        "a",
        "This is a very long sentence that might cause performance issues in regex matching or other operations",
        "مرحبا", // Arabic
        "こんにちは", // Japanese
        "你好", // Chinese
        "Hello world this is English text",
        "123 456 789 numbers only",
        "Mixed 123 content with مرحبا numbers and text",
    ];

    let detector = LanguageDetector::new();
    group.bench_function("edge_case_detection", |b| {
        b.iter(|| {
            for &text in &edge_cases {
                // This should be fast even for edge cases
                let _ = black_box(detector.detect(text));
            }
        })
    });

    group.finish();
}

/// Sustained load performance test
fn bench_sustained_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("sustained_load");

    // Simulate sustained translation load
    group.bench_function("sustained_translation_load", |b| {
        // Pre-create translators to avoid creation overhead
        let translators: Vec<_> = ALL_LANGUAGES.iter()
            .filter_map(|&lang| Translator::new(lang).ok())
            .collect();

        b.iter(|| {
            // Simulate sustained mixed-language translation load
            for translator in &translators {
                black_box(translator.translate("Hello"));
                black_box(translator.translate("Goodbye"));
                black_box(translator.translate("Thank you"));
            }
        })
    });

    // Simulate sustained detection load
    let detector = LanguageDetector::new();
    group.bench_function("sustained_detection_load", |b| {
        b.iter(|| {
            for &(_, phrase) in TEST_PHRASES {
                black_box(detector.detect(phrase));
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_language_validation_all,
    bench_translation_all_languages,
    bench_language_detection_all,
    bench_stress_test_10x,
    bench_stress_test_100x,
    bench_memory_hotpaths,
    bench_regression_detection,
    bench_sustained_load
);

criterion_main!(benches);