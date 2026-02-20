// SPDX-License-Identifier: Apache-2.0 OR MIT
#![allow(unused_results)]
#![allow(missing_docs)]
#![allow(dead_code)]

use criterion::{BenchmarkGroup, measurement::WallTime};
use criterion::{BenchmarkId, Criterion, Throughput};
use langweave::{
    detect_language, detect_language_async, is_language_supported,
    language_detector::LanguageDetector,
    language_detector_trait::LanguageDetectorTrait,
    optimized::{
        is_language_supported_optimized,
        is_language_supported_zero_alloc,
        supported_languages_optimized,
    },
    supported_languages, translate,
};
use std::hint::black_box;
use std::time::Duration;

#[inline]
fn bench_fast() -> bool {
    match std::env::var("BENCH_FAST") {
        Ok(v) => {
            let v = v.trim().to_ascii_lowercase();
            matches!(v.as_str(), "1" | "true" | "yes" | "on")
        }
        Err(_) => false,
    }
}

#[inline]
fn apply_profile(
    group: &mut BenchmarkGroup<'_, WallTime>,
    normal_warm: Duration,
    normal_measure: Duration,
) {
    if bench_fast() {
        group.warm_up_time(Duration::from_millis(100));
        group.measurement_time(Duration::from_millis(300));
        group.sample_size(10);
    } else {
        group.warm_up_time(normal_warm);
        group.measurement_time(normal_measure);
    }
}

/// Establish baseline performance metrics for language detection.
pub(crate) fn benchmark_language_detection_baselines(
    c: &mut Criterion,
) {
    let mut group = c.benchmark_group("language_detection_baselines");
    apply_profile(
        &mut group,
        Duration::from_secs(3),
        Duration::from_secs(10),
    );

    let detector = LanguageDetector::new();

    let medium_en = "Hello world this is a test sentence. ".repeat(10);
    let medium_fr =
        "Bonjour le monde ceci est une phrase de test. ".repeat(10);
    let medium_de = "Hallo Welt das ist ein Testsatz. ".repeat(10);
    let large_en =
        "Hello world this is a test sentence with many words. "
            .repeat(100);
    let large_fr = "Bonjour le monde ceci est une phrase de test avec beaucoup de mots. ".repeat(100);
    let large_de =
        "Hallo Welt das ist ein Testsatz mit vielen Wörtern. "
            .repeat(100);
    let mixed_large =
        "Hello world bonjour le monde hallo Welt. ".repeat(50);

    let mut test_cases: Vec<(&str, &str)> = vec![
        ("tiny_en", "Hello"),
        ("tiny_fr", "Bonjour"),
        ("tiny_de", "Hallo"),
        ("medium_en", &medium_en),
        ("medium_fr", &medium_fr),
        ("medium_de", &medium_de),
        ("large_en", &large_en),
        ("large_fr", &large_fr),
        ("large_de", &large_de),
        ("mixed_small", "Hello bonjour hallo"),
        ("mixed_large", &mixed_large),
    ];
    if bench_fast() {
        test_cases = vec![
            ("tiny_en", "Hello"),
            ("medium_en", &medium_en),
            ("large_en", &large_en),
            ("mixed_small", "Hello bonjour hallo"),
        ];
    }

    for (name, text) in test_cases {
        group.throughput(Throughput::Bytes(text.len() as u64));

        group.bench_with_input(
            BenchmarkId::new("sync_detect", name),
            &text,
            |b, text| {
                b.iter(|| black_box(detect_language(black_box(text))))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("direct_detect", name),
            &text,
            |b, text| {
                b.iter(|| black_box(detector.detect(black_box(text))))
            },
        );
    }

    group.finish();
}

/// Benchmark async language detection performance.
pub(crate) fn benchmark_async_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_detection_performance");
    apply_profile(
        &mut group,
        Duration::from_secs(3),
        Duration::from_secs(10),
    );

    let rt = tokio::runtime::Runtime::new().unwrap();
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
                    black_box(
                        detect_language_async(black_box(*text)).await,
                    )
                })
            },
        );
    }

    group.finish();
}

/// Benchmark translation performance with various input sizes.
pub(crate) fn benchmark_translation_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("translation_baselines");
    apply_profile(
        &mut group,
        Duration::from_secs(2),
        Duration::from_secs(8),
    );

    let (languages, keys): (&[&str], &[&str]) = if bench_fast() {
        (&["en", "fr"], &["Hello", "Goodbye"])
    } else {
        (
            &["en", "fr", "de", "es"],
            &["Hello", "Goodbye", "Thank you", "Please"],
        )
    };

    for &lang in languages {
        for &key in keys {
            group.bench_with_input(
                BenchmarkId::new(
                    "translate",
                    format!("{}_{}", lang, key),
                ),
                &(lang, key),
                |b, (lang, key)| {
                    b.iter(|| {
                        black_box(translate(
                            black_box(lang),
                            black_box(key),
                        ))
                    })
                },
            );
        }
    }

    group.finish();
}

/// Benchmark language support checking with different optimization levels.
pub(crate) fn benchmark_language_support_optimizations(
    c: &mut Criterion,
) {
    let mut group = c.benchmark_group("language_support_optimizations");
    apply_profile(
        &mut group,
        Duration::from_secs(1),
        Duration::from_secs(5),
    );

    let test_languages: &[&str] = if bench_fast() {
        &["en", "fr", "invalid", "EN"]
    } else {
        &["en", "fr", "de", "es", "invalid", "zz", "EN", "FR"]
    };

    for &lang in test_languages {
        group.bench_with_input(
            BenchmarkId::new("original", lang),
            lang,
            |b, lang| {
                b.iter(|| {
                    black_box(is_language_supported(black_box(lang)))
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("optimized", lang),
            lang,
            |b, lang| {
                b.iter(|| {
                    black_box(is_language_supported_optimized(
                        black_box(lang),
                    ))
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("zero_alloc", lang),
            lang,
            |b, lang| {
                b.iter(|| {
                    black_box(is_language_supported_zero_alloc(
                        black_box(lang),
                    ))
                })
            },
        );
    }

    group.finish();
}

/// Benchmark memory allocation patterns in hot paths.
pub(crate) fn benchmark_memory_hotspots(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation_hotspots");
    apply_profile(
        &mut group,
        Duration::from_secs(1),
        Duration::from_secs(5),
    );

    group.bench_function("supported_languages_vec", |b| {
        b.iter(|| black_box(supported_languages()))
    });

    group.bench_function("supported_languages_optimized", |b| {
        b.iter(|| black_box(supported_languages_optimized()))
    });

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

/// Stress test with extreme workload (1000x typical).
pub(crate) fn benchmark_extreme_stress_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("extreme_stress_test");
    apply_profile(
        &mut group,
        Duration::from_secs(5),
        Duration::from_secs(15),
    );

    let huge_text = if bench_fast() {
        "The quick brown fox jumps over the lazy dog. ".repeat(1000)
    } else {
        "The quick brown fox jumps over the lazy dog. ".repeat(10000)
    };

    group.throughput(Throughput::Bytes(huge_text.len() as u64));
    group.bench_function("detect_1mb_text", |b| {
        b.iter(|| black_box(detect_language(black_box(&huge_text))))
    });

    group.bench_function("sustained_detection_burst", |b| {
        let texts = vec![
            "Hello world",
            "Bonjour monde",
            "Hallo Welt",
            "Hola mundo",
            "Ciao mondo",
        ];

        b.iter(|| {
            let rounds = if bench_fast() { 10 } else { 100 };
            for _ in 0..rounds {
                for text in &texts {
                    let _ = black_box(detect_language(black_box(text)));
                }
            }
        })
    });

    group.finish();
}

/// Concurrent access performance test.
pub(crate) fn benchmark_concurrent_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_performance");
    apply_profile(
        &mut group,
        Duration::from_secs(2),
        Duration::from_secs(8),
    );

    let rt = tokio::runtime::Runtime::new().unwrap();

    let levels: &[usize] = if bench_fast() {
        &[1, 4, 8]
    } else {
        &[1, 4, 8, 16, 32]
    };

    for &concurrency in levels {
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
                            _ => "Hola mundo",
                        };

                        handles.push(tokio::spawn(async move {
                            detect_language_async(text).await
                        }));
                    }

                    for handle in handles {
                        let _ = black_box(handle.await);
                    }
                })
            },
        );
    }

    group.finish();
}

/// Regression detector pattern benchmarks.
pub(crate) fn benchmark_regex_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex_performance");
    apply_profile(
        &mut group,
        Duration::from_secs(2),
        Duration::from_secs(8),
    );

    let detector = LanguageDetector::new();
    let mut test_cases = vec![
        ("english_patterns", "Hello world thank you please"),
        ("french_patterns", "Bonjour monde merci s'il vous plaît"),
        ("german_patterns", "Hallo Welt danke bitte schön"),
        ("mixed_scripts", "Hello مرحبا 你好 こんにちは"),
        ("no_match_fallback", "xyz123 nonexistent patterns abc"),
        ("unicode_heavy", "مرحبا أهلاً وسهلاً كيف الحال؟"),
        ("cjk_patterns", "你好世界 こんにちは世界 안녕하세요 세계"),
    ];
    if bench_fast() {
        test_cases = vec![
            ("english_patterns", "Hello world thank you please"),
            ("mixed_scripts", "Hello مرحبا 你好 こんにちは"),
            ("no_match_fallback", "xyz123 nonexistent patterns abc"),
        ];
    }

    for (name, text) in test_cases {
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &text,
            |b, text| {
                b.iter(|| black_box(detector.detect(black_box(text))))
            },
        );
    }

    group.finish();
}
